pub mod conversations;
pub mod models;
pub mod roles;
pub mod service;

use crate::db::{self, UnifiedDbManager};
use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use futures_util::StreamExt;
use models::{stream_message_with_images_from_ai, CustomModel,
};
// use roles::{get_ai_role};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use tauri::{AppHandle, Emitter, Manager, State};
use tokio::sync::Mutex as TokioMutex;
use self::service::SaveAiConfigRequest;
use genai::chat::{ChatMessage, ChatRequest, ChatRole};
// use genai::Client;
use roles::get_ai_role_internal;
use conversations::list_ai_messages_internal;
use crate::api::settings::get_client_with_proxy;


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
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let (client, model_name) = if provider_id.starts_with("custom_") {
        let id = provider_id.strip_prefix("custom_").unwrap();
        let cfg_json = db::get_setting(&conn, "custom_ai_models")
            .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
        let custom_models: Vec<models::CustomModelConfig> = serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
        let m = custom_models.into_iter().find(|c| c.id == id).ok_or(format!("Custom model {} not found", id))?;
        
        let client = models::create_custom_genai_client(
            m.endpoint,
            m.api_key.unwrap_or_default(),
            m.model_name.clone(),
            m.adapter_type.clone(),
            None,
            db_manager.inner(),
        ).await?;
        (client, m.model_name)
    } else {
        let config_json = db::get_setting(&conn, "ai_providers_config")
            .await.map_err(|e| e.to_string())?.ok_or_else(|| "AI configuration not found".to_string())?;
        let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
        let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;
        let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for '{}' not found", provider_id))?;
        let model_name = provider_config.default_model.clone();

        let client = models::create_genai_client(api_key, &provider_id, db_manager.inner()).await?;
        (client, model_name)
    };
    
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
        chat_messages.push(ChatMessage {
            role: ChatRole::System,
            content: system_content.into(),
            options: Default::default(),
        });
    }

    // 加载历史消息，确保不会重复
    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id {
        let history = list_ai_messages_internal(conv_id.clone(), db_manager.inner().clone()).await?;
        
        println!("Loaded {} history messages for conversation {}", history.len(), conv_id);
        
        let history_chat_messages: Vec<ChatMessage> = history
            .into_iter()
            .filter_map(|m| {
                // 检查消息内容是否已经处理过，避免重复
                if seen_contents.insert(m.content.clone()) {
                    let role = match m.role.as_str() {
                        "user" => ChatRole::User,
                        "assistant" => ChatRole::Assistant,
                        "system" => ChatRole::System,
                        _ => ChatRole::User,
                    };
                    Some(ChatMessage { 
                        role, 
                        content: m.content.into(), 
                        options: Default::default() 
                    })
                } else {
                    println!("Skipping duplicate message content");
                    None
                }
            })
            .collect();
        
        chat_messages.extend(history_chat_messages);
    }

    // 添加当前用户消息，确保不重复
    if seen_contents.insert(message.clone()) {
        chat_messages.push(ChatMessage {
            role: ChatRole::User,
            content: message.clone().into(),
            options: Default::default(),
        });
    } else {
        println!("Current message is a duplicate of an existing message, skipping");
    }

    println!("chat_messages (non-stream): {:?}", chat_messages.len());

    // 针对特定模型添加自定义选项
    let mut request_options = genai::chat::ChatOptions::default();
    if provider_id.starts_with("custom_") {
        // 自定义模型的选项在请求中设置
        request_options.temperature = Some(0.7);
        request_options.max_tokens = Some(2000);
    }
    
    // 启用内容捕获，确保在流式响应结束时能获取完整内容
    request_options.capture_content = Some(true);
    
    println!("Request options: {:?}", request_options);

    let chat_req = ChatRequest::new(chat_messages);
    println!("Sending request to model: {}, adapter type: {}", model_name, if provider_id.starts_with("custom_") { "custom" } else { "standard" });
    let chat_res = client.exec_chat(&model_name, chat_req, Some(&request_options)).await.map_err(|e| {
        println!("Request failed: {}", e);
        e.to_string()
    })?;
    
    let result_str = chat_res.first_text().unwrap_or("").to_string();
    println!("Received response: {}", if result_str.is_empty() { "[empty]" } else { &result_str[..std::cmp::min(50, result_str.len())] });

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
        // Always emit final done marker once
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

    // 打印请求参数，帮助调试
    println!("Stream request params: provider_id={}, role_id={:?}, conversation_id={:?}", 
             provider_id, role_id, conversation_id);
    
    // 自定义模型优先尝试流式输出，不支持时回退到非流式模拟
    if provider_id.starts_with("custom_") {
        match handle_custom_model_as_stream(app.clone(), message.clone(), stream_id.clone(), provider_id.clone(), role_id.clone(), conversation_id.clone(), db_manager.clone(), should_cancel.clone()).await {
            Ok(()) => return Ok(()),
            Err(e) => {
                println!("[Custom Model] Stream not supported or failed, fallback to non-stream: {}", e);
                return handle_custom_model_as_nonstream(app, message, stream_id, provider_id, role_id, conversation_id, 
                                                        db_manager, should_cancel).await;
            }
        }
    }

    let (client, model_name) = if provider_id.starts_with("custom_") {
        let id = provider_id.strip_prefix("custom_").unwrap();
        let cfg_json = db::get_setting(&conn, "custom_ai_models")
            .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
        let custom_models: Vec<models::CustomModelConfig> = serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
        let m = custom_models.into_iter().find(|c| c.id == id).ok_or(format!("Custom model {} not found", id))?;
        
        println!("[Custom Model] Found model: id={}, name={}, adapter={}", id, m.model_name, m.adapter_type);
        
        let client = models::create_custom_genai_client(
            m.endpoint.clone(),
            m.api_key.clone().unwrap_or_default(),
            m.model_name.clone(),
            m.adapter_type.clone(),
            None,
            &db_manager,
        ).await?;
        (client, m.model_name.clone())
    } else {
        let config_json = db::get_setting(&conn, "ai_providers_config")
            .await.map_err(|e| e.to_string())?.ok_or_else(|| "AI configuration not found".to_string())?;
        let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
        let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;
        let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for '{}' not found", provider_id))?;
        let model_name = provider_config.default_model.clone();

        let client = models::create_genai_client(api_key, &provider_id, &db_manager).await?;
        (client, model_name)
    };

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
        chat_messages.push(ChatMessage {
            role: ChatRole::System,
            content: system_content.into(),
            options: Default::default(),
        });
    }

    // 加载历史消息，确保不会重复
    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id {
        let history = list_ai_messages_internal(conv_id.clone(), db_manager.clone()).await.unwrap_or_default();
        
        println!("Loaded {} history messages for conversation {}", history.len(), conv_id);
        
        let history_msgs: Vec<ChatMessage> = history.into_iter()
            .filter_map(|m| {
                // 检查消息内容是否已经处理过，避免重复
                if seen_contents.insert(m.content.clone()) {
                    let role = match m.role.as_str() {
                        "user" => ChatRole::User,
                        "assistant" => ChatRole::Assistant,
                        "system" => ChatRole::System,
                        _ => ChatRole::User,
                    };
                    Some(ChatMessage { 
                        role, 
                        content: m.content.into(), 
                        options: Default::default() 
                    })
                } else {
                    println!("Skipping duplicate message content");
                    None
                }
            })
            .collect();
        
        chat_messages.extend(history_msgs);
    }

    // 添加当前用户消息，确保不重复
    if seen_contents.insert(message.clone()) {
        chat_messages.push(ChatMessage {
            role: ChatRole::User,
            content: message.clone().into(),
            options: Default::default(),
        });
    } else {
        println!("Current message is a duplicate of an existing message, skipping");
    }

    // 打印详细的消息列表，便于调试
    println!("Final chat messages ({}):", chat_messages.len());
    for (i, msg) in chat_messages.iter().enumerate() {
        let content = match &msg.content {
            genai::chat::MessageContent::Text(text) => {
                // Safe char-boundary truncate to avoid UTF-8 panic
                let max_chars = 30;
                let preview: String = if text.chars().count() > max_chars {
                    text.chars().take(max_chars).collect::<String>() + "..."
                } else {
                    text.clone()
                };
                preview
            },
            _ => "[non-text content]".to_string(),
        };
        println!("  {}: {} - {}", i, msg.role, content);
    }

    // 如果没有消息，添加一个默认消息以避免空请求
    if chat_messages.is_empty() {
        println!("Warning: No messages to send, adding a default message");
        chat_messages.push(ChatMessage {
            role: ChatRole::User,
            content: "Hello".into(),
            options: Default::default(),
        });
    }

    // 针对特定模型添加自定义选项
    let mut request_options = genai::chat::ChatOptions::default();
    if provider_id.starts_with("custom_") {
        // 自定义模型的选项在请求中设置
        request_options.temperature = Some(0.7);
        request_options.max_tokens = Some(2000);
    }
    
    // 启用内容捕获，确保在流式响应结束时能获取完整内容
    request_options.capture_content = Some(true);
    
    println!("Request options: {:?}", request_options);
    
    let chat_req = ChatRequest::new(chat_messages.clone());
    println!("Sending stream request to model: {}, adapter type: {}", model_name, if provider_id.starts_with("custom_") { "custom" } else { "standard" });
    
    println!("Executing stream chat request to model: {}", model_name);
    let stream_resp = client.exec_chat_stream(&model_name, chat_req, Some(&request_options)).await.map_err(|e| {
        println!("Stream request failed: {}", e);
        e.to_string()
    })?;
    
    let mut stream = stream_resp.stream;
    println!("Stream connection established, waiting for events...");
    // 打印流响应信息
    println!("Stream response model: {:?}", stream_resp.model_iden);
    
    while let Some(event) = stream.next().await {
        if should_cancel.load(Ordering::SeqCst) {
            println!("Stream {} cancelled.", stream_id);
            break;
        }

        match event {
            Ok(genai::chat::ChatStreamEvent::Chunk(c)) => {
                let txt = c.content;
                println!("Received chunk: {}", txt);
                app.emit(
                    "ai-stream-chunk",
                    serde_json::json!({ "id": stream_id, "chunk": txt, "done": false }),
                ).ok();
            },
            Ok(genai::chat::ChatStreamEvent::End(end)) => {
                println!("Received End event: {:?}", end);
                // 处理End事件中可能包含的内容
                if let Some(contents) = end.captured_content {
                    for content in contents {
                        if let genai::chat::MessageContent::Text(text) = content {
                            if !text.is_empty() {
                                println!("Sending captured content: {}", text);
                                app.emit(
                                    "ai-stream-chunk",
                                    serde_json::json!({ "id": stream_id, "chunk": text, "done": false }),
                                ).ok();
                            }
                        }
                    }
                }
            },
            Ok(other_event) => {
                println!("Received non-chunk event: {:?}", other_event);
            },
            Err(e) => {
                println!("Stream error: {}", e);
                app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
                break;
            },
        }
    }
    
    println!("Stream completed or closed for id: {}", stream_id);

    Ok(())
}

