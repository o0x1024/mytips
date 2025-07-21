use anyhow::{anyhow, Result};
use chrono::Utc;
use libsql::{Connection, Database, Builder};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use tracing::{info, warn};

// 模块导入
mod connection_pool;
pub mod builtin_sync;
pub mod error_handling;
pub mod monitoring;
pub mod transaction_manager;
pub mod conflict_resolver;
pub mod incremental_sync;
pub mod health_checker;
pub mod libsql_sync_manager;
pub mod libsql_adapter;

// 重新导出公共API
pub use builtin_sync::{BuiltinSyncAdapter, BuiltinSyncConfig, BuiltinSyncStatus, BuiltinSyncStats};
pub use error_handling::{ErrorAnalyzer, LibSqlErrorType, SmartRetryExecutor, RetryStrategy};
pub use monitoring::{PerformanceMonitor, StructuredLogger, OperationMetrics, PerformanceReport};
pub use transaction_manager::{AtomicTransactionManager, TransactionConsistencyValidator, ConsistencyReport, TransactionType, TransactionStatus};
pub use conflict_resolver::{
    EnhancedConflictResolver, EnhancedConflictData, EnhancedConflictResolutionResult, 
    BatchConflictResolutionResult, FieldConflict, FieldMergeStrategy, ConflictSeverity, FieldConflictType
};
pub use incremental_sync::{IncrementalSyncManager, IncrementalSyncConfig, IncrementalSyncStats, ChangeDetector};
pub use health_checker::{ConnectionHealthChecker, HealthCheckConfig, ConnectionStatus, DatabaseConnectionStatus, HealthCheckResult};
pub use libsql_sync_manager::{LibSqlSyncManager, LibSqlSyncConfig, SyncResult as LibSqlSyncResult};
pub use libsql_adapter::{LibSqlAdapter, test_libsql_connection};

use connection_pool::{ConnectionPoolManager, OptimizedConnectionPoolConfig};

use crate::db::{
    self, SyncConfig, SyncMode, SyncOperation, SyncStatusRecord,
    DataVersion, ConflictResolutionStrategy
};

/// 冲突数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictData {
    pub local_version: DataVersion,
    pub remote_version: DataVersion,
    pub local_content: String,
    pub remote_content: String,
}

/// 冲突解决结果
#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictResolutionResult {
    pub strategy: ConflictResolutionStrategy,
    pub resolved_by: String,
}

/// 同步统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub total_records: u64,
    pub synced_records: u64,
    pub pending_records: u64,
    pub failed_records: u64,
    pub last_sync_time: i64,
    pub is_online: bool,  // 是否在线
}

/// 同步事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    SyncStarted,
    SyncProgress { current: u64, total: u64 },
    SyncCompleted { stats: SyncStats },
    SyncFailed { error: String },
    ConflictDetected { record_id: String, table_name: String },
    ConnectionStatusChanged { is_online: bool },
}

/// 同步事件监听器
pub type SyncEventListener = Box<dyn Fn(SyncEvent) + Send + Sync>;

/// 基础冲突解决器
pub struct ConflictResolver {
    default_strategy: ConflictResolutionStrategy,
}

impl ConflictResolver {
    /// 解决冲突
    pub async fn resolve_conflict(
        &self,
        _record: &SyncStatusRecord,
        _conflict: &ConflictData,
    ) -> Result<ConflictResolutionResult> {
        Ok(ConflictResolutionResult {
            strategy: self.default_strategy.clone(),
            resolved_by: "BASIC_RESOLVER".to_string(),
        })
    }

    /// 设置默认策略
    pub fn set_default_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.default_strategy = strategy;
    }
}

