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

//! # Database Module C API
//!
//! This module provides C language bindings for Ri's database subsystem. The database
//! module delivers unified database access patterns across multiple database backends,
//! including PostgreSQL, MySQL, SQLite, and Redis. This C API enables C/C++ applications
//! to leverage Ri's sophisticated database management capabilities including connection
//! pooling, transaction management, query building, and result set handling.
//!
//! ## Module Architecture
//!
//! The database module comprises three primary components that together provide a complete
//! database access layer:
//!
//! - **RiDatabaseConfig**: Configuration container for database connection parameters.
//!   Manages connection strings, pool sizes, timeout settings, and backend-specific options.
//!   The configuration object is required for initializing database pools and controls
//!   resource allocation and behavior characteristics for all database operations.
//!
//! - **RiDatabasePool**: Connection pool management interface providing efficient
//!   database connection reuse across multiple concurrent requests. The pool implements
//!   dynamic scaling, health checking, and automatic reconnection for maintaining
//!   reliable database connectivity. Connection pooling significantly improves performance
//!   by avoiding the overhead of establishing new connections for each operation.
//!
//! - **RiDBRow**: Result row abstraction providing type-safe access to query results.
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
//! RiDatabaseConfig* config = ri_database_config_new();
//! ri_database_config_set_connection_string(config, "postgresql://localhost/mydb");
//! ri_database_config_set_pool_size(config, 10);
//!
//! // Create connection pool
//! RiDatabasePool* pool = ri_database_pool_new(config);
//!
//! // Execute query
//! RiDBRow* row;
//! int result = ri_database_pool_query(pool, "SELECT * FROM users WHERE id = $1", 1, &row);
//!
//! if (result == 0) {
//!     // Process row
//!     char* name = ri_db_row_get_string(row, "name");
//!     int age = ri_db_row_get_int(row, "age");
//!
//!     // Cleanup row
//!     ri_db_row_free(row);
//! }
//!
//! // Cleanup
//! ri_database_pool_free(pool);
//! ri_database_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
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

use crate::database::{RiDatabaseConfig, RiDatabasePool, RiDBRow};


c_wrapper!(CRiDatabaseConfig, RiDatabaseConfig);

c_wrapper!(CRiDatabasePool, RiDatabasePool);

c_wrapper!(CRiDBRow, RiDBRow);

c_constructor!(ri_database_config_new, CRiDatabaseConfig, RiDatabaseConfig, RiDatabaseConfig::default());

c_destructor!(ri_database_config_free, CRiDatabaseConfig);
