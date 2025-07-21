use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use chrono::Utc;
use libsql::params;
use uuid::Uuid;

use crate::db::UnifiedDbManager;
use super::{AudioData, AudioFileInfo, AudioFile, AudioError};

/// 保存音频文件到数据库
pub async fn save_audio_file_to_db(
    db_manager: &UnifiedDbManager,
    audio_data: AudioData,
) -> Result<String, String> {
    // 验证输入数据
    if audio_data.tip_id.is_empty() {
        return Err("Tip ID cannot be empty".to_string());
    }

    if audio_data.audio_data.is_empty() {
        return Err("Audio data cannot be empty".to_string());
    }

    // 解码 Base64 音频数据
    let decoded_audio = general_purpose::STANDARD
        .decode(&audio_data.audio_data)
        .map_err(|e| format!("Failed to decode audio data: {}", e))?;

    // 检查文件大小限制 (50MB)
    const MAX_FILE_SIZE: usize = 50 * 1024 * 1024;
    if decoded_audio.len() > MAX_FILE_SIZE {
        return Err(format!("Audio file too large: {} bytes (max: {} bytes)", 
                          decoded_audio.len(), MAX_FILE_SIZE));
    }

    // 生成文件名
    let file_name = audio_data.file_name.unwrap_or_else(|| {
        format!("recording_{}.{}", 
               Utc::now().format("%Y%m%d_%H%M%S"), 
               audio_data.file_format)
    });

    // 创建音频文件记录
    let audio_file = AudioFile::new(
        audio_data.tip_id,
        file_name,
        audio_data.file_format,
        decoded_audio,
        audio_data.duration,
    );

    // 保存到数据库
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let audio_id = audio_file.audio_id.clone(); // 克隆audio_id以避免移动后使用

    conn.execute(
        "INSERT INTO tip_audio_files (
            id, tip_id, audio_id, file_name, file_format, 
            audio_data, file_size, duration, transcription, 
            transcription_confidence, created_at, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![
            audio_file.id,
            audio_file.tip_id,
            audio_file.audio_id,
            audio_file.file_name,
            audio_file.file_format,
            audio_file.audio_data,
            audio_file.file_size,
            audio_file.duration,
            audio_file.transcription,
            audio_file.transcription_confidence,
            audio_file.created_at,
            audio_file.updated_at,
        ],
    )
    .await
    .map_err(|e| format!("Failed to save audio file: {}", e))?;

    Ok(audio_id)
}

/// 获取音频文件信息
pub async fn get_audio_file_info(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<AudioFileInfo, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut rows = conn
        .query(
            "SELECT id, tip_id, audio_id, file_name, file_format, 
                    file_size, duration, transcription, transcription_confidence,
                    created_at, updated_at 
             FROM tip_audio_files WHERE audio_id = ?1",
            params![audio_id],
        )
        .await
        .map_err(|e| format!("Failed to query audio file: {}", e))?;

    if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        Ok(AudioFileInfo {
            id: row.get(0).map_err(|e| format!("Failed to get id: {}", e))?,
            tip_id: row.get(1).map_err(|e| format!("Failed to get tip_id: {}", e))?,
            audio_id: row.get(2).map_err(|e| format!("Failed to get audio_id: {}", e))?,
            file_name: row.get(3).map_err(|e| format!("Failed to get file_name: {}", e))?,
            file_format: row.get(4).map_err(|e| format!("Failed to get file_format: {}", e))?,
            file_size: row.get(5).map_err(|e| format!("Failed to get file_size: {}", e))?,
            duration: row.get(6).map_err(|e| format!("Failed to get duration: {}", e))?,
            transcription: row.get(7).map_err(|e| format!("Failed to get transcription: {}", e))?,
            transcription_confidence: row.get(8).map_err(|e| format!("Failed to get confidence: {}", e))?,
            created_at: row.get(9).map_err(|e| format!("Failed to get created_at: {}", e))?,
            updated_at: row.get(10).map_err(|e| format!("Failed to get updated_at: {}", e))?,
        })
    } else {
        Err(format!("Audio file not found: {}", audio_id))
    }
}

/// 更新音频文件数据
pub async fn update_audio_file_data(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
    new_audio_data: Vec<u8>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    // 更新音频数据
    let query = "UPDATE tip_audio_files SET audio_data = ?1, updated_at = ?2 WHERE audio_id = ?3";
    let updated_at = chrono::Utc::now().timestamp();
    
    let affected_rows = conn.execute(
        query,
        params![new_audio_data, updated_at, audio_id]
    ).await.map_err(|e| format!("Failed to update audio data: {}", e))?;

    if affected_rows == 0 {
        return Err(format!("Audio file with ID {} not found", audio_id));
    }

    Ok(())
}

