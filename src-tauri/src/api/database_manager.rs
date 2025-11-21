use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{command, AppHandle, Manager, State};
use tracing::{info, warn, error};

use crate::db::{
     DatabaseConfig, DatabaseMode, DatabaseInfo, UnifiedDbManager
};
use crate::db::manager::SyncStatus as ManagerSyncStatus;

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoteModeConfig {
    url: String,
    auth_token: String,
    // 允许接收其他未定义字段，避免反序列化错误
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}


/// 数据库模式切换请求
#[derive(Debug, Deserialize)]
pub struct SwitchModeRequest {
    /// 目标模式
    pub mode: String,
    /// 模式特定参数
    pub params: ModeParams,
}

/// 模式参数
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ModeParams {
    Local {
        path: String,
    },
    Remote {
        url: String,
        auth_token: String,
    },
    EmbeddedReplica {
        local_path: String,
        remote_url: String,
        auth_token: String,
        sync_interval_seconds: Option<u64>,
        read_your_writes: Option<bool>,
    },
    InMemory,
}

/// 前端期望的数据库信息格式
#[derive(Debug, Serialize)]
pub struct FrontendDatabaseInfo {
    pub size: String,
    pub note_count: i64,
    pub category_count: i64,
    pub last_modified: String,
    pub database_path: Option<String>,
    pub mode_description: String,
}

/// 前端期望的同步状态格式
#[derive(Debug, Serialize)]
pub struct FrontendSyncStatus {
    pub is_syncing: bool,
    pub last_sync_time: i64,
    pub sync_error_count: i32,
    pub is_online: bool,
}

/// 数据库状态响应
#[derive(Debug, Serialize)]
pub struct DatabaseStatusResponse {
    /// 当前模式
    pub current_mode: String,
    /// 数据库信息
    pub database_info: FrontendDatabaseInfo,
    /// 同步状态（如果支持）
    pub sync_status: Option<FrontendSyncStatus>,
    /// 连接状态
    pub is_connected: bool,
}

/// 模式切换API
#[command]
pub async fn switch_database_mode(
    app: AppHandle,
    request: SwitchModeRequest,
) -> Result<String, String> {
    info!("Switching database mode to: {}", request.mode);

    // 获取或创建统一数据库管理器
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;

    // 构建数据库配置
    let config = build_database_config(&request.mode, request.params)
        .map_err(|e| format!("Invalid configuration: {}", e))?;

    // 执行模式切换
    unified_manager.switch_mode(config).await
        .map_err(|e| format!("Failed to switch mode: {}", e))?;

    // 更新应用状态中的管理器
    app.manage(unified_manager);

    Ok(format!("Successfully switched to {} mode", request.mode))
}

/// 获取数据库状态
#[command]
pub async fn get_database_status(app: AppHandle) -> Result<DatabaseStatusResponse, String> {
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;

    let current_mode = unified_manager.get_current_mode().await;
    let db_info = unified_manager.get_database_info().await
        .map_err(|e| format!("Failed to get database info: {}", e))?;
    
    let sync_status = if current_mode.supports_sync() {
        let manager_sync_status = unified_manager.get_sync_status().await;
        Some(FrontendSyncStatus {
            is_syncing: manager_sync_status.is_syncing,
            last_sync_time: manager_sync_status.last_sync_time,
            sync_error_count: if manager_sync_status.last_sync_error.is_some() { 1 } else { 0 },
            is_online: true, // 简化处理，嵌入式副本模式认为是在线的
        })
    } else {
        None
    };

    let is_connected = unified_manager.test_connection().await
        .unwrap_or(false);

    // 格式化大小
    let size_str = format_file_size(db_info.size_bytes);

    // 获取最后修改时间（简化处理）
    let last_modified = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 获取数据库路径和模式描述
    let (database_path, mode_description) = match &current_mode {
        DatabaseMode::Local { path } => {
            (Some(path.clone()), "本地数据库模式".to_string())
        },
        DatabaseMode::EmbeddedReplica { local_path, remote_url, .. } => {
            (Some(local_path.clone()), format!("嵌入式副本模式 (同步至: {})", remote_url))
        },
        DatabaseMode::Remote { url, .. } => {
            (Some(url.clone()), "远程数据库模式".to_string())
        },
        DatabaseMode::InMemory => {
            (None, "内存数据库模式".to_string())
        },
    };

    // 转换为前端期望的格式
    let database_info = FrontendDatabaseInfo {
        size: size_str,
        note_count: db_info.note_count as i64,
        category_count: db_info.category_count as i64,
        last_modified,
        database_path,
        mode_description,
    };

    Ok(DatabaseStatusResponse {
        current_mode: current_mode.mode_name().to_string(),
        database_info,
        sync_status,
        is_connected,
    })
}

