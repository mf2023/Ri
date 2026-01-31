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

//! # Database Configuration
//!
//! This module provides database configuration types and settings for DMSC.
//! It supports multiple database backends including MySQL, PostgreSQL, SQLite, and in-memory databases.
//!
//! ## Key Components
//!
//! - **DMSCDatabaseConfig**: Enum for different database configurations
//! - **DatabaseType**: Enum for supported database engines
//! - **PoolConfig**: Connection pool configuration settings
//!
//! ## Design Principles
//!
//! 1. **Type Safety**: Each database type has its own configuration variant
//! 2. **Flexible Pooling**: Configurable connection pool settings for performance
//! 3. **Backend-Agnostic**: Unified interface across different database engines
//! 4. **Default Values**: Sensible defaults for all configuration options
//!
//! ## Usage Example
//!
//! ```rust
//! use dmsc::database::{DMSCDatabaseConfig, DatabaseType, PoolConfig};
//!
//! let config = DMSCDatabaseConfig::new_mysql(
//!     "localhost",
//!     3306,
//!     "root",
//!     "password",
//!     "test_db",
//! );
//!
//! let pool_config = PoolConfig::new(10, 300, 600);
//! ```

use serde::{Deserialize, Serialize};
use std::env;

/// Enumeration of supported database engine types.
///
/// This enum represents the different database backends that DMSC can connect to.
/// Each database type has specific connection requirements and may use different
/// underlying drivers or client libraries.
///
/// ## Currently Implemented
///
/// | Database Type | Feature Flag | Status |
/// |---------------|--------------|--------|
/// | PostgreSQL | `postgres` | ✅ Available |
/// | MySQL | `mysql` | ✅ Available |
/// | SQLite | `sqlite` | ✅ Available |
/// | MongoDB | `mongodb` | 🔜 Planned |
/// | Redis | `redis` | 🔜 Planned |
///
/// ## Roadmap
///
/// MongoDB and Redis support are planned for future releases. The enum variants
/// are reserved to maintain API stability when these features are added.
///
/// ## Usage
///
/// ```rust
/// use dmsc::database::DatabaseType;
///
/// fn get_preferred_db() -> DatabaseType {
///     DatabaseType::Postgres
/// }
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseType {
    /// PostgreSQL database engine.
    ///
    /// PostgreSQL is a powerful, open source object-relational database system.
    /// It is known for its reliability, feature richness, and performance.
    /// Default port is 5432.
    ///
    /// ## Features
    ///
    /// - Full ACID compliance
    /// - Complex queries and joins
    /// - Foreign key support
    /// - Triggers and views
    /// - Stored procedures
    Postgres,
    /// MySQL database engine.
    ///
    /// MySQL is the world's most popular open source database.
    /// It is widely used for web applications and is known for its speed and reliability.
    /// Default port is 3306.
    ///
    /// ## Features
    ///
    /// - ACID compliance (with InnoDB)
    /// - Cross-platform support
    /// - Stored procedures and triggers
    /// - Full-text indexing
    MySQL,
    /// SQLite database engine.
    ///
    /// SQLite is a lightweight, file-based database engine.
    /// It requires no server and is embedded directly into the application.
    /// Suitable for development, testing, and desktop applications.
    ///
    /// ## Features
    ///
    /// - Serverless architecture
    /// - Zero-configuration
    /// - Single file storage
    /// - Full SQL support
    SQLite,
    /// MongoDB database engine.
    ///
    /// MongoDB is a document-oriented NoSQL database.
    /// It uses JSON-like documents with optional schemas.
    /// Default port is 27017.
    ///
    /// ## Features
    ///
    /// - Flexible document schema
    /// - Horizontal scaling
    /// - Rich query language
    /// - Automatic sharding
    MongoDB,
    /// Redis database engine.
    ///
    /// Redis is an in-memory data structure store.
    /// It can be used as a database, cache, and message broker.
    /// Default port is 6379.
    ///
    /// ## Features
    ///
    /// - In-memory storage
    /// - Data structures (strings, hashes, lists, sets)
    /// - Pub/Sub messaging
    /// - Persistence options
    Redis,
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
            DatabaseType::MongoDB => write!(f, "mongodb"),
            DatabaseType::Redis => write!(f, "redis"),
        }
    }
}

