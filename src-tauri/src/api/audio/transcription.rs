use anyhow::Result;
use reqwest::multipart::{Form, Part};
use serde_json::Value;
use std::time::Duration;

use crate::db::UnifiedDbManager;
use crate::api::settings::get_client_with_proxy;
use super::{TranscriptionRequest, TranscriptionResult, TranscriptionService, storage};

/// 转录音频文件
pub async fn transcribe_audio_file(
    db_manager: &UnifiedDbManager,
    request: TranscriptionRequest,
) -> Result<String, String> {
    // 获取音频数据
    let audio_data = storage::get_audio_file_data(db_manager, &request.audio_id).await?;
    
    if audio_data.is_empty() {
        return Err("Audio file is empty".to_string());
    }

    // 根据服务类型选择转录方法
    let service = TranscriptionService::from_str(&request.service)
        .ok_or_else(|| format!("Unsupported transcription service: {}", request.service))?;

    let result = match service {
        TranscriptionService::OpenAI => {
            transcribe_with_openai(db_manager, audio_data, request.language).await?
        },
        TranscriptionService::Azure => {
            transcribe_with_azure(audio_data, request.language).await?
        },
        TranscriptionService::Local => {
            transcribe_with_local(audio_data, request.language).await?
        },
        TranscriptionService::Google => {
            transcribe_with_google(audio_data, request.language).await?
        },
    };

    // 更新数据库中的转录结果
    storage::update_audio_transcription(
        db_manager,
        &request.audio_id,
        &result.text,
        result.confidence,
    ).await?;

    Ok(result.text)
}

