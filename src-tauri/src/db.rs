use anyhow::{anyhow, Result};
use chrono::Utc;
use libsql::{params, Connection, Database, Builder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use uuid::Uuid;
use tauri::{AppHandle, Manager};
use tokio::sync::Mutex;

// 导入加密状态结构
use crate::api::encryption::EncryptionStatus;

// 笔记类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TipType {
    Text,
    Markdown,
    Code,
    AIPrompt,
}

// 从字符串转换为TipType
impl TryFrom<String> for TipType {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "text" => Ok(TipType::Text),
            "markdown" => Ok(TipType::Markdown),
            "code" => Ok(TipType::Code),
            "ai-prompt" => Ok(TipType::AIPrompt),
            _ => Err(anyhow!("Invalid tip type: {}", s)),
        }
    }
}

// 转换为字符串
impl From<TipType> for String {
    fn from(tip_type: TipType) -> Self {
        match tip_type {
            TipType::Text => "text".to_string(),
            TipType::Markdown => "markdown".to_string(),
            TipType::Code => "code".to_string(),
            TipType::AIPrompt => "ai-prompt".to_string(),
        }
    }
}

// 笔记模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tip {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tip_type: TipType,
    pub language: Option<String>,
    pub category_id: Option<String>,
    pub created_at: i64, // 毫秒级时间戳
    pub updated_at: i64, // 毫秒级时间戳
}

// 分类模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

// 标签模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
}

// 笔记标签关联
#[derive(Debug, Serialize, Deserialize)]
pub struct TipTag {
    pub tip_id: String,
    pub tag_id: String,
}

// 剪贴板历史记录模型
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardHistory {
    pub id: i64,
    pub content: String,
    pub source: Option<String>,
    pub created_at: i64,
}

// AI角色模型
#[derive(Debug, Serialize, Deserialize)]
pub struct AIRole {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// 数据库连接池别名  
pub type DbPool = Arc<Database>;
pub type DbConnection = Connection;

// 数据库管理器
pub struct DbManager {
    pub database: DbPool,
}

impl DbManager {
    // 初始化数据库连接池
    pub async fn init(app_handle: AppHandle) -> Result<Self> {
        let db_path = get_db_path(&app_handle)?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let db = Builder::new_local(db_path).build().await?;
        let database = Arc::new(db);

        // 初始化 schema
        let conn = database.connect()?;
        init_schema(&conn).await?;
        
        // 可以在这里初始化默认数据
        init_default_roles(&conn).await?;
        
        // 初始化同步相关表结构
        init_sync_schema(&conn).await?;

        Ok(Self { database })
    }

    // 从池中获取一个连接
    pub async fn get_conn(&self) -> Result<DbConnection> {
        self.database.connect()
            .map_err(|e| anyhow!("Failed to get db connection: {}", e))
    }
}

/// 使用给定的连接初始化数据库 schema
async fn init_schema(conn: &Connection) -> Result<()> {
    // 启用外键约束
    conn.execute("PRAGMA foreign_keys = ON", ()).await?;

    // 创建 tips 表
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
        (),
    ).await?;
    
