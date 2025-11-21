use crate::api::settings::get_client_with_proxy;
use crate::db::UnifiedDbManager;
use anyhow::{anyhow, Result};
use futures_util::StreamExt;
use rig::completion::Prompt;
use rig::providers::{anthropic, gemini, openai};
use rig::client::CompletionClient;
use serde_json::json;
use std::pin::Pin;
use futures::Stream;

/// Message structure for chat conversations
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: String) -> Self {
        Self {
            role: "system".to_string(),
            content,
        }
    }

    pub fn user(content: String) -> Self {
        Self {
            role: "user".to_string(),
            content,
        }
    }

    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".to_string(),
            content,
        }
    }
}

/// Unified AI provider wrapper for rig-core
pub enum RigProvider {
    OpenAI(openai::Client),
    Anthropic(anthropic::Client),
    Gemini(gemini::Client),
    Custom(CustomProvider),
}

/// Custom provider for non-standard AI services
pub struct CustomProvider {
    pub endpoint: String,
    pub api_key: String,
    pub provider_type: String,
}

impl RigProvider {
    /// Create a new provider based on provider ID
    pub fn new(
        provider_id: &str,
        api_key: String,
        endpoint: Option<String>,
    ) -> Result<Self> {
        match provider_id {
            "openai" => {
                let client = openai::Client::new(&api_key);
                Ok(RigProvider::OpenAI(client))
            }
            "anthropic" => {
                let client = anthropic::Client::new(&api_key);
                Ok(RigProvider::Anthropic(client))
            }
            "gemini" => {
                let client = gemini::Client::new(&api_key);
                Ok(RigProvider::Gemini(client))
            }
            _ => {
                // For other providers (deepseek, qwen, doubao, ollama, groq, cohere, zhipu, etc.)
                Ok(RigProvider::Custom(CustomProvider {
                    endpoint: endpoint.unwrap_or_default(),
                    api_key,
                    provider_type: provider_id.to_string(),
                }))
            }
        }
    }

    /// Send a simple prompt and get response (non-streaming)
    pub async fn prompt(&self, model_name: &str, prompt_text: &str) -> Result<String> {
        match self {
            RigProvider::OpenAI(client) => {
                let agent = client.agent(model_name).build();
                let response = agent.prompt(prompt_text).await
                    .map_err(|e| anyhow!("OpenAI prompt failed: {}", e))?;
                Ok(response)
            }
            RigProvider::Anthropic(client) => {
                let agent = client.agent(model_name).build();
                let response = agent.prompt(prompt_text).await
                    .map_err(|e| anyhow!("Anthropic prompt failed: {}", e))?;
                Ok(response)
            }
            RigProvider::Gemini(client) => {
                let agent = client.agent(model_name).build();
                let response = agent.prompt(prompt_text).await
                    .map_err(|e| anyhow!("Gemini prompt failed: {}", e))?;
                Ok(response)
            }
            RigProvider::Custom(custom) => {
                // For custom providers, use HTTP request
                custom.send_request_simple(model_name, prompt_text).await
            }
        }
    }

    /// Send chat with message history (non-streaming)
    pub async fn chat(&self, model_name: &str, messages: Vec<ChatMessage>) -> Result<String> {
        match self {
            RigProvider::Custom(_custom) => {
                // For custom providers, we still need db_manager for HTTP client
                // This will be handled at a higher level with send_request instead
                Err(anyhow!("Custom provider chat requires db_manager context. Use send_request instead"))
            }
            _ => {
                // For rig-core native providers, convert to prompt format
                let combined_prompt = messages
                    .iter()
                    .map(|msg| format!("{}: {}", msg.role, msg.content))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                self.prompt(model_name, &combined_prompt).await
            }
        }
    }

    /// Stream a prompt response
    pub async fn prompt_stream(
        &self,
        _model_name: &str,
        _prompt_text: &str,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        match self {
            RigProvider::OpenAI(_) | RigProvider::Anthropic(_) | RigProvider::Gemini(_) => {
                // For standard providers, streaming needs to be handled via CustomProvider
                Err(anyhow!("Streaming not yet implemented for standard providers. Use CustomProvider instead."))
            }
            RigProvider::Custom(_) => {
                Err(anyhow!("Custom provider streaming requires special handling"))
            }
        }
    }
}

