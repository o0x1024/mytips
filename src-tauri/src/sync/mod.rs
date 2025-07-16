use anyhow::{anyhow, Result};
use chrono::Utc;
use futures::future::BoxFuture;
use libsql::{params, Connection, Database, Builder};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{Mutex, RwLock};
use uuid::Uuid;
use std::time::Duration;
use tokio::time::sleep;

mod connection_pool;
use connection_pool::{ConnectionPool, ConnectionPoolConfig, ConnectionPoolManager, PooledConnectionHandle, DatabaseConfig};

use crate::db::{
    self, SyncConfig, SyncMode, SyncStatus, SyncOperation, SyncStatusRecord,
    DataVersion, ConflictResolution, ConflictResolutionStrategy,
    DbManager, Tip, Category, Tag, TipTag, ClipboardHistory
};

// 同步守卫，确保同步状态在函数退出时被重置
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

// 同步管理器主结构体
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
}

// 冲突解决器
pub struct ConflictResolver {
    default_strategy: ConflictResolutionStrategy,
}

// 同步统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub total_records: u64,
    pub synced_records: u64,
    pub pending_records: u64,
    pub failed_records: u64,
    pub last_sync_time: i64,
    pub is_online: bool,
}

// 同步事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncEvent {
    SyncStarted,
    SyncProgress { current: u64, total: u64 },
    SyncCompleted { stats: SyncStats },
    SyncFailed { error: String },
    ConflictDetected { record_id: String, table_name: String },
    ConnectionStatusChanged { is_online: bool },
}

// 同步监听器类型
pub type SyncEventListener = Box<dyn Fn(SyncEvent) + Send + Sync>;

impl SyncManager {
    /// 创建新的同步管理器
    pub async fn new(local_db: Arc<Database>) -> Result<Self> {
        Self::new_with_db_path(local_db, None).await
    }
    
    /// 创建新的同步管理器（带数据库路径）
    pub async fn new_with_db_path(local_db: Arc<Database>, db_path: Option<String>) -> Result<Self> {
        let sync_config = {
            let conn = local_db.connect().map_err(|e| {
                anyhow!("Failed to connect to local database during SyncManager initialization: {}", e)
            })?;
            
            // 设置安全的数据库配置
            let _ = conn.execute("PRAGMA synchronous=NORMAL", ()).await;
            let _ = conn.execute("PRAGMA cache_size=10000", ()).await;
            let _ = conn.execute("PRAGMA temp_store=memory", ()).await;
            let _ = conn.execute("PRAGMA wal_autocheckpoint=1000", ()).await;
            
            let config = db::get_sync_config(&conn).await?;
            drop(conn); // 显式释放连接
            Arc::new(RwLock::new(config))
        };
        
        let conflict_resolver = Arc::new(ConflictResolver {
            default_strategy: ConflictResolutionStrategy::LocalWins,
        });
        
        // 创建连接池管理器 - 使用更保守的配置防止WAL冲突
        let pool_config = ConnectionPoolConfig {
            max_connections: 2, // 大幅减少最大连接数，避免WAL冲突
            idle_timeout: 60,   // 1分钟空闲超时，更快释放连接
            max_lifetime: 300,  // 5分钟最大生存时间，避免长时间持有连接
            acquire_timeout: 15, // 15秒获取连接超时，减少等待时间
        };
        let mut pool_manager = ConnectionPoolManager::new(pool_config);
        
        // 直接使用现有的本地数据库实例，避免连接池创建不同的数据库连接
        // 由于连接池需要DatabaseConfig来重新创建Database实例，
        // 但我们已经有了现有的Database实例，所以暂时不使用连接池的本地数据库功能
        // 改为直接使用SyncManager的local_db字段创建连接
        
        let manager = Self {
            local_db,
            remote_db: Arc::new(RwLock::new(None)),
            sync_config,
            sync_queue: Arc::new(Mutex::new(VecDeque::new())),
            is_syncing: Arc::new(AtomicBool::new(false)),
            conflict_resolver,
            sync_handle: Arc::new(Mutex::new(None)),
            connection_pool_manager: Arc::new(Mutex::new(pool_manager)),
            cleanup_task_handle: Arc::new(Mutex::new(None)),
        };
        
        // 如果配置中有远程数据库信息且不是离线模式，尝试自动连接
        {
            let config = manager.sync_config.read().await;
            if config.sync_mode != SyncMode::Offline {
                if let Some(ref remote_url) = config.remote_url {
                    let remote_url = remote_url.clone();
                    let auth_token = config.auth_token.clone();
                    drop(config);
                    
                    // 尝试建立远程连接（如果失败，只记录错误但不影响创建）
                    if let Err(e) = manager.configure_remote_db(remote_url, auth_token).await {
                        eprintln!("Failed to auto-connect to remote database during initialization: {}", e);
                    }
                }
            }
        }
        
        Ok(manager)
    }
    