/// 同步管理器 - 核心同步逻辑
pub struct SyncManager {
    local_db: Arc<Database>,
    remote_db: Arc<RwLock<Option<Database>>>,
    sync_config: Arc<RwLock<SyncConfig>>,
    sync_queue: Arc<Mutex<VecDeque<SyncOperation>>>,
    is_syncing: Arc<AtomicBool>,
    conflict_resolver: Arc<ConflictResolver>,
    sync_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    connection_pool_manager: Arc<Mutex<ConnectionPoolManager>>,
    cleanup_task_handle: Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>,
    /// 原子事务管理器
    transaction_manager: Arc<AtomicTransactionManager>,
    /// 一致性验证器
    consistency_validator: Arc<TransactionConsistencyValidator>,
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    /// 结构化日志记录器
    structured_logger: Arc<StructuredLogger>,
    /// LibSQL 内置同步适配器
    builtin_sync_adapter: Arc<RwLock<Option<BuiltinSyncAdapter>>>,
    /// 增强的智能冲突解决器
    enhanced_conflict_resolver: Arc<EnhancedConflictResolver>,
    /// 增量同步管理器
    incremental_sync_manager: Arc<RwLock<Option<IncrementalSyncManager>>>,
    /// 连接健康检查器
    connection_health_checker: Arc<RwLock<Option<ConnectionHealthChecker>>>,
    /// LibSQL适配器（用于WAL安全的同步）
    libsql_adapter: Arc<LibSqlAdapter>,
}

impl SyncManager {
    /// 创建新的同步管理器
    pub async fn new(local_db: Arc<Database>) -> Result<Self> {
        Self::new_with_db_path(local_db, None).await
    }

    /// 使用指定数据库路径创建同步管理器
    pub async fn new_with_db_path(local_db: Arc<Database>, db_path: Option<String>) -> Result<Self> {
        // 初始化性能监控器和日志记录器
        let performance_monitor = Arc::new(PerformanceMonitor::new());
        let structured_logger = Arc::new(StructuredLogger::new("INFO".to_string(), true));

        // 初始化连接池管理器
        let pool_config = OptimizedConnectionPoolConfig::default();
        let connection_pool_manager = Arc::new(Mutex::new(
            ConnectionPoolManager::new(pool_config)
        ));

        // 初始化事务管理器和一致性验证器
        let remote_db = Arc::new(RwLock::new(None));
        let transaction_manager = Arc::new(AtomicTransactionManager::new(
            local_db.clone(),
            remote_db.clone(),
        ));
        let consistency_validator = Arc::new(TransactionConsistencyValidator::new(
            local_db.clone(),
            remote_db.clone(),
        ));

        // 初始化增强冲突解决器
        let enhanced_conflict_resolver = Arc::new(EnhancedConflictResolver::new(
            performance_monitor.clone(),
            structured_logger.clone(),
        ));

        // 加载或创建默认同步配置
        let sync_config = {
            let conn = local_db.connect()?;
            let config = db::get_sync_config(&conn).await.unwrap_or(None).unwrap_or_else(|| {
                SyncConfig {
                    id: "1".to_string(),
                    remote_url: None,
                    auth_token: None,
                    sync_mode: SyncMode::Manual,
                    sync_interval: 300,
                    auto_sync_enabled: false,
                    is_online: false,
                    last_sync_at: 0,
                    created_at: Utc::now().timestamp_millis(),
                    updated_at: Utc::now().timestamp_millis(),
                }
            });
            Arc::new(RwLock::new(config))
        };

        let sync_manager = Self {
            local_db,
            remote_db,
            sync_config,
            sync_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_syncing: Arc::new(AtomicBool::new(false)),
            conflict_resolver: Arc::new(ConflictResolver {
                default_strategy: ConflictResolutionStrategy::UserChoice,
            }),
            sync_handle: Arc::new(Mutex::new(None)),
            connection_pool_manager,
            cleanup_task_handle: Arc::new(Mutex::new(None)),
            transaction_manager,
            consistency_validator,
            performance_monitor,
            structured_logger,
            builtin_sync_adapter: Arc::new(RwLock::new(None)),
            enhanced_conflict_resolver,
            incremental_sync_manager: Arc::new(RwLock::new(None)),
            connection_health_checker: Arc::new(RwLock::new(None)),
            libsql_adapter: Arc::new(LibSqlAdapter::new()),
        };

        // 如果已有同步配置且不是离线模式，尝试初始化同步组件
        {
            let config = sync_manager.sync_config.read().await;
            if config.sync_mode != SyncMode::Offline && config.remote_url.is_some() {
                info!("Existing sync configuration detected, will initialize sync components when needed");
                // 不在这里直接初始化，而是在首次使用时进行初始化
            }
        }

        Ok(sync_manager)
    }

