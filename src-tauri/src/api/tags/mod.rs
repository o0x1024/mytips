use crate::db::DbManager;
use tauri::State;

// 获取所有标签
#[tauri::command]
pub async fn get_all_tags(db_manager: State<'_, DbManager>) -> Result<Vec<crate::db::Tag>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    crate::db::get_all_tags(&conn).map_err(|e| e.to_string())
}

// 创建标签
#[tauri::command]
pub async fn create_tag(
    db_manager: State<'_, DbManager>,
    name: String,
) -> Result<crate::db::Tag, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    crate::db::create_tag(&conn, &name).map_err(|e| e.to_string())
}

// 删除标签
#[tauri::command]
pub async fn delete_tag(db_manager: State<'_, DbManager>, id: String) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    crate::db::delete_tag(&conn, &id).map_err(|e| e.to_string())
}
