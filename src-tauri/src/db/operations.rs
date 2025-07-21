use anyhow::{anyhow, Result};
use chrono::Utc;
use libsql::{params, Builder, Connection, Database};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::models::*;
use crate::api::encryption::EncryptionStatus;
use crate::api::tips::TipSummary;
use crate::api::clipboard_api::ClipboardHistory;
use crate::sync::SyncManager;

/// 数据库连接类型别名
pub type DbConnection = Connection;

// DbManager 已被移除，请使用 UnifiedDbManager

/// 获取数据库文件路径
fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| anyhow!("Failed to get app data directory: {}", e))?;
    
    Ok(app_dir.join("mytips.db"))
}

/// 创建所有数据库表
pub async fn create_all_tables(conn: &Connection) -> Result<()> {
    // 启用外键约束
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;

    // 创建分类表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            version INTEGER DEFAULT 1,
            last_synced_at INTEGER DEFAULT 0,
            sync_hash TEXT,
            is_encrypted BOOLEAN DEFAULT FALSE,
            encryption_key_id TEXT,
            FOREIGN KEY (parent_id) REFERENCES categories (id)
        )",
        (),
    ).await?;

    // 创建标签表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            version INTEGER DEFAULT 1,
            last_synced_at INTEGER DEFAULT 0,
            sync_hash TEXT
        )",
        (),
    ).await?;

    // 创建笔记表
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
            version INTEGER DEFAULT 1,
            last_synced_at INTEGER DEFAULT 0,
            sync_hash TEXT,
            is_encrypted BOOLEAN DEFAULT FALSE,
            encryption_key_id TEXT,
            encrypted_content TEXT,
            FOREIGN KEY (category_id) REFERENCES categories (id)
        )",
        (),
    ).await?;

    // 创建笔记标签关联表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_tags (
            tip_id TEXT NOT NULL,
            tag_id TEXT NOT NULL,
            PRIMARY KEY (tip_id, tag_id),
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建图片表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_images (
            id TEXT PRIMARY KEY,
            tip_id TEXT NOT NULL,
            image_id TEXT NOT NULL UNIQUE,
            image_data TEXT NOT NULL,
            image_format TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            width INTEGER,
            height INTEGER,
            alt_text TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建音频文件表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_audio_files (
            id TEXT PRIMARY KEY,
            tip_id TEXT NOT NULL,
            audio_id TEXT NOT NULL UNIQUE,
            file_name TEXT NOT NULL,
            file_format TEXT NOT NULL,
            audio_data BLOB NOT NULL,
            file_size INTEGER NOT NULL,
            duration INTEGER,
            transcription TEXT,
            transcription_confidence REAL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建AI角色表
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
    ).await?;

    // 创建AI对话表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model TEXT NOT NULL,
            role_id TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (role_id) REFERENCES ai_roles (id)
        )",
        (),
    ).await?;

    // 创建AI消息表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (conversation_id) REFERENCES ai_conversations (id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建模板表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tip_templates (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            template TEXT NOT NULL,
            category TEXT,
            description TEXT,
            is_system BOOLEAN DEFAULT FALSE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建剪贴板历史表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            source TEXT,
            created_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建应用设置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建加密密钥表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS encryption_keys (
            id TEXT PRIMARY KEY,
            key_name TEXT NOT NULL UNIQUE,
            key_data TEXT NOT NULL,
            salt TEXT NOT NULL,
            algorithm TEXT NOT NULL DEFAULT 'AES-256-GCM',
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建加密会话表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS encryption_sessions (
            id TEXT PRIMARY KEY,
            item_id TEXT NOT NULL,
            item_type TEXT NOT NULL,
            session_token TEXT NOT NULL,
            expires_at INTEGER NOT NULL,
            created_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建同步配置表
    conn.execute(
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
        (),
    ).await?;

    // 创建同步状态表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sync_status (
            id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            record_id TEXT NOT NULL,
            operation TEXT NOT NULL,
            sync_status TEXT DEFAULT 'PENDING',
            error_message TEXT,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            UNIQUE(table_name, record_id, operation)
        )",
        (),
    ).await?;

    // 创建数据版本表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data_versions (
            id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            record_id TEXT NOT NULL,
            version INTEGER NOT NULL DEFAULT 1,
            hash TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE(table_name, record_id)
        )",
        (),
    ).await?;

    // 创建冲突解决记录表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS conflict_resolutions (
            id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            record_id TEXT NOT NULL,
            strategy TEXT NOT NULL,
            resolved_by TEXT NOT NULL,
            local_content TEXT,
            remote_content TEXT,
            resolved_content TEXT,
            created_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建同步统计表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS sync_statistics (
            id TEXT PRIMARY KEY,
            sync_session_id TEXT NOT NULL,
            table_name TEXT NOT NULL,
            total_records INTEGER DEFAULT 0,
            synced_records INTEGER DEFAULT 0,
            failed_records INTEGER DEFAULT 0,
            conflict_records INTEGER DEFAULT 0,
            start_time INTEGER NOT NULL,
            end_time INTEGER,
            duration_ms INTEGER
        )",
        (),
    ).await?;

    // 创建所有索引
    create_all_indexes(conn).await?;

    tracing::info!("All database tables and indexes created successfully");
    Ok(())
}

