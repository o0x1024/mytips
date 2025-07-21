use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use tokio::time::sleep;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use tracing::{info, warn, debug, error};
use chrono::Utc;
use libsql::{Connection, Database};
use super::monitoring::{PerformanceMonitor, StructuredLogger};

/// 连接健康检查器
pub struct ConnectionHealthChecker {
    /// 本地数据库连接
    local_db: Arc<Database>,
    /// 远程数据库连接
    remote_db: Arc<RwLock<Option<Database>>>,
    /// 健康检查配置
    config: Arc<RwLock<HealthCheckConfig>>,
    /// 连接状态
    connection_status: Arc<RwLock<ConnectionStatus>>,
    /// 健康检查任务句柄
    health_check_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    /// 结构化日志记录器
    structured_logger: Arc<StructuredLogger>,
    /// 重连回调函数
    reconnect_callbacks: Arc<Mutex<Vec<ReconnectCallback>>>,
}

/// 健康检查配置
#[derive(Debug, Clone)]
pub struct HealthCheckConfig {
    /// 是否启用健康检查
    pub enabled: bool,
    /// 检查间隔（秒）
    pub check_interval_seconds: u64,
    /// 连接超时时间（秒）
    pub connection_timeout_seconds: u64,
    /// 查询超时时间（秒）
    pub query_timeout_seconds: u64,
    /// 最大重试次数
    pub max_retry_attempts: u32,
    /// 重试间隔（毫秒）
    pub retry_interval_ms: u64,
    /// 是否自动重连
    pub auto_reconnect: bool,
    /// 重连时的退避策略
    pub backoff_strategy: BackoffStrategy,
}

impl Default for HealthCheckConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            check_interval_seconds: 30,      // 30秒检查一次
            connection_timeout_seconds: 10,  // 10秒连接超时
            query_timeout_seconds: 5,        // 5秒查询超时
            max_retry_attempts: 3,
            retry_interval_ms: 2000,         // 2秒重试间隔
            auto_reconnect: true,
            backoff_strategy: BackoffStrategy::Exponential,
        }
    }
}

/// 退避策略
#[derive(Debug, Clone)]
pub enum BackoffStrategy {
    /// 固定间隔
    Fixed,
    /// 线性增长
    Linear,
    /// 指数增长
    Exponential,
}

/// 连接状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    /// 本地连接状态
    pub local_status: DatabaseConnectionStatus,
    /// 远程连接状态
    pub remote_status: DatabaseConnectionStatus,
    /// 最后检查时间
    pub last_check_time: i64,
    /// 连续失败次数
    pub consecutive_failures: u32,
    /// 总检查次数
    pub total_checks: u64,
    /// 成功检查次数
    pub successful_checks: u64,
    /// 自动重连次数
    pub reconnect_attempts: u32,
}

impl Default for ConnectionStatus {
    fn default() -> Self {
        Self {
            local_status: DatabaseConnectionStatus::Unknown,
            remote_status: DatabaseConnectionStatus::Unknown,
            last_check_time: 0,
            consecutive_failures: 0,
            total_checks: 0,
            successful_checks: 0,
            reconnect_attempts: 0,
        }
    }
}

/// 数据库连接状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DatabaseConnectionStatus {
    /// 健康
    Healthy,
    /// 缓慢
    Slow,
    /// 断开连接
    Disconnected,
    /// 错误
    Error(String),
    /// 未知
    Unknown,
}

/// 健康检查结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// 检查时间
    pub check_time: i64,
    /// 本地数据库结果
    pub local_result: DatabaseHealthResult,
    /// 远程数据库结果
    pub remote_result: Option<DatabaseHealthResult>,
    /// 总体健康状态
    pub overall_healthy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseHealthResult {
    /// 连接状态
    pub status: DatabaseConnectionStatus,
    /// 响应时间（毫秒）
    pub response_time_ms: u64,
    /// 连接数量
    pub connection_count: Option<u32>,
    /// 错误信息
    pub error_message: Option<String>,
    /// 是否需要重连
    pub needs_reconnect: bool,
}

/// 重连回调函数类型
pub type ReconnectCallback = Box<dyn Fn(&str, bool) + Send + Sync>;

impl ConnectionHealthChecker {
    /// 创建新的连接健康检查器
    pub async fn new(
        local_db: Arc<Database>,
        remote_db: Arc<RwLock<Option<Database>>>,
        performance_monitor: Arc<PerformanceMonitor>,
        structured_logger: Arc<StructuredLogger>,
    ) -> Self {
        let config = Arc::new(RwLock::new(HealthCheckConfig::default()));
        let connection_status = Arc::new(RwLock::new(ConnectionStatus::default()));
        let health_check_handle = Arc::new(Mutex::new(None));
        let reconnect_callbacks = Arc::new(Mutex::new(Vec::new()));

        Self {
            local_db,
            remote_db,
            config,
            connection_status,
            health_check_handle,
            performance_monitor,
            structured_logger,
            reconnect_callbacks,
        }
    }

