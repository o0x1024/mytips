use crate::db::DbManager;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct AIConversation {
    pub id: String,
    pub title: String,
    pub model: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AIMessage {
    pub id: String,
    pub conversation_id: String,
    pub role: String,
    pub content: String,
    pub timestamp: i64,
}

// 获取所有对话
#[tauri::command]
pub async fn list_ai_conversations() -> Result<Vec<AIConversation>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare("SELECT id, title, model, created_at, updated_at FROM ai_conversations ORDER BY updated_at DESC")
        .map_err(|e| e.to_string())?;

    let conversations = stmt
        .query_map([], |row| {
            Ok(AIConversation {
                id: row.get(0)?,
                title: row.get(1)?,
                model: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for conversation in conversations {
        result.push(conversation.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

// 获取单个对话的所有消息
#[tauri::command(rename_all = "snake_case")]
pub async fn list_ai_messages(conversation_id: String) -> Result<Vec<AIMessage>, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let mut stmt = db.conn.prepare("SELECT id, conversation_id, role, content, timestamp FROM ai_messages WHERE conversation_id = ? ORDER BY timestamp ASC")
        .map_err(|e| e.to_string())?;

    let messages = stmt
        .query_map([conversation_id], |row| {
            Ok(AIMessage {
                id: row.get(0)?,
                conversation_id: row.get(1)?,
                role: row.get(2)?,
                content: row.get(3)?,
                timestamp: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for message in messages {
        result.push(message.map_err(|e| e.to_string())?);
    }

    Ok(result)
}

// 创建新对话
#[tauri::command]
pub async fn create_ai_conversation(
    model: String,
    title: Option<String>,
) -> Result<String, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();
    let title =
        title.unwrap_or_else(|| format!("新对话 {}", chrono::Local::now().format("%m-%d %H:%M")));

    db.conn.execute(
        "INSERT INTO ai_conversations (id, title, model, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        &[&id, &title, &model, &now.to_string(), &now.to_string()],
    ).map_err(|e| e.to_string())?;

    Ok(id)
}

// 删除对话
#[tauri::command(rename_all = "snake_case")]
pub async fn delete_ai_conversation(conversation_id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;

    // 先删除所有关联的消息
    db.conn
        .execute(
            "DELETE FROM ai_messages WHERE conversation_id = ?",
            &[&conversation_id],
        )
        .map_err(|e| e.to_string())?;

    // 再删除对话本身
    db.conn
        .execute(
            "DELETE FROM ai_conversations WHERE id = ?",
            &[&conversation_id],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

// 更新对话标题
#[tauri::command(rename_all = "snake_case")]
pub async fn update_ai_conversation_title(
    conversation_id: String,
    new_title: String,
) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();

    db.conn
        .execute(
            "UPDATE ai_conversations SET title = ?, updated_at = ? WHERE id = ?",
            &[&new_title, &now.to_string(), &conversation_id],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

// 添加消息
#[tauri::command(rename_all = "snake_case")]
pub async fn add_ai_message(
    conversation_id: String,
    role: String,
    content: String,
) -> Result<String, String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().timestamp_millis();

    // 插入消息
    db.conn.execute(
        "INSERT INTO ai_messages (id, conversation_id, role, content, timestamp) VALUES (?, ?, ?, ?, ?)",
        &[&id, &conversation_id, &role, &content, &now.to_string()],
    ).map_err(|e| e.to_string())?;

    // 更新对话的更新时间
    db.conn
        .execute(
            "UPDATE ai_conversations SET updated_at = ? WHERE id = ?",
            &[&now.to_string(), &conversation_id],
        )
        .map_err(|e| e.to_string())?;

    Ok(id)
}

// 清空对话中的所有消息（但保留对话本身）
#[tauri::command(rename_all = "snake_case")]
pub async fn clear_ai_conversation(conversation_id: String) -> Result<(), String> {
    let db = DbManager::init().map_err(|e| e.to_string())?;
    let now = Utc::now().timestamp_millis();

    // 删除该对话的所有消息
    db.conn
        .execute(
            "DELETE FROM ai_messages WHERE conversation_id = ?",
            &[&conversation_id],
        )
        .map_err(|e| e.to_string())?;

    // 更新对话的更新时间
    db.conn
        .execute(
            "UPDATE ai_conversations SET updated_at = ? WHERE id = ?",
            &[&now.to_string(), &conversation_id],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}