/// 创建所有索引
async fn create_all_indexes(conn: &Connection) -> Result<()> {
    // 基础索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_category_id ON tips (category_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_created_at ON tips (created_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_updated_at ON tips (updated_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories (parent_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_ai_messages_conversation_id ON ai_messages (conversation_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_ai_conversations_role_id ON ai_conversations (role_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_clipboard_history_created_at ON clipboard_history (created_at)", ()).await?;

    // 图片相关索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tip_images_tip_id ON tip_images (tip_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tip_images_image_id ON tip_images (image_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tip_images_created_at ON tip_images (created_at)", ()).await?;

    // 加密相关索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_encryption_keys_key_name ON encryption_keys (key_name)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_encryption_sessions_item_id ON encryption_sessions (item_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_encryption_sessions_expires_at ON encryption_sessions (expires_at)", ()).await?;

    // 同步相关索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_status_table_record ON sync_status (table_name, record_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_status_status ON sync_status (sync_status)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_status_created_at ON sync_status (created_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_data_versions_table_record ON data_versions (table_name, record_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_data_versions_hash ON data_versions (hash)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_conflict_resolutions_table_record ON conflict_resolutions (table_name, record_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_conflict_resolutions_created_at ON conflict_resolutions (created_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_statistics_session_id ON sync_statistics (sync_session_id)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_statistics_table_name ON sync_statistics (table_name)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_sync_statistics_start_time ON sync_statistics (start_time)", ()).await?;

    // 版本控制索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_version ON tips (version)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_last_synced_at ON tips (last_synced_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_sync_hash ON tips (sync_hash)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_categories_version ON categories (version)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_categories_last_synced_at ON categories (last_synced_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_categories_sync_hash ON categories (sync_hash)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags_version ON tags (version)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags_last_synced_at ON tags (last_synced_at)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags_sync_hash ON tags (sync_hash)", ()).await?;

    // 搜索优化索引
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_title ON tips (title)", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_content_substr ON tips (substr(content, 1, 100))", ()).await?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tips_title_content_composite ON tips (title, updated_at)", ()).await?;

    Ok(())
}

/// 初始化默认数据
pub async fn init_default_data(conn: &Connection) -> Result<()> {
    let now = Utc::now().timestamp_millis();

    // 插入默认分类
    let default_categories = vec![
        ("uncategorized", "未分类"),
    ];

    for (id, name) in default_categories {
        conn.execute(
            "INSERT OR IGNORE INTO categories (id, name, parent_id, created_at, updated_at) VALUES (?1, ?2, NULL, ?3, ?4)",
            params![id, name, now, now]
        ).await?;
    }

    // 初始化默认AI角色
    init_default_roles(conn).await?;

    tracing::info!("Default data initialized successfully");
    Ok(())
}

/// 初始化默认AI角色
async fn init_default_roles(conn: &Connection) -> Result<()> {
    // 检查是否已有角色
    let count: i64 = conn.query("SELECT COUNT(*) FROM ai_roles", ()).await?
        .next().await?
        .unwrap()
        .get(0)?;
    
    if count > 0 {
        return Ok(());
    }

    let now = Utc::now().timestamp_millis();
    
    // 创建默认助手角色
    let default_role = AIRole {
        id: Uuid::new_v4().to_string(),
        name: "默认助手".to_string(),
        description: Some("通用AI助手，可以帮助您解答各种问题".to_string()),
        system_prompt: "你是一个有用的AI助手，请尽力帮助用户解决问题。".to_string(),
        model: Some("gpt-3.5-turbo".to_string()),
        temperature: Some(0.7),
        max_tokens: Some(2048),
        is_default: true,
        created_at: now,
        updated_at: now,
    };

    conn.execute(
        "INSERT INTO ai_roles (id, name, description, system_prompt, model, temperature, max_tokens, is_default, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            default_role.id.as_str(),
            default_role.name.as_str(),
            default_role.description.as_deref(),
            default_role.system_prompt.as_str(),
            default_role.model.as_deref(),
            default_role.temperature,
            default_role.max_tokens,
            default_role.is_default,
            default_role.created_at,
            default_role.updated_at
        ]
    ).await?;

    tracing::info!("Default AI roles created successfully");
    Ok(())
}

// ===============================================
// 笔记相关数据库操作函数
// ===============================================

