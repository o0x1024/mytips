use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

// 笔记类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum TipType {
    Text,
    Markdown,
    Code,
    AIPrompt,
}

// 从字符串转换为TipType
impl TryFrom<String> for TipType {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "text" => Ok(TipType::Text),
            "markdown" => Ok(TipType::Markdown),
            "code" => Ok(TipType::Code),
            "ai-prompt" => Ok(TipType::AIPrompt),
            _ => Err(anyhow!("Invalid tip type: {}", s)),
        }
    }
}

// 转换为字符串
impl From<TipType> for String {
    fn from(tip_type: TipType) -> Self {
        match tip_type {
            TipType::Text => "text".to_string(),
            TipType::Markdown => "markdown".to_string(),
            TipType::Code => "code".to_string(),
            TipType::AIPrompt => "ai-prompt".to_string(),
        }
    }
}

// 笔记模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tip {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tip_type: TipType,
    pub language: Option<String>,
    pub category_id: Option<String>,
    pub created_at: i64, // 毫秒级时间戳
    pub updated_at: i64, // 毫秒级时间戳
    // v3 新增字段
    pub version: Option<i32>,
    pub last_synced_at: Option<i64>,
    pub sync_hash: Option<String>,
    // v2 新增字段
    pub is_encrypted: Option<bool>,
    pub encryption_key_id: Option<String>,
    pub encrypted_content: Option<String>,
}

// 分类模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
    // v3 新增字段
    pub version: Option<i32>,
    pub last_synced_at: Option<i64>,
    pub sync_hash: Option<String>,
    // v2 新增字段
    pub is_encrypted: Option<bool>,
    pub encryption_key_id: Option<String>,
}

// 标签模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
    // v3 新增字段
    pub version: Option<i32>,
    pub last_synced_at: Option<i64>,
    pub sync_hash: Option<String>,
}

// 笔记标签关联
#[derive(Debug, Serialize, Deserialize)]
pub struct TipTag {
    pub tip_id: String,
    pub tag_id: String,
}

// AI角色模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIRole {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub is_default: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// AI对话模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIConversation {
    pub id: String,
    pub title: String,
    pub model: String,
    pub role_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// AI消息模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AIMessage {
    pub id: String,
    pub conversation_id: String,
    pub role: String, // 'user', 'assistant', 'system'
    pub content: String,
    pub created_at: i64,
}

// 提示词模板模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TipTemplate {
    pub id: String,
    pub name: String,
    pub template: String,
    pub category: Option<String>,
    pub description: Option<String>,
    pub is_system: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// 剪贴板历史记录模型
#[derive(Debug, Serialize, Deserialize)]
pub struct ClipboardHistory {
    pub id: i64,
    pub content: String,
    pub source: Option<String>,
    pub created_at: i64,
}

// v2 新增：加密密钥模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionKey {
    pub id: String,
    pub key_name: String,
    pub key_hash: String,
    pub salt: String,
    pub iterations: i32,
    pub algorithm: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// v2 新增：加密数据日志模型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptedDataLog {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub operation: String, // 'encrypt' or 'decrypt'
    pub encryption_key_id: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub created_at: i64,
}

// 同步模式枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncMode {
    Offline,
    Manual,
    Auto,
}

impl ToString for SyncMode {
    fn to_string(&self) -> String {
        match self {
            SyncMode::Offline => "OFFLINE".to_string(),
            SyncMode::Manual => "MANUAL".to_string(),
            SyncMode::Auto => "AUTO".to_string(),
        }
    }
}

impl From<String> for SyncMode {
    fn from(s: String) -> Self {
        match s.to_uppercase().as_str() {
            "AUTO" => SyncMode::Auto,
            "MANUAL" => SyncMode::Manual,
            _ => SyncMode::Offline,
        }
    }
}

// 同步状态枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncStatus {
    Pending,
    Synced,
    Failed,
    Conflict,
}

// 同步操作枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SyncOperation {
    Insert,
    Update,
    Delete,
}

// 冲突解决策略
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ConflictResolutionStrategy {
    LocalWins,
    RemoteWins,
    Merge,
    UserChoice,
}

// v3 新增：同步配置结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncConfig {
    pub id: String,
    pub remote_url: Option<String>,
    pub auth_token: Option<String>,
    pub sync_mode: SyncMode,
    pub sync_interval: i64,
    pub last_sync_at: i64,
    pub is_online: bool,
    pub auto_sync_enabled: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

// v3 新增：同步状态记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatusRecord {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub operation: SyncOperation,
    pub sync_status: SyncStatus,
    pub error_message: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

// v3 新增：数据版本记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DataVersion {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub version: i64,
    pub hash: String,
    pub created_at: i64,
}

// v3 新增：冲突解决记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictResolution {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub strategy: ConflictResolutionStrategy,
    pub resolved_by: String,
    pub local_content: Option<String>,
    pub remote_content: Option<String>,
    pub resolved_content: Option<String>,
    pub created_at: i64,
}

// v3 新增：同步统计
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncStatistics {
    pub id: String,
    pub sync_session_id: String,
    pub table_name: String,
    pub total_records: i32,
    pub synced_records: i32,
    pub failed_records: i32,
    pub conflict_records: i32,
    pub start_time: i64,
    pub end_time: Option<i64>,
    pub duration_ms: Option<i32>,
} 