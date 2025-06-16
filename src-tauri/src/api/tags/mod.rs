use crate::db::DbManager;

// 获取所有标签
#[tauri::command]
pub async fn get_all_tags() -> Result<Vec<crate::db::Tag>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.get_all_tags().map_err(|e| e.to_string())
}

// 创建标签
#[tauri::command]
pub async fn create_tag(name: String) -> Result<crate::db::Tag, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.create_tag(&name).map_err(|e| e.to_string())
}

// 删除标签
#[tauri::command]
pub async fn delete_tag(id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.delete_tag(&id).map_err(|e| e.to_string())
}
