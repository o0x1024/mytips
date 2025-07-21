use anyhow::{anyhow, Result};
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use libsql::{Connection, Database};
use tracing::{debug, error, info, warn};
use futures::future::BoxFuture;

/// 事务类型
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionType {
    /// 本地事务
    Local,
    /// 远程事务
    Remote,
    /// 分布式事务（跨本地和远程）
    Distributed,
}

/// 事务状态
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    /// 事务开始
    Started,
    /// 事务提交中
    Committing,
    /// 事务已提交
    Committed,
    /// 事务回滚中
    RollingBack,
    /// 事务已回滚
    RolledBack,
    /// 事务失败
    Failed,
}

/// 原子事务管理器
pub struct AtomicTransactionManager {
    local_db: Arc<Database>,
    remote_db: Arc<RwLock<Option<Database>>>,
    transaction_timeout: Duration,
    max_retries: u32,
}

impl AtomicTransactionManager {
    pub fn new(local_db: Arc<Database>, remote_db: Arc<RwLock<Option<Database>>>) -> Self {
        Self {
            local_db,
            remote_db,
            transaction_timeout: Duration::from_secs(30),
            max_retries: 3,
        }
    }

    /// 执行原子批处理操作
    pub async fn execute_atomic_batch<F, T>(&self, operations: F) -> Result<T>
    where
        F: FnOnce(&Connection, &Connection) -> BoxFuture<'static, Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let local_conn = self.local_db.connect()?;
        
        let remote_db_guard = self.remote_db.read().await;
        let remote_db = remote_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        drop(remote_db_guard);

        // 设置事务隔离级别
        local_conn.execute("PRAGMA synchronous=FULL", ()).await?;
        remote_conn.execute("PRAGMA synchronous=FULL", ()).await?;

        // 开始分布式事务
        local_conn.execute("BEGIN IMMEDIATE", ()).await
            .map_err(|e| anyhow!("Failed to begin local transaction: {}", e))?;
        
        let remote_transaction_result = remote_conn.execute("BEGIN IMMEDIATE", ()).await;
        
        if let Err(e) = remote_transaction_result {
            // 回滚本地事务
            let _ = local_conn.execute("ROLLBACK", ()).await;
            return Err(anyhow!("Failed to begin remote transaction: {}", e));
        }

        // 执行操作
        let operation_result = tokio::time::timeout(
            self.transaction_timeout,
            operations(&local_conn, &remote_conn)
        ).await;

