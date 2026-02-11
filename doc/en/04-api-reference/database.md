<div align="center">

# Database API Reference

**Version: 0.1.7**

**Last modified date: 2026-01-18**

The database module provides a unified database access layer, supporting multiple database types, connection pool management, transaction processing, and ORM query builders.

## Module Overview

</div>

The database module includes the following sub-modules:

- **core**: Database core interfaces and type definitions
- **pools**: Connection pool management
- **orm**: Object-relational mapping and query builders
- **migration**: Database migrations
- **transaction**: Transaction management

<div align="center">

## Core Components

</div>

### DMSCDatabase

Database operation interface trait, providing unified database access.

**Note**: DMSCDatabase is a trait and cannot be instantiated directly. You need to obtain a concrete database connection through DMSCDatabasePool.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `execute(sql)` | Execute SQL query | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | Execute query and return results | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `query_one(sql)` | Execute query and return single row | `sql: &str` | `DMSCResult<Option<DMSCDBRow>>` |
| `ping()` | Test database connection | None | `DMSCResult<bool>` |
| `is_connected()` | Check connection status | None | `bool` |
| `close()` | Close connection | None | `DMSCResult<()>` |
| `transaction()` | Begin transaction | None | `DMSCResult<Box<dyn DMSCDatabaseTransaction>>` |
| `execute_with_params(sql, params)` | Execute SQL with parameters | `sql: &str`, `params: &[serde_json::Value]` | `DMSCResult<u64>` |
| `query_with_params(sql, params)` | Execute query with parameters | `sql: &str`, `params: &[serde_json::Value]` | `DMSCResult<DMSCDBResult>` |
| `batch_execute(sql, params)` | Batch execution | `sql: &str`, `params: &[Vec<serde_json::Value>]` | `DMSCResult<Vec<u64>>` |
| `batch_query(sql, params)` | Batch query | `sql: &str`, `params: &[Vec<serde_json::Value>]` | `DMSCResult<Vec<DMSCDBResult>>` |

#### Usage Example

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres()
        .host("localhost")
        .database("myapp")
        .build();

    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // Execute query
    let results = db.query("SELECT id, name, email FROM users WHERE active = true").await?;

    for row in results {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        
        println!("User {}: {} <{}>", id, name, email);
    }

    // Execute update
    db.execute("UPDATE users SET last_login = NOW() WHERE id = 123").await?;

    // Get single row result
    let user = db.query_one("SELECT * FROM users WHERE id = 123").await?;
    if let Some(row) = user {
        let name: String = row.get("name")?;
    }

    // Query with parameters
    let rows = db.query_with_params(
        "SELECT * FROM users WHERE name = $1 AND active = $2",
        &[serde_json::json!("alice"), serde_json::json!(true)]
    ).await?;

    Ok(())
}
```

### DMSCDatabaseConfig

Database configuration builder.

#### Static Methods

| Method | Description | Return Value |
|:--------|:-------------|:--------|
| `postgres()` | Create PostgreSQL default config | `DMSCDatabaseConfig` |
| `mysql()` | Create MySQL default config | `DMSCDatabaseConfig` |
| `sqlite()` | Create SQLite default config | `DMSCDatabaseConfig` |

#### Builder Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `host()` | Set host | `host: &str` | `Self` |
| `port()` | Set port | `port: u16` | `Self` |
| `database()` | Set database name | `database: &str` | `Self` |
| `user()` | Set username | `user: &str` | `Self` |
| `password()` | Set password | `password: &str` | `Self` |
| `max_connections()` | Maximum connections | `n: u32` | `Self` |
| `min_idle_connections()` | Minimum idle connections | `n: u32` | `Self` |
| `connection_timeout_secs()` | Connection timeout (seconds) | `secs: u64` | `Self` |
| `idle_timeout_secs()` | Idle timeout (seconds) | `secs: u64` | `Self` |
| `max_lifetime_secs()` | Maximum lifetime (seconds) | `secs: u64` | `Self` |
| `build()` | Build configuration | None | `DMSCDatabaseConfig` |

#### Configuration Example

```rust
use dmsc::database::DMSCDatabaseConfig;

