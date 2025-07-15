// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod api;
pub mod clipboard;
pub mod db;

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
    get_ai_chat_models, get_ai_config, get_ai_embedding_models, get_ai_service_status,
    get_ai_usage_stats, get_default_ai_model, reload_ai_services, save_ai_config,
    set_default_ai_model, test_ai_connection,
};
use api::clipboard_api::get_clipboard_ids_for_last_days;
use api::*;

use db::DbManager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() -> anyhow::Result<()> {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder
            .plugin(tauri_plugin_autostart::init(
                tauri_plugin_autostart::MacosLauncher::LaunchAgent,
                Some(vec!["--flag1", "--flag2"]),
            ))
            .plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
                println!("{}, {args:?}, {cwd}", app.package_info().name);
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }))
            .plugin(tauri_plugin_window_state::Builder::new().build())
            .on_window_event(|window, event| match event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    window.hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            });
    }

    builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // Set up the database here
            let db_manager = DbManager::init(app.handle().clone())?;
            app.manage(db_manager);

            // Start the clipboard listener
            #[cfg(desktop)]
            clipboard::start_clipboard_listener(app.handle().clone());

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
            // Settings-related APIs
            api::settings::save_proxy_settings,
            api::settings::get_proxy_settings,
            api::settings::test_proxy_connection,
            // Export and backup APIs
            api::export::backup_database,
            api::export::restore_database,
            export_as_markdown,
            export_as_html,
            export_as_pdf,
            // Clipboard history APIs
            get_clipboard_history,
            delete_clipboard_entries,
            create_note_from_history,
            copy_to_clipboard,
            add_clipboard_entry,
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
            save_custom_model_config,
            get_custom_model_config,
            list_custom_model_configs,
            delete_custom_model_config,
            test_custom_model_connection,
            summarize_clipboard_entries,
            // Database path management APIs
            api::database::get_current_database_path,
            api::database::get_database_info,
            api::database::select_database_file,
            api::database::create_new_database,
            api::database::reset_to_default_database,
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
