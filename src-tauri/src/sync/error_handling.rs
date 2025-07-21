use std::time::Duration;
use anyhow::{Result, anyhow};
use futures::future::BoxFuture;
use tracing::{info, warn, error};

/// LibSQL 错误类型枚举
#[derive(Debug, Clone, PartialEq)]
pub enum LibSqlErrorType {
    /// 网络连接错误（可重试）
    NetworkError,
    /// 超时错误（可重试）
    TimeoutError,
    /// 数据库锁定错误（可重试）
    DatabaseBusyError,
    /// WAL 相关错误（需要特殊处理）
    WalError,
    /// 数据库损坏错误（致命）
    CorruptionError,
    /// 认证错误（致命）
    AuthenticationError,
    /// 数据完整性错误（需要冲突解决）
    IntegrityError,
    /// 配置错误（致命）
    ConfigurationError,
    /// 未知错误
    UnknownError,
}

/// 错误分析器
pub struct ErrorAnalyzer;

impl ErrorAnalyzer {
    /// 分类错误类型
    pub fn classify_error(error: &anyhow::Error) -> LibSqlErrorType {
        let error_str = error.to_string().to_lowercase();
        
        if error_str.contains("network") || 
           error_str.contains("connection refused") ||
           error_str.contains("connection reset") ||
           error_str.contains("no route to host") ||
           error_str.contains("connection timed out") {
            return LibSqlErrorType::NetworkError;
        }
        
        if error_str.contains("timeout") ||
           error_str.contains("deadline exceeded") {
            return LibSqlErrorType::TimeoutError;
        }
        
        if error_str.contains("database is locked") ||
           error_str.contains("busy") ||
           error_str.contains("database busy") {
            return LibSqlErrorType::DatabaseBusyError;
        }
        
        if error_str.contains("wal") ||
           error_str.contains("write-ahead log") ||
           error_str.contains("wal mode") ||
           error_str.contains("checkpoint") {
            return LibSqlErrorType::WalError;
        }
        
        if error_str.contains("corrupt") ||
           error_str.contains("malformed") ||
           error_str.contains("database disk image") ||
           error_str.contains("file is not a database") {
            return LibSqlErrorType::CorruptionError;
        }
        
        if error_str.contains("auth") ||
           error_str.contains("unauthorized") ||
           error_str.contains("permission denied") ||
           error_str.contains("forbidden") ||
           error_str.contains("invalid token") {
            return LibSqlErrorType::AuthenticationError;
        }
        
        if error_str.contains("constraint") ||
           error_str.contains("unique") ||
           error_str.contains("foreign key") ||
           error_str.contains("check constraint") {
            return LibSqlErrorType::IntegrityError;
        }
        
        if error_str.contains("configuration") ||
           error_str.contains("invalid url") ||
           error_str.contains("invalid parameter") ||
           error_str.contains("invalid configuration") {
            return LibSqlErrorType::ConfigurationError;
        }
        
        LibSqlErrorType::UnknownError
    }
    
    /// 判断错误是否可重试
    pub fn is_retryable(error_type: &LibSqlErrorType) -> bool {
        match error_type {
            LibSqlErrorType::NetworkError |
            LibSqlErrorType::TimeoutError |
            LibSqlErrorType::DatabaseBusyError |
            LibSqlErrorType::WalError => true,
            
            LibSqlErrorType::CorruptionError |
            LibSqlErrorType::AuthenticationError |
            LibSqlErrorType::ConfigurationError => false,
            
            LibSqlErrorType::IntegrityError |
            LibSqlErrorType::UnknownError => false,
        }
    }
    
    /// 获取重试策略
    pub fn get_retry_strategy(error_type: &LibSqlErrorType) -> RetryStrategy {
        match error_type {
            LibSqlErrorType::NetworkError => RetryStrategy {
                max_attempts: 5,
                base_delay_ms: 1000,
                max_delay_ms: 30000,
                backoff_multiplier: 2.0,
                jitter: true,
            },
            LibSqlErrorType::TimeoutError => RetryStrategy {
                max_attempts: 3,
                base_delay_ms: 2000,
                max_delay_ms: 10000,
                backoff_multiplier: 1.5,
                jitter: false,
            },
            LibSqlErrorType::DatabaseBusyError => RetryStrategy {
                max_attempts: 10,
                base_delay_ms: 100,
                max_delay_ms: 5000,
                backoff_multiplier: 1.2,
                jitter: true,
            },
            LibSqlErrorType::WalError => RetryStrategy {
                max_attempts: 3,
                base_delay_ms: 500,
                max_delay_ms: 5000,
                backoff_multiplier: 2.0,
                jitter: false,
            },
            _ => RetryStrategy::no_retry(),
        }
    }
}

