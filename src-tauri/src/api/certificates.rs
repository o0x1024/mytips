use anyhow::{anyhow, Result};
use rcgen::{Certificate, CertificateParams, DistinguishedName, DnType, KeyPair, SanType};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use time::{Duration, OffsetDateTime};
use tauri::command;
use ed25519_dalek::{SigningKey, VerifyingKey};
use pkcs8::EncodePrivateKey;
use rand_core::OsRng;
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use base64::{Engine as _, engine::general_purpose};
use std::collections::HashMap;
use pem;

#[derive(Debug, Serialize, Deserialize)]
pub struct CertificateInfo {
    pub name: String,
    pub cert_path: String,
    pub key_path: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateCertificatesResult {
    pub ca_cert: CertificateInfo,
    pub server_cert: CertificateInfo,
    pub client_cert: CertificateInfo,
    pub success: bool,
    pub message: String,
}

/// 生成私钥
fn generate_key_pair() -> Result<KeyPair> {
    Ok(KeyPair::generate()?)
}

/// 生成CA证书
fn generate_ca_cert(ca_key: &KeyPair) -> Result<Certificate> {
    let mut ca_params = CertificateParams::new(vec!["sqld dev CA".to_string()])?;
    
    // 设置证书有效期（3天）
    let not_before = OffsetDateTime::now_utc();
    let not_after = not_before + Duration::days(3);
    ca_params.not_before = not_before;
    ca_params.not_after = not_after;
    
    // 设置为CA证书
    ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    
    // 设置Distinguished Name
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, "sqld dev CA");
    ca_params.distinguished_name = dn;
    
    // 设置密钥用途
    ca_params.key_usages = vec![
        rcgen::KeyUsagePurpose::KeyCertSign,
        rcgen::KeyUsagePurpose::CrlSign,
    ];
    
    // 生成CA证书
    let cert = ca_params.self_signed(ca_key)?;
    Ok(cert)
}

/// 生成对等证书（服务器或客户端）
fn generate_peer_cert(
    ca_cert: &Certificate,
    ca_key: &KeyPair,
    peer_key: &KeyPair,
    common_name: &str,
    dns_names: Vec<String>,
) -> Result<Certificate> {
    let mut peer_params = CertificateParams::new(dns_names.clone())?;
    
    // 设置证书有效期（3天）
    let not_before = OffsetDateTime::now_utc();
    let not_after = not_before + Duration::days(3);
    peer_params.not_before = not_before;
    peer_params.not_after = not_after;
    
    // 设置为非CA证书
    peer_params.is_ca = rcgen::IsCa::NoCa;
    
    // 设置Distinguished Name
    let mut dn = DistinguishedName::new();
    dn.push(DnType::CommonName, common_name);
    peer_params.distinguished_name = dn;
    
    // 设置密钥用途
    peer_params.key_usages = vec![rcgen::KeyUsagePurpose::DigitalSignature];
    
    // 设置Subject Alternative Names
    if !dns_names.is_empty() {
        peer_params.subject_alt_names = dns_names
            .into_iter()
            .map(|name| SanType::DnsName(rcgen::Ia5String::try_from(name).unwrap()))
            .collect();
    }
    
    // 生成由CA签名的证书
    let cert = peer_params.signed_by(peer_key, ca_cert, ca_key)?;
    Ok(cert)
}

