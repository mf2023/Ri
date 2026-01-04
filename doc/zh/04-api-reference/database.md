<div align="center">

# Database API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

database模块提供统一数据库访问层，支持多种数据库类型、连接池管理、事务处理与查询构建器。

## 模块概述

</div>

database模块包含以下子模块：

- **core**: 数据库核心接口和类型定义
- **pools**: 连接池管理
- **query**: 查询构建器
- **migration**: 数据库迁移
- **transaction**: 事务管理

<div align="center">

## 核心组件

</div>

### DMSCDatabase

数据库管理器主接口，提供统一的数据库访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `execute(query)` | 执行SQL查询 | `query: &str` | `DMSCResult<DMSCQueryResult>` |
| `query(query)` | 执行查询并返回结果 | `query: &str` | `DMSCResult<Vec<DMSCRow>>` |
| `query_one(query)` | 执行查询并返回单行结果 | `query: &str` | `DMSCResult<DMSCRow>` |
| `begin_transaction()` | 开始事务 | 无 | `DMSCResult<DMSCTransaction>` |
| `migrate()` | 执行数据库迁移 | 无 | `DMSCResult<()>` |
| `get_connection()` | 获取数据库连接 | 无 | `DMSCResult<DMSCConnection>` |
| `get_pool_stats()` | 获取连接池统计 | 无 | `DMSCResult<PoolStats>` |
| `ping()` | 测试数据库连接 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dms::prelude::*;

// 执行查询
let results = ctx.database().query("SELECT id, name, email FROM users WHERE active = true")?;

for row in results {
    let id: i32 = row.get("id")?;
    let name: String = row.get("name")?;
    let email: String = row.get("email")?;
    
    println!("User {}: {} <{}>", id, name, email);
}

// 执行更新
ctx.database().execute("UPDATE users SET last_login = NOW() WHERE id = 123")?;

// 获取单行结果
let user = ctx.database().query_one("SELECT * FROM users WHERE id = 123")?;
let name: String = user.get("name")?;
```

### DMSCDatabaseConfig

数据库配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `database_type` | `DMSCDatabaseType` | 数据库类型 | `Postgres` |
| `host` | `String` | 数据库主机 | `"localhost"` |
| `port` | `u16` | 数据库端口 | `5432` |
| `database` | `String` | 数据库名称 | `""` |
| `username` | `String` | 用户名 | `""` |
| `password` | `String` | 密码 | `""` |
| `max_connections` | `u32` | 最大连接数 | `10` |
| `min_connections` | `u32` | 最小连接数 | `1` |
| `connection_timeout` | `Duration` | 连接超时 | `30s` |
| `idle_timeout` | `Duration` | 空闲超时 | `600s` |
| `max_lifetime` | `Duration` | 连接最大生命周期 | `1800s` |

#### 配置示例

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

数据库类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Postgres` | PostgreSQL |
| `MySQL` | MySQL |
| `SQLite` | SQLite |
| `MongoDB` | MongoDB |
| `Redis` | Redis |

## 查询构建器

### DMSCQueryBuilder

查询构建器，用于构建类型安全的SQL查询。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `select(columns)` | 选择列 | `columns: &[&str]` | `Self` |
| `from(table)` | 指定表 | `table: &str` | `Self` |
| `where(condition)` | 添加条件 | `condition: &str` | `Self` |
| `where_eq(column, value)` | 添加等值条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_in(column, values)` | 添加IN条件 | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `order_by(column, direction)` | 添加排序 | `column: &str`, `direction: OrderDirection` | `Self` |
| `limit(count)` | 设置限制 | `count: i64` | `Self` |
| `offset(count)` | 设置偏移 | `count: i64` | `Self` |
| `join(table, on)` | 添加连接 | `table: &str`, `on: &str` | `Self` |
| `build()` | 构建查询 | 无 | `DMSCResult<String>` |

#### 使用示例

```rust
use dms::prelude::*;

// 构建SELECT查询
let query = DMSCQueryBuilder::new()
    .select(&["id", "name", "email"])
    .from("users")
    .where_eq("active", true)
    .where_eq("role", "admin")
    .order_by("created_at", OrderDirection::Desc)
    .limit(10)
    .build()?;

let results = ctx.database().query(&query)?;

