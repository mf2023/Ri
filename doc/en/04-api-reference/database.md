<div align="center">

# Database API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The database module provides a unified database access layer, supporting multiple database types, connection pool management, transaction processing, and query builders.

## Module Overview

</div>

The database module includes the following sub-modules:

- **core**: Database core interfaces and type definitions
- **pools**: Connection pool management
- **query**: Query builders
- **migration**: Database migrations
- **transaction**: Transaction management

<div align="center">

## Core Components

</div>

### DMSCDatabase

Database manager main interface, providing unified database access.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `execute(query)` | Execute SQL query | `query: &str` | `DMSCResult<DMSCQueryResult>` |
| `query(query)` | Execute query and return results | `query: &str` | `DMSCResult<Vec<DMSCRow>>` |
| `query_one(query)` | Execute query and return single row result | `query: &str` | `DMSCResult<DMSCRow>` |
| `begin_transaction()` | Begin transaction | None | `DMSCResult<DMSCTransaction>` |
| `migrate()` | Execute database migration | None | `DMSCResult<()>` |
| `get_connection()` | Get database connection | None | `DMSCResult<DMSCConnection>` |
| `get_pool_stats()` | Get connection pool statistics | None | `DMSCResult<PoolStats>` |
| `ping()` | Test database connection | None | `DMSCResult<()>` |

#### Usage Example

```rust
use dms::prelude::*;

// Execute query
let results = ctx.database().query("SELECT id, name, email FROM users WHERE active = true")?;

for row in results {
    let id: i32 = row.get("id")?;
    let name: String = row.get("name")?;
    let email: String = row.get("email")?;
    
    println!("User {}: {} <{}>", id, name, email);
}

// Execute update
ctx.database().execute("UPDATE users SET last_login = NOW() WHERE id = 123")?;

// Get single row result
let user = ctx.database().query_one("SELECT * FROM users WHERE id = 123")?;
let name: String = user.get("name")?;
```

### DMSCDatabaseConfig

Database configuration struct.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `database_type` | `DMSCDatabaseType` | Database type | `Postgres` |
| `host` | `String` | Database host | `"localhost"` |
| `port` | `u16` | Database port | `5432` |
| `database` | `String` | Database name | `""` |
| `username` | `String` | Username | `""` |
| `password` | `String` | Password | `""` |
| `max_connections` | `u32` | Maximum connections | `10` |
| `min_connections` | `u32` | Minimum connections | `1` |
| `connection_timeout` | `Duration` | Connection timeout | `30s` |
| `idle_timeout` | `Duration` | Idle timeout | `600s` |
| `max_lifetime` | `Duration` | Connection maximum lifetime | `1800s` |

#### Configuration Example

```rust
use dms::prelude::*;

let db_config = DMSCDatabaseConfig {
    database_type: DMSCDatabaseType::Postgres,
    host: "localhost".to_string(),
    port: 5432,
    database: "myapp".to_string(),
    username: "postgres".to_string(),
    password: "password".to_string(),
    max_connections: 20,
    min_connections: 5,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(1800),
};
```

### DMSCDatabaseType

Database type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Postgres` | PostgreSQL |
| `MySQL` | MySQL |
| `SQLite` | SQLite |
| `MongoDB` | MongoDB |
| `Redis` | Redis |

## Query Builders

### DMSCQueryBuilder

Query builder for building type-safe SQL queries.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `select(columns)` | Select columns | `columns: &[&str]` | `Self` |
| `from(table)` | Specify table | `table: &str` | `Self` |
| `where(condition)` | Add condition | `condition: &str` | `Self` |
| `where_eq(column, value)` | Add equality condition | `column: &str`, `value: impl ToSql` | `Self` |
| `where_in(column, values)` | Add IN condition | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `order_by(column, direction)` | Add ordering | `column: &str`, `direction: OrderDirection` | `Self` |
| `limit(count)` | Set limit | `count: i64` | `Self` |
| `offset(count)` | Set offset | `count: i64` | `Self` |
| `join(table, on)` | Add join | `table: &str`, `on: &str` | `Self` |
| `build()` | Build query | None | `DMSCResult<String>` |

#### Usage Example

```rust
use dms::prelude::*;

// Build SELECT query
let query = DMSCQueryBuilder::new()
    .select(&["id", "name", "email"])
    .from("users")
    .where_eq("active", true)
    .where_eq("role", "admin")
    .order_by("created_at", OrderDirection::Desc)
    .limit(10)
    .build()?;

let results = ctx.database().query(&query)?;

// Build complex query
let complex_query = DMSCQueryBuilder::new()
    .select(&["u.id", "u.name", "COUNT(o.id) as order_count"])
    .from("users u")
    .join("orders o", "u.id = o.user_id")
    .where_eq("u.active", true)
    .where_in("u.role", &["admin", "user"])
    .group_by(&["u.id", "u.name"])
    .having("COUNT(o.id) > 5")
    .order_by("order_count", OrderDirection::Desc)
    .build()?;
