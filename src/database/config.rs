//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

use serde::{Deserialize, Serialize};
use std::env;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    Postgres,
    MySQL,
    SQLite,
}

impl Default for DatabaseType {
    fn default() -> Self {
        DatabaseType::Postgres
    }
}

impl std::fmt::Display for DatabaseType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseType::Postgres => write!(f, "postgresql"),
            DatabaseType::MySQL => write!(f, "mysql"),
            DatabaseType::SQLite => write!(f, "sqlite"),
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDatabaseConfig {
    pub database_type: DatabaseType,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    pub max_connections: u32,
    pub min_idle_connections: u32,
    pub connection_timeout_secs: u64,
    pub idle_timeout_secs: u64,
    pub max_lifetime_secs: u64,
    pub ssl_mode: SslMode,
    pub statement_cache_size: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SslMode {
    Disable,
    Prefer,
    Require,
}

impl Default for SslMode {
    fn default() -> Self {
        SslMode::Prefer
    }
}

impl DMSCDatabaseConfig {
    pub fn postgres() -> Self {
        Self {
            database_type: DatabaseType::Postgres,
            host: env::var("DMSC_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DMSC_DB_PORT")
                .unwrap_or_else(|_| "5432".to_string())
                .parse()
                .unwrap_or(5432),
            database: env::var("DMSC_DB_NAME").unwrap_or_else(|_| "dmsc".to_string()),
            username: env::var("DMSC_DB_USER").unwrap_or_else(|_| "dmsc".to_string()),
            password: env::var("DMSC_DB_PASSWORD").unwrap_or_else(|_| "".to_string()),
            max_connections: 10,
            min_idle_connections: 2,
            connection_timeout_secs: 30,
            idle_timeout_secs: 600,
            max_lifetime_secs: 3600,
            ssl_mode: SslMode::Prefer,
            statement_cache_size: 100,
        }
    }

    pub fn mysql() -> Self {
        Self {
            database_type: DatabaseType::MySQL,
            host: env::var("DMSC_DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DMSC_DB_PORT")
                .unwrap_or_else(|_| "3306".to_string())
                .parse()
                .unwrap_or(3306),
            database: env::var("DMSC_DB_NAME").unwrap_or_else(|_| "dmsc".to_string()),
            username: env::var("DMSC_DB_USER").unwrap_or_else(|_| "dmsc".to_string()),
            password: env::var("DMSC_DB_PASSWORD").unwrap_or_else(|_| "".to_string()),
            max_connections: 10,
            min_idle_connections: 2,
            connection_timeout_secs: 30,
            idle_timeout_secs: 600,
            max_lifetime_secs: 3600,
            ssl_mode: SslMode::Prefer,
            statement_cache_size: 100,
        }
    }

    pub fn sqlite(path: &str) -> Self {
        Self {
            database_type: DatabaseType::SQLite,
            host: "".to_string(),
            port: 0,
            database: path.to_string(),
            username: "".to_string(),
            password: "".to_string(),
            max_connections: 10,
            min_idle_connections: 1,
            connection_timeout_secs: 30,
            idle_timeout_secs: 600,
            max_lifetime_secs: 3600,
            ssl_mode: SslMode::Disable,
            statement_cache_size: 100,
        }
    }

    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    pub fn user(mut self, user: &str) -> Self {
        self.username = user.to_string();
        self
    }

    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    pub fn min_idle_connections(mut self, min: u32) -> Self {
        self.min_idle_connections = min;
        self
    }

    pub fn connection_timeout_secs(mut self, secs: u64) -> Self {
        self.connection_timeout_secs = secs;
        self
    }

    pub fn idle_timeout_secs(mut self, secs: u64) -> Self {
        self.idle_timeout_secs = secs;
        self
    }

    pub fn max_lifetime_secs(mut self, secs: u64) -> Self {
        self.max_lifetime_secs = secs;
        self
    }

    pub fn ssl_mode(mut self, mode: SslMode) -> Self {
        self.ssl_mode = mode;
        self
    }

    pub fn statement_cache_size(mut self, size: u32) -> Self {
        self.statement_cache_size = size;
        self
    }

    pub fn build(self) -> DMSCDatabaseConfig {
        self
    }

    pub fn connection_string(&self) -> String {
        match self.database_type {
            DatabaseType::Postgres => {
                format!(
                    "postgresql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::MySQL => {
                format!(
                    "mysql://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::SQLite => self.database.clone(),
        }
    }
}

impl Default for DMSCDatabaseConfig {
    fn default() -> Self {
        Self::postgres()
    }
}