// 自定义模型的流式实现：若失败则由调用方回退到非流式
async fn handle_custom_model_as_stream(
    app: AppHandle,
    message: String,
    stream_id: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: UnifiedDbManager,
    should_cancel: Arc<AtomicBool>,
) -> Result<(), String> {
    // 仅处理 custom_
    if !provider_id.starts_with("custom_") {
        return Err("Not a custom provider".to_string());
    }

    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let id = provider_id
        .strip_prefix("custom_")
        .ok_or_else(|| "Invalid custom provider id".to_string())?;

    let cfg_json = db::get_setting(&conn, "custom_ai_models")
        .await
        .map_err(|e| e.to_string())?
        .ok_or("No custom models configured")?;
    let custom_models: Vec<models::CustomModelConfig> =
        serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
    let m = custom_models
        .into_iter()
        .find(|c| c.id == id)
        .ok_or_else(|| format!("Custom model {} not found", id))?;

    println!(
        "[Custom Stream] Using model: id={}, name={}, adapter={}",
        id, m.model_name, m.adapter_type
    );

    // 角色描述
    let role_description = if let Some(ref role_id_str) = role_id {
        get_ai_role_internal(role_id_str.clone(), db_manager.clone())
            .await
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

    // 组装消息（含历史）
    let mut chat_messages: Vec<genai::chat::ChatMessage> = Vec::new();
    if !system_content.trim().is_empty() {
        chat_messages.push(genai::chat::ChatMessage {
            role: genai::chat::ChatRole::System,
            content: system_content.into(),
            options: Default::default(),
        });
    }

    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id.clone() {
        if let Ok(history) = list_ai_messages_internal(conv_id.clone(), db_manager.clone()).await {
            println!(
                "[Custom Stream] Loaded {} history messages for conversation {}",
                history.len(), conv_id
            );
            let history_msgs: Vec<genai::chat::ChatMessage> = history
                .into_iter()
                .filter_map(|m| {
                    if seen_contents.insert(m.content.clone()) {
                        let role = match m.role.as_str() {
                            "user" => genai::chat::ChatRole::User,
                            "assistant" => genai::chat::ChatRole::Assistant,
                            "system" => genai::chat::ChatRole::System,
                            _ => genai::chat::ChatRole::User,
                        };
                        Some(genai::chat::ChatMessage { role, content: m.content.into(), options: Default::default() })
                    } else {
                        println!("[Custom Stream] Skipping duplicate message content");
                        None
                    }
                })
                .collect();
            chat_messages.extend(history_msgs);
        }
    }

    if seen_contents.insert(message.clone()) {
        chat_messages.push(genai::chat::ChatMessage {
            role: genai::chat::ChatRole::User,
            content: message.clone().into(),
            options: Default::default(),
        });
    }

    // 如果是 Ollama，直接使用 /api/generate 进行流式输出
    if m.adapter_type.to_lowercase() == "ollama" {
        use serde_json::json;
        let client = get_client_with_proxy(&db_manager)
            .await
            .map_err(|e| e.to_string())?;
        let base = m
            .endpoint
            .trim_end_matches('/')
            .trim_end_matches("/v1")
            .trim_end_matches("/api")
            .to_string();
        let url = format!("{}/api/generate", base);
        println!("[Custom Stream][Ollama] POST {} (stream)", url);

        let mut req = client.post(&url).header("Content-Type", "application/json");
        if let Some(ref key) = m.api_key {
            if !key.is_empty() {
                req = req.header("Authorization", format!("Bearer {}", key));
            }
        }

        let payload = json!({
            "model": m.model_name,
            "prompt": message,
            "stream": true
        });

        let resp = req
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Ollama stream request failed: {}", e))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            return Err(format!("Ollama HTTP {}: {}", status, body_text));
        }

        let mut byte_stream = resp.bytes_stream();
        let mut buffer = String::new();

        while let Some(item) = byte_stream.next().await {
            if should_cancel.load(Ordering::SeqCst) {
                println!("Stream {} cancelled.", stream_id);
                break;
            }

            let chunk = item.map_err(|e| format!("Ollama stream read failed: {}", e))?;
            let s = String::from_utf8_lossy(&chunk);
            buffer.push_str(&s);

            // 逐行解析 JSON 行
            loop {
                if let Some(pos) = buffer.find('\n') {
                    let line = buffer[..pos].trim().to_string();
                    buffer.drain(..=pos);
                    if line.is_empty() { continue; }

                    match serde_json::from_str::<serde_json::Value>(&line) {
                        Ok(v) => {
                            // done 标记
                            if v.get("done").and_then(|b| b.as_bool()) == Some(true) {
                                println!("[Custom Stream][Ollama] Done received");
                                break;
                            }

                            if let Some(text) = v.get("response").and_then(|t| t.as_str()) {
                                if !text.is_empty() {
                                    app.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({ "id": stream_id, "chunk": text, "done": false }),
                                    ).ok();
                                }
                            }
                        }
                        Err(_) => {
                            // 如果不是标准 JSON 行，作为纯文本输出
                            app.emit(
                                "ai-stream-chunk",
                                serde_json::json!({ "id": stream_id, "chunk": line, "done": false }),
                            ).ok();
                        }
                    }
                } else {
                    break;
                }
            }
        }

        println!("[Custom Stream][Ollama] Stream finished for id: {}", stream_id);
        return Ok(());
    }

    // 客户端与选项（非 Ollama 适配器走 genai 流式）
    let client = models::create_custom_genai_client(
        m.endpoint.clone(),
        m.api_key.clone().unwrap_or_default(),
        m.model_name.clone(),
        m.adapter_type.clone(),
        None,
        &db_manager,
    )
    .await?;

    let mut request_options = genai::chat::ChatOptions::default();
    request_options.capture_content = Some(true);
    // 可选：设置自定义默认参数
    request_options.temperature = Some(0.7);
    request_options.max_tokens = Some(2000);

    let chat_req = genai::chat::ChatRequest::new(chat_messages.clone());
    println!(
        "[Custom Stream] Executing stream chat request to model: {}",
        m.model_name
    );

    let stream_resp = client
        .exec_chat_stream(&m.model_name, chat_req, Some(&request_options))
        .await
        .map_err(|e| {
            println!("[Custom Stream] Stream request failed: {}", e);
            e.to_string()
        })?;

    let mut stream = stream_resp.stream;
    println!("[Custom Stream] Stream connection established, waiting for events...");
    println!(
        "[Custom Stream] Stream response model: {:?}",
        stream_resp.model_iden
    );

    while let Some(event) = stream.next().await {
        if should_cancel.load(Ordering::SeqCst) {
            println!("Stream {} cancelled.", stream_id);
            break;
        }

        match event {
            Ok(genai::chat::ChatStreamEvent::Chunk(c)) => {
                let txt = c.content;
                if !txt.is_empty() {
                    app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": txt, "done": false }),
                    )
                    .ok();
                }
            }
            Ok(genai::chat::ChatStreamEvent::End(end)) => {
                if let Some(contents) = end.captured_content {
                    for content in contents {
                        if let genai::chat::MessageContent::Text(text) = content {
                            if !text.is_empty() {
                                app.emit(
                                    "ai-stream-chunk",
                                    serde_json::json!({ "id": stream_id, "chunk": text, "done": false }),
                                )
                                .ok();
                            }
                        }
                    }
                }
            }
            Ok(other) => {
                println!("[Custom Stream] Received non-chunk event: {:?}", other);
            }
            Err(e) => {
                println!("[Custom Stream] Stream error: {}", e);
                app.emit(
                    "ai-stream-error",
                    serde_json::json!({ "id": stream_id, "error": e.to_string() }),
                )
                .ok();
                break;
            }
        }
    }

    println!("[Custom Stream] Stream completed or closed for id: {}", stream_id);
    Ok(())
}