/// Configuration for database connections in DMSC.
///
/// This struct encapsulates all configuration options needed to establish and manage
/// database connections. It supports multiple database backends through the `DatabaseType`
/// enum and provides a fluent builder API for configuration.
///
/// ## Connection Pooling
///
/// DMSC uses connection pooling to efficiently manage database connections.
/// The pool maintains a set of connections that are reused across requests,
/// reducing the overhead of establishing new connections.
///
/// ## Configuration Methods
///
/// The struct provides several factory methods for creating configurations:
/// - [`postgres()`][DMSCDatabaseConfig::postgres] - PostgreSQL with default settings
/// - [`mysql()`][DMSCDatabaseConfig::mysql] - MySQL with default settings
/// - [`sqlite(path)`][DMSCDatabaseConfig::sqlite] - SQLite at specified path
///
/// ## Builder Pattern
///
/// Configuration can be customized using the builder pattern:
///
/// ```rust
/// use dmsc::database::{DMSCDatabaseConfig, SslMode};
///
/// let config = DMSCDatabaseConfig::postgres()
///     .host("db.example.com")
///     .port(5432)
///     .database("myapp")
///     .user("admin")
///     .password("secret")
///     .max_connections(20)
///     .ssl_mode(SslMode::Require)
///     .build();
/// ```
///
/// ## Environment Variables
///
/// Default values can be overridden using environment variables:
/// - `DMSC_DB_HOST` - Database server hostname
/// - `DMSC_DB_PORT` - Database server port
/// - `DMSC_DB_NAME` - Database name
/// - `DMSC_DB_USER` - Database username
/// - `DMSC_DB_PASSWORD` - Database password
///
/// ## Thread Safety
///
/// This struct is clonable and can be shared across threads.
/// However, modifications should be done before the configuration is passed
/// to the database manager.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDatabaseConfig {
    /// The type of database backend to connect to.
    ///
    /// This determines which driver and connection logic will be used.
    /// Common values are `DatabaseType::Postgres`, `DatabaseType::MySQL`,
    /// and `DatabaseType::SQLite`.
    pub database_type: DatabaseType,

    /// Hostname or IP address of the database server.
    ///
    /// For local development, this is typically `"localhost"` or `"127.0.0.1"`.
    /// For production, this should be the database server's hostname.
    ///
    /// ## Examples
    ///
    /// - `localhost` - Local database server
    /// - `db.example.com` - Remote database server
    /// - `192.168.1.100` - IP address of database server
    pub host: String,

    /// Port number for database connections.
    ///
    /// Each database type has a default port:
    /// - PostgreSQL: 5432
    /// - MySQL: 3306
    /// - MongoDB: 27017
    /// - Redis: 6379
    ///
    /// SQLite ignores this field as it uses file-based connections.
    pub port: u16,

    /// Name of the database to connect to.
    ///
    /// For PostgreSQL and MySQL, this is the name of a specific database
    /// within the database server.
    ///
    /// For SQLite, this is the file path (`:memory:` for in-memory database).
    pub database: String,

    /// Username for database authentication.
    ///
    /// This user must have sufficient privileges to perform the required
    /// database operations. For security, consider using environment variables
    /// or secrets management to provide this value.
    pub username: String,

    /// Password for database authentication.
    ///
    /// This password is used together with the username to authenticate
    /// with the database server. For security, consider using environment
    /// variables or secrets management to provide this value.
    pub password: String,

    /// Maximum number of concurrent database connections.
    ///
    /// This setting controls the upper bound of the connection pool.
    /// Higher values allow more concurrent database operations but increase
    /// resource usage on both the application and database server.
    ///
    /// ## Recommendations
    ///
    /// - Development: 5-10 connections
    /// - Production: 10-50 connections (depends on workload)
    /// - Consider database server's max_connections setting
    pub max_connections: u32,

    /// Minimum number of idle connections to maintain.
    ///
    /// The connection pool will maintain at least this many idle connections
    /// to reduce the latency of new database operations. These connections
    /// are still subject to the idle timeout.
    ///
    /// ## Default Value
    ///
    /// Typically 1-2 connections, depending on expected concurrency.
    pub min_idle_connections: u32,

    /// Timeout for establishing new connections in seconds.
    ///
    /// If a connection cannot be established within this time, the operation
    /// will fail with a timeout error. This prevents the application from
    /// hanging indefinitely when the database is unreachable.
    ///
    /// ## Common Values
    ///
    /// - 30 seconds for most scenarios
    /// - 5-10 seconds for latency-sensitive applications
    /// - 60+ seconds for distant database servers
    pub connection_timeout_secs: u64,

    /// Maximum time a connection can be idle before being closed.
    ///
    /// Idle connections that have not been used for this duration will be
    /// closed and removed from the pool. This helps free resources on both
    /// the application and database server.
    ///
    /// ## Recommendations
    ///
    /// - 600 seconds (10 minutes) for web applications
    /// - 300 seconds (5 minutes) for batch processing
    /// - Consider database server's connection timeout settings
    pub idle_timeout_secs: u64,

    /// Maximum lifetime of a connection in seconds.
    ///
    /// Connections older than this will be closed and replaced with new ones.
    /// This prevents connections from becoming stale due to:
    /// - Network interruptions
    /// - Database server restarts
    /// - Connection timeout on the database side
    ///
    /// ## Recommendations
    ///
    /// - 1800-3600 seconds (30-60 minutes) for most applications
    /// - Shorter values for long-running applications
    /// - Disable (use None) for very short-lived applications
    pub max_lifetime_secs: u64,

    /// SSL/TLS mode for encrypted connections.
    ///
    /// This setting controls whether and how SSL/TLS encryption is used
    /// for database connections. It is ignored by SQLite.
    ///
    /// ## Security
    ///
    /// Always use `SslMode::Require` in production environments to ensure
    /// all database traffic is encrypted.
    pub ssl_mode: SslMode,

    /// Maximum number of prepared statements to cache.
    ///
    /// Prepared statements are cached to reduce the overhead of repeated
    /// query compilation. Higher values improve performance for complex
    /// queries but increase memory usage.
    ///
    /// ## Recommendations
    ///
    /// - 100-500 for typical applications
    /// - 1000+ for applications with many repeated complex queries
    /// - 0 to disable statement caching
    pub statement_cache_size: u32,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCDatabaseConfig {
    #[new]
    fn py_new(
        database_type: DatabaseType,
        host: String,
        port: u16,
        database: String,
        username: String,
        password: String,
        max_connections: u32,
        min_idle_connections: u32,
        connection_timeout_secs: u64,
        idle_timeout_secs: u64,
        max_lifetime_secs: u64,
        ssl_mode: SslMode,
        statement_cache_size: u32,
    ) -> Self {
        Self {
            database_type,
            host,
            port,
            database,
            username,
            password,
            max_connections,
            min_idle_connections,
            connection_timeout_secs,
            idle_timeout_secs,
            max_lifetime_secs,
            ssl_mode,
            statement_cache_size,
        }
    }

    #[staticmethod]
    fn create_postgres() -> Self {
        Self::postgres()
    }

    #[staticmethod]
    fn create_mysql() -> Self {
        Self::mysql()
    }

    #[staticmethod]
    fn create_sqlite() -> Self {
        Self::sqlite(":memory:")
    }

    fn get_database_type(&self) -> DatabaseType {
        self.database_type
    }

    fn set_database_type(&self, _database_type: DatabaseType) {
        // Can't modify in pyo3, use create functions instead
    }

    fn get_host(&self) -> String {
        self.host.clone()
    }

    fn set_host(&mut self, host: String) {
        self.host = host;
    }

    fn get_port(&self) -> u16 {
        self.port
    }

    fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    fn get_database(&self) -> String {
        self.database.clone()
    }

    fn set_database(&mut self, database: String) {
        self.database = database;
    }

    fn get_username(&self) -> String {
        self.username.clone()
    }

    fn set_username(&mut self, username: String) {
        self.username = username;
    }

    fn get_password(&self) -> String {
        self.password.clone()
    }

    fn set_password(&mut self, password: String) {
        self.password = password;
    }

    fn get_max_connections(&self) -> u32 {
        self.max_connections
    }

    fn set_max_connections(&mut self, max_connections: u32) {
        self.max_connections = max_connections;
    }

    fn get_min_idle_connections(&self) -> u32 {
        self.min_idle_connections
    }

    fn set_min_idle_connections(&mut self, min_idle_connections: u32) {
        self.min_idle_connections = min_idle_connections;
    }
}

