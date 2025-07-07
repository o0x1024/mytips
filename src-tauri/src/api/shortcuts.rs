use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

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
pub fn get_global_shortcut_config(app: AppHandle) -> Result<ShortcutConfig, String> {
    let db_state = app
        .try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;

    let db = db_state
        .lock()
        .map_err(|e| format!("锁定数据库失败: {}", e))?;

    match db.get_setting("global_shortcut") {
        Ok(Some(config_str)) => match serde_json::from_str::<ShortcutConfig>(&config_str) {
            Ok(config) => Ok(config),
            Err(e) => {
                eprintln!("解析全局快捷键配置失败: {}", e);
                Ok(ShortcutConfig::default())
            }
        },
        Ok(None) => Ok(ShortcutConfig::default()),
        Err(e) => {
            eprintln!("获取全局快捷键配置失败: {}", e);
            Err(format!("获取全局快捷键配置失败: {}", e))
        }
    }
}

/// 更新全局快捷键配置
#[tauri::command]
pub fn update_global_shortcut(app: AppHandle, config: ShortcutConfig) -> Result<(), String> {
    let db_state = app
        .try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;

    let db = db_state
        .lock()
        .map_err(|e| format!("锁定数据库失败: {}", e))?;

    let config_str = match serde_json::to_string(&config) {
        Ok(s) => s,
        Err(e) => return Err(format!("序列化快捷键配置失败: {}", e)),
    };

    match db.save_setting("global_shortcut", &config_str) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("保存全局快捷键配置失败: {}", e)),
    }
}
