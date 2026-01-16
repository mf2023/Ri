<div align="center">

# Database Usage Examples

**Version: 0.1.4**

**Last modified date: 2026-01-15**

This example demonstrates how to use DMSC's database module for database connections, query building, transaction management, connection pooling, and migration functionality.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- PostgreSQL, MySQL, SQLite database connections
- Query builders and complex queries
- Transaction management and connection pools
- Database migration and schema management
- Data Access Object (DAO) pattern
- Error handling and connection monitoring

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of SQL and basic database concepts
- (Optional) PostgreSQL, MySQL, or SQLite database server

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-database-example
cd dms-database-example
```

### 2. Add Dependencies

Add the following dependencies to the `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

```yaml
service:
  name: "dms-database-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

database:
  default: "postgresql"
  connections:
    postgresql:
      database_type: "postgresql"
      host: "localhost"
      port: 5432
      database: "dms_example"
      username: "postgres"
      password: "password"
      pool_size: 20
      connection_timeout: 30
      idle_timeout: 600
      max_lifetime: 3600
      ssl_mode: "require"
    mysql:
      database_type: "mysql"
      host: "localhost"
      port: 3306
      database: "dms_example"
      username: "root"
      password: "password"
      pool_size: 15
      charset: "utf8mb4"
      collation: "utf8mb4_unicode_ci"
    sqlite:
      database_type: "sqlite"
      database: "./data/dms_example.db"
      pool_size: 5
      foreign_keys: true
      journal_mode: "wal"
```

### 4. Write Main Code

Replace the `src/main.rs` file with the following content:

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_database(DMSCDatabaseConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Database Example started")?;
        
        // Basic database operations example
        basic_database_operations(&ctx).await?;
        
        // Query builder examples
        query_builder_examples(&ctx).await?;
        
        // Transaction management examples
        transaction_examples(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Database Example completed")?;
        
        Ok(())
    }).await
}

async fn basic_database_operations(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("database", "Starting basic database operations")?;
    
    // Test database connection
    match ctx.database().ping().await {
        Ok(_) => ctx.logger().info("database", "Database connection successful")?,
        Err(e) => {
            ctx.logger().error("database", &format!("Database connection failed: {}", e))?;
            return Err(e);
        }
    }
    
    // Simple query
    let users = ctx.database()
        .query("SELECT id, name, email FROM users WHERE active = $1", vec![true.into()])
        .await?;
    
    for user in users {
        ctx.logger().info("database", &format!("User: {} - {} - {}", 
            user.get::<i32>("id").unwrap_or(0),
            user.get::<String>("name").unwrap_or_default(),
            user.get::<String>("email").unwrap_or_default()
        ))?;
    }
    
    Ok(())
}

async fn query_builder_examples(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("database", "Starting query builder examples")?;
    
    // Build complex query
    let query = DMSCQueryBuilder::new()
        .select(vec!["id", "name", "email", "created_at"])
        .from("users")
        .where_clause("active = $1", vec![true.into()])
        .and_where("age >= $1", vec![18.into()])
        .order_by("created_at", false)
        .limit(10);
    
    let users = ctx.database().execute_query(query).await?;
    ctx.logger().info("database", &format!("Found {} active users", users.len()))?;
    
    Ok(())
}

async fn transaction_examples(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("database", "Starting transaction examples")?;
    
    // Begin transaction
    let mut tx = ctx.database().begin_transaction().await?;
    
    // Execute multiple operations in transaction
    match tx.execute(
        "INSERT INTO users (name, email, age, active) VALUES ($1, $2, $3, $4) RETURNING id",
        vec![
            "John Doe".into(),
            "john@example.com".into(),
            30.into(),
            true.into(),
        ]
    ).await {
        Ok(user_id) => {
            ctx.logger().info("database", &format!("Created user with ID: {}", user_id))?;
            
            // Commit transaction
            tx.commit().await?;
            ctx.logger().info("database", "Transaction committed successfully")?;
        },
        Err(e) => {
            // Rollback transaction
            tx.rollback().await?;
            ctx.logger().error("database", &format!("Transaction rolled back: {}", e))?;
            return Err(e);
        }
    }
    
    Ok(())
}
```

<div align="center">

## Code Analysis

</div>

The database module provides usage examples for database connections, query building, transaction management, connection pooling, and migration functionality.

## Basic Database Operations

### Connection Management

```rust
use dmsc::prelude::*;
use serde_json::json;