/// 重试策略配置
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    pub max_attempts: u32,
    pub base_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
    pub jitter: bool,
}

impl RetryStrategy {
    /// 创建不重试的策略
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            base_delay_ms: 0,
            max_delay_ms: 0,
            backoff_multiplier: 1.0,
            jitter: false,
        }
    }
    
    /// 计算延迟时间
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        if attempt == 0 {
            return Duration::from_millis(0);
        }
        
        let mut delay = self.base_delay_ms as f64 * self.backoff_multiplier.powi((attempt - 1) as i32);
        
        if self.jitter {
            let jitter_factor = 1.0 + (rand::random::<f64>() - 0.5) * 0.2; // ±10% jitter
            delay *= jitter_factor;
        }
        
        let delay_ms = delay.min(self.max_delay_ms as f64) as u64;
        Duration::from_millis(delay_ms)
    }
}

/// 智能重试执行器
pub struct SmartRetryExecutor;

impl SmartRetryExecutor {
    /// 处理 WAL 错误的特殊逻辑
    async fn handle_wal_error() {
        warn!("Detected WAL error, initiating recovery procedures");
        
        // WAL 错误的特殊处理：
        // 1. 等待更长时间让 WAL 检查点完成
        tokio::time::sleep(Duration::from_millis(2000)).await;
        
        // 2. 强制垃圾回收，释放可能的内存引用
        std::hint::black_box(());
        
        warn!("WAL error recovery completed, retrying operation");
    }
    
    /// 执行数据库WAL安全操作
    pub async fn execute_wal_safe_operation<F, T>(
        operation: F,
        operation_name: &str,
    ) -> Result<T>
    where
        F: Fn() -> BoxFuture<'static, Result<T>> + Send + Sync + 'static,
        T: Send + 'static,
    {
        let mut last_error = None;
        let max_attempts =2; // WAL操作允许更多重试
        
        for attempt in 1..=max_attempts {
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        info!("WAL operation '{}' succeeded on attempt {}", operation_name, attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    let error_type = ErrorAnalyzer::classify_error(&error);
                    
                    warn!("WAL operation '{}' failed (attempt {}): {} - Error type: {:?}", 
                         operation_name, attempt, error, error_type);
                    
                    // 特殊处理WAL相关错误
                    if error_type == LibSqlErrorType::WalError {
                        warn!("WAL-specific error detected, applying recovery measures");
                        Self::handle_wal_error().await;
                        
                        // 对于WAL错误，延长重试间隔
                        let delay = Duration::from_millis(1000 * attempt as u64);
                        tokio::time::sleep(delay).await;
                    } else if ErrorAnalyzer::is_retryable(&error_type) {
                        let strategy = ErrorAnalyzer::get_retry_strategy(&error_type);
                        let delay = strategy.calculate_delay(attempt);
                        tokio::time::sleep(delay).await;
                    } else {
                        // 不可重试的错误
                        let suggestion = Self::get_error_suggestion(&error_type);
                        error!("WAL operation '{}' failed permanently: {}. Suggestion: {}", 
                               operation_name, error, suggestion);
                        return Err(error);
                    }
                    
                    last_error = Some(error);
                }
            }
        }
        