/// 创建笔记
pub async fn create_tip(conn: &DbConnection, tip: &Tip) -> Result<()> {
    conn.execute(
        "INSERT INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            tip.id.as_str(),
            tip.title.as_str(),
            tip.content.as_str(),
            String::from(tip.tip_type),
            tip.language.as_deref(),
            tip.category_id.as_deref(),
            tip.created_at,
            tip.updated_at
        ]
    ).await?;
    Ok(())
}

/// 获取所有笔记
pub async fn list_tips(conn: &DbConnection) -> Result<Vec<Tip>> {
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at,
                version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content 
         FROM tips ORDER BY updated_at DESC",
        ()
    ).await?;

    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        let tip = Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: tip_type_str.try_into()?,
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            version: row.get(8)?,
            last_synced_at: row.get(9)?,
            sync_hash: row.get(10)?,
            is_encrypted: row.get(11)?,
            encryption_key_id: row.get(12)?,
            encrypted_content: row.get(13)?,
        };
        tips.push(tip);
    }
    Ok(tips)
}

/// 根据ID获取笔记
pub async fn get_tip_by_id(conn: &DbConnection, tip_id: &str) -> Result<Option<Tip>> {
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at,
                version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content 
         FROM tips WHERE id = ?1",
        params![tip_id]
    ).await?;

    if let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        let tip = Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: tip_type_str.try_into()?,
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            version: row.get(8)?,
            last_synced_at: row.get(9)?,
            sync_hash: row.get(10)?,
            is_encrypted: row.get(11)?,
            encryption_key_id: row.get(12)?,
            encrypted_content: row.get(13)?,
        };
        Ok(Some(tip))
    } else {
        Ok(None)
    }
}

/// 更新笔记
pub async fn update_tip(conn: &DbConnection, tip: &Tip) -> Result<()> {
    conn.execute(
        "UPDATE tips SET title = ?1, content = ?2, tip_type = ?3, language = ?4, category_id = ?5, updated_at = ?6 WHERE id = ?7",
        params![
            tip.title.as_str(),
            tip.content.as_str(),
            String::from(tip.tip_type),
            tip.language.as_deref(),
            tip.category_id.as_deref(),
            tip.updated_at,
            tip.id.as_str()
        ]
    ).await?;
    Ok(())
}

/// 删除笔记
pub async fn delete_tip(conn: &DbConnection, tip_id: &str) -> Result<()> {
    // 启动事务
    conn.execute("BEGIN TRANSACTION", ()).await?;
    
    match delete_tip_with_dependencies(conn, tip_id).await {
        Ok(_) => {
            conn.execute("COMMIT", ()).await?;
            Ok(())
        }
        Err(e) => {
            conn.execute("ROLLBACK", ()).await?;
            Err(e)
        }
    }
}

/// 删除笔记及其依赖关系的内部函数
async fn delete_tip_with_dependencies(conn: &DbConnection, tip_id: &str) -> Result<()> {
    // 1. 先查询并记录要删除的图片数量
    let image_count: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?
    .next().await?
    .unwrap()
    .get(0)?;
    
    println!("Deleting tip {} with {} images", tip_id, image_count);
    tracing::info!("Deleting tip {} with {} images", tip_id, image_count);
    
    // 2. 删除相关的图片
    let deleted_images = conn.execute(
        "DELETE FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?;
    
    println!("Deleted {} images for tip {}", deleted_images, tip_id);
    tracing::info!("Deleted {} images for tip {}", deleted_images, tip_id);
    
    // 3. 删除相关的标签关联
    let deleted_tags = conn.execute(
        "DELETE FROM tip_tags WHERE tip_id = ?1", 
        params![tip_id]
    ).await?;
    
    tracing::info!("Deleted {} tag associations for tip {}", deleted_tags, tip_id);
    
    // 4. 删除笔记本身
    let rows_affected = conn.execute(
        "DELETE FROM tips WHERE id = ?1", 
        params![tip_id]
    ).await?;
    
    if rows_affected == 0 {
        return Err(anyhow!("Tip not found or already deleted: {}", tip_id));
    }
    
    println!("Successfully deleted tip {} and all its dependencies", tip_id);
    tracing::info!("Successfully deleted tip {} and all its dependencies", tip_id);
    
    Ok(())
}

// ===============================================
// 分类相关数据库操作函数
// ===============================================

/// 创建分类
pub async fn create_category(conn: &DbConnection, category: &Category) -> Result<()> {
    conn.execute(
        "INSERT INTO categories (id, name, parent_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            category.id.as_str(),
            category.name.as_str(),
            category.parent_id.as_deref(),
            category.created_at,
            category.updated_at
        ]
    ).await?;
    Ok(())
}

/// 获取所有分类
pub async fn list_categories(conn: &DbConnection) -> Result<Vec<Category>> {
    let mut rows = conn.query(
        "SELECT id, name, parent_id, created_at, updated_at, version, last_synced_at, sync_hash, is_encrypted, encryption_key_id 
         FROM categories ORDER BY created_at ASC",
        ()
    ).await?;

    let mut categories = Vec::new();
    while let Some(row) = rows.next().await? {
        let category = Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            version: row.get(5)?,
            last_synced_at: row.get(6)?,
            sync_hash: row.get(7)?,
            is_encrypted: row.get(8)?,
            encryption_key_id: row.get(9)?,
        };
        categories.push(category);
    }
    Ok(categories)
}

