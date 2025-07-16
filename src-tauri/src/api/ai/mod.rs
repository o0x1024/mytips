pub mod conversations;
pub mod models;
pub mod roles;
pub mod service;

use crate::db::{self, DbManager};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use conversations::{Conversation, Message};
use futures_util::StreamExt;
use models::{
    send_message_to_ai, stream_message_from_ai, stream_message_with_images_from_ai, CustomModel,
};
use roles::{create_ai_role, get_ai_role};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::{mpsc, Mutex as TokioMutex};
use self::service::SaveAiConfigRequest;
use genai::chat::{ChatMessage, ChatRole};

// 系统提示词常量
const SYSTEM_PROMPT: &str = "";

type StreamCancelMap = Arc<TokioMutex<HashMap<String, Arc<AtomicBool>>>>;

// 全局流取消句柄
lazy_static::lazy_static! {
    static ref STREAM_CANCEL_MAP: StreamCancelMap = Arc::new(TokioMutex::new(HashMap::new()));
}

/// Command to cancel a specific AI stream.
#[tauri::command]
pub async fn cancel_ai_stream(stream_id: String) -> Result<(), String> {
    let cancel_map = STREAM_CANCEL_MAP.lock().await;
    if let Some(should_cancel) = cancel_map.get(&stream_id) {
        should_cancel.store(true, Ordering::SeqCst);
        println!("Stream {} cancellation requested.", stream_id);
    } else {
        eprintln!("Stream {} not found for cancellation.", stream_id);
    }
    Ok(())
}