    // 创建 categories 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT
        )",
        (),
    ).await?;

    // 创建 ai_conversations 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            model TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建 ai_messages 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_messages (
            id TEXT PRIMARY KEY,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            timestamp INTEGER NOT NULL,
            FOREIGN KEY(conversation_id) REFERENCES ai_conversations(id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建 tags 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        (),
    ).await?;

    // 创建 tip_tags 表
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

    // 创建 clipboard_history 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            source TEXT,
            created_at INTEGER NOT NULL
        )",
        (),
    ).await?;

    // 创建设置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        (),
    ).await?;

    // 创建 ai_roles 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ai_roles (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        )",
            (),
    ).await?;

    // 创建 images 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id TEXT PRIMARY KEY,
            tip_id TEXT NOT NULL,
            image_data TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE
        )",
        (),
    ).await?;

    // 创建 encryption_status 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS encryption_status (
            id TEXT PRIMARY KEY,
            item_type TEXT NOT NULL,
            item_id TEXT NOT NULL,
            is_encrypted BOOLEAN DEFAULT FALSE,
            is_unlocked BOOLEAN DEFAULT FALSE,
            created_at INTEGER DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER DEFAULT (strftime('%s', 'now')),
            UNIQUE(item_type, item_id)
        )",
        (),
    ).await?;

    // 创建FTS5虚拟表用于全文搜索
    conn.execute("DROP TRIGGER IF EXISTS tips_after_insert", ()).await?;
    conn.execute("DROP TRIGGER IF EXISTS tips_after_delete", ()).await?;
    conn.execute("DROP TRIGGER IF EXISTS tips_after_update", ()).await?;
    conn.execute("DROP TABLE IF EXISTS tips_fts", ()).await?;
    
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS tips_fts USING fts5(
            tip_id UNINDEXED, 
            title, 
            content, 
            tokenize = 'porter unicode61'
        )",
        (),
    ).await?;

    // 创建触发器，用于在原始表数据发生变化时同步更新FTS表
    // 在插入后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_insert AFTER INSERT ON tips BEGIN
            INSERT INTO tips_fts(tip_id, title, content) VALUES (new.id, new.title, new.content);
        END;",
        (),
    ).await?;
    // 在删除后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_delete AFTER DELETE ON tips BEGIN
            DELETE FROM tips_fts WHERE tip_id = old.id;
        END;",
        (),
    ).await?;
    // 在更新后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_update AFTER UPDATE ON tips BEGIN
            DELETE FROM tips_fts WHERE tip_id = old.id;
            INSERT INTO tips_fts(tip_id, title, content) VALUES (new.id, new.title, new.content);
        END;",
        (),
    ).await?;

    // 检查FTS表是否为空，如果为空，则从主表同步数据
    let fts_count: i64 = {
        let mut rows = conn.query("SELECT COUNT(*) FROM tips_fts", ()).await?;
        if let Some(row) = rows.next().await? {
            row.get::<i64>(0)?
        } else {
            0
        }
    };
    if fts_count == 0 {
        conn.execute(
            "INSERT INTO tips_fts(tip_id, title, content) SELECT id, title, content FROM tips",
            (),
        ).await?;
    }

    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tips_category_id ON tips(category_id)",
        (),
    ).await?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tip_tags_tip_id ON tip_tags(tip_id)",
        (),
    ).await?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tip_tags_tag_id ON tip_tags(tag_id)",
            (),
    ).await?;

    // 创建一些默认数据（如果是新数据库）
    let count: i64 = {
        let mut rows = conn.query("SELECT COUNT(*) FROM categories", ()).await?;
        if let Some(row) = rows.next().await? {
            row.get::<i64>(0)?
        } else {
            0
        }
    };

    if count == 0 {
        // 插入默认分类
        let categories = vec![
            Category {
                id: Uuid::new_v4().to_string(),
                name: "未分类".to_string(),
                parent_id: None,
            },
        ];

        for category in categories {
            conn.execute(
                "INSERT INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
                params![category.id, category.name, category.parent_id],
            ).await?;
        }
    }

    Ok(())
}

/// 公开的函数，用于创建数据库表结构
pub async fn create_tables(conn: &Connection) -> Result<()> {
    init_schema(conn).await
}

async fn init_default_roles(conn: &Connection) -> Result<()> {
    let count: i64 = {
        let mut rows = conn.query("SELECT COUNT(*) FROM ai_roles", ()).await?;
        if let Some(row) = rows.next().await? {
            row.get::<i64>(0)?
        } else {
            0
        }
    };
    if count == 0 {
        let roles = vec![
            ("default", "默认助手", "一个通用的AI助手，可以回答各种问题。"),
            ("code", "代码助手", "专注于生成和解释代码的AI助手。"),
            ("translator", "翻译助手", "一个强大的翻译工具，支持多种语言。"),
        ];

        for (id, name, description) in roles {
            let now = Utc::now().timestamp_millis();
            conn.execute(
                "INSERT INTO ai_roles (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                params![id, name, description, now, now],
            ).await?;
        }
    }
    Ok(())
}

// ===============================================
// Database Operations (as standalone functions)
// ===============================================

// 获取所有笔记
pub async fn get_all_tips(conn: &DbConnection) -> Result<Vec<Tip>> {
    let mut rows = conn.query("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC", ()).await?;
    let mut tips = Vec::new();
    
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        tips.push(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }
    
    Ok(tips)
}

// 分页获取笔记
pub async fn get_tips_paged(conn: &DbConnection, page: i64, page_size: i64) -> Result<Vec<Tip>> {
    let offset = (page - 1) * page_size;
    let mut rows = conn.query("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2", params![page_size, offset]).await?;
    let mut tips = Vec::new();
    
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        tips.push(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }
    
    Ok(tips)
}

// 获取单个笔记
pub async fn get_tip(conn: &DbConnection, id: &str) -> Result<Tip> {
    let mut rows = conn.query("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips WHERE id = ?1", params![id]).await?;
    
    if let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        Ok(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    } else {
        Err(anyhow!("Tip not found"))
    }
}

// 保存/更新笔记
pub async fn save_tip(conn: &DbConnection, mut tip: Tip) -> Result<Tip> {
    tip.updated_at = Utc::now().timestamp_millis();
    let tip_type_str: String = tip.tip_type.into();

    conn.execute(
        "INSERT INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
         ON CONFLICT(id) DO UPDATE SET
           title = excluded.title,
           content = excluded.content,
           tip_type = excluded.tip_type,
           language = excluded.language,
           category_id = excluded.category_id,
           updated_at = excluded.updated_at",
        params![
            tip.id.clone(),
            tip.title.clone(),
            tip.content.clone(),
            tip_type_str.clone(),
            tip.language.clone(),
            tip.category_id.clone(),
            tip.created_at,
            tip.updated_at
        ],
    ).await?;

    Ok(tip)
}

// 删除笔记
pub async fn delete_tip(conn: &DbConnection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM tips WHERE id = ?", params![id]).await?;
    Ok(())
}

