use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use libsql::Database;
use tracing::{info, warn, error, debug};
use super::monitoring::{PerformanceMonitor, StructuredLogger};
use super::error_handling::{SmartRetryExecutor, ErrorAnalyzer, LibSqlErrorType};

/// 内置同步配置
#[derive(Debug, Clone)]
pub struct BuiltinSyncConfig {
    /// 同步间隔（秒）
    pub sync_interval: u64,
    /// 是否启用自动同步
    pub auto_sync_enabled: bool,
    /// 同步超时时间（秒）
    pub sync_timeout: u64,
    /// 最大重试次数
    pub max_retry_attempts: u32,
    /// 重试间隔（毫秒）
    pub retry_interval_ms: u64,
    /// 是否启用冲突检测
    pub conflict_detection_enabled: bool,
}

impl Default for BuiltinSyncConfig {
    fn default() -> Self {
        Self {
            sync_interval: 60, // 1分钟
            auto_sync_enabled: true,
            sync_timeout: 30, // 30秒
            max_retry_attempts: 3,
            retry_interval_ms: 1000, // 1秒
            conflict_detection_enabled: true,
        }
    }
}

/// 内置同步状态
#[derive(Debug, Clone)]
pub struct BuiltinSyncStatus {
    /// 是否连接到远程
    pub is_connected: bool,
    /// 最后同步时间
    pub last_sync_time: i64,
    /// 同步错误信息
    pub last_error: Option<String>,
    /// 本地 WAL 大小
    pub local_wal_size: u64,
    /// 远程落后的帧数
    pub frames_behind: u64,
    /// 同步统计
    pub sync_stats: BuiltinSyncStats,
}

/// 内置同步统计
#[derive(Debug, Clone)]
pub struct BuiltinSyncStats {
    /// 总同步次数
    pub total_syncs: u64,
    /// 成功同步次数
    pub successful_syncs: u64,
    /// 失败同步次数
    pub failed_syncs: u64,
    /// 冲突解决次数
    pub conflicts_resolved: u64,
    /// 平均同步耗时（毫秒）
    pub avg_sync_duration_ms: u64,
}

impl Default for BuiltinSyncStats {
    fn default() -> Self {
        Self {
            total_syncs: 0,
            successful_syncs: 0,
            failed_syncs: 0,
            conflicts_resolved: 0,
            avg_sync_duration_ms: 0,
        }
    }
}

/// 内置同步适配器
pub struct BuiltinSyncAdapter {
    /// 本地数据库（replica）
    local_db: Arc<Database>,
    /// 同步配置
    config: Arc<RwLock<BuiltinSyncConfig>>,
    /// 同步状态
    status: Arc<RwLock<BuiltinSyncStatus>>,
    /// 自动同步任务句柄
    auto_sync_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// 同步中标志
    is_syncing: Arc<AtomicBool>,
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    /// 结构化日志记录器
    structured_logger: Arc<StructuredLogger>,
}

impl BuiltinSyncAdapter {
    /// 创建新的内置同步适配器
    pub async fn new(
        local_db: Arc<Database>,
        performance_monitor: Arc<PerformanceMonitor>,
        structured_logger: Arc<StructuredLogger>,
    ) -> Result<Self> {
        let config = Arc::new(RwLock::new(BuiltinSyncConfig::default()));
        let status = Arc::new(RwLock::new(BuiltinSyncStatus {
            is_connected: false,
            last_sync_time: 0,
            last_error: None,
            local_wal_size: 0,
            frames_behind: 0,
            sync_stats: BuiltinSyncStats::default(),
        }));

        Ok(Self {
            local_db,
            config,
            status,
            auto_sync_handle: Arc::new(Mutex::new(None)),
            is_syncing: Arc::new(AtomicBool::new(false)),
            performance_monitor,
            structured_logger,
        })
    }

    /// 获取同步配置
    pub async fn get_config(&self) -> BuiltinSyncConfig {
        self.config.read().await.clone()
    }

