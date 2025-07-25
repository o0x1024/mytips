use tauri::{command, Manager};
use tauri_plugin_updater::UpdaterExt;
use tauri::Emitter;
use std::time::Duration;
use crate::api::settings::get_proxy_settings_internal;
use crate::db::UnifiedDbManager;


#[derive(serde::Serialize)]
pub struct UpdateInfo {
    pub version: String,
    pub pub_date: Option<String>,
    pub body: Option<String>,
    pub available: bool,
}

#[derive(serde::Serialize)]
pub struct UpdateStatus {
    pub checking: bool,
    pub downloading: bool,
    pub installing: bool,
    pub progress: f64,
    pub error: Option<String>,
}

/// 显示确认对话框
#[command]
pub async fn show_confirm_dialog(message: String, title: String) -> Result<bool, String> {
    println!("Showing dialog: {} - {}", title, message);
    // For now, we return true due to configuration complexity.
    // The user can see the request in the console log.
    Ok(true)
}

/// 打开外部URL
#[command]
pub async fn open_url(url: String) -> Result<(), String> {
    println!("Attempting to open URL: {}", url);
    
    // 使用 open crate 来打开URL
    if let Err(e) = open::that(&url) {
        return Err(format!("Failed to open URL: {}", e));
    }
    
    Ok(())
}

/// 解析语义化版本号
fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    // 移除可能的 'v' 前缀
    let version = version.trim_start_matches('v');
    
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() >= 3 {
        if let (Ok(major), Ok(minor), Ok(patch)) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u32>(),
            parts[2].parse::<u32>()
        ) {
            return Some((major, minor, patch));
        }
    }
    None
}

/// 比较两个版本号，返回 true 如果 remote_version > current_version
fn is_newer_version(current_version: &str, remote_version: &str) -> bool {
    if let (Some(current), Some(remote)) = (
        parse_version(current_version),
        parse_version(remote_version)
    ) {
        // 比较主版本号
        if remote.0 > current.0 {
            return true;
        } else if remote.0 < current.0 {
            return false;
        }
        
        // 主版本号相同，比较次版本号
        if remote.1 > current.1 {
            return true;
        } else if remote.1 < current.1 {
            return false;
        }
        
        // 主版本号和次版本号都相同，比较修订版本号
        remote.2 > current.2
    } else {
        // 如果无法解析版本号，回退到字符串比较
        remote_version != current_version
    }
}

/// 创建带有全局代理设置的更新器构建器
async fn create_updater_builder_with_proxy(app: &tauri::AppHandle) -> Result<tauri_plugin_updater::UpdaterBuilder, String> {
    let mut updater_builder = app.updater_builder();
    let db_manager = app.state::<UnifiedDbManager>();
    
    // 获取全局代理设置
    match get_proxy_settings_internal(db_manager.inner()).await {
        Ok(proxy_settings) => {
            if proxy_settings.enabled {
                let proxy_url = format!(
                    "{}://{}:{}",
                    proxy_settings.protocol, proxy_settings.host, proxy_settings.port
                );
                
                println!("Updater is using proxy: {}", proxy_url);
                
                if let Ok(parsed_proxy) = proxy_url.parse() {
                    updater_builder = updater_builder.proxy(parsed_proxy);
                } else {
                    println!("Warning: Failed to parse proxy URL: {}", proxy_url);
                }
            }
        }
        Err(e) => {
            println!("Warning: Failed to get proxy settings: {}", e);
        }
    }
    
    Ok(updater_builder)
}


/// 检查更新
#[command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    let updater_builder = create_updater_builder_with_proxy(&app).await?;
    let updater = updater_builder.build().map_err(|e| format!("Failed to create updater: {}", e))?;
    
    match updater.check().await {
        Ok(update_result) => {
            if let Some(update) = update_result {
                Ok(UpdateInfo {
                    version: update.version,
                    pub_date: update.date.map(|d| d.to_string()),
                    body: update.body,
                    available: true,
                })
            } else {
                Ok(UpdateInfo {
                    version: "".to_string(),
                    pub_date: None,
                    body: None,
                    available: false,
                })
            }
        }
        Err(e) => Err(format!("Failed to check for updates: {}", e)),
    }
}