        match operation_result {
            Ok(Ok(result)) => {
                // 提交两个事务
                let local_commit = local_conn.execute("COMMIT", ()).await;
                let remote_commit = remote_conn.execute("COMMIT", ()).await;

                match (local_commit, remote_commit) {
                    (Ok(_), Ok(_)) => Ok(result),
                    (Err(e), _) => {
                        // 尝试回滚远程事务
                        let _ = remote_conn.execute("ROLLBACK", ()).await;
                        Err(anyhow!("Local commit failed: {}", e))
                    }
                    (_, Err(e)) => {
                        // 尝试回滚本地事务
                        let _ = local_conn.execute("ROLLBACK", ()).await;
                        Err(anyhow!("Remote commit failed: {}", e))
                    }
                }
            }
            Ok(Err(e)) => {
                // 操作失败，回滚两个事务
                let _ = local_conn.execute("ROLLBACK", ()).await;
                let _ = remote_conn.execute("ROLLBACK", ()).await;
                Err(e)
            }
            Err(_) => {
                // 超时，回滚两个事务
                let _ = local_conn.execute("ROLLBACK", ()).await;
                let _ = remote_conn.execute("ROLLBACK", ()).await;
                Err(anyhow!("Transaction timed out after {:?}", self.transaction_timeout))
            }
        }
    }

    /// 原子批量插入
    pub async fn atomic_batch_insert<T>(&self, table_name: &str, records: Vec<T>) -> Result<u64>
    where
        T: Send + Sync + 'static,
    {
        let table_name = table_name.to_string();
        let record_count = records.len() as u64;

        self.execute_atomic_batch(move |local_conn, remote_conn| {
            Box::pin(async move {
                // 这里需要根据具体的记录类型实现插入逻辑
                // 由于T是泛型，我们需要在调用处提供具体的插入逻辑
                
                // 示例：假设我们有一个通用的插入方法
                for _record in records {
                    // 插入到本地
                    // local_conn.execute(&insert_sql, params).await?;
                    
                    // 插入到远程
                    // remote_conn.execute(&insert_sql, params).await?;
                }

                Ok(record_count)
            })
        }).await
    }

    /// 执行嵌套事务（使用保存点）
    pub async fn execute_nested_transaction<F, T>(&self, savepoint_name: &str, operations: F) -> Result<T>
    where
        F: FnOnce(&Connection, &Connection) -> BoxFuture<'static, Result<T>> + Send + 'static,
        T: Send + 'static,
    {
        let local_conn = self.local_db.connect()?;
        
        let remote_db_guard = self.remote_db.read().await;
        let remote_db = remote_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        drop(remote_db_guard);

        // 创建保存点
        let savepoint_sql = format!("SAVEPOINT {}", savepoint_name);
        local_conn.execute(&savepoint_sql, ()).await
            .map_err(|e| anyhow!("Failed to create local savepoint: {}", e))?;
        
        let remote_savepoint_result = remote_conn.execute(&savepoint_sql, ()).await;
        if let Err(e) = remote_savepoint_result {
            // 回滚本地保存点
            let rollback_sql = format!("ROLLBACK TO {}", savepoint_name);
            let _ = local_conn.execute(&rollback_sql, ()).await;
            return Err(anyhow!("Failed to create remote savepoint: {}", e));
        }

        // 执行嵌套操作
        let operation_result = operations(&local_conn, &remote_conn).await;

        match operation_result {
            Ok(result) => {
                // 释放保存点
                let release_sql = format!("RELEASE {}", savepoint_name);
                let local_release = local_conn.execute(&release_sql, ()).await;
                let remote_release = remote_conn.execute(&release_sql, ()).await;

                match (local_release, remote_release) {
                    (Ok(_), Ok(_)) => Ok(result),
                    _ => {
                        // 如果释放失败，尝试回滚到保存点
                        let rollback_sql = format!("ROLLBACK TO {}", savepoint_name);
                        let _ = local_conn.execute(&rollback_sql, ()).await;
                        let _ = remote_conn.execute(&rollback_sql, ()).await;
                        Err(anyhow!("Failed to release savepoints"))
                    }
                }
            }
            Err(e) => {
                // 回滚到保存点
                let rollback_sql = format!("ROLLBACK TO {}", savepoint_name);
                let _ = local_conn.execute(&rollback_sql, ()).await;
                let _ = remote_conn.execute(&rollback_sql, ()).await;
                Err(e)
            }
        }
    }
}

/// 事务一致性验证器
pub struct TransactionConsistencyValidator {
    local_db: Arc<Database>,
    remote_db: Arc<RwLock<Option<Database>>>,
}

impl TransactionConsistencyValidator {
    pub fn new(local_db: Arc<Database>, remote_db: Arc<RwLock<Option<Database>>>) -> Self {
        Self {
            local_db,
            remote_db,
        }
    }

    /// 验证一致性
    pub async fn validate_consistency(&self, tables: &[&str]) -> Result<ConsistencyReport> {
        let mut report = ConsistencyReport::new();
        
        let local_conn = self.local_db.connect()?;
        
        let remote_db_guard = self.remote_db.read().await;
        let remote_db = remote_db_guard.as_ref()
            .ok_or_else(|| anyhow!("Remote database not connected"))?;
        let remote_conn = remote_db.connect()?;
        drop(remote_db_guard);

        for table in tables {
            // 检查记录数量
            let local_count = self.get_table_count(&local_conn, table).await?;
            let remote_count = self.get_table_count(&remote_conn, table).await?;
            
            if local_count != remote_count {
                report.add_mismatch(
                    table,
                    "record_count",
                    local_count.to_string(),
                    remote_count.to_string(),
                );
            }

            // 检查数据校验和
            let local_checksum = self.get_table_checksum(&local_conn, table).await?;
            let remote_checksum = self.get_table_checksum(&remote_conn, table).await?;
            
            if local_checksum != remote_checksum {
                report.add_mismatch(
                    table,
                    "data_checksum",
                    local_checksum,
                    remote_checksum,
                );
            }
        }

        Ok(report)
    }

