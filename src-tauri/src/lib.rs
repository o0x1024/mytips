// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod api;
mod clipboard;
mod db;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod global_shortcut;
mod sync;

#[cfg(desktop)]
use tauri::menu::{Menu, MenuItem};
#[cfg(desktop)]
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};
use tauri::Manager;

// Directly import all public APIs provided by the api module
use api::ai::conversations::{
    add_ai_message, clear_ai_conversation, create_ai_conversation, delete_ai_conversation,
    list_ai_conversations, list_ai_messages, update_ai_conversation_title,
};
use api::ai::roles::{create_ai_role, delete_ai_role, get_ai_role, list_ai_roles, update_ai_role};
use api::ai::service::{
    add_custom_model_config, delete_custom_model_config, get_ai_chat_models, get_ai_config,
    get_ai_embedding_models, get_ai_service_status, get_ai_usage_stats, get_default_ai_model,
    list_custom_model_configs, reload_ai_services, save_ai_config, set_default_ai_model,
    test_ai_connection, update_custom_model_config,
};
use api::audio::{
    analyze_audio_content, batch_analyze_tip_audio, batch_optimize_audio_files,
    batch_transcribe_tip_audio, build_audio_search_index, cleanup_audio_cache, delete_audio_file,
    get_audio_file, get_audio_optimization_stats, get_audio_search_stats,
    get_available_transcription_services, get_supported_transcription_languages,
    get_tip_audio_files, optimize_audio_file, save_audio_file, search_audio_content_by_text,
    transcribe_audio, update_audio_transcription, validate_transcription_service_config,
};
use api::clipboard_api::get_clipboard_ids_for_last_days;
use api::database_manager::{
    get_database_status, get_supported_database_modes, switch_database_mode,
    switch_to_embedded_replica_mode, switch_to_local_mode, switch_to_remote_mode, sync_database,
    test_database_connection,
};
use api::*;

use db::UnifiedDbManager;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
use tauri_plugin_global_shortcut::{GlobalShortcutExt, ShortcutState};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::certificates::*;
use crate::api::database_manager::cleanup_local_database_files;

// 打开开发者工具的命令
#[tauri::command]
fn open_devtools(app: tauri::AppHandle) {
    let window = app.get_webview_window("main").unwrap();
    window.open_devtools();
}