/// SSL/TLS connection mode for database connections.
///
/// This enum controls whether and how SSL/TLS encryption is used when
/// connecting to the database. SSL/TLS provides:
/// - **Confidentiality**: Encryption prevents eavesdropping on database traffic
/// - **Integrity**: Protection against data tampering during transmission
/// - **Authentication**: Verification of the database server's identity
///
/// ## Security Recommendations
///
/// | Environment | Recommended Mode | Reason |
/// |-------------|------------------|--------|
/// | Production | `Require` | Maximum security, prevents MITM attacks |
/// | Development | `Prefer` | Encryption when available |
/// | Testing | `Prefer` or `Disable` | Convenience during development |
///
/// ## Database Support
///
/// - **PostgreSQL**: Fully supports SSL with all modes
/// - **MySQL**: Fully supports SSL with all modes
/// - **MongoDB**: Supports SSL with all modes
/// - **SQLite**: Does not support SSL (ignored)
/// - **Redis**: Uses separate TLS configuration
///
/// ## Certificate Verification
///
/// When using `Require`, the client will verify the server's certificate.
/// This requires the server to have a valid certificate signed by a trusted
/// certificate authority. Self-signed certificates will fail verification.
///
/// For development with self-signed certificates, you may need to:
/// 1. Add the certificate to your system's trust store
/// 2. Configure the database client to trust the specific certificate
/// 3. Use `Prefer` mode (less secure, not recommended for production)
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SslMode {
    /// SSL/TLS is disabled. Connections are unencrypted.
    ///
    /// ## Use Cases
    ///
    /// - Local development with local database
    /// - Testing in isolated environments
    /// - Situations where encryption overhead is unacceptable
    ///
    /// ## Security Warning
    ///
    /// This mode provides no protection against eavesdropping or tampering.
    /// Never use in production or when transmitting sensitive data.
    Disable,

    /// SSL/TLS is preferred but not required.
    ///
    /// The client will attempt to establish an SSL connection if the server
    /// supports it. If SSL is not available, the connection will fall back
    /// to an unencrypted connection.
    ///
    /// ## Behavior
    ///
    /// 1. Client requests SSL connection
    /// 2. If server supports SSL, use encrypted connection
    /// 3. If server rejects SSL, use unencrypted connection
    ///
    /// ## Security Warning
    ///
    /// This mode allows fallback to unencrypted connections, which could
    /// be exploited in man-in-the-middle attacks. Consider using `Require`
    /// for better security.
    Prefer,

    /// SSL/TLS is required.
    ///
    /// The client will only establish connections that are encrypted with
    /// SSL/TLS. The connection will fail if SSL is not available or if
    /// certificate verification fails.
    ///
    /// ## Server Certificate Verification
    ///
    /// When `Require` mode is used, the client verifies:
    /// - The certificate is not expired
    /// - The certificate is signed by a trusted CA
    /// - The certificate hostname matches the server hostname
    ///
    /// ## Use Cases
    ///
    /// - Production environments
    /// - When transmitting sensitive data
    /// - Compliance with security regulations
    ///
    /// ## Common Errors
    ///
    /// - `certificate verify failed`: Certificate not trusted
    /// - `certificate expired`: Certificate has expired
    /// - `hostname mismatch`: Certificate not issued for this server
    Require,
}

