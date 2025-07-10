use crate::db::DbManager;
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: String,
    pub title: String,
    pub model: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: i64,
}

#[tauri::command]
pub async fn list_ai_conversations(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<Conversation>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, title, model, created_at, updated_at FROM ai_conversations ORDER BY updated_at DESC",
        )
        .map_err(|e| e.to_string())?;

    let conv_iter = stmt
        .query_map([], |row| {
            Ok(Conversation {
                id: row.get(0)?,
                title: row.get(1)?,
                model: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    conv_iter
        .map(|c| c.map_err(|e| e.to_string()))
        .collect()
}

#[tauri::command]
pub async fn list_ai_messages(
    conversation_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<Vec<Message>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT id, conversation_id, role, content, timestamp FROM ai_messages WHERE conversation_id = ? ORDER BY timestamp ASC",
        )
        .map_err(|e| e.to_string())?;

    let msg_iter = stmt
        .query_map(params![conversation_id], |row| {
            Ok(Message {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    msg_iter.map(|m| m.map_err(|e| e.to_string())).collect()
}

#[tauri::command]
pub async fn create_ai_conversation(
    title: String,
    model: String,
    db_manager: State<'_, DbManager>,
) -> Result<Conversation, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    let conversation = Conversation {
        id: Uuid::new_v4().to_string(),
        title,
        model,
        created_at: now,
        updated_at: now,
    };
    conn.execute(
        "INSERT INTO ai_conversations (id, title, model, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        params![
            conversation.id,
            conversation.title,
            conversation.model,
            conversation.created_at,
            conversation.updated_at
        ],
    )
    .map_err(|e| e.to_string())?;
    Ok(conversation)
}

#[tauri::command]
pub async fn delete_ai_conversation(
    conversation_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM ai_conversations WHERE id = ?",
        params![conversation_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn clear_ai_conversation(
    conversation_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM ai_messages WHERE conversation_id = ?",
        params![conversation_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_ai_conversation_title(
    conversation_id: String,
    title: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE ai_conversations SET title = ?, updated_at = ? WHERE id = ?",
        params![title, now, conversation_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_ai_message(
    conversation_id: String,
    role: String,
    content: String,
    db_manager: State<'_, DbManager>,
) -> Result<Message, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    let message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role,
        content,
        timestamp: now,
    };
    conn.execute(
        "INSERT INTO ai_messages (id, conversation_id, role, content, timestamp) VALUES (?, ?, ?, ?, ?)",
        params![
            message.id,
            message.conversation_id,
            message.role,
            message.content,
            message.timestamp
        ],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE ai_conversations SET updated_at = ? WHERE id = ?",
        params![now, conversation_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(message)
}
