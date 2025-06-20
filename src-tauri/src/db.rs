use anyhow::{anyhow, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

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
    pub created_at: i64, // 时间戳
    pub updated_at: i64, // 时间戳
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
                Category {
                    id: Uuid::new_v4().to_string(),
                    name: "前端开发".to_string(),
                    parent_id: None,
                },
                Category {
                    id: Uuid::new_v4().to_string(),
                    name: "后端开发".to_string(),
                    parent_id: None,
                },
                Category {
                    id: Uuid::new_v4().to_string(),
                    name: "工具使用".to_string(),
                    parent_id: None,
                },
                Category {
                    id: Uuid::new_v4().to_string(),
                    name: "AI提示词".to_string(),
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
    pub fn save_tip(&self, tip: Tip) -> Result<Tip> {
        let tip_id = if tip.id.is_empty() {
            Uuid::new_v4().to_string()
        } else {
            tip.id.clone()
        };

        let now = Utc::now().timestamp_millis();
        let tip_type_str: String = tip.tip_type.into();

        self.conn.execute(
            "INSERT INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
             title = excluded.title,
             content = excluded.content,
             tip_type = excluded.tip_type,
             language = excluded.language,
             category_id = excluded.category_id,
             updated_at = excluded.updated_at",
            params![
                tip_id,
                tip.title,
                tip.content,
                tip_type_str,
                tip.language,
                tip.category_id,
                tip.created_at,
                now
            ],
        )?;

        Ok(Tip {
            id: tip_id,
            title: tip.title,
            content: tip.content,
            tip_type: tip.tip_type.clone(),
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: now,
        })
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
        let search_pattern = format!("%{}%", query);

        let mut stmt = self.conn.prepare(
            "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at
             FROM tips 
             WHERE title LIKE ? OR content LIKE ?
             ORDER BY updated_at DESC",
        )?;

        let tip_iter = stmt.query_map(params![search_pattern, search_pattern], |row| {
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

        self.conn.execute(
            "INSERT INTO images (id, tip_id, image_data, created_at)
             VALUES (?, ?, ?, ?)
             ON CONFLICT(id) DO UPDATE SET
             image_data = excluded.image_data",
            params![image_id, tip_id, image_data, now],
        )?;

        Ok(())
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
    pub fn get_tip_images_paginated(&self, tip_id: &str, limit: i32, offset: i32) -> Result<Vec<(String, String)>> {
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

    // 获取所有剪贴板条目
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

    // 删除指定的剪贴板条目
    pub fn delete_clipboard_entries(&self, ids: &[i64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let placeholders = ids
            .iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(",");

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
        self.conn.execute("DELETE FROM ai_roles WHERE id = ?", params![id])?;
        Ok(())
    }

    // 根据ID获取角色
    pub fn get_role_by_id(&self, id: &str) -> Result<AIRole> {
        let mut stmt = self.conn.prepare("SELECT id, name, description, created_at, updated_at FROM ai_roles WHERE id = ?")?;
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
}

// 获取数据库文件路径
fn get_db_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir()
        .ok_or_else(|| anyhow!("Could not determine data directory"))?
        .join("mytips");
    // println!("data_dir: {:?}", data_dir);
    // 确保目录存在
    fs::create_dir_all(&data_dir)?;

    Ok(data_dir.join("mytips.db"))
}
