pub mod conversations;
pub mod models;
pub mod doubao;
pub mod grok;
pub mod roles;

use base64::engine::general_purpose;
use base64::Engine;
use futures_util::stream::StreamExt;
use genai::chat::ChatMessage;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{async_runtime::JoinHandle, Emitter, Manager};

// 从models重新导出所有API函数
pub use models::*;

// 从conversations重新导出所有API函数
pub use conversations::*;

// 从roles重新导出所有API函数
pub use roles::*;

use crate::api::ai::doubao::{doubao_stream_chat_with_history, stream_from_doubao};
use crate::api::ai::grok::{send_to_grok, stream_from_grok, grok_stream_chat_with_history};

// 全局存储流式输出任务
lazy_static::lazy_static! {
    static ref STREAMING_TASKS: Arc<Mutex<HashMap<String, JoinHandle<()>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// 将前端消息格式转换为GenAI库的ChatMessage格式
fn convert_to_chat_message(message: &serde_json::Value) -> Option<ChatMessage> {
    let role = message.get("role")?.as_str()?;
    let content = message.get("content")?.as_str()?;

    match role {
        "user" => Some(ChatMessage::user(content)),
        "assistant" => Some(ChatMessage::assistant(content)),
        "system" => Some(ChatMessage::system(content)),
        _ => None,
    }
}

// 保存API密钥
#[tauri::command]
pub async fn save_api_key(
    app: tauri::AppHandle,
    model_id: String,
    api_key: String,
) -> Result<(), String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 简单加密API密钥
    let encoded_key = general_purpose::STANDARD.encode(&api_key);
    
    // 使用模型ID作为键名保存API密钥
    let key = format!("api_key_{}", model_id);
    db.save_setting(&key, &encoded_key).map_err(|e| e.to_string())?;

    Ok(())
}

// 获取API密钥
#[tauri::command]
pub async fn get_api_key(app: tauri::AppHandle, model_id: String) -> Result<String, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名获取API密钥
    let key = format!("api_key_{}", model_id);
    match db.get_setting(&key).map_err(|e| e.to_string())? {
        Some(encoded_key) => {
            // 解码API密钥
            let api_key = general_purpose::STANDARD
                .decode(encoded_key)
                .map_err(|e| e.to_string())?;
            let api_key = String::from_utf8(api_key).map_err(|e| e.to_string())?;
            Ok(api_key)
        },
        None => Ok("".to_string()),
    }
}

// 检查是否有特定模型的API密钥
#[tauri::command]
pub async fn has_api_key(app: tauri::AppHandle, model_id: String) -> Result<bool, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名检查API密钥是否存在
    let key = format!("api_key_{}", model_id);
    match db.get_setting(&key).map_err(|e| e.to_string())? {
        Some(value) => Ok(!value.is_empty()),
        None => Ok(false),
    }
}

// 获取所有AI API密钥
#[tauri::command]
pub async fn get_ai_api_keys(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 由于没有直接的方法获取所有以api_key_开头的设置，
    // 我们检查常用的模型ID列表
    let known_models = vec![
        "openai", "claude", "gemini", "qwen", "doubao", "grok", 
        "deepseek", "moonshot", "custom"
    ];
    
    let mut models_with_keys = Vec::new();
    
    for model_id in known_models {
        let key = format!("api_key_{}", model_id);
        if let Ok(Some(value)) = db.get_setting(&key) {
            if !value.is_empty() {
                models_with_keys.push(model_id.to_string());
            }
        }
    }

    Ok(models_with_keys)
}

// 保存自定义API端点
#[tauri::command]
pub async fn save_api_endpoint(app: tauri::AppHandle, endpoint: String) -> Result<(), String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 保存自定义API端点配置
    db.save_setting("custom_api_endpoint", &endpoint).map_err(|e| e.to_string())?;

    Ok(())
}