let config = DMSCDatabaseConfig::postgres()
    .host("localhost")
    .port(5432)
    .database("myapp")
    .user("postgres")
    .password("password")
    .max_connections(20)
    .min_idle_connections(5)
    .connection_timeout_secs(30)
    .idle_timeout_secs(600)
    .max_lifetime_secs(1800)
    .build();
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

<div align="center">

## Connection Pool Management

</div>

### DMSCDatabasePool

Database connection pool manager.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create connection pool | `config: DMSCDatabaseConfig` | `DMSCResult<Self>` |
| `get()` | Get connection | None | `DMSCResult<PooledDatabase>` |
| `close()` | Close connection pool | None | `DMSCResult<()>` |
| `metrics()` | Get statistics | None | `DatabaseMetrics` |

#### Usage Example

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres()
        .database("mydb")
        .max_connections(10)
        .build();

    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // Use database connection...

    // Get pool statistics
    let metrics = pool.metrics();
    println!("Active connections: {}", metrics.active_connections);
    println!("Idle connections: {}", metrics.idle_connections);

    Ok(())
}
```

### PooledDatabase

Pooled database connection wrapper.

Implements DMSCDatabase trait, can be used directly for database operations.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `id()` | Get connection ID | None | `u32` |
| `execute(sql)` | Execute SQL | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | Execute query | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `query_one(sql)` | Query single row | `sql: &str` | `DMSCResult<Option<DMSCDBRow>>` |
| `ping()` | Test connection | None | `DMSCResult<bool>` |
| `is_connected()` | Check connection | None | `bool` |
| `pool_metrics()` | Get pool statistics | None | `DatabaseMetrics` |

### DatabaseMetrics

Connection pool statistics.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `active_connections` | `u64` | Active connections count |
| `idle_connections` | `u64` | Idle connections count |
| `total_connections` | `u64` | Total connections count |
| `queries_executed` | `u64` | Queries executed count |
| `query_duration_ms` | `f64` | Average query duration (ms) |
| `errors` | `u64` | Errors count |

<div align="center">

## ORM and Query Builders

</div>

### QueryBuilder

ORM query builder, providing type-safe query building.

**Note**: QueryBuilder is located in the `dmsc::database::orm` module.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new instance | None | `Self` |
| `table(name)` | Set table name | `name: &str` | `Self` |
| `select(columns)` | Select columns | `columns: &[&str]` | `Self` |
| `and_where(condition)` | Add AND condition | `condition: &str` | `Self` |
| `or_where(condition)` | Add OR condition | `condition: &str` | `Self` |
| `where_eq(column, value)` | Equality condition | `column: &str`, `value: serde_json::Value` | `Self` |
| `where_in(column, values)` | IN condition | `column: &str`, `values: &[serde_json::Value]` | `Self` |
| `order_by(column, order)` | Order by | `column: &str`, `order: SortOrder` | `Self` |
| `limit(n)` | Limit count | `n: u64` | `Self` |
| `offset(n)` | Offset | `n: u64` | `Self` |
| `build()` | Build SQL | None | `String` |

#### Usage Example

```rust
use dmsc::database::orm::{QueryBuilder, SortOrder};

let query = QueryBuilder::new()
    .table("users")
    .select(&["id", "name", "email"])
    .where_eq("active", serde_json::json!(true))
    .and_where("role = 'admin'")
    .order_by("created_at", SortOrder::Desc)
    .limit(10)
    .build();

let results = db.query(&query).await?;
```

### TableDefinition

Table definition structure for schema management.

```rust
use dmsc::database::orm::{TableDefinition, ColumnDefinition, ColumnType};

let table = TableDefinition::new("users");
table.add_column(ColumnDefinition::new("id", ColumnType::BigInt).primary_key(true));
table.add_column(ColumnDefinition::new("name", ColumnType::VarChar(255)));
table.add_column(ColumnDefinition::new("email", ColumnType::VarChar(255)).unique(true));
table.set_primary_key(vec!["id".to_string()]);

let sql = table.get_create_sql();
println!("{}", sql);
```

### ColumnDefinition

Column definition structure.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `name` | `String` | Column name |
| `column_type` | `ColumnType` | Column type |
| `is_nullable` | `bool` | Whether nullable |
| `is_primary_key` | `bool` | Whether primary key |
| `is_unique` | `bool` | Whether unique |
| `default_value` | `Option<String>` | Default value |

### Criteria

Query condition for building WHERE clauses.

```rust
use dmsc::database::orm::{Criteria, ComparisonOperator, LogicalOperator};

