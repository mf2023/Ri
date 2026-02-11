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

//! # Database Module C API
//!
//! This module provides C language bindings for DMSC's database subsystem. The database
//! module delivers unified database access patterns across multiple database backends,
//! including PostgreSQL, MySQL, SQLite, and Redis. This C API enables C/C++ applications
//! to leverage DMSC's sophisticated database management capabilities including connection
//! pooling, transaction management, query building, and result set handling.
//!
//! ## Module Architecture
//!
//! The database module comprises three primary components that together provide a complete
//! database access layer:
//!
//! - **DMSCDatabaseConfig**: Configuration container for database connection parameters.
//!   Manages connection strings, pool sizes, timeout settings, and backend-specific options.
//!   The configuration object is required for initializing database pools and controls
//!   resource allocation and behavior characteristics for all database operations.
//!
//! - **DMSCDatabasePool**: Connection pool management interface providing efficient
//!   database connection reuse across multiple concurrent requests. The pool implements
//!   dynamic scaling, health checking, and automatic reconnection for maintaining
//!   reliable database connectivity. Connection pooling significantly improves performance
//!   by avoiding the overhead of establishing new connections for each operation.
//!
//! - **DMSCDBRow**: Result row abstraction providing type-safe access to query results.
//!   The row object supports column-by-column access with automatic type conversion.
//!   Multiple rows are typically returned as a collection that can be iterated efficiently.
//!
//! ## Supported Databases
//!
//! The database module provides native support for:
//!
//! - **PostgreSQL**: Full-featured relational database with advanced data types,
//!   JSON support, and powerful query capabilities. Accessed via sqlx with
//!   async/await support and connection pooling.
//!
//! - **MySQL**: Popular relational database with high performance and wide adoption.
//!   Supports replication, partitioning, and stored procedures through sqlx.
//!
//! - **SQLite**: Embedded database requiring no separate server process. Ideal for
//!   local storage, testing, and applications with modest concurrency requirements.
//!
//! - **Redis**: In-memory data store used for caching, session storage, and
//!   message queuing. Accessed through the Redis protocol with pub/sub support.
//!
//! ## Connection Pooling
//!
//! The connection pool implementation provides:
//!
//! - **Dynamic Sizing**: Pool size adjusts based on demand up to configured maximum.
//!   Idle connections are released when not needed to conserve resources.
//!
//! - **Health Checking**: Connections are validated before use to detect stale or
//!   broken connections. Failed connections are automatically recreated.
//!
//! - **Connection Lifetime**: Connections have maximum lifetime limits to prevent
//!   resource exhaustion. Long-lived connections are periodically refreshed.
//!
//! - **Wait Semantics**: When pool is exhausted, requests can either wait for a
//!   connection or fail immediately based on configuration.
//!
//! ## Transaction Management
//!
//! Full transaction support includes:
//!
//! - **Explicit Transactions**: BEGIN, COMMIT, ROLLBACK control for fine-grained
//!   transaction boundaries.
//!
//! - **Savepoints**: Nested transaction support with savepoint rollback capabilities
//!   for partial transaction recovery.
//!
//! - **Isolation Levels**: Configurable isolation levels (Read Committed, Repeatable
//!   Read, Serializable) matching database capabilities.
//!
//! - **Auto-Commit Mode**: Per-statement execution with automatic commit for
//!   simple operations without transaction overhead.
//!
//! ## Query Operations
//!
//! The module provides comprehensive query capabilities:
//!
//! - **Prepared Statements**: Pre-compiled queries for repeated execution with
//!   parameter binding and automatic type conversion.
//!
//! - **Query Builder**: Fluent API for constructing queries programmatically
//!   without raw SQL string manipulation.
//!
//! - **Batch Operations**: Efficient bulk inserts and updates with transaction
//!   batching for high-throughput data loading.
//!
//! - **Streaming Results**: Large result sets can be streamed row-by-row to
//!   minimize memory footprint for big queries.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Result sets must be properly iterated and freed
//! - Null pointer checks are required before all operations
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Connection pools support concurrent access from multiple threads
//! - Individual connections are not thread-safe (use pool for concurrency)
//! - Query execution and result processing require synchronization
//!
//! ## Performance Characteristics
//!
//! Database operations have the following performance profiles:
//!
//! - Connection acquisition: O(1) average case
//! - Simple query execution: O(log n) for query planning, O(n) for results
//! - Bulk operations: O(n) with batching optimizations
//! - Connection reuse: Eliminates ~10ms connection establishment overhead
//!
//! ## Error Handling
//!
//! Database operations use error codes and optional error messages:
//!
//! - Success/failure indication through return values
//! - Detailed error messages available for debugging
//! - Connection failures trigger automatic retry (configurable)
//! - Deadlock detection and transaction restart
//!
//! ## Usage Example
//!
//! ```c
//! // Create database configuration
//! DMSCDatabaseConfig* config = dmsc_database_config_new();
//! dmsc_database_config_set_connection_string(config, "postgresql://localhost/mydb");
//! dmsc_database_config_set_pool_size(config, 10);
//!
//! // Create connection pool
//! DMSCDatabasePool* pool = dmsc_database_pool_new(config);
//!
//! // Execute query
//! DMSCDBRow* row;
//! int result = dmsc_database_pool_query(pool, "SELECT * FROM users WHERE id = $1", 1, &row);
//!
//! if (result == 0) {
//!     // Process row
//!     char* name = dmsc_db_row_get_string(row, "name");
//!     int age = dmsc_db_row_get_int(row, "age");
//!
//!     // Cleanup row
//!     dmsc_db_row_free(row);
//! }
//!
//! // Cleanup
//! dmsc_database_pool_free(pool);
//! dmsc_database_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::database`: Rust database module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! Database support is enabled through individual feature flags:
//!
//! - "postgres": PostgreSQL database support
//! - "mysql": MySQL database support
//! - "sqlite": SQLite database support
//! - Disable features to reduce binary size

