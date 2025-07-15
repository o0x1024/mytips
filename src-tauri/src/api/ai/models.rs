use crate::{api::settings::get_client_with_proxy, db::DbManager};
use base64::{engine::general_purpose, Engine as _};
use futures::{Stream, StreamExt, FutureExt};
use genai::{
    adapter::AdapterKind,
    chat::{ChatMessage, ChatRequest, ContentPart, ImageSource, MessageContent, StreamChunk},
    resolver::{AuthData, Endpoint, ServiceTargetResolver},
    Client, ModelIden, ServiceTarget,
};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use serde::{Deserialize, Serialize};

// 自定义模型配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomModel {
    pub model_id: String,
    pub model_name: String,
    pub endpoint: String,
    pub api_key: String,
    pub adapter_type: String, // "openai", "gemini", etc.
    pub custom_headers: Option<std::collections::HashMap<String, String>>,
}

// 创建GenAI客户端并添加认证
pub async fn create_genai_client(
    api_key: String,
    model_id: &str,
    db_manager: &DbManager,
) -> Result<Client, String> {
    // 获取代理设置
    let proxy_settings = crate::api::settings::get_proxy_settings_internal(db_manager).await?;

    // 如果代理启用，设置环境变量（这会影响reqwest，而GenAI库内部使用reqwest）
    if proxy_settings.enabled {
        // 构建代理URL
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.r#type, proxy_settings.host, proxy_settings.port
        );

        println!("为GenAI客户端配置代理: {}", proxy_url);

        // 设置HTTP_PROXY和HTTPS_PROXY环境变量
        std::env::set_var("HTTP_PROXY", &proxy_url);
        std::env::set_var("HTTPS_PROXY", &proxy_url);

        // 如果需要认证，目前不支持，打印警告
        if proxy_settings.auth {
            println!("警告：通过环境变量设置代理时不支持认证");
        }
    } else {
        // 确保代理环境变量被清除
        std::env::remove_var("HTTP_PROXY");
        std::env::remove_var("HTTPS_PROXY");
    }

    // 为不同的闭包准备单独的 api_key 克隆，避免多次移动
    let api_key_for_resolver = api_key.clone();

    // 创建 ServiceTargetResolver
    let service_target_resolver = ServiceTargetResolver::from_resolver_async_fn(
        move |mut service_target: ServiceTarget| {
            let key = api_key_for_resolver.clone();
            async move {
                let model_name = service_target.model.model_name.to_string();
                if model_name.starts_with("doubao") {
                    // 豆包 Ark 平台，兼容 OpenAI 协议
                    service_target.endpoint = Endpoint::from_static(
                        "https://ark.cn-beijing.volces.com/api/v3/",
                    );
                    service_target.auth = AuthData::from_single(key);
                    // 强制使用 OpenAI 适配器，避免被误判为 Ollama
                    service_target.model = ModelIden::new(AdapterKind::OpenAI, &model_name);
                } else if model_name.starts_with("grok") {
                    service_target.endpoint = Endpoint::from_static("https://api.x.ai/v1");
                    service_target.auth = AuthData::from_single(key);
                }
                Ok::<_, genai::resolver::Error>(service_target)
            }
            .boxed()
        },
    );

    // 创建带有认证和自定义解析器的客户端构建器
    let client = Client::builder()
        .with_service_target_resolver(service_target_resolver)
        .with_auth_resolver_fn(move |_| Ok(Some(AuthData::from_single(api_key.clone()))))
        .build();

    Ok(client)
}

// 获取特定模型名称
pub fn get_model_name(model_id: &str, custom_model_name: Option<&str>) -> String {
    // 如果提供了自定义模型名称，则优先使用
    if let Some(name) = custom_model_name {
        return name.to_string();
    }

    // 否则使用默认映射
    match model_id {
        "chatgpt" => "gpt-3.5-turbo".to_string(),
        "gemini" => "gemini-2.0-flash".to_string(),
        "deepseek" => "deepseek-chat".to_string(),
        "claude" => "claude-3.5-sonnet".to_string(),
        "qwen" => "qwen-max".to_string(), // 千问暂时不支持，可能需要自定义实现
        "doubao" => "doubao-1.5-pro-32k".to_string(), // 豆包默认模型
        "grok" => "grok-lpu".to_string(),      // Grok默认模型
        "custom" => "gpt-3.5-turbo".to_string(), // 自定义默认使用OpenAI格式
        _ => model_id.to_string(),
    }
}