    /// 配置远程数据库连接
    pub async fn configure_remote_db(
        &self,
        remote_url: String,
        auth_token: Option<String>,
    ) -> Result<()> {
        // 创建临时本地副本文件，使用更安全的路径
        let temp_dir = std::env::temp_dir();
        let temp_path = temp_dir.join(format!("mytips_replica_{}.db", Uuid::new_v4()));
        let temp_path_str = temp_path.to_string_lossy().to_string();
        
        println!("Creating remote replica at: {}", temp_path_str);
        
        // 清理可能存在的旧副本文件
        if temp_path.exists() {
            let _ = std::fs::remove_file(&temp_path);
        }
        
        // 构建远程数据库连接，添加超时和重试机制
        let remote_db = {
            let mut retry_count = 0;
            const MAX_BUILD_RETRIES: u32 = 3;
            
            loop {
                let build_result = if let Some(token) = auth_token.clone() {
                    // 为远程连接添加超时设置
                    tokio::time::timeout(
                        tokio::time::Duration::from_secs(30), // 30秒超时
                        Builder::new_remote_replica(temp_path_str.clone(), remote_url.clone(), token).build()
                    ).await
                } else {
                    tokio::time::timeout(
                        tokio::time::Duration::from_secs(30),
                        Builder::new_remote_replica(temp_path_str.clone(), remote_url.clone(), String::new()).build()
                    ).await
                };
                
                match build_result {
                    Ok(Ok(db)) => {
                        println!("Remote database builder created successfully");
                        break db;
                    }
                    Ok(Err(e)) => {
                        retry_count += 1;
                        let error_str = e.to_string();
                        eprintln!("Build attempt {} failed: {}", retry_count, error_str);
                        
                        if retry_count >= MAX_BUILD_RETRIES {
                            return Err(anyhow!("Failed to build remote database connection after {} attempts: {}", MAX_BUILD_RETRIES, e));
                        }
                        
                        // 检查是否是致命错误
                        if error_str.contains("invalid URL") || error_str.contains("authentication") {
                            return Err(anyhow!("Remote database connection configuration error: {}", e));
                        }
                        
                        // 在重试前等待
                        tokio::time::sleep(tokio::time::Duration::from_millis(1000 * retry_count as u64)).await;
                    }
                    Err(_) => {
                        retry_count += 1;
                        eprintln!("Build attempt {} timed out", retry_count);
                        
                        if retry_count >= MAX_BUILD_RETRIES {
                            return Err(anyhow!("Remote database connection timed out after {} attempts", MAX_BUILD_RETRIES));
                        }
                        
                        tokio::time::sleep(tokio::time::Duration::from_millis(2000 * retry_count as u64)).await;
                    }
                }
            }
        };
        
        // 测试连接，增加超时和重试机制
        for attempt in 0..3 {
            println!("Remote sync attempt {}", attempt + 1);
            
            // 为每次同步操作添加超时控制
            let sync_result = tokio::time::timeout(
                tokio::time::Duration::from_secs(60), // 60秒同步超时
                remote_db.sync()
            ).await;
            
            match sync_result {
                Ok(Ok(_)) => {
                    println!("Remote sync successful on attempt {}", attempt + 1);
                    break;
                }
                Ok(Err(e)) => {
                    let error_str = e.to_string();
                    eprintln!("Sync attempt {} failed: {}", attempt + 1, error_str);
                    
                    if attempt == 2 {
                        // 最后一次尝试失败，检查错误类型
                        if error_str.contains("database disk image is malformed") || 
                           error_str.contains("malformed") ||
                           error_str.contains("null pointer dereference") {
                            return Err(anyhow!("Remote database is corrupted or has WAL issues. Please reinitialize or contact administrator. Error: {}", e));
                        } else if error_str.contains("Timed out") || error_str.contains("timeout") {
                            return Err(anyhow!("Remote database connection timeout. Please check network connection and server status. Error: {}", e));
                        }
                        return Err(anyhow!("Failed to connect to remote database after 3 attempts: {}", e));
                    }
                    
                    // 在重试前等待，根据错误类型调整等待时间
                    let wait_time = if error_str.contains("Timed out") || error_str.contains("timeout") {
                        5000 * (attempt + 1) as u64 // 超时错误等待更久
                    } else {
                        2000 * (attempt + 1) as u64
                    };
                    tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
                }
                Err(_) => {
                    eprintln!("Sync attempt {} timed out", attempt + 1);
                    
                    if attempt == 2 {
                        return Err(anyhow!("Remote database sync operation timed out after 3 attempts. Please check network connection and server performance."));
                    }
                    
                    // 超时后等待更长时间再重试
                    tokio::time::sleep(tokio::time::Duration::from_millis(5000 * (attempt + 1) as u64)).await;
                }
            }
        }
        
        // 获取连接并设置安全的数据库配置，添加超时控制
        let remote_conn = tokio::time::timeout(
            tokio::time::Duration::from_secs(30),
            async {
                remote_db.connect().map_err(|e| {
                    anyhow!("Failed to get remote connection after sync: {}", e)
                })
            }
        ).await
        .map_err(|_| anyhow!("Remote connection establishment timed out"))??;
        
        // 设置数据库为WAL模式并配置安全参数
        let _ = remote_conn.execute("PRAGMA journal_mode=WAL", ()).await;
        let _ = remote_conn.execute("PRAGMA synchronous=NORMAL", ()).await;
        let _ = remote_conn.execute("PRAGMA cache_size=10000", ()).await;
        let _ = remote_conn.execute("PRAGMA temp_store=memory", ()).await;
        let _ = remote_conn.execute("PRAGMA mmap_size=268435456", ()).await; // 256MB
        
        // 执行数据库完整性检查
        match remote_conn.query("PRAGMA integrity_check", libsql::params!()).await {
            Ok(mut rows) => {
                if let Ok(Some(row)) = rows.next().await {
                    let result: String = row.get(0).unwrap_or_else(|_| "unknown".to_string());
                    if result != "ok" {
                        return Err(anyhow!("Remote database integrity check failed: {}", result));
                    }
                    println!("Remote database integrity check passed");
                }
            }
            Err(e) => {
                eprintln!("Warning: Failed to check remote database integrity: {}", e);
                // 继续执行，某些数据库可能不支持PRAGMA命令
            }
        }
        
        // 确保远程数据库有正确的表结构
        println!("Initializing remote database schema");
        crate::db::init_schema(&remote_conn).await.map_err(|e| {
            anyhow!("Failed to initialize remote database schema: {}", e)
        })?;
        
        // 释放连接，然后再次同步以确保表结构更新到远程
        drop(remote_conn);
        
        println!("Final sync to update remote schema");
        remote_db.sync().await.map_err(|e| {
            let error_str = e.to_string();
            if error_str.contains("database disk image is malformed") || 
               error_str.contains("malformed") ||
               error_str.contains("null pointer dereference") {
                anyhow!("Remote database became corrupted during schema sync. WAL issue detected. Error: {}", e)
            } else {
                anyhow!("Failed to sync schema to remote: {}", e)
            }
        })?;
        
        // 更新配置
        {
            let mut config = self.sync_config.write().await;
            config.remote_url = Some(remote_url.clone());
            config.auth_token = auth_token.clone();
            config.is_online = true;
            config.updated_at = Utc::now().timestamp_millis();
            
            // 保存到数据库
            let conn = self.local_db.connect()?;
            db::save_sync_config(&conn, &config).await?;
            drop(conn); // 显式释放连接
        }
        
        // 设置远程数据库
        *self.remote_db.write().await = Some(remote_db);
        
        // 将远程数据库配置添加到连接池管理器
        {
            let mut pool_manager = self.connection_pool_manager.lock().await;
            let remote_db_config = DatabaseConfig::Remote {
                replica_path: temp_path_str.clone(),
                remote_url: remote_url,
                auth_token: auth_token,
            };
            pool_manager.set_remote_database(remote_db_config).await;
        }
        
        // 启动清理任务
        {
            let mut cleanup_handle = self.cleanup_task_handle.lock().await;
            if cleanup_handle.is_none() {
                let pool_manager = self.connection_pool_manager.clone();
                let task_handle = tokio::spawn(async move {
                    let mut interval = tokio::time::interval(Duration::from_secs(300)); // 每5分钟清理一次
                    
                    loop {
                        interval.tick().await;
                        
                        let manager = pool_manager.lock().await;
                        let _cleanup_task = manager.start_cleanup_task();
                        // 清理任务会在后台运行，我们不需要等待它
                    }
                });
                *cleanup_handle = Some(task_handle);
            }
        }
        
        println!("Remote database configuration completed successfully");
        Ok(())
    }
    
