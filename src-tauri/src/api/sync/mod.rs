use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{command, State};

use crate::db::{DbManager, SyncMode, SyncConfig, ConflictResolutionStrategy};
use crate::sync::{SyncManager, SyncStats, ConflictData};

// 同步配置请求
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncConfigRequest {
    pub remote_url: String,
    pub auth_token: Option<String>,
    pub sync_mode: String, // "AUTO", "MANUAL", "OFFLINE"
    pub sync_interval: i64,
    pub auto_sync_enabled: bool,
}

// 同步状态响应
#[derive(Debug, Serialize, Deserialize)]
pub struct SyncStatusResponse {
    pub is_enabled: bool,
    pub is_online: bool,
    pub sync_mode: String,
    pub last_sync_time: i64,
    pub stats: Option<SyncStats>,
}

// 冲突解决请求
#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictResolutionRequest {
    pub record_id: String,
    pub table_name: String,
    pub strategy: String, // "LOCAL_WINS", "REMOTE_WINS", "MERGE", "USER_CHOICE"
}

// 数据迁移状态
#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationStatus {
    pub is_migrating: bool,
    pub progress: f64,
    pub current_step: String,
    pub total_steps: usize,
    pub completed_steps: usize,
}

/// 保存同步配置（不需要同步管理器）
#[command]
pub async fn save_sync_config_only(
    db_manager: State<'_, DbManager>,
    config: SyncConfigRequest,
) -> Result<(), String> {
    use chrono::Utc;
    
    // 确保数据库有同步表结构
    let conn = db_manager.get_conn().await;
    
    // 确保同步配置表存在（如果数据库刚创建）
    // 这些表在init_schema中已经创建，这里只是确保存在
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS sync_config (
            id TEXT PRIMARY KEY DEFAULT 'default',
            remote_url TEXT,
            auth_token TEXT,
            sync_mode TEXT DEFAULT 'OFFLINE',
            sync_interval INTEGER DEFAULT 300,
            last_sync_at INTEGER DEFAULT 0,
            is_online BOOLEAN DEFAULT FALSE,
            auto_sync_enabled BOOLEAN DEFAULT TRUE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        ()
    ).await;
    
    // 解析同步模式
    let sync_mode = match config.sync_mode.to_uppercase().as_str() {
        "AUTO" => SyncMode::Auto,
        "MANUAL" => SyncMode::Manual,
        "OFFLINE" => SyncMode::Offline,
        _ => return Err("Invalid sync mode".to_string()),
    };
    
    // 创建或更新同步配置
    let now = Utc::now().timestamp_millis();
    let sync_config = SyncConfig {
        id: "default".to_string(),
        remote_url: if sync_mode == SyncMode::Offline { None } else { Some(config.remote_url.clone()) },
        auth_token: if sync_mode == SyncMode::Offline { None } else { config.auth_token.clone() },
        sync_mode: sync_mode.clone(),
        sync_interval: config.sync_interval,
        last_sync_at: 0,
        is_online: sync_mode != SyncMode::Offline,
        auto_sync_enabled: config.auto_sync_enabled,
        created_at: now,
        updated_at: now,
    };
    
    // 保存配置到数据库
    crate::db::save_sync_config(&conn, &sync_config)
        .await
        .map_err(|e| format!("Failed to save sync config: {}", e))?;
    
    Ok(())
}

/// 配置远程数据库（需要同步管理器）
#[command]
pub async fn configure_remote_database(
    db_manager: State<'_, DbManager>,
    config: SyncConfigRequest,
) -> Result<(), String> {
    // 首先保存配置
    save_sync_config_only(db_manager.clone(), config.clone()).await?;
    
    // 如果有同步管理器，尝试配置远程数据库
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        let sync_mode = match config.sync_mode.to_uppercase().as_str() {
            "AUTO" => SyncMode::Auto,
            "MANUAL" => SyncMode::Manual,
            "OFFLINE" => SyncMode::Offline,
            _ => return Err("Invalid sync mode".to_string()),
        };
        
        // 配置远程数据库
        sync_manager.configure_remote_db(config.remote_url, config.auth_token)
            .await
            .map_err(|e| format!("Failed to configure remote database: {}", e))?;
        
        // 设置同步模式
        sync_manager.set_sync_mode(sync_mode)
            .await
            .map_err(|e| format!("Failed to set sync mode: {}", e))?;
    }
    
    Ok(())
}

/// 断开远程数据库连接
#[command]
pub async fn disconnect_remote_database(
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.disconnect_remote_db()
            .await
            .map_err(|e| format!("Failed to disconnect remote database: {}", e))?;
    }
    
    Ok(())
}

