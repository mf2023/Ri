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
use crate::database::{DMSCDatabaseConfig, DMSCDBResult, DMSCDBRow, DatabaseType};
use async_trait::async_trait;
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct SQLiteDatabase {
    conn: Arc<Mutex<Connection>>,
    config: DMSCDatabaseConfig,
}

impl SQLiteDatabase {
    pub fn new(conn: Connection, config: DMSCDatabaseConfig) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
            config,
        }
    }

    pub fn open(path: &str, config: DMSCDatabaseConfig) -> DMSCResult<Self> {
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)
                    .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
            }
        }
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
        )
        .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        conn.busy_timeout(std::time::Duration::from_secs(30))
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(Self::new(conn, config))
    }
}

#[async_trait]
impl DMSCDatabase for SQLiteDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::SQLite
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let conn = self.conn.lock().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let affected = conn.execute(sql, []).map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(affected as u64)
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let conn = self.conn.lock().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let mut stmt = conn.prepare(sql).map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let column_count = stmt.column_count();
        let mut rows = Vec::new();
        
        let mut row_mapper = |row: &rusqlite::Row| -> Result<DMSCDBRow, rusqlite::Error> {
            let mut dmsc_row = DMSCDBRow::new();
            for i in 0..column_count {
                let name = row.column_name(i).unwrap_or("");
                let value: Option<rusqlite::Value> = row.get(i);
                let json = self.value_to_json(value);
                dmsc_row.add_value(name, json);
            }
            Ok(dmsc_row)
        };

        let sqlite_rows = stmt.query_and_then([], row_mapper)
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
            
        for row in sqlite_rows {
            rows.push(row.map_err(|e| crate::core::DMSCError::Config(e.to_string()))?);
        }
        
        Ok(DMSCDBResult::with_rows(rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let conn = self.conn.lock().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let mut stmt = conn.prepare(sql).map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let column_count = stmt.column_count();
        
        let mut rows = stmt.query_and_then([], |row: &rusqlite::Row| -> Result<DMSCDBRow, rusqlite::Error> {
            let mut dmsc_row = DMSCDBRow::new();
            for i in 0..column_count {
                let name = row.column_name(i).unwrap_or("");
                let value: Option<rusqlite::Value> = row.get(i);
                let json = self.value_to_json(value);
                dmsc_row.add_value(name, json);
            }
            Ok(dmsc_row)
        }).map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        
        if let Some(Ok(row)) = rows.next() {
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    async fn ping(&self) -> DMSCResult<bool> {
        let conn = self.conn.lock().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        conn.execute("SELECT 1", []).map(|_| true).map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }

    fn is_connected(&self) -> bool {
        self.conn.lock().is_ok()
    }

    async fn close(&self) -> DMSCResult<()> {
        let conn = self.conn.lock().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        conn.close();
        Ok(())
    }
}

impl SQLiteDatabase {
    fn value_to_json(&self, value: Option<rusqlite::Value>) -> serde_json::Value {
        match value {
            Some(rusqlite::Value::Null) => serde_json::Value::Null,
            Some(rusqlite::Value::Integer(v)) => serde_json::Value::Number(serde_json::Number::from(v)),
            Some(rusqlite::Value::Real(v)) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap_or_default()),
            Some(rusqlite::Value::Text(v)) => serde_json::Value::String(v),
            Some(rusqlite::Value::Blob(v)) => serde_json::Value::String(hex::encode(v)),
            None => serde_json::Value::Null,
        }
    }
}