/// Custom provider implementation for non-standard AI services
impl CustomProvider {
    /// Send simple prompt request (for rig-core compatibility)
    pub async fn send_request_simple(
        &self,
        model_name: &str,
        prompt_text: &str,
    ) -> Result<String> {
        let messages_json = vec![
            json!({
                "role": "user",
                "content": prompt_text
            })
        ];

        let request_body = json!({
            "model": model_name,
            "messages": messages_json,
            "stream": false
        });

        let url = self.get_completion_url();
        let client = reqwest::Client::new();

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json");

        if !self.api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = request
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("API error {}: {}", status, error_text));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        self.extract_content(&response_json)
    }

    /// Send non-streaming request with db_manager
    pub async fn send_request(
        &self,
        model_name: &str,
        messages: Vec<ChatMessage>,
        db_manager: &UnifiedDbManager,
    ) -> Result<String> {
        let client = get_client_with_proxy(db_manager).await
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let messages_json: Vec<serde_json::Value> = messages
            .iter()
            .map(|msg| {
                json!({
                    "role": msg.role,
                    "content": msg.content
                })
            })
            .collect();

        let request_body = json!({
            "model": model_name,
            "messages": messages_json,
            "stream": false
        });

        let url = self.get_completion_url();

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json");

        if !self.api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = request
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("API error {}: {}", status, error_text));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| anyhow!("Failed to parse response: {}", e))?;

        self.extract_content(&response_json)
    }

    /// Send streaming request
    pub async fn send_stream_request(
        &self,
        model_name: &str,
        messages: Vec<ChatMessage>,
        db_manager: &UnifiedDbManager,
    ) -> Result<Pin<Box<dyn Stream<Item = Result<String>> + Send>>> {
        let client = get_client_with_proxy(db_manager).await
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

        let messages_json: Vec<serde_json::Value> = messages
            .iter()
            .map(|msg| {
                json!({
                    "role": msg.role,
                    "content": msg.content
                })
            })
            .collect();

        let request_body = json!({
            "model": model_name,
            "messages": messages_json,
            "stream": true
        });

        let url = self.get_completion_url();
        println!("[Stream] Provider: {}, URL: {}, Model: {}", self.provider_type, url, model_name);

        let mut request = client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream");

        if !self.api_key.is_empty() {
            request = request.header("Authorization", format!("Bearer {}", self.api_key));
        }

        let response = request
            .json(&request_body)
            .send()
            .await
            .map_err(|e| anyhow!("Stream request failed: builder error - {}. URL: {}, Provider: {}", e, url, self.provider_type))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("Stream API error {}: {}", status, error_text));
        }

        let stream = response.bytes_stream();
        let provider_type = self.provider_type.clone();
        
        let mapped_stream = stream.map(move |chunk_result| {
            match chunk_result {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    Self::parse_sse_chunk(&text, &provider_type)
                }
                Err(e) => Err(anyhow!("Stream error: {}", e)),
            }
        })
        .filter_map(|result| async move {
            match result {
                Ok(Some(content)) => Some(Ok(content)),
                Ok(None) => None,
                Err(e) => Some(Err(e)),
            }
        });

        Ok(Box::pin(mapped_stream))
    }

    fn get_completion_url(&self) -> String {
        match self.provider_type.as_str() {
            "deepseek" => format!("{}/chat/completions", self.endpoint.trim_end_matches('/')),
            "qwen" => format!("{}/chat/completions", self.endpoint.trim_end_matches('/')),
            "doubao" => format!("{}/chat/completions", self.endpoint.trim_end_matches('/')),
            "ollama" => format!("{}/api/chat", self.endpoint.trim_end_matches('/')),
            "groq" => format!("{}/openai/v1/chat/completions", self.endpoint.trim_end_matches('/')),
            "cohere" => format!("{}/v1/chat", self.endpoint.trim_end_matches('/')),
            "zhipu" => format!("{}/api/paas/v4/chat/completions", self.endpoint.trim_end_matches('/')),
            _ => format!("{}/chat/completions", self.endpoint.trim_end_matches('/')),
        }
    }

    fn extract_content(&self, response: &serde_json::Value) -> Result<String> {
        // Try OpenAI format (most common)
        if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            return Ok(content.to_string());
        }

        // Try Ollama format
        if let Some(content) = response["message"]["content"].as_str() {
            return Ok(content.to_string());
        }

        // Try Cohere format
        if let Some(content) = response["text"].as_str() {
            return Ok(content.to_string());
        }

        // Try other formats
        if let Some(text) = response["output"]["text"].as_str() {
            return Ok(text.to_string());
        }

        // Try Zhipu format
        if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            return Ok(content.to_string());
        }

        Err(anyhow!("Unable to extract content from response: {:?}", response))
    }

    fn parse_sse_chunk(text: &str, _provider_type: &str) -> Result<Option<String>> {
        // Handle SSE format: data: {...}
        for line in text.lines() {
            let line = line.trim();
            
            if line.is_empty() {
                continue;
            }

            if line.starts_with("data: ") {
                let data = &line[6..];
                
                if data == "[DONE]" {
                    return Ok(None);
                }

                match serde_json::from_str::<serde_json::Value>(data) {
                    Ok(json) => {
                        // OpenAI/DeepSeek/Qwen/Doubao format
                        if let Some(content) = json["choices"][0]["delta"]["content"].as_str() {
                            if !content.is_empty() {
                                return Ok(Some(content.to_string()));
                            }
                        }

                        // Ollama format
                        if let Some(content) = json["message"]["content"].as_str() {
                            if !content.is_empty() {
                                return Ok(Some(content.to_string()));
                            }
                        }

                        // Cohere format
                        if let Some(content) = json["text"].as_str() {
                            if !content.is_empty() {
                                return Ok(Some(content.to_string()));
                            }
                        }
                    }
                    Err(_) => {
                        // If not JSON, might be plain text
                        if !data.is_empty() && data != "[DONE]" {
                            return Ok(Some(data.to_string()));
                        }
                    }
                }
            }
        }

        Ok(None)
    }
}