use crate::database::{DMSCDatabaseConfig, DMSCDatabasePool, DMSCDBRow};


/// Opaque C wrapper structure for DMSCDatabaseConfig.
///
/// Provides C-compatible memory layout for database configuration parameters.
/// The wrapper encapsulates all database connection settings including connection
/// strings, pool sizing, timeout values, and backend-specific options.
///
/// # Configuration Parameters
///
/// The database configuration controls the following aspects:
///
/// - **Connection String**: Database connection URI containing protocol, host, port,
///   database name, credentials, and optional parameters. Format varies by database:
///   - PostgreSQL: postgresql://[user[:password]@][host[:port],]/dbname[?params]
///   - MySQL: mysql://[user[:password]@][host[:port]]/dbname[?params]
///   - SQLite: file:path/to/database.db
///
/// - **Pool Size**: Maximum number of concurrent connections in the pool.
///   Default is typically 10 connections. Optimal sizing depends on workload
///   characteristics and database connection limits.
///
/// - **Connection Timeout**: Maximum time to wait for a connection from pool.
///   Prevents indefinite blocking when pool is exhausted.
///
/// - **Idle Timeout**: Maximum time a connection can remain idle before being
///   released. Prevents connection stagnation during low-activity periods.
///
/// - **Max Lifetime**: Maximum lifetime for any single connection. Connections
///   exceeding this age are replaced to prevent issues from accumulated state.
///
/// # Memory Layout
///
/// The structure uses #[repr(C)] ensuring binary compatibility:
/// - Consistent field alignment across Rust versions
/// - Predictable size for FFI boundaries
/// - No hidden padding affecting pointer arithmetic
c_wrapper!(CDMSCDatabaseConfig, DMSCDatabaseConfig);

/// Opaque C wrapper structure for DMSCDatabasePool.
///
/// High-performance connection pool management interface. The pool provides efficient
/// database connection reuse through a managed collection of pre-established connections.
///
/// # Pool Management
///
/// The database pool handles:
///
/// - **Connection Acquisition**: Obtain available connection from pool or wait
///   until one becomes available based on timeout configuration.
/// - **Connection Return**: Release connection back to pool for reuse after use.
/// - **Health Monitoring**: Periodically verify connection health and replace
///   failed connections.
/// - **Dynamic Scaling**: Adjust pool size based on demand within configured limits.
///
/// # Pool States
///
/// Connections in the pool transition through states:
///
/// 1. **IDLE**: Available for immediate use by new requests
/// 2. **IN_USE**: Currently executing a database operation
/// 3. **VALIDATING**: Being checked for health before reuse
/// 4. **FAILED**: Connection test failed, pending removal
/// 5. **REMOVED**: Connection closed and removed from pool
///
/// # Thread Safety
///
/// The connection pool is fully thread-safe:
///
/// - Multiple threads can acquire and release connections concurrently
/// - Internal synchronization prevents race conditions
/// - Lock-free data structures minimize contention
/// - Wait queues properly handle concurrent waiters
///
/// # Performance Optimization
///
/// The pool implements several performance optimizations:
///
/// - **Prefetching**: Anticipatory connection creation during low activity
/// - **Keep-Alive**: Periodic minimal queries to prevent connection timeout
/// - **Batching**: Multiple operations can share connection lifecycle
/// - **Lazy Validation**: Connections validated on acquire only when needed
c_wrapper!(CDMSCDatabasePool, DMSCDatabasePool);

