use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::future::Future;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow::{Result, anyhow};
use tracing::{info, warn, error};
use super::{SyncStats};

/// 性能监控器
pub struct PerformanceMonitor {
    operation_metrics: Arc<Mutex<HashMap<String, OperationMetrics>>>,
    start_time: Instant,
}

/// 操作指标
#[derive(Debug, Clone)]
pub struct OperationMetrics {
    /// 操作名称
    pub operation_name: String,
    /// 总执行次数
    pub total_executions: u64,
    /// 成功次数
    pub successful_executions: u64,
    /// 失败次数
    pub failed_executions: u64,
    /// 总执行时间（毫秒）
    pub total_duration_ms: u64,
    /// 平均执行时间（毫秒）
    pub average_duration_ms: f64,
    /// 最小执行时间（毫秒）
    pub min_duration_ms: u64,
    /// 最大执行时间（毫秒）
    pub max_duration_ms: u64,
    /// 最后执行时间
    pub last_execution_time: i64,
    /// 错误信息统计
    pub error_counts: HashMap<String, u64>,
}

impl OperationMetrics {
    pub fn new(operation_name: String) -> Self {
        Self {
            operation_name,
            total_executions: 0,
            successful_executions: 0,
            failed_executions: 0,
            total_duration_ms: 0,
            average_duration_ms: 0.0,
            min_duration_ms: u64::MAX,
            max_duration_ms: 0,
            last_execution_time: Utc::now().timestamp_millis(),
            error_counts: HashMap::new(),
        }
    }
    
    pub fn record_success(&mut self, duration_ms: u64) {
        self.total_executions += 1;
        self.successful_executions += 1;
        self.total_duration_ms += duration_ms;
        self.average_duration_ms = self.total_duration_ms as f64 / self.total_executions as f64;
        self.min_duration_ms = self.min_duration_ms.min(duration_ms);
        self.max_duration_ms = self.max_duration_ms.max(duration_ms);
        self.last_execution_time = Utc::now().timestamp_millis();
    }
    
    pub fn record_failure(&mut self, duration_ms: u64, error: &str) {
        self.total_executions += 1;
        self.failed_executions += 1;
        self.total_duration_ms += duration_ms;
        self.average_duration_ms = self.total_duration_ms as f64 / self.total_executions as f64;
        self.min_duration_ms = self.min_duration_ms.min(duration_ms);
        self.max_duration_ms = self.max_duration_ms.max(duration_ms);
        self.last_execution_time = Utc::now().timestamp_millis();
        
        let error_category = Self::categorize_error(error);
        *self.error_counts.entry(error_category).or_insert(0) += 1;
    }
    
    fn categorize_error(error: &str) -> String {
        let error_lower = error.to_lowercase();
        
        if error_lower.contains("network") || error_lower.contains("connection") {
            "network_error".to_string()
        } else if error_lower.contains("timeout") {
            "timeout_error".to_string()
        } else if error_lower.contains("lock") || error_lower.contains("busy") {
            "locking_error".to_string()
        } else if error_lower.contains("constraint") || error_lower.contains("integrity") {
            "constraint_error".to_string()
        } else {
            "other_error".to_string()
        }
    }
    
    pub fn get_success_rate(&self) -> f64 {
        if self.total_executions == 0 {
            0.0
        } else {
            self.successful_executions as f64 / self.total_executions as f64 * 100.0
        }
    }
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            operation_metrics: Arc::new(Mutex::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }
    
    /// 监控操作执行
    pub async fn monitor_operation<F, T>(&self, operation_name: &str, operation: F) -> Result<T>
    where
        F: Future<Output = Result<T>>,
    {
        let start_time = Instant::now();
        let result = operation.await;
        let duration = start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;
        
        let mut metrics = self.operation_metrics.lock().unwrap();
        let operation_metrics = metrics.entry(operation_name.to_string())
            .or_insert_with(|| OperationMetrics::new(operation_name.to_string()));
        
        match &result {
            Ok(_) => operation_metrics.record_success(duration_ms),
            Err(e) => operation_metrics.record_failure(duration_ms, &e.to_string()),
        }
        
        result
    }
    
