use anyhow::{anyhow, Result};
use libsql::{Connection, Database, Builder};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::{Duration, Instant};
use uuid::Uuid;

/// 数据库配置信息
#[derive(Debug, Clone)]
pub enum DatabaseConfig {
    Local {
        path: String,
    },
    Remote {
        replica_path: String,
        remote_url: String,
        auth_token: Option<String>,
    },
}

impl DatabaseConfig {
    /// 根据配置创建Database实例
    pub async fn build_database(&self) -> Result<Database> {
        match self {
            DatabaseConfig::Local { path } => {
                let db = Builder::new_local(path.clone()).build().await
                    .map_err(|e| anyhow!("Failed to build local database: {}", e))?;
                
                // 确保本地数据库有正确的表结构
                let conn = db.connect().map_err(|e| anyhow!("Failed to connect to local database: {}", e))?;
                if let Err(e) = crate::db::init_schema(&conn).await {
                    eprintln!("Warning: Failed to initialize local database schema: {}", e);
                }
                drop(conn); // 释放连接
                
                Ok(db)
            }
            DatabaseConfig::Remote { replica_path, remote_url, auth_token } => {
                let builder = if let Some(token) = auth_token {
                    Builder::new_remote_replica(replica_path.clone(), remote_url.clone(), token.clone())
                } else {
                    Builder::new_remote_replica(replica_path.clone(), remote_url.clone(), String::new())
                };
                
                let db = tokio::time::timeout(
                    Duration::from_secs(30),
                    builder.build()
                ).await
                .map_err(|_| anyhow!("Database build timeout"))?
                .map_err(|e| anyhow!("Failed to build remote database: {}", e))?;
                
                // 同步数据库
                tokio::time::timeout(
                    Duration::from_secs(60),
                    db.sync()
                ).await
                .map_err(|_| anyhow!("Database sync timeout"))?
                .map_err(|e| anyhow!("Failed to sync remote database: {}", e))?;
                
                // 确保远程数据库有正确的表结构
                let conn = db.connect().map_err(|e| anyhow!("Failed to connect to remote database: {}", e))?;
                if let Err(e) = crate::db::init_schema(&conn).await {
                    eprintln!("Warning: Failed to initialize remote database schema: {}", e);
                }
                drop(conn); // 释放连接
                
                // 再次同步以确保表结构更新到远程
                tokio::time::timeout(
                    Duration::from_secs(60),
                    db.sync()
                ).await
                .map_err(|_| anyhow!("Database final sync timeout"))?
                .map_err(|e| anyhow!("Failed to sync schema to remote: {}", e))?;
                
                Ok(db)
            }
        }
    }
}

/// 连接池配置
#[derive(Debug, Clone)]
pub struct ConnectionPoolConfig {
    /// 最大连接数
    pub max_connections: usize,
    /// 连接空闲超时时间（秒）
    pub idle_timeout: u64,
    /// 连接最大生存时间（秒）
    pub max_lifetime: u64,
    /// 获取连接的超时时间（秒）
    pub acquire_timeout: u64,
}

impl Default for ConnectionPoolConfig {
    fn default() -> Self {
        Self {
            max_connections: 10, // 限制最大连接数
            idle_timeout: 300,   // 5分钟空闲超时
            max_lifetime: 1800,  // 30分钟最大生存时间
            acquire_timeout: 30, // 30秒获取连接超时
        }
    }
}

/// 连接包装器，包含连接和元数据
#[derive(Debug)]
struct PooledConnection {
    connection: Arc<Connection>,
    created_at: Instant,
    last_used: Arc<Mutex<Instant>>,
    id: String,
}

impl PooledConnection {
    fn new(connection: Connection) -> Self {
        let now = Instant::now();
        Self {
            connection: Arc::new(connection),
            created_at: now,
            last_used: Arc::new(Mutex::new(now)),
            id: Uuid::new_v4().to_string(),
        }
    }

    async fn is_expired(&self, config: &ConnectionPoolConfig) -> bool {
        let last_used = *self.last_used.lock().await;
        let now = Instant::now();
        
        // 检查是否超过最大生存时间
        if now.duration_since(self.created_at).as_secs() > config.max_lifetime {
            return true;
        }
        
        // 检查是否超过空闲超时时间
        if now.duration_since(last_used).as_secs() > config.idle_timeout {
            return true;
        }
        
        false
    }