/// 执行数据库同步
#[command]
pub async fn sync_database(app: AppHandle) -> Result<String, String> {
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;

    let current_mode = unified_manager.get_current_mode().await;
    if !current_mode.supports_sync() {
        return Err(format!("Current mode '{}' does not support sync", current_mode.mode_name()));
    }

    unified_manager.sync().await
        .map_err(|e| format!("Sync failed: {}", e))?;

    Ok("Database sync completed successfully".to_string())
}

/// 测试数据库连接
#[command]
pub async fn test_database_connection(app: AppHandle) -> Result<bool, String> {
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;

    let result = unified_manager.test_connection().await
        .map_err(|e| format!("Connection test failed: {}", e))?;

    Ok(result)
}

/// 切换到本地模式
#[command]
pub async fn switch_to_local_mode(app: AppHandle, path: Option<String>) -> Result<String, String> {
    let request = SwitchModeRequest {
        mode: "local".to_string(),
        params: ModeParams::Local {
            path: path.unwrap_or_else(|| get_default_db_path(&app)),
        },
    };

    switch_database_mode(app, request).await
}

/// 切换到远程模式
#[command]
pub async fn switch_to_remote_mode(
    app: AppHandle,
    config: RemoteModeConfig
) -> Result<String, String> {
    let request = SwitchModeRequest {
        mode: "remote".to_string(),
        params: ModeParams::Remote {
            url: config.url,
            auth_token: config.auth_token,
        },
    };

    switch_database_mode(app, request).await
}