    /// 更新同步配置
    pub async fn update_config(&self, new_config: BuiltinSyncConfig) -> Result<()> {
        let should_restart_auto_sync = {
            let current_config = self.config.read().await;
            current_config.auto_sync_enabled != new_config.auto_sync_enabled ||
            current_config.sync_interval != new_config.sync_interval
        };

        *self.config.write().await = new_config.clone();

        if should_restart_auto_sync && new_config.auto_sync_enabled {
            self.start_auto_sync().await?;
        }

        self.structured_logger.log_sync_operation(
            "config_update",
            0, 0, 0,
            0,
        );

        Ok(())
    }

    /// 获取同步状态
    pub async fn get_status(&self) -> BuiltinSyncStatus {
        // 更新 WAL 信息
        if let Ok(wal_info) = self.get_wal_info().await {
            let mut status = self.status.write().await;
            status.local_wal_size = wal_info.size;
            status.frames_behind = wal_info.frames_behind;
        }

        self.status.read().await.clone()
    }

    /// 执行一次同步
    pub async fn sync_once(&self) -> Result<BuiltinSyncStats> {
        // 检查是否已在同步中
        if self.is_syncing.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(anyhow!("Sync is already in progress"));
        }

        let _sync_guard = SyncGuard::new(&self.is_syncing);
        let start_time = std::time::Instant::now();

        let sync_result = self.performance_monitor.monitor_operation("builtin_sync", async {
            // 检查数据库连接
            let conn = self.local_db.connect()
                .map_err(|e| anyhow!("Failed to connect to local database: {}", e))?;

            // 执行同步操作
            let db = self.local_db.clone();
            let sync_result = tokio::time::timeout(
                Duration::from_secs(self.config.read().await.sync_timeout),
                async {
                    SmartRetryExecutor::execute_with_retry(
                        move || {
                            let db = db.clone();
                            Box::pin(async move {
                                // 这里应该调用实际的LibSQL同步API
                                // 由于我们使用的是replica模式，需要调用database.sync()
                                db.sync().await
                                    .map_err(|e| anyhow!("LibSQL sync failed: {}", e))
                            })
                        },
                        "libsql_builtin_sync",
                    ).await
                }
            ).await;

            match sync_result {
                Ok(Ok(_)) => {
                    // 同步成功，更新状态
                    let mut status = self.status.write().await;
                    status.is_connected = true;
                    status.last_sync_time = Utc::now().timestamp_millis();
                    status.last_error = None;
                    status.sync_stats.total_syncs += 1;
                    status.sync_stats.successful_syncs += 1;
                    
                    let duration_ms = start_time.elapsed().as_millis() as u64;
                    status.sync_stats.avg_sync_duration_ms = (
                        (status.sync_stats.avg_sync_duration_ms * (status.sync_stats.total_syncs - 1)) + 
                        duration_ms
                    ) / status.sync_stats.total_syncs;

                    self.structured_logger.log_sync_operation(
                        "builtin_sync_success",
                        1, 1, 0,
                        duration_ms,
                    );

                    Ok(status.sync_stats.clone())
                }
                Ok(Err(e)) => {
                    // 同步失败，更新状态
                    let mut status = self.status.write().await;
                    status.last_error = Some(e.to_string());
                    status.sync_stats.total_syncs += 1;
                    status.sync_stats.failed_syncs += 1;

                    // 分析错误类型
                    let error_type = ErrorAnalyzer::classify_error(&e);
                    let is_connection_error = matches!(error_type, 
                        LibSqlErrorType::NetworkError | 
                        LibSqlErrorType::TimeoutError
                    );
                    
                    if is_connection_error {
                        status.is_connected = false;
                    }

                    self.structured_logger.log_sync_operation(
                        "builtin_sync_error",
                        1, 0, 1,
                        start_time.elapsed().as_millis() as u64,
                    );

                    Err(e)
                }
                Err(_) => {
                    // 超时
                    let mut status = self.status.write().await;
                    status.last_error = Some("Sync timeout".to_string());
                    status.sync_stats.total_syncs += 1;
                    status.sync_stats.failed_syncs += 1;
                    status.is_connected = false;

                    self.structured_logger.log_sync_operation(
                        "builtin_sync_timeout",
                        1, 0, 1,
                        start_time.elapsed().as_millis() as u64,
                    );

                    Err(anyhow!("Sync operation timed out"))
                }
            }
        }).await;