    /// 断开远程数据库连接
    pub async fn disconnect_remote_db(&self) -> Result<()> {
        {
            let mut config = self.sync_config.write().await;
            config.remote_url = None;
            config.auth_token = None;
            config.is_online = false;
            config.sync_mode = SyncMode::Offline;
            config.updated_at = Utc::now().timestamp_millis();
            
            // 保存到数据库
            let conn = self.local_db.connect()?;
            db::save_sync_config(&conn, &config).await?;
        }
        
        // 清除远程数据库连接
        *self.remote_db.write().await = None;
        
        Ok(())
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
    
    /// 确保远程数据库连接（如果配置存在但连接不存在，则重新连接）
    pub async fn ensure_remote_connection(&self) -> Result<()> {
        let remote_db = self.remote_db.read().await;
        if remote_db.is_some() {
            // 连接已存在
            return Ok(());
        }
        drop(remote_db);
        
        // 检查配置中是否有远程数据库信息
        let config = self.sync_config.read().await;
        if let Some(ref remote_url) = config.remote_url {
            let remote_url = remote_url.clone();
            let auth_token = config.auth_token.clone();
            drop(config);
            
            // 重新建立连接
            self.configure_remote_db(remote_url, auth_token).await?;
        } else {
            return Err(anyhow!("No remote database configured"));
        }
        
        Ok(())
    }
    
    /// 设置同步模式
    pub async fn set_sync_mode(&self, mode: SyncMode) -> Result<()> {
        {
            let mut config = self.sync_config.write().await;
            config.sync_mode = mode.clone();
            config.updated_at = Utc::now().timestamp_millis();
            
            // 保存到数据库
            let conn = self.local_db.connect()?;
            db::save_sync_config(&conn, &config).await?;
        }
        
        // 如果切换到自动模式，启动自动同步
        if mode == SyncMode::Auto {
            self.start_auto_sync().await?;
        } else {
            self.stop_auto_sync().await?;
        }
        
        Ok(())
    }
    
    /// 启动自动同步
    pub async fn start_auto_sync(&self) -> Result<()> {
        let config = self.sync_config.read().await;
        if config.sync_mode != SyncMode::Auto || !config.auto_sync_enabled {
            return Ok(());
        }
        
        let sync_interval = config.sync_interval;
        drop(config);
        
        // 停止现有的同步任务
        self.stop_auto_sync().await?;
        
        // 创建新的同步任务
        let sync_manager = Arc::new(self.clone());
        let handle = tokio::spawn(async move {
            loop {
                sleep(Duration::from_secs(sync_interval as u64)).await;
                
                let config = sync_manager.sync_config.read().await;
                if config.sync_mode != SyncMode::Auto || !config.auto_sync_enabled {
                    break;
                }
                drop(config);
                
                if let Err(e) = sync_manager.sync().await {
                    eprintln!("Auto sync failed: {}", e);
                }
            }
        });
        
        *self.sync_handle.lock().await = Some(handle);
        
        Ok(())
    }
    
    /// 停止自动同步
    pub async fn stop_auto_sync(&self) -> Result<()> {
        let mut handle_guard = self.sync_handle.lock().await;
        if let Some(handle) = handle_guard.take() {
            handle.abort();
        }
        Ok(())
    }
    
    /// 执行同步操作（主要入口点）
    pub async fn sync(&self) -> Result<SyncStats> {
        // 检查是否已经在同步中
        if self.is_syncing.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_err() {
            return Err(anyhow!("Sync is already in progress"));
        }
        
        // 确保在函数退出时重置同步状态
        let _sync_guard = SyncGuard::new(&self.is_syncing);
        
        println!("Starting sync operation");
        
        // 检查数据库完整性
        println!("Checking database integrity");
        if let Err(e) = self.check_database_integrity().await {
            eprintln!("Database integrity check failed: {}", e);
            return Err(anyhow!("Database integrity check failed: {}", e));
        }
        
        // 为每个同步阶段创建独立的连接，避免借用冲突
        let (pending_records, synced_count, failed_count) = {
            println!("Performing data consistency check and cleanup");
            
            // 阶段1：数据清理 - 使用独立连接
            {
                let cleanup_conn = self.local_db.connect().map_err(|e| {
                    anyhow!("Failed to connect to local database for cleanup: {}", e)
                })?;
                
                // 设置安全的连接配置
                let _ = cleanup_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                let _ = cleanup_conn.execute("PRAGMA synchronous=FULL", ()).await;
                let _ = cleanup_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                
                self.cleanup_invalid_sync_records(&cleanup_conn).await?;
                drop(cleanup_conn); // 显式释放连接
            }
            
            // 阶段2：获取待同步记录 - 使用独立连接
            let pending_records = {
                let query_conn = self.local_db.connect().map_err(|e| {
                    anyhow!("Failed to connect to local database for query: {}", e)
                })?;
                
                // 设置安全的连接配置
                let _ = query_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                let _ = query_conn.execute("PRAGMA synchronous=FULL", ()).await;
                let _ = query_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                
                let records = db::get_pending_sync_records(&query_conn).await?;
                drop(query_conn); // 显式释放连接
                records
            };
            
            println!("Found {} pending sync records", pending_records.len());
            
            let mut synced_count = 0u64;
            let mut failed_count = 0u64;
            
            // 按依赖关系排序同步记录
            let sorted_records = self.sort_records_by_dependency(&pending_records);
            
            // 阶段3：记录同步 - 每个记录使用独立连接
            const BATCH_SIZE: usize = 1; // 单个记录处理，避免并发冲突
            for (batch_index, batch) in sorted_records.chunks(BATCH_SIZE).enumerate() {
                println!("Processing batch {}/{}", batch_index + 1, (sorted_records.len() + BATCH_SIZE - 1) / BATCH_SIZE);
                
                for (index, record) in batch.iter().enumerate() {
                    println!("Processing record {}/{}: table={}, record_id={}, operation={:?}", 
                             batch_index * BATCH_SIZE + index + 1, sorted_records.len(), 
                             record.table_name, record.record_id, record.operation);
                    
                    // 在每个记录同步前添加延迟，避免连接冲突
                    if batch_index > 0 || index > 0 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                    }
                    
                    // 为每个记录创建独立的验证连接
                    let validation_result = {
                        let validation_conn = self.local_db.connect().map_err(|e| {
                            eprintln!("Failed to create validation connection for record {}: {}", record.record_id, e);
                            anyhow!("Failed to create validation connection: {}", e)
                        })?;
                        
                        // 设置安全的连接配置
                        let _ = validation_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                        let _ = validation_conn.execute("PRAGMA synchronous=FULL", ()).await;
                        let _ = validation_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                        
                        let result = self.validate_sync_record(&validation_conn, record).await;
                        drop(validation_conn); // 显式释放连接
                        result
                    };
                    
                    // 检查记录是否仍然有效（防止数据不一致）
                    if !validation_result {
                        println!("Skipping invalid or deleted record: {}", record.record_id);
                        
                        // 使用独立连接更新同步状态
                        let update_conn = self.local_db.connect().map_err(|e| {
                            eprintln!("Failed to create update connection for record {}: {}", record.record_id, e);
                            anyhow!("Failed to create update connection: {}", e)
                        })?;
                        
                        // 设置安全的连接配置
                        let _ = update_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                        let _ = update_conn.execute("PRAGMA synchronous=FULL", ()).await;
                        let _ = update_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                        
                        if let Err(e) = db::update_sync_status(&update_conn, &record.id, SyncStatus::Synced, None).await {
                            eprintln!("Failed to update sync status for invalid record {}: {}", record.record_id, e);
                        }
                        drop(update_conn); // 显式释放连接
                        continue;
                    }
                    
                    // 执行记录同步
                    match self.sync_record_safe_isolated(record).await {
                        Ok(_) => {
                            synced_count += 1;
                            println!("Successfully synced record: {}", record.record_id);
                            
                            // 使用独立连接更新同步状态
                            let update_conn = self.local_db.connect().map_err(|e| {
                                eprintln!("Failed to create update connection for record {}: {}", record.record_id, e);
                                anyhow!("Failed to create update connection: {}", e)
                            })?;
                            
                            // 设置安全的连接配置
                            let _ = update_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                            let _ = update_conn.execute("PRAGMA synchronous=FULL", ()).await;
                            let _ = update_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                            
                            if let Err(e) = db::update_sync_status(&update_conn, &record.id, SyncStatus::Synced, None).await {
                                eprintln!("Failed to update sync status for record {}: {}", record.record_id, e);
                            }
                            drop(update_conn); // 显式释放连接
                        }
                        Err(e) => {
                            failed_count += 1;
                            let error_str = e.to_string();
                            eprintln!("Failed to sync record {}: {}", record.record_id, error_str);
                            
                            // 如果是borrowing相关错误，立即中断同步并进行恢复
                            if error_str.contains("already mutably borrowed") || 
                               error_str.contains("BorrowError") ||
                               error_str.contains("null pointer dereference") || 
                               error_str.contains("WAL") {
                                eprintln!("CRITICAL: Database borrowing/WAL error detected, stopping sync and performing emergency recovery...");
                                
                                // 执行紧急恢复
                                self.emergency_connection_recovery().await?;
                                
                                // 返回部分结果
                                return Ok(SyncStats {
                                    total_records: sorted_records.len() as u64,
                                    synced_records: synced_count,
                                    pending_records: (sorted_records.len() as u64).saturating_sub(synced_count + failed_count),
                                    failed_records: failed_count,
                                    last_sync_time: Utc::now().timestamp_millis(),
                                    is_online: false, // 标记为离线，需要手动重试
                                });
                            }
                            
                            // 使用独立连接更新失败状态
                            let update_conn = self.local_db.connect().map_err(|e| {
                                eprintln!("Failed to create update connection for failed record {}: {}", record.record_id, e);
                                anyhow!("Failed to create update connection: {}", e)
                            })?;
                            
                            // 设置安全的连接配置
                            let _ = update_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                            let _ = update_conn.execute("PRAGMA synchronous=FULL", ()).await;
                            let _ = update_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                            
                            if let Err(e2) = db::update_sync_status(&update_conn, &record.id, SyncStatus::Failed, Some(e.to_string())).await {
                                eprintln!("Failed to update sync status for failed record {}: {}", record.record_id, e2);
                            }
                            drop(update_conn); // 显式释放连接
                        }
                    }
                }
                
                // 在批次之间添加更长的延迟，确保连接稳定
                if batch_index < (sorted_records.len() + BATCH_SIZE - 1) / BATCH_SIZE - 1 {
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
                }
            }
            
            println!("Sync record processing complete. Synced: {}, Failed: {}", synced_count, failed_count);
            
            // 阶段4：远程同步 - 使用独立连接
            println!("Starting remote to local sync");
            {
                let remote_sync_conn = self.local_db.connect().map_err(|e| {
                    anyhow!("Failed to connect to local database for remote sync: {}", e)
                })?;
                
                // 设置安全的连接配置
                let _ = remote_sync_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                let _ = remote_sync_conn.execute("PRAGMA synchronous=FULL", ()).await;
                let _ = remote_sync_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                
                if let Err(e) = self.sync_from_remote_safe(&remote_sync_conn).await {
                    eprintln!("Failed to sync from remote: {}", e);
                } else {
                    println!("Remote to local sync completed successfully");
                }
                drop(remote_sync_conn); // 显式释放连接
            }
            
            // 阶段5：更新配置 - 使用独立连接
            {
                let config_conn = self.local_db.connect().map_err(|e| {
                    anyhow!("Failed to connect to local database for config update: {}", e)
                })?;
                
                // 设置安全的连接配置
                let _ = config_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
                let _ = config_conn.execute("PRAGMA synchronous=FULL", ()).await;
                let _ = config_conn.execute("PRAGMA busy_timeout=30000", ()).await;
                
                // 更新最后同步时间
                {
                    let mut config = self.sync_config.write().await;
                    config.last_sync_at = Utc::now().timestamp_millis();
                    println!("Saving sync config: mode={:?}, remote_url={:?}", config.sync_mode, config.remote_url);
                    if let Err(e) = db::save_sync_config(&config_conn, &config).await {
                        eprintln!("Failed to save sync config: {}", e);
                    } else {
                        println!("Sync config saved successfully");
                    }
                }
                
                // 清理已同步的记录
                println!("Cleaning up synced records");
                if let Err(e) = db::clear_synced_records(&config_conn).await {
                    eprintln!("Failed to clear synced records: {}", e);
                }
                
                drop(config_conn); // 显式释放连接
            }
            
            (pending_records, synced_count, failed_count)
        };
        
        let stats = SyncStats {
            total_records: pending_records.len() as u64,
            synced_records: synced_count,
            pending_records: (pending_records.len() as u64).saturating_sub(synced_count + failed_count),
            failed_records: failed_count,
            last_sync_time: Utc::now().timestamp_millis(),
            is_online: true,
        };
        
        println!("Sync operation completed with stats: {:?}", stats);
        Ok(stats)
    }
    
