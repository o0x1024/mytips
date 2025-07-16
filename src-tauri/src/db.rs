use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, params_from_iter, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use tauri::{AppHandle, Manager};

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
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
}

// 标签模型
#[derive(Debug, Serialize, Deserialize)]
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
pub type DbPool = Pool<SqliteConnectionManager>;
pub type DbConnection = PooledConnection<SqliteConnectionManager>;

// 数据库管理器
pub struct DbManager {
    pub pool: DbPool,
}

impl DbManager {
    // 初始化数据库连接池
    pub fn init(app_handle: AppHandle) -> Result<Self> {
        let db_path = get_db_path(&app_handle)?;
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let manager = SqliteConnectionManager::file(db_path);
        let pool = Pool::builder().build(manager)?;

        // 初始化 schema
        let conn = pool.get()?;
        init_schema(&conn)?;
        
        // 可以在这里初始化默认数据
        init_default_roles(&conn)?;
        
        // 初始化同步相关表结构
        init_sync_schema(&conn)?;

        Ok(Self { pool })
    }

    // 从池中获取一个连接
    pub fn get_conn(&self) -> Result<DbConnection> {
        self.pool
            .get()
            .map_err(|e| anyhow!("Failed to get db connection from pool: {}", e))
    }
}

/// 使用给定的连接初始化数据库 schema
fn init_schema(conn: &Connection) -> Result<()> {
    // 启用外键约束
    conn.execute("PRAGMA foreign_keys = ON", [])?;

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
        [],
    )?;
    
    // 创建 categories 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            parent_id TEXT
        )",
        [],
    )?;

    // 创建 ai_conversations 表
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
        [],
    )?;

    // 创建 tags 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    // 创建 tip_tags 表
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

    // 创建 clipboard_history 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clipboard_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT NOT NULL,
            source TEXT,
            created_at INTEGER NOT NULL
        )",
        [],
    )?;

    // 创建设置表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    // 创建 ai_roles 表
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

    // 创建 images 表
    conn.execute(
        "CREATE TABLE IF NOT EXISTS images (
            id TEXT PRIMARY KEY,
            tip_id TEXT NOT NULL,
            image_data TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (tip_id) REFERENCES tips (id) ON DELETE CASCADE
        )",
        [],
    )?;

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
        [],
    )?;

    // 创建FTS5虚拟表用于全文搜索
    conn.execute("DROP TRIGGER IF EXISTS tips_after_insert", [])?;
    conn.execute("DROP TRIGGER IF EXISTS tips_after_delete", [])?;
    conn.execute("DROP TRIGGER IF EXISTS tips_after_update", [])?;
    conn.execute("DROP TABLE IF EXISTS tips_fts", [])?;
    
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS tips_fts USING fts5(
            tip_id UNINDEXED, 
            title, 
            content, 
            tokenize = 'porter unicode61'
        )",
        [],
    )?;

    // 创建触发器，用于在原始表数据发生变化时同步更新FTS表
    // 在插入后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_insert AFTER INSERT ON tips BEGIN
            INSERT INTO tips_fts(tip_id, title, content) VALUES (new.id, new.title, new.content);
        END;",
        [],
    )?;
    // 在删除后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_delete AFTER DELETE ON tips BEGIN
            DELETE FROM tips_fts WHERE tip_id = old.id;
        END;",
        [],
    )?;
    // 在更新后更新
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS tips_after_update AFTER UPDATE ON tips BEGIN
            DELETE FROM tips_fts WHERE tip_id = old.id;
            INSERT INTO tips_fts(tip_id, title, content) VALUES (new.id, new.title, new.content);
        END;",
        [],
    )?;

    // 检查FTS表是否为空，如果为空，则从主表同步数据
    let fts_count: i64 = conn.query_row("SELECT COUNT(*) FROM tips_fts", [], |row| row.get(0))?;
    if fts_count == 0 {
        conn.execute(
            "INSERT INTO tips_fts(tip_id, title, content) SELECT id, title, content FROM tips",
            [],
        )?;
    }

    // 创建索引以提高查询性能
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tips_category_id ON tips(category_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tip_tags_tip_id ON tip_tags(tip_id)",
        [],
    )?;

    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_tip_tags_tag_id ON tip_tags(tag_id)",
        [],
    )?;

    // 创建一些默认数据（如果是新数据库）
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM categories", [], |row| row.get(0))?;

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
            )?;
        }
    }

    Ok(())
}

