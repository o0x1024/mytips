use crate::{api::settings::get_client_with_proxy, db::UnifiedDbManager};
use base64::{engine::general_purpose, Engine as _};
use futures::{FutureExt, Stream, StreamExt};
use genai::{
    adapter::AdapterKind,
    chat::{ChatMessage, ChatRequest, ContentPart, ImageSource, MessageContent},
    resolver::{AuthData, Endpoint, ServiceTargetResolver},
    Client, ModelIden, ServiceTarget,
};
use reqwest::Proxy;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomModelConfig {
    pub id: String,
    pub name: String,
    pub endpoint: String,
    pub model_name: String,
    pub adapter_type: String, // openai, ollama, etc.
    pub api_key: Option<String>,
}

pub async fn create_genai_client(
    api_key: String,
    _provider: &str,
    db_manager: &UnifiedDbManager,
) -> Result<genai::Client, String> {
    // 获取代理设置
    let proxy_settings = crate::api::settings::get_proxy_settings_internal(db_manager).await?;

    let api_key_clone = api_key.clone();
    // 创建 ServiceTargetResolver
    let service_target_resolver = ServiceTargetResolver::from_resolver_fn(
        move |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
            let model_name = service_target.model.model_name.to_string();
            let mut endpoint = service_target.endpoint;
            let mut auth = service_target.auth;
            let mut model = service_target.model;
            if model_name.starts_with("doubao") {
                endpoint = Endpoint::from_static("https://ark.cn-beijing.volces.com/api/v3");
                auth = AuthData::from_single(api_key_clone.clone());
                model = ModelIden::new(AdapterKind::OpenAI, &model_name);
            } else if model_name.starts_with("qwen") {
                endpoint = Endpoint::from_static("https://dashscope.aliyuncs.com/compatible-mode/v1/");
                auth = AuthData::from_single(api_key_clone.clone());
                model = ModelIden::new(AdapterKind::OpenAI, &model_name);
            } else {
                model = ModelIden::new(AdapterKind::OpenAI, &model_name);
            }
            Ok(ServiceTarget {
                auth,
                model,
                endpoint,
            })
        },
    );

    // 创建带代理的reqwest客户端
    let mut client_builder = reqwest::Client::builder();
    
    if proxy_settings.enabled {
        // 构建代理URL
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.protocol, proxy_settings.host, proxy_settings.port
        );
        
        println!("AI Client using proxy: {}", proxy_url);
        
        match Proxy::all(&proxy_url) {
            Ok(proxy) => {
                client_builder = client_builder.proxy(proxy);
                println!("Proxy configured successfully for AI client");
            }
            Err(e) => {
                println!("Failed to create proxy for AI client: {}", e);
            }
        }
    } else {
        println!("AI Client not using proxy");
    }
    
    let reqwest_client = client_builder
        .build()
        .unwrap_or_else(|e| {
            println!("Failed to build reqwest client: {}", e);
            reqwest::Client::new()
        });

    // 创建带有认证和自定义解析器的客户端构建器
    let client = Client::builder()
        .with_reqwest(reqwest_client)
        .with_service_target_resolver(service_target_resolver)
        // .with_auth_resolver_fn(move |_| Ok(Some(AuthData::from_single(api_key.clone()))))
        .build();

    Ok(client)
}

// 获取特定模型名称
pub fn get_model_name(model_id: &str, custom_model_name: Option<&str>) -> String {
    // 如果提供了自定义模型名称，则优先使用
    if let Some(name) = custom_model_name {
        return name.to_string();
    }

    "".to_string()
}

// 处理AI错误响应的辅助函数，将错误格式化为可以显示的回答
pub fn format_ai_error(error: &str) -> String {
    format!("抱歉，发生了错误：{}", error)
}

// 流式API响应类型
pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>;