/// Opaque C wrapper structure for DMSCDBRow.
///
/// Result row abstraction providing type-safe column access. Each row represents
/// a single record returned from a query result set.
///
/// # Row Access
///
/// Columns can be accessed by name or zero-based index:
///
/// - **By Name**: dmsc_db_row_get_string(row, "column_name")
/// - **By Index**: dmsc_db_row_get_string_at(row, 0)
///
/// Column access supports automatic type conversion:
///
/// - **String**: VARCHAR, TEXT, CHAR types
/// - **Integer**: INT, BIGINT, SMALLINT types
/// - **Float**: FLOAT, DOUBLE, DECIMAL types
/// - **Boolean**: BOOLEAN, BIT types
/// - **Binary**: BLOB, BYTEA, BINARY types
///
/// # Null Handling
///
/// Database NULL values are distinguished from empty strings or zero values:
///
/// - Nullable columns return special null marker
/// - Type conversion from null uses configurable defaults
/// - NULL checking available before value extraction
///
/// # Lifecycle
///
/// Row objects are typically obtained from query results:
///
/// 1. Query execution returns result set
/// 2. Rows are iterated (or accessed directly for single-row results)
/// 3. Column values are extracted from each row
/// 4. Row is released (freed) before processing next row
///
/// # Memory Ownership
///
/// Row objects hold borrowed references to underlying result buffers:
///
/// - Row validity limited to result set lifetime
/// - Values must be copied for persistent use
/// - Do not free row objects from query result directly
c_wrapper!(CDMSCDBRow, DMSCDBRow);

/// Creates a new CDMSCDatabaseConfig instance with default configuration values.
///
/// Initializes a database configuration object with sensible production defaults:
/// - Default pool size: 10 connections
/// - Default connection timeout: 30 seconds
/// - Default idle timeout: 10 minutes
/// - Default max lifetime: 1 hour
/// - Default auto-validation: enabled
///
/// # Returns
///
/// Pointer to newly allocated DMSCDatabaseConfig on success, or NULL if memory
/// allocation fails. The returned pointer must be freed using dmsc_database_config_free().
///
/// # Default Configuration
///
/// The default configuration is suitable for moderate workloads:
/// - Ten connections handle dozens of concurrent requests
/// - Thirty-second timeout prevents indefinite blocking
/// - Ten-minute idle release balances resource usage and latency
/// - One-hour lifetime prevents connection staleness
///
/// # Customization
///
/// After creation, configuration can be customized:
/// - dmsc_database_config_set_connection_string() for database URI
/// - dmsc_database_config_set_pool_size() for concurrency tuning
/// - dmsc_database_config_set_timeout() for timeout adjustment
/// - dmsc_database_config_set_ssl_mode() for encryption settings
c_constructor!(dmsc_database_config_new, CDMSCDatabaseConfig, DMSCDatabaseConfig, DMSCDatabaseConfig::default());

/// Frees a previously allocated DMSCDatabaseConfig instance.
///
/// Releases all memory associated with the configuration object including any
/// internally allocated connection strings, SSL certificates, or sub-objects.
///
/// # Parameters
///
/// - `config`: Pointer to DMSCDatabaseConfig to free. NULL is safe and returns immediately.
///
/// # Safety
///
/// Safe to call with NULL. Calling with already-freed pointer is undefined behavior.
/// Implement proper ownership tracking to prevent double-free vulnerabilities.
c_destructor!(dmsc_database_config_free, CDMSCDatabaseConfig);