impl Default for SslMode {
    fn default() -> Self {
        SslMode::Prefer
    }
}

impl DMSCDatabaseConfig {
    /// Creates a configuration for PostgreSQL with default settings.
    ///
    /// This factory method initializes a configuration with sensible defaults
    /// for PostgreSQL connections. Default values can be overridden using
    /// environment variables or the builder methods.
    ///
    /// ## Defaults
    ///
    /// - Host: `localhost` (or `DMSC_DB_HOST` env var)
    /// - Port: `5432` (or `DMSC_DB_PORT` env var)
    /// - Database: `dmsc` (or `DMSC_DB_NAME` env var)
    /// - Username: `dmsc` (or `DMSC_DB_USER` env var)
    /// - Password: empty (or `DMSC_DB_PASSWORD` env var)
    /// - Max connections: 10
    /// - Min idle: 2
    /// - Connection timeout: 30 seconds
    /// - Idle timeout: 600 seconds
    /// - Max lifetime: 3600 seconds
    /// - SSL mode: `Prefer`
    /// - Statement cache: 100
    ///
    /// ## Environment Variable Override
    ///
    /// Default values are read from environment variables if available:
    /// ```bash
    /// export DMSC_DB_HOST=db.example.com
    /// export DMSC_DB_PORT=5432
    /// export DMSC_DB_NAME=myapp
    /// export DMSC_DB_USER=admin
    /// export DMSC_DB_PASSWORD=secret
    /// ```
    ///
    /// # Returns
    ///
    /// A new `DMSCDatabaseConfig` instance configured for PostgreSQL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// // Basic PostgreSQL configuration
    /// let config = DMSCDatabaseConfig::postgres();
    ///
    /// // With environment variable overrides
    /// // (assumes DMSC_DB_* variables are set)
    /// let config = DMSCDatabaseConfig::postgres();
    /// ```
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