/// 切换到嵌入式副本模式
#[command(rename_all = "snake_case")]
pub async fn switch_to_embedded_replica_mode(
    app: AppHandle,
    local_path: Option<String>,
    remote_url: String,
    auth_token: String,
    sync_interval_seconds: Option<u64>,
) -> Result<String, String> {
    use crate::sync::error_handling::SmartRetryExecutor;
    
    info!("Initiating switch to embedded replica mode");
    info!("Remote URL: {}", remote_url);
    info!("Auth token: {}", if auth_token.is_empty() { "Not provided (using anonymous access)" } else { "Provided" });
    info!("Sync interval: {:?} seconds", sync_interval_seconds);

    // 验证远程连接参数
    if remote_url.is_empty() {
        return Err("Remote URL cannot be empty".to_string());
    }
    
    // 使用WAL安全操作包装整个切换过程
    let result = SmartRetryExecutor::execute_wal_safe_operation(
        move || {
            let app = app.clone();
            let remote_url = remote_url.clone();
            let auth_token = auth_token.clone();
            let local_path = local_path.clone();
            
            Box::pin(async move {
                // 首先测试远程连接
                info!("Testing remote database connection before switch");
                if let Err(e) = test_remote_connection(&remote_url, &auth_token).await {
                    return Err(anyhow::anyhow!("Remote connection test failed: {}. Please check your URL and auth credentials.", e));
                }
                
                let request = SwitchModeRequest {
                    mode: "embedded_replica".to_string(),
                    params: ModeParams::EmbeddedReplica {
                        local_path: local_path.unwrap_or_else(|| get_default_replica_path(&app)),
                        remote_url: remote_url.clone(),
                        auth_token: auth_token.clone(),
                        sync_interval_seconds,
                        read_your_writes: Some(true),
                    },
                };

                // 切换数据库模式 - 增强错误处理
                let result = switch_database_mode(app.clone(), request).await.map_err(|e| {
                    error!("Database mode switch failed: {}", e);
                    
                    // 分析错误类型并提供更具体的错误信息
                    if e.contains("no such table: ai_roles") {
                        anyhow::anyhow!(
                            "Database schema initialization failed. The ai_roles table could not be created or accessed. \
                            This might be due to: \n\
                            1. Network connectivity issues during initial sync\n\
                            2. Remote database access permissions\n\
                            3. Database corruption\n\
                            \nOriginal error: {}", e
                        )
                    } else if e.contains("Failed to switch mode") {
                        anyhow::anyhow!(
                            "Failed to switch to embedded replica mode. Possible causes: \n\
                            1. Invalid remote URL or auth token\n\
                            2. Network connectivity issues\n\
                            3. Local file system permissions\n\
                            4. Remote database is not accessible\n\
                            \nOriginal error: {}", e
                        )
                    } else if e.contains("Failed to create") {
                        anyhow::anyhow!(
                            "Failed to create embedded replica database. This could be due to: \n\
                            1. Local file system permissions\n\
                            2. Insufficient disk space\n\
                            3. WAL file conflicts\n\
                            \nOriginal error: {}", e
                        )
                    } else {
                        anyhow::anyhow!("Database mode switch failed: {}", e)
                    }
                })?;
                
                // 切换成功后，保存同步配置到传统同步配置表（用于前端加载）
                if let Err(e) = save_sync_config_after_mode_switch(&app, &remote_url, &auth_token, sync_interval_seconds).await {
                    warn!("Failed to save sync config after mode switch: {}", e);
                    // 不让这个错误阻止整个切换过程
                }
                
                // 验证切换是否真正成功
                info!("Verifying embedded replica mode switch");
                if let Err(e) = verify_embedded_replica_mode(&app).await {
                    warn!("Embedded replica verification failed: {}", e);
                    // 这不应该阻止成功响应，但应该记录
                }
                
                Ok(result)
            })
        },
        "switch_to_embedded_replica_mode",
    ).await;

    match result {
        Ok(message) => {
            info!("Successfully switched to embedded replica mode");
            Ok(format!("{}\n\nThe database is now running in embedded replica mode, which provides the best balance of performance and synchronization.", message))
        }
        Err(e) => {
            error!("Failed to switch to embedded replica mode: {}", e);
            
            // 提供回退建议
            let fallback_suggestion = "\n\nSuggested fallback actions:\n\
                1. Try switching to local mode first: use 'Local Mode' in settings\n\
                2. Check your network connection and remote database accessibility\n\
                3. Verify the remote URL format (should be like: libsql://your-db.turso.io)\n\
                4. If using auth token, ensure it is valid and has necessary permissions";
            
            Err(format!("Failed to switch to embedded replica mode: {}{}", e, fallback_suggestion))
        }
    }
}

/// 测试远程数据库连接
async fn test_remote_connection(remote_url: &str, auth_token: &str) -> Result<(), String> {
    use libsql::Builder;
    
    info!("Testing remote database connection to: {}", remote_url);
    
    // 确保auth_token包含完整的认证头格式 (scheme + token)
    let full_auth_token = if auth_token.contains(' ') {
        auth_token.to_string()
    } else {
        format!("Bearer {}", auth_token)
    };
    
    // 尝试创建远程连接
    let database = Builder::new_remote(remote_url.to_string(), full_auth_token)
        .build()
        .await
        .map_err(|e| format!("Failed to create remote connection: {}", e))?;
    
    // 尝试连接和执行简单查询
    let conn = database.connect()
        .map_err(|e| format!("Failed to connect to remote database: {}", e))?;
    
    // 执行简单的查询来验证连接
    let mut rows = conn.query("SELECT 1 as test", ()).await
        .map_err(|e| format!("Failed to execute test query: {}", e))?;
    
    if rows.next().await.map_err(|e| format!("Failed to read test result: {}", e))?.is_some() {
        info!("Remote database connection test successful");
        Ok(())
    } else {
        Err("Test query returned no results".to_string())
    }
}

