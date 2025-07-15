use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::{command, AppHandle, Manager, State};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::db::DbManager;

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

// 获取当前数据库路径
#[command]
pub async fn get_current_database_path(app: AppHandle) -> Result<String, String> {
    match get_current_db_path(&app) {
        Ok(path) => Ok(path.to_string_lossy().to_string()),
        Err(e) => Err(format!("获取数据库路径失败: {}", e)),
    }
}

// 获取数据库信息
#[command]
pub async fn get_database_info(
    app: AppHandle,
    db_manager: State<'_, DbManager>,
) -> Result<DatabaseInfo, String> {
    let db_path = get_current_db_path(&app).map_err(|e| format!("获取数据库路径失败: {}", e))?;
    
    // 获取文件大小
    let file_size = match fs::metadata(&db_path) {
        Ok(metadata) => {
            let size_bytes = metadata.len();
            format_file_size(size_bytes)
        }
        Err(_) => "未知".to_string(),
    };

    // 获取最后修改时间
    let last_modified = match fs::metadata(&db_path) {
        Ok(metadata) => {
                         match metadata.modified() {
                 Ok(time) => {
                     let datetime: DateTime<Local> = time.into();
                     datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                 }
                 Err(_) => "未知".to_string(),
             }
        }
        Err(_) => "未知".to_string(),
    };

    // 获取笔记和分类数量
    let conn = db_manager
        .get_conn()
        .map_err(|e| format!("获取数据库连接失败: {}", e))?;

    let note_count = conn
        .query_row("SELECT COUNT(*) FROM tips", [], |row| row.get(0))
        .unwrap_or(0);

    let category_count = conn
        .query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))
        .unwrap_or(0);

    Ok(DatabaseInfo {
        size: file_size,
        note_count,
        category_count,
        last_modified,
    })
}

// 选择数据库文件
#[command]
pub async fn select_database_file(app: AppHandle) -> Result<DatabaseOperationResult, String> {
    let dialog = app.dialog();
    
    let file_path = dialog
        .file()
        .add_filter("数据库文件", &["db", "sqlite", "sqlite3"])
        .set_title("选择数据库文件")
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let path_buf = path.as_path().unwrap().to_path_buf();
            let path_str = path_buf.to_string_lossy().to_string();
            
            // 验证文件是否存在且可读
            if !path_buf.exists() {
                return Err("选择的文件不存在".to_string());
            }

            // 尝试打开数据库文件验证其有效性
            match rusqlite::Connection::open(&path_buf) {
                Ok(_) => {
                    // 保存新的数据库路径到设置
                    if let Err(e) = save_database_path(&app, &path_str) {
                        return Err(format!("保存数据库路径失败: {}", e));
                    }

                    Ok(DatabaseOperationResult {
                        path: Some(path_str),
                        restart_required: true,
                    })
                }
                Err(e) => Err(format!("无效的数据库文件: {}", e)),
            }
        }
        None => Ok(DatabaseOperationResult {
            path: None,
            restart_required: false,
        }),
    }
}

// 创建新数据库
#[command]
pub async fn create_new_database(app: AppHandle) -> Result<DatabaseOperationResult, String> {
    let dialog = app.dialog();
    
    let file_path = dialog
        .file()
        .add_filter("数据库文件", &["db"])
        .set_title("创建新数据库文件")
        .set_file_name("mytips_new.db")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            let path_buf = path.as_path().unwrap().to_path_buf();
            let path_str = path_buf.to_string_lossy().to_string();
            
            // 如果文件已存在，询问是否覆盖
            if path_buf.exists() {
                let dialog_result = dialog
                    .message("文件已存在，是否覆盖？")
                    .kind(MessageDialogKind::Warning)
                    .blocking_show();

                if !dialog_result {
                    return Ok(DatabaseOperationResult {
                        path: None,
                        restart_required: false,
                    });
                }
            }

            // 创建新的数据库文件
            match create_empty_database(&path_buf) {
                Ok(_) => {
                    // 保存新的数据库路径到设置
                    if let Err(e) = save_database_path(&app, &path_str) {
                        return Err(format!("保存数据库路径失败: {}", e));
                    }

                    Ok(DatabaseOperationResult {
                        path: Some(path_str),
                        restart_required: true,
                    })
                }
                Err(e) => Err(format!("创建数据库失败: {}", e)),
            }
        }
        None => Ok(DatabaseOperationResult {
            path: None,
            restart_required: false,
        }),
    }
}

// 重置到默认数据库位置
#[command]
pub async fn reset_to_default_database(app: AppHandle) -> Result<DatabaseOperationResult, String> {
    // 删除自定义数据库路径设置
    if let Err(e) = remove_database_path_setting(&app) {
        return Err(format!("重置数据库路径失败: {}", e));
    }

    let default_path = get_default_db_path(&app)
        .map_err(|e| format!("获取默认数据库路径失败: {}", e))?;

    Ok(DatabaseOperationResult {
        path: Some(default_path.to_string_lossy().to_string()),
        restart_required: true,
    })
}

// 辅助函数：获取当前数据库路径
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

// 获取默认数据库路径
fn get_default_db_path(app: &AppHandle) -> Result<PathBuf> {
    let data_dir = app.path()
        .app_data_dir()
        .map_err(|_| anyhow!("无法确定数据目录"))?;
    
    // 确保目录存在
    fs::create_dir_all(&data_dir)?;
    
    Ok(data_dir.join("mytips.db"))
}

// 获取自定义数据库路径设置
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

// 保存数据库路径设置
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

// 删除自定义数据库路径设置
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

// 创建一个空的数据库文件
fn create_empty_database(path: &PathBuf) -> Result<()> {
    // 如果文件存在，先删除
    if path.exists() {
        fs::remove_file(path)?;
    }

    // 确保父目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    // 创建新的数据库连接，这会创建文件
    let conn = rusqlite::Connection::open(path)?;

    // 创建基本表结构（复制自db.rs的init方法）
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tips (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            tip_type TEXT NOT NULL,
            language TEXT,
            category_id TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (category_id) REFERENCES categories (id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY(conversation_id) REFERENCES ai_conversations(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_tags (
            tip_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (tip_id, tag_id),
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            source TEXT,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_images (
            id TEXT PRIMARY KEY,
            tip_id TEXT NOT NULL,
            image_data TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_roles (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS encryption_status (
            id TEXT PRIMARY KEY,
            item_type TEXT NOT NULL,
            item_id TEXT NOT NULL,
            is_encrypted BOOLEAN NOT NULL DEFAULT FALSE,
            is_unlocked BOOLEAN NOT NULL DEFAULT FALSE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            UNIQUE(item_type, item_id)
        )",
        [],
    )?;

    // 创建索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_category ON tips(category_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_created_at ON tips(created_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_clipboard_created_at ON clipboard_history(created_at)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_ai_messages_conversation ON ai_messages(conversation_id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_encryption_status_item ON encryption_status(item_type, item_id)", [])?;

    Ok(())
}

// 格式化文件大小
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
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

 