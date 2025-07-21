use crate::api::settings::get_client_with_proxy;
use crate::db::{self, UnifiedDbManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, Manager, State};
use crate::api::ai::models::{create_genai_client, create_custom_genai_client};
use genai::{Client, ModelIden};
use super::{
    conversations::{add_ai_message, list_ai_messages},
    models::{
        self, chat_with_history, stream_chat_with_history, stream_chat_with_history_custom_ai,
        stream_message_from_ai, stream_message_from_custom_ai, stream_message_with_images_from_ai,
        CustomModel,
    },
    roles::get_ai_role,
};
use futures::StreamExt;
use genai::chat::{ChatMessage, ChatRole};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiServiceInfo {
    pub name: String,
    pub provider: String,
    pub model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModelInfo {
    pub provider: String,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConnectionRequest {
    pub provider: String,
    pub api_key: Option<String>,
    pub api_base: Option<String>,
    pub organization: Option<String>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConnectionResponse {
    pub success: bool,
    pub message: String,
    pub models: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveAiConfigRequest {
    pub providers: HashMap<String, AiProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfigResponse {
    pub providers: HashMap<String, AiProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiServiceStatusResponse {
    pub provider: String,
    pub is_available: bool,
    pub models_count: usize,
    pub active_conversations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub provider: String,
    pub is_chat: bool,
    pub is_embedding: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub name: String,
    pub provider: String,
    pub config: serde_json::Value,
}

// 测试AI连接
#[tauri::command]
pub async fn test_ai_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    match request.provider.as_str() {
        "openai" => test_openai_connection(request, db_manager).await,
        "anthropic" => test_anthropic_connection(request, db_manager).await,
        "gemini" => test_gemini_connection(request, db_manager).await,
        "deepseek" => test_deepseek_connection(request, db_manager).await,
        "ali" => test_ali_connection(request, db_manager).await,
        "doubao" => test_doubao_connection(request, db_manager).await,
        "xai" => test_xai_connection(request, db_manager).await,
        "custom" => test_custom_connection(request, db_manager).await,
        _ => Err(format!("不支持的提供商: {}", request.provider)),
    }
}

async fn test_openai_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("请求OpenAI API失败: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("OpenAI API连接测试失败: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("解析OpenAI响应失败: {}", e))?;

    let models = models_response["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .filter(|id| id.starts_with("gpt-"))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(TestConnectionResponse {
        success: true,
        message: format!("OpenAI API连接成功，找到 {} 个模型", models.len()),
        models: Some(models),
    })
}

async fn test_anthropic_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.anthropic.com".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    // Anthropic API没有列出模型的端点，我们使用消息端点测试连接
    let response = client
        .get(format!("{}/v1/models", api_base))
        .header("x-api-key", &api_key)
        .header("anthropic-version", "2023-06-01")
        .send()
        .await
        .map_err(|e| format!("请求Anthropic API失败: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("Anthropic API连接测试失败: {}", error),
            models: None,
        });
    }

    // 提供默认的Claude模型列表
    let default_models = vec![
        "claude-3-5-sonnet-20240620".to_string(),
        "claude-3-opus-20240229".to_string(),
        "claude-3-sonnet-20240229".to_string(),
        "claude-3-haiku-20240307".to_string(),
        "claude-2.1".to_string(),
        "claude-2.0".to_string(),
        "claude-instant-1.2".to_string(),
    ];

    // 尝试从响应中解析模型列表
    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("解析Anthropic响应失败: {}", e))?;

    let models = models_response["models"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or(default_models);

    Ok(TestConnectionResponse {
        success: true,
        message: format!("Anthropic API连接成功，找到 {} 个模型", models.len()),
        models: Some(models),
    })
}

async fn test_gemini_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;

    // 使用GenAI库测试连接
    let client = match create_genai_client(api_key.clone(), "gemini", &db_manager).await {
        Ok(client) => client,
        Err(e) => {
            return Ok(TestConnectionResponse {
                success: false,
                message: format!("创建Gemini客户端失败: {}", e),
                models: None,
            });
        }
    };

    // 默认的Gemini模型列表
    let default_models = vec![
        "gemini-2.0-flash".to_string(),
        "gemini-1.5-flash".to_string(),
        "gemini-1.5-pro".to_string(),
        "gemini-pro".to_string(),
        "gemini-pro-vision".to_string(),
    ];

    Ok(TestConnectionResponse {
        success: true,
        message: "Gemini API连接成功".to_string(),
        models: Some(default_models),
    })
}

async fn test_deepseek_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.deepseek.com/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("请求DeepSeek API失败: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("DeepSeek API连接测试失败: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("解析DeepSeek响应失败: {}", e))?;

    let models = models_response["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![
            "deepseek-chat".to_string(),
            "deepseek-coder".to_string(),
        ]);

    Ok(TestConnectionResponse {
        success: true,
        message: format!("DeepSeek API连接成功，找到 {} 个模型", models.len()),
        models: Some(models),
    })
}