    /// Creates a configuration for MySQL with default settings.
    ///
    /// This factory method initializes a configuration with sensible defaults
    /// for MySQL connections. Default values can be overridden using
    /// environment variables or the builder methods.
    ///
    /// ## Defaults
    ///
    /// - Host: `localhost` (or `DMSC_DB_HOST` env var)
    /// - Port: `3306` (or `DMSC_DB_PORT` env var)
    /// - Database: `dmsc` (or `DMSC_DB_NAME` env var)
    /// - Username: `dmsc` (or `DMSC_DB_USER` env var)
    /// - Password: empty (or `DMSC_DB_PASSWORD` env var)
    /// - Max connections: 10
    /// - Min idle: 2
    /// - Connection timeout: 30 seconds
    /// - Idle timeout: 600 seconds
    /// - Max lifetime: 3600 seconds
    /// - SSL mode: `Prefer`
    /// - Statement cache: 100
    ///
    /// ## MySQL-Specific Notes
    ///
    /// - MySQL uses `mysql://` URI scheme in connection strings
    /// - MySQL 8.0+ uses `caching_sha2_password` by default
    /// - Consider using `SslMode::Require` for production
    ///
    /// # Returns
    ///
    /// A new `DMSCDatabaseConfig` instance configured for MySQL
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// // Basic MySQL configuration
    /// let config = DMSCDatabaseConfig::mysql();
    ///
    /// // Customized configuration
    /// let config = DMSCDatabaseConfig::mysql()
    ///     .host("db.example.com")
    ///     .database("myapp")
    ///     .user("app_user")
    ///     .password("secure_password");
    /// ```
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

    /// Creates a configuration for SQLite at the specified path.
    ///
    /// This factory method initializes a configuration for SQLite database
    /// at the given file path. SQLite is a serverless database that stores
    /// data in a single file.
    ///
    /// ## Special Considerations
    ///
    /// - The `host` and `port` fields are ignored
    /// - The `username` and `password` fields are ignored
    /// - The `ssl_mode` field is ignored
    /// - File path can be `:memory:` for in-memory database
    ///
    /// ## Path Handling
    ///
    /// - Relative paths are resolved relative to the current working directory
    /// - Parent directories are created automatically if they don't exist
    /// - Use absolute paths for reliability in production
    ///
    /// ## File Permissions
    ///
    /// The SQLite file and its directory must be writable by the application.
    /// Consider the following:
    /// - The application user needs write permission to the database file
    /// - The directory containing the database must be writable (for journal files)
    /// - Consider file permissions (0600 recommended for the database file)
    ///
    /// # Arguments
    ///
    /// * `path` - File path for the SQLite database (or `:memory:` for in-memory)
    ///
    /// # Returns
    ///
    /// A new `DMSCDatabaseConfig` instance configured for SQLite
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// // File-based database
    /// let config = DMSCDatabaseConfig::sqlite("./data/myapp.db");
    ///
    /// // In-memory database (for testing)
    /// let config = DMSCDatabaseConfig::sqlite(":memory:");
    ///
    /// // Absolute path
    /// let config = DMSCDatabaseConfig::sqlite("/var/lib/dmsc/database.db");
    /// ```
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

    /// Sets the database server hostname.
    ///
    /// This method configures the host address for database connections.
    /// It accepts hostnames, domain names, and IP addresses.
    ///
    /// # Arguments
    ///
    /// * `host` - The hostname or IP address of the database server
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .host("db.example.com");
    ///
    /// let config = DMSCDatabaseConfig::mysql()
    ///     .host("192.168.1.100");
    /// ```
    pub fn host(mut self, host: &str) -> Self {
        self.host = host.to_string();
        self
    }

