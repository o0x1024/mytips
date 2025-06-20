// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod api;
pub mod clipboard;
pub mod db;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::tray::TrayIconBuilder;
use tauri::{Manager, Emitter};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_autostart::ManagerExt;
// 直接导入api模块提供的所有公共API
use api::*;

use db::DbManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let db_manager = DbManager::init().expect("Failed to initialize database");

    tauri::Builder::default()
        .manage(std::sync::Mutex::new(db_manager))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--flag1", "--flag2"]),
        ))
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }))
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            // 笔记相关API
            get_all_tips,
            get_tip,
            save_tip,
            delete_tip,
            search_tips,
            get_tips_by_category,
            get_tips_by_tag,
            // 图片相关API
            save_tip_image,
            get_tip_images,
            get_tip_images_count,
            delete_tip_image,
            // 分类相关API
            get_all_categories,
            create_category,
            update_category,
            delete_category,
            // 标签相关API
            get_all_tags,
            create_tag,
            delete_tag,
            // 导入相关API
            import_from_directory,
            import_markdown_file,
            get_import_preview,
            // AI相关API
            send_ai_message,
            send_ai_message_stream,
            send_ai_message_with_images,
            send_ai_message_with_images_stream,
            cancel_ai_generation,
            get_ai_api_keys,
            save_api_key,
            get_api_key,
            has_api_key,
            save_api_endpoint,
            get_api_endpoint,
            save_model_name,
            get_model_name_config,
            save_max_tokens_config,
            get_max_tokens_config,
            migrate_config_to_database,
            // AI对话数据库相关API
            list_ai_conversations,
            list_ai_messages,
            create_ai_conversation,
            delete_ai_conversation,
            update_ai_conversation_title,
            add_ai_message,
            // AI角色相关API
            list_ai_roles,
            create_ai_role,
            update_ai_role,
            delete_ai_role,
            get_ai_role,
            // 设置相关API
            save_proxy_settings,
            get_proxy_settings,
            test_proxy_connection,
            // 导出与备份相关API
            backup_database,
            export_as_markdown,
            export_as_html,
            export_as_pdf,
            restore_database,
            // 剪贴板历史相关API
            get_clipboard_history,
            delete_clipboard_entries,
            create_note_from_history,
            copy_to_clipboard,
            add_clipboard_entry,
            add_selection_to_clipboard,
            // 剪贴板设置相关API
            get_clipboard_settings,
            save_clipboard_settings,
            clean_expired_clipboard_entries,
            clear_all_clipboard_entries,
            // 剪贴板监听控制
            clipboard::start_clipboard_monitoring,
            clipboard::stop_clipboard_monitoring,
            // 快捷键相关API
            get_global_shortcut_config,
            update_global_shortcut,
        ])
        .setup(|app| {
            // 初始化数据库
            // let db_manager = DbManager::init().expect("Database initialization failed");
            // app.manage(db_manager);

            // 启动剪贴板监听
            clipboard::start_clipboard_listener(app.handle().clone());

            // 获取自动启动管理器
            let autostart_manager = app.autolaunch();
            // 启用 autostart
            let _ = autostart_manager.enable();
            // 检查 enable 状态
            println!(
                "registered for autostart? {}",
                autostart_manager.is_enabled().unwrap()
            );
            // 禁用 autostart
            let _ = autostart_manager.disable();

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let show_i = MenuItem::with_id(app, "show", "Show windows", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

            TrayIconBuilder::new()
                .menu(&menu)
                // .menu_on_left_click(true)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    #[cfg(target_os = "macos")]
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                        app.show().unwrap();
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .build(app)?;

            // 设置全局快捷键
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                // 从数据库获取快捷键配置
                let shortcut_config = {
                    let db_state = app.state::<std::sync::Mutex<DbManager>>();
                    let db = db_state.lock().unwrap();
                    match db.get_setting("global_shortcut") {
                        Ok(Some(config_str)) => {
                            match serde_json::from_str::<crate::api::shortcuts::ShortcutConfig>(&config_str) {
                                Ok(config) => config,
                                Err(_) => crate::api::shortcuts::ShortcutConfig::default(),
                            }
                        },
                        _ => crate::api::shortcuts::ShortcutConfig::default(),
                    }
                };

                // 构建修饰键
                let mut modifiers = Modifiers::empty();
                for modifier in &shortcut_config.modifiers {
                    match modifier.as_str() {
                        "meta" => {
                            // 在Windows下，meta键对应Control键
                            #[cfg(target_os = "windows")]
                            { modifiers |= Modifiers::CONTROL; }
                            // 在macOS下，meta键对应Command键
                            #[cfg(target_os = "macos")]
                            { modifiers |= Modifiers::META; }
                            // 在Linux下，meta键对应Super键，但通常使用Control
                            #[cfg(target_os = "linux")]
                            { modifiers |= Modifiers::CONTROL; }
                        },
                        "shift" => modifiers |= Modifiers::SHIFT,
                        "alt" => modifiers |= Modifiers::ALT,
                        "control" => modifiers |= Modifiers::CONTROL,
                        _ => {}
                    }
                }

                // 构建按键码
                let key_code = match shortcut_config.key.to_lowercase().as_str() {
                    "a" => Code::KeyA, "b" => Code::KeyB, "c" => Code::KeyC, "d" => Code::KeyD,
                    "e" => Code::KeyE, "f" => Code::KeyF, "g" => Code::KeyG, "h" => Code::KeyH,
                    "i" => Code::KeyI, "j" => Code::KeyJ, "k" => Code::KeyK, "l" => Code::KeyL,
                    "m" => Code::KeyM, "n" => Code::KeyN, "o" => Code::KeyO, "p" => Code::KeyP,
                    "q" => Code::KeyQ, "r" => Code::KeyR, "s" => Code::KeyS, "t" => Code::KeyT,
                    "u" => Code::KeyU, "v" => Code::KeyV, "w" => Code::KeyW, "x" => Code::KeyX,
                    "y" => Code::KeyY, "z" => Code::KeyZ,
                    "1" => Code::Digit1, "2" => Code::Digit2, "3" => Code::Digit3, "4" => Code::Digit4,
                    "5" => Code::Digit5, "6" => Code::Digit6, "7" => Code::Digit7, "8" => Code::Digit8,
                    "9" => Code::Digit9, "0" => Code::Digit0,
                    _ => Code::KeyC, // 默认使用C键
                };

                let global_shortcut = Shortcut::new(Some(modifiers), key_code);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            println!("快捷键触发: {:?}", shortcut);
                            if shortcut == &global_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        #[cfg(target_os = "windows")]
                                        println!("快捷键 Ctrl+Shift+C 被按下！");
                                        #[cfg(target_os = "macos")]
                                        println!("快捷键 Cmd+Shift+C 被按下！");
                                        #[cfg(target_os = "linux")]
                                        println!("快捷键 Ctrl+Shift+C 被按下！");
                                        
                                        // 首先标记我们正在模拟复制，防止剪贴板监听器触发
                                        crate::clipboard::SIMULATING_COPY.store(true, std::sync::atomic::Ordering::SeqCst);
                                        
                                        // 给标记有时间生效
                                        std::thread::sleep(std::time::Duration::from_millis(100));
                                        
                                        // 获取应用句柄，用于调用剪贴板API
                                        let app_handle = app.app_handle().clone();
                                        
                                        // 在单独线程中处理，避免阻塞快捷键处理
                                        std::thread::spawn(move || {
                                            // 获取当前选中的文本并添加到临时笔记区
                                            match crate::api::clipboard_api::add_selection_to_clipboard(app_handle.clone()) {
                                                Ok(_) => {
                                                    println!("已将选中内容添加到临时笔记区");
                                                    // 通知前端更新
                                                    if let Err(e) = app_handle.emit("new-clipboard-entry", ()) {
                                                        eprintln!("发送new-clipboard-entry事件失败: {}", e);
                                                    }
                                                },
                                                Err(e) => eprintln!("添加选中内容到临时笔记区失败: {}", e)
                                            }
                                            
                                            // 延迟后重置标记
                                            std::thread::sleep(std::time::Duration::from_millis(2000));
                                            crate::clipboard::SIMULATING_COPY.store(false, std::sync::atomic::Ordering::SeqCst);
                                            println!("SIMULATING_COPY标记已重置");
                                        });
                                    }
                                    ShortcutState::Released => {
                                        // 快捷键释放时不需要特殊处理
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                // 注册快捷键
                if let Err(e) = app.global_shortcut().register(global_shortcut) {
                    eprintln!("注册全局快捷键失败: {}", e);
                } else {
                    println!("全局快捷键注册成功: {:?}", global_shortcut);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