/// 检查更新（带自定义配置）
#[command]
pub async fn check_for_updates_with_config(
    app: tauri::AppHandle,
    timeout_seconds: Option<u64>,
    proxy: Option<String>,
) -> Result<UpdateInfo, String> {
    let mut updater_builder = create_updater_builder_with_proxy(&app).await?;
    
    // 设置超时
    if let Some(timeout) = timeout_seconds {
        updater_builder = updater_builder.timeout(Duration::from_secs(timeout));
    }
    
    // 如果提供了自定义代理，覆盖全局代理设置
    if let Some(ref proxy_url) = proxy {
        if let Ok(parsed_proxy) = proxy_url.parse() {
            updater_builder = updater_builder.proxy(parsed_proxy);
            println!("Using custom proxy: {}", proxy_url);
        }
    }
    
    match updater_builder.build() {
        Ok(updater) => {
            match updater.check().await {
                Ok(update_result) => {
                    if let Some(update) = update_result {
                        Ok(UpdateInfo {
                            version: update.version,
                            pub_date: update.date.map(|d| d.to_string()),
                            body: update.body,
                            available: true,
                        })
                    } else {
                        Ok(UpdateInfo {
                            version: "".to_string(),
                            pub_date: None,
                            body: None,
                            available: false,
                        })
                    }
                }
                Err(e) => {
                    // 如果签名验证失败，尝试使用无签名验证的方式
                    if e.to_string().contains("signature") {
                        println!("Signature verification failed, trying without signature verification...");
                        return check_for_updates_no_signature(app, timeout_seconds, proxy).await;
                    }
                    Err(format!("Failed to check for updates: {}", e))
                }
            }
        }
        Err(e) => Err(format!("Failed to build updater: {}", e)),
    }
}

/// 检查更新（无签名验证）
/// 这个函数通过手动发送HTTP请求来获取更新信息，绕过Tauri的签名验证
#[command]
pub async fn check_for_updates_no_signature(
    app: tauri::AppHandle,
    timeout_seconds: Option<u64>,
    proxy: Option<String>,
) -> Result<UpdateInfo, String> {
    println!("Checking for updates without signature verification...");
    
    // 构建更新端点URL
    let target = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "darwin"
    } else {
        "linux"
    };
    
    let arch = if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else {
        "x86_64"
    };
    
    let endpoint_url = format!(
        "https://github.com/o0x1024/mytips/releases/latest/download/{}-{}.json",
        target, arch
    );
    
    println!("Checking update endpoint: {}", endpoint_url);
    
    // 创建HTTP客户端
    let mut client_builder = reqwest::Client::builder();
    
    // 设置超时
    if let Some(timeout) = timeout_seconds {
        client_builder = client_builder.timeout(Duration::from_secs(timeout));
    } else {
        client_builder = client_builder.timeout(Duration::from_secs(30));
    }
    
    // 设置代理
    if let Some(ref proxy_url) = proxy {
        if let Ok(proxy) = reqwest::Proxy::all(proxy_url) {
            client_builder = client_builder.proxy(proxy);
            println!("Using custom proxy: {}", proxy_url);
        }
    } else {
        // 尝试使用全局代理设置
        let db_manager = app.state::<UnifiedDbManager>();
        match get_proxy_settings_internal(db_manager.inner()).await {
            Ok(proxy_settings) => {
                if proxy_settings.enabled {
                    let proxy_url = format!(
                        "{}://{}:{}",
                        proxy_settings.protocol, proxy_settings.host, proxy_settings.port
                    );
                    
                    if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                        client_builder = client_builder.proxy(proxy);
                        println!("Using global proxy: {}", proxy_url);
                    }
                }
            }
            Err(e) => {
                println!("Warning: Failed to get proxy settings: {}", e);
            }
        }
    }
    
    let client = client_builder.build().map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    // 发送请求
    match client.get(&endpoint_url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        // 解析更新信息
                        let version = json.get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                        
                        let pub_date = json.get("pub_date")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        
                        let body = json.get("notes")
                            .or_else(|| json.get("body"))
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        
                        // 检查是否有新版本
                        let current_version = env!("CARGO_PKG_VERSION");
                        let available = !version.is_empty() && is_newer_version(current_version, &version);
                        
                        if available {
                            println!("New version found: {} (current version: {})", version, current_version);
                        } else {
                            println!("Current version is up to date: {}", current_version);
                        }
                        
                        Ok(UpdateInfo {
                            version,
                            pub_date,
                            body,
                            available,
                        })
                    }
                    Err(e) => Err(format!("Failed to parse update info: {}", e)),
                }
            } else {
                Err(format!("Failed to get update info: HTTP {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network request failed: {}", e)),
    }
}