/// 根据ID获取分类
pub async fn get_category_by_id(conn: &DbConnection, category_id: &str) -> Result<Option<Category>> {
    let mut rows = conn.query(
        "SELECT id, name, parent_id, created_at, updated_at, version, last_synced_at, sync_hash, is_encrypted, encryption_key_id 
         FROM categories WHERE id = ?1",
        params![category_id]
    ).await?;

    if let Some(row) = rows.next().await? {
        let category = Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            version: row.get(5)?,
            last_synced_at: row.get(6)?,
            sync_hash: row.get(7)?,
            is_encrypted: row.get(8)?,
            encryption_key_id: row.get(9)?,
        };
        Ok(Some(category))
    } else {
        Ok(None)
    }
}

/// 更新分类
pub async fn update_category(conn: &DbConnection, category: &Category) -> Result<()> {
    conn.execute(
        "UPDATE categories SET name = ?1, parent_id = ?2, updated_at = ?3 WHERE id = ?4",
        params![
            category.name.as_str(),
            category.parent_id.as_deref(),
            category.updated_at,
            category.id.as_str()
        ]
    ).await?;
    Ok(())
}

/// 删除分类
pub async fn delete_category(conn: &DbConnection, category_id: &str) -> Result<()> {
    // 启动事务
    conn.execute("BEGIN TRANSACTION", ()).await?;
    
    match delete_category_with_dependencies(conn, category_id).await {
        Ok(_) => {
            conn.execute("COMMIT", ()).await?;
            Ok(())
        }
        Err(e) => {
            conn.execute("ROLLBACK", ()).await?;
            Err(e)
        }
    }
}

/// 删除分类及其依赖关系的内部函数
async fn delete_category_with_dependencies(conn: &DbConnection, category_id: &str) -> Result<()> {
    println!("Starting to delete category {} and all its dependencies", category_id);
    tracing::info!("Starting to delete category {} and all its dependencies", category_id);
    
    // 1. 递归查找所有子分类ID（包括自身）
    let mut all_category_ids = vec![category_id.to_string()];
    let mut stack = vec![category_id.to_string()];
    while let Some(current_id) = stack.pop() {
        let mut rows = conn.query(
            "SELECT id FROM categories WHERE parent_id = ?1",
            params![current_id.as_str()]
        ).await?;
        while let Some(row) = rows.next().await? {
            let child_id: String = row.get(0)?;
            all_category_ids.push(child_id.clone());
            stack.push(child_id);
        }
    }
    
    println!("Found {} categories to delete: {:?}", all_category_ids.len(), all_category_ids);
    tracing::info!("Found {} categories to delete: {:?}", all_category_ids.len(), all_category_ids);

    // 2. 删除所有相关分类下的笔记及其依赖数据
    for cat_id in &all_category_ids {
        // 查找该分类下所有笔记ID
        let mut tip_rows = conn.query(
            "SELECT id FROM tips WHERE category_id = ?1",
            params![cat_id.as_str()]
        ).await?;
        let mut tip_ids: Vec<String> = Vec::new();
        while let Some(row) = tip_rows.next().await? {
            tip_ids.push(row.get(0)?);
        }
        
        println!("Category {} has {} tips to delete", cat_id, tip_ids.len());
        tracing::info!("Category {} has {} tips to delete", cat_id, tip_ids.len());
        
        // 删除所有笔记（及其图片、标签关联等）
        for tip_id in &tip_ids {
            delete_tip_with_dependencies(conn, tip_id).await?;
        }
    }

    // 3. 解除所有子分类的parent_id依赖，避免外键约束失败
    for cat_id in &all_category_ids {
        conn.execute(
            "UPDATE categories SET parent_id = NULL WHERE parent_id = ?1",
            params![cat_id.as_str()]
        ).await?;
    }

    // 4. 删除所有子分类（包括自身）
    for cat_id in &all_category_ids {
        conn.execute(
            "DELETE FROM categories WHERE id = ?1",
            params![cat_id.as_str()]
        ).await?;
        tracing::info!("Deleted category {}", cat_id);
    }

    tracing::info!("Successfully deleted category {} and all its dependencies", category_id);
    Ok(())
}

/// 确保"未分类"分类存在
async fn ensure_uncategorized_exists(conn: &DbConnection) -> Result<()> {
    // 检查是否存在
    let mut rows = conn.query(
        "SELECT id FROM categories WHERE id = 'uncategorized'",
        ()
    ).await?;
    
    if rows.next().await?.is_none() {
        // 如果不存在，创建"未分类"分类
        let now = Utc::now().timestamp_millis();
        conn.execute(
            "INSERT INTO categories (id, name, parent_id, created_at, updated_at, version, last_synced_at) 
             VALUES ('uncategorized', '未分类', NULL, ?1, ?2, 1, 0)",
            params![now, now]
        ).await?;
    }
    
    Ok(())
}

