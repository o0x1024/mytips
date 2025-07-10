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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let config_json_opt = db::get_setting(&conn, "ai_providers_config").map_err(|e| e.to_string())?;
    let config_json = config_json_opt.ok_or_else(|| "AI configuration not found in database".to_string())?;
    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
    let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Configuration for provider '{}' not found", provider_id))?;

    let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for provider '{}' not found", provider_id))?;
    let model_name = &provider_config.default_model;

    let final_message = if let Some(role_id) = role_id {
        if let Ok(role) = get_ai_role(role_id, db_manager.clone()).await {
            format!("{}\n{}", role.description, message)
        } else {
            message
        }
    } else {
        message
    };

    let result_str = if let Some(conv_id) = conversation_id {
        let history = conversations::list_ai_messages(conv_id, db_manager.clone()).await?;
        let history_chat_messages = history
            .into_iter()
            .map(|m| {
                let role = match m.role.as_str() {
                    "user" => genai::chat::ChatRole::User,
                    "assistant" => genai::chat::ChatRole::Assistant,
                    "system" => genai::chat::ChatRole::System,
                    _ => genai::chat::ChatRole::User, // 默认
                };
                genai::chat::ChatMessage {
                    role,
                    content: m.content.into(),
                    options: Default::default(),
                }
            })
            .collect();
        models::chat_with_history(api_key, model_name, history_chat_messages).await?
    } else {
        send_message_to_ai(api_key, model_name, final_message).await?
    };

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

    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let config_json = db::get_setting(&conn, "ai_providers_config")
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
            format!("{}\n{}", role.description, message)
        } else {
            message
        }
    } else {
        message
    };

    let history_chat_messages = if let Some(conv_id) = conversation_id {
        conversations::list_ai_messages(conv_id, db_manager.clone())
            .await
            .unwrap_or_default()
            .into_iter()
            .map(|m| {
                let role = match m.role.as_str() {
                    "user" => genai::chat::ChatRole::User,
                    "assistant" => genai::chat::ChatRole::Assistant,
                    "system" => genai::chat::ChatRole::System,
                    _ => genai::chat::ChatRole::User,
                };
                genai::chat::ChatMessage {
                    role,
                    content: m.content.into(),
                    options: Default::default(),
                }
            })
            .collect::<Vec<_>>()
            .into()
    } else {
        None
    };

    let emitter_stream_id = stream_id.clone();
    let handle = tauri::async_runtime::spawn(async move {
        let stream_result = if let Some(history) = history_chat_messages {
            models::stream_chat_with_history(
                api_key,
                &model_name,
                history,
                None, // custom model name is not needed here
            )
            .await
        } else {
            stream_message_from_ai(
                api_key,
                &model_name,
                final_message,
                None, // custom model name is not needed here
            )
            .await
        };

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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let config_json_opt = db::get_setting(&conn, "ai_providers_config").map_err(|e| e.to_string())?;
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

    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let config_json = db::get_setting(&conn, "ai_providers_config")
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
    let handle = tauri::async_runtime::spawn(async move {
        let stream_result = stream_message_with_images_from_ai(
            api_key,
            &model_name,
            final_message,
            image_files,
            None,
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

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
                if let Ok(db_value) = crate::db::get_setting(&conn, &key) {
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

    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    db::save_setting(&conn, &config_key, &config_str).map_err(|e| e.to_string())?;
    db::save_setting(&conn, &api_key_key, &encoded_key).map_err(|e| e.to_string())?;

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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    let config_str = match db::get_setting(&conn, &config_key).map_err(|e| e.to_string())? {
        Some(s) => s,
        None => return Ok(None),
    };

    let mut config: CustomModel = serde_json::from_str(&config_str).map_err(|e| e.to_string())?;

    if let Ok(Some(encoded_key)) = db::get_setting(&conn, &api_key_key) {
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    let config_keys = db::get_settings_by_prefix(&conn, "custom_model_config_")
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
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    db::save_setting(&conn, &config_key, "").map_err(|e| e.to_string())?;
    db::save_setting(&conn, &api_key_key, "").map_err(|e| e.to_string())?;

    Ok(())
}

// 测试自定义模型连接
#[tauri::command]
pub async fn test_custom_model_connection(
    config: CustomModel,
) -> Result<serde_json::Value, String> {
    let result_str = models::send_message_to_custom_ai(
        config.endpoint,
        config.api_key,
        config.model_name,
        config.adapter_type,
        config.custom_headers,
        "Test connection".to_string(),
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

    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    // Create a string of placeholders: "?,?,?"
    let placeholders = entry_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!("SELECT content FROM clipboard_history WHERE id IN ({})", placeholders);
    
    // Convert Vec<i64> to something rusqlite can use with `params_from_iter`
    let params_from_ids = rusqlite::params_from_iter(entry_ids.iter());

    let entries: Vec<String> = conn
        .prepare(&sql)
        .map_err(|e| e.to_string())?
        .query_map(params_from_ids, |row| row.get(0))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<String>, _>>()
        .map_err(|e| e.to_string())?;

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