// 通用函数，用于处理自定义API端点
// 注意：对于自定义API，需要实现特殊处理，这里暂时保留旧的实现
pub async fn send_to_custom(
    api_key: String,
    endpoint: String,
    message: String,
    db_manager: &UnifiedDbManager,
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

    let url = if endpoint.trim_end_matches('/').ends_with("chat/completions") {
        endpoint
    } else {
        format!("{}/chat/completions", endpoint.trim_end_matches('/'))
    };

    let response = client
        .post(&url)
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
    db_manager: &UnifiedDbManager,
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
    match chat_res.first_text() {
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
    db_manager: &UnifiedDbManager,
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
                if let Some(content) = response.first_text() {
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


// 支持图片的AI消息发送（非流式）
pub async fn send_message_with_images_to_ai(
    client: Client,
    model_id: &str,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
) -> Result<String, String> {
    let model_name = get_model_name(model_id, None);

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
        Err(e) => return Err(format!("AI request failed: {}", e)),
    };

    match chat_res.first_text() {
        Some(content) => Ok(content.to_string()),
        None => Err("Unable to get AI response content".to_string()),
    }
}

// 支持图片的AI消息发送（流式）
pub async fn stream_message_with_images_from_ai(
    client: Client,
    model_id: &str,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
) -> Result<TextStream, String> {
    let model_name = get_model_name(model_id, None);

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

    // 创建请求选项，启用内容捕获
    let mut request_options = genai::chat::ChatOptions::default();
    request_options.capture_content = Some(true);

    let chat_stream_response = client
        .exec_chat_stream(&model_name, chat_req, Some(&request_options))
        .await
        .map_err(|e| e.to_string())?;

    let (tx, rx) = mpsc::channel(100);

    tokio::spawn(async move {
        let mut stream = chat_stream_response.stream;
        let mut has_content = false;

        while let Some(event) = stream.next().await {
            match event {
                Ok(genai::chat::ChatStreamEvent::Chunk(chunk)) => {
                    has_content = true;
                    if tx.send(Ok(chunk.content)).await.is_err() {
                        break;
                    }
                }
                Ok(genai::chat::ChatStreamEvent::End(end)) => {
                    println!("Image stream received End event: {:?}", end);
                    // 处理End事件中可能包含的内容
                    if let Some(contents) = end.captured_content {
                        for content in contents {
                            if let genai::chat::MessageContent::Text(text) = content {
                                if !text.is_empty() {
                                    has_content = true;
                                    println!(
                                        "Sending captured content from image stream: {}",
                                        if text.len() > 50 {
                                            format!("{}...", &text[..50])
                                        } else {
                                            text.clone()
                                        }
                                    );
                                    let _ = tx.send(Ok(text)).await;
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let _ = tx.send(Err(e.to_string())).await;
                    break;
                }
                _ => {
                    println!("Image stream received other event type");
                } // 记录其他事件类型但不处理
            }
        }

        // 如果整个流程结束后没有收到任何内容，发送一个后备消息
        if !has_content {
            println!("No content received from image stream, sending fallback message");
            let _ = tx.send(Ok("I'm processing your message with images. Please wait a moment or try again if no response appears.".to_string())).await;
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 创建自定义GenAI客户端，使用ServiceTargetResolver
pub async fn create_custom_genai_client(
    endpoint_url: String,
    api_key: String,
    model_name: String,
    adapter_type: String,
    _custom_headers: Option<std::collections::HashMap<String, String>>,
    db_manager: &UnifiedDbManager,
) -> Result<Client, String> {
    // 获取代理设置，并在本地端点时绕过代理（避免对 127.0.0.1/localhost 的代理导致 404）
    let proxy_settings = crate::api::settings::get_proxy_settings_internal(db_manager).await?;
    
    // 检查是否为本地端点
    let is_local_endpoint = endpoint_url.contains("127.0.0.1")
        || endpoint_url.contains("localhost")
        || endpoint_url.contains("::1");

    let mut client_builder = reqwest::Client::builder();
    
    if proxy_settings.enabled && !is_local_endpoint {
        let proxy_url = format!(
            "{}://{}:{}",
            proxy_settings.protocol, proxy_settings.host, proxy_settings.port
        );
        
        println!("[Custom Client] Using proxy: {}", proxy_url);
        
        match reqwest::Proxy::all(&proxy_url) {
            Ok(proxy) => {
                client_builder = client_builder.proxy(proxy);
                println!("[Custom Client] Proxy configured successfully");
            }
            Err(e) => {
                println!("[Custom Client] Failed to create proxy: {}", e);
            }
        }
    } else if is_local_endpoint {
        println!("[Custom Client] Bypassing proxy for local endpoint: {}", endpoint_url);
    } else {
        println!("[Custom Client] Not using proxy");
    }

    let reqwest_client = client_builder
        .build()
        .map_err(|e| format!("Failed to build reqwest client: {}", e))?;

    // 解析适配器类型
    let adapter_kind = match adapter_type.to_lowercase().as_str() {
        "openai" => AdapterKind::OpenAI,
        "anthropic" | "claude" => AdapterKind::Anthropic,
        "gemini" | "google" => AdapterKind::Gemini,
        "deepseek" => AdapterKind::DeepSeek,
        "ollama" => AdapterKind::Ollama,
        _ => AdapterKind::OpenAI, // 默认使用OpenAI格式
    };

    println!(
        "[Custom Client] Creating client: endpoint={}, model={}, adapter={:?}",
        endpoint_url, model_name, adapter_kind
    );

    // 创建ServiceTargetResolver
    let target_resolver =
        ServiceTargetResolver::from_resolver_async_fn(move |mut service_target: ServiceTarget| {
            let endpoint_url = endpoint_url.clone();
            let api_key = api_key.clone();
            let model_name = model_name.clone();
            let adapter_kind = adapter_kind.clone();

            async move {
                // 规范化端点：
                // - OpenAI 习惯以 /v1 结尾
                // - Ollama 使用根路径（http://host:11434），其自身拼接 /api/chat 等
                let mut base = endpoint_url.trim_end_matches('/').to_string();
                match adapter_kind {
                    AdapterKind::OpenAI => {
                        if !base.ends_with("/v1") {
                            base.push_str("/v1");
                        }
                    }
                    AdapterKind::Ollama => {
                        // 对于 Ollama，genai 适配器会自行拼接 /api/chat 等路径
                        // 这里确保端点为根地址（去掉 /v1 或 /api 等后缀）
                        base = base
                            .trim_end_matches("/v1")
                            .trim_end_matches("/api")
                            .trim_end_matches('/')
                            .to_string();
                    }
                    _ => {}
                }
                // 使用规范化端点
                service_target.endpoint = Endpoint::from_owned(base);

                // 使用提供的API密钥
                service_target.auth = if api_key.is_empty() {
                    AuthData::from_single("")
                } else {
                    // 根据不同的适配器类型，可能需要不同的认证头格式
                    match adapter_kind {
                        AdapterKind::OpenAI => {
                            // OpenAI 格式通常是 "Bearer {api_key}"
                            AuthData::from_single(api_key)
                        }
                        AdapterKind::Anthropic => {
                            // Anthropic 可能使用 "x-api-key" 头
                            AuthData::from_single(api_key)
                        }
                        AdapterKind::Gemini => {
                            // Gemini 可能有不同的认证方式
                            AuthData::from_single(api_key)
                        }
                        _ => AuthData::from_single(api_key),
                    }
                };

                // 创建模型标识符
                service_target.model = ModelIden::new(adapter_kind, &model_name);

                // 打印更多调试信息
                println!(
                    "[Custom Client] Resolved target: endpoint={}, model={}, adapter={:?}",
                    service_target.endpoint.base_url(),
                    service_target.model.model_name,
                    service_target.model.adapter_kind
                );

                Ok(service_target)
            }
            .boxed()
        });

    // 构建客户端
    let client = Client::builder()
        .with_reqwest(reqwest_client)
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
    db_manager: &UnifiedDbManager,
) -> Result<String, String> {
    println!("[Custom AI] Sending message to model: {}", model_name);
    // 对 Ollama 走原生 HTTP API，避免第三方库路径拼接差异导致的 404
    if adapter_type.to_lowercase() == "ollama" {
        use serde_json::json;
        let client = get_client_with_proxy(db_manager).await?;
        let base = endpoint_url
            .trim_end_matches('/')
            .trim_end_matches("/v1")
            .trim_end_matches("/api")
            .to_string();
        let url = format!("{}/api/chat", base);
        println!("[Custom AI][Ollama] POST {}", url);

        let mut req = client.post(&url).header("Content-Type", "application/json");
        if !api_key.is_empty() {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }
        if let Some(headers) = &custom_headers {
            for (k, v) in headers {
                req = req.header(k, v);
            }
        }

        let payload = json!({
            "model": model_name,
            "messages": [{"role": "user", "content": message}],
            "stream": false
        });

        let resp = req
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Ollama request failed: {}", e))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body_text = resp.text().await.unwrap_or_default();
            return Ok(format_ai_error(&format!(
                "Ollama HTTP {}: {}",
                status, body_text
            )));
        }
        let v: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Ollama parse json failed: {}", e))?;
        if let Some(content) = v
            .get("message")
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
        {
            return Ok(content.to_string());
        }
        return Ok(format_ai_error("无法解析 Ollama 响应"));
    }

    // 其他适配器使用 genai 客户端
    let client = create_custom_genai_client(
        endpoint_url,
        api_key,
        model_name.clone(),
        adapter_type,
        custom_headers,
        db_manager,
    )
    .await?;

    let chat_req = ChatRequest::new(vec![ChatMessage::user(message)]);
    println!("[Custom AI] Executing chat request...");
    let chat_res = match client.exec_chat(&model_name, chat_req, None).await {
        Ok(res) => res,
        Err(e) => {
            let error_message = format!("Custom AI request failed: {}", e);
            println!("[Custom AI] Error: {}", error_message);
            return Ok(format_ai_error(&error_message));
        }
    };

    println!("[Custom AI] Chat request successful, processing response...");
    match chat_res.first_text() {
        Some(content) => Ok(content.to_string()),
        None => Ok(format_ai_error("无法获取自定义AI响应内容")),
    }
}
