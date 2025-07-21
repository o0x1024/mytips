use anyhow::{anyhow, Result};
use libsql::{Builder, Connection, Database};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn, error, debug};
use uuid::Uuid;

use super::models::*;
use super::operations::{create_all_tables, init_default_data};

/// 数据库模式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DatabaseMode {
    /// 本地数据库模式
    Local {
        path: String,
    },
    /// 远程数据库模式
    Remote {
        url: String,
        auth_token: String,
    },
    /// 嵌入式副本模式（推荐）
    EmbeddedReplica {
        local_path: String,
        remote_url: String,
        auth_token: String,
        sync_interval: Option<Duration>,
        read_your_writes: bool,
    },
    /// 内存数据库模式（用于测试）
    InMemory,
}

impl DatabaseMode {
    /// 是否为本地模式
    pub fn is_local(&self) -> bool {
        matches!(self, DatabaseMode::Local { .. } | DatabaseMode::InMemory)
    }

    /// 是否为远程模式
    pub fn is_remote(&self) -> bool {
        matches!(self, DatabaseMode::Remote { .. })
    }

    /// 是否为嵌入式副本模式
    pub fn is_embedded_replica(&self) -> bool {
        matches!(self, DatabaseMode::EmbeddedReplica { .. })
    }

    /// 是否支持同步
    pub fn supports_sync(&self) -> bool {
        matches!(self, DatabaseMode::EmbeddedReplica { .. })
    }

    /// 获取模式名称
    pub fn mode_name(&self) -> &'static str {
        match self {
            DatabaseMode::Local { .. } => "local",
            DatabaseMode::Remote { .. } => "remote",
            DatabaseMode::EmbeddedReplica { .. } => "embedded_replica",
            DatabaseMode::InMemory => "in_memory",
        }
    }
}

/// 数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// 数据库模式
    pub mode: DatabaseMode,
    /// 连接超时时间（秒）
    pub connection_timeout: u64,
    /// 查询超时时间（秒）
    pub query_timeout: u64,
    /// 最大连接数
    pub max_connections: u32,
    /// 是否启用WAL模式
    pub enable_wal: bool,
    /// 是否启用外键约束
    pub enable_foreign_keys: bool,
    /// 自定义PRAGMA设置
    pub pragma_settings: Vec<(String, String)>,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            mode: DatabaseMode::Local {
                path: ":memory:".to_string(),
            },
            connection_timeout: 30,
            query_timeout: 60,
            max_connections: 10,
            enable_wal: true,
            enable_foreign_keys: true,
            pragma_settings: vec![
                ("synchronous".to_string(), "NORMAL".to_string()),
                ("cache_size".to_string(), "2000".to_string()),
                ("temp_store".to_string(), "memory".to_string()),
                ("mmap_size".to_string(), "268435456".to_string()), // 256MB
            ],
        }
    }
}

/// 统一数据库管理器
#[derive(Clone)]
pub struct UnifiedDbManager {
    /// 当前数据库实例
    database: Arc<RwLock<Option<Database>>>,
    /// 数据库配置
    config: Arc<RwLock<DatabaseConfig>>,
    /// 应用句柄
    app_handle: AppHandle,
    /// 连接池
    connection_pool: Arc<Mutex<Vec<Connection>>>,
    /// 同步状态
    sync_status: Arc<RwLock<SyncStatus>>,
}

/// 同步状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatus {
    /// 是否正在同步
    pub is_syncing: bool,
    /// 最后同步时间
    pub last_sync_time: i64,
    /// 同步错误信息
    pub last_sync_error: Option<String>,
    /// 同步统计
    pub sync_stats: SyncStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    /// 总记录数
    pub total_records: u64,
    /// 已同步记录数
    pub synced_records: u64,
    /// 失败记录数
    pub failed_records: u64,
    /// 同步耗时（毫秒）
    pub duration_ms: u64,
}

impl Default for SyncStatus {
    fn default() -> Self {
        Self {
            is_syncing: false,
            last_sync_time: 0,
            last_sync_error: None,
            sync_stats: SyncStats {
                total_records: 0,
                synced_records: 0,
                failed_records: 0,
                duration_ms: 0,
            },
        }
    }
}

impl UnifiedDbManager {
    /// 创建新的数据库管理器
    pub async fn new(app_handle: AppHandle) -> Result<Self> {
        let manager = Self {
            database: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(DatabaseConfig::default())),
            app_handle,
            connection_pool: Arc::new(Mutex::new(Vec::new())),
            sync_status: Arc::new(RwLock::new(SyncStatus::default())),
        };

        // 尝试加载保存的数据库配置，如果没有则初始化默认数据库
        manager.initialize_with_saved_config().await?;

