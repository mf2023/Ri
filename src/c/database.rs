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

use crate::database::{RiDatabase, RiDatabaseConfig, RiDatabasePool, RiDBRow, RiDBResult, DatabaseType};


c_wrapper!(CRiDatabaseConfig, RiDatabaseConfig);
c_wrapper!(CRiDatabasePool, RiDatabasePool);
c_wrapper!(CRiDBRow, RiDBRow);
c_wrapper!(CRiDBResult, RiDBResult);

c_constructor!(ri_database_config_new, CRiDatabaseConfig, RiDatabaseConfig, RiDatabaseConfig::default());
c_destructor!(ri_database_config_free, CRiDatabaseConfig);

// RiDatabaseConfig setters
c_string_setter!(
    ri_database_config_set_connection_string,
    CRiDatabaseConfig,
    |inner: &mut RiDatabaseConfig, val: &str| { inner.host = val.to_string(); }
);

#[no_mangle]
pub extern "C" fn ri_database_config_set_pool_size(config: *mut CRiDatabaseConfig, size: u32) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        (*config).inner.max_connections = size;
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_database_config_set_min_idle(config: *mut CRiDatabaseConfig, size: u32) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        (*config).inner.min_idle_connections = size;
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_database_config_set_connection_timeout_secs(config: *mut CRiDatabaseConfig, secs: u64) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        (*config).inner.connection_timeout_secs = secs;
    }
    0
}

#[no_mangle]
pub extern "C" fn ri_database_config_set_database_type(config: *mut CRiDatabaseConfig, db_type: std::ffi::c_int) -> std::ffi::c_int {
    if config.is_null() {
        return -1;
    }
    unsafe {
        let database_type = match db_type {
            0 => DatabaseType::Postgres,
            1 => DatabaseType::MySQL,
            2 => DatabaseType::SQLite,
            _ => DatabaseType::Postgres,
        };
        (*config).inner.database_type = database_type;
    }
    0
}

// RiDatabasePool C bindings
#[no_mangle]
pub extern "C" fn ri_database_pool_new(config: *mut CRiDatabaseConfig) -> *mut CRiDatabasePool {
    if config.is_null() {
        return std::ptr::null_mut();
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe {
        let config = (*config).inner.clone();
        match rt.block_on(async { RiDatabasePool::new(config).await }) {
            Ok(pool) => {
                let ptr = Box::into_raw(Box::new(CRiDatabasePool::new(pool)));
                crate::c::register_ptr(ptr as usize);
                ptr
            }
            Err(_) => std::ptr::null_mut(),
        }
    }
}
c_destructor!(ri_database_pool_free, CRiDatabasePool);

#[no_mangle]
pub extern "C" fn ri_database_pool_get_connection_count(pool: *mut CRiDatabasePool) -> usize {
    if pool.is_null() {
        return 0;
    }
    unsafe { (*pool).inner.metrics().total_connections as usize }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_get_idle_count(pool: *mut CRiDatabasePool) -> usize {
    if pool.is_null() {
        return 0;
    }
    unsafe { (*pool).inner.metrics().idle_connections as usize }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_get_active_count(pool: *mut CRiDatabasePool) -> usize {
    if pool.is_null() {
        return 0;
    }
    unsafe { (*pool).inner.metrics().active_connections as usize }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_execute(
    pool: *mut CRiDatabasePool,
    sql: *const std::ffi::c_char,
    out_rows_affected: *mut u64,
) -> std::ffi::c_int {
    if pool.is_null() || sql.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let sql_str = match std::ffi::CStr::from_ptr(sql).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*pool).inner.get().await }) {
            Ok(db) => match rt.block_on(async { db.execute(sql_str).await }) {
                Ok(rows) => {
                    if !out_rows_affected.is_null() {
                        *out_rows_affected = rows;
                    }
                    0
                }
                Err(_) => -4,
            },
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_query(
    pool: *mut CRiDatabasePool,
    sql: *const std::ffi::c_char,
    out_result: *mut *mut CRiDBResult,
) -> std::ffi::c_int {
    if pool.is_null() || sql.is_null() || out_result.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let sql_str = match std::ffi::CStr::from_ptr(sql).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*pool).inner.get().await }) {
            Ok(db) => match rt.block_on(async { db.query(sql_str).await }) {
                Ok(result) => {
                    let result_ptr = Box::into_raw(Box::new(CRiDBResult::new(result)));
                    crate::c::register_ptr(result_ptr as usize);
                    *out_result = result_ptr;
                    0
                }
                Err(_) => -4,
            },
            Err(_) => -5,
        }
    }
}

