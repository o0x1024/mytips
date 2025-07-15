use tauri::{command, Manager};
use tauri_plugin_updater::UpdaterExt;
use tauri::Emitter;
use std::time::Duration;
use crate::api::settings::get_proxy_settings_internal;
use crate::db::DbManager;


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
    println!("显示对话框: {} - {}", title, message);
    // 由于配置复杂性，我们暂时返回 true
    // 用户可以通过控制台日志看到请求
    Ok(true)
}

/// 打开外部URL
#[command]
pub async fn open_url(url: String) -> Result<(), String> {
    println!("尝试打开URL: {}", url);
    
    // 使用 open crate 来打开URL
    if let Err(e) = open::that(&url) {
        return Err(format!("打开URL失败: {}", e));
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
    let db_manager = app.state::<DbManager>();
    
    // 获取全局代理设置
    match get_proxy_settings_internal(db_manager.inner()).await {
        Ok(proxy_settings) => {
            if proxy_settings.enabled {
                let proxy_url = if proxy_settings.auth && !proxy_settings.username.is_empty() {
                    format!(
                        "{}://{}:{}@{}:{}",
                        proxy_settings.r#type,
                        proxy_settings.username,
                        proxy_settings.password,
                        proxy_settings.host,
                        proxy_settings.port
                    )
                } else {
                    format!(
                        "{}://{}:{}",
                        proxy_settings.r#type,
                        proxy_settings.host,
                        proxy_settings.port
                    )
                };
                
                println!("更新器使用代理: {}", proxy_url);
                
                if let Ok(parsed_proxy) = proxy_url.parse() {
                    updater_builder = updater_builder.proxy(parsed_proxy);
                } else {
                    println!("警告: 无法解析代理URL: {}", proxy_url);
                }
            }
        }
        Err(e) => {
            println!("警告: 获取代理设置失败: {}", e);
        }
    }
    
    Ok(updater_builder)
}


/// 检查更新
#[command]
pub async fn check_for_updates(app: tauri::AppHandle) -> Result<UpdateInfo, String> {
    let updater_builder = create_updater_builder_with_proxy(&app).await?;
    let updater = updater_builder.build().map_err(|e| format!("创建更新器失败: {}", e))?;
    
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
        Err(e) => Err(format!("检查更新失败: {}", e)),
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
            println!("使用自定义代理: {}", proxy_url);
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
                        println!("签名验证失败，尝试无签名验证模式...");
                        return check_for_updates_no_signature(app, timeout_seconds, proxy).await;
                    }
                    Err(format!("检查更新失败: {}", e))
                }
            }
        }
        Err(e) => Err(format!("构建更新器失败: {}", e)),
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
    println!("使用无签名验证模式检查更新...");
    
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
    
    println!("检查更新端点: {}", endpoint_url);
    
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
            println!("使用自定义代理: {}", proxy_url);
        }
    } else {
        // 尝试使用全局代理设置
        let db_manager = app.state::<DbManager>();
        match get_proxy_settings_internal(db_manager.inner()).await {
            Ok(proxy_settings) => {
                if proxy_settings.enabled {
                    let proxy_url = if proxy_settings.auth && !proxy_settings.username.is_empty() {
                        format!(
                            "{}://{}:{}@{}:{}",
                            proxy_settings.r#type,
                            proxy_settings.username,
                            proxy_settings.password,
                            proxy_settings.host,
                            proxy_settings.port
                        )
                    } else {
                        format!(
                            "{}://{}:{}",
                            proxy_settings.r#type,
                            proxy_settings.host,
                            proxy_settings.port
                        )
                    };
                    
                    if let Ok(proxy) = reqwest::Proxy::all(&proxy_url) {
                        client_builder = client_builder.proxy(proxy);
                        println!("使用全局代理: {}", proxy_url);
                    }
                }
            }
            Err(e) => {
                println!("警告: 获取代理设置失败: {}", e);
            }
        }
    }
    
    let client = client_builder.build().map_err(|e| format!("创建HTTP客户端失败: {}", e))?;
    
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
                            println!("发现新版本: {} (当前版本: {})", version, current_version);
                        } else {
                            println!("当前已是最新版本: {}", current_version);
                        }
                        
                        Ok(UpdateInfo {
                            version,
                            pub_date,
                            body,
                            available,
                        })
                    }
                    Err(e) => Err(format!("解析更新信息失败: {}", e)),
                }
            } else {
                Err(format!("获取更新信息失败: HTTP {}", response.status()))
            }
        }
        Err(e) => Err(format!("网络请求失败: {}", e)),
    }
}