// 获取所有分类
pub async fn get_all_categories(conn: &DbConnection) -> Result<Vec<Category>> {
    let mut rows = conn.query("SELECT id, name, parent_id FROM categories", ()).await?;
    let mut categories = Vec::new();
    
    while let Some(row) = rows.next().await? {
        categories.push(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        });
    }
    
    Ok(categories)
}

// 创建分类
pub async fn create_category(conn: &DbConnection, name: &str, parent_id: Option<&str>) -> Result<Category> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
        params![id.clone(), name, parent_id],
    ).await?;

    Ok(Category {
        id,
        name: name.to_string(),
        parent_id: parent_id.map(|s| s.to_string()),
    })
}

// 更新分类
pub async fn update_category(conn: &DbConnection, id: &str, name: &str) -> Result<Category> {
    conn.execute(
        "UPDATE categories SET name = ? WHERE id = ?",
        params![name, id],
    ).await?;

    Ok(Category {
        id: id.to_string(),
        name: name.to_string(),
        parent_id: None,
    })
}

// 删除分类
pub async fn delete_category(conn: &DbConnection, id: &str) -> Result<()> {
    // 首先将该分类下的笔记的category_id设为null
    conn.execute(
        "UPDATE tips SET category_id = NULL WHERE category_id = ?",
        params![id],
    ).await?;

    // 然后删除分类
    conn.execute("DELETE FROM categories WHERE id = ?", params![id]).await?;

    Ok(())
}

// 递归删除分类及其所有子分类和笔记
pub fn delete_category_recursive<'a>(conn: &'a DbConnection, id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
    // 1. 找到所有子分类
    let mut rows = conn.query("SELECT id FROM categories WHERE parent_id = ?1", params![id]).await?;
    let mut child_ids: Vec<String> = Vec::new();
    
    while let Some(row) = rows.next().await? {
        child_ids.push(row.get(0)?);
    }
    
    // 2. 递归删除所有子分类
    for child_id in child_ids {
        delete_category_recursive(conn, &child_id).await?;
    }

    // 3. 删除该分类下的所有笔记
    conn.execute("DELETE FROM tips WHERE category_id = ?", params![id]).await?; 

    // 4. 删除该分类
    conn.execute("DELETE FROM categories WHERE id = ?", params![id]).await?;

    Ok(())
    })
}

// 递归获取所有子分类ID（包括自身）
pub fn get_all_subcategory_ids<'a>(conn: &'a DbConnection, category_id: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<String>>> + Send + 'a>> {
    Box::pin(async move {
    let mut all_ids = vec![category_id.to_string()];
    
    // 1. 找到所有直接子分类
    let mut rows = conn.query("SELECT id FROM categories WHERE parent_id = ?1", params![category_id]).await?;
    let mut child_ids: Vec<String> = Vec::new();
    
    while let Some(row) = rows.next().await? {
        child_ids.push(row.get(0)?);
    }

    // 2. 递归获取每个子分类的所有子分类
    for child_id in child_ids {
        let mut child_all_ids = get_all_subcategory_ids(conn, &child_id).await?;
        all_ids.append(&mut child_all_ids);
    }
    
    Ok(all_ids)
    })
}

// 递归获取分类及其所有子分类下的所有笔记
pub async fn get_tips_by_category_recursive(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    // 获取所有子分类ID（包括自身）
    let all_category_ids = get_all_subcategory_ids(conn, category_id).await?;
    
    let mut all_tips = Vec::new();
    
    // 获取每个分类下的所有笔记
    for cat_id in all_category_ids {
        let mut tips = get_tips_by_category(conn, &cat_id).await?;
        all_tips.append(&mut tips);
    }
    
    Ok(all_tips)
}

// 获取所有标签
pub async fn get_all_tags(conn: &DbConnection) -> Result<Vec<Tag>> {
    let mut rows = conn.query("SELECT id, name FROM tags", ()).await?;
    let mut tags = Vec::new();
    
    while let Some(row) = rows.next().await? {
        tags.push(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    }
    
    Ok(tags)
}

// 创建标签
pub async fn create_tag(conn: &DbConnection, name: &str) -> Result<Tag> {
    let id = Uuid::new_v4().to_string();

    // 先检查是否已存在
    let existing = {
        let mut rows = conn.query("SELECT id FROM tags WHERE name = ?1", params![name]).await?;
        if let Some(row) = rows.next().await? {
            Some(row.get::<String>(0)?)
        } else {
            None
        }
    };

    match existing {
        Some(existing_id) => Ok(Tag {
            id: existing_id,
            name: name.to_string(),
        }),
        None => {
            conn.execute(
                "INSERT INTO tags (id, name) VALUES (?, ?)",
                params![id.clone(), name],
            ).await?;
            Ok(Tag {
                id,
                name: name.to_string(),
            })
        }
    }
}

// 为笔记设置标签
pub async fn set_tip_tags(conn: &DbConnection, tip_id: &str, tag_ids: &[String]) -> Result<()> {
    // 先删除现有关联
    conn.execute("DELETE FROM tip_tags WHERE tip_id = ?", params![tip_id]).await?;

    // 添加新关联
    for tag_id in tag_ids {
        conn.execute(
            "INSERT INTO tip_tags (tip_id, tag_id) VALUES (?, ?)",
            params![tip_id, tag_id.clone()],
        ).await?;
    }
    Ok(())
}

// 删除标签
pub async fn delete_tag(conn: &DbConnection, id: &str) -> Result<()> {
    // 删除标签关联
    conn.execute("DELETE FROM tip_tags WHERE tag_id = ?", params![id]).await?;

    // 删除标签
    conn.execute("DELETE FROM tags WHERE id = ?", params![id]).await?;

    Ok(())
}

// 获取笔记的所有标签
pub async fn get_tip_tags(conn: &DbConnection, tip_id: &str) -> Result<Vec<Tag>> {
    let mut rows = conn.query(
        "SELECT t.id, t.name FROM tags t
         JOIN tip_tags tt ON t.id = tt.tag_id
         WHERE tt.tip_id = ?",
        params![tip_id]
    ).await?;
    
    let mut tags = Vec::new();
    while let Some(row) = rows.next().await? {
        tags.push(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        });
    }
    Ok(tags)
}

