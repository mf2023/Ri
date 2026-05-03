//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use crate::core::RiResult;
use crate::database::{RiDatabase, RiDatabaseConfig, RiDBResult, RiDBRow};
use dashmap::DashMap;
use std::sync::atomic::{AtomicU64, AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, Instant};
use std::sync::RwLock;

fn handle_poisoned_lock<T>(lock_result: Result<T, std::sync::PoisonError<T>>) -> T {
    match lock_result {
        Ok(guard) => guard,
        Err(e) => {
            log::warn!("[Ri.Database] Lock was poisoned, recovering...");
            e.into_inner()
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Default)]
pub struct RiDatabaseMetrics {
    pub active_connections: u64,
    pub idle_connections: u64,
    pub total_connections: u64,
    pub queries_executed: u64,
    pub query_duration_ms: f64,
    pub errors: u64,
    pub utilization_rate: f64,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct RiDynamicPoolConfig {
    pub enable_dynamic_scaling: bool,
    pub scale_up_threshold: f64,
    pub scale_down_threshold: f64,
    pub scale_down_cooldown_secs: u64,
    pub min_connections: u32,
    pub max_connections: u32,
    pub scale_up_step: u32,
    pub scale_down_step: u32,
}

impl Default for RiDynamicPoolConfig {
    fn default() -> Self {
        Self {
            enable_dynamic_scaling: true,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            scale_down_cooldown_secs: 300,
            min_connections: 2,
            max_connections: 50,
            scale_up_step: 2,
            scale_down_step: 1,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiDynamicPoolConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn create() -> Self {
        Self::default()
    }

    fn get_enable_dynamic_scaling(&self) -> bool {
        self.enable_dynamic_scaling
    }

    fn set_enable_dynamic_scaling(&mut self, value: bool) {
        self.enable_dynamic_scaling = value;
    }

    fn get_scale_up_threshold(&self) -> f64 {
        self.scale_up_threshold
    }

    fn set_scale_up_threshold(&mut self, value: f64) {
        self.scale_up_threshold = value;
    }

    fn get_scale_down_threshold(&self) -> f64 {
        self.scale_down_threshold
    }

    fn set_scale_down_threshold(&mut self, value: f64) {
        self.scale_down_threshold = value;
    }

    fn get_scale_down_cooldown_secs(&self) -> u64 {
        self.scale_down_cooldown_secs
    }

    fn set_scale_down_cooldown_secs(&mut self, value: u64) {
        self.scale_down_cooldown_secs = value;
    }

    fn get_min_connections(&self) -> u32 {
        self.min_connections
    }

    fn set_min_connections(&mut self, value: u32) {
        self.min_connections = value;
    }

    fn get_max_connections(&self) -> u32 {
        self.max_connections
    }

    fn set_max_connections(&mut self, value: u32) {
        self.max_connections = value;
    }

    fn get_scale_up_step(&self) -> u32 {
        self.scale_up_step
    }

    fn set_scale_up_step(&mut self, value: u32) {
        self.scale_up_step = value;
    }

    fn get_scale_down_step(&self) -> u32 {
        self.scale_down_step
    }

    fn set_scale_down_step(&mut self, value: u32) {
        self.scale_down_step = value;
    }
}

#[derive(Clone)]
pub struct PooledDatabase {
    id: u32,
    inner: Arc<dyn RiDatabase>,
    pool: Arc<RiDatabasePool>,
}

impl Drop for PooledDatabase {
    fn drop(&mut self) {
        // Automatically release the connection back to the pool when dropped
        // This prevents resource leaks if the user forgets to call release()
        let pool = self.pool.clone();
        let id = self.id;
        let inner = self.inner.clone();
        
        // Use tokio::spawn to handle the async release operation
        // since Drop cannot be async
        tokio::spawn(async move {
            pool.active_connections.fetch_sub(1, Ordering::SeqCst);
            pool.idle_connections.fetch_add(1, Ordering::SeqCst);
            
            pool.available.insert(id, PoolConnection { 
                db: inner,
                acquired_at: Instant::now(),
                created_at: Instant::now(),
            });

            let _ = pool.check_and_scale().await;
        });
    }
}

impl PooledDatabase {
    pub fn new(id: u32, inner: Arc<dyn RiDatabase>, pool: Arc<RiDatabasePool>) -> Self {
        Self { id, inner, pool }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub async fn execute(&self, sql: &str) -> RiResult<u64> {
        self.inner.execute(sql).await
    }

    pub async fn query(&self, sql: &str) -> RiResult<RiDBResult> {
        self.inner.query(sql).await
    }

    pub async fn query_one(&self, sql: &str) -> RiResult<Option<RiDBRow>> {
        self.inner.query_one(sql).await
    }

    pub async fn ping(&self) -> RiResult<bool> {
        self.inner.ping().await
    }

    pub fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    pub fn pool_metrics(&self) -> RiDatabaseMetrics {
        self.pool.metrics()
    }
}

#[async_trait::async_trait]
impl RiDatabase for PooledDatabase {
    fn database_type(&self) -> crate::database::DatabaseType {
        self.inner.database_type()
    }

    async fn execute(&self, sql: &str) -> RiResult<u64> {
        self.inner.execute(sql).await
    }

    async fn query(&self, sql: &str) -> RiResult<RiDBResult> {
        self.inner.query(sql).await
    }

    async fn query_one(&self, sql: &str) -> RiResult<Option<RiDBRow>> {
        self.inner.query_one(sql).await
    }

    async fn ping(&self) -> RiResult<bool> {
        self.inner.ping().await
    }

    fn is_connected(&self) -> bool {
        self.inner.is_connected()
    }

    async fn close(&self) -> RiResult<()> {
        self.pool.close().await
    }

    async fn batch_execute(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<u64>> {
        self.inner.batch_execute(sql, params).await
    }

    async fn batch_query(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<RiDBResult>> {
        self.inner.batch_query(sql, params).await
    }

    async fn execute_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<u64> {
        self.inner.execute_with_params(sql, params).await
    }

    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<RiDBResult> {
        self.inner.query_with_params(sql, params).await
    }

    async fn transaction(&self) -> RiResult<Box<dyn crate::database::RiDatabaseTransaction>> {
        self.inner.transaction().await
    }
}

struct PoolConnection {
    db: Arc<dyn RiDatabase>,
    acquired_at: Instant,
    created_at: Instant,
}

struct LowUtilizationTracker {
    below_threshold_since: Option<Instant>,
    was_below_threshold: bool,
}

impl LowUtilizationTracker {
    fn new() -> Self {
        Self {
            below_threshold_since: None,
            was_below_threshold: false,
        }
    }

    fn update(&mut self, is_below_threshold: bool) {
        if is_below_threshold && !self.was_below_threshold {
            self.below_threshold_since = Some(Instant::now());
            self.was_below_threshold = true;
        } else if !is_below_threshold {
            self.below_threshold_since = None;
            self.was_below_threshold = false;
        }
    }

    fn duration_below_threshold(&self) -> Option<Duration> {
        self.below_threshold_since.map(|since| since.elapsed())
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiDatabasePool {
    config: RiDatabaseConfig,
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
    dynamic_config: Arc<RwLock<RiDynamicPoolConfig>>,
    low_utilization_tracker: Arc<RwLock<LowUtilizationTracker>>,
    scaling_in_progress: Arc<AtomicBool>,
    current_max_connections: Arc<AtomicU64>,
}

impl RiDatabasePool {
    pub async fn new(config: RiDatabaseConfig) -> RiResult<Self> {
        let dynamic_config = RiDynamicPoolConfig {
            enable_dynamic_scaling: true,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            scale_down_cooldown_secs: 300,
            min_connections: config.min_idle_connections,
            max_connections: config.max_connections,
            scale_up_step: 2,
            scale_down_step: 1,
        };

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
            dynamic_config: Arc::new(RwLock::new(dynamic_config)),
            low_utilization_tracker: Arc::new(RwLock::new(LowUtilizationTracker::new())),
            scaling_in_progress: Arc::new(AtomicBool::new(false)),
            current_max_connections: Arc::new(AtomicU64::new(config.max_connections as u64)),
        };

        for _ in 0..config.min_idle_connections {
            if let Ok(conn) = pool.create_connection().await {
                let id = pool.connection_ids.fetch_add(1, Ordering::SeqCst) as u32;
                let now = Instant::now();
                pool.available.insert(id, PoolConnection { 
                    db: conn, 
                    acquired_at: now,
                    created_at: now,
                });
                pool.idle_connections.fetch_add(1, Ordering::SeqCst);
                pool.total_connections.fetch_add(1, Ordering::SeqCst);
            }
        }

        Ok(pool)
    }

    async fn create_connection(&self) -> RiResult<Arc<dyn RiDatabase>> {
        match self.config.database_type {
            #[cfg(feature = "postgres")]
            crate::database::DatabaseType::Postgres => {
                let connection_string = self.config.connection_string();
                let db = crate::database::postgres::PostgresDatabase::new(&connection_string, self.config.clone()).await
                    .map_err(|e| crate::core::RiError::Config(e.to_string()))?;
                Ok(Arc::new(db) as Arc<dyn RiDatabase>)
            }
            #[cfg(feature = "mysql")]
            crate::database::DatabaseType::MySQL => {
                let connection_string = self.config.connection_string();
                let db = crate::database::mysql::MySQLDatabase::new(&connection_string, self.config.clone()).await
                    .map_err(|e| crate::core::RiError::Config(e.to_string()))?;
                Ok(Arc::new(db) as Arc<dyn RiDatabase>)
            }
            #[cfg(feature = "sqlite")]
            crate::database::DatabaseType::SQLite => {
                let url = format!("sqlite:{}", self.config.database);
                let db = tokio::runtime::Handle::current().block_on(
                    crate::database::sqlite::SQLiteDatabase::new(&url, self.config.clone())
                );
                match db {
                    Ok(db) => Ok(Arc::new(db) as Arc<dyn RiDatabase>),
                    Err(e) => Err(crate::core::RiError::Config(e.to_string())),
                }
            }
            _ => Err(crate::core::RiError::Config("Unsupported database type".to_string())),
        }
    }

    pub fn utilization_rate(&self) -> f64 {
        let active = self.active_connections.load(Ordering::SeqCst);
        let total = self.total_connections.load(Ordering::SeqCst);
        
        if total == 0 {
            return 0.0;
        }
        
        (active as f64) / (total as f64)
    }

    pub async fn check_and_scale(&self) -> RiResult<()> {
        let dynamic_config = handle_poisoned_lock(self.dynamic_config.read()).clone();
        
        if !dynamic_config.enable_dynamic_scaling {
            return Ok(());
        }

        if self.scaling_in_progress.compare_exchange(
            false,
            true,
            Ordering::SeqCst,
            Ordering::SeqCst,
        ).is_err() {
            return Ok(());
        }

        let result = self.do_scaling(&dynamic_config).await;
        
        self.scaling_in_progress.store(false, Ordering::SeqCst);
        
        result
    }

    async fn do_scaling(&self, dynamic_config: &RiDynamicPoolConfig) -> RiResult<()> {
        let utilization = self.utilization_rate();
        let total = self.total_connections.load(Ordering::SeqCst) as u32;
        let _active = self.active_connections.load(Ordering::SeqCst) as u32;
        
        if utilization > dynamic_config.scale_up_threshold {
            {
                let mut tracker = handle_poisoned_lock(self.low_utilization_tracker.write());
                tracker.update(false);
            }
            
            if total < dynamic_config.max_connections {
                let to_add = std::cmp::min(
                    dynamic_config.scale_up_step,
                    dynamic_config.max_connections - total,
                );
                
                for _ in 0..to_add {
                    match self.create_connection().await {
                        Ok(conn) => {
                            let id = self.connection_ids.fetch_add(1, Ordering::SeqCst) as u32;
                            let now = Instant::now();
                            self.available.insert(id, PoolConnection { 
                                db: conn, 
                                acquired_at: now,
                                created_at: now,
                            });
                            self.idle_connections.fetch_add(1, Ordering::SeqCst);
                            self.total_connections.fetch_add(1, Ordering::SeqCst);
                        }
                        Err(e) => {
                            self.errors.fetch_add(1, Ordering::SeqCst);
                            return Err(e);
                        }
                    }
                }
                
                let new_total = self.total_connections.load(Ordering::SeqCst);
                self.current_max_connections.store(new_total, Ordering::SeqCst);
            }
        } else if utilization < dynamic_config.scale_down_threshold {
            let should_scale_down = {
                let mut tracker = handle_poisoned_lock(self.low_utilization_tracker.write());
                tracker.update(true);
                
                if let Some(duration) = tracker.duration_below_threshold() {
                    duration >= Duration::from_secs(dynamic_config.scale_down_cooldown_secs)
                } else {
                    false
                }
            };
            
            if should_scale_down && total > dynamic_config.min_connections {
                let idle = self.idle_connections.load(Ordering::SeqCst) as u32;
                let to_remove = std::cmp::min(
                    std::cmp::min(dynamic_config.scale_down_step, idle),
                    total - dynamic_config.min_connections,
                );
                
                if to_remove > 0 {
                    self.remove_idle_connections(to_remove).await;
                }
            }
        } else {
            let mut tracker = handle_poisoned_lock(self.low_utilization_tracker.write());
            tracker.update(false);
        }
        
        Ok(())
    }

    async fn remove_idle_connections(&self, count: u32) {
        let mut removed = 0u32;
        let now = Instant::now();
        
        let to_remove: Vec<u32> = self.available
            .iter()
            .filter(|entry| {
                let conn = entry.value();
                now.duration_since(conn.acquired_at) > self.max_idle_time
            })
            .take(count as usize)
            .map(|entry| *entry.key())
            .collect();
        
        for id in to_remove {
            if removed >= count {
                break;
            }
            
            if let Some((_, conn)) = self.available.remove(&id) {
                let _ = conn.db.close().await;
                self.idle_connections.fetch_sub(1, Ordering::SeqCst);
                self.total_connections.fetch_sub(1, Ordering::SeqCst);
                removed += 1;
            }
        }
        
        if removed < count {
            let additional_remove: Vec<u32> = self.available
                .iter()
                .take((count - removed) as usize)
                .map(|entry| *entry.key())
                .collect();
            
            for id in additional_remove {
                if removed >= count {
                    break;
                }
                
                if let Some((_, conn)) = self.available.remove(&id) {
                    let _ = conn.db.close().await;
                    self.idle_connections.fetch_sub(1, Ordering::SeqCst);
                    self.total_connections.fetch_sub(1, Ordering::SeqCst);
                    removed += 1;
                }
            }
        }
        
        let new_total = self.total_connections.load(Ordering::SeqCst);
        self.current_max_connections.store(new_total, Ordering::SeqCst);
    }

    pub async fn get(&self) -> RiResult<PooledDatabase> {
        let _permit = self.semaphore.acquire().await.map_err(|e| crate::core::RiError::Config(e.to_string()))?;

        let mut reused_db = None;
        let mut reused_id = None;

        let now = Instant::now();
        
        for entry in self.available.iter() {
            let id = *entry.key();
            let conn = entry.value();
            if now.duration_since(conn.acquired_at) > self.max_idle_time || now.duration_since(conn.created_at) > self.max_lifetime {
                self.available.remove(&id);
                let _ = conn.db.close().await;
                self.idle_connections.fetch_sub(1, Ordering::SeqCst);
                self.total_connections.fetch_sub(1, Ordering::SeqCst);
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

        let _ = self.check_and_scale().await;

        Ok(PooledDatabase::new(id, db, Arc::new(self.clone())))
    }

    pub async fn release(&self, db: PooledDatabase) {
        self.active_connections.fetch_sub(1, Ordering::SeqCst);
        self.idle_connections.fetch_add(1, Ordering::SeqCst);
        
        self.available.insert(db.id(), PoolConnection { 
            db: db.inner.clone(),
            acquired_at: Instant::now(),
            created_at: Instant::now(),
        });

        let _ = self.check_and_scale().await;
    }

    pub async fn close(&self) -> RiResult<()> {
        self.semaphore.close();
        for entry in self.connections.iter() {
            let _ = entry.value().db.close().await;
        }
        self.connections.clear();
        self.available.clear();
        Ok(())
    }

    pub fn metrics(&self) -> RiDatabaseMetrics {
        let active = self.active_connections.load(Ordering::SeqCst);
        let total = self.total_connections.load(Ordering::SeqCst);
        
        RiDatabaseMetrics {
            active_connections: active,
            idle_connections: self.idle_connections.load(Ordering::SeqCst),
            total_connections: total,
            queries_executed: self.queries_executed.load(Ordering::SeqCst),
            query_duration_ms: 0.0,
            errors: self.errors.load(Ordering::SeqCst),
            utilization_rate: if total > 0 { (active as f64) / (total as f64) } else { 0.0 },
        }
    }

    pub fn get_dynamic_config(&self) -> RiDynamicPoolConfig {
        handle_poisoned_lock(self.dynamic_config.read()).clone()
    }

    pub fn set_dynamic_config(&self, config: RiDynamicPoolConfig) {
        let mut current = handle_poisoned_lock(self.dynamic_config.write());
        *current = config;
    }

    pub fn update_dynamic_config<F>(&self, f: F) 
    where
        F: FnOnce(&mut RiDynamicPoolConfig),
    {
        let mut config = handle_poisoned_lock(self.dynamic_config.write());
        f(&mut config);
    }

    pub fn is_scaling_in_progress(&self) -> bool {
        self.scaling_in_progress.load(Ordering::SeqCst)
    }

    pub fn get_current_max_connections(&self) -> u32 {
        self.current_max_connections.load(Ordering::SeqCst) as u32
    }

    pub async fn force_scale_up(&self, count: u32) -> RiResult<()> {
        let dynamic_config = handle_poisoned_lock(self.dynamic_config.read()).clone();
        let total = self.total_connections.load(Ordering::SeqCst) as u32;
        
        let to_add = std::cmp::min(count, dynamic_config.max_connections.saturating_sub(total));
        
        for _ in 0..to_add {
            match self.create_connection().await {
                Ok(conn) => {
                    let id = self.connection_ids.fetch_add(1, Ordering::SeqCst) as u32;
                    let now = Instant::now();
                    self.available.insert(id, PoolConnection { 
                        db: conn, 
                        acquired_at: now,
                        created_at: now,
                    });
                    self.idle_connections.fetch_add(1, Ordering::SeqCst);
                    self.total_connections.fetch_add(1, Ordering::SeqCst);
                }
                Err(e) => {
                    self.errors.fetch_add(1, Ordering::SeqCst);
                    return Err(e);
                }
            }
        }
        
        let new_total = self.total_connections.load(Ordering::SeqCst);
        self.current_max_connections.store(new_total, Ordering::SeqCst);
        
        Ok(())
    }

    pub async fn force_scale_down(&self, count: u32) -> RiResult<()> {
        let dynamic_config = handle_poisoned_lock(self.dynamic_config.read()).clone();
        let total = self.total_connections.load(Ordering::SeqCst) as u32;
        
        let to_remove = std::cmp::min(
            count,
            total.saturating_sub(dynamic_config.min_connections)
        );
        
        if to_remove > 0 {
            self.remove_idle_connections(to_remove).await;
        }
        
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiDatabasePool {
    #[new]
    fn py_new(config: RiDatabaseConfig) -> Self {
        let dynamic_config = RiDynamicPoolConfig {
            enable_dynamic_scaling: true,
            scale_up_threshold: 0.8,
            scale_down_threshold: 0.3,
            scale_down_cooldown_secs: 300,
            min_connections: config.min_idle_connections,
            max_connections: config.max_connections,
            scale_up_step: 2,
            scale_down_step: 1,
        };

        Self {
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
            dynamic_config: Arc::new(RwLock::new(dynamic_config)),
            low_utilization_tracker: Arc::new(RwLock::new(LowUtilizationTracker::new())),
            scaling_in_progress: Arc::new(AtomicBool::new(false)),
            current_max_connections: Arc::new(AtomicU64::new(config.max_connections as u64)),
        }
    }

    fn status(&self) -> String {
        format!(
            "Pool status - Active: {}, Idle: {}, Total: {}, Queries: {}, Errors: {}, Utilization: {:.2}%",
            self.active_connections.load(Ordering::SeqCst),
            self.idle_connections.load(Ordering::SeqCst),
            self.total_connections.load(Ordering::SeqCst),
            self.queries_executed.load(Ordering::SeqCst),
            self.errors.load(Ordering::SeqCst),
            self.utilization_rate() * 100.0
        )
    }

    fn get_config(&self) -> RiDatabaseConfig {
        self.config.clone()
    }

    fn get_utilization_rate(&self) -> f64 {
        self.utilization_rate()
    }

    fn get_metrics(&self) -> RiDatabaseMetrics {
        self.metrics()
    }

    fn get_dynamic_pool_config(&self) -> RiDynamicPoolConfig {
        self.get_dynamic_config()
    }

    fn set_dynamic_pool_config(&self, config: RiDynamicPoolConfig) {
        self.set_dynamic_config(config);
    }

    fn set_enable_dynamic_scaling(&self, enable: bool) {
        self.update_dynamic_config(|c| c.enable_dynamic_scaling = enable);
    }

    fn set_scale_up_threshold(&self, threshold: f64) {
        self.update_dynamic_config(|c| c.scale_up_threshold = threshold);
    }

    fn set_scale_down_threshold(&self, threshold: f64) {
        self.update_dynamic_config(|c| c.scale_down_threshold = threshold);
    }

    fn set_scale_down_cooldown_secs(&self, secs: u64) {
        self.update_dynamic_config(|c| c.scale_down_cooldown_secs = secs);
    }

    fn set_min_connections(&self, min: u32) {
        self.update_dynamic_config(|c| c.min_connections = min);
    }

    fn set_max_connections(&self, max: u32) {
        self.update_dynamic_config(|c| c.max_connections = max);
    }

    fn set_scale_up_step(&self, step: u32) {
        self.update_dynamic_config(|c| c.scale_up_step = step);
    }

    fn set_scale_down_step(&self, step: u32) {
        self.update_dynamic_config(|c| c.scale_down_step = step);
    }

    fn is_scaling(&self) -> bool {
        self.is_scaling_in_progress()
    }

    fn get_current_pool_size(&self) -> u32 {
        self.get_current_max_connections()
    }
}

impl Clone for RiDatabasePool {
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
            dynamic_config: self.dynamic_config.clone(),
            low_utilization_tracker: self.low_utilization_tracker.clone(),
            scaling_in_progress: self.scaling_in_progress.clone(),
            current_max_connections: self.current_max_connections.clone(),
        }
    }
}
