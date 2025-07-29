use crate::db::{UnifiedDbManager, models::{Tip, TipType, Tag, Category}, operations};
use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager};
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
    pub tags: Vec<Tag>,
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
    pub tags: Vec<Tag>,
    pub is_encrypted: bool,
    pub content: Option<String>, // 添加content字段用于搜索预览
}

// 分类浏览响应数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryBrowseResponse {
    pub current_category: Option<Category>,
    pub subcategories: Vec<Category>,
    pub featured_tip: Option<TipWithTags>, // 第一条笔记的完整内容
    pub tip_summaries: Vec<TipSummary>,    // 其他笔记的摘要信息
    pub total_tips_count: i64,             // 当前分类下的笔记总数
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
            version: Some(1),
            last_synced_at: Some(0),
            sync_hash: None,
            is_encrypted: Some(false),
            encryption_key_id: None,
            encrypted_content: None,
        })
    }
}

// 辅助函数：获取统一数据库管理器
async fn get_unified_manager(app: &AppHandle) -> Result<UnifiedDbManager, String> {
    if let Some(manager) = app.try_state::<UnifiedDbManager>() {
        Ok((*manager.inner()).clone())
    } else {
        // 如果没有找到，尝试创建
        UnifiedDbManager::new(app.clone()).await
            .map_err(|e| format!("Failed to create unified manager: {}", e))
    }
}

// 辅助函数：在需要时触发后台同步
async fn trigger_background_sync_if_needed(app: &AppHandle) {
    if let Ok(unified_manager) = get_unified_manager(app).await {
        let current_mode = unified_manager.get_current_mode().await;
        if current_mode.supports_sync() {
            tokio::spawn(async move {
                if let Err(e) = unified_manager.sync().await {
                    tracing::warn!("Background sync failed after database operation: {}", e);
                } else {
                    tracing::info!("Background sync completed after database operation");
                }
            });
        }
    }
}

// 获取所有笔记
#[tauri::command]
pub async fn get_all_tips(app: AppHandle) -> Result<Vec<TipWithTags>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = operations::list_tips(&conn).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = match operations::get_tip_images(&conn, &tip.id).await {
            Ok(image_list) => {
                if image_list.is_empty() {
                    None
                } else {
                    let mut image_map = HashMap::new();
                    for (id, data) in image_list {
                        image_map.insert(id, data);
                    }
                    Some(image_map)
                }
            }
            Err(_) => None, // 如果获取图片失败，继续处理其他数据
        };

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
            images,
        });
    }

    Ok(result)
}

// 获取所有笔记的摘要
#[tauri::command]
pub async fn get_all_tip_summaries(app: AppHandle) -> Result<Vec<TipSummary>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = operations::list_tips(&conn).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = false; // TODO: 实现加密功能

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
            content: None, // 摘要列表不包含内容
        });
    }

    Ok(result)
}

