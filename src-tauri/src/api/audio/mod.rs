use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use crate::db::UnifiedDbManager;
use base64::Engine;

pub mod storage;
pub mod transcription;
pub mod models;
pub mod ai_analysis;
pub mod search;
pub mod optimization;

// 重新导出主要类型
pub use models::*;
pub use storage::*;
pub use transcription::*;
pub use ai_analysis::*;
pub use search::*;
pub use optimization::*;

/// 音频数据传输结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioData {
    pub tip_id: String,
    pub audio_data: String, // Base64 编码的音频数据
    pub file_format: String, // webm, mp3, wav
    pub duration: Option<i64>, // 时长（毫秒）
    pub file_name: Option<String>,
}

/// 音频文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFileInfo {
    pub id: String,
    pub tip_id: String,
    pub audio_id: String,
    pub file_name: String,
    pub file_format: String,
    pub duration: Option<i64>,
    pub file_size: i64,
    pub transcription: Option<String>,
    pub transcription_confidence: Option<f64>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 响应给前端的音频文件结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFileResponse {
    pub audio_data: String, // Base64 编码的音频数据
    pub file_format: String,
}

/// 转录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionRequest {
    pub audio_id: String,
    pub language: Option<String>,
    pub service: String, // openai, azure, local
}

/// 转录结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub confidence: Option<f64>,
    pub segments: Option<Vec<TranscriptionSegment>>,
}

/// 转录片段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start: f64,
    pub end: f64,
    pub text: String,
    pub confidence: Option<f64>,
}

/// 保存音频文件
#[tauri::command]
pub async fn save_audio_file(
    audio_data: AudioData,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    storage::save_audio_file_to_db(&*db_manager, audio_data).await
}

/// 获取音频文件
#[tauri::command]
pub async fn get_audio_file(
    audio_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<AudioFileResponse, String> {
    let audio_file = storage::get_audio_file_info(&*db_manager, &audio_id).await?;
    let audio_data = storage::get_audio_file_data(&*db_manager, &audio_id).await?;

    let encoded_data = base64::engine::general_purpose::STANDARD.encode(&audio_data);

    Ok(AudioFileResponse {
        audio_data: encoded_data,
        file_format: audio_file.file_format,
    })
}

/// 获取笔记的所有音频
#[tauri::command]
pub async fn get_tip_audio_files(
    tip_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<AudioFileInfo>, String> {
    storage::get_tip_audio_files(&*db_manager, &tip_id).await
}

/// 删除音频文件
#[tauri::command]
pub async fn delete_audio_file(
    audio_id: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    storage::delete_audio_file_from_db(&*db_manager, &audio_id).await
}

/// 转录音频
#[tauri::command]
pub async fn transcribe_audio(
    audio_id: String,
    language: Option<String>,
    service: String,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<String, String> {
    let request = TranscriptionRequest {
        audio_id,
        language,
        service,
    };
    
    transcription::transcribe_audio_file(&*db_manager, request).await
}

/// 更新音频转录
#[tauri::command]
pub async fn update_audio_transcription(
    audio_id: String,
    transcription: String,
    confidence: Option<f64>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    storage::update_audio_transcription(&*db_manager, &audio_id, &transcription, confidence).await
}

/// 批量转录笔记的所有音频
#[tauri::command]
pub async fn batch_transcribe_tip_audio(
    tip_id: String,
    service: String,
    language: Option<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<String>, String> {
    transcription::batch_transcribe_audio_files(&*db_manager, &tip_id, &service, language).await
}

/// 获取支持的语言列表
#[tauri::command]
pub async fn get_supported_transcription_languages() -> Result<Vec<(String, String)>, String> {
    let languages = transcription::get_supported_languages()
        .into_iter()
        .map(|(code, name)| (code.to_string(), name.to_string()))
        .collect();
    Ok(languages)
}

/// 验证转录服务配置
#[tauri::command]
pub async fn validate_transcription_service_config(service: String) -> Result<bool, String> {
    transcription::validate_transcription_service(&service).await
}

/// 获取可用的转录服务列表
#[tauri::command]
pub async fn get_available_transcription_services() -> Result<Vec<String>, String> {
    let mut services = Vec::new();
    
    // 检查各个服务的可用性
    if transcription::validate_transcription_service("openai").await.is_ok() {
        services.push("openai".to_string());
    }
    if transcription::validate_transcription_service("azure").await.is_ok() {
        services.push("azure".to_string());
    }
    if transcription::validate_transcription_service("google").await.is_ok() {
        services.push("google".to_string());
    }
    if transcription::validate_transcription_service("local").await.is_ok() {
        services.push("local".to_string());
    }
    
    Ok(services)
} 

/// 分析音频内容
#[tauri::command]
pub async fn analyze_audio_content(
    request: AudioAnalysisRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<AudioAnalysisResult, String> {
    ai_analysis::analyze_audio_content(&*db_manager, request).await
}

/// 搜索音频内容
#[tauri::command]
pub async fn search_audio_content_by_text(
    request: AudioSearchRequest,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<AudioSearchResult>, String> {
    search::search_audio_content(&*db_manager, request).await
}

/// 获取音频搜索统计信息
#[tauri::command]
pub async fn get_audio_search_stats(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<AudioSearchStats, String> {
    search::get_audio_search_stats(&*db_manager).await
}

/// 构建音频搜索索引
#[tauri::command]
pub async fn build_audio_search_index(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<(), String> {
    search::build_audio_search_index(&*db_manager).await.map(|_| ())
}

/// 批量分析笔记的音频内容
#[tauri::command]
pub async fn batch_analyze_tip_audio(
    tip_id: String,
    analysis_types: Vec<String>,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<(String, AudioAnalysisResult)>, String> {
    ai_analysis::batch_analyze_audio_content(&*db_manager, &tip_id, analysis_types).await
}

/// 优化音频文件
#[tauri::command]
pub async fn optimize_audio_file(
    audio_id: String,
    config: AudioOptimizationConfig,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<AudioOptimizationResult, String> {
    optimization::optimize_audio_file(&*db_manager, &audio_id, &config).await
}

/// 批量优化音频文件
#[tauri::command]
pub async fn batch_optimize_audio_files(
    tip_id: Option<String>,
    config: AudioOptimizationConfig,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<Vec<AudioOptimizationResult>, String> {
    optimization::batch_optimize_audio_files(&*db_manager, tip_id, &config).await
}

/// 获取音频存储统计信息
#[tauri::command]
pub async fn get_audio_optimization_stats(
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<AudioStorageStats, String> {
    optimization::get_audio_storage_stats(&*db_manager).await
}

/// 清理音频缓存
#[tauri::command]
pub async fn cleanup_audio_cache(
    max_age_days: u32,
    db_manager: State<'_, UnifiedDbManager>,
) -> Result<u64, String> {
    optimization::cleanup_audio_cache(&*db_manager, max_age_days).await
} 