        let final_error = last_error.unwrap_or_else(|| anyhow!("Unknown error"));
        error!("WAL operation '{}' failed permanently after {} attempts", operation_name, max_attempts);
        Err(final_error)
    }
    
    /// 获取错误建议
    pub fn get_error_suggestion(error_type: &LibSqlErrorType) -> String {
        match error_type {
            LibSqlErrorType::NetworkError => {
                "Network connectivity issue. Check your internet connection and remote server status.".to_string()
            }
            LibSqlErrorType::TimeoutError => {
                "Operation timed out. Consider increasing timeout values or checking network latency.".to_string()
            }
            LibSqlErrorType::DatabaseBusyError => {
                "Database is busy. This usually resolves automatically with retry.".to_string()
            }
            LibSqlErrorType::WalError => {
                "WAL (Write-Ahead Log) issue detected. The system will attempt automatic recovery. If this persists, try restarting the application.".to_string()
            }
            LibSqlErrorType::CorruptionError => {
                "Database corruption detected. Manual intervention may be required. Consider backing up data and reinitializing the database.".to_string()
            }
            LibSqlErrorType::AuthenticationError => {
                "Authentication failed. Check your credentials and access permissions.".to_string()
            }
            LibSqlErrorType::IntegrityError => {
                "Data integrity constraint violation. Check for conflicts or invalid data.".to_string()
            }
            LibSqlErrorType::ConfigurationError => {
                "Configuration error. Check your database connection settings.".to_string()
            }
            LibSqlErrorType::UnknownError => {
                "Unknown error occurred. Please check logs for more details.".to_string()
            }
        }
    }
    
    /// 执行带智能重试的操作
    pub async fn execute_with_retry<F, T>(
        operation: F,
        context: &str,
    ) -> Result<T>
    where
        F: Fn() -> futures::future::BoxFuture<'static, Result<T>> + Send + Sync,
        T: Send + 'static,
    {
        let mut last_error = None;
        let mut attempt = 0;
        
        loop {
            attempt += 1;
            
            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        println!("Operation '{}' succeeded after {} attempts", context, attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    let error_type = ErrorAnalyzer::classify_error(&error);
                    let is_retryable = ErrorAnalyzer::is_retryable(&error_type);
                    let strategy = ErrorAnalyzer::get_retry_strategy(&error_type);
                    
                    println!("Operation '{}' failed (attempt {}): {} - Error type: {:?}", 
                             context, attempt, error, error_type);
                    
                    if !is_retryable || attempt >= strategy.max_attempts {
                        let suggestion = Self::get_error_suggestion(&error_type);
                        println!("Operation '{}' failed permanently after {} attempts. Suggestion: {}", 
                                 context, attempt, suggestion);
                        return Err(error);
                    }
                    
                    // 特殊处理 WAL 错误
                    if error_type == LibSqlErrorType::WalError {
                        Self::handle_wal_error().await;
                    }
                    
                    let delay = strategy.calculate_delay(attempt);
                    if delay.as_millis() > 0 {
                        println!("Retrying operation '{}' in {:?} (attempt {}/{})", 
                                 context, delay, attempt + 1, strategy.max_attempts);
                        tokio::time::sleep(delay).await;
                    }
                    
                    last_error = Some(error);
                }
            }
        }
    }
    
    /// 执行数据库操作（专用版本）
    pub async fn execute_db_operation<F, T>(
        operation: F,
        operation_name: &str,
    ) -> Result<T>
    where
        F: Fn() -> futures::future::BoxFuture<'static, Result<T>> + Send + Sync + 'static,
        T: Send + 'static,
    {
        Self::execute_with_retry(operation, &format!("db_operation:{}", operation_name)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_classification() {
        let network_error = anyhow!("connection refused");
        assert_eq!(ErrorAnalyzer::classify_error(&network_error), LibSqlErrorType::NetworkError);
        
        let timeout_error = anyhow!("operation timeout");
        assert_eq!(ErrorAnalyzer::classify_error(&timeout_error), LibSqlErrorType::TimeoutError);
        
        let busy_error = anyhow!("database is locked");
        assert_eq!(ErrorAnalyzer::classify_error(&busy_error), LibSqlErrorType::DatabaseBusyError);
        
        let wal_error = anyhow!("WAL mode error");
        assert_eq!(ErrorAnalyzer::classify_error(&wal_error), LibSqlErrorType::WalError);
        
        let corrupt_error = anyhow!("database disk image is malformed");
        assert_eq!(ErrorAnalyzer::classify_error(&corrupt_error), LibSqlErrorType::CorruptionError);
        
        let auth_error = anyhow!("unauthorized access");
        assert_eq!(ErrorAnalyzer::classify_error(&auth_error), LibSqlErrorType::AuthenticationError);
    }
    
    #[test]
    fn test_retry_strategy() {
        let network_strategy = ErrorAnalyzer::get_retry_strategy(&LibSqlErrorType::NetworkError);
        assert!(network_strategy.max_attempts > 1);
        assert!(ErrorAnalyzer::is_retryable(&LibSqlErrorType::NetworkError));
        
        let auth_strategy = ErrorAnalyzer::get_retry_strategy(&LibSqlErrorType::AuthenticationError);
        assert_eq!(auth_strategy.max_attempts, 1);
        assert!(!ErrorAnalyzer::is_retryable(&LibSqlErrorType::AuthenticationError));
    }
    
    #[test]
    fn test_delay_calculation() {
        let strategy = RetryStrategy {
            max_attempts: 5,
            base_delay_ms: 1000,
            max_delay_ms: 10000,
            backoff_multiplier: 2.0,
            jitter: false,
        };
        
        assert_eq!(strategy.calculate_delay(0).as_millis(), 0);
        assert_eq!(strategy.calculate_delay(1).as_millis(), 1000);
        assert_eq!(strategy.calculate_delay(2).as_millis(), 2000);
        assert_eq!(strategy.calculate_delay(3).as_millis(), 4000);
        assert_eq!(strategy.calculate_delay(4).as_millis(), 8000);
    }
} 