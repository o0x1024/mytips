use crate::api::ai::models::format_ai_error;
use crate::api::{settings::get_client_with_proxy, TextStream};
use base64::{engine::general_purpose, Engine as _};
use genai::chat::ChatMessage;
use tokio::sync::mpsc;

// 从GenAI库的MessageContent中提取纯文本内容
fn extract_text_content(content: &genai::chat::MessageContent) -> String {
    // 尝试将MessageContent转换为字符串
    // MessageContent通常有一个text()方法或者可以序列化为JSON
    if let Ok(json_value) = serde_json::to_value(content) {
        // 如果content已经是字符串，直接返回
        if let Some(text) = json_value.as_str() {
            return text.to_string();
        }

        // 如果content是对象，尝试提取Text字段
        if let Some(obj) = json_value.as_object() {
            if let Some(text) = obj.get("Text") {
                if let Some(text_str) = text.as_str() {
                    return text_str.to_string();
                }
            }
            // 尝试其他可能的字段名
            if let Some(text) = obj.get("text") {
                if let Some(text_str) = text.as_str() {
                    return text_str.to_string();
                }
            }
        }

        // 如果都不匹配，返回JSON字符串的表示（作为fallback）
        return json_value.to_string();
    }

    // 如果序列化失败，返回空字符串
    String::new()
}