        sync_result
    }

    /// 带重试的同步
    pub async fn sync_with_retry(&self) -> Result<BuiltinSyncStats> {
        let config = self.config.read().await;
        let max_attempts = config.max_retry_attempts;
        let retry_interval = config.retry_interval_ms;
        drop(config);

        for attempt in 1..=max_attempts {
            match self.sync_once().await {
                Ok(stats) => return Ok(stats),
                Err(e) if attempt < max_attempts => {
                    self.structured_logger.log_error_recovery(
                        "builtin_sync_retry",
                        &format!("Attempt {}/{}", attempt, max_attempts),
                        false,
                        attempt,
                        0,
                    );

                    tokio::time::sleep(Duration::from_millis(retry_interval * attempt as u64)).await;
                }
                Err(e) => return Err(e),
            }
        }

        Err(anyhow!("Sync failed after {} attempts", max_attempts))
    }

    /// 启动自动同步
    pub async fn start_auto_sync(&self) -> Result<()> {
        let config = self.config.read().await;
        if !config.auto_sync_enabled {
            return Ok(());
        }

        let sync_interval = config.sync_interval;
        drop(config);

        // 停止现有的自动同步任务
        self.stop_auto_sync().await?;

        // 创建新的自动同步任务
        let adapter = self.clone();
        let handle = tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(sync_interval));
            
            loop {
                interval.tick().await;
                
                // 检查是否仍然启用自动同步
                let config = adapter.config.read().await;
                if !config.auto_sync_enabled {
                    break;
                }
                drop(config);

                // 执行同步
                if let Err(e) = adapter.sync_with_retry().await {
                    adapter.structured_logger.log_sync_operation(
                        "auto_sync_error",
                        1, 0, 1,
                        0,
                    );
                    eprintln!("Auto sync failed: {}", e);
                }
            }
        });

        *self.auto_sync_handle.lock().await = Some(handle);

        self.structured_logger.log_sync_operation(
            "auto_sync_started",
            0, 0, 0,
            0,
        );

        Ok(())
    }

    /// 停止自动同步
    pub async fn stop_auto_sync(&self) -> Result<()> {
        let mut handle_guard = self.auto_sync_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
            
            self.structured_logger.log_sync_operation(
                "auto_sync_stopped",
                0, 0, 0,
                0,
            );
        }
        Ok(())
    }

    /// 检查连接状态
    pub async fn check_connection(&self) -> bool {
        match self.sync_once().await {
            Ok(_) => true,
            Err(_) => {
                let mut status = self.status.write().await;
                status.is_connected = false;
                false
            }
        }
    }

    /// 获取WAL信息
    async fn get_wal_info(&self) -> Result<WalInfo> {
        let conn = self.local_db.connect()?;
        
        // 查询WAL文件大小
        let mut rows = conn.query("PRAGMA wal_checkpoint(PASSIVE)", ()).await?;
        let wal_size = if let Some(row) = rows.next().await? {
            row.get::<i64>(1).unwrap_or(0) as u64
        } else {
            0
        };

        // 获取frames信息
        let mut frames_rows = conn.query("PRAGMA wal_autocheckpoint", ()).await?;
        let frames_behind = if let Some(row) = frames_rows.next().await? {
            row.get::<i64>(0).unwrap_or(0) as u64
        } else {
            0
        };

        Ok(WalInfo {
            size: wal_size,
            frames_behind,
        })
    }
}

/// WAL信息
struct WalInfo {
    /// WAL 文件大小
    size: u64,
    /// 落后的帧数
    frames_behind: u64,
}

impl Clone for BuiltinSyncAdapter {
    fn clone(&self) -> Self {
        Self {
            local_db: self.local_db.clone(),
            config: self.config.clone(),
            status: self.status.clone(),
            auto_sync_handle: self.auto_sync_handle.clone(),
            is_syncing: self.is_syncing.clone(),
            performance_monitor: self.performance_monitor.clone(),
            structured_logger: self.structured_logger.clone(),
        }
    }
}

/// 同步守护程序
struct SyncGuard {
    is_syncing: Arc<AtomicBool>,
}

impl SyncGuard {
    fn new(is_syncing: &Arc<AtomicBool>) -> Self {
        Self {
            is_syncing: is_syncing.clone(),
        }
    }
}

impl Drop for SyncGuard {
    fn drop(&mut self) {
        self.is_syncing.store(false, Ordering::SeqCst);
    }
} 