```

### Insert Builder

```rust
use dms::prelude::*;

// Build INSERT query
let insert_query = DMSCInsertBuilder::new()
    .into("users")
    .columns(&["name", "email", "created_at"])
    .values(&["John Doe", "john@example.com", "NOW()"])
    .build()?;

ctx.database().execute(&insert_query)?;

// Batch insert
let batch_insert = DMSCInsertBuilder::new()
    .into("users")
    .columns(&["name", "email"])
    .values(&["Alice", "alice@example.com"])
    .values(&["Bob", "bob@example.com"])
    .values(&["Charlie", "charlie@example.com"])
    .build()?;

ctx.database().execute(&batch_insert)?;
```

### Update Builder

```rust
use dms::prelude::*;

// Build UPDATE query
let update_query = DMSCUpdateBuilder::new()
    .table("users")
    .set("last_login", "NOW()")
    .set("login_count", "login_count + 1")
    .where_eq("id", 123)
    .build()?;

ctx.database().execute(&update_query)?;
```

### Delete Builder

```rust
use dms::prelude::*;

// Build DELETE query
let delete_query = DMSCDeleteBuilder::new()
    .from("users")
    .where_eq("active", false)
    .where_lt("last_login", "NOW() - INTERVAL '1 year'")
    .build()?;

ctx.database().execute(&delete_query)?;
```
<div align="center">

## Transaction Management

</div>

### DMSCTransaction

Transaction interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `execute(query)` | Execute query in transaction | `query: &str` | `DMSCResult<DMSCQueryResult>` |
| `query(query)` | Execute query in transaction and return results | `query: &str` | `DMSCResult<Vec<DMSCRow>>` |
| `commit()` | Commit transaction | None | `DMSCResult<()>` |
| `rollback()` | Rollback transaction | None | `DMSCResult<()>` |
| `savepoint(name)` | Create savepoint | `name: &str` | `DMSCResult<()>` |
| `rollback_to_savepoint(name)` | Rollback to savepoint | `name: &str` | `DMSCResult<()>` |

#### Usage Example

```rust
use dms::prelude::*;

// Begin transaction
let mut transaction = ctx.database().begin_transaction()?;

try {
    // Execute operations in transaction
    transaction.execute("UPDATE accounts SET balance = balance - 100 WHERE id = 1")?;
    transaction.execute("UPDATE accounts SET balance = balance + 100 WHERE id = 2")?;
    
    // Create savepoint
    transaction.savepoint("before_fee")?;
    
    // Execute operation that might fail
    match transaction.execute("UPDATE accounts SET balance = balance - 5 WHERE id = 2") {
        Ok(_) => {
            // Operation succeeded, commit transaction
            transaction.commit()?;
        }
        Err(_) => {
            // Operation failed, rollback to savepoint
            transaction.rollback_to_savepoint("before_fee")?;
            transaction.commit()?;
        }
    }
} catch (e) {
    // Error occurred, rollback entire transaction
    transaction.rollback()?;
    return Err(e);
}
```

### Transaction Isolation Levels

```rust
use dms::prelude::*;

// Set transaction isolation level
let transaction = ctx.database()
    .begin_transaction_with_isolation(TransactionIsolation::Serializable)?;

// Different isolation levels
let read_uncommitted = TransactionIsolation::ReadUncommitted;
let read_committed = TransactionIsolation::ReadCommitted;
let repeatable_read = TransactionIsolation::RepeatableRead;
let serializable = TransactionIsolation::Serializable;
```

<div align="center">

## Connection Pool Management

</div>

### Connection Pool Configuration

```rust
use dms::prelude::*;

let pool_config = DMSCPoolConfig {
    max_connections: 50,
    min_connections: 10,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(1800),
    test_on_check_out: true,
    test_on_check_in: false,
    test_while_idle: true,
    validation_query: "SELECT 1".to_string(),
};
```

### Connection Pool Monitoring

```rust
use dms::prelude::*;

// Get connection pool statistics
let stats = ctx.database().get_pool_stats()?;

println!("Active connections: {}", stats.active_connections);
println!("Idle connections: {}", stats.idle_connections);
println!("Waiting connections: {}", stats.waiting_connections);
println!("Total connections created: {}", stats.total_created);
println!("Total connections destroyed: {}", stats.total_destroyed);
```

### Connection Pool Health Check

```rust
use dms::prelude::*;

// Check connection pool health status
match ctx.database().ping() {
    Ok(_) => {
        println!("Database connection is healthy");
    }
    Err(e) => {
        println!("Database connection failed: {}", e);
        
        // Try to recreate connection pool
        ctx.database().recreate_pool()?;
    }
}
```
<div align="center">

## Database Migrations

</div>

### DMSCMigration

Migration interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `add_migration(migration)` | Add migration | `migration: impl Migration` | `()` |
| `migrate()` | Execute migration | None | `DMSCResult<()>` |
| `rollback()` | Rollback migration | None | `DMSCResult<()>` |
| `get_status()` | Get migration status | None | `DMSCResult<Vec<MigrationStatus>>` |

### Creating Migrations

```rust
use dms::prelude::*;

