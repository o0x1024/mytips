use crate::db::DbManager;
use tauri::State;

// 获取所有分类
#[tauri::command]
pub async fn get_all_categories(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<crate::db::Category>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::get_all_categories(&conn).await.map_err(|e| e.to_string())
}

// 创建分类
#[tauri::command]
pub async fn create_category(
    name: String,
    parent_id: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<crate::db::Category, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::create_category(&conn, &name, parent_id.as_deref()).await.map_err(|e| e.to_string())
}

// 更新分类
#[tauri::command]
pub async fn update_category(
    id: String,
    name: String,
    db_manager: State<'_, DbManager>,
) -> Result<crate::db::Category, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::update_category(&conn, &id, &name).await.map_err(|e| e.to_string())
}

// 删除分类
#[tauri::command]
pub async fn delete_category(
    id: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::delete_category_recursive(&conn, &id).await.map_err(|e| e.to_string())
}