/// 使用 OpenAI Whisper API 转录
async fn transcribe_with_openai(
    db_manager: &UnifiedDbManager,
    audio_data: Vec<u8>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    // 获取 OpenAI API 密钥
    let api_key = std::env::var("OPENAI_API_KEY")
        .or_else(|_| {
            // 尝试从设置中获取
            // TODO: 从数据库设置中获取 API 密钥
            Err("OpenAI API key not found".to_string())
        })?;

    let client = get_client_with_proxy(db_manager)
        .await
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

    // 构建多部分表单
    let mut form = Form::new()
        .text("model", "whisper-1")
        .text("response_format", "verbose_json");

    // 添加语言参数
    if let Some(lang) = language {
        if lang != "auto" {
            form = form.text("language", lang);
        }
    }

    // 添加音频文件
    let audio_part = Part::bytes(audio_data)
        .file_name("audio.webm")
        .mime_str("audio/webm")
        .map_err(|e| format!("Failed to create audio part: {}", e))?;
    
    form = form.part("file", audio_part);

    // 发送请求
    let response = client
        .post("https://api.openai.com/v1/audio/transcriptions")
        .header("Authorization", format!("Bearer {}", api_key))
        .multipart(form)
        .timeout(Duration::from_secs(120)) // 2分钟超时
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("OpenAI API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let text = result["text"]
        .as_str()
        .ok_or_else(|| "No text in response".to_string())?
        .to_string();

    // 计算平均置信度
    let confidence = if let Some(segments) = result["segments"].as_array() {
        let total_confidence: f64 = segments
            .iter()
            .filter_map(|s| s["avg_logprob"].as_f64())
            .map(|logprob| (-logprob).exp()) // 将 log 概率转换为概率
            .sum();
        
        if segments.is_empty() {
            None
        } else {
            Some(total_confidence / segments.len() as f64)
        }
    } else {
        None
    };

    Ok(TranscriptionResult {
        text,
        confidence,
        segments: None, // 暂时不解析segments
    })
}

/// 使用 Azure Speech Service 转录
async fn transcribe_with_azure(
    audio_data: Vec<u8>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    // 获取 Azure Speech Service 配置
    let subscription_key = std::env::var("AZURE_SPEECH_KEY")
        .map_err(|_| "Azure Speech subscription key not found".to_string())?;
    let region = std::env::var("AZURE_SPEECH_REGION")
        .map_err(|_| "Azure Speech region not found".to_string())?;

    let client = reqwest::Client::new();
    
    // Azure Speech Service REST API 端点
    let url = format!(
        "https://{}.stt.speech.microsoft.com/speech/recognition/conversation/cognitiveservices/v1",
        region
    );

    // 构建查询参数
    let lang = language.unwrap_or_else(|| "zh-CN".to_string());
    let lang = match lang.as_str() {
        "auto" => "zh-CN", // Azure 不支持自动检测，默认中文
        "zh" => "zh-CN",
        "en" => "en-US",
        "ja" => "ja-JP",
        "ko" => "ko-KR",
        "es" => "es-ES",
        "fr" => "fr-FR",
        "de" => "de-DE",
        "it" => "it-IT",
        "pt" => "pt-PT",
        "ru" => "ru-RU",
        _ => "zh-CN", // 不支持的语言默认中文
    };

    // 发送请求
    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .header("Content-Type", "audio/webm; codecs=opus")
        .query(&[
            ("language", lang),
            ("format", "detailed"),
            ("profanity", "masked"),
        ])
        .body(audio_data)
        .timeout(Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| format!("Azure Speech request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Azure Speech API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse Azure response: {}", e))?;

    // 解析 Azure 响应
    let text = match result["RecognitionStatus"].as_str() {
        Some("Success") => {
            result["DisplayText"]
                .as_str()
                .ok_or_else(|| "No text in Azure response".to_string())?
                .to_string()
        },
        Some(status) => {
            return Err(format!("Azure recognition failed with status: {}", status));
        },
        None => {
            return Err("Invalid Azure response format".to_string());
        }
    };

    // Azure 不直接提供置信度，使用固定值
    let confidence = Some(0.85);

    // 解析详细结果中的segments（如果有）
    let segments = if let Some(nbest) = result["NBest"].as_array() {
        if let Some(first_result) = nbest.first() {
            if let Some(words) = first_result["Words"].as_array() {
                let mut segs = Vec::new();
                for word in words {
                    if let (Some(word_text), Some(offset), Some(duration)) = (
                        word["Word"].as_str(),
                        word["Offset"].as_u64(),
                        word["Duration"].as_u64(),
                    ) {
                        let start = offset as f64 / 10_000_000.0; // 转换为秒
                        let end = start + (duration as f64 / 10_000_000.0);
                        segs.push(super::TranscriptionSegment {
                            start,
                            end,
                            text: word_text.to_string(),
                            confidence: word.get("Confidence").and_then(|c| c.as_f64()),
                        });
                    }
                }
                Some(segs)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    Ok(TranscriptionResult {
        text,
        confidence,
        segments,
    })
}

/// 使用本地 Whisper 模型转录
async fn transcribe_with_local(
    audio_data: Vec<u8>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    // 检查本地 Whisper 可执行文件
    let whisper_cmd = std::env::var("WHISPER_EXECUTABLE")
        .unwrap_or_else(|_| "whisper".to_string());

    // 检查是否安装了 whisper
    let check_output = std::process::Command::new(&whisper_cmd)
        .arg("--help")
        .output();

    if check_output.is_err() {
        return Err("Local Whisper not found. Please install whisper and set WHISPER_EXECUTABLE if needed".to_string());
    }

    // 创建临时文件
    let temp_dir = std::env::temp_dir();
    let audio_file_path = temp_dir.join(format!("audio_{}.webm", uuid::Uuid::new_v4()));
    let output_file_path = temp_dir.join(format!("output_{}.txt", uuid::Uuid::new_v4()));

    // 写入音频数据到临时文件
    std::fs::write(&audio_file_path, audio_data)
        .map_err(|e| format!("Failed to write audio file: {}", e))?;

    // 准备 whisper 命令参数
    let mut cmd = std::process::Command::new(&whisper_cmd);
    cmd.arg(&audio_file_path)
        .arg("--model").arg("base") // 默认使用 base 模型
        .arg("--output_format").arg("txt")
        .arg("--output_dir").arg(&temp_dir)
        .arg("--verbose").arg("False");

    // 设置语言参数
    if let Some(lang) = language {
        if lang != "auto" {
            let whisper_lang = match lang.as_str() {
                "zh" => "zh",
                "en" => "en",
                "ja" => "ja",
                "ko" => "ko",
                "es" => "es",
                "fr" => "fr",
                "de" => "de",
                "it" => "it",
                "pt" => "pt",
                "ru" => "ru",
                "ar" => "ar",
                "hi" => "hi",
                "th" => "th",
                "vi" => "vi",
                _ => "auto",
            };
            if whisper_lang != "auto" {
                cmd.arg("--language").arg(whisper_lang);
            }
        }
    }

    // 执行 whisper 命令
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute whisper: {}", e))?;

    // 清理音频文件
    let _ = std::fs::remove_file(&audio_file_path);

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Whisper execution failed: {}", error_msg));
    }

    // 查找输出文件
    let audio_stem = audio_file_path.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| "Invalid audio file name".to_string())?;
    
    let output_txt_path = temp_dir.join(format!("{}.txt", audio_stem));
    
    // 读取转录结果
    let text = if output_txt_path.exists() {
        std::fs::read_to_string(&output_txt_path)
            .map_err(|e| format!("Failed to read output file: {}", e))?
            .trim()
            .to_string()
    } else {
        // 从标准输出中提取文本（备用方案）
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|line| !line.starts_with('['))  // 过滤时间戳行
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string()
    };

    // 清理输出文件
    let _ = std::fs::remove_file(&output_txt_path);

    if text.is_empty() {
        return Err("No transcription text generated".to_string());
    }

    // 本地 Whisper 不提供置信度信息
    Ok(TranscriptionResult {
        text,
        confidence: None,
        segments: None,
    })
}

/// 使用 Google Speech-to-Text API 转录
async fn transcribe_with_google(
    audio_data: Vec<u8>,
    language: Option<String>,
) -> Result<TranscriptionResult, String> {
    // 获取 Google Cloud 认证信息
    let credentials_path = std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
        .map_err(|_| "Google Application Credentials not found".to_string())?;
    
    // 简化的 Google Cloud 认证实现
    // 在实际生产环境中，应该使用 Google Cloud SDK
    let project_id = std::env::var("GOOGLE_CLOUD_PROJECT_ID")
        .map_err(|_| "Google Cloud Project ID not found".to_string())?;

    let client = reqwest::Client::new();
    
    // Google Speech-to-Text API 端点
    let url = format!(
        "https://speech.googleapis.com/v1/speech:recognize?key={}",
        std::env::var("GOOGLE_API_KEY")
            .map_err(|_| "Google API key not found".to_string())?
    );

    // 构建语言代码
    let lang = language.unwrap_or_else(|| "zh-CN".to_string());
    let lang = match lang.as_str() {
        "auto" => "zh-CN", // Google 支持自动检测，但需要特殊配置
        "zh" => "zh-CN",
        "en" => "en-US",
        "ja" => "ja-JP",
        "ko" => "ko-KR",
        "es" => "es-ES",
        "fr" => "fr-FR",
        "de" => "de-DE",
        "it" => "it-IT",
        "pt" => "pt-PT",
        "ru" => "ru-RU",
        "ar" => "ar",
        "hi" => "hi-IN",
        "th" => "th-TH",
        "vi" => "vi-VN",
        _ => "zh-CN",
    };

    // Base64 编码音频数据
    let audio_content = base64::encode(&audio_data);

    // 构建请求体
    let request_body = serde_json::json!({
        "config": {
            "encoding": "WEBM_OPUS",
            "sampleRateHertz": 48000,
            "languageCode": lang,
            "enableWordTimeOffsets": true,
            "enableWordConfidence": true,
            "model": "latest_long",
            "useEnhanced": true
        },
        "audio": {
            "content": audio_content
        }
    });

    // 发送请求
    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .timeout(Duration::from_secs(120))
        .send()
        .await
        .map_err(|e| format!("Google Speech request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Google Speech API error: {}", error_text));
    }

    let result: Value = response.json().await
        .map_err(|e| format!("Failed to parse Google response: {}", e))?;

    // 解析 Google Speech 响应
    let results = result["results"]
        .as_array()
        .ok_or_else(|| "No results in Google response".to_string())?;

    if results.is_empty() {
        return Err("No recognition results from Google Speech".to_string());
    }

    let first_result = &results[0];
    let alternatives = first_result["alternatives"]
        .as_array()
        .ok_or_else(|| "No alternatives in Google response".to_string())?;

    if alternatives.is_empty() {
        return Err("No alternatives in Google Speech result".to_string());
    }

    let best_alternative = &alternatives[0];
    let text = best_alternative["transcript"]
        .as_str()
        .ok_or_else(|| "No transcript in Google response".to_string())?
        .to_string();

    let confidence = best_alternative["confidence"].as_f64();

    // 解析词级别的时间戳和置信度
    let segments = if let Some(words) = best_alternative["words"].as_array() {
        let mut segs = Vec::new();
        for word in words {
            if let (Some(word_text), Some(start_time), Some(end_time)) = (
                word["word"].as_str(),
                word["startTime"].as_str(),
                word["endTime"].as_str(),
            ) {
                // 解析时间格式 "1.200s"
                let start = parse_google_time(start_time).unwrap_or(0.0);
                let end = parse_google_time(end_time).unwrap_or(0.0);
                
                segs.push(super::TranscriptionSegment {
                    start,
                    end,
                    text: word_text.to_string(),
                    confidence: word.get("confidence").and_then(|c| c.as_f64()),
                });
            }
        }
        if segs.is_empty() { None } else { Some(segs) }
    } else {
        None
    };

    Ok(TranscriptionResult {
        text,
        confidence,
        segments,
    })
}

