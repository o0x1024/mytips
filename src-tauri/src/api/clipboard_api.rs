use crate::clipboard::ClipboardSettings;
use crate::db::{UnifiedDbManager, models::{Tip, TipType}, operations};
use tauri_plugin_clipboard_manager::ClipboardExt;
use chrono::Utc;
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Emitter, Manager, AppHandle};
use uuid::Uuid;
use libsql::params;

// 临时的剪贴板历史结构体
#[derive(Serialize, Clone)]
pub struct ClipboardHistory {
    pub id: i64,
    pub content: String,
    pub source: Option<String>,
    pub created_at: i64,
}

#[derive(Serialize)]
pub struct ClipboardHistoryPage {
    entries: Vec<ClipboardHistory>,
    total: i64,
}

// 辅助函数：获取统一数据库管理器
async fn get_unified_manager(app: &AppHandle) -> Result<UnifiedDbManager, String> {
    if let Some(manager) = app.try_state::<UnifiedDbManager>() {
        Ok((*manager.inner()).clone())
    } else {
        // 如果没有找到，尝试创建
        UnifiedDbManager::new(app.clone()).await
            .map_err(|e| format!("Failed to create unified manager: {}", e))
    }
}

#[tauri::command]
pub async fn get_clipboard_history(
    app: AppHandle,
    page: i64,
    page_size: i64,
    query: Option<String>,
) -> Result<ClipboardHistoryPage, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 计算偏移量
    let offset = page * page_size;
    
    // 获取总数和条目
    let (total, entries) = if let Some(query) = query {
        // 搜索模式
        let search_query = format!("%{}%", query);
        
        // 获取总数
        let mut total_rows = conn.query(
            "SELECT COUNT(*) FROM clipboard_history WHERE content LIKE ?1",
            params![search_query.clone()]
        ).await.map_err(|e| format!("Failed to get total count: {}", e))?;
        
        let total: i64 = if let Some(row) = total_rows.next().await.map_err(|e| format!("Failed to read total: {}", e))? {
            row.get(0).map_err(|e| format!("Failed to parse total: {}", e))?
        } else {
            0
        };
        
        // 获取条目
        let mut rows = conn.query(
            "SELECT id, content, source, created_at FROM clipboard_history 
             WHERE content LIKE ?1 
             ORDER BY created_at DESC 
             LIMIT ?2 OFFSET ?3",
            params![search_query, page_size, offset]
        ).await.map_err(|e| format!("Failed to query clipboard history: {}", e))?;
        
        let mut entries = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
            entries.push(ClipboardHistory {
                id: row.get(0).map_err(|e| format!("Failed to parse id: {}", e))?,
                content: row.get(1).map_err(|e| format!("Failed to parse content: {}", e))?,
                source: row.get(2).map_err(|e| format!("Failed to parse source: {}", e))?,
                created_at: row.get(3).map_err(|e| format!("Failed to parse created_at: {}", e))?,
            });
        }
        
        (total, entries)
    } else {
        // 正常分页模式
        // 获取总数
        let mut total_rows = conn.query(
            "SELECT COUNT(*) FROM clipboard_history",
            ()
        ).await.map_err(|e| format!("Failed to get total count: {}", e))?;
        
        let total: i64 = if let Some(row) = total_rows.next().await.map_err(|e| format!("Failed to read total: {}", e))? {
            row.get(0).map_err(|e| format!("Failed to parse total: {}", e))?
        } else {
            0
        };
        
        // 获取条目
        let mut rows = conn.query(
            "SELECT id, content, source, created_at FROM clipboard_history 
             ORDER BY created_at DESC 
             LIMIT ?1 OFFSET ?2",
            params![page_size, offset]
        ).await.map_err(|e| format!("Failed to query clipboard history: {}", e))?;
        
        let mut entries = Vec::new();
        while let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
            entries.push(ClipboardHistory {
                id: row.get(0).map_err(|e| format!("Failed to parse id: {}", e))?,
                content: row.get(1).map_err(|e| format!("Failed to parse content: {}", e))?,
                source: row.get(2).map_err(|e| format!("Failed to parse source: {}", e))?,
                created_at: row.get(3).map_err(|e| format!("Failed to parse created_at: {}", e))?,
            });
        }
        
        (total, entries)
    };

    Ok(ClipboardHistoryPage { entries, total })
}

#[tauri::command]
pub async fn get_clipboard_ids_for_last_days(
    days: u32,
    app: AppHandle,
) -> Result<Vec<i64>, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 计算N天前的时间戳（毫秒）
    let now = Utc::now();
    let days_ago = now - chrono::Duration::days(days as i64);
    let cutoff_timestamp = days_ago.timestamp_millis();

    let mut rows = conn.query(
        "SELECT id FROM clipboard_history WHERE created_at >= ?1 ORDER BY created_at DESC",
        params![cutoff_timestamp]
    ).await.map_err(|e| e.to_string())?;

    let mut ids = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        ids.push(row.get(0).map_err(|e| e.to_string())?);
    }

    Ok(ids)
}

