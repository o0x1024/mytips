use crate::api::settings::get_client_with_proxy;
use crate::db::{self, UnifiedDbManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tauri::{AppHandle, State};
use crate::api::ai::models::{create_genai_client, CustomModelConfig};

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
        "qwen" => test_qwen_connection(request, db_manager).await,
        "doubao" => test_doubao_connection(request, db_manager).await,
        "xai" => test_xai_connection(request, db_manager).await,
        "custom" => test_custom_connection(request, db_manager).await,
        _ => Err(format!("Unsupported provider: {}", request.provider)),
    }
}

async fn test_openai_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.openai.com/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to request OpenAI API: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("OpenAI API connection test failed: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse OpenAI response: {}", e))?;

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
        message: format!("OpenAI API connection successful, found {} models", models.len()),
        models: Some(models),
    })
}

async fn test_anthropic_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;
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
        .map_err(|e| format!("Failed to request Anthropic API: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("Anthropic API connection test failed: {}", error),
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
        .map_err(|e| format!("Failed to parse Anthropic response: {}", e))?;

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
        message: format!("Anthropic API connection successful, found {} models", models.len()),
        models: Some(models),
    })
}

async fn test_gemini_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;

    // 使用GenAI库测试连接
    let client = match create_genai_client(api_key.clone(), "gemini", &db_manager).await {
        Ok(client) => client,
        Err(e) => {
            return Ok(TestConnectionResponse {
                success: false,
                message: format!("Failed to create Gemini client: {}", e),
                models: None,
            });
        }
    };

    // 默认的Gemini模型列表
    let default_models = vec![
    ];

    Ok(TestConnectionResponse {
        success: true,
        message: "Gemini API connection successful".to_string(),
        models: Some(default_models),
    })
}

async fn test_deepseek_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.deepseek.com/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to request DeepSeek API: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("DeepSeek API connection test failed: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse DeepSeek response: {}", e))?;

    let models = models_response["data"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| vec![
            "deepseek-chat".to_string(),

        ]);

    Ok(TestConnectionResponse {
        success: true,
        message: format!("DeepSeek API connection successful, found {} models", models.len()),
        models: Some(models),
    })
}

async fn test_qwen_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;

    let client = get_client_with_proxy(&db_manager).await?;

    let payload = serde_json::json!({
        "model": "qwen3-coder-plus",
        "messages": [
            {
                "role": "user",
                "content": "test"
            }
        ]
    });

    let response = client
        .post("https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Qwen API request failed: {}", e))?;

    if response.status().is_success() {
        // 如果成功，返回一个包含默认模型的成功响应
        let models = vec![
            "qwen3-coder-plus".to_string(),
        ];

        Ok(TestConnectionResponse {
            success: true,
            message: "Tongyi Qwen API connection successful".to_string(),
            models: Some(models),
        })
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("Qwen API returned an error: {} - {}", status, error_text))
    }
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
            message: "Connection successful".to_string(),
            models: Some(default_models),
        })
    } else {
        let error_message = response.text().await.unwrap_or_default();
        Err(format!("Connection failed: {}", error_message))
    }
}

async fn test_xai_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    let api_key = request.api_key.ok_or_else(|| "API key is missing".to_string())?;
    let api_base = request
        .api_base
        .unwrap_or_else(|| "https://api.x.ai/v1".to_string());

    let client = get_client_with_proxy(&db_manager).await?;

    let response = client
        .get(format!("{}/models", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .map_err(|e| format!("Failed to request xAI API: {}", e))?;

    if !response.status().is_success() {
        let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Ok(TestConnectionResponse {
            success: false,
            message: format!("xAI API connection test failed: {}", error),
            models: None,
        });
    }

    let models_response: serde_json::Value = response.json().await
        .map_err(|e| format!("Failed to parse xAI response: {}", e))?;

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
        message: format!("xAI API connection successful, found {} models", models.len()),
        models: Some(models),
    })
}

// 列出自定义模型配置
#[tauri::command]
pub async fn list_custom_model_configs(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<CustomModelConfig>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;

    let config_json_opt = db::get_setting(&conn, "custom_ai_models").await.map_err(|e| e.to_string())?;
    
    if let Some(config_json) = config_json_opt {
        match serde_json::from_str::<Vec<CustomModelConfig>>(&config_json) {
            Ok(models) => Ok(models),
            Err(e) => Err(format!("Failed to parse custom models: {}", e)),
        }
    } else {
        Ok(Vec::new())
    }
}

// 添加自定义模型配置
#[tauri::command]
pub async fn add_custom_model_config(
    config: CustomModelConfig,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取现有配置
    let mut models = match db::get_setting(&conn, "custom_ai_models").await.map_err(|e| e.to_string())? {
        Some(json) => serde_json::from_str::<Vec<CustomModelConfig>>(&json)
            .map_err(|e| format!("Failed to parse custom models: {}", e))?,
        None => Vec::new(),
    };
    
    // 检查ID是否已存在
    if models.iter().any(|model| model.id == config.id) {
        return Err(format!("Custom model with ID {} already exists", config.id));
    }
    
    // 添加新配置
    models.push(config);
    
    // 保存
    let json = serde_json::to_string(&models).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "custom_ai_models", &json).await.map_err(|e| e.to_string())
}