// 构建复杂查询
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

### 插入构建器

```rust
use dms::prelude::*;

// 构建INSERT查询
let insert_query = DMSCInsertBuilder::new()
    .into("users")
    .columns(&["name", "email", "created_at"])
    .values(&["John Doe", "john@example.com", "NOW()"])
    .build()?;

ctx.database().execute(&insert_query)?;

// 批量插入
let batch_insert = DMSCInsertBuilder::new()
    .into("users")
    .columns(&["name", "email"])
    .values(&["Alice", "alice@example.com"])
    .values(&["Bob", "bob@example.com"])
    .values(&["Charlie", "charlie@example.com"])
    .build()?;

ctx.database().execute(&batch_insert)?;
```

### 更新构建器

```rust
use dms::prelude::*;

// 构建UPDATE查询
let update_query = DMSCUpdateBuilder::new()
    .table("users")
    .set("last_login", "NOW()")
    .set("login_count", "login_count + 1")
    .where_eq("id", 123)
    .build()?;

ctx.database().execute(&update_query)?;
```

### 删除构建器

```rust
use dms::prelude::*;

// 构建DELETE查询
let delete_query = DMSCDeleteBuilder::new()
    .from("users")
    .where_eq("active", false)
    .where_lt("last_login", "NOW() - INTERVAL '1 year'")
    .build()?;

ctx.database().execute(&delete_query)?;
```
<div align="center">

## 事务管理

</div>

### DMSCTransaction

事务接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `execute(query)` | 在事务中执行查询 | `query: &str` | `DMSCResult<DMSCQueryResult>` |
| `query(query)` | 在事务中执行查询并返回结果 | `query: &str` | `DMSCResult<Vec<DMSCRow>>` |
| `commit()` | 提交事务 | 无 | `DMSCResult<()>` |
| `rollback()` | 回滚事务 | 无 | `DMSCResult<()>` |
| `savepoint(name)` | 创建保存点 | `name: &str` | `DMSCResult<()>` |
| `rollback_to_savepoint(name)` | 回滚到保存点 | `name: &str` | `DMSCResult<()>` |

#### 使用示例

```rust
use dms::prelude::*;

// 开始事务
let mut transaction = ctx.database().begin_transaction()?;

try {
    // 在事务中执行操作
    transaction.execute("UPDATE accounts SET balance = balance - 100 WHERE id = 1")?;
    transaction.execute("UPDATE accounts SET balance = balance + 100 WHERE id = 2")?;
    
    // 创建保存点
    transaction.savepoint("before_fee")?;
    
    // 执行可能失败的操作
    match transaction.execute("UPDATE accounts SET balance = balance - 5 WHERE id = 2") {
        Ok(_) => {
            // 操作成功，提交事务
            transaction.commit()?;
        }
        Err(_) => {
            // 操作失败，回滚到保存点
            transaction.rollback_to_savepoint("before_fee")?;
            transaction.commit()?;
        }
    }
} catch (e) {
    // 发生错误，回滚整个事务
    transaction.rollback()?;
    return Err(e);
}
```

### 事务隔离级别

```rust
use dms::prelude::*;

// 设置事务隔离级别
let transaction = ctx.database()
    .begin_transaction_with_isolation(TransactionIsolation::Serializable)?;

// 不同的隔离级别
let read_uncommitted = TransactionIsolation::ReadUncommitted;
let read_committed = TransactionIsolation::ReadCommitted;
let repeatable_read = TransactionIsolation::RepeatableRead;
let serializable = TransactionIsolation::Serializable;
```

<div align="center">

## 连接池管理

</div>

### 连接池配置

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

### 连接池监控

```rust
use dms::prelude::*;

// 获取连接池统计信息
let stats = ctx.database().get_pool_stats()?;

println!("Active connections: {}", stats.active_connections);
println!("Idle connections: {}", stats.idle_connections);
println!("Waiting connections: {}", stats.waiting_connections);
println!("Total connections created: {}", stats.total_created);
println!("Total connections destroyed: {}", stats.total_destroyed);
```

### 连接池健康检查