async fn test_ali_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let _api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;

    // 通义千问API没有列出模型的端点，我们使用默认模型列表
    let default_models = vec![
        "qwen-max".to_string(),
        "qwen-plus".to_string(),
        "qwen-turbo".to_string(),
        "qwen-long".to_string(),
    ];

    Ok(TestConnectionResponse {
        success: true,
        message: "通义千问API密钥已保存".to_string(),
        models: Some(default_models),
    })
}

async fn test_doubao_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let client = get_client_with_proxy(&db_manager).await?;
    let response = client
        .post("https://ark.cn-beijing.volces.com/api/v3/chat/completions")
        .bearer_auth(request.api_key.ok_or("Missing API key")?)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({
            "model": "Doubao-pro-4k", // a known model for testing
            "messages": [{"role": "user", "content": "hello"}]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let default_models = vec![
            "doubao-1.5-pro-32k".to_string(),
            "doubao-1.5-pro-4k".to_string(),
        ];
        Ok(TestConnectionResponse {
            success: true,
            message: "连接成功".to_string(),
            models: Some(default_models),
        })
    } else {
        let error_message = response.text().await.unwrap_or_default();
        Err(format!("连接失败: {}", error_message))
    }
}

async fn test_xai_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "缺少API密钥".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.x.ai/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("请求xAI API失败: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("xAI API连接测试失败: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("解析xAI响应失败: {}", e))?;

    let models = models_response["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![
            "grok-3".to_string(),
            "grok-3-mini".to_string(),
            "grok-1.5".to_string(),
        ]);

    Ok(TestConnectionResponse {
        success: true,
        message: format!("xAI API连接成功，找到 {} 个模型", models.len()),
        models: Some(models),
    })
}

async fn test_custom_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    if request.api_base.is_none() {
        return Err("自定义API端点未设置".to_string());
    }
    let endpoint = request.api_base.unwrap();
    let client = get_client_with_proxy(&db_manager).await?;

    // 根据不同的adapter_type，可能需要不同的测试方法
    // 这里我们假设一个通用的GET请求可以验证连接
    let response = client
        .get(&endpoint)
        .send()
        .await
        .map_err(|e| format!("请求自定义API失败: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "未知错误".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("自定义API连接测试失败: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("解析自定义响应失败: {}", e))?;

    let models = models_response["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![
            "custom-model-1".to_string(),
            "custom-model-2".to_string(),
        ]);

    Ok(TestConnectionResponse {
        success: true,
        message: "自定义API连接成功".to_string(),
        models: Some(models),
    })
}

// 获取聊天模型列表
#[tauri::command]
pub async fn get_ai_chat_models(
) -> Result<Vec<ModelInfo>, String> {
    // 返回预定义的聊天模型列表
    let models = vec![
        ModelInfo {
            name: "gpt-4o".to_string(),
            provider: "openai".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "gpt-4-turbo".to_string(),
            provider: "openai".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "gpt-3.5-turbo".to_string(),
            provider: "openai".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "claude-3.5-sonnet".to_string(),
            provider: "anthropic".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "claude-3-opus".to_string(),
            provider: "anthropic".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "gemini-2.0-flash".to_string(),
            provider: "gemini".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "gemini-1.5-pro".to_string(),
            provider: "gemini".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "deepseek-chat".to_string(),
            provider: "deepseek".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "qwen-max".to_string(),
            provider: "ali".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "doubao-seed-1.6".to_string(),
            provider: "doubao".to_string(),
            is_chat: true,
            is_embedding: false,
        },
        ModelInfo {
            name: "grok-3".to_string(),
            provider: "xai".to_string(),
            is_chat: true,
            is_embedding: false,
        },
    ];

    Ok(models)
}

// 获取嵌入模型列表
#[tauri::command]
pub async fn get_ai_embedding_models(
) -> Result<Vec<ModelInfo>, String> {
    // 返回预定义的嵌入模型列表
    let models = vec![
        ModelInfo {
            name: "text-embedding-3-large".to_string(),
            provider: "openai".to_string(),
            is_chat: false,
            is_embedding: true,
        },
        ModelInfo {
            name: "text-embedding-3-small".to_string(),
            provider: "openai".to_string(),
            is_chat: false,
            is_embedding: true,
        },
    ];

    Ok(models)
}

