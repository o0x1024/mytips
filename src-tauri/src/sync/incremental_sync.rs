use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{info, warn, debug, error};
use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, RwLock};
use libsql::{params, Connection, Database};
use blake3;
use sha2::{Sha256, Digest};
// use crate::db::Database; // 使用 libsql::Database 替代
use super::monitoring::{PerformanceMonitor, StructuredLogger};

/// 增量同步管理器
pub struct IncrementalSyncManager {
    /// 本地数据库连接
    local_db: Arc<Database>,
    /// 远程数据库连接
    remote_db: Arc<RwLock<Option<Database>>>,
    /// 增量同步配置
    config: Arc<RwLock<IncrementalSyncConfig>>,
    /// 最后同步时间戳缓存
    last_sync_timestamps: Arc<RwLock<HashMap<String, i64>>>,
    /// 变更检测器
    change_detector: Arc<ChangeDetector>,
    /// 性能监控器
    performance_monitor: Arc<PerformanceMonitor>,
    /// 结构化日志记录器
    structured_logger: Arc<StructuredLogger>,
}

/// 增量同步配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalSyncConfig {
    /// 是否启用增量同步
    pub enabled: bool,
    /// 最大同步批次大小
    pub max_batch_size: usize,
    /// 最小同步间隔（毫秒）
    pub min_sync_interval_ms: u64,
    /// 强制全量同步间隔（小时）
    pub force_full_sync_hours: u64,
    /// 是否压缩传输数据
    pub compress_data: bool,
    /// 是否启用变更检测优化
    pub change_detection_enabled: bool,
    /// 最大历史版本保留数
    pub max_version_history: u32,
}

impl Default for IncrementalSyncConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            max_batch_size: 1000,
            min_sync_interval_ms: 5000,       // 5秒
            force_full_sync_hours: 24,        // 24小时强制全量同步
            compress_data: true,
            change_detection_enabled: true,
            max_version_history: 10,
        }
    }
}

/// 增量同步统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncrementalSyncStats {
    /// 检查的记录数
    pub checked_records: u64,
    /// 变更的记录数
    pub changed_records: u64,
    /// 同步的记录数
    pub synced_records: u64,
    /// 跳过的记录数（无变更）
    pub skipped_records: u64,
    /// 节省的传输时间（毫秒）
    pub time_saved_ms: u64,
    /// 节省的数据量（字节）
    pub data_saved_bytes: u64,
    /// 检测耗时（毫秒）
    pub detection_duration_ms: u64,
    /// 同步耗时（毫秒）
    pub sync_duration_ms: u64,
}

impl Default for IncrementalSyncStats {
    fn default() -> Self {
        Self {
            checked_records: 0,
            changed_records: 0,
            synced_records: 0,
            skipped_records: 0,
            time_saved_ms: 0,
            data_saved_bytes: 0,
            detection_duration_ms: 0,
            sync_duration_ms: 0,
        }
    }
}

/// 变更检测器
#[derive(Debug)]
pub struct ChangeDetector {
    /// 本地数据库连接
    local_db: Arc<Database>,
    /// 变更检测算法配置
    detection_config: ChangeDetectionConfig,
}

#[derive(Debug, Clone)]
pub struct ChangeDetectionConfig {
    /// 使用哈希值检测变更
    pub use_hash_detection: bool,
    /// 使用时间戳检测变更
    pub use_timestamp_detection: bool,
    /// 哈希算法类型
    pub hash_algorithm: HashAlgorithm,
    /// 时间戳精度（毫秒）
    pub timestamp_precision_ms: u64,
}

#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Blake3,
    Sha256,
    FastHash,
}

impl Default for ChangeDetectionConfig {
    fn default() -> Self {
        Self {
            use_hash_detection: true,
            use_timestamp_detection: true,
            hash_algorithm: HashAlgorithm::Blake3,
            timestamp_precision_ms: 1000, // 1秒精度
        }
    }
}

/// 变更检测结果
#[derive(Debug, Clone)]
pub struct ChangeDetectionResult {
    /// 表名
    pub table_name: String,
    /// 变更的记录列表
    pub changed_records: Vec<ChangedRecord>,
    /// 检测开始时间
    pub detection_start_time: i64,
    /// 检测结束时间
    pub detection_end_time: i64,
    /// 总检查记录数
    pub total_checked: u64,
    /// 变更记录数
    pub total_changed: u64,
}

