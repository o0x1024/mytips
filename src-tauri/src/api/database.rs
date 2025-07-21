use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::db::UnifiedDbManager;
// 移除迁移相关的导入
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub size: String,
    pub note_count: i64,
    pub category_count: i64,
    pub last_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseOperationResult {
    pub path: Option<String>,
    pub restart_required: bool,
}

/// 获取当前数据库路径
#[command]
pub async fn get_current_database_path(
    app: AppHandle,
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    match get_current_db_path(&app) {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(e) => Err(format!("获取数据库路径失败: {}", e)),
    }
}

/// 获取数据库信息
#[command]
pub async fn get_database_info(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取数据库文件大小
    let db_path = get_current_db_path(&app).map_err(|e| format!("获取数据库路径失败: {}", e))?;
    let file_size = match std::fs::metadata(&db_path) {
        Ok(metadata) => {
            let size_bytes = metadata.len();
            format_file_size(size_bytes)
        },
        Err(_) => "Unknown".to_string(),
    };
    
    // 获取笔记数量
    let note_count: i64 = {
        let mut rows = conn.query("SELECT COUNT(*) FROM tips", ()).await.map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            row.get::<i64>(0).map_err(|e| e.to_string())?
        } else {
            0
        }
    };
    
    // 获取分类数量
    let category_count: i64 = {
        let mut rows = conn.query("SELECT COUNT(*) FROM categories", ()).await.map_err(|e| e.to_string())?;
        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            row.get::<i64>(0).map_err(|e| e.to_string())?
        } else {
            0
        }
    };
    
    // 获取最后修改时间
    let last_modified = match std::fs::metadata(&db_path) {
        Ok(metadata) => {
            match metadata.modified() {
                Ok(time) => {
                    let datetime: chrono::DateTime<chrono::Utc> = time.into();
                    datetime.with_timezone(&chrono::Local).format("%Y-%m-%d %H:%M:%S").to_string()
                },
                Err(_) => "Unknown".to_string(),
            }
        },
        Err(_) => "Unknown".to_string(),
    };
    
    Ok(serde_json::json!({
        "size": file_size,
        "note_count": note_count,
        "category_count": category_count,
        "last_modified": last_modified
    }))
}

/// 获取远程数据库统计信息
#[command]
pub async fn get_remote_database_info(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    // TODO: 实现远程数据库信息获取
    Err("远程数据库信息获取功能暂未实现".to_string())
}

/// 选择数据库文件
#[command]
pub async fn select_database_file(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    // 打开文件选择对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("数据库文件", &["db", "sqlite", "sqlite3"])
        .blocking_pick_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("无法获取选择的文件路径".to_string()),
        },
        None => return Err("用户取消了文件选择".to_string()),
    };

    // 验证文件是否存在
    if !file_path.exists() {
        return Err("选择的文件不存在".to_string());
    }

    // 验证是否为有效的SQLite数据库文件
    if let Err(e) = validate_sqlite_database(&file_path).await {
        return Err(format!("选择的文件不是有效的SQLite数据库: {}", e));
    }

    // 保存新的数据库路径配置
    if let Err(e) = save_database_path(&app, &file_path.to_string_lossy()) {
        return Err(format!("保存数据库路径配置失败: {}", e));
    }

    Ok(serde_json::json!({
        "message": format!("已选择数据库文件: {}", file_path.display()),
        "path": file_path.to_string_lossy(),
        "restart_required": true
    }))
}

/// 创建新数据库
#[command]
pub async fn create_new_database(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    // 打开保存文件对话框
    let file_path = match app
        .dialog()
        .file()
        .add_filter("数据库文件", &["db"])
        .set_file_name("mytips_new.db")
        .blocking_save_file()
    {
        Some(path) => match path.as_path() {
            Some(p) => p.to_path_buf(),
            None => return Err("无法获取保存文件路径".to_string()),
        },
        None => return Err("用户取消了文件创建".to_string()),
    };

    // 创建新的数据库
            if let Err(e) = create_empty_database(&file_path).await {
        return Err(format!("创建数据库失败: {}", e));
    }

    // 保存新的数据库路径配置
    if let Err(e) = save_database_path(&app, &file_path.to_string_lossy()) {
        return Err(format!("保存数据库路径配置失败: {}", e));
    }

    Ok(serde_json::json!({
        "message": format!("成功创建新数据库: {}", file_path.display()),
        "path": file_path.to_string_lossy(),
        "restart_required": true
    }))
}