    /// 记录数据库操作指标
    pub async fn record_db_operation(&self, operation_type: &str, table_name: &str, record_count: u64, duration: Duration) {
        let operation_name = format!("db_{}_{}", operation_type, table_name);
        let duration_ms = duration.as_millis() as u64;
        
        let mut metrics = self.operation_metrics.lock().unwrap();
        let operation_metrics = metrics.entry(operation_name.clone())
            .or_insert_with(|| OperationMetrics::new(operation_name));
        
        operation_metrics.record_success(duration_ms);
        
        info!("DB operation recorded: {} on {} ({} records) completed in {}ms", 
              operation_type, table_name, record_count, duration_ms);
    }
    
    /// 获取性能报告
    pub async fn get_performance_report(&self) -> PerformanceReport {
        let metrics = self.operation_metrics.lock().unwrap();
        let operations = metrics.values().cloned().collect();
        let total_uptime_ms = self.start_time.elapsed().as_millis() as u64;
        
        PerformanceReport {
            total_uptime_ms,
            operations,
            generated_at: Utc::now().timestamp_millis(),
        }
    }
    
    /// 清理旧指标
    pub async fn cleanup_old_metrics(&self, max_age_hours: u64) {
        let cutoff_time = Utc::now().timestamp_millis() - (max_age_hours * 3600 * 1000) as i64;
        let mut metrics = self.operation_metrics.lock().unwrap();
        
        metrics.retain(|_, metric| metric.last_execution_time > cutoff_time);
        
        info!("Cleaned up old performance metrics older than {} hours", max_age_hours);
    }
    
    /// 记录同步指标
    pub async fn record_sync_metrics(&self, stats: &SyncStats, sync_duration: Duration) {
        let duration_ms = sync_duration.as_millis() as u64;
        
        let mut metrics = self.operation_metrics.lock().unwrap();
        let sync_metrics = metrics.entry("sync_operation".to_string())
            .or_insert_with(|| OperationMetrics::new("sync_operation".to_string()));
        
        if stats.failed_records == 0 {
            sync_metrics.record_success(duration_ms);
        } else {
            sync_metrics.record_failure(duration_ms, &format!("{} records failed", stats.failed_records));
        }
        
        info!("Sync metrics recorded: {} total, {} synced, {} failed in {}ms",
              stats.total_records, stats.synced_records, stats.failed_records, duration_ms);
    }
}

/// 性能报告
#[derive(Debug, Clone)]
pub struct PerformanceReport {
    pub total_uptime_ms: u64,
    pub operations: Vec<OperationMetrics>,
    pub generated_at: i64,
}

/// 结构化日志记录器
pub struct StructuredLogger {
    log_level: String,
    include_performance: bool,
}

impl StructuredLogger {
    pub fn new(log_level: String, include_performance: bool) -> Self {
        Self {
            log_level,
            include_performance,
        }
    }
    
