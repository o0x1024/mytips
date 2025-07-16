use crate::db::{self, DbManager, Tip, TipType};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use uuid::Uuid;
use libsql::{params, OptionalExtension};
use futures::TryFutureExt;

// 前端传递的笔记数据
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TipData {
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub tip_type: String,
    pub language: Option<String>,
    pub category_id: Option<String>,
    pub tags: Vec<String>,
}

// 添加图片数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageData {
    pub tip_id: String,
    pub image_id: String,
    pub image_data: String,
}

// 从TipData转换为Tip模型
impl TryFrom<TipData> for Tip {
    type Error = anyhow::Error;

    fn try_from(data: TipData) -> Result<Self, Self::Error> {
        let tip_type = TipType::try_from(data.tip_type)?;
        let now = Utc::now().timestamp_millis();

        Ok(Tip {
            id: data.id.unwrap_or_default(),
            title: data.title,
            content: data.content,
            tip_type,
            language: data.language,
            category_id: data.category_id,
            created_at: now, // 这里总是使用当前时间，导致导入时可能覆盖原始创建时间
            updated_at: now,
        })
    }
}

// 返回给前端的笔记数据，包含标签和图片
#[derive(Debug, Serialize, Deserialize)]
pub struct TipWithTags {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tip_type: String,
    pub language: Option<String>,
    pub category_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub tags: Vec<crate::db::Tag>,
    pub images: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TipSummary {
    pub id: String,
    pub title: String,
    pub tip_type: String,
    pub language: Option<String>,
    pub category_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    pub tags: Vec<crate::db::Tag>,
    pub is_encrypted: bool,
}

// 获取所有笔记
#[tauri::command]
pub async fn get_all_tips(db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_all_tips(&conn).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = get_images_for_tip(&conn, &tip.id).await?;

        // 检查加密状态
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").await.unwrap_or(false);
        let is_unlocked = db::is_item_unlocked(&conn, &tip.id, "note").await.unwrap_or(false);
        
        // 如果笔记已加密但未解锁，返回占位符内容
        let content = if is_encrypted && !is_unlocked {
            "[此笔记已加密，请解锁后查看]".to_string()
        } else {
            tip.content
        };

        result.push(TipWithTags {
            id: tip.id,
            title: tip.title,
            content,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            images,
        });
    }

    Ok(result)
}

// 获取所有笔记的摘要
#[tauri::command]
pub async fn get_all_tip_summaries(db_manager: State<'_, DbManager>) -> Result<Vec<TipSummary>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_all_tips(&conn).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").await.unwrap_or(false);

        result.push(TipSummary {
            id: tip.id,
            title: tip.title,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            is_encrypted,
        });
    }

    Ok(result)
}


// 获取单个笔记
#[tauri::command]
pub async fn get_tip(id: String, db_manager: State<'_, DbManager>) -> Result<TipWithTags, String> {
    let conn = db_manager.conn.lock().await;
    let tip = db::get_tip(&conn, &id).await.map_err(|e| e.to_string())?;
    let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;

    // 获取笔记的图片
    let images = get_images_for_tip(&conn, &tip.id).await?;

    let tip_type_str: String = tip.tip_type.into();

    // 检查加密状态
    let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").await.unwrap_or(false);
    let is_unlocked = db::is_item_unlocked(&conn, &tip.id, "note").await.unwrap_or(false);
    
    // 如果笔记已加密但未解锁，返回占位符内容
    let content = if is_encrypted && !is_unlocked {
        "[此笔记已加密，请解锁后查看]".to_string()
    } else {
        tip.content
    };

    let result = TipWithTags {
        id: tip.id,
        title: tip.title,
        content,
        tip_type: tip_type_str,
        language: tip.language,
        category_id: tip.category_id,
        created_at: tip.created_at,
        updated_at: tip.updated_at,
        tags,
        images,
    };

    Ok(result)
}

// 保存笔记
#[tauri::command]
pub async fn save_tip(tip_data: TipData, db_manager: State<'_, DbManager>) -> Result<TipWithTags, String> {
    let conn = db_manager.conn.lock().await;

    let now = Utc::now().timestamp_millis();
    let tip_type = TipType::try_from(tip_data.tip_type.clone())
        .map_err(|e| format!("Invalid tip type: {}", e))?;

    let tip_id = tip_data.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let is_new_tip = tip_data.id.is_none();
    
    // 如果是更新操作，需要获取原始的创建时间
    let created_at = if !is_new_tip {
        conn.query_row(
            "SELECT created_at FROM tips WHERE id = ?1",
            params![&tip_id],
            |row| row.get(0)
        ).await.unwrap_or(Ok(now)).unwrap_or(now)
    } else {
        now
    };

    let tip_to_save = Tip {
        id: tip_id.clone(),
        title: tip_data.title.clone(),
        content: tip_data.content.clone(),
        tip_type,
        language: tip_data.language.clone(),
        category_id: tip_data.category_id.clone(),
        created_at,
        updated_at: now,
    };

    db::save_tip(&conn, tip_to_save).await.map_err(|e| e.to_string())?;

    // 更新标签
    let mut tags = Vec::new();
    for tag_name in tip_data.tags.iter().filter(|t| !t.trim().is_empty()) {
        let tag = db::create_tag(&conn, tag_name).await.map_err(|e| e.to_string())?;
        tags.push(tag);
    }
    let tag_ids: Vec<String> = tags.iter().map(|t| t.id.clone()).collect();
    db::set_tip_tags(&conn, &tip_id, &tag_ids).await.map_err(|e| e.to_string())?;
    
    let tip = TipWithTags {
        id: tip_id,
        title: tip_data.title,
        content: tip_data.content,
        tip_type: tip_data.tip_type,
        language: tip_data.language,
        category_id: tip_data.category_id,
        created_at,
        updated_at: now,
        tags,
        images: None, // 图片在另外的接口处理
    };
    
    Ok(tip)
}

