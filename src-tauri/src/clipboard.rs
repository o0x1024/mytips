use tauri_plugin_clipboard_manager::ClipboardExt;
use base64::{engine::general_purpose, Engine as _};
use chrono::{TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering}, Mutex,
    },
    thread,
    time::Duration,
};
use tauri::{AppHandle, Emitter, Manager, State};
use tracing::{info, warn, error, debug};

use crate::db::UnifiedDbManager;

// 用于标记模拟复制操作的全局变量
lazy_static::lazy_static! {
    pub static ref SIMULATING_COPY: AtomicBool = AtomicBool::new(false);
    pub static ref MONITORING_ENABLED: AtomicBool = AtomicBool::new(true);
}

// 剪贴板设置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardSettings {
    pub ignore_sensitive_content: bool,
    pub capture_images: bool,
    pub capture_source_info: bool,
    pub retention_days: i32,
    pub encrypt_storage: bool,
    pub enable_monitoring: bool,
    pub enable_app_whitelist: bool,
    pub whitelist_apps: Vec<String>,
}

impl Default for ClipboardSettings {
    fn default() -> Self {
        Self {
            ignore_sensitive_content: true,
            capture_images: false,
            capture_source_info: true,
            retention_days: 7,
            encrypt_storage: false,
            enable_monitoring: true,
            enable_app_whitelist: false,
            whitelist_apps: Vec::new(),
        }
    }
}

// 实现序列化和反序列化方法
impl ClipboardSettings {
    // 从JSON字符串加载设置
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse clipboard settings: {}", e))
    }

    // 将设置转换为JSON字符串
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Failed to serialize clipboard settings: {}", e))
    }
}

/// 获取当前活动窗口标题
#[cfg(target_os = "macos")]
pub fn get_active_window_title() -> Option<String> {
    use std::process::Command;

    // 使用macOS的osascript获取当前活动窗口标题
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first application process whose frontmost is true")
        .output()
        .ok()?;

    if output.status.success() {
        let title = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !title.is_empty() {
            return Some(title);
        }
    }

    None
}

/// 获取当前活动进程名称（用于白名单匹配）
#[cfg(target_os = "macos")]
pub fn get_active_process_name() -> Option<String> {
    use std::process::Command;

    // 获取当前活动应用的进程名称
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first application process whose frontmost is true")
        .output()
        .ok()?;

    if output.status.success() {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }

    None
}

// A helper function to simulate the copy action based on the OS.
fn simulate_copy() {
    #[cfg(target_os = "macos")]
    {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("tell application \"System Events\" to keystroke \"c\" using {command down}")
            .output();
    }
    #[cfg(not(target_os = "macos"))]
    {
        use enigo::{Enigo, Key, Keyboard, Settings};
        if let Ok(mut enigo) = Enigo::new(&Settings::default()) {
            let _ = enigo.key(Key::Control, enigo::Direction::Press);
            let _ = enigo.key(Key::Unicode('c'), enigo::Direction::Click);
            let _ = enigo.key(Key::Control, enigo::Direction::Release);
        } else {
            error!("Failed to create enigo instance for copy simulation");
        }
    }
}


/// Gets the currently selected text by simulating a copy action.
#[cfg(desktop)]
pub fn get_selected_text(app: &AppHandle) -> Option<String> {
    SIMULATING_COPY.store(true, Ordering::SeqCst);

    let clipboard = app.clipboard();
    let original_content_result = clipboard.read_text();

    // 1. Clear the clipboard to reliably detect new content.
    if clipboard.write_text("").is_err() {
        warn!("Failed to clear clipboard, cannot get selected text.");
        SIMULATING_COPY.store(false, Ordering::SeqCst);
        return None;
    }

    // 2. Simulate the copy command.
    simulate_copy();

    // 3. Wait for the clipboard to be updated and read the new content.
    thread::sleep(Duration::from_millis(200));
    let selected_text = clipboard.read_text().unwrap_or_default();

    // 4. Restore the original clipboard content.
    if let Ok(original_content) = original_content_result {
        if !original_content.is_empty() {
            if let Err(e) = clipboard.write_text(original_content) {
                warn!("Failed to restore clipboard content: {}", e);
            }
        }
    }

    // 5. Reset the simulation flag after a delay to avoid race conditions.
    let reset_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;
        SIMULATING_COPY.store(false, Ordering::SeqCst);
    });

    // 6. Return the captured text if it's not empty.
    if !selected_text.is_empty() {
        Some(selected_text)
    } else {
        None
    }
}

