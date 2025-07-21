use serde::{Deserialize, Serialize};
use tauri::{Manager, State, AppHandle};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub modifiers: Vec<String>,
    pub key: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            modifiers: vec!["meta".to_string(), "shift".to_string()],
            key: "c".to_string(),
        }
    }
}

/// 获取全局快捷键配置
#[tauri::command]
pub async fn get_global_shortcut_config(
    db_manager: State<'_, crate::db::UnifiedDbManager>,
) -> Result<ShortcutConfig, String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    match crate::db::operations::get_setting(&conn, "global_shortcut").await {
        Ok(Some(config_str)) => match serde_json::from_str::<ShortcutConfig>(&config_str) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("Failed to parse global shortcut config: {}", e);
                Ok(ShortcutConfig::default())
            }
        },
        Ok(None) => Ok(ShortcutConfig::default()),
        Err(e) => {
            eprintln!("Failed to get global shortcut config: {}", e);
            Err(format!("Failed to get global shortcut config: {}", e))
        }
    }
}

/// 更新全局快捷键配置
#[tauri::command]
pub async fn update_global_shortcut(
    db_manager: State<'_, crate::db::UnifiedDbManager>,
    app: AppHandle,
    config: ShortcutConfig,
) -> Result<(), String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    let config_str = match serde_json::to_string(&config) {
        Ok(s) => s,
        Err(e) => return Err(format!("Failed to serialize shortcut config: {}", e)),
    };

    match crate::db::operations::save_setting(&conn, "global_shortcut", &config_str).await {
        Ok(_) => {
            // 重新注册全局快捷键
            #[cfg(desktop)]
            {
                if let Err(e) = crate::global_shortcut::update_global_shortcut(&app, &config).await {
                    return Err(format!("Failed to update global shortcut: {}", e));
                }
            }
            Ok(())
        },
        Err(e) => Err(format!("Failed to save global shortcut config: {}", e)),
    }
}