// 获取自定义API端点
#[tauri::command]
pub async fn get_api_endpoint(app: tauri::AppHandle) -> Result<String, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 获取自定义API端点配置
    match db.get_setting("custom_api_endpoint").map_err(|e| e.to_string())? {
        Some(endpoint) => Ok(endpoint),
        None => Ok("".to_string()),
    }
}

// 发送消息到AI
#[tauri::command]
pub async fn send_ai_message(
    app: tauri::AppHandle,
    model_id: String,
    message: String,
    role_id: Option<String>,
) -> Result<String, String> {
    // 获取API密钥
    let api_key = get_api_key(app.clone(), model_id.clone()).await?;

    if api_key.is_empty() {
        return Err("未配置API密钥".to_string());
    }

    // 处理角色信息，构建最终消息
    let final_message = if let Some(role_id) = role_id {
        // 获取角色信息
        if let Ok(role) = get_ai_role(role_id).await {
            // 将角色描述作为系统提示词添加到消息前面
            format!("{}\\n\\n{}", role.description, message)
        } else {
            message
        }
    } else {
        message
    };

    // 根据不同的模型调用不同的API
    match model_id.as_str() {
        "qwen" => send_to_qwen(api_key, final_message).await, // 阿里千问暂时没有GenAI支持
        "doubao" => send_to_doubao(api_key, final_message, None).await, // 豆包模型
        "grok" => send_to_grok(api_key, final_message, None).await, // Grok模型
        "custom" => {
            let endpoint = get_api_endpoint(app).await?;
            if endpoint.is_empty() {
                return Err("未配置自定义API端点".to_string());
            }
            send_to_custom(api_key, endpoint, final_message).await
        }
        // 使用统一的GenAI处理方式
        _ => send_message_to_ai(api_key, &model_id, final_message).await,
    }
}