/// Fallback for non-desktop platforms.
#[cfg(not(desktop))]
pub fn get_selected_text(_app: &AppHandle) -> Option<String> {
    None
}

#[cfg(target_os = "windows")]
pub fn get_active_window_title() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW};

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }

        let mut buffer: [u16; 512] = [0; 512];
        let len = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

        if len > 0 {
            let title = OsString::from_wide(&buffer[..len as usize])
                .to_string_lossy()
                .to_string();
            if !title.is_empty() {
                return Some(title);
            }
        }
    }

    None
}

/// 获取当前活动进程名称（Windows）
#[cfg(target_os = "windows")]
pub fn get_active_process_name() -> Option<String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use winapi::shared::minwindef::DWORD;
    use winapi::um::processthreadsapi::GetCurrentProcessId;
    use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};
    use winapi::um::psapi::GetModuleBaseNameW;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::winnt::PROCESS_QUERY_INFORMATION;

    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.is_null() {
            return None;
        }

        let mut process_id: DWORD = 0;
        GetWindowThreadProcessId(hwnd, &mut process_id);

        if process_id == 0 {
            return None;
        }

        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id);
        if process_handle.is_null() {
            return None;
        }

        let mut buffer: [u16; 512] = [0; 512];
        let len = GetModuleBaseNameW(process_handle, std::ptr::null_mut(), buffer.as_mut_ptr(), buffer.len() as DWORD);

        if len > 0 {
            let name = OsString::from_wide(&buffer[..len as usize])
                .to_string_lossy()
                .to_string();
            if !name.is_empty() {
                return Some(name);
            }
        }
    }

    None
}

#[cfg(target_os = "linux")]
pub fn get_active_window_title() -> Option<String> {
    // Linux上可能需要使用X11或Wayland相关API
    // 简化实现，返回None
    None
}

/// 获取当前活动进程名称（Linux）
#[cfg(target_os = "linux")]
pub fn get_active_process_name() -> Option<String> {
    // Linux上简化实现，可以使用 xdotool 或其他工具
    use std::process::Command;

    // 尝试使用 xdotool 获取活动窗口的进程信息
    let output = Command::new("sh")
        .arg("-c")
        .arg("xdotool getactivewindow getwindowpid 2>/dev/null | xargs ps -p 2>/dev/null | tail -1 | awk '{print $4}'")
        .output()
        .ok()?;

    if output.status.success() {
        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !name.is_empty() {
            return Some(name);
        }
    }

    None
}

#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn get_active_window_title() -> Option<String> {
    None
}

/// 获取当前活动进程名称（通用）
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn get_active_process_name() -> Option<String> {
    None
}

/// 判断内容是否可能是敏感信息
fn is_sensitive_content(content: &str) -> bool {
    // 检查是否是密码形式（只包含特定字符且长度在一定范围内）
    let is_password_like = content.len() >= 6
        && content.len() <= 64
        && !content.contains(' ')
        && (content.contains(|c: char| c.is_ascii_punctuation())
            || content.contains(|c: char| c.is_ascii_digit())
            || content.contains(|c: char| c.is_uppercase()));

    // 检查是否是特定格式的敏感数据
    let contains_sensitive_pattern = content.contains("password")
        || content.contains("secret")
        || content.contains("token")
        || content.contains("api key")
        || content.contains("private key")
        || content.contains("credit card");

    is_password_like || contains_sensitive_pattern
}

/// 清理过期的剪贴板条目
#[cfg(desktop)]
pub async fn clean_expired_entries(app_handle: &AppHandle) {
    let db_manager = app_handle.state::<UnifiedDbManager>();
    if let Ok(conn) = db_manager.get_conn().await {
        if let Ok(setting) = crate::db::operations::get_setting(&conn, "clipboard_settings").await {
            let settings = setting
                .and_then(|s| ClipboardSettings::from_json(&s).ok())
                .unwrap_or_default();

            // 如果设置了保留天数（不为0），则执行清理
            if settings.retention_days > 0 {
                let retention_days = settings.retention_days;

                // 计算过期时间点
                let now = Utc::now();
                let expire_date = now - chrono::Duration::days(retention_days as i64);
                let expire_timestamp = expire_date.timestamp();

                // 删除过期条目
                if let Err(e) = crate::db::operations::delete_expired_clipboard_entries(&conn, expire_timestamp).await {
                    eprintln!("Failed to clean expired clipboard entries: {}", e);
                }
            }
        }
    }
}