let criteria = Criteria::new("age", ComparisonOperator::GreaterThan, serde_json::json!(18));
let criteria2 = Criteria::new("status", ComparisonOperator::Equal, serde_json::json!("active"));
```

### JoinClause

Join clause for building JOIN queries.

```rust
use dmsc::database::orm::{JoinClause, JoinType};

let join = JoinClause::new(
    "orders",
    JoinType::Inner,
    "user_id",
    "id"
);
```

### LogicalOperator

Logical operator for combining criteria.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `And` | AND operator |
| `Or` | OR operator |

### ComparisonOperator

Comparison operator for WHERE clauses.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Equal` | = |
| `NotEqual` | != |
| `GreaterThan` | > |
| `LessThan` | < |
| `GreaterThanOrEqual` | >= |
| `LessThanOrEqual` | <= |
| `Like` | LIKE |
| `In` | IN |
| `NotIn` | NOT IN |
| `IsNull` | IS NULL |
| `IsNotNull` | IS NOT NULL |

### Pagination

Pagination information for query results.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `page` | `u64` | Current page number (1-based) |
| `page_size` | `u64` | Number of items per page |

### SortOrder

Sort order enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Asc` | Ascending order |
| `Desc` | Descending order |

### DMSCORMSimpleRepository

Simple ORM repository, providing basic CRUD operations。

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(table_name)` | Create instance | `table_name: &str` | `Self` |
| `find_all()` | Query all | None | `DMSCResult<Vec<serde_json::Value>>` |
| `find_by_id(id)` | Query by ID | `id: i64` | `DMSCResult<Option<serde_json::Value>>` |
| `find_by_where(where)` | Conditional query | `where: &str` | `DMSCResult<Vec<serde_json::Value>>` |
| `create(data)` | Create record | `data: &serde_json::Value` | `DMSCResult<serde_json::Value>` |
| `update(id, data)` | Update record | `id: i64`, `data: &serde_json::Value` | `DMSCResult<serde_json::Value>` |
| `delete(id)` | Delete record | `id: i64` | `DMSCResult<()>` |

<div align="center">

## Transaction Management

</div>

### DMSCDatabaseTransaction

Transaction interface trait.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `execute(sql)` | Execute SQL | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | Execute query | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `commit()` | Commit transaction | None | `DMSCResult<()>` |
| `rollback()` | Rollback transaction | None | `DMSCResult<()>` |
| `close()` | Close transaction | None | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres()
        .database("mydb")
        .build();

    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // Start transaction
    let mut tx = db.transaction().await?;

    // Execute operations within transaction
    tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", &[&1]).await?;
    tx.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $1", &[&2]).await?;

    // Commit transaction
    tx.commit().await?;

    Ok(())
}
```

<div align="center">

## Database Migrations

</div>

### DMSCDatabaseMigrator

Database migration manager.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create new instance | None | `Self` |
| `with_migrations_dir(dir)` | Set migrations directory | `dir: PathBuf` | `Self` |
| `add_migration(migration)` | Add migration | `migration: DMSCDatabaseMigration` | `()` |
| `get_migrations()` | Get all migrations | None | `&[DMSCDatabaseMigration]` |
| `load_migrations_from_dir(dir)` | Load from directory | `dir: &str` | `std::io::Result<()>` |

### DMSCDatabaseMigration

Migration definition struct.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `version` | `u32` | Version number |
| `name` | `String` | Migration name |
| `sql_up` | `String` | Upgrade SQL |
| `sql_down` | `Option<String>` | Rollback SQL |
| `timestamp` | `DateTime<Utc>` | Creation time |

#### Creating Migrations

```rust
use dmsc::database::{DMSCDatabaseMigrator, DMSCDatabaseMigration};

let migration = DMSCDatabaseMigration::new(
    20240115000001,
    "create_users_table",
    r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )
    "#,
    Some("DROP TABLE users")
);