/// 获取指定分类的直接子分类
pub async fn get_subcategories(conn: &DbConnection, parent_id: &str) -> Result<Vec<Category>> {
    let mut rows = conn.query(
        "SELECT id, name, parent_id, created_at, updated_at, version, last_synced_at, sync_hash, is_encrypted, encryption_key_id 
         FROM categories WHERE parent_id = ?1 ORDER BY name",
        params![parent_id]
    ).await?;

    let mut categories = Vec::new();
    while let Some(row) = rows.next().await? {
        let category = Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            version: row.get(5)?,
            last_synced_at: row.get(6)?,
            sync_hash: row.get(7)?,
            is_encrypted: row.get(8)?,
            encryption_key_id: row.get(9)?,
        };
        categories.push(category);
    }
    Ok(categories)
}

/// 递归获取指定分类及其所有子分类的ID列表
pub async fn get_category_ids_recursive(conn: &DbConnection, category_id: &str) -> Result<Vec<String>> {
    let mut category_ids = vec![category_id.to_string()];
    let mut to_process = vec![category_id.to_string()];

    while let Some(current_id) = to_process.pop() {
        let subcategories = get_subcategories(conn, &current_id).await?;
        for subcategory in subcategories {
            category_ids.push(subcategory.id.clone());
            to_process.push(subcategory.id);
        }
    }

    Ok(category_ids)
}

/// 按分类ID查询笔记
pub async fn get_tips_by_category(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at,
                version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content 
         FROM tips WHERE category_id = ?1 ORDER BY updated_at DESC",
        params![category_id]
    ).await?;

    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        let tip = Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: tip_type_str.try_into()?,
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            version: row.get(8)?,
            last_synced_at: row.get(9)?,
            sync_hash: row.get(10)?,
            is_encrypted: row.get(11)?,
            encryption_key_id: row.get(12)?,
            encrypted_content: row.get(13)?,
        };
        tips.push(tip);
    }
    Ok(tips)
}

/// 递归查询分类及其子分类下的所有笔记
pub async fn get_tips_by_category_recursive(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    // 获取所有相关的分类ID（包括自己和所有子分类）
    let category_ids = get_category_ids_recursive(conn, category_id).await?;
    
    if category_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 使用简单的方法：依次查询每个分类的笔记
    let mut all_tips = Vec::new();
    for id in category_ids {
        let tips = get_tips_by_category(conn, &id).await?;
        all_tips.extend(tips);
    }

    // 按更新时间排序
    all_tips.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    Ok(all_tips)
}

/// 分页查询分类下的笔记
pub async fn get_tips_by_category_paged(conn: &DbConnection, category_id: &str, limit: i32, offset: i32) -> Result<Vec<Tip>> {
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at,
                version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content 
         FROM tips WHERE category_id = ?1 ORDER BY updated_at DESC LIMIT ?2 OFFSET ?3",
        params![category_id, limit, offset]
    ).await?;

    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        let tip = Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: tip_type_str.try_into()?,
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            version: row.get(8)?,
            last_synced_at: row.get(9)?,
            sync_hash: row.get(10)?,
            is_encrypted: row.get(11)?,
            encryption_key_id: row.get(12)?,
            encrypted_content: row.get(13)?,
        };
        tips.push(tip);
    }
    Ok(tips)
}

/// 计算分类下的笔记总数
pub async fn count_tips_by_category(conn: &DbConnection, category_id: &str) -> Result<i64> {
    let count: i64 = conn.query(
        "SELECT COUNT(*) FROM tips WHERE category_id = ?1",
        params![category_id]
    ).await?
    .next().await?
    .unwrap()
    .get(0)?;
    
    Ok(count)
}

/// 递归计算分类及其子分类下的笔记总数
pub async fn count_tips_by_category_recursive(conn: &DbConnection, category_id: &str) -> Result<i64> {
    let category_ids = get_category_ids_recursive(conn, category_id).await?;
    
    if category_ids.is_empty() {
        return Ok(0);
    }

    let mut total_count = 0;
    for id in category_ids {
        let count = count_tips_by_category(conn, &id).await?;
        total_count += count;
    }
    
    Ok(total_count)
}

/// 递归分页查询单个分类及其子分类下的笔记
pub async fn get_tips_by_category_recursive_paged_single(conn: &DbConnection, category_id: &str, limit: i32, offset: i32) -> Result<Vec<Tip>> {
    // 获取所有相关的分类ID（包括自己和所有子分类）
    let category_ids = get_category_ids_recursive(conn, category_id).await?;
    
    if category_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 收集所有分类的笔记并按更新时间排序
    let mut all_tips = Vec::new();
    for id in category_ids {
        let tips = get_tips_by_category(conn, &id).await?;
        all_tips.extend(tips);
    }

    // 按更新时间排序
    all_tips.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    // 应用分页
    let start = offset as usize;
    let end = std::cmp::min(start + limit as usize, all_tips.len());
    
    if start >= all_tips.len() {
        return Ok(Vec::new());
    }
    
    Ok(all_tips[start..end].to_vec())
}