// 删除笔记
#[tauri::command]
pub async fn delete_tip(id: String, db_manager: State<'_, DbManager>) -> Result<(), String> {
    let conn = db_manager.conn.lock().await;
    db::delete_tip(&conn, &id).await.map_err(|e| e.to_string())
}

// 搜索笔记
#[tauri::command]
pub async fn search_tips(query: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::search_tips(&conn, &query).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        result.push(TipWithTags {
            id: tip.id,
            title: tip.title,
            content: tip.content,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            images: None,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn search_tips_summary(query: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipSummary>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::search_tips(&conn, &query).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").await.unwrap_or(false);

        result.push(TipSummary {
            id: tip.id,
            title: tip.title,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            is_encrypted,
        });
    }

    Ok(result)
}

// 按分类获取笔记
#[tauri::command]
pub async fn get_tips_by_category(category_id: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_tips_by_category(&conn, &category_id).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        result.push(TipWithTags {
            id: tip.id,
            title: tip.title,
            content: tip.content,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            images: None,
        });
    }

    Ok(result)
}

// 按标签获取笔记
#[tauri::command]
pub async fn get_tips_by_tag(tag_id: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_tips_by_tag(&conn, &tag_id).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        result.push(TipWithTags {
            id: tip.id,
            title: tip.title,
            content: tip.content,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            images: None,
        });
    }

    Ok(result)
}

// 保存图片
#[tauri::command]
pub async fn save_tip_image(image_data: ImageData, db_manager: State<'_, DbManager>) -> Result<String, String> {
    let conn = db_manager.conn.lock().await;
    db::save_image(&conn, &image_data.tip_id, &image_data.image_id, &image_data.image_data).await.map_err(|e| e.to_string())?;
    Ok(image_data.image_id)
}

// 获取笔记的图片
#[tauri::command]
pub async fn get_tip_images(
    tip_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
    db_manager: State<'_, DbManager>
) -> Result<HashMap<String, String>, String> {
    let conn = db_manager.conn.lock().await;
    let images = if let (Some(limit), Some(offset)) = (limit, offset) {
        db::get_tip_images_paginated(&conn, &tip_id, limit, offset).await.map_err(|e| e.to_string())?
    } else {
        db::get_tip_images(&conn, &tip_id).await.map_err(|e| e.to_string())?
    };
    let mut image_map = HashMap::new();
    for (id, data) in images {
        image_map.insert(id, data);
    }
    Ok(image_map)
}

// 获取笔记的图片总数
#[tauri::command]
pub async fn get_tip_images_count(tip_id: String, db_manager: State<'_, DbManager>) -> Result<i64, String> {
    let conn = db_manager.conn.lock().await;
    db::get_tip_images_count(&conn, &tip_id).await.map_err(|e| e.to_string())
}

// 删除图片
#[tauri::command]
pub async fn delete_tip_image(image_id: String, db_manager: State<'_, DbManager>) -> Result<(), String> {
    let conn = db_manager.conn.lock().await;
    db::delete_image(&conn, &image_id).await.map_err(|e| e.to_string())
}

async fn get_images_for_tip(
    conn: &libsql::Connection,
    tip_id: &str,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db::get_tip_images(conn, tip_id).await.map_err(|e| e.to_string())?;
    if images.is_empty() {
        Ok(None)
    } else {
        let mut image_map = HashMap::new();
        for (id, data) in images {
            image_map.insert(id, data);
        }
        Ok(Some(image_map))
    }
}

async fn get_images_for_tip_paginated(
    conn: &libsql::Connection,
    tip_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db::get_tip_images_paginated(conn, tip_id, limit, offset).await.map_err(|e| e.to_string())?;
    if images.is_empty() {
        Ok(None)
    } else {
        let mut image_map = HashMap::new();
        for (id, data) in images {
            image_map.insert(id, data);
        }
        Ok(Some(image_map))
    }
}

#[tauri::command]
pub async fn get_tips_by_category_recursive(category_id: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_tips_by_category_recursive(&conn, &category_id).await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        result.push(TipWithTags {
            id: tip.id,
            title: tip.title,
            content: tip.content,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            images: None,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_tips_paged(
    page: i64,
    page_size: i64,
    db_manager: State<'_, DbManager>
) -> Result<Vec<TipSummary>, String> {
    let conn = db_manager.conn.lock().await;
    let tips = db::get_tips_paged(&conn, page, page_size).await.map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).await.map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").await.unwrap_or(false);
        result.push(TipSummary {
            id: tip.id,
            title: tip.title,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            is_encrypted,
        });
    }

    Ok(result)
}

#[tauri::command]
pub async fn get_tip_content(id: String, db_manager: State<'_, DbManager>) -> Result<String, String> {
    let conn = db_manager.conn.lock().await;
    let tip = db::get_tip(&conn, &id).await.map_err(|e| e.to_string())?;
    Ok(tip.content)
} 