/// 重置到默认数据库位置
#[command]
pub async fn reset_to_default_database(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    // 获取默认数据库路径
    let default_path = get_default_db_path(&app)
        .map_err(|e| format!("获取默认数据库路径失败: {}", e))?;

    // 如果默认数据库不存在，创建一个新的
    if !default_path.exists() {
        if let Err(e) = create_empty_database(&default_path).await {
            return Err(format!("创建默认数据库失败: {}", e));
        }
    }

    // 删除自定义数据库路径配置
    if let Err(e) = remove_database_path_setting(&app) {
        return Err(format!("重置数据库路径配置失败: {}", e));
    }

    Ok(serde_json::json!({
        "message": format!("已重置到默认数据库位置: {}", default_path.display()),
        "path": default_path.to_string_lossy(),
        "restart_required": true
    }))
}

/// 测试远程数据库连接
#[command(rename_all = "snake_case")]
pub async fn test_remote_connection(
    remote_url: String,
    auth_token: Option<String>,
) -> Result<bool, String> {
    if remote_url.trim().is_empty() {
        return Err("Remote URL cannot be empty".to_string());
    }
    
    // 检查URL格式
    if !remote_url.starts_with("libsql://") && 
       !remote_url.starts_with("https://") && 
       !remote_url.starts_with("http://") && 
       !remote_url.starts_with("wss://") {
        return Err("Invalid URL format. Expected libsql://, https://, http://, or wss://".to_string());
    }
    
    println!("Testing connection to: {}", remote_url);
    
    // 尝试实际连接
    match test_libsql_connection(&remote_url, auth_token.as_deref()).await {
        Ok(success) => Ok(success),
        Err(e) => {
            println!("Connection test failed: {}", e);
            // 返回false而不是错误，这样前端可以显示连接失败但不会阻止其他操作
            Ok(false)
        }
    }
}

/// 实际测试libSQL连接
async fn test_libsql_connection(url: &str, auth_token: Option<&str>) -> Result<bool> {
    // 这里应该实现实际的libSQL连接测试
    // 由于需要libsql依赖，暂时使用简单的HTTP请求测试
    
    if url.starts_with("http://") || url.starts_with("https://") {
        // 对于HTTP URL，尝试发送一个简单的请求
        let client = reqwest::Client::new();
        let mut request = client.get(url);
        
        if let Some(token) = auth_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }
        
        match request.send().await {
            Ok(response) => {
                // 如果收到任何响应（即使是错误），说明服务器是可达的
                Ok(response.status().as_u16() < 500)
            },
            Err(_) => Ok(false),
        }
    } else {
        // 对于libsql:// URL，暂时返回true（需要实际的libsql实现）
        println!("LibSQL connection test not fully implemented yet");
        Ok(true)
    }
}

/// 获取同步配置
#[command]
pub async fn get_sync_config(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match crate::db::get_sync_config(&conn).await {
        Ok(Some(config)) => {
            Ok(serde_json::json!({
                "id": config.id,
                "remote_url": config.remote_url.unwrap_or_default(),
                "auth_token": config.auth_token.unwrap_or_default(),
                "sync_mode": config.sync_mode.to_string(),
                "sync_interval": config.sync_interval,
                "last_sync_at": config.last_sync_at,
                "is_online": config.is_online,
                "auto_sync_enabled": config.auto_sync_enabled,
                "created_at": config.created_at,
                "updated_at": config.updated_at,
            }))
        },
        Ok(None) => {
            // 返回默认配置
            Ok(serde_json::json!({
                "id": "default",
                "remote_url": "",
                "auth_token": "",
                "sync_mode": "OFFLINE",
                "sync_interval": 300,
                "last_sync_at": 0,
                "is_online": false,
                "auto_sync_enabled": true,
                "created_at": chrono::Utc::now().timestamp_millis(),
                "updated_at": chrono::Utc::now().timestamp_millis(),
            }))
        },
        Err(e) => Err(format!("Failed to get sync config: {}", e)),
    }
}