// 使用流式输出方式发送消息到AI
#[tauri::command]
pub async fn send_ai_message_stream(
    app: tauri::AppHandle,
    model_id: String,
    message: String,
    stream_id: String,
    messages: Option<Vec<serde_json::Value>>,
    custom_model_name: Option<String>,
    max_tokens: Option<i32>,
    role_id: Option<String>,
) -> Result<(), String> {
    // 记录日志用于调试
    println!(
        "接收到流式请求: model={}, message_len={}, stream_id={}, role_id={:?}",
        model_id,
        message.len(),
        stream_id,
        role_id
    );

    // 处理角色信息
    let mut final_messages = messages.unwrap_or_default();
    if let Some(role_id) = role_id {
        // 获取角色信息
        if let Ok(role) = get_ai_role(role_id).await {
            // 在消息历史开头添加系统消息（角色描述）
            let system_message = serde_json::json!({
                "role": "system",
                "content": role.description
            });
            final_messages.insert(0, system_message);
            println!("已添加角色系统提示: {}", role.name);
        }
    }

    if !final_messages.is_empty() {
        println!("接收到消息历史: {} 条", final_messages.len());
    } else {
        println!("没有接收到消息历史");
    }

    // println!("final_messages: {:?}", final_messages);

    // 检查是否有自定义模型名称
    let model_name_from_param = custom_model_name.clone();

    // 如果没有传递自定义模型名称，尝试从配置中读取
    let model_name_from_config = if model_name_from_param.is_none() {
        get_model_name_config(app.clone(), model_id.clone())
            .await
            .ok()
    } else {
        None
    };

    // 获取API密钥
    let api_key = get_api_key(app.clone(), model_id.clone()).await?;

    if api_key.is_empty() {
        return Err("未配置API密钥".to_string());
    }

    // 克隆final_messages用于传递到异步任务中
    let final_messages_for_doubao = final_messages.clone();
    let final_messages_for_grok = final_messages.clone();
    let final_messages_for_genai = final_messages.clone();

    // 创建流式处理任务
    let stream_task = match model_id.as_str() {
        "qwen" => {
            // 千问暂时不支持流式输出，保留单独实现的可能性
            return Err("暂不支持千问的流式输出".to_string());
        }
        "doubao" => {
            // 豆包模型的特殊处理
            let app_handle = app.clone();
            let stream_id_clone = stream_id.clone();
            let model_id_clone = model_id.clone();
            let custom_model_name = model_name_from_param.or(model_name_from_config);
            let custom_model_name_for_stream = custom_model_name.clone();

            tauri::async_runtime::spawn(async move {
                println!("处理豆包流式请求...");
                if let Some(name) = &custom_model_name_for_stream {
                    println!("使用自定义豆包模型名称: {}", name);
                }

                // 根据是否提供了消息历史，选择不同的处理方式
                let stream_result = if !final_messages_for_doubao.is_empty() {
                    // 转换前端消息格式到GenAI库的ChatMessage格式
                    let chat_messages: Vec<ChatMessage> = final_messages_for_doubao
                        .iter()
                        .filter_map(|msg| convert_to_chat_message(msg))
                        .collect();

                    println!("转换后有 {} 条有效消息用于豆包对话历史", chat_messages.len());

                    // 使用豆包历史对话流式函数
                    doubao_stream_chat_with_history(
                        api_key,
                        chat_messages,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                } else {
                    // 使用豆包单条消息流式函数
                    println!("使用豆包单条消息: {}", message);
                    stream_from_doubao(
                        api_key,
                        message,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                };

                match stream_result {
                    Ok(mut stream) => {
                        println!("开始处理豆包流...");

                        // 处理流式输出
                        while let Some(chunk) = stream.next().await {
                            match chunk {
                                Ok(text) => {
                                    // 向前端发送事件
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": text,
                                            "done": false
                                        }),
                                    );
                                }
                                Err(e) => {
                                    // 发送错误信息
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": format!("豆包处理错误: {}", e),
                                            "done": true
                                        }),
                                    );
                                    return;
                                }
                            }
                        }

                        // 流结束
                        println!("豆包流处理完成");
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": "",
                                "done": true
                            }),
                        );
                    }
                    Err(e) => {
                        // 初始化流出错
                        println!("无法初始化豆包流: {}", e);
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": format!("无法连接到豆包服务: {}", e),
                                "done": true
                            }),
                        );
                    }
                }
            })
        }
        "grok" => {
            // Grok模型的特殊处理
            let app_handle = app.clone();
            let stream_id_clone = stream_id.clone();
            let model_id_clone = model_id.clone();
            let custom_model_name = model_name_from_param.or(model_name_from_config);
            let custom_model_name_for_stream = custom_model_name.clone();

            tauri::async_runtime::spawn(async move {
                println!("处理Grok流式请求...");
                if let Some(name) = &custom_model_name_for_stream {
                    println!("使用自定义Grok模型名称: {}", name);
                }

                // 根据是否提供了消息历史，选择不同的处理方式
                let stream_result = if !final_messages_for_grok.is_empty() {
                    println!("转换后有 {} 条有效消息用于Grok对话历史", final_messages_for_grok.len());

                    // 使用Grok历史对话流式函数
                    grok_stream_chat_with_history(
                        api_key,
                        final_messages_for_grok,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                } else {
                    // 使用Grok单条消息流式函数
                    println!("使用Grok单条消息: {}", message);
                    stream_from_grok(
                        api_key,
                        message,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                };

                match stream_result {
                    Ok(mut stream) => {
                        println!("开始处理Grok流...");

                        // 处理流式输出
                        while let Some(chunk) = stream.next().await {
                            match chunk {
                                Ok(text) => {
                                    // 向前端发送事件
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": text,
                                            "done": false
                                        }),
                                    );
                                }
                                Err(e) => {
                                    // 发送错误信息
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": format!("Grok处理错误: {}", e),
                                            "done": true
                                        }),
                                    );
                                    return;
                                }
                            }
                        }

                        // 流结束
                        println!("Grok流处理完成");
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": "",
                                "done": true
                            }),
                        );
                    }
                    Err(e) => {
                        // 初始化流出错
                        println!("无法初始化Grok流: {}", e);
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": format!("无法连接到Grok服务: {}", e),
                                "done": true
                            }),
                        );
                    }
                }
            })
        }
        "custom" => {
            // 自定义API的流式处理
            let endpoint = get_api_endpoint(app.clone()).await?;
            if endpoint.is_empty() {
                return Err("未配置自定义API端点".to_string());
            }

            // 暂时不支持自定义API的流式输出
            return Err("暂不支持自定义API的流式输出".to_string());
        }
        // 使用GenAI统一处理其他模型
        _ => {
            let app_handle = app.clone();
            let stream_id_clone = stream_id.clone();
            let model_id_clone = model_id.clone();
            // 合并自定义模型名称参数和配置
            let custom_model_name = model_name_from_param.or(model_name_from_config);
            // 克隆字符串而不是借用引用
            let custom_model_name_for_stream = custom_model_name.clone();

            tauri::async_runtime::spawn(async move {
                println!("处理流式请求...");
                if let Some(name) = &custom_model_name_for_stream {
                    println!("使用自定义模型名称: {}", name);
                }

                // 根据是否提供了消息历史，选择不同的处理方式
                let stream_result = if !final_messages_for_genai.is_empty() {
                    // 转换前端消息格式到GenAI库的ChatMessage格式
                    let chat_messages: Vec<ChatMessage> = final_messages_for_genai
                        .iter()
                        .filter_map(|msg| convert_to_chat_message(msg))
                        .collect();

                    println!("转换后有 {} 条有效消息用于对话历史", chat_messages.len());

                    // 使用对话历史和自定义模型名称
                    stream_chat_with_history(
                        api_key,
                        &model_id_clone,
                        chat_messages,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                } else {
                    // 使用单条消息和自定义模型名称
                    println!("使用单条消息: {}", message);
                    stream_message_from_ai(
                        api_key,
                        &model_id_clone,
                        message,
                        custom_model_name_for_stream.as_deref(),
                    )
                    .await
                };

                match stream_result {
                    Ok(mut stream) => {
                        println!("开始处理流...");

                        // 处理流式输出
                        while let Some(chunk) = stream.next().await {
                            match chunk {
                                Ok(text) => {
                                    // println!("流式输出: {}", text);
                                    // 向前端发送事件
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": text,
                                            "done": false
                                        }),
                                    );
                                }
                                Err(e) => {
                                    // 发送错误信息
                                    let _ = app_handle.emit(
                                        "ai-stream-chunk",
                                        serde_json::json!({
                                            "id": stream_id_clone,
                                            "chunk": format!("处理错误: {}", e),
                                            "done": true
                                        }),
                                    );
                                    return;
                                }
                            }
                        }

                        // 流结束
                        println!("流处理完成");
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": "",
                                "done": true
                            }),
                        );
                    }
                    Err(e) => {
                        // 初始化流出错
                        println!("无法初始化流: {}", e);
                        let _ = app_handle.emit(
                            "ai-stream-chunk",
                            serde_json::json!({
                                "id": stream_id_clone,
                                "chunk": format!("无法连接到AI服务: {}", e),
                                "done": true
                            }),
                        );
                    }
                }
            })
        }
    };

    // 保存任务引用，以便稍后取消
    let mut tasks = STREAMING_TASKS.lock().unwrap();
    tasks.insert(stream_id, stream_task);

    println!("流式请求已提交处理");
    Ok(())
}