    /// 启用内置同步模式
    pub async fn enable_builtin_sync(&self) -> Result<()> {
        info!("Enabling LibSQL builtin sync mode");

        // 检查是否已有远程数据库连接
        let remote_db = self.remote_db.read().await;
        if remote_db.is_none() {
            return Err(anyhow!("Remote database must be configured before enabling builtin sync"));
        }
        drop(remote_db);

        // 创建内置同步适配器
        let adapter = BuiltinSyncAdapter::new(
            self.local_db.clone(),
            self.performance_monitor.clone(),
            self.structured_logger.clone(),
        ).await?;

        // 启动自动同步
        adapter.start_auto_sync().await?;

        *self.builtin_sync_adapter.write().await = Some(adapter);

        // 初始化增量同步管理器
        let incremental_manager = IncrementalSyncManager::new(
            self.local_db.clone(),
            self.remote_db.clone(),
            self.performance_monitor.clone(),
            self.structured_logger.clone(),
        ).await?;

        *self.incremental_sync_manager.write().await = Some(incremental_manager);

        // 初始化连接健康检查器
        let health_checker = ConnectionHealthChecker::new(
            self.local_db.clone(),
            self.remote_db.clone(),
            self.performance_monitor.clone(),
            self.structured_logger.clone(),
        ).await;

        health_checker.start_health_check().await?;
        *self.connection_health_checker.write().await = Some(health_checker);

        info!("LibSQL builtin sync mode enabled successfully");
        Ok(())
    }

    /// 禁用内置同步模式
    pub async fn disable_builtin_sync(&self) -> Result<()> {
        info!("Disabling LibSQL builtin sync mode");

        // 停止内置同步适配器
        if let Some(adapter) = self.builtin_sync_adapter.write().await.take() {
            adapter.stop_auto_sync().await?;
        }

        // 停止增量同步管理器
        *self.incremental_sync_manager.write().await = None;

        // 停止健康检查器
        if let Some(checker) = self.connection_health_checker.write().await.take() {
            checker.stop_health_check().await;
        }

        info!("LibSQL builtin sync mode disabled");
        Ok(())
    }

    /// 检查是否启用了内置同步
    pub async fn is_builtin_sync_enabled(&self) -> bool {
        let adapter = self.builtin_sync_adapter.read().await;
        adapter.is_some()
    }

    /// 使用内置同步执行同步
    pub async fn sync_with_builtin(&self) -> Result<BuiltinSyncStats> {
        let adapter = self.builtin_sync_adapter.read().await;
        let adapter = adapter.as_ref()
            .ok_or_else(|| anyhow!("Builtin sync is not enabled"))?;
        
        adapter.sync_with_retry().await
    }

    /// 获取内置同步状态
    pub async fn get_builtin_sync_status(&self) -> Result<BuiltinSyncStatus> {
        let adapter = self.builtin_sync_adapter.read().await;
        let adapter = adapter.as_ref()
            .ok_or_else(|| anyhow!("Builtin sync is not enabled"))?;
        
        Ok(adapter.get_status().await)
    }

    /// 更新内置同步配置
    pub async fn update_builtin_sync_config(&self, config: BuiltinSyncConfig) -> Result<()> {
        let adapter = self.builtin_sync_adapter.read().await;
        let adapter = adapter.as_ref()
            .ok_or_else(|| anyhow!("Builtin sync is not enabled"))?;
        
        adapter.update_config(config).await
    }