```rust
use dms::prelude::*;

// 检查连接池健康状态
match ctx.database().ping() {
    Ok(_) => {
        println!("Database connection is healthy");
    }
    Err(e) => {
        println!("Database connection failed: {}", e);
        
        // 尝试重新创建连接池
        ctx.database().recreate_pool()?;
    }
}
```
<div align="center">

## 数据库迁移

</div>

### DMSCMigration

迁移接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_migration(migration)` | 添加迁移 | `migration: impl Migration` | `()` |
| `migrate()` | 执行迁移 | 无 | `DMSCResult<()>` |
| `rollback()` | 回滚迁移 | 无 | `DMSCResult<()>` |
| `get_status()` | 获取迁移状态 | 无 | `DMSCResult<Vec<MigrationStatus>>` |

### 创建迁移

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

// 添加迁移
ctx.database().migration().add_migration(CreateUsersTable)?;

// 执行迁移
ctx.database().migrate()?;
```

### 迁移管理

```rust
use dms::prelude::*;

// 获取迁移状态
let migration_status = ctx.database().migration().get_status()?;

for status in migration_status {
    println!("Migration: {} ({})", status.name, status.version);
    println!("Status: {:?}", status.status);
    println!("Applied at: {:?}", status.applied_at);
}

// 回滚最后一步迁移
ctx.database().migration().rollback()?;

// 回滚到指定版本
ctx.database().migration().rollback_to("20240115000001")?;
```

<div align="center">

## 高级功能

</div>

### 批量操作

```rust
use dms::prelude::*;

// 批量插入
let users = vec![
    ("Alice", "alice@example.com"),
    ("Bob", "bob@example.com"),
    ("Charlie", "charlie@example.com"),
];

ctx.database().batch_insert("users", &["name", "email"], users)?;

// 批量更新
let updates = vec![
    (1, "Alice Smith"),
    (2, "Bob Johnson"),
    (3, "Charlie Brown"),
];

ctx.database().batch_update("users", "id", &["name"], updates)?;
```

### 预处理语句

```rust
use dms::prelude::*;

// 准备预处理语句
let stmt = ctx.database().prepare("SELECT * FROM users WHERE id = $1 AND active = $2")?;

// 执行预处理语句
let results = stmt.query(&[123, true])?;

// 多次执行
for user_id in [123, 456, 789] {
    let results = stmt.query(&[user_id, true])?;
    // 处理结果
}
```

### 异步操作

```rust
use dms::prelude::*;

// 异步查询
let results = ctx.database().query_async("SELECT * FROM users WHERE active = true").await?;

// 异步事务
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

## 错误处理

</div>  

### 数据库错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `DATABASE_CONNECTION_ERROR` | 数据库连接错误 |
| `DATABASE_QUERY_ERROR` | 数据库查询错误 |
| `DATABASE_TRANSACTION_ERROR` | 数据库事务错误 |
| `DATABASE_MIGRATION_ERROR` | 数据库迁移错误 |
| `DATABASE_POOL_ERROR` | 连接池错误 |

### 错误处理示例

```rust
use dms::prelude::*;

match ctx.database().query("SELECT * FROM users WHERE id = 123") {
    Ok(results) => {
        // 查询成功
        for row in results {
            println!("User: {:?}", row);
        }
    }
    Err(DMSCError { code, .. }) if code == "DATABASE_CONNECTION_ERROR" => {
        // 连接错误，尝试重新连接
        ctx.log().error("Database connection lost, attempting to reconnect");
        ctx.database().recreate_pool()?;
        
        // 重试查询
        let results = ctx.database().query("SELECT * FROM users WHERE id = 123")?;
    }
    Err(e) => {
        // 其他错误
        return Err(e);
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **使用连接池**: 避免频繁创建和销毁连接
2. **使用预处理语句**: 提高性能并防止SQL注入
3. **合理使用事务**: 保持事务简短，避免长时间锁定
4. **正确处理错误**: 区分可恢复和不可恢复的错误
5. **监控连接池**: 监控连接池的使用情况和性能
6. **使用迁移**: 使用数据库迁移管理schema变化
7. **索引优化**: 为常用查询添加适当的索引
8. **批量操作**: 使用批量操作减少数据库往返

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置
- [cache](./cache.md): 缓存模块，提供多后端缓存抽象，缓存用户会话和权限数据
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [security](./security.md): 安全模块，提供加密、哈希和验证功能
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据