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

    async fn batch_execute(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> DMSCResult<Vec<u64>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.execute_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn batch_query(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> DMSCResult<Vec<DMSCDBResult>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.query_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn execute_with_params(&self, sql: &str, params: &[serde_json::Value]) -> DMSCResult<u64> {
        let conn = self.get_connection().await?;
        let sqlite_params: Vec<&dyn rusqlite::types::ToSql> = params.iter()
            .map(|v| v as &dyn rusqlite::types::ToSql)
            .collect();

        let result = conn.execute(sql, rusqlite::params_from_iter(sqlite_params))
            .map_err(|e| DMSCError::Other(format!("SQLite execute_with_params error: {}", e)))?;

        self.return_connection(conn).await;
        Ok(result as u64)
    }

    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> DMSCResult<DMSCDBResult> {
        let conn = self.get_connection().await?;
        let sqlite_params: Vec<&dyn rusqlite::types::ToSql> = params.iter()
            .map(|v| v as &dyn rusqlite::types::ToSql)
            .collect();

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DMSCError::Other(format!("SQLite query_with_params prepare error: {}", e)))?;

        let rows = stmt.query_map(rusqlite::params_from_iter(sqlite_params), |row| Ok(Self::row_to_dmsc_row(row)))
            .map_err(|e| DMSCError::Other(format!("SQLite query_with_params error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in rows {
            dmsc_rows.push(row.map_err(|e| DMSCError::Other(format!("SQLite row error: {}", e)))?);
        }

        self.return_connection(conn).await;
        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn transaction(&self) -> DMSCResult<Box<dyn crate::database::DMSCDatabaseTransaction>> {
        let conn = self.get_connection().await?;
        conn.execute("BEGIN TRANSACTION", params![])
            .map_err(|e| DMSCError::Other(format!("SQLite transaction begin error: {}", e)))?;

        Ok(Box::new(SQLiteTransaction { conn: Some(conn), committed: false }))
    }
}

struct SQLiteTransaction {
    conn: Option<rusqlite::Connection>,
    committed: bool,
}

#[async_trait::async_trait]
impl crate::database::DMSCDatabaseTransaction for SQLiteTransaction {
    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let conn = self.conn.as_ref()
            .ok_or_else(|| DMSCError::Other("SQLite transaction already closed".to_string()))?;

        let result = conn.execute(sql, params![])
            .map_err(|e| DMSCError::Other(format!("SQLite transaction execute error: {}", e)))?;
        Ok(result as u64)
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let conn = self.conn.as_ref()
            .ok_or_else(|| DMSCError::Other("SQLite transaction already closed".to_string()))?;

        let mut stmt = conn.prepare(sql)
            .map_err(|e| DMSCError::Other(format!("SQLite transaction query prepare error: {}", e)))?;

        let rows = stmt.query_map(params![], |row| Ok(SQLiteDatabase::row_to_dmsc_row(row)))
            .map_err(|e| DMSCError::Other(format!("SQLite transaction query error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in rows {
            dmsc_rows.push(row.map_err(|e| DMSCError::Other(format!("SQLite transaction row error: {}", e)))?);
        }

        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn commit(&self) -> DMSCResult<()> {
        let conn = self.conn.take()
            .ok_or_else(|| DMSCError::Other("SQLite transaction already closed".to_string()))?;

        conn.execute("COMMIT", params![])
            .map_err(|e| DMSCError::Other(format!("SQLite transaction commit error: {}", e)))?;

        Ok(())
    }

    async fn rollback(&self) -> DMSCResult<()> {
        let conn = self.conn.take()
            .ok_or_else(|| DMSCError::Other("SQLite transaction already closed".to_string()))?;

        conn.execute("ROLLBACK", params![])
            .map_err(|e| DMSCError::Other(format!("SQLite transaction rollback error: {}", e)))?;

        Ok(())
    }

    async fn close(&self) -> DMSCResult<()> {
        if !self.committed {
            self.rollback().await?;
        }
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl SQLiteDatabase {
    #[staticmethod]
    pub fn from_path(path: &str, max_connections: u32) -> Result<Self, pyo3::PyErr> {
        let conn = match rusqlite::Connection::open(path) {
            Ok(c) => c,
            Err(e) => {
                return Err(pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to open SQLite database: {}", e),
                ));
            }
        };

        if let Err(e) = conn.pragma_update(None, "journal_mode", "WAL") {
            log::warn!("Failed to set WAL mode: {}", e);
        }

        if let Err(e) = conn.pragma_update(None, "synchronous", "NORMAL") {
            log::warn!("Failed to set synchronous mode: {}", e);
        }

        if let Err(e) = conn.pragma_update(None, "busy_timeout", 30000) {
            log::warn!("Failed to set busy timeout: {}", e);
        }

        let db_config = DMSCDatabaseConfig::sqlite(path)
            .max_connections(max_connections)
            .build();

        Ok(Self::new(conn, db_config))
    }

    pub fn execute_sync(&self, sql: &str) -> Result<u64, DMSCError> {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => return Err(DMSCError::Other(format!("Failed to create Tokio runtime: {}", e))),
        };
        rt.block_on(async {
            self.execute(sql).await
        })
    }

    pub fn query_sync(&self, sql: &str) -> Result<DMSCDBResult, DMSCError> {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => return Err(DMSCError::Other(format!("Failed to create Tokio runtime: {}", e))),
        };
        rt.block_on(async {
            self.query(sql).await
        })
    }

    pub fn ping_sync(&self) -> Result<bool, DMSCError> {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(e) => return Err(DMSCError::Other(format!("Failed to create Tokio runtime: {}", e))),
        };
        rt.block_on(async {
            self.ping().await
        })
    }
}