    /// 执行增量同步
    pub async fn sync_incremental(&self) -> Result<SyncStats> {
        // 检查增量同步管理器是否已初始化，如果没有则尝试初始化
        {
            let manager = self.incremental_sync_manager.read().await;
            if manager.is_none() {
                drop(manager);
                
                // 检查是否有远程数据库配置
                let remote_db = self.remote_db.read().await;
                if remote_db.is_none() {
                    return Err(anyhow!("Remote database is not configured. Please configure remote database first."));
                }
                drop(remote_db);
                
                // 初始化增量同步管理器
                let incremental_manager = IncrementalSyncManager::new(
                    self.local_db.clone(),
                    self.remote_db.clone(),
                    self.performance_monitor.clone(),
                    self.structured_logger.clone(),
                ).await?;
                
                *self.incremental_sync_manager.write().await = Some(incremental_manager);
                info!("Incremental sync manager initialized on-demand");
            }
        }

        let manager = self.incremental_sync_manager.read().await;
        let manager = manager.as_ref()
            .ok_or_else(|| anyhow!("Failed to initialize incremental sync manager"))?;

        let incremental_stats = manager.sync_incremental().await?;

        // 转换为标准SyncStats
        Ok(SyncStats {
            total_records: incremental_stats.checked_records,
            synced_records: incremental_stats.synced_records,
            pending_records: incremental_stats.changed_records - incremental_stats.synced_records,
            failed_records: if incremental_stats.changed_records > incremental_stats.synced_records {
                incremental_stats.changed_records - incremental_stats.synced_records
            } else {
                0
            },
            last_sync_time: Utc::now().timestamp_millis(),
            is_online: self.is_online().await,
        })
    }

    /// 获取增量同步配置
    pub async fn get_incremental_sync_config(&self) -> Result<IncrementalSyncConfig> {
        let manager = self.incremental_sync_manager.read().await;
        let manager = manager.as_ref()
            .ok_or_else(|| anyhow!("Incremental sync is not enabled"))?;

        Ok(manager.get_config().await)
    }

    /// 更新增量同步配置
    pub async fn update_incremental_sync_config(&self, config: IncrementalSyncConfig) -> Result<()> {
        let manager = self.incremental_sync_manager.read().await;
        let manager = manager.as_ref()
            .ok_or_else(|| anyhow!("Incremental sync is not enabled"))?;

        manager.update_config(config).await
    }

    /// 启动健康检查
    pub async fn start_health_check(&self) -> Result<()> {
        let checker = self.connection_health_checker.read().await;
        let checker = checker.as_ref()
            .ok_or_else(|| anyhow!("Health checker is not initialized"))?;

        checker.start_health_check().await
    }

    /// 停止健康检查
    pub async fn stop_health_check(&self) -> Result<()> {
        let checker = self.connection_health_checker.read().await;
        let checker = checker.as_ref()
            .ok_or_else(|| anyhow!("Health checker is not initialized"))?;

        checker.stop_health_check().await;
        Ok(())
    }

    /// 获取连接状态
    pub async fn get_connection_status(&self) -> Result<ConnectionStatus> {
        let checker = self.connection_health_checker.read().await;
        let checker = checker.as_ref()
            .ok_or_else(|| anyhow!("Health checker is not initialized"))?;

        Ok(checker.get_connection_status().await)
    }

    /// 强制执行健康检查
    pub async fn force_health_check(&self) -> Result<HealthCheckResult> {
        let checker = self.connection_health_checker.read().await;
        let checker = checker.as_ref()
            .ok_or_else(|| anyhow!("Health checker is not initialized"))?;

        checker.force_health_check().await
    }

    /// 混合同步策略
    pub async fn sync_hybrid(&self) -> Result<SyncStats> {
        info!("Starting hybrid sync operation");

        // 优先使用LibSQL同步（WAL安全）
        match self.sync_with_libsql().await {
            Ok(libsql_stats) => {
                info!("Hybrid sync completed using LibSQL sync");
                return Ok(libsql_stats);
            }
            Err(e) => {
                warn!("LibSQL sync failed, falling back to other methods: {}", e);
            }
        }

        // 检查是否启用了内置同步
        if self.is_builtin_sync_enabled().await {
            // 使用内置同步
            match self.sync_with_builtin().await {
                Ok(builtin_stats) => {
                    info!("Hybrid sync completed using builtin sync");
                    return Ok(SyncStats {
                        total_records: builtin_stats.total_syncs,
                        synced_records: builtin_stats.successful_syncs,
                        pending_records: 0,
                        failed_records: builtin_stats.failed_syncs,
                        last_sync_time: Utc::now().timestamp_millis(),
                        is_online: true,
                    });
                }
                Err(_e) => {
                    // 降级到增量同步
                    return self.sync_incremental().await;
                }
            }
        }

        // 如果没有启用内置同步，使用增量同步
        self.sync_incremental().await
    }