// 豆包流式响应实现
pub async fn stream_from_doubao(
    api_key: String,
    message: String,
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    use futures::StreamExt;
    use serde_json::{json, Value};

    println!("启动豆包流式请求");

    let client = get_client_with_proxy().await?;

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
        "stream": true
    });

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("豆包流式API请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
        let error_msg = format_ai_error(&format!("豆包API返回错误 {}: {}", status, error_text));
        let _ = tx.send(Ok(error_msg)).await;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        return Ok(Box::pin(stream) as TextStream);
    }

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理流式响应
    tokio::spawn(async move {
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // 处理SSE格式的数据
                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.starts_with("data: ") {
                            let data = &line[6..]; // 移除 "data: " 前缀

                            if data == "[DONE]" {
                                break;
                            }

                            // 尝试解析JSON
                            if let Ok(json_data) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json_data["choices"].as_array() {
                                    if let Some(first_choice) = choices.first() {
                                        if let Some(delta) = first_choice.get("delta") {
                                            if let Some(content) = delta["content"].as_str() {
                                                if !content.is_empty() {
                                                    let _ = tx.send(Ok(content.to_string())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format_ai_error(&format!("豆包流式响应错误: {}", e));
                    let _ = tx.send(Ok(error_msg)).await;
                    break;
                }
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 豆包历史对话实现
pub async fn doubao_chat_with_history(
    api_key: String,
    messages: Vec<ChatMessage>,
    custom_model_name: Option<&str>,
) -> Result<String, String> {
    use serde_json::{json, Value};

    let client = get_client_with_proxy().await?;

    // 豆包API端点
    let endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

    // 获取模型名称
    let model_name = custom_model_name.unwrap_or("doubao-1.5-pro-32k");

    // 转换消息格式
    let api_messages: Vec<Value> = messages
        .iter()
        .map(|msg| {
            // 确保角色名是小写的，符合豆包API要求
            let role = match msg.role.to_string().to_lowercase().as_str() {
                "user" => "user",
                "assistant" => "assistant",
                "system" => "system",
                _ => "user", // 默认为user
            };

            // 提取消息内容，处理GenAI库的复杂content格式
            let content = extract_text_content(&msg.content);

            json!({
                "role": role,
                "content": content
            })
        })
        .collect();

    let payload = json!({
        "model": model_name,
        "messages": api_messages,
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

// 豆包历史对话流式实现
pub async fn doubao_stream_chat_with_history(
    api_key: String,
    messages: Vec<ChatMessage>,
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    use futures::StreamExt;
    use serde_json::{json, Value};

    println!("启动豆包历史对话流式请求");

    let client = get_client_with_proxy().await?;

    // 豆包API端点
    let endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

    // 获取模型名称
    let model_name = custom_model_name.unwrap_or("doubao-1.5-pro-32k");

    // 转换消息格式
    let api_messages: Vec<Value> = messages
        .iter()
        .map(|msg| {
            // 确保角色名是小写的，符合豆包API要求
            let role = match msg.role.to_string().to_lowercase().as_str() {
                "user" => "user",
                "assistant" => "assistant",
                "system" => "system",
                _ => "user", // 默认为user
            };

            // 提取消息内容，处理GenAI库的复杂content格式
            let content = extract_text_content(&msg.content);

            json!({
                "role": role,
                "content": content
            })
        })
        .collect();

    let payload = json!({
        "model": model_name,
        "messages": api_messages,
        "temperature": 0.7,
        "max_tokens": 2000,
        "stream": true
    });

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("豆包流式API请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
        let error_msg = format_ai_error(&format!("豆包API返回错误 {}: {}", status, error_text));
        let _ = tx.send(Ok(error_msg)).await;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        return Ok(Box::pin(stream) as TextStream);
    }

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理流式响应
    tokio::spawn(async move {
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // 处理SSE格式的数据
                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.starts_with("data: ") {
                            let data = &line[6..]; // 移除 "data: " 前缀

                            if data == "[DONE]" {
                                break;
                            }

                            // 尝试解析JSON
                            if let Ok(json_data) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json_data["choices"].as_array() {
                                    if let Some(first_choice) = choices.first() {
                                        if let Some(delta) = first_choice.get("delta") {
                                            if let Some(content) = delta["content"].as_str() {
                                                if !content.is_empty() {
                                                    let _ = tx.send(Ok(content.to_string())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format_ai_error(&format!("豆包流式响应错误: {}", e));
                    let _ = tx.send(Ok(error_msg)).await;
                    break;
                }
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 检测图片格式的辅助函数
fn detect_image_format(filename: &str, file_data: &[u8]) -> Result<&'static str, String> {
    let filename_lower = filename.to_lowercase();

    // 首先通过文件头（魔数）检测真实格式
    if file_data.len() >= 8 {
        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if file_data.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]) {
            return Ok("png");
        }
        // JPEG: FF D8 FF
        if file_data.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return Ok("jpeg");
        }
        // GIF: GIF87a 或 GIF89a
        if file_data.starts_with(b"GIF87a") || file_data.starts_with(b"GIF89a") {
            return Ok("gif");
        }
        // WEBP: RIFF....WEBP
        if file_data.len() >= 12 && file_data.starts_with(b"RIFF") && &file_data[8..12] == b"WEBP" {
            return Ok("webp");
        }
        // BMP: BM
        if file_data.starts_with(b"BM") {
            return Ok("bmp");
        }
        // TIFF: II*\0 或 MM\0*
        if file_data.starts_with(&[0x49, 0x49, 0x2A, 0x00])
            || file_data.starts_with(&[0x4D, 0x4D, 0x00, 0x2A])
        {
            return Ok("tiff");
        }
    }

    // 如果魔数检测失败，回退到文件扩展名检测
    if filename_lower.ends_with(".png") || filename_lower.ends_with(".apng") {
        Ok("png")
    } else if filename_lower.ends_with(".jpg") || filename_lower.ends_with(".jpeg") {
        Ok("jpeg")
    } else if filename_lower.ends_with(".gif") {
        Ok("gif")
    } else if filename_lower.ends_with(".webp") {
        Ok("webp")
    } else if filename_lower.ends_with(".bmp") || filename_lower.ends_with(".dib") {
        Ok("bmp")
    } else if filename_lower.ends_with(".tiff") || filename_lower.ends_with(".tif") {
        Ok("tiff")
    } else if filename_lower.ends_with(".ico") {
        Ok("x-icon")
    } else if filename_lower.ends_with(".icns") {
        Ok("icns")
    } else if filename_lower.ends_with(".sgi") {
        Ok("sgi")
    } else if filename_lower.ends_with(".j2c")
        || filename_lower.ends_with(".j2k")
        || filename_lower.ends_with(".jp2")
        || filename_lower.ends_with(".jpc")
        || filename_lower.ends_with(".jpf")
        || filename_lower.ends_with(".jpx")
    {
        Ok("jp2")
    } else {
        // 如果无法识别格式，返回错误
        Err(format!("不支持的图片格式: {}", filename))
    }
}

// 豆包视觉理解 - 支持图片输入的对话
pub async fn doubao_chat_with_images(
    api_key: String,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
    custom_model_name: Option<&str>,
) -> Result<String, String> {
    use serde_json::{json, Value};

    let client = get_client_with_proxy().await?;

    // 豆包API端点
    let endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

    // 获取模型名称，使用支持视觉的模型
    let model_name = custom_model_name.unwrap_or("doubao-1.5-vision-pro-32k");

    // 构建消息内容数组
    let mut content_parts = Vec::new();

    // 添加文本内容
    if !text_message.trim().is_empty() {
        content_parts.push(json!({
            "type": "text",
            "text": text_message
        }));
    }

    // 添加图片内容
    for (filename, file_data) in image_files {
        // 将图片数据转换为Base64
        let base64_image = general_purpose::STANDARD.encode(&file_data);

        // 检测图片格式
        let image_format = match detect_image_format(&filename, &file_data) {
            Ok(format) => format,
            Err(e) => {
                return Err(format!("图片格式检测失败: {}", e));
            }
        };

        // 构建Base64 URL
        let image_url = format!("data:image/{};base64,{}", image_format, base64_image);

        content_parts.push(json!({
            "type": "image_url",
            "image_url": {
                "url": image_url
            }
        }));
    }

    let payload = json!({
        "model": model_name,
        "messages": [
            {
                "role": "user",
                "content": content_parts
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
        .map_err(|e| format!("豆包视觉API请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("豆包视觉API返回错误 {}: {}", status, error_text));
    }

    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析豆包视觉API响应失败: {}", e))?;

    // 提取响应内容
    if let Some(choices) = response_json["choices"].as_array() {
        if let Some(first_choice) = choices.first() {
            if let Some(content) = first_choice["message"]["content"].as_str() {
                return Ok(content.to_string());
            }
        }
    }

    Err("无法从豆包视觉API响应中提取内容".to_string())
}

// 豆包视觉理解 - 支持图片输入的流式对话
pub async fn doubao_stream_chat_with_images(
    api_key: String,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>, // (文件名, 文件数据)
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    use futures::StreamExt;
    use serde_json::{json, Value};

    println!("启动豆包视觉流式请求");

    let client = get_client_with_proxy().await?;

    // 豆包API端点
    let endpoint = "https://ark.cn-beijing.volces.com/api/v3/chat/completions";

    // 获取模型名称，使用支持视觉的模型
    let model_name = custom_model_name.unwrap_or("doubao-1.5-vision-pro-32k");

    // 构建消息内容数组
    let mut content_parts = Vec::new();

    // 添加文本内容
    if !text_message.trim().is_empty() {
        content_parts.push(json!({
            "type": "text",
            "text": text_message
        }));
    }

    // 添加图片内容
    for (filename, file_data) in image_files {
        // 将图片数据转换为Base64
        let base64_image = general_purpose::STANDARD.encode(&file_data);

        // 检测图片格式
        let image_format = match detect_image_format(&filename, &file_data) {
            Ok(format) => format,
            Err(e) => {
                return Err(format!("图片格式检测失败: {}", e));
            }
        };

        // 构建Base64 URL
        let image_url = format!("data:image/{};base64,{}", image_format, base64_image);

        content_parts.push(json!({
            "type": "image_url",
            "image_url": {
                "url": image_url
            }
        }));
    }

    let payload = json!({
        "model": model_name,
        "messages": [
            {
                "role": "user",
                "content": content_parts
            }
        ],
        "temperature": 0.7,
        "max_tokens": 2000,
        "stream": true
    });

    let response = client
        .post(endpoint)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("豆包视觉流式API请求失败: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        let (tx, rx) = mpsc::channel::<Result<String, String>>(1);
        let error_msg = format_ai_error(&format!("豆包视觉API返回错误 {}: {}", status, error_text));
        let _ = tx.send(Ok(error_msg)).await;
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        return Ok(Box::pin(stream) as TextStream);
    }

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理流式响应
    tokio::spawn(async move {
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // 处理SSE格式的数据
                    while let Some(line_end) = buffer.find('\n') {
                        let line = buffer[..line_end].trim().to_string();
                        buffer = buffer[line_end + 1..].to_string();

                        if line.starts_with("data: ") {
                            let data = &line[6..]; // 移除 "data: " 前缀

                            if data == "[DONE]" {
                                break;
                            }

                            // 尝试解析JSON
                            if let Ok(json_data) = serde_json::from_str::<Value>(data) {
                                if let Some(choices) = json_data["choices"].as_array() {
                                    if let Some(first_choice) = choices.first() {
                                        if let Some(delta) = first_choice.get("delta") {
                                            if let Some(content) = delta["content"].as_str() {
                                                if !content.is_empty() {
                                                    let _ = tx.send(Ok(content.to_string())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    let error_msg = format_ai_error(&format!("豆包视觉流式响应错误: {}", e));
                    let _ = tx.send(Ok(error_msg)).await;
                    break;
                }
            }
        }
    });

    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}