// PostgreSQL connection configuration
let pg_config = DMSCDatabaseConfig {
    database_type: DMSCDatabaseType::PostgreSQL,
    host: "localhost".to_string(),
    port: 5432,
    database: "myapp".to_string(),
    username: "postgres".to_string(),
    password: "password".to_string(),
    pool_size: 20,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(3600),
    ssl_mode: DMSCSslMode::Require,
    ssl_cert: Some("/path/to/client-cert.pem".to_string()),
    ssl_key: Some("/path/to/client-key.pem".to_string()),
    ssl_root_cert: Some("/path/to/root-cert.pem".to_string()),
};

// MySQL connection configuration
let mysql_config = DMSCDatabaseConfig {
    database_type: DMSCDatabaseType::MySQL,
    host: "localhost".to_string(),
    port: 3306,
    database: "myapp".to_string(),
    username: "root".to_string(),
    password: "password".to_string(),
    pool_size: 15,
    connection_timeout: Duration::from_secs(30),
    idle_timeout: Duration::from_secs(600),
    max_lifetime: Duration::from_secs(3600),
    charset: "utf8mb4".to_string(),
    collation: "utf8mb4_unicode_ci".to_string(),
};

// SQLite connection configuration
let sqlite_config = DMSCDatabaseConfig {
    database_type: DMSCDatabaseType::SQLite,
    database: "./data/myapp.db".to_string(),
    pool_size: 5,
    connection_timeout: Duration::from_secs(10),
    busy_timeout: Duration::from_secs(5),
    foreign_keys: true,
    journal_mode: DMSCJournalMode::WAL,
};

// Initialize database connection
ctx.database().init(pg_config).await?;

// Test connection
match ctx.database().ping().await {
    Ok(_) => ctx.log().info("Database connection successful"),
    Err(e) => {
        ctx.log().error(format!("Database connection failed: {}", e));
        return Err(e);
    }
}
```

### Basic Queries

```rust
use dmsc::prelude::*;
use serde_json::json;

// Simple query
let users = ctx.database()
    .query("SELECT id, name, email FROM users WHERE active = $1", vec![true.into()])
    .await?;

for user in users {
    ctx.log().info(format!("User: {} - {} - {}", 
        user.get::<i32>("id").unwrap_or(0),
        user.get::<String>("name").unwrap_or_default(),
        user.get::<String>("email").unwrap_or_default()
    ));
}