    /// 获取增强冲突数据
    pub async fn get_enhanced_conflicts(&self) -> Result<Vec<EnhancedConflictData>> {
        // 暂时返回空的冲突列表，因为 get_conflicts 函数不存在
        // TODO: 实现实际的冲突检测逻辑
        let enhanced_conflicts = Vec::new();
        Ok(enhanced_conflicts)
    }

    /// 批量解决冲突
    pub async fn resolve_conflicts_batch(
        &self,
        strategy: ConflictResolutionStrategy,
    ) -> Result<BatchConflictResolutionResult> {
        // 暂时返回空的批量解决结果
        // TODO: 实现实际的批量冲突解决逻辑
        let resolution_results = Vec::new();
        let resolved_count = 0;
        let failed_count = 0;

        info!("Starting batch conflict resolution (placeholder)");

        Ok(BatchConflictResolutionResult {
            total_conflicts: resolved_count + failed_count,
            resolved_count,
            failed_count,
            resolution_results,
            duration_ms: 0,
        })
    }

    /// 解决单个冲突（使用指定策略）
    pub async fn resolve_single_conflict_with_strategy(
        &self,
        record: &SyncStatusRecord,
        conflict: &ConflictData,
        strategy: ConflictResolutionStrategy,
    ) -> Result<EnhancedConflictResolutionResult> {
        // 根据策略解决冲突
        let resolved_content = match strategy {
            ConflictResolutionStrategy::LocalWins => conflict.local_content.clone(),
            ConflictResolutionStrategy::RemoteWins => conflict.remote_content.clone(),
            ConflictResolutionStrategy::Merge => {
                // 使用增强解决器进行智能合并
                let enhanced_result = self.enhanced_conflict_resolver
                    .resolve_conflict_intelligently(record, conflict).await?;
                enhanced_result.resolved_content
            }
            ConflictResolutionStrategy::UserChoice => {
                return Err(anyhow!("User choice strategy requires user interaction"));
            }
        };

        Ok(EnhancedConflictResolutionResult {
            strategy,
            resolved_content,
            enhanced_conflict: self.enhanced_conflict_resolver
                .analyze_conflict_details(record, conflict).await?,
            resolution_confidence: 85, // 基础置信度
            applied_field_resolutions: Vec::new(),
            resolved_by: "ENHANCED_RESOLVER".to_string(),
        })
    }

    /// 检查是否在线
    pub async fn is_online(&self) -> bool {
        let config = self.sync_config.read().await;
        config.is_online && config.remote_url.is_some()
    }

    /// 检查远程数据库是否已连接
    pub async fn is_remote_connected(&self) -> bool {
        let remote_db = self.remote_db.read().await;
        remote_db.is_some()
    }

    /// 获取远程统计信息（别名为get_sync_stats）
    pub async fn get_remote_stats(&self) -> Result<SyncStats> {
        self.get_sync_stats().await
    }

    /// 停止自动同步
    pub async fn stop_auto_sync(&self) -> Result<()> {
        let mut handle_guard = self.sync_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
        Ok(())
    }

    /// 断开远程数据库连接
    pub async fn disconnect_remote_db(&self) -> Result<()> {
        {
            let mut remote_db = self.remote_db.write().await;
            *remote_db = None;
        }
        
        {
            let mut config = self.sync_config.write().await;
            config.is_online = false;
        }
        
        // 停止自动同步
        self.stop_auto_sync().await?;
        
        info!("Disconnected from remote database");
        Ok(())
    }

    /// 执行同步操作（主要入口点）
    pub async fn sync(&self) -> Result<SyncStats> {
        // 检查是否已经在同步中
        if self.is_syncing.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(anyhow!("Sync is already in progress"));
        }
        
        info!("Starting sync operation");
        
        // 确保远程数据库连接已建立
        if let Err(e) = self.ensure_remote_db_connection().await {
            self.is_syncing.store(false, Ordering::SeqCst);
            return Err(anyhow!("Failed to establish remote database connection: {}", e));
        }
        
        // 使用混合同步策略
        let result = self.sync_hybrid().await;
        
        // 重置同步状态
        self.is_syncing.store(false, Ordering::SeqCst);
        