/// 记录数据枚举（用于安全传输）
#[derive(Debug, Clone)]
pub enum RecordData {
    Tip {
        id: String,
        title: String,
        content: String,
        tip_type: String,
        language: Option<String>,
        category_id: Option<String>,
        created_at: i64,
        updated_at: i64,
    },
    Category {
        id: String,
        name: String,
        parent_id: Option<String>,
        created_at: i64,
        updated_at: i64,
    },
    Tag {
        id: String,
        name: String,
        created_at: i64,
        updated_at: i64,
    },
}

#[derive(Debug, Clone)]
pub struct ChangedRecord {
    /// 记录ID
    pub record_id: String,
    /// 变更类型
    pub change_type: ChangeType,
    /// 本地时间戳
    pub local_timestamp: i64,
    /// 远程时间戳（如果存在）
    pub remote_timestamp: Option<i64>,
    /// 本地哈希值
    pub local_hash: Option<String>,
    /// 远程哈希值（如果存在）
    pub remote_hash: Option<String>,
    /// 预估数据大小（字节）
    pub estimated_size: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    /// 新增记录
    Insert,
    /// 更新记录
    Update,
    /// 删除记录
    Delete,
    /// 冲突（本地和远程都有变更）
    Conflict,
}

impl IncrementalSyncManager {
    /// 创建新的增量同步管理器
    pub async fn new(
        local_db: Arc<Database>,
        remote_db: Arc<RwLock<Option<Database>>>,
        performance_monitor: Arc<PerformanceMonitor>,
        structured_logger: Arc<StructuredLogger>,
    ) -> Result<Self> {
        let config = Arc::new(RwLock::new(IncrementalSyncConfig::default()));
        let last_sync_timestamps = Arc::new(RwLock::new(HashMap::new()));
        
        let change_detector = Arc::new(ChangeDetector::new(
            local_db.clone(),
            ChangeDetectionConfig::default(),
        ));

        let manager = Self {
            local_db,
            remote_db,
            config,
            last_sync_timestamps,
            change_detector,
            performance_monitor,
            structured_logger,
        };

        // 初始化同步时间戳
        manager.load_sync_timestamps().await?;

        Ok(manager)
    }

    /// 执行增量同步
    pub async fn sync_incremental(&self) -> Result<IncrementalSyncStats> {
        let sync_start = Instant::now();
        info!("Starting incremental sync operation");

        let config = self.config.read().await;
        if !config.enabled {
            info!("Incremental sync is disabled");
            return Ok(IncrementalSyncStats::default());
        }
        drop(config);

        // 记录操作开始
        self.structured_logger.log_sync_operation(
            "incremental_sync_start",
            0,
            0,
            0,
            0,
        );

        let mut total_stats = IncrementalSyncStats::default();

        // 检查是否需要强制全量同步
        if self.should_force_full_sync().await? {
            info!("Forcing full sync due to time interval");
            return self.perform_full_sync().await;
        }

        // 逐表执行增量同步
        let tables = vec!["categories", "tags", "tips", "tip_tags"];
        
        for table_name in tables {
            info!("Processing incremental sync for table: {}", table_name);
            
            match self.sync_table_incremental(table_name).await {
                Ok(table_stats) => {
                    total_stats.checked_records += table_stats.checked_records;
                    total_stats.changed_records += table_stats.changed_records;
                    total_stats.synced_records += table_stats.synced_records;
                    total_stats.skipped_records += table_stats.skipped_records;
                    total_stats.time_saved_ms += table_stats.time_saved_ms;
                    total_stats.data_saved_bytes += table_stats.data_saved_bytes;
                    total_stats.detection_duration_ms += table_stats.detection_duration_ms;
                    
                    info!("Table {} sync completed: {} changed, {} synced, {} skipped", 
                          table_name, table_stats.changed_records, 
                          table_stats.synced_records, table_stats.skipped_records);
                }
                Err(e) => {
                    error!("Failed to sync table {}: {}", table_name, e);
                    // 继续处理其他表
                }
            }
        }

        let sync_duration = sync_start.elapsed();
        total_stats.sync_duration_ms = sync_duration.as_millis() as u64;

        // 更新同步时间戳
        self.update_sync_timestamps().await?;

        // 记录操作完成
        self.structured_logger.log_sync_operation(
            "incremental_sync_complete",
            total_stats.synced_records,
            total_stats.synced_records,
            0,
            total_stats.sync_duration_ms,
        );

        info!("Incremental sync completed: {} total checked, {} changed, {} synced, {} skipped",
              total_stats.checked_records, total_stats.changed_records,
              total_stats.synced_records, total_stats.skipped_records);

        Ok(total_stats)
    }