// 搜索笔记
pub async fn search_tips(conn: &DbConnection, query: &str) -> Result<Vec<Tip>> {
    let search_query = format!("%{}%", query);
    let mut rows = conn.query(
        "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
         FROM tips t
         LEFT JOIN tags tg ON t.id = tg.id
         WHERE t.title LIKE ?1 OR t.content LIKE ?1 OR tg.name LIKE ?1
         GROUP BY t.id
         ORDER BY t.updated_at DESC",
        params![search_query]
    ).await?;
    
    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        tips.push(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }
    
    Ok(tips)
}

// 按分类筛选笔记
pub async fn get_tips_by_category(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    let mut rows = conn.query(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at
         FROM tips 
         WHERE category_id = ? 
         ORDER BY updated_at DESC",
        params![category_id]
    ).await?;
    
    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        tips.push(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }
    
    Ok(tips)
}

// 按标签筛选笔记
pub async fn get_tips_by_tag(conn: &DbConnection, tag_id: &str) -> Result<Vec<Tip>> {
    let mut rows = conn.query(
        "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
         FROM tips t
         JOIN tip_tags tt ON t.id = tt.tip_id
         WHERE tt.tag_id = ?
         ORDER BY t.updated_at DESC",
        params![tag_id]
    ).await?;
    
    let mut tips = Vec::new();
    while let Some(row) = rows.next().await? {
        let tip_type_str: String = row.get(3)?;
        tips.push(Tip {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            tip_type: TipType::try_from(tip_type_str).unwrap_or(TipType::Text),
            language: row.get(4)?,
            category_id: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        });
    }
    
    Ok(tips)
}

// 获取分类
pub async fn get_category_by_id(conn: &DbConnection, id: &str) -> Result<Category> {
    let mut rows = conn.query("SELECT id, name, parent_id FROM categories WHERE id = ?", params![id]).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    } else {
        Err(anyhow!("Category not found"))
    }
}

// 保存图片
pub async fn save_image(conn: &DbConnection, tip_id: &str, image_id: &str, image_data: &str) -> Result<()> {
    let now = Utc::now().timestamp_millis();

    // 先检查tip_id是否存在，避免外键约束错误
    let mut rows = conn.query(
        "SELECT EXISTS(SELECT 1 FROM tips WHERE id = ?)",
        params![tip_id],
    ).await?;
    
    let tip_exists: bool = if let Some(row) = rows.next().await? {
        row.get(0)?
    } else {
        false
    };

    if !tip_exists {
        return Err(anyhow!("Tip with id {} not found", tip_id));
    }

    // 保存图片
    conn.execute(
        "INSERT INTO images (id, tip_id, image_data, created_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
           image_data = excluded.image_data",
        params![image_id, tip_id, image_data, now],
    ).await?;
    
    Ok(())
}

// 获取笔记的所有图片
pub async fn get_tip_images(conn: &DbConnection, tip_id: &str) -> Result<Vec<(String, String)>> {
    let mut rows = conn.query(
        "SELECT id, image_data FROM images
         WHERE tip_id = ?
         ORDER BY created_at ASC",
        params![tip_id]
    ).await?;
    
    let mut images = Vec::new();
    while let Some(row) = rows.next().await? {
        images.push((row.get(0)?, row.get(1)?));
    }
    
    Ok(images)
}

