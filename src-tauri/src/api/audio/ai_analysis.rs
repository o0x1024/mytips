use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;

use crate::db::UnifiedDbManager;
use crate::api::settings::get_client_with_proxy;

/// AI分析结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysisResult {
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub keywords: Vec<String>,
    pub sentiment: Option<String>,
    pub language: Option<String>,
}

/// AI分析请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioAnalysisRequest {
    pub audio_id: String,
    pub transcription_text: String,
    pub analysis_type: Vec<String>, // ["title", "tags", "summary", "keywords", "sentiment"]
    pub language: Option<String>,
}

/// 对音频转录内容进行AI分析
pub async fn analyze_audio_content(
    db_manager: &UnifiedDbManager,
    request: AudioAnalysisRequest,
) -> Result<AudioAnalysisResult, String> {
    if request.transcription_text.trim().is_empty() {
        return Err("Transcription text is empty".to_string());
    }

    let mut result = AudioAnalysisResult {
        title: None,
        tags: Vec::new(),
        summary: None,
        keywords: Vec::new(),
        sentiment: None,
        language: None,
    };

    // 根据请求的分析类型执行相应分析
    for analysis_type in &request.analysis_type {
        match analysis_type.as_str() {
            "title" => {
                result.title = generate_smart_title(&request.transcription_text, db_manager).await?;
            },
            "tags" => {
                result.tags = extract_auto_tags(&request.transcription_text, db_manager).await?;
            },
            "summary" => {
                result.summary = generate_content_summary(&request.transcription_text, db_manager).await?;
            },
            "keywords" => {
                result.keywords = extract_keywords(&request.transcription_text, db_manager).await?;
            },
            "sentiment" => {
                result.sentiment = analyze_sentiment(&request.transcription_text, db_manager).await?;
            },
            _ => {
                eprintln!("Unknown analysis type: {}", analysis_type);
            }
        }
    }

    // 检测语言
    result.language = detect_language(&request.transcription_text).await.ok();

    Ok(result)
}

/// 生成智能标题
async fn generate_smart_title(
    text: &str,
    db_manager: &UnifiedDbManager,
) -> Result<Option<String>, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OpenAI API key not configured for title generation".to_string())?;

    let client = get_client_with_proxy(db_manager)
        .await
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // 创建提示词
    let prompt = format!(
        "请为以下录音转录内容生成一个简洁、准确的标题。标题应该：
1. 不超过30个字符
2. 概括主要内容
3. 简洁明了
4. 中文输出

转录内容：
{}

只返回标题，不要其他内容：",
        text.chars().take(800).collect::<String>() // 限制文本长度
    );

    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "你是一个专业的内容标题生成助手，擅长为录音转录内容生成准确简洁的标题。"
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 50,
        "temperature": 0.3
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let title = result["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    Ok(title)
}

/// 自动提取标签
async fn extract_auto_tags(
    text: &str,
    db_manager: &UnifiedDbManager,
) -> Result<Vec<String>, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OpenAI API key not configured for tag extraction".to_string())?;

    let client = get_client_with_proxy(db_manager)
        .await
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let prompt = format!(
        "请从以下录音转录内容中提取5-8个关键标签。标签应该：
1. 概括主要话题和内容
2. 简洁明了，每个标签不超过6个字符
3. 中文输出
4. 用逗号分隔

转录内容：
{}

只返回标签，用逗号分隔：",
        text.chars().take(1000).collect::<String>()
    );

    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "你是一个专业的内容标签提取助手，擅长从录音转录内容中提取准确的关键标签。"
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 100,
        "temperature": 0.2
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let tags_text = result["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default();

    let tags: Vec<String> = tags_text
        .split(',')
        .map(|tag| tag.trim().to_string())
        .filter(|tag| !tag.is_empty() && tag.len() <= 20)
        .take(8)
        .collect();

    Ok(tags)
}