    /// 同步单个表的增量数据
    async fn sync_table_incremental(&self, table_name: &str) -> Result<IncrementalSyncStats> {
        let detection_start = Instant::now();
        
        // 获取表的变更检测结果
        let detection_result = self.change_detector
            .detect_changes(table_name).await?;
        
        let detection_duration = detection_start.elapsed();
        
        let mut stats = IncrementalSyncStats {
            checked_records: detection_result.total_checked,
            changed_records: detection_result.total_changed,
            detection_duration_ms: detection_duration.as_millis() as u64,
            ..Default::default()
        };

        if detection_result.changed_records.is_empty() {
            info!("No changes detected for table: {}", table_name);
            stats.skipped_records = stats.checked_records;
            return Ok(stats);
        }

        info!("Detected {} changes in table {}", detection_result.changed_records.len(), table_name);

        // 批量处理变更记录
        let config = self.config.read().await;
        let batch_size = config.max_batch_size;
        drop(config);

        let sync_start = Instant::now();
        
        for batch in detection_result.changed_records.chunks(batch_size) {
            match self.sync_changed_records_batch(table_name, batch).await {
                Ok(batch_synced) => {
                    stats.synced_records += batch_synced;
                }
                Err(e) => {
                    error!("Failed to sync batch for table {}: {}", table_name, e);
                    // 继续处理下一批次
                }
            }
        }

        let sync_duration = sync_start.elapsed();
        stats.sync_duration_ms = sync_duration.as_millis() as u64;
        
        // 计算节省的资源
        stats.skipped_records = stats.checked_records - stats.changed_records;
        stats.time_saved_ms = (stats.skipped_records * 50) as u64; // 假设每个跳过的记录节省50ms
        stats.data_saved_bytes = stats.skipped_records * 1024; // 假设每个跳过的记录节省1KB

        Ok(stats)
    }

    /// 批量同步变更记录（避免WAL冲突的安全版本）
    async fn sync_changed_records_batch(
        &self, 
        table_name: &str, 
        changed_records: &[ChangedRecord]
    ) -> Result<u64> {
        // 使用排队机制，避免并发访问
        static SYNC_MUTEX: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
        let _lock = SYNC_MUTEX.lock().await;
        
        info!("Starting batch sync for {} records in table {}", changed_records.len(), table_name);
        
        let mut synced_count = 0;
        let mut successful_records = Vec::new();

        // 逐条同步，避免并发问题
        for record in changed_records {
            match self.sync_single_record_isolated(table_name, record).await {
                Ok(_) => {
                    synced_count += 1;
                    successful_records.push(record.record_id.clone());
                    info!("Successfully synced record: {} in table {}", record.record_id, table_name);
                }
                Err(e) => {
                    warn!("Failed to sync record {} in table {}: {}", 
                          record.record_id, table_name, e);
                    // 继续处理其他记录
                }
            }
        }

        // 批量更新同步状态
        if synced_count > 0 {
            match self.update_sync_status_batch(&successful_records, table_name).await {
                Ok(_) => info!("Updated sync status for {} records", successful_records.len()),
                Err(e) => warn!("Failed to update sync status: {}", e),
            }
        }
        
        info!("Batch sync completed: {} of {} records synced in table {}", 
              synced_count, changed_records.len(), table_name);
        Ok(synced_count)
    }

    /// 完全隔离的单记录同步（彻底避免WAL冲突）
    async fn sync_single_record_isolated(
        &self,
        table_name: &str,
        changed_record: &ChangedRecord,
    ) -> Result<()> {
        // 采用序列化同步策略，确保连接完全隔离
        match changed_record.change_type {
            ChangeType::Insert | ChangeType::Update => {
                self.sync_record_to_remote_isolated(table_name, &changed_record.record_id).await
            }
            ChangeType::Delete => {
                self.delete_record_from_remote_isolated(table_name, &changed_record.record_id).await
            }
            ChangeType::Conflict => {
                // 冲突时优先本地数据
                self.sync_record_to_remote_isolated(table_name, &changed_record.record_id).await
            }
        }
    }

    /// 将单条记录同步到远程（完全隔离）
    async fn sync_record_to_remote_isolated(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        // 第一步：从本地读取数据
        let record_data = {
            let local_conn = self.local_db.connect()?;
            self.read_record_data(&local_conn, table_name, record_id).await?
        };
        
        // 短暂延迟，确保连接完全释放
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        
        // 第二步：写入远程数据库
        self.write_record_to_remote(table_name, record_data).await?;
        
        Ok(())
    }

