use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use std::str::FromStr;
use tracing::{info, warn, error};

use crate::api::shortcuts::ShortcutConfig;
use crate::db::UnifiedDbManager;

/// 初始化全局快捷键
pub async fn setup_global_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // 获取快捷键配置
    let config = match get_shortcut_config(app).await {
        Ok(config) => config,
        Err(e) => {
            warn!("Failed to load shortcut config, using defaults: {}", e);
            ShortcutConfig::default()
        }
    };

    // 注册快捷键
    register_shortcut(app, &config).await?;

    info!("Global shortcuts initialized successfully");
    Ok(())
}

/// 获取快捷键配置
async fn get_shortcut_config(app: &AppHandle) -> Result<ShortcutConfig, String> {
    let db_manager = app.state::<UnifiedDbManager>();
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get database connection: {}", e))?;

    match crate::db::operations::get_setting(&conn, "global_shortcut").await {
        Ok(Some(config_str)) => {
            match serde_json::from_str::<ShortcutConfig>(&config_str) {
                Ok(config) => Ok(config),
                Err(e) => {
                    warn!("Failed to parse shortcut config: {}", e);
                    Ok(ShortcutConfig::default())
                }
            }
        },
        Ok(None) => Ok(ShortcutConfig::default()),
        Err(e) => Err(format!("Failed to get shortcut config: {}", e)),
    }
}

/// 注册全局快捷键
async fn register_shortcut(app: &AppHandle, config: &ShortcutConfig) -> Result<(), Box<dyn std::error::Error>> {
    // 构建快捷键字符串
    let mut shortcut_parts = Vec::new();
    
    // 添加修饰键
    for modifier in &config.modifiers {
        match modifier.as_str() {
            "meta" | "cmd" | "super" => {
                #[cfg(target_os = "macos")]
                shortcut_parts.push("cmd");
                #[cfg(not(target_os = "macos"))]
                shortcut_parts.push("ctrl");
            },
            "ctrl" | "control" => shortcut_parts.push("ctrl"),
            "alt" | "option" => shortcut_parts.push("alt"),
            "shift" => shortcut_parts.push("shift"),
            _ => {
                warn!("Unknown modifier: {}", modifier);
            }
        }
    }
    
    // 添加按键
    shortcut_parts.push(&config.key);
    
    let shortcut_str = shortcut_parts.join("+");
    info!("Registering global shortcut: {}", shortcut_str);

    // 先取消之前的注册
    if let Err(e) = app.global_shortcut().unregister_all() {
        warn!("Failed to unregister previous shortcuts: {}", e);
    }

    // Convert shortcut_str to Shortcut type
    let shortcut = Shortcut::from_str(&shortcut_str)?;

    // 注册新的快捷键
    app.global_shortcut().register(shortcut)?;

    info!("Global shortcut registered successfully: {}", shortcut_str);
    Ok(())
}

/// 处理快捷键触发事件
pub async fn handle_shortcut_triggered(app: &AppHandle) -> Result<(), String> {
    info!("Processing shortcut trigger - adding selection to clipboard");
    
    // 调用已有的添加选中文本到剪贴板的功能
    match crate::api::clipboard_api::add_selection_to_clipboard(app.clone()).await {
        Ok(_) => {
            info!("Successfully added selection to clipboard via global shortcut");
            Ok(())
        }
        Err(e) => {
            error!("Failed to add selection to clipboard: {}", e);
            Err(e)
        }
    }
}

/// 更新全局快捷键
pub async fn update_global_shortcut(app: &AppHandle, config: &ShortcutConfig) -> Result<(), Box<dyn std::error::Error>> {
    info!("Updating global shortcut configuration");
    
    // 重新注册快捷键
    register_shortcut(app, config).await?;
    
    info!("Global shortcut updated successfully");
    Ok(())
} 