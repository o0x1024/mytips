use crate::api::settings::get_client_with_proxy;
use futures::Stream;
use futures_util::stream::StreamExt;
use serde_json::{json, Value};
use std::pin::Pin;
use tokio::sync::mpsc;

// Grok API的基础URL
const GROK_BASE_URL: &str = "https://api.x.ai/v1";

// 流式API响应类型
pub type TextStream = Pin<Box<dyn Stream<Item = Result<String, String>> + Send>>;

// 发送消息到Grok并获取响应
pub async fn send_to_grok(
    api_key: String,
    message: String,
    custom_model_name: Option<&str>,
) -> Result<String, String> {
    let client = get_client_with_proxy().await?;

    // 获取模型名称，默认使用grok-beta
    let model = custom_model_name.unwrap_or("grok-beta");

    // 构建请求体
    let request_body = json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "temperature": 0.7,
        "max_tokens": 2048
    });

    // 发送请求
    let response = client
        .post(&format!("{}/chat/completions", GROK_BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("发送请求到Grok失败: {}", e))?;

    // 检查响应状态
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Grok API错误 {}: {}", status, error_text));
    }

    // 解析响应
    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析Grok响应失败: {}", e))?;

    // 提取内容
    let content = response_json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .ok_or("无法从Grok响应中提取内容")?;

    Ok(content.to_string())
}

// 流式发送消息到Grok
pub async fn stream_from_grok(
    api_key: String,
    message: String,
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    println!("启动Grok流式请求");

    let client = get_client_with_proxy().await?;

    // 获取模型名称，默认使用grok-beta
    let model = custom_model_name.unwrap_or("grok-beta");

    // 构建请求体
    let request_body = json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ],
        "temperature": 0.7,
        "max_tokens": 2048,
        "stream": true
    });

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理流式响应
    tokio::spawn(async move {
        // 发送流式请求
        let response = match client
            .post(&format!("{}/chat/completions", GROK_BASE_URL))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                let _ = tx.send(Ok(format!("连接Grok失败: {}", e))).await;
                return;
            }
        };

        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let _ = tx
                .send(Ok(format!("Grok API错误 {}: {}", status, error_text)))
                .await;
            return;
        }

        // 处理流式响应
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    // 将字节转换为字符串
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // 处理SSE格式的数据
                    let lines: Vec<&str> = buffer.lines().collect();
                    for line in &lines[..lines.len().saturating_sub(1)] {
                        if line.starts_with("data: ") {
                            let data_part = &line[6..]; // 移除 "data: " 前缀

                            if data_part == "[DONE]" {
                                return; // 流结束
                            }

                            // 尝试解析JSON
                            if let Ok(json_data) = serde_json::from_str::<Value>(data_part) {
                                if let Some(content) = json_data
                                    .get("choices")
                                    .and_then(|choices| choices.get(0))
                                    .and_then(|choice| choice.get("delta"))
                                    .and_then(|delta| delta.get("content"))
                                    .and_then(|content| content.as_str())
                                {
                                    if !content.is_empty() {
                                        let _ = tx.send(Ok(content.to_string())).await;
                                    }
                                }
                            }
                        }
                    }

                    // 保留最后一行（可能不完整）
                    if let Some(last_line) = lines.last() {
                        buffer = last_line.to_string();
                    } else {
                        buffer.clear();
                    }
                }
                Err(e) => {
                    let _ = tx.send(Ok(format!("流式读取错误: {}", e))).await;
                    break;
                }
            }
        }
    });

    // 将接收器转换为流
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// 带历史记录的Grok聊天
pub async fn grok_chat_with_history(
    api_key: String,
    messages: Vec<serde_json::Value>,
    custom_model_name: Option<&str>,
) -> Result<String, String> {
    let client = get_client_with_proxy().await?;

    // 获取模型名称，默认使用grok-beta
    let model = custom_model_name.unwrap_or("grok-beta");

    // 转换消息格式
    let formatted_messages: Vec<Value> = messages
        .into_iter()
        .filter_map(|msg| {
            let role = msg.get("role")?.as_str()?;
            let content = msg.get("content")?.as_str()?;
            Some(json!({
                "role": role,
                "content": content
            }))
        })
        .collect();

    // 构建请求体
    let request_body = json!({
        "model": model,
        "messages": formatted_messages,
        "temperature": 0.7,
        "max_tokens": 2048
    });

    // 发送请求
    let response = client
        .post(&format!("{}/chat/completions", GROK_BASE_URL))
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("发送请求到Grok失败: {}", e))?;

    // 检查响应状态
    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Grok API错误 {}: {}", status, error_text));
    }

    // 解析响应
    let response_json: Value = response
        .json()
        .await
        .map_err(|e| format!("解析Grok响应失败: {}", e))?;

    // 提取内容
    let content = response_json
        .get("choices")
        .and_then(|choices| choices.get(0))
        .and_then(|choice| choice.get("message"))
        .and_then(|message| message.get("content"))
        .and_then(|content| content.as_str())
        .ok_or("无法从Grok响应中提取内容")?;

    Ok(content.to_string())
}