        Ok(manager)
    }

    /// 尝试使用保存的配置初始化，如果没有则使用默认配置
    async fn initialize_with_saved_config(&self) -> Result<()> {
        info!("Attempting to load saved database configuration");
        
        // 尝试加载保存的数据库配置
        match self.load_saved_database_config().await {
            Ok(Some(config)) => {
                info!("Found saved database configuration: {}", config.mode.mode_name());
                
                // 验证配置是否有效
                if self.validate_database_config(&config).await {
                    info!("Saved configuration is valid, attempting to apply it");
                    match self.switch_mode(config.clone()).await {
                        Ok(_) => {
                            info!("Successfully restored saved database configuration: {}", config.mode.mode_name());
                            return Ok(());
                        }
                        Err(e) => {
                            error!("Failed to apply saved configuration: {}", e);
                            error!("Error details: {:?}", e);
                            
                            // 对于嵌入式副本模式，尝试降级到本地模式但保留远程配置信息
                            if let DatabaseMode::EmbeddedReplica { local_path, .. } = &config.mode {
                                warn!("Attempting to fallback to local mode with embedded replica path: {}", local_path);
                                let fallback_config = DatabaseConfig {
                                    mode: DatabaseMode::Local {
                                        path: local_path.clone(),
                                    },
                                    ..config.clone()
                                };
                                
                                info!("Trying fallback to local mode...");
                                match self.switch_mode_without_saving(fallback_config).await {
                                    Ok(_) => {
                                        warn!("Successfully fallback to local mode, original embedded replica config preserved in file");
                                        info!("You can try switching back to embedded replica mode manually when the network is available");
                                        return Ok(());
                                    }
                                    Err(fallback_error) => {
                                        error!("Fallback to local mode also failed: {}", fallback_error);
                                    }
                                }
                            }
                            
                            warn!("All fallback attempts failed, using default configuration");
                        }
                    }
                } else {
                    warn!("Saved configuration validation failed, falling back to default");
                }
            }
            Ok(None) => {
                info!("No saved database configuration found, using default");
            }
            Err(e) => {
                warn!("Failed to load saved configuration: {}, using default", e);
            }
        }
        
        // 如果所有恢复尝试都失败，则初始化默认数据库
        warn!("All configuration restore attempts failed, initializing default database");
        self.initialize_default_database().await
    }

    /// 验证数据库配置是否有效
    async fn validate_database_config(&self, config: &DatabaseConfig) -> bool {
        match &config.mode {
            DatabaseMode::Local { path } => {
                // 对于本地模式，检查路径是否可访问
                if path == ":memory:" {
                    return true;
                }
                
                let path_buf = std::path::Path::new(path);
                if let Some(parent) = path_buf.parent() {
                    parent.exists() || std::fs::create_dir_all(parent).is_ok()
                } else {
                    true
                }
            }
            DatabaseMode::EmbeddedReplica { local_path, remote_url, auth_token, .. } => {
                // 检查本地路径
                if local_path != ":memory:" {
                    let path_buf = std::path::Path::new(local_path);
                    if let Some(parent) = path_buf.parent() {
                        if !parent.exists() && std::fs::create_dir_all(parent).is_err() {
                            warn!("Cannot create directory for local path: {}", local_path);
                            return false;
                        }
                    }
                }
                
                // 检查远程URL格式（支持更多格式用于开发和测试）
                if remote_url.is_empty() {
                    warn!("Empty remote URL");
                    return false;
                }
                
                // 支持 libsql://, http://, https:// 格式
                let valid_url_format = remote_url.starts_with("libsql://") || 
                                     remote_url.starts_with("http://") || 
                                     remote_url.starts_with("https://");
                
                if !valid_url_format {
                    warn!("Invalid remote URL format: {}. Expected libsql://, http://, or https://", remote_url);
                    return false;
                }
                
                info!("Remote URL format validation passed for: {}", remote_url);
                
                // 检查认证令牌（本地开发环境允许空token）
                let is_local_dev = remote_url.starts_with("http://127.0.0.1") || 
                                 remote_url.starts_with("http://localhost") ||
                                 remote_url.starts_with("https://127.0.0.1") ||
                                 remote_url.starts_with("https://localhost");
                
                if auth_token.is_empty() && !is_local_dev {
                    warn!("Empty auth token for embedded replica mode (production environment)");
                    return false;
                } else if auth_token.is_empty() && is_local_dev {
                    info!("Empty auth token allowed for local development environment: {}", remote_url);
                } else {
                    info!("Auth token provided for remote URL: {}", remote_url);
                }
                
                // 对于嵌入式副本模式，即使无法立即连接到远程也认为配置有效
                // 这样可以在应用启动时保持用户选择的模式，而不是因为网络问题回退到本地模式
                info!("Embedded replica configuration appears valid, proceeding without network validation");
                true
            }
            DatabaseMode::Remote { url, auth_token } => {
                // 检查远程URL和认证令牌
                !url.is_empty() && !auth_token.is_empty() && url.starts_with("libsql://")
            }
            DatabaseMode::InMemory => true,
        }
    }

    /// 加载保存的数据库配置
    async fn load_saved_database_config(&self) -> Result<Option<DatabaseConfig>> {
        let config_path = self.get_database_config_path()?;
        
        info!("Checking for saved database config at: {}", config_path.display());
        
        if !config_path.exists() {
            info!("No saved database config file found");
            return Ok(None);
        }
        
        info!("Reading saved database config file");
        let config_str = tokio::fs::read_to_string(&config_path).await
            .map_err(|e| anyhow!("Failed to read config file: {}", e))?;
        
        debug!("Config file content: {}", config_str);
        
        info!("Parsing database configuration");
        let config: DatabaseConfig = serde_json::from_str(&config_str)
            .map_err(|e| anyhow!("Failed to parse config: {}", e))?;
        
        info!("Successfully loaded saved database config: {}", config.mode.mode_name());
        
        // 记录关键配置信息
        match &config.mode {
            DatabaseMode::EmbeddedReplica { remote_url, auth_token, .. } => {
                info!("Loading embedded replica config: {}", remote_url);
                let is_local_dev = remote_url.starts_with("http://127.0.0.1") || 
                                 remote_url.starts_with("http://localhost") ||
                                 remote_url.starts_with("https://127.0.0.1") ||
                                 remote_url.starts_with("https://localhost");
                
                if is_local_dev && auth_token.is_empty() {
                    info!("Local dev environment detected, empty token allowed");
                }
            }
            DatabaseMode::Local { path } => {
                info!("Loading local database config: {}", path);
            }
            DatabaseMode::Remote { url, .. } => {
                info!("Loading remote database config: {}", url);
            }
            DatabaseMode::InMemory => {
                info!("Loading in-memory database config");
            }
        }
        
        Ok(Some(config))
    }

    /// 保存数据库配置到持久化存储
    async fn save_database_config(&self, config: &DatabaseConfig) -> Result<()> {
        let config_path = self.get_database_config_path()?;
        
        // 确保配置目录存在
        if let Some(parent) = config_path.parent() {
            tokio::fs::create_dir_all(parent).await
                .map_err(|e| anyhow!("Failed to create config directory: {}", e))?;
        }
        
        let config_str = serde_json::to_string_pretty(config)
            .map_err(|e| anyhow!("Failed to serialize config: {}", e))?;
        
        tokio::fs::write(&config_path, config_str).await
            .map_err(|e| anyhow!("Failed to write config file: {}", e))?;
        
        info!("Database configuration saved to: {}", config_path.display());
        Ok(())
    }

    /// 获取数据库配置文件路径
    fn get_database_config_path(&self) -> Result<std::path::PathBuf> {
        let config_dir = self.app_handle
            .path()
            .app_config_dir()
            .map_err(|e| anyhow!("Failed to get app config directory: {}", e))?;
        
        Ok(config_dir.join("database_config.json"))
    }

    /// 删除保存的数据库配置
    async fn remove_saved_database_config(&self) -> Result<()> {
        let config_path = self.get_database_config_path()?;
        
        if config_path.exists() {
            tokio::fs::remove_file(&config_path).await
                .map_err(|e| anyhow!("Failed to remove config file: {}", e))?;
            info!("Removed saved database configuration from: {}", config_path.display());
        }
        
        Ok(())
    }

    /// 初始化默认数据库（不保存配置）
    async fn initialize_default_database(&self) -> Result<()> {
        let db_path = self.get_default_db_path()?;
        let config = DatabaseConfig {
            mode: DatabaseMode::Local {
                path: db_path.to_string_lossy().to_string(),
            },
            ..Default::default()
        };

        // 使用不保存配置的模式切换
        self.switch_mode_without_saving(config).await
    }

    /// 切换数据库模式但不保存配置（用于回退到默认模式）
    async fn switch_mode_without_saving(&self, config: DatabaseConfig) -> Result<()> {
        info!("Starting database mode switch to: {} (without saving config)", config.mode.mode_name());

        // 1. 更新同步状态，标记正在切换
        {
            let mut sync_status = self.sync_status.write().await;
            sync_status.is_syncing = true;
            sync_status.last_sync_error = None;
        }

        let switch_result = async {
            // 2. 安全关闭当前数据库连接
            self.close_current_database().await?;

            // 3. 等待一段时间确保所有资源被释放
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            // 4. 创建新的数据库实例
            info!("Creating new database instance for mode: {}", config.mode.mode_name());
            let database = self.create_database(&config).await?;

            // 5. 初始化数据库结构
            info!("Initializing database schema");
            self.initialize_database_schema(&database).await?;

            // 6. 验证新数据库是否正常工作
            info!("Validating new database connection");
            let test_conn = database.connect()
                .map_err(|e| anyhow!("Failed to create test connection: {}", e))?;
            
            // 使用查询方法验证连接，而不是 execute
            let mut rows = test_conn.query("SELECT 1 as test_value", ()).await
                .map_err(|e| anyhow!("Database validation failed: {}", e))?;
            
            // 验证查询是否返回了预期的结果
            if rows.next().await.map_err(|e| anyhow!("Failed to read validation result: {}", e))?.is_none() {
                return Err(anyhow!("Database validation failed: no rows returned"));
            }

            // 7. 更新配置和数据库实例（但不保存到文件）
            *self.config.write().await = config.clone();
            *self.database.write().await = Some(database);

            info!("Database mode switch completed successfully (config not saved)");
            Ok::<(), anyhow::Error>(())
        }.await;

        // 8. 更新同步状态
        {
            let mut sync_status = self.sync_status.write().await;
            sync_status.is_syncing = false;
            sync_status.last_sync_time = chrono::Utc::now().timestamp_millis();
            
            match &switch_result {
                Ok(_) => {
                    sync_status.last_sync_error = None;
                    info!("Database mode switched successfully (without saving config)");
                }
                Err(e) => {
                    sync_status.last_sync_error = Some(e.to_string());
                    error!("Database mode switch failed: {}", e);
                }
            }
        }

        switch_result
    }

    /// 获取默认数据库路径
    fn get_default_db_path(&self) -> Result<PathBuf> {
        let app_dir = self.app_handle
            .path()
            .app_data_dir()
            .map_err(|e| anyhow!("Failed to get app data directory: {}", e))?;
        
        std::fs::create_dir_all(&app_dir)?;
        Ok(app_dir.join("mytips.db"))
    }

    /// 切换数据库模式
    pub async fn switch_mode(&self, config: DatabaseConfig) -> Result<()> {
        info!("Starting database mode switch to: {}", config.mode.mode_name());

        // 1. 更新同步状态，标记正在切换
        {
            let mut sync_status = self.sync_status.write().await;
            sync_status.is_syncing = true;
            sync_status.last_sync_error = None;
        }

        let switch_result = async {
            // 2. 安全关闭当前数据库连接
            self.close_current_database().await?;

            // 3. 等待一段时间确保所有资源被释放
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;

            // 4. 创建新的数据库实例
            info!("Creating new database instance for mode: {}", config.mode.mode_name());
            let database = self.create_database(&config).await?;

            // 5. 初始化数据库结构
            info!("Initializing database schema");
            self.initialize_database_schema(&database).await?;

            // 6. 验证新数据库是否正常工作
            info!("Validating new database connection");
            let test_conn = database.connect()
                .map_err(|e| anyhow!("Failed to create test connection: {}", e))?;
            
            // 使用查询方法验证连接，而不是 execute
            let mut rows = test_conn.query("SELECT 1 as test_value", ()).await
                .map_err(|e| anyhow!("Database validation failed: {}", e))?;
            
            // 验证查询是否返回了预期的结果
            if rows.next().await.map_err(|e| anyhow!("Failed to read validation result: {}", e))?.is_none() {
                return Err(anyhow!("Database validation failed: no rows returned"));
            }

            // 7. 更新配置和数据库实例
            *self.config.write().await = config.clone();
            *self.database.write().await = Some(database);

            // 8. 保存配置到持久化存储
            if let Err(e) = self.save_database_config(&config).await {
                warn!("Failed to save database configuration: {}", e);
                // 不要因为保存配置失败而让整个切换操作失败
            }

            info!("Database mode switch completed successfully");
            Ok::<(), anyhow::Error>(())
        }.await;

        // 9. 更新同步状态
        {
            let mut sync_status = self.sync_status.write().await;
            sync_status.is_syncing = false;
            sync_status.last_sync_time = chrono::Utc::now().timestamp_millis();
            
            match &switch_result {
                Ok(_) => {
                    sync_status.last_sync_error = None;
                    info!("Database mode switched successfully");
                }
                Err(e) => {
                    sync_status.last_sync_error = Some(e.to_string());
                    error!("Database mode switch failed: {}", e);
                }
            }
        }

        switch_result
    }

    /// 创建数据库实例
    async fn create_database(&self, config: &DatabaseConfig) -> Result<Database> {
        let database = match &config.mode {
            DatabaseMode::Local { path } => {
                info!("Creating local database at: {}", path);
                self.create_local_database(path, config).await?
            },
            DatabaseMode::Remote { url, auth_token } => {
                info!("Creating remote database connection to: {}", url);
                self.create_remote_database(url, auth_token, config).await?
            },
            DatabaseMode::EmbeddedReplica {
                local_path,
                remote_url,
                auth_token,
                sync_interval,
                read_your_writes,
            } => {
                info!("Creating embedded replica: {} -> {}", local_path, remote_url);
                self.create_embedded_replica_database(
                    local_path,
                    remote_url,
                    auth_token,
                    *sync_interval,
                    *read_your_writes,
                    config,
                ).await?
            },
            DatabaseMode::InMemory => {
                info!("Creating in-memory database");
                self.create_local_database(":memory:", config).await?
            },
        };

        Ok(database)
    }

    /// 创建本地数据库
    async fn create_local_database(&self, path: &str, config: &DatabaseConfig) -> Result<Database> {
        // 确保父目录存在
        if path != ":memory:" {
            if let Some(parent) = std::path::Path::new(path).parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
        }

        let mut builder = Builder::new_local(path);
        
        // 应用配置
        // Note: libsql Builder 没有直接的超时配置，我们在连接时设置

        let database = builder.build().await
            .map_err(|e| anyhow!("Failed to create local database: {}", e))?;

        Ok(database)
    }

    /// 创建远程数据库
    async fn create_remote_database(&self, url: &str, auth_token: &str, _config: &DatabaseConfig) -> Result<Database> {
        let builder = Builder::new_remote(url.to_string(), auth_token.to_string());
        
        let database = builder.build().await
            .map_err(|e| anyhow!("Failed to create remote database: {}", e))?;

        Ok(database)
    }

    /// 创建嵌入式副本数据库
    async fn create_embedded_replica_database(
        &self,
        local_path: &str,
        remote_url: &str,
        auth_token: &str,
        sync_interval: Option<Duration>,
        read_your_writes: bool,
        _config: &DatabaseConfig,
    ) -> Result<Database> {
        info!(
            "Attempting to create or reuse embedded replica: {} -> {}",
            local_path, remote_url
        );

        let local_path_buf = PathBuf::from(local_path);
        
        // 1. 检查是否存在可重用的本地副本
        if local_path_buf.exists() {
            match self.validate_local_replica(local_path).await {
                Ok(_) => {
                    info!("Validation successful. Reusing existing local replica.");
                    // 构建数据库实例并直接返回
                    return Self::build_replica(local_path, remote_url, auth_token, sync_interval, read_your_writes)
                        .await
                        .map_err(|e| anyhow!("Failed to build database from existing replica: {}. Please try restarting the application.", e));
                }
                Err(e) => {
                    warn!("Local replica validation failed: {}. It will be recreated.", e);
                }
            }
        } else {
            info!("No local replica found. A new one will be created.");
        }
        
        // 2. 如果无法重用，则执行完整的重建流程
        info!("Proceeding with full recreation of the local replica.");
        self.create_fresh_embedded_replica(local_path, remote_url, auth_token, sync_interval, read_your_writes).await
    }

    /// 构建并返回一个libsql远程副本数据库实例
    async fn build_replica(
        local_path: &str,
        remote_url: &str,
        auth_token: &str,
        sync_interval: Option<Duration>,
        read_your_writes: bool,
    ) -> Result<Database> {
        let mut builder = Builder::new_remote_replica(
            local_path.to_string(),
            remote_url.to_string(),
            auth_token.to_string(),
        );
        
        builder = builder.read_your_writes(read_your_writes);

        if let Some(interval) = sync_interval {
            builder = builder.sync_interval(interval);
        }

        builder.build().await.map_err(|e| e.into())
    }

    /// 创建一个全新的嵌入式副本数据库，包含清理和复杂的后同步逻辑
    async fn create_fresh_embedded_replica(
        &self,
        local_path: &str,
        remote_url: &str,
        auth_token: &str,
        sync_interval: Option<Duration>,
        read_your_writes: bool,
    ) -> Result<Database> {
        info!(
            "Creating fresh embedded replica: {} -> {}",
            local_path, remote_url
        );

        // 在创建副本前，先清理可能存在的旧文件
        info!(
            "Cleaning up existing database files for embedded replica creation: {}",
            local_path
        );
        self.cleanup_existing_database_files(local_path).await;

        let mut last_error: Option<anyhow::Error> = None;
        const MAX_ATTEMPTS: usize = 3;

        for attempt in 1..=MAX_ATTEMPTS {
            info!(
                "Creating embedded replica attempt {} of {}",
                attempt, MAX_ATTEMPTS
            );

            // 构建数据库实例
            match Self::build_replica(local_path, remote_url, auth_token, sync_interval, read_your_writes).await {
                Ok(database) => {
                    info!(
                        "Embedded replica created successfully on attempt {}",
                        attempt
                    );

                    match database.connect() {
                        Ok(conn) => {
                            // 设置安全的PRAGMA参数
                            if let Err(e) = self.setup_safe_pragmas(&conn).await {
                                warn!("Failed to setup PRAGMA settings: {}", e);
                            }
                            
                            // 步骤1: 创建本地schema（在同步之前）
                            info!("Creating comprehensive local schema before sync operations");
                            match self.ensure_replica_schema(&conn).await {
                                Ok(_) => {
                                    info!("Initial local schema creation successful");
                                },
                                Err(e) => {
                                    error!("Initial local schema creation failed: {}", e);
                                    last_error = Some(anyhow!("Initial local schema setup failed: {}", e));
                                    continue;
                                }
                            }
                            
                            // 步骤2: 执行同步操作（可能会覆盖部分本地数据）
                            info!("Performing initial sync operation");
                            let sync_result = database.sync().await;
                            
                            match sync_result {
                                Ok(_) => {
                                    info!("Initial sync completed successfully");
                                },
                                Err(e) => {
                                    warn!("Initial sync failed: {}", e);
                                    // 同步失败不应该阻止副本创建，但需要记录
                                }
                            }
                            
                            // 步骤3: 同步后立即验证和修复schema
                            info!("Post-sync schema verification and repair");
                            
                            // 多次重试schema验证和修复，以应对同步延迟
                            let mut schema_rebuild_attempts = 0;
                            const MAX_SCHEMA_REBUILD_ATTEMPTS: usize = 3;
                            
                            loop {
                                schema_rebuild_attempts += 1;
                                
                                match self.verify_critical_tables(&conn).await {
                                    Ok(_) => {
                                        info!("Post-sync schema verification passed on attempt {}", schema_rebuild_attempts);
                                        break;
                                    },
                                    Err(e) => {
                                        warn!("Post-sync schema verification failed on attempt {}: {}", schema_rebuild_attempts, e);
                                        
                                        if schema_rebuild_attempts > MAX_SCHEMA_REBUILD_ATTEMPTS {
                                            error!("Schema verification failed after {} attempts, giving up", MAX_SCHEMA_REBUILD_ATTEMPTS);
                                            last_error = Some(anyhow!("Persistent schema verification failure after {} attempts: {}", MAX_SCHEMA_REBUILD_ATTEMPTS, e));
                                            break;
                                        }
                                        
                                        // 第一次尝试失败后，稍作等待
                                        if schema_rebuild_attempts == 1 {
                                            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
                                        }

                                        // 如果同步后schema丢失，重新创建
                                        info!("Rebuilding schema after sync operation (attempt {})", schema_rebuild_attempts);
                                        match self.ensure_replica_schema_post_sync(&conn).await {
                                            Ok(_) => {
                                                info!("Post-sync schema rebuild successful on attempt {}", schema_rebuild_attempts);
                                                
                                                // 短暂等待后重新验证
                                                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                                            },
                                            Err(rebuild_error) => {
                                                error!("Post-sync schema rebuild failed on attempt {}: {}", schema_rebuild_attempts, rebuild_error);
                                                
                                                if schema_rebuild_attempts > MAX_SCHEMA_REBUILD_ATTEMPTS {
                                                    last_error = Some(anyhow!("Post-sync schema rebuild failed after {} attempts: {}", MAX_SCHEMA_REBUILD_ATTEMPTS, rebuild_error));
                                                    break;
                                                }
                                                
                                                // 短暂等待后重试
                                                tokio::time::sleep(std::time::Duration::from_millis(2000)).await;
                                            }
                                        }
                                    }
                                }
                            }
                            
                            // 如果schema重建失败，继续下一次尝试
                            if last_error.is_some() {
                                continue;
                            }
                            
                            info!("Schema verification and rebuild completed successfully");
                            
                            // 步骤4: 执行额外的同步以确保schema更新到远程
                            info!("Performing final sync to push local schema to remote");
                            if let Err(e) = database.sync().await {
                                warn!("Final schema sync failed: {}", e);
                                // 这个失败不应该阻止整个过程
                            } else {
                                info!("Final schema sync completed successfully");
                            }
                            
                            // 最终等待时间
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            
                            info!("Embedded replica database is ready and verified");
                            return Ok(database);
                        }
                        Err(e) => {
                            error!("Failed to connect to newly created replica: {}", e);
                            last_error = Some(anyhow!("Connection failed: {}", e));
                        }
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to build embedded replica on attempt {}: {}",
                        attempt, e
                    );
                    last_error = Some(anyhow!("Build failed: {}", e));

                    // 如果是IO错误，可能是文件锁定问题，等待一段时间再试
                    if e.to_string().contains("IO error") {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| anyhow!("Failed to create embedded replica after all attempts")))
    }
    
    /// 验证本地副本数据库的健康状况
    async fn validate_local_replica(&self, path: &str) -> Result<()> {
        info!("Validating existing local replica at: {}", path);

        // 1. 尝试作为普通本地数据库连接
        let db = Builder::new_local(path).build().await
            .map_err(|e| anyhow!("Failed to open local replica for validation: {}", e))?;
        let conn = db.connect()
            .map_err(|e| anyhow!("Failed to connect to local replica for validation: {}", e))?;

        // 2. 验证关键表结构
        self.verify_critical_tables(&conn).await?;

        // 3. (可选) 更深入的检查，例如检查是否有数据
        if let Ok(mut rows) = conn.query("SELECT count(*) FROM tips", ()).await {
            if let Some(row) = rows.next().await? {
                let count: i64 = row.get(0)?;
                info!("Validation check: Found {} tips in existing replica.", count);
            }
        }

        info!("Local replica at {} is valid and reusable.", path);
        Ok(())
    }

    /// 确保副本数据库的表结构正确
    async fn ensure_replica_schema(&self, conn: &Connection) -> Result<()> {
        info!("Ensuring replica database schema");
        
        // 列出当前存在的表，用于调试
        match conn.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", ()).await {
            Ok(mut table_rows) => {
                let mut table_names = Vec::new();
                while let Some(row) = table_rows.next().await? {
                    table_names.push(row.get::<String>(0)?);
                }
                info!("Found {} tables in replica database before schema creation", table_names.len());
            },
            Err(list_error) => {
                error!("Failed to list tables for debugging: {}", list_error);
            }
        }
        
        // 创建所有表
        info!("Creating all required tables in replica database");
        conn.execute("BEGIN", ()).await?;
        match create_all_tables(conn).await {
            Ok(_) => {
                info!("Successfully created all tables in transaction");
                conn.execute("COMMIT", ()).await?;
            }
            Err(e) => {
                error!("Failed to create tables in transaction: {}", e);
                conn.execute("ROLLBACK", ()).await?;
                return Err(e);
            }
        }
        
        // 再次列出存在的表，确认创建成功
        match conn.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", ()).await {
            Ok(mut table_rows) => {
                let mut table_names = Vec::new();
                while let Some(row) = table_rows.next().await? {
                    table_names.push(row.get::<String>(0)?);
                }
                info!("Found {} tables in replica database after schema creation", table_names.len());
            },
            Err(list_error) => {
                error!("Failed to list tables for debugging: {}", list_error);
            }
        }
        
        // 初始化默认数据（仅在表为空时）
        if let Err(e) = init_default_data(conn).await {
            warn!("Failed to initialize default data in replica: {}", e);
            // 不要因为默认数据失败而阻止整个过程
        }
        
        Ok(())
    }

    /// 同步后确保副本数据库的表结构正确（专用于同步后修复）
    async fn ensure_replica_schema_post_sync(&self, conn: &Connection) -> Result<()> {
        info!("Rebuilding replica database schema after sync operation");
        
        // 列出当前存在的表，用于调试
        match conn.query("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name", ()).await {
            Ok(mut table_rows) => {
                let mut table_names = Vec::new();
                while let Some(row) = table_rows.next().await? {
                    table_names.push(row.get::<String>(0)?);
                }
                info!("Current tables in database before post-sync repair: {:?}", table_names);
            },
            Err(list_error) => {
                error!("Failed to list tables for debugging: {}", list_error);
            }
        }
        
        // 强制等待确保所有锁都被释放
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        // 禁用外键约束，避免创建表时的顺序问题
        if let Err(e) = conn.execute("PRAGMA foreign_keys = OFF", ()).await {
            warn!("Failed to disable foreign keys: {}", e);
        }
        
        // 使用事务重新创建表，增强错误处理
        let transaction_result = async {
            conn.execute("BEGIN IMMEDIATE", ()).await?;
            
            match create_all_tables(conn).await {
                Ok(_) => {
                    info!("Successfully recreated all tables after sync");
                    
                    // 验证关键表确实已创建
                    let verification_result = self.verify_critical_tables(conn).await;
                    if let Err(e) = verification_result {
                        warn!("Table verification failed immediately after creation: {}", e);
                        conn.execute("ROLLBACK", ()).await?;
                        return Err(anyhow!("Table verification failed after creation: {}", e));
                    }
                    
                    conn.execute("COMMIT", ()).await?;
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to recreate tables after sync: {}", e);
                    conn.execute("ROLLBACK", ()).await?;
                    Err(anyhow!("Failed to recreate tables after sync: {}", e))
                }
            }
        }.await;
        
        match transaction_result {
            Ok(_) => {
                info!("Transaction completed successfully");
            },
            Err(e) => {
                error!("Transaction failed: {}", e);
                
                // 尝试强制释放任何剩余的锁
                let _ = conn.execute("PRAGMA optimize", ()).await;
                tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                
                return Err(e);
            }
        }
        
        // 重新启用外键约束
        if let Err(e) = conn.execute("PRAGMA foreign_keys = ON", ()).await {
            warn!("Failed to re-enable foreign keys: {}", e);
        }
        
        // 验证表创建结果
        self.verify_critical_tables(conn).await
    }
    
    /// 修复副本数据库的表结构
    async fn repair_replica_schema(&self, conn: &Connection) -> Result<()> {
        info!("Repairing replica database schema");
        
        // 直接调用 create_all_tables 来确保所有表都存在
        // 这是最安全和可靠的方法，因为所有的 CREATE TABLE 语句都使用了 IF NOT EXISTS
        create_all_tables(conn).await.map_err(|e| {
            anyhow!("Failed to repair schema using create_all_tables: {}", e)
        })?;
        
        info!("Replica schema repair completed using create_all_tables");
        Ok(())
    }

    /// 设置安全的PRAGMA参数
    async fn setup_safe_pragmas(&self, conn: &Connection) -> Result<()> {
        debug!("Setting up safe PRAGMA parameters for embedded replica");

        // 对于嵌入式副本，大多数文件级PRAGMA都不支持或由远程管理
        // 我们只设置最关键的、且被支持的PRAGMA

        // 确保启用了外键
        if let Err(e) = conn.execute("PRAGMA foreign_keys=ON", ()).await {
            warn!("Failed to enable foreign keys in safe pragmas setup: {}", e);
        } else {
            debug!("Enabled foreign keys in safe pragmas setup for embedded replica");
        }

        Ok(())
    }

    /// 优化WAL文件大小（专用于嵌入式副本）
    pub async fn optimize_wal_files(&self) -> Result<()> {
        let database_guard = self.database.read().await;
        let database = database_guard.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;

        let conn = database.connect()
            .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;

        info!("Starting WAL optimization");

        // 1. 执行 WAL 检查点，将数据移动到主数据库
        match conn.execute("PRAGMA wal_checkpoint(RESTART)", ()).await {
            Ok(_) => {
                info!("Successfully executed WAL checkpoint");
            },
            Err(e) => {
                warn!("WAL checkpoint failed: {}", e);
            }
        }

        // 2. 分析数据库以优化内部结构
        if let Err(e) = conn.execute("PRAGMA optimize", ()).await {
            warn!("Database optimization failed: {}", e);
        } else {
            debug!("Database optimization completed");
        }

        // 3. 收缩数据库以回收空间（如果支持）
        if let Err(e) = conn.execute("VACUUM", ()).await {
            warn!("Database vacuum failed: {}", e);
        } else {
            info!("Database vacuum completed");
        }

        // 4. 获取WAL文件信息用于诊断
        match conn.query("PRAGMA wal_checkpoint", ()).await {
            Ok(mut rows) => {
                if let Ok(Some(row)) = rows.next().await {
                    let busy: i64 = row.get(0).unwrap_or(0);
                    let log: i64 = row.get(1).unwrap_or(0);
                    let checkpointed: i64 = row.get(2).unwrap_or(0);
                    
                    info!("WAL status - Busy: {}, Log pages: {}, Checkpointed: {}", busy, log, checkpointed);
                    
                    if log > 1000 {
                        warn!("WAL file is large ({} pages), consider more frequent checkpoints", log);
                    }
                }
            },
            Err(e) => {
                debug!("Could not get WAL status: {}", e);
            }
        }

        info!("WAL optimization completed");
        Ok(())
    }

    /// 定期WAL维护任务
    pub async fn start_wal_maintenance(&self) -> Result<()> {
        let self_clone = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(300)); // 5分钟间隔
            
            loop {
                interval.tick().await;
                
                // 只在嵌入式副本模式下执行WAL维护
                let config = self_clone.config.read().await;
                if config.mode.is_embedded_replica() {
                    drop(config);
                    
                    if let Err(e) = self_clone.optimize_wal_files().await {
                        debug!("WAL maintenance failed: {}", e);
                    } else {
                        debug!("WAL maintenance completed successfully");
                    }
                } else {
                    drop(config);
                    break; // 如果不是嵌入式副本模式，退出维护任务
                }
            }
        });
        
        info!("WAL maintenance task started");
        Ok(())
    }

    /// 初始化数据库结构
    async fn initialize_database_schema(&self, database: &Database) -> Result<()> {
        info!("Initializing database schema");
        
        let conn = database.connect()
            .map_err(|e| anyhow!("Failed to connect to database: {}", e))?;

        // 应用PRAGMA设置（跳过嵌入式副本模式不支持的设置）
        let config = self.config.read().await;
        let is_embedded_replica = matches!(config.mode, DatabaseMode::EmbeddedReplica { .. });
        
        for (key, value) in &config.pragma_settings {
            // 嵌入式副本模式跳过某些不支持的PRAGMA设置
            if is_embedded_replica && matches!(key.as_str(), "synchronous" | "cache_size" | "temp_store" | "mmap_size" | "busy_timeout" | "wal_autocheckpoint") {
                debug!("Skipping PRAGMA {} for embedded replica mode", key);
                continue;
            }
            
            let pragma_sql = format!("PRAGMA {} = {}", key, value);
            if let Err(e) = conn.execute(&pragma_sql, ()).await {
                warn!("Failed to set PRAGMA {} = {}: {}", key, value, e);
            } else {
                debug!("Successfully set PRAGMA {} = {}", key, value);
            }
        }

        if config.enable_foreign_keys {
            conn.execute("PRAGMA foreign_keys = ON", ()).await
                .map_err(|e| anyhow!("Failed to enable foreign keys: {}", e))?;
        }

        // 创建表结构 - 增强错误处理
        match create_all_tables(&conn).await {
            Ok(_) => {
                info!("Database tables created successfully");
            },
            Err(e) => {
                error!("Failed to create database tables: {}", e);
                
                // 尝试诊断具体哪个表创建失败
                if let Err(diag_error) = self.diagnose_table_creation_failure(&conn).await {
                    error!("Table creation diagnosis failed: {}", diag_error);
                }
                
                return Err(anyhow!("Table creation failed: {}", e));
            }
        }
        
        // 验证关键表是否存在
        match self.verify_critical_tables(&conn).await {
            Ok(_) => {
                info!("Critical tables verification passed");
            },
            Err(e) => {
                error!("Critical tables verification failed: {}", e);
                
                // 尝试重新创建缺失的表
                if let Err(recreate_error) = self.recreate_missing_tables(&conn).await {
                    error!("Failed to recreate missing tables: {}", recreate_error);
                }
                
                return Err(anyhow!("Critical tables verification failed: {}", e));
            }
        }
        
        // 初始化默认数据
        match init_default_data(&conn).await {
            Ok(_) => {
                info!("Default data initialized successfully");
            },
            Err(e) => {
                warn!("Failed to initialize default data: {}", e);
                // 默认数据初始化失败不应该阻止数据库初始化
            }
        }

        info!("Database schema initialized successfully");
        Ok(())
    }

    /// 诊断表创建失败原因
    async fn diagnose_table_creation_failure(&self, conn: &Connection) -> Result<()> {
        info!("Diagnosing table creation failure");
        
        // 检查数据库是否可写
        match conn.execute("CREATE TEMP TABLE test_write (id INTEGER)", ()).await {
            Ok(_) => {
                conn.execute("DROP TABLE test_write", ()).await.ok();
                info!("Database is writable");
            },
            Err(e) => {
                error!("Database is not writable: {}", e);
                return Err(anyhow!("Database is not writable: {}", e));
            }
        }
        
        // 检查外键约束状态
        let mut rows = conn.query("PRAGMA foreign_keys", ()).await?;
        if let Some(row) = rows.next().await? {
            let fk_enabled: i32 = row.get(0)?;
            info!("Foreign keys enabled: {}", fk_enabled == 1);
        }
        
        // 检查已存在的表
        let mut tables = Vec::new();
        let mut rows = conn.query("SELECT name FROM sqlite_master WHERE type='table'", ()).await?;
        while let Some(row) = rows.next().await? {
            let table_name: String = row.get(0)?;
            tables.push(table_name);
        }
        info!("Existing tables: {:?}", tables);
        
        Ok(())
    }

    /// 验证关键表是否存在
    async fn verify_critical_tables(&self, conn: &Connection) -> Result<()> {
        let critical_tables = vec![
            "categories",
            "tags", 
            "tips",
            "tip_tags",
            "ai_roles",
            "ai_conversations", 
            "ai_messages",
            "app_settings",
            "sync_config"
        ];
        
        for table in &critical_tables {
            println!("Checking table: {}", table);
            let mut rows = conn.query(
                "SELECT name FROM sqlite_master WHERE type='table' AND name=?", 
                [*table]
            ).await?;
            
            if rows.next().await?.is_none() {
                println!("❌ Table {} does not exist", table);
                return Err(anyhow!("Critical table '{}' does not exist", table));
            }
        }
        
        // 特别检查ai_roles表结构
        match self.verify_ai_roles_table_structure(conn).await {
            Ok(_) => {
                info!("ai_roles table structure is correct");
            },
            Err(e) => {
                error!("ai_roles table structure verification failed: {}", e);
                return Err(e);
            }
        }
        
        info!("All critical tables exist and are properly structured");
        Ok(())
    }

    /// 验证ai_roles表结构
    async fn verify_ai_roles_table_structure(&self, conn: &Connection) -> Result<()> {
        // 检查表是否存在
        let mut rows = conn.query(
            "SELECT sql FROM sqlite_master WHERE type='table' AND name='ai_roles'", 
            ()
        ).await?;
        
        if let Some(row) = rows.next().await? {
            let create_sql: String = row.get(0)?;
            
            // 检查必要的列是否存在
            let required_columns = vec!["id", "name", "system_prompt", "created_at", "updated_at"];
            for column in required_columns {
                if !create_sql.contains(column) {
                    return Err(anyhow!("ai_roles table missing required column: {}", column));
                }
            }
        } else {
            return Err(anyhow!("ai_roles table does not exist"));
        }
        
        // 尝试查询表结构
        match conn.query("PRAGMA table_info(ai_roles)", ()).await {
            Ok(mut rows) => {
                let mut columns = Vec::new();
                while let Some(row) = rows.next().await? {
                    let column_name: String = row.get(1)?;
                    columns.push(column_name);
                }
            },
            Err(e) => {
                return Err(anyhow!("Failed to get ai_roles table info: {}", e));
            }
        }
        
        // 尝试执行简单查询测试表功能
        match conn.query("SELECT COUNT(*) FROM ai_roles", ()).await {
            Ok(_) => {
                info!("ai_roles table is functional");
            },
            Err(e) => {
                return Err(anyhow!("ai_roles table is not functional: {}", e));
            }
        }
        
        Ok(())
    }

    /// 重新创建缺失的表
    async fn recreate_missing_tables(&self, conn: &Connection) -> Result<()> {
        info!("Attempting to recreate missing tables");
        
        // 特别处理ai_roles表
        let mut rows = conn.query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='ai_roles'", 
            ()
        ).await?;
        
        if rows.next().await?.is_none() {
            warn!("ai_roles table is missing, attempting to recreate");
            
            // 创建ai_roles表
            conn.execute(
                "CREATE TABLE IF NOT EXISTS ai_roles (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    system_prompt TEXT NOT NULL,
                    model TEXT,
                    temperature REAL DEFAULT 0.7,
                    max_tokens INTEGER,
                    is_default BOOLEAN DEFAULT FALSE,
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL
                )",
                (),
            ).await.map_err(|e| anyhow!("Failed to recreate ai_roles table: {}", e))?;
            
            info!("ai_roles table recreated successfully");
        }
        
        // 可以在这里添加其他关键表的重新创建逻辑
        
        Ok(())
    }

    /// 关闭当前数据库
    async fn close_current_database(&self) -> Result<()> {
        info!("Closing current database connections");
        
        // 1. 清空连接池 - 强制关闭所有连接
        {
            let mut pool = self.connection_pool.lock().await;
            let connection_count = pool.len();
            pool.clear();
            info!("Cleared {} connections from pool", connection_count);
        }

        // 2. 获取当前数据库实例并执行安全关闭
        let current_db = {
            let mut db_guard = self.database.write().await;
            db_guard.take()
        };

        if let Some(database) = current_db {
            // 3. 对于嵌入式副本模式，先执行最后一次同步
            let current_config = self.config.read().await;
            if current_config.mode.is_embedded_replica() {
                info!("Performing final sync before closing embedded replica");
                if let Err(e) = database.sync().await {
                    warn!("Final sync failed during close: {}", e);
                }
            }

            // 4. 显式等待一段时间让 WAL 检查点完成
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            
            // 5. 如果是本地或副本数据库，清理 WAL 相关文件
            if let DatabaseMode::Local { path } | 
               DatabaseMode::EmbeddedReplica { local_path: path, .. } = &current_config.mode {
                if path != ":memory:" {
                    self.cleanup_wal_files(path).await;
                }
            }
            drop(current_config);
        }

        // 6. 额外的等待时间确保所有资源都被释放
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;

        info!("Database closed successfully");
        Ok(())
    }

    /// 清理 WAL 相关文件
    async fn cleanup_wal_files(&self, db_path: &str) {
        if db_path == ":memory:" {
            return;
        }

        let wal_path = format!("{}-client_wal_index", db_path);
        let wal_path = format!("{}-_wal", db_path);
        let shm_path = format!("{}-shm", db_path);
        let journal_path = format!("{}-journal", db_path);
        
        info!("Starting WAL cleanup for database: {}", db_path);
        
        // 清理 WAL 文件
        match tokio::fs::remove_file(&wal_path).await {
            Ok(_) => {
                info!("Successfully removed WAL file: {}", wal_path);
            },
            Err(e) => {
                debug!("Could not remove WAL file {}: {} (file may not exist)", wal_path, e);
            }
        }
        
        // 清理 SHM 文件  
        match tokio::fs::remove_file(&shm_path).await {
            Ok(_) => {
                info!("Successfully removed SHM file: {}", shm_path);
            },
            Err(e) => {
                debug!("Could not remove SHM file {}: {} (file may not exist)", shm_path, e);
            }
        }
        
        // 清理 journal 文件
        match tokio::fs::remove_file(&journal_path).await {
            Ok(_) => {
                info!("Successfully removed journal file: {}", journal_path);
            },
            Err(e) => {
                debug!("Could not remove journal file {}: {} (file may not exist)", journal_path, e);
            }
        }
        
        info!("WAL cleanup completed for database: {}", db_path);
    }

    /// 更彻底地清理现有数据库文件（用于嵌入式副本创建）
    async fn cleanup_existing_database_files(&self, db_path: &str) {
        if db_path == ":memory:" {
            return;
        }

        info!("Cleaning up existing database files for embedded replica creation: {}", db_path);

        let main_db_path = std::path::Path::new(db_path);
        let wal_path = format!("{}-wal", db_path);
        let shm_path = format!("{}-shm", db_path);
        let journal_path = format!("{}-journal", db_path);

        // 检查主数据库文件是否存在
        if main_db_path.exists() {
            info!("Existing database file found, will remove it to create fresh replica");
            
            // 先清理WAL和SHM文件
            for file_path in &[&wal_path, &shm_path, &journal_path] {
                if let Err(e) = tokio::fs::remove_file(file_path).await {
                    debug!("Could not remove file {}: {} (may not exist)", file_path, e);
                } else {
                    info!("Removed file: {}", file_path);
                }
            }

            // 最后移除主数据库文件
            if let Err(e) = tokio::fs::remove_file(main_db_path).await {
                warn!("Could not remove main database file {}: {}", db_path, e);
            } else {
                info!("Removed main database file: {}", db_path);
            }
        } else {
            // 即使主文件不存在，也要清理可能存在的WAL文件
            self.cleanup_wal_files(db_path).await;
        }
        
        // 额外清理：确保所有可能的SQLite相关文件都被删除
        let additional_files = vec![
            format!("{}-client_wal_index", db_path),
            format!("{}-wal", db_path),
            format!("{}-shm", db_path), 
            format!("{}-journal", db_path),
        ];
        
        for file_path in &additional_files {
            if let Err(e) = tokio::fs::remove_file(file_path).await {
                debug!("Additional cleanup: Could not remove {}: {} (may not exist)", file_path, e);
            } else {
                debug!("Additional cleanup: Removed {}", file_path);
            }
        }
        
        info!("Database file cleanup completed for: {}", db_path);
    }

    /// 获取数据库连接
    pub async fn get_connection(&self) -> Result<Connection> {
        let database_guard = self.database.read().await;
        let database = database_guard.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;

        let conn = database.connect()
            .map_err(|e| anyhow!("Failed to create connection: {}", e))?;

        Ok(conn)
    }

    /// 获取数据库连接（DbManager兼容别名）
    pub async fn get_conn(&self) -> Result<Connection> {
        self.get_connection().await
    }

    /// 获取带连接池的连接
    pub async fn get_pooled_connection(&self) -> Result<Connection> {
        // 尝试从连接池获取连接
        {
            let mut pool = self.connection_pool.lock().await;
            if let Some(conn) = pool.pop() {
                // 验证连接是否有效
                if self.validate_connection(&conn).await.is_ok() {
                    return Ok(conn);
                }
            }
        }

        // 如果连接池为空或连接无效，创建新连接
        self.get_connection().await
    }

    /// 归还连接到连接池
    pub async fn return_connection(&self, connection: Connection) {
        let config = self.config.read().await;
        let mut pool = self.connection_pool.lock().await;
        
        if pool.len() < config.max_connections as usize {
            pool.push(connection);
        }
        // 如果连接池已满，连接会被自动丢弃
    }

    /// 验证连接是否有效
    async fn validate_connection(&self, conn: &Connection) -> Result<()> {
        // 使用 query 方法而不是 execute 来执行 SELECT 语句
        let mut rows = conn.query("SELECT 1 as test_value", ()).await
            .map_err(|e| anyhow!("Connection validation failed: {}", e))?;
        
        // 确保查询返回了结果
        if rows.next().await.map_err(|e| anyhow!("Failed to read validation result: {}", e))?.is_none() {
            return Err(anyhow!("Connection validation failed: no rows returned"));
        }
        
        Ok(())
    }

    /// 执行同步（仅适用于嵌入式副本模式）
    pub async fn sync(&self) -> Result<()> {
        let config = self.config.read().await;
        if !config.mode.supports_sync() {
            return Err(anyhow!("Current database mode does not support sync"));
        }

        let database_guard = self.database.read().await;
        let database = database_guard.as_ref()
            .ok_or_else(|| anyhow!("Database not initialized"))?;

        // 更新同步状态
        {
            let mut status = self.sync_status.write().await;
            status.is_syncing = true;
            status.last_sync_error = None;
        }

        let start_time = std::time::Instant::now();

        let sync_result = database.sync().await;

        let duration = start_time.elapsed();

        // 更新同步状态
        {
            let mut status = self.sync_status.write().await;
            status.is_syncing = false;
            status.last_sync_time = chrono::Utc::now().timestamp_millis();
            status.sync_stats.duration_ms = duration.as_millis() as u64;

            match sync_result {
                Ok(_) => {
                    info!("Database sync completed successfully in {:?}", duration);
                    status.last_sync_error = None;
                    status.sync_stats.synced_records += 1; // libSQL不提供具体记录数
                },
                Err(e) => {
                    error!("Database sync failed: {}", e);
                    status.last_sync_error = Some(e.to_string());
                    status.sync_stats.failed_records += 1;
                    return Err(anyhow!("Sync failed: {}", e));
                }
            }
        }

        Ok(())
    }

    /// 获取当前数据库模式
    pub async fn get_current_mode(&self) -> DatabaseMode {
        self.config.read().await.mode.clone()
    }

    /// 获取同步状态
    pub async fn get_sync_status(&self) -> SyncStatus {
        self.sync_status.read().await.clone()
    }

    /// 测试数据库连接
    pub async fn test_connection(&self) -> Result<bool> {
        match self.get_connection().await {
            Ok(conn) => {
                // 使用 query 方法而不是 execute 来执行 SELECT 语句
                match conn.query("SELECT 1 as test_value", ()).await {
                    Ok(mut rows) => {
                        // 验证查询是否返回了结果
                        match rows.next().await {
                            Ok(Some(_)) => Ok(true),
                            Ok(None) => {
                                warn!("Database connection test failed: no rows returned");
                                Ok(false)
                            }
                            Err(e) => {
                                warn!("Database connection test failed: {}", e);
                                Ok(false)
                            }
                        }
                    },
                    Err(e) => {
                        warn!("Database connection test failed: {}", e);
                        Ok(false)
                    }
                }
            },
            Err(e) => {
                warn!("Failed to get database connection: {}", e);
                Ok(false)
            }
        }
    }

    /// 获取数据库信息
    pub async fn get_database_info(&self) -> Result<DatabaseInfo> {
        let config = self.config.read().await;
        let conn = self.get_connection().await?;
        
        // 获取数据库大小
        let size = match conn.query("SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()", ()).await {
            Ok(mut rows) => {
                if let Some(row) = rows.next().await? {
                    row.get::<i64>(0).unwrap_or(0) as u64
                } else {
                    0
                }
            },
            Err(_) => 0,
        };

        // 获取表数量
        let table_count = match conn.query("SELECT COUNT(*) FROM sqlite_master WHERE type='table'", ()).await {
            Ok(mut rows) => {
                if let Some(row) = rows.next().await? {
                    row.get::<i64>(0).unwrap_or(0) as u32
                } else {
                    0
                }
            },
            Err(_) => 0,
        };

        // 获取笔记数量
        let note_count = match conn.query("SELECT COUNT(*) FROM tips", ()).await {
            Ok(mut rows) => {
                if let Some(row) = rows.next().await? {
                    row.get::<i64>(0).unwrap_or(0) as u64
                } else {
                    0
                }
            },
            Err(_) => 0,
        };

        // 获取分类数量
        let category_count = match conn.query("SELECT COUNT(*) FROM categories", ()).await {
            Ok(mut rows) => {
                if let Some(row) = rows.next().await? {
                    row.get::<i64>(0).unwrap_or(0) as u64
                } else {
                    0
                }
            },
            Err(_) => 0,
        };

        Ok(DatabaseInfo {
            mode: config.mode.clone(),
            size_bytes: size,
            table_count,
            connection_count: self.connection_pool.lock().await.len() as u32,
            is_connected: true,
            supports_sync: config.mode.supports_sync(),
            note_count,
            category_count,
        })
    }

    /// 关闭数据库管理器
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down database manager");

        // 如果支持同步，执行最后一次同步
        if self.config.read().await.mode.supports_sync() {
            let _ = self.sync().await; // 忽略错误
        }

        // 关闭数据库
        self.close_current_database().await?;

        info!("Database manager shutdown complete");
        Ok(())
    }
}