        result
    }

    /// 使用LibSQL进行WAL安全的同步
    pub async fn sync_with_libsql(&self) -> Result<SyncStats> {
        info!("Starting LibSQL-based sync");

        // 检查LibSQL适配器是否已配置
        if !self.libsql_adapter.is_configured().await {
            // 如果未配置，先尝试配置
            let config = self.sync_config.read().await;
            if config.sync_mode != crate::db::SyncMode::Offline && config.remote_url.is_some() {
                drop(config);
                if let Err(e) = self.configure_libsql_adapter().await {
                    return Err(anyhow!("Failed to configure LibSQL adapter: {}", e));
                }
            } else {
                return Err(anyhow!("LibSQL sync not configured and no valid remote configuration found"));
            }
        }

        // 执行LibSQL同步
        self.libsql_adapter.manual_sync().await
    }

    /// 配置LibSQL适配器
    async fn configure_libsql_adapter(&self) -> Result<()> {
        let config = self.sync_config.read().await.clone();
        self.libsql_adapter.configure(config).await
    }

    /// 配置远程数据库连接（延迟初始化方式，避免WAL冲突）
    pub async fn configure_remote_db(
        &self,
        remote_url: String,
        auth_token: Option<String>,
    ) -> Result<()> {
        info!("Configuring remote database: {}", remote_url);
        
        // 只更新配置，不创建实际的数据库副本（避免WAL冲突）
        {
            let mut config = self.sync_config.write().await;
            config.remote_url = Some(remote_url.clone());
            config.auth_token = auth_token.clone();
            config.is_online = true;
            config.updated_at = Utc::now().timestamp_millis();
            
            // 保存到数据库
            let conn = self.local_db.connect()?;
            db::save_sync_config(&conn, &config).await?;
        }
        
        info!("Remote database configuration saved, actual connection will be created when needed");
        Ok(())
    }
    
    /// 延迟创建远程数据库连接（在实际需要时调用）
    async fn ensure_remote_db_connection(&self) -> Result<()> {
        // 使用全局锁确保只有一个连接创建过程
        static CONNECTION_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
        let _lock = CONNECTION_MUTEX.lock().await;
        
        // 再次检查是否已经有连接（双重检查）
        {
            let remote_guard = self.remote_db.read().await;
            if remote_guard.is_some() {
                return Ok(());
            }
        }
        
        info!("Creating remote database connection");
        
        // 获取配置
        let (remote_url, auth_token) = {
            let config = self.sync_config.read().await;
            match (&config.remote_url, &config.auth_token) {
                (Some(url), token) => (url.clone(), token.clone()),
                _ => return Err(anyhow!("Remote database not configured")),
            }
        };
        
        // 使用完全隔离的临时目录，避免WAL冲突
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow!("Unable to determine app data directory"))?
            .join("mytips")
            .join("sync_replicas");
        
        // 确保目录存在
        tokio::fs::create_dir_all(&app_data_dir).await?;
        
        let replica_file = format!("replica_{}_{}.db", 
                                 std::process::id(), 
                                 chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
        let replica_path = app_data_dir.join(replica_file);
        let replica_path_str = replica_path.to_string_lossy().to_string();
        
        info!("Creating isolated remote replica at: {}", replica_path_str);
        
        // 清理任何可能存在的旧文件
        if replica_path.exists() {
            let _ = tokio::fs::remove_file(&replica_path).await;
        }
        
        // 清理相关的WAL和SHM文件
        let wal_path = format!("{}-wal", replica_path_str);
        let shm_path = format!("{}-shm", replica_path_str);
        let _ = tokio::fs::remove_file(&wal_path).await;
        let _ = tokio::fs::remove_file(&shm_path).await;
        
        // 等待文件系统操作完成
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        
        // 安全地构建远程数据库连接，使用错误处理避免空指针
        let remote_db = match if let Some(token) = auth_token {
            Builder::new_remote_replica(replica_path_str.clone(), remote_url.clone(), token)
                .build().await
        } else {
            Builder::new_remote_replica(replica_path_str.clone(), remote_url.clone(), String::new())
                .build().await
        } {
            Ok(db) => db,
            Err(e) => {
                warn!("Failed to create remote replica, cleaning up files: {}", e);
                // 清理失败的文件
                let _ = tokio::fs::remove_file(&replica_path).await;
                let _ = tokio::fs::remove_file(&wal_path).await;
                let _ = tokio::fs::remove_file(&shm_path).await;
                return Err(anyhow!("Failed to create remote replica: {}", e));
            }
        };
        
        // 安全地进行初始同步，避免WAL冲突
        info!("Performing safe initial sync");
        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        
        if let Err(e) = remote_db.sync().await {
            warn!("Initial sync failed, but continuing: {}", e);
            // 不要因为初始同步失败就完全失败，可能是网络问题
        }
        
        // 验证连接是否可用
        match remote_db.connect() {
            Ok(conn) => {
                // 尝试创建表结构
                if let Err(e) = crate::db::operations::create_all_tables(&conn).await {
                    warn!("Failed to create tables in remote replica: {}", e);
                }
                drop(conn);
            }
            Err(e) => {
                warn!("Remote replica connection test failed: {}", e);
                // 清理文件
                let _ = tokio::fs::remove_file(&replica_path).await;
                return Err(anyhow!("Remote replica connection failed: {}", e));
            }
        }
        
        // 设置远程数据库
        *self.remote_db.write().await = Some(remote_db);
        
        // 初始化增量同步管理器（如果尚未初始化）
        if self.incremental_sync_manager.read().await.is_none() {
            match IncrementalSyncManager::new(
                self.local_db.clone(),
                self.remote_db.clone(),
                self.performance_monitor.clone(),
                self.structured_logger.clone(),
            ).await {
                Ok(incremental_manager) => {
                    *self.incremental_sync_manager.write().await = Some(incremental_manager);
                    info!("Incremental sync manager initialized for remote database");
                }
                Err(e) => {
                    warn!("Failed to initialize incremental sync manager: {}", e);
                    // 不要因为这个失败就完全失败
                }
            }
        }
        
        info!("Remote database connection established successfully");
        Ok(())
    }

    /// 获取同步统计信息
    pub async fn get_sync_stats(&self) -> Result<SyncStats> {
        Ok(SyncStats {
            total_records: 0,
            synced_records: 0,
            pending_records: 0,
            failed_records: 0,
            last_sync_time: {
                let config = self.sync_config.read().await;
                config.last_sync_at
            },
            is_online: self.is_online().await,
        })
    }

    /// 验证一致性
    pub async fn validate_consistency(&self) -> Result<ConsistencyReport> {
        let tables = &["tips", "categories", "tags"];
        self.consistency_validator.validate_consistency(tables).await
    }

    /// 检测冲突
    async fn detect_conflict(
        &self,
        _local_conn: &Connection,
        _remote_conn: &Connection,
        _record: &SyncStatusRecord,
    ) -> Result<Option<ConflictData>> {
        // 简单的实现，返回None表示没有冲突
        Ok(None)
    }
}