/// 递归分页查询多个分类下的笔记（用于根目录）
pub async fn get_tips_by_category_recursive_paged(conn: &DbConnection, category_ids: &[&str], limit: i32, offset: i32) -> Result<Vec<Tip>> {
    if category_ids.is_empty() {
        return Ok(Vec::new());
    }

    // 收集所有指定分类及其子分类的笔记
    let mut all_tips = Vec::new();
    for &category_id in category_ids {
        let tips = get_tips_by_category_recursive(conn, category_id).await?;
        all_tips.extend(tips);
    }

    // 按更新时间排序并去重（可能同一笔记在多个分类路径中）
    all_tips.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    all_tips.dedup_by(|a, b| a.id == b.id);
    
    // 应用分页
    let start = offset as usize;
    let end = std::cmp::min(start + limit as usize, all_tips.len());
    
    if start >= all_tips.len() {
        return Ok(Vec::new());
    }
    
    Ok(all_tips[start..end].to_vec())
}

// ===============================================
// 标签相关数据库操作函数
// ===============================================

/// 创建标签
pub async fn create_tag(conn: &DbConnection, tag: &Tag) -> Result<()> {
    conn.execute(
        "INSERT INTO tags (id, name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4)",
        params![tag.id.as_str(), tag.name.as_str(), tag.created_at, tag.updated_at]
    ).await?;
    Ok(())
}

/// 获取所有标签
pub async fn list_tags(conn: &DbConnection) -> Result<Vec<Tag>> {
    let mut rows = conn.query(
        "SELECT id, name, created_at, updated_at, version, last_synced_at, sync_hash 
         FROM tags ORDER BY name",
        ()
    ).await?;

    let mut tags = Vec::new();
    while let Some(row) = rows.next().await? {
        let tag = Tag {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            updated_at: row.get(3)?,
            version: row.get(4)?,
            last_synced_at: row.get(5)?,
            sync_hash: row.get(6)?,
        };
        tags.push(tag);
    }
    Ok(tags)
}

/// 更新标签
pub async fn update_tag(conn: &DbConnection, tag: &Tag) -> Result<()> {
    conn.execute(
        "UPDATE tags SET name = ?1, updated_at = ?2 WHERE id = ?3",
        params![tag.name.as_str(), tag.updated_at, tag.id.as_str()]
    ).await?;
    Ok(())
}

/// 删除标签
pub async fn delete_tag(conn: &DbConnection, tag_id: &str) -> Result<()> {
    conn.execute("DELETE FROM tags WHERE id = ?1", params![tag_id]).await?;
    Ok(())
}

// ===============================================
// 图片相关数据库操作函数
// ===============================================

/// 保存图片到数据库
pub async fn save_tip_image(
    conn: &DbConnection, 
    tip_id: &str, 
    image_id: &str, 
    image_data: &str
) -> Result<()> {
    let now = Utc::now().timestamp_millis();
    
    // 解析图片数据获取格式和大小信息
    let (image_format, file_size, _width, _height) = parse_image_data(image_data)?;
    
    conn.execute(
        "INSERT OR REPLACE INTO tip_images 
         (id, tip_id, image_id, image_data, image_format, file_size, width, height, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            Uuid::new_v4().to_string(),
            tip_id,
            image_id,
            image_data,
            image_format,
            file_size,
            None::<i32>,
            None::<i32>,
            now,
            now
        ]
    ).await?;
    
    Ok(())
}

/// 获取笔记的所有图片
pub async fn get_tip_images(conn: &DbConnection, tip_id: &str) -> Result<Vec<(String, String)>> {
    let mut rows = conn.query(
        "SELECT image_id, image_data FROM tip_images WHERE tip_id = ?1 ORDER BY created_at",
        params![tip_id]
    ).await?;

    let mut images = Vec::new();
    while let Some(row) = rows.next().await? {
        let image_id: String = row.get(0)?;
        let image_data: String = row.get(1)?;
        images.push((image_id, image_data));
    }
    Ok(images)
}

/// 分页获取笔记图片
pub async fn get_tip_images_paginated(
    conn: &DbConnection, 
    tip_id: &str, 
    limit: i32, 
    offset: i32
) -> Result<Vec<(String, String)>> {
    let mut rows = conn.query(
        "SELECT image_id, image_data FROM tip_images WHERE tip_id = ?1 
         ORDER BY created_at LIMIT ?2 OFFSET ?3",
        params![tip_id, limit, offset]
    ).await?;

    let mut images = Vec::new();
    while let Some(row) = rows.next().await? {
        let image_id: String = row.get(0)?;
        let image_data: String = row.get(1)?;
        images.push((image_id, image_data));
    }
    Ok(images)
}