/// 解析 Google 时间格式 "1.200s" -> 1.2
fn parse_google_time(time_str: &str) -> Option<f64> {
    if time_str.ends_with('s') {
        time_str[..time_str.len() - 1].parse().ok()
    } else {
        None
    }
}

/// 批量转录音频文件
pub async fn batch_transcribe_audio_files(
    db_manager: &UnifiedDbManager,
    tip_id: &str,
    service: &str,
    language: Option<String>,
) -> Result<Vec<String>, String> {
    // 获取笔记的所有未转录音频文件
    let audio_files = storage::get_tip_audio_files(db_manager, tip_id).await?;
    let untranscribed_files: Vec<_> = audio_files
        .into_iter()
        .filter(|f| f.transcription.is_none())
        .collect();

    if untranscribed_files.is_empty() {
        return Ok(vec![]);
    }

    let mut results = Vec::new();
    
    for audio_file in untranscribed_files {
        let request = TranscriptionRequest {
            audio_id: audio_file.audio_id.clone(),
            language: language.clone(),
            service: service.to_string(),
        };

        match transcribe_audio_file(db_manager, request).await {
            Ok(text) => {
                results.push(format!("✓ {}: {}", audio_file.file_name, text));
            },
            Err(e) => {
                results.push(format!("✗ {}: Error - {}", audio_file.file_name, e));
            }
        }
    }

    Ok(results)
}

