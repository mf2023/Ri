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

//! # Database Module
//!
//! This module provides a comprehensive database abstraction layer for DMSC,
//! supporting multiple database backends with a unified interface.
//!
//! ## Key Components
//!
//! - **DMSCDatabase**: Trait defining the database interface
//! - **DMSCDatabasePool**: Connection pool management
//! - **DMSCDatabaseConfig**: Database configuration
//! - **DMSCDBRow**: Row representation
//! - **DMSCDBResult**: Query result set
//!
//! ## Supported Databases
//!
//! - PostgreSQL (via sqlx or tokio-postgres)
//! - MySQL (via sqlx or tokio-mysql)
//! - SQLite (for embedded scenarios)
//!
//! ## Usage Example
//!
//! ```rust
//! use dms::database::{DMSCDatabase, DMSCDatabaseConfig, PooledDatabase};
//!
//! #[tokio::main]
//! async fn main() -> DMSCResult<()> {
//!     let config = DMSCDatabaseConfig::postgres()
//!         .host("localhost")
//!         .port(5432)
//!         .database("mydb")
//!         .user("user")
//!         .password("pass")
//!         .max_connections(10)
//!         .build();
//!
//!     let pool = DMSCDatabasePool::new(config).await?;
//!     let db = pool.get().await?;
//!
//!     let rows = db.query("SELECT * FROM users WHERE id = $1", &[&1]).await?;
//!     for row in rows {
//!         let id: i64 = row.get("id");
//!         let name: String = row.get("name");
//!         println!("User: {} - {}", id, name);
//!     }
//!
//!     Ok(())
//! }
//! ```

mod config;
mod pool;
mod row;
mod result;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub use config::{DMSCDatabaseConfig, DatabaseType};
pub use pool::{DMSCDatabasePool, PooledDatabase};
pub use row::DMSCDBRow;
pub use result::DMSCDBResult;

use crate::core::DMSCResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait DMSCDatabase: Send + Sync {
    fn database_type(&self) -> DatabaseType;

    async fn execute(&self, sql: &str) -> DMSCResult<u64>;

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult>;

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>>;

    async fn ping(&self) -> DMSCResult<bool>;

    fn is_connected(&self) -> bool;

    async fn close(&self) -> DMSCResult<()>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseMetrics {
    pub active_connections: u64,
    pub idle_connections: u64,
    pub total_connections: u64,
    pub queries_executed: u64,
    pub query_duration_ms: f64,
    pub errors: u64,
}

impl Default for DatabaseMetrics {
    fn default() -> Self {
        Self {
            active_connections: 0,
            idle_connections: 0,
            total_connections: 0,
            queries_executed: 0,
            query_duration_ms: 0.0,
            errors: 0,
        }
    }
}