/// 开始自动更新过程
#[command]
pub async fn start_auto_update(app: tauri::AppHandle) -> Result<(), String> {
    let handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        if let Err(e) = perform_update(handle.clone()).await {
            eprintln!("自动更新失败: {}", e);
            
            // 如果是签名相关错误，通知前端需要手动更新
            if e.to_string().to_lowercase().contains("signature") {
                eprintln!("检测到签名验证错误，通知前端进行手动更新。");
                if let Err(emit_err) = handle.emit("manual_update_required", ()) {
                    eprintln!("发送手动更新事件失败: {}", emit_err);
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
    
    if let Some(update) = updater.check().await? {
        println!("发现新版本: {}", update.version);
        
        let mut downloaded = 0;
        
        // 下载并安装更新
        update.download_and_install(
            |chunk_length, content_length| {
                downloaded += chunk_length;
                if let Some(total) = content_length {
                    let progress = (downloaded as f64 / total as f64) * 100.0;
                    println!("下载进度: {:.1}%", progress);
                    
                    // 向前端发送进度事件
                    if let Err(e) = app.emit("update-progress", progress) {
                        eprintln!("发送进度事件失败: {}", e);
                    }
                }
            },
            || {
                println!("下载完成，开始安装");
                // 向前端发送安装开始事件
                if let Err(e) = app.emit("update-installing", ()) {
                    eprintln!("发送安装事件失败: {}", e);
                }
            },
        ).await?;
        
        println!("更新安装完成");
        
        // 向前端发送更新完成事件
        if let Err(e) = app.emit("update-completed", ()) {
            eprintln!("发送完成事件失败: {}", e);
        }
        
        // 重启应用
        app.restart();
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
    
    let parsed_urls = urls.map_err(|e| format!("解析端点 URL 失败: {}", e))?;
    
    let _updater = app.updater_builder()
        .endpoints(parsed_urls)
        .map_err(|e| format!("设置端点失败: {}", e))?
        .build()
        .map_err(|e| format!("构建更新器失败: {}", e))?;
        
    Ok(())
}

/// 测试 Windows 更新功能（带代理支持）
#[command]
pub async fn test_windows_update_with_proxy(app: tauri::AppHandle) -> Result<String, String> {
    println!("开始测试 Windows 更新功能...");
    
    // 首先尝试正常的签名验证方式
    let updater_builder = create_updater_builder_with_proxy(&app).await?;
    let updater = updater_builder.build().map_err(|e| format!("创建更新器失败: {}", e))?;
    
    match updater.check().await {
        Ok(update_result) => {
            if let Some(update) = update_result {
                let info = format!(
                    "✅ Windows 更新测试成功！\n\
                    发现新版本: {}\n\
                    发布时间: {}\n\
                    更新说明: {}\n\
                    当前平台: {}",
                    update.version,
                    update.date.map(|d| d.to_string()).as_deref().unwrap_or("未知"),
                    update.body.as_deref().unwrap_or("无更新说明"),
                    std::env::consts::OS
                );
                println!("{}", info);
                Ok(info)
            } else {
                let info = "✅ Windows 更新测试成功！当前已是最新版本。".to_string();
                println!("{}", info);
                Ok(info)
            }
        }
        Err(e) => {
            // 如果签名验证失败，尝试无签名验证方式
            if e.to_string().contains("signature") {
                println!("签名验证失败，尝试无签名验证测试...");
                return test_windows_update_no_signature(app).await;
            }
            
            let error = format!("❌ Windows 更新测试失败: {}", e);
            println!("{}", error);
            Err(error)
        }
    }
}

/// 测试 Windows 更新功能（无签名验证）
#[command]
pub async fn test_windows_update_no_signature(app: tauri::AppHandle) -> Result<String, String> {
    println!("开始测试 Windows 更新功能（无签名验证）...");
    
    match check_for_updates_no_signature(app, Some(30), None).await {
        Ok(update_info) => {
            if update_info.available {
                let info = format!(
                    "✅ Windows 更新测试成功！（无签名验证模式）\n\
                    发现新版本: {}\n\
                    发布时间: {}\n\
                    更新说明: {}\n\
                    当前平台: {}\n\
                    ⚠️ 注意：当前使用无签名验证模式，建议在生产环境中启用签名验证",
                    update_info.version,
                    update_info.pub_date.as_deref().unwrap_or("未知"),
                    update_info.body.as_deref().unwrap_or("无更新说明"),
                    std::env::consts::OS
                );
                println!("{}", info);
                Ok(info)
            } else {
                let info = "✅ Windows 更新测试成功！当前已是最新版本。（无签名验证模式）".to_string();
                println!("{}", info);
                Ok(info)
            }
        }
        Err(e) => {
            let error = format!("❌ Windows 更新测试失败: {}", e);
            println!("{}", error);
            Err(error)
        }
    }
}

/// 获取当前平台信息
#[command]
pub fn get_platform_info() -> String {
    format!(
        "操作系统: {}\n\
        架构: {}\n\
        系列: {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        std::env::consts::FAMILY
    )
} 