// Parameterized query (prevents SQL injection)
let user_id = 123;
let user = ctx.database()
    .query_one("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
    .await?;

if let Some(user_data) = user {
    ctx.log().info(format!("Found user: {:?}", user_data));
}

// Insert data
let new_user = json!({
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30,
    "active": true,
    "created_at": "2024-01-15T10:30:00Z",
});

let inserted_id = ctx.database()
    .execute(
        "INSERT INTO users (name, email, age, active, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING id",
        vec![
            new_user["name"].as_str().unwrap().into(),
            new_user["email"].as_str().unwrap().into(),
            new_user["age"].as_i64().unwrap().into(),
            new_user["active"].as_bool().unwrap().into(),
            new_user["created_at"].as_str().unwrap().into(),
        ]
    )
    .await?;

ctx.log().info(format!("Inserted user with ID: {}", inserted_id));

// Update data
let updated_rows = ctx.database()
    .execute(
        "UPDATE users SET last_login = $1 WHERE email = $2",
        vec![
            "2024-01-15T11:00:00Z".into(),
            "john@example.com".into(),
        ]
    )
    .await?;

ctx.log().info(format!("Updated {} rows", updated_rows));

// Delete data
let deleted_rows = ctx.database()
    .execute("DELETE FROM users WHERE id = $1", vec![999.into()])
    .await?;

ctx.log().info(format!("Deleted {} rows", deleted_rows));
```

## Query Builder

### Building Complex Queries

```rust
use dmsc::prelude::*;

// Build SELECT query
let query = DMSCQueryBuilder::new()
    .select(vec!["id", "name", "email", "created_at"])
    .from("users")
    .where_clause("active = $1", vec![true.into()])
    .and_where("age >= $1", vec![18.into()])
    .and_where("created_at >= $1", vec!["2024-01-01".into()])
    .order_by("created_at", false) // DESC
    .limit(10)
    .offset(20);

let users = ctx.database().execute_query(query).await?;

// Build JOIN query
let order_query = DMSCQueryBuilder::new()
    .select(vec![
        "o.id as order_id",
        "o.total_amount",
        "o.status",
        "u.name as user_name",
        "u.email as user_email",
        "COUNT(oi.id) as item_count",
    ])
    .from("orders o")
    .join("users u", "o.user_id = u.id")
    .left_join("order_items oi", "o.id = oi.order_id")
    .where_clause("o.created_at >= $1", vec!["2024-01-01".into()])
    .group_by(vec!["o.id", "o.total_amount", "o.status", "u.name", "u.email"])
    .having("COUNT(oi.id) > $1", vec![5.into()])
    .order_by("o.total_amount", false)
    .limit(50);

let orders = ctx.database().execute_query(order_query).await?;

// Build aggregation query
let stats_query = DMSCQueryBuilder::new()
    .select(vec![
        "COUNT(*) as total_users",
        "AVG(age) as average_age",
        "MAX(created_at) as latest_user",
        "COUNT(CASE WHEN active = true THEN 1 END) as active_users",
    ])
    .from("users");

let stats = ctx.database().query_one(stats_query).await?;
if let Some(stats_data) = stats {
    ctx.log().info(format!("User statistics: {:?}", stats_data));
}

// Build subquery
let subquery = DMSCQueryBuilder::new()
    .select(vec!["user_id"])
    .from("orders")
    .where_clause("total_amount > $1", vec![1000.into()])
    .group_by(vec!["user_id"])
    .having("COUNT(*) >= $1", vec![3.into()]);

let main_query = DMSCQueryBuilder::new()
    .select(vec!["id", "name", "email"])
    .from("users")
    .where_in("id", subquery);

let vip_users = ctx.database().execute_query(main_query).await?;
```

### Type-Safe Queries

```rust
use dmsc::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
    age: i32,
    active: bool,
    created_at: String,
}

// Type-safe query
let users: Vec<User> = ctx.database()
    .query_as::<User>("SELECT id, name, email, age, active, created_at FROM users WHERE active = $1", vec![true.into()])
    .await?;

for user in users {
    ctx.log().info(format!("User: {:?}", user));
}

// Single record query
let user: Option<User> = ctx.database()
    .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![123.into()])
    .await?;

if let Some(u) = user {
    ctx.log().info(format!("Found user: {:?}", u));
}
```

## Transaction Management

### Basic Transactions

```rust
use dmsc::prelude::*;
use serde_json::json;

// Manual transaction management
let tx = ctx.database().begin_transaction().await?;

try {
    // Execute multiple operations in transaction
    let user_id = ctx.database()
        .execute_in_transaction(
            &tx,
            "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id",
            vec!["Alice".into(), "alice@example.com".into()]
        )
        .await?;
    
    ctx.database()
        .execute_in_transaction(
            &tx,
            "INSERT INTO user_profiles (user_id, bio) VALUES ($1, $2)",
            vec![user_id.into(), "Software developer".into()]
        )
        .await?;
    
    ctx.database()
        .execute_in_transaction(
            &tx,
            "INSERT INTO user_settings (user_id, theme) VALUES ($1, $2)",
            vec![user_id.into(), "dark".into()]
        )
        .await?;
    
    // Commit transaction
    ctx.database().commit_transaction(tx).await?;
    ctx.log().info(format!("User created with ID: {}", user_id));
    
} catch (e) {
    // Rollback transaction
    ctx.database().rollback_transaction(tx).await?;
    ctx.log().error(format!("Transaction failed, rolled back: {}", e));
    return Err(e);
}
```

### Transaction Convenience Methods

```rust
use dmsc::prelude::*;

