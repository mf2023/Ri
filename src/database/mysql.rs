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

use async_trait::async_trait;
use sqlx::mysql::{MySqlPool, MySqlRow};
use sqlx::{Row, Column};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::core::{RiResult, RiError};
use crate::database::{
    RiDatabase, RiDatabaseConfig, DatabaseType,
    RiDBResult, RiDBRow
};

#[derive(Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[allow(dead_code)]
pub struct MySQLDatabase {
    pool: MySqlPool,
    config: RiDatabaseConfig,
}

impl MySQLDatabase {
    pub async fn new(database_url: &str, config: RiDatabaseConfig) -> Result<Self, RiError> {
        let pool = MySqlPool::connect(database_url)
            .await
            .map_err(|e| RiError::Other(format!("Failed to connect to MySQL: {}", e)))?;
        
        Ok(Self { pool, config })
    }

    fn row_to_ri_row(row: &MySqlRow) -> RiDBRow {
        let columns: Vec<String> = (0..row.len())
            .map(|i| row.column(i).name().to_string())
            .collect();

        let values: Vec<Option<serde_json::Value>> = (0..row.len())
            .map(|idx| Self::value_to_json(row, idx))
            .collect();

        RiDBRow { columns, values }
    }

    fn value_to_json(row: &MySqlRow, idx: usize) -> Option<serde_json::Value> {
        match row.try_get::<i64, _>(idx) {
            Ok(v) => Some(serde_json::json!(v)),
            Err(_) => {
                match row.try_get::<f64, _>(idx) {
                    Ok(v) => Some(serde_json::json!(v)),
                    Err(_) => {
                        match row.try_get::<String, _>(idx) {
                            Ok(v) => Some(serde_json::json!(v)),
                            Err(_) => {
                                match row.try_get::<bool, _>(idx) {
                                    Ok(v) => Some(serde_json::json!(v)),
                                    Err(_) => {
                                        match row.try_get::<Vec<u8>, _>(idx) {
                                            Ok(v) => Some(serde_json::json!(v)),
                                            Err(_) => None,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[async_trait]
impl RiDatabase for MySQLDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::MySQL
    }

    async fn execute(&self, sql: &str) -> RiResult<u64> {
        let result = sqlx::query::<sqlx::MySql>(sql)
            .execute(&self.pool)
            .await
            .map_err(|e| RiError::Other(format!("MySQL execute error: {}", e)))?;
        Ok(result.rows_affected())
    }

    async fn query(&self, sql: &str) -> RiResult<RiDBResult> {
        let rows = sqlx::query::<sqlx::MySql>(sql)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RiError::Other(format!("MySQL query error: {}", e)))?;

        let ri_rows: Vec<RiDBRow> = rows.iter()
            .map(|row| Self::row_to_ri_row(row))
            .collect();

        Ok(RiDBResult::with_rows(ri_rows))
    }

    async fn query_one(&self, sql: &str) -> RiResult<Option<RiDBRow>> {
        let row = sqlx::query::<sqlx::MySql>(sql)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RiError::Other(format!("MySQL query_one error: {}", e)))?;

        Ok(row.map(|r| Self::row_to_ri_row(&r)))
    }

    async fn ping(&self) -> RiResult<bool> {
        sqlx::query::<sqlx::MySql>("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map(|_| true)
            .map_err(|e| RiError::Other(format!("MySQL ping error: {}", e)))
    }

    fn is_connected(&self) -> bool {
        !self.pool.is_closed()
    }

    async fn close(&self) -> RiResult<()> {
        self.pool.close().await;
        Ok(())
    }

    async fn batch_execute(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<u64>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.execute_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn batch_query(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<RiDBResult>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.query_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    async fn execute_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<u64> {
        let mut query = sqlx::query::<sqlx::MySql>(sql);
        
        for param in params {
            let param_str = param.to_string();
            query = query.bind(param_str);
        }
        
        let result = query
            .execute(&self.pool)
            .await
            .map_err(|e| RiError::Other(format!("MySQL execute_with_params error: {}", e)))?;
        Ok(result.rows_affected())
    }

    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<RiDBResult> {
        let mut query = sqlx::query::<sqlx::MySql>(sql);
        
        for param in params {
            let param_str = param.to_string();
            query = query.bind(param_str);
        }
        
        let rows = query
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RiError::Other(format!("MySQL query_with_params error: {}", e)))?;

        let ri_rows: Vec<RiDBRow> = rows.iter()
            .map(|row| Self::row_to_ri_row(row))
            .collect();

        Ok(RiDBResult::with_rows(ri_rows))
    }

    async fn transaction(&self) -> RiResult<Box<dyn crate::database::RiDatabaseTransaction>> {
        let tx = self.pool.begin().await
            .map_err(|e| RiError::Other(format!("MySQL transaction begin error: {}", e)))?;

        Ok(Box::new(MySQLTransaction::new(tx)))
    }
}

struct MySQLTransaction {
    tx: Arc<Mutex<Option<sqlx::Transaction<'static, sqlx::MySql>>>>,
}

impl MySQLTransaction {
    pub fn new(tx: sqlx::Transaction<'static, sqlx::MySql>) -> Self {
        Self {
            tx: Arc::new(Mutex::new(Some(tx))),
        }
    }
}

#[async_trait::async_trait]
impl crate::database::RiDatabaseTransaction for MySQLTransaction {
    async fn execute(&self, sql: &str) -> RiResult<u64> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut()
            .ok_or_else(|| RiError::Other("MySQL transaction already closed".to_string()))?;
        
        let result = sqlx::query::<sqlx::MySql>(sql)
            .execute(&mut **tx)
            .await
            .map_err(|e| RiError::Other(format!("MySQL transaction execute error: {}", e)))?;
        Ok(result.rows_affected())
    }

    async fn query(&self, sql: &str) -> RiResult<RiDBResult> {
        let mut guard = self.tx.lock().await;
        let tx = guard.as_mut()
            .ok_or_else(|| RiError::Other("MySQL transaction already closed".to_string()))?;
        
        let rows = sqlx::query::<sqlx::MySql>(sql)
            .fetch_all(&mut **tx)
            .await
            .map_err(|e| RiError::Other(format!("MySQL transaction query error: {}", e)))?;

        let ri_rows: Vec<RiDBRow> = rows.iter()
            .map(|row| MySQLDatabase::row_to_ri_row(row))
            .collect();

        Ok(RiDBResult::with_rows(ri_rows))
    }

    async fn commit(&self) -> RiResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.take()
            .ok_or_else(|| RiError::Other("MySQL transaction already closed".to_string()))?;
        
        tx.commit().await
            .map_err(|e| RiError::Other(format!("MySQL transaction commit error: {}", e)))
    }

    async fn rollback(&self) -> RiResult<()> {
        let mut guard = self.tx.lock().await;
        let tx = guard.take()
            .ok_or_else(|| RiError::Other("MySQL transaction already closed".to_string()))?;
        
        tx.rollback().await
            .map_err(|e| RiError::Other(format!("MySQL transaction rollback error: {}", e)))
    }

    async fn close(&self) -> RiResult<()> {
        self.rollback().await
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl MySQLDatabase {
    #[staticmethod]
    pub fn from_connection_string(conn_string: &str, max_connections: u32) -> Result<Self, pyo3::PyErr> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to create Tokio runtime: {}", e),
            ))?;
        
        rt.block_on(async {
            let pool = MySqlPool::connect(conn_string)
                .await
                .map_err(|e| pyo3::PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to connect to MySQL: {}", e),
                ))?;

            let db_config = RiDatabaseConfig::mysql()
                .host("localhost")
                .port(3306)
                .database("mysql")
                .max_connections(max_connections)
                .min_idle_connections(1)
                .build();

            Ok(Self { pool, config: db_config })
        })
    }

    pub fn execute_sync(&self, sql: &str) -> Result<u64, RiError> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RiError::Other(format!("Failed to create Tokio runtime: {}", e)))?;
        rt.block_on(async {
            self.execute(sql).await
        })
    }

    pub fn query_sync(&self, sql: &str) -> Result<RiDBResult, RiError> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RiError::Other(format!("Failed to create Tokio runtime: {}", e)))?;
        rt.block_on(async {
            self.query(sql).await
        })
    }

    pub fn ping_sync(&self) -> Result<bool, RiError> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| RiError::Other(format!("Failed to create Tokio runtime: {}", e)))?;
        rt.block_on(async {
            self.ping().await
        })
    }
}