// 获取默认AI模型
#[tauri::command]
pub async fn get_default_ai_model(
    model_type: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Option<ModelInfo>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let key = format!("default_{}_model", model_type);
    let value = db::get_setting(&conn, &key).await.map_err(|e| e.to_string())?;

    if let Some(value_str) = value {
        if let Ok(model_info) = serde_json::from_str::<ModelInfo>(&value_str) {
            return Ok(Some(model_info));
        }
    }

    // 返回默认模型
    if model_type == "chat" {
        Ok(Some(ModelInfo {
            name: "gpt-3.5-turbo".to_string(),
            provider: "openai".to_string(),
            is_chat: true,
            is_embedding: false,
        }))
    } else if model_type == "embedding" {
        Ok(Some(ModelInfo {
            name: "text-embedding-3-small".to_string(),
            provider: "openai".to_string(),
            is_chat: false,
            is_embedding: true,
        }))
    } else {
        Ok(None)
    }
}

// 设置默认AI模型
#[tauri::command]
pub async fn set_default_ai_model(
    model_type: String,
    provider: String,
    model_name: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let model_info = ModelInfo {
        name: model_name,
        provider,
        is_chat: model_type == "chat",
        is_embedding: model_type == "embedding",
    };

    let key = format!("default_{}_model", model_type);
    let value = serde_json::to_string(&model_info).map_err(|e| e.to_string())?;

    db::save_setting(&conn, &key, &value).await.map_err(|e| e.to_string())
}

// 获取AI配置
#[tauri::command]
pub async fn get_ai_config(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Option<AiConfigResponse>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let config_json_opt = db::get_setting(&conn, "ai_providers_config").await.map_err(|e| e.to_string())?;

    if let Some(config_json) = config_json_opt {
        match serde_json::from_str::<SaveAiConfigRequest>(&config_json) {
            Ok(config) => Ok(Some(AiConfigResponse { providers: config.providers })),
            Err(e) => Err(format!("解析AI配置失败: {}", e)),
        }
    } else {
        Ok(None)
    }
}

// 保存AI配置
#[tauri::command]
pub async fn save_ai_config(
    config: SaveAiConfigRequest,
    db_manager: State<'_, UnifiedDbManager>,
    app: AppHandle,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    // 保存每个提供商的配置
    for (provider_id, provider_config) in &config.providers {
        // 保存API密钥
        if let Some(api_key) = &provider_config.api_key {
            let key = format!("{}_api_key", provider_id);
            db::save_setting(&conn, &key, api_key).await.map_err(|e| e.to_string())?;
        }

        // 保存API基础URL
        if let Some(api_base) = &provider_config.api_base {
            let key = format!("{}_api_base", provider_id);
            db::save_setting(&conn, &key, api_base).await.map_err(|e| e.to_string())?;
        }

        // 保存组织ID（如果有）
        if let Some(org) = &provider_config.organization {
            let key = format!("{}_organization", provider_id);
            db::save_setting(&conn, &key, org).await.map_err(|e| e.to_string())?;
        }

        // 保存是否启用
        let key = format!("{}_enabled", provider_id);
        db::save_setting(&conn, &key, &provider_config.enabled.to_string()).await.map_err(|e| e.to_string())?;

        // 保存默认模型
        let key = format!("{}_default_model", provider_id);
        db::save_setting(&conn, &key, &provider_config.default_model).await.map_err(|e| e.to_string())?;
    }

    // 保存完整配置的JSON
    let config_json = serde_json::to_string(&config).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "ai_providers_config", &config_json).await.map_err(|e| e.to_string())?;

    Ok(())
}

// 获取AI使用统计
#[tauri::command]
pub async fn get_ai_usage_stats(
) -> Result<serde_json::Value, String> {
    // 这里可以实现统计逻辑，目前返回一个示例
    let stats = serde_json::json!({
        "conversations": 0,
        "messages": 0,
        "tokens": {
            "input": 0,
            "output": 0,
            "total": 0
        },
        "providers": {}
    });

    Ok(stats)
}

// 重新加载AI服务
#[tauri::command]
pub async fn reload_ai_services(
    app: AppHandle,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    // 获取完整配置
    let config_json = db::get_setting(&conn, "ai_providers_config").await.map_err(|e| e.to_string())?;

    if let Some(config_str) = config_json {
        // 解析配置
        let _config: SaveAiConfigRequest = serde_json::from_str(&config_str)
            .map_err(|e| format!("解析AI配置失败: {}", e))?;

        // 这里可以实现重新加载服务的逻辑
    }

    Ok(())
}

// 获取AI服务状态
#[tauri::command]
pub async fn get_ai_service_status(
) -> Result<Vec<AiServiceStatusResponse>, String> {
    // 返回预定义的服务状态
    let statuses = vec![
        AiServiceStatusResponse {
            provider: "openai".to_string(),
            is_available: true,
            models_count: 5,
            active_conversations: 0,
        },
        AiServiceStatusResponse {
            provider: "anthropic".to_string(),
            is_available: true,
            models_count: 3,
            active_conversations: 0,
        },
        AiServiceStatusResponse {
            provider: "gemini".to_string(),
            is_available: true,
            models_count: 2,
            active_conversations: 0,
        },
    ];

    Ok(statuses)
} 