// 获取单个笔记
#[tauri::command]
pub async fn get_tip(id: String, app: AppHandle) -> Result<TipWithTags, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tip = operations::get_tip_by_id(&conn, &id).await.map_err(|e| e.to_string())?
        .ok_or("Tip not found")?;
    let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能

    let tip_type_str: String = tip.tip_type.into();

    // 获取笔记的图片
    let images = match operations::get_tip_images(&conn, &tip.id).await {
        Ok(image_list) => {
            if image_list.is_empty() {
                None
            } else {
                let mut image_map = HashMap::new();
                for (id, data) in image_list {
                    image_map.insert(id, data);
                }
                Some(image_map)
            }
        }
        Err(_) => None, // 如果获取图片失败，继续处理其他数据
    };

    let result = TipWithTags {
        id: tip.id,
        title: tip.title,
        content: tip.content,
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
pub async fn save_tip(tip_data: TipData, app: AppHandle) -> Result<TipWithTags, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;

    let now = Utc::now().timestamp_millis();
    let tip_type = TipType::try_from(tip_data.tip_type.clone())
        .map_err(|e| format!("Invalid tip type: {}", e))?;

    let tip_id = tip_data.id.clone().unwrap_or_else(|| Uuid::new_v4().to_string());
    let is_new_tip = tip_data.id.is_none();
    
    // 如果是更新操作，需要获取原始的创建时间
    let created_at = if !is_new_tip {
        if let Ok(Some(existing_tip)) = operations::get_tip_by_id(&conn, &tip_id).await {
            existing_tip.created_at
        } else {
            now
        }
    } else {
        now
    };

    let tip = Tip {
        id: tip_id.clone(),
        title: tip_data.title.clone(),
        content: tip_data.content.clone(),
        tip_type,
        language: tip_data.language.clone(),
        category_id: tip_data.category_id.clone(),
        created_at,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
        is_encrypted: Some(false),
        encryption_key_id: None,
        encrypted_content: None,
    };

    if is_new_tip {
        operations::create_tip(&conn, &tip).await.map_err(|e| e.to_string())?;
    } else {
        operations::update_tip(&conn, &tip).await.map_err(|e| e.to_string())?;
    }

    // 触发后台同步（如果在嵌入式副本模式下）
    trigger_background_sync_if_needed(&app).await;

    Ok(TipWithTags {
        id: tip_id,
        title: tip_data.title,
        content: tip_data.content,
        tip_type: tip_data.tip_type,
        language: tip_data.language,
        category_id: tip_data.category_id,
        created_at,
        updated_at: now,
        tags: Vec::new(), // TODO: 实现标签功能
        images: None, // TODO: 实现图片功能
    })
}

// 删除笔记
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_tip(tip_id: String, app: AppHandle) -> Result<(), String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 删除笔记及其所有依赖关系（包括图片、标签关联等）
    operations::delete_tip(&conn, &tip_id).await.map_err(|e| e.to_string())?;
    
    // 触发后台同步（如果在嵌入式副本模式下）
    trigger_background_sync_if_needed(&app).await;
    
    Ok(())
}

// 搜索笔记
#[tauri::command]
pub async fn search_tips(query: String, app: AppHandle) -> Result<Vec<TipWithTags>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = operations::search_tips(&conn, &query).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = match operations::get_tip_images(&conn, &tip.id).await {
            Ok(image_list) => {
                if image_list.is_empty() {
                    None
                } else {
                    let mut image_map = HashMap::new();
                    for (id, data) in image_list {
                        image_map.insert(id, data);
                    }
                    Some(image_map)
                }
            }
            Err(_) => None, // 如果获取图片失败，继续处理其他数据
        };

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
            images,
        });
    }

    Ok(result)
}

// 搜索笔记摘要 - 优化版本
#[tauri::command]
pub async fn search_tips_summary(query: String, app: AppHandle) -> Result<Vec<TipSummary>, String> {
    if query.trim().is_empty() {
        return Ok(Vec::new());
    }
    
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 使用优化的快速搜索函数，限制30个结果
    let summaries = operations::search_tips_summary_fast(&conn, &query, 30).await.map_err(|e| e.to_string())?;
    
    Ok(summaries)
}

// 按分类获取笔记
#[tauri::command]
pub async fn get_tips_by_category(category_id: String, app: AppHandle) -> Result<Vec<TipWithTags>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = operations::get_tips_by_category(&conn, &category_id).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = match operations::get_tip_images(&conn, &tip.id).await {
            Ok(image_list) => {
                if image_list.is_empty() {
                    None
                } else {
                    let mut image_map = HashMap::new();
                    for (id, data) in image_list {
                        image_map.insert(id, data);
                    }
                    Some(image_map)
                }
            }
            Err(_) => None, // 如果获取图片失败，继续处理其他数据
        };

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
            images,
        });
    }

    Ok(result)
}

// 按标签获取笔记 - 临时返回空结果
#[tauri::command]
pub async fn get_tips_by_tag(_tag_id: String, _app: AppHandle) -> Result<Vec<TipWithTags>, String> {
    Ok(Vec::new()) // TODO: 实现标签查询功能
}