/// 获取支持的语言列表
pub fn get_supported_languages() -> Vec<(&'static str, &'static str)> {
    vec![
        ("auto", "自动检测"),
        ("zh", "中文"),
        ("en", "English"),
        ("ja", "日本語"),
        ("ko", "한국어"),
        ("es", "Español"),
        ("fr", "Français"),
        ("de", "Deutsch"),
        ("it", "Italiano"),
        ("pt", "Português"),
        ("ru", "Русский"),
        ("ar", "العربية"),
        ("hi", "हिन्दी"),
        ("th", "ไทย"),
        ("vi", "Tiếng Việt"),
    ]
}

/// 验证转录服务配置
pub async fn validate_transcription_service(service: &str) -> Result<bool, String> {
    match service {
        "openai" => {
            // 检查 OpenAI API 密钥
            if std::env::var("OPENAI_API_KEY").is_err() {
                return Err("OpenAI API key not configured".to_string());
            }
            Ok(true)
        },
        "azure" => {
            // 检查 Azure 配置
            if std::env::var("AZURE_SPEECH_KEY").is_err() || 
               std::env::var("AZURE_SPEECH_REGION").is_err() {
                return Err("Azure Speech Service not configured".to_string());
            }
            Ok(true)
        },
        "local" => {
            // 检查本地 Whisper 是否可用
            let whisper_cmd = std::env::var("WHISPER_EXECUTABLE")
                .unwrap_or_else(|_| "whisper".to_string());
            
            let check_result = std::process::Command::new(&whisper_cmd)
                .arg("--help")
                .output();
                
            match check_result {
                Ok(_) => Ok(true),
                Err(_) => Err("Local Whisper not found. Please install whisper CLI tool".to_string()),
            }
        },
        "google" => {
            // 检查 Google 配置
            let has_credentials = std::env::var("GOOGLE_APPLICATION_CREDENTIALS").is_ok();
            let has_api_key = std::env::var("GOOGLE_API_KEY").is_ok();
            let has_project_id = std::env::var("GOOGLE_CLOUD_PROJECT_ID").is_ok();
            
            if !has_api_key {
                return Err("Google API key not configured".to_string());
            }
            if !has_project_id {
                return Err("Google Cloud Project ID not configured".to_string());
            }
            
            Ok(true)
        },
        _ => Err(format!("Unsupported transcription service: {}", service)),
    }
} 