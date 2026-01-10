//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use crate::core::DMSCResult;
use crate::database::{DatabaseMetrics, DMSCDatabase, DMSCDatabaseConfig, DMSCDBResult, DMSCDBRow};
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};

#[derive(Clone)]
pub struct PooledDatabase {
    id: u32,
    inner: Arc<dyn DMSCDatabase>,
    pool: Arc<DMSCDatabasePool>,
}

impl PooledDatabase {
    pub fn new(id: u32, inner: Arc<dyn DMSCDatabase>, pool: Arc<DMSCDatabasePool>) -> Self {
        Self { id, inner, pool }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        self.inner.execute(sql).await
    }

    pub async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        self.inner.query(sql).await
    }

    pub async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        self.inner.query_one(sql).await
    }

    pub async fn ping(&self) -> DMSCResult<bool> {
        self.inner.ping().await
    }

    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    pub fn pool_metrics(&self) -> DatabaseMetrics {
        self.pool.metrics()
    }
}

#[async_trait::async_trait]
impl DMSCDatabase for PooledDatabase {
    fn database_type(&self) -> crate::database::DatabaseType {
        self.inner.database_type()
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        self.inner.execute(sql).await
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        self.inner.query(sql).await
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        self.inner.query_one(sql).await
    }

    async fn ping(&self) -> DMSCResult<bool> {
        self.inner.ping().await
    }

    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    async fn close(&self) -> DMSCResult<()> {
        self.pool.close().await
    }