    async fn update_last_used(&self) {
        *self.last_used.lock().await = Instant::now();
    }

    /// 测试连接是否仍然有效
    async fn is_valid(&self) -> bool {
        match self.connection.execute("SELECT 1", ()).await {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

/// 高效的数据库连接池
pub struct ConnectionPool {
    db_config: DatabaseConfig,
    config: ConnectionPoolConfig,
    pool: Arc<Mutex<Vec<PooledConnection>>>,
    semaphore: Arc<Semaphore>,
    stats: Arc<Mutex<PoolStats>>,
    // 缓存Database实例（可选优化）
    cached_database: Arc<Mutex<Option<Arc<Database>>>>,
}

#[derive(Debug, Default)]
pub struct PoolStats {
    pub total_connections: usize,
    pub active_connections: usize,
    pub idle_connections: usize,
    pub connections_created: u64,
    pub connections_closed: u64,
    pub acquire_timeouts: u64,
}

impl ConnectionPool {
    pub fn new(db_config: DatabaseConfig, config: ConnectionPoolConfig) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_connections));
        
        Self {
            db_config,
            config,
            pool: Arc::new(Mutex::new(Vec::new())),
            semaphore,
            stats: Arc::new(Mutex::new(PoolStats::default())),
            cached_database: Arc::new(Mutex::new(None)),
        }
    }

    /// 获取或创建Database实例
    async fn get_database(&self) -> Result<Arc<Database>> {
        // 首先检查缓存
        {
            let cached = self.cached_database.lock().await;
            if let Some(ref db) = *cached {
                return Ok(db.clone());
            }
        }
        
        // 创建新的Database实例
        let db = self.db_config.build_database().await?;
        let db_arc = Arc::new(db);
        
        // 缓存Database实例
        {
            let mut cached = self.cached_database.lock().await;
            *cached = Some(db_arc.clone());
        }
        
        Ok(db_arc)
    }

    /// 获取连接
    pub async fn acquire(&self) -> Result<PooledConnectionHandle> {
        // 使用信号量限制并发连接数
        let permit = tokio::time::timeout(
            Duration::from_secs(self.config.acquire_timeout),
            Arc::clone(&self.semaphore).acquire_owned()
        ).await
        .map_err(|_| anyhow!("Connection acquire timeout"))?
        .map_err(|_| anyhow!("Semaphore closed"))?;

        // 首先尝试从池中获取可用连接
        if let Some(connection) = self.get_pooled_connection().await? {
            connection.update_last_used().await;
            return Ok(PooledConnectionHandle::new(
                connection.connection.clone(),
                self.pool.clone(),
                permit,
                connection.id.clone(),
            ));
        }

        // 池中没有可用连接，创建新连接
        let new_connection = self.create_new_connection().await?;
        let connection_id = new_connection.id.clone();
        let connection_arc = new_connection.connection.clone();

        // 更新统计信息
        {
            let mut stats = self.stats.lock().await;
            stats.connections_created += 1;
            stats.active_connections += 1;
        }

        Ok(PooledConnectionHandle::new(
            connection_arc,
            self.pool.clone(),
            permit,
            connection_id,
        ))
    }

    /// 从池中获取可用连接
    async fn get_pooled_connection(&self) -> Result<Option<PooledConnection>> {
        let mut pool = self.pool.lock().await;
        
        // 清理过期连接
        pool.retain(|conn| {
            let config = &self.config;
            let now = Instant::now();
            
            // 检查连接是否过期
            let is_expired = now.duration_since(conn.created_at).as_secs() > config.max_lifetime;
            
            if is_expired {
                tokio::spawn(async move {
                    // 异步统计更新
                });
            }
            
            !is_expired
        });

        // 查找有效的连接
        while let Some(connection) = pool.pop() {
            // 检查连接是否过期
            if connection.is_expired(&self.config).await {
                // 连接过期，丢弃并继续
                continue;
            }

            // 测试连接是否有效
            if connection.is_valid().await {
                return Ok(Some(connection));
            }
        }

        Ok(None)
    }