// 图片相关API
#[tauri::command]
pub async fn save_tip_image(image_data: ImageData, app: AppHandle) -> Result<String, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::save_tip_image(&conn, &image_data.tip_id, &image_data.image_id, &image_data.image_data)
        .await
        .map_err(|e| e.to_string())?;
    
    // 触发后台同步
    trigger_background_sync_if_needed(&app).await;
    
    Ok(image_data.image_id)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images(
    tip_id: String, 
    limit: Option<i32>, 
    offset: Option<i32>, 
    app: AppHandle
) -> Result<HashMap<String, String>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    let images = if let (Some(limit), Some(offset)) = (limit, offset) {
        operations::get_tip_images_paginated(&conn, &tip_id, limit, offset)
            .await
            .map_err(|e| e.to_string())?
    } else {
        operations::get_tip_images(&conn, &tip_id)
            .await
            .map_err(|e| e.to_string())?
    };
    
    let mut image_map = HashMap::new();
    for (id, data) in images {
        image_map.insert(id, data);
    }
    Ok(image_map)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_tip_images_count(tip_id: String, app: AppHandle) -> Result<i64, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::get_tip_images_count(&conn, &tip_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn delete_tip_image(image_id: String, app: AppHandle) -> Result<(), String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::delete_tip_image(&conn, &image_id)
        .await
        .map_err(|e| e.to_string())?;
    
    // 触发后台同步
    trigger_background_sync_if_needed(&app).await;
    
    Ok(())
}

#[tauri::command]
pub async fn get_tips_by_category_recursive(category_id: String, app: AppHandle) -> Result<Vec<TipWithTags>, String> {
    // 记录警告日志，建议使用新的API
    tracing::warn!("get_tips_by_category_recursive is deprecated. Consider using browse_category for better performance");
    
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tips = operations::get_tips_by_category_recursive(&conn, &category_id).await.map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();

        // 获取笔记的图片
        let images = match operations::get_tip_images(&conn, &tip.id).await {
            Ok(image_list) => {
                if image_list.is_empty() {
                    None
                } else {
                    let mut image_map = HashMap::new();
                    for (id, data) in image_list {
                        image_map.insert(id, data);
                    }
                    Some(image_map)
                }
            }
            Err(_) => None, // 如果获取图片失败，继续处理其他数据
        };

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
            images,
        });
    }

    Ok(result)
}

// 新的分类浏览API - 优化性能
#[tauri::command]
pub async fn browse_category(category_id: String, app: AppHandle) -> Result<CategoryBrowseResponse, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取当前分类信息
    let current_category = if category_id.is_empty() {
        None
    } else {
        operations::get_category_by_id(&conn, &category_id).await.map_err(|e| e.to_string())?
    };
    
    // 获取子分类列表
    let subcategories = if category_id.is_empty() {
        // 如果是根目录，获取所有顶级分类
        operations::list_categories(&conn).await.map_err(|e| e.to_string())?
            .into_iter()
            .filter(|cat| cat.parent_id.is_none())
            .collect()
    } else {
        operations::get_subcategories(&conn, &category_id).await.map_err(|e| e.to_string())?
    };
    
    // 获取当前分类下的笔记总数（递归计算）
    let total_tips_count = if category_id.is_empty() {
        // 根目录：计算所有顶级分类及其子分类的笔记总数
        let mut total = 0i64;
        for subcategory in &subcategories {
            total += operations::count_tips_by_category_recursive(&conn, &subcategory.id).await.map_err(|e| e.to_string())?;
        }
        total
    } else {
        // 特定分类：使用递归计数，包括所有子分类
        operations::count_tips_by_category_recursive(&conn, &category_id).await.map_err(|e| e.to_string())?
    };
    
    // 获取第一条笔记的完整内容（如果有的话）
    let featured_tip = if total_tips_count > 0 {
        let tips = if category_id.is_empty() {
            // 根目录：从所有子分类中递归获取笔记
            operations::get_tips_by_category_recursive_paged(&conn, &subcategories.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), 1, 0).await.map_err(|e| e.to_string())?
        } else {
            // 特定分类：递归获取该分类及其子分类的笔记
            operations::get_tips_by_category_recursive_paged_single(&conn, &category_id, 1, 0).await.map_err(|e| e.to_string())?
        };
        if let Some(tip) = tips.first() {
            let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
            let tip_type_str: String = tip.tip_type.into();

            // 获取笔记的图片
            let images = match operations::get_tip_images(&conn, &tip.id).await {
                Ok(image_list) => {
                    if image_list.is_empty() {
                        None
                    } else {
                        let mut image_map = HashMap::new();
                        for (id, data) in image_list {
                            image_map.insert(id, data);
                        }
                        Some(image_map)
                    }
                }
                Err(_) => None,
            };

            Some(TipWithTags {
                id: tip.id.clone(),
                title: tip.title.clone(),
                content: tip.content.clone(),
                tip_type: tip_type_str,
                language: tip.language.clone(),
                category_id: tip.category_id.clone(),
                created_at: tip.created_at,
                updated_at: tip.updated_at,
                tags,
                images,
            })
        } else {
            None
        }
    } else {
        None
    };
    
    // 获取其他笔记的摘要信息（跳过第一条）
    let tip_summaries = if total_tips_count > 1 {
        let tips = if category_id.is_empty() {
            // 根目录：从所有子分类中递归获取笔记
            operations::get_tips_by_category_recursive_paged(&conn, &subcategories.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), 20, 1).await.map_err(|e| e.to_string())?
        } else {
            // 特定分类：递归获取该分类及其子分类的笔记
            operations::get_tips_by_category_recursive_paged_single(&conn, &category_id, 20, 1).await.map_err(|e| e.to_string())?
        }; // 最多20条摘要，跳过第一条
        let mut summaries = Vec::new();
        for tip in tips {
            let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
            let tip_type_str: String = tip.tip_type.into();
            let is_encrypted = tip.is_encrypted.unwrap_or(false);

            summaries.push(TipSummary {
                id: tip.id,
                title: tip.title,
                tip_type: tip_type_str,
                language: tip.language,
                category_id: tip.category_id,
                created_at: tip.created_at,
                updated_at: tip.updated_at,
                tags,
                is_encrypted,
                content: None, // 摘要不包含内容
            });
        }
        summaries
    } else {
        Vec::new()
    };

    Ok(CategoryBrowseResponse {
        current_category,
        subcategories,
        featured_tip,
        tip_summaries,
        total_tips_count,
    })
}