// Use transaction convenience method
let result = ctx.database().with_transaction(|| async {
    // Transfer operation
    let from_account = 1001;
    let to_account = 1002;
    let amount = 500.00;
    
    // Deduct from source account
    let from_balance = ctx.database()
        .query_one_in_transaction(
            "SELECT balance FROM accounts WHERE id = $1 FOR UPDATE",
            vec![from_account.into()]
        )
        .await?;
    
    if let Some(balance_data) = from_balance {
        let current_balance: f64 = balance_data.get("balance").unwrap_or(0.0);
        if current_balance < amount {
            return Err(DMSCError::business("Insufficient funds".to_string()));
        }
        
        ctx.database()
            .execute_in_transaction(
                "UPDATE accounts SET balance = balance - $1 WHERE id = $2",
                vec![amount.into(), from_account.into()]
            )
            .await?;
    } else {
        return Err(DMSCError::not_found("Source account not found".to_string()));
    }
    
    // Add to target account
    ctx.database()
        .execute_in_transaction(
            "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
            vec![amount.into(), to_account.into()]
        )
        .await?;
    
    // Record transaction
    ctx.database()
        .execute_in_transaction(
            "INSERT INTO transactions (from_account, to_account, amount, type) VALUES ($1, $2, $3, $4)",
            vec![from_account.into(), to_account.into(), amount.into(), "transfer".into()]
        )
        .await?;
    
    Ok(json!({
        "from_account": from_account,
        "to_account": to_account,
        "amount": amount,
        "status": "completed"
    }))
}).await?;

ctx.log().info(format!("Transfer completed: {:?}", result));
```

### Isolation Levels

```rust
use dmsc::prelude::*;

// Set transaction isolation levels
let isolation_levels = vec![
    DMSCIsolationLevel::ReadUncommitted,
    DMSCIsolationLevel::ReadCommitted,
    DMSCIsolationLevel::RepeatableRead,
    DMSCIsolationLevel::Serializable,
];

for level in isolation_levels {
    let result = ctx.database().with_transaction_isolation(level, || async {
        // Execute sensitive operations under high isolation level
        let inventory_count = ctx.database()
            .query_one_in_transaction(
                "SELECT COUNT(*) as count FROM inventory WHERE product_id = $1 FOR UPDATE",
                vec![123.into()]
            )
            .await?;
        
        if let Some(count_data) = inventory_count {
            let count: i64 = count_data.get("count").unwrap_or(0);
            if count > 0 {
                ctx.database()
                    .execute_in_transaction(
                        "UPDATE inventory SET quantity = quantity - 1 WHERE product_id = $1",
                        vec![123.into()]
                    )
                    .await?;
                
                return Ok(json!({"status": "inventory_updated", "remaining": count - 1}));
            }
        }
        
        Ok(json!({"status": "no_inventory"}))
    }).await?;
    
    ctx.log().info(format!("Transaction with {:?} completed: {:?}", level, result));
}
```

## Connection Pool Management

### Connection Pool Monitoring

```rust
use dmsc::prelude::*;
use serde_json::json;

// Get connection pool statistics
let pool_stats = ctx.database().get_pool_stats().await?;
ctx.log().info(format!("Pool stats: {:?}", pool_stats));

// Monitor connection pool
let pool_metrics = json!({
    "active_connections": pool_stats.active_connections,
    "idle_connections": pool_stats.idle_connections,
    "max_connections": pool_stats.max_connections,
    "waiting_connections": pool_stats.waiting_connections,
    "average_wait_time_ms": pool_stats.average_wait_time.as_millis(),
    "average_usage_time_ms": pool_stats.average_usage_time.as_millis(),
    "timeouts": pool_stats.timeouts,
    "errors": pool_stats.errors,
});

ctx.log().info(format!("Pool metrics: {}", pool_metrics));

// Set connection pool event listener
ctx.database().on_connection_event(|event| {
    match event {
        DMSCConnectionEvent::ConnectionAcquired => {
            ctx.log().debug("Database connection acquired");
        }
        DMSCConnectionEvent::ConnectionReleased => {
            ctx.log().debug("Database connection released");
        }
        DMSCConnectionEvent::ConnectionTimeout => {
            ctx.log().warn("Database connection timeout");
        }
        DMSCConnectionEvent::ConnectionError(error) => {
            ctx.log().error(format!("Database connection error: {}", error));
        }
    }
}).await?;
```

### Connection Pool Tuning

```rust
use dmsc::prelude::*;