/// 获取同步状态
#[command]
pub async fn get_sync_status(
    db_manager: State<'_, DbManager>,
) -> Result<SyncStatusResponse, String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        let is_online = sync_manager.is_online().await;
        let stats = sync_manager.get_sync_stats()
            .await
            .map_err(|e| format!("Failed to get sync stats: {}", e))?;
        
        // 获取同步配置
        let conn = db_manager.get_conn().await;
        let config = crate::db::get_sync_config(&conn)
            .await
            .map_err(|e| format!("Failed to get sync config: {}", e))?;
        
        Ok(SyncStatusResponse {
            is_enabled: true,
            is_online,
            sync_mode: config.sync_mode.to_string(),
            last_sync_time: config.last_sync_at,
            stats: Some(stats),
        })
    } else {
        Ok(SyncStatusResponse {
            is_enabled: false,
            is_online: false,
            sync_mode: "OFFLINE".to_string(),
            last_sync_time: 0,
            stats: None,
        })
    }
}

/// 手动执行同步
#[command]
pub async fn manual_sync(
    db_manager: State<'_, DbManager>,
) -> Result<SyncStats, String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        // 确保远程数据库连接
        if let Err(e) = sync_manager.ensure_remote_connection().await {
            return Err(format!("Failed to establish remote connection: {}", e));
        }
        
        sync_manager.sync()
            .await
            .map_err(|e| format!("Sync failed: {}", e))
    } else {
        Err("Sync manager not available".to_string())
    }
}

/// 设置同步模式
#[command]
pub async fn set_sync_mode(
    db_manager: State<'_, DbManager>,
    mode: String,
) -> Result<(), String> {
    let sync_mode = match mode.to_uppercase().as_str() {
        "AUTO" => SyncMode::Auto,
        "MANUAL" => SyncMode::Manual,
        "OFFLINE" => SyncMode::Offline,
        _ => return Err("Invalid sync mode".to_string()),
    };
    
    // 首先更新数据库中的同步配置
    let conn = db_manager.get_conn().await;
    
    // 获取现有配置或创建默认配置
    let mut sync_config = match crate::db::get_sync_config(&conn).await {
        Ok(config) => config,
        Err(_) => {
            // 创建默认配置
            let now = chrono::Utc::now().timestamp_millis();
            SyncConfig {
                id: "default".to_string(),
                remote_url: None,
                auth_token: None,
                sync_mode: sync_mode.clone(),
                sync_interval: 300,
                last_sync_at: 0,
                is_online: false,
                auto_sync_enabled: true,
                created_at: now,
                updated_at: now,
            }
        }
    };
    
    // 更新同步模式
    sync_config.sync_mode = sync_mode.clone();
    sync_config.updated_at = chrono::Utc::now().timestamp_millis();
    
    // 保存到数据库
    crate::db::save_sync_config(&conn, &sync_config)
        .await
        .map_err(|e| format!("Failed to save sync config: {}", e))?;
    
    // 如果有同步管理器，也更新同步管理器的配置
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.set_sync_mode(sync_mode)
            .await
            .map_err(|e| format!("Failed to set sync mode on sync manager: {}", e))?;
    }
    
    Ok(())
}

/// 迁移本地数据到远程数据库
#[command]
pub async fn migrate_to_remote(
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.migrate_to_remote()
            .await
            .map_err(|e| format!("Migration failed: {}", e))
    } else {
        Err("Sync manager not available".to_string())
    }
}

/// 重置同步状态
#[command]
pub async fn reset_sync_status(
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.reset_sync_status()
            .await
            .map_err(|e| format!("Failed to reset sync status: {}", e))
    } else {
        Err("Sync manager not available".to_string())
    }
}

/// 获取冲突列表
#[command]
pub async fn get_sync_conflicts(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<ConflictData>, String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.get_conflicts()
            .await
            .map_err(|e| format!("Failed to get conflicts: {}", e))
    } else {
        Ok(vec![])
    }
}

/// 解决冲突
#[command]
pub async fn resolve_conflict(
    db_manager: State<'_, DbManager>,
    request: ConflictResolutionRequest,
) -> Result<(), String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        let strategy = match request.strategy.to_uppercase().as_str() {
            "LOCAL_WINS" => ConflictResolutionStrategy::LocalWins,
            "REMOTE_WINS" => ConflictResolutionStrategy::RemoteWins,
            "MERGE" => ConflictResolutionStrategy::Merge,
            "USER_CHOICE" => ConflictResolutionStrategy::UserChoice,
            _ => return Err("Invalid conflict resolution strategy".to_string()),
        };
        
        sync_manager.resolve_conflict_by_user(&request.record_id, &request.table_name, strategy)
            .await
            .map_err(|e| format!("Failed to resolve conflict: {}", e))
    } else {
        Err("Sync manager not available".to_string())
    }
}

