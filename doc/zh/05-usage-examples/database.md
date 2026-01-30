<div align="center">

# 数据库使用示例

**Version: 0.1.6**

**Last modified date: 2026-01-30**

本示例展示如何使用DMSC的database模块进行数据库连接、查询构建、事务管理、连接池和迁移功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- PostgreSQL、MySQL、SQLite数据库连接
- 查询构建器和复杂查询
- 事务管理和连接池
- 数据库迁移和模式管理
- 数据访问对象(DAO)模式
- 错误处理和连接监控

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解SQL和数据库基本概念
- （可选）PostgreSQL、MySQL或SQLite数据库服务器

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-database-example
cd dms-database-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

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

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_database(DMSCDatabaseConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Database Example started")?;
        
        // 基本数据库操作示例
        basic_database_operations(&ctx).await?;
        
        // 查询构建器示例
        query_builder_examples(&ctx).await?;
        
        // 事务管理示例
        transaction_examples(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Database Example completed")?;
        
        Ok(())
    }).await
}

async fn basic_database_operations(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("database", "Starting basic database operations")?;
    
    // 测试数据库连接
    match ctx.database().ping().await {
        Ok(_) => ctx.logger().info("database", "Database connection successful")?,
        Err(e) => {
            ctx.logger().error("database", &format!("Database connection failed: {}", e))?;
            return Err(e);
        }
    }
    
    // 简单查询
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
    
    // 构建复杂查询
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
    
    // 开始事务
    let mut tx = ctx.database().begin_transaction().await?;
    
    // 在事务中执行多个操作
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
            
            // 提交事务
            tx.commit().await?;
            ctx.logger().info("database", "Transaction committed successfully")?;
        },
        Err(e) => {
            // 回滚事务
            tx.rollback().await?;
            ctx.logger().error("database", &format!("Transaction rolled back: {}", e))?;
            return Err(e);
        }
    }
    
    Ok(())
}
```

<div align="center">

## 代码解析

</div>

database模块提供数据库连接、查询构建、事务管理、连接池和迁移功能的使用示例。

## 基本数据库操作

### 连接管理

```rust
use dmsc::prelude::*;
use serde_json::json;

// PostgreSQL连接配置
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

// MySQL连接配置
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

// SQLite连接配置
let sqlite_config = DMSCDatabaseConfig {
    database_type: DMSCDatabaseType::SQLite,
    database: "./data/myapp.db".to_string(),
    pool_size: 5,
    connection_timeout: Duration::from_secs(10),
    busy_timeout: Duration::from_secs(5),
    foreign_keys: true,
    journal_mode: DMSCJournalMode::WAL,
};

// 初始化数据库连接
ctx.database().init(pg_config).await?;

// 测试连接
match ctx.database().ping().await {
    Ok(_) => ctx.log().info("Database connection successful"),
    Err(e) => {
        ctx.log().error(format!("Database connection failed: {}", e));
        return Err(e);
    }
}
```

### 基本查询

```rust
use dmsc::prelude::*;
use serde_json::json;

// 简单查询
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

// 参数化查询（防止SQL注入）
let user_id = 123;
let user = ctx.database()
    .query_one("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
    .await?;

if let Some(user_data) = user {
    ctx.log().info(format!("Found user: {:?}", user_data));
}

// 插入数据
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

// 更新数据
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

// 删除数据
let deleted_rows = ctx.database()
    .execute("DELETE FROM users WHERE id = $1", vec![999.into()])
    .await?;

ctx.log().info(format!("Deleted {} rows", deleted_rows));
```

## 查询构建器

### 构建复杂查询

```rust
use dmsc::prelude::*;

// 构建SELECT查询
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

// 构建JOIN查询
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

// 构建聚合查询
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

// 构建子查询
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

### 类型安全查询

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

// 类型安全查询
let users: Vec<User> = ctx.database()
    .query_as::<User>("SELECT id, name, email, age, active, created_at FROM users WHERE active = $1", vec![true.into()])
    .await?;

for user in users {
    ctx.log().info(format!("User: {:?}", user));
}

// 单条记录查询
let user: Option<User> = ctx.database()
    .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![123.into()])
    .await?;

if let Some(u) = user {
    ctx.log().info(format!("Found user: {:?}", u));
}
```

## 事务管理

### 基本事务

```rust
use dmsc::prelude::*;
use serde_json::json;