    /// 启动健康检查
    pub async fn start_health_check(&self) -> Result<()> {
        let config = self.config.read().await;
        if !config.enabled {
            info!("Connection health check is disabled");
            return Ok(());
        }
        
        let check_interval = config.check_interval_seconds;
        drop(config);

        // 停止现有的健康检查任务
        self.stop_health_check().await;

        // 创建新的健康检查任务
        let checker_clone = Arc::new(self.clone());
        let handle = tokio::spawn(async move {
            loop {
                let start_time = Instant::now();
                
                match checker_clone.perform_health_check().await {
                    Ok(result) => {
                        checker_clone.handle_health_check_result(result).await;
                    }
                    Err(e) => {
                        error!("Health check failed: {}", e);
                        checker_clone.handle_health_check_error(e).await;
                    }
                }

                // 等待下一次检查
                let elapsed = start_time.elapsed();
                let sleep_duration = Duration::from_secs(check_interval)
                    .saturating_sub(elapsed);
                
                if sleep_duration > Duration::from_millis(100) {
                    sleep(sleep_duration).await;
                }
            }
        });

        *self.health_check_handle.lock().await = Some(handle);
        
        info!("Connection health check started with interval: {}s", check_interval);
        Ok(())
    }

    /// 停止健康检查
    pub async fn stop_health_check(&self) {
        let mut handle_guard = self.health_check_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
            info!("Connection health check stopped");
        }
    }

    /// 执行健康检查
    pub async fn perform_health_check(&self) -> Result<HealthCheckResult> {
        let check_time = Utc::now().timestamp_millis();
        
        debug!("Performing connection health check");

        // 记录操作开始
        self.structured_logger.log_sync_operation(
            "health_check_start",
            0,
            0,
            0,
            0,
        );

        // 检查本地数据库
        let local_result = self.check_local_database().await;
        
        // 检查远程数据库
        let remote_result = self.check_remote_database().await;

        let overall_healthy = local_result.status == DatabaseConnectionStatus::Healthy
            && remote_result
                .as_ref()
                .map(|r| r.status == DatabaseConnectionStatus::Healthy)
                .unwrap_or(true);

        let result = HealthCheckResult {
            check_time,
            local_result,
            remote_result,
            overall_healthy,
        };

        // 记录操作完成
        self.structured_logger.log_db_operation(
            "health_check_complete",
            "all",
            "",
            (Utc::now().timestamp_millis() - check_time) as u64,
            result.overall_healthy,
            None,
        );

        debug!("Health check completed: overall_healthy={}", result.overall_healthy);
        
        Ok(result)
    }

    /// 检查本地数据库
    async fn check_local_database(&self) -> DatabaseHealthResult {
        let start_time = Instant::now();
        
        let config = self.config.read().await;
        let timeout = Duration::from_secs(config.query_timeout_seconds);
        drop(config);

        let check_result = tokio::time::timeout(timeout, async {
            let conn = self.local_db.connect()?;
            
            // 执行简单的查询测试连接
            let mut rows = conn.query("SELECT 1", ()).await?;
            rows.next().await?;
            
            // 获取连接信息
            let mut pragma_rows = conn.query("PRAGMA database_list", ()).await?;
            let mut db_count = 0;
            while pragma_rows.next().await?.is_some() {
                db_count += 1;
            }
            
            Ok::<u32, anyhow::Error>(db_count)
        }).await;

        let response_time_ms = start_time.elapsed().as_millis() as u64;

        match check_result {
            Ok(Ok(connection_count)) => {
                let status = if response_time_ms > 1000 {
                    DatabaseConnectionStatus::Slow
                } else {
                    DatabaseConnectionStatus::Healthy
                };

                DatabaseHealthResult {
                    status,
                    response_time_ms,
                    connection_count: Some(connection_count),
                    error_message: None,
                    needs_reconnect: false,
                }
            }
            Ok(Err(e)) => DatabaseHealthResult {
                status: DatabaseConnectionStatus::Error(e.to_string()),
                response_time_ms,
                connection_count: None,
                error_message: Some(e.to_string()),
                needs_reconnect: true,
            },
            Err(_) => DatabaseHealthResult {
                status: DatabaseConnectionStatus::Error("Timeout".to_string()),
                response_time_ms,
                connection_count: None,
                error_message: Some("Query timeout".to_string()),
                needs_reconnect: false,
            },
        }
    }

    /// 检查远程数据库
    async fn check_remote_database(&self) -> Option<DatabaseHealthResult> {
        let start_time = Instant::now();
        
        let remote_db_guard = self.remote_db.read().await;
        if let Some(remote_db) = remote_db_guard.as_ref() {
            let config = self.config.read().await;
            let timeout = Duration::from_secs(config.query_timeout_seconds);
            drop(config);
    
            let check_result = tokio::time::timeout(timeout, async {
                let conn = remote_db.connect()?;
                let mut rows = conn.query("SELECT 1", ()).await?;
                rows.next().await?;
                Ok::<(), anyhow::Error>(())
            }).await;
    
            let response_time_ms = start_time.elapsed().as_millis() as u64;
    
            match check_result {
                Ok(Ok(_)) => {
                    let status = if response_time_ms > 2000 {
                        DatabaseConnectionStatus::Slow
                    } else {
                        DatabaseConnectionStatus::Healthy
                    };
    
                    Some(DatabaseHealthResult {
                        status,
                        response_time_ms,
                        connection_count: None,
                        error_message: None,
                        needs_reconnect: false,
                    })
                }
                Ok(Err(e)) => Some(DatabaseHealthResult {
                    status: DatabaseConnectionStatus::Error(e.to_string()),
                    response_time_ms,
                    connection_count: None,
                    error_message: Some(e.to_string()),
                    needs_reconnect: true,
                }),
                Err(_) => Some(DatabaseHealthResult {
                    status: DatabaseConnectionStatus::Error("Timeout".to_string()),
                    response_time_ms,
                    connection_count: None,
                    error_message: Some("Remote query timeout".to_string()),
                    needs_reconnect: false,
                }),
            }
        } else {
            Some(DatabaseHealthResult {
                status: DatabaseConnectionStatus::Disconnected,
                response_time_ms: 0,
                connection_count: None,
                error_message: Some("Remote database not configured".to_string()),
                needs_reconnect: true,
            })
        }
    }

    /// 处理健康检查结果
    async fn handle_health_check_result(&self, result: HealthCheckResult) {
        // 更新连接状态
        {
            let mut status = self.connection_status.write().await;
            status.local_status = result.local_result.status.clone();
            status.remote_status = result.remote_result
                .as_ref()
                .map(|r| r.status.clone())
                .unwrap_or(DatabaseConnectionStatus::Unknown);
            status.last_check_time = result.check_time;
            status.total_checks += 1;

            if result.overall_healthy {
                status.successful_checks += 1;
                status.consecutive_failures = 0;
            } else {
                status.consecutive_failures += 1;
            }
        }

        // 记录健康检查结果
        self.log_health_check_result(&result).await;

        // 如果健康检查失败，尝试恢复
        if !result.overall_healthy {
            self.handle_connection_failure(&result).await;
        }
    }

    /// 处理连接失败
    async fn handle_connection_failure(&self, result: &HealthCheckResult) {
        let config = self.config.read().await;
        let auto_reconnect = config.auto_reconnect;
        let max_retries = config.max_retry_attempts;
        drop(config);

        if !auto_reconnect {
            warn!("Auto-reconnect is disabled, skipping recovery attempt");
            return;
        }

        let mut status = self.connection_status.write().await;
        if status.consecutive_failures > max_retries {
            error!("Too many consecutive failures ({}), stopping auto-reconnect attempts", 
                   status.consecutive_failures);
            drop(status);
            return;
        }
        drop(status);

        // 尝试重连远程数据库
        if let Some(remote_result) = &result.remote_result {
            if remote_result.needs_reconnect {
                info!("Attempting to reconnect remote database");
                self.attempt_remote_reconnection().await;
            }
        }
    }

    /// 尝试远程重连
    async fn attempt_remote_reconnection(&self) {
        {
            let mut status = self.connection_status.write().await;
            status.reconnect_attempts += 1;
        }

        match self.try_reconnect_remote().await {
            Ok(_) => {
                info!("Remote database reconnection successful");
                self.notify_reconnect_callbacks("remote", true).await;
                
                // 重置失败计数
                let mut status = self.connection_status.write().await;
                status.consecutive_failures = 0;
                status.remote_status = DatabaseConnectionStatus::Healthy;
            }
            Err(e) => {
                error!("Remote database reconnection failed: {}", e);
                self.notify_reconnect_callbacks("remote", false).await;
            }
        }
    }

    /// 尝试重新连接远程数据库
    async fn try_reconnect_remote(&self) -> Result<()> {
        // 这里应该重新建立远程数据库连接
        // 由于我们没有保存原始的连接参数，这里只是一个占位符实现
        // 实际应用中需要从配置中重新获取连接参数
        
        let remote_db_guard = self.remote_db.read().await;
        if let Some(remote_db) = remote_db_guard.as_ref() {
            // 尝试同步以测试连接
            remote_db.sync().await
                .map_err(|e| anyhow!("Failed to sync remote database: {}", e))?;
            
            info!("Remote database connection verified");
            Ok(())
        } else {
            Err(anyhow!("No remote database configured"))
        }
    }

    /// 处理健康检查错误
    async fn handle_health_check_error(&self, error: anyhow::Error) {
        error!("Health check encountered an error: {}", error);
        
        // 更新连接状态
        {
            let mut status = self.connection_status.write().await;
            status.consecutive_failures += 1;
            status.total_checks += 1;
            status.local_status = DatabaseConnectionStatus::Error(error.to_string());
            status.remote_status = DatabaseConnectionStatus::Unknown;
        }

        // 记录错误恢复
        self.structured_logger.log_error_recovery(
            "health_check_error",
            "logged_and_counted",
            false,
            1,
            0,
        );
    }

    /// 记录健康检查结果
    async fn log_health_check_result(&self, result: &HealthCheckResult) {
        self.structured_logger.log_sync_operation(
            "health_check_result",
            1,
            if result.overall_healthy { 1 } else { 0 },
            if result.overall_healthy { 0 } else { 1 },
            (Utc::now().timestamp_millis() - result.check_time) as u64,
        );
    }

    /// 通知重连回调函数
    async fn notify_reconnect_callbacks(&self, database_type: &str, success: bool) {
        let callbacks = self.reconnect_callbacks.lock().await;
        for callback in callbacks.iter() {
            callback(database_type, success);
        }
    }

    /// 添加重连回调函数
    pub async fn add_reconnect_callback(&self, callback: ReconnectCallback) {
        let mut callbacks = self.reconnect_callbacks.lock().await;
        callbacks.push(callback);
    }

    /// 获取当前连接状态
    pub async fn get_connection_status(&self) -> ConnectionStatus {
        self.connection_status.read().await.clone()
    }

    /// 获取健康检查配置
    pub async fn get_config(&self) -> HealthCheckConfig {
        self.config.read().await.clone()
    }

    /// 更新健康检查配置
    pub async fn update_config(&self, new_config: HealthCheckConfig) -> Result<()> {
        let should_restart = {
            let current_config = self.config.read().await;
            current_config.enabled != new_config.enabled ||
            current_config.check_interval_seconds != new_config.check_interval_seconds
        };

        *self.config.write().await = new_config;

        if should_restart {
            self.start_health_check().await?;
        }

        info!("Health check configuration updated");
        Ok(())
    }

    /// 强制执行一次健康检查
    pub async fn force_health_check(&self) -> Result<HealthCheckResult> {
        info!("Forcing immediate health check");
        self.perform_health_check().await
    }
}