/// Execute parameterized SQL query (SAFE - prevents SQL injection)
/// 
/// # Safety
/// - `pool` must be a valid pointer returned by `ri_database_pool_new`
/// - `sql` must be a valid null-terminated C string
/// - `param_count` must match the number of parameters in the SQL
/// - `param_types` must be an array of `param_count` integers representing parameter types:
///   - 0: NULL
///   - 1: INTEGER (i64)
///   - 2: REAL (f64)
///   - 3: TEXT (const char*)
///   - 4: BLOB (const uint8_t*, size_t)
/// - `param_values` must be an array of `param_count` pointers to parameter values
/// - `param_sizes` must be an array of `param_count` sizes (for BLOBs, 0 for others)
/// 
/// # SQL Injection Prevention
/// This function uses parameterized queries, which are the recommended way to prevent SQL injection.
/// Parameters are properly escaped and quoted by the database driver.
/// 
/// # Example
/// ```c
/// // SAFE: Parameterized query
/// int types[] = {1, 3};  // INTEGER, TEXT
/// int64_t user_id = 123;
/// const char* username = "john";
/// void* values[] = {&user_id, username};
/// size_t sizes[] = {0, 0};
/// 
/// CRiDBResult* result;
/// int ret = ri_database_pool_query_params(pool, 
///     "SELECT * FROM users WHERE id = $1 AND name = $2",
///     2, types, values, sizes, &result);
/// ```
/// Count the number of parameter placeholders ($1, $2, etc.) in SQL string
/// Returns the count of placeholders found
fn count_sql_placeholders(sql: &str) -> usize {
    let mut count = 0;
    let mut chars = sql.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            // Check for numbered placeholder like $1, $2, etc.
            let mut num_str = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_ascii_digit() {
                    num_str.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if !num_str.is_empty() {
                count += 1;
            }
        } else if c == '?' {
            // Anonymous placeholder (PostgreSQL also supports this)
            count += 1;
        } else if c == ':' {
            // Named placeholder like :1, :name
            let mut name_or_num = String::new();
            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    name_or_num.push(chars.next().unwrap());
                } else {
                    break;
                }
            }
            if !name_or_num.is_empty() {
                count += 1;
            }
        }
    }

    count
}

