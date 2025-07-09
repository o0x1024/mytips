use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

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

// 数据库管理器
pub struct DbManager {
    pub conn: Connection,
}

impl DbManager {
    // 初始化数据库
    pub fn init() -> Result<Self> {
        let db_path = get_db_path()?;

        // 确保目录存在
        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let conn = Connection::open(&db_path)?;
        
        // 启用外键约束
        conn.execute("PRAGMA foreign_keys = ON", [])?;

        // 创建表
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

        // 创建AI消息表
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

        // 创建剪贴板历史记录表
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

        // 创建AI角色表
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

        // 创建图片存储表
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

        // 创建加密状态表
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

        let db_manager = DbManager { conn };

        // 初始化默认角色
        db_manager.init_default_roles()?;

        Ok(db_manager)
    }

    // 获取所有笔记
    pub fn get_all_tips(&self) -> Result<Vec<Tip>> {
        let mut stmt = self.conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC")?;
        let tip_iter = stmt.query_map([], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut tips = Vec::new();
        for tip in tip_iter {
            tips.push(tip?);
        }

        Ok(tips)
    }

    // 分页获取笔记
    pub fn get_tips_paged(&self, page: i64, page_size: i64) -> Result<Vec<Tip>> {
        let offset = (page - 1) * page_size;
        let mut stmt = self.conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips ORDER BY updated_at DESC LIMIT ?1 OFFSET ?2")?;
        let tip_iter = stmt.query_map(params![page_size, offset], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut tips = Vec::new();
        for tip in tip_iter {
            tips.push(tip?);
        }
        Ok(tips)
    }

    // 获取单个笔记
    pub fn get_tip(&self, id: &str) -> Result<Tip> {
        let mut stmt = self.conn.prepare("SELECT id, title, content, tip_type, language, category_id, created_at, updated_at FROM tips WHERE id = ?")?;
        let tip = stmt.query_row(params![id], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        Ok(tip)
    }

    // 创建或更新笔记
    pub fn save_tip(&self, mut tip: Tip) -> Result<Tip> {
        let now = Utc::now().timestamp_millis();

        if tip.id.is_empty() {
            tip.id = Uuid::new_v4().to_string();
            tip.created_at = now;
            tip.updated_at = now;
        } else {
            // For existing tips or imports, ensure created_at is valid
            if tip.created_at <= 0 {
                tip.created_at = now;
            }
            // Always update updated_at for existing tips
            tip.updated_at = now;
        }

        let tip_type_str: String = tip.tip_type.into();

        self.conn.execute(
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
                &tip.id,
                &tip.title,
                &tip.content,
                &tip_type_str,
                &tip.language,
                &tip.category_id,
                tip.created_at,
                tip.updated_at,
            ],
        )?;

        // Re-fetch to get the definitive state from the DB
        let final_tip = self.get_tip(&tip.id)?;
        Ok(final_tip)
    }

    // 删除笔记
    pub fn delete_tip(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM tips WHERE id = ?", params![id])?;
        Ok(())
    }

    // 获取所有分类
    pub fn get_all_categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, parent_id FROM categories")?;
        let category_iter = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                name: row.get(1)?,
                parent_id: row.get(2)?,
            })
        })?;

        let mut categories = Vec::new();
        for category in category_iter {
            categories.push(category?);
        }

        Ok(categories)
    }

    // 创建分类
    pub fn create_category(&self, name: &str, parent_id: Option<&str>) -> Result<Category> {
        let id = Uuid::new_v4().to_string();
        self.conn.execute(
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
    pub fn update_category(&self, id: &str, name: &str) -> Result<Category> {
        self.conn.execute(
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
    pub fn delete_category(&self, id: &str) -> Result<()> {
        // 首先将该分类下的笔记的category_id设为null
        self.conn.execute(
            "UPDATE tips SET category_id = NULL WHERE category_id = ?",
            params![id],
        )?;

        // 然后删除分类
        self.conn
            .execute("DELETE FROM categories WHERE id = ?", params![id])?;

        Ok(())
    }

    // 递归删除分类及其所有子分类和笔记
    pub fn delete_category_recursive(&self, id: &str) -> Result<()> {
        // 1. 找到所有子分类
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM categories WHERE parent_id = ?")?;
        let child_ids: Vec<String> = stmt
            .query_map(params![id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // 2. 递归删除所有子分类
        for child_id in child_ids {
            self.delete_category_recursive(&child_id)?;
        }

        // 3. 删除该分类下的所有笔记
        self.conn
            .execute("DELETE FROM tips WHERE category_id = ?", params![id])?;

        // 4. 删除该分类
        self.conn
            .execute("DELETE FROM categories WHERE id = ?", params![id])?;

        Ok(())
    }

    // 递归获取所有子分类ID（包括自身）
    pub fn get_all_subcategory_ids(&self, category_id: &str) -> Result<Vec<String>> {
        let mut all_ids = vec![category_id.to_string()];
        
        // 1. 找到所有直接子分类
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM categories WHERE parent_id = ?")?;
        let child_ids: Vec<String> = stmt
            .query_map(params![category_id], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        // 2. 递归获取每个子分类的所有子分类
        for child_id in child_ids {
            let mut child_all_ids = self.get_all_subcategory_ids(&child_id)?;
            all_ids.append(&mut child_all_ids);
        }

        Ok(all_ids)
    }

    // 递归获取分类及其所有子分类下的所有笔记
    pub fn get_tips_by_category_recursive(&self, category_id: &str) -> Result<Vec<Tip>> {
        // 获取所有子分类ID（包括自身）
        let all_category_ids = self.get_all_subcategory_ids(category_id)?;
        
        let mut all_tips = Vec::new();
        
        // 获取每个分类下的所有笔记
        for cat_id in all_category_ids {
            let mut tips = self.get_tips_by_category(&cat_id)?;
            all_tips.append(&mut tips);
        }
        
        Ok(all_tips)
    }

    // 获取所有标签
    pub fn get_all_tags(&self) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare("SELECT id, name FROM tags")?;
        let tag_iter = stmt.query_map([], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;

        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag?);
        }

        Ok(tags)
    }

    // 创建标签
    pub fn create_tag(&self, name: &str) -> Result<Tag> {
        let id = Uuid::new_v4().to_string();

        // 先检查是否已存在
        let existing =
            self.conn
                .query_row("SELECT id FROM tags WHERE name = ?", params![name], |row| {
                    row.get::<_, String>(0)
                });

        match existing {
            Ok(existing_id) => Ok(Tag {
                id: existing_id,
                name: name.to_string(),
            }),
            Err(_) => {
                self.conn.execute(
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
    pub fn set_tip_tags(&self, tip_id: &str, tag_ids: &[String]) -> Result<()> {
        // 先删除现有关联
        self.conn
            .execute("DELETE FROM tip_tags WHERE tip_id = ?", params![tip_id])?;

        // 添加新关联
        for tag_id in tag_ids {
            self.conn.execute(
                "INSERT INTO tip_tags (tip_id, tag_id) VALUES (?, ?)",
                params![tip_id, tag_id],
            )?;
        }

        Ok(())
    }

    // 删除标签
    pub fn delete_tag(&self, id: &str) -> Result<()> {
        // 删除标签关联
        self.conn
            .execute("DELETE FROM tip_tags WHERE tag_id = ?", params![id])?;

        // 删除标签
        self.conn
            .execute("DELETE FROM tags WHERE id = ?", params![id])?;

        Ok(())
    }

    // 获取笔记的所有标签
    pub fn get_tip_tags(&self, tip_id: &str) -> Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
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

        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag?);
        }

        Ok(tags)
    }

    // 搜索笔记
    pub fn search_tips(&self, query: &str) -> Result<Vec<Tip>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
             FROM tips t
             JOIN tips_fts fts ON t.id = fts.tip_id
             WHERE tips_fts MATCH ?
             ORDER BY t.updated_at DESC",
        )?;

        let tip_iter = stmt.query_map(params![query], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut tips = Vec::new();
        for tip in tip_iter {
            tips.push(tip?);
        }

        Ok(tips)
    }

    // 按分类筛选笔记
    pub fn get_tips_by_category(&self, category_id: &str) -> Result<Vec<Tip>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at
             FROM tips 
             WHERE category_id = ?
             ORDER BY updated_at DESC",
        )?;

        let tip_iter = stmt.query_map(params![category_id], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut tips = Vec::new();
        for tip in tip_iter {
            tips.push(tip?);
        }

        Ok(tips)
    }

    // 按标签筛选笔记
    pub fn get_tips_by_tag(&self, tag_id: &str) -> Result<Vec<Tip>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.title, t.content, t.tip_type, t.language, t.category_id, t.created_at, t.updated_at
             FROM tips t
             JOIN tip_tags tt ON t.id = tt.tip_id
             WHERE tt.tag_id = ?
             ORDER BY t.updated_at DESC"
        )?;

        let tip_iter = stmt.query_map(params![tag_id], |row| {
            let tip_type_str: String = row.get(3)?;
            let tip_type = TipType::try_from(tip_type_str).unwrap_or(TipType::Text);

            Ok(Tip {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                tip_type,
                language: row.get(4)?,
                category_id: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?;

        let mut tips = Vec::new();
        for tip in tip_iter {
            tips.push(tip?);
        }

        Ok(tips)
    }

    // 获取分类
    pub fn get_category_by_id(&self, id: &str) -> Result<Category> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, parent_id FROM categories WHERE id = ?")?;
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
    pub fn save_image(&self, tip_id: &str, image_id: &str, image_data: &str) -> Result<()> {
        let now = Utc::now().timestamp_millis();

        // 检查参数有效性
        if tip_id.is_empty() {
            return Err(anyhow!("笔记ID为空，无法保存图片"));
        }

        if image_id.is_empty() {
            return Err(anyhow!("图片ID为空，无法保存图片"));
        }

        // 先检查tip_id是否存在，避免外键约束错误
        let tip_exists: bool = match self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM tips WHERE id = ?)",
            params![tip_id],
            |row| row.get(0),
        ) {
            Ok(exists) => exists,
            Err(e) => return Err(anyhow!("检查笔记ID是否存在时出错: {}", e)),
        };

        if !tip_exists {
            return Err(anyhow!("笔记ID '{}' 不存在，无法保存图片", tip_id));
        }

        // 保存图片
        match self.conn.execute(
            "INSERT INTO images (id, tip_id, image_data, created_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
             image_data = excluded.image_data",
            params![image_id, tip_id, image_data, now],
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("保存图片到数据库失败: {}", e)),
        }
    }

    // 获取笔记的所有图片
    pub fn get_tip_images(&self, tip_id: &str) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, image_data FROM images
             WHERE tip_id = ?
             ORDER BY created_at DESC",
        )?;

        let image_iter = stmt.query_map(params![tip_id], |row| {
            let id: String = row.get(0)?;
            let image_data: String = row.get(1)?;
            Ok((id, image_data))
        })?;

        let mut images = Vec::new();
        for image in image_iter {
            images.push(image?);
        }

        Ok(images)
    }

    // 分页获取笔记的图片
    pub fn get_tip_images_paginated(
        &self,
        tip_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<(String, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, image_data FROM images
             WHERE tip_id = ?
             ORDER BY created_at DESC
             LIMIT ? OFFSET ?",
        )?;

        let image_iter = stmt.query_map(params![tip_id, limit, offset], |row| {
            let id: String = row.get(0)?;
            let image_data: String = row.get(1)?;
            Ok((id, image_data))
        })?;

        let mut images = Vec::new();
        for image in image_iter {
            images.push(image?);
        }

        Ok(images)
    }

    // 获取笔记的图片总数
    pub fn get_tip_images_count(&self, tip_id: &str) -> Result<i64> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM images WHERE tip_id = ?",
            params![tip_id],
            |row| row.get(0),
        )?;

        Ok(count)
    }

    // 删除笔记的所有图片
    pub fn delete_tip_images(&self, tip_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM images WHERE tip_id = ?", params![tip_id])?;

        Ok(())
    }

    // 删除单个图片
    pub fn delete_image(&self, image_id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM images WHERE id = ?", params![image_id])?;

        Ok(())
    }

    // --- 剪贴板历史记录相关 ---

    // 添加剪贴板条目
    pub fn add_clipboard_entry(&self, content: &str, source: Option<&str>) -> Result<()> {
        self.conn.execute(
            "INSERT INTO clipboard_history (content, source, created_at) VALUES (?, ?, ?)",
            params![content, source, chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }

    // 获取所有剪贴板条目（已弃用，保留以备后用）
    #[allow(dead_code)]
    pub fn get_all_clipboard_entries(&self) -> Result<Vec<ClipboardHistory>> {
        let mut stmt = self.conn.prepare("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC")?;
        let entry_iter = stmt.query_map([], |row| {
            Ok(ClipboardHistory {
                id: row.get(0)?,
                content: row.get(1)?,
                source: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry?);
        }

        Ok(entries)
    }

    // 根据时间戳获取剪贴板条目
    pub fn get_clipboard_entries_since(&self, since_timestamp: i64) -> Result<Vec<ClipboardHistory>> {
        let mut stmt = self.conn.prepare("SELECT id, content, source, created_at FROM clipboard_history WHERE created_at >= ? ORDER BY created_at DESC")?;
        let entry_iter = stmt.query_map(params![since_timestamp], |row| {
            Ok(ClipboardHistory {
                id: row.get(0)?,
                content: row.get(1)?,
                source: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;

        entry_iter.map(|e| e.map_err(Into::into)).collect()
    }

    // 分页获取剪贴板条目
    pub fn get_clipboard_entries_paged(
        &self,
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
            let mut stmt = self.conn.prepare("SELECT id, content, source, created_at FROM clipboard_history WHERE content LIKE ? ORDER BY created_at DESC LIMIT ? OFFSET ?")?;
            let entries_iter = stmt.query_map(params![search_term, page_size, offset], map_row)?;
            entries_iter.map(|e| Ok(e?)).collect()
        } else {
            let mut stmt = self.conn.prepare("SELECT id, content, source, created_at FROM clipboard_history ORDER BY created_at DESC LIMIT ? OFFSET ?")?;
            let entries_iter = stmt.query_map(params![page_size, offset], map_row)?;
            entries_iter.map(|e| Ok(e?)).collect()
        }
    }

    // 获取剪贴板条目总数
    pub fn get_clipboard_entries_count(&self, query: Option<String>) -> Result<i64> {
        if let Some(q) = query {
            let search_term = format!("%{}%", q);
            self.conn.query_row(
                "SELECT COUNT(*) FROM clipboard_history WHERE content LIKE ?",
                params![search_term],
                |row| row.get(0),
            ).map_err(Into::into)
        } else {
            self.conn.query_row(
                "SELECT COUNT(*) FROM clipboard_history",
                [],
                |row| row.get(0),
            ).map_err(Into::into)
        }
    }

    // 删除指定的剪贴板条目
    pub fn delete_clipboard_entries(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

        let query = format!(
            "DELETE FROM clipboard_history WHERE id IN ({})",
            placeholders
        );

        let params = ids
            .iter()
            .map(|id| id as &dyn rusqlite::ToSql)
            .collect::<Vec<_>>();

        self.conn.execute(&query, &params[..])?;
        Ok(())
    }

    // 清除所有剪贴板条目
    pub fn clear_all_clipboard_entries(&self) -> Result<()> {
        self.conn.execute("DELETE FROM clipboard_history", [])?;
        Ok(())
    }

    // 删除过期的剪贴板条目
    pub fn delete_expired_clipboard_entries(&self, expire_timestamp: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM clipboard_history WHERE created_at < ?",
            params![expire_timestamp],
        )?;
        Ok(())
    }

    // 检查剪贴板条目是否已存在
    pub fn check_clipboard_entry_exists(&self, content: &str) -> Result<bool> {
        // 查询最近30秒内是否有相同内容的条目
        let now = Utc::now().timestamp();
        let thirty_seconds_ago = now - 30; // 30秒内的条目视为重复

        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM clipboard_history WHERE content = ? AND created_at > ?",
            params![content, thirty_seconds_ago],
            |row| row.get(0),
        )?;

        Ok(count > 0)
    }

    // 保存设置
    pub fn save_setting(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO settings (key, value) VALUES (?, ?) 
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }

    // 获取设置
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let result = self.conn.query_row(
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
    pub fn get_settings_by_prefix(&self, prefix: &str) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT key FROM settings WHERE key LIKE ? AND value != ''"
        )?;
        let pattern = format!("{}%", prefix);
        let key_iter = stmt.query_map(params![pattern], |row| {
            Ok(row.get::<_, String>(0)?)
        })?;

        let mut keys = Vec::new();
        for key in key_iter {
            keys.push(key?);
        }

        Ok(keys)
    }

    // --- 剪贴板历史记录相关结束 ---

    // --- AI角色相关 ---

    // 获取所有角色
    pub fn get_all_roles(&self) -> Result<Vec<AIRole>> {
        let mut stmt = self.conn.prepare("SELECT id, name, description, created_at, updated_at FROM ai_roles ORDER BY created_at DESC")?;
        let role_iter = stmt.query_map([], |row| {
            Ok(AIRole {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;

        let mut roles = Vec::new();
        for role in role_iter {
            roles.push(role?);
        }

        Ok(roles)
    }

    // 创建角色
    pub fn create_role(&self, name: &str, description: &str) -> Result<AIRole> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().timestamp_millis();

        self.conn.execute(
            "INSERT INTO ai_roles (id, name, description, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
            params![id, name, description, now, now],
        )?;

        Ok(AIRole {
            id,
            name: name.to_string(),
            description: description.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    // 更新角色
    pub fn update_role(&self, id: &str, name: &str, description: &str) -> Result<AIRole> {
        let now = Utc::now().timestamp_millis();

        self.conn.execute(
            "UPDATE ai_roles SET name = ?, description = ?, updated_at = ? WHERE id = ?",
            params![name, description, now, id],
        )?;

        Ok(AIRole {
            id: id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            created_at: 0, // 这里不重新查询created_at，调用方如果需要可以重新获取
            updated_at: now,
        })
    }

    // 删除角色
    pub fn delete_role(&self, id: &str) -> Result<()> {
        self.conn
            .execute("DELETE FROM ai_roles WHERE id = ?", params![id])?;
        Ok(())
    }

    // 根据ID获取角色
    pub fn get_role_by_id(&self, id: &str) -> Result<AIRole> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, description, created_at, updated_at FROM ai_roles WHERE id = ?",
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

    // --- AI角色相关结束 ---

    // 初始化默认角色
    pub fn init_default_roles(&self) -> Result<()> {
        // 检查是否已经有角色，如果有则不创建默认角色
        let roles = self.get_all_roles()?;
        if !roles.is_empty() {
            return Ok(());
        }

        // 创建默认角色
        let default_roles = vec![
            (
                "编程助手",
                "你是一个专业的编程助手，擅长多种编程语言和技术栈。你可以帮助用户解决编程问题、代码调试、架构设计、最佳实践等。请用清晰、准确的方式回答问题，并在适当时提供代码示例。"
            ),
            (
                "写作导师",
                "你是一个经验丰富的写作导师，精通各种文体的写作技巧。你可以帮助用户改进文章结构、提升表达能力、润色文字、纠正语法错误。请提供建设性的反馈和具体的改进建议。"
            ),
            (
                "学习伙伴",
                "你是一个耐心的学习伙伴，善于用简单易懂的方式解释复杂概念。你可以帮助用户理解各种学科知识、回答疑问、提供学习建议。请用循序渐进的方式进行讲解，确保用户能够理解。"
            ),
            (
                "创意顾问",
                "你是一个富有创意的顾问，擅长发散思维和创新思考。你可以帮助用户产生新的想法、解决创意问题、进行头脑风暴、提供不同角度的思考方式。请保持开放的心态，鼓励创新思维。"
            ),
            (
                "翻译专家",
                "你是一个专业的翻译专家，精通多种语言之间的准确翻译。你不仅能够进行字面翻译，还能考虑文化背景、语境和表达习惯，提供自然流畅的翻译结果。请确保翻译的准确性和地道性。"
            ),
        ];

        for (name, description) in default_roles {
            self.create_role(name, description)?;
        }

        Ok(())
    }

    // --- 加密相关方法开始 ---

    /// 获取所有加密状态
    pub fn get_encryption_statuses(&self) -> Result<Vec<EncryptionStatus>> {
        let mut stmt = self.conn.prepare(
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
        
        let mut statuses = Vec::new();
        for status in status_iter {
            statuses.push(status?);
        }
        
        Ok(statuses)
    }

    /// 检查项目是否已加密
    pub fn is_item_encrypted(&self, item_id: &str, item_type: &str) -> Result<bool> {
        let result = self.conn.query_row(
            "SELECT is_encrypted FROM encryption_status WHERE item_id = ? AND item_type = ?",
            params![item_id, item_type],
            |row| row.get(0),
        );
        
        match result {
            Ok(is_encrypted) => Ok(is_encrypted),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// 检查项目是否已解锁
    pub fn is_item_unlocked(&self, item_id: &str, item_type: &str) -> Result<bool> {
        let result = self.conn.query_row(
            "SELECT is_unlocked FROM encryption_status WHERE item_id = ? AND item_type = ?",
            params![item_id, item_type],
            |row| row.get(0),
        );
        
        match result {
            Ok(is_unlocked) => Ok(is_unlocked),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// 加密笔记
    pub fn encrypt_note(&self, note_id: &str, encrypted_content: &str) -> Result<()> {
        // 更新笔记内容为加密内容
        let now = Utc::now().timestamp_millis();
        self.conn.execute(
            "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
            params![encrypted_content, now, note_id],
        )?;
        
        // 更新或插入加密状态
        let status_id = Uuid::new_v4().to_string();
        let now_seconds = Utc::now().timestamp();
        self.conn.execute(
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
    pub fn decrypt_note(&self, note_id: &str, decrypted_content: &str) -> Result<()> {
        // 更新笔记内容为解密内容
        let now = Utc::now().timestamp_millis();
        self.conn.execute(
            "UPDATE tips SET content = ?, updated_at = ? WHERE id = ?",
            params![decrypted_content, now, note_id],
        )?;
        
        // 更新加密状态
        let now_seconds = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
             WHERE item_id = ? AND item_type = ?",
            params![false, false, now_seconds, note_id, "note"],
        )?;
        
        Ok(())
    }

    /// 标记项目为已解锁（会话级别）
    pub fn mark_item_unlocked(&self, item_id: &str, item_type: &str) -> Result<()> {
        let now_seconds = Utc::now().timestamp();
        
        // 如果记录不存在，创建一个新记录
        let status_id = Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO encryption_status (id, item_type, item_id, is_encrypted, is_unlocked, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(item_type, item_id) DO UPDATE SET
             is_unlocked = excluded.is_unlocked,
             updated_at = excluded.updated_at",
            params![status_id, item_type, item_id, true, true, now_seconds, now_seconds],
        )?;
        
        Ok(())
    }

    /// 加密笔记本
    pub fn encrypt_notebook(&self, notebook_id: &str) -> Result<()> {
        let status_id = Uuid::new_v4().to_string();
        let now_seconds = Utc::now().timestamp();
        
        self.conn.execute(
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
    pub fn decrypt_notebook(&self, notebook_id: &str) -> Result<()> {
        let now_seconds = Utc::now().timestamp();
        
        self.conn.execute(
            "UPDATE encryption_status SET is_encrypted = ?, is_unlocked = ?, updated_at = ?
             WHERE item_id = ? AND item_type = ?",
            params![false, false, now_seconds, notebook_id, "notebook"],
        )?;
        
        Ok(())
    }

    /// 清除所有会话级别的解锁状态（应用重启时调用）
    pub fn clear_session_unlocks(&self) -> Result<()> {
        self.conn.execute(
            "UPDATE encryption_status SET is_unlocked = ? WHERE is_unlocked = ?",
            params![false, true],
        )?;
        
        Ok(())
    }

    // --- 加密相关方法结束 ---
}

// 获取数据库文件路径
fn get_db_path() -> Result<PathBuf> {
    // 首先检查是否有自定义路径设置
    if let Ok(Some(custom_path)) = get_custom_database_path() {
        let path = PathBuf::from(custom_path);
        if path.exists() {
            return Ok(path);
        }
    }

    // 如果没有自定义路径或文件不存在，返回默认路径
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow!("Could not determine data directory"))?
        .join("mytips");
    // println!("data_dir: {:?}", data_dir);
    // 确保目录存在
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir.join("mytips.db"))
}

// 获取自定义数据库路径设置
fn get_custom_database_path() -> Result<Option<String>> {
    let config_dir = dirs::config_dir()
        .ok_or_else(|| anyhow!("无法确定配置目录"))?
        .join("mytips");
    
    let config_file = config_dir.join("database_path.txt");
    
    if config_file.exists() {
        let path = fs::read_to_string(config_file)?;
        Ok(Some(path.trim().to_string()))
    } else {
        Ok(None)
    }
}
