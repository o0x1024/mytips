use anyhow::Result;
use libsql::params;
use crate::db::UnifiedDbManager;
use super::{AudioFileInfo, AudioFile};
use base64::Engine;

pub async fn get_audio_file_info(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<AudioFileInfo, String> {
    let conn = db_manager.get_conn().await.map_err(|e| format!("Database connection failed: {}", e))?;

    let mut rows = conn.query(
        "SELECT id, tip_id, audio_id, file_name, file_format, file_size, duration, transcription, transcription_confidence, created_at, updated_at FROM tip_audio_files WHERE audio_id = ?1",
        params![audio_id],
    ).await.map_err(|e| format!("Failed to query audio file: {}", e))?;

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

pub async fn get_audio_file_data(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<Vec<u8>, String> {
    let conn = db_manager.get_conn().await.map_err(|e| format!("Database connection failed: {}", e))?;

    let query = "SELECT audio_data FROM tip_audio_files WHERE audio_id = ?1";
    
    let mut rows = conn.query(query, params![audio_id]).await.map_err(|e| format!("Failed to get audio data: {}", e))?;
    
    if let Some(row) = rows.next().await.map_err(|e| format!("Failed to read row: {}", e))? {
        row.get::<Vec<u8>>(0).map_err(|e| format!("Failed to get audio_data: {}", e))
    } else {
        Err(format!("Audio file with ID {} not found", audio_id))
    }
}

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

pub async fn update_audio_file_data(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
    new_audio_data: Vec<u8>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await
        .map_err(|e| format!("Database connection failed: {}", e))?;

    let query = "UPDATE tip_audio_files SET audio_data = ?1, updated_at = ?2 WHERE audio_id = ?3";
    let updated_at = chrono::Utc::now().timestamp_millis();
    
    let affected_rows = conn.execute(
        query,
        params![new_audio_data, updated_at, audio_id]
    ).await.map_err(|e| format!("Failed to update audio data: {}", e))?;

    if affected_rows == 0 {
        return Err(format!("Audio file with ID {} not found", audio_id));
    }

    Ok(())
}

pub async fn save_audio_file_to_db(
    db_manager: &UnifiedDbManager,
    audio_data: super::AudioData,
) -> Result<String, String> {
    let conn = db_manager.get_conn().await.map_err(|e| format!("Database connection failed: {}", e))?;
    let audio_file = AudioFile::new(
        audio_data.tip_id,
        audio_data.file_name.unwrap_or_default(),
        audio_data.file_format,
        base64::engine::general_purpose::STANDARD.decode(audio_data.audio_data).unwrap(),
        audio_data.duration,
    );
    let audio_id = audio_file.audio_id.clone();
    conn.execute(
        "INSERT INTO tip_audio_files (id, tip_id, audio_id, file_name, file_format, audio_data, file_size, duration, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            audio_file.id,
            audio_file.tip_id,
            audio_file.audio_id,
            audio_file.file_name,
            audio_file.file_format,
            audio_file.audio_data,
            audio_file.file_size,
            audio_file.duration,
            audio_file.created_at,
            audio_file.updated_at,
        ],
    ).await.map_err(|e| format!("Failed to save audio file: {}", e))?;
    Ok(audio_id)
}

pub async fn delete_audio_file_from_db(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| format!("Database connection failed: {}", e))?;
    conn.execute(
        "DELETE FROM tip_audio_files WHERE audio_id = ?1",
        params![audio_id],
    ).await.map_err(|e| format!("Failed to delete audio file: {}", e))?;
    Ok(())
}

pub async fn update_audio_transcription(
    db_manager: &UnifiedDbManager,
    audio_id: &str,
    transcription: &str,
    confidence: Option<f64>,
) -> Result<(), String> {
    let conn = db_manager.get_conn().await.map_err(|e| format!("Database connection failed: {}", e))?;
    conn.execute(
        "UPDATE tip_audio_files SET transcription = ?1, transcription_confidence = ?2, updated_at = ?3 WHERE audio_id = ?4",
        params![transcription, confidence, chrono::Utc::now().timestamp_millis(), audio_id],
    ).await.map_err(|e| format!("Failed to update transcription: {}", e))?;
    Ok(())
} 