impl Clone for SyncManager {
    fn clone(&self) -> Self {
        Self {
            local_db: self.local_db.clone(),
            remote_db: self.remote_db.clone(),
            sync_config: self.sync_config.clone(),
            sync_queue: self.sync_queue.clone(),
            is_syncing: self.is_syncing.clone(),
            conflict_resolver: self.conflict_resolver.clone(),
            sync_handle: self.sync_handle.clone(),
            connection_pool_manager: self.connection_pool_manager.clone(),
            cleanup_task_handle: self.cleanup_task_handle.clone(),
            transaction_manager: self.transaction_manager.clone(),
            consistency_validator: self.consistency_validator.clone(),
            performance_monitor: self.performance_monitor.clone(),
            structured_logger: self.structured_logger.clone(),
            builtin_sync_adapter: self.builtin_sync_adapter.clone(),
            enhanced_conflict_resolver: self.enhanced_conflict_resolver.clone(),
            incremental_sync_manager: self.incremental_sync_manager.clone(),
            connection_health_checker: self.connection_health_checker.clone(),
            libsql_adapter: self.libsql_adapter.clone(),
        }
    }
}

/// 标记同步记录
pub async fn mark_for_sync(
    _conn: &Connection,
    _table_name: &str,
    _record_id: &str,
    _operation: SyncOperation,
) -> Result<()> {
    // TODO: 实现实际的同步标记逻辑
    Ok(())
} 