// 发送AI消息
#[tauri::command]
pub async fn send_ai_message(
    _app: AppHandle,
    message: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_json_opt = db::get_setting(&conn, "ai_providers_config").await.map_err(|e| e.to_string())?;
    let config_json = config_json_opt.ok_or_else(|| "AI configuration not found in database".to_string())?;
    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
    let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Configuration for provider '{}' not found", provider_id))?;

    let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for provider '{}' not found", provider_id))?;
    let model_name = &provider_config.default_model;

    let role_description = if let Some(ref role_id) = role_id {
        if let Ok(role) = get_ai_role(role_id.clone(), db_manager.clone()).await {
            role.description
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    // 构建系统提示词
    let system_content = if role_description.is_empty() {
        SYSTEM_PROMPT.to_string()
    } else {
        format!("{}\n{}", SYSTEM_PROMPT, role_description)
    };

    // 收集聊天消息
    let mut chat_messages: Vec<ChatMessage> = Vec::new();
    chat_messages.push(ChatMessage {
        role: ChatRole::System,
        content: system_content.into(),
        options: Default::default(),
    });

    // 加载历史
    if let Some(conv_id) = conversation_id.clone() {
        let history = conversations::list_ai_messages(conv_id, db_manager.clone()).await?;
        let history_chat_messages: Vec<ChatMessage> = history
            .into_iter()
            .map(|m| {
                let role = match m.role.as_str() {
                    "user" => ChatRole::User,
                    "assistant" => ChatRole::Assistant,
                    "system" => ChatRole::System,
                    _ => ChatRole::User,
                };
                ChatMessage {
                    role,
                    content: m.content.into(),
                    options: Default::default(),
                }
            })
            .collect();
        chat_messages.extend(history_chat_messages);
    }

    // 当前用户消息
    chat_messages.push(ChatMessage {
        role: ChatRole::User,
        content: message.clone().into(),
        options: Default::default(),
    });

    println!("chat_messages: {:?}", chat_messages);
    // 调用AI
    let result_str = models::chat_with_history(api_key, model_name, chat_messages, &db_manager).await?;

    Ok(serde_json::json!({ "reply": result_str }))
}

// 流式发送AI消息
#[tauri::command]
pub async fn send_ai_message_stream(
    app: AppHandle,
    message: String,
    stream_id: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let (tx, mut rx) = mpsc::channel(1);

    let should_cancel = Arc::new(AtomicBool::new(false));
    STREAM_CANCEL_MAP.lock().await.insert(stream_id.clone(), should_cancel.clone());

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_json = db::get_setting(&conn, "ai_providers_config")
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "AI configuration not found".to_string())?;

    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse AI config: {}", e))?;

    let provider_config = ai_config.providers.get(&provider_id)
        .ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;

    let api_key = provider_config.api_key.clone()
        .ok_or_else(|| format!("API key for '{}' not found", provider_id))?;
    let model_name = provider_config.default_model.clone();

    let role_description = if let Some(ref role_id_str) = role_id {
        if let Ok(role) = get_ai_role(role_id_str.clone(), db_manager.clone()).await {
            role.description
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let system_content = if role_description.is_empty() {
        SYSTEM_PROMPT.to_string()
    } else {
        format!("{}\n{}", SYSTEM_PROMPT, role_description)
    };

    let mut chat_messages: Vec<ChatMessage> = Vec::new();
    chat_messages.push(ChatMessage {
        role: ChatRole::System,
        content: system_content.into(),
        options: Default::default(),
    });

    if let Some(conv_id) = conversation_id.clone() {
        let history = conversations::list_ai_messages(conv_id, db_manager.clone()).await.unwrap_or_default();
        let history_msgs: Vec<ChatMessage> = history
            .into_iter()
            .map(|m| {
                let role = match m.role.as_str() {
                    "user" => ChatRole::User,
                    "assistant" => ChatRole::Assistant,
                    "system" => ChatRole::System,
                    _ => ChatRole::User,
                };
                ChatMessage {
                    role,
                    content: m.content.into(),
                    options: Default::default(),
                }
            })
            .collect();
        chat_messages.extend(history_msgs);
    }

    chat_messages.push(ChatMessage {
        role: ChatRole::User,
        content: message.clone().into(),
        options: Default::default(),
    });

    println!("chat_messages: {:?}", chat_messages);
    let emitter_stream_id = stream_id.clone();
    let app_clone = app.clone();
    let handle = tauri::async_runtime::spawn(async move {
        let db_manager = app_clone.state::<DbManager>();
        let stream_result = models::stream_chat_with_history(
            api_key,
            &model_name,
            chat_messages,
            None,
            db_manager.inner(),
        )
        .await;

        match stream_result {
            Ok(mut stream) => {
                while let Some(chunk_result) = stream.next().await {
                    if should_cancel.load(Ordering::SeqCst) {
                        println!("Stream {} cancelled.", stream_id);
                        break;
                    }
                    let chunk = chunk_result.unwrap_or_else(|e| e.to_string());
                    if tx.send(Ok(serde_json::json!({ "chunk": chunk }))).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e.to_string())).await;
            }
        }
        STREAM_CANCEL_MAP.lock().await.remove(&stream_id);
        println!("Cleaned up stream_id: {}", stream_id);
    });

    let emitter_handle = tauri::async_runtime::spawn(async move {
        while let Some(result) = rx.recv().await {
            match result {
                Ok(json_val) => {
                    let mut payload = json_val.as_object().unwrap().clone();
                    payload.insert("id".to_string(), Value::String(emitter_stream_id.clone()));
                    payload.insert("done".to_string(), Value::Bool(false));
                    app.emit("ai-stream-chunk", payload).unwrap();
                }
                Err(e) => {
                    app.emit("ai-stream-error", e.to_string()).unwrap();
                    break;
                }
            }
        }
        app.emit(
            "ai-stream-chunk",
            serde_json::json!({
                "id": emitter_stream_id,
                "chunk": "",
                "done": true
            }),
        )
        .unwrap();
    });
    Ok(())
}

// 支持图片上传的AI消息发送（非流式）
#[tauri::command]
pub async fn send_ai_message_with_images(
    _app: AppHandle,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>,
    provider_id: String,
    role_id: Option<String>,
    _conversation_id: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_json_opt = db::get_setting(&conn, "ai_providers_config").await.map_err(|e| e.to_string())?;
    let config_json = config_json_opt.ok_or_else(|| "AI configuration not found in database".to_string())?;
    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
    let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Configuration for provider '{}' not found", provider_id))?;

    let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for provider '{}' not found", provider_id))?;
    let model_name = &provider_config.default_model;

    let final_message = if let Some(role_id) = role_id {
        if let Ok(role) = get_ai_role(role_id, db_manager.clone()).await {
            format!("{}\n{}", role.description, text_message)
        } else {
            text_message
        }
    } else {
        text_message
    };

    let result_str = models::send_message_with_images_to_ai(
        api_key,
        model_name,
        final_message,
        image_files,
        None,
        &db_manager,
    )
    .await?;
    Ok(serde_json::json!({ "reply": result_str }))
}

// 流式发送带图片的消息
#[tauri::command]
pub async fn send_ai_message_with_images_stream(
    app: AppHandle,
    text_message: String,
    stream_id: String,
    image_files: Vec<(String, Vec<u8>)>,
    provider_id: String,
    role_id: Option<String>,
    _conversation_id: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let (tx, mut rx) = mpsc::channel(1);

    let should_cancel = Arc::new(AtomicBool::new(false));
    STREAM_CANCEL_MAP.lock().await.insert(stream_id.clone(), should_cancel.clone());

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_json = db::get_setting(&conn, "ai_providers_config")
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "AI configuration not found".to_string())?;

    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json)
        .map_err(|e| format!("Failed to parse AI config: {}", e))?;

    let provider_config = ai_config.providers.get(&provider_id)
        .ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;

    let api_key = provider_config.api_key.clone()
        .ok_or_else(|| format!("API key for '{}' not found", provider_id))?;
    let model_name = provider_config.default_model.clone();

    let final_message = if let Some(role_id_str) = role_id {
        if let Ok(role) = get_ai_role(role_id_str, db_manager.clone()).await {
            format!("{}\n{}", role.description, text_message)
        } else {
            text_message.clone()
        }
    } else {
        text_message.clone()
    };

    let emitter_stream_id = stream_id.clone();
    let app_clone = app.clone();
    let handle = tauri::async_runtime::spawn(async move {
        let db_manager = app_clone.state::<DbManager>();
        let stream_result = stream_message_with_images_from_ai(
            api_key,
            &model_name,
            final_message,
            image_files,
            None,
            db_manager.inner(),
        )
        .await;

        match stream_result {
            Ok(mut stream) => {
                while let Some(chunk_result) = stream.next().await {
                    if should_cancel.load(Ordering::SeqCst) {
                        println!("Stream {} cancelled.", stream_id);
                        break;
                    }
                    let chunk = chunk_result.unwrap_or_else(|e| e.to_string());
                    if tx.send(Ok(serde_json::json!({ "chunk": chunk }))).await.is_err() {
                        break;
                    }
                }
            }
            Err(e) => {
                let _ = tx.send(Err(e.to_string())).await;
            }
        }
        STREAM_CANCEL_MAP.lock().await.remove(&stream_id);
        println!("Cleaned up stream_id for images: {}", stream_id);
    });

    let emitter_handle = tauri::async_runtime::spawn(async move {
        while let Some(result) = rx.recv().await {
            match result {
                Ok(json_val) => {
                    let mut payload = json_val.as_object().unwrap().clone();
                    payload.insert("id".to_string(), Value::String(emitter_stream_id.clone()));
                    payload.insert("done".to_string(), Value::Bool(false));
                    app.emit("ai-stream-chunk", payload).unwrap();
                }
                Err(e) => {
                    app.emit("ai-stream-error", e.to_string()).unwrap();
                    break;
                }
            }
        }
        app.emit(
            "ai-stream-chunk",
            serde_json::json!({
                "id": emitter_stream_id,
                "chunk": "",
                "done": true
            }),
        )
        .unwrap();
    });
    Ok(())
}