struct CreateUsersTable;

impl Migration for CreateUsersTable {
    fn version(&self) -> &str {
        "20240115000001"
    }
    
    fn name(&self) -> &str {
        "create_users_table"
    }
    
    fn up(&self, db: &DMSCDatabase) -> DMSCResult<()> {
        db.execute(r#"
            CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT NOW(),
                updated_at TIMESTAMP DEFAULT NOW()
            )
        "#)?;
        
        Ok(())
    }
    
    fn down(&self, db: &DMSCDatabase) -> DMSCResult<()> {
        db.execute("DROP TABLE users")?;
        Ok(())
    }
}

// Add migration
ctx.database().migration().add_migration(CreateUsersTable)?;

// Execute migration
ctx.database().migrate()?;
```

### Migration Management

```rust
use dms::prelude::*;

// Get migration status
let migration_status = ctx.database().migration().get_status()?;

for status in migration_status {
    println!("Migration: {} ({})", status.name, status.version);
    println!("Status: {:?}", status.status);
    println!("Applied at: {:?}", status.applied_at);
}

// Rollback last migration
ctx.database().migration().rollback()?;

// Rollback to specific version
ctx.database().migration().rollback_to("20240115000001")?;
```

<div align="center">

## Advanced Features

</div>

### Batch Operations

```rust
use dms::prelude::*;

// Batch insert
let users = vec![
    ("Alice", "alice@example.com"),
    ("Bob", "bob@example.com"),
    ("Charlie", "charlie@example.com"),
];

ctx.database().batch_insert("users", &["name", "email"], users)?;

// Batch update
let updates = vec![
    (1, "Alice Smith"),
    (2, "Bob Johnson"),
    (3, "Charlie Brown"),
];

ctx.database().batch_update("users", "id", &["name"], updates)?;
```

### Prepared Statements

```rust
use dms::prelude::*;

// Prepare statement
let stmt = ctx.database().prepare("SELECT * FROM users WHERE id = $1 AND active = $2")?;

// Execute prepared statement
let results = stmt.query(&[123, true])?;

// Execute multiple times
for user_id in [123, 456, 789] {
    let results = stmt.query(&[user_id, true])?;
    // Process results
}
```

### Async Operations

```rust
use dms::prelude::*;

// Async query
let results = ctx.database().query_async("SELECT * FROM users WHERE active = true").await?;

// Async transaction
let mut transaction = ctx.database().begin_transaction_async().await?;

try {
    transaction.execute_async("UPDATE accounts SET balance = balance - 100 WHERE id = 1").await?;
    transaction.execute_async("UPDATE accounts SET balance = balance + 100 WHERE id = 2").await?;
    transaction.commit_async().await?;
} catch (e) {
    transaction.rollback_async().await?;
    return Err(e);
}
```

<div align="center">

## Error Handling

</div>  

### Database Error Codes

| Error Code | Description |
|:--------|:-------------|
| `DATABASE_CONNECTION_ERROR` | Database connection error |
| `DATABASE_QUERY_ERROR` | Database query error |
| `DATABASE_TRANSACTION_ERROR` | Database transaction error |
| `DATABASE_MIGRATION_ERROR` | Database migration error |
| `DATABASE_POOL_ERROR` | Connection pool error |

### Error Handling Example

```rust
use dms::prelude::*;

match ctx.database().query("SELECT * FROM users WHERE id = 123") {
    Ok(results) => {
        // Query succeeded
        for row in results {
            println!("User: {:?}", row);
        }
    }
    Err(DMSCError { code, .. }) if code == "DATABASE_CONNECTION_ERROR" => {
        // Connection error, try to reconnect
        ctx.log().error("Database connection lost, attempting to reconnect");
        ctx.database().recreate_pool()?;
        
        // Retry query
        let results = ctx.database().query("SELECT * FROM users WHERE id = 123")?;
    }
    Err(e) => {
        // Other errors
        return Err(e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Use connection pools**: Avoid frequent creation and destruction of connections
2. **Use prepared statements**: Improve performance and prevent SQL injection
3. **Use transactions appropriately**: Keep transactions short, avoid long locks
4. **Handle errors correctly**: Distinguish between recoverable and unrecoverable errors
5. **Monitor connection pools**: Monitor connection pool usage and performance
6. **Use migrations**: Use database migrations to manage schema changes
7. **Index optimization**: Add appropriate indexes for common queries
8. **Batch operations**: Use batch operations to reduce database round trips

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, providing JWT, OAuth2, and RBAC authentication and authorization functionality
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [config](./config.md): Configuration module, managing authentication configuration and key settings
- [cache](./cache.md): Cache module, providing multi-backend cache abstraction, caching user sessions and permission data
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [observability](./observability.md): Observability module, monitoring authentication performance and security events
- [security](./security.md): Security module, providing encryption, hashing, and validation functionality
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
