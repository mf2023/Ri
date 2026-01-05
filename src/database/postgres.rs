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
use std::sync::Arc;
use tokio_postgres::Client;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct PostgresDatabase {
    client: Client,
    config: DMSCDatabaseConfig,
}

impl PostgresDatabase {
    pub fn new(client: Client, config: DMSCDatabaseConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl DMSCDatabase for PostgresDatabase {
    fn database_type(&self) -> DatabaseType {
        DatabaseType::Postgres
    }

    async fn execute(&self, sql: &str) -> DMSCResult<u64> {
        self.client
            .execute(sql, &[])
            .await
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }

    async fn query(&self, sql: &str) -> DMSCResult<DMSCDBResult> {
        let rows = self
            .client
            .query(sql, &[])
            .await
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        let dmsc_rows: Vec<DMSCDBRow> = rows.iter().map(|r| self.row_to_dmsc_row(r)).collect();
        Ok(DMSCDBResult::with_rows(dmsc_rows))
    }

    async fn query_one(&self, sql: &str) -> DMSCResult<Option<DMSCDBRow>> {
        let rows = self
            .client
            .query(sql, &[])
            .await
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))?;
        Ok(rows.first().map(|r| self.row_to_dmsc_row(r)))
    }

    async fn ping(&self) -> DMSCResult<bool> {
        self.client
            .execute("SELECT 1", &[])
            .await
            .map(|_| true)
            .map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }

    fn is_connected(&self) -> bool {
        !self.client.is_closed()
    }

    async fn close(&self) -> DMSCResult<()> {
        self.client.close().await.map_err(|e| crate::core::DMSCError::Config(e.to_string()))
    }
}

impl PostgresDatabase {
    fn row_to_dmsc_row(&self, row: &tokio_postgres::Row) -> DMSCDBRow {
        let mut dmsc_row = DMSCDBRow::new();
        let columns = row.columns();
        for col in columns {
            let name = col.name();
            let value: Option<tokio_postgres::types::Json<serde_json::Value>> = row.get(name);
            let json = match value {
                Some(tokio_postgres::types::Json(v)) => v,
                None => serde_json::Value::Null,
            };
            dmsc_row.add_value(name, json);
        }
        dmsc_row
    }
}