    async fn batch_execute(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> DMSCResult<Vec<u64>> {
        self.inner.batch_execute(sql, params).await
    }

    async fn batch_query(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> DMSCResult<Vec<DMSCDBResult>> {
        self.inner.batch_query(sql, params).await
    }

    async fn execute_with_params(&self, sql: &str, params: &[serde_json::Value]) -> DMSCResult<u64> {
        self.inner.execute_with_params(sql, params).await
    }

    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> DMSCResult<DMSCDBResult> {
        self.inner.query_with_params(sql, params).await
    }

    async fn transaction(&self) -> DMSCResult<Box<dyn crate::database::DMSCDatabaseTransaction>> {
        self.inner.transaction().await
    }
}

struct PoolConnection {
    db: Arc<dyn DMSCDatabase>,
    acquired_at: Instant,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDatabasePool {
    config: DMSCDatabaseConfig,
    connections: Arc<DashMap<u32, PoolConnection>>,
    available: Arc<DashMap<u32, PoolConnection>>,
    connection_ids: Arc<AtomicU64>,
    semaphore: Arc<Semaphore>,
    max_idle_time: Duration,
    max_lifetime: Duration,
    idle_connections: Arc<AtomicU64>,
    active_connections: Arc<AtomicU64>,
    total_connections: Arc<AtomicU64>,
    queries_executed: Arc<AtomicU64>,
    errors: Arc<AtomicU64>,
}

impl DMSCDatabasePool {
    pub async fn new(config: DMSCDatabaseConfig) -> DMSCResult<Self> {
        let pool = Self {
            config: config.clone(),
            connections: Arc::new(DashMap::new()),
            available: Arc::new(DashMap::new()),
            connection_ids: Arc::new(AtomicU64::new(0)),
            semaphore: Arc::new(Semaphore::new(config.max_connections as usize)),
            max_idle_time: Duration::from_secs(config.idle_timeout_secs),
            max_lifetime: Duration::from_secs(config.max_lifetime_secs),
            idle_connections: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
            total_connections: Arc::new(AtomicU64::new(0)),
            queries_executed: Arc::new(AtomicU64::new(0)),
            errors: Arc::new(AtomicU64::new(0)),
        };

        for _ in 0..config.min_idle_connections {
            if let Ok(conn) = pool.create_connection().await {
                let id = pool.connection_ids.fetch_add(1, Ordering::SeqCst) as u32;
                pool.available.insert(id, PoolConnection { db: conn, acquired_at: Instant::now() });
                pool.idle_connections.fetch_add(1, Ordering::SeqCst);
                pool.total_connections.fetch_add(1, Ordering::SeqCst);
            }
        }

        Ok(pool)
    }

    async fn create_connection(&self) -> DMSCResult<Arc<dyn DMSCDatabase>> {
        match self.config.database_type {
            #[cfg(feature = "postgres")]
            crate::database::DatabaseType::Postgres => {
                let connection_string = self.config.connection_string();
                let (client, conn) = tokio_postgres::Config::new()
                    .connect_timeout(Duration::from_secs(self.config.connection_timeout_secs))
                    .connect(&connection_string)
                    .await
                    .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
                tokio::spawn(async move {
                    if let Err(e) = conn.await {
                        eprintln!("PostgreSQL connection error: {}", e);
                    }
                });
                Ok(Arc::new(crate::database::postgres::PostgresDatabase::new(client, self.config.clone())) as Arc<dyn DMSCDatabase>)
            }
            #[cfg(feature = "mysql")]
            crate::database::DatabaseType::MySQL => {
                let connection_string = self.config.connection_string();
                let opts = mysql::Opts::from_url(&connection_string).map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
                let pool = mysql::Pool::new(mysql::PoolOpts::default().with_conn_idle_timeout(Duration::from_secs(self.config.idle_timeout_secs)));
                Ok(Arc::new(crate::database::mysql::MySQLDatabase::new(pool, self.config.clone())) as Arc<dyn DMSCDatabase>)
            }
            #[cfg(feature = "sqlite")]
            crate::database::DatabaseType::SQLite => {
                let conn = rusqlite::Connection::open(&self.config.database)
                    .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
                conn.busy_timeout(Duration::from_secs(self.config.connection_timeout_secs))
                    .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
                Ok(Arc::new(crate::database::sqlite::SQLiteDatabase::new(conn, self.config.clone())) as Arc<dyn DMSCDatabase>)
            }
            _ => Err(crate::core::DMSCError::Config("Unsupported database type".to_string())),
        }
    }

    pub async fn get(&self) -> DMSCResult<PooledDatabase> {
        let _permit = self.semaphore.acquire().await.map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;

        let mut reused_db = None;
        let mut reused_id = None;

        let now = Instant::now();
        
        for entry in self.available.iter() {
            let id = *entry.key();
            let conn = entry.value();
            if now.duration_since(conn.acquired_at) > self.max_idle_time || now.duration_since(conn.acquired_at) > self.max_lifetime {
                self.available.remove(&id);
                let _ = conn.db.close().await;
            } else {
                reused_db = Some(conn.db.clone());
                reused_id = Some(id);
                self.available.remove(&id);
                self.idle_connections.fetch_sub(1, Ordering::SeqCst);
                self.active_connections.fetch_add(1, Ordering::SeqCst);
                break;
            }
        }

        let (db, id) = if let Some((existing_db, existing_id)) = reused_db.zip(reused_id) {
            (existing_db, existing_id)
        } else {
            match self.create_connection().await {
                Ok(new_conn) => {
                    let id = self.connection_ids.fetch_add(1, Ordering::SeqCst) as u32;
                    self.total_connections.fetch_add(1, Ordering::SeqCst);
                    self.active_connections.fetch_add(1, Ordering::SeqCst);
                    (new_conn, id)
                }
                Err(e) => {
                    self.errors.fetch_add(1, Ordering::SeqCst);
                    return Err(e);
                }
            }
        };

        Ok(PooledDatabase::new(id, db, Arc::new(self.clone())))
    }

    pub async fn release(&self, db: PooledDatabase) {
        self.active_connections.fetch_sub(1, Ordering::SeqCst);
        self.idle_connections.fetch_add(1, Ordering::SeqCst);
        
        self.available.insert(db.id(), PoolConnection { 
            db: db.inner.clone(),
            acquired_at: Instant::now(),
        });
    }

    pub async fn close(&self) -> DMSCResult<()> {
        self.semaphore.close();
        for entry in self.connections.iter() {
            let _ = entry.value().db.close().await;
        }
        self.connections.clear();
        self.available.clear();
        Ok(())
    }

    pub fn metrics(&self) -> DatabaseMetrics {
        DatabaseMetrics {
            active_connections: self.active_connections.load(Ordering::SeqCst),
            idle_connections: self.idle_connections.load(Ordering::SeqCst),
            total_connections: self.total_connections.load(Ordering::SeqCst),
            queries_executed: self.queries_executed.load(Ordering::SeqCst),
            query_duration_ms: 0.0,
            errors: self.errors.load(Ordering::SeqCst),
        }
    }
}

impl Clone for DMSCDatabasePool {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            connections: self.connections.clone(),
            available: self.available.clone(),
            connection_ids: self.connection_ids.clone(),
            semaphore: self.semaphore.clone(),
            max_idle_time: self.max_idle_time,
            max_lifetime: self.max_lifetime,
            idle_connections: self.idle_connections.clone(),
            active_connections: self.active_connections.clone(),
            total_connections: self.total_connections.clone(),
            queries_executed: self.queries_executed.clone(),
            errors: self.errors.clone(),
        }
    }
}
