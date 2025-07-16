use crate::db::DbManager;
use tauri::State;

// 获取所有角色
#[tauri::command]
pub async fn list_ai_roles(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<crate::db::AIRole>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::get_all_roles(&conn).await.map_err(|e| e.to_string())
}

// 创建角色
#[tauri::command]
pub async fn create_ai_role(
    name: String,
    description: String,
    db_manager: State<'_, DbManager>,
) -> Result<crate::db::AIRole, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::create_role(&conn, &name, &description).await.map_err(|e| e.to_string())
}

// 更新角色
#[tauri::command]
pub async fn update_ai_role(
    role_id: String,
    name: String,
    description: String,
    db_manager: State<'_, DbManager>,
) -> Result<crate::db::AIRole, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::update_role(&conn, &role_id, &name, &description).await.map_err(|e| e.to_string())
}

// 删除角色
#[tauri::command]
pub async fn delete_ai_role(
    role_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::delete_role(&conn, &role_id).await.map_err(|e| e.to_string())
}

// 获取指定ID的角色
#[tauri::command]
pub async fn get_ai_role(
    role_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<crate::db::AIRole, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::get_role_by_id(&conn, &role_id).await.map_err(|e| e.to_string())
}