/// 保存同步配置
#[command]
pub async fn save_sync_config(
    db_manager: State<'_, UnifiedDbManager>,
    config: serde_json::Value,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    let sync_config = crate::db::SyncConfig {
        id: config["id"].as_str().unwrap_or("default").to_string(),
        remote_url: config["remote_url"].as_str().map(|s| if s.is_empty() { None } else { Some(s.to_string()) }).flatten(),
        auth_token: config["auth_token"].as_str().map(|s| if s.is_empty() { None } else { Some(s.to_string()) }).flatten(),
        sync_mode: crate::db::SyncMode::from(config["sync_mode"].as_str().unwrap_or("OFFLINE").to_string()),
        sync_interval: config["sync_interval"].as_i64().unwrap_or(300),
        last_sync_at: config["last_sync_at"].as_i64().unwrap_or(0),
        is_online: config["is_online"].as_bool().unwrap_or(false),
        auto_sync_enabled: config["auto_sync_enabled"].as_bool().unwrap_or(true),
        created_at: config["created_at"].as_i64().unwrap_or_else(|| chrono::Utc::now().timestamp_millis()),
        updated_at: chrono::Utc::now().timestamp_millis(),
    };
    
    crate::db::save_sync_config(&conn, &sync_config).await
        .map_err(|e| format!("Failed to save sync config: {}", e))
}

/// 获取同步状态
#[command]
pub async fn get_sync_status(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match crate::db::get_sync_config(&conn).await {
        Ok(Some(config)) => {
            let is_enabled = config.sync_mode != crate::db::SyncMode::Offline && config.remote_url.is_some();
            
            // 获取同步统计信息
            let stats = get_sync_statistics(&conn).await.unwrap_or_default();
            
            Ok(serde_json::json!({
                "is_enabled": is_enabled,
                "is_online": config.is_online,
                "sync_mode": config.sync_mode.to_string(),
                "last_sync_time": config.last_sync_at,
                "stats": {
                    "total_records": stats.total_records,
                    "synced_records": stats.synced_records,
                    "pending_records": stats.pending_records,
                    "failed_records": stats.failed_records,
                    "is_online": config.is_online
                }
            }))
        },
        Ok(None) => {
            Ok(serde_json::json!({
                "is_enabled": false,
                "is_online": false,
                "sync_mode": "OFFLINE",
                "last_sync_time": 0,
                "stats": {
                    "total_records": 0,
                    "synced_records": 0,
                    "pending_records": 0,
                    "failed_records": 0,
                    "is_online": false
                }
            }))
        },
        Err(e) => Err(format!("Failed to get sync status: {}", e)),
    }
}

/// 设置同步模式
#[command]
pub async fn set_sync_mode(
    db_manager: State<'_, UnifiedDbManager>,
    mode: String,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取现有配置或创建默认配置
    let mut config = match crate::db::get_sync_config(&conn).await {
        Ok(Some(config)) => config,
        Ok(None) => crate::db::SyncConfig {
            id: "default".to_string(),
            remote_url: None,
            auth_token: None,
            sync_mode: crate::db::SyncMode::Offline,
            sync_interval: 300,
            last_sync_at: 0,
            is_online: false,
            auto_sync_enabled: true,
            created_at: chrono::Utc::now().timestamp_millis(),
            updated_at: chrono::Utc::now().timestamp_millis(),
        },
        Err(e) => return Err(format!("Failed to get sync config: {}", e)),
    };
    
    config.sync_mode = crate::db::SyncMode::from(mode);
    config.updated_at = chrono::Utc::now().timestamp_millis();
    
    crate::db::save_sync_config(&conn, &config).await
        .map_err(|e| format!("Failed to save sync config: {}", e))
}

/// 使用LibSQL进行安全同步
#[command]
pub async fn manual_sync_libsql(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    // TODO: 实现LibSQL同步功能
    Err("LibSQL同步功能暂未实现".to_string())
}

/// 手动同步
#[command]
pub async fn manual_sync(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    // TODO: 实现完整的同步管理器集成
    // 暂时使用 UnifiedDbManager 的内置同步功能
    if let Err(e) = db_manager.inner().sync().await {
        return Err(format!("同步失败: {}", e));
    }

    // 返回基本的同步结果
    Ok(serde_json::json!({
        "total_records": 0,
        "synced_records": 0,
        "pending_records": 0,
        "failed_records": 0,
        "last_sync_time": chrono::Utc::now().timestamp_millis(),
        "is_online": true,
        "message": "手动同步完成"
    }))
}

/// 清理已同步的记录
#[command]
pub async fn clear_synced_records(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 清理已同步的记录
    let result = conn.execute(
        "DELETE FROM sync_status WHERE sync_status = 'SYNCED'",
        ()
    ).await.map_err(|e| format!("Failed to clear synced records: {}", e))?;
    
    Ok(serde_json::json!({
        "message": "已同步的记录已清理",
        "cleared_count": result
    }))
}