    /// 从远程删除记录（完全隔离）
    async fn delete_record_from_remote_isolated(
        &self,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        let guard = self.remote_db.read().await;
        let remote_db = guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not connected"))?
            .clone();
        
        let remote_conn = remote_db.connect()?;
        remote_conn.execute(
            &format!("DELETE FROM {} WHERE id = ?", table_name),
            params![record_id]
        ).await?;
        
        info!("Deleted record {} from remote table {}", record_id, table_name);
        Ok(())
    }

    /// 安全的单记录同步（避免并发问题）
    async fn sync_single_record_safe(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        changed_record: &ChangedRecord,
    ) -> Result<()> {
        // 使用 spawn_blocking 来避免异步上下文中的数据库操作问题
        let table_name = table_name.to_string();
        let record_id = changed_record.record_id.clone();
        let change_type = changed_record.change_type.clone();
        
        match change_type {
            ChangeType::Insert | ChangeType::Update => {
                // 插入和更新使用相同逻辑
                self.sync_insert_record_safe(local_conn, remote_conn, &table_name, &record_id).await
            }
            ChangeType::Delete => {
                self.sync_delete_record_safe(remote_conn, &table_name, &record_id).await
            }
            ChangeType::Conflict => {
                // 对于冲突，默认使用本地优先策略
                self.sync_insert_record_safe(local_conn, remote_conn, &table_name, &record_id).await
            }
        }
    }

