use crate::db::UnifiedDbManager;
use tauri::State;

// TODO: AI角色功能暂时禁用，等实现相关数据库操作后再启用

// 获取所有角色
#[tauri::command]
pub async fn list_ai_roles(
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<crate::db::models::AIRole>, String> {
    Ok(Vec::new()) // 临时返回空列表
}

// 创建角色
#[tauri::command]
pub async fn create_ai_role(
    _name: String,
    _description: String,
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<crate::db::models::AIRole, String> {
    Err("AI角色功能暂未实现".to_string())
}

// 更新角色
#[tauri::command]
pub async fn update_ai_role(
    _role_id: String,
    _name: String,
    _description: String,
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<crate::db::models::AIRole, String> {
    Err("AI角色功能暂未实现".to_string())
}

// 删除角色
#[tauri::command]
pub async fn delete_ai_role(
    _role_id: String,
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    Err("AI角色功能暂未实现".to_string())
}

// 获取指定ID的角色
#[tauri::command]
pub async fn get_ai_role(
    role_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<crate::db::models::AIRole, String> {
    get_ai_role_internal(role_id, db_manager.inner().clone()).await
}

pub async fn get_ai_role_internal(
    _role_id: String,
    _db_manager: UnifiedDbManager,
) -> Result<crate::db::models::AIRole, String> {
    Err("AI role functionality is not yet implemented".to_string())
}