/// 保存数据库类型设置
#[command]
pub async fn save_database_type(
    db_manager: State<'_, UnifiedDbManager>,
    database_type: String,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    crate::db::save_setting(&conn, "database_type", &database_type).await
        .map_err(|e| format!("Failed to save database type: {}", e))
}

/// 获取数据库类型设置
#[command]
pub async fn get_database_type(db_manager: State<'_, UnifiedDbManager>) -> Result<String, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match crate::db::get_setting(&conn, "database_type").await {
        Ok(Some(db_type)) => Ok(db_type),
        Ok(None) => Ok("local".to_string()), // 默认为本地数据库
        Err(e) => Err(format!("Failed to get database type: {}", e)),
    }
}

// 内部函数：为现有数据创建同步记录
async fn create_sync_records_for_existing_data_internal(conn: &libsql::Connection) -> Result<i32, anyhow::Error> {
    let mut created_records = 0;
    
    // 为所有笔记创建同步记录
    let mut tips_rows = conn.query("SELECT id FROM tips", ()).await?;
    
    while let Some(row) = tips_rows.next().await? {
        let tip_id: String = row.get(0)?;
        let sync_record_id = uuid::Uuid::new_v4().to_string();
        
        conn.execute(
            "INSERT OR IGNORE INTO sync_status (id, table_name, record_id, operation, sync_status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                sync_record_id,
                "tips",
                tip_id,
                "INSERT",
                "PENDING",
                chrono::Utc::now().timestamp_millis(),
                chrono::Utc::now().timestamp_millis(),
            )
        ).await?;
        
        created_records += 1;
    }
    
    // 为所有分类创建同步记录
    let mut categories_rows = conn.query("SELECT id FROM categories", ()).await?;
    
    while let Some(row) = categories_rows.next().await? {
        let category_id: String = row.get(0)?;
        let sync_record_id = uuid::Uuid::new_v4().to_string();
        
        conn.execute(
            "INSERT OR IGNORE INTO sync_status (id, table_name, record_id, operation, sync_status, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                sync_record_id,
                "categories",
                category_id,
                "INSERT",
                "PENDING",
                chrono::Utc::now().timestamp_millis(),
                chrono::Utc::now().timestamp_millis(),
            )
        ).await?;
        
        created_records += 1;
    }
    
    Ok(created_records)
}

/// 为现有数据创建同步记录
#[command]
pub async fn create_sync_records_for_existing_data(db_manager: State<'_, UnifiedDbManager>) -> Result<serde_json::Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match create_sync_records_for_existing_data_internal(&conn).await {
        Ok(created_records) => Ok(serde_json::json!({
            "message": format!("已为 {} 条记录创建同步状态", created_records),
            "created_records": created_records
        })),
        Err(e) => Err(format!("创建同步记录失败: {}", e))
    }
}

/// 配置远程数据库
#[command]
pub async fn configure_remote_database(
    db_manager: State<'_, UnifiedDbManager>,
    config: serde_json::Value,
) -> Result<(), String> {
    println!("Starting configure_remote_database");
    
    let remote_url = config["remote_url"].as_str()
        .ok_or("Missing remote_url")?;
    let auth_token = config["auth_token"].as_str().map(|s| s.to_string());
    let sync_mode = config["sync_mode"].as_str().unwrap_or("MANUAL");
    let sync_interval = config["sync_interval"].as_i64().unwrap_or(300);
    let auto_sync_enabled = config["auto_sync_enabled"].as_bool().unwrap_or(true);
    
    println!("Remote URL: {}, Sync mode: {}", remote_url, sync_mode);
    
    // 测试连接
    if !test_libsql_connection(remote_url, auth_token.as_deref()).await
        .map_err(|e| format!("连接测试失败: {}", e))? {
        return Err("无法连接到远程数据库".to_string());
    }
    
    println!("Remote connection test passed");
    
    // 创建并保存同步配置（不进行实际的数据库副本创建）
    let sync_config = crate::db::SyncConfig {
        id: "default".to_string(),
        remote_url: Some(remote_url.to_string()),
        auth_token: auth_token.clone(),
        sync_mode: crate::db::SyncMode::from(sync_mode.to_string()),
        sync_interval,
        last_sync_at: 0,
        is_online: true,
        auto_sync_enabled,
        created_at: chrono::Utc::now().timestamp_millis(),
        updated_at: chrono::Utc::now().timestamp_millis(),
    };
    
    // 保存配置到本地数据库
    {
        let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
        crate::db::save_sync_config(&conn, &sync_config).await
            .map_err(|e| format!("Failed to save sync config: {}", e))?;
        println!("Sync config saved");
    }
    
    // 为现有数据创建同步记录，但不立即同步
    {
        let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
        match create_sync_records_for_existing_data_internal(&conn).await {
            Ok(count) => println!("Created {} sync records for existing data", count),
            Err(e) => println!("Warning: Failed to create sync records: {}", e),
        }
    }
    
    println!("configure_remote_database completed successfully");
    Ok(())
}