// 分页加载分类下的更多笔记
#[tauri::command]
pub async fn load_more_tips_by_category(
    category_id: String, 
    offset: i32, 
    limit: i32, 
    app: AppHandle
) -> Result<Vec<TipSummary>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    let tips = if category_id.is_empty() {
        // 根目录：从所有顶级分类中递归获取笔记
        let subcategories = operations::list_categories(&conn).await.map_err(|e| e.to_string())?
            .into_iter()
            .filter(|cat| cat.parent_id.is_none())
            .collect::<Vec<_>>();
        operations::get_tips_by_category_recursive_paged(&conn, &subcategories.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(), limit, offset).await.map_err(|e| e.to_string())?
    } else {
        // 特定分类：递归获取该分类及其子分类的笔记
        operations::get_tips_by_category_recursive_paged_single(&conn, &category_id, limit, offset).await.map_err(|e| e.to_string())?
    };

    let mut summaries = Vec::new();
    for tip in tips {
        let tags: Vec<Tag> = Vec::new(); // TODO: 实现标签功能
        let tip_type_str: String = tip.tip_type.into();
        let is_encrypted = tip.is_encrypted.unwrap_or(false);

        summaries.push(TipSummary {
            id: tip.id,
            title: tip.title,
            tip_type: tip_type_str,
            language: tip.language,
            category_id: tip.category_id,
            created_at: tip.created_at,
            updated_at: tip.updated_at,
            tags,
            is_encrypted,
            content: None, // 摘要不包含内容
        });
    }

    Ok(summaries)
}

#[tauri::command]
pub async fn get_tips_paged(
    _page: i64,
    _page_size: i64,
    _app: AppHandle
) -> Result<Vec<TipSummary>, String> {
    Ok(Vec::new()) // TODO: 实现分页功能
}

#[tauri::command]
pub async fn get_tip_content(id: String, app: AppHandle) -> Result<String, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let tip = operations::get_tip_by_id(&conn, &id).await.map_err(|e| e.to_string())?
        .ok_or("Tip not found")?;
    Ok(tip.content)
}