/// 生成内容总结
async fn generate_content_summary(
    text: &str,
    db_manager: &UnifiedDbManager,
) -> Result<Option<String>, String> {
    // 只对长文本进行总结
    if text.len() < 200 {
        return Ok(None);
    }

    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OpenAI API key not configured for content summary".to_string())?;

    let client = get_client_with_proxy(db_manager)
        .await
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let prompt = format!(
        "请为以下录音转录内容生成一个简洁的总结。总结应该：
1. 不超过150个字符
2. 概括主要观点和结论
3. 保持客观中性
4. 中文输出

转录内容：
{}

总结：",
        text.chars().take(2000).collect::<String>()
    );

    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "你是一个专业的内容总结助手，擅长为录音转录内容生成简洁准确的总结。"
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 200,
        "temperature": 0.3
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let summary = result["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    Ok(summary)
}

/// 提取关键词
async fn extract_keywords(
    text: &str,
    _db_manager: &UnifiedDbManager,
) -> Result<Vec<String>, String> {
    // 简单的关键词提取算法
    let processed_text = text
        .chars()
        .map(|c| if c.is_ascii_punctuation() { ' ' } else { c })
        .collect::<String>();
    
    let words: Vec<String> = processed_text
        .split_whitespace()
        .filter(|word| word.len() > 1)
        .map(|word| word.to_string())
        .collect();

    // 统计词频
    let mut word_count = std::collections::HashMap::new();
    for word in words {
        let word = word.to_lowercase();
        if is_meaningful_word(&word) {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    // 按频率排序并返回前10个
    let mut keywords: Vec<_> = word_count.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));
    
    let result: Vec<String> = keywords
        .into_iter()
        .take(10)
        .map(|(word, _)| word)
        .collect();

    Ok(result)
}

/// 判断是否为有意义的词
fn is_meaningful_word(word: &str) -> bool {
    // 过滤停用词和无意义词
    let stop_words = [
        "的", "了", "在", "是", "我", "有", "和", "就", "不", "人", "都", "一", "个", "上", "也", "很", "到", "说", "要", "去", "你", "会", "着", "没有", "看", "好", "自己", "这", "那", "里", "被", "从", "跟", "与", "但是", "然后", "所以", "因为", "如果", "when", "the", "and", "or", "but", "in", "on", "at", "to", "for", "of", "with", "by", "a", "an", "is", "are", "was", "were", "be", "been", "have", "has", "had", "do", "does", "did", "will", "would", "should", "could", "can", "may", "might", "this", "that", "these", "those", "i", "you", "he", "she", "it", "we", "they", "me", "him", "her", "us", "them"
    ];

    !stop_words.contains(&word) && word.len() > 1
}

/// 情感分析
async fn analyze_sentiment(
    text: &str,
    db_manager: &UnifiedDbManager,
) -> Result<Option<String>, String> {
    let api_key = std::env::var("OPENAI_API_KEY")
        .map_err(|_| "OpenAI API key not configured for sentiment analysis".to_string())?;

    let client = get_client_with_proxy(db_manager)
        .await
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    let prompt = format!(
        "请分析以下录音转录内容的情感倾向。只返回以下选项之一：
- 积极
- 消极
- 中性

转录内容：
{}

情感倾向：",
        text.chars().take(1000).collect::<String>()
    );

    let request_body = serde_json::json!({
        "model": "gpt-3.5-turbo",
        "messages": [
            {
                "role": "system",
                "content": "你是一个专业的情感分析助手，能够准确分析文本的情感倾向。"
            },
            {
                "role": "user",
                "content": prompt
            }
        ],
        "max_tokens": 10,
        "temperature": 0.1
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let sentiment = result["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.trim().to_string())
        .filter(|s| ["积极", "消极", "中性"].contains(&s.as_str()));

    Ok(sentiment)
}

/// 检测语言
async fn detect_language(text: &str) -> Result<String, String> {
    // 简单的语言检测逻辑
    let chinese_chars: usize = text.chars().filter(|c| is_chinese_char(*c)).count();
    let total_chars = text.len();
    
    if chinese_chars > total_chars / 3 {
        Ok("zh".to_string())
    } else if text.chars().all(|c| c.is_ascii() || c.is_ascii_punctuation() || c.is_whitespace()) {
        Ok("en".to_string())
    } else {
        Ok("unknown".to_string())
    }
}

/// 判断是否为中文字符
fn is_chinese_char(c: char) -> bool {
    matches!(c, '\u{4e00}'..='\u{9fff}' | '\u{3400}'..='\u{4dbf}' | '\u{20000}'..='\u{2a6df}')
}

/// 批量分析音频内容
pub async fn batch_analyze_audio_content(
    db_manager: &UnifiedDbManager,
    tip_id: &str,
    analysis_types: Vec<String>,
) -> Result<Vec<(String, AudioAnalysisResult)>, String> {
    // 获取笔记的所有音频文件
    let audio_files = crate::api::audio::storage::get_tip_audio_files(db_manager, tip_id).await?;
    
    let mut results = Vec::new();
    
    for audio_file in audio_files {
        if let Some(transcription) = audio_file.transcription {
            if !transcription.trim().is_empty() {
                let request = AudioAnalysisRequest {
                    audio_id: audio_file.audio_id.clone(),
                    transcription_text: transcription,
                    analysis_type: analysis_types.clone(),
                    language: None,
                };
                
                match analyze_audio_content(db_manager, request).await {
                    Ok(analysis) => {
                        results.push((audio_file.audio_id, analysis));
                    },
                    Err(e) => {
                        eprintln!("Failed to analyze audio {}: {}", audio_file.audio_id, e);
                    }
                }
            }
        }
    }
    
    Ok(results)
} 