// 处理AI错误响应的辅助函数，将错误格式化为可以显示的回答
pub fn format_ai_error(error: &str) -> String {
    format!("抱歉，发生了错误：{}", error)
}

// 发送消息到AI模型并获取响应
pub async fn send_message_to_ai(
    api_key: String,
    model_id: &str,
    message: String,
    db_manager: &DbManager,
) -> Result<String, String> {
    // 创建genai客户端
    let client = create_genai_client(api_key, model_id, db_manager).await?;

    // 获取适合当前模型的模型名称
    let model_name = get_model_name(model_id, None);

    // 创建聊天请求
    let chat_req = ChatRequest::new(vec![ChatMessage::user(message)]);

    // 发送请求并获取响应
    let chat_res = match client.exec_chat(&model_name, chat_req, None).await {
        Ok(res) => res,
        Err(e) => return Ok(format_ai_error(&format!("AI请求失败: {}", e))), // 直接返回格式化的错误作为回答
    };

    // 提取响应文本
    match chat_res.content_text_as_str() {
        Some(content) => Ok(content.to_string()),
        None => Ok(format_ai_error("无法获取AI响应内容")), // 直接返回格式化的错误作为回答
    }
}

// 流式API响应类型
pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>;

// 流式传输AI响应
pub async fn stream_message_from_ai(
    api_key: String,
    model_id: &str,
    message: String,
    custom_model_name: Option<&str>,
    db_manager: &DbManager,
) -> Result<TextStream, String> {
    println!("启动单消息流式请求: model={}", model_id);

    // 创建genai客户端
    let client = match create_genai_client(api_key, model_id, db_manager).await {
        Ok(client) => client,
        Err(e) => {
            // 如果客户端创建失败，直接返回一个只包含错误消息的流
            let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
            let error_msg = format_ai_error(&format!("创建AI客户端失败: {}", e));
            let _ = tx.send(Ok(error_msg)).await;
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
            return Ok(Box::pin(stream) as TextStream);
        }
    };

    // 获取适合当前模型的模型名称
    let model_name = get_model_name(model_id, custom_model_name);
    println!("使用模型名称: {}", model_name);

    // 创建聊天请求
    let chat_req = ChatRequest::new(vec![ChatMessage::user(message)]);

    // 创建mpsc通道，用于将非流式结果转换为我们自己的流
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理，先获取完整响应，然后模拟流式输出
    tokio::spawn(async move {
        // 使用常规非流式API获取全部响应
        match client.exec_chat(&model_name, chat_req, None).await {
            Ok(response) => {
                // 获取完整的响应文本
                if let Some(content) = response.content_text_as_str() {
                    // 模拟流式输出，每次发送一小段内容
                    for (i, chunk) in content.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                        let chunk_str: String = chunk.iter().collect();
                        let _ = tx.send(Ok(chunk_str)).await;
                        // 模拟网络延迟，使其看起来像真实的流式传输
                        sleep(Duration::from_millis(30)).await;
                    }
                } else {
                    // 将错误作为正常消息返回，这样前端可以直接显示
                    let error_msg = format_ai_error("无法获取AI响应内容");
                    let _ = tx.send(Ok(error_msg)).await;
                }
            }
            Err(e) => {
                // 将错误作为正常消息返回，这样前端可以直接显示
                let error_msg = format_ai_error(&format!("AI请求失败: {}", e));
                let _ = tx.send(Ok(error_msg)).await;
            }
        }
    });

    // 将接收器转换为流
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 通用函数，用于处理自定义API端点
// 注意：对于自定义API，需要实现特殊处理，这里暂时保留旧的实现
pub async fn send_to_custom(
    api_key: String,
    endpoint: String,
    message: String,
    db_manager: &DbManager,
) -> Result<String, String> {
    use serde_json::{json, Value};

    let client = get_client_with_proxy(db_manager).await?;

    // 假设自定义API遵循OpenAI格式
    let payload = json!({
        "model": "custom",
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ]
    });

    let response = client
        .post(&endpoint)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "无法获取错误信息".to_string());
        return Err(format!("API返回错误: {}", error_text));
    }

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    // 尝试提取回复内容，适配不同的API格式
    // OpenAI格式
    if let Some(content) = response_json["choices"][0]["message"]["content"].as_str() {
        return Ok(content.to_string());
    }

    // 尝试其他可能的格式
    if let Some(text) = response_json["output"]["text"].as_str() {
        return Ok(text.to_string());
    }

    Err("无法解析自定义API的响应".to_string())
}