/// 测试远程数据库连接
#[command]
pub async fn test_remote_connection(
    remote_url: String,
    auth_token: Option<String>,
) -> Result<bool, String> {
    use libsql::Builder;
    use std::env;
    use uuid::Uuid;
    
    // 创建临时本地副本文件
    let temp_path = env::temp_dir().join(format!("mytips_test_{}.db", Uuid::new_v4()));
    let temp_path_str = temp_path.to_string_lossy().to_string();
    
    // 构建远程数据库连接进行测试
    let builder_result = if let Some(token) = auth_token {
        Builder::new_remote_replica(temp_path_str, remote_url, token).build().await
    } else {
        Builder::new_remote_replica(temp_path_str, remote_url, String::new()).build().await
    };
    
    match builder_result {
        Ok(db) => {
            // 尝试连接和同步
            match db.sync().await {
                Ok(_) => {
                    // 确保远程数据库有正确的表结构
                    let conn = match db.connect() {
                        Ok(conn) => conn,
                        Err(_) => {
                            let _ = std::fs::remove_file(&temp_path);
                            return Ok(false);
                        }
                    };
                    if let Err(e) = crate::db::init_schema(&conn).await {
                        eprintln!("Warning: Failed to initialize remote database schema during test: {}", e);
                        // 继续执行，但可能会影响后续功能
                    }
                    
                    // 再次同步以确保表结构更新到远程
                    if let Err(e) = db.sync().await {
                        eprintln!("Warning: Failed to sync schema to remote during test: {}", e);
                    }
                    
                    let _ = std::fs::remove_file(&temp_path);
                    Ok(true)
                }
                Err(_) => {
                    let _ = std::fs::remove_file(&temp_path);
                    Ok(false)
                }
            }
        }
        Err(_) => {
            let _ = std::fs::remove_file(&temp_path);
            Ok(false)
        }
    }
}

/// 获取同步历史记录
#[command]
pub async fn get_sync_history(
    db_manager: State<'_, DbManager>,
    limit: Option<u32>,
) -> Result<Vec<crate::sync::SyncEvent>, String> {
    if let Some(sync_manager) = db_manager.get_sync_manager().await {
        sync_manager.get_sync_history(limit.unwrap_or(50))
            .await
            .map_err(|e| format!("Failed to get sync history: {}", e))
    } else {
        Ok(vec![])
    }
}

/// 获取同步配置
#[command]
pub async fn get_sync_config(
    db_manager: State<'_, DbManager>,
) -> Result<SyncConfig, String> {
    let conn = db_manager.get_conn().await;
    crate::db::get_sync_config(&conn)
        .await
        .map_err(|e| format!("Failed to get sync config: {}", e))
}

/// 保存同步配置
#[command]
pub async fn save_sync_config(
    db_manager: State<'_, DbManager>,
    config: SyncConfig,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await;
    crate::db::save_sync_config(&conn, &config)
        .await
        .map_err(|e| format!("Failed to save sync config: {}", e))
}

/// 启用同步功能
#[command]
pub async fn enable_sync(
    db_manager: State<'_, DbManager>,
    remote_url: String,
    auth_token: Option<String>,
) -> Result<(), String> {
    // 使用DbManager的enable_sync方法
    db_manager.enable_sync(remote_url, auth_token)
        .await
        .map_err(|e| format!("Failed to enable sync: {}", e))
}

/// 动态启用同步功能（新增方法）
#[command]
pub async fn enable_cloud_sync(
    db_manager: State<'_, DbManager>,
    remote_url: String,
    auth_token: Option<String>,
) -> Result<(), String> {
    // 首先保存远程数据库配置到设置表
    {
        let conn = db_manager.get_conn().await;
        
        // 保存远程数据库配置到settings表
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('remote_db_url', ?)",
            libsql::params![remote_url.clone()]
        ).await.map_err(|e| format!("Failed to save remote URL: {}", e))?;
        
        if let Some(ref token) = auth_token {
            conn.execute(
                "INSERT OR REPLACE INTO settings (key, value) VALUES ('remote_db_auth_token', ?)",
                libsql::params![token.clone()]
            ).await.map_err(|e| format!("Failed to save auth token: {}", e))?;
        }
    }
    
    // 使用DbManager的enable_sync方法
    db_manager.enable_sync(remote_url, auth_token)
        .await
        .map_err(|e| format!("Failed to enable sync: {}", e))
}

/// 禁用同步功能  
#[command]
pub async fn disable_sync(
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    db_manager.disable_sync()
        .await
        .map_err(|e| format!("Failed to disable sync: {}", e))
}

/// 重新初始化远程数据库（清空并重新同步）
#[command]
pub async fn reinitialize_remote_database(
    db_manager: State<'_, DbManager>,
    remote_url: String,
    auth_token: Option<String>,
) -> Result<(), String> {
    // 先禁用现有同步
    if let Err(e) = db_manager.disable_sync().await {
        eprintln!("Warning: Failed to disable existing sync: {}", e);
    }
    
    // 等待一段时间确保清理完成
    tokio::time::sleep(tokio::time::Duration::from_millis(2000)).await;
    
    // 重新启用同步，这会创建全新的远程数据库
    db_manager.enable_sync(remote_url, auth_token)
        .await
        .map_err(|e| format!("Failed to reinitialize remote database: {}", e))
} 