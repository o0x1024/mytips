use crate::db::UnifiedDbManager;
use chrono::Utc;
use libsql::params;
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
    pub timestamp: i64, // 保持前端兼容性，仍使用timestamp字段名
}

#[tauri::command]
pub async fn list_ai_conversations(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<Conversation>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let mut rows = conn
        .query(
            "SELECT id, title, model, created_at, updated_at FROM ai_conversations ORDER BY updated_at DESC",
            ()
        )
        .await
        .map_err(|e| e.to_string())?;

    let mut conversations = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        conversations.push(Conversation {
            id: row.get::<String>(0).map_err(|e| e.to_string())?,
            title: row.get::<String>(1).map_err(|e| e.to_string())?,
            model: row.get::<String>(2).map_err(|e| e.to_string())?,
            created_at: row.get::<i64>(3).map_err(|e| e.to_string())?,
            updated_at: row.get::<i64>(4).map_err(|e| e.to_string())?,
        });
    }

    Ok(conversations)
}

#[tauri::command]
pub async fn list_ai_messages(
    conversation_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<Message>, String> {
    list_ai_messages_internal(conversation_id, db_manager.inner().clone()).await
}

pub async fn list_ai_messages_internal(
    conversation_id: String,
    db_manager: UnifiedDbManager,
) -> Result<Vec<Message>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let mut rows = conn
        .query(
            "SELECT id, conversation_id, role, content, created_at FROM ai_messages WHERE conversation_id = ? ORDER BY created_at ASC",
            params![conversation_id]
        )
        .await
        .map_err(|e| e.to_string())?;

    let mut messages = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        messages.push(Message {
            id: row.get::<String>(0).map_err(|e| e.to_string())?,
            conversation_id: row.get::<String>(1).map_err(|e| e.to_string())?,
            role: row.get::<String>(2).map_err(|e| e.to_string())?,
            content: row.get::<String>(3).map_err(|e| e.to_string())?,
            timestamp: row.get::<i64>(4).map_err(|e| e.to_string())?, // 从created_at读取但返回为timestamp
        });
    }

    Ok(messages)
}

#[tauri::command]
pub async fn create_ai_conversation(
    title: String,
    model: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Conversation, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
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
            conversation.id.clone(),
            conversation.title.clone(),
            conversation.model.clone(),
            conversation.created_at,
            conversation.updated_at
        ],
    ).await
    .map_err(|e| e.to_string())?;
    Ok(conversation)
}

#[tauri::command]
pub async fn delete_ai_conversation(
    conversation_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM ai_conversations WHERE id = ?",
        params![conversation_id],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn clear_ai_conversation(
    conversation_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    conn.execute(
        "DELETE FROM ai_messages WHERE conversation_id = ?",
        params![conversation_id],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn update_ai_conversation_title(
    conversation_id: String,
    title: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    conn.execute(
        "UPDATE ai_conversations SET title = ?, updated_at = ? WHERE id = ?",
        params![title, now, conversation_id],
    )
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn add_ai_message(
    conversation_id: String,
    role: String,
    content: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Message, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();
    let message = Message {
        id: Uuid::new_v4().to_string(),
        conversation_id: conversation_id.clone(),
        role,
        content,
        timestamp: now,
    };
    conn.execute(
        "INSERT INTO ai_messages (id, conversation_id, role, content, created_at) VALUES (?, ?, ?, ?, ?)",
        params![
            message.id.clone(),
            message.conversation_id.clone(),
            message.role.clone(),
            message.content.clone(),
            message.timestamp
        ],
    )
    .await
    .map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE ai_conversations SET updated_at = ? WHERE id = ?",
        params![now, conversation_id],
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(message)
}