/// 公开的函数，用于创建数据库表结构
pub fn create_tables(conn: &Connection) -> Result<()> {
    init_schema(conn)
}

fn init_default_roles(conn: &Connection) -> Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM ai_roles", [], |row| row.get(0))?;
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
            )?;
        }
    }
    Ok(())
}

// ===============================================
// Database Operations (as standalone functions)
// ===============================================

// 获取所有笔记
pub fn get_all_tips(conn: &DbConnection) -> Result<Vec<Tip>> {
    let mut stmt = conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC")?;
    let tip_iter = stmt.query_map([], |row| {
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
    })?;

    tip_iter.map(|t| t.map_err(|e| anyhow!(e))).collect()
}

// 分页获取笔记
pub fn get_tips_paged(conn: &DbConnection, page: i64, page_size: i64) -> Result<Vec<Tip>> {
    let offset = (page - 1) * page_size;
    let mut stmt = conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2")?;
    let tip_iter = stmt.query_map(params![page_size, offset], |row| {
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
    })?;

    tip_iter.map(|t| t.map_err(|e| anyhow!(e))).collect()
}

// 获取单个笔记
pub fn get_tip(conn: &DbConnection, id: &str) -> Result<Tip> {
    let mut stmt = conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips WHERE id = ?")?;
    let tip = stmt.query_row(params![id], |row| {
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
    })?;

    Ok(tip)
}

// 保存/更新笔记
pub fn save_tip(conn: &DbConnection, mut tip: Tip) -> Result<Tip> {
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
            tip.id,
            tip.title,
            tip.content,
            tip_type_str,
            tip.language,
            tip.category_id,
            tip.created_at,
            tip.updated_at
        ],
    )?;

    Ok(tip)
}

// 删除笔记
pub fn delete_tip(conn: &DbConnection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM tips WHERE id = ?", params![id])?;
    Ok(())
}

// 获取所有分类
pub fn get_all_categories(conn: &DbConnection) -> Result<Vec<Category>> {
    let mut stmt = conn.prepare("SELECT id, name, parent_id FROM categories")?;
    let category_iter = stmt.query_map([], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;
    category_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 创建分类
pub fn create_category(conn: &DbConnection, name: &str, parent_id: Option<&str>) -> Result<Category> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
        params![id, name, parent_id],
    )?;

    Ok(Category {
        id,
        name: name.to_string(),
        parent_id: parent_id.map(|s| s.to_string()),
    })
}

// 更新分类
pub fn update_category(conn: &DbConnection, id: &str, name: &str) -> Result<Category> {
    conn.execute(
        "UPDATE categories SET name = ? WHERE id = ?",
        params![name, id],
    )?;

    Ok(Category {
        id: id.to_string(),
        name: name.to_string(),
        parent_id: None,
    })
}

// 删除分类
pub fn delete_category(conn: &DbConnection, id: &str) -> Result<()> {
    // 首先将该分类下的笔记的category_id设为null
    conn.execute(
        "UPDATE tips SET category_id = NULL WHERE category_id = ?",
        params![id],
    )?;

    // 然后删除分类
    conn.execute("DELETE FROM categories WHERE id = ?", params![id])?;

    Ok(())
}