// 分页获取笔记的图片
pub async fn get_tip_images_paginated(
    conn: &DbConnection,
    tip_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<(String, String)>> {
    let mut rows = conn.query(
        "SELECT id, image_data FROM images
         WHERE tip_id = ?
         ORDER BY created_at ASC
         LIMIT ? OFFSET ?",
        params![tip_id, limit, offset]
    ).await?;
    
    let mut images = Vec::new();
    while let Some(row) = rows.next().await? {
        images.push((row.get(0)?, row.get(1)?));
    }
    
    Ok(images)
}

// 获取笔记的图片总数
pub async fn get_tip_images_count(conn: &DbConnection, tip_id: &str) -> Result<i64> {
    let mut rows = conn.query(
        "SELECT COUNT(*) FROM images WHERE tip_id = ?",
        params![tip_id],
    ).await?;
    
    let count: i64 = if let Some(row) = rows.next().await? {
        row.get(0)?
    } else {
        0
    };
    
    Ok(count)
}

// 删除笔记的所有图片
pub async fn delete_tip_images(conn: &DbConnection, tip_id: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE tip_id = ?", params![tip_id]).await?;
    Ok(())
}

// 删除单个图片
pub async fn delete_image(conn: &DbConnection, image_id: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE id = ?", params![image_id]).await?;
    Ok(())
}

// --- 剪贴板历史记录相关 ---

// 添加剪贴板条目
pub async fn add_clipboard_entry(conn: &DbConnection, content: &str, source: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT INTO clipboard_history (content, source, created_at) VALUES (?, ?, ?)",
        params![content, source, chrono::Utc::now().timestamp()],
    ).await?;
    Ok(())
}

// 获取所有剪贴板条目（已弃用，保留以备后用）
#[allow(dead_code)]
pub async fn get_all_clipboard_entries(conn: &DbConnection) -> Result<Vec<ClipboardHistory>> {
    let mut rows = conn.query("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC", ()).await?;
    let mut entries = Vec::new();
    
    while let Some(row) = rows.next().await? {
        entries.push(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        });
    }
    
    Ok(entries)
}

// 根据时间戳获取剪贴板条目
pub async fn get_clipboard_entries_since(conn: &DbConnection, since_timestamp: i64) -> Result<Vec<ClipboardHistory>> {
    let mut rows = conn.query("SELECT id, content, source, created_at FROM clipboard_history WHERE created_at >= ? ORDER BY created_at DESC", params![since_timestamp]).await?;
    let mut entries = Vec::new();
    
    while let Some(row) = rows.next().await? {
        entries.push(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        });
    }
    
    Ok(entries)
}

// 分页获取剪贴板条目
pub async fn get_clipboard_entries_paged(
    conn: &DbConnection,
    page: i64,
    page_size: i64,
    query: Option<String>,
) -> Result<Vec<ClipboardHistory>> {
    let offset = (page - 1) * page_size;

    if let Some(q) = query {
        let search_term = format!("%{}%", q);
        let mut rows = conn.query("SELECT id, content, source, created_at FROM clipboard_history WHERE content LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?", params![search_term, page_size, offset]).await?;
        let mut entries = Vec::new();
        
        while let Some(row) = rows.next().await? {
            entries.push(ClipboardHistory {
                id: row.get(0)?,
                content: row.get(1)?,
                source: row.get(2)?,
                created_at: row.get(3)?,
            });
        }
        
        Ok(entries)
    } else {
        let mut rows = conn.query("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC LIMIT ? OFFSET ?", params![page_size, offset]).await?;
        let mut entries = Vec::new();
        
        while let Some(row) = rows.next().await? {
            entries.push(ClipboardHistory {
                id: row.get(0)?,
                content: row.get(1)?,
                source: row.get(2)?,
                created_at: row.get(3)?,
            });
        }
        
        Ok(entries)
    }
}

// 获取剪贴板条目总数
pub async fn get_clipboard_entries_count(conn: &DbConnection, query: Option<String>) -> Result<i64> {
    if let Some(q) = query {
        let search_term = format!("%{}%", q);
        let mut rows = conn.query(
            "SELECT COUNT(*) FROM clipboard_history WHERE content LIKE ?",
            params![search_term],
        ).await?;
        
        if let Some(row) = rows.next().await? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    } else {
        let mut rows = conn.query(
            "SELECT COUNT(*) FROM clipboard_history",
            (),
        ).await?;
        
        if let Some(row) = rows.next().await? {
            Ok(row.get(0)?)
        } else {
            Ok(0)
        }
    }
}

// 删除指定的剪贴板条目
pub async fn delete_clipboard_entries(conn: &DbConnection, ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }

    let params_sql = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("DELETE FROM clipboard_history WHERE id IN ({})", params_sql);
    
    let values: Vec<libsql::Value> = ids.iter().map(|&id| libsql::Value::from(id)).collect();
    conn.execute(&query, values).await?;
    Ok(())
}

// 清除所有剪贴板条目
pub async fn clear_all_clipboard_entries(conn: &DbConnection) -> Result<()> {
    conn.execute("DELETE FROM clipboard_history", ()).await?;
    Ok(())
}

// 删除过期的剪贴板条目
pub async fn delete_expired_clipboard_entries(conn: &DbConnection, expire_timestamp: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM clipboard_history WHERE created_at < ?",
        params![expire_timestamp],
    ).await?;
    Ok(())
}

