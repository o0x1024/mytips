use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tauri::{AppHandle, Manager, State};
use crate::db::{self, UnifiedDbManager};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct NetworkSettings {
    pub proxy: ProxySettings,
    pub web_server: WebServerSettings,
    pub sync_server: SyncServerSettings,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProxySettings {
    pub enabled: bool,
    pub protocol: String, // "http", "socks5"
    pub host: String,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebServerSettings {
    pub enabled: bool,
    pub port: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncServerSettings {
    pub sync_url: String,
}

impl Default for WebServerSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            port: 8080,
        }
    }
}

impl Default for SyncServerSettings {
    fn default() -> Self {
        Self {
            sync_url: "".to_string(),
        }
    }
}

pub async fn get_network_settings_from_db(
    conn: &crate::db::DbConnection,
) -> Result<NetworkSettings, String> {
    match crate::db::operations::get_setting(conn, "network_settings").await {
        Ok(Some(settings_str)) => serde_json::from_str(&settings_str)
            .map_err(|e| format!("Failed to parse network settings: {}", e)),
        Ok(None) => Ok(NetworkSettings::default()),
        Err(e) => Err(format!("Failed to get network settings from db: {}", e)),
    }
}

#[tauri::command]
pub async fn get_network_settings(app: AppHandle) -> Result<NetworkSettings, String> {
    let manager = app.state::<UnifiedDbManager>().inner();
    let conn = manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;
    get_network_settings_from_db(&conn).await
}

#[tauri::command]
pub async fn save_network_settings(
    db_manager: State<'_, UnifiedDbManager>,
    network_settings: NetworkSettings,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let network_json = serde_json::to_string(&network_settings).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "network_settings", &network_json).await.map_err(|e| e.to_string())?;
    Ok(())
}

// 获取带有代理设置的HTTP客户端
pub async fn get_client_with_proxy(db_manager: &UnifiedDbManager) -> Result<Client, String> {
    let proxy_settings = get_proxy_settings_internal(db_manager).await?;

    let mut client_builder = Client::builder().timeout(Duration::from_secs(30));

    if proxy_settings.enabled {
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.protocol, proxy_settings.host, proxy_settings.port
        );

        let proxy = Proxy::all(&proxy_url).map_err(|e| format!("创建代理失败: {}", e))?;

        client_builder = client_builder.proxy(proxy);
    }

    let client = client_builder
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    Ok(client)
}

// 内部函数：获取代理设置（不依赖AppHandle）
pub async fn get_proxy_settings_internal(db_manager: &UnifiedDbManager) -> Result<ProxySettings, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    match db::get_setting(&conn, "proxy_settings").await {
        Ok(Some(settings_json)) => {
            // 尝试直接解析为新格式
            if let Ok(settings) = serde_json::from_str(&settings_json) {
                return Ok(settings);
            }

            // 如果失败，则尝试作为通用JSON值解析，以处理旧格式
            let v: Value = serde_json::from_str(&settings_json)
                .map_err(|e| format!("无法解析代理设置JSON: {}", e))?;

            // 从旧格式("type")或新格式("protocol")中获取协议
            let protocol = v.get("protocol")
                .and_then(Value::as_str)
                .or_else(|| v.get("type").and_then(Value::as_str)) // 兼容旧的 `type` 字段
                .unwrap_or("http") // 如果两者都不存在，则默认为 "http"
                .to_string();

            Ok(ProxySettings {
                enabled: v.get("enabled").and_then(Value::as_bool).unwrap_or(false),
                protocol,
                host: v.get("host").and_then(Value::as_str).unwrap_or("").to_string(),
                port: v.get("port").and_then(Value::as_u64).unwrap_or(0) as u16,
            })
        }
        Ok(None) => Ok(ProxySettings::default()),
        Err(e) => Err(e.to_string()),
    }
}

// 保存代理设置到数据库
#[tauri::command]
pub async fn save_proxy_settings(
    db_manager: State<'_, UnifiedDbManager>,
    proxy_settings: ProxySettings,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let proxy_json = serde_json::to_string(&proxy_settings).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "proxy_settings", &proxy_json).await.map_err(|e| e.to_string())?;
    Ok(())
}

// 从数据库获取代理设置
#[tauri::command]
pub async fn get_proxy_settings(db_manager: State<'_, UnifiedDbManager>) -> Result<ProxySettings, String> {
    get_proxy_settings_internal(&db_manager).await
}

// 测试代理连接
#[tauri::command]
pub async fn test_proxy_connection(
    db_manager: State<'_, UnifiedDbManager>,
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