// 递归删除分类及其所有子分类和笔记
pub fn delete_category_recursive(conn: &DbConnection, id: &str) -> Result<()> {
    // 1. 找到所有子分类
    let mut stmt = conn.prepare("SELECT id FROM categories WHERE parent_id = ?")?;
    let child_ids: Vec<String> = stmt
        .query_map(params![id], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    // 2. 递归删除所有子分类
    for child_id in child_ids {
        delete_category_recursive(conn, &child_id)?;
    }

    // 3. 删除该分类下的所有笔记
    conn.execute("DELETE FROM tips WHERE category_id = ?", params![id])?;

    // 4. 删除该分类
    conn.execute("DELETE FROM categories WHERE id = ?", params![id])?;

    Ok(())
}

// 递归获取所有子分类ID（包括自身）
pub fn get_all_subcategory_ids(conn: &DbConnection, category_id: &str) -> Result<Vec<String>> {
    let mut all_ids = vec![category_id.to_string()];
    
    // 1. 找到所有直接子分类
    let mut stmt = conn.prepare("SELECT id FROM categories WHERE parent_id = ?")?;
    let child_ids: Vec<String> = stmt
        .query_map(params![category_id], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();

    // 2. 递归获取每个子分类的所有子分类
    for child_id in child_ids {
        let mut child_all_ids = get_all_subcategory_ids(conn, &child_id)?;
        all_ids.append(&mut child_all_ids);
    }
    
    Ok(all_ids)
}

// 递归获取分类及其所有子分类下的所有笔记
pub fn get_tips_by_category_recursive(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    // 获取所有子分类ID（包括自身）
    let all_category_ids = get_all_subcategory_ids(conn, category_id)?;
    
    let mut all_tips = Vec::new();
    
    // 获取每个分类下的所有笔记
    for cat_id in all_category_ids {
        let mut tips = get_tips_by_category(conn, &cat_id)?;
        all_tips.append(&mut tips);
    }
    
    Ok(all_tips)
}

// 获取所有标签
pub fn get_all_tags(conn: &DbConnection) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare("SELECT id, name FROM tags")?;
    let tag_iter = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    tag_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 创建标签
pub fn create_tag(conn: &DbConnection, name: &str) -> Result<Tag> {
    let id = Uuid::new_v4().to_string();

    // 先检查是否已存在
    let existing =
        conn.query_row("SELECT id FROM tags WHERE name = ?", params![name], |row| {
            row.get::<_, String>(0)
        });

    match existing {
        Ok(existing_id) => Ok(Tag {
            id: existing_id,
            name: name.to_string(),
        }),
        Err(_) => {
            conn.execute(
                "INSERT INTO tags (id, name) VALUES (?, ?)",
                params![id, name],
            )?;
            Ok(Tag {
                id,
                name: name.to_string(),
            })
        }
    }
}

// 为笔记设置标签
pub fn set_tip_tags(conn: &DbConnection, tip_id: &str, tag_ids: &[String]) -> Result<()> {
    // 先删除现有关联
    conn.execute("DELETE FROM tip_tags WHERE tip_id = ?", params![tip_id])?;

    // 添加新关联
    for tag_id in tag_ids {
        conn.execute(
            "INSERT INTO tip_tags (tip_id, tag_id) VALUES (?, ?)",
            params![tip_id, tag_id],
        )?;
    }
    Ok(())
}

// 删除标签
pub fn delete_tag(conn: &DbConnection, id: &str) -> Result<()> {
    // 删除标签关联
    conn.execute("DELETE FROM tip_tags WHERE tag_id = ?", params![id])?;

    // 删除标签
    conn.execute("DELETE FROM tags WHERE id = ?", params![id])?;

    Ok(())
}

// 获取笔记的所有标签
pub fn get_tip_tags(conn: &DbConnection, tip_id: &str) -> Result<Vec<Tag>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.name FROM tags t
         JOIN tip_tags tt ON t.id = tt.tag_id
         WHERE tt.tip_id = ?",
    )?;
    let tag_iter = stmt.query_map(params![tip_id], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;
    tag_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 搜索笔记
pub fn search_tips(conn: &DbConnection, query: &str) -> Result<Vec<Tip>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
         FROM tips t
         LEFT JOIN tags tg ON t.id = tg.id
         WHERE t.title LIKE ?1 OR t.content LIKE ?1 OR tg.name LIKE ?1
         GROUP BY t.id
         ORDER BY t.updated_at DESC",
    )?;
    let query = format!("%{}%", query);
    let tip_iter = stmt.query_map(params![query], |row| {
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
    })?;

    tip_iter.map(|t| t.map_err(|e| anyhow!(e))).collect()
}

// 按分类筛选笔记
pub fn get_tips_by_category(conn: &DbConnection, category_id: &str) -> Result<Vec<Tip>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at
         FROM tips 
         WHERE category_id = ? 
         ORDER BY updated_at DESC",
    )?;
    let tip_iter = stmt.query_map(params![category_id], |row| {
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
    })?;

    tip_iter.map(|t| t.map_err(|e| anyhow!(e))).collect()
}