// 支持对话历史的AI对话
pub async fn chat_with_history(
    api_key: String,
    model_id: &str,
    messages: Vec<ChatMessage>,
    db_manager: &DbManager,
) -> Result<String, String> {
    // 创建genai客户端
    let client = create_genai_client(api_key, model_id, db_manager).await?;

    // 获取适合当前模型的模型名称
    let model_name = get_model_name(model_id, None);

    // 创建聊天请求，使用完整的消息历史
    let chat_req = ChatRequest::new(messages);

    // 发送请求并获取响应
    let chat_res = match client.exec_chat(&model_name, chat_req, None).await {
        Ok(res) => res,
        Err(e) => return Ok(format_ai_error(&format!("AI请求失败: {}", e))), // 直接返回格式化的错误作为回答
    };

    // 提取响应文本
    match chat_res.content_text_as_str() {
        Some(content) => Ok(content.to_string()),
        None => Ok(format_ai_error("无法获取AI响应内容")), // 直接返回格式化的错误作为回答
    }
}

// 流式对话历史支持
pub async fn stream_chat_with_history(
    api_key: String,
    model_id: &str,
    messages: Vec<ChatMessage>,
    custom_model_name: Option<&str>,
    db_manager: &DbManager,
) -> Result<TextStream, String> {
    println!(
        "启动对话历史流式请求: model={}, messages={}",
        model_id,
        messages.len()
    );

    // 创建genai客户端
    let client = match create_genai_client(api_key, model_id, db_manager).await {
        Ok(client) => client,
        Err(e) => {
            // 如果客户端创建失败，直接返回一个只包含错误消息的流
            let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
            let error_msg = format_ai_error(&format!("创建AI客户端失败: {}", e));
            let _ = tx.send(Ok(error_msg)).await;
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
            return Ok(Box::pin(stream) as TextStream);
        }
    };

    // 获取适合当前模型的模型名称
    let model_name = get_model_name(model_id, custom_model_name);
    println!("使用模型名称: {}", model_name);

    // 创建聊天请求，使用完整的消息历史
    let chat_req = ChatRequest::new(messages);

    // 创建mpsc通道，用于将非流式结果转换为我们自己的流
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理，先获取完整响应，然后模拟流式输出
    tokio::spawn(async move {
        println!("开始获取带对话历史的AI响应...");

        // 使用常规非流式API获取全部响应
        match client.exec_chat(&model_name, chat_req, None).await {
            Ok(response) => {
                println!("获取AI响应成功");

                // 获取完整的响应文本
                if let Some(content) = response.content_text_as_str() {
                    println!("响应内容长度: {} 字符", content.len());

                    // 模拟流式输出，每次发送一小段内容
                    for (i, chunk) in content.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                        if i % 20 == 0 {
                            println!("发送第 {} 块...", i);
                        }

                        let chunk_str: String = chunk.iter().collect();
                        let _ = tx.send(Ok(chunk_str)).await;
                        // 模拟网络延迟，使其看起来像真实的流式传输
                        sleep(Duration::from_millis(30)).await;
                    }
                    println!("全部响应内容已发送完毕");
                } else {
                    println!("无法从响应中提取内容");
                    // 将错误作为正常消息返回，这样前端可以直接显示
                    let error_msg = format_ai_error("无法从响应中提取内容");
                    let _ = tx.send(Ok(error_msg)).await;
                }
            }
            Err(e) => {
                println!("AI请求失败: {}", e);
                // 将错误作为正常消息返回，这样前端可以直接显示
                let error_msg = format_ai_error(&format!("AI请求失败: {}", e));
                let _ = tx.send(Ok(error_msg)).await;
            }
        }
    });

    // 将接收器转换为流
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 阿里通义千问模型实现 (genai库暂不支持，保留原实现)
pub async fn send_to_qwen(
    api_key: String,
    message: String,
    db_manager: &DbManager,
) -> Result<String, String> {
    use serde_json::{json, Value};

    let client = get_client_with_proxy(db_manager).await?;

    let payload = json!({
        "model": "qwen-max",
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ]
    });

    let response = client
        .post("https://dashscope.aliyuncs.com/api/v1/services/aigc/text-generation/generation")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("请求失败: {}", e))?;

    if !response.status().is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "无法获取错误信息".to_string());
        return Err(format!("API返回错误: {}", error_text));
    }

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    // 提取回复内容
    if let Some(content) = response_json["output"]["text"].as_str() {
        Ok(content.to_string())
    } else {
        Err("无法解析AI响应".to_string())
    }
}

