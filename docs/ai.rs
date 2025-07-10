use crate::services::ai::{AiServiceManager, AiConfig, AiToolCall};
use crate::models::database::{AiConversation, AiMessage};
use crate::services::database::{Database, DatabaseService};
use tauri::{AppHandle, State, Manager, Emitter};
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use genai::chat::{ChatMessage, ChatRequest, ChatStreamEvent, Tool};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use futures::StreamExt;
use genai::{Client as GenaiClient};
use genai::chat::{ChatOptions, StreamChunk};
use reqwest::header::{HeaderMap, HeaderValue};
use std::time::Duration;
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    pub title: Option<String>,
    pub service_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: String,
    pub message: String,
    pub service_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SendStreamMessageRequest {
    pub conversation_id: String,
    pub message: String,
    pub service_name: Option<String>,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
    pub system_prompt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExecuteToolCallRequest {
    pub conversation_id: String,
    pub service_name: String,
    pub tool_call: AiToolCall,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiServiceInfo {
    pub name: String,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiModelInfo {
    pub provider: String,
    pub models: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiProviderConfig {
    pub id: String,
    pub provider: String,
    pub name: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub organization: Option<String>,
    pub enabled: bool,
    pub default_model: String,
    pub models: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionRequest {
    pub provider: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub organization: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestConnectionResponse {
    pub success: bool,
    pub message: String,
    pub models: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveAiConfigRequest {
    pub providers: HashMap<String, AiProviderConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiServiceStatusResponse {
    pub provider: String,
    pub is_available: bool,
    pub models_count: usize,
    pub active_conversations: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub provider: String,
    pub is_chat: bool,
    pub is_embedding: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelConfig {
    pub name: String,
    pub provider: String,
    pub config: serde_json::Value,
}

// 列出所有AI服务
#[tauri::command]
pub async fn list_ai_services(
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Vec<String>, String> {
    Ok(ai_manager.list_services())
}

// 打印所有AI对话消息
#[tauri::command]
pub async fn print_ai_conversations(
    app: AppHandle,
) -> Result<String, String> {
    // 获取数据库服务
    let db = match app.try_state::<Arc<crate::services::database::DatabaseService>>() {
        Some(db) => db,
        None => return Err("Database service not initialized".to_string()),
    };
    
    // 获取所有对话
    let conversations = match db.get_ai_conversations().await {
        Ok(convs) => convs,
        Err(e) => return Err(format!("Failed to get conversation list: {}", e)),
    };
    
    // 如果没有对话，返回提示信息
    if conversations.is_empty() {
        return Ok("No AI conversation records found".to_string());
    }
    
    // 构建输出字符串
    let mut output = String::new();
    output.push_str(&format!("Found {} AI conversations\n\n", conversations.len()));
    
    // 遍历每个对话
    for (idx, conv) in conversations.iter().enumerate() {
        output.push_str(&format!("对话 {}/{}: {} (ID: {})\n", 
            idx + 1, 
            conversations.len(),
            conv.title.as_deref().unwrap_or("No title"),
            conv.id
        ));
        output.push_str(&format!("Created time: {}\n", conv.created_at.format("%Y-%m-%d %H:%M:%S")));
        output.push_str(&format!("Model: {} ({})\n", conv.model_name, conv.model_provider.as_deref().unwrap_or("Unknown")));
        output.push_str(&format!("Message count: {}\n", conv.total_messages));
        output.push_str("------------------------------------\n");
        
        // 获取此对话的所有消息
        let messages = match db.get_ai_messages_by_conversation(&conv.id).await {
            Ok(msgs) => msgs,
            Err(e) => {
                output.push_str(&format!("Failed to get messages: {}\n\n", e));
                continue;
            }
        };
        
        // 打印每条消息
        for (msg_idx, msg) in messages.iter().enumerate() {
            let role_str = match msg.role.as_str() {
                "user" => "User",
                "assistant" => "Assistant",
                "system" => "System",
                "tool" => "Tool",
                _ => msg.role.as_str(),
            };
            
            output.push_str(&format!("Message {}/{} - {} ({})\n", 
                msg_idx + 1, 
                messages.len(),
                role_str,
                msg.timestamp.format("%H:%M:%S")
            ));
            
            // 打印消息内容
            output.push_str(&format!("{}\n", msg.content));
            
            // 如果有Token使用情况，打印出来
            if let Some(token_count) = msg.token_count {
                output.push_str(&format!("Token usage: {}\n", token_count));
                
                if let Some(cost) = msg.cost {
                    output.push_str(&format!("Cost: ${:.6}\n", cost));
                }
            }
            
            // 如果有工具调用情况，打印出来
            if let Some(tool_calls_json) = &msg.tool_calls {
                // 尝试解析JSON
                if let Ok(tool_calls) = serde_json::from_str::<Vec<serde_json::Value>>(tool_calls_json) {
                    if !tool_calls.is_empty() {
                        let tool_names: Vec<String> = tool_calls.iter()
                            .filter_map(|t| t.get("name").and_then(|n| n.as_str()).map(String::from))
                            .collect();
                        
                        if !tool_names.is_empty() {
                            output.push_str(&format!("Used tools: {}\n", tool_names.join(", ")));
                        }
                    }
                }
            }
            
            output.push_str("------------------------------------\n");
        }
        
        output.push_str("\n\n");
    }
    
    Ok(output)
}

// 添加AI服务
#[tauri::command]
pub async fn add_ai_service(
    name: String,
    config: AiConfig,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<(), String> {
    ai_manager.add_service(name, config).await.map_err(|e| e.to_string())
}

// 移除AI服务
#[tauri::command]
pub async fn remove_ai_service(
    name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<bool, String> {
    Ok(ai_manager.remove_service(&name))
}

// 创建AI对话
#[tauri::command]
pub async fn create_ai_conversation(
    request: CreateConversationRequest,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<String, String> {
    if let Some(service) = ai_manager.get_service(&request.service_name) {
        service.create_conversation(request.title).await.map_err(|e| e.to_string())
    } else {
        // Fallback or default service creation
        if let Some(default_service) = ai_manager.get_service("default") {
            return default_service.create_conversation(request.title).await.map_err(|e| e.to_string());
        }
        Err(format!("AI service '{}' not found and no default service is available.", request.service_name))
    }
}

// 发送AI消息
#[tauri::command]
pub async fn send_ai_message(
    request: SendMessageRequest,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<String, String> {
    if let Some(service) = ai_manager.get_service(&request.service_name) {
        service.send_message(&request.message, Some(request.conversation_id)).await.map_err(|e| e.to_string())
    } else {
        Err("AI service not found".to_string())
    }
}

// 发送AI消息（流式）
#[tauri::command]
pub async fn send_ai_message_stream(
    request: SendStreamMessageRequest,
    app: tauri::AppHandle,
) -> Result<String, String> {
    // 如果提供了service_name，则使用服务管理器
    if let Some(service_name) = &request.service_name {
        if let Some(ai_manager) = app.try_state::<Arc<AiServiceManager>>() {
            if let Some(service) = ai_manager.get_service(service_name) {
                return service.send_message_stream_with_prompt(
                    &request.message,
                    request.conversation_id.clone(),
                    request.system_prompt,
                ).await.map_err(|e| e.to_string());
            }
            return Err(format!("AI service '{}' not found", service_name));
        }
        return Err("AI service manager not initialized".to_string());
    } 
    // 否则，直接使用提供的provider和model，通过genai库发送真实请求
    else if let (Some(provider), Some(model)) = (&request.provider, &request.model) {
        let conversation_id = request.conversation_id;
        let message_id = Uuid::new_v4().to_string();
        let message_id_clone = message_id.clone();
        let app_handle_clone = app.clone();
        let provider_str = provider.clone();
        let model_str = model.clone();
        let message_text = request.message.clone();
        let system_prompt_str = request.system_prompt.clone();
        let temperature = request.temperature;
        let max_tokens = request.max_tokens;
        
        // 使用tokio spawn异步处理请求
        tokio::spawn(async move {
            let db = match app_handle_clone.try_state::<Arc<DatabaseService>>() {
                Some(db) => db,
                None => {
                    tracing::error!("Database service not initialized");
                    return;
                }
            };

            // 1. 保存当前用户消息
            let user_msg = AiMessage {
                id: Uuid::new_v4().to_string(),
                conversation_id: conversation_id.clone(),
                role: "user".to_string(),
                content: message_text.clone(),
                timestamp: Utc::now(),
                metadata: None,
                token_count: None,
                cost: None,
                tool_calls: None,
                attachments: None,
            };
            if let Err(e) = db.create_ai_message(&user_msg).await {
                tracing::error!("Failed to save user message: {}", e);
            }
            
            // 2. 准备工具
            let mut tools_xml = String::new();
            let mut tools_for_genai: Vec<Tool> = Vec::new();
            let mut has_tools = false;
            if let Some(mcp_service) = app_handle_clone.try_state::<Arc<crate::services::mcp::McpService>>() {
                if let Ok(available_tools) = mcp_service.get_available_tools().await {
                    if !available_tools.is_empty() {
                        has_tools = true;
                        tools_xml.push_str("<tools>\n");
                        for tool in available_tools {
                             let schema_str = serde_json::to_string(&tool.parameters.schema).unwrap_or_else(|_| "{}".to_string());
                            tools_xml.push_str(&format!(
                                "<tool>\n  <name>{}</name>\n  <description>{}</description>\n  <arguments>{}</arguments>\n</tool>\n",
                                &tool.name, &tool.description, schema_str
                            ));
                            tools_for_genai.push(Tool {
                                name: tool.name.clone(),
                                description: Some(tool.description.clone()),
                                schema: Some(if tool.parameters.schema.is_null() {
                                    serde_json::json!({"type": "object", "properties": {}})
                                } else {
                                    tool.parameters.schema.clone()
                                }),
                                config: None,
                            });
                        }
                        tools_xml.push_str("</tools>");
                    }
                }
            }
            
            // 3. 构建系统提示
            let mut final_system_prompt = system_prompt_str.unwrap_or_default();
            if has_tools {
                let tool_instructions = format!(r#"
---
You have access to a set of tools to answer the user's question.
You use tools step-by-step to accomplish a given task, with each tool use informed by the result of the previous tool use.

## Tool Use Formatting
Tool use is formatted using XML-style tags. The tool name is enclosed in opening and closing tags, and each parameter is similarly enclosed within its own set of tags. Here's the structure:

<tool_use>
<name>{{tool_name}}</name>
<arguments>{{json_arguments}}</arguments>
</tool_use>

The tool name should be the exact name of the tool you are using, and the arguments should be a JSON object containing the parameters required by that tool.

The user will respond with the result of the tool use, which should be formatted as follows:

<tool_use_result>
<name>{{tool_name}}</name>
<result>{{result}}</result>
</tool_use_result>

## Tool Use Rules
1. Always use the right arguments for the tools. Never use variable names as the action arguments, use the value instead.
2. Call a tool only when needed.
3. If no tool call is needed, just answer the question directly.

## Available Tools
{}
"#,
                    tools_xml
                );
                final_system_prompt.push_str(&tool_instructions);
            }

            // 4. 获取历史消息并构建请求
            let mut chat_messages = Vec::new();
            if !final_system_prompt.is_empty() {
                chat_messages.push(ChatMessage::system(&final_system_prompt));
            }
            
            match db.get_ai_messages_by_conversation(&conversation_id).await {
                Ok(history) => {
                    for msg in history {
                        let role = msg.role.as_str();
                        match role {
                            "user" => chat_messages.push(ChatMessage::user(&msg.content)),
                            "assistant" => {
                                // 无论是否有工具调用，都使用普通assistant消息
                                chat_messages.push(ChatMessage::assistant(&msg.content));
                            },
                            "tool" => {
                                // 工具结果消息，使用系统消息
                                if let Some(metadata) = &msg.metadata {
                                    if let Ok(tool_metadata) = serde_json::from_str::<serde_json::Value>(metadata) {
                                        if let Some(tool_name) = tool_metadata.get("tool_name").and_then(|v| v.as_str()) {
                                            let formatted_result = format!("Tool result from {}: {}", tool_name, msg.content);
                                            chat_messages.push(ChatMessage::system(&formatted_result));
                                        }
                                    }
                                }
                            },
                            _ => {},
                        }
                    }
                },
                Err(e) => tracing::error!("Failed to get conversation history: {}", e),
            }
            // 确保最新的用户消息在最后
            if chat_messages.last().map_or(true, |m| {
                // 简单检查是否需要添加新的用户消息
                true
            }) {
                chat_messages.push(ChatMessage::user(&message_text));
            }

            // 5. 设置并执行第一次AI调用
            let mut chat_options = ChatOptions::default();
            if let Some(temp) = temperature { chat_options.temperature = Some(temp as f64); }
            if let Some(tokens) = max_tokens { chat_options.max_tokens = Some(tokens); }
            
            let mut chat_request = ChatRequest::new(chat_messages.clone());
            if has_tools {
                chat_request = chat_request.with_tools(tools_for_genai.clone());
            }

            // 修改递归调用方式，不使用tokio::spawn
            // 使用更新后的消息再次调用AI
            let new_message_id = Uuid::new_v4().to_string();
            let new_request = ChatRequest::new(chat_messages);

            // 直接调用，不使用tokio::spawn
            let db_inner = db.inner().clone();
            run_ai_interaction_cycle(
                app, 
                db_inner, 
                conversation_id, 
                new_message_id, 
                provider_str, 
                model_str, 
                new_request, 
                chat_options
            ).await;
        });
        
        return Ok(message_id);
    } else {
        return Err("Missing required parameters: provider and model".to_string());
    }
}

/// 运行一个完整的AI交互周期，包括可能的工具调用
fn run_ai_interaction_cycle(
    app: AppHandle,
    db: Arc<DatabaseService>,
    conversation_id: String,
    message_id: String,
    provider: String,
    model: String,
    chat_request: ChatRequest,
    chat_options: ChatOptions,
) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
    Box::pin(async move {
        // 创建genai客户端
        let genai_client = GenaiClient::default();
        // ... 此处省略了原有的设置API Key和Base URL的代码，实际应保留 ...

        tracing::info!("Executing chat request: {:?}", chat_request);
        match genai_client.exec_chat_stream(&model, chat_request.clone(), Some(&chat_options)).await {
            Ok(mut stream) => {
                let mut content = String::new();
                let tool_calls: Vec<genai::chat::ToolCall> = Vec::new();

                // 处理流式响应
                while let Some(event) = stream.stream.next().await {
                    match event {
                        Ok(ChatStreamEvent::Chunk(chunk)) => {
                            content.push_str(&chunk.content);
                            let _ = app.emit("ai_stream_message", serde_json::json!({
                                "conversation_id": &conversation_id, "message_id": &message_id,
                                "content": &content, "is_complete": false,
                            }));
                        },
                        Err(e) => {
                            tracing::error!("Stream error: {}", e);
                            // ... 发送错误到前端 ...
                            return;
                        },
                        _ => {}
                    }
                }

                // 流结束，判断是否需要工具调用
                if !tool_calls.is_empty() {
                    tracing::info!("AI requested {} tool calls", tool_calls.len());
                    // 保存助手的工具调用消息
                    let assistant_msg = AiMessage {
                        id: message_id.clone(),
                        conversation_id: conversation_id.clone(),
                        role: "assistant".to_string(),
                        content: content.clone(),
                        tool_calls: Some(serde_json::to_string(&tool_calls).unwrap_or_default()),
                        timestamp: Utc::now(),
                        metadata: None,
                        token_count: None,
                        cost: None,
                        attachments: None,
                    };
                    if let Err(e) = db.create_ai_message(&assistant_msg).await {
                        tracing::error!("Failed to save assistant tool call message: {}", e);
                    }

                    // 更新聊天记录
                    let mut current_messages = chat_request.messages;
                    current_messages.push(ChatMessage::assistant(&content));

                    // 执行工具
                    if let Some(mcp_service) = app.try_state::<Arc<crate::services::mcp::McpService>>() {
                        for tool_call in tool_calls {
                            let args_str = tool_call.fn_arguments.to_string();
                            let args: serde_json::Value = serde_json::from_str(&args_str).unwrap_or_default();
                            let result = mcp_service.execute_tool(&tool_call.fn_name, args).await;
                            let result_str = match result {
                                Ok(res) => serde_json::to_string(&res).unwrap_or_default(),
                                Err(e) => format!("Error: {}", e),
                            };

                            // 保存工具结果消息
                            let tool_msg = AiMessage {
                                id: Uuid::new_v4().to_string(),
                                conversation_id: conversation_id.clone(),
                                role: "tool".to_string(),
                                content: result_str.clone(),
                                metadata: Some(serde_json::json!({"tool_name": &tool_call.fn_name}).to_string()),
                                timestamp: Utc::now(),
                                token_count: None,
                                cost: None,
                                tool_calls: None,
                                attachments: None,
                            };
                            if let Err(e) = db.create_ai_message(&tool_msg).await {
                                tracing::error!("Failed to save tool result message: {}", e);
                            }

                            let formatted_result = format!("Tool result from {}: {}", tool_call.fn_name, result_str);
                            current_messages.push(ChatMessage::system(&formatted_result));
                        }
                    }
                    
                    // 使用更新后的消息再次调用AI
                    let new_message_id = Uuid::new_v4().to_string();
                    let new_request = ChatRequest::new(current_messages);
                    
                    // 递归调用
                    run_ai_interaction_cycle(
                        app, 
                        db, 
                        conversation_id, 
                        new_message_id, 
                        provider, 
                        model, 
                        new_request, 
                        chat_options
                    ).await;

                } else {
                    // 没有工具调用，是最终回复
                    let final_msg = AiMessage {
                        id: message_id.clone(),
                        conversation_id: conversation_id.clone(),
                        role: "assistant".to_string(),
                        content: content.clone(),
                        timestamp: Utc::now(),
                        token_count: Some(content.split_whitespace().count() as i32),
                        metadata: None,
                        cost: None,
                        tool_calls: None,
                        attachments: None,
                    };
                    if let Err(e) = db.create_ai_message(&final_msg).await {
                        tracing::error!("Failed to save final assistant message: {}", e);
                    }
                    let _ = app.emit("ai_stream_message", serde_json::json!({
                        "conversation_id": &conversation_id, "message_id": &message_id,
                        "content": &content, "is_complete": true,
                    }));
                }
            },
            Err(e) => {
                tracing::error!("AI request failed: {}", e);
                // ... 发送错误到前端 ...
            }
        }
    })
}

// 执行AI工具调用
#[tauri::command]
pub async fn execute_ai_tool_call(
    request: ExecuteToolCallRequest,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Value, String> {
    if let Some(service) = ai_manager.get_service(&request.service_name) {
        return service
            .execute_tool_call(
                &request.conversation_id,
                &request.tool_call.name,
                request.tool_call.arguments,
            )
            .await
            .map_err(|e| e.to_string());
    }
    Err("AI service not found".to_string())
}

// 删除AI对话
#[tauri::command]
pub async fn delete_ai_conversation(
    conversation_id: String,
    service_name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<(), String> {
    if let Some(service) = ai_manager.get_service(&service_name) {
        return service.delete_conversation(&conversation_id).await.map_err(|e| e.to_string());
    }
    Err(format!("AI service '{}' not found for deleting conversation.", service_name))
}

// 更新对话标题
#[tauri::command]
pub async fn update_ai_conversation_title(
    conversation_id: String,
    title: String,
    service_name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<(), String> {
    if let Some(service) = ai_manager.get_service(&service_name) {
        return service.update_conversation_title(&conversation_id, &title).await.map_err(|e| e.to_string());
    }
    Err("AI service not found".to_string())
}

// 归档对话
#[tauri::command]
pub async fn archive_ai_conversation(
    conversation_id: String,
    service_name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<(), String> {
    if let Some(service) = ai_manager.get_service(&service_name) {
        return service.archive_conversation(&conversation_id).await.map_err(|e| e.to_string());
    }
    Err("AI service not found".to_string())
}

// 获取AI服务信息
#[tauri::command]
pub async fn get_ai_service_info(
    service_name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Option<AiServiceInfo>, String> {
    if let Some(service) = ai_manager.get_service(&service_name) {
        let config = service.get_config();
        let info = AiServiceInfo {
            name: service_name,
            provider: config.provider.clone(),
            model: config.model.clone(),
        };
        return Ok(Some(info));
    }
    Ok(None)
}

// 配置AI服务
#[tauri::command]
pub async fn configure_ai_service(
    service_name: String,
    mut config: AiConfig,
    ai_manager: State<'_, Arc<AiServiceManager>>,
    app: AppHandle,
) -> Result<(), String> {
    // 如果配置中没有API密钥，尝试从数据库获取
    if config.api_key.is_none() {
        if let Some(database) = app.try_state::<Arc<crate::services::database::DatabaseService>>() {
            let api_key_name = format!("api_key_{}", config.provider.to_lowercase());
            match database.get_config("ai", &api_key_name).await {
                Ok(Some(api_key)) => {
                    config.api_key = Some(api_key);
                    tracing::info!("Loading API key for {} from database", config.provider);
                },
                Ok(None) => {
                    tracing::warn!("API key for {} not found in database", config.provider);
                },
                Err(e) => {
                    tracing::error!("Failed to load API key for {} from database: {}", config.provider, e);
                }
            }
        }
    }
    
    // 如果有API密钥，设置为环境变量
    if let Some(api_key) = &config.api_key {
        let env_var_name = match config.provider.as_str() {
            "openai" => "OPENAI_API_KEY",
            "anthropic" => "ANTHROPIC_API_KEY",
            "gemini" => "GEMINI_API_KEY",
            "deepseek" => "DEEPSEEK_API_KEY",
            "groq" => "GROQ_API_KEY",
            "cohere" => "COHERE_API_KEY",
            "xai" => "XAI_API_KEY",
            _ => {
                tracing::warn!("Unknown AI provider: {}", config.provider);
                return Err(format!("Unsupported AI provider: {}", config.provider));
            }
        };
        std::env::set_var(env_var_name, api_key);
        tracing::info!("Set API key environment variable for {}: {}", config.provider, env_var_name);
    } else {
        tracing::warn!("No API key found for {}, requests may fail", config.provider);
    }
    
    // 先移除旧配置
    ai_manager.remove_service(&service_name);
    // 添加新配置
    ai_manager.add_service(service_name, config).await.map_err(|e| e.to_string())
}

// 获取AI对话列表
#[tauri::command]
pub async fn get_ai_conversations(
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Vec<AiConversation>, String> {
    // 获取默认服务的对话列表
    let services = ai_manager.list_services();
    if let Some(service_name) = services.first() {
        if let Some(service) = ai_manager.get_service(service_name) {
            return service.list_conversations().await.map_err(|e| e.to_string());
        }
    }
    Ok(vec![])
}

// 获取对话历史
#[tauri::command]
pub async fn get_ai_conversation_history(
    conversation_id: String,
    service_name: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Vec<AiMessage>, String> {
    if let Some(service) = ai_manager.get_service(&service_name) {
        return service.get_conversation_history(&conversation_id).await.map_err(|e| e.to_string());
    }
    Err(format!("AI service '{}' not found for getting conversation history.", service_name))
}

// 获取所有可用的AI模型
#[tauri::command]
pub async fn get_available_ai_models() -> Result<Vec<AiModelInfo>, String> {
    let mut models = Vec::new();
    
    // 基于环境变量返回默认模型列表
    // 这个函数主要用于初始化，实际的模型列表由前端通过update_ai_models命令更新
    
    // OpenAI模型
    if std::env::var("OPENAI_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "openai".to_string(),
            models: vec![
                "gpt-4o".to_string(),
                "gpt-4o-mini".to_string(),
                "gpt-4-turbo".to_string(),
                "gpt-4".to_string(),
                "gpt-3.5-turbo".to_string(),
            ],
        });
    }
    
    // Anthropic模型
    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "anthropic".to_string(),
            models: vec![
                "claude-3-opus-20240229".to_string(),
                "claude-3-sonnet-20240229".to_string(),
                "claude-3-haiku-20240307".to_string(),
                "claude-2.1".to_string(),
                "claude-2.0".to_string(),
                "claude-instant-1.2".to_string(),
            ],
        });
    }
    
    // Gemini模型
    if std::env::var("GEMINI_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "gemini".to_string(),
            models: vec![
                "gemini-2.0-pro".to_string(),
                "gemini-2.0-flash".to_string(),
                "gemini-1.5-pro".to_string(),
                "gemini-1.5-flash".to_string(),
                "gemini-1.0-pro".to_string(),
            ],
        });
    }
    
    // DeepSeek模型
    if std::env::var("DEEPSEEK_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "deepseek".to_string(),
            models: vec![
                "deepseek-coder".to_string(),
                "deepseek-chat".to_string(),
                "deepseek-llm-7b".to_string(),
                "deepseek-llm-67b".to_string(),
            ],
        });
    }
    
    // xAI模型
    if std::env::var("XAI_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "xai".to_string(),
            models: vec![
                "grok-1".to_string(),
            ],
        });
    }
    
    // Groq模型
    if std::env::var("GROQ_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "groq".to_string(),
            models: vec![
                "llama3-8b-8192".to_string(),
                "llama3-70b-8192".to_string(),
                "mixtral-8x7b-32768".to_string(),
            ],
        });
    }
    
    // Cohere模型
    if std::env::var("COHERE_API_KEY").is_ok() {
        models.push(AiModelInfo {
            provider: "cohere".to_string(),
            models: vec![
                "command-r".to_string(),
                "command-r-plus".to_string(),
                "command-light".to_string(),
            ],
        });
    }
    
    // Ollama模型（本地模型，不需要API密钥）
    models.push(AiModelInfo {
        provider: "ollama".to_string(),
        models: vec![
            "llama3.2:3b".to_string(),
            "llama3.2:8b".to_string(),
            "llama3.2:70b".to_string(),
            "llama3:8b".to_string(),
            "llama3:70b".to_string(),
            "mistral:7b".to_string(),
            "mixtral:8x7b".to_string(),
            "codellama:7b".to_string(),
            "codellama:13b".to_string(),
            "codellama:34b".to_string(),
            "phi3:mini".to_string(),
            "phi3:medium".to_string(),
            "phi3:small".to_string(),
        ],
    });
    
    Ok(models)
}

// 更新AI模型列表（由前端调用，传递实际测试获取的模型）
#[tauri::command]
pub async fn update_ai_models(
    models: Vec<AiModelInfo>,
    app: AppHandle,
) -> Result<(), String> {
    tracing::info!("Updating AI model list: {:?}", models);
    
    // 将模型列表保存到应用状态中
    if let Some(ai_manager) = app.try_state::<Arc<AiServiceManager>>() {
        // 这里可以将模型信息保存到AI服务管理器中
        // 目前先记录日志
        for model_info in &models {
            tracing::info!("Provider {}: {} models", model_info.provider, model_info.models.len());
        }
    }
    
    Ok(())
}

// 测试AI提供商连接
#[tauri::command]
pub async fn test_ai_connection(
    request: TestConnectionRequest,
) -> Result<TestConnectionResponse, String> {
    // 根据提供商类型选择不同的测试方法
    match request.provider.to_lowercase().as_str() {
        "openai" => test_openai_connection(request).await,
        "anthropic" => test_anthropic_connection(request).await,
        "gemini" => test_gemini_connection(request).await,
        "deepseek" => test_deepseek_connection(request).await,
        "ollama" => test_ollama_connection(request).await,
        _ => Ok(TestConnectionResponse {
            success: false,
            message: format!("Unsupported AI provider: {}", request.provider),
            models: None,
        }),
    }
}

// 测试OpenAI连接
async fn test_openai_connection(request: TestConnectionRequest) -> Result<TestConnectionResponse, String> {
    if request.api_key.is_none() {
        return Ok(TestConnectionResponse {
            success: false,
            message: "OpenAI API key cannot be empty".to_string(),
            models: None,
        });
    }

    let client = reqwest::Client::new();
    let api_base = request.api_base.unwrap_or_else(|| "https://api.openai.com/v1".to_string());
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization", 
        format!("Bearer {}", request.api_key.unwrap()).parse().map_err(|e| format!("无效的API密钥: {}", e))?
    );
    
    if let Some(org) = &request.organization {
        if !org.is_empty() {
            headers.insert(
                "OpenAI-Organization", 
                org.parse().map_err(|e| format!("无效的组织ID: {}", e))?
            );
        }
    }
    
    // 测试连接 - 获取模型列表
    let response = client.get(format!("{}/models", api_base))
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to OpenAI: {}", e))?;
    
    if response.status().is_success() {
        let models_response: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // 提取模型ID列表
        if let Some(models_array) = models_response.get("data").and_then(|d| d.as_array()) {
            let models: Vec<String> = models_array
                .iter()
                .filter_map(|m| m.get("id").and_then(|id| id.as_str()).map(String::from))
                .collect();
            
            Ok(TestConnectionResponse {
                success: true,
                message: format!("Successfully connected to OpenAI, found {} models", models.len()),
                models: Some(models),
            })
        } else {
            Ok(TestConnectionResponse {
                success: true,
                message: "Successfully connected to OpenAI, but failed to get model list".to_string(),
                models: None,
            })
        }
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Ok(TestConnectionResponse {
            success: false,
            message: format!("Failed to connect to OpenAI: {}", error_text),
            models: None,
        })
    }
}

// 测试Anthropic连接
async fn test_anthropic_connection(request: TestConnectionRequest) -> Result<TestConnectionResponse, String> {
    if request.api_key.is_none() {
        return Ok(TestConnectionResponse {
            success: false,
            message: "Anthropic API key cannot be empty".to_string(),
            models: None,
        });
    }

    let client = reqwest::Client::new();
    let api_base = request.api_base.unwrap_or_else(|| "https://api.anthropic.com".to_string());
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "x-api-key", 
        request.api_key.unwrap().parse().map_err(|e| format!("无效的API密钥: {}", e))?
    );
    headers.insert(
        "anthropic-version", 
        "2023-06-01".parse().unwrap()
    );
    
    // Anthropic没有列出模型的公开API，我们只能测试连接
    // 使用简单的消息请求来测试连接
    let test_payload = serde_json::json!({
        "model": "claude-3-haiku-20240307",
        "max_tokens": 1,
        "messages": [
            {
                "role": "user",
                "content": "Hello"
            }
        ]
    });
    
    let response = client.post(format!("{}/v1/messages", api_base))
        .headers(headers)
        .json(&test_payload)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Anthropic: {}", e))?;
    
    if response.status().is_success() {
        // 预定义Anthropic模型列表
        let models = vec![
            "claude-3-opus-20240229".to_string(),
            "claude-3-sonnet-20240229".to_string(),
            "claude-3-haiku-20240307".to_string(),
            "claude-2.1".to_string(),
            "claude-2.0".to_string(),
            "claude-instant-1.2".to_string(),
        ];
        
        Ok(TestConnectionResponse {
            success: true,
            message: "Successfully connected to Anthropic Claude API".to_string(),
            models: Some(models),
        })
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        Ok(TestConnectionResponse {
            success: false,
            message: format!("Failed to connect to Anthropic: {}", error_text),
            models: None,
        })
    }
}

// 测试Gemini连接
async fn test_gemini_connection(request: TestConnectionRequest) -> Result<TestConnectionResponse, String> {
    if request.api_key.is_none() {
        return Ok(TestConnectionResponse {
            success: false,
            message: "Gemini API key cannot be empty".to_string(),
            models: None,
        });
    }

    let client = reqwest::Client::new();
    let api_key = request.api_key.unwrap();
    
    // 使用Gemini API测试连接
    let response = client.get(format!("https://generativelanguage.googleapis.com/v1/models?key={}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Gemini: {}", e))?;
    
    if response.status().is_success() {
        let models_response: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // 提取模型列表
        if let Some(models_array) = models_response.get("models").and_then(|m| m.as_array()) {
            let models: Vec<String> = models_array
                .iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(|s| s.replace("models/", "")))
                .collect();
            
            Ok(TestConnectionResponse {
                success: true,
                message: format!("Successfully connected to Gemini, found {} models", models.len()),
                models: Some(models),
            })
        } else {
            Ok(TestConnectionResponse {
                success: true,
                message: "Successfully connected to Gemini, but failed to get model list".to_string(),
                models: None,
            })
        }
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        Ok(TestConnectionResponse {
            success: false,
            message: format!("Failed to connect to Gemini: {}", error_text),
            models: None,
        })
    }
}

// 测试DeepSeek连接
async fn test_deepseek_connection(request: TestConnectionRequest) -> Result<TestConnectionResponse, String> {
    if request.api_key.is_none() {
        return Ok(TestConnectionResponse {
            success: false,
            message: "DeepSeek API key cannot be empty".to_string(),
            models: None,
        });
    }

    let client = reqwest::Client::new();
    let api_base = request.api_base.unwrap_or_else(|| "https://api.deepseek.com/v1".to_string());
    
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Authorization", 
        format!("Bearer {}", request.api_key.unwrap()).parse().map_err(|e| format!("无效的API密钥: {}", e))?
    );
    
    // 测试连接 - 获取模型列表
    let response = client.get(format!("{}/models", api_base))
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to connect to DeepSeek: {}", e))?;
    
    if response.status().is_success() {
        let models_response: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // 提取模型ID列表
        if let Some(models_array) = models_response.get("data").and_then(|d| d.as_array()) {
            let models: Vec<String> = models_array
                .iter()
                .filter_map(|m| m.get("id").and_then(|id| id.as_str()).map(String::from))
                .collect();
            
            Ok(TestConnectionResponse {
                success: true,
                message: format!("Successfully connected to DeepSeek, found {} models", models.len()),
                models: Some(models),
            })
        } else {
            // 如果无法解析模型列表，使用默认模型
            Ok(TestConnectionResponse {
                success: true,
                message: "Successfully connected to DeepSeek, using default model list".to_string(),
                models: Some(vec![
                    "deepseek-coder".to_string(),
                    "deepseek-chat".to_string(),
                    "deepseek-llm-7b".to_string(),
                    "deepseek-llm-67b".to_string(),
                ]),
            })
        }
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        Ok(TestConnectionResponse {
            success: false,
            message: format!("Failed to connect to DeepSeek: {}", error_text),
            models: None,
        })
    }
}

// 测试Ollama连接(本地)
async fn test_ollama_connection(request: TestConnectionRequest) -> Result<TestConnectionResponse, String> {
    let client = reqwest::Client::new();
    let api_base = request.api_base.unwrap_or_else(|| "http://localhost:11434".to_string());
    
    // 获取可用模型列表
    let response = client.get(format!("{}/api/tags", api_base))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;
    
    if response.status().is_success() {
        let models_response: serde_json::Value = response.json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        // 提取模型列表
        if let Some(models_array) = models_response.get("models").and_then(|m| m.as_array()) {
            let models: Vec<String> = models_array
                .iter()
                .filter_map(|m| m.get("name").and_then(|n| n.as_str()).map(String::from))
                .collect();
            
            Ok(TestConnectionResponse {
                success: true,
                message: format!("Successfully connected to Ollama, found {} local models", models.len()),
                models: Some(models),
            })
        } else {
            Ok(TestConnectionResponse {
                success: true,
                message: "Successfully connected to Ollama, but failed to get model list".to_string(),
                models: None,
            })
        }
    } else {
        let error_text = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        Ok(TestConnectionResponse {
            success: false,
            message: format!("Failed to connect to Ollama: {}", error_text),
            models: None,
        })
    }
}

// 保存AI配置
#[tauri::command]
pub async fn save_ai_config(
    config: SaveAiConfigRequest,
    db: State<'_, Arc<DatabaseService>>,
    ai_manager: State<'_, Arc<AiServiceManager>>,
    app: AppHandle,
) -> Result<(), String> {
    tracing::info!("Starting to save AI configuration...");
    
    // 直接使用注入的数据库服务
    let db_service = db.inner().clone();
    
    // 保存providers配置为JSON
    let config_str = serde_json::to_string(&config.providers)
        .map_err(|e| format!("Failed to serialize providers config: {}", e))?;
    
    tracing::info!("Saving providers config: {}", config_str);
    
    db_service.set_config("ai", "providers_config", &config_str, Some("AI providers configuration"))
        .await
        .map_err(|e| format!("Failed to save providers config to DB: {}", e))?;
    
    // 分别保存每个提供商的API密钥
    for (_id, provider) in &config.providers {
        if provider.enabled {
            if let Some(api_key) = &provider.api_key {
                if !api_key.is_empty() {
                    let key_name = format!("api_key_{}", provider.provider.to_lowercase());
                    let description = format!("{} API密钥", provider.provider);
                    if let Err(e) = db_service.set_config("ai", &key_name, api_key, Some(&description)).await {
                        tracing::error!("Failed to save API key for {}: {}", provider.provider, e);
                    } else {
                        tracing::info!("Saved API key for {}", provider.provider);
                    }
                }
            }
        }
    }
    
    tracing::info!("AI configuration saved successfully");
    
    // 触发AI服务重新加载
    if let Err(e) = reload_ai_services(app.clone()).await {
        tracing::warn!("Failed to reload AI services: {}", e);
        // 不返回错误，因为配置已保存成功
    }
    
    // 验证配置是否正确加载
    if let Some(ai_manager) = app.try_state::<Arc<AiServiceManager>>() {
        match ai_manager.get_chat_models().await {
            Ok(models) => {
                if models.is_empty() {
                    tracing::warn!("No chat models found after reloading AI services");
                } else {
                    tracing::info!("Successfully loaded {} chat models", models.len());
                    for model in &models {
                        tracing::info!("  - {}/{}", model.provider, model.name);
                    }
                }
            },
            Err(e) => {
                tracing::error!("Failed to get chat models after reloading AI services: {}", e);
            }
        }
    }
    
    Ok(())
}

// 获取AI使用统计
#[tauri::command]
pub async fn get_ai_usage_stats(
    _ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<serde_json::Value, String> {
    // 这里应该从数据库获取使用统计数据
    // 目前返回模拟数据
    let stats = serde_json::json!({
        "openai": {
            "input_tokens": 1250,
            "output_tokens": 890,
            "total_tokens": 2140,
            "cost": 0.0125
        },
        "anthropic": {
            "input_tokens": 850,
            "output_tokens": 650,
            "total_tokens": 1500,
            "cost": 0.0088
        }
    });
    
    Ok(stats)
}

// 重新加载AI服务
#[tauri::command]
pub async fn reload_ai_services(
    app: AppHandle,
) -> Result<(), String> {
    tracing::info!("Starting to reload AI services...");
    
    match app.try_state::<Arc<AiServiceManager>>() {
        Some(ai_manager) => {
            // 使用新的reload_services方法
            match ai_manager.reload_services().await {
                Ok(_) => {
                    tracing::info!("AI services reloaded successfully");
                    // 验证模型是否正确加载
                    match ai_manager.get_chat_models().await {
                        Ok(models) => {
                            if models.is_empty() {
                                tracing::warn!("No chat models found after reloading AI services");
                            } else {
                                tracing::info!("Successfully loaded {} chat models", models.len());
                                for model in &models {
                                    tracing::info!("  - {}/{}", model.provider, model.name);
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to get chat models after reloading AI services: {}", e);
                        }
                    }
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("Failed to reload AI services: {}", e);
                    tracing::error!("{}", error_msg);
                    Err(error_msg)
                }
            }
        },
        None => {
            tracing::warn!("AI service manager not found, trying to recreate...");
            
            let database = match app.try_state::<Arc<crate::services::database::DatabaseService>>() {
                Some(db) => db.inner().clone(),
                None => {
                    let error_msg = "Database service not initialized, cannot reload AI services.".to_string();
                    tracing::error!("{}", error_msg);
                    return Err(error_msg);
                }
            };
            
            // 创建新的AI服务管理器
            let mut new_ai_manager = AiServiceManager::new(database);
            
            // 设置MCP服务（如果可用）
            if let Some(mcp_service) = app.try_state::<Arc<crate::services::McpService>>() {
                new_ai_manager.set_mcp_service(mcp_service.inner().clone());
            }
            
            // 设置应用句柄
            new_ai_manager.set_app_handle(app.clone());
            
            // 初始化默认服务
            match new_ai_manager.init_default_services().await {
                Ok(_) => {
                    // 更新应用状态
                    app.manage(Arc::new(new_ai_manager.clone()));
                    tracing::info!("AI service manager recreated and initialized successfully");
                    
                    // 验证模型是否正确加载
                    match new_ai_manager.get_chat_models().await {
                        Ok(models) => {
                            if models.is_empty() {
                                tracing::warn!("No chat models found after recreating AI service manager");
                            } else {
                                tracing::info!("Successfully loaded {} chat models", models.len());
                                for model in &models {
                                    tracing::info!("  - {}/{}", model.provider, model.name);
                                }
                            }
                        },
                        Err(e) => {
                            tracing::error!("Failed to get chat models after recreating AI service manager: {}", e);
                        }
                    }
                    
                    Ok(())
                },
                Err(e) => {
                    let error_msg = format!("Failed to recreate AI service manager: {}", e);
                    tracing::error!("{}", error_msg);
                    Err(error_msg)
                }
            }
        }
    }
}

// 获取AI服务状态
#[tauri::command]
pub async fn get_ai_service_status() -> Result<Vec<AiServiceStatusResponse>, String> {
    // 示例实现
    Ok(vec![])
}

#[tauri::command]
pub async fn get_ai_chat_models(
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Vec<ModelInfo>, String> {
    tracing::info!("Getting available AI chat models...");
    
    match ai_manager.get_chat_models().await {
        Ok(models) => {
            if models.is_empty() {
                tracing::warn!("No chat models found");
            } else {
                tracing::info!("Found {} chat models", models.len());
                for model in &models {
                    tracing::info!("  - {}/{}", model.provider, model.name);
                }
            }
            Ok(models)
        },
        Err(e) => {
            let error_msg = format!("Failed to get chat models: {}", e);
            tracing::error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[tauri::command]
pub async fn get_ai_embedding_models(
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Vec<ModelInfo>, String> {
    ai_manager.get_embedding_models().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_default_ai_model(
    model_type: String,
    ai_manager: State<'_, Arc<AiServiceManager>>,
) -> Result<Option<ModelInfo>, String> {
    tracing::info!("Getting default AI model for type: {}", model_type);
    
    // 尝试从AI服务管理器获取默认模型
    match ai_manager.get_default_model(&model_type).await {
        Ok(Some(model)) => {
            tracing::info!("Found default model: {}/{}", model.provider, model.name);
            return Ok(Some(model));
        },
        Ok(None) => {
            tracing::info!("No default model found, trying to find first available model");
            
            // 如果没有设置默认模型，尝试获取第一个可用的模型
            match ai_manager.get_chat_models().await {
                Ok(models) if !models.is_empty() => {
                    let first_model = &models[0];
                    tracing::info!("Using first available model as default: {}/{}", first_model.provider, first_model.name);
                    return Ok(Some(first_model.clone()));
                },
                Ok(_) => {
                    tracing::warn!("No models available to use as default");
                    return Ok(None);
                },
                Err(e) => {
                    let error_msg = format!("Failed to get models: {}", e);
                    tracing::error!("{}", error_msg);
                    return Err(error_msg);
                }
            }
        },
        Err(e) => {
            let error_msg = format!("Failed to get default model: {}", e);
            tracing::error!("{}", error_msg);
            return Err(error_msg);
        }
    }
}

#[tauri::command]
pub async fn set_default_ai_model(
    ai_manager: State<'_, Arc<AiServiceManager>>,
    model_type: String,
    provider: String,
    model_name: String,
) -> Result<(), String> {
    ai_manager.set_default_model(&model_type, &provider, &model_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ai_model_config(
    ai_manager: State<'_, Arc<AiServiceManager>>,
    provider: String,
    model_name: String,
) -> Result<Option<ModelConfig>, String> {
    ai_manager.get_model_config(&provider, &model_name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_ai_model_config(
    ai_manager: State<'_, Arc<AiServiceManager>>,
    config: ModelConfig,
) -> Result<(), String> {
    ai_manager.update_model_config(config).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_ai_providers_config(
    config: String,
    app: AppHandle,
) -> Result<(), String> {
    let db = app.try_state::<Arc<dyn Database + Send + Sync>>()
        .ok_or_else(|| "Database service not initialized".to_string())?;

    // 这里 config 已经是 JSON string，直接保存
    db.set_config("ai", "providers_config", &config, Some("AI providers configuration"))
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(())
} 