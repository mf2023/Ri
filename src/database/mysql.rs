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
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl MySQLDatabase {
    #[staticmethod]
    pub fn from_connection_string(conn_string: &str, max_connections: u32) -> Self {
        let opts = mysql_crate::Opts::from_url(conn_string)
            .unwrap_or_else(|_| mysql_crate::Opts::from_url("mysql://localhost:3306").unwrap());

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