// 检查剪贴板条目是否已存在
pub async fn check_clipboard_entry_exists(conn: &DbConnection, content: &str) -> Result<bool> {
    // 查询最近30秒内是否有相同内容的条目
    let now = Utc::now().timestamp();
    let thirty_seconds_ago = now - 30; // 30秒内的条目视为重复

    let mut rows = conn.query(
        "SELECT COUNT(*) FROM clipboard_history WHERE content = ? AND created_at > ?",
        params![content, thirty_seconds_ago],
    ).await?;
    
    let count: i64 = if let Some(row) = rows.next().await? {
        row.get::<i64>(0)?
    } else {
        0
    };

    Ok(count > 0)
}

// 获取剪贴板条目ID列表，基于天数
pub async fn get_clipboard_entry_ids_by_days(
    conn: &DbConnection,
    days: u32,
) -> Result<Vec<i64>> {
    let mut rows = conn.query(
        "SELECT id FROM clipboard_history WHERE created_at >= strftime('%s', 'now', ?)",
        params![format!("-{} day", days)],
    ).await?;
    
    let mut ids = Vec::new();
    while let Some(row) = rows.next().await? {
        ids.push(row.get(0)?);
    }
    
    Ok(ids)
}

// 保存设置
pub async fn save_setting(conn: &DbConnection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?, ?) 
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    ).await?;
    Ok(())
}

// 获取设置
pub async fn get_setting(conn: &DbConnection, key: &str) -> Result<Option<String>> {
    let mut rows = conn.query(
        "SELECT value FROM settings WHERE key = ?",
        params![key],
    ).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

// 获取所有以指定前缀开头的设置键
pub async fn get_settings_by_prefix(conn: &DbConnection, prefix: &str) -> Result<Vec<String>> {
    let like_prefix = format!("{}%", prefix);
    let mut rows = conn.query(
        "SELECT key FROM settings WHERE key LIKE ? AND value != ''",
        params![like_prefix]
    ).await?;
    
    let mut keys = Vec::new();
    while let Some(row) = rows.next().await? {
        keys.push(row.get(0)?);
    }
    
    Ok(keys)
}


// --- AI 相关方法 ---

// 获取所有角色
pub async fn get_all_roles(conn: &DbConnection) -> Result<Vec<AIRole>> {
    let mut rows = conn.query("SELECT id, name, description, created_at, updated_at FROM ai_roles ORDER BY created_at DESC", ()).await?;
    let mut roles = Vec::new();
    
    while let Some(row) = rows.next().await? {
        roles.push(AIRole {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        });
    }
    
    Ok(roles)
}

// 创建角色
pub async fn create_role(conn: &DbConnection, name: &str, description: &str) -> Result<AIRole> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "INSERT INTO ai_roles (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        params![id.clone(), name, description, now, now],
    ).await?;
    Ok(AIRole { id, name: name.to_string(), description: description.to_string(), created_at: now, updated_at: now })
}

// 更新角色
pub async fn update_role(conn: &DbConnection, id: &str, name: &str, description: &str) -> Result<AIRole> {
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "UPDATE ai_roles SET name = ?, description = ?, updated_at = ? WHERE id = ?",
        params![name, description, now, id],
    ).await?;
    // This is not ideal as created_at is not known here.
    // A full fetch would be better, but for now we return what we can.
    Ok(AIRole { id: id.to_string(), name: name.to_string(), description: description.to_string(), created_at: 0, updated_at: now })
}

// 删除角色
pub async fn delete_role(conn: &DbConnection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM ai_roles WHERE id = ?", params![id]).await?;
    Ok(())
}

// 根据ID获取角色
pub async fn get_role_by_id(conn: &DbConnection, id: &str) -> Result<AIRole> {
    let mut rows = conn.query(
        "SELECT id, name, description, created_at, updated_at FROM ai_roles WHERE id = ?",
        params![id]
    ).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(AIRole {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    } else {
        Err(anyhow!("Role not found"))
    }
}

// --- 加密相关方法 ---

/// 获取所有加密状态
pub async fn get_encryption_statuses(conn: &DbConnection) -> Result<Vec<EncryptionStatus>> {
    let mut rows = conn.query(
        "SELECT item_type, item_id, is_encrypted, is_unlocked FROM encryption_status",
        ()
    ).await?;
    
    let mut statuses = Vec::new();
    while let Some(row) = rows.next().await? {
        let item_type: String = row.get(0)?;
        let item_id: String = row.get(1)?;
        let is_encrypted: bool = row.get(2)?;
        let is_unlocked: bool = row.get(3)?;
        
        statuses.push(EncryptionStatus {
            note_id: if item_type == "note" { Some(item_id.clone()) } else { None },
            notebook_id: if item_type == "notebook" { Some(item_id) } else { None },
            is_encrypted,
            is_unlocked,
        })
    }
    
    Ok(statuses)
}

/// 检查项目是否已加密
pub async fn is_item_encrypted(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<bool> {
    let mut rows = conn.query(
        "SELECT is_encrypted FROM encryption_status WHERE item_id = ? AND item_type = ?",
        params![item_id, item_type],
    ).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(row.get(0)?)
    } else {
        Ok(false) // 如果没有记录，则未加密
    }
}

/// 检查项目是否已解锁
pub async fn is_item_unlocked(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<bool> {
    let mut rows = conn.query(
        "SELECT is_unlocked FROM encryption_status WHERE item_id = ? AND item_type = ?",
        params![item_id, item_type],
    ).await?;
    
    if let Some(row) = rows.next().await? {
        Ok(row.get(0)?)
    } else {
        Ok(false) // 如果没有记录，则未解锁
    }
}

/// 加密笔记
pub async fn encrypt_note(conn: &DbConnection, note_id: &str, encrypted_content: &str) -> Result<()> {
    // 更新笔记内容为加密内容
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
        params![encrypted_content, now, note_id],
    ).await?;

    // 更新或插入加密状态
    let status_id = Uuid::new_v4().to_string();
    let now_seconds = Utc::now().timestamp();
    conn.execute(
        "INSERT INTO encryption_status (id, item_type, item_id, is_encrypted, is_unlocked, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(item_type, item_id) DO UPDATE SET
           is_encrypted = excluded.is_encrypted,
           is_unlocked = excluded.is_unlocked,
           updated_at = excluded.updated_at",
        params![status_id, "note", note_id, true, false, now_seconds, now_seconds],
    ).await?;

    Ok(())
}

/// 解密笔记
pub async fn decrypt_note(conn: &DbConnection, note_id: &str, decrypted_content: &str) -> Result<()> {
    // 更新笔记内容为解密内容
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
        params![decrypted_content, now, note_id],
    ).await?;

    // 更新加密状态
    let now_seconds = Utc::now().timestamp();
    conn.execute(
        "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
         WHERE item_id = ? AND item_type = ?",
        params![false, true, now_seconds, note_id, "note"],
    ).await?;

    Ok(())
}