async fn handle_custom_model_as_nonstream(
    app: AppHandle,
    message: String,
    stream_id: String,
    provider_id: String,
    role_id: Option<String>,
    conversation_id: Option<String>,
    db_manager: UnifiedDbManager,
    should_cancel: Arc<AtomicBool>,
) -> Result<(), String> {
    println!("Using non-streaming fallback for custom model");
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let id = provider_id.strip_prefix("custom_").unwrap();
    let cfg_json = db::get_setting(&conn, "custom_ai_models")
        .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
    let custom_models: Vec<models::CustomModelConfig> = serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
    let m = custom_models.into_iter().find(|c| c.id == id).ok_or(format!("Custom model {} not found", id))?;
    
    println!("[Non-Stream] Using model: id={}, name={}, adapter={}", id, m.model_name, m.adapter_type);
    
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

    let mut chat_messages: Vec<genai::chat::ChatMessage> = Vec::new();
    if !system_content.trim().is_empty() {
        chat_messages.push(genai::chat::ChatMessage {
            role: genai::chat::ChatRole::System,
            content: system_content.into(),
            options: Default::default(),
        });
    }

    // 加载历史消息，确保不会重复
    let mut seen_contents = std::collections::HashSet::new();
    if let Some(conv_id) = conversation_id {
        if let Ok(history) = list_ai_messages_internal(conv_id.clone(), db_manager.clone()).await {
            println!("Loaded {} history messages for conversation {}", history.len(), conv_id);
            let history_msgs: Vec<genai::chat::ChatMessage> = history.into_iter()
                .filter_map(|m| {
                    // 检查消息内容是否已经处理过，避免重复
                    if seen_contents.insert(m.content.clone()) {
                        let role = match m.role.as_str() {
                            "user" => genai::chat::ChatRole::User,
                            "assistant" => genai::chat::ChatRole::Assistant,
                            "system" => genai::chat::ChatRole::System,
                            _ => genai::chat::ChatRole::User,
                        };
                        Some(genai::chat::ChatMessage { 
                            role, 
                            content: m.content.into(), 
                            options: Default::default() 
                        })
                    } else {
                        println!("Skipping duplicate message content");
                        None
                    }
                })
                .collect();
            chat_messages.extend(history_msgs);
        }
    }

    // 添加当前用户消息，确保不重复
    if seen_contents.insert(message.clone()) {
        chat_messages.push(genai::chat::ChatMessage {
            role: genai::chat::ChatRole::User,
            content: message.clone().into(),
            options: Default::default(),
        });
    } else {
        println!("Current message is a duplicate of an existing message, skipping");
    }

    // 使用非流式API获取全部响应
    println!("[Non-Stream] Sending request to model with {} messages", chat_messages.len());
    
    // 发起非流式请求
    tokio::spawn(async move {
        // 使用自定义API直接发送请求
        let result = models::send_message_to_custom_ai(
            m.endpoint,
            m.api_key.unwrap_or_default(),
            m.model_name,
            m.adapter_type,
            None,
            message,
            &db_manager,
        ).await;

        match result {
            Ok(full_response) => {
                println!("[Non-Stream] Received response of length: {} content:{}", full_response.len(),full_response);
                
                // 模拟流式输出，每次发送一小段内容
                for (i, chunk) in full_response.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                    if should_cancel.load(Ordering::SeqCst) {
                        println!("Stream {} cancelled.", stream_id);
                        break;
                    }
                    
                    if i % 20 == 0 {
                        println!("Sending chunk {} of simulated stream", i);
                    }

                    let chunk_str: String = chunk.iter().collect();
                    println!("[Debug] Emitting chunk: '{}', id: {}", chunk_str, stream_id);
                    // 确保事件名称和结构与前端预期一致
                    if let Err(err) = app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": chunk_str, "done": false }),
                    ) {
                        println!("[Error] Failed to emit chunk: {}", err);
                    }
                    
                    // 模拟网络延迟，使其看起来像真实的流式传输
                    tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
                }
                
                // 非流式模拟不再在此处发送最终结束标记，由外层统一发送
            },
                            Err(e) => {
                    println!("[Non-Stream] Error: {}", e);
                    app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
                    // 确保即使出错也发送流结束标志
                    // 先添加错误消息
                    let error_chunk = format!("Error: {}", e);
                    app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": error_chunk, "done": false }),
                    ).ok();
                    
                    // 添加短暂延迟
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    
                    // 非流式模拟错误时也不在此处发送结束标记，由外层统一发送
                }
        }
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
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Value, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    let config_json_opt = db::get_setting(&conn, "ai_providers_config").await.map_err(|e| e.to_string())?;
    let config_json = config_json_opt.ok_or_else(|| "AI configuration not found in database".to_string())?;
    let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
    let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Configuration for provider '{}' not found", provider_id))?;

    let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for provider '{}' not found", provider_id))?;
    let model_name = &provider_config.default_model;

    let final_message = if let Some(role_id) = role_id {
        if let Ok(role) = get_ai_role_internal(role_id, db_manager.inner().clone()).await {
            format!("{}\n{}", role.description.unwrap_or_default(), text_message)
        } else {
            text_message.clone()
        }
    } else {
        text_message.clone()
    };
    
    let client = models::create_genai_client(api_key, &provider_id, &db_manager).await?;
    let result_str = models::send_message_with_images_to_ai(
        client,
        model_name,
        final_message,
        image_files,
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
    _conversation_id: Option<String>, // Note: conversation_id is not used yet for image streams
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

    // 自定义模型使用非流式请求，然后模拟流式输出
    if provider_id.starts_with("custom_") {
        println!("Using non-streaming fallback for custom model with images");
        
        let id = provider_id.strip_prefix("custom_").unwrap();
        let cfg_json = db::get_setting(&conn, "custom_ai_models")
            .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
        let custom_models: Vec<models::CustomModelConfig> = serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
        let m = custom_models.into_iter().find(|c| c.id == id).ok_or(format!("Custom model {} not found", id))?;
        
        // 由于自定义模型可能不支持图片，我们转换为base64格式附加到消息中
        let mut enhanced_message = text_message.clone();
            for (i, (filename, _image_data)) in image_files.iter().enumerate() {
            enhanced_message.push_str(&format!("\n\n[Image {}]: {}", i + 1, filename));
        }
        
        let result = models::send_message_to_custom_ai(
            m.endpoint,
            m.api_key.unwrap_or_default(),
            m.model_name,
            m.adapter_type,
            None,
            enhanced_message,
            &db_manager,
        ).await;
        
        tokio::spawn(async move {
            match result {
                Ok(full_response) => {
                    println!("[Non-Stream] Received image response of length: {}", full_response.len());
                    
                    // 模拟流式输出，每次发送一小段内容
                    for (i, chunk) in full_response.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                        if should_cancel.load(Ordering::SeqCst) {
                            println!("Image stream {} cancelled.", stream_id);
                            break;
                        }
                        
                        if i % 20 == 0 {
                            println!("Sending chunk {} of simulated image stream", i);
                        }

                        let chunk_str: String = chunk.iter().collect();
                        println!("[Debug] Emitting image chunk: '{}', id: {}", chunk_str, stream_id);
                        // 确保事件名称和结构与前端预期一致
                        if let Err(err) = app.emit(
                            "ai-stream-chunk",
                            serde_json::json!({ "id": stream_id, "chunk": chunk_str, "done": false }),
                        ) {
                            println!("[Error] Failed to emit image chunk: {}", err);
                        }
                        
                        // 模拟网络延迟，使其看起来像真实的流式传输
                        tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
                    }
                    
                    // 发送结束标志
                    println!("[Non-Stream] Sending image stream end marker");
                    // 先添加一个明显的最终消息块，确保内容被发送
                    let final_chunk = ".";
                    if let Err(err) = app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": final_chunk, "done": false }),
                    ) {
                        println!("[Error] Failed to emit final image chunk: {}", err);
                    }
                    
                    // 添加短暂延迟确保前一个消息被处理
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    
                    // 发送结束标记
                    if let Err(err) = app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": "", "done": true }),
                    ) {
                        println!("[Error] Failed to emit image end marker: {}", err);
                    }
                },
                Err(e) => {
                    println!("[Non-Stream] Image request error: {}", e);
                    app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
                    // 确保即使出错也发送流结束标志
                    // 先添加错误消息
                    let error_chunk = format!("Error: {}", e);
                    app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": error_chunk, "done": false }),
                    ).ok();
                    
                    // 添加短暂延迟
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    
                    // 发送结束标记
                    app.emit(
                        "ai-stream-chunk",
                        serde_json::json!({ "id": stream_id, "chunk": "", "done": true }),
                    ).ok();
                }
            }
        });
        
        return Ok(());
    }

    let (client, model_name) = if provider_id.starts_with("custom_") {
         let id = provider_id.strip_prefix("custom_").unwrap();
        let cfg_json = db::get_setting(&conn, "custom_ai_models")
            .await.map_err(|e| e.to_string())?.ok_or("No custom models configured")?;
        let custom_models: Vec<models::CustomModelConfig> = serde_json::from_str(&cfg_json).map_err(|e| e.to_string())?;
        let m = custom_models.into_iter().find(|c| c.id == id).ok_or(format!("Custom model {} not found", id))?;
        
        let client = models::create_custom_genai_client(
            m.endpoint,
            m.api_key.unwrap_or_default(),
            m.model_name.clone(),
            m.adapter_type.clone(),
            None,
            &db_manager,
        ).await?;
        (client, m.model_name)
    } else {
        let config_json = db::get_setting(&conn, "ai_providers_config")
            .await.map_err(|e| e.to_string())?.ok_or_else(|| "AI configuration not found".to_string())?;
        let ai_config: SaveAiConfigRequest = serde_json::from_str(&config_json).map_err(|e| format!("Failed to parse AI config: {}", e))?;
        let provider_config = ai_config.providers.get(&provider_id).ok_or_else(|| format!("Provider '{}' not found in config", provider_id))?;
        let api_key = provider_config.api_key.clone().ok_or_else(|| format!("API key for '{}' not found", provider_id))?;
        let model_name = provider_config.default_model.clone();

        let client = models::create_genai_client(api_key, &provider_id, &db_manager).await?;
        (client, model_name)
    };

    let final_message = if let Some(role_id_str) = role_id {
        if let Ok(role) = get_ai_role_internal(role_id_str, db_manager.clone()).await {
            format!("{}\n{}", role.description.unwrap_or_default(), text_message)
        } else {
            text_message.clone()
        }
    } else {
        text_message.clone()
    };
    
    let stream_result = stream_message_with_images_from_ai(
        client,
        &model_name,
        final_message,
        image_files,
    )
    .await;

    match stream_result {
        Ok(mut stream) => {
            while let Some(chunk_result) = stream.next().await {
                if should_cancel.load(Ordering::SeqCst) {
                    println!("Stream {} cancelled.", stream_id);
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
            
            // 如果没有收到任何响应内容，尝试发送一个后备消息
            app.emit(
                "ai-stream-chunk",
                serde_json::json!({ 
                    "id": stream_id, 
                    "chunk": "I'm processing your message. Please wait a moment or try again if no response appears.", 
                    "done": false 
                }),
            ).ok();
        }
        Err(e) => {
             app.emit("ai-stream-error", serde_json::json!({ "id": stream_id, "error": e.to_string() })).ok();
        }
    }

    Ok(())
}


// 迁移旧的文件配置到数据库
#[tauri::command]
pub async fn migrate_config_to_database(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
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
    db_manager: State<'_, UnifiedDbManager>,
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
    db_manager: State<'_, UnifiedDbManager>,
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
pub async fn list_legacy_custom_model_configs(
    _db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<CustomModel>, String> {
    // TODO: 临时返回空列表，等实现get_settings_by_prefix函数后再启用
    Ok(Vec::new())
}

// 删除自定义模型配置
#[tauri::command]
pub async fn delete_legacy_custom_model_config(
    _app: tauri::AppHandle,
    model_id: String,
    db_manager: State<'_, UnifiedDbManager>,
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
    db_manager: State<'_, UnifiedDbManager>,
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
    db_manager: State<'_, UnifiedDbManager>,
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
// pub use service::{ save_ai_config };
