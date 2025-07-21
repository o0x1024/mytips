use crate::db::{UnifiedDbManager, models::Category, operations};
use tauri::{AppHandle, Manager};
use chrono::Utc;
use uuid::Uuid;

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

// 获取所有分类
#[tauri::command]
pub async fn get_all_categories(app: AppHandle) -> Result<Vec<Category>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::list_categories(&conn).await.map_err(|e| e.to_string())
}

// 创建分类
#[tauri::command]
pub async fn create_category(
    name: String,
    parent_id: Option<String>,
    app: AppHandle,
) -> Result<Category, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    
    let category = Category {
        id: Uuid::new_v4().to_string(),
        name,
        parent_id,
        created_at: now,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
        is_encrypted: Some(false),
        encryption_key_id: None,
    };
    
    operations::create_category(&conn, &category).await.map_err(|e| e.to_string())?;
    
    // 触发后台同步（如果在嵌入式副本模式下）
    trigger_background_sync_if_needed(&app).await;
    
    Ok(category)
}

// 更新分类
#[tauri::command]
pub async fn update_category(
    id: String,
    name: String,
    app: AppHandle,
) -> Result<Category, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 先获取现有分类
    let mut category = operations::get_category_by_id(&conn, &id).await.map_err(|e| e.to_string())?
        .ok_or("Category not found")?;
    
    // 更新字段
    category.name = name;
    category.updated_at = Utc::now().timestamp_millis();
    
    // 保存更新
    operations::update_category(&conn, &category).await.map_err(|e| e.to_string())?;
    
    // 触发后台同步（如果在嵌入式副本模式下）
    trigger_background_sync_if_needed(&app).await;
    
    Ok(category)
}

// 删除分类
#[tauri::command]
pub async fn delete_category(
    id: String,
    app: AppHandle,
) -> Result<(), String> {
    println!("API: Starting to delete category with id: {}", id);
    tracing::info!("API: Starting to delete category with id: {}", id);
    
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match operations::delete_category(&conn, &id).await {
        Ok(_) => {
            println!("API: Successfully deleted category with id: {}", id);
            tracing::info!("API: Successfully deleted category with id: {}", id);
            
            // 触发后台同步（如果在嵌入式副本模式下）
            trigger_background_sync_if_needed(&app).await;
            
            Ok(())
        }
        Err(e) => {
            println!("API: Failed to delete category with id: {}, error: {}", id, e);
            tracing::error!("API: Failed to delete category with id: {}, error: {}", id, e);
            Err(e.to_string())
        }
    }
}
