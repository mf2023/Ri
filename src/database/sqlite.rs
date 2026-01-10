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

use async_trait::async_trait;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::{params, Row as SqliteRow, Statement};

use crate::core::{DMSCResult, DMSCError};
use crate::database::{
    DMSCDatabase, DMSCDatabaseConfig, DatabaseType,
    DMSCDBResult, DMSCDBRow
};

#[derive(Clone)]
pub struct SQLiteDatabase {
    path: String,
    pool: Arc<Mutex<Vec<rusqlite::Connection>>>,
    config: DMSCDatabaseConfig,
    max_pool_size: usize,
}

impl SQLiteDatabase {
    pub fn new(conn: rusqlite::Connection, config: DMSCDatabaseConfig) -> Self {
        let path = config.database.clone();
        let pool = Arc::new(Mutex::new(vec![conn]));
        let max_pool_size = config.max_connections as usize;

        Self {
            path,
            pool,
            config,
            max_pool_size,
        }
    }

    async fn get_connection(&self) -> Result<rusqlite::Connection, DMSCError> {
        let mut pool = self.pool.lock().await;

        if let Some(conn) = pool.pop() {
            return Ok(conn);
        }

        if pool.len() < self.max_pool_size {
            let conn = rusqlite::Connection::open(&self.path)
                .map_err(|e| DMSCError::Other(format!("SQLite connection error: {}", e)))?;

            conn.busy_timeout(std::time::Duration::from_secs(self.config.connection_timeout_secs))
                .map_err(|e| DMSCError::Other(format!("SQLite busy timeout error: {}", e)))?;

            return Ok(conn);
        }

        Err(DMSCError::PoolError("SQLite connection pool exhausted".to_string()))
    }

    async fn return_connection(&self, conn: rusqlite::Connection) {
        let mut pool = self.pool.lock().await;
        if pool.len() < self.max_pool_size {
            pool.push(conn);
        }
    }

    fn row_to_dmsc_row(row: &SqliteRow) -> DMSCDBRow {
        let columns: Vec<String> = (0..row.column_count())
            .map(|i| row.column_name(i).unwrap_or("").to_string())
            .collect();

        let values: Vec<Option<serde_json::Value>> = (0..row.column_count())
            .map(|idx| Self::value_to_json(row, idx))
            .collect();

        DMSCDBRow { columns, values }
    }

    fn value_to_json(row: &SqliteRow, idx: usize) -> Option<serde_json::Value> {
        let col_type = row.column_type(idx);

        match col_type {
            rusqlite::types::Type::Null => None,
            rusqlite::types::Type::Integer => {
                row.get::<_, i64>(idx).ok().map(serde_json::json)
            }
            rusqlite::types::Type::Real => {
                row.get::<_, f64>(idx).ok().map(serde_json::json)
            }
            rusqlite::types::Type::Text => {
                row.get::<_, String>(idx).ok().map(serde_json::json)
            }
            rusqlite::types::Type::Blob => {
                row.get::<_, Vec<u8>>(idx).ok().map(serde_json::json)
            }
        }
    }
}

#[async_trait]
impl DMSCDatabase for SQLiteDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::SQLite
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let conn = self.get_connection().await?;
        let result = conn.execute(sql, params![])
            .map_err(|e| DMSCError::Other(format!("SQLite execute error: {}", e)))?;

        self.return_connection(conn).await;
        Ok(result as u64)
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let conn = self.get_connection().await?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DMSCError::Other(format!("SQLite prepare error: {}", e)))?;

        let rows = stmt.query_map(params![], |row| Ok(Self::row_to_dmsc_row(row)))
            .map_err(|e| DMSCError::Other(format!("SQLite query error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in rows {
            dmsc_rows.push(row.map_err(|e| DMSCError::Other(format!("SQLite row error: {}", e)))?);
        }

        self.return_connection(conn).await;
        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let conn = self.get_connection().await?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DMSCError::Other(format!("SQLite prepare error: {}", e)))?;

        let rows: Vec<DMSCDBRow> = stmt.query_map(params![], |row| Ok(Self::row_to_dmsc_row(row)))
            .map_err(|e| DMSCError::Other(format!("SQLite query error: {}", e)))?
            .filter_map(|r| r.ok())
            .take(1)
            .collect();

        self.return_connection(conn).await;

        if rows.is_empty() {
            Ok(None)
        } else {
            Ok(Some(rows[0].clone()))
        }
    }

    async fn ping(&self) -> DMSCResult<bool> {
        let conn = self.get_connection().await?;
        let result = conn.query_row("SELECT 1", params![], |_| Ok(()));
        self.return_connection(conn).await;
        result.map(|_| true)
            .map_err(|e| DMSCError::Other(format!("SQLite ping error: {}", e)))
    }

    fn is_connected(&self) -> bool {
        Path::new(&self.path).exists()
    }

    async fn close(&self) -> DMSCResult<()> {
        let mut pool = self.pool.lock().await;
        pool.clear();
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl SQLiteDatabase {
    #[staticmethod]
    pub fn from_path(path: &str, max_connections: u32) -> Self {
        let conn = rusqlite::Connection::open(path)
            .expect("Failed to open SQLite database");

        conn.pragma_update(None, "journal_mode", "WAL")
            .expect("Failed to set WAL mode");

        conn.pragma_update(None, "synchronous", "NORMAL")
            .expect("Failed to set synchronous mode");

        conn.pragma_update(None, "busy_timeout", 30000)
            .expect("Failed to set busy timeout");

        let db_config = DMSCDatabaseConfig::sqlite(path)
            .max_connections(max_connections)
            .build();

        Self::new(conn, db_config)
    }

    pub fn execute_sync(&self, sql: &str) -> Result<u64, DMSCError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                self.execute(sql).await
            })
    }

    pub fn query_sync(&self, sql: &str) -> Result<DMSCDBResult, DMSCError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                self.query(sql).await
            })
    }

    pub fn ping_sync(&self) -> Result<bool, DMSCError> {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                self.ping().await
            })
    }
}