// 取消生成
#[tauri::command]
pub async fn cancel_ai_generation(stream_id: String) -> Result<(), String> {
    let mut tasks = STREAMING_TASKS.lock().unwrap();

    if let Some(task) = tasks.remove(&stream_id) {
        // 中止任务
        task.abort();
        Ok(())
    } else {
        Err("未找到指定的生成任务".to_string())
    }
}

// 保存自定义模型名称
#[tauri::command]
pub async fn save_model_name(
    app: tauri::AppHandle,
    model_id: String,
    model_name: String,
) -> Result<(), String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名保存自定义模型名称
    let key = format!("model_name_{}", model_id);
    db.save_setting(&key, &model_name).map_err(|e| e.to_string())?;

    Ok(())
}

// 获取自定义模型名称
#[tauri::command]
pub async fn get_model_name_config(
    app: tauri::AppHandle,
    model_id: String,
) -> Result<String, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名获取自定义模型名称
    let key = format!("model_name_{}", model_id);
    match db.get_setting(&key).map_err(|e| e.to_string())? {
        Some(model_name) => Ok(model_name),
        None => Ok("".to_string()),
    }
}

// 保存max_tokens配置
#[tauri::command]
pub async fn save_max_tokens_config(
    app: tauri::AppHandle,
    model_id: String,
    max_tokens: i32,
) -> Result<(), String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名保存max_tokens配置
    let key = format!("max_tokens_{}", model_id);
    db.save_setting(&key, &max_tokens.to_string()).map_err(|e| e.to_string())?;

    Ok(())
}

