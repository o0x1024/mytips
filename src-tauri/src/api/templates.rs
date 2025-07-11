use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri::Manager;

use crate::db;
use crate::db::DbManager;

#[derive(Serialize, Deserialize, Clone)]
pub struct TipTemplate {
    pub name: String,
    pub content: String,
}

const TEMPLATE_KEY: &str = "tip_templates";

/// 获取全部提示词模板
#[tauri::command]
pub fn get_tip_templates(app: AppHandle) -> Result<Vec<TipTemplate>, String> {
    let db_state = app.state::<DbManager>();
    let conn = db_state.get_conn().map_err(|e| e.to_string())?;
    match db::get_setting(&conn, TEMPLATE_KEY) {
        Ok(Some(json_str)) => {
            let list: Vec<TipTemplate> = serde_json::from_str(&json_str).unwrap_or_default();
            Ok(list)
        }
        Ok(None) => Ok(vec![]),
        Err(e) => Err(e.to_string()),
    }
}

/// 保存（新增或更新）单个模板
#[tauri::command]
pub fn save_tip_template(app: AppHandle, template: TipTemplate) -> Result<(), String> {
    let db_state = app.state::<DbManager>();
    let conn = db_state.get_conn().map_err(|e| e.to_string())?;
    // 获取现有
    let mut templates: Vec<TipTemplate> = match db::get_setting(&conn, TEMPLATE_KEY).map_err(|e| e.to_string())? {
        Some(s) => serde_json::from_str(&s).unwrap_or_default(),
        None => vec![],
    };

    // 查看是否已有同名模板
    if let Some(existing) = templates.iter_mut().find(|t| t.name == template.name) {
        existing.content = template.content.clone();
    } else {
        templates.push(template);
    }

    let json_str = serde_json::to_string(&templates).map_err(|e| e.to_string())?;
    db::save_setting(&conn, TEMPLATE_KEY, &json_str).map_err(|e| e.to_string())?;
    Ok(())
}

/// 删除模板
#[tauri::command]
pub fn delete_tip_template(app: AppHandle, name: String) -> Result<(), String> {
    let db_state = app.state::<DbManager>();
    let conn = db_state.get_conn().map_err(|e| e.to_string())?;
    let mut templates: Vec<TipTemplate> = match db::get_setting(&conn, TEMPLATE_KEY).map_err(|e| e.to_string())? {
        Some(s) => serde_json::from_str(&s).unwrap_or_default(),
        None => vec![],
    };
    templates.retain(|t| t.name != name);
    let json_str = serde_json::to_string(&templates).map_err(|e| e.to_string())?;
    db::save_setting(&conn, TEMPLATE_KEY, &json_str).map_err(|e| e.to_string())?;
    Ok(())
} 