// 迁移旧的文件配置到数据库
#[tauri::command]
pub async fn migrate_config_to_database(
    app: AppHandle,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let config_dir = app.path().app_config_dir().unwrap();
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let api_keys = vec![
        "openai",
        "deepseek",
        "anthropic",
        "qwen",
        "doubao",
        "gemini",
        "grok",
    ];

    for key_name in api_keys {
        let key = format!("api_key_{}", key_name);
        let config_file_path = config_dir.join(format!("{}.conf", key));
        if config_file_path.exists() {
            if let Ok(value) = std::fs::read_to_string(config_file_path) {
                if let Ok(db_value) = crate::db::get_setting(&conn, &key).await {
                    if db_value.is_none() || db_value.unwrap_or_default().is_empty() {
                        let encoded_value = general_purpose::STANDARD.encode(value.trim());
                        let _ = crate::db::save_setting(&conn, &key, &encoded_value);
                    }
                }
            }
        }
    }

    Ok(())
}

// 保存自定义模型配置
#[tauri::command]
pub async fn save_custom_model_config(
    _app: tauri::AppHandle,
    config: CustomModel,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let model_id = config.model_id.clone();
    let config_key = format!("custom_model_config_{}", model_id);
    let api_key_key = format!("custom_api_key_{}", model_id);

    let config_str = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    let encoded_key = general_purpose::STANDARD.encode(&config.api_key);

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    db::save_setting(&conn, &config_key, &config_str).await.map_err(|e| e.to_string())?;
    db::save_setting(&conn, &api_key_key, &encoded_key).await.map_err(|e| e.to_string())?;

    Ok(())
}

