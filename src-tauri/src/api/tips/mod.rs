use crate::db::{DbManager, Tip, TipType};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
            created_at: now,
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

// 获取所有笔记
#[tauri::command]
pub async fn get_all_tips() -> Result<Vec<TipWithTags>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = db.get_all_tips().map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = get_images_for_tip(&db, &tip.id)?;

        // 检查加密状态
        let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
        let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
        
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

// 获取单个笔记
#[tauri::command]
pub async fn get_tip(id: String) -> Result<TipWithTags, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tip = db.get_tip(&id).map_err(|e| e.to_string())?;
    let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;

    // 获取笔记的图片
    let images = get_images_for_tip(&db, &tip.id)?;

    let tip_type_str: String = tip.tip_type.into();

    // 检查加密状态
    let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
    let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
    
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
pub async fn save_tip(tip_data: TipData) -> Result<TipWithTags, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;

    // 转换为数据库模型
    let tip = Tip::try_from(tip_data.clone()).map_err(|e| e.to_string())?;

    // 保存笔记
    let saved_tip = db.save_tip(tip).map_err(|e| e.to_string())?;

    // 处理标签
    db.set_tip_tags(&saved_tip.id, &tip_data.tags)
        .map_err(|e| e.to_string())?;

    // 获取保存后的标签
    let tags = db.get_tip_tags(&saved_tip.id).map_err(|e| e.to_string())?;

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
pub async fn delete_tip(id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.delete_tip(&id).map_err(|e| e.to_string())?;

    Ok(())
}

// 搜索笔记
#[tauri::command]
pub async fn search_tips(query: String) -> Result<Vec<TipWithTags>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = db.search_tips(&query).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 检查加密状态
        let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
        let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
        
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
            images: None,
        });
    }

    Ok(result)
}

// 按分类获取笔记
#[tauri::command]
pub async fn get_tips_by_category(category_id: String) -> Result<Vec<TipWithTags>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = db
        .get_tips_by_category(&category_id)
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 检查加密状态
        let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
        let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
        
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
            images: None,
        });
    }

    Ok(result)
}

// 按标签获取笔记
#[tauri::command]
pub async fn get_tips_by_tag(tag_id: String) -> Result<Vec<TipWithTags>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = db.get_tips_by_tag(&tag_id).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 检查加密状态
        let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
        let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
        
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
            images: None,
        });
    }

    Ok(result)
}

// 实现保存图片的API
#[tauri::command]
pub async fn save_tip_image(image_data: ImageData) -> Result<String, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;

    // 保存图片到数据库
    db.save_image(
        &image_data.tip_id,
        &image_data.image_id,
        &image_data.image_data,
    )
    .map_err(|e| e.to_string())?;

    Ok(image_data.image_id)
}

// 实现获取笔记图片的API - 优化版本，支持分页和大小限制
#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images(
    tip_id: String,
    limit: Option<i32>,
    offset: Option<i32>,
) -> Result<HashMap<String, String>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;

    // 设置默认限制，防止一次性加载过多数据
    let limit = limit.unwrap_or(10).min(50); // 最多50张图片
    let offset = offset.unwrap_or(0).max(0);

    let images_map = get_images_for_tip_paginated(&db, &tip_id, limit, offset)?;

    Ok(images_map.unwrap_or_default())
}

// 实现获取笔记图片总数的API
#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images_count(tip_id: String) -> Result<i64, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;

    let count = db
        .get_tip_images_count(&tip_id)
        .map_err(|e| e.to_string())?;

    Ok(count)
}

// 实现删除图片的API
#[tauri::command]
pub async fn delete_tip_image(image_id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.delete_image(&image_id).map_err(|e| e.to_string())?;

    Ok(())
}

// 工具函数：获取笔记的所有图片并转换为HashMap
fn get_images_for_tip(
    db: &DbManager,
    tip_id: &str,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db.get_tip_images(tip_id).map_err(|e| e.to_string())?;

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
    db: &DbManager,
    tip_id: &str,
    limit: i32,
    offset: i32,
) -> Result<Option<HashMap<String, String>>, String> {
    let images = db
        .get_tip_images_paginated(tip_id, limit, offset)
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
pub async fn get_tips_by_category_recursive(category_id: String) -> Result<Vec<TipWithTags>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let tips = db.get_tips_by_category_recursive(&category_id).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags = db.get_tip_tags(&tip.id).map_err(|e| e.to_string())?;
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = get_images_for_tip(&db, &tip.id)?;

        // 检查加密状态
        let is_encrypted = db.is_item_encrypted(&tip.id, "note").unwrap_or(false);
        let is_unlocked = db.is_item_unlocked(&tip.id, "note").unwrap_or(false);
        
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