// 为 ConnectionHealthChecker 实现 Clone（需要用于异步任务）
impl Clone for ConnectionHealthChecker {
    fn clone(&self) -> Self {
        Self {
            local_db: self.local_db.clone(),
            remote_db: self.remote_db.clone(),
            config: self.config.clone(),
            connection_status: self.connection_status.clone(),
            health_check_handle: self.health_check_handle.clone(),
            performance_monitor: self.performance_monitor.clone(),
            structured_logger: self.structured_logger.clone(),
            reconnect_callbacks: self.reconnect_callbacks.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_config_default() {
        let config = HealthCheckConfig::default();
        assert!(config.enabled);
        assert_eq!(config.check_interval_seconds, 30);
        assert_eq!(config.connection_timeout_seconds, 10);
        assert_eq!(config.query_timeout_seconds, 5);
        assert!(config.auto_reconnect);
    }

    #[test]
    fn test_connection_status_default() {
        let status = ConnectionStatus::default();
        assert_eq!(status.local_status, DatabaseConnectionStatus::Unknown);
        assert_eq!(status.remote_status, DatabaseConnectionStatus::Unknown);
        assert_eq!(status.consecutive_failures, 0);
        assert_eq!(status.total_checks, 0);
        assert_eq!(status.successful_checks, 0);
    }

    #[test]
    fn test_database_connection_status_equality() {
        assert_eq!(DatabaseConnectionStatus::Healthy, DatabaseConnectionStatus::Healthy);
        assert_ne!(DatabaseConnectionStatus::Healthy, DatabaseConnectionStatus::Slow);
        assert_ne!(
            DatabaseConnectionStatus::Error("test".to_string()),
            DatabaseConnectionStatus::Error("other".to_string())
        );
    }
} 