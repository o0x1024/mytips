use crate::db::{self, DbManager, Tip, TipType};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::State;
use uuid::Uuid;

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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_all_tips(&conn).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = get_images_for_tip(&conn, &tip.id)?;

        // 检查加密状态
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").unwrap_or(false);
        let is_unlocked = db::is_item_unlocked(&conn, &tip.id, "note").unwrap_or(false);
        
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_all_tips(&conn).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").unwrap_or(false);

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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tip = db::get_tip(&conn, &id).map_err(|e| e.to_string())?;
    let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;

    // 获取笔记的图片
    let images = get_images_for_tip(&conn, &tip.id)?;

    let tip_type_str: String = tip.tip_type.into();

    // 检查加密状态
    let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").unwrap_or(false);
    let is_unlocked = db::is_item_unlocked(&conn, &tip.id, "note").unwrap_or(false);
    
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    let now = Utc::now().timestamp_millis();
    
    let is_new = tip_data.id.is_none() || tip_data.id.as_ref().unwrap().is_empty();

    let id = if is_new {
        Uuid::new_v4().to_string()
    } else {
        tip_data.id.clone().unwrap()
    };
    
    let created_at = if !is_new {
        match db::get_tip(&conn, &id) {
            Ok(existing_tip) => existing_tip.created_at,
            Err(_) => now
        }
    } else {
        now
    };
    
    let tip_type = TipType::try_from(tip_data.tip_type.clone()).map_err(|e| e.to_string())?;
    
    let tip = Tip {
        id,
        title: tip_data.title.clone(),
        content: tip_data.content.clone(),
        tip_type,
        language: tip_data.language.clone(),
        category_id: tip_data.category_id.clone(),
        created_at,
        updated_at: now,
    };

    let saved_tip: Tip = db::save_tip(&conn, tip).map_err(|e| e.to_string())?;

    db::set_tip_tags(&conn, &saved_tip.id, &tip_data.tags)
        .map_err(|e| e.to_string())?;

    let tags = db::get_tip_tags(&conn, &saved_tip.id).map_err(|e| e.to_string())?;

    let tip_type_str: String = saved_tip.tip_type.into();

    let result = TipWithTags {
        id: saved_tip.id,
        title: saved_tip.title,
        content: saved_tip.content,
        tip_type: tip_type_str,
        language: saved_tip.language,
        category_id: saved_tip.category_id,
        created_at: saved_tip.created_at,
        updated_at: saved_tip.updated_at,
        tags,
        images: None,
    };

    Ok(result)
}

// 删除笔记
#[tauri::command]
pub async fn delete_tip(id: String, db_manager: State<'_, DbManager>) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    db::delete_tip(&conn, &id).map_err(|e| e.to_string())
}

// 搜索笔记
#[tauri::command]
pub async fn search_tips(query: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::search_tips(&conn, &query).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
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
            images: None, // Simplified for now
        });
    }
    Ok(result)
}

// 搜索笔记摘要
#[tauri::command]
pub async fn search_tips_summary(query: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipSummary>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::search_tips(&conn, &query).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").unwrap_or(false);
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_tips_by_category(&conn, &category_id).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_tips_by_tag(&conn, &tag_id).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
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

// 实现保存图片的API
#[tauri::command]
pub async fn save_tip_image(image_data: ImageData, db_manager: State<'_, DbManager>) -> Result<String, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    db::save_image(&conn, &image_data.tip_id, &image_data.image_id, &image_data.image_data)
        .map_err(|e| e.to_string())?;
    Ok(image_data.image_id)
}

// 实现获取笔记图片的API - 优化版本，支持分页和大小限制
#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images(
    tip_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
    db_manager: State<'_, DbManager>
) -> Result<HashMap<String, String>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let images = if let (Some(l), Some(o)) = (limit, offset) {
        db::get_tip_images_paginated(&conn, &tip_id, l, o)
    } else {
        db::get_tip_images(&conn, &tip_id)
    }.map_err(|e| e.to_string())?;
    
    Ok(images.into_iter().collect())
}

// 实现获取笔记图片总数的API
#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images_count(tip_id: String, db_manager: State<'_, DbManager>) -> Result<i64, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    db::get_tip_images_count(&conn, &tip_id).map_err(|e| e.to_string())
}

// 实现删除图片的API
#[tauri::command]
pub async fn delete_tip_image(image_id: String, db_manager: State<'_, DbManager>) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    db::delete_image(&conn, &image_id).map_err(|e| e.to_string())
}

// 工具函数：获取笔记的所有图片并转换为HashMap
fn get_images_for_tip(
    conn: &db::DbConnection,
    tip_id: &str,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db::get_tip_images(conn, tip_id).map_err(|e| e.to_string())?;

    if images.is_empty() {
        return Ok(None);
    }

    let mut images_map = HashMap::new();
    for (id, data) in images {
        images_map.insert(id, data);
    }

    Ok(Some(images_map))
}

// 工具函数：分页获取笔记的图片并转换为HashMap
fn get_images_for_tip_paginated(
    conn: &db::DbConnection,
    tip_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db::get_tip_images_paginated(conn, tip_id, limit, offset)
        .map_err(|e| e.to_string())?;

    if images.is_empty() {
        return Ok(None);
    }

    let mut images_map = HashMap::new();
    for (id, data) in images {
        images_map.insert(id, data);
    }

    Ok(Some(images_map))
}

#[tauri::command]
pub async fn get_tips_by_category_recursive(category_id: String, db_manager: State<'_, DbManager>) -> Result<Vec<TipWithTags>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_tips_by_category_recursive(&conn, &category_id)
        .map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tips = db::get_tips_paged(&conn, page, page_size).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db::get_tip_tags(&conn, &tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = db::is_item_encrypted(&conn, &tip.id, "note").unwrap_or(false);

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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let tip = db::get_tip(&conn, &id).map_err(|e| e.to_string())?;
    Ok(tip.content)
}