// Dynamically adjust connection pool size
ctx.database().set_pool_size(30).await?;
ctx.log().info("Increased pool size to 30");

// Get connection pool configuration
let pool_config = ctx.database().get_pool_config().await?;
ctx.log().info(format!("Current pool config: {:?}", pool_config));

// Set connection timeout
ctx.database().set_connection_timeout(Duration::from_secs(45)).await?;

// Clean up idle connections
let cleaned = ctx.database().cleanup_idle_connections().await?;
ctx.log().info(format!("Cleaned up {} idle connections", cleaned));
```

## Database Migrations

### Creating Migrations

```rust
use dmsc::prelude::*;

// Create new migration file
let migration = DMSCMigration {
    version: 2024011501,
    name: "create_users_table".to_string(),
    up: r#"
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            age INTEGER,
            active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        CREATE INDEX idx_users_email ON users(email);
        CREATE INDEX idx_users_created_at ON users(created_at);
    "#.to_string(),
    down: r#"
        DROP TABLE IF EXISTS users;
    "#.to_string(),
};

ctx.database().create_migration(migration).await?;

// Create complex migration
let complex_migration = DMSCMigration {
    version: 2024011502,
    name: "create_ecommerce_schema".to_string(),
    up: r#"
        -- Create users table
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create products table
        CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(200) NOT NULL,
            description TEXT,
            price DECIMAL(10,2) NOT NULL,
            stock_quantity INTEGER DEFAULT 0,
            category_id INTEGER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create orders table
        CREATE TABLE orders (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id),
            total_amount DECIMAL(10,2) NOT NULL,
            status VARCHAR(50) DEFAULT 'pending',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- Create order items table
        CREATE TABLE order_items (
            id SERIAL PRIMARY KEY,
            order_id INTEGER REFERENCES orders(id),
            product_id INTEGER REFERENCES products(id),
            quantity INTEGER NOT NULL,
            unit_price DECIMAL(10,2) NOT NULL,
            subtotal DECIMAL(10,2) NOT NULL
        );
        
        -- Create indexes
        CREATE INDEX idx_orders_user_id ON orders(user_id);
        CREATE INDEX idx_orders_status ON orders(status);
        CREATE INDEX idx_order_items_order_id ON order_items(order_id);
        CREATE INDEX idx_order_items_product_id ON order_items(product_id);
    "#.to_string(),
    down: r#"
        DROP TABLE IF EXISTS order_items;
        DROP TABLE IF EXISTS orders;
        DROP TABLE IF EXISTS products;
        DROP TABLE IF EXISTS users;
    "#.to_string(),
};

ctx.database().create_migration(complex_migration).await?;
```

### Running Migrations

```rust
use dmsc::prelude::*;

// Run all pending migrations
let migration_result = ctx.database().migrate().await?;
ctx.log().info(format!("Applied {} migrations", migration_result.applied_migrations.len()));

// Get migration status
let migration_status = ctx.database().get_migration_status().await?;
for status in migration_status {
    ctx.log().info(format!("Migration {}: {:?}", status.version, status.status));
}

// Rollback migration
let rollback_result = ctx.database().rollback(1).await?; // Rollback 1 migration
ctx.log().info(format!("Rolled back {} migrations", rollback_result.rolled_back_migrations.len()));

// Reset database
ctx.database().reset().await?;
ctx.log().warn("Database reset completed - all data lost");
```

### Data Migration

```rust
use dmsc::prelude::*;

// Data migration example
let data_migration = DMSCMigration {
    version: 2024011503,
    name: "migrate_user_data".to_string(),
    up: r#"
        -- Add new column
        ALTER TABLE users ADD COLUMN full_name VARCHAR(200);
        
        -- Migrate data
        UPDATE users SET full_name = name;
        
        -- Update data structure
        UPDATE users SET name = SPLIT_PART(full_name, ' ', 1);
        
        -- Add constraint
        ALTER TABLE users ALTER COLUMN full_name SET NOT NULL;
    "#.to_string(),
    down: r#"
        -- Rollback data migration
        UPDATE users SET name = full_name;
        ALTER TABLE users DROP COLUMN full_name;
    "#.to_string(),
};

