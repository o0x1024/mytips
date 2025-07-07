use crate::db::{AIRole, DbManager};

// 获取所有角色
#[tauri::command]
pub async fn list_ai_roles() -> Result<Vec<AIRole>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.get_all_roles().map_err(|e| e.to_string())
}

// 创建角色
#[tauri::command]
pub async fn create_ai_role(name: String, description: String) -> Result<AIRole, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.create_role(&name, &description)
        .map_err(|e| e.to_string())
}

// 更新角色
#[tauri::command(rename_all = "snake_case")]
pub async fn update_ai_role(
    role_id: String,
    name: String,
    description: String,
) -> Result<AIRole, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.update_role(&role_id, &name, &description)
        .map_err(|e| e.to_string())
}

// 删除角色
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_ai_role(role_id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.delete_role(&role_id).map_err(|e| e.to_string())
}

// 根据ID获取角色
#[tauri::command(rename_all = "snake_case")]
pub async fn get_ai_role(role_id: String) -> Result<AIRole, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    db.get_role_by_id(&role_id).map_err(|e| e.to_string())
}