    /// 检查数据库完整性
    async fn check_database_integrity(&self) -> Result<()> {
        println!("Checking database integrity...");
        
        // 检查本地数据库
        let local_conn = self.local_db.connect()?;
        
        // 禁用WAL模式进行检查
        let _ = local_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
        let _ = local_conn.execute("PRAGMA synchronous=FULL", ()).await;
        
        match local_conn.query("PRAGMA integrity_check", ()).await {
            Ok(mut rows) => {
                if let Ok(Some(row)) = rows.next().await {
                    let result: String = row.get(0).unwrap_or_else(|_| "unknown".to_string());
                    if result != "ok" {
                        drop(local_conn);
                        return Err(anyhow!("Local database integrity check failed: {}", result));
                    }
                    println!("Local database integrity check passed");
                }
            }
            Err(e) => {
                drop(local_conn);
                return Err(anyhow!("Failed to check local database integrity: {}", e));
            }
        }
        
        drop(local_conn);
        
        // 检查远程数据库（如果已连接）
        if self.is_remote_connected().await {
            let remote_db = self.remote_db.read().await;
            if let Some(ref db) = *remote_db {
                let remote_conn = db.connect().map_err(|e| {
                    anyhow!("Failed to connect to remote database for integrity check: {}", e)
                })?;
                
                match remote_conn.query("PRAGMA integrity_check", ()).await {
                    Ok(mut rows) => {
                        if let Ok(Some(row)) = rows.next().await {
                            let result: String = row.get(0).unwrap_or_else(|_| "unknown".to_string());
                            if result != "ok" {
                                drop(remote_conn);
                                drop(remote_db);
                                return Err(anyhow!("Remote database integrity check failed: {}", result));
                            }
                            println!("Remote database integrity check passed");
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to check remote database integrity: {}", e);
                        // 远程数据库检查失败不阻止同步，只记录警告
                    }
                }
                
                drop(remote_conn);
            }
            drop(remote_db);
        }
        
        Ok(())
    }
    

    
    /// 清理无效的同步记录
    async fn cleanup_invalid_sync_records(&self, local_conn: &Connection) -> Result<()> {
        println!("Starting cleanup of invalid sync records");
        
        // 清理不存在的categories记录
        let deleted_count = local_conn.execute(
            "DELETE FROM sync_status 
             WHERE table_name = 'categories' 
             AND record_id NOT IN (SELECT id FROM categories)",
            ()
        ).await?;
        println!("Cleaned up {} invalid category sync records", deleted_count);
        
        // 清理不存在的tips记录
        let deleted_count = local_conn.execute(
            "DELETE FROM sync_status 
             WHERE table_name = 'tips' 
             AND record_id NOT IN (SELECT id FROM tips)",
            ()
        ).await?;
        println!("Cleaned up {} invalid tip sync records", deleted_count);
        
        // 清理不存在的tags记录
        let deleted_count = local_conn.execute(
            "DELETE FROM sync_status 
             WHERE table_name = 'tags' 
             AND record_id NOT IN (SELECT id FROM tags)",
            ()
        ).await?;
        println!("Cleaned up {} invalid tag sync records", deleted_count);
        
        // 清理重复的同步记录（保留最新的）
        let deleted_count = local_conn.execute(
            "DELETE FROM sync_status 
             WHERE id NOT IN (
                 SELECT MIN(id) 
                 FROM sync_status 
                 GROUP BY table_name, record_id, operation
             )",
            ()
        ).await?;
        println!("Cleaned up {} duplicate sync records", deleted_count);
        
        Ok(())
    }
    
    /// 验证同步记录是否有效
    async fn validate_sync_record(&self, local_conn: &Connection, record: &SyncStatusRecord) -> bool {
        match record.table_name.as_str() {
            "categories" => {
                if record.operation == SyncOperation::Delete {
                    return true; // DELETE操作不需要记录存在
                }
                
                let mut rows = match local_conn.query(
                    "SELECT EXISTS(SELECT 1 FROM categories WHERE id = ?)",
                    params![record.record_id.clone()]
                ).await {
                    Ok(rows) => rows,
                    Err(_) => return false,
                };
                
                if let Ok(Some(row)) = rows.next().await {
                    row.get(0).unwrap_or(false)
                } else {
                    false
                }
            }
            "tips" => {
                if record.operation == SyncOperation::Delete {
                    return true;
                }
                
                let mut rows = match local_conn.query(
                    "SELECT EXISTS(SELECT 1 FROM tips WHERE id = ?)",
                    params![record.record_id.clone()]
                ).await {
                    Ok(rows) => rows,
                    Err(_) => return false,
                };
                
                if let Ok(Some(row)) = rows.next().await {
                    row.get(0).unwrap_or(false)
                } else {
                    false
                }
            }
            "tags" => {
                if record.operation == SyncOperation::Delete {
                    return true;
                }
                
                let mut rows = match local_conn.query(
                    "SELECT EXISTS(SELECT 1 FROM tags WHERE id = ?)",
                    params![record.record_id.clone()]
                ).await {
                    Ok(rows) => rows,
                    Err(_) => return false,
                };
                
                if let Ok(Some(row)) = rows.next().await {
                    row.get(0).unwrap_or(false)
                } else {
                    false
                }
            }
            _ => true, // 未知表类型，默认有效
        }
    }
    

    
    /// 按依赖关系排序同步记录
    fn sort_records_by_dependency<'a>(&self, records: &'a [SyncStatusRecord]) -> Vec<&'a SyncStatusRecord> {
        let mut sorted_records = Vec::new();
        
        // 定义表的依赖优先级（数字越小优先级越高）
        let table_priority = |table_name: &str| -> u8 {
            match table_name {
                "categories" => 1,  // 分类优先级最高
                "tags" => 2,        // 标签次之
                "tips" => 3,        // 笔记依赖分类和标签
                _ => 99,            // 其他表最低优先级
            }
        };
        
        // 按表优先级和操作类型排序
        let mut records_vec: Vec<&SyncStatusRecord> = records.iter().collect();
        records_vec.sort_by(|a, b| {
            let priority_a = table_priority(&a.table_name);
            let priority_b = table_priority(&b.table_name);
            
            // 先按表优先级排序
            if priority_a != priority_b {
                return priority_a.cmp(&priority_b);
            }
            
            // 同一个表内，Insert操作优先于Update和Delete
            let op_priority = |op: &SyncOperation| -> u8 {
                match op {
                    SyncOperation::Insert => 1,
                    SyncOperation::Update => 2,
                    SyncOperation::Delete => 3,
                }
            };
            
            op_priority(&a.operation).cmp(&op_priority(&b.operation))
        });
        