    /// 同步单个变更记录
    async fn sync_single_changed_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        changed_record: &ChangedRecord,
    ) -> Result<()> {
        match changed_record.change_type {
            ChangeType::Insert => {
                self.sync_insert_record(local_conn, remote_conn, table_name, &changed_record.record_id).await
            }
            ChangeType::Update => {
                self.sync_update_record(local_conn, remote_conn, table_name, &changed_record.record_id).await
            }
            ChangeType::Delete => {
                self.sync_delete_record(remote_conn, table_name, &changed_record.record_id).await
            }
            ChangeType::Conflict => {
                self.resolve_conflict_record(local_conn, remote_conn, table_name, changed_record).await
            }
        }
    }

    /// 安全的插入记录同步
    async fn sync_insert_record_safe(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        match table_name {
            "tips" => {
                if let Some(tip) = crate::db::operations::get_tip_by_id(local_conn, record_id).await? {
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                        params![
                            tip.id, tip.title, tip.content, 
                            String::from(tip.tip_type), tip.language, tip.category_id,
                            tip.created_at, tip.updated_at
                        ]
                    ).await?;
                    info!("Synced tip to remote: {}", record_id);
                }
            }
            "categories" => {
                if let Some(category) = crate::db::operations::get_category_by_id(local_conn, record_id).await? {
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO categories (id, name, parent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                        params![category.id, category.name, category.parent_id, category.created_at, category.updated_at]
                    ).await?;
                    info!("Synced category to remote: {}", record_id);
                }
            }
            "tags" => {
                let mut rows = local_conn.query(
                    "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    let tag_id: String = row.get(0)?;
                    let tag_name: String = row.get(1)?;
                    let created_at: i64 = row.get(2)?;
                    let updated_at: i64 = row.get(3)?;
                    
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO tags (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
                        params![tag_id, tag_name, created_at, updated_at]
                    ).await?;
                    info!("Synced tag to remote: {}", record_id);
                }
            }
            _ => {
                return Err(anyhow!("Unsupported table for incremental sync: {}", table_name));
            }
        }
        
        Ok(())
    }

    /// 读取记录数据
    async fn read_record_data(
        &self,
        local_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<RecordData> {
        match table_name {
            "tips" => {
                if let Some(tip) = crate::db::operations::get_tip_by_id(local_conn, record_id).await? {
                    Ok(RecordData::Tip {
                        id: tip.id,
                        title: tip.title,
                        content: tip.content,
                        tip_type: String::from(tip.tip_type),
                        language: tip.language,
                        category_id: tip.category_id,
                        created_at: tip.created_at,
                        updated_at: tip.updated_at,
                    })
                } else {
                    Err(anyhow!("Tip not found: {}", record_id))
                }
            }
            "categories" => {
                if let Some(category) = crate::db::operations::get_category_by_id(local_conn, record_id).await? {
                    Ok(RecordData::Category {
                        id: category.id,
                        name: category.name,
                        parent_id: category.parent_id,
                        created_at: category.created_at,
                        updated_at: category.updated_at,
                    })
                } else {
                    Err(anyhow!("Category not found: {}", record_id))
                }
            }
            "tags" => {
                let mut rows = local_conn.query(
                    "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    Ok(RecordData::Tag {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        created_at: row.get(2)?,
                        updated_at: row.get(3)?,
                    })
                } else {
                    Err(anyhow!("Tag not found: {}", record_id))
                }
            }
            _ => Err(anyhow!("Unsupported table: {}", table_name)),
        }
    }

    /// 写入记录到远程
    async fn write_record_to_remote(
        &self,
        table_name: &str,
        record_data: RecordData,
    ) -> Result<()> {
        let guard = self.remote_db.read().await;
        let remote_db = guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not connected"))?
            .clone();
        
        let remote_conn = remote_db.connect()?;
        
        match record_data {
            RecordData::Tip { id, title, content, tip_type, language, category_id, created_at, updated_at } => {
                remote_conn.execute(
                    "INSERT OR REPLACE INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at) 
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                    params![id, title, content, tip_type, language, category_id, created_at, updated_at]
                ).await?;
            }
            RecordData::Category { id, name, parent_id, created_at, updated_at } => {
                remote_conn.execute(
                    "INSERT OR REPLACE INTO categories (id, name, parent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                    params![id, name, parent_id, created_at, updated_at]
                ).await?;
            }
            RecordData::Tag { id, name, created_at, updated_at } => {
                remote_conn.execute(
                    "INSERT OR REPLACE INTO tags (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
                    params![id, name, created_at, updated_at]
                ).await?;
            }
        }
        
        info!("Successfully wrote record to remote table {}", table_name);
        Ok(())
    }

    /// 批量更新同步状态
    async fn update_sync_status_batch(
        &self,
        record_ids: &[String],
        table_name: &str,
    ) -> Result<()> {
        if record_ids.is_empty() {
            return Ok(());
        }
        
        let local_conn = self.local_db.connect()?;
        let now = chrono::Utc::now().timestamp_millis();
        
        // 批量更新
        for record_id in record_ids {
            local_conn.execute(
                "UPDATE sync_status SET sync_status = 'SYNCED', updated_at = ? 
                 WHERE table_name = ? AND record_id = ? AND sync_status = 'PENDING'",
                params![now, table_name, record_id.clone()]
            ).await?;
        }
        
        Ok(())
    }

    /// 安全的删除记录同步  
    async fn sync_delete_record_safe(
        &self,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        remote_conn.execute(
            &format!("DELETE FROM {} WHERE id = ?", table_name),
            params![record_id]
        ).await?;
        info!("Deleted record from remote: {} in table {}", record_id, table_name);
        Ok(())
    }

    /// 同步插入记录
    async fn sync_insert_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        match table_name {
            "tips" => {
                if let Some(tip) = crate::db::operations::get_tip_by_id(local_conn, record_id).await? {
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at, version, last_synced_at, sync_hash, is_encrypted, encryption_key_id, encrypted_content) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
                        params![
                            tip.id, tip.title, tip.content, 
                            String::from(tip.tip_type), tip.language, tip.category_id,
                            tip.created_at, tip.updated_at, tip.version.unwrap_or(1), 
                            tip.last_synced_at.unwrap_or(0), tip.sync_hash,
                            tip.is_encrypted.unwrap_or(false), tip.encryption_key_id, tip.encrypted_content
                        ]
                    ).await?;
                    info!("Synced tip to remote: {}", record_id);
                }
            }
            "categories" => {
                if let Some(category) = crate::db::operations::get_category_by_id(local_conn, record_id).await? {
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO categories (id, name, parent_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
                        params![category.id, category.name, category.parent_id, category.created_at, category.updated_at]
                    ).await?;
                    info!("Synced category to remote: {}", record_id);
                }
            }
            "tags" => {
                // 直接从数据库查询tag信息
                let mut rows = local_conn.query(
                    "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    let tag_id: String = row.get(0)?;
                    let tag_name: String = row.get(1)?;
                    let created_at: i64 = row.get(2)?;
                    let updated_at: i64 = row.get(3)?;
                    
                    remote_conn.execute(
                        "INSERT OR REPLACE INTO tags (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
                        params![tag_id, tag_name, created_at, updated_at]
                    ).await?;
                    info!("Synced tag to remote: {}", record_id);
                }
            }
            _ => {
                return Err(anyhow!("Unsupported table for incremental sync: {}", table_name));
            }
        }
        
        Ok(())
    }

    /// 同步更新记录
    async fn sync_update_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        // 更新记录和插入记录的逻辑相同，使用 INSERT OR REPLACE
        self.sync_insert_record(local_conn, remote_conn, table_name, record_id).await
    }

    /// 同步删除记录
    async fn sync_delete_record(
        &self,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        remote_conn.execute(
            &format!("DELETE FROM {} WHERE id = ?", table_name),
            params![record_id]
        ).await?;
        
        Ok(())
    }

    /// 解决冲突记录
    async fn resolve_conflict_record(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        changed_record: &ChangedRecord,
    ) -> Result<()> {
        // 使用时间戳比较解决冲突，newer wins
        match (changed_record.local_timestamp, changed_record.remote_timestamp) {
            (local_ts, Some(remote_ts)) => {
                if local_ts > remote_ts {
                    // 本地较新，同步到远程
                    self.sync_insert_record(local_conn, remote_conn, table_name, &changed_record.record_id).await
                } else {
                    // 远程较新，从远程同步到本地
                    self.sync_from_remote_single(local_conn, remote_conn, table_name, &changed_record.record_id).await
                }
            }
            _ => {
                // 无法确定时间戳，默认本地优先
                self.sync_insert_record(local_conn, remote_conn, table_name, &changed_record.record_id).await
            }
        }
    }

    /// 从远程同步单个记录到本地
    async fn sync_from_remote_single(
        &self,
        local_conn: &Connection,
        remote_conn: &Connection,
        table_name: &str,
        record_id: &str,
    ) -> Result<()> {
        match table_name {
            "tips" => {
                let mut rows = remote_conn.query(
                    "SELECT id, title, content, tip_type, language, category_id, created_at, updated_at 
                     FROM tips WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    local_conn.execute(
                        "INSERT OR REPLACE INTO tips (id, title, content, tip_type, language, category_id, created_at, updated_at) 
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                        params![
                            row.get::<String>(0)?, row.get::<String>(1)?, row.get::<String>(2)?,
                            row.get::<String>(3)?, row.get::<Option<String>>(4)?, row.get::<Option<String>>(5)?,
                            row.get::<i64>(6)?, row.get::<i64>(7)?
                        ]
                    ).await?;
                }
            }
            "categories" => {
                let mut rows = remote_conn.query(
                    "SELECT id, name, parent_id FROM categories WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    local_conn.execute(
                        "INSERT OR REPLACE INTO categories (id, name, parent_id) VALUES (?, ?, ?)",
                        params![
                            row.get::<String>(0)?, row.get::<String>(1)?, row.get::<Option<String>>(2)?
                        ]
                    ).await?;
                }
            }
            "tags" => {
                let mut rows = remote_conn.query(
                    "SELECT id, name FROM tags WHERE id = ?",
                    params![record_id]
                ).await?;
                
                if let Some(row) = rows.next().await? {
                    local_conn.execute(
                        "INSERT OR REPLACE INTO tags (id, name) VALUES (?, ?)",
                        params![row.get::<String>(0)?, row.get::<String>(1)?]
                    ).await?;
                }
            }
            _ => {
                return Err(anyhow!("Unsupported table for remote sync: {}", table_name));
            }
        }
        
        Ok(())
    }

    /// 检查是否需要强制全量同步
    async fn should_force_full_sync(&self) -> Result<bool> {
        let config = self.config.read().await;
        let force_interval_ms = config.force_full_sync_hours * 3600 * 1000;
        drop(config);

        let timestamps = self.last_sync_timestamps.read().await;
        let last_full_sync = timestamps.get("__full_sync__").copied().unwrap_or(0);
        drop(timestamps);

        let now = Utc::now().timestamp_millis();
        Ok(now - last_full_sync > force_interval_ms as i64)
    }

    /// 执行全量同步
    async fn perform_full_sync(&self) -> Result<IncrementalSyncStats> {
        info!("Performing forced full sync");
        
        let sync_start = Instant::now();
        
        // 记录全量同步时间戳
        {
            let mut timestamps = self.last_sync_timestamps.write().await;
            timestamps.insert("__full_sync__".to_string(), Utc::now().timestamp_millis());
        }

        // 这里可以调用标准的全量同步逻辑
        // 为了示例，我们返回一个基础的统计
        let sync_duration = sync_start.elapsed();
        
        Ok(IncrementalSyncStats {
            checked_records: 0,
            changed_records: 0,
            synced_records: 0,
            skipped_records: 0,
            time_saved_ms: 0,
            data_saved_bytes: 0,
            detection_duration_ms: 0,
            sync_duration_ms: sync_duration.as_millis() as u64,
        })
    }

    /// 加载同步时间戳
    async fn load_sync_timestamps(&self) -> Result<()> {
        let conn = self.local_db.connect()?;
        
        // 从设置表加载同步时间戳
        let mut rows = conn.query(
            "SELECT key, value FROM app_settings WHERE key LIKE 'incremental_sync_timestamp_%'",
            ()
        ).await?;

        let mut timestamps = self.last_sync_timestamps.write().await;
        
        while let Some(row) = rows.next().await? {
            let key: String = row.get(0)?;
            let value_str: String = row.get(1)?;
            
            if let Ok(timestamp) = value_str.parse::<i64>() {
                let table_name = key.strip_prefix("incremental_sync_timestamp_")
                    .unwrap_or(&key);
                timestamps.insert(table_name.to_string(), timestamp);
            }
        }

        debug!("Loaded {} sync timestamps", timestamps.len());
        Ok(())
    }

    /// 更新同步时间戳
    async fn update_sync_timestamps(&self) -> Result<()> {
        let conn = self.local_db.connect()?;
        let now = Utc::now().timestamp_millis();
        
        let mut timestamps = self.last_sync_timestamps.write().await;
        
        // 更新所有表的同步时间戳
        for table_name in &["tips", "categories", "tags", "tip_tags"] {
            timestamps.insert(table_name.to_string(), now);
            
            // 保存到数据库
            let setting_key = format!("incremental_sync_timestamp_{}", table_name);
            conn.execute(
                "INSERT OR REPLACE INTO app_settings (key, value) VALUES (?, ?)",
                params![setting_key, now.to_string()]
            ).await?;
        }
        
        info!("Updated sync timestamps for all tables");
        Ok(())
    }

    /// 获取增量同步配置
    pub async fn get_config(&self) -> IncrementalSyncConfig {
        self.config.read().await.clone()
    }

    /// 更新增量同步配置
    pub async fn update_config(&self, new_config: IncrementalSyncConfig) -> Result<()> {
        *self.config.write().await = new_config;
        info!("Incremental sync configuration updated");
        Ok(())
    }
}