// 获取自定义模型配置
#[tauri::command]
pub async fn get_custom_model_config(
    model_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<Option<CustomModel>, String> {
    let config_key = format!("custom_model_config_{}", model_id);
    let api_key_key = format!("custom_api_key_{}", model_id);
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let config_str = match db::get_setting(&conn, &config_key).await.map_err(|e| e.to_string())? {
        Some(s) => s,
        None => return Ok(None),
    };

    let mut config: CustomModel = serde_json::from_str(&config_str).map_err(|e| e.to_string())?;

    if let Ok(Some(encoded_key)) = db::get_setting(&conn, &api_key_key).await {
        if let Ok(decoded_key) = general_purpose::STANDARD.decode(encoded_key) {
            if let Ok(api_key) = String::from_utf8(decoded_key) {
                config.api_key = api_key;
            }
        }
    }

    Ok(Some(config))
}

// 列出所有自定义模型配置
#[tauri::command]
pub async fn list_custom_model_configs(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<CustomModel>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_keys = db::get_settings_by_prefix(&conn, "custom_model_config_").await
        .map_err(|e| e.to_string())?;
    let mut configs = Vec::new();

    for key in config_keys {
        let model_id = key.replace("custom_model_config_", "");
        if let Ok(Some(mut config)) =
            get_custom_model_config(model_id, db_manager.clone()).await
        {
            config.api_key = "********".to_string(); // Mask API key for list view
            configs.push(config);
        }
    }

    Ok(configs)
}

// 删除自定义模型配置
#[tauri::command]
pub async fn delete_custom_model_config(
    _app: tauri::AppHandle,
    model_id: String,
    db_manager: State<'_, DbManager>,
) -> Result<(), String> {
    let config_key = format!("custom_model_config_{}", model_id);
    let api_key_key = format!("custom_api_key_{}", model_id);
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    db::save_setting(&conn, &config_key, "").await.map_err(|e| e.to_string())?;
    db::save_setting(&conn, &api_key_key, "").await.map_err(|e| e.to_string())?;

    Ok(())
}

// 测试自定义模型连接
#[tauri::command]
pub async fn test_custom_model_connection(
    config: CustomModel,
    db_manager: State<'_, DbManager>,
) -> Result<serde_json::Value, String> {
    let result_str = models::send_message_to_custom_ai(
        config.endpoint,
        config.api_key,
        config.model_name,
        config.adapter_type,
        config.custom_headers,
        "Test connection".to_string(),
        &db_manager,
    )
    .await?;

    Ok(serde_json::json!({ "reply": result_str }))
}

#[tauri::command]
pub async fn summarize_clipboard_entries(
    _app: AppHandle,
    entry_ids: Vec<i64>,
    provider_id: String,
    prompt: Option<String>,
    db_manager: State<'_, DbManager>,
) -> Result<serde_json::Value, String> {
    if entry_ids.is_empty() {
        return Err("No entry IDs provided for summarization.".to_string());
    }

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    // Create a string of placeholders: "?,?,?"
    let placeholders = entry_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!("SELECT content FROM clipboard_history WHERE id IN ({})", placeholders);
    
    // Convert Vec<i64> to Vec<i32> for libsql params
    let params: Vec<i32> = entry_ids.iter().map(|id| *id as i32).collect();

    let mut rows = conn
        .query(&sql, libsql::params_from_iter(params))
        .await
        .map_err(|e| e.to_string())?;

    let mut entries = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| e.to_string())? {
        let content: String = row.get(0).map_err(|e| e.to_string())?;
        entries.push(content);
    }

    let content_to_summarize = entries.join("\n---\n");
    
    let summary_prompt = if let Some(p) = prompt {
        if p.trim().is_empty() {
            format!("请总结以下内容：\n\n{}", content_to_summarize)
        } else {
            p.replace("{{CONTENT}}", &content_to_summarize)
        }
    } else {
        format!("请总结以下内容：\n\n{}", content_to_summarize)
    };

    send_ai_message(
        _app,
        summary_prompt,
        provider_id,
        None,
        None,
        db_manager,
    ).await
}

// 导出service模块的命令
pub use service::{
    test_ai_connection,
    get_ai_chat_models,
    get_ai_embedding_models,
    get_default_ai_model,
    set_default_ai_model,
    save_ai_config,
    get_ai_usage_stats,
    reload_ai_services,
    get_ai_service_status,
};