    /// 创建新连接
    async fn create_new_connection(&self) -> Result<PooledConnection> {
        let database = self.get_database().await?;
        
        let connection = tokio::time::timeout(
            Duration::from_secs(30),
            async { database.connect() }
        ).await
        .map_err(|_| anyhow!("Connection creation timeout"))?
        .map_err(|e| anyhow!("Failed to create connection: {}", e))?;

        // 为新连接设置WAL安全参数
        let _ = connection.execute("PRAGMA synchronous=FULL", ()).await; // 使用FULL同步模式，更安全
        let _ = connection.execute("PRAGMA journal_mode=DELETE", ()).await; // 禁用WAL模式，使用传统日志
        let _ = connection.execute("PRAGMA cache_size=500", ()).await; // 减少缓存大小，降低内存压力
        let _ = connection.execute("PRAGMA temp_store=memory", ()).await;
        let _ = connection.execute("PRAGMA busy_timeout=30000", ()).await; // 30秒忙等超时
        let _ = connection.execute("PRAGMA wal_autocheckpoint=100", ()).await; // 降低WAL检查点频率

        // 确保数据库表结构已初始化（对于新创建的数据库连接）
        if let Err(e) = crate::db::init_schema(&connection).await {
            eprintln!("Warning: Failed to initialize database schema for new connection: {}", e);
            // 不阻止连接创建，但记录警告
        }

        Ok(PooledConnection::new(connection))
    }

    /// 归还连接到池中
    async fn return_connection(&self, connection_id: String) {
        let mut stats = self.stats.lock().await;
        stats.active_connections = stats.active_connections.saturating_sub(1);
        stats.idle_connections += 1;
    }

    /// 关闭连接池
    pub async fn close(&self) {
        let mut pool = self.pool.lock().await;
        pool.clear();
        
        // 清除缓存的Database实例
        {
            let mut cached = self.cached_database.lock().await;
            *cached = None;
        }
        
        let mut stats = self.stats.lock().await;
        stats.connections_closed += stats.total_connections as u64;
        stats.total_connections = 0;
        stats.active_connections = 0;
        stats.idle_connections = 0;
    }

    /// 获取池统计信息
    pub async fn stats(&self) -> PoolStats {
        let stats = self.stats.lock().await;
        PoolStats {
            total_connections: stats.total_connections,
            active_connections: stats.active_connections,
            idle_connections: stats.idle_connections,
            connections_created: stats.connections_created,
            connections_closed: stats.connections_closed,
            acquire_timeouts: stats.acquire_timeouts,
        }
    }

    /// 清理过期连接（后台任务）
    pub async fn cleanup_expired_connections(&self) {
        let mut pool = self.pool.lock().await;
        let initial_count = pool.len();
        
        pool.retain(|conn| {
            let config = &self.config;
            let now = Instant::now();
            
            // 检查连接是否过期
            let is_expired = now.duration_since(conn.created_at).as_secs() > config.max_lifetime;
            !is_expired
        });
        
        let removed_count = initial_count - pool.len();
        if removed_count > 0 {
            println!("Cleaned up {} expired connections", removed_count);
            
            let mut stats = self.stats.lock().await;
            stats.connections_closed += removed_count as u64;
            stats.total_connections = stats.total_connections.saturating_sub(removed_count);
        }
    }

    /// 重新创建Database实例（用于处理连接问题）
    pub async fn recreate_database(&self) -> Result<()> {
        let mut cached = self.cached_database.lock().await;
        *cached = None; // 清除缓存，强制重新创建
        drop(cached);
        
        // 预先创建新的Database实例
        self.get_database().await?;
        Ok(())
    }
}

/// 连接句柄，自动管理连接的生命周期
pub struct PooledConnectionHandle {
    connection: Arc<Connection>,
    pool: Arc<Mutex<Vec<PooledConnection>>>,
    _permit: tokio::sync::OwnedSemaphorePermit,
    connection_id: String,
}

impl PooledConnectionHandle {
    fn new(
        connection: Arc<Connection>,
        pool: Arc<Mutex<Vec<PooledConnection>>>,
        permit: tokio::sync::OwnedSemaphorePermit,
        connection_id: String,
    ) -> Self {
        Self {
            connection,
            pool,
            _permit: permit,
            connection_id,
        }
    }

    /// 获取底层连接
    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}