impl ChangeDetector {
    pub fn new(local_db: Arc<Database>, config: ChangeDetectionConfig) -> Self {
        Self {
            local_db,
            detection_config: config,
        }
    }

    pub async fn detect_changes(&self, table_name: &str) -> Result<ChangeDetectionResult> {
        let start_time = Utc::now().timestamp_millis();
        
        let conn = self.local_db.connect()?;
        
        let changed_records = match table_name {
            "tips" => self.detect_tips_changes(&conn).await?,
            "categories" => self.detect_categories_changes(&conn).await?,
            "tags" => self.detect_tags_changes(&conn).await?,
            "tip_tags" => self.detect_tip_tags_changes(&conn).await?,
            _ => {
                return Err(anyhow!("Unsupported table for change detection: {}", table_name));
            }
        };

        let end_time = Utc::now().timestamp_millis();
        let total_checked = self.get_table_record_count(&conn, table_name).await?;
        let total_changed = changed_records.len() as u64;

        Ok(ChangeDetectionResult {
            table_name: table_name.to_string(),
            changed_records,
            detection_start_time: start_time,
            detection_end_time: end_time,
            total_checked,
            total_changed,
        })
    }

    async fn detect_tips_changes(&self, conn: &Connection) -> Result<Vec<ChangedRecord>> {
        let mut changed_records = Vec::new();

        // 查找所有有同步记录且状态为PENDING的tips
        let mut rows = conn.query(
            "SELECT DISTINCT t.id, t.updated_at, ss.operation 
             FROM tips t 
             INNER JOIN sync_status ss ON t.id = ss.record_id 
             WHERE ss.table_name = 'tips' AND ss.sync_status = 'PENDING'",
            ()
        ).await?;

        while let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let updated_at: i64 = row.get(1)?;
            let operation: String = row.get(2)?;

            let change_type = match operation.as_str() {
                "INSERT" => ChangeType::Insert,
                "UPDATE" => ChangeType::Update,
                "DELETE" => ChangeType::Delete,
                _ => ChangeType::Update,
            };

            changed_records.push(ChangedRecord {
                record_id: id.clone(),
                change_type,
                local_timestamp: updated_at,
                remote_timestamp: None,
                local_hash: None,
                remote_hash: None,
                estimated_size: 1024, // 假设每个tip约1KB
            });
        }