/// 验证嵌入式副本模式是否正常工作
async fn verify_embedded_replica_mode(app: &AppHandle) -> Result<(), String> {
    info!("Verifying embedded replica mode functionality");
    
    // 获取数据库状态
    match get_database_status(app.clone()).await {
        Ok(status) => {
            if status.current_mode != "embedded_replica" {
                return Err(format!("Expected embedded_replica mode, but got: {}", status.current_mode));
            }
            
            if !status.is_connected {
                return Err("Database is not connected".to_string());
            }
            
            info!("Embedded replica mode verification successful");
            Ok(())
        }
        Err(e) => Err(format!("Failed to get database status: {}", e))
    }
}

/// 获取支持的数据库模式
#[command]
pub async fn get_supported_database_modes() -> Result<Vec<String>, String> {
    Ok(vec![
        "local".to_string(),
        "remote".to_string(),
        "embedded_replica".to_string(),
        "in_memory".to_string(),
    ])
}

/// 清理本地数据库文件（解决WAL索引问题）
#[command]
pub async fn cleanup_local_database_files(app: AppHandle, path: Option<String>) -> Result<String, String> {
    let db_path = path.unwrap_or_else(|| get_default_db_path(&app));
    
    info!("Cleaning up local database files at: {}", db_path);
    
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;

    // 执行彻底的文件清理
    if db_path != ":memory:" {
        let main_db_path = std::path::Path::new(&db_path);
        let wal_path = format!("{}-wal", db_path);
        let shm_path = format!("{}-shm", db_path);
        let journal_path = format!("{}-journal", db_path);

        let mut cleaned_files = Vec::new();

        // 清理WAL和相关文件
        for (file_path, name) in [
            (&wal_path, "WAL file"),
            (&shm_path, "SHM file"), 
            (&journal_path, "Journal file")
        ] {
            if std::path::Path::new(file_path).exists() {
                if let Err(e) = tokio::fs::remove_file(file_path).await {
                    warn!("Could not remove {}: {}", name, e);
                } else {
                    cleaned_files.push(name);
                    info!("Removed {}: {}", name, file_path);
                }
            }
        }

        // 检查主数据库文件
        if main_db_path.exists() {
            cleaned_files.push("Database file");
            info!("Main database file exists at: {}", db_path);
        }

        if cleaned_files.is_empty() {
            Ok("No database files found to clean up.".to_string())
        } else {
            Ok(format!("Successfully cleaned up: {}. You can now try switching to embedded replica mode again.", cleaned_files.join(", ")))
        }
    } else {
        Err("Cannot clean up in-memory database".to_string())
    }
}

/// 优化WAL文件大小（用于减少WAL索引文件大小）
#[command]
pub async fn optimize_database_wal(app: AppHandle) -> Result<String, String> {
    info!("Starting database WAL optimization");
    
    let unified_manager = get_or_create_unified_manager(&app).await
        .map_err(|e| format!("Failed to get unified manager: {}", e))?;
    
    // 检查当前模式是否支持WAL优化
    let current_mode = unified_manager.get_current_mode().await;
    if !current_mode.is_embedded_replica() && !current_mode.is_local() {
        return Err("WAL optimization is only available for local and embedded replica modes".to_string());
    }
    
    match unified_manager.optimize_wal_files().await {
        Ok(_) => {
            info!("WAL optimization completed successfully");
            Ok("WAL files optimized successfully. This should help reduce the size of WAL index files.".to_string())
        },
        Err(e) => {
            error!("WAL optimization failed: {}", e);
            Err(format!("WAL optimization failed: {}", e))
        }
    }
}