/// 初始化日志系统
fn init_logging() {
    use std::env;
    use tracing_subscriber::{fmt, EnvFilter};

    // 设置默认日志级别为 info，可以通过环境变量 RUST_LOG 覆盖
    let default_level = "info";

    // 创建自定义环境过滤器，过滤掉包含 handshake 的模块
    let mut filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(default_level));

    // 添加特定的过滤规则来屏蔽包含 handshake 的日志
    filter = filter
        .add_directive("libsql::replication::remote_client=off".parse().unwrap())
        .add_directive("libsql=warn".parse().unwrap());

    // 创建格式化层
    let fmt_layer = fmt::layer()
        .with_target(true) // show module path
        .with_level(true) // show log level
        .with_thread_ids(false) // do not show thread id (less noise)
        .with_file(true) // show file name
        .with_line_number(true) // show line number
        .compact(); // use compact format

    // 初始化订阅器
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt_layer)
        .init();

    // 记录初始化成功信息
    tracing::info!("Log system initialized successfully");
    tracing::info!(
        "Current log level: {}",
        env::var("RUST_LOG").unwrap_or_else(|_| default_level.to_string())
    );
    tracing::info!("Filtered out handshake logs from libsql replication");
    tracing::info!(
        "You can adjust log level via RUST_LOG environment variable, e.g.: RUST_LOG=debug"
    );
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
    // 初始化日志系统
    init_logging();

    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(
                #[cfg(not(any(target_os = "android", target_os = "ios")))]
                tauri_plugin_autostart::init(
                    tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                    Some(vec!["--flag1", "--flag2"]),
                ),
            )
            .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
                println!("{}, {args:?}, {cwd}", app.package_info().name);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }))
            .plugin(tauri_plugin_window_state::Builder::new().build());
    }

    builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            // Set up the unified database manager here
            let rt = tokio::runtime::Runtime::new().unwrap();
            let unified_manager = rt.block_on(UnifiedDbManager::new(app_handle.clone()))?;
            app.manage(unified_manager);

            // Setup window close event handler
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent closing, hide window and remove dock/taskbar icon on macOS
                        api.prevent_close();
                        #[cfg(desktop)]
                        {
                            let _ = window_clone.hide();
                        }
                    }
                });
            }

            // Setup global shortcut handler when building the plugin
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            {
                app.handle().plugin(
                    #[cfg(not(any(target_os = "android", target_os = "ios")))]
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            if event.state() == ShortcutState::Pressed {
                                let app_clone = app_handle.clone();
                                let shortcut = shortcut.clone();
                                tauri::async_runtime::spawn(async move {
                                    if let Err(e) =
                                        global_shortcut::handle_shortcut_triggered(&app_clone).await
                                    {
                                        tracing::error!(
                                            "Failed to handle shortcut {:?}: {}",
                                            shortcut,
                                            e
                                        );
                                    }
                                });
                            }
                        })
                        .build(),
                )?;
            }

            // Setup system tray for desktop platforms
            #[cfg(desktop)]
            setup_system_tray(app)?;

            // Start the clipboard listener and initial shortcut setup
            #[cfg(desktop)]
            {
                // Setup initial global shortcuts
                let app_handle_clone = app.handle().clone();
                rt.block_on(async {
                    if let Err(e) = global_shortcut::setup_global_shortcuts(&app_handle_clone).await
                    {
                        tracing::warn!("Failed to setup global shortcuts: {}", e);
                    }
                });

                clipboard::start_clipboard_listener(app.handle().clone());
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Note-related APIs
            get_all_tips,
            get_all_tip_summaries,
            get_tips_paged,
            get_tip_content,
            get_tip,
            save_tip,
            delete_tip,
            search_tips,
            search_tips_summary,
            get_tips_by_category,
            get_tips_by_category_recursive,
            browse_category,
            load_more_tips_by_category,
            get_tips_by_tag,
            // Image-related APIs
            save_tip_image,
            get_tip_images,
            get_tip_images_count,
            delete_tip_image,
            // Category-related APIs
            get_all_categories,
            create_category,
            update_category,
            delete_category,
            // Tag-related APIs
            get_all_tags,
            create_tag,
            delete_tag,
            // Import-related APIs
            import_from_directory,
            import_markdown_file,
            get_import_preview,
            get_clipboard_ids_for_last_days,
            // AI-related APIs
            send_ai_message,
            send_ai_message_stream,
            send_ai_message_with_images,
            send_ai_message_with_images_stream,
            migrate_config_to_database,
            // AI conversation database APIs
            list_ai_conversations,
            list_ai_messages,
            create_ai_conversation,
            delete_ai_conversation,
            clear_ai_conversation,
            update_ai_conversation_title,
            add_ai_message,
            // AI role-related APIs
            list_ai_roles,
            create_ai_role,
            update_ai_role,
            delete_ai_role,
            get_ai_role,
            // New: AI stream control API
            cancel_ai_stream,
            // AI service management APIs
            test_ai_connection,
            get_ai_chat_models,
            get_ai_embedding_models,
            get_default_ai_model,
            set_default_ai_model,
            save_ai_config,
            get_ai_config,
            get_ai_usage_stats,
            reload_ai_services,
            get_ai_service_status,
            // Audio APIs
            save_audio_file,
            get_audio_file,
            get_tip_audio_files,
            delete_audio_file,
            transcribe_audio,
            update_audio_transcription,
            batch_transcribe_tip_audio,
            get_supported_transcription_languages,
            validate_transcription_service_config,
            get_available_transcription_services,
            // Audio AI analysis APIs
            analyze_audio_content,
            batch_analyze_tip_audio,
            // Audio search APIs
            search_audio_content_by_text,
            get_audio_search_stats,
            build_audio_search_index,
            // Audio optimization APIs
            optimize_audio_file,
            batch_optimize_audio_files,
            get_audio_optimization_stats,
            cleanup_audio_cache,
            // Settings-related APIs
            api::settings::save_proxy_settings,
            api::settings::get_proxy_settings,
            api::settings::test_proxy_connection,
            api::settings::debug_proxy_settings,
            // Export and backup APIs
            api::export::backup_database,
            api::export::restore_database,
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            api::export::export_notebook_to_folder,
            api::export::export_notebook_to_pdf,
            api::export::export_notebook_to_word,
            api::import::cancel_import,
            // Clipboard history APIs
            get_clipboard_history,
            delete_clipboard_entries,
            create_note_from_history,
            copy_to_clipboard,
            add_selection_to_clipboard,
            // Tip template APIs
            get_tip_templates,
            save_tip_template,
            delete_tip_template,
            // Clipboard settings APIs
            get_clipboard_settings,
            save_clipboard_settings,
            #[cfg(desktop)]
            clean_expired_clipboard_entries,
            clear_all_clipboard_entries,
            // Clipboard monitoring control
            clipboard::start_clipboard_monitoring,
            clipboard::stop_clipboard_monitoring,
            api::database_manager::optimize_database_wal,
            // Shortcut-related APIs
            get_global_shortcut_config,
            update_global_shortcut,
            // Update-related APIs
            check_for_updates,
            check_for_updates_with_config,
            check_for_updates_no_signature,
            start_auto_update,
            get_current_version,
            set_update_endpoints,
            test_windows_update_with_proxy,
            test_windows_update_no_signature,
            get_platform_info,
            show_confirm_dialog,
            open_url,
            open_devtools,
            api::import::import_from_github,
            // Encryption-related APIs
            api::encryption::get_encryption_statuses,
            api::encryption::encrypt_note,
            api::encryption::decrypt_note,
            api::encryption::unlock_note,
            api::encryption::encrypt_notebook,
            api::encryption::decrypt_notebook,
            api::encryption::unlock_notebook,
            api::encryption::get_unlocked_note_content,
            api::encryption::encrypt_data_cmd,
            api::encryption::clear_session_unlocks,
            // Custom model config APIs
            list_legacy_custom_model_configs,
            add_custom_model_config,
            update_custom_model_config,
            delete_legacy_custom_model_config,
            test_custom_model_connection,
            summarize_clipboard_entries,
            // Database path management APIs
            api::database::get_current_database_path,
            api::database::get_database_info,
            api::database::get_remote_database_info,
            api::database::select_database_file,
            api::database::create_new_database,
            api::database::reset_to_default_database,
            // Remote connection testing
            api::database::test_remote_connection,
            // Simplified sync APIs
            api::database::get_sync_config,
            api::database::save_sync_config,
            api::database::get_sync_status,
            api::database::set_sync_mode,
            api::database::manual_sync,
            api::database::manual_sync_libsql,
            api::database::configure_remote_database,
            api::database::clear_synced_records,
            api::database::create_sync_records_for_existing_data,
            // Database type settings
            api::database::save_database_type,
            api::database::get_database_type,
            // New unified database management APIs
            switch_database_mode,
            get_database_status,
            sync_database,
            test_database_connection,
            switch_to_local_mode,
            switch_to_remote_mode,
            switch_to_embedded_replica_mode,
            get_supported_database_modes,
            cleanup_local_database_files,
            api::ai::service::list_custom_model_configs,
            api::ai::service::add_custom_model_config,
            api::ai::service::update_custom_model_config,
            api::ai::service::delete_custom_model_config,
            // 证书生成 APIs
            generate_dev_certificates,
            get_default_cert_directory,
            // JWT生成 APIs
            generate_jwt_keys,
            get_default_jwt_directory,

            api::tips::list_latest_tip_summaries,
            api::tips::search_tips_for_selector,
            api::tips::get_tip_by_title,
        ])
        .build(tauri::generate_context!())?
        .run(|_app_handle, event| {
            if let tauri::RunEvent::ExitRequested { api, .. } = event {
                // Keep the app running in the background
                #[cfg(desktop)]
                api.prevent_exit();
            }
        });
    Ok(())
}