    async fn get_table_count(&self, conn: &Connection, table: &str) -> Result<i64> {
        let sql = format!("SELECT COUNT(*) FROM {}", table);
        let mut rows = conn.query(&sql, ()).await?;
        
        if let Some(row) = rows.next().await? {
            Ok(row.get::<i64>(0).unwrap_or(0))
        } else {
            Ok(0)
        }
    }

    async fn get_table_checksum(&self, conn: &Connection, table: &str) -> Result<String> {
        // 使用简单的哈希算法计算表的校验和
        // 注意：这里使用的是简化的实现，实际应用中可能需要更复杂的校验和算法
        let sql = format!(
            "SELECT GROUP_CONCAT(
                CASE 
                    WHEN id IS NULL THEN 'NULL' 
                    ELSE CAST(id AS TEXT) 
                END || '|' ||
                CASE 
                    WHEN created_at IS NULL THEN 'NULL' 
                    ELSE CAST(created_at AS TEXT) 
                END
            ) FROM {} ORDER BY id",
            table
        );
        
        let mut rows = conn.query(&sql, ()).await?;
        
        if let Some(row) = rows.next().await? {
            let data = row.get::<Option<String>>(0).unwrap_or(None).unwrap_or_default();
            Ok(format!("{:x}", md5::compute(data.as_bytes())))
        } else {
            Ok("empty".to_string())
        }
    }
}

/// 一致性报告
#[derive(Debug, Clone)]
pub struct ConsistencyReport {
    mismatches: Vec<ConsistencyMismatch>,
    validation_time: i64,
}

#[derive(Debug, Clone)]
pub struct ConsistencyMismatch {
    table_name: String,
    field_name: String,
    local_value: String,
    remote_value: String,
}

impl ConsistencyReport {
    pub fn new() -> Self {
        Self {
            mismatches: Vec::new(),
            validation_time: Utc::now().timestamp_millis(),
        }
    }

    pub fn add_mismatch(&mut self, table: &str, field: &str, local_value: String, remote_value: String) {
        self.mismatches.push(ConsistencyMismatch {
            table_name: table.to_string(),
            field_name: field.to_string(),
            local_value,
            remote_value,
        });
    }

    pub fn mismatch_count(&self) -> usize {
        self.mismatches.len()
    }

    pub fn is_consistent(&self) -> bool {
        self.mismatches.is_empty()
    }

    pub fn get_mismatches(&self) -> &[ConsistencyMismatch] {
        &self.mismatches
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    #[test]
    fn test_consistency_report() {
        let mut report = ConsistencyReport::new();
        
        assert!(report.is_consistent());
        assert_eq!(report.mismatch_count(), 0);
        
        report.add_mismatch("tips", "count", "100".to_string(), "99".to_string());
        
        assert!(!report.is_consistent());
        assert_eq!(report.mismatch_count(), 1);
        
        let mismatches = report.get_mismatches();
        assert_eq!(mismatches[0].table_name, "tips");
        assert_eq!(mismatches[0].field_name, "count");
        assert_eq!(mismatches[0].local_value, "100");
        assert_eq!(mismatches[0].remote_value, "99");
    }

    #[test]
    fn test_transaction_status() {
        assert_eq!(TransactionStatus::Started, TransactionStatus::Started);
        assert_ne!(TransactionStatus::Started, TransactionStatus::Committed);
    }

    #[test]
    fn test_transaction_type() {
        assert_eq!(TransactionType::Local, TransactionType::Local);
        assert_ne!(TransactionType::Local, TransactionType::Distributed);
    }
} 