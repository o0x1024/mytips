use reqwest::{Client, Proxy};
use serde::{Deserialize, Serialize};
use std::time::Duration;
// 代理设置
#[derive(Debug, Serialize, Deserialize)]
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
pub async fn get_client_with_proxy() -> Result<Client, String> {
    let proxy_settings = get_proxy_settings_internal().await?;

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
pub async fn get_proxy_settings_internal() -> Result<ProxySettings, String> {
    let app_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips");
    let config_dir = app_dir.join("config");
    let proxy_file = config_dir.join("proxy_settings.json");

    if !proxy_file.exists() {
        // 返回默认设置
        return Ok(ProxySettings {
            enabled: false,
            r#type: "http".to_string(),
            host: "127.0.0.1".to_string(),
            port: 10809,
            auth: false,
            username: "".to_string(),
            password: "".to_string(),
        });
    }

    let proxy_json = std::fs::read_to_string(proxy_file).map_err(|e| e.to_string())?;
    let proxy_settings: ProxySettings =
        serde_json::from_str(&proxy_json).map_err(|e| e.to_string())?;

    Ok(proxy_settings)
}

// 保存代理设置
#[tauri::command]
pub async fn save_proxy_settings(
    app: tauri::AppHandle,
    proxy_settings: ProxySettings,
) -> Result<(), String> {
    let app_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips");
    let config_dir = app_dir.join("config");

    // 确保目录存在
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    // 保存配置
    let proxy_file = config_dir.join("proxy_settings.json");
    let proxy_json = serde_json::to_string(&proxy_settings).map_err(|e| e.to_string())?;
    std::fs::write(proxy_file, proxy_json).map_err(|e| e.to_string())?;

    Ok(())
}

// 获取代理设置
#[tauri::command]
pub async fn get_proxy_settings(app: tauri::AppHandle) -> Result<ProxySettings, String> {
    let app_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips");
    let config_dir = app_dir.join("config");
    let proxy_file = config_dir.join("proxy_settings.json");

    if !proxy_file.exists() {
        // 返回默认设置
        return Ok(ProxySettings {
            enabled: false,
            r#type: "http".to_string(),
            host: "127.0.0.1".to_string(),
            port: 10809,
            auth: false,
            username: "".to_string(),
            password: "".to_string(),
        });
    }

    let proxy_json = std::fs::read_to_string(proxy_file).map_err(|e| e.to_string())?;
    let proxy_settings: ProxySettings =
        serde_json::from_str(&proxy_json).map_err(|e| e.to_string())?;

    Ok(proxy_settings)
}

// 测试代理连接
#[tauri::command]
pub async fn test_proxy_connection(proxy_settings: ProxySettings) -> Result<String, String> {
    if !proxy_settings.enabled {
        return Err("代理未启用".to_string());
    }

    // 先保存设置
    let app_dir = dirs::data_dir()
        .ok_or_else(|| "无法获取应用数据目录".to_string())?
        .join("mytips");
    let config_dir = app_dir.join("config");

    // 确保目录存在
    std::fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    // 保存配置
    let proxy_file = config_dir.join("proxy_settings.json");
    let proxy_json = serde_json::to_string(&proxy_settings).map_err(|e| e.to_string())?;
    std::fs::write(proxy_file, proxy_json).map_err(|e| e.to_string())?;

    // 获取带有代理的客户端
    let client = get_client_with_proxy().await?;

    // 测试连接 (使用google.com作为测试站点)
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