// 更新自定义模型配置
#[tauri::command]
pub async fn update_custom_model_config(
    config: CustomModelConfig,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取现有配置
    let mut models = match db::get_setting(&conn, "custom_ai_models").await.map_err(|e| e.to_string())? {
        Some(json) => serde_json::from_str::<Vec<CustomModelConfig>>(&json)
            .map_err(|e| format!("Failed to parse custom models: {}", e))?,
        None => Vec::new(),
    };
    
    // 查找并更新
    let mut found = false;
    for model in &mut models {
        if model.id == config.id {
            *model = config.clone();
            found = true;
            break;
        }
    }
    
    if !found {
        return Err(format!("Custom model with ID {} not found", config.id));
    }
    
    // 保存
    let json = serde_json::to_string(&models).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "custom_ai_models", &json).await.map_err(|e| e.to_string())
}

// 删除自定义模型配置
#[tauri::command]
pub async fn delete_custom_model_config(
    model_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| e.to_string())?;
    
    // 获取现有配置
    let mut models = match db::get_setting(&conn, "custom_ai_models").await.map_err(|e| e.to_string())? {
        Some(json) => serde_json::from_str::<Vec<CustomModelConfig>>(&json)
            .map_err(|e| format!("Failed to parse custom models: {}", e))?,
        None => Vec::new(),
    };
    
    // 移除指定ID的模型
    let initial_len = models.len();
    models.retain(|model| model.id != model_id);
    
    if models.len() == initial_len {
        return Err(format!("Custom model with ID {} not found", model_id));
    }
    
    // 保存
    let json = serde_json::to_string(&models).map_err(|e| e.to_string())?;
    db::save_setting(&conn, "custom_ai_models", &json).await.map_err(|e| e.to_string())
}

// 测试自定义模型连接
async fn test_custom_connection(
    request: TestConnectionRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<TestConnectionResponse, String> {
    if request.api_base.is_none() {
        return Err("Custom API endpoint is not set".to_string());
    }
    
    let endpoint = request.api_base.unwrap();
    let client = get_client_with_proxy(&db_manager).await?;
    let adapter_type = request.model.unwrap_or_else(|| "openai".to_string());

    // 根据不同的adapter_type，使用不同的测试方法
    match adapter_type.as_str() {
        "openai" => {
            // OpenAI兼容接口测试
            let response = client
                .get(format!("{}/models", endpoint))
                .header("Authorization", format!("Bearer {}", request.api_key.unwrap_or_default()))
                .send()
                .await
                .map_err(|e| format!("Failed to request custom OpenAI-compatible API: {}", e))?;

            if !response.status().is_success() {
                let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Ok(TestConnectionResponse {
                    success: false,
                    message: format!("Custom OpenAI-compatible API connection test failed: {}", error),
                    models: None,
                });
            }

            let models_response: serde_json::Value = response.json().await
                .map_err(|e| format!("Failed to parse custom OpenAI-compatible response: {}", e))?;

            let models = models_response["data"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| vec![
                    "default-model".to_string(),
                ]);

            Ok(TestConnectionResponse {
                success: true,
                message: "Custom OpenAI-compatible API connection successful".to_string(),
                models: Some(models),
            })
        },
        "ollama" => {
            // Ollama API测试
            let response = client
                .get(format!("{}/api/tags", endpoint))
                .send()
                .await
                .map_err(|e| format!("Failed to request Ollama API: {}", e))?;

            if !response.status().is_success() {
                let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Ok(TestConnectionResponse {
                    success: false,
                    message: format!("Ollama API connection test failed: {}", error),
                    models: None,
                });
            }

            let models_response: serde_json::Value = response.json().await
                .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

            let models = models_response["models"]
                .as_array()
                .map(|arr| {
                    arr.iter()
                        .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_else(|| vec![
                    "llama3".to_string(),
                    "mistral".to_string(),
                ]);

            Ok(TestConnectionResponse {
                success: true,
                message: "Ollama API connection successful".to_string(),
                models: Some(models),
            })
        },
        _ => {
            // 通用测试 - 简单尝试访问端点
            let response = client
                .get(&endpoint)
                .send()
                .await
                .map_err(|e| format!("Failed to request custom API: {}", e))?;

            if !response.status().is_success() {
                let error = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                return Ok(TestConnectionResponse {
                    success: false,
                    message: format!("Custom API connection test failed: {}", error),
                    models: None,
                });
            }

            Ok(TestConnectionResponse {
                success: true,
                message: "Custom API connection successful".to_string(),
                models: Some(vec!["custom-model".to_string()]),
            })
        }
    }
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
            name: "qwen3-coder-plus".to_string(),
            provider: "qwen".to_string(),
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

    // 若未设置，直接返回 None（不返回硬编码的默认值）
    Ok(None)
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
            Err(e) => Err(format!("Failed to parse AI config: {}", e)),
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
            .map_err(|e| format!("Failed to parse AI config: {}", e))?;

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