// 带历史记录的Grok流式聊天
pub async fn grok_stream_chat_with_history(
    api_key: String,
    messages: Vec<serde_json::Value>,
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    println!("启动Grok历史记录流式请求");

    let client = get_client_with_proxy().await?;

    // 获取模型名称，默认使用grok-beta
    let model = custom_model_name.unwrap_or("grok-beta");

    // 转换消息格式
    let formatted_messages: Vec<Value> = messages
        .into_iter()
        .filter_map(|msg| {
            let role = msg.get("role")?.as_str()?;
            let content = msg.get("content")?.as_str()?;
            Some(json!({
                "role": role,
                "content": content
            }))
        })
        .collect();

    // 构建请求体
    let request_body = json!({
        "model": model,
        "messages": formatted_messages,
        "temperature": 0.7,
        "max_tokens": 2048,
        "stream": true
    });

    // 创建mpsc通道
    let (tx, rx) = mpsc::channel::<Result<String, String>>(100);

    // 在后台任务中处理流式响应
    tokio::spawn(async move {
        // 发送流式请求
        let response = match client
            .post(&format!("{}/chat/completions", GROK_BASE_URL))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
        {
            Ok(response) => response,
            Err(e) => {
                let _ = tx.send(Ok(format!("连接Grok失败: {}", e))).await;
                return;
            }
        };

        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            let _ = tx
                .send(Ok(format!("Grok API错误 {}: {}", status, error_text)))
                .await;
            return;
        }

        // 处理流式响应
        let mut stream = response.bytes_stream();
        let mut buffer = String::new();

        while let Some(chunk_result) = stream.next().await {
            match chunk_result {
                Ok(chunk) => {
                    // 将字节转换为字符串
                    let chunk_str = String::from_utf8_lossy(&chunk);
                    buffer.push_str(&chunk_str);

                    // 处理SSE格式的数据
                    let lines: Vec<&str> = buffer.lines().collect();
                    for line in &lines[..lines.len().saturating_sub(1)] {
                        if line.starts_with("data: ") {
                            let data_part = &line[6..]; // 移除 "data: " 前缀

                            if data_part == "[DONE]" {
                                return; // 流结束
                            }

                            // 尝试解析JSON
                            if let Ok(json_data) = serde_json::from_str::<Value>(data_part) {
                                if let Some(content) = json_data
                                    .get("choices")
                                    .and_then(|choices| choices.get(0))
                                    .and_then(|choice| choice.get("delta"))
                                    .and_then(|delta| delta.get("content"))
                                    .and_then(|content| content.as_str())
                                {
                                    if !content.is_empty() {
                                        // println!("Grok流式响应: {}", content);
                                        let _ = tx.send(Ok(content.to_string())).await;
                                    }
                                }
                            }
                        }
                    }

                    // 保留最后一行（可能不完整）
                    if let Some(last_line) = lines.last() {
                        buffer = last_line.to_string();
                    } else {
                        buffer.clear();
                    }
                }
                Err(e) => {
                    let _ = tx.send(Ok(format!("流式读取错误: {}", e))).await;
                    break;
                }
            }
        }
    });

    // 将接收器转换为流
    let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
    Ok(Box::pin(stream) as TextStream)
}

// Grok带图片的聊天（目前Grok-beta不支持图片，但为将来的vision模型做准备）
pub async fn grok_chat_with_images(
    api_key: String,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>,
    custom_model_name: Option<&str>,
) -> Result<String, String> {
    // 目前Grok-beta不支持图片，返回错误信息
    Err("Grok当前不支持图片处理，请使用其他支持图片的AI模型".to_string())
}

// Grok流式图片聊天（为将来的vision模型做准备）
pub async fn grok_stream_chat_with_images(
    api_key: String,
    text_message: String,
    image_files: Vec<(String, Vec<u8>)>,
    custom_model_name: Option<&str>,
) -> Result<TextStream, String> {
    // 目前Grok-beta不支持图片，返回错误信息
    Err("Grok当前不支持图片处理，请使用其他支持图片的AI模型".to_string())
}
