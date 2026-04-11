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

//! # Ri Database Module Example
//!
//! This example demonstrates how to use the database module in Ri,
//! including connection pool management, CRUD operations, and transactions.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example database --features postgres
//! ```
//!
//! ## Features Demonstrated
//!
//! - Connection pool configuration
//! - CRUD operations (Create, Read, Update, Delete)
//! - Parameterized queries to prevent SQL injection
//! - Transactions with rollback support
//! - Batch operations for bulk inserts

use ri::database::{RiDatabasePool, RiDatabaseConfig, RiDBRow};
use ri::core::RiResult;
use serde_json::json;

/// User struct for demo data mapping.
///
/// This struct represents the database table structure and is used
/// for deserializing query results into strongly-typed objects.
/// The derive macros provide Debug, Clone capabilities for the example.
#[derive(Debug, Clone)]
struct User {
    id: i64,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

/// Main entry point for the database module example.
///
/// This function demonstrates the complete database workflow including:
/// - Connection pool creation and configuration
/// - Schema creation (CREATE TABLE) for initializing database structure
/// - CRUD operations (Create, Read, Update, Delete) for data manipulation
/// - Parameterized query execution to prevent SQL injection attacks
/// - Transaction management with commit and rollback capabilities
/// - Batch operations for efficient bulk data insertion
///
/// The example uses PostgreSQL database to demonstrate Ri's database
/// capabilities including connection pooling, async query execution,
/// and type-safe result handling in Rust.
fn main() -> RiResult<()> {
    println!("=== Ri Database Module Example ===\n");

    // Create async runtime for handling asynchronous database operations
    // tokio::runtime::Runtime provides the executor for async/await code
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute all async database operations within the runtime
    rt.block_on(async {
        // Configuration Setup: Create database connection configuration
        // Using PostgreSQL-specific builder pattern for connection parameters
        // Each method call configures a specific parameter:
        // - host: Database server hostname or IP address
        // - port: Database server port (PostgreSQL default: 5432)
        // - database: Name of the database to connect to
        // - user: Database username for authentication
        // - password: Database password for authentication
        // - max_connections: Maximum number of connections in the pool
        // - min_idle_connections: Minimum number of idle connections to maintain
        // - connection_timeout_secs: Maximum time to wait for a connection
        // - build(): Finalizes configuration into RiDatabaseConfig struct
        let config = RiDatabaseConfig::postgres()
            .host("localhost")
            .port(5432)
            .database("ri")
            .user("postgres")
            .password("password")
            .max_connections(10)
            .min_idle_connections(2)
            .connection_timeout_secs(30)
            .build();

        // Step 1: Create connection pool
        // Connection pools manage database connections efficiently:
        // - Reuses connections to reduce connection overhead
        // - Limits total connections to prevent database overload
        // - Provides thread-safe access to connections
        // - Automatically handles connection health checks
        // The pool automatically creates initial connections on creation
        println!("1. Creating connection pool...");
        let pool = RiDatabasePool::new(config).await?;
        println!("   Connection pool created successfully\n");

        // Step 2: Create database schema (table)
        // Execute DDL (Data Definition Language) statement
        // CREATE TABLE IF NOT EXISTS ensures idempotent schema creation
        // This prevents errors if the table already exists
        println!("2. Creating table...");
        let db = pool.get().await?;
        db.execute("
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(100) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )
        ", &[]).await?;
        println!("   Table 'users' created or already exists\n");

        // Step 3: INSERT operation (Create)
        // Demonstrates parameterized query execution for data insertion
        // Parameters:
        // - query: SQL statement with $1, $2 placeholders
        // - params: Vector of parameter values for placeholders
        // The RETURNING clause retrieves the inserted record data
        // This is more efficient than separate INSERT + SELECT queries
        println!("3. Inserting new user (Create)...");
        let db = pool.get().await?;
        let rows = db.execute(
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email, created_at",
            &[&"John Doe", &"john@example.com"],
        ).await?;
        if let Some(row) = rows.first() {
            // Extract values from row using typed access methods
            // get<T>() converts database types to Rust types
            println!("   User inserted: id={}, name={}, email={}",
                row.get::<i64>("id"),
                row.get::<String>("name"),
                row.get::<String>("email")
            );
        }
        println!();

        // Step 4: SELECT operation (Read)
        // Demonstrates query execution to retrieve data
        // Returns vector of rows matching the query
        // Each row is a RiDBRow with typed access methods
        println!("4. Querying users (Read)...");
        let db = pool.get().await?;
        let rows = db.query("SELECT id, name, email FROM users ORDER BY id", &[]).await?;
        println!("   Found {} users:", rows.len());
        for row in &rows {
            // Iterate and print each user's details
            println!("   - id={}, name={}, email={}",
                row.get::<i64>("id"),
                row.get::<String>("name"),
                row.get::<String>("email")
            );
        }
        println!();

        // Step 5: UPDATE operation (Update)
        // Demonstrates data modification with parameterized queries
        // Returns number of affected rows (u64)
        // $1, $2 placeholders prevent SQL injection
        println!("5. Updating user (Update)...");
        let db = pool.get().await?;
        let affected = db.execute(
            "UPDATE users SET email = $1 WHERE name = $2",
            &[&"john.doe@example.com", &"John Doe"],
        ).await?;
        println!("   Updated {} row(s)\n", affected);

        // Step 6: Transaction management with rollback
        // Demonstrates ACID transaction properties:
        // - BEGIN: Transaction starts with begin_transaction()
        // - Execute multiple statements within transaction
        // - ROLLBACK: Discards all changes (atomicity guarantee)
        // Transactions ensure data consistency for complex operations
        println!("6. Using transaction (Rollback demo)...");
        let mut tx = pool.begin_transaction().await?;
        
        // Insert test user within transaction
        tx.execute("INSERT INTO users (name, email) VALUES ($1, $2)",
            &[&"Test User", &"test@example.com"]).await?;
        println!("   Inserted test user in transaction");
        
        // Rollback discards the insert
        // This demonstrates transaction atomicity
        tx.rollback().await?;
        println!("   Transaction rolled back\n");

        // Step 7: Verify rollback worked
        // Query to check if test user was actually added
        // This proves transaction rollback was successful
        println!("7. Verifying no test user was added...");
        let db = pool.get().await?;
        let rows = db.query("SELECT COUNT(*) as count FROM users WHERE email = $1",
            &[&"test@example.com"]).await?;
        // Extract count from first row
        let count: i64 = rows.first().unwrap().get("count");
        println!("   Users with test@example.com: {}\n", count);

        // Step 8: Batch insert operation
        // Demonstrates efficient bulk data insertion
        // execute_many() processes multiple rows in single call
        // More efficient than individual INSERT statements
        // Reduces database round-trips and improves performance
        println!("8. Batch insert users...");
        let db = pool.get().await?;
        let users = vec![
            // Each inner vector represents a row's parameters
            vec![json!("Alice"), json!("alice@example.com")],
            vec![json!("Bob"), json!("bob@example.com")],
            vec![json!("Charlie"), json!("charlie@example.com")],
        ];
        let inserted = db.execute_many(
            "INSERT INTO users (name, email) VALUES ($1, $2)",
            &users,
        ).await?;
        println!("   Batch inserted {} user(s)\n", inserted);

        // Step 9: Aggregation query
        // Demonstrates COUNT aggregation function
        // Returns single row with count value
        println!("9. Final user count...");
        let db = pool.get().await?;
        let rows = db.query("SELECT COUNT(*) as count FROM users", &[]).await?;
        let count: i64 = rows.first().unwrap().get("count");
        println!("   Total users in database: {}\n", count);

        // Step 10: Cleanup operation
        // Demonstrates DELETE operation with pattern matching
        // LIKE operator with % wildcard for pattern matching
        // Cleans up test data to maintain database hygiene
        println!("10. Cleanup (Delete test data)...");
        let db = pool.get().await?;
        db.execute("DELETE FROM users WHERE email LIKE $1", 
            &[&"%.example.com"]).await?;
        println!("   Cleaned up test data\n");

        println!("=== Database Example Completed ===");
        Ok::<(), RiError>(())
    })?
}