ctx.database().create_migration(data_migration).await?;
```

## Advanced Features

### Batch Operations

```rust
use dmsc::prelude::*;
use serde_json::json;

// Batch insert
let users = vec![
    json!({"name": "Alice", "email": "alice@example.com", "age": 25}),
    json!({"name": "Bob", "email": "bob@example.com", "age": 30}),
    json!({"name": "Charlie", "email": "charlie@example.com", "age": 35}),
];

let inserted_ids = ctx.database().batch_insert(
    "users",
    vec!["name", "email", "age"],
    users.iter().map(|u| vec![
        u["name"].as_str().unwrap().into(),
        u["email"].as_str().unwrap().into(),
        u["age"].as_i64().unwrap().into(),
    ]).collect()
).await?;

ctx.log().info(format!("Inserted {} users with IDs: {:?}", inserted_ids.len(), inserted_ids));

// Batch update
let updates = vec![
    (1, json!({"name": "Alice Smith", "age": 26})),
    (2, json!({"name": "Bob Johnson", "age": 31})),
    (3, json!({"name": "Charlie Brown", "age": 36})),
];

let updated_count = ctx.database().batch_update(
    "users",
    vec!["name", "age"],
    "id",
    updates.iter().map(|(id, data)| (*id, vec![
        data["name"].as_str().unwrap().into(),
        data["age"].as_i64().unwrap().into(),
    ])).collect()
).await?;

ctx.log().info(format!("Updated {} users", updated_count));
```

### Database Functions

```rust
use dmsc::prelude::*;

// Create database function
let create_function = r#"
    CREATE OR REPLACE FUNCTION calculate_user_stats(user_id INTEGER)
    RETURNS TABLE(total_orders BIGINT, total_spent DECIMAL, avg_order_value DECIMAL) AS $$
    BEGIN
        RETURN QUERY
        SELECT 
            COUNT(o.id) as total_orders,
            COALESCE(SUM(o.total_amount), 0) as total_spent,
            COALESCE(AVG(o.total_amount), 0) as avg_order_value
        FROM orders o
        WHERE o.user_id = $1 AND o.status = 'completed';
    END;
    $$ LANGUAGE plpgsql;
"#;

ctx.database().execute(create_function, vec![]).await?;

// Call database function
let user_stats = ctx.database()
    .query("SELECT * FROM calculate_user_stats($1)", vec![123.into()])
    .await?;

for stat in user_stats {
    ctx.log().info(format!("User stats: total_orders={}, total_spent={}, avg_order_value={}",
        stat.get::<i64>("total_orders").unwrap_or(0),
        stat.get::<f64>("total_spent").unwrap_or(0.0),
        stat.get::<f64>("avg_order_value").unwrap_or(0.0)
    ));
}
```

### Full-Text Search

```rust
use dmsc::prelude::*;

// PostgreSQL full-text search
let search_query = r#"
    SELECT id, name, description, 
           ts_rank(to_tsvector('english', name || ' ' || description), 
                   plainto_tsquery('english', $1)) as rank
    FROM products
    WHERE to_tsvector('english', name || ' ' || description) @@ plainto_tsquery('english', $1)
    ORDER BY rank DESC
    LIMIT 20;
"#;

let search_term = "rust programming";
let search_results = ctx.database()
    .query(search_query, vec![search_term.into()])
    .await?;

for result in search_results {
    ctx.log().info(format!("Product: {} (rank: {})",
        result.get::<String>("name").unwrap_or_default(),
        result.get::<f64>("rank").unwrap_or(0.0)
    ));
}

// Create full-text search index
let create_index = r#"
    CREATE INDEX idx_products_search 
    ON products 
    USING gin(to_tsvector('english', name || ' ' || description));
"#;

