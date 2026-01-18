#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Database Module Example

This example demonstrates how to use the database module in DMSC,
including connection pool management, CRUD operations, and transactions.

Features Demonstrated:
- Connection pool configuration
- CRUD operations (Create, Read, Update, Delete)
- Parameterized queries to prevent SQL injection
- Transactions with rollback support
- Batch operations for bulk inserts
"""

import dmsc
from dmsc.database import DMSCDatabasePool, DMSCDatabaseConfig
import asyncio


async def main():
    """
    Main async entry point for the database module example.
    
    This function demonstrates the complete database workflow including:
    - Connection pool creation and configuration
    - Schema creation (CREATE TABLE)
    - CRUD operations (Create, Read, Update, Delete)
    - Parameterized query execution with SQL injection prevention
    - Transaction management with commit/rollback
    - Batch operations for efficient bulk data insertion
    
    The example uses PostgreSQL database to demonstrate DMSC's database
    capabilities including connection pooling and async query execution.
    """
    print("=== DMSC Database Module Example ===\n")
    
    # Configuration Setup: Create database connection configuration
    # This configuration establishes connection parameters for the database pool
    # Using postgres() factory method for PostgreSQL-specific defaults
    # Parameters:
    # - host: Database server hostname or IP address
    # - port: Database server port (PostgreSQL default: 5432)
    # - database: Name of the database to connect to
    # - user: Database username for authentication
    # - password: Database password for authentication
    # - max_connections: Maximum number of connections in the pool
    # - min_idle_connections: Minimum number of idle connections to maintain
    # - connection_timeout_secs: Maximum time to wait for a connection
    config = DMSCDatabaseConfig.postgres(
        host="localhost",
        port=5432,
        database="dmsc_example",
        user="postgres",
        password="password",
        max_connections=10,
        min_idle_connections=2,
        connection_timeout_secs=30,
    )
    
    # Step 1: Create connection pool
    # Connection pools manage database connections efficiently:
    # - Reuses connections to reduce overhead
    # - Limits total connections to prevent overload
    # - Provides thread-safe access to connections
    # The pool automatically handles connection creation and health checks
    print("1. Creating connection pool...")
    pool = await DMSCDatabasePool.create(config)
    print("   Connection pool created successfully\n")
    
    # Step 2: Create database schema (table)
    # Execute DDL (Data Definition Language) statements
    # CREATE TABLE IF NOT EXISTS ensures idempotent schema creation
    print("2. Creating table...")
    db = await pool.get()
    await db.execute("""
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)
    print("   Table 'users' created or already exists\n")
    
    # Step 3: INSERT operation (Create)
    # Demonstrates parameterized query execution for data insertion
    # - Uses $1, $2 placeholders for parameter binding
    # - Prevents SQL injection by separating data from query
    # - RETURNING clause retrieves inserted record data
    print("3. Inserting new user (Create)...")
    db = await pool.get()
    rows = await db.execute(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email, created_at",
        ["John Doe", "john@example.com"],
    )
    if rows and len(rows) > 0:
        row = rows[0]
        print(f"   User inserted: id={row['id']}, name={row['name']}, email={row['email']}\n")
    
    # Step 4: SELECT operation (Read)
    # Demonstrates query execution to retrieve data
    # - Returns list of rows matching the query
    # - Each row is a dictionary-like object
    # - Query parameters prevent SQL injection
    print("4. Querying users (Read)...")
    db = await pool.get()
    rows = await db.query("SELECT id, name, email FROM users ORDER BY id")
    print(f"   Found {len(rows)} users:")
    for row in rows:
        print(f"   - id={row['id']}, name={row['name']}, email={row['email']}")
    print()
    
    # Step 5: UPDATE operation (Update)
    # Demonstrates data modification with parameterized queries
    # - Returns number of affected rows
    # - $1, $2 placeholders prevent SQL injection
    print("5. Updating user (Update)...")
    db = await pool.get()
    affected = await db.execute(
        "UPDATE users SET email = $1 WHERE name = $2",
        ["john.doe@example.com", "John Doe"],
    )
    print(f"   Updated {affected} row(s)\n")
    
    # Step 6: Transaction management with rollback
    # Demonstrates ACID transaction properties:
    # - BEGIN: Start transaction
    # - Execute multiple statements within transaction
    # - ROLLBACK: Discard all changes (atomicity)
    # Transactions ensure data consistency for complex operations
    print("6. Using transaction (Rollback demo)...")
    tx = await pool.begin_transaction()
    
    # Insert test user within transaction
    await tx.execute("INSERT INTO users (name, email) VALUES ($1, $2)",
        ["Test User", "test@example.com"])
    print("   Inserted test user in transaction")
    
    # Rollback discards the insert, maintaining data integrity
    await tx.rollback()
    print("   Transaction rolled back\n")
    
    # Step 7: Verify rollback worked
    # Query to check if test user was actually added
    # This proves transaction rollback was successful
    print("7. Verifying no test user was added...")
    db = await pool.get()
    rows = await db.query("SELECT COUNT(*) as count FROM users WHERE email = $1",
        ["test@example.com"])
    count = rows[0]['count'] if rows else 0
    print(f"   Users with test@example.com: {count}\n")
    
    # Step 8: Batch insert operation
    # Demonstrates efficient bulk data insertion
    # - execute_many() processes multiple rows in single call
    # - More efficient than individual inserts
    # - Reduces database round-trips
    print("8. Batch insert users...")
    db = await pool.get()
    users = [
        ["Alice", "alice@example.com"],
        ["Bob", "bob@example.com"],
        ["Charlie", "charlie@example.com"],
    ]
    inserted = await db.execute_many(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        users,
    )
    print(f"   Batch inserted {inserted} user(s)\n")
    
    # Step 9: Aggregation query
    # Demonstrates COUNT aggregation function
    print("9. Final user count...")
    db = await pool.get()
    rows = await db.query("SELECT COUNT(*) as count FROM users")
    count = rows[0]['count'] if rows else 0
    print(f"   Total users in database: {count}\n")
    
    # Step 10: Cleanup operation
    # Demonstrates DELETE operation with pattern matching
    # - LIKE operator with % wildcard for pattern matching
    # - Cleans up test data to maintain database hygiene
    print("10. Cleanup (Delete test data)...")
    db = await pool.get()
    await db.execute("DELETE FROM users WHERE email LIKE $1", 
        ["%.example.com"])
    print("   Cleaned up test data\n")
    
    print("=== Database Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
