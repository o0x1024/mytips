use tauri::command;
use tauri_plugin_updater::UpdaterExt;
use tauri::Emitter;
use std::time::Duration;
use crate::api::settings::get_proxy_settings_internal;

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

/// 创建带有全局代理设置的更新器构建器
async fn create_updater_builder_with_proxy(app: &tauri::AppHandle) -> Result<tauri_plugin_updater::UpdaterBuilder, String> {
    let mut updater_builder = app.updater_builder();
    
    // 获取全局代理设置
    match get_proxy_settings_internal().await {
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
    if let Some(proxy_url) = proxy {
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
                Err(e) => Err(format!("检查更新失败: {}", e)),
            }
        }
        Err(e) => Err(format!("构建更新器失败: {}", e)),
    }
}

/// 开始自动更新过程
#[command]
pub async fn start_auto_update(app: tauri::AppHandle) -> Result<(), String> {
    let handle = app.clone();
    
    tauri::async_runtime::spawn(async move {
        if let Err(e) = perform_update(handle).await {
            eprintln!("自动更新失败: {}", e);
        }
    });
    
    Ok(())
}

/// 执行更新
async fn perform_update(app: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
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
                    update.date.map(|d| d.to_string()).unwrap_or_else(|| "未知".to_string()),
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