ctx.database().execute(create_index, vec![]).await?;
```

## Error Handling

### Database Error Handling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Handle database errors
match ctx.database().query("SELECT * FROM non_existent_table", vec![]).await {
    Ok(results) => {
        ctx.log().info(format!("Query returned {} rows", results.len()));
    }
    Err(DMSCError::DatabaseConnectionError(e)) => {
        ctx.log().error(format!("Database connection failed: {}", e));
        // Try to reconnect or downgrade
        retry_database_connection().await?;
    }
    Err(DMSCError::DatabaseQueryError(e)) => {
        ctx.log().error(format!("Database query failed: {}", e));
        // Check if it's syntax error or table doesn't exist
        if e.contains("doesn't exist") {
            ctx.log().warn("Table does not exist, consider running migrations");
        }
    }
    Err(DMSCError::DatabaseTimeoutError(e)) => {
        ctx.log().warn(format!("Database query timed out: {}", e));
        // Optimize query or increase timeout
        optimize_query_performance().await?;
    }
    Err(DMSCError::DatabaseConstraintError(e)) => {
        ctx.log().error(format!("Database constraint violation: {}", e));
        // Handle unique constraints, foreign key constraints, etc.
        handle_constraint_violation(&e).await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected database error: {}", e));
        return Err(e);
    }
}

// Connection pool downgrade handling
async fn handle_database_unavailability() -> DMSCResult<()> {
    ctx.log().warn("Database is unavailable, switching to read-only cache mode");
    
    // Enable cache downgrade
    ctx.cache().set_readonly_mode(true)?;
    
    // Retry connection periodically
    let mut retry_count = 0;
    while retry_count < 10 {
        match ctx.database().ping().await {
            Ok(_) => {
                ctx.log().info("Database connection restored");
                ctx.cache().set_readonly_mode(false)?;
                break;
            }
            Err(e) => {
                ctx.log().warn(format!("Database still unavailable (retry {}): {}", retry_count + 1, e));
                sleep(Duration::from_secs(10)).await;
                retry_count += 1;
            }
        }
    }
    
    if retry_count >= 10 {
        return Err(DMSCError::service_unavailable("Database is still unavailable after 10 retries".to_string()));
    }
    
    Ok(())
}
```

<div align="center">

## Running Steps

</div>

### 1. Build Project

```bash
cargo build
```

### 2. Run Project

```bash
cargo run
```

<div align="center">

## Expected Results

</div>

After running the example, you should see output similar to the following:

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC Database Example started","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Starting basic database operations","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Database connection successful","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Starting query builder examples","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Found 5 active users","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Starting transaction examples","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Created user with ID: 101","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"database","message":"Transaction committed successfully","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC Database Example completed","trace_id":"abc123","span_id":"def456"}
```

<div align="center">

## Extended Features

</div>

### 1. Implement Complex Query Optimization

```rust
// Use prepared statements to optimize repeated queries
let mut stmt = ctx.database()
    .prepare("SELECT * FROM users WHERE id = $1 AND active = $2")
    .await?;

for user_id in vec![1, 2, 3, 4, 5] {
    let user = stmt.query_one(vec![user_id.into(), true.into()]).await?;
    if let Some(u) = user {
        ctx.logger().info("database", &format!("User {}: {:?}", user_id, u))?;
    }
}

// Use batch queries to reduce network round trips
let user_ids = vec![1, 2, 3, 4, 5];
let users = ctx.database()
    .query(
        "SELECT * FROM users WHERE id = ANY($1) AND active = $2",
        vec![user_ids.into(), true.into()]
    )
    .await?;

ctx.logger().info("database", &format!("Batch query returned {} users", users.len()))?;
```

### 2. Implement Database Connection Monitoring

```rust
// Set connection pool monitoring
ctx.database().set_connection_listener(|event| {
    match event {
        DMSCConnectionEvent::ConnectionCreated => {
            ctx.logger().debug("database", "New database connection created")?;
        }
        DMSCConnectionEvent::ConnectionClosed => {
            ctx.logger().debug("database", "Database connection closed")?;
        }
        DMSCConnectionEvent::ConnectionTimeout => {
            ctx.logger().warn("database", "Database connection timeout occurred")?;
        }
        DMSCConnectionEvent::PoolExhausted => {
            ctx.logger().warn("database", "Database connection pool exhausted")?;
        }
    }
    Ok(())
}).await?;