// 按标签筛选笔记
pub fn get_tips_by_tag(conn: &DbConnection, tag_id: &str) -> Result<Vec<Tip>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
         FROM tips t
         JOIN tip_tags tt ON t.id = tt.tip_id
         WHERE tt.tag_id = ?
         ORDER BY t.updated_at DESC",
    )?;
    let tip_iter = stmt.query_map(params![tag_id], |row| {
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
    })?;

    tip_iter.map(|t| t.map_err(|e| anyhow!(e))).collect()
}

// 获取分类
pub fn get_category_by_id(conn: &DbConnection, id: &str) -> Result<Category> {
    let mut stmt = conn.prepare("SELECT id, name, parent_id FROM categories WHERE id = ?")?;
    let category = stmt.query_row(params![id], |row| {
        Ok(Category {
            id: row.get(0)?,
            name: row.get(1)?,
            parent_id: row.get(2)?,
        })
    })?;
    Ok(category)
}

// 保存图片
pub fn save_image(conn: &DbConnection, tip_id: &str, image_id: &str, image_data: &str) -> Result<()> {
    let now = Utc::now().timestamp_millis();

    // 先检查tip_id是否存在，避免外键约束错误
    let tip_exists: bool = match conn.query_row(
        "SELECT EXISTS(SELECT 1 FROM tips WHERE id = ?)",
        params![tip_id],
        |row| row.get(0),
    ) {
        Ok(exists) => exists,
        Err(_) => false,
    };

    if !tip_exists {
        return Err(anyhow!("Tip with id {} not found", tip_id));
    }

    // 保存图片
    match conn.execute(
        "INSERT INTO images (id, tip_id, image_data, created_at)
         VALUES (?, ?, ?, ?)
         ON CONFLICT(id) DO UPDATE SET
           image_data = excluded.image_data",
        params![image_id, tip_id, image_data, now],
    ) {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow!("Failed to save image: {}", e)),
    }
}

// 获取笔记的所有图片
pub fn get_tip_images(conn: &DbConnection, tip_id: &str) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, image_data FROM images
         WHERE tip_id = ?
         ORDER BY created_at ASC",
    )?;
    let image_iter = stmt.query_map(params![tip_id], |row| Ok((row.get(0)?, row.get(1)?)))?;

    image_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 分页获取笔记的图片
pub fn get_tip_images_paginated(
    conn: &DbConnection,
    tip_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Vec<(String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, image_data FROM images
         WHERE tip_id = ?
         ORDER BY created_at ASC
         LIMIT ? OFFSET ?",
    )?;
    let image_iter =
        stmt.query_map(params![tip_id, limit, offset], |row| Ok((row.get(0)?, row.get(1)?)))?;

    image_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 获取笔记的图片总数
pub fn get_tip_images_count(conn: &DbConnection, tip_id: &str) -> Result<i64> {
    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM images WHERE tip_id = ?",
        params![tip_id],
        |row| row.get(0),
    )?;
    Ok(count)
}

// 删除笔记的所有图片
pub fn delete_tip_images(conn: &DbConnection, tip_id: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE tip_id = ?", params![tip_id])?;

    Ok(())
}

// 删除单个图片
pub fn delete_image(conn: &DbConnection, image_id: &str) -> Result<()> {
    conn.execute("DELETE FROM images WHERE id = ?", params![image_id])?;

    Ok(())
}

// --- 剪贴板历史记录相关 ---

// 添加剪贴板条目
pub fn add_clipboard_entry(conn: &DbConnection, content: &str, source: Option<&str>) -> Result<()> {
    conn.execute(
        "INSERT INTO clipboard_history (content, source, created_at) VALUES (?, ?, ?)",
        params![content, source, chrono::Utc::now().timestamp()],
    )?;
    Ok(())
}

