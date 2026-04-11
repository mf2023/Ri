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

//! # Database Module
//!
//! This module provides a comprehensive database abstraction layer for Ri,
//! supporting multiple database backends with a unified interface.
//!
//! ## Key Components
//!
//! - **RiDatabase**: Trait defining the database interface for all supported databases
//! - **RiDatabasePool**: Connection pool management with automatic reuse and health checks
//! - **RiDatabaseConfig**: Builder-style configuration for database connections
//! - **RiDatabaseMigrator**: Database schema migration management
//! - **RiDBRow**: Row representation with type-safe column access
//! - **RiDBResult**: Query result set with iterator support
//! - **RiDatabaseTransaction**: ACID transaction support for data integrity
//!
//! ## Supported Databases
//!
//! - **PostgreSQL** - Enterprise-grade relational database with full feature support
//! - **MySQL** - Popular open-source database with high performance
//! - **SQLite** - Embedded database for lightweight scenarios
//!
//! ## Features
//!
//! - **Connection Pooling**: Efficient connection reuse with configurable pool size
//! - **Async/Await**: Full asynchronous API using Tokio runtime
//! - **Parameter Binding**: Safe SQL parameter binding to prevent injection attacks
//! - **Batch Operations**: Efficient bulk insert/update operations
//! - **Transactions**: ACID-compliant transaction support
//! - **Migrations**: Version-controlled schema migrations
//! - **Type Conversion**: Automatic conversion between database and JSON types
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use ri::database::{RiDatabase, RiDatabaseConfig, PooledDatabase};
//!
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     let config = RiDatabaseConfig::postgres()
//!         .host("localhost")
//!         .port(5432)
//!         .database("mydb")
//!         .user("user")
//!         .password("pass")
//!         .max_connections(10)
//!         .min_idle_connections(2)
//!         .connection_timeout_secs(30)
//!         .build();
//!
//!     let pool = RiDatabasePool::new(config).await?;
//!     let db = pool.get().await?;
//!
//!     // Execute a query with parameter binding
//!     use serde_json::json;
//!     let rows = db.query("SELECT * FROM users WHERE id = $1", &[&json!(1)]).await?;
//!     for row in rows {
//!         let id: i64 = row.get("id");
//!         let name: String = row.get("name");
//!         println!("User: {} - {}", id, name);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Batch Operations Example
//!
//! ```rust,ignore
//! use ri::database::{RiDatabasePool, RiDatabaseConfig};
//!
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     let config = RiDatabaseConfig::postgres()
//!         .host("localhost")
//!         .database("mydb")
//!         .build();
//!
//!     let pool = RiDatabasePool::new(config).await?;
//!     let db = pool.get().await?;
//!
//!     // Batch insert users
//!     let users = vec![
//!         vec![serde_json::json!("alice"), serde_json::json!("alice@example.com")],
//!         vec![serde_json::json!("bob"), serde_json::json!("bob@example.com")],
//!         vec![serde_json::json!("charlie"), serde_json::json!("charlie@example.com")],
//!     ];
//!
//!     let results = db.batch_execute(
//!         "INSERT INTO users (name, email) VALUES ($1, $2)",
//!         &users
//!     ).await?;
//!
//!     println!("Inserted {} rows", results.len());
//!     Ok(())
//! }
//! ```
//!
//! ## Transaction Example
//!
//! ```rust,ignore
//! use ri::database::{RiDatabasePool, RiDatabaseConfig};
//!
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     let config = RiDatabaseConfig::postgres()
//!         .host("localhost")
//!         .database("mydb")
//!         .build();
//!
//!     let pool = RiDatabasePool::new(config).await?;
//!     let db = pool.get().await?;
//!
//!     // Start transaction
//!     let mut tx = db.transaction().await?;
//!
//!     // Execute operations within transaction
//!     tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", &[&1]).await?;
//!     tx.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $1", &[&2]).await?;
//!
//!     // Commit transaction
//!     tx.commit().await?;
//!
//!     Ok(())
//! }
//! ```

mod config;
mod pool;
mod row;
mod result;
mod migration;
pub mod orm;

#[cfg(feature = "postgres")]
pub mod postgres;
#[cfg(feature = "mysql")]
pub mod mysql;
#[cfg(feature = "sqlite")]
pub mod sqlite;

pub use config::{RiDatabaseConfig, DatabaseType};
pub use pool::{RiDatabasePool, PooledDatabase, DatabaseMetrics, DynamicPoolConfig};
pub use row::RiDBRow;
pub use result::RiDBResult;
pub use migration::{RiDatabaseMigration, RiMigrationHistory, RiDatabaseMigrator};
pub use orm::{QueryBuilder, Criteria, SortOrder, Pagination, ComparisonOperator, 
    TableDefinition, ColumnDefinition, IndexDefinition, ForeignKeyDefinition,
    RiORMSimpleRepository, RiORMCrudRepository, RiORMRepository};

