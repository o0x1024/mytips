use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::fs;
use std::path::Path;

use crate::db::UnifiedDbManager;
use super::{storage, AudioFileInfo};

/// 音频优化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOptimizationConfig {
    pub enable_compression: bool,
    pub target_bitrate: u32, // kbps
    pub target_sample_rate: u32, // Hz
    pub remove_silence: bool,
    pub normalize_volume: bool,
    pub quality_level: AudioQualityLevel,
    pub max_file_size_mb: f32,
}

/// 音频质量级别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioQualityLevel {
    Low,    // 64kbps, 22050Hz
    Medium, // 128kbps, 44100Hz
    High,   // 256kbps, 48000Hz
    Custom, // 使用自定义配置
}

/// 音频优化结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioOptimizationResult {
    pub audio_id: String,
    pub original_size: u64,
    pub optimized_size: u64,
    pub compression_ratio: f32,
    pub duration_ms: i64,
    pub original_bitrate: Option<u32>,
    pub optimized_bitrate: u32,
    pub quality_score: f32,
    pub success: bool,
    pub error_message: Option<String>,
}

/// 音频统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioStorageStats {
    pub total_files: u32,
    pub total_size_bytes: u64,
    pub total_duration_ms: i64,
    pub average_file_size: u64,
    pub compressed_files: u32,
    pub potential_savings_bytes: u64,
}

impl Default for AudioOptimizationConfig {
    fn default() -> Self {
        Self {
            enable_compression: true,
            target_bitrate: 128,
            target_sample_rate: 44100,
            remove_silence: false,
            normalize_volume: false,
            quality_level: AudioQualityLevel::Medium,
            max_file_size_mb: 10.0,
        }
    }
}

impl AudioQualityLevel {
    pub fn get_bitrate(&self) -> u32 {
        match self {
            AudioQualityLevel::Low => 64,
            AudioQualityLevel::Medium => 128,
            AudioQualityLevel::High => 256,
            AudioQualityLevel::Custom => 128, // 默认值
        }
    }
    
    pub fn get_sample_rate(&self) -> u32 {
        match self {
            AudioQualityLevel::Low => 22050,
            AudioQualityLevel::Medium => 44100,
            AudioQualityLevel::High => 48000,
            AudioQualityLevel::Custom => 44100, // 默认值
        }
    }
}

/// 优化单个音频文件
pub async fn optimize_audio_file(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
    config: &AudioOptimizationConfig,
) -> Result<AudioOptimizationResult, String> {
    // 获取音频文件信息
    let audio_info = storage::get_audio_file_info(db_manager, audio_id).await?;
    let audio_data = storage::get_audio_file_data(db_manager, audio_id).await?;
    
    if audio_data.is_empty() {
        return Err("Audio file is empty".to_string());
    }

    let original_size = audio_data.len() as u64;
    
    // 检查是否需要优化
    if !should_optimize_file(&audio_info, config, original_size) {
        return Ok(AudioOptimizationResult {
            audio_id: audio_id.to_string(),
            original_size,
            optimized_size: original_size,
            compression_ratio: 1.0,
            duration_ms: audio_info.duration.unwrap_or(0),
            original_bitrate: None,
            optimized_bitrate: config.target_bitrate,
            quality_score: 1.0,
            success: true,
            error_message: None,
        });
    }

    // 执行音频压缩
    match compress_audio_data(&audio_data, &audio_info.file_format, config).await {
        Ok(compressed_data) => {
            let optimized_size = compressed_data.len() as u64;
            let compression_ratio = original_size as f32 / optimized_size as f32;
            
            // 更新数据库中的音频数据
            if let Err(e) = storage::update_audio_file_data(db_manager, audio_id, compressed_data).await {
                return Err(format!("Failed to update audio data: {}", e));
            }
            
            Ok(AudioOptimizationResult {
                audio_id: audio_id.to_string(),
                original_size,
                optimized_size,
                compression_ratio,
                duration_ms: audio_info.duration.unwrap_or(0),
                original_bitrate: None,
                optimized_bitrate: config.target_bitrate,
                quality_score: calculate_quality_score(compression_ratio),
                success: true,
                error_message: None,
            })
        },
        Err(e) => {
            Ok(AudioOptimizationResult {
                audio_id: audio_id.to_string(),
                original_size,
                optimized_size: original_size,
                compression_ratio: 1.0,
                duration_ms: audio_info.duration.unwrap_or(0),
                original_bitrate: None,
                optimized_bitrate: config.target_bitrate,
                quality_score: 0.0,
                success: false,
                error_message: Some(e),
            })
        }
    }
}