impl Drop for PooledConnectionHandle {
    fn drop(&mut self) {
        // 连接句柄被释放时，permit会自动释放，允许新的连接获取
        // 这里可以添加额外的清理逻辑
    }
}

/// 连接池管理器
pub struct ConnectionPoolManager {
    local_pool: Option<Arc<ConnectionPool>>,
    remote_pool: Option<Arc<ConnectionPool>>,
    config: ConnectionPoolConfig,
}

impl ConnectionPoolManager {
    pub fn new(config: ConnectionPoolConfig) -> Self {
        Self {
            local_pool: None,
            remote_pool: None,
            config,
        }
    }

    /// 设置本地数据库连接池
    pub async fn set_local_database(&mut self, db_config: DatabaseConfig) {
        self.local_pool = Some(Arc::new(ConnectionPool::new(db_config, self.config.clone())));
    }

    /// 设置远程数据库连接池
    pub async fn set_remote_database(&mut self, db_config: DatabaseConfig) {
        self.remote_pool = Some(Arc::new(ConnectionPool::new(db_config, self.config.clone())));
    }

    /// 获取本地连接
    pub async fn acquire_local(&self) -> Result<PooledConnectionHandle> {
        match &self.local_pool {
            Some(pool) => pool.acquire().await,
            None => Err(anyhow!("Local database pool not initialized")),
        }
    }

    /// 获取远程连接
    pub async fn acquire_remote(&self) -> Result<PooledConnectionHandle> {
        match &self.remote_pool {
            Some(pool) => pool.acquire().await,
            None => Err(anyhow!("Remote database pool not initialized")),
        }
    }

    /// 获取连接（默认获取远程连接）
    pub async fn get_connection(&self) -> Result<PooledConnectionHandle> {
        self.acquire_remote().await
    }

    /// 创建指定名称的连接池
    pub async fn create_pool(&mut self, name: String, db_config: DatabaseConfig, config: ConnectionPoolConfig) -> Result<()> {
        match name.as_str() {
            "emergency_remote" | "remote" => {
                self.config = config;
                self.remote_pool = Some(Arc::new(ConnectionPool::new(db_config, self.config.clone())));
                Ok(())
            }
            "local" => {
                self.config = config;
                self.local_pool = Some(Arc::new(ConnectionPool::new(db_config, self.config.clone())));
                Ok(())
            }
            _ => Err(anyhow!("Unknown pool name: {}", name))
        }
    }

    /// 重置所有连接池
    pub async fn reset_all_pools(&mut self) -> Result<()> {
        // 关闭现有连接池
        if let Some(pool) = &self.local_pool {
            pool.close().await;
        }
        if let Some(pool) = &self.remote_pool {
            pool.close().await;
        }

        // 清除池引用
        self.local_pool = None;
        self.remote_pool = None;

        println!("All connection pools have been reset");
        Ok(())
    }

    /// 获取本地连接池统计
    pub async fn local_stats(&self) -> Option<PoolStats> {
        match &self.local_pool {
            Some(pool) => Some(pool.stats().await),
            None => None,
        }
    }

    /// 获取远程连接池统计
    pub async fn remote_stats(&self) -> Option<PoolStats> {
        match &self.remote_pool {
            Some(pool) => Some(pool.stats().await),
            None => None,
        }
    }

    /// 关闭所有连接池
    pub async fn close_all(&self) {
        if let Some(pool) = &self.local_pool {
            pool.close().await;
        }
        if let Some(pool) = &self.remote_pool {
            pool.close().await;
        }
    }

    /// 重新创建远程数据库连接（用于错误恢复）
    pub async fn recreate_remote_database(&self) -> Result<()> {
        match &self.remote_pool {
            Some(pool) => pool.recreate_database().await,
            None => Err(anyhow!("Remote database pool not initialized")),
        }
    }

    /// 启动后台清理任务
    pub fn start_cleanup_task(&self) -> tokio::task::JoinHandle<()> {
        let local_pool = self.local_pool.clone();
        let remote_pool = self.remote_pool.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // 每5分钟清理一次
            
            loop {
                interval.tick().await;
                
                if let Some(pool) = &local_pool {
                    pool.cleanup_expired_connections().await;
                }
                
                if let Some(pool) = &remote_pool {
                    pool.cleanup_expired_connections().await;
                }
            }
        })
    }
} 