/// 标记项目为已解锁（会话级别）
pub async fn mark_item_unlocked(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<()> {
    let now_seconds = Utc::now().timestamp();
    
    // 如果记录不存在，创建一个新记录
    let status_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO encryption_status (id, item_type, item_id, is_encrypted, is_unlocked, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(item_type, item_id) DO UPDATE SET
           is_unlocked = excluded.is_unlocked,
           updated_at = excluded.updated_at",
        params![status_id, item_type, item_id, false, true, now_seconds, now_seconds],
    ).await?;

    Ok(())
}

/// 加密笔记本
pub async fn encrypt_notebook(conn: &DbConnection, notebook_id: &str) -> Result<()> {
    let status_id = Uuid::new_v4().to_string();
    let now_seconds = Utc::now().timestamp();
    
    conn.execute(
        "INSERT INTO encryption_status (id, item_type, item_id, is_encrypted, is_unlocked, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(item_type, item_id) DO UPDATE SET
           is_encrypted = excluded.is_encrypted,
           is_unlocked = excluded.is_unlocked,
           updated_at = excluded.updated_at",
        params![status_id, "notebook", notebook_id, true, false, now_seconds, now_seconds],
    ).await?;

    Ok(())
}

/// 解密笔记本
pub async fn decrypt_notebook(conn: &DbConnection, notebook_id: &str) -> Result<()> {
    let now_seconds = Utc::now().timestamp();
    
    conn.execute(
        "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
         WHERE item_id = ? AND item_type = ?",
        params![false, true, now_seconds, notebook_id, "notebook"],
    ).await?;

    Ok(())
}

/// 清除所有会话级别的解锁状态（应用重启时调用）
pub async fn clear_session_unlocks(conn: &DbConnection) -> Result<()> {
    conn.execute(
        "UPDATE encryption_status SET is_unlocked = ? WHERE is_unlocked = ?",
        params![false, true],
    ).await?;
    Ok(())
}

// --- 加密相关方法结束 ---

// ===============================================
// 同步相关数据结构和方法
// ===============================================

// 同步模式枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncMode {
    #[serde(rename = "OFFLINE")]
    Offline,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "AUTO")]
    Auto,
}

impl ToString for SyncMode {
    fn to_string(&self) -> String {
        match self {
            SyncMode::Offline => "OFFLINE".to_string(),
            SyncMode::Manual => "MANUAL".to_string(),
            SyncMode::Auto => "AUTO".to_string(),
        }
    }
}

impl From<String> for SyncMode {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "AUTO" => SyncMode::Auto,
            "MANUAL" => SyncMode::Manual,
            _ => SyncMode::Offline,
        }
    }
}

// 同步状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncStatus {
    Pending,
    Synced,
    Failed,
    Conflict,
}

// 同步操作枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncOperation {
    Insert,
    Update,
    Delete,
}

// 冲突解决策略
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConflictResolutionStrategy {
    LocalWins,
    RemoteWins,
    Merge,
    UserChoice,
}

// 同步配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncConfig {
    pub id: String,
    pub remote_url: Option<String>,
    pub auth_token: Option<String>,
    pub sync_mode: SyncMode,
    pub sync_interval: i64,
    pub last_sync_at: i64,
    pub is_online: bool,
    pub auto_sync_enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// 同步状态记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatusRecord {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub operation: SyncOperation,
    pub sync_status: SyncStatus,
    pub error_message: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// 数据版本记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataVersion {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub version: i64,
    pub hash: String,
    pub created_at: i64,
}

