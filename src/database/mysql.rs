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
use mysql as mysql_crate;

use crate::core::{DMSCResult, DMSCError};
use crate::database::{
    DMSCDatabase, DMSCDatabaseConfig, DatabaseType,
    DMSCDBResult, DMSCDBRow
};

#[derive(Clone)]
pub struct MySQLDatabase {
    pool: mysql_crate::Pool,
    config: DMSCDatabaseConfig,
}

impl MySQLDatabase {
    pub fn new(pool: mysql_crate::Pool, config: DMSCDatabaseConfig) -> Self {
        Self { pool, config }
    }

    fn row_to_dmsc_row(row: &mysql_crate::Row) -> DMSCDBRow {
        let columns: Vec<String> = row.columns().iter()
            .map(|col| col.name_str().to_string())
            .collect();

        let values: Vec<Option<serde_json::Value>> = row.columns().iter()
            .enumerate()
            .map(|(idx, col)| {
                let value: Option<mysql_crate::Value> = row.get(idx);
                Self::value_to_json(value, col.column_type())
            })
            .collect();

        DMSCDBRow { columns, values }
    }

    fn value_to_json(value: Option<mysql_crate::Value>, col_type: mysql_crate::ColumnType) -> Option<serde_json::Value> {
        match value {
            None => None,
            Some(mysql_crate::Value::NULL) => None,
            Some(mysql_crate::Value::Bytes(v)) => Some(serde_json::json!(String::from_utf8_lossy(&v).to_string())),
            Some(mysql_crate::Value::Int(v)) => Some(serde_json::json!(v)),
            Some(mysql_crate::Value::UInt(v)) => Some(serde_json::json!(v)),
            Some(mysql_crate::Value::Float(f)) => Some(serde_json::json!(f)),
            Some(mysql_crate::Value::Double(d)) => Some(serde_json::json!(d)),
            Some(mysql_crate::Value::Date(y, m, d, hh, mm, ss, frac)) => {
                Some(serde_json::json!(format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                    y, m, d, hh, mm, ss, frac)))
            }
            Some(mysql_crate::Value::Time(neg, d, h, m, s, frac)) => {
                let sign = if neg { "-" } else { "" };
                Some(serde_json::json!(format!("{} {:02}:{:02}:{:02}.{:06}",
                    sign, h + d * 24, m, s, frac)))
            }
        }
    }
}