/// 获取笔记图片总数
pub async fn get_tip_images_count(conn: &DbConnection, tip_id: &str) -> Result<i64> {
    let count: i64 = conn.query(
        "SELECT COUNT(*) FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?
    .next().await?
    .unwrap()
    .get(0)?;
    
    Ok(count)
}

/// 删除单个图片
pub async fn delete_tip_image(conn: &DbConnection, image_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM tip_images WHERE image_id = ?1",
        params![image_id]
    ).await?;
    Ok(())
}

/// 删除笔记的所有图片
pub async fn delete_all_tip_images(conn: &DbConnection, tip_id: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM tip_images WHERE tip_id = ?1",
        params![tip_id]
    ).await?;
    Ok(())
}

/// 获取单个图片信息
pub async fn get_tip_image(conn: &DbConnection, image_id: &str) -> Result<Option<(String, String, String, i64)>> {
    let mut rows = conn.query(
        "SELECT tip_id, image_data, image_format, file_size FROM tip_images WHERE image_id = ?1",
        params![image_id]
    ).await?;

    if let Some(row) = rows.next().await? {
        Ok(Some((
            row.get(0)?, // tip_id
            row.get(1)?, // image_data
            row.get(2)?, // image_format
            row.get(3)?, // file_size
        )))
    } else {
        Ok(None)
    }
}

/// 解析图片数据格式和大小
fn parse_image_data(base64_data: &str) -> Result<(String, i64, Option<i32>, Option<i32>)> {
    // 简单的格式检测
    let format = if base64_data.starts_with("data:image/png") {
        "png"
    } else if base64_data.starts_with("data:image/jpeg") || base64_data.starts_with("data:image/jpg") {
        "jpg"
    } else if base64_data.starts_with("data:image/gif") {
        "gif"
    } else if base64_data.starts_with("data:image/webp") {
        "webp"
    } else {
        "unknown"
    };

    // 估算文件大小（Base64编码大约比原文件大33%）
    let base64_part = if let Some(comma_pos) = base64_data.find(',') {
        &base64_data[comma_pos + 1..]
    } else {
        base64_data
    };
    
    let estimated_size = (base64_part.len() as f64 * 0.75) as i64;

    Ok((format.to_string(), estimated_size, None, None))
}

// ===============================================
// 应用设置相关数据库操作函数
// ===============================================

/// 获取应用设置
pub async fn get_setting(conn: &DbConnection, key: &str) -> Result<Option<String>> {
    let mut rows = conn.query(
        "SELECT value FROM app_settings WHERE key = ?1",
        params![key]
    ).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

/// 保存应用设置
pub async fn save_setting(conn: &DbConnection, key: &str, value: &str) -> Result<()> {
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value, created_at, updated_at) VALUES (?1, ?2, ?3, ?4)",
        params![key, value, now, now],
    ).await?;
    
    Ok(())
}

// ===============================================
// 同步配置相关数据库操作函数
// ===============================================

/// 获取同步配置
pub async fn get_sync_config(conn: &DbConnection) -> Result<Option<SyncConfig>> {
    let mut rows = conn.query(
        "SELECT id, remote_url, auth_token, sync_mode, sync_interval, last_sync_at, is_online, auto_sync_enabled, created_at, updated_at 
         FROM sync_config WHERE id = 'default'",
        ()
    ).await?;
    
    if let Some(row) = rows.next().await? {
        let sync_mode_str: String = row.get(3)?;
        Ok(Some(SyncConfig {
            id: row.get(0)?,
            remote_url: row.get(1)?,
            auth_token: row.get(2)?,
            sync_mode: SyncMode::from(sync_mode_str),
            sync_interval: row.get(4)?,
            last_sync_at: row.get(5)?,
            is_online: row.get(6)?,
            auto_sync_enabled: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        }))
    } else {
        Ok(None)
    }
}

/// 保存同步配置
pub async fn save_sync_config(conn: &DbConnection, config: &SyncConfig) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO sync_config 
         (id, remote_url, auth_token, sync_mode, sync_interval, last_sync_at, is_online, auto_sync_enabled, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            config.id.as_str(),
            config.remote_url.as_deref(),
            config.auth_token.as_deref(),
            config.sync_mode.to_string(),
            config.sync_interval,
            config.last_sync_at,
            config.is_online,
            config.auto_sync_enabled,
            config.created_at,
            config.updated_at
        ]
    ).await?;
    Ok(())
} 

// ===============================================
// 剪贴板相关数据库操作函数
// ===============================================