use crate::core::{RiResult, RiError};
use async_trait::async_trait;
use thiserror::Error as ThisError;
use serde::{Serialize, Deserialize};

#[derive(Debug, ThisError, Clone, Serialize, Deserialize)]
pub enum RiDatabaseTransactionError {
    #[error("Transaction commit failed: {message}")]
    CommitFailed { message: String },
    #[error("Transaction rollback failed: {message}")]
    RollbackFailed { message: String },
    #[error("Transaction already completed")]
    AlreadyCompleted,
    #[error("Transaction operation failed: {message}")]
    OperationFailed { message: String },
}

impl From<RiDatabaseTransactionError> for RiError {
    fn from(e: RiDatabaseTransactionError) -> Self {
        RiError::Database(e.to_string())
    }
}

impl From<RiDatabaseTransactionError> for RiResult<()> {
    fn from(e: RiDatabaseTransactionError) -> Self {
        Err(e.into())
    }
}

/// Trait defining the database interface for all supported databases.
///
/// This trait provides a unified API for database operations across different
/// database backends. Implementations handle backend-specific details while
/// presenting a consistent interface to callers.
///
/// ## Supported Operations
///
/// - **Execution**: Execute SQL statements without returning results
/// - **Querying**: Execute SELECT statements and iterate over results
/// - **Parameter Binding**: Safe SQL parameter binding to prevent injection
/// - **Batch Operations**: Efficient bulk execution and querying
/// - **Transactions**: ACID-compliant transaction support
///
/// ## Example
///
/// ```rust,ignore
/// use ri::database::{RiDatabase, RiDatabasePool, RiDatabaseConfig};
///
/// #[tokio::main]
/// async fn main() -> RiResult<()> {
///     let config = RiDatabaseConfig::postgres()
///         .host("localhost")
///         .database("mydb")
///         .build();
///
///     let pool = RiDatabasePool::new(config).await?;
///     let db = pool.get().await?;
///
///     // Execute a statement
///     db.execute("CREATE TABLE IF NOT EXISTS users (id SERIAL PRIMARY KEY, name TEXT)").await?;
///
///     // Query with parameters
///     let rows = db.query_with_params(
///         "SELECT * FROM users WHERE name = $1",
///         &[serde_json::json!("alice")]
///     ).await?;
///
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait RiDatabase: Send + Sync {
    /// Returns the type of database this instance connects to.
    ///
    /// This can be used to identify which database backend is in use
    /// and potentially branch behavior based on database type.
    fn database_type(&self) -> DatabaseType;

    /// Executes a SQL statement without returning results.
    ///
    /// This method is suitable for INSERT, UPDATE, DELETE, and DDL statements.
    /// Returns the number of rows affected.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL statement to execute
    ///
    /// ## Returns
    ///
    /// The number of rows affected, or an error if execution fails.
    async fn execute(&self, sql: &str) -> RiResult<u64>;

    /// Executes a SQL query and returns all matching rows.
    ///
    /// This method is suitable for SELECT statements. The returned result
    /// can be iterated over to access individual rows.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL query to execute
    ///
    /// ## Returns
    ///
    /// A result set containing all matching rows, or an error if the query fails.
    async fn query(&self, sql: &str) -> RiResult<RiDBResult>;

    /// Executes a SQL query and returns at most one row.
    ///
    /// This method is equivalent to adding "LIMIT 1" to the query.
    /// Returns `None` if no rows match.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL query to execute
    ///
    /// ## Returns
    ///
    /// `Some(row)` if a row was found, `None` if no rows match, or an error.
    async fn query_one(&self, sql: &str) -> RiResult<Option<RiDBRow>>;

    /// Checks if the database connection is alive.
    ///
    /// This method can be used to verify that the connection is still
    /// responsive before executing important operations.
    ///
    /// ## Returns
    ///
    /// `true` if the connection is healthy, `false` otherwise.
    async fn ping(&self) -> RiResult<bool>;

    /// Checks if the database connection is currently connected.
    ///
    /// Unlike `ping()`, this method does not perform any network operation.
    ///
    /// ## Returns
    ///
    /// `true` if connected, `false` otherwise.
    fn is_connected(&self) -> bool;

    /// Closes the database connection.
    ///
    /// This method should be called when the connection is no longer needed
    /// to release resources. After calling this method, the connection
    /// should not be used.
    async fn close(&self) -> RiResult<()>;

    /// Executes the same SQL statement multiple times with different parameters.
    ///
    /// This is more efficient than executing statements individually when
    /// performing bulk operations.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL statement with placeholders ($1, $2, etc.)
    /// - `params`: A slice of parameter sets, one for each execution
    ///
    /// ## Returns
    ///
    /// A vector of affected row counts, one for each execution.
    async fn batch_execute(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<u64>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.execute_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    /// Executes the same query multiple times with different parameters.
    ///
    /// This is more efficient than executing queries individually when
    /// performing bulk reads.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL query with placeholders ($1, $2, etc.)
    /// - `params`: A slice of parameter sets, one for each query
    ///
    /// ## Returns
    ///
    /// A vector of result sets, one for each query.
    async fn batch_query(&self, sql: &str, params: &[Vec<serde_json::Value>]) -> RiResult<Vec<RiDBResult>> {
        let mut results = Vec::with_capacity(params.len());
        for param_set in params {
            let result = self.query_with_params(sql, param_set).await?;
            results.push(result);
        }
        Ok(results)
    }

    /// Executes a SQL statement with parameters.
    ///
    /// Parameters are bound using placeholder syntax ($1, $2, etc.) to
    /// prevent SQL injection attacks.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL statement with placeholders
    /// - `params`: The parameter values to bind
    ///
    /// ## Returns
    ///
    /// The number of rows affected.
    async fn execute_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<u64>;

    /// Executes a SQL query with parameters.
    ///
    /// Parameters are bound using placeholder syntax ($1, $2, etc.) to
    /// prevent SQL injection attacks.
    ///
    /// ## Parameters
    ///
    /// - `sql`: The SQL query with placeholders
    /// - `params`: The parameter values to bind
    ///
    /// ## Returns
    ///
    /// A result set containing all matching rows.
    async fn query_with_params(&self, sql: &str, params: &[serde_json::Value]) -> RiResult<RiDBResult>;

    /// Starts a new database transaction.
    ///
    /// Transactions allow multiple operations to be grouped together
    /// with atomic commit/rollback semantics.
    ///
    /// ## Returns
    ///
    /// A transaction handle that can be used to execute operations
    /// within the transaction.
    async fn transaction(&self) -> RiResult<Box<dyn RiDatabaseTransaction>>;
}

/// Trait representing a database transaction.
///
/// Transactions provide atomic operations where multiple SQL statements
/// can be executed together and either committed or rolled back as a unit.
/// This ensures data integrity even when operations fail partway through.
///
/// ## Transaction Lifecycle
///
/// 1. **Begin**: Transaction starts with `RiDatabase::transaction()`
/// 2. **Execute**: Run SQL statements within the transaction
/// 3. **Commit**: Save all changes with `commit()`
///    OR **Rollback**: Discard all changes with `rollback()`
///
/// ## Example
///
/// ```rust,ignore
/// use ri::database::{RiDatabasePool, RiDatabaseConfig};
///
/// #[tokio::main]
/// async fn main() -> RiResult<()> {
///     let config = RiDatabaseConfig::postgres()
///         .host("localhost")
///         .database("mydb")
///         .build();
///
///     let pool = RiDatabasePool::new(config).await?;
///     let db = pool.get().await?;
///
///     let mut tx = db.transaction().await?;
///
///     // Transfer funds between accounts
///     tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", &[&1]).await?;
///     tx.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $1", &[&2]).await?;
///
///     tx.commit().await?;
///     Ok(())
/// }
/// ```
///
/// ## Auto-Rollback
///
/// If a transaction is dropped without explicitly calling `commit()` or
/// `rollback()`, it will automatically be rolled back to ensure no
/// partial changes are persisted.
#[async_trait]
pub trait RiDatabaseTransaction: Send + Sync {
    /// Executes a SQL statement within the transaction.
    ///
    /// See [`RiDatabase::execute()`] for more details.
    async fn execute(&self, sql: &str) -> RiResult<u64>;

    /// Executes a SQL query within the transaction.
    ///
    /// See [`RiDatabase::query()`] for more details.
    async fn query(&self, sql: &str) -> RiResult<RiDBResult>;

    /// Commits all changes made within the transaction.
    ///
    /// After calling this method, the transaction is complete and
    /// all changes are permanent.
    ///
    /// ## Errors
    ///
    /// Returns an error if the commit fails. In this case, the
    /// transaction should be considered failed and no further
    /// operations should be attempted.
    async fn commit(&self) -> RiResult<()>;

    /// Rolls back all changes made within the transaction.
    ///
    /// This discards all changes made since the transaction began.
    /// The transaction is then complete and cannot be used further.
    ///
    /// ## Errors
    ///
    /// Returns an error if the rollback fails. In this case, the
    /// transaction state is uncertain and the database should be
    /// checked for consistency.
    async fn rollback(&self) -> RiResult<()>;

    /// Closes the transaction without committing.
    ///
    /// If the transaction has not been committed, this will implicitly
    /// roll back any changes. This method is primarily for cleanup
    /// and should not be used as a substitute for explicit rollback.
    async fn close(&self) -> RiResult<()>;
}