// 豆包模型专门实现
pub async fn send_to_doubao(
    api_key: String,
    message: String,
    custom_model_name: Option<&str>,
    db_manager: &DbManager,
) -> Result<String, String> {
    use serde_json::{json, Value};

    let client = get_client_with_proxy(db_manager).await?;

    // 豆包API端点
    let endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

    // 获取模型名称
    let model_name = custom_model_name.unwrap_or("doubao-1.5-pro-32k");

    let payload = json!({
        "model": model_name,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "temperature": 0.7,
        "max_tokens": 2000,
        "stream": false
    });

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("豆包API请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("豆包API返回错误 {}: {}", status, error_text));
    }

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析豆包API响应失败: {}", e))?;

    // 提取响应内容
    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(first_choice) = choices.first() {
            if let Some(content) = first_choice["message"]["content"].as_str() {
                return Ok(content.to_string());
            }
        }
    }

    Err("无法从豆包API响应中提取内容".to_string())
}

// 支持图片的AI消息发送（非流式）
pub async fn send_message_with_images_to_ai(
    api_key: String,
    model_id: &str,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
    custom_model_name: Option<&str>,
    db_manager: &DbManager,
) -> Result<String, String> {
    let client = create_genai_client(api_key, model_id, db_manager).await?;

    let model_name = get_model_name(model_id, custom_model_name);

    // 构建消息内容
    let mut content_parts: Vec<ContentPart> = Vec::new();

    // 添加图片部分
    for (filename, file_data) in image_files {
        let mime_type = match filename.split('.').last() {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            _ => "application/octet-stream", // 默认
        };
        content_parts.push(ContentPart::Image {
            content_type: mime_type.into(),
            source: ImageSource::Base64(general_purpose::STANDARD.encode(&file_data).into()),
        });
    }

    // 添加文本部分
    content_parts.push(ContentPart::Text(text_message.into()));

    let message = ChatMessage {
        role: genai::chat::ChatRole::User,
        content: MessageContent::Parts(content_parts),
        options: Default::default(),
    };

    let chat_req = ChatRequest::new(vec![message]);

    let chat_res = match client.exec_chat(&model_name, chat_req, None).await {
        Ok(res) => res,
        Err(e) => return Err(format!("AI请求失败: {}", e)),
    };

    match chat_res.content_text_as_str() {
        Some(content) => Ok(content.to_string()),
        None => Err("无法获取AI响应内容".to_string()),
    }
}