    /// Sets the database server port.
    ///
    /// This method configures the port number for database connections.
    /// Each database type has a default port, but this can be overridden
    /// for non-standard configurations or when using database proxies.
    ///
    /// # Arguments
    ///
    /// * `port` - The port number for database connections (1-65535)
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Common Ports
    ///
    /// - PostgreSQL: 5432
    /// - MySQL: 3306
    /// - MongoDB: 27017
    /// - Redis: 6379
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// // Non-standard PostgreSQL port
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .port(15432);
    /// ```
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the database name.
    ///
    /// This method configures the name of the database to connect to.
    /// For PostgreSQL and MySQL, this is the logical database name.
    /// For SQLite, use the `sqlite()` constructor instead.
    ///
    /// # Arguments
    ///
    /// * `database` - The name of the database to connect to
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .database("production_db");
    /// ```
    pub fn database(mut self, database: &str) -> Self {
        self.database = database.to_string();
        self
    }

    /// Sets the database username.
    ///
    /// This method configures the username for database authentication.
    /// The specified user must have sufficient privileges to perform
    /// the required database operations.
    ///
    /// # Arguments
    ///
    /// * `user` - The username for database authentication
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Security Note
    ///
    /// For security, consider using environment variables instead of
    /// hardcoding credentials in your code:
    /// ```rust
    /// use std::env;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .user(&env::var("DB_USER").unwrap());
    /// ```
    pub fn user(mut self, user: &str) -> Self {
        self.username = user.to_string();
        self
    }

    /// Sets the database password.
    ///
    /// This method configures the password for database authentication.
    /// The password is used together with the username to authenticate
    /// with the database server.
    ///
    /// # Arguments
    ///
    /// * `password` - The password for database authentication
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Security Warning
    ///
    /// **Never** hardcode passwords in your source code. Use:
    /// - Environment variables
    /// - Secret management services (AWS Secrets Manager, HashiCorp Vault)
    /// - Configuration files with restricted permissions
    ///
    /// # Examples
    ///
    /// ```rust
    /// use std::env;
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .password(&env::var("DB_PASSWORD").unwrap());
    /// ```
    pub fn password(mut self, password: &str) -> Self {
        self.password = password.to_string();
        self
    }