// Regularly check connection pool status
let pool_stats = ctx.database().get_pool_stats().await?;
ctx.logger().info("database", &format!(
    "Pool stats - Active: {}, Idle: {}, Total: {}",
    pool_stats.active_connections,
    pool_stats.idle_connections,
    pool_stats.total_connections
))?;
```

### 3. Implement Data Caching Strategy

```rust
// Implement query result caching
async fn get_user_with_cache(ctx: &DMSCServiceContext, user_id: i32) -> DMSCResult<Option<User>> {
    let cache_key = format!("user:{}", user_id);
    
    // Try to get from cache first
    if let Some(cached_user) = ctx.cache().get::<User>(&cache_key).await? {
        ctx.logger().debug("cache", &format!("User {} found in cache", user_id))?;
        return Ok(Some(cached_user));
    }
    
    // Cache miss, query from database
    let user = ctx.database()
        .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
        .await?;
    
    // Store result in cache (5 minute expiration)
    if let Some(ref u) = user {
        ctx.cache().set(&cache_key, u, 300).await?;
        ctx.logger().debug("cache", &format!("User {} cached for 5 minutes", user_id))?;
    }
    
    Ok(user)
}

// Implement cache invalidation strategy
async fn update_user_with_cache_invalidation(ctx: &DMSCServiceContext, user: &User) -> DMSCResult<()> {
    // Update database
    ctx.database()
        .execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            vec![user.name.clone().into(), user.email.clone().into(), user.id.into()]
        )
        .await?;
    
    // Invalidate related caches
    let cache_key = format!("user:{}", user.id);
    ctx.cache().delete(&cache_key).await?;
    ctx.cache().delete("users:list").await?;
    
    ctx.logger().info("cache", &format!("Cache invalidated for user {}", user.id))?;
    Ok(())
}
```

### 4. Implement Database Sharding

```rust
// Implement sharding strategy based on user ID
struct UserShardManager {
    shard_count: u32,
}

impl UserShardManager {
    fn get_shard_for_user(&self, user_id: u32) -> u32 {
        user_id % self.shard_count
    }
    
    fn get_shard_connection(&self, shard_id: u32) -> String {
        format!("shard_{}", shard_id)
    }
}

// Use sharding in application
let shard_manager = UserShardManager { shard_count: 4 };

// Route to corresponding shard based on user ID
let user_id = 12345;
let shard_id = shard_manager.get_shard_for_user(user_id);
let shard_connection = shard_manager.get_shard_connection(shard_id);

ctx.logger().info("shard", &format!("User {} routed to shard {}", user_id, shard_id))?;

// Execute query on specified shard
let user = ctx.database()
    .using_connection(&shard_connection)
    .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
    .await?;
```

<div align="center">

## Best Practices

</div>

1. **Use parameterized queries**: Prevent SQL injection attacks and improve query performance
2. **Use transactions reasonably**: Keep transactions short and avoid long-term resource locking
3. **Connection pool management**: Adjust connection pool size based on load and concurrency
4. **Index optimization**: Create composite indexes for frequently queried fields, avoid over-indexing
5. **Query optimization**: Avoid N+1 query problems, use JOIN or batch queries
6. **Error handling**: Handle database errors properly, implement degradation and retry strategies
7. **Data validation**: Validate data at the application layer to reduce database constraint errors
8. **Migration management**: Use versioned migrations to manage database schema changes
9. **Performance monitoring**: Monitor query performance, connection pool status, and slow queries
10. **Backup strategy**: Regularly backup databases, test recovery procedures and verify data integrity

<div align="center">

## Summary

</div>

This example demonstrates how to use the DMSC database module for database operations, including:

- Multi-database connection configuration (PostgreSQL, MySQL, SQLite)
- Basic queries and parameterized queries
- Query builders and complex queries
- Transaction management and isolation levels
- Connection pool monitoring and tuning
- Database migrations and version management
- Batch operations and performance optimization
- Error handling and degradation strategies

Through this example, you should have mastered the core functions and usage methods of the DMSC database module. You can build more complex database applications based on this foundation.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation for all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, understand how to use cache modules to improve application performance
- [database](./database.md): Database examples, learn database connection and query operations
- [http](./http.md): HTTP service examples, build Web applications and RESTful APIs
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing and security best practices
- [storage](./storage.md): Storage examples, file upload/download and storage management
- [validation](./validation.md): Validation examples, data validation and cleanup operations