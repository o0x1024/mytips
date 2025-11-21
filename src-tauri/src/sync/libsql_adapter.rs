use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

use super::libsql_sync_manager::{LibSqlSyncManager, LibSqlSyncConfig, SyncResult};
use crate::db::{SyncConfig, SyncMode};

/// LibSQL适配器，用于集成到现有的同步系统
pub struct LibSqlAdapter {
    /// LibSQL同步管理器
    sync_manager: Arc<RwLock<Option<LibSqlSyncManager>>>,
    /// 配置
    config: Arc<RwLock<Option<SyncConfig>>>,
}

impl LibSqlAdapter {
    /// 创建新的适配器
    pub fn new() -> Self {
        Self {
            sync_manager: Arc::new(RwLock::new(None)),
            config: Arc::new(RwLock::new(None)),
        }
    }

    /// 配置并初始化LibSQL同步管理器
    pub async fn configure(&self, sync_config: SyncConfig) -> Result<()> {
        info!("Configuring LibSQL adapter");

        // 验证配置
        if sync_config.sync_mode == SyncMode::Offline {
            return Err(anyhow!("Cannot configure LibSQL for offline mode"));
        }

        let remote_url = sync_config.remote_url.as_ref()
            .ok_or_else(|| anyhow!("Remote URL is required for LibSQL sync"))?;

        // 创建本地数据库路径
        let local_path = self.get_local_db_path().await?;

        // 创建LibSQL配置
        let libsql_config = LibSqlSyncConfig {
            sync_interval_secs: sync_config.sync_interval as u64,
            auto_sync_enabled: sync_config.auto_sync_enabled,
            read_your_writes: true,
            sync_timeout_secs: 30,
            connection_pool_size: 5,
        };

        // 创建并初始化同步管理器
        let manager = LibSqlSyncManager::new(
            local_path,
            remote_url.clone(),
            sync_config.auth_token.clone(),
            libsql_config,
        ).await?;

        // 初始化连接
        manager.initialize().await?;

        // 保存管理器和配置
        *self.sync_manager.write().await = Some(manager);
        *self.config.write().await = Some(sync_config);

        info!("LibSQL adapter configured successfully");
        Ok(())
    }

    /// 执行手动同步
    pub async fn manual_sync(&self) -> Result<crate::sync::SyncStats> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        let result = manager.manual_sync().await?;
        
        // 转换为标准的SyncStats格式
        Ok(crate::sync::SyncStats {
            total_records: if result.success { result.records_synced } else { 0 },
            synced_records: if result.success { result.records_synced } else { 0 },
            pending_records: 0,
            failed_records: if result.success { 0 } else { 1 },
            last_sync_time: result.sync_time,
            is_online: result.success,
        })
    }

    /// 获取连接（用于数据库操作）
    pub async fn get_connection(&self) -> Result<libsql::Connection> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        manager.get_connection().await
    }

    /// 获取本地连接（用于读操作）
    pub async fn get_local_connection(&self) -> Result<libsql::Connection> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        manager.get_local_connection().await
    }

    /// 获取远程连接（用于写操作）
    pub async fn get_remote_connection(&self) -> Result<libsql::Connection> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        manager.get_remote_connection().await
    }

    /// 测试连接
    pub async fn test_connection(&self) -> Result<bool> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        manager.test_connection().await
    }

    /// 获取同步状态
    pub async fn get_sync_status(&self) -> Result<crate::sync::SyncStats> {
        let manager_guard = self.sync_manager.read().await;
        let manager = manager_guard.as_ref()
            .ok_or_else(|| anyhow!("LibSQL sync manager not configured"))?;

        let state = manager.get_sync_status().await;
        
        Ok(crate::sync::SyncStats {
            total_records: 0, // libSQL不直接提供
            synced_records: 0,
            pending_records: 0,
            failed_records: state.sync_error_count as u64,
            last_sync_time: state.last_sync_time,
            is_online: state.is_online,
        })
    }

    /// 检查是否已配置
    pub async fn is_configured(&self) -> bool {
        self.sync_manager.read().await.is_some()
    }

    /// 关闭适配器
    pub async fn shutdown(&self) -> Result<()> {
        info!("Shutting down LibSQL adapter");

        let mut manager_guard = self.sync_manager.write().await;
        if let Some(manager) = manager_guard.take() {
            manager.shutdown().await?;
        }

        *self.config.write().await = None;

        info!("LibSQL adapter shutdown complete");
        Ok(())
    }

    /// 获取本地数据库路径
    async fn get_local_db_path(&self) -> Result<PathBuf> {
        // 使用应用数据目录
        let app_data_dir = dirs::data_dir()
            .ok_or_else(|| anyhow!("Unable to determine app data directory"))?
            .join("mytips");

        // 确保目录存在
        tokio::fs::create_dir_all(&app_data_dir).await?;

        Ok(app_data_dir.join("mytips_libsql.db"))
    }
}

impl Default for LibSqlAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// 用于测试连接的辅助函数
pub async fn test_libsql_connection(
    remote_url: &str,
    auth_token: Option<&str>,
) -> Result<bool> {
    use libsql::Builder;

    info!("Testing LibSQL connection to: {}", remote_url);

    // 确保auth_token包含完整的认证头格式 (scheme + token)
    let full_auth_token = match auth_token {
        Some(token) if token.contains(' ') => token.to_string(),
        Some(token) => format!("Bearer {}", token),
        None => String::new(),
    };

    let builder = Builder::new_remote(
        remote_url.to_string(),
        full_auth_token,
    );

    match builder.build().await {
        Ok(db) => {
            match db.connect() {
                Ok(conn) => {
                    // 使用 query 方法而不是 execute 来执行 SELECT 语句
                    match conn.query("SELECT 1 as test_value", ()).await {
                        Ok(mut rows) => {
                            // 验证查询是否返回了结果
                            match rows.next().await {
                                Ok(Some(_)) => {
                                    info!("LibSQL connection test successful");
                                    Ok(true)
                                },
                                Ok(None) => {
                                    error!("LibSQL connection test failed: no rows returned");
                                    Ok(false)
                                },
                                Err(e) => {
                                    error!("LibSQL connection test failed during row reading: {}", e);
                                    Ok(false)
                                }
                            }
                        }
                        Err(e) => {
                            error!("LibSQL connection test failed during query: {}", e);
                            Ok(false)
                        }
                    }
                }
                Err(e) => {
                    error!("LibSQL connection test failed during connect: {}", e);
                    Ok(false)
                }
            }
        }
        Err(e) => {
            error!("LibSQL connection test failed during build: {}", e);
            Ok(false)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::SyncMode;

    #[tokio::test]
    async fn test_adapter_creation() {
        let adapter = LibSqlAdapter::new();
        assert!(!adapter.is_configured().await);
    }

    #[tokio::test]
    async fn test_offline_config_rejection() {
        let adapter = LibSqlAdapter::new();
        let config = SyncConfig {
            id: "test".to_string(),
            remote_url: None,
            auth_token: None,
            sync_mode: SyncMode::Offline,
            sync_interval: 300,
            last_sync_at: 0,
            is_online: false,
            auto_sync_enabled: false,
            created_at: 0,
            updated_at: 0,
        };

        let result = adapter.configure(config).await;
        assert!(result.is_err());
    }
} 