/// 批量优化音频文件
pub async fn batch_optimize_audio_files(
    db_manager: &UnifiedDbManager,
    tip_id: Option<String>,
    config: &AudioOptimizationConfig,
) -> Result<Vec<AudioOptimizationResult>, String> {
    // 获取要优化的音频文件列表
    let audio_files = if let Some(tip_id) = tip_id {
        storage::get_tip_audio_files(db_manager, &tip_id).await?
    } else {
        get_all_audio_files(db_manager).await?
    };

    let mut results = Vec::new();
    
    for audio_file in audio_files {
        match optimize_audio_file(db_manager, &audio_file.audio_id, config).await {
            Ok(result) => results.push(result),
            Err(e) => {
                results.push(AudioOptimizationResult {
                    audio_id: audio_file.audio_id,
                    original_size: 0,
                    optimized_size: 0,
                    compression_ratio: 1.0,
                    duration_ms: audio_file.duration.unwrap_or(0),
                    original_bitrate: None,
                    optimized_bitrate: config.target_bitrate,
                    quality_score: 0.0,
                    success: false,
                    error_message: Some(e),
                });
            }
        }
    }
    
    Ok(results)
}

/// 获取音频存储统计信息
pub async fn get_audio_storage_stats(
    db_manager: &UnifiedDbManager,
) -> Result<AudioStorageStats, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    // 统计基本信息
    let query = "
        SELECT 
            COUNT(*) as total_files,
            SUM(LENGTH(audio_data)) as total_size,
            SUM(duration) as total_duration,
            AVG(LENGTH(audio_data)) as avg_size
        FROM tip_audio_files 
        WHERE audio_data IS NOT NULL
    ";
    
    let mut rows = conn.query(query, ()).await
        .map_err(|e| format!("Failed to execute query: {}", e))?;
    
    let (total_files, total_size, total_duration, average_file_size) = if let Some(row) = rows.next().await
        .map_err(|e| format!("Failed to read row: {}", e))? {
        (
            row.get::<u32>(0).map_err(|e| format!("Failed to get total_files: {}", e))?,
            row.get::<Option<i64>>(1).map_err(|e| format!("Failed to get total_size: {}", e))?.unwrap_or(0) as u64,
            row.get::<Option<i64>>(2).map_err(|e| format!("Failed to get total_duration: {}", e))?.unwrap_or(0),
            row.get::<Option<i64>>(3).map_err(|e| format!("Failed to get avg_size: {}", e))?.unwrap_or(0) as u64,
        )
    } else {
        (0, 0, 0, 0)
    };

    // 估算可能的节省空间（假设30%的压缩率）
    let potential_savings = (total_size as f32 * 0.3) as u64;

    Ok(AudioStorageStats {
        total_files,
        total_size_bytes: total_size,
        total_duration_ms: total_duration,
        average_file_size,
        compressed_files: 0, // TODO: 跟踪已压缩的文件
        potential_savings_bytes: potential_savings,
    })
}

/// 获取所有音频文件（内部函数）
async fn get_all_audio_files(
    db_manager: &UnifiedDbManager,
) -> Result<Vec<AudioFileInfo>, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let query = "SELECT id, tip_id, audio_id, file_name, file_format, duration, transcription, created_at, updated_at FROM tip_audio_files ORDER BY created_at DESC";
    
    let mut rows = conn.query(query, ()).await
        .map_err(|e| format!("Failed to execute query: {}", e))?;

    let mut audio_files = Vec::new();
    while let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        let audio_file = AudioFileInfo {
            id: row.get::<String>(0).map_err(|e| format!("Failed to get id: {}", e))?,
            tip_id: row.get::<String>(1).map_err(|e| format!("Failed to get tip_id: {}", e))?,
            audio_id: row.get::<String>(2).map_err(|e| format!("Failed to get audio_id: {}", e))?,
            file_name: row.get::<String>(3).map_err(|e| format!("Failed to get file_name: {}", e))?,
            file_format: row.get::<String>(4).map_err(|e| format!("Failed to get file_format: {}", e))?,
            duration: row.get::<Option<i64>>(5).map_err(|e| format!("Failed to get duration: {}", e))?,
            file_size: 0, // 将在后续计算
            transcription: row.get::<Option<String>>(6).map_err(|e| format!("Failed to get transcription: {}", e))?,
            transcription_confidence: None,
            created_at: row.get::<i64>(7).map_err(|e| format!("Failed to get created_at: {}", e))?,
            updated_at: row.get::<i64>(8).map_err(|e| format!("Failed to get updated_at: {}", e))?,
        };
        audio_files.push(audio_file);
    }

    Ok(audio_files)
}

/// 判断是否需要优化文件
fn should_optimize_file(
    audio_info: &AudioFileInfo,
    config: &AudioOptimizationConfig,
    file_size: u64,
) -> bool {
    if !config.enable_compression {
        return false;
    }
    
    // 检查文件大小限制
    let max_size_bytes = (config.max_file_size_mb * 1024.0 * 1024.0) as u64;
    if file_size < max_size_bytes / 4 {
        // 文件已经很小，不需要优化
        return false;
    }
    
    // 检查文件格式
    match audio_info.file_format.to_lowercase().as_str() {
        "webm" | "mp3" | "ogg" => true,
        "wav" | "flac" => true, // 无损格式，有较大优化空间
        _ => false,
    }
}

