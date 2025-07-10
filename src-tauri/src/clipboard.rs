use arboard::Clipboard;
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

use crate::db::DbManager;

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
        serde_json::from_str(json_str).map_err(|e| format!("解析剪贴板设置失败: {}", e))
    }

    // 将设置转换为JSON字符串
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("序列化剪贴板设置失败: {}", e))
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

/// 获取当前选中的文本（macOS）
#[cfg(target_os = "macos")]
pub fn get_selected_text() -> Option<String> {
    // 标记我们正在模拟复制，防止剪贴板监听器触发
    SIMULATING_COPY.store(true, Ordering::SeqCst);

    // 使用AppleScript获取当前选中文本
    let output = Command::new("osascript")
        .arg("-e")
        .arg(
            r#"
        tell application "System Events"
            keystroke "c" using {command down}
            delay 0.1
        end tell
        delay 0.1
        set the clipboard_content to the clipboard
        return the clipboard_content
        "#,
        )
        .output()
        .ok()?;

    // 处理结果
    let result = if output.status.success() {
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !text.is_empty() {
            Some(text)
        } else {
            None
        }
    } else {
        None
    };

    // 延迟一小段时间后重置标记
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(500));
        SIMULATING_COPY.store(false, Ordering::SeqCst);
    });

    result
}

/// 获取当前选中的文本（Windows）
#[cfg(target_os = "windows")]
pub fn get_selected_text() -> Option<String> {
    // 标记我们正在模拟复制，防止剪贴板监听器触发
    SIMULATING_COPY.store(true, Ordering::SeqCst);

    // 在Windows上模拟Ctrl+C获取选中文本
    // 使用临时剪贴板存储原始内容
    let mut clipboard = Clipboard::new().ok()?;
    let original_content = clipboard.get_text().unwrap_or_default();

    // 模拟Ctrl+C按键
    use enigo::{Enigo, Key, Keyboard, Settings};

    // 创建enigo实例
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("无法创建Enigo实例: {}", e);
            SIMULATING_COPY.store(false, Ordering::SeqCst);
            return None;
        }
    };

    // 模拟Ctrl+C按键组合
    if let Err(e) = enigo.key(Key::Control, enigo::Direction::Press) {
        eprintln!("按下Ctrl键失败: {}", e);
        SIMULATING_COPY.store(false, Ordering::SeqCst);
        return None;
    }

    if let Err(e) = enigo.key(Key::Unicode('c'), enigo::Direction::Click) {
        eprintln!("按下C键失败: {}", e);
        let _ = enigo.key(Key::Control, enigo::Direction::Release);
        SIMULATING_COPY.store(false, Ordering::SeqCst);
        return None;
    }

    if let Err(e) = enigo.key(Key::Control, enigo::Direction::Release) {
        eprintln!("释放Ctrl键失败: {}", e);
    }

    // 等待一小段时间确保剪贴板已更新
    thread::sleep(Duration::from_millis(200));

    // 获取剪贴板内容（即选中文本）
    let selected_text = clipboard.get_text().unwrap_or_default();

    // 处理结果
    let result = if !selected_text.is_empty() && selected_text != original_content {
        Some(selected_text)
    } else {
        None
    };

    // 恢复原始剪贴板内容
    if !original_content.is_empty() {
        let _ = clipboard.set_text(original_content);
    }

    // 延迟一小段时间后重置标记
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        SIMULATING_COPY.store(false, Ordering::SeqCst);
    });

    result
}

/// 获取当前选中的文本（Linux）
#[cfg(target_os = "linux")]
pub fn get_selected_text() -> Option<String> {
    // 标记我们正在模拟复制，防止剪贴板监听器触发
    SIMULATING_COPY.store(true, Ordering::SeqCst);

    // 在Linux上模拟Ctrl+C获取选中文本
    // 使用临时剪贴板存储原始内容
    let mut clipboard = Clipboard::new().ok()?;
    let original_content = clipboard.get_text().unwrap_or_default();

    // 模拟Ctrl+C按键
    use enigo::{Enigo, Key, Keyboard, Settings};

    // 创建enigo实例
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(enigo) => enigo,
        Err(e) => {
            eprintln!("无法创建Enigo实例: {}", e);
            SIMULATING_COPY.store(false, Ordering::SeqCst);
            return None;
        }
    };

    // 模拟Ctrl+C按键组合
    if let Err(e) = enigo.key(Key::Control, enigo::Direction::Press) {
        eprintln!("按下Ctrl键失败: {}", e);
        SIMULATING_COPY.store(false, Ordering::SeqCst);
        return None;
    }

    if let Err(e) = enigo.key(Key::Unicode('c'), enigo::Direction::Click) {
        eprintln!("按下C键失败: {}", e);
        let _ = enigo.key(Key::Control, enigo::Direction::Release);
        SIMULATING_COPY.store(false, Ordering::SeqCst);
        return None;
    }

    if let Err(e) = enigo.key(Key::Control, enigo::Direction::Release) {
        eprintln!("释放Ctrl键失败: {}", e);
    }

    // 等待一小段时间确保剪贴板已更新
    thread::sleep(Duration::from_millis(200));

    // 获取剪贴板内容（即选中文本）
    let selected_text = clipboard.get_text().unwrap_or_default();

    // 处理结果
    let result = if !selected_text.is_empty() && selected_text != original_content {
        Some(selected_text)
    } else {
        None
    };

    // 恢复原始剪贴板内容
    if !original_content.is_empty() {
        let _ = clipboard.set_text(original_content);
    }

    // 延迟一小段时间后重置标记
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        SIMULATING_COPY.store(false, Ordering::SeqCst);
    });

    result
}