let mut migrator = DMSCDatabaseMigrator::new();
migrator.add_migration(migration);
```

### DMSCMigrationHistory

Migration history record.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `version` | `u32` | Version number |
| `name` | `String` | Migration name |
| `applied_at` | `DateTime<Utc>` | Applied time |
| `checksum` | `String` | Checksum |

<div align="center">

## Row Data Operations

</div>

### DMSCDBRow

Database result row.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `get<T>(column)` | Get column value | `column: &str` | `T` |
| `get_opt<T>(column)` | Get optional value | `column: &str` | `DMSCResult<Option<T>>` |
| `len()` | Column count | None | `usize` |
| `is_empty()` | Is empty | None | `bool` |
| `columns()` | Column names | None | `Vec<String>` |

#### Usage Example

```rust
let rows = db.query("SELECT * FROM users WHERE id = $1", &[&1]).await?;

for row in rows {
    let id: i64 = row.get("id")?;
    let name: String = row.get("name")?;
    let email: Option<String> = row.get_opt("email")?;
    
    println!("User: {} - {} - {:?}", id, name, email);
}
```

### DMSCDBResult

Query result set.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `len()` | Row count | None | `usize` |
| `is_empty()` | Is empty | None | `bool` |
| `columns()` | Column names | None | `Vec<String>` |
| `iter()` | Iterator | None | `Iter<'_, DMSCDBRow>` |
| `into_iter()` | Into iterator | None | `IntoIter<DMSCDBRow>` |

#### Implemented Traits

- `IntoIterator` - Supports `for row in results` iteration

<div align="center">

## Batch Operations

</div>

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres().build();
    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // Batch insert
    let users = vec![
        vec![serde_json::json!("Alice"), serde_json::json!("alice@example.com")],
        vec![serde_json::json!("Bob"), serde_json::json!("bob@example.com")],
        vec![serde_json::json!("Charlie"), serde_json::json!("charlie@example.com")],
    ];

    let results = db.batch_execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &users
    ).await?;

    println!("Inserted {} rows", results.len());

    Ok(())
}
```

<div align="center">

## Error Handling

</div>

### Error Codes

| Error Code | Description |
|:--------|:-------------|
| `DATABASE_CONNECTION_ERROR` | Database connection error |
| `DATABASE_QUERY_ERROR` | Database query error |
| `DATABASE_TRANSACTION_ERROR` | Database transaction error |
| `DATABASE_MIGRATION_ERROR` | Database migration error |
| `DATABASE_POOL_ERROR` | Connection pool error |

### Error Handling Example

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres().build();
    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    match db.query("SELECT * FROM users WHERE id = $1", &[&123]).await {
        Ok(results) => {
            for row in results {
                println!("User: {:?}", row);
            }
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
```

<div align="center">

## Best Practices

</div>

1. **Use Connection Pool**: Avoid frequent connection creation and destruction
2. **Use Parameter Binding**: Use `query_with_params` to prevent SQL injection
3. **Use Transactions Appropriately**: Keep transactions short, avoid long locks
4. **Handle Errors Correctly**: Distinguish between recoverable and unrecoverable errors
5. **Monitor Connection Pool**: Use `metrics()` to monitor pool usage
6. **Use Migrations**: Use DMSCDatabaseMigrator to manage schema changes
7. **Async Operations**: All database operations are asynchronous, use `.await`
8. **Batch Operations**: Use `batch_execute` to reduce database round trips

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, handling user authentication and authorization
- [cache](./cache.md): Cache module, providing memory cache and distributed cache support
- [config](./config.md): Configuration module, managing application configuration
- [core](./core.md): Core module, providing error handling and service context
- [device](./device.md): Device module, using protocols for device communication
- [fs](./fs.md): File system module, providing file operation functions
- [gateway](./gateway.md): Gateway module, providing API gateway functions
- [grpc](./grpc.md): gRPC Module, with service registration and Python bindings
- [hooks](./hooks.md): Hooks module, providing lifecycle hook support
- [log](./log.md): Log module, recording protocol events
- [observability](./observability.md): Observability module, monitoring protocol performance
- [protocol](./protocol.md): Protocol module, providing communication protocol support
- [queue](./queue.md): Message queue module, providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module, using protocols for inter-service communication
- [validation](./validation.md): Validation module, providing data validation functions
- [ws](./ws.md): WebSocket Module, with Python bindings for real-time communication