/// 数据库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    /// 数据库模式
    pub mode: DatabaseMode,
    /// 数据库大小（字节）
    pub size_bytes: u64,
    /// 表数量
    pub table_count: u32,
    /// 连接数量
    pub connection_count: u32,
    /// 是否已连接
    pub is_connected: bool,
    /// 是否支持同步
    pub supports_sync: bool,
    /// 笔记数量
    pub note_count: u64,
    /// 分类数量
    pub category_count: u64,
}

/// 便捷宏用于创建数据库配置
#[macro_export]
macro_rules! db_config {
    (local $path:expr) => {
        DatabaseConfig {
            mode: DatabaseMode::Local {
                path: $path.to_string(),
            },
            ..Default::default()
        }
    };
    (remote $url:expr, $token:expr) => {
        DatabaseConfig {
            mode: DatabaseMode::Remote {
                url: $url.to_string(),
                auth_token: $token.to_string(),
            },
            ..Default::default()
        }
    };
    (replica $local_path:expr, $remote_url:expr, $token:expr) => {
        DatabaseConfig {
            mode: DatabaseMode::EmbeddedReplica {
                local_path: $local_path.to_string(),
                remote_url: $remote_url.to_string(),
                auth_token: $token.to_string(),
                sync_interval: Some(Duration::from_secs(300)), // 5分钟
                read_your_writes: true,
            },
            ..Default::default()
        }
    };
    (in_memory) => {
        DatabaseConfig {
            mode: DatabaseMode::InMemory,
            ..Default::default()
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_database_mode_properties() {
        let local_mode = DatabaseMode::Local { path: "/tmp/test.db".to_string() };
        assert!(local_mode.is_local());
        assert!(!local_mode.is_remote());
        assert!(!local_mode.supports_sync());

        let replica_mode = DatabaseMode::EmbeddedReplica {
            local_path: "/tmp/replica.db".to_string(),
            remote_url: "https://example.com".to_string(),
            auth_token: "token".to_string(),
            sync_interval: None,
            read_your_writes: true,
        };
        assert!(replica_mode.is_embedded_replica());
        assert!(replica_mode.supports_sync());
    }

    #[tokio::test]
    async fn test_db_config_macros() {
        let local_config = db_config!(local "/tmp/test.db");
        assert!(local_config.mode.is_local());

        let replica_config = db_config!(replica "/tmp/replica.db", "https://example.com", "token");
        assert!(replica_config.mode.is_embedded_replica());

        let memory_config = db_config!(in_memory);
        assert_eq!(memory_config.mode, DatabaseMode::InMemory);
    }
} 
