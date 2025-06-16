use crate::db::DbManager;

// 获取所有分类
#[tauri::command]
pub async fn get_all_categories() -> Result<Vec<crate::db::Category>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.get_all_categories().map_err(|e| e.to_string())
}

// 创建分类
#[tauri::command(rename_all = "snake_case")]
pub async fn create_category(
    name: String,
    parent_id: Option<String>,
) -> Result<crate::db::Category, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.create_category(&name, parent_id.as_deref())
        .map_err(|e| e.to_string())
}

// 更新分类
#[tauri::command]
pub async fn update_category(id: String, name: String) -> Result<crate::db::Category, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.update_category(&id, &name).map_err(|e| e.to_string())
}

// 删除分类
#[tauri::command]
pub async fn delete_category(id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.delete_category_recursive(&id).map_err(|e| e.to_string())
}