// 获取max_tokens配置
#[tauri::command]
pub async fn get_max_tokens_config(
    app: tauri::AppHandle,
    model_id: String,
) -> Result<i32, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    // 使用模型ID作为键名获取max_tokens配置
    let key = format!("max_tokens_{}", model_id);
    match db.get_setting(&key).map_err(|e| e.to_string())? {
        Some(value) => {
            value.parse::<i32>().map_err(|e| format!("解析max_tokens值失败: {}", e))
        },
        None => Ok(0), // 返回0表示未配置，前端会使用默认值
    }
}

// 支持图片上传的AI消息发送（非流式）
#[tauri::command]
pub async fn send_ai_message_with_images(
    app: tauri::AppHandle,
    model_id: String,
    message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
) -> Result<String, String> {
    // 获取API密钥
    let api_key = get_api_key(app.clone(), model_id.clone()).await?;

    if api_key.is_empty() {
        return Err("未配置API密钥".to_string());
    }

    // 获取自定义模型名称
    let custom_model_name = get_model_name_config(app.clone(), model_id.clone())
        .await
        .ok();

    // 调用支持图片的AI函数
    send_message_with_images_to_ai(
        api_key,
        &model_id,
        message,
        image_files,
        custom_model_name.as_deref(),
    )
    .await
}

// 支持图片上传的AI消息发送（流式）
#[tauri::command]
pub async fn send_ai_message_with_images_stream(
    app: tauri::AppHandle,
    model_id: String,
    message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
    stream_id: String,
) -> Result<(), String> {
    println!(
        "接收到带图片的流式请求: model={}, message_len={}, images={}, stream_id={}",
        model_id,
        message.len(),
        image_files.len(),
        stream_id
    );

    // 获取API密钥
    let api_key = get_api_key(app.clone(), model_id.clone()).await?;

    if api_key.is_empty() {
        return Err("未配置API密钥".to_string());
    }

    // 获取自定义模型名称
    let custom_model_name = get_model_name_config(app.clone(), model_id.clone())
        .await
        .ok();

    // 创建流式处理任务
    let app_handle = app.clone();
    let stream_id_clone = stream_id.clone();
    let model_id_clone = model_id.clone();

    tauri::async_runtime::spawn(async move {
        println!("处理带图片的流式请求...");

        let stream_result = stream_message_with_images_from_ai(
            api_key,
            &model_id_clone,
            message,
            image_files,
            custom_model_name.as_deref(),
        )
        .await;

        match stream_result {
            Ok(mut stream) => {
                println!("开始处理带图片的流...");

                // 处理流式输出
                while let Some(chunk) = stream.next().await {
                    match chunk {
                        Ok(text) => {
                            // 向前端发送事件
                            let _ = app_handle.emit(
                                "ai-stream-chunk",
                                serde_json::json!({
                                    "id": stream_id_clone,
                                    "chunk": text,
                                    "done": false
                                }),
                            );
                        }
                        Err(e) => {
                            // 发送错误信息
                            let _ = app_handle.emit(
                                "ai-stream-chunk",
                                serde_json::json!({
                                    "id": stream_id_clone,
                                    "chunk": format!("处理错误: {}", e),
                                    "done": true
                                }),
                            );
                            return;
                        }
                    }
                }

                // 流结束
                println!("带图片的流处理完成");
                let _ = app_handle.emit(
                    "ai-stream-chunk",
                    serde_json::json!({
                        "id": stream_id_clone,
                        "chunk": "",
                        "done": true
                    }),
                );
            }
            Err(e) => {
                // 初始化流出错
                println!("无法初始化带图片的流: {}", e);
                let _ = app_handle.emit(
                    "ai-stream-chunk",
                    serde_json::json!({
                        "id": stream_id_clone,
                        "chunk": format!("无法连接到AI服务: {}", e),
                        "done": true
                    }),
                );
            }
        }
    });

    Ok(())
}

