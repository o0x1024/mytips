// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod api;
pub mod clipboard;
pub mod db;
use tauri::menu::Menu;
use tauri::menu::MenuItem;
use tauri::tray::TrayIconBuilder;
use tauri::Manager;
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
            // AI对话数据库相关API
            list_ai_conversations,
            list_ai_messages,
            create_ai_conversation,
            delete_ai_conversation,
            update_ai_conversation_title,
            add_ai_message,
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

                // 根据操作系统设置不同的快捷键
                #[cfg(target_os = "macos")]
                let shortcut_modifiers = Modifiers::META | Modifiers::ALT;
                #[cfg(not(target_os = "macos"))]
                let shortcut_modifiers = Modifiers::CONTROL | Modifiers::ALT;

                let ctrl_alt_c_shortcut: Shortcut =
                    Shortcut::new(Some(shortcut_modifiers), Code::KeyC);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            println!("快捷键事件: {:?}, 状态: {:?}", shortcut, event.state());
                            if shortcut == &ctrl_alt_c_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        #[cfg(target_os = "macos")]
                                        println!("快捷键 Cmd+Alt+C 被按下！");
                                        #[cfg(target_os = "windows")]
                                        println!("快捷键 Ctrl+Alt+C 被按下！");
                                        #[cfg(target_os = "linux")]
                                        println!("快捷键 Ctrl+Alt+C 被按下！");
                                        
                                        // 获取应用句柄，用于调用剪贴板API
                                        let app_handle = app.app_handle().clone();
                                        
                                        // 在单独的线程中处理快捷键操作，避免阻塞UI
                                        std::thread::spawn(move || {
                                            println!("开始处理快捷键操作...");
                                            
                                            // 获取当前选中的文本并添加到临时笔记区
                                            match crate::api::clipboard_api::add_selection_to_clipboard(app_handle.clone()) {
                                                Ok(_) => {
                                                    println!("✅ 已将选中内容添加到临时笔记区");
                                                    
                                                    // 可选：显示系统通知
                                                    #[cfg(target_os = "windows")]
                                                    {
                                                        use std::process::Command;
                                                        let _ = Command::new("powershell")
                                                            .args(&[
                                                                "-Command",
                                                                "Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('已添加到临时笔记区', 'MyTips', 'OK', 'Information')"
                                                            ])
                                                            .output();
                                                    }
                                                },
                                                Err(e) => {
                                                    eprintln!("❌ 添加选中内容到临时笔记区失败: {}", e);
                                                    
                                                    // 显示错误通知
                                                    #[cfg(target_os = "windows")]
                                                    {
                                                        use std::process::Command;
                                                        let _ = Command::new("powershell")
                                                            .args(&[
                                                                "-Command",
                                                                &format!("Add-Type -AssemblyName System.Windows.Forms; [System.Windows.Forms.MessageBox]::Show('操作失败: {}', 'MyTips', 'OK', 'Error')", e)
                                                            ])
                                                            .output();
                                                    }
                                                }
                                            }
                                        });
                                    }
                                    ShortcutState::Released => {
                                        // 释放事件通常不需要处理
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                // 注册快捷键
                match app.global_shortcut().register(ctrl_alt_c_shortcut.clone()) {
                    Ok(_) => {
                        #[cfg(target_os = "macos")]
                        println!("✅ 全局快捷键 Cmd+Alt+C 注册成功");
                        #[cfg(not(target_os = "macos"))]
                        println!("✅ 全局快捷键 Ctrl+Alt+C 注册成功");
                    },
                    Err(e) => {
                        eprintln!("❌ 全局快捷键注册失败: {}", e);
                        #[cfg(target_os = "windows")]
                        eprintln!("提示：在Windows上，请确保以管理员权限运行应用程序");
                    }
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
