pub mod conversations;
pub mod rig_client;
pub mod roles;
pub mod service;

use crate::db::{self, UnifiedDbManager};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use futures_util::StreamExt;
use rig_client::{ChatMessage, RigProvider};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex as TokioMutex;
use self::service::SaveAiConfigRequest;
use roles::get_ai_role_internal;
use conversations::list_ai_messages_internal;

// System prompt constant
const SYSTEM_PROMPT: &str = "";

type StreamCancelMap = Arc<TokioMutex<HashMap<String, Arc<AtomicBool>>>>;

// Global stream cancel handles
lazy_static::lazy_static! {
    static ref STREAM_CANCEL_MAP: StreamCancelMap = Arc::new(TokioMutex::new(HashMap::new()));
}

/// Cancel a specific AI stream
#[tauri::command]
pub async fn cancel_ai_stream(stream_id: String) -> Result<(), String> {
    let cancel_map = STREAM_CANCEL_MAP.lock().await;
    if let Some(should_cancel) = cancel_map.get(&stream_id) {
        should_cancel.store(true, Ordering::SeqCst);
        println!("Stream {} cancellation requested", stream_id);
    } else {
        eprintln!("Stream {} not found for cancellation", stream_id);
    }
    Ok(())
}

/// Send AI message (non-streaming)
#[tauri::command]
pub async fn send_ai_message(
    _app: AppHandle,
    message: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let (provider, model_name) = create_provider_from_config(
        &provider_id,
        &conn,
    )
    .await?;
    
    let role_description = if let Some(ref role_id) = role_id {
        if let Ok(role) = get_ai_role_internal(role_id.clone(), db_manager.inner().clone()).await {
            role.description.unwrap_or_default()
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
    if !system_content.trim().is_empty() {
        chat_messages.push(ChatMessage::system(system_content));
    }

    // Load history messages, ensure no duplicates
    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id {
        let history = list_ai_messages_internal(conv_id.clone(), db_manager.inner().clone()).await.map_err(|e| e.to_string())?;
        
        println!("Loaded {} history messages for conversation {}", history.len(), conv_id);
        
        for m in history {
            if seen_contents.insert(m.content.clone()) {
                let msg = match m.role.as_str() {
                    "user" => ChatMessage::user(m.content),
                    "assistant" => ChatMessage::assistant(m.content),
                    "system" => ChatMessage::system(m.content),
                    _ => ChatMessage::user(m.content),
                };
                chat_messages.push(msg);
            }
        }
    }

    // Add current user message
    if seen_contents.insert(message.clone()) {
        chat_messages.push(ChatMessage::user(message.clone()));
    }

    println!("chat_messages count (non-stream): {}", chat_messages.len());

    let result_str = send_message_to_provider(&provider, &model_name, chat_messages, &db_manager)
        .await
        .map_err(|e| {
            println!("Request failed: {}", e);
            e.to_string()
        })?;
    
    println!("Received response: {}", if result_str.is_empty() { "[empty]" } else { &result_str[..std::cmp::min(50, result_str.len())] });

    Ok(serde_json::json!({ "reply": result_str }))
}

/// Stream AI message
#[tauri::command]
pub async fn send_ai_message_stream(
    app: AppHandle,
    message: String,
    stream_id: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let should_cancel = Arc::new(AtomicBool::new(false));
    STREAM_CANCEL_MAP.lock().await.insert(stream_id.clone(), should_cancel.clone());

    let app_clone = app.clone();
    let db_manager_clone = db_manager.inner().clone();
    let stream_id_clone = stream_id.clone();

    tauri::async_runtime::spawn(async move {
        let stream_result = handle_stream_request(
            app_clone.clone(),
            message,
            stream_id_clone.clone(),
            provider_id.clone(),
            role_id,
            conversation_id,
            db_manager_clone,
            should_cancel.clone(),
        ).await;

        if let Err(e) = stream_result {
            app_clone.emit("ai-stream-error", serde_json::json!({ "id": stream_id_clone, "error": e.to_string() })).ok();
        }
        
        // Always emit final done marker
        app_clone.emit(
            "ai-stream-chunk",
            serde_json::json!({
                "id": stream_id_clone,
                "chunk": "",
                "done": true
            }),
        ).ok();
        
        STREAM_CANCEL_MAP.lock().await.remove(&stream_id_clone);
        println!("Cleaned up stream_id: {}", stream_id_clone);
    });

    Ok(())
}

async fn handle_stream_request(
    app: AppHandle,
    message: String,
    stream_id: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: UnifiedDbManager,
    should_cancel: Arc<AtomicBool>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    println!("Stream request params: provider_id={}, role_id={:?}, conversation_id={:?}", 
             provider_id, role_id, conversation_id);
    
    let (provider, model_name) = create_provider_from_config(
        &provider_id,
        &conn,
    )
    .await?;

    let role_description = if let Some(ref role_id_str) = role_id {
        get_ai_role_internal(role_id_str.clone(), db_manager.clone()).await
            .map(|role| role.description.unwrap_or_default())
            .unwrap_or_default()
    } else {
        String::new()
    };

    let system_content = if role_description.is_empty() {
        SYSTEM_PROMPT.to_string()
    } else {
        format!("{}\n{}", SYSTEM_PROMPT, role_description)
    };

    let mut chat_messages: Vec<ChatMessage> = Vec::new();
    if !system_content.trim().is_empty() {
        chat_messages.push(ChatMessage::system(system_content));
    }

    // Load history messages, ensure no duplicates
    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id {
        let history = list_ai_messages_internal(conv_id.clone(), db_manager.clone()).await.unwrap_or_default();
        
        println!("Loaded {} history messages for conversation {}", history.len(), conv_id);
        
        for m in history {
            if seen_contents.insert(m.content.clone()) {
                let msg = match m.role.as_str() {
                    "user" => ChatMessage::user(m.content),
                    "assistant" => ChatMessage::assistant(m.content),
                    "system" => ChatMessage::system(m.content),
                    _ => ChatMessage::user(m.content),
                };
                chat_messages.push(msg);
            }
        }
    }

    // Add current user message
    if seen_contents.insert(message.clone()) {
        chat_messages.push(ChatMessage::user(message.clone()));
    }

    println!("Final chat messages: {}", chat_messages.len());

    // If no messages, add a default one
    if chat_messages.is_empty() {
        println!("Warning: No messages to send, adding default message");
        chat_messages.push(ChatMessage::user("Hello".to_string()));
    }

    println!("Sending stream request to model: {}", model_name);
    
    let stream_result = stream_message_from_provider(&provider, &model_name, chat_messages, &db_manager).await;
    
    match stream_result {
        Ok(mut stream) => {
            println!("Stream connection established, waiting for events...");
            
            while let Some(chunk_result) = stream.next().await {
                if should_cancel.load(Ordering::SeqCst) {
                    println!("Stream {} cancelled", stream_id);
                    break;
                }

                match chunk_result {
                    Ok(txt) => {
                        if !txt.is_empty() {
                            println!("Received chunk: {}", txt);
                            app.emit(
                                "ai-stream-chunk",
                                serde_json::json!({ "id": stream_id, "chunk": txt, "done": false }),
                            ).ok();
                        }
                    },
                    Err(e) => {
                        println!("Stream error: {}", e);
                        app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
                        break;
                    },
                }
            }
            
            println!("Stream completed for id: {}", stream_id);
        },
        Err(e) => {
            println!("Stream request failed: {}", e);
            app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
        }
    }

    Ok(())
}

/// Send message with images (non-streaming)
#[tauri::command]
pub async fn send_ai_message_with_images(
    _app: AppHandle,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>,
    provider_id: String,
    role_id: Option<String>,
    _conversation_id: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    let (provider, model_name) = create_provider_from_config(
        &provider_id,
        &conn,
    )
    .await?;

    let final_message = if let Some(role_id) = role_id {
        if let Ok(role) = get_ai_role_internal(role_id, db_manager.inner().clone()).await {
            format!("{}\n{}", role.description.unwrap_or_default(), text_message)
        } else {
            text_message.clone()
        }
    } else {
        text_message.clone()
    };
    
    // For now, convert images to base64 and include in message
    let mut enhanced_message = final_message;
    for (i, (filename, image_data)) in image_files.iter().enumerate() {
        let base64_image = general_purpose::STANDARD.encode(image_data);
        enhanced_message.push_str(&format!("\n\n[Image {}]: {} (base64: {}...)", 
            i + 1, filename, &base64_image[..std::cmp::min(50, base64_image.len())]));
    }

    let messages = vec![ChatMessage::user(enhanced_message)];
    let result_str = send_message_to_provider(&provider, &model_name, messages, &db_manager)
        .await
        .map_err(|e| e.to_string())?;

    Ok(serde_json::json!({ "reply": result_str }))
}

/// Send message with images (streaming)
#[tauri::command]
pub async fn send_ai_message_with_images_stream(
    app: AppHandle,
    text_message: String,
    stream_id: String,
    image_files: Vec<(String, Vec<u8>)>,
    provider_id: String,
    role_id: Option<String>,
    _conversation_id: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let should_cancel = Arc::new(AtomicBool::new(false));
    STREAM_CANCEL_MAP.lock().await.insert(stream_id.clone(), should_cancel.clone());

    let app_clone = app.clone();
    let db_manager_clone = db_manager.inner().clone();
    let stream_id_clone = stream_id.clone();

    tauri::async_runtime::spawn(async move {
        let result = handle_stream_request_with_images(
            app_clone.clone(),
            text_message,
            stream_id_clone.clone(),
            image_files,
            provider_id,
            role_id,
            db_manager_clone,
            should_cancel,
        ).await;

        if let Err(e) = result {
            app_clone.emit("ai-stream-error", serde_json::json!({ "id": stream_id_clone, "error": e.to_string() })).ok();
        }

        app_clone.emit(
            "ai-stream-chunk",
            serde_json::json!({
                "id": stream_id_clone,
                "chunk": "",
                "done": true
            }),
        ).ok();

        STREAM_CANCEL_MAP.lock().await.remove(&stream_id_clone);
        println!("Cleaned up stream_id for images: {}", stream_id_clone);
    });

    Ok(())
}

async fn handle_stream_request_with_images(
    app: AppHandle,
    text_message: String,
    stream_id: String,
    image_files: Vec<(String, Vec<u8>)>,
    provider_id: String,
    role_id: Option<String>,
    db_manager: UnifiedDbManager,
    should_cancel: Arc<AtomicBool>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let (provider, model_name) = create_provider_from_config(
        &provider_id,
        &conn,
    )
    .await?;

    let final_message = if let Some(role_id_str) = role_id {
        if let Ok(role) = get_ai_role_internal(role_id_str, db_manager.clone()).await {
            format!("{}\n{}", role.description.unwrap_or_default(), text_message)
        } else {
            text_message.clone()
        }
    } else {
        text_message.clone()
    };
    
    // Convert images to base64 and include in message
    let mut enhanced_message = final_message;
    for (i, (filename, image_data)) in image_files.iter().enumerate() {
        let base64_image = general_purpose::STANDARD.encode(image_data);
        enhanced_message.push_str(&format!("\n\n[Image {}]: {} (base64: {}...)", 
            i + 1, filename, &base64_image[..std::cmp::min(50, base64_image.len())]));
    }

    let messages = vec![ChatMessage::user(enhanced_message)];
    
    let stream_result = stream_message_from_provider(&provider, &model_name, messages, &db_manager).await;

    match stream_result {
        Ok(mut stream) => {
            while let Some(chunk_result) = stream.next().await {
                if should_cancel.load(Ordering::SeqCst) {
                    println!("Stream {} cancelled", stream_id);
                    break;
                }
                match chunk_result {
                    Ok(chunk) => {
                        app.emit("ai-stream-chunk", serde_json::json!({ "id": stream_id, "chunk": chunk, "done": false })).ok();
                    }
                    Err(e) => {
                        app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
                        break;
                    }
                }
            }
        }
        Err(e) => {
             app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
        }
    }

    Ok(())
}

/// Summarize clipboard entries
#[tauri::command]
pub async fn summarize_clipboard_entries(
    _app: AppHandle,
    entry_ids: Vec<i64>,
    provider_id: String,
    prompt: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<serde_json::Value, String> {
    if entry_ids.is_empty() {
        return Err("No entry IDs provided for summarization".to_string());
    }

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    // Create placeholders
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
            format!("Please summarize the following content:\n\n{}", content_to_summarize)
        } else {
            p.replace("{{CONTENT}}", &content_to_summarize)
        }
    } else {
        format!("Please summarize the following content:\n\n{}", content_to_summarize)
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

// ============ Helper Functions ============

/// Create a provider from configuration
async fn create_provider_from_config(
    provider_id: &str,
    conn: &libsql::Connection,
) -> Result<(RigProvider, String), String> {
    if provider_id.starts_with("custom_") {
        let id = provider_id.strip_prefix("custom_").unwrap();
        let cfg_json = db::get_setting(conn, "custom_ai_models")
            .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
        
        let custom_models: Vec<service::CustomModelConfig> = serde_json::from_str(&cfg_json)
            .map_err(|e| format!("Failed to parse custom models: {}", e))?;
        
        let m = custom_models.into_iter().find(|c| c.id == id)
            .ok_or(format!("Custom model {} not found", id))?;
        
        let provider = RigProvider::new(
            &m.adapter_type,
            m.api_key.unwrap_or_default(),
            Some(m.endpoint),
        ).map_err(|e| e.to_string())?;
        
        Ok((provider, m.model_name))
    } else {
        let config_json = db::get_setting(conn, "ai_providers_config")
            .await.map_err(|e| e.to_string())?
            .ok_or_else(|| "AI configuration not found".to_string())?;
        
        let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json)
            .map_err(|e| format!("Failed to parse AI config: {}", e))?;
        
        let provider_config = ai_config.providers.get(provider_id)
            .ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;
        
        let api_key = if provider_id.to_lowercase().contains("ollama") {
            String::new()
        } else {
            provider_config.api_key.clone()
                .ok_or_else(|| format!("API key for '{}' not found", provider_id))?
        };
        
        let model_name = provider_config.default_model.clone();
        
        // Ensure endpoint has a valid default
        let endpoint = provider_config.api_base.clone().or_else(|| {
            match provider_id {
                "deepseek" => Some("https://api.deepseek.com/v1".to_string()),
                "qwen" => Some("https://dashscope.aliyuncs.com/api/v1".to_string()),
                "doubao" => Some("https://ark.cn-beijing.volces.com/api/v3".to_string()),
                "ollama" => Some("http://localhost:11434".to_string()),
                "groq" => Some("https://api.groq.com/openai/v1".to_string()),
                "cohere" => Some("https://api.cohere.ai".to_string()),
                "zhipu" => Some("https://open.bigmodel.cn".to_string()),
                _ => None,
            }
        });

        let provider = RigProvider::new(provider_id, api_key, endpoint)
            .map_err(|e| e.to_string())?;
        
        Ok((provider, model_name))
    }
}

/// Send message to provider
async fn send_message_to_provider(
    provider: &RigProvider,
    model_name: &str,
    messages: Vec<ChatMessage>,
    db_manager: &UnifiedDbManager,
) -> Result<String, String> {
    match provider {
        RigProvider::Custom(custom) => {
            custom.send_request(model_name, messages, db_manager).await
                .map_err(|e| e.to_string())
        }
        _ => {
            // For rig-core native providers, convert to combined prompt
            let combined_prompt = messages
                .iter()
                .map(|msg| format!("{}: {}", msg.role, msg.content))
                .collect::<Vec<_>>()
                .join("\n");
            
            provider.prompt(model_name, &combined_prompt).await
                .map_err(|e| e.to_string())
        }
    }
}

/// Stream message from provider
async fn stream_message_from_provider(
    provider: &RigProvider,
    model_name: &str,
    messages: Vec<ChatMessage>,
    db_manager: &UnifiedDbManager,
) -> Result<std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<String, anyhow::Error>> + Send>>, String> {
    match provider {
        RigProvider::Custom(custom) => {
            custom.send_stream_request(model_name, messages, db_manager).await
                .map_err(|e| e.to_string())
        }
        _ => {
            Err("Streaming not implemented for standard providers".to_string())
        }
    }
}
