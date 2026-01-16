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

//! # Database Migration
//!
//! This module provides database migration and schema management functionality for DMSC.
//! It handles version-controlled schema changes with support for up and down migrations.
//!
//! ## Key Components
//!
//! - **Migration**: Struct representing a single migration with version, name, and SQL statements
//! - **MigrationManager**: Manages migration lifecycle and execution
//!
//! ## Design Principles
//!
//! 1. **Version Control**: Each migration has a unique version number
//! 2. **Idempotent Operations**: Migrations can be safely run multiple times
//! 3. **Transactional Safety**: Migrations execute within transactions
//! 4. **Bidirectional Changes**: Support for both up and down migrations
//! 5. **History Tracking**: Maintain migration execution history
//!
//! ## Usage Example
//!
//! ```rust
//! use dmsc::database::migration::{Migration, MigrationManager};
//!
//! let migration = Migration::new(
//!     1,
//!     "create_users_table",
//!     "CREATE TABLE users (id INT PRIMARY KEY, name TEXT)",
//!     "DROP TABLE users",
//! );
//!
//! let manager = MigrationManager::new(migrations);
//! manager.migrate().await?;
//! ```

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DMSCDatabaseMigration {
    pub version: u32,
    pub name: String,
    pub sql_up: String,
    pub sql_down: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl DMSCDatabaseMigration {
    pub fn new(version: u32, name: &str, sql_up: &str, sql_down: Option<&str>) -> Self {
        Self {
            version,
            name: name.to_string(),
            sql_up: sql_up.to_string(),
            sql_down: sql_down.map(|s| s.to_string()),
            timestamp: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DMSCMigrationHistory {
    pub version: u32,
    pub name: String,
    pub applied_at: chrono::DateTime<chrono::Utc>,
    pub checksum: String,
}

impl DMSCMigrationHistory {
    pub fn new(version: u32, name: &str, checksum: &str) -> Self {
        Self {
            version,
            name: name.to_string(),
            applied_at: chrono::Utc::now(),
            checksum: checksum.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDatabaseMigrator {
    migrations: Vec<DMSCDatabaseMigration>,
    migrations_dir: Option<PathBuf>,
}

impl DMSCDatabaseMigrator {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
            migrations_dir: None,
        }
    }

    pub fn with_migrations_dir(mut self, dir: PathBuf) -> Self {
        self.migrations_dir = Some(dir);
        self
    }

    pub fn add_migration(&mut self, migration: DMSCDatabaseMigration) {
        self.migrations.push(migration);
        self.migrations.sort_by(|a, b| a.version.cmp(&b.version));
    }

    pub fn add_migrations(&mut self, migrations: Vec<DMSCDatabaseMigration>) {
        self.migrations.extend(migrations);
        self.migrations.sort_by(|a, b| a.version.cmp(&b.version));
    }

    pub fn get_migrations(&self) -> &[DMSCDatabaseMigration] {
        &self.migrations
    }

    pub fn get_migration(&self, version: u32) -> Option<&DMSCDatabaseMigration> {
        self.migrations.iter().find(|m| m.version == version)
    }

    pub fn get_pending_migrations(&self, applied: &[DMSCMigrationHistory]) -> Vec<&DMSCDatabaseMigration> {
        let applied_versions: std::collections::HashSet<u32> = applied.iter().map(|h| h.version).collect();
        self.migrations.iter()
            .filter(|m| !applied_versions.contains(&m.version))
            .collect()
    }

    pub fn get_applied_version(&self, applied: &[DMSCMigrationHistory]) -> Option<u32> {
        applied.iter()
            .map(|h| h.version)
            .max()
    }

    pub fn calculate_checksum(_sql: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        _sql.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    pub fn load_migrations_from_dir(&mut self, dir: &str) -> std::io::Result<()> {
        let path = PathBuf::from(dir);
        if !path.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(&path)?;
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map(|e| e.to_str()) == Some(Some("sql")) {
                if let Some(file_name) = path.file_stem().and_then(|n| n.to_str()) {
                    let sql_content = std::fs::read_to_string(&path)?;
                    let version: u32 = file_name.split('_').next().unwrap_or("0").parse().unwrap_or(0);
                    let name = file_name.splitn(2, '_').nth(1).unwrap_or(file_name).to_string();
                    self.add_migration(DMSCDatabaseMigration::new(version, &name, &sql_content, None));
                }
            }
        }
        Ok(())
    }
}

impl Default for DMSCDatabaseMigrator {
    fn default() -> Self {
        Self::new()
    }
}
