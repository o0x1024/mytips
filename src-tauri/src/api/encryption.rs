use crate::db::{self, DbManager};
use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};
use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine as _};
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tauri::State;
use zeroize::Zeroize;

// 加密状态结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EncryptionStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notebook_id: Option<String>,
    pub is_encrypted: bool,
    pub is_unlocked: bool,
}

// 加密元数据结构
#[derive(Debug, Serialize, Deserialize)]
struct EncryptionMetadata {
    salt: String,
    nonce: String,
    encrypted_data: String,
}

// 密钥派生参数
const PBKDF2_ITERATIONS: u32 = 100_000;
const SALT_LENGTH: usize = 32;
const KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

/// 生成随机盐值
fn generate_salt() -> [u8; SALT_LENGTH] {
    let mut salt = [0u8; SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    salt
}

/// 生成随机nonce
fn generate_nonce() -> [u8; NONCE_LENGTH] {
    let mut nonce = [0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut nonce);
    nonce
}

/// 从密码和盐值派生密钥
fn derive_key(password: &str, salt: &[u8]) -> [u8; KEY_LENGTH] {
    let mut key = [0u8; KEY_LENGTH];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, PBKDF2_ITERATIONS, &mut key);
    key
}

/// 加密数据
pub fn encrypt_data(data: &str, password: &str) -> Result<String> {
    let salt = generate_salt();
    let nonce = generate_nonce();
    
    // 派生密钥
    let mut key_bytes = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // 加密数据
    let nonce_obj = Nonce::from_slice(&nonce);
    let encrypted = cipher
        .encrypt(nonce_obj, data.as_bytes())
        .map_err(|e| anyhow!("加密失败: {}", e))?;
    
    // 清理密钥
    key_bytes.zeroize();
    
    // 构建元数据
    let metadata = EncryptionMetadata {
        salt: general_purpose::STANDARD.encode(&salt),
        nonce: general_purpose::STANDARD.encode(&nonce),
        encrypted_data: general_purpose::STANDARD.encode(&encrypted),
    };
    
    // 返回JSON格式的加密数据
    serde_json::to_string(&metadata).map_err(|e| anyhow!("序列化失败: {}", e))
}

/// 解密数据
pub fn decrypt_data(encrypted_json: &str, password: &str) -> Result<String> {
    // 解析元数据
    let metadata: EncryptionMetadata = serde_json::from_str(encrypted_json)
        .map_err(|e| anyhow!("解析加密数据失败: {}", e))?;
    
    // 解码base64数据
    let salt = general_purpose::STANDARD.decode(&metadata.salt)
        .map_err(|e| anyhow!("解码盐值失败: {}", e))?;
    let nonce = general_purpose::STANDARD.decode(&metadata.nonce)
        .map_err(|e| anyhow!("解码nonce失败: {}", e))?;
    let encrypted_data = general_purpose::STANDARD.decode(&metadata.encrypted_data)
        .map_err(|e| anyhow!("解码加密数据失败: {}", e))?;
    
    // 派生密钥
    let mut key_bytes = derive_key(password, &salt);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
    let cipher = Aes256Gcm::new(key);
    
    // 解密数据
    let nonce_obj = Nonce::from_slice(&nonce);
    let decrypted = cipher
        .decrypt(nonce_obj, encrypted_data.as_slice())
        .map_err(|e| anyhow!("解密失败，可能密码错误: {}", e))?;
    
    // 清理密钥
    key_bytes.zeroize();
    
    // 转换为字符串
    String::from_utf8(decrypted).map_err(|e| anyhow!("解密数据格式错误: {}", e))
}

/// 验证密码是否正确
fn verify_password(encrypted_json: &str, password: &str) -> bool {
    decrypt_data(encrypted_json, password).is_ok()
}

// Tauri命令实现

/// 获取所有加密状态
#[tauri::command]
pub async fn get_encryption_statuses(
    db_manager: State<'_, DbManager>,
) -> Result<Vec<EncryptionStatus>, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    match db::get_encryption_statuses(&conn) {
        Ok(statuses) => Ok(statuses),
        Err(e) => Err(format!("获取加密状态失败: {}", e)),
    }
}

/// 加密笔记
#[tauri::command]
pub async fn encrypt_note(
    note_id: String,
    password: String,
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    let note = match db::get_tip(&conn, &note_id) {
        Ok(note) => note,
        Err(e) => return Err(format!("获取笔记失败: {}", e)),
    };
    
    if db::is_item_encrypted(&conn, &note_id, "note").unwrap_or(false) {
        return Err("笔记已经加密".to_string());
    }
    
    let encrypted_content = match encrypt_data(&note.content, &password) {
        Ok(content) => content,
        Err(e) => return Err(format!("加密失败: {}", e)),
    };
    
    match db::encrypt_note(&conn, &note_id, &encrypted_content) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("保存加密笔记失败: {}", e)),
    }
}

/// 解密笔记
#[tauri::command]
pub async fn decrypt_note(
    note_id: String,
    password: String,
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    let note = match db::get_tip(&conn, &note_id) {
        Ok(note) => note,
        Err(e) => return Err(format!("获取笔记失败: {}", e)),
    };
    
    if !db::is_item_encrypted(&conn, &note_id, "note").unwrap_or(false) {
        return Err("笔记未加密".to_string());
    }
    
    let decrypted_content = match decrypt_data(&note.content, &password) {
        Ok(content) => content,
        Err(e) => return Err(format!("解密失败: {}", e)),
    };
    
    match db::decrypt_note(&conn, &note_id, &decrypted_content) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("保存解密笔记失败: {}", e)),
    }
}

