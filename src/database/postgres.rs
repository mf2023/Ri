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
use tokio_postgres::types::Type;
use tokio_postgres::Row as PgRow;

use crate::core::{DMSCResult, DMSCError};
use crate::database::{
    DMSCDatabase, DMSCDatabaseConfig, DatabaseType,
    DMSCDBResult, DMSCDBRow
};

#[derive(Clone)]
pub struct PostgresDatabase {
    client: tokio_postgres::Client,
    config: DMSCDatabaseConfig,
}

impl PostgresDatabase {
    pub fn new(client: tokio_postgres::Client, config: DMSCDatabaseConfig) -> Self {
        Self { client, config }
    }

    fn row_to_dmsc_row(row: &PgRow) -> DMSCDBRow {
        let columns: Vec<String> = row.columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect();

        let values: Vec<Option<serde_json::Value>> = row.columns()
            .iter()
            .enumerate()
            .map(|(idx, col)| {
                let value: Option<tokio_postgres::types::Json<serde_json::Value>> = row.try_get(idx).ok();
                match value {
                    Some(tokio_postgres::types::Json(json)) => json,
                    None => {
                        if row.try_get::<usize, tokio_postgres::types::Uuid>(idx).is_ok() {
                            serde_json::to_value(row.try_get::<usize, tokio_postgres::types::Uuid>(idx).unwrap()).ok()
                        } else if let Ok(v) = row.try_get::<usize, i32>(idx) {
                            serde_json::json!(v)
                        } else if let Ok(v) = row.try_get::<usize, i64>(idx) {
                            serde_json::json!(v)
                        } else if let Ok(v) = row.try_get::<usize, f64>(idx) {
                            serde_json::json!(v)
                        } else if let Ok(v) = row.try_get::<usize, bool>(idx) {
                            serde_json::json!(v)
                        } else if let Ok(v) = row.try_get::<usize, String>(idx) {
                            serde_json::json!(v)
                        } else if let Ok(v) = row.try_get::<usize, &[u8]>(idx) {
                            serde_json::json!(v)
                        } else {
                            serde_json::json!(null)
                        }
                    }
                }
            })
            .collect();

        DMSCDBRow { columns, values }
    }

    fn row_to_json_value(row: &PgRow, idx: usize, col: &tokio_postgres::Column) -> Option<serde_json::Value> {
        let type_oid = col.type_oid();

        if row.is_null(idx) {
            return None;
        }

        match type_oid {
            Type::BOOL_OID => row.try_get::<usize, bool>(idx).ok().map(serde_json::json),
            Type::INT2_OID => row.try_get::<usize, i16>(idx).ok().map(serde_json::json),
            Type::INT4_OID => row.try_get::<usize, i32>(idx).ok().map(serde_json::json),
            Type::INT8_OID => row.try_get::<usize, i64>(idx).ok().map(serde_json::json),
            Type::FLOAT4_OID => row.try_get::<usize, f32>(idx).ok().map(serde_json::json),
            Type::FLOAT8_OID => row.try_get::<usize, f64>(idx).ok().map(serde_json::json),
            Type::VARCHAR_OID | Type::TEXT_OID | Type::BPCHAR_OID => {
                row.try_get::<usize, String>(idx).ok().map(serde_json::json)
            }
            Type::BYTEA_OID => row.try_get::<usize, Vec<u8>>(idx).ok().map(serde_json::json),
            Type::TIMESTAMP_OID | Type::TIMESTAMPTZ_OID => {
                row.try_get::<usize, chrono::DateTime<chrono::Utc>>(idx)
                    .ok()
                    .map(|dt| serde_json::json!(dt.to_rfc3339()))
            }
            Type::DATE_OID => {
                row.try_get::<usize, chrono::NaiveDate>(idx)
                    .ok()
                    .map(|d| serde_json::json!(d.to_string()))
            }
            Type::JSON_OID | Type::JSONB_OID => {
                row.try_get::<usize, serde_json::Value>(idx).ok()
            }
            Type::UUID_OID => row.try_get::<usize, uuid::Uuid>(idx).ok().map(serde_json::json),
            _ => {
                if let Ok(v) = row.try_get::<usize, String>(idx) {
                    serde_json::json!(v)
                } else {
                    serde_json::json!(null)
                }
            }
        }
    }
}

#[async_trait]
impl DMSCDatabase for PostgresDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::Postgres
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let result = self.client.execute(sql, &[]).await
            .map_err(|e| DMSCError::Other(format!("PostgreSQL execute error: {}", e)))?;
        Ok(result as u64)
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let rows = self.client.query(sql, &[]).await
            .map_err(|e| DMSCError::Other(format!("PostgreSQL query error: {}", e)))?;

        let dmsc_rows: Vec<DMSCDBRow> = rows.iter()
            .map(|row| {
                let columns: Vec<String> = row.columns()
                    .iter()
                    .map(|col| col.name().to_string())
                    .collect();

                let values: Vec<Option<serde_json::Value>> = row.columns()
                    .iter()
                    .enumerate()
                    .map(|(idx, col)| Self::row_to_json_value(&row, idx, col))
                    .collect();

                DMSCDBRow { columns, values }
            })
            .collect();

        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let rows = self.client.query(sql, &[]).await
            .map_err(|e| DMSCError::Other(format!("PostgreSQL query error: {}", e)))?;

        if rows.is_empty() {
            return Ok(None);
        }

        let row = &rows[0];
        let columns: Vec<String> = row.columns()
            .iter()
            .map(|col| col.name().to_string())
            .collect();

        let values: Vec<Option<serde_json::Value>> = row.columns()
            .iter()
            .enumerate()
            .map(|(idx, col)| Self::row_to_json_value(&row, idx, col))
            .collect();

        Ok(Some(DMSCDBRow { columns, values }))
    }

    async fn ping(&self) -> DMSCResult<bool> {
        self.client.query("SELECT 1", &[]).await
            .map(|_| true)
            .map_err(|e| DMSCError::Other(format!("PostgreSQL ping error: {}", e)))
    }

    fn is_connected(&self) -> bool {
        !self.client.is_closed()
    }

    async fn close(&self) -> DMSCResult<()> {
        self.client.close().await
            .map_err(|e| DMSCError::Other(format!("PostgreSQL close error: {}", e)))
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl PostgresDatabase {
    #[staticmethod]
    pub fn from_connection_string(conn_string: &str, max_connections: u32) -> Self {
        let config = tokio_postgres::Config::from(conn_string);
        let (client, conn) = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async {
                config.connect(tokio_postgres::NoTls).await
            })
            .unwrap();

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("PostgreSQL connection error: {}", e);
            }
        });

        let db_config = DMSCDatabaseConfig::postgres()
            .max_connections(max_connections)
            .build();

        Self::new(client, db_config)
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
