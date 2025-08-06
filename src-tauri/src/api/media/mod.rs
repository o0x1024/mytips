use crate::db::operations::{cleanup_orphaned_media_files, get_media_statistics as db_get_media_statistics, MediaStatistics};
use crate::db::UnifiedDbManager;
use tauri::State;

/// 清理孤儿媒体文件
#[tauri::command]
pub async fn cleanup_orphaned_media(
    state: State<'_, UnifiedDbManager>,
) -> Result<(u64, u64), String> {
    let conn = state.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    cleanup_orphaned_media_files(&conn).await
        .map_err(|e| format!("Media cleanup failed: {}", e))
}

/// 获取媒体文件统计信息
#[tauri::command]
pub async fn get_media_statistics(
    state: State<'_, UnifiedDbManager>,
) -> Result<MediaStatistics, String> {
    let conn = state.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;
    
    db_get_media_statistics(&conn).await
        .map_err(|e| format!("Get media statistics failed: {}", e))
}