        sorted_records.extend(records_vec);
        sorted_records
    }
    
    /// 同步单个记录（安全版本 - 使用独立连接和超时控制）
    async fn sync_record_safe(&self, record: &SyncStatusRecord) -> Result<()> {
        // 为整个同步操作添加超时控制
        tokio::time::timeout(
            tokio::time::Duration::from_secs(120), // 2分钟超时
            self.sync_record_with_retry(record)
        ).await
        .map_err(|_| anyhow!("Sync record operation timed out for record: {}", record.record_id))?
    }
    
    /// 带重试机制的同步记录方法
    async fn sync_record_with_retry(&self, record: &SyncStatusRecord) -> Result<()> {
        let mut retry_count = 0;
        const MAX_RECORD_RETRIES: u32 = 2; // 减少重试次数，快速失败
        
        while retry_count < MAX_RECORD_RETRIES {
            match self.try_sync_single_record(record).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    retry_count += 1;
                    let error_str = e.to_string();
                    eprintln!("Sync record attempt {} failed for {}: {}", retry_count, record.record_id, error_str);
                    
                    // WAL相关错误立即失败，不进行重试
                    if error_str.contains("null pointer dereference") || error_str.contains("WAL") {
                        eprintln!("WAL error detected, aborting retries for record: {}", record.record_id);
                        return Err(anyhow!("WAL error syncing record {}: {}", record.record_id, e));
                    }
                    
                    if retry_count >= MAX_RECORD_RETRIES {
                        return Err(anyhow!("Failed to sync record {} after {} attempts: {}", record.record_id, MAX_RECORD_RETRIES, e));
                    }
                    
                    // 检查是否是不可恢复的错误
                    if error_str.contains("database disk image is malformed") ||
                       error_str.contains("authentication") ||
                       error_str.contains("Category not found") {
                        return Err(anyhow!("Fatal error syncing record {}: {}", record.record_id, e));
                    }
                    
                    // 短暂等待后重试
                    let wait_time = if error_str.contains("Timed out") || error_str.contains("timeout") {
                        2000 * retry_count as u64 // 超时错误等待更久
                    } else {
                        500 * retry_count as u64
                    };
                    tokio::time::sleep(tokio::time::Duration::from_millis(wait_time)).await;
                }
            }
        }
        
        Err(anyhow!("Sync record failed after all retries"))
    }
    
    /// 尝试同步单个记录（混合模式：本地直连，远程连接池）
    async fn try_sync_single_record(&self, record: &SyncStatusRecord) -> Result<()> {
        // 直接从本地数据库创建连接，确保使用正确的数据库文件
        let local_conn = self.local_db.connect()
            .map_err(|e| anyhow!("Failed to create local connection: {}", e))?;
        
        // 为本地连接设置安全参数，避免与远程连接的WAL冲突
        let _ = local_conn.execute("PRAGMA synchronous=FULL", ()).await;
        let _ = local_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
        let _ = local_conn.execute("PRAGMA busy_timeout=30000", ()).await;
        
        // 从连接池获取远程连接
        let pool_manager = self.connection_pool_manager.lock().await;
        let remote_handle = pool_manager.acquire_remote().await
            .map_err(|e| anyhow!("Failed to acquire remote connection from pool: {}", e))?;
        drop(pool_manager); // 释放池管理器锁
        
        let remote_conn = remote_handle.connection();
        
        let result = match record.operation {
            SyncOperation::Insert => {
                self.sync_insert_record(&local_conn, remote_conn, record).await
            }
            SyncOperation::Update => {
                self.sync_update_record(&local_conn, remote_conn, record).await
            }
            SyncOperation::Delete => {
                self.sync_delete_record(&local_conn, remote_conn, record).await
            }
        };
        
        // 显式释放本地连接，远程连接会在句柄被drop时自动归还到池中
        drop(local_conn);
        
        result
    }

    /// 同步单个记录（原版本 - 保留用于向后兼容）
    async fn sync_record(&self, local_conn: &Connection, record: &SyncStatusRecord) -> Result<()> {
        let remote_db = self.remote_db.read().await;
        let remote_db = remote_db.as_ref().ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        
        match record.operation {
            SyncOperation::Insert => {
                self.sync_insert_record(local_conn, &remote_conn, record).await
            }
            SyncOperation::Update => {
                self.sync_update_record(local_conn, &remote_conn, record).await
            }
            SyncOperation::Delete => {
                self.sync_delete_record(local_conn, &remote_conn, record).await
            }
        }
    }
    
    /// 同步插入记录
    async fn sync_insert_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        record: &SyncStatusRecord,
    ) -> Result<()> {
        println!("Syncing insert record: table={}, record_id={}", record.table_name, record.record_id);
        
        match record.table_name.as_str() {
            "tips" => {
                println!("Getting tip from local database: {}", record.record_id);
                let tip = db::get_tip(local_conn, &record.record_id).await.map_err(|e| {
                    eprintln!("Failed to get tip from local database: {}", e);
                    anyhow!("Failed to get tip from local database: {}", e)
                })?;
                
                // 先同步依赖的分类
                if let Some(ref category_id) = tip.category_id {
                    if let Err(e) = self.ensure_category_exists(local_conn, remote_conn, category_id).await {
                        println!("Warning: Failed to ensure category exists: {}", e);
                        // 如果分类同步失败，将tip的category_id设为None以避免外键约束失败
                        let mut tip_clone = tip.clone();
                        tip_clone.category_id = None;
                        
                        println!("Saving tip to remote database without category: {}", tip_clone.id);
                        db::save_tip(remote_conn, &tip_clone).await.map_err(|e| {
                            eprintln!("Failed to save tip to remote database: {}", e);
                            anyhow!("Failed to save tip to remote database: {}", e)
                        })?;
                        
                        println!("Successfully synced tip without category: {}", tip_clone.id);
                        return Ok(());
                    }
                }
                
                // 同步依赖的标签
                for tag in &tip.tags {
                    if let Err(e) = self.ensure_tag_exists(remote_conn, tag).await {
                        println!("Warning: Failed to ensure tag {} exists: {}", tag.name, e);
                    }
                }
                
                println!("Saving tip to remote database: {}", tip.id);
                db::save_tip(remote_conn, &tip).await.map_err(|e| {
                    eprintln!("Failed to save tip to remote database: {}", e);
                    anyhow!("Failed to save tip to remote database: {}", e)
                })?;
                
                println!("Successfully synced tip: {}", tip.id);
            }
            "categories" => {
                println!("Getting category from local database: {}", record.record_id);
                let category = db::get_category_by_id(local_conn, &record.record_id).await.map_err(|e| {
                    eprintln!("Failed to get category from local database: {}", e);
                    anyhow!("Failed to get category from local database: {}", e)
                })?;
                
                // 如果有父分类，先确保父分类存在
                if let Some(ref parent_id) = category.parent_id {
                    if let Err(e) = self.ensure_category_exists(local_conn, remote_conn, parent_id).await {
                        println!("Warning: Failed to ensure parent category exists: {}", e);
                    }
                }
                
                println!("Saving category to remote database: {}", category.id);
                // 直接插入分类而不是调用create_category（避免生成新ID）
                remote_conn.execute(
                    "INSERT OR REPLACE INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
                    params![category.id.clone(), category.name.clone(), category.parent_id.clone()],
                ).await.map_err(|e| {
                    eprintln!("Failed to save category to remote database: {}", e);
                    anyhow!("Failed to save category to remote database: {}", e)
                })?;
                
                println!("Successfully synced category: {}", category.id);
            }
            "tags" => {
                println!("Getting tag from local database: {}", record.record_id);
                // 获取标签信息
                let mut rows = local_conn.query(
                    "SELECT id, name FROM tags WHERE id = ?", 
                    params![record.record_id.clone()]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    let tag_id: String = row.get(0)?;
                    let tag_name: String = row.get(1)?;
                    
                    let tag = Tag {
                        id: tag_id,
                        name: tag_name,
                    };
                    
                    println!("Saving tag to remote database: {}", tag.id);
                    if let Err(e) = self.ensure_tag_exists(remote_conn, &tag).await {
                        eprintln!("Failed to save tag to remote database: {}", e);
                        return Err(anyhow!("Failed to save tag to remote database: {}", e));
                    }
                    
                    println!("Successfully synced tag: {}", tag.id);
                } else {
                    return Err(anyhow!("Tag not found in local database: {}", record.record_id));
                }
            }
            _ => {
                return Err(anyhow!("Unsupported table for sync: {}", record.table_name));
            }
        }
        Ok(())
    }
    
    /// 同步更新记录
    async fn sync_update_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        record: &SyncStatusRecord,
    ) -> Result<()> {
        println!("Syncing update record: table={}, record_id={}", record.table_name, record.record_id);
        
        // 检查冲突
        if let Some(conflict) = self.detect_conflict(local_conn, remote_conn, record).await? {
            println!("Conflict detected for record: {}", record.record_id);
            // 处理冲突
            self.resolve_conflict(local_conn, remote_conn, record, conflict).await?;
        } else {
            println!("No conflict detected, proceeding with sync");
            // 无冲突，直接同步
            self.sync_insert_record(local_conn, remote_conn, record).await?;
        }
        Ok(())
    }
    
    /// 同步删除记录
    async fn sync_delete_record(
        &self,
        _local_conn: &Connection,
        remote_conn: &Connection,
        record: &SyncStatusRecord,
    ) -> Result<()> {
        println!("Syncing delete record: table={}, record_id={}", record.table_name, record.record_id);
        
        match record.table_name.as_str() {
            "tips" => {
                println!("Deleting tip from remote database: {}", record.record_id);
                db::delete_tip(remote_conn, &record.record_id).await.map_err(|e| {
                    eprintln!("Failed to delete tip from remote database: {}", e);
                    anyhow!("Failed to delete tip from remote database: {}", e)
                })?;
                println!("Successfully deleted tip from remote: {}", record.record_id);
            }
            "categories" => {
                println!("Deleting category from remote database: {}", record.record_id);
                db::delete_category(remote_conn, &record.record_id).await.map_err(|e| {
                    eprintln!("Failed to delete category from remote database: {}", e);
                    anyhow!("Failed to delete category from remote database: {}", e)
                })?;
                println!("Successfully deleted category from remote: {}", record.record_id);
            }
            _ => {
                return Err(anyhow!("Unsupported table for sync: {}", record.table_name));
            }
        }
        Ok(())
    }
    
    /// 从远程同步到本地（安全版本）
    async fn sync_from_remote_safe(&self, local_conn: &Connection) -> Result<()> {
        // 使用重试机制和错误恢复
        let mut retry_count = 0;
        const MAX_RETRIES: u32 = 3;
        
        while retry_count < MAX_RETRIES {
            match self.try_sync_from_remote(local_conn).await {
                Ok(()) => return Ok(()),
                Err(e) => {
                    retry_count += 1;
                    eprintln!("Sync from remote attempt {} failed: {}", retry_count, e);
                    
                    if retry_count >= MAX_RETRIES {
                        return Err(anyhow!("Failed to sync from remote after {} attempts: {}", MAX_RETRIES, e));
                    }
                    
                    // 在重试前等待一段时间
                    tokio::time::sleep(tokio::time::Duration::from_millis(1000 * retry_count as u64)).await;
                }
            }
        }
        
        Err(anyhow!("Sync from remote failed after all retries"))
    }
    
    /// 尝试从远程同步到本地
    async fn try_sync_from_remote(&self, local_conn: &Connection) -> Result<()> {
        let remote_db = self.remote_db.read().await;
        let remote_db = remote_db.as_ref().ok_or_else(|| anyhow!("Remote database not connected"))?;
        
        // 创建独立的远程连接
        let remote_conn = remote_db.connect().map_err(|e| {
            anyhow!("Failed to create remote connection: {}", e)
        })?;
        drop(remote_db); // 释放读锁
        
        let result = self.perform_remote_to_local_sync(local_conn, &remote_conn).await;
        
        // 显式关闭远程连接
        drop(remote_conn);
        
        result
    }

    /// 执行实际的远程到本地同步
    async fn perform_remote_to_local_sync(&self, local_conn: &Connection, remote_conn: &Connection) -> Result<()> {
        // 先同步分类（因为tips可能依赖分类）
        println!("Syncing categories from remote to local");
        let remote_categories = db::get_all_categories(&remote_conn).await?;
        for category in remote_categories {
            // 检查本地是否存在该分类
            match db::get_category_by_id(local_conn, &category.id).await {
                Ok(_) => {
                    // 本地已存在，可能需要更新（这里简化处理，不做更新）
                    // 在实际应用中，可以比较updated_at时间戳
                }
                Err(_) => {
                    // 本地不存在，直接插入
                    local_conn.execute(
                        "INSERT OR REPLACE INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
                        params![category.id.clone(), category.name.clone(), category.parent_id.clone()],
                    ).await?;
                    println!("Synced category from remote: {} ({})", category.name, category.id);
                }
            }
        }
        
        // 同步标签
        println!("Syncing tags from remote to local");
        let remote_tags = db::get_all_tags(&remote_conn).await?;
        for tag in remote_tags {
            // 检查本地是否存在该标签
            let mut rows = local_conn.query(
                "SELECT EXISTS(SELECT 1 FROM tags WHERE id = ?)",
                params![tag.id.clone()]
            ).await?;
            
            let exists: bool = if let Some(row) = rows.next().await? {
                row.get(0)?
            } else {
                false
            };
            
            if !exists {
                // 本地不存在，直接插入
                local_conn.execute(
                    "INSERT OR IGNORE INTO tags (id, name) VALUES (?, ?)",
                    params![tag.id.clone(), tag.name.clone()],
                ).await?;
                println!("Synced tag from remote: {} ({})", tag.name, tag.id);
            }
        }
        
        // 同步笔记
        println!("Syncing tips from remote to local");
        let remote_tips = db::get_all_tips(&remote_conn).await?;
        for tip in remote_tips {
            // 检查本地是否存在
            match db::get_tip(local_conn, &tip.id).await {
                Ok(local_tip) => {
                    // 检查是否需要更新
                    if tip.updated_at > local_tip.updated_at {
                        // 需要先获取完整的tip信息（包括标签）
                        let tip_with_tags = self.get_tip_with_tags(&remote_conn, &tip).await?;
                        db::save_tip(local_conn, &tip_with_tags).await?;
                        println!("Updated tip from remote: {} ({})", tip.title, tip.id);
                    }
                }
                Err(_) => {
                    // 本地不存在，直接插入
                    let tip_with_tags = self.get_tip_with_tags(&remote_conn, &tip).await?;
                    db::save_tip(local_conn, &tip_with_tags).await?;
                    println!("Synced tip from remote: {} ({})", tip.title, tip.id);
                }
            }
        }
        
        Ok(())
    }

    /// 从远程同步到本地（原版本 - 保留用于向后兼容）
    async fn sync_from_remote(&self, local_conn: &Connection) -> Result<()> {
        let remote_db = self.remote_db.read().await;
        let remote_db = remote_db.as_ref().ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        
        self.perform_remote_to_local_sync(local_conn, &remote_conn).await
    }
    
    /// 获取包含标签的完整tip信息
    async fn get_tip_with_tags(&self, conn: &Connection, tip: &Tip) -> Result<Tip> {
        let tags = db::get_tip_tags(conn, &tip.id).await?;
        
        let mut tip_with_tags = tip.clone();
        tip_with_tags.tags = tags;
        
        Ok(tip_with_tags)
    }
    
    /// 检测冲突
    async fn detect_conflict(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        record: &SyncStatusRecord,
    ) -> Result<Option<ConflictData>> {
        // 获取本地版本
        let local_version = db::get_or_create_data_version(
            local_conn,
            &record.table_name,
            &record.record_id,
            &self.get_record_content(local_conn, &record.table_name, &record.record_id).await?,
        ).await?;
        
        // 获取远程版本（如果存在）
        let remote_content = match self.get_record_content(remote_conn, &record.table_name, &record.record_id).await {
            Ok(content) => content,
            Err(_) => return Ok(None), // 远程不存在，无冲突
        };
        
        let remote_version = db::get_or_create_data_version(
            remote_conn,
            &record.table_name,
            &record.record_id,
            &remote_content,
        ).await?;
        
        // 比较版本
        if local_version.checksum != remote_version.checksum {
            Ok(Some(ConflictData {
                local_version,
                remote_version,
                local_content: self.get_record_content(local_conn, &record.table_name, &record.record_id).await?,
                remote_content,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// 解决冲突
    async fn resolve_conflict(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        record: &SyncStatusRecord,
        conflict: ConflictData,
    ) -> Result<()> {
        let resolution = self.conflict_resolver.resolve_conflict(record, &conflict).await?;
        
        match resolution.strategy {
            ConflictResolutionStrategy::LocalWins => {
                // 本地数据胜出，同步到远程
                self.sync_insert_record(local_conn, remote_conn, record).await?;
            }
            ConflictResolutionStrategy::RemoteWins => {
                // 远程数据胜出，同步到本地
                self.apply_remote_data(local_conn, &record.table_name, &record.record_id, &conflict.remote_content).await?;
            }
            ConflictResolutionStrategy::Merge => {
                // 尝试合并数据
                let merged_content = self.merge_record_content(record, &conflict)?;
                self.apply_merged_data(local_conn, remote_conn, &record.table_name, &record.record_id, &merged_content).await?;
            }
            ConflictResolutionStrategy::UserChoice => {
                // 等待用户选择
                db::update_sync_status(local_conn, &record.id, SyncStatus::Conflict, Some(serde_json::to_string(&conflict)?)).await?;
                return Ok(());
            }
        }
        
        // 记录冲突解决
        db::record_conflict_resolution(
            local_conn,
            &record.table_name,
            &record.record_id,
            resolution.strategy,
            &resolution.resolved_by,
        ).await?;
        
        Ok(())
    }
    
    /// 获取记录内容
    async fn get_record_content(&self, conn: &Connection, table_name: &str, record_id: &str) -> Result<String> {
        match table_name {
            "tips" => {
                let tip = db::get_tip(conn, record_id).await?;
                Ok(serde_json::to_string(&tip)?)
            }
            _ => Err(anyhow!("Unsupported table: {}", table_name)),
        }
    }
    
    /// 应用远程数据到本地
    async fn apply_remote_data(&self, local_conn: &Connection, table_name: &str, record_id: &str, content: &str) -> Result<()> {
        match table_name {
            "tips" => {
                let tip: Tip = serde_json::from_str(content)?;
                db::save_tip(local_conn, &tip).await?;
            }
            _ => return Err(anyhow!("Unsupported table: {}", table_name)),
        }
        Ok(())
    }
    
    /// 应用合并后的数据
    async fn apply_merged_data(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        _record_id: &str,
        content: &str,
    ) -> Result<()> {
        match table_name {
            "tips" => {
                let tip: Tip = serde_json::from_str(content)?;
                db::save_tip(local_conn, &tip).await?;
                db::save_tip(remote_conn, &tip).await?;
            }
            _ => return Err(anyhow!("Unsupported table: {}", table_name)),
        }
        Ok(())
    }
    
    /// 合并记录内容
    fn merge_record_content(&self, record: &SyncStatusRecord, conflict: &ConflictData) -> Result<String> {
        match record.table_name.as_str() {
            "tips" => {
                let local_tip: Tip = serde_json::from_str(&conflict.local_content)?;
                let remote_tip: Tip = serde_json::from_str(&conflict.remote_content)?;
                
                // 简单合并策略：使用最新的更新时间
                let merged_tip = if local_tip.updated_at > remote_tip.updated_at {
                    local_tip
                } else {
                    remote_tip
                };
                
                Ok(serde_json::to_string(&merged_tip)?)
            }
            _ => Err(anyhow!("Unsupported table for merge: {}", record.table_name)),
        }
    }
    
    /// 获取同步统计信息
    pub async fn get_sync_stats(&self) -> Result<SyncStats> {
        let conn = self.local_db.connect()?;
        let pending_records = db::get_pending_sync_records(&conn).await?;
        
        // 显式关闭连接（libsql 会在drop时关闭，但我们提前释放）
        drop(conn);
        let config = self.sync_config.read().await;
        
        let total_records = pending_records.len() as u64;
        let failed_records = pending_records.iter()
            .filter(|r| r.sync_status == SyncStatus::Failed)
            .count() as u64;
        
        Ok(SyncStats {
            total_records,
            synced_records: 0, // 已同步的记录在清理后为0
            pending_records: total_records - failed_records,
            failed_records,
            last_sync_time: config.last_sync_at,
            is_online: config.is_online,
        })
    }
    
    /// 重置同步状态
    pub async fn reset_sync_status(&self) -> Result<()> {
        let conn = self.local_db.connect()?;
        
        // 使用事务批量执行操作，减少数据库往返
        conn.execute("BEGIN TRANSACTION", ()).await?;
        
        // 清除所有同步记录
        conn.execute("DELETE FROM sync_status", ()).await?;
        
        // 重置数据版本
        conn.execute("DELETE FROM data_versions", ()).await?;
        
        // 重置冲突解决记录
        conn.execute("DELETE FROM conflict_resolutions", ()).await?;
        
        conn.execute("COMMIT", ()).await?;
        
        // 显式关闭连接
        drop(conn);
        
        Ok(())
    }
    
    /// 迁移本地数据到远程
    pub async fn migrate_to_remote(&self) -> Result<()> {
        let remote_db = self.remote_db.read().await;
        let remote_db = remote_db.as_ref().ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        let local_conn = self.local_db.connect()?;
        
        // 迁移所有笔记
        let tips = db::get_all_tips(&local_conn).await?;
        for tip in tips {
            db::save_tip(&remote_conn, &tip).await?;
        }
        
        // 迁移所有分类
        let categories = db::get_all_categories(&local_conn).await?;
        for category in categories {
            db::create_category(&remote_conn, &category.name, category.parent_id.as_deref()).await?;
        }
        
        // 迁移设置
        // 这里需要实现设置的迁移逻辑
        
        Ok(())
    }
    
    /// 获取冲突列表
    pub async fn get_conflicts(&self) -> Result<Vec<ConflictData>> {
        let conn = self.local_db.connect()?;
        
        // 获取状态为冲突的同步记录
        let mut rows = conn.query(
            "SELECT id, table_name, record_id, operation, sync_status, created_at, updated_at, conflict_data, retry_count
             FROM sync_status 
             WHERE sync_status = 'CONFLICT'
             ORDER BY created_at DESC",
            ()
        ).await?;
        
        let mut conflicts = Vec::new();
        while let Some(row) = rows.next().await? {
            if let Some(conflict_data_str) = row.get::<Option<String>>(7)? {
                if let Ok(conflict_data) = serde_json::from_str::<ConflictData>(&conflict_data_str) {
                    conflicts.push(conflict_data);
                }
            }
        }
        
        Ok(conflicts)
    }
    
    /// 公开的冲突解决方法
    pub async fn resolve_conflict_by_user(
        &self,
        record_id: &str,
        table_name: &str,
        strategy: ConflictResolutionStrategy,
    ) -> Result<()> {
        let conn = self.local_db.connect()?;
        
        // 记录冲突解决
        db::record_conflict_resolution(
            &conn,
            table_name,
            record_id,
            strategy,
            "USER",
        ).await?;
        
        // 更新同步状态为已解决
        // 首先找到对应的同步记录
        let mut rows = conn.query(
            "SELECT id FROM sync_status WHERE table_name = ? AND record_id = ? AND sync_status = 'CONFLICT' LIMIT 1",
            libsql::params![table_name, record_id]
        ).await?;
        
        if let Some(row) = rows.next().await? {
            let sync_record_id: String = row.get(0)?;
            db::update_sync_status(
                &conn,
                &sync_record_id,
                SyncStatus::Synced,
                None,
            ).await?;
        }
        
        Ok(())
    }
    
    /// 获取同步历史
    pub async fn get_sync_history(&self, limit: u32) -> Result<Vec<SyncEvent>> {
        let conn = self.local_db.connect()?;
        
        // 从冲突解决记录中构建同步事件历史
        let mut rows = conn.query(
            "SELECT table_name, record_id, resolution_strategy, resolved_at, resolved_by
             FROM conflict_resolutions 
             ORDER BY resolved_at DESC 
             LIMIT ?",
            libsql::params![limit as i64]
        ).await?;
        
        let mut history = Vec::new();
        while let Some(row) = rows.next().await? {
            let table_name: String = row.get(0)?;
            let record_id: String = row.get(1)?;
            let _strategy: String = row.get(2)?;
            let resolved_at: i64 = row.get(3)?;
            let _resolved_by: String = row.get(4)?;
            
            // 创建同步完成事件
            history.push(SyncEvent::SyncCompleted {
                stats: SyncStats {
                    total_records: 1,
                    synced_records: 1,
                    pending_records: 0,
                    failed_records: 0,
                    last_sync_time: resolved_at,
                    is_online: true,
                }
            });
            
            // 也可以添加冲突检测事件
            history.push(SyncEvent::ConflictDetected {
                record_id,
                table_name,
            });
        }
        
        Ok(history)
    }
    
    /// 确保分类在远程数据库中存在
    fn ensure_category_exists<'a>(
        &'a self,
        local_conn: &'a Connection,
        remote_conn: &'a Connection,
        category_id: &'a str,
    ) -> BoxFuture<'a, Result<()>> {
        Box::pin(async move {
        // 检查远程数据库中是否已存在该分类
        let mut rows = remote_conn.query(
            "SELECT EXISTS(SELECT 1 FROM categories WHERE id = ?)",
            params![category_id]
        ).await?;
        
        let exists: bool = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            false
        };
        
        if exists {
            return Ok(());
        }
        
        // 如果不存在，从本地数据库获取分类信息
        let category = db::get_category_by_id(local_conn, category_id).await?;
        
        // 如果有父分类，递归确保父分类存在
        if let Some(ref parent_id) = category.parent_id {
            self.ensure_category_exists(local_conn, remote_conn, parent_id).await?;
        }
        
        // 在远程数据库中创建分类
        remote_conn.execute(
            "INSERT OR REPLACE INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
            params![category.id.clone(), category.name.clone(), category.parent_id.clone()],
        ).await?;
        
        println!("Created category in remote database: {} ({})", category.name, category.id);
        Ok(())
        })
    }
    
    /// 确保标签在远程数据库中存在
    async fn ensure_tag_exists(
        &self,
        remote_conn: &Connection,
        tag: &Tag,
    ) -> Result<()> {
        // 检查远程数据库中是否已存在该标签
        let mut rows = remote_conn.query(
            "SELECT EXISTS(SELECT 1 FROM tags WHERE id = ?)",
            params![tag.id.clone()]
        ).await?;
        
        let exists: bool = if let Some(row) = rows.next().await? {
            row.get(0)?
        } else {
            false
        };
        
        if exists {
            return Ok(());
        }
        
        // 如果不存在，在远程数据库中创建标签
        remote_conn.execute(
            "INSERT OR IGNORE INTO tags (id, name) VALUES (?, ?)",
            params![tag.id.clone(), tag.name.clone()],
        ).await?;
        
        println!("Created tag in remote database: {} ({})", tag.name, tag.id);
        Ok(())
    }

    /// 同步单个记录（隔离版本 - 使用完全独立的连接避免借用冲突）
    async fn sync_record_safe_isolated(&self, record: &SyncStatusRecord) -> Result<()> {
        println!("Starting isolated sync for record: {}", record.record_id);
        
        // 为本地连接创建完全独立的实例
        let local_conn = self.local_db.connect().map_err(|e| {
            anyhow!("Failed to create isolated local connection: {}", e)
        })?;
        
        // 设置极其安全的连接配置
        let _ = local_conn.execute("PRAGMA journal_mode=DELETE", ()).await;
        let _ = local_conn.execute("PRAGMA synchronous=FULL", ()).await;
        let _ = local_conn.execute("PRAGMA busy_timeout=30000", ()).await;
        let _ = local_conn.execute("PRAGMA temp_store=memory", ()).await;
        let _ = local_conn.execute("PRAGMA cache_size=1000", ()).await; // 减少缓存避免冲突
        
        // 获取远程连接（通过连接池）
        let remote_handle = {
            let pool_manager = self.connection_pool_manager.lock().await;
            pool_manager.get_connection().await
                .map_err(|e| anyhow!("Failed to acquire remote connection from pool: {}", e))?
        };
        
        let remote_conn = remote_handle.connection();
        
        // 同步操作
        let result = match record.operation {
            SyncOperation::Insert => {
                self.sync_insert_record(&local_conn, remote_conn, record).await
            }
            SyncOperation::Update => {
                self.sync_update_record(&local_conn, remote_conn, record).await
            }
            SyncOperation::Delete => {
                self.sync_delete_record(&local_conn, remote_conn, record).await
            }
        };
        
        // 显式释放连接（本地连接在函数结束时自动drop，远程连接通过handle归还）
        drop(local_conn);
        // remote_handle会在函数结束时自动归还连接到池中
        
        println!("Completed isolated sync for record: {} with result: {:?}", record.record_id, result.is_ok());
        result
    }

    /// 紧急连接恢复（处理借用冲突和WAL错误）
    async fn emergency_connection_recovery(&self) -> Result<()> {
        println!("Starting emergency connection recovery...");
        
        // 1. 重置所有连接池
        {
            let mut pool_manager = self.connection_pool_manager.lock().await;
            println!("Resetting connection pool...");
            pool_manager.reset_all_pools().await?;
            
            // 重新配置连接池为最保守设置
            let emergency_config = ConnectionPoolConfig {
                max_connections: 1,     // 最小连接数
                idle_timeout: 30,       // 极短空闲超时
                max_lifetime: 120,      // 极短生存时间
                acquire_timeout: 10,    // 短获取超时
            };
            
            // 如果有远程配置，重新初始化连接池
            let sync_config = self.sync_config.read().await;
            if sync_config.sync_mode == SyncMode::Auto {
                if let Some(ref remote_url) = sync_config.remote_url {
                    let db_config = DatabaseConfig::Remote {
                        replica_path: "/tmp/mytips_emergency.db".to_string(),
                        remote_url: remote_url.clone(),
                        auth_token: sync_config.auth_token.clone(),
                    };
                    
                    println!("Reinitializing remote connection pool with emergency config...");
                    if let Err(e) = pool_manager.create_pool("emergency_remote".to_string(), db_config, emergency_config).await {
                        eprintln!("Failed to create emergency remote pool: {}", e);
                    }
                }
            }
        }
        
        // 2. 等待所有连接完全释放
        println!("Waiting for connections to stabilize...");
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // 3. 强制垃圾回收（如果可能）
        // Rust不提供显式GC，但我们可以确保所有Arc引用被正确释放
        
        // 4. 验证本地数据库连接
        println!("Validating local database connection...");
        let test_conn = self.local_db.connect().map_err(|e| {
            anyhow!("Failed to create test connection after recovery: {}", e)
        })?;
        
        // 设置最安全的连接配置
        test_conn.execute("PRAGMA journal_mode=DELETE", ()).await
            .map_err(|e| anyhow!("Failed to set journal mode during recovery: {}", e))?;
        test_conn.execute("PRAGMA synchronous=FULL", ()).await
            .map_err(|e| anyhow!("Failed to set synchronous mode during recovery: {}", e))?;
        test_conn.execute("PRAGMA busy_timeout=30000", ()).await
            .map_err(|e| anyhow!("Failed to set busy timeout during recovery: {}", e))?;
        
        // 测试基本操作
        let mut rows = test_conn.query("SELECT COUNT(*) FROM sync_status", ()).await
            .map_err(|e| anyhow!("Failed to test query during recovery: {}", e))?;
        
        if rows.next().await?.is_some() {
            println!("Local database connection validated successfully");
        } else {
            return Err(anyhow!("Local database validation failed"));
        }
        
        drop(test_conn); // 释放测试连接
        
        println!("Emergency connection recovery completed successfully");
        Ok(())
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
        }
    }
}

// 冲突数据结构
#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictData {
    pub local_version: DataVersion,
    pub remote_version: DataVersion,
    pub local_content: String,
    pub remote_content: String,
}

// 冲突解决结果
#[derive(Debug)]
pub struct ConflictResolutionResult {
    pub strategy: ConflictResolutionStrategy,
    pub resolved_by: String,
}

impl ConflictResolver {
    /// 解决冲突
    pub async fn resolve_conflict(
        &self,
        _record: &SyncStatusRecord,
        _conflict: &ConflictData,
    ) -> Result<ConflictResolutionResult> {
        // 使用默认策略
        Ok(ConflictResolutionResult {
            strategy: self.default_strategy.clone(),
            resolved_by: "AUTO".to_string(),
        })
    }
    
    /// 设置默认冲突解决策略
    pub fn set_default_strategy(&mut self, strategy: ConflictResolutionStrategy) {
        self.default_strategy = strategy;
    }
}

// 辅助函数：为记录添加同步标记
pub async fn mark_for_sync(
    conn: &Connection,
    table_name: &str,
    record_id: &str,
    operation: SyncOperation,
) -> Result<()> {
    db::add_sync_record(conn, table_name, record_id, operation).await
} 