// 手动事务管理
let tx = ctx.database().begin_transaction().await?;

try {
    // 在事务中执行多个操作
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
    
    // 提交事务
    ctx.database().commit_transaction(tx).await?;
    ctx.log().info(format!("User created with ID: {}", user_id));
    
} catch (e) {
    // 回滚事务
    ctx.database().rollback_transaction(tx).await?;
    ctx.log().error(format!("Transaction failed, rolled back: {}", e));
    return Err(e);
}
```

### 事务便捷方法

```rust
use dmsc::prelude::*;

// 使用事务便捷方法
let result = ctx.database().with_transaction(|| async {
    // 转账操作
    let from_account = 1001;
    let to_account = 1002;
    let amount = 500.00;
    
    // 扣减源账户
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
    
    // 增加目标账户
    ctx.database()
        .execute_in_transaction(
            "UPDATE accounts SET balance = balance + $1 WHERE id = $2",
            vec![amount.into(), to_account.into()]
        )
        .await?;
    
    // 记录交易
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

### 隔离级别

```rust
use dmsc::prelude::*;

// 设置事务隔离级别
let isolation_levels = vec![
    DMSCIsolationLevel::ReadUncommitted,
    DMSCIsolationLevel::ReadCommitted,
    DMSCIsolationLevel::RepeatableRead,
    DMSCIsolationLevel::Serializable,
];

for level in isolation_levels {
    let result = ctx.database().with_transaction_isolation(level, || async {
        // 在高隔离级别下执行敏感操作
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

## 连接池管理

### 连接池监控

```rust
use dmsc::prelude::*;
use serde_json::json;

// 获取连接池统计信息
let pool_stats = ctx.database().get_pool_stats().await?;
ctx.log().info(format!("Pool stats: {:?}", pool_stats));

// 监控连接池
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

// 设置连接池事件监听器
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

### 连接池调优

```rust
use dmsc::prelude::*;

// 动态调整连接池大小
ctx.database().set_pool_size(30).await?;
ctx.log().info("Increased pool size to 30");

// 获取连接池配置
let pool_config = ctx.database().get_pool_config().await?;
ctx.log().info(format!("Current pool config: {:?}", pool_config));

// 设置连接超时
ctx.database().set_connection_timeout(Duration::from_secs(45)).await?;

// 清理空闲连接
let cleaned = ctx.database().cleanup_idle_connections().await?;
ctx.log().info(format!("Cleaned up {} idle connections", cleaned));
```

## 数据库迁移

### 创建迁移

```rust
use dmsc::prelude::*;

// 创建新的迁移文件
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

// 创建复杂迁移
let complex_migration = DMSCMigration {
    version: 2024011502,
    name: "create_ecommerce_schema".to_string(),
    up: r#"
        -- 创建用户表
        CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- 创建产品表
        CREATE TABLE products (
            id SERIAL PRIMARY KEY,
            name VARCHAR(200) NOT NULL,
            description TEXT,
            price DECIMAL(10,2) NOT NULL,
            stock_quantity INTEGER DEFAULT 0,
            category_id INTEGER,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- 创建订单表
        CREATE TABLE orders (
            id SERIAL PRIMARY KEY,
            user_id INTEGER REFERENCES users(id),
            total_amount DECIMAL(10,2) NOT NULL,
            status VARCHAR(50) DEFAULT 'pending',
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        
        -- 创建订单项表
        CREATE TABLE order_items (
            id SERIAL PRIMARY KEY,
            order_id INTEGER REFERENCES orders(id),
            product_id INTEGER REFERENCES products(id),
            quantity INTEGER NOT NULL,
            unit_price DECIMAL(10,2) NOT NULL,
            subtotal DECIMAL(10,2) NOT NULL
        );
        
        -- 创建索引
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

### 运行迁移

```rust
use dmsc::prelude::*;

// 运行所有待处理迁移
let migration_result = ctx.database().migrate().await?;
ctx.log().info(format!("Applied {} migrations", migration_result.applied_migrations.len()));

// 获取迁移状态
let migration_status = ctx.database().get_migration_status().await?;
for status in migration_status {
    ctx.log().info(format!("Migration {}: {:?}", status.version, status.status));
}

// 回滚迁移
let rollback_result = ctx.database().rollback(1).await?; // 回滚1个迁移
ctx.log().info(format!("Rolled back {} migrations", rollback_result.rolled_back_migrations.len()));

// 重置数据库
ctx.database().reset().await?;
ctx.log().warn("Database reset completed - all data lost");
```

### 数据迁移

```rust
use dmsc::prelude::*;

// 数据迁移示例
let data_migration = DMSCMigration {
    version: 2024011503,
    name: "migrate_user_data".to_string(),
    up: r#"
        -- 添加新列
        ALTER TABLE users ADD COLUMN full_name VARCHAR(200);
        
        -- 迁移数据
        UPDATE users SET full_name = name;
        
        -- 更新数据结构
        UPDATE users SET name = SPLIT_PART(full_name, ' ', 1);
        
        -- 添加约束
        ALTER TABLE users ALTER COLUMN full_name SET NOT NULL;
    "#.to_string(),
    down: r#"
        -- 回滚数据迁移
        UPDATE users SET name = full_name;
        ALTER TABLE users DROP COLUMN full_name;
    "#.to_string(),
};

ctx.database().create_migration(data_migration).await?;
```

## 高级功能

### 批量操作

```rust
use dmsc::prelude::*;
use serde_json::json;

// 批量插入
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

// 批量更新
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

### 数据库函数

```rust
use dmsc::prelude::*;

// 创建数据库函数
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

// 调用数据库函数
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

### 全文搜索

```rust
use dmsc::prelude::*;

// PostgreSQL全文搜索
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

// 创建全文搜索索引
let create_index = r#"
    CREATE INDEX idx_products_search 
    ON products 
    USING gin(to_tsvector('english', name || ' ' || description));
"#;

ctx.database().execute(create_index, vec![]).await?;
```

## 错误处理

### 数据库错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 处理数据库错误
match ctx.database().query("SELECT * FROM non_existent_table", vec![]).await {
    Ok(results) => {
        ctx.log().info(format!("Query returned {} rows", results.len()));
    }
    Err(DMSCError::DatabaseConnectionError(e)) => {
        ctx.log().error(format!("Database connection failed: {}", e));
        // 尝试重新连接或降级处理
        retry_database_connection().await?;
    }
    Err(DMSCError::DatabaseQueryError(e)) => {
        ctx.log().error(format!("Database query failed: {}", e));
        // 检查是否是语法错误或表不存在
        if e.contains("doesn't exist") {
            ctx.log().warn("Table does not exist, consider running migrations");
        }
    }
    Err(DMSCError::DatabaseTimeoutError(e)) => {
        ctx.log().warn(format!("Database query timed out: {}", e));
        // 优化查询或增加超时时间
        optimize_query_performance().await?;
    }
    Err(DMSCError::DatabaseConstraintError(e)) => {
        ctx.log().error(format!("Database constraint violation: {}", e));
        // 处理唯一性约束、外键约束等
        handle_constraint_violation(&e).await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected database error: {}", e));
        return Err(e);
    }
}

// 连接池降级处理
async fn handle_database_unavailability() -> DMSCResult<()> {
    ctx.log().warn("Database is unavailable, switching to read-only cache mode");
    
    // 启用缓存降级
    ctx.cache().set_readonly_mode(true)?;
    
    // 定期重试连接
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

## 运行步骤

</div>

### 1. 构建项目

```bash
cargo build
```

### 2. 运行项目

```bash
cargo run
```

<div align="center">

## 预期结果

</div>

运行示例后，您应该会看到类似以下的输出：

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

## 扩展功能

</div>

### 1. 实现复杂查询优化

```rust
// 使用预编译语句优化重复查询
let mut stmt = ctx.database()
    .prepare("SELECT * FROM users WHERE id = $1 AND active = $2")
    .await?;

for user_id in vec![1, 2, 3, 4, 5] {
    let user = stmt.query_one(vec![user_id.into(), true.into()]).await?;
    if let Some(u) = user {
        ctx.logger().info("database", &format!("User {}: {:?}", user_id, u))?;
    }
}

// 使用批量查询减少网络往返
let user_ids = vec![1, 2, 3, 4, 5];
let users = ctx.database()
    .query(
        "SELECT * FROM users WHERE id = ANY($1) AND active = $2",
        vec![user_ids.into(), true.into()]
    )
    .await?;

ctx.logger().info("database", &format!("Batch query returned {} users", users.len()))?;
```

### 2. 实现数据库连接监控

```rust
// 设置连接池监控
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

// 定期检查连接池状态
let pool_stats = ctx.database().get_pool_stats().await?;
ctx.logger().info("database", &format!(
    "Pool stats - Active: {}, Idle: {}, Total: {}",
    pool_stats.active_connections,
    pool_stats.idle_connections,
    pool_stats.total_connections
))?;
```

### 3. 实现数据缓存策略

```rust
// 实现查询结果缓存
async fn get_user_with_cache(ctx: &DMSCServiceContext, user_id: i32) -> DMSCResult<Option<User>> {
    let cache_key = format!("user:{}", user_id);
    
    // 先尝试从缓存获取
    if let Some(cached_user) = ctx.cache().get::<User>(&cache_key).await? {
        ctx.logger().debug("cache", &format!("User {} found in cache", user_id))?;
        return Ok(Some(cached_user));
    }
    
    // 缓存未命中，从数据库查询
    let user = ctx.database()
        .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
        .await?;
    
    // 将结果存入缓存（5分钟过期）
    if let Some(ref u) = user {
        ctx.cache().set(&cache_key, u, 300).await?;
        ctx.logger().debug("cache", &format!("User {} cached for 5 minutes", user_id))?;
    }
    
    Ok(user)
}

// 实现缓存失效策略
async fn update_user_with_cache_invalidation(ctx: &DMSCServiceContext, user: &User) -> DMSCResult<()> {
    // 更新数据库
    ctx.database()
        .execute(
            "UPDATE users SET name = $1, email = $2 WHERE id = $3",
            vec![user.name.clone().into(), user.email.clone().into(), user.id.into()]
        )
        .await?;
    
    // 失效相关缓存
    let cache_key = format!("user:{}", user.id);
    ctx.cache().delete(&cache_key).await?;
    ctx.cache().delete("users:list").await?;
    
    ctx.logger().info("cache", &format!("Cache invalidated for user {}", user.id))?;
    Ok(())
}
```

### 4. 实现数据库分片

```rust
// 实现基于用户ID的分片策略
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

// 在应用中使用分片
let shard_manager = UserShardManager { shard_count: 4 };

// 根据用户ID路由到对应的分片
let user_id = 12345;
let shard_id = shard_manager.get_shard_for_user(user_id);
let shard_connection = shard_manager.get_shard_connection(shard_id);

ctx.logger().info("shard", &format!("User {} routed to shard {}", user_id, shard_id))?;

// 在指定分片上执行查询
let user = ctx.database()
    .using_connection(&shard_connection)
    .query_one_as::<User>("SELECT * FROM users WHERE id = $1", vec![user_id.into()])
    .await?;
```

<div align="center">

## 最佳实践

</div>

1. **使用参数化查询**: 防止SQL注入攻击，提高查询性能
2. **合理使用事务**: 保持事务简短，避免长时间锁定资源
3. **连接池管理**: 根据负载和并发量调整连接池大小
4. **索引优化**: 为常用查询字段创建复合索引，避免过度索引
5. **查询优化**: 避免N+1查询问题，使用JOIN或批量查询
6. **错误处理**: 妥善处理数据库错误，实现降级和重试策略
7. **数据验证**: 在应用层验证数据，减少数据库约束错误
8. **迁移管理**: 使用版本化迁移管理数据库结构变更
9. **监控性能**: 监控查询性能、连接池状态和慢查询
10. **备份策略**: 定期备份数据库，测试恢复流程并验证数据完整性

<div align="center">

## 总结

</div>

本示例展示了如何使用DMSC的database模块进行数据库操作，包括：

- 多数据库连接配置（PostgreSQL、MySQL、SQLite）
- 基本查询和参数化查询
- 查询构建器和复杂查询
- 事务管理和隔离级别
- 连接池监控和调优
- 数据库迁移和版本管理
- 批量操作和性能优化
- 错误处理和降级策略

通过本示例，您应该已经掌握了DMSC数据库模块的核心功能和使用方法。您可以在此基础上构建更复杂的数据库应用。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信