        info!("Detected {} pending tips for sync", changed_records.len());
        Ok(changed_records)
    }

    async fn detect_categories_changes(&self, conn: &Connection) -> Result<Vec<ChangedRecord>> {
        let mut changed_records = Vec::new();

        // 查找所有有同步记录且状态为PENDING的categories
        let mut rows = conn.query(
            "SELECT DISTINCT c.id, c.updated_at, ss.operation 
             FROM categories c 
             INNER JOIN sync_status ss ON c.id = ss.record_id 
             WHERE ss.table_name = 'categories' AND ss.sync_status = 'PENDING'",
            ()
        ).await?;

        while let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let updated_at: i64 = row.get(1)?;
            let operation: String = row.get(2)?;

            let change_type = match operation.as_str() {
                "INSERT" => ChangeType::Insert,
                "UPDATE" => ChangeType::Update,
                "DELETE" => ChangeType::Delete,
                _ => ChangeType::Update,
            };

            changed_records.push(ChangedRecord {
                record_id: id.clone(),
                change_type,
                local_timestamp: updated_at,
                remote_timestamp: None,
                local_hash: None,
                remote_hash: None,
                estimated_size: 256, // 假设每个category约256字节
            });
        }

        info!("Detected {} pending categories for sync", changed_records.len());
        Ok(changed_records)
    }

    async fn detect_tags_changes(&self, conn: &Connection) -> Result<Vec<ChangedRecord>> {
        let mut changed_records = Vec::new();

        // 查找所有有同步记录且状态为PENDING的tags
        let mut rows = conn.query(
            "SELECT DISTINCT t.id, t.updated_at, ss.operation 
             FROM tags t 
             INNER JOIN sync_status ss ON t.id = ss.record_id 
             WHERE ss.table_name = 'tags' AND ss.sync_status = 'PENDING'",
            ()
        ).await?;

        while let Some(row) = rows.next().await? {
            let id: String = row.get(0)?;
            let updated_at: i64 = row.get(1)?;
            let operation: String = row.get(2)?;

            let change_type = match operation.as_str() {
                "INSERT" => ChangeType::Insert,
                "UPDATE" => ChangeType::Update,
                "DELETE" => ChangeType::Delete,
                _ => ChangeType::Update,
            };

            changed_records.push(ChangedRecord {
                record_id: id.clone(),
                change_type,
                local_timestamp: updated_at,
                remote_timestamp: None,
                local_hash: None,
                remote_hash: None,
                estimated_size: 128, // 假设每个tag约128字节
            });
        }

        info!("Detected {} pending tags for sync", changed_records.len());
        Ok(changed_records)
    }

    async fn detect_tip_tags_changes(&self, _conn: &Connection) -> Result<Vec<ChangedRecord>> {
        // tip_tags是关联表，暂时跳过变更检测
        Ok(Vec::new())
    }

    async fn get_table_record_count(&self, conn: &Connection, table_name: &str) -> Result<u64> {
        let sql = format!("SELECT COUNT(*) FROM {}", table_name);
        let mut rows = conn.query(&sql, ()).await?;
        
        if let Some(row) = rows.next().await? {
            Ok(row.get::<i64>(0).unwrap_or(0) as u64)
        } else {
            Ok(0)
        }
    }

    async fn get_last_sync_timestamp(&self, _table_name: &str) -> i64 {
        // Since we're now using sync_status table for change detection,
        // we don't need the timestamp-based logic anymore
        0
    }

    fn calculate_hash(&self, content: &str) -> String {
        match self.detection_config.hash_algorithm {
            HashAlgorithm::Blake3 => {
                format!("{}", blake3::hash(content.as_bytes()).to_hex())
            }
            HashAlgorithm::Sha256 => {
                let mut hasher = Sha256::new();
                hasher.update(content.as_bytes());
                format!("{:x}", hasher.finalize())
            }
            HashAlgorithm::FastHash => {
                // 使用简单的哈希算法
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                content.hash(&mut hasher);
                format!("{:x}", hasher.finish())
            }
        }
    }

    /// 计算内容哈希
    fn compute_content_hash(&self, content: &str) -> String {
        format!("{}", blake3::hash(content.as_bytes()).to_hex())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_incremental_sync_config_default() {
        let config = IncrementalSyncConfig::default();
        assert!(config.enabled);
        assert_eq!(config.max_batch_size, 1000);
        assert_eq!(config.min_sync_interval_ms, 5000);
        assert_eq!(config.force_full_sync_hours, 24);
    }

    #[test]
    fn test_change_detection_config_default() {
        let config = ChangeDetectionConfig::default();
        assert!(config.use_hash_detection);
        assert!(config.use_timestamp_detection);
        assert_eq!(config.timestamp_precision_ms, 1000);
    }

    #[test]
    fn test_incremental_sync_stats_default() {
        let stats = IncrementalSyncStats::default();
        assert_eq!(stats.checked_records, 0);
        assert_eq!(stats.changed_records, 0);
        assert_eq!(stats.synced_records, 0);
        assert_eq!(stats.skipped_records, 0);
    }

    #[test]
    fn test_change_type_equality() {
        assert_eq!(ChangeType::Insert, ChangeType::Insert);
        assert_ne!(ChangeType::Insert, ChangeType::Update);
    }
} 