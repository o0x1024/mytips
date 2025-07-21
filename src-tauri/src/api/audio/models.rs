use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

/// 音频文件数据库模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFile {
    pub id: String,
    pub tip_id: String,
    pub audio_id: String,
    pub file_name: String,
    pub file_format: String,
    pub audio_data: Vec<u8>, // 音频二进制数据
    pub file_size: i64,
    pub duration: Option<i64>, // 时长（毫秒）
    pub transcription: Option<String>,
    pub transcription_confidence: Option<f64>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl AudioFile {
    /// 创建新的音频文件记录
    pub fn new(
        tip_id: String,
        file_name: String,
        file_format: String,
        audio_data: Vec<u8>,
        duration: Option<i64>,
    ) -> Self {
        let now = Utc::now().timestamp_millis();
        let audio_id = format!("audio_{}", Uuid::new_v4().to_string().replace("-", ""));
        
        Self {
            id: Uuid::new_v4().to_string(),
            tip_id,
            audio_id,
            file_name,
            file_format,
            file_size: audio_data.len() as i64,
            audio_data,
            duration,
            transcription: None,
            transcription_confidence: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// 更新转录文本
    pub fn update_transcription(&mut self, text: String, confidence: Option<f64>) {
        self.transcription = Some(text);
        self.transcription_confidence = confidence;
        self.updated_at = Utc::now().timestamp_millis();
    }
}

/// 音频格式枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    WebM,
    MP3,
    WAV,
    M4A,
    OGG,
}

impl AudioFormat {
    pub fn from_str(format: &str) -> Option<Self> {
        match format.to_lowercase().as_str() {
            "webm" => Some(AudioFormat::WebM),
            "mp3" => Some(AudioFormat::MP3),
            "wav" => Some(AudioFormat::WAV),
            "m4a" => Some(AudioFormat::M4A),
            "ogg" => Some(AudioFormat::OGG),
            _ => None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AudioFormat::WebM => "webm".to_string(),
            AudioFormat::MP3 => "mp3".to_string(),
            AudioFormat::WAV => "wav".to_string(),
            AudioFormat::M4A => "m4a".to_string(),
            AudioFormat::OGG => "ogg".to_string(),
        }
    }

    pub fn mime_type(&self) -> &'static str {
        match self {
            AudioFormat::WebM => "audio/webm",
            AudioFormat::MP3 => "audio/mpeg",
            AudioFormat::WAV => "audio/wav",
            AudioFormat::M4A => "audio/mp4",
            AudioFormat::OGG => "audio/ogg",
        }
    }
}

/// 语音转文字服务类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscriptionService {
    OpenAI,
    Azure,
    Local,
    Google,
}

impl TranscriptionService {
    pub fn from_str(service: &str) -> Option<Self> {
        match service.to_lowercase().as_str() {
            "openai" => Some(TranscriptionService::OpenAI),
            "azure" => Some(TranscriptionService::Azure),
            "local" => Some(TranscriptionService::Local),
            "google" => Some(TranscriptionService::Google),
            _ => None,
        }
    }
}

/// 音频处理错误类型
#[derive(Debug, thiserror::Error)]
pub enum AudioError {
    #[error("Audio file not found: {0}")]
    NotFound(String),
    
    #[error("Invalid audio format: {0}")]
    InvalidFormat(String),
    
    #[error("Audio file too large: {0} bytes")]
    FileTooLarge(i64),
    
    #[error("Transcription failed: {0}")]
    TranscriptionFailed(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Encoding error: {0}")]
    EncodingError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
}

impl From<AudioError> for String {
    fn from(error: AudioError) -> Self {
        error.to_string()
    }
} 