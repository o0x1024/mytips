use crate::db::{UnifiedDbManager, models::Tag, operations};
use tauri::State;
use chrono::Utc;
use uuid::Uuid;

// 获取所有标签
#[tauri::command]
pub async fn get_all_tags(db_manager: State<'_, UnifiedDbManager>) -> Result<Vec<Tag>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::list_tags(&conn).await.map_err(|e| e.to_string())
}

// 创建标签
#[tauri::command]
pub async fn create_tag(
    db_manager: State<'_, UnifiedDbManager>,
    name: String,
) -> Result<Tag, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    
    let tag = Tag {
        id: Uuid::new_v4().to_string(),
        name,
        created_at: now,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
    };
    
    operations::create_tag(&conn, &tag).await.map_err(|e| e.to_string())?;
    Ok(tag)
}

// 删除标签
#[tauri::command]
pub async fn delete_tag(db_manager: State<'_, UnifiedDbManager>, id: String) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    operations::delete_tag(&conn, &id).await.map_err(|e| e.to_string())
}