/// 添加剪贴板条目
pub async fn add_clipboard_entry(conn: &DbConnection, content: &str, source: Option<&str>) -> Result<()> {
    let now = Utc::now().timestamp_millis();
    
    conn.execute(
        "INSERT INTO clipboard_history (content, source, created_at) VALUES (?1, ?2, ?3)",
        params![content, source, now]
    ).await?;
    
    Ok(())
}

/// 检查剪贴板条目是否存在
pub async fn check_clipboard_entry_exists(conn: &DbConnection, content: &str) -> Result<bool> {
    let mut rows = conn.query(
        "SELECT COUNT(*) FROM clipboard_history WHERE content = ?1",
        params![content]
    ).await?;
    
    if let Some(row) = rows.next().await? {
        let count: i64 = row.get(0)?;
        Ok(count > 0)
    } else {
        Ok(false)
    }
}

/// 删除过期的剪贴板条目
pub async fn delete_expired_clipboard_entries(conn: &DbConnection, expire_timestamp: i64) -> Result<()> {
    let expire_timestamp_ms = expire_timestamp * 1000; // 转换为毫秒
    
    conn.execute(
        "DELETE FROM clipboard_history WHERE created_at < ?1",
        params![expire_timestamp_ms]
    ).await?;
    
    Ok(())
}

/// 获取剪贴板历史记录
pub async fn get_clipboard_history(conn: &DbConnection, limit: Option<i32>) -> Result<Vec<ClipboardHistory>> {
    let sql = if let Some(limit) = limit {
        format!("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC LIMIT {}", limit)
    } else {
        "SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC".to_string()
    };
    
    let mut rows = conn.query(&sql, ()).await?;
    let mut history = Vec::new();
    
    while let Some(row) = rows.next().await? {
        history.push(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        });
    }
    
    Ok(history)
}

/// 搜索笔记 - 优化版本
pub async fn search_tips(conn: &DbConnection, query: &str) -> Result<Vec<Tip>> {
    search_tips_with_limit(conn, query, 50).await
}

/// 搜索笔记（带限制）- 性能优化版本
pub async fn search_tips_with_limit(conn: &DbConnection, query: &str, limit: i32) -> Result<Vec<Tip>> {
    let search_pattern = format!("%{}%", query);
    
    // 优化的查询：
    // 1. 添加LIMIT减少结果数量
    // 2. 优先匹配标题，然后是内容
    // 3. 使用CASE WHEN进行相关性排序
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at,
                version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content,
                CASE 
                    WHEN title LIKE ?1 THEN 1 
                    WHEN content LIKE ?1 THEN 2 
                    ELSE 3 
                END as relevance_score
         FROM tips 
         WHERE title LIKE ?1 OR content LIKE ?1 
         ORDER BY relevance_score ASC, updated_at DESC
         LIMIT ?2",
        params![search_pattern, limit]
    ).await?;

    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        let tip = Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: tip_type_str.try_into()?,
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
            version: row.get(8)?,
            last_synced_at: row.get(9)?,
            sync_hash: row.get(10)?,
            is_encrypted: row.get(11)?,
            encryption_key_id: row.get(12)?,
            encrypted_content: row.get(13)?,
        };
        tips.push(tip);
    }
    Ok(tips)
}

/// 快速搜索笔记摘要 - 只返回必要字段
pub async fn search_tips_summary_fast(conn: &DbConnection, query: &str, limit: i32) -> Result<Vec<TipSummary>> {
    let search_pattern = format!("%{}%", query);
    
    // 只查询摘要需要的字段，大幅减少数据传输
    let mut rows = conn.query(
        "SELECT id, title, tip_type, language, category_id, created_at, updated_at, is_encrypted,
                CASE 
                    WHEN title LIKE ?1 THEN 1 
                    WHEN content LIKE ?1 THEN 2 
                    ELSE 3 
                END as relevance_score,
                substr(content, 1, 200) as content_preview
         FROM tips 
         WHERE title LIKE ?1 OR content LIKE ?1 
         ORDER BY relevance_score ASC, updated_at DESC
         LIMIT ?2",
        params![search_pattern, limit]
    ).await?;

    let mut summaries = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(2)?;
        let summary = TipSummary {
            id: row.get(0)?,
            title: row.get(1)?,
            tip_type: tip_type_str,
            language: row.get(3)?,
            category_id: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
            tags: Vec::new(), // 搜索时不加载标签，提高速度
            is_encrypted: row.get(7)?,
            content: Some(row.get(9)?), // 只返回前200字符作为预览
        };
        summaries.push(summary);
    }
    Ok(summaries)
}

/// 删除剪贴板条目
pub async fn delete_clipboard_entry(conn: &DbConnection, entry_id: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM clipboard_history WHERE id = ?1",
        params![entry_id]
    ).await?;
    
    Ok(())
}

/// 清空所有剪贴板历史记录
pub async fn clear_clipboard_history(conn: &DbConnection) -> Result<()> {
    conn.execute("DELETE FROM clipboard_history", ()).await?;
    Ok(())
}

 