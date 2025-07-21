use anyhow::{anyhow, Result};
use libsql::{Builder, Connection, Database};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock};
use tracing::{info, warn, error, debug};
use serde::{Deserialize, Serialize};

/// 基于官方最佳实践的libSQL同步管理器
/// 参考: https://github.com/tursodatabase/libsql/blob/main/libsql/examples/remote_sync.rs
pub struct LibSqlSyncManager {
    /// 本地数据库路径
    local_path: PathBuf,
    /// 远程数据库URL
    remote_url: String,
    /// 认证令牌
    auth_token: Option<String>,
    /// 本地数据库实例（用于嵌入式副本）
    local_db: Arc<RwLock<Option<Database>>>,
    /// 远程数据库实例（用于直接远程连接）
    remote_db: Arc<RwLock<Option<Database>>>,
    /// 同步配置
    sync_config: LibSqlSyncConfig,
    /// 同步状态
    sync_state: Arc<Mutex<SyncState>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibSqlSyncConfig {
    /// 同步间隔（秒）
    pub sync_interval_secs: u64,
    /// 是否启用自动同步
    pub auto_sync_enabled: bool,
    /// 是否启用读写分离
    pub read_your_writes: bool,
    /// 同步超时时间（秒）
    pub sync_timeout_secs: u64,
    /// 连接池大小
    pub connection_pool_size: usize,
}

impl Default for LibSqlSyncConfig {
    fn default() -> Self {
        Self {
            sync_interval_secs: 300, // 5分钟
            auto_sync_enabled: true,
            read_your_writes: true,
            sync_timeout_secs: 30,
            connection_pool_size: 5,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyncState {
    /// 是否正在同步
    pub is_syncing: bool,
    /// 最后同步时间
    pub last_sync_time: i64,
    /// 同步错误计数
    pub sync_error_count: u32,
    /// 是否在线
    pub is_online: bool,
}

impl Default for SyncState {
    fn default() -> Self {
        Self {
            is_syncing: false,
            last_sync_time: 0,
            sync_error_count: 0,
            is_online: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub message: String,
    pub sync_time: i64,
    pub records_synced: u64,
}

impl LibSqlSyncManager {
    /// 创建新的同步管理器
    pub async fn new(
        local_path: PathBuf,
        remote_url: String,
        auth_token: Option<String>,
        config: LibSqlSyncConfig,
    ) -> Result<Self> {
        info!("Creating LibSQL sync manager with remote URL: {}", remote_url);
        
        let manager = Self {
            local_path,
            remote_url,
            auth_token,
            local_db: Arc::new(RwLock::new(None)),
            remote_db: Arc::new(RwLock::new(None)),
            sync_config: config,
            sync_state: Arc::new(Mutex::new(SyncState::default())),
        };

        Ok(manager)
    }

    /// 初始化数据库连接
    /// 基于官方示例: https://docs.turso.tech/sdk/rust/quickstart
    pub async fn initialize(&self) -> Result<()> {
        info!("Initializing LibSQL databases");

        // 1. 首先创建本地数据库（嵌入式副本）
        let local_db = self.create_embedded_replica().await?;
        *self.local_db.write().await = Some(local_db);

        // 2. 创建远程数据库连接（用于写操作）
        let remote_db = self.create_remote_connection().await?;
        *self.remote_db.write().await = Some(remote_db);

        // 3. 更新状态
        {
            let mut state = self.sync_state.lock().await;
            state.is_online = true;
        }

        // 4. 执行初始同步
        self.perform_initial_sync().await?;

        // 5. 启动自动同步（如果启用）
        if self.sync_config.auto_sync_enabled {
            self.start_auto_sync().await?;
        }

        info!("LibSQL sync manager initialized successfully");
        Ok(())
    }

    /// 创建嵌入式副本
    /// 参考: https://github.com/tursodatabase/libsql/blob/main/libsql/examples/replica.rs
    async fn create_embedded_replica(&self) -> Result<Database> {
        info!("Creating embedded replica at: {:?}", self.local_path);

        // 确保本地路径的父目录存在
        if let Some(parent) = self.local_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let local_path_str = self.local_path.to_string_lossy().to_string();

        let mut builder = Builder::new_remote_replica(
            local_path_str,
            self.remote_url.clone(),
            self.auth_token.clone().unwrap_or_default(),
        );

        // 配置同步间隔
        if self.sync_config.auto_sync_enabled {
            builder = builder.sync_interval(Duration::from_secs(self.sync_config.sync_interval_secs));
        }

        // 配置读写策略
        builder = builder.read_your_writes(self.sync_config.read_your_writes);

        let db = builder.build().await.map_err(|e| {
            error!("Failed to create embedded replica: {}", e);
            anyhow!("Failed to create embedded replica: {}", e)
        })?;

        info!("Embedded replica created successfully");
        Ok(db)
    }

    /// 创建远程连接
    /// 参考: https://github.com/tursodatabase/libsql/blob/main/libsql/examples/remote_sync.rs
    async fn create_remote_connection(&self) -> Result<Database> {
        info!("Creating remote database connection");

        let db = Builder::new_remote(
            self.remote_url.clone(),
            self.auth_token.clone().unwrap_or_default(),
        )
        .build()
        .await
        .map_err(|e| {
            error!("Failed to create remote connection: {}", e);
            anyhow!("Failed to create remote connection: {}", e)
        })?;

        info!("Remote database connection created successfully");
        Ok(db)
    }

    /// 执行初始同步
    async fn perform_initial_sync(&self) -> Result<()> {
        info!("Performing initial sync");

        let local_db_guard = self.local_db.read().await;
        let local_db = local_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Local database not initialized"))?;

        // 执行同步
        local_db.sync().await.map_err(|e| {
            error!("Initial sync failed: {}", e);
            anyhow!("Initial sync failed: {}", e)
        })?;

        // 更新状态
        {
            let mut state = self.sync_state.lock().await;
            state.last_sync_time = chrono::Utc::now().timestamp_millis();
            state.sync_error_count = 0;
        }

        info!("Initial sync completed successfully");
        Ok(())
    }

    /// 启动自动同步
    async fn start_auto_sync(&self) -> Result<()> {
        info!("Starting auto sync with interval: {} seconds", self.sync_config.sync_interval_secs);

        let local_db = self.local_db.clone();
        let sync_state = self.sync_state.clone();
        let interval = Duration::from_secs(self.sync_config.sync_interval_secs);

        tokio::spawn(async move {
            let mut sync_interval = tokio::time::interval(interval);
            sync_interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                sync_interval.tick().await;

                // 检查是否正在同步
                {
                    let state = sync_state.lock().await;
                    if state.is_syncing {
                        debug!("Sync already in progress, skipping");
                        continue;
                    }
                }

                // 执行同步
                if let Some(db) = local_db.read().await.as_ref() {
                    let mut state = sync_state.lock().await;
                    state.is_syncing = true;
                    drop(state);

                    match db.sync().await {
                        Ok(_) => {
                            debug!("Auto sync completed successfully");
                            let mut state = sync_state.lock().await;
                            state.last_sync_time = chrono::Utc::now().timestamp_millis();
                            state.sync_error_count = 0;
                            state.is_syncing = false;
                        }
                        Err(e) => {
                            warn!("Auto sync failed: {}", e);
                            let mut state = sync_state.lock().await;
                            state.sync_error_count += 1;
                            state.is_syncing = false;
                        }
                    }
                }
            }
        });

        info!("Auto sync started");
        Ok(())
    }

    /// 执行手动同步
    pub async fn manual_sync(&self) -> Result<SyncResult> {
        info!("Starting manual sync");

        // 检查是否正在同步
        {
            let state = self.sync_state.lock().await;
            if state.is_syncing {
                return Ok(SyncResult {
                    success: false,
                    message: "Sync already in progress".to_string(),
                    sync_time: state.last_sync_time,
                    records_synced: 0,
                });
            }
        }

        // 设置同步状态
        {
            let mut state = self.sync_state.lock().await;
            state.is_syncing = true;
        }

        let sync_start = std::time::Instant::now();
        let result = {
            let local_db_guard = self.local_db.read().await;
            let local_db = local_db_guard.as_ref()
                .ok_or_else(|| anyhow!("Local database not initialized"))?;

            // 执行同步操作
            local_db.sync().await
        };

        let sync_duration = sync_start.elapsed();
        let sync_time = chrono::Utc::now().timestamp_millis();

        // 更新状态
        {
            let mut state = self.sync_state.lock().await;
            state.is_syncing = false;
            
            match &result {
                Ok(_) => {
                    state.last_sync_time = sync_time;
                    state.sync_error_count = 0;
                    info!("Manual sync completed in {:?}", sync_duration);
                }
                Err(e) => {
                    state.sync_error_count += 1;
                    error!("Manual sync failed: {}", e);
                }
            }
        }

        match result {
            Ok(_) => Ok(SyncResult {
                success: true,
                message: format!("Sync completed in {:?}", sync_duration),
                sync_time,
                records_synced: 0, // libSQL不直接提供这个信息
            }),
            Err(e) => Ok(SyncResult {
                success: false,
                message: format!("Sync failed: {}", e),
                sync_time,
                records_synced: 0,
            }),
        }
    }

    /// 获取本地连接（用于读操作）
    pub async fn get_local_connection(&self) -> Result<Connection> {
        let local_db_guard = self.local_db.read().await;
        let local_db = local_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Local database not initialized"))?;

        local_db.connect().map_err(|e| {
            error!("Failed to get local connection: {}", e);
            anyhow!("Failed to get local connection: {}", e)
        })
    }

    /// 获取远程连接（用于写操作）
    pub async fn get_remote_connection(&self) -> Result<Connection> {
        let remote_db_guard = self.remote_db.read().await;
        let remote_db = remote_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not initialized"))?;

        remote_db.connect().map_err(|e| {
            error!("Failed to get remote connection: {}", e);
            anyhow!("Failed to get remote connection: {}", e)
        })
    }

    /// 获取混合连接（自动选择本地或远程）
    pub async fn get_connection(&self) -> Result<Connection> {
        // 对于嵌入式副本，优先使用本地连接
        self.get_local_connection().await
    }

    /// 获取同步状态
    pub async fn get_sync_status(&self) -> SyncState {
        self.sync_state.lock().await.clone()
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<bool> {
        info!("Testing database connections");

        // 测试本地连接
        let local_result = self.get_local_connection().await;
        if let Err(e) = local_result {
            warn!("Local connection test failed: {}", e);
            return Ok(false);
        }

        // 测试远程连接
        let remote_result = self.get_remote_connection().await;
        if let Err(e) = remote_result {
            warn!("Remote connection test failed: {}", e);
            return Ok(false);
        }

        info!("All connection tests passed");
        Ok(true)
    }

    /// 停止同步管理器
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down LibSQL sync manager");

        // 执行最后一次同步
        if self.sync_config.auto_sync_enabled {
            let _ = self.manual_sync().await;
        }

        // 清理连接
        *self.local_db.write().await = None;
        *self.remote_db.write().await = None;

        info!("LibSQL sync manager shutdown complete");
        Ok(())
    }
}

/// 安全的数据库操作助手
pub struct SafeDbOperations;

impl SafeDbOperations {
    /// 安全执行SQL语句
    pub async fn execute_safe(
        conn: &Connection,
        sql: &str,
        params: impl Into<libsql::params::Params>,
    ) -> Result<u64> {
        conn.execute(sql, params.into()).await.map_err(|e| {
            error!("SQL execution failed: {} - Error: {}", sql, e);
            anyhow!("SQL execution failed: {}", e)
        })
    }

    /// 安全查询数据
    pub async fn query_safe(
        conn: &Connection,
        sql: &str,
        params: impl Into<libsql::params::Params>,
    ) -> Result<libsql::Rows> {
        conn.query(sql, params.into()).await.map_err(|e| {
            error!("SQL query failed: {} - Error: {}", sql, e);
            anyhow!("SQL query failed: {}", e)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_sync_manager_creation() {
        let temp_dir = tempdir().unwrap();
        let local_path = temp_dir.path().join("test.db");
        let remote_url = "libsql://test.turso.io".to_string();
        let auth_token = Some("test_token".to_string());
        let config = LibSqlSyncConfig::default();

        let manager = LibSqlSyncManager::new(
            local_path,
            remote_url,
            auth_token,
            config,
        ).await;

        assert!(manager.is_ok());
    }
} 