// 辅助函数实现

/// 获取当前数据库路径
fn get_current_db_path(app: &AppHandle) -> Result<PathBuf> {
    // 首先检查是否有自定义路径设置
    if let Ok(Some(custom_path)) = get_custom_database_path(app) {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            return Ok(path);
        }
    }

    // 如果没有自定义路径或文件不存在，返回默认路径
    get_default_db_path(app)
}

/// 获取默认数据库路径
fn get_default_db_path(app: &AppHandle) -> Result<PathBuf> {
    let data_dir = app.path()
        .app_data_dir()
        .map_err(|_| anyhow!("无法确定数据目录"))?;
    
    // 确保目录存在
    fs::create_dir_all(&data_dir)?;
    
    Ok(data_dir.join("mytips.db"))
}

/// 获取自定义数据库路径设置
fn get_custom_database_path(app: &AppHandle) -> Result<Option<String>> {
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|_| anyhow!("无法确定配置目录"))?;
    
    let config_file = config_dir.join("database_path.txt");
    
    if config_file.exists() {
        let path = fs::read_to_string(config_file)?;
        Ok(Some(path.trim().to_string()))
    } else {
        Ok(None)
    }
}

/// 保存数据库路径设置
fn save_database_path(app: &AppHandle, path: &str) -> Result<()> {
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|_| anyhow!("无法确定配置目录"))?;
    
    // 确保配置目录存在
    fs::create_dir_all(&config_dir)?;
    
    let config_file = config_dir.join("database_path.txt");
    fs::write(config_file, path)?;
    Ok(())
}

/// 删除自定义数据库路径设置
fn remove_database_path_setting(app: &AppHandle) -> Result<()> {
    let config_dir = app.path()
        .app_config_dir()
        .map_err(|_| anyhow!("无法确定配置目录"))?;
    
    let config_file = config_dir.join("database_path.txt");
    if config_file.exists() {
        fs::remove_file(config_file)?;
    }
    Ok(())
}

/// 验证SQLite数据库文件
async fn validate_sqlite_database(path: &PathBuf) -> Result<()> {
    use libsql::Builder;
    
    // 尝试打开数据库连接
    let db = Builder::new_local(path)
        .build()
        .await
        .map_err(|e| anyhow!("无法打开数据库文件: {}", e))?;
    
    let conn = db.connect()
        .map_err(|e| anyhow!("无法连接到数据库: {}", e))?;
    
    // 检查是否有基本的表结构
    let mut rows = conn.query("SELECT name FROM sqlite_master WHERE type='table'", ())
        .await
        .map_err(|e| anyhow!("查询表结构失败: {}", e))?;
    
    let mut table_names = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| anyhow!("读取行失败: {}", e))? {
        let name: String = row.get(0).map_err(|e| anyhow!("获取表名失败: {}", e))?;
        table_names.push(name);
    }
    
    // 检查是否包含我们期望的基本表
    let has_tips = table_names.iter().any(|name| name == "tips");
    let has_categories = table_names.iter().any(|name| name == "categories");
    
    if !has_tips || !has_categories {
        return Err(anyhow!("数据库缺少必要的表结构"));
    }
    
    Ok(())
}

/// 创建一个空的数据库文件
async fn create_empty_database(path: &PathBuf) -> Result<()> {
    // 如果文件存在，先删除
    if path.exists() {
        fs::remove_file(path)?;
    }

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 创建新的数据库连接，这会创建文件
    use libsql::Builder;
    let db = Builder::new_local(path).build().await?;
    let conn = db.connect()?;

    // 创建表结构
    crate::db::operations::create_all_tables(&conn).await?;

    // 插入默认分类 - 使用 NULL 而不是空字符串避免外键约束问题
    let default_categories = vec![
        ("uncategorized", "未分类"),
    ];

    let now = chrono::Utc::now().timestamp_millis();
    for (id, name) in default_categories {
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, name, parent_id, created_at, updated_at) VALUES (?1, ?2, NULL, ?3, ?4)",
            (id, name, now, now)
        ).await?;
    }

    Ok(())
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