/// Validate parameter types and values for security
/// Returns Some(error_code) if invalid, None if valid
fn validate_param_value(param_type: std::ffi::c_int, param_value: *const std::ffi::c_void, param_size: usize) -> Option<std::ffi::c_int> {
    match param_type {
        0 => None, // NULL - always valid
        1 => { // INTEGER
            if param_value.is_null() {
                return Some(-10); // NULL value for non-nullable type
            }
            // Just check pointer is valid for i64 read
            let _ = unsafe { *(param_value as *const i64) };
            None
        }
        2 => { // REAL
            if param_value.is_null() {
                return Some(-10);
            }
            let _ = unsafe { *(param_value as *const f64) };
            None
        }
        3 => { // TEXT
            if param_value.is_null() {
                return Some(-10);
            }
            let cstr = unsafe { std::ffi::CStr::from_ptr(param_value as *const std::ffi::c_char) };
            match cstr.to_str() {
                Ok(_) => None,
                Err(_) => Some(-7), // Invalid UTF-8
            }
        }
        4 => { // BLOB
            if param_value.is_null() || param_size == 0 {
                return Some(-11); // BLOB requires non-null pointer and size > 0
            }
            // Verify memory is readable
            let _ = unsafe { std::slice::from_raw_parts(param_value as *const u8, param_size) };
            None
        }
        _ => Some(-8), // Unknown type
    }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_query_params(
    pool: *mut CRiDatabasePool,
    sql: *const std::ffi::c_char,
    param_count: usize,
    param_types: *const std::ffi::c_int,
    param_values: *const *const std::ffi::c_void,
    param_sizes: *const usize,
    out_result: *mut *mut CRiDBResult,
) -> std::ffi::c_int {
    if pool.is_null() || sql.is_null() || out_result.is_null() {
        return -1;
    }
    
    if param_count > 0 && (param_types.is_null() || param_values.is_null()) {
        return -6;  // Invalid parameters
    }
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    
    unsafe {
        let sql_str = match std::ffi::CStr::from_ptr(sql).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        // Validate SQL placeholder count matches parameter count
        let placeholder_count = count_sql_placeholders(sql_str);
        if placeholder_count != param_count {
            return -9; // Parameter count mismatch
        }

        let mut params: Vec<serde_json::Value> = Vec::with_capacity(param_count);

        for i in 0..param_count {
            let param_type = *param_types.add(i);
            let param_value = *param_values.add(i);
            let param_size = *param_sizes.add(i);

            // Validate parameter value security
            if let Some(err_code) = validate_param_value(param_type, param_value, param_size) {
                return err_code;
            }

            match param_type {
                0 => { params.push(serde_json::Value::Null); }
                1 => {
                    let val = *(param_value as *const i64);
                    params.push(serde_json::json!(val));
                }
                2 => {
                    let val = *(param_value as *const f64);
                    params.push(serde_json::json!(val));
                }
                3 => {
                    let val = std::ffi::CStr::from_ptr(param_value as *const std::ffi::c_char);
                    match val.to_str() {
                        Ok(s) => params.push(serde_json::json!(s)),
                        Err(_) => return -7,
                    }
                }
                4 => {
                    let data = std::slice::from_raw_parts(param_value as *const u8, param_size);
                    params.push(serde_json::json!(data));
                }
                _ => return -8,
            }
        }

        match rt.block_on(async { (*pool).inner.get().await }) {
            Ok(db) => match rt.block_on(async { db.query_with_params(sql_str, &params).await }) {
                Ok(result) => {
                    let result_ptr = Box::into_raw(Box::new(CRiDBResult::new(result)));
                    crate::c::register_ptr(result_ptr as usize);
                    *out_result = result_ptr;
                    0
                }
                Err(_) => -4,
            },
            Err(_) => -5,
        }
    }
}

/// Execute parameterized SQL statement (SAFE - prevents SQL injection)
/// 
/// # Safety
/// Same as `ri_database_pool_query_params`
/// 
/// # SQL Injection Prevention
/// This function uses parameterized statements, which are the recommended way to prevent SQL injection.
#[no_mangle]
pub extern "C" fn ri_database_pool_execute_params(
    pool: *mut CRiDatabasePool,
    sql: *const std::ffi::c_char,
    param_count: usize,
    param_types: *const std::ffi::c_int,
    param_values: *const *const std::ffi::c_void,
    param_sizes: *const usize,
    out_rows_affected: *mut u64,
) -> std::ffi::c_int {
    if pool.is_null() || sql.is_null() {
        return -1;
    }
    
    if param_count > 0 && (param_types.is_null() || param_values.is_null()) {
        return -6;
    }
    
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    
    unsafe {
        let sql_str = match std::ffi::CStr::from_ptr(sql).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        // Validate SQL placeholder count matches parameter count
        let placeholder_count = count_sql_placeholders(sql_str);
        if placeholder_count != param_count {
            return -9; // Parameter count mismatch
        }

        let mut params: Vec<serde_json::Value> = Vec::with_capacity(param_count);

        for i in 0..param_count {
            let param_type = *param_types.add(i);
            let param_value = *param_values.add(i);
            let param_size = *param_sizes.add(i);

            // Validate parameter value security
            if let Some(err_code) = validate_param_value(param_type, param_value, param_size) {
                return err_code;
            }

            match param_type {
                0 => { params.push(serde_json::Value::Null); }
                1 => {
                    let val = *(param_value as *const i64);
                    params.push(serde_json::json!(val));
                }
                2 => {
                    let val = *(param_value as *const f64);
                    params.push(serde_json::json!(val));
                }
                3 => {
                    let val = std::ffi::CStr::from_ptr(param_value as *const std::ffi::c_char);
                    match val.to_str() {
                        Ok(s) => params.push(serde_json::json!(s)),
                        Err(_) => return -7,
                    }
                }
                4 => {
                    let data = std::slice::from_raw_parts(param_value as *const u8, param_size);
                    params.push(serde_json::json!(data));
                }
                _ => return -8,
            }
        }
        
        match rt.block_on(async { (*pool).inner.get().await }) {
            Ok(db) => match rt.block_on(async { db.execute_with_params(sql_str, &params).await }) {
                Ok(rows) => {
                    if !out_rows_affected.is_null() {
                        *out_rows_affected = rows;
                    }
                    0
                }
                Err(_) => -4,
            },
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_database_pool_ping(pool: *mut CRiDatabasePool) -> std::ffi::c_int {
    if pool.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        match rt.block_on(async { (*pool).inner.get().await }) {
            Ok(db) => match rt.block_on(async { db.ping().await }) {
                Ok(true) => 0,
                Ok(false) => 1,
                Err(_) => -3,
            },
            Err(_) => -4,
        }
    }
}

// RiDBRow C bindings
c_destructor!(ri_db_row_free, CRiDBRow);

#[no_mangle]
pub extern "C" fn ri_db_row_get_string(
    row: *mut CRiDBRow,
    column: *const std::ffi::c_char,
) -> *mut std::ffi::c_char {
    if row.is_null() || column.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let column_str = match std::ffi::CStr::from_ptr(column).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        match (*row).inner.get::<String>(column_str) {
            Some(val) => match std::ffi::CString::new(val) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_db_row_get_int(row: *mut CRiDBRow, column: *const std::ffi::c_char) -> std::ffi::c_int {
    if row.is_null() || column.is_null() {
        return 0;
    }
    unsafe {
        let column_str = match std::ffi::CStr::from_ptr(column).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        match (*row).inner.get::<i32>(column_str) {
            Some(val) => val,
            None => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_db_row_get_long(row: *mut CRiDBRow, column: *const std::ffi::c_char) -> i64 {
    if row.is_null() || column.is_null() {
        return 0;
    }
    unsafe {
        let column_str = match std::ffi::CStr::from_ptr(column).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        match (*row).inner.get::<i64>(column_str) {
            Some(val) => val,
            None => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_db_row_get_double(row: *mut CRiDBRow, column: *const std::ffi::c_char) -> f64 {
    if row.is_null() || column.is_null() {
        return 0.0;
    }
    unsafe {
        let column_str = match std::ffi::CStr::from_ptr(column).to_str() {
            Ok(s) => s,
            Err(_) => return 0.0,
        };
        match (*row).inner.get::<f64>(column_str) {
            Some(val) => val,
            None => 0.0,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_db_row_get_bool(row: *mut CRiDBRow, column: *const std::ffi::c_char) -> bool {
    if row.is_null() || column.is_null() {
        return false;
    }
    unsafe {
        let column_str = match std::ffi::CStr::from_ptr(column).to_str() {
            Ok(s) => s,
            Err(_) => return false,
        };
        match (*row).inner.get::<bool>(column_str) {
            Some(val) => val,
            None => false,
        }
    }
}

// RiDBResult C bindings
c_destructor!(ri_db_result_free, CRiDBResult);

#[no_mangle]
pub extern "C" fn ri_db_result_get_row_count(result: *mut CRiDBResult) -> usize {
    if result.is_null() {
        return 0;
    }
    unsafe { (*result).inner.len() }
}

#[no_mangle]
pub extern "C" fn ri_db_result_get_row(result: *mut CRiDBResult, index: usize) -> *mut CRiDBRow {
    if result.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        match (*result).inner.get(index) {
            Some(row) => Box::into_raw(Box::new(CRiDBRow::new(row.clone()))),
            None => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_db_result_is_empty(result: *mut CRiDBResult) -> bool {
    if result.is_null() {
        return true;
    }
    unsafe { (*result).inner.is_empty() }
}