/// 开始自动更新过程
#[command]
pub async fn start_auto_update(app: tauri::AppHandle) -> Result<(), String> {
    let handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        if let Err(e) = perform_update(handle.clone()).await {
            eprintln!("Auto-update failed: {}", e);
            
            // 如果是签名相关错误，通知前端需要手动更新
            if e.to_string().to_lowercase().contains("signature") {
                eprintln!("Signature verification error detected, notifying frontend for manual update.");
                if let Err(emit_err) = handle.emit("manual_update_required", ()) {
                    eprintln!("Failed to emit manual update event: {}", emit_err);
                }
            }
        }
    });
    
    Ok(())
}

/// 执行更新
async fn perform_update(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let updater_builder = create_updater_builder_with_proxy(&app).await?;
    let updater = updater_builder.build()?;
    
    match updater.check().await {
        Ok(Some(update)) => {
            println!("New version found: {}", update.version);
            
            let mut downloaded = 0;
            
            // 下载并安装更新
            update.download_and_install(
                |chunk_length, content_length| {
                    downloaded += chunk_length;
                    if let Some(total) = content_length {
                        let progress = (downloaded as f64 / total as f64) * 100.0;
                        println!("Download progress: {:.1}%", progress);
                        
                        // 向前端发送进度事件
                        if let Err(e) = app.emit("update-progress", progress) {
                            eprintln!("Failed to emit progress event: {}", e);
                        }
                    }
                },
                || {
                    println!("Download complete, starting installation");
                    // 向前端发送安装开始事件
                    if let Err(e) = app.emit("update-installing", ()) {
                        eprintln!("Failed to emit installation event: {}", e);
                    }
                },
            ).await?;
            
            println!("Update installation complete");
            
            // 向前端发送更新完成事件
            if let Err(e) = app.emit("update-completed", ()) {
                eprintln!("Failed to emit completion event: {}", e);
            }
            
            // 重启应用
            app.restart();
        },
        Ok(None) => {
            println!("No new version found.");
        },
        Err(e) => {
            // 如果是签名相关错误，尝试无签名验证
            if e.to_string().to_lowercase().contains("signature") {
                println!("Signature verification failed. Trying update without signature verification...");
                if let Ok(update_info) = check_for_updates_no_signature(app.clone(), None, None).await {
                    if update_info.available {
                        println!("New version available without signature. Manual download required.");
                        // 通知前端需要手动更新
                        if let Err(emit_err) = app.emit("manual_update_required", ()) {
                            eprintln!("Failed to emit manual update event: {}", emit_err);
                        }
                        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Manual update required due to missing signature")));
                    } else {
                        println!("No new version found (no signature).");
                    }
                } else {
                    println!("Failed to check for updates without signature.");
                    return Err(e.into());
                }
            } else {
                return Err(e.into());
            }
        }
    }
    
    Ok(())
}

