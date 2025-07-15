use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, Manager, State};
use crate::db::{self, DbManager};

// 代理设置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProxySettings {
    pub enabled: bool,
    pub r#type: String,
    pub host: String,
    pub port: u16,
    pub auth: bool,
    pub username: String,
    pub password: String,
}

// 获取带有代理设置的HTTP客户端
pub async fn get_client_with_proxy(db_manager: &DbManager) -> Result<Client, String> {
    let proxy_settings = get_proxy_settings_internal(db_manager).await?;

    let mut client_builder = Client::builder().timeout(Duration::from_secs(30));

    if proxy_settings.enabled {
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.r#type, proxy_settings.host, proxy_settings.port
        );

        let mut proxy = Proxy::all(&proxy_url).map_err(|e| format!("创建代理失败: {}", e))?;

        if proxy_settings.auth {
            proxy = proxy.basic_auth(&proxy_settings.username, &proxy_settings.password);
        }

        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    Ok(client)
}

// 内部函数：获取代理设置（不依赖AppHandle）
pub async fn get_proxy_settings_internal(db_manager: &DbManager) -> Result<ProxySettings, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    match db::get_setting(&conn, "proxy_settings") {
        Ok(Some(settings_json)) => {
            serde_json::from_str(&settings_json).map_err(|e| e.to_string())
        }
        Ok(None) => {
            // 返回默认设置
            Ok(ProxySettings {
                enabled: false,
                r#type: "http".to_string(),
                host: "127.0.0.1".to_string(),
                port: 10809,
                auth: false,
                username: "".to_string(),
                password: "".to_string(),
            })
        }
        Err(e) => Err(e.to_string()),
    }
}

// 保存代理设置到数据库
#[tauri::command]
pub async fn save_proxy_settings(
    db_manager: State<'_, DbManager>,
    proxy_settings: ProxySettings,
) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let proxy_json = serde_json::to_string(&proxy_settings).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "proxy_settings", &proxy_json).map_err(|e| e.to_string())?;
    Ok(())
}

// 从数据库获取代理设置
#[tauri::command]
pub async fn get_proxy_settings(db_manager: State<'_, DbManager>) -> Result<ProxySettings, String> {
    get_proxy_settings_internal(&db_manager).await
}

// 测试代理连接
#[tauri::command]
pub async fn test_proxy_connection(
    db_manager: State<'_, DbManager>,
    proxy_settings: ProxySettings,
) -> Result<String, String> {
    if !proxy_settings.enabled {
        return Err("代理未启用".to_string());
    }

    // 先保存设置
    save_proxy_settings(db_manager.clone(), proxy_settings.clone()).await?;

    // 获取带有代理的客户端
    let client = get_client_with_proxy(&db_manager).await?;

    // 测试连接
    let response = client
        .get("https://www.google.com")
        .send()
        .await
        .map_err(|e| format!("连接测试失败: {}", e))?;

    if response.status().is_success() {
        Ok("代理连接测试成功".to_string())
    } else {
        Err(format!("代理连接测试失败，状态码: {}", response.status()))
    }
}