// 冲突解决记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictResolution {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub strategy: ConflictResolutionStrategy,
    pub resolved_by: String,
    pub created_at: i64,
}

// ===============================================
// 同步相关数据库操作函数
// ===============================================

// 初始化同步相关表结构
pub async fn init_sync_schema(conn: &DbConnection) -> Result<()> {
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
        ()
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
        ()
    ).await?;

    // 创建数据版本表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS data_versions (
            id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            record_id TEXT NOT NULL,
            version INTEGER NOT NULL,
            hash TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            UNIQUE(table_name, record_id)
        )",
        ()
    ).await?;

    // 创建冲突解决表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS conflict_resolutions (
            id TEXT PRIMARY KEY,
            table_name TEXT NOT NULL,
            record_id TEXT NOT NULL,
            strategy TEXT NOT NULL,
            resolved_by TEXT NOT NULL,
            created_at INTEGER NOT NULL
        )",
        ()
    ).await?;

    // 创建索引
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sync_status_table_record ON sync_status(table_name, record_id)",
        ()
    ).await?;

    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sync_status_status ON sync_status(sync_status)",
        ()
    ).await?;

    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_data_versions_table_record ON data_versions(table_name, record_id)",
        ()
    ).await?;

    Ok(())
}

// 简化的同步配置管理（使用设置表）
pub async fn get_sync_config(conn: &DbConnection) -> Result<SyncConfig> {
    // 尝试从设置表获取同步配置
    let remote_url = get_setting(conn, "sync_remote_url").await?;
    let auth_token = get_setting(conn, "sync_auth_token").await?;
    let sync_mode_str = get_setting(conn, "sync_mode").await?.unwrap_or_else(|| "OFFLINE".to_string());
    let sync_interval_str = get_setting(conn, "sync_interval").await?.unwrap_or_else(|| "300".to_string());
    let last_sync_at_str = get_setting(conn, "sync_last_sync_at").await?.unwrap_or_else(|| "0".to_string());
    let is_online_str = get_setting(conn, "sync_is_online").await?.unwrap_or_else(|| "false".to_string());
    let auto_sync_enabled_str = get_setting(conn, "sync_auto_sync_enabled").await?.unwrap_or_else(|| "true".to_string());

    let now = Utc::now().timestamp_millis();
    
    Ok(SyncConfig {
        id: "default".to_string(),
        remote_url,
        auth_token,
        sync_mode: SyncMode::from(sync_mode_str),
        sync_interval: sync_interval_str.parse().unwrap_or(300),
        last_sync_at: last_sync_at_str.parse().unwrap_or(0),
        is_online: is_online_str.parse().unwrap_or(false),
        auto_sync_enabled: auto_sync_enabled_str.parse().unwrap_or(true),
        created_at: now,
        updated_at: now,
    })
}

pub async fn save_sync_config(conn: &DbConnection, config: &SyncConfig) -> Result<()> {
    // 保存到设置表
    if let Some(ref remote_url) = config.remote_url {
        save_setting(conn, "sync_remote_url", remote_url).await?;
    }
    if let Some(ref auth_token) = config.auth_token {
        save_setting(conn, "sync_auth_token", auth_token).await?;
    }
    save_setting(conn, "sync_mode", &config.sync_mode.to_string()).await?;
    save_setting(conn, "sync_interval", &config.sync_interval.to_string()).await?;
    save_setting(conn, "sync_last_sync_at", &config.last_sync_at.to_string()).await?;
    save_setting(conn, "sync_is_online", &config.is_online.to_string()).await?;
    save_setting(conn, "sync_auto_sync_enabled", &config.auto_sync_enabled.to_string()).await?;
    
    Ok(())
}

// 获取数据库文件路径
fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf> {
    // 首先检查是否有自定义路径设置
    if let Ok(Some(custom_path_str)) = get_custom_database_path(app_handle) {
        if !custom_path_str.is_empty() {
            let custom_path = PathBuf::from(custom_path_str);
            if custom_path.exists() {
                println!("Using custom database path: {}", custom_path.display());
                return Ok(custom_path);
            } else {
                eprintln!("Custom database path does not exist: {}. Falling back to default.", custom_path.display());
            }
        }
    }
    
    // 如果没有自定义路径或文件不存在，返回默认路径
    let db_name = "mytips.db";
    let data_dir = app_handle.path().app_data_dir()
        .map_err(|_| anyhow!("Cannot find data directory"))?;
    fs::create_dir_all(&data_dir)?;
    Ok(data_dir.join(db_name))
}

// 从设置中获取自定义数据库路径
fn get_custom_database_path(app_handle: &AppHandle) -> Result<Option<String>> {
    let config_dir = app_handle.path().app_config_dir()
        .map_err(|_| anyhow!("无法确定配置目录"))?;
    
    let config_file = config_dir.join("database_path.txt");
    
    if config_file.exists() {
        let path = fs::read_to_string(config_file)?;
        Ok(Some(path.trim().to_string()))
    } else {
        Ok(None)
    }
}