    /// 记录数据库操作
    pub fn log_db_operation(
        &self,
        operation: &str,
        table: &str,
        record_id: &str,
        duration_ms: u64,
        success: bool,
        error: Option<&str>,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "type": "db_operation",
            "operation": operation,
            "table": table,
            "record_id": record_id,
            "duration_ms": duration_ms,
            "success": success,
            "error": error,
            "performance_included": self.include_performance
        });
        
        if success {
            info!("DB Operation: {}", log_entry);
        } else {
            error!("DB Operation Failed: {}", log_entry);
        }
    }
    
    /// 记录同步操作
    pub fn log_sync_operation(
        &self,
        sync_type: &str,
        total_records: u64,
        synced_records: u64,
        failed_records: u64,
        duration_ms: u64,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "type": "sync_operation",
            "sync_type": sync_type,
            "total_records": total_records,
            "synced_records": synced_records,
            "failed_records": failed_records,
            "duration_ms": duration_ms,
            "success_rate": if total_records > 0 { 
                (synced_records as f64 / total_records as f64) * 100.0 
            } else { 
                100.0 
            }
        });
        
        if failed_records == 0 {
            info!("Sync Operation: {}", log_entry);
        } else {
            warn!("Sync Operation with Failures: {}", log_entry);
        }
    }
    
    /// 记录连接池操作
    pub fn log_connection_pool_operation(
        &self,
        pool_type: &str,
        operation: &str,
        active_connections: usize,
        idle_connections: usize,
        duration_ms: u64,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "type": "connection_pool_operation",
            "pool_type": pool_type,
            "operation": operation,
            "active_connections": active_connections,
            "idle_connections": idle_connections,
            "total_connections": active_connections + idle_connections,
            "duration_ms": duration_ms
        });
        
        info!("Connection Pool: {}", log_entry);
    }
    
    /// 记录错误恢复操作
    pub fn log_error_recovery(
        &self,
        error_type: &str,
        recovery_action: &str,
        success: bool,
        attempts: u32,
        duration_ms: u64,
    ) {
        let log_entry = serde_json::json!({
            "timestamp": Utc::now().to_rfc3339(),
            "type": "error_recovery",
            "error_type": error_type,
            "recovery_action": recovery_action,
            "success": success,
            "attempts": attempts,
            "duration_ms": duration_ms
        });
        
        if success {
            info!("Error Recovery Success: {}", log_entry);
        } else {
            error!("Error Recovery Failed: {}", log_entry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::{sleep, Duration};
    
    #[tokio::test]
    async fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new();
        
        // 测试成功的操作
        let result = monitor.monitor_operation("test_operation", async {
            sleep(Duration::from_millis(10)).await;
            Ok::<String, anyhow::Error>("success".to_string())
        }).await;
        
        assert!(result.is_ok());
        
        // 测试失败的操作
        let result = monitor.monitor_operation("test_operation", async {
            sleep(Duration::from_millis(5)).await;
            Err::<String, anyhow::Error>(anyhow::anyhow!("test error"))
        }).await;
        
        assert!(result.is_err());
        
        // 验证指标
        let report = monitor.get_performance_report().await;
        assert!(!report.operations.is_empty());
        
        let test_metrics = report.operations.iter()
            .find(|m| m.operation_name == "test_operation")
            .unwrap();
        
        assert_eq!(test_metrics.total_executions, 2);
        assert_eq!(test_metrics.successful_executions, 1);
        assert_eq!(test_metrics.failed_executions, 1);
    }
    
    #[test]
    fn test_operation_metrics() {
        let mut metrics = OperationMetrics::new("test".to_string());
        
        metrics.record_success(100);
        metrics.record_success(200);
        metrics.record_failure(150, "test error");
        
        assert_eq!(metrics.total_executions, 3);
        assert_eq!(metrics.successful_executions, 2);
        assert_eq!(metrics.failed_executions, 1);
        assert_eq!(metrics.average_duration_ms, 150.0);
        assert_eq!(metrics.min_duration_ms, 100);
        assert_eq!(metrics.max_duration_ms, 200);
        assert_eq!(metrics.get_success_rate(), 66.66666666666667);
    }
    
    #[test]
    fn test_structured_logger() {
        let logger = StructuredLogger::new("INFO".to_string(), true);
        
        // 测试各种日志记录方法
        logger.log_db_operation("INSERT", "tips", "test_id", 100, true, None);
        logger.log_db_operation("UPDATE", "tips", "test_id", 150, false, Some("constraint violation"));
        
        logger.log_sync_operation("manual_sync", 100, 95, 5, 5000);
        
        logger.log_connection_pool_operation("local", "acquire", 5, 3, 10);
        
        logger.log_error_recovery("network_error", "reconnect", true, 3, 2000);
    }
} 