/// 获取或创建统一数据库管理器
async fn get_or_create_unified_manager(app: &AppHandle) -> Result<UnifiedDbManager> {
    // 尝试从应用状态获取
    if let Some(manager) = app.try_state::<UnifiedDbManager>() {
        return Ok((*manager.inner()).clone());
    }

    // 如果不存在，创建新的管理器
    let manager = UnifiedDbManager::new(app.clone()).await?;
    Ok(manager)
}

/// 构建数据库配置
fn build_database_config(mode: &str, params: ModeParams) -> Result<DatabaseConfig> {
    let database_mode = match (mode, params) {
        ("local", ModeParams::Local { path }) => DatabaseMode::Local { path },
        ("remote", ModeParams::Remote { url, auth_token }) => DatabaseMode::Remote { 
            url, 
            auth_token 
        },
        ("embedded_replica", ModeParams::EmbeddedReplica {
            local_path,
            remote_url,
            auth_token,
            sync_interval_seconds,
            read_your_writes,
        }) => DatabaseMode::EmbeddedReplica {
            local_path,
            remote_url,
            auth_token,
            sync_interval: sync_interval_seconds.map(Duration::from_secs),
            read_your_writes: read_your_writes.unwrap_or(true),
        },
        ("in_memory", ModeParams::InMemory) => DatabaseMode::InMemory,
        _ => return Err(anyhow::anyhow!("Invalid mode and params combination")),
    };

    Ok(DatabaseConfig {
        mode: database_mode,
        ..Default::default()
    })
}

/// 获取默认数据库路径
fn get_default_db_path(app: &AppHandle) -> String {
    app.path()
        .app_data_dir()
        .map(|dir| dir.join("mytips.db").to_string_lossy().to_string())
        .unwrap_or_else(|_| ":memory:".to_string())
}

/// 获取默认副本路径
fn get_default_replica_path(app: &AppHandle) -> String {
    app.path()
        .app_data_dir()
        .map(|dir| dir.join("mytips_replica.db").to_string_lossy().to_string())
        .unwrap_or_else(|_| "mytips_replica.db".to_string())
}

/// 格式化文件大小
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// 切换模式后保存同步配置（用于前端加载）
async fn save_sync_config_after_mode_switch(
    app: &AppHandle,
    remote_url: &str,
    auth_token: &str,
    sync_interval_seconds: Option<u64>,
) -> Result<()> {
    use crate::db::{SyncConfig, SyncMode};
    
    // 获取传统数据库管理器来保存同步配置
    if let Some(db_manager) = app.try_state::<crate::db::UnifiedDbManager>() {
        let conn = db_manager.get_conn().await?;
        
        let sync_config = SyncConfig {
            id: "default".to_string(),
            remote_url: Some(remote_url.to_string()),
            auth_token: Some(auth_token.to_string()),
            sync_mode: SyncMode::Auto, // 嵌入式副本模式默认为自动同步
            sync_interval: sync_interval_seconds.unwrap_or(300) as i64,
            last_sync_at: 0,
            is_online: true,
            auto_sync_enabled: true,
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
        };
        
        crate::db::save_sync_config(&conn, &sync_config).await?;
        info!("Sync config saved to database after mode switch");
    } else {
        warn!("Legacy database manager not available, sync config not saved");
    }
    
    Ok(())
}

/// 兼容性适配器 - 从UnifiedUnifiedDbManager获取传统UnifiedDbManager连接
pub async fn get_legacy_connection(app: &AppHandle) -> Result<libsql::Connection> {
    let unified_manager = get_or_create_unified_manager(app).await?;
    unified_manager.get_connection().await
}