// 支持图片的AI消息发送（流式）
pub async fn stream_message_with_images_from_ai(
    api_key: String,
    model_id: &str,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
    custom_model_name: Option<&str>,
    db_manager: &DbManager,
) -> Result<TextStream, String> {
    let client = create_genai_client(api_key, model_id, db_manager).await?;

    let model_name = get_model_name(model_id, custom_model_name);

    // 构建消息内容
    let mut content_parts: Vec<ContentPart> = Vec::new();

    // 添加图片部分
    for (filename, file_data) in image_files {
        let mime_type = match filename.split('.').last() {
            Some("png") => "image/png",
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("gif") => "image/gif",
            Some("webp") => "image/webp",
            _ => "application/octet-stream", // 默认
        };
        content_parts.push(ContentPart::Image {
            content_type: mime_type.into(),
            source: ImageSource::Base64(general_purpose::STANDARD.encode(&file_data).into()),
        });
    }

    // 添加文本部分
    content_parts.push(ContentPart::Text(text_message.into()));

    let message = ChatMessage {
        role: genai::chat::ChatRole::User,
        content: MessageContent::Parts(content_parts),
        options: Default::default(),
    };

    let chat_req = ChatRequest::new(vec![message]);

    let chat_stream_response = client
        .exec_chat_stream(&model_name, chat_req, None)
        .await
        .map_err(|e| e.to_string())?;

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stream = chat_stream_response.stream;
        while let Some(event) = stream.next().await {
            match event {
                Ok(genai::chat::ChatStreamEvent::Chunk(chunk)) => {
                    if tx.send(Ok(chunk.content)).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e.to_string())).await;
                    break;
                }
                _ => {} // 其他事件如ToolCall, End等暂时忽略
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 获取模型显示名称的辅助函数
fn get_model_display_name(model_id: &str) -> &str {
    match model_id {
        "chatgpt" => "ChatGPT",
        "gemini" => "Gemini",
        "deepseek" => "DeepSeek",
        "claude" => "Claude",
        "qwen" => "通义千问",
        "doubao" => "豆包",
        "grok" => "Grok",
        "custom" => "自定义模型",
        _ => "未知模型",
    }
}

// 创建自定义GenAI客户端，使用ServiceTargetResolver
pub async fn create_custom_genai_client(
    endpoint_url: String,
    api_key: String,
    model_name: String,
    adapter_type: String,
    custom_headers: Option<std::collections::HashMap<String, String>>,
    db_manager: &DbManager,
) -> Result<Client, String> {
    // 获取代理设置
    let proxy_settings = crate::api::settings::get_proxy_settings_internal(db_manager).await?;

    // 如果代理启用，设置环境变量
    if proxy_settings.enabled {
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.r#type, proxy_settings.host, proxy_settings.port
        );
        println!("为自定义GenAI客户端配置代理: {}", proxy_url);
        std::env::set_var("HTTP_PROXY", &proxy_url);
        std::env::set_var("HTTPS_PROXY", &proxy_url);

        if proxy_settings.auth {
            println!("警告：通过环境变量设置代理时不支持认证");
        }
    } else {
        std::env::remove_var("HTTP_PROXY");
        std::env::remove_var("HTTPS_PROXY");
    }

    // 解析适配器类型
    let adapter_kind = match adapter_type.to_lowercase().as_str() {
        "openai" => AdapterKind::OpenAI,
        "anthropic" | "claude" => AdapterKind::Anthropic,
        "gemini" | "google" => AdapterKind::Gemini,
        "deepseek" => AdapterKind::DeepSeek,
        _ => AdapterKind::OpenAI, // 默认使用OpenAI格式
    };

    println!("创建自定义客户端: endpoint={}, model={}, adapter={:?}", 
             endpoint_url, model_name, adapter_kind);

    // 创建ServiceTargetResolver
    let target_resolver = ServiceTargetResolver::from_resolver_fn(
        move |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
            let ServiceTarget { .. } = service_target;
            
            // 使用自定义端点
            let endpoint = Endpoint::from_owned(endpoint_url.clone());
            
            // 使用提供的API密钥，如果为空则不设置认证
            let auth = if api_key.is_empty() {
                AuthData::from_single("") // 空认证
            } else {
                AuthData::from_single(api_key.clone())
            };
            
            // 创建模型标识符，使用自定义模型名称
            let model_iden = ModelIden::new(adapter_kind, &model_name);

            Ok(ServiceTarget {
                endpoint,
                auth,
                model: model_iden,
            })
        },
    );

    // 构建客户端
    let client = Client::builder()
        .with_service_target_resolver(target_resolver)
        .build();

    Ok(client)
}

// 使用自定义配置发送消息
pub async fn send_message_to_custom_ai(
    endpoint_url: String,
    api_key: String,
    model_name: String,
    adapter_type: String,
    custom_headers: Option<std::collections::HashMap<String, String>>,
    message: String,
    db_manager: &DbManager,
) -> Result<String, String> {
    // 创建自定义genai客户端
    let client = create_custom_genai_client(
        endpoint_url,
        api_key,
        model_name.clone(),
        adapter_type,
        custom_headers,
        db_manager,
    ).await?;

    // 创建聊天请求
    let chat_req = ChatRequest::new(vec![ChatMessage::user(message)]);

    // 发送请求并获取响应
    let chat_res = match client.exec_chat(&model_name, chat_req, None).await {
        Ok(res) => res,
        Err(e) => return Ok(format_ai_error(&format!("自定义AI请求失败: {}", e))),
    };

    // 提取响应文本
    match chat_res.content_text_as_str() {
        Some(content) => Ok(content.to_string()),
        None => Ok(format_ai_error("无法获取自定义AI响应内容")),
    }
}