/// 通用实现，用于不支持的平台
#[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
pub fn get_selected_text() -> Option<String> {
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
        || content.contains("密码")
        || content.contains("token")
        || content.contains("api key")
        || content.contains("secret")
        || content.contains("private key")
        || content.contains("credit card")
        || content.contains("信用卡");

    is_password_like || contains_sensitive_pattern
}

/// 清理过期的剪贴板条目
pub fn clean_expired_entries(app_handle: &AppHandle) {
    let db_manager = app_handle.state::<DbManager>();
    if let Ok(conn) = db_manager.get_conn() {
        if let Ok(setting) = crate::db::get_setting(&conn, "clipboard_settings") {
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
                if let Err(e) = crate::db::delete_expired_clipboard_entries(&conn, expire_timestamp) {
                    eprintln!("清理过期剪贴板条目失败: {}", e);
                }
            }
        }
    }
}

pub fn start_clipboard_listener(app_handle: AppHandle) {
    thread::spawn(move || {
        // 初始化剪贴板
        let mut clipboard = match Clipboard::new() {
            Ok(clipboard) => clipboard,
            Err(e) => {
                eprintln!("无法初始化剪贴板监听: {}", e);
                return;
            }
        };

        // 获取初始剪贴板内容
        let mut last_text = clipboard.get_text().unwrap_or_default();
        println!("剪贴板监听已启动，初始内容: {}", last_text);

        // 用于比较图片内容的哈希值
        let mut last_image_hash = String::new();

        // 定义清理计时器
        let mut last_cleanup_time = Utc::now();

        // 记录最近处理过的内容和时间戳，用于防止短时间内重复添加
        let mut recent_contents: Vec<(String, i64)> = Vec::new();

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

            // 清理超过10秒的记录
            let now = Utc::now().timestamp();
            recent_contents.retain(|(_, timestamp)| now - timestamp < 10);

            // 检查是否需要清理过期条目（每天检查一次）
            let current_time = Utc::now();
            if (current_time - last_cleanup_time).num_hours() >= 24 {
                clean_expired_entries(&app_handle);
                last_cleanup_time = current_time;
            }

            // 如果当前正在模拟复制操作，跳过此次检查
            if SIMULATING_COPY.load(Ordering::SeqCst) {
                continue;
            }

            // 获取剪贴板设置
            let settings = {
                let db_manager = app_handle.state::<DbManager>();
                if let Ok(conn) = db_manager.get_conn() {
                    match crate::db::get_setting(&conn, "clipboard_settings") {
                        Ok(Some(settings_str)) => {
                            ClipboardSettings::from_json(&settings_str).unwrap_or_default()
                        }
                        _ => ClipboardSettings::default(),
                    }
                } else {
                    ClipboardSettings::default()
                }
            };

            // 检查文本内容
            let mut has_new_content = false;
            if let Ok(current_text) = clipboard.get_text() {
                // 检查内容是否为空或与上次相同
                if !current_text.is_empty() && current_text != last_text {
                    last_text = current_text.clone();

                    // 检查此内容是否最近被处理过，防止重复添加
                    if has_recent_content(&current_text, &recent_contents) {
                        println!("内容最近已处理过，跳过");
                        continue;
                    }

                    // 记录此内容已被处理
                    recent_contents.push((current_text.clone(), Utc::now().timestamp()));

                    // 检查是否为敏感内容
                    // if settings.ignore_sensitive_content && is_sensitive_content(&current_text) {
                    //     println!("检测到敏感内容，跳过添加");
                    //     continue;
                    // }

                    // 获取当前活动窗口标题
                    let source = if settings.capture_source_info {
                        get_active_window_title()
                    } else {
                        None
                    };

                    if let Some(ref source_name) = source {
                        println!("内容来源: {}", source_name);
                        
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
                                    println!("进程名称: {}", proc_name);
                                }
                                continue;
                            }
                        }
                    }

                    // 获取数据库连接
                    let db_manager = app_handle.state::<DbManager>();
                    if let Ok(conn) = db_manager.get_conn() {
                        // 检查是否已经存在相同内容
                        let content_exists =
                            match crate::db::check_clipboard_entry_exists(&conn, &current_text) {
                                Ok(exists) => exists,
                                Err(e) => {
                                    eprintln!("检查剪贴板内容是否存在失败: {}", e);
                                    false // 如果检查失败，继续尝试添加
                                }
                            };

                        if content_exists {
                            println!("相同内容已存在于数据库，跳过添加");
                            continue;
                        }

                        // 添加到数据库
                        if let Err(e) =
                            crate::db::add_clipboard_entry(&conn, &current_text, source.as_deref())
                        {
                            eprintln!("添加剪贴板内容到数据库失败: {}", e);
                        } else {
                            println!("剪贴板文本内容已添加到临时笔记区");
                            has_new_content = true;
                        }
                    } else {
                        eprintln!("获取数据库连接失败");
                        continue;
                    }
                }
            }

            // 检查图片内容（如果启用了图片捕获）
            if settings.capture_images {
                match clipboard.get_image() {
                    Ok(img) => {
                        // 计算图片内容的哈希，用于比较是否发生变化
                        let image_data = img.bytes.clone();
                        let mut hasher = blake3::Hasher::new();
                        hasher.update(&image_data);
                        let hash = hasher.finalize().to_hex().to_string();

                        // 检查图片是否与上次不同
                        if hash != last_image_hash {
                            println!(
                                "检测到新的剪贴板图片内容，尺寸: {}x{}",
                                img.width, img.height
                            );
                            last_image_hash = hash;

                            // 将图片数据转换为Base64编码的字符串
                            let base64_image = general_purpose::STANDARD.encode(&image_data);
                            let img_text = format!("data:image/png;base64,{}", base64_image);

                            // 检查此图片内容是否最近被处理过，防止重复添加
                            if has_recent_content(&img_text, &recent_contents) {
                                println!("图片内容最近已处理过，跳过");
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
                                println!("内容来源: {}", source_name);
                                
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
                                            println!("进程名称: {}", proc_name);
                                        }
                                        continue;
                                    }
                                }
                            }

                            // 获取数据库连接并保存图片内容
                            let db_manager = app_handle.state::<DbManager>();
                             if let Ok(conn) = db_manager.get_conn() {
                                // 检查是否已经存在相同内容
                                let content_exists =
                                    match crate::db::check_clipboard_entry_exists(&conn, &img_text) {
                                        Ok(exists) => exists,
                                        Err(e) => {
                                            eprintln!("检查剪贴板图片内容是否存在失败: {}", e);
                                            false // 如果检查失败，继续尝试添加
                                        }
                                    };

                                if content_exists {
                                    println!("相同图片内容已存在于数据库，跳过添加");
                                    continue;
                                }

                                if let Err(e) = crate::db::add_clipboard_entry(&conn, &img_text, source.as_deref())
                                {
                                    eprintln!("添加剪贴板图片内容到数据库失败: {}", e);
                                } else {
                                    println!("剪贴板图片内容已添加到临时笔记区");
                                    has_new_content = true;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // 忽略无图片内容的错误
                        // if !e.to_string().contains("no image content available") {
                        //     eprintln!("获取剪贴板图片失败: {}", e);
                        // }
                    }
                }
            }

            // 如果有新内容（文本或图片），通知前端更新
            if has_new_content {
                // 通知前端更新
                if let Err(e) = app_handle.emit("new-clipboard-entry", ()) {
                    eprintln!("发送new-clipboard-entry事件失败: {}", e);
                }
            }
        }
    });
}

/// 启动剪贴板监听
#[tauri::command]
pub fn start_clipboard_monitoring() -> Result<String, String> {
    MONITORING_ENABLED.store(true, Ordering::SeqCst);
    Ok("剪贴板监听已启动".to_string())
}

/// 停止剪贴板监听
#[tauri::command]
pub fn stop_clipboard_monitoring() -> Result<String, String> {
    MONITORING_ENABLED.store(false, Ordering::SeqCst);
    Ok("剪贴板监听已停止".to_string())
}