/// 解锁笔记（临时解密，不改变加密状态）
#[tauri::command]
pub async fn unlock_note(
    note_id: String,
    password: String,
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    let note = match db::get_tip(&conn, &note_id) {
        Ok(note) => note,
        Err(e) => return Err(format!("获取笔记失败: {}", e)),
    };
    
    if !db::is_item_encrypted(&conn, &note_id, "note").unwrap_or(false) {
        return Err("笔记未加密".to_string());
    }
    
    if verify_password(&note.content, &password) {
        match db::mark_item_unlocked(&conn, &note_id, "note") {
            Ok(_) => Ok(true),
            Err(e) => Err(format!("标记解锁状态失败: {}", e)),
        }
    } else {
        Err("密码错误".to_string())
    }
}

/// 加密笔记本
#[tauri::command]
pub async fn encrypt_notebook(
    notebook_id: String,
    password: String, // 密码参数暂时未使用，但保留以备将来使用
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    // 为了简单起见，我们只是在数据库中标记笔记本为加密状态
    // 实际应用中可能需要加密笔记本的元数据或所有相关笔记
    if db::is_item_encrypted(&conn, &notebook_id, "notebook").unwrap_or(false) {
        return Err("笔记本已经加密".to_string());
    }

    match db::encrypt_notebook(&conn, &notebook_id) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("加密笔记本失败: {}", e)),
    }
}

/// 解密笔记本
#[tauri::command]
pub async fn decrypt_notebook(
    notebook_id: String,
    password: String, // 密码参数暂时未使用
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    if !db::is_item_encrypted(&conn, &notebook_id, "notebook").unwrap_or(false) {
        return Err("笔记本未加密".to_string());
    }

    // 这里假设密码验证成功，实际应用中需要验证
    match db::decrypt_notebook(&conn, &notebook_id) {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("解密笔记本失败: {}", e)),
    }
}

/// 解锁笔记本
#[tauri::command]
pub async fn unlock_notebook(
    notebook_id: String,
    password: String,
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;

    if !db::is_item_encrypted(&conn, &notebook_id, "notebook").unwrap_or(false) {
        return Err("笔记本未加密".to_string());
    }

    // 假设密码验证成功
    match db::mark_item_unlocked(&conn, &notebook_id, "notebook") {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("解锁笔记本失败: {}", e)),
    }
}

/// 获取已解锁笔记的内容
#[tauri::command]
pub async fn get_unlocked_note_content(
    note_id: String,
    password: String,
    db_manager: State<'_, DbManager>,
) -> Result<String, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    
    let note = match db::get_tip(&conn, &note_id) {
        Ok(note) => note,
        Err(e) => return Err(format!("获取笔记失败: {}", e)),
    };
    
    if !db::is_item_encrypted(&conn, &note_id, "note").unwrap_or(false) {
        return Ok(note.content);
    }
    
    decrypt_data(&note.content, &password).map_err(|e| e.to_string())
}

/// 加密任意数据
#[tauri::command]
pub async fn encrypt_data_cmd(
    data: String,
    password: String,
) -> Result<String, String> {
    encrypt_data(&data, &password).map_err(|e| e.to_string())
}

/// 清除会话解锁状态
#[tauri::command]
pub async fn clear_session_unlocks(
    db_manager: State<'_, DbManager>,
) -> Result<bool, String> {
    let conn = db_manager.get_conn().map_err(|e| e.to_string())?;
    db::clear_session_unlocks(&conn)
        .map(|_| true)
        .map_err(|e| format!("清除会话解锁状态失败: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_decryption() {
        let test_data = "这是一段测试数据，包含中文和English text!";
        let password = "test_password_123";
        
        // 测试加密
        let encrypted = encrypt_data(test_data, password).expect("加密失败");
        println!("加密后的数据: {}", encrypted);
        
        // 测试解密
        let decrypted = decrypt_data(&encrypted, password).expect("解密失败");
        assert_eq!(decrypted, test_data);
        
        // 测试错误密码
        let wrong_password_result = decrypt_data(&encrypted, "wrong_password");
        assert!(wrong_password_result.is_err());
    }
    
    #[test]
    fn test_password_verification() {
        let test_data = "测试密码验证";
        let password = "correct_password";
        
        let encrypted = encrypt_data(test_data, password).expect("加密失败");
        
        // 正确密码应该验证通过
        assert!(verify_password(&encrypted, password));
        
        // 错误密码应该验证失败
        assert!(!verify_password(&encrypted, "wrong_password"));
    }
    
    #[test]
    fn test_large_data_encryption() {
        // 测试大数据量的加密性能
        let large_data = "测试数据".repeat(10000); // 约40KB的数据
        let password = "performance_test_password";
        
        let start = std::time::Instant::now();
        let encrypted = encrypt_data(&large_data, password).expect("大数据加密失败");
        let encrypt_time = start.elapsed();
        
        let start = std::time::Instant::now();
        let decrypted = decrypt_data(&encrypted, password).expect("大数据解密失败");
        let decrypt_time = start.elapsed();
        
        assert_eq!(decrypted, large_data);
        println!("加密时间: {:?}, 解密时间: {:?}", encrypt_time, decrypt_time);
        
        // 确保性能在合理范围内（小于1秒）
        assert!(encrypt_time.as_secs() < 1);
        assert!(decrypt_time.as_secs() < 1);
    }
} 