    /// Sets the maximum number of concurrent connections.
    ///
    /// This method configures the upper bound of the connection pool.
    /// The pool will not create more than this number of connections,
    /// even under heavy load.
    ///
    /// # Arguments
    ///
    /// * `max` - Maximum number of concurrent connections (minimum 1)
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Performance Considerations
    ///
    /// - Higher values allow more concurrent database operations
    /// - Each connection consumes memory on both client and server
    /// - Database server has its own connection limits (e.g., PostgreSQL's `max_connections`)
    /// - Consider using connection pooling middleware for very high concurrency
    ///
    /// # Recommendations
    ///
    /// - Development: 5-10 connections
    /// - Production: 10-50 connections (depends on workload)
    /// - Monitor database server connection counts
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }

    /// Sets the minimum number of idle connections.
    ///
    /// This method configures the minimum number of idle connections
    /// that the pool will maintain. Having idle connections ready reduces
    /// the latency of new database operations.
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum number of idle connections (must be <= max_connections)
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Trade-offs
    ///
    /// - Benefits: Reduced latency for new operations
    /// - Cost: Increased memory usage and database server load
    ///
    /// # Recommendations
    ///
    /// - Set to expected concurrency level for best latency
    /// - Or use a small value (1-2) if memory is constrained
    pub fn min_idle_connections(mut self, min: u32) -> Self {
        self.min_idle_connections = min;
        self
    }

    /// Sets the connection timeout in seconds.
    ///
    /// This method configures the maximum time to wait when establishing
    /// a new database connection. If a connection cannot be established
    /// within this time, the operation will fail with a timeout error.
    ///
    /// # Arguments
    ///
    /// * `secs` - Timeout in seconds for connection establishment
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Considerations
    ///
    /// - Too short: May cause false failures under normal load
    /// - Too long: May hide database server problems
    /// - Consider network latency to database server
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// // 60 second timeout for distant databases
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .connection_timeout_secs(60);
    /// ```
    pub fn connection_timeout_secs(mut self, secs: u64) -> Self {
        self.connection_timeout_secs = secs;
        self
    }

    /// Sets the idle connection timeout in seconds.
    ///
    /// This method configures how long an idle connection can exist
    /// before being closed. Idle connections are those that have been
    /// checked back into the pool but not reused.
    ///
    /// # Arguments
    ///
    /// * `secs` - Maximum idle time in seconds before connection is closed
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Purpose
    ///
    /// - Frees resources on both client and database server
    /// - Handles database server connection timeouts
    /// - Reconnects after network interruptions
    ///
    /// # Recommendations
    ///
    /// - 600 seconds (10 minutes) for typical web applications
    /// - 300 seconds (5 minutes) for batch processing
    /// - Consider database server's `wait_timeout` setting (MySQL)
    pub fn idle_timeout_secs(mut self, secs: u64) -> Self {
        self.idle_timeout_secs = secs;
        self
    }

    /// Sets the maximum connection lifetime in seconds.
    ///
    /// This method configures the maximum age of a connection.
    /// Connections older than this will be closed and replaced with
    /// new ones when they are returned to the pool.
    ///
    /// # Arguments
    ///
    /// * `secs` - Maximum connection lifetime in seconds
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Purpose
    ///
    /// - Prevents stale connections from database server timeouts
    /// - Handles database server restarts gracefully
    /// - Rotates connections to handle network interruptions
    ///
    /// # Recommendations
    ///
    /// - 1800-3600 seconds (30-60 minutes) for most applications
    /// - Shorter values for very long-running applications
    /// - Disable by using a very large value if needed
    pub fn max_lifetime_secs(mut self, secs: u64) -> Self {
        self.max_lifetime_secs = secs;
        self
    }

    /// Sets the SSL/TLS mode for connections.
    ///
    /// This method configures whether and how SSL/TLS encryption is used
    /// for database connections. For production environments, `SslMode::Require`
    /// is recommended to ensure all data is encrypted.
    ///
    /// # Arguments
    ///
    /// * `mode` - The SSL/TLS mode to use
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::{DMSCDatabaseConfig, SslMode};
    ///
    /// // Require SSL for production
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .ssl_mode(SslMode::Require);
    /// ```
    pub fn ssl_mode(mut self, mode: SslMode) -> Self {
        self.ssl_mode = mode;
        self
    }

    /// Sets the prepared statement cache size.
    ///
    /// This method configures the maximum number of prepared statements
    /// to cache. Prepared statements are cached to reduce the overhead
    /// of repeated query compilation.
    ///
    /// # Arguments
    ///
    /// * `size` - Maximum number of prepared statements to cache (0 to disable)
    ///
    /// # Returns
    ///
    /// The updated configuration (for method chaining)
    ///
    /// # Performance Impact
    ///
    /// - Benefit: Reduces query compilation overhead for repeated queries
    /// - Cost: Increased memory usage for statement metadata
    /// - Trade-off: Balance between memory and CPU usage
    ///
    /// # Recommendations
    ///
    /// - 100-500 for typical applications
    /// - 1000+ for applications with many repeated complex queries
    /// - 0 to disable statement caching (for debugging)
    pub fn statement_cache_size(mut self, size: u32) -> Self {
        self.statement_cache_size = size;
        self
    }

    /// Builds the final configuration.
    ///
    /// This method finalizes the configuration and returns the complete
    /// `DMSCDatabaseConfig` instance. It is the terminal method in the
    /// builder chain.
    ///
    /// # Returns
    ///
    /// The complete configuration ready for use with `DMSCDatabaseManager`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .host("localhost")
    ///     .database("myapp")
    ///     .user("app")
    ///     .password("secret")
    ///     .max_connections(10)
    ///     .build();
    /// ```
    pub fn build(self) -> DMSCDatabaseConfig {
        self
    }

    /// Generates a connection string for the configured database.
    ///
    /// This method creates a database-specific connection string URI
    /// that can be used with various database client libraries.
    ///
    /// # Returns
    ///
    /// A String containing the connection string
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::database::DMSCDatabaseConfig;
    ///
    /// let config = DMSCDatabaseConfig::postgres()
    ///     .host("localhost")
    ///     .port(5432)
    ///     .database("myapp")
    ///     .user("app")
    ///     .password("secret");
    ///
    /// let connection_string = config.connection_string();
    /// // postgresql://app:secret@localhost:5432/myapp
    /// ```
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
            DatabaseType::MongoDB => {
                format!(
                    "mongodb://{}:{}@{}:{}/{}",
                    self.username, self.password, self.host, self.port, self.database
                )
            }
            DatabaseType::Redis => {
                format!(
                    "redis://{}:{}@{}:{}",
                    self.username, self.password, self.host, self.port
                )
            }
        }
    }
}

impl Default for DMSCDatabaseConfig {
    fn default() -> Self {
        Self::postgres()
    }
}