// 获取所有剪贴板条目（已弃用，保留以备后用）
#[allow(dead_code)]
pub fn get_all_clipboard_entries(conn: &DbConnection) -> Result<Vec<ClipboardHistory>> {
    let mut stmt = conn.prepare("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC")?;
    let entry_iter = stmt.query_map([], |row| {
        Ok(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    entry_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 根据时间戳获取剪贴板条目
pub fn get_clipboard_entries_since(conn: &DbConnection, since_timestamp: i64) -> Result<Vec<ClipboardHistory>> {
    let mut stmt = conn.prepare("SELECT id, content, source, created_at FROM clipboard_history WHERE created_at >= ? ORDER BY created_at DESC")?;
    let entry_iter = stmt.query_map(params![since_timestamp], |row| {
        Ok(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        })
    })?;

    entry_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 分页获取剪贴板条目
pub fn get_clipboard_entries_paged(
    conn: &DbConnection,
    page: i64,
    page_size: i64,
    query: Option<String>,
) -> Result<Vec<ClipboardHistory>> {
    let offset = (page - 1) * page_size;

    let map_row = |row: &rusqlite::Row| {
        Ok(ClipboardHistory {
            id: row.get(0)?,
            content: row.get(1)?,
            source: row.get(2)?,
            created_at: row.get(3)?,
        })
    };

    if let Some(q) = query {
        let search_term = format!("%{}%", q);
        let mut stmt = conn.prepare("SELECT id, content, source, created_at FROM clipboard_history WHERE content LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?")?;
        let entries_iter = stmt.query_map(params![search_term, page_size, offset], map_row)?;
        entries_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    } else {
        let mut stmt = conn.prepare("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC LIMIT ? OFFSET ?")?;
        let entries_iter = stmt.query_map(params![page_size, offset], map_row)?;
        entries_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }
}

// 获取剪贴板条目总数
pub fn get_clipboard_entries_count(conn: &DbConnection, query: Option<String>) -> Result<i64> {
    if let Some(q) = query {
        let search_term = format!("%{}%", q);
        Ok(conn.query_row(
            "SELECT COUNT(*) FROM clipboard_history WHERE content LIKE ?",
            params![search_term],
            |row| row.get(0),
        )?)
    } else {
        Ok(conn.query_row(
            "SELECT COUNT(*) FROM clipboard_history",
            [],
            |row| row.get(0),
        )?)
    }
}

// 删除指定的剪贴板条目
pub fn delete_clipboard_entries(conn: &DbConnection, ids: &[i64]) -> Result<()> {
    if ids.is_empty() {
        return Ok(());
    }

    let params_sql = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("DELETE FROM clipboard_history WHERE id IN ({})", params_sql);
    
    conn.execute(&query, params_from_iter(ids.iter()))?;
    Ok(())
}

// 清除所有剪贴板条目
pub fn clear_all_clipboard_entries(conn: &DbConnection) -> Result<()> {
    conn.execute("DELETE FROM clipboard_history", [])?;
    Ok(())
}

// 删除过期的剪贴板条目
pub fn delete_expired_clipboard_entries(conn: &DbConnection, expire_timestamp: i64) -> Result<()> {
    conn.execute(
        "DELETE FROM clipboard_history WHERE created_at < ?",
        params![expire_timestamp],
    )?;
    Ok(())
}

// 检查剪贴板条目是否已存在
pub fn check_clipboard_entry_exists(conn: &DbConnection, content: &str) -> Result<bool> {
    // 查询最近30秒内是否有相同内容的条目
    let now = Utc::now().timestamp();
    let thirty_seconds_ago = now - 30; // 30秒内的条目视为重复

    let count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM clipboard_history WHERE content = ? AND created_at > ?",
        params![content, thirty_seconds_ago],
        |row| row.get(0),
    )?;

    Ok(count > 0)
}

// 获取剪贴板条目ID列表，基于天数
pub fn get_clipboard_entry_ids_by_days(
    conn: &DbConnection,
    days: u32,
) -> Result<Vec<i64>> {
    let mut stmt = conn.prepare(
        "SELECT id FROM clipboard_history WHERE created_at >= strftime('%s', 'now', ?)",
    )?;
    let ids = stmt
        .query_map([format!("-{} day", days)], |row| row.get(0))?
        .collect::<Result<Vec<i64>, _>>()?;
    Ok(ids)
}

// 保存设置
pub fn save_setting(conn: &DbConnection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?, ?) 
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

// 获取设置
pub fn get_setting(conn: &DbConnection, key: &str) -> Result<Option<String>> {
    let result = conn.query_row(
        "SELECT value FROM settings WHERE key = ?",
        params![key],
        |row| row.get(0),
    );
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

// 获取所有以指定前缀开头的设置键
pub fn get_settings_by_prefix(conn: &DbConnection, prefix: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT key FROM settings WHERE key LIKE ? AND value != ''"
    )?;
    let like_prefix = format!("{}%", prefix);
    let key_iter = stmt.query_map(params![like_prefix], |row| {
        Ok(row.get(0)?)
    })?;

    key_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}


// --- AI 相关方法 ---

// 获取所有角色
pub fn get_all_roles(conn: &DbConnection) -> Result<Vec<AIRole>> {
    let mut stmt = conn.prepare("SELECT id, name, description, created_at, updated_at FROM ai_roles ORDER BY created_at DESC")?;
    let role_iter = stmt.query_map([], |row| {
        Ok(AIRole {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    role_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

// 创建角色
pub fn create_role(conn: &DbConnection, name: &str, description: &str) -> Result<AIRole> {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "INSERT INTO ai_roles (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        params![id, name, description, now, now],
    )?;
    Ok(AIRole { id, name: name.to_string(), description: description.to_string(), created_at: now, updated_at: now })
}

// 更新角色
pub fn update_role(conn: &DbConnection, id: &str, name: &str, description: &str) -> Result<AIRole> {
    let now = Utc::now().timestamp_millis();

    conn.execute(
        "UPDATE ai_roles SET name = ?, description = ?, updated_at = ? WHERE id = ?",
        params![name, description, now, id],
    )?;
    // This is not ideal as created_at is not known here.
    // A full fetch would be better, but for now we return what we can.
    Ok(AIRole { id: id.to_string(), name: name.to_string(), description: description.to_string(), created_at: 0, updated_at: now })
}

// 删除角色
pub fn delete_role(conn: &DbConnection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM ai_roles WHERE id = ?", params![id])?;
    Ok(())
}

// 根据ID获取角色
pub fn get_role_by_id(conn: &DbConnection, id: &str) -> Result<AIRole> {
    let mut stmt = conn.prepare(
        "SELECT id, name, description, created_at, updated_at FROM ai_roles WHERE id = ?"
    )?;
    let role = stmt.query_row(params![id], |row| {
        Ok(AIRole {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;
    Ok(role)
}

// --- 加密相关方法 ---

/// 获取所有加密状态
pub fn get_encryption_statuses(conn: &DbConnection) -> Result<Vec<EncryptionStatus>> {
    let mut stmt = conn.prepare(
        "SELECT item_type, item_id, is_encrypted, is_unlocked FROM encryption_status"
    )?;
    
    let status_iter = stmt.query_map([], |row| {
        let item_type: String = row.get(0)?;
        let item_id: String = row.get(1)?;
        let is_encrypted: bool = row.get(2)?;
        let is_unlocked: bool = row.get(3)?;
        
        Ok(EncryptionStatus {
            note_id: if item_type == "note" { Some(item_id.clone()) } else { None },
            notebook_id: if item_type == "notebook" { Some(item_id) } else { None },
            is_encrypted,
            is_unlocked,
        })
    })?;
    
    status_iter.collect::<Result<Vec<_>, _>>().map_err(Into::into)
}

/// 检查项目是否已加密
pub fn is_item_encrypted(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<bool> {
    let result = conn.query_row(
        "SELECT is_encrypted FROM encryption_status WHERE item_id = ? AND item_type = ?",
        params![item_id, item_type],
        |row| row.get(0),
    );
    match result {
        Ok(encrypted) => Ok(encrypted),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false), // 如果没有记录，则未加密
        Err(e) => Err(e.into()),
    }
}

/// 检查项目是否已解锁
pub fn is_item_unlocked(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<bool> {
    let result = conn.query_row(
        "SELECT is_unlocked FROM encryption_status WHERE item_id = ? AND item_type = ?",
        params![item_id, item_type],
        |row| row.get(0),
    );
    match result {
        Ok(unlocked) => Ok(unlocked),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false), // 如果没有记录，则未解锁
        Err(e) => Err(e.into()),
    }
}

/// 加密笔记
pub fn encrypt_note(conn: &DbConnection, note_id: &str, encrypted_content: &str) -> Result<()> {
    // 更新笔记内容为加密内容
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
        params![encrypted_content, now, note_id],
    )?;

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
    )?;

    Ok(())
}

/// 解密笔记
pub fn decrypt_note(conn: &DbConnection, note_id: &str, decrypted_content: &str) -> Result<()> {
    // 更新笔记内容为解密内容
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
        params![decrypted_content, now, note_id],
    )?;

    // 更新加密状态
    let now_seconds = Utc::now().timestamp();
    conn.execute(
        "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
         WHERE item_id = ? AND item_type = ?",
        params![false, true, now_seconds, note_id, "note"],
    )?;

    Ok(())
}

/// 标记项目为已解锁（会话级别）
pub fn mark_item_unlocked(conn: &DbConnection, item_id: &str, item_type: &str) -> Result<()> {
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
    )?;

    Ok(())
}

/// 加密笔记本
pub fn encrypt_notebook(conn: &DbConnection, notebook_id: &str) -> Result<()> {
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
    )?;

    Ok(())
}

/// 解密笔记本
pub fn decrypt_notebook(conn: &DbConnection, notebook_id: &str) -> Result<()> {
    let now_seconds = Utc::now().timestamp();
    
    conn.execute(
        "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
         WHERE item_id = ? AND item_type = ?",
        params![false, true, now_seconds, notebook_id, "notebook"],
    )?;

    Ok(())
}

/// 清除所有会话级别的解锁状态（应用重启时调用）
pub fn clear_session_unlocks(conn: &DbConnection) -> Result<()> {
    conn.execute(
        "UPDATE encryption_status SET is_unlocked = ? WHERE is_unlocked = ?",
        params![false, true],
    )?;
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
pub fn init_sync_schema(conn: &DbConnection) -> Result<()> {
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
        []
    )?;

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
        []
    )?;

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
        []
    )?;

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
        []
    )?;

    // 创建索引
    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sync_status_table_record ON sync_status(table_name, record_id)",
        []
    );

    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sync_status_status ON sync_status(sync_status)",
        []
    );

    let _ = conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_data_versions_table_record ON data_versions(table_name, record_id)",
        []
    );

    Ok(())
}