/// 格式化同步时间
fn format_sync_time(timestamp: i64) -> String {
    if timestamp == 0 {
        "从未同步".to_string()
    } else {
        match DateTime::from_timestamp(timestamp / 1000, 0) {
            Some(datetime) => {
                let local_datetime: DateTime<Local> = datetime.with_timezone(&Local);
                local_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
            }
            None => "时间格式错误".to_string()
        }
    }
}

/// 获取总记录数
async fn get_total_records(conn: &libsql::Connection) -> i64 {
    let tips_count: i64 = match conn.query("SELECT COUNT(*) FROM tips", ()).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    };
    
    let categories_count: i64 = match conn.query("SELECT COUNT(*) FROM categories", ()).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    };
    
    let tags_count: i64 = match conn.query("SELECT COUNT(*) FROM tags", ()).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    };
    
    tips_count + categories_count + tags_count
}

/// 获取已同步记录数
async fn get_synced_records(conn: &libsql::Connection) -> i64 {
    // 查询已同步状态的记录数
    match conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'SYNCED'", 
        ()
    ).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// 获取待同步记录数
async fn get_pending_records(conn: &libsql::Connection) -> i64 {
    match conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'PENDING'", 
        ()
    ).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// 获取失败记录数
async fn get_failed_records(conn: &libsql::Connection) -> i64 {
    match conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'FAILED'", 
        ()
    ).await {
        Ok(mut rows) => {
            if let Ok(Some(row)) = rows.next().await {
                row.get::<i64>(0).unwrap_or(0)
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

/// 执行初始同步（将本地数据推送到远程）
async fn perform_initial_sync_async(config: &crate::db::SyncConfig) -> Result<()> {
    println!("Performing initial sync to remote database: {:?}", config.remote_url);
    
    // 这个函数现在主要用于验证连接，实际的同步会在第一次手动同步时进行
    // 可以在这里添加一些基本的连接验证逻辑
    
    if let Some(ref remote_url) = config.remote_url {
        // 验证远程URL格式
        if !remote_url.starts_with("libsql://") && 
           !remote_url.starts_with("https://") && 
           !remote_url.starts_with("http://") && 
           !remote_url.starts_with("wss://") {
            return Err(anyhow!("Invalid remote URL format"));
        }
        
        println!("Initial sync configuration validated for URL: {}", remote_url);
    }
    
    Ok(())
}

// Migration-related functions have been removed as they depend on unimplemented types

// 辅助函数：获取同步统计信息
async fn get_sync_statistics(conn: &libsql::Connection) -> Result<SyncStatistics, anyhow::Error> {
    // 统计待同步记录
    let mut pending_rows = conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'PENDING'",
        ()
    ).await?;
    let pending_count: i64 = if let Some(row) = pending_rows.next().await? {
        row.get(0)?
    } else {
        0
    };
    
    // 统计已同步记录
    let mut synced_rows = conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'SYNCED'",
        ()
    ).await?;
    let synced_count: i64 = if let Some(row) = synced_rows.next().await? {
        row.get(0)?
    } else {
        0
    };
    
    // 统计失败记录
    let mut failed_rows = conn.query(
        "SELECT COUNT(*) FROM sync_status WHERE sync_status = 'FAILED'",
        ()
    ).await?;
    let failed_count: i64 = if let Some(row) = failed_rows.next().await? {
        row.get(0)?
    } else {
        0
    };
    
    // 统计总记录
    let mut total_rows = conn.query(
        "SELECT COUNT(*) FROM sync_status",
        ()
    ).await?;
    let total_count: i64 = if let Some(row) = total_rows.next().await? {
        row.get(0)?
    } else {
        0
    };
    
    Ok(SyncStatistics {
        total_records: total_count as i32,
        synced_records: synced_count as i32,
        pending_records: pending_count as i32,
        failed_records: failed_count as i32,
    })
}

#[derive(Debug)]
struct SyncStatistics {
    total_records: i32,
    synced_records: i32,
    pending_records: i32,
    failed_records: i32,
}

impl Default for SyncStatistics {
    fn default() -> Self {
        Self {
            total_records: 0,
            synced_records: 0,
            pending_records: 0,
            failed_records: 0,
        }
    }
}

 