#[cfg(desktop)]
pub fn start_clipboard_listener(app_handle: AppHandle) {
    thread::spawn(move || {
        // 创建一个新的异步运行时
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        rt.block_on(async move {
            // 检查数据库管理器是否可用
            let db_manager = match app_handle.try_state::<UnifiedDbManager>() {
                Some(manager) => manager,
                None => {
                    eprintln!("Database manager not initialized, clipboard listener failed to start");
                    return;
                }
            };

            // 验证数据库连接
            if let Err(e) = db_manager.get_conn().await {
                error!("Database connection failed, clipboard listener failed to start: {}", e);
                return;
            }

            debug!("Clipboard listener started successfully");

            // 初始化剪贴板
            let clipboard = app_handle.clipboard();

            // 获取初始剪贴板内容
            let mut last_text = clipboard.read_text().unwrap_or_default();
            debug!("Clipboard listener started, initial content length: {} characters", last_text.len());

            // 用于比较图片内容的哈希值
            let mut last_image_hash = String::new();

            // 定义清理计时器
            let mut last_cleanup_time = Utc::now();

            // 记录最近处理过的内容和时间戳，用于防止短时间内重复添加
            let mut recent_contents: Vec<(String, i64)> = Vec::new();

            // 连续失败计数器
            let mut consecutive_failures = 0;
            const MAX_CONSECUTIVE_FAILURES: u32 = 5;

            // 辅助函数：检查内容是否在recent_contents中
            fn has_recent_content(content: &str, contents: &[(String, i64)]) -> bool {
                contents.iter().any(|(c, _)| c == content)
            }

            loop {
                // 每隔一秒检查一次
                thread::sleep(Duration::from_millis(1000));

                // 检查是否启用了监听
                if !MONITORING_ENABLED.load(Ordering::SeqCst) {
                    // 未启用监听，跳过此次循环
                    continue;
                }

                // 检查连续失败次数
                if consecutive_failures >= MAX_CONSECUTIVE_FAILURES {
                    error!("Clipboard listener failed {} consecutive times, pausing for 30 seconds", consecutive_failures);
                    thread::sleep(Duration::from_secs(30));
                    consecutive_failures = 0;
                    continue;
                }

                // 清理超过10秒的记录
                let now = Utc::now().timestamp();
                recent_contents.retain(|(_, timestamp)| now - timestamp < 10);

                // 检查是否需要清理过期条目（每天检查一次）
                let current_time = Utc::now();
                if (current_time - last_cleanup_time).num_hours() >= 24 {
                    clean_expired_entries(&app_handle).await;
                    last_cleanup_time = current_time;
                }

                // 如果当前正在模拟复制操作，跳过此次检查
                if SIMULATING_COPY.load(Ordering::SeqCst) {
                    continue;
                }

                // 获取剪贴板设置
                let settings = {
                    match db_manager.get_conn().await {
                        Ok(conn) => {
                            match crate::db::operations::get_setting(&conn, "clipboard_settings").await {
                                Ok(Some(settings_str)) => {
                                    ClipboardSettings::from_json(&settings_str).unwrap_or_default()
                                }
                                Ok(None) => ClipboardSettings::default(),
                                Err(e) => {
                                    warn!("Failed to get clipboard settings: {}", e);
                                    ClipboardSettings::default()
                                }
                            }
                        }
                        Err(e) => {
                            error!("Failed to get database connection: {}", e);
                            consecutive_failures += 1;
                            continue;
                        }
                    }
                };

                // 检查文本内容
                let mut has_new_content = false;
                match clipboard.read_text() {
                    Ok(current_text) => {
                        // 检查内容是否为空或与上次相同
                        if !current_text.is_empty() && current_text != last_text {
                            last_text = current_text.clone();

                            // 检查此内容是否最近被处理过，防止重复添加
                            if has_recent_content(&current_text, &recent_contents) {
                                info!("Content has been processed recently, skipping");
                                continue;
                            }

                            // 记录此内容已被处理
                            recent_contents.push((current_text.clone(), Utc::now().timestamp()));

                            // 检查是否为敏感内容
                            // if settings.ignore_sensitive_content && is_sensitive_content(&current_text) {
                            //     info!("检测到敏感内容，跳过添加");
                            //     continue;
                            // }

                            // 获取当前活动窗口标题
                            let source = if settings.capture_source_info {
                                get_active_window_title()
                            } else {
                                None
                            };

                            if let Some(ref source_name) = source {
                                info!("Content source: {}", source_name);
                                
                                // 检查白名单应用
                                if settings.enable_app_whitelist && !settings.whitelist_apps.is_empty() {
                                    // 获取进程名称以提高匹配准确性
                                    let process_name = get_active_process_name();
                                    
                                    let should_ignore = settings.whitelist_apps.iter().any(|app| {
                                        let app_lower = app.to_lowercase();
                                        let source_lower = source_name.to_lowercase();
                                        
                                        // 首先检查窗口标题/应用名称匹配
                                        let title_match = source_lower.contains(&app_lower) || 
                                                         app_lower.contains(&source_lower) ||
                                                         source_lower == app_lower;
                                        
                                        // 如果有进程名称，也检查进程名称匹配
                                        let process_match = if let Some(ref proc_name) = process_name {
                                            let proc_lower = proc_name.to_lowercase();
                                            proc_lower.contains(&app_lower) || 
                                            app_lower.contains(&proc_lower) ||
                                            proc_lower == app_lower
                                        } else {
                                            false
                                        };
                                        
                                        title_match || process_match
                                    });
                                    
                                    if should_ignore {
                                        if let Some(ref proc_name) = process_name {
                                            debug!("Process name: {}, in whitelist, skipping", proc_name);
                                        }
                                        continue;
                                    }
                                }
                            }

                            // 获取数据库连接并保存
                            match db_manager.get_conn().await {
                                Ok(conn) => {
                                    // 检查是否已经存在相同内容
                                    let content_exists = match crate::db::operations::check_clipboard_entry_exists(&conn, &current_text).await {
                                        Ok(exists) => exists,
                                        Err(e) => {
                                            warn!("Failed to check if clipboard content exists: {}", e);
                                            false // 如果检查失败，继续尝试添加
                                        }
                                    };

                                    if content_exists {
                                        debug!("Same content already exists in the database, skipping");
                                        continue;
                                    }

                                    // 添加到数据库
                                    match crate::db::operations::add_clipboard_entry(&conn, &current_text, source.as_deref()).await {
                                        Ok(_) => {
                                            debug!("Clipboard text content has been added to the temporary notes area");
                                            has_new_content = true;
                                            consecutive_failures = 0; // 重置失败计数
                                        }
                                        Err(e) => {
                                            error!("Failed to add clipboard content to the database: {}", e);
                                            consecutive_failures += 1;
                                        }
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to get database connection: {}", e);
                                    consecutive_failures += 1;
                                    continue;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let error_message = e.to_string();
                        // 忽略由于剪贴板为空或内容为图片等非文本格式而导致的“错误”
                        // "format" 对应 "not available in the requested format"
                        // "empty" 对应 "clipboard is empty"
                        if !error_message.contains("format") && !error_message.contains("empty") {
                            warn!("Failed to read clipboard text: {}", error_message);
                            consecutive_failures += 1;
                        }
                    }
                }

                // 检查图片内容（如果启用了图片捕获）
                if settings.capture_images {
                    match clipboard.read_image() {
                        Ok(img) => {
                            // 计算图片内容的哈希，用于比较是否发生变化
                            let image_data = img.rgba();
                            let mut hasher = blake3::Hasher::new();
                            hasher.update(&image_data);
                            let hash = hasher.finalize().to_hex().to_string();

                            // 检查图片是否与上次不同
                            if hash != last_image_hash {
                                // debug!(
                                //     "检测到新的剪贴板图片内容，尺寸: {}x{}",
                                //     img.size().width, img.size().height
                                // );
                                last_image_hash = hash;

                                // 将图片数据转换为Base64编码的字符串
                                let base64_image = general_purpose::STANDARD.encode(&image_data);
                                let img_text = format!("data:image/png;base64,{}", base64_image);

                                // 检查此图片内容是否最近被处理过，防止重复添加
                                if has_recent_content(&img_text, &recent_contents) {
                                    debug!("Image content has been processed recently, skipping");
                                    continue;
                                }

                                // 记录此图片内容已被处理
                                recent_contents.push((img_text.clone(), Utc::now().timestamp()));

                                // 获取当前活动窗口标题
                                let source = if settings.capture_source_info {
                                    get_active_window_title()
                                } else {
                                    None
                                };

                                if let Some(ref source_name) = source {
                                    debug!("Content source: {}", source_name);
                                    
                                    // 检查白名单应用
                                    if settings.enable_app_whitelist && !settings.whitelist_apps.is_empty() {
                                        // 获取进程名称以提高匹配准确性
                                        let process_name = get_active_process_name();
                                        
                                        let should_ignore = settings.whitelist_apps.iter().any(|app| {
                                            let app_lower = app.to_lowercase();
                                            let source_lower = source_name.to_lowercase();
                                            
                                            // 首先检查窗口标题/应用名称匹配
                                            let title_match = source_lower.contains(&app_lower) || 
                                                             app_lower.contains(&source_lower) ||
                                                             source_lower == app_lower;
                                            
                                            // 如果有进程名称，也检查进程名称匹配
                                            let process_match = if let Some(ref proc_name) = process_name {
                                                let proc_lower = proc_name.to_lowercase();
                                                proc_lower.contains(&app_lower) || 
                                                app_lower.contains(&proc_lower) ||
                                                proc_lower == app_lower
                                            } else {
                                                false
                                            };
                                            
                                            title_match || process_match
                                        });
                                        
                                        if should_ignore {
                                            if let Some(ref proc_name) = process_name {
                                                debug!("Process name: {}, in whitelist, skipping", proc_name);
                                            }
                                            continue;
                                        }
                                    }
                                }

                                // 获取数据库连接并保存图片内容
                                match db_manager.get_conn().await {
                                    Ok(conn) => {
                                        // 检查是否已经存在相同内容
                                        let content_exists = match crate::db::operations::check_clipboard_entry_exists(&conn, &img_text).await {
                                            Ok(exists) => exists,
                                            Err(e) => {
                                                warn!("Failed to check if clipboard image content exists: {}", e);
                                                false // 如果检查失败，继续尝试添加
                                            }
                                        };

                                        if content_exists {
                                            debug!("Same image content already exists in the database, skipping");
                                            continue;
                                        }

                                        match crate::db::operations::add_clipboard_entry(&conn, &img_text, source.as_deref()).await {
                                            Ok(_) => {
                                                debug!("Clipboard image content has been added to the temporary notes area");
                                                has_new_content = true;
                                                consecutive_failures = 0; // 重置失败计数
                                            }
                                            Err(e) => {
                                                error!("Failed to add clipboard image content to the database: {}", e);
                                                consecutive_failures += 1;
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        error!("Failed to get database connection: {}", e);
                                        consecutive_failures += 1;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            // 通常是剪贴板没有图片，这是正常情况。
                            // 记录到debug日志以备调试。
                            debug!("Could not read clipboard image (this is usually normal as clipboard content may not be an image): {}", e);
                        }
                    }
                }

                // 如果有新内容（文本或图片），通知前端更新
                if has_new_content {
                    // 通知前端更新
                    if let Err(e) = app_handle.emit("new-clipboard-entry", ()) {
                        error!("Failed to send new-clipboard-entry event: {}", e);
                    }
                }
            }
        });
    });
}

/// 启动剪贴板监听
#[tauri::command]
pub fn start_clipboard_monitoring() -> Result<String, String> {
    MONITORING_ENABLED.store(true, Ordering::SeqCst);
    Ok("Clipboard monitoring started".to_string())
}

/// 停止剪贴板监听
#[tauri::command]
pub fn stop_clipboard_monitoring() -> Result<String, String> {
    MONITORING_ENABLED.store(false, Ordering::SeqCst);
    Ok("Clipboard monitoring stopped".to_string())
}