// 简化的同步配置管理（使用设置表）
pub fn get_sync_config(conn: &DbConnection) -> Result<SyncConfig> {
    // 尝试从设置表获取同步配置
    let remote_url = get_setting(conn, "sync_remote_url")?;
    let auth_token = get_setting(conn, "sync_auth_token")?;
    let sync_mode_str = get_setting(conn, "sync_mode")?.unwrap_or_else(|| "OFFLINE".to_string());
    let sync_interval_str = get_setting(conn, "sync_interval")?.unwrap_or_else(|| "300".to_string());
    let last_sync_at_str = get_setting(conn, "sync_last_sync_at")?.unwrap_or_else(|| "0".to_string());
    let is_online_str = get_setting(conn, "sync_is_online")?.unwrap_or_else(|| "false".to_string());
    let auto_sync_enabled_str = get_setting(conn, "sync_auto_sync_enabled")?.unwrap_or_else(|| "true".to_string());

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

pub fn save_sync_config(conn: &DbConnection, config: &SyncConfig) -> Result<()> {
    // 保存到设置表
    if let Some(ref remote_url) = config.remote_url {
        save_setting(conn, "sync_remote_url", remote_url)?;
    }
    if let Some(ref auth_token) = config.auth_token {
        save_setting(conn, "sync_auth_token", auth_token)?;
    }
    save_setting(conn, "sync_mode", &config.sync_mode.to_string())?;
    save_setting(conn, "sync_interval", &config.sync_interval.to_string())?;
    save_setting(conn, "sync_last_sync_at", &config.last_sync_at.to_string())?;
    save_setting(conn, "sync_is_online", &config.is_online.to_string())?;
    save_setting(conn, "sync_auto_sync_enabled", &config.auto_sync_enabled.to_string())?;
    
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