#[cfg(desktop)]
fn setup_system_tray(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create tray menu items
    let show = MenuItem::with_id(app, "show", "Show Main Window", true, None::<&str>)?;
    let hide = MenuItem::with_id(app, "hide", "Hide Window", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

    // Create the tray menu
    let menu = Menu::with_items(app, &[&show, &hide, &quit])?;

    // Create and setup tray icon
    let _tray = TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            "hide" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.hide();
                }
            }
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    rect: _,
                    ..
                } => {
                    tracing::info!("Tray icon clicked, trying to show window");
                    if let Some(app) = tray.app_handle().get_webview_window("main") {
                        tracing::info!("Found main window, showing it");
                        let _ = app.show();
                        let _ = app.unminimize();
                        let _ = app.set_focus();
                    } else {
                        tracing::error!("Main window not found, cannot show it");
                        // 如果窗口不存在，尝试创建一个新窗口
                        let app_handle = tray.app_handle().clone();
                        let _ = tauri::WebviewWindowBuilder::new(
                            &app_handle,
                            "main",
                            tauri::WebviewUrl::App("index.html".into()),
                        )
                        .title("mytips")
                        .inner_size(1400.0, 800.0)
                        .min_inner_size(1400.0, 800.0)
                        .center()
                        .build();
                    }
                }
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    rect: _,
                    ..
                } => {
                    tracing::info!("Tray icon double-clicked, trying to show window");
                    if let Some(app) = tray.app_handle().get_webview_window("main") {
                        tracing::info!("Found main window, showing it");
                        let _ = app.show();
                        let _ = app.unminimize();
                        let _ = app.set_focus();
                    } else {
                        tracing::error!("Main window not found, cannot show it");
                    }
                }
                _ => {}
            }
        })
        .build(app)?;

    Ok(())
}