#[async_trait]
impl DMSCDatabase for MySQLDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::MySQL
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let result = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL connection error: {}", e)))?
            .query_drop(sql)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL execute error: {}", e)))?;

        Ok(result.affected_rows())
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let conn = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL connection error: {}", e)))?;

        let result = conn.query_iter(sql)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL query error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in result {
            let mysql_row = row.map_err(|e| DMSCError::Other(format!("MySQL row error: {}", e)))?;
            dmsc_rows.push(Self::row_to_dmsc_row(&mysql_row));
        }

        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let conn = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL connection error: {}", e)))?;

        let result = conn.query_iter(sql)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL query error: {}", e)))?;

        if let Some(row) = result.next().transpose()? {
            Ok(Some(Self::row_to_dmsc_row(&row)))
        } else {
            Ok(None)
        }
    }

    async fn ping(&self) -> DMSCResult<bool> {
        self.pool.get_conn()
            .await
            .map(|_| true)
            .map_err(|e| DMSCError::Other(format!("MySQL ping error: {}", e)))
    }

    fn is_connected(&self) -> bool {
        !self.pool.is_closed()
    }

    async fn close(&self) -> DMSCResult<()> {
        drop(self.pool);
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
        let conn = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL connection error: {}", e)))?;

        let mysql_params: Vec<mysql_crate::Value> = params.iter()
            .map(|v| {
                match v {
                    serde_json::Value::Null => mysql_crate::Value::NULL,
                    serde_json::Value::Bool(b) => mysql_crate::Value::Int(*b as i64),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            mysql_crate::Value::Int(i)
                        } else if let Some(f) = n.as_f64() {
                            mysql_crate::Value::Float(f)
                        } else {
                            mysql_crate::Value::NULL
                        }
                    }
                    serde_json::Value::String(s) => mysql_crate::Value::Bytes(s.as_bytes().to_vec()),
                    _ => mysql_crate::Value::Bytes(serde_json::to_string(v).unwrap_or_default().into_bytes()),
                }
            })
            .collect();

        let result = conn.exec_drop(sql, mysql_params)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL execute_with_params error: {}", e)))?;

        Ok(result.affected_rows())
    }

    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> DMSCResult<DMSCDBResult> {
        let conn = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL connection error: {}", e)))?;

        let mysql_params: Vec<mysql_crate::Value> = params.iter()
            .map(|v| {
                match v {
                    serde_json::Value::Null => mysql_crate::Value::NULL,
                    serde_json::Value::Bool(b) => mysql_crate::Value::Int(*b as i64),
                    serde_json::Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            mysql_crate::Value::Int(i)
                        } else if let Some(f) = n.as_f64() {
                            mysql_crate::Value::Float(f)
                        } else {
                            mysql_crate::Value::NULL
                        }
                    }
                    serde_json::Value::String(s) => mysql_crate::Value::Bytes(s.as_bytes().to_vec()),
                    _ => mysql_crate::Value::Bytes(serde_json::to_string(v).unwrap_or_default().into_bytes()),
                }
            })
            .collect();

        let result = conn.exec_iter(sql, mysql_params)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL query_with_params error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in result {
            let mysql_row = row.map_err(|e| DMSCError::Other(format!("MySQL row error: {}", e)))?;
            dmsc_rows.push(Self::row_to_dmsc_row(&mysql_row));
        }

        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn transaction(&self) -> DMSCResult<Box<dyn crate::database::DMSCDatabaseTransaction>> {
        let conn = self.pool.get_conn()
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction begin error: {}", e)))?;

        conn.cmd_query("START TRANSACTION").await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction begin error: {}", e)))?;

        Ok(Box::new(MySQLTransaction { conn }))
    }
}

struct MySQLTransaction {
    conn: mysql_crate::pool::PooledConn,
}

#[async_trait::async_trait]
impl crate::database::DMSCDatabaseTransaction for MySQLTransaction {
    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        let result = self.conn.query_drop(sql)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction execute error: {}", e)))?;
        Ok(result.affected_rows())
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let result = self.conn.query_iter(sql)
            .await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction query error: {}", e)))?;

        let mut dmsc_rows = Vec::new();
        for row in result {
            let mysql_row = row.map_err(|e| DMSCError::Other(format!("MySQL transaction row error: {}", e)))?;
            dmsc_rows.push(MySQLDatabase::row_to_dmsc_row(&mysql_row));
        }

        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn commit(&self) -> DMSCResult<()> {
        self.conn.cmd_query("COMMIT").await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction commit error: {}", e)))
    }

    async fn rollback(&self) -> DMSCResult<()> {
        self.conn.cmd_query("ROLLBACK").await
            .map_err(|e| DMSCError::Other(format!("MySQL transaction rollback error: {}", e)))
    }

    async fn close(&self) -> DMSCResult<()> {
        self.rollback().await
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl MySQLDatabase {
    #[staticmethod]
    pub fn from_connection_string(conn_string: &str, max_connections: u32) -> Self {
        let opts = match mysql_crate::Opts::from_url(conn_string) {
            Ok(o) => o,
            Err(_) => {
                mysql_crate::Opts::from_url("mysql://localhost:3306")
                    .unwrap_or_else(|_| {
                        mysql_crate::OptsBuilder::new()
                            .ip_or_hostname("localhost")
                            .tcp_port(3306)
                            .user(None)
                            .pass(None)
                            .db_name(None)
                            .clone()
                    })
            }
        };

        let pool_opts = mysql_crate::PoolOpts::default()
            .with_conn_idle_timeout(std::time::Duration::from_secs(600))
            .with_max_idle_connections(max_connections as u16);

        let pool = mysql_crate::Pool::new_opts(opts, pool_opts);

        let db_config = DMSCDatabaseConfig::mysql()
            .max_connections(max_connections)
            .build();

        Self::new(pool, db_config)
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