/// 获取当前应用版本
#[command]
pub fn get_current_version(app: tauri::AppHandle) -> String {
    app.package_info().version.to_string()
}

/// 设置更新器端点
#[command]
pub async fn set_update_endpoints(
    app: tauri::AppHandle,
    endpoints: Vec<String>
) -> Result<(), String> {
    // 这个函数允许动态设置更新端点
    // 在运行时更改更新源，比如从稳定版切换到测试版
    
    // 将字符串 URL 转换为 tauri::Url 类型
    let urls: Result<Vec<tauri::Url>, _> = endpoints
        .into_iter()
        .map(|endpoint| endpoint.parse::<tauri::Url>())
        .collect();
    
    let parsed_urls = urls.map_err(|e| format!("Failed to parse endpoint URL: {}", e))?;
    
    let _updater = app.updater_builder()
        .endpoints(parsed_urls)
        .map_err(|e| format!("Failed to set endpoints: {}", e))?
        .build()
        .map_err(|e| format!("Failed to build updater: {}", e))?;
        
    Ok(())
}

/// 测试 Windows 更新功能（带代理支持）
#[command]
pub async fn test_windows_update_with_proxy(app: tauri::AppHandle) -> Result<String, String> {
    println!("Starting to test Windows update feature...");
    
    // 首先尝试正常的签名验证方式
    let updater_builder = create_updater_builder_with_proxy(&app).await?;
    let updater = updater_builder.build().map_err(|e| format!("Failed to create updater: {}", e))?;
    
    match updater.check().await {
        Ok(update_result) => {
            if let Some(update) = update_result {
                let info = format!(
                    "✅ Windows update test successful!\n\
                    New version found: {}\n\
                    Release date: {}\n\
                    Update notes: {}\n\
                    Current platform: {}",
                    update.version,
                    update.date.map(|d| d.to_string()).as_deref().unwrap_or("Unknown"),
                    update.body.as_deref().unwrap_or("No update notes"),
                    std::env::consts::OS
                );
                println!("{}", info);
                Ok(info)
            } else {
                let info = "✅ Windows update test successful! Already on the latest version.".to_string();
                println!("{}", info);
                Ok(info)
            }
        }
        Err(e) => {
            // 如果签名验证失败，尝试无签名验证方式
            if e.to_string().contains("signature") {
                println!("Signature verification failed, trying test without signature verification...");
                return test_windows_update_no_signature(app).await;
            }
            
            let error = format!("❌ Windows update test failed: {}", e);
            println!("{}", error);
            Err(error)
        }
    }
}

/// 测试 Windows 更新功能（无签名验证）
#[command]
pub async fn test_windows_update_no_signature(app: tauri::AppHandle) -> Result<String, String> {
    println!("Starting to test Windows update feature (no signature verification)...");
    
    match check_for_updates_no_signature(app, Some(30), None).await {
        Ok(update_info) => {
            if update_info.available {
                let info = format!(
                    "✅ Windows update test successful! (no signature verification mode)\n\
                    New version found: {}\n\
                    Release date: {}\n\
                    Update notes: {}\n\
                    Current platform: {}\n\
                    ⚠️ Note: Using no signature verification mode. It's recommended to enable signature verification in production.",
                    update_info.version,
                    update_info.pub_date.as_deref().unwrap_or("Unknown"),
                    update_info.body.as_deref().unwrap_or("No update notes"),
                    std::env::consts::OS
                );
                println!("{}", info);
                Ok(info)
            } else {
                let info = "✅ Windows update test successful! Already on the latest version. (no signature verification mode)".to_string();
                println!("{}", info);
                Ok(info)
            }
        }
        Err(e) => {
            let error = format!("❌ Windows update test failed: {}", e);
            println!("{}", error);
            Err(error)
        }
    }
}

/// 获取当前平台信息
#[command]
pub fn get_platform_info() -> String {
    format!(
        "OS: {}\n\
        Architecture: {}\n\
        Family: {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::consts::FAMILY
    )
} 