/// 获取音频文件的原始数据
pub async fn get_audio_file_data(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<Vec<u8>, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let query = "SELECT audio_data FROM tip_audio_files WHERE audio_id = ?1";
    
    let mut rows = conn.query(query, params![audio_id]).await
        .map_err(|e| format!("Failed to get audio data: {}", e))?;
    
    let audio_data = if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        row.get::<Vec<u8>>(0).map_err(|e| format!("Failed to get audio_data: {}", e))?
    } else {
        return Err(format!("Audio file with ID {} not found", audio_id));
    };

    Ok(audio_data)
}

/// 获取笔记的所有音频文件
pub async fn get_tip_audio_files(
    db_manager: &UnifiedDbManager,
    tip_id: &str,
) -> Result<Vec<AudioFileInfo>, String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut rows = conn
        .query(
            "SELECT id, tip_id, audio_id, file_name, file_format, 
                    file_size, duration, transcription, transcription_confidence,
                    created_at, updated_at 
             FROM tip_audio_files WHERE tip_id = ?1 ORDER BY created_at DESC",
            params![tip_id],
        )
        .await
        .map_err(|e| format!("Failed to query audio files: {}", e))?;

    let mut audio_files = Vec::new();
    
    while let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        audio_files.push(AudioFileInfo {
            id: row.get(0).map_err(|e| format!("Failed to get id: {}", e))?,
            tip_id: row.get(1).map_err(|e| format!("Failed to get tip_id: {}", e))?,
            audio_id: row.get(2).map_err(|e| format!("Failed to get audio_id: {}", e))?,
            file_name: row.get(3).map_err(|e| format!("Failed to get file_name: {}", e))?,
            file_format: row.get(4).map_err(|e| format!("Failed to get file_format: {}", e))?,
            file_size: row.get(5).map_err(|e| format!("Failed to get file_size: {}", e))?,
            duration: row.get(6).map_err(|e| format!("Failed to get duration: {}", e))?,
            transcription: row.get(7).map_err(|e| format!("Failed to get transcription: {}", e))?,
            transcription_confidence: row.get(8).map_err(|e| format!("Failed to get confidence: {}", e))?,
            created_at: row.get(9).map_err(|e| format!("Failed to get created_at: {}", e))?,
            updated_at: row.get(10).map_err(|e| format!("Failed to get updated_at: {}", e))?,
        });
    }

    Ok(audio_files)
}

/// 更新音频转录
pub async fn update_audio_transcription(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
    transcription: &str,
    confidence: Option<f64>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let updated_at = Utc::now().timestamp_millis();

    conn.execute(
        "UPDATE tip_audio_files 
         SET transcription = ?1, transcription_confidence = ?2, updated_at = ?3 
         WHERE audio_id = ?4",
        params![transcription, confidence, updated_at, audio_id],
    )
    .await
    .map_err(|e| format!("Failed to update transcription: {}", e))?;

    Ok(())
}

/// 删除音频文件
pub async fn delete_audio_file_from_db(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let result = conn
        .execute(
            "DELETE FROM tip_audio_files WHERE audio_id = ?1",
            params![audio_id],
        )
        .await
        .map_err(|e| format!("Failed to delete audio file: {}", e))?;

    if result == 0 {
        return Err(format!("Audio file not found: {}", audio_id));
    }

    Ok(())
}

/// 获取音频文件统计信息
pub async fn get_audio_stats(
    db_manager: &UnifiedDbManager,
) -> Result<(i64, i64, i64), String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let mut rows = conn
        .query(
            "SELECT COUNT(*) as total_files, 
                    SUM(file_size) as total_size,
                    SUM(CASE WHEN transcription IS NOT NULL THEN 1 ELSE 0 END) as transcribed_files
             FROM tip_audio_files",
            (),
        )
        .await
        .map_err(|e| format!("Failed to query audio stats: {}", e))?;

    if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read stats: {}", e))? {
        let total_files: i64 = row.get(0).unwrap_or(0);
        let total_size: Option<i64> = row.get(1).unwrap_or(None);
        let transcribed_files: i64 = row.get(2).unwrap_or(0);
        
        Ok((total_files, total_size.unwrap_or(0), transcribed_files))
    } else {
        Ok((0, 0, 0))
    }
} 