#[tauri::command]
pub async fn delete_clipboard_entries(
    ids: Vec<i64>,
    app: AppHandle,
) -> Result<(), String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    for id in ids {
        operations::delete_clipboard_entry(&conn, id).await
            .map_err(|e| format!("Failed to delete clipboard entry {}: {}", id, e))?;
    }

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
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 添加到数据库
    operations::add_clipboard_entry(&conn, &selected_text, source.as_deref()).await
        .map_err(|e| format!("Failed to add selection to clipboard: {}", e))?;

    println!("选中文本已添加到临时笔记区");

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
    app: AppHandle,
) -> Result<serde_json::Value, String> {
    if ids.is_empty() {
        return Err("No clipboard entries selected".to_string());
    }

    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    // 获取选中的剪贴板条目内容
    let mut contents = Vec::new();
    for id in &ids {
        let mut rows = conn.query(
            "SELECT content, source, created_at FROM clipboard_history WHERE id = ?1",
            params![id]
        ).await.map_err(|e| e.to_string())?;

        if let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
            let content: String = row.get(0).map_err(|e| e.to_string())?;
            let source: Option<String> = row.get(1).map_err(|e| e.to_string())?;
            let created_at: i64 = row.get(2).map_err(|e| e.to_string())?;
            
            let formatted_content = if let Some(src) = source {
                format!("{}\n\n---\n来源：{}\n时间：{}", 
                    content, 
                    src, 
                    chrono::DateTime::from_timestamp_millis(created_at)
                        .unwrap_or_default()
                        .format("%Y-%m-%d %H:%M:%S")
                )
            } else {
                format!("{}\n\n---\n时间：{}", 
                    content,
                    chrono::DateTime::from_timestamp_millis(created_at)
                        .unwrap_or_default()
                        .format("%Y-%m-%d %H:%M:%S")
                )
            };
            contents.push(formatted_content);
        }
    }

    if contents.is_empty() {
        return Err("No content found for selected entries".to_string());
    }

    // 合并内容
    let combined_content = contents.join("\n\n---\n\n");
    let title = if contents.len() == 1 {
        "来自临时笔记的内容".to_string()
    } else {
        format!("来自临时笔记的内容 ({} 项)", contents.len())
    };

    let now = Utc::now().timestamp_millis();
    let tip = Tip {
        id: Uuid::new_v4().to_string(),
        title: title.clone(),
        content: combined_content.clone(),
        tip_type: TipType::Markdown,
        language: None,
        category_id: None,
        created_at: now,
        updated_at: now,
        version: Some(1),
        last_synced_at: Some(0),
        sync_hash: None,
        is_encrypted: Some(false),
        encryption_key_id: None,
        encrypted_content: None,
    };

    operations::create_tip(&conn, &tip).await
        .map_err(|e| e.to_string())?;
    
    Ok(serde_json::json!({
        "id": tip.id,
        "title": tip.title,
        "content": combined_content
    }))
}

#[tauri::command]
pub async fn copy_to_clipboard(app: tauri::AppHandle, text: String) -> Result<(), String> {
    app.clipboard()
        .write_text(text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_clipboard_settings(
    app: AppHandle,
) -> Result<ClipboardSettings, String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    match crate::db::operations::get_setting(&conn, "clipboard_settings").await {
        Ok(Some(settings_str)) => {
            match ClipboardSettings::from_json(&settings_str) {
                Ok(settings) => Ok(settings),
                Err(e) => {
                    eprintln!("Failed to parse clipboard settings: {}", e);
                    Ok(ClipboardSettings::default())
                }
            }
        },
        Ok(None) => Ok(ClipboardSettings::default()),
        Err(e) => {
            eprintln!("Failed to get clipboard settings: {}", e);
            Ok(ClipboardSettings::default())
        }
    }
}

#[tauri::command]
pub async fn save_clipboard_settings(
    app: AppHandle,
    settings: ClipboardSettings,
) -> Result<(), String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    let settings_str = settings.to_json()
        .map_err(|e| format!("Failed to serialize clipboard settings: {}", e))?;

    crate::db::operations::save_setting(&conn, "clipboard_settings", &settings_str).await
        .map_err(|e| format!("Failed to save clipboard settings: {}", e))?;

    // 同时更新监听状态
    if settings.enable_monitoring {
        crate::clipboard::MONITORING_ENABLED.store(true, std::sync::atomic::Ordering::SeqCst);
    } else {
        crate::clipboard::MONITORING_ENABLED.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    Ok(())
}

#[cfg(desktop)]
#[tauri::command]
pub async fn clean_expired_clipboard_entries(app: tauri::AppHandle) -> Result<(), String> {
    crate::clipboard::clean_expired_entries(&app).await;
    Ok(())
}

// 清除所有临时笔记的函数
#[tauri::command]
pub async fn clear_all_clipboard_entries(app: AppHandle) -> Result<(), String> {
    let unified_manager = get_unified_manager(&app).await?;
    let conn = unified_manager
        .get_conn()
        .await
        .map_err(|e| format!("Failed to get db connection: {}", e))?;

    conn.execute("DELETE FROM clipboard_history", ())
        .await
        .map_err(|e| format!("Failed to clear clipboard history: {}", e))?;

    Ok(())
}
