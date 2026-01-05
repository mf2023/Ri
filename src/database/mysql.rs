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
use mysql::Pool;
use std::sync::Arc;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct MySQLDatabase {
    pool: Pool,
    config: DMSCDatabaseConfig,
}

impl MySQLDatabase {
    pub fn new(pool: Pool, config: DMSCDatabaseConfig) -> Self {
        Self { pool, config }
    }
}

#[async_trait]
impl DMSCDatabase for MySQLDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::MySQL
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        self.pool
            .prep_exec(sql, ())
            .await
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(0)
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let mut conn = self.pool.get_conn().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let rows: Vec<DMSCDBRow> = conn
            .query_map(sql, |row: mysql::Row| {
                let mut dmsc_row = DMSCDBRow::new();
                let columns = row.columns_ref();
                for col in columns {
                    let name = col.name_str();
                    let value: Option<mysql::Value> = row.get(name.as_str());
                    let json = value_to_json(value);
                    dmsc_row.add_value(name.as_str(), json);
                }
                dmsc_row
            })
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(DMSCDBResult::with_rows(rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let mut conn = self.pool.get_conn().map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let result: Option<DMSCDBRow> = conn
            .query_first(sql)
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(result)
    }

    async fn ping(&self) -> DMSCResult<bool> {
        self.pool.test_conn().map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }

    fn is_connected(&self) -> bool {
        self.pool.is_active()
    }

    async fn close(&self) -> DMSCResult<()> {
        self.pool.disconnect().map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }
}

fn value_to_json(value: Option<mysql::Value>) -> serde_json::Value {
    match value {
        Some(mysql::Value::NULL) => serde_json::Value::Null,
        Some(mysql::Value::Bytes(v)) => serde_json::Value::String(hex::encode(v)),
        Some(mysql::Value::Int(v)) => serde_json::Value::Number(serde_json::Number::from(v)),
        Some(mysql::Value::UInt(v)) => serde_json::Value::Number(serde_json::Number::from(v)),
        Some(mysql::Value::Float(v)) => serde_json::Value::Number(serde_json::Number::from_f64(v as f64).unwrap_or_default()),
        Some(mysql::Value::Double(v)) => serde_json::Value::Number(serde_json::Number::from_f64(v).unwrap_or_default()),
        Some(mysql::Value::Date(y, m, d, h, min, s, _)) => {
            serde_json::Value::String(format!("{}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, h, min, s))
        }
        Some(mysql::Value::Time(is_neg, d, h, m, s, _)) => {
            let sign = if is_neg { "-" } else { "" };
            serde_json::Value::String(format!("{}{}:{}:{}:{}", sign, d, h, m, s))
        }
        None => serde_json::Value::Null,
    }
}