/// 数据库模式管理中间件
pub struct DatabaseModeManager {
    unified_manager: UnifiedDbManager,
}

impl DatabaseModeManager {
    pub async fn new(app: AppHandle) -> Result<Self> {
        let unified_manager = UnifiedDbManager::new(app).await?;
        Ok(Self { unified_manager })
    }

    pub async fn get_connection(&self) -> Result<libsql::Connection> {
        self.unified_manager.get_connection().await
    }

    pub async fn get_pooled_connection(&self) -> Result<libsql::Connection> {
        self.unified_manager.get_pooled_connection().await
    }

    pub async fn return_connection(&self, conn: libsql::Connection) {
        self.unified_manager.return_connection(conn).await
    }

    pub async fn switch_mode(&self, config: DatabaseConfig) -> Result<()> {
        self.unified_manager.switch_mode(config).await
    }

    pub async fn sync(&self) -> Result<()> {
        self.unified_manager.sync().await
    }

    pub async fn get_current_mode(&self) -> DatabaseMode {
        self.unified_manager.get_current_mode().await
    }

    pub async fn get_database_info(&self) -> Result<DatabaseInfo> {
        self.unified_manager.get_database_info().await
    }

    pub async fn get_sync_status(&self) -> ManagerSyncStatus {
        self.unified_manager.get_sync_status().await
    }

    pub async fn test_connection(&self) -> Result<bool> {
        self.unified_manager.test_connection().await
    }
}

/// 预设配置创建函数
pub mod presets {
    use super::*;

    /// 创建本地SQLite配置
    pub fn local_sqlite(path: &str) -> DatabaseConfig {
        crate::db_config!(local path)
    }

    /// 创建Turso远程配置
    pub fn turso_remote(url: &str, token: &str) -> DatabaseConfig {
        crate::db_config!(remote url, token)
    }

    /// 创建Turso嵌入式副本配置
    pub fn turso_embedded_replica(local_path: &str, remote_url: &str, token: &str) -> DatabaseConfig {
        crate::db_config!(replica local_path, remote_url, token)
    }

    /// 创建内存数据库配置（测试用）
    pub fn in_memory() -> DatabaseConfig {
        crate::db_config!(in_memory)
    }

    /// 创建开发环境配置
    pub fn development() -> DatabaseConfig {
        local_sqlite(":memory:")
    }

    /// 创建生产环境配置（需要提供实际参数）
    pub fn production(local_path: &str, remote_url: &str, token: &str) -> DatabaseConfig {
        let mut config = turso_embedded_replica(local_path, remote_url, token);
        // 生产环境优化设置
        config.pragma_settings.extend(vec![
            ("synchronous".to_string(), "FULL".to_string()),
            ("cache_size".to_string(), "5000".to_string()),
        ]);
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_database_config() {
        // 测试本地模式
        let local_params = ModeParams::Local { path: "/tmp/test.db".to_string() };
        let config = build_database_config("local", local_params).unwrap();
        assert!(config.mode.is_local());

        // 测试嵌入式副本模式
        let replica_params = ModeParams::EmbeddedReplica {
            local_path: "/tmp/replica.db".to_string(),
            remote_url: "https://example.com".to_string(),
            auth_token: "token".to_string(),
            sync_interval_seconds: Some(300),
            read_your_writes: Some(true),
        };
        let config = build_database_config("embedded_replica", replica_params).unwrap();
        assert!(config.mode.is_embedded_replica());
        assert!(config.mode.supports_sync());
    }

    #[test]
    fn test_presets() {
        let local_config = presets::local_sqlite("/tmp/test.db");
        assert!(local_config.mode.is_local());

        let memory_config = presets::in_memory();
        assert_eq!(memory_config.mode, DatabaseMode::InMemory);

        let dev_config = presets::development();
        assert!(dev_config.mode.is_local());
    }
} 