// 迁移旧的文件配置到数据库
#[tauri::command]
pub async fn migrate_config_to_database(app: tauri::AppHandle) -> Result<String, String> {
    let db_state = app.try_state::<std::sync::Mutex<crate::db::DbManager>>()
        .ok_or("数据库状态未初始化")?;
    
    let db = db_state.lock().map_err(|e| format!("锁定数据库失败: {}", e))?;
    
    let app_dir = dirs::data_dir()
        .ok_or("无法获取应用数据目录".to_string())?
        .join("mytips");
    
    let mut migrated_count = 0;
    
    // 迁移API密钥
    let keys_dir = app_dir.join("ai_keys");
    if keys_dir.exists() {
        for entry in std::fs::read_dir(keys_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "key") {
                if let Some(stem) = path.file_stem() {
                    if let Some(model_id) = stem.to_str() {
                        // 读取旧的加密密钥
                        if let Ok(encoded_key) = std::fs::read_to_string(&path) {
                            let key = format!("api_key_{}", model_id);
                            // 直接保存已加密的密钥到数据库
                            if db.save_setting(&key, &encoded_key).is_ok() {
                                migrated_count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 迁移自定义API端点
    let endpoint_file = app_dir.join("config").join("custom_endpoint.txt");
    if endpoint_file.exists() {
        if let Ok(endpoint) = std::fs::read_to_string(endpoint_file) {
            if !endpoint.is_empty() {
                if db.save_setting("custom_api_endpoint", &endpoint).is_ok() {
                    migrated_count += 1;
                }
            }
        }
    }
    
    // 迁移自定义模型名称
    let model_config_dir = app_dir.join("config").join("models");
    if model_config_dir.exists() {
        for entry in std::fs::read_dir(model_config_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_file() {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if file_name.ends_with("_name.txt") {
                        let model_id = file_name.replace("_name.txt", "");
                        if let Ok(model_name) = std::fs::read_to_string(&path) {
                            if !model_name.trim().is_empty() {
                                let key = format!("model_name_{}", model_id);
                                if db.save_setting(&key, model_name.trim()).is_ok() {
                                    migrated_count += 1;
                                }
                            }
                        }
                    } else if file_name.ends_with("_max_tokens.txt") {
                        let model_id = file_name.replace("_max_tokens.txt", "");
                        if let Ok(max_tokens_str) = std::fs::read_to_string(&path) {
                            if let Ok(max_tokens) = max_tokens_str.trim().parse::<i32>() {
                                let key = format!("max_tokens_{}", model_id);
                                if db.save_setting(&key, &max_tokens.to_string()).is_ok() {
                                    migrated_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if migrated_count > 0 {
        Ok(format!("成功迁移 {} 个配置项到数据库", migrated_count))
    } else {
        Ok("没有发现需要迁移的配置".to_string())
    }
}
