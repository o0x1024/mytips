use crate::clipboard::ClipboardSettings;
use crate::db::{ClipboardHistory, DbManager, Tip, TipType};
use tauri_plugin_clipboard_manager::ClipboardExt;
use chrono::Utc;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};
use uuid::Uuid;

#[derive(Serialize)]
pub struct ClipboardHistoryPage {
    entries: Vec<ClipboardHistory>,
    total: i64,
}

#[tauri::command]
pub async fn get_clipboard_history(
    db_manager: State<'_, DbManager>,
    page: i64,
    page_size: i64,
    query: Option<String>,
) -> Result<ClipboardHistoryPage, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let entries =
        crate::db::get_clipboard_entries_paged(&conn, page, page_size, query.clone())
            .await.map_err(|e| e.to_string())?;
    let total = crate::db::get_clipboard_entries_count(&conn, query)
        .await.map_err(|e| e.to_string())?;

    Ok(ClipboardHistoryPage {
        entries,
        total,
    })
}

#[tauri::command]
pub async fn get_clipboard_ids_for_last_days(
    days: u32,
    db_manager: State<'_, DbManager>,
) -> Result<Vec<i64>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    crate::db::get_clipboard_entry_ids_by_days(&conn, days).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_clipboard_entries(
    ids: Vec<i64>,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;
    crate::db::delete_clipboard_entries(&conn, &ids).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_clipboard_entry(
    content: String,
    source: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    if content.is_empty() {
        return Err("Content cannot be empty".to_string());
    }

    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;
    crate::db::add_clipboard_entry(&conn, &content, source.as_deref()).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_selection_to_clipboard(app: tauri::AppHandle) -> Result<(), String> {
    // 尝试获取当前选中文本
    let selected_text = match crate::clipboard::get_selected_text(&app) {
        Some(text) => {
            if text.is_empty() {
                return Err("没有选中文本".to_string());
            }
            text
        }
        None => return Err("无法获取选中文本".to_string()),
    };

    // 获取当前窗口标题作为来源
    let source = crate::clipboard::get_active_window_title();
    println!("获取到的选中文本: {}", selected_text);
    println!("来源: {:?}", source);

    // 获取数据库连接
    let db_manager: State<'_, DbManager> = app.state();
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 检查是否已经存在相同内容
    let content_exists = match crate::db::check_clipboard_entry_exists(&conn, &selected_text).await {
        Ok(exists) => exists,
        Err(e) => {
            eprintln!("检查剪贴板内容是否存在失败: {}", e);
            false // 如果检查失败，继续尝试添加
        }
    };

    if content_exists {
        println!("相同内容已存在，跳过添加");
        return Ok(());
    }

    // 添加到数据库
    if let Err(e) = crate::db::add_clipboard_entry(&conn, &selected_text, source.as_deref()).await {
        return Err(format!("添加到临时笔记区失败: {}", e));
    }

    // 通知前端更新
    let app_handle = app.clone();
    if let Err(e) = app_handle.emit("new-clipboard-entry", ()) {
        eprintln!("发送new-clipboard-entry事件失败: {}", e);
        return Err(format!("发送更新事件失败: {}", e));
    }

    Ok(())
}

#[tauri::command]
pub async fn create_note_from_history(
    ids: Vec<i64>,
    db_manager: State<'_, DbManager>,
) -> Result<serde_json::Value, String> {
    if ids.is_empty() {
        return Err("No clipboard entries selected".to_string());
    }

    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;
    let entries = crate::db::get_all_clipboard_entries(&conn).await.map_err(|e| e.to_string())?;

    let selected_entries: Vec<_> = entries
        .into_iter()
        .filter(|e| ids.contains(&e.id))
        .collect();
    if selected_entries.is_empty() {
        return Err("No valid clipboard entries found".to_string());
    }

    let content = selected_entries
        .iter()
        .map(|e| {
            if let Some(ref source) = e.source {
                format!("// 来源: {}\n{}\n\n", source, e.content)
            } else {
                format!("{}\n\n", e.content)
            }
        })
        .collect::<String>();

    let tip = Tip {
        id: Uuid::new_v4().to_string(),
        title: "来自临时笔记的内容".to_string(),
        content,
        tip_type: TipType::Markdown,
        language: None,
        category_id: None,
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
    };

    crate::db::save_tip(&conn, tip).await
        .map_err(|e| e.to_string())
        .map(|t| {
            serde_json::json!({
                "id": t.id,
                "title": t.title,
                "content": t.content
            })
        })
}

#[tauri::command]
pub async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_clipboard_settings(
    db_manager: State<'_, DbManager>,
) -> Result<ClipboardSettings, String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    match crate::db::get_setting(&conn, "clipboard_settings").await {
        Ok(Some(settings_json)) => ClipboardSettings::from_json(&settings_json),
        Ok(None) => {
            // 返回默认设置
            Ok(ClipboardSettings::default())
        }
        Err(e) => Err(format!("获取剪贴板设置失败: {}", e)),
    }
}

#[tauri::command]
pub async fn save_clipboard_settings(
    settings: ClipboardSettings,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 序列化设置
    let settings_json = settings.to_json()?;

    // 保存到数据库
    crate::db::save_setting(&conn, "clipboard_settings", &settings_json).await
        .map_err(|e| format!("保存剪贴板设置失败: {}", e))
}

#[cfg(desktop)]
#[tauri::command]
pub async fn clean_expired_clipboard_entries(app: tauri::AppHandle) -> Result<(), String> {
    crate::clipboard::clean_expired_entries(&app).await;
    Ok(())
}

// 新增清除所有临时笔记的函数
#[tauri::command]
pub async fn clear_all_clipboard_entries(db_manager: State<'_, DbManager>) -> Result<(), String> {
    let conn = db_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;
    crate::db::clear_all_clipboard_entries(&conn).await.map_err(|e| format!("清除所有临时笔记失败: {}", e))
}