/// 保存证书链和私钥到文件
fn store_cert_chain_and_key(
    cert_chain: &[&Certificate],
    key: &KeyPair,
    name: &str,
    output_dir: &Path,
) -> Result<CertificateInfo> {
    let cert_file = output_dir.join(format!("{}_cert.pem", name));
    let key_file = output_dir.join(format!("{}_key.pem", name));
    
    // 写入证书链
    let mut cert_pem = String::new();
    for cert in cert_chain {
        cert_pem.push_str(&cert.pem());
    }
    fs::write(&cert_file, cert_pem)?;
    
    // 写入私钥
    let key_pem = key.serialize_pem();
    fs::write(&key_file, key_pem.as_bytes())?;
    
    // 获取证书过期时间
    let expires_at = if let Some(cert) = cert_chain.first() {
        cert.params().not_after.format(&time::format_description::well_known::Rfc3339)
            .unwrap_or_else(|_| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    };
    
    Ok(CertificateInfo {
        name: name.to_string(),
        cert_path: cert_file.to_string_lossy().to_string(),
        key_path: key_file.to_string_lossy().to_string(),
        expires_at,
    })
}

/// 生成开发用的X.509证书
#[command]
pub async fn generate_dev_certificates(output_dir: String) -> Result<GenerateCertificatesResult, String> {
    let output_path = Path::new(&output_dir);
    
    // 确保输出目录存在
    if let Err(e) = fs::create_dir_all(output_path) {
        return Ok(GenerateCertificatesResult {
            ca_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            server_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            client_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            success: false,
            message: format!("无法创建输出目录: {}", e),
        });
    }
    
    match generate_certificates_internal(output_path).await {
        Ok((ca_cert, server_cert, client_cert)) => {
            Ok(GenerateCertificatesResult {
                ca_cert,
                server_cert,
                client_cert,
                success: true,
                message: "证书生成成功！这些是开发用证书，将在3天后过期。".to_string(),
            })
        }
        Err(e) => Ok(GenerateCertificatesResult {
            ca_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            server_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            client_cert: CertificateInfo {
                name: "".to_string(),
                cert_path: "".to_string(),
                key_path: "".to_string(),
                expires_at: "".to_string(),
            },
            success: false,
            message: format!("证书生成失败: {}", e),
        }),
    }
}

async fn generate_certificates_internal(
    output_dir: &Path,
) -> Result<(CertificateInfo, CertificateInfo, CertificateInfo)> {
    // 生成CA密钥和证书
    let ca_key = generate_key_pair()?;
    let ca_cert = generate_ca_cert(&ca_key)?;
    let ca_info = store_cert_chain_and_key(&[&ca_cert], &ca_key, "ca", output_dir)?;
    
    // 生成服务器密钥和证书
    let server_key = generate_key_pair()?;
    let server_cert = generate_peer_cert(&ca_cert, &ca_key, &server_key, "sqld", vec!["sqld".to_string()])?;
    let server_info = store_cert_chain_and_key(&[&server_cert, &ca_cert], &server_key, "server", output_dir)?;
    
    // 生成客户端密钥和证书
    let client_key = generate_key_pair()?;
    let client_cert = generate_peer_cert(&ca_cert, &ca_key, &client_key, "sqld replica", vec![])?;
    let client_info = store_cert_chain_and_key(&[&client_cert, &ca_cert], &client_key, "client", output_dir)?;
    
    Ok((ca_info, server_info, client_info))
}

/// 获取默认证书保存目录
#[command]
pub async fn get_default_cert_directory() -> Result<String, String> {
    match dirs::home_dir() {
        Some(home) => {
            let cert_dir = home.join(".mytips").join("certificates");
            Ok(cert_dir.to_string_lossy().to_string())
        }
        None => Err("无法获取用户主目录".to_string()),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtKeyInfo {
    pub public_key_pem_path: String,
    pub public_key_base64_path: String,
    pub full_access_token: String,
    pub read_only_token: String,
    pub expires_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateJwtResult {
    pub jwt_info: Option<JwtKeyInfo>,
    pub success: bool,
    pub message: String,
}

/// 生成Ed25519密钥和JWT令牌
#[command]
pub async fn generate_jwt_keys(output_dir: String) -> Result<GenerateJwtResult, String> {
    let output_path = Path::new(&output_dir);
    
    // 确保输出目录存在
    if let Err(e) = fs::create_dir_all(output_path) {
        return Ok(GenerateJwtResult {
            jwt_info: None,
            success: false,
            message: format!("无法创建输出目录: {}", e),
        });
    }
    
    match generate_jwt_internal(output_path).await {
        Ok(jwt_info) => {
            Ok(GenerateJwtResult {
                jwt_info: Some(jwt_info),
                success: true,
                message: "JWT密钥和令牌生成成功！令牌将在3天后过期。".to_string(),
            })
        }
        Err(e) => Ok(GenerateJwtResult {
            jwt_info: None,
            success: false,
            message: format!("JWT生成失败: {}", e),
        }),
    }
}

async fn generate_jwt_internal(output_dir: &Path) -> Result<JwtKeyInfo> {
    // 生成Ed25519私钥
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    
    // 获取公钥的PEM格式
    let public_key_bytes = verifying_key.to_bytes();
    let public_key_pem = format!(
        "-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n",
        general_purpose::STANDARD.encode(&public_key_bytes)
    );
    
    // 获取公钥的base64格式（URL安全，无填充）
    let mut public_key_base64 = general_purpose::URL_SAFE_NO_PAD.encode(&public_key_bytes);
    // 移除末尾的等号（如果有的话）
    while public_key_base64.ends_with('=') {
        public_key_base64.pop();
    }
    
    // 保存公钥文件
    let pem_path = output_dir.join("jwt_key.pem");
    let base64_path = output_dir.join("jwt_key.base64");
    
    fs::write(&pem_path, &public_key_pem)
        .map_err(|e| anyhow!("写入PEM文件失败: {}", e))?;
    fs::write(&base64_path, &public_key_base64)
        .map_err(|e| anyhow!("写入base64文件失败: {}", e))?;
    
    // 创建JWT声明
    let exp = chrono::Utc::now() + chrono::Duration::days(90);
    let exp_timestamp = exp.timestamp();
    
    // 创建JWT头部
    let header = Header {
        alg: Algorithm::EdDSA,
        ..Default::default()
    };
    
    // 创建编码密钥 - 使用 Ed25519 私钥的 PKCS8 DER 格式
    let private_key_der = signing_key.to_pkcs8_der()
        .map_err(|e| anyhow!("转换私钥为DER格式失败: {}", e))?;
    let encoding_key = EncodingKey::from_ed_der(private_key_der.as_bytes());
    
    // 生成完全访问令牌
    let mut full_claims = serde_json::Map::new();
    full_claims.insert("exp".to_string(), serde_json::Value::Number(serde_json::Number::from(exp_timestamp)));
    
    let full_access_token = encode(&header, &full_claims, &encoding_key)
        .map_err(|e| anyhow!("生成完全访问令牌失败: {}", e))?;
    
    // 生成只读访问令牌
    let mut ro_claims = serde_json::Map::new();
    ro_claims.insert("exp".to_string(), serde_json::Value::Number(serde_json::Number::from(exp_timestamp)));
    ro_claims.insert("a".to_string(), serde_json::Value::String("ro".to_string()));
    
    let read_only_token = encode(&header, &ro_claims, &encoding_key)
        .map_err(|e| anyhow!("生成只读令牌失败: {}", e))?;
    
    Ok(JwtKeyInfo {
        public_key_pem_path: pem_path.to_string_lossy().to_string(),
        public_key_base64_path: base64_path.to_string_lossy().to_string(),
        full_access_token,
        read_only_token,
        expires_at: exp.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
    })
}

/// 获取默认JWT保存目录
#[command]
pub async fn get_default_jwt_directory() -> Result<String, String> {
    match dirs::home_dir() {
        Some(home) => {
            let jwt_dir = home.join(".mytips").join("jwt");
            Ok(jwt_dir.to_string_lossy().to_string())
        }
        None => Err("无法获取用户主目录".to_string()),
    }
}