/// 压缩音频数据
async fn compress_audio_data(
    audio_data: &[u8],
    format: &str,
    config: &AudioOptimizationConfig,
) -> Result<Vec<u8>, String> {
    // 检查是否安装了FFmpeg
    if !is_ffmpeg_available() {
        return simple_audio_compression(audio_data, format, config).await;
    }
    
    ffmpeg_compress_audio(audio_data, format, config).await
}

/// 检查FFmpeg是否可用
fn is_ffmpeg_available() -> bool {
    Command::new("ffmpeg")
        .arg("-version")
        .output()
        .is_ok()
}

/// 使用FFmpeg压缩音频
async fn ffmpeg_compress_audio(
    audio_data: &[u8],
    _format: &str,
    config: &AudioOptimizationConfig,
) -> Result<Vec<u8>, String> {
    let temp_dir = std::env::temp_dir();
    let input_path = temp_dir.join(format!("input_{}.webm", uuid::Uuid::new_v4()));
    let output_path = temp_dir.join(format!("output_{}.webm", uuid::Uuid::new_v4()));
    
    // 写入临时文件
    fs::write(&input_path, audio_data)
        .map_err(|e| format!("Failed to write temp file: {}", e))?;
    
    // 构建FFmpeg命令
    let mut cmd = Command::new("ffmpeg");
    cmd.arg("-i").arg(&input_path)
       .arg("-c:a").arg("libopus") // 使用Opus编码器
       .arg("-b:a").arg(format!("{}k", config.target_bitrate))
       .arg("-ar").arg(config.target_sample_rate.to_string())
       .arg("-y") // 覆盖输出文件
       .arg(&output_path);
    
    // 添加音频处理选项
    if config.remove_silence {
        cmd.arg("-af").arg("silenceremove=start_periods=1:start_silence=0.1:start_threshold=-50dB");
    }
    
    if config.normalize_volume {
        cmd.arg("-af").arg("loudnorm");
    }
    
    // 执行命令
    let output = cmd.output()
        .map_err(|e| format!("Failed to execute ffmpeg: {}", e))?;
    
    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("FFmpeg error: {}", error));
    }
    
    // 读取压缩后的文件
    let compressed_data = fs::read(&output_path)
        .map_err(|e| format!("Failed to read compressed file: {}", e))?;
    
    // 清理临时文件
    let _ = fs::remove_file(&input_path);
    let _ = fs::remove_file(&output_path);
    
    Ok(compressed_data)
}

/// 简单音频压缩（当FFmpeg不可用时）
async fn simple_audio_compression(
    audio_data: &[u8],
    _format: &str,
    config: &AudioOptimizationConfig,
) -> Result<Vec<u8>, String> {
    // 简单的数据压缩（使用deflate算法）
    use flate2::Compression;
    use flate2::write::DeflateEncoder;
    use std::io::Write;
    
    let compression_level = match config.quality_level {
        AudioQualityLevel::Low => Compression::fast(),
        AudioQualityLevel::Medium => Compression::default(),
        AudioQualityLevel::High => Compression::best(),
        AudioQualityLevel::Custom => Compression::default(),
    };
    
    let mut encoder = DeflateEncoder::new(Vec::new(), compression_level);
    encoder.write_all(audio_data)
        .map_err(|e| format!("Compression failed: {}", e))?;
    
    let compressed = encoder.finish()
        .map_err(|e| format!("Compression finish failed: {}", e))?;
    
    // 检查压缩效果
    if compressed.len() >= audio_data.len() {
        // 压缩后更大，返回原数据
        Ok(audio_data.to_vec())
    } else {
        Ok(compressed)
    }
}

/// 计算质量评分
fn calculate_quality_score(compression_ratio: f32) -> f32 {
    // 基于压缩比计算质量评分
    if compression_ratio < 1.1 {
        1.0 // 压缩效果不明显
    } else if compression_ratio < 2.0 {
        0.9 // 良好的压缩
    } else if compression_ratio < 3.0 {
        0.8 // 较好的压缩
    } else if compression_ratio < 5.0 {
        0.7 // 高压缩比
    } else {
        0.6 // 可能质量损失较大
    }
}

/// 清理音频缓存和临时文件
pub async fn cleanup_audio_cache(
    db_manager: &UnifiedDbManager,
    max_age_days: u32,
) -> Result<u64, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let cutoff_timestamp = chrono::Utc::now().timestamp() - (max_age_days as i64 * 24 * 60 * 60);
    
    // 删除旧的音频文件
    let deleted_rows = conn.execute(
        "DELETE FROM tip_audio_files WHERE created_at < ?1 AND transcription IS NULL",
        [cutoff_timestamp]
    ).await.map_err(|e| format!("Failed to cleanup old files: {}", e))?;

    Ok(deleted_rows as u64)
} 