// 使用自定义配置进行流式聊天
pub async fn stream_message_from_custom_ai(
    endpoint_url: String,
    api_key: String,
    model_name: String,
    adapter_type: String,
    custom_headers: Option<std::collections::HashMap<String, String>>,
    message: String,
    db_manager: &DbManager,
) -> Result<TextStream, String> {
    println!("启动自定义AI流式请求: endpoint={}, model={}", endpoint_url, model_name);

    // 创建自定义genai客户端
    let client = match create_custom_genai_client(
        endpoint_url,
        api_key,
        model_name.clone(),
        adapter_type,
        custom_headers,
        db_manager,
    ).await {
        Ok(client) => client,
        Err(e) => {
            let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
            let error_msg = format_ai_error(&format!("创建自定义AI客户端失败: {}", e));
            let _ = tx.send(Ok(error_msg)).await;
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
            return Ok(Box::pin(stream) as TextStream);
        }
    };

    // 创建聊天请求
    let chat_req = ChatRequest::new(vec![ChatMessage::user(message)]);

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理
    tokio::spawn(async move {
        println!("开始获取自定义AI响应...");

        match client.exec_chat(&model_name, chat_req, None).await {
            Ok(response) => {
                println!("获取自定义AI响应成功");

                if let Some(content) = response.content_text_as_str() {
                    println!("响应内容长度: {} 字符", content.len());

                    // 模拟流式输出
                    for (i, chunk) in content.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                        if i % 20 == 0 {
                            println!("发送第 {} 块...", i);
                        }

                        let chunk_str: String = chunk.iter().collect();
                        let _ = tx.send(Ok(chunk_str)).await;
                        sleep(Duration::from_millis(30)).await;
                    }
                    println!("自定义AI响应内容已发送完毕");
                } else {
                    println!("无法从自定义AI响应中提取内容");
                    let error_msg = format_ai_error("无法从自定义AI响应中提取内容");
                    let _ = tx.send(Ok(error_msg)).await;
                }
            }
            Err(e) => {
                println!("自定义AI请求失败: {}", e);
                let error_msg = format_ai_error(&format!("自定义AI请求失败: {}", e));
                let _ = tx.send(Ok(error_msg)).await;
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 使用自定义配置进行历史对话
pub async fn stream_chat_with_history_custom_ai(
    endpoint_url: String,
    api_key: String,
    model_name: String,
    adapter_type: String,
    custom_headers: Option<std::collections::HashMap<String, String>>,
    messages: Vec<ChatMessage>,
    db_manager: &DbManager,
) -> Result<TextStream, String> {
    println!(
        "启动自定义AI历史对话流式请求: endpoint={}, model={}, messages={}",
        endpoint_url, model_name, messages.len()
    );

    // 创建自定义genai客户端
    let client = match create_custom_genai_client(
        endpoint_url,
        api_key,
        model_name.clone(),
        adapter_type,
        custom_headers,
        db_manager,
    ).await {
        Ok(client) => client,
        Err(e) => {
            let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
            let error_msg = format_ai_error(&format!("创建自定义AI客户端失败: {}", e));
            let _ = tx.send(Ok(error_msg)).await;
            let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
            return Ok(Box::pin(stream) as TextStream);
        }
    };

    // 创建聊天请求，使用完整的消息历史
    let chat_req = ChatRequest::new(messages);

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理
    tokio::spawn(async move {
        println!("开始获取自定义AI历史对话响应...");

        match client.exec_chat(&model_name, chat_req, None).await {
            Ok(response) => {
                println!("获取自定义AI历史对话响应成功");

                if let Some(content) = response.content_text_as_str() {
                    println!("响应内容长度: {} 字符", content.len());

                    // 模拟流式输出
                    for (i, chunk) in content.chars().collect::<Vec<_>>().chunks(5).enumerate() {
                        if i % 20 == 0 {
                            println!("发送第 {} 块...", i);
                        }

                        let chunk_str: String = chunk.iter().collect();
                        let _ = tx.send(Ok(chunk_str)).await;
                        sleep(Duration::from_millis(30)).await;
                    }
                    println!("自定义AI历史对话响应内容已发送完毕");
                } else {
                    println!("无法从自定义AI历史对话响应中提取内容");
                    let error_msg = format_ai_error("无法从自定义AI历史对话响应中提取内容");
                    let _ = tx.send(Ok(error_msg)).await;
                }
            }
            Err(e) => {
                println!("自定义AI历史对话请求失败: {}", e);
                let error_msg = format_ai_error(&format!("自定义AI历史对话请求失败: {}", e));
                let _ = tx.send(Ok(error_msg)).await;
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}
