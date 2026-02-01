<div align="center">

# Database API参考

**Version: 0.1.6**

**Last modified date: 2026-01-18**

database模块提供统一数据库访问层，支持多种数据库类型、连接池管理、事务处理与ORM查询构建器。

## 模块概述

</div>

database模块包含以下子模块：

- **core**: 数据库核心接口和类型定义
- **pools**: 连接池管理
- **orm**: 对象关系映射和查询构建器
- **migration**: 数据库迁移
- **transaction**: 事务管理

<div align="center">

## 核心组件

</div>

### DMSCDatabase

数据库操作接口trait，提供统一的数据库访问。

**注意**: DMSCDatabase是一个trait，不能直接实例化。需要通过DMSCDatabasePool获取具体的数据库连接。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `execute(sql)` | 执行SQL查询 | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | 执行查询并返回结果 | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `query_one(sql)` | 执行查询并返回单行结果 | `sql: &str` | `DMSCResult<Option<DMSCDBRow>>` |
| `ping()` | 测试数据库连接 | 无 | `DMSCResult<bool>` |
| `is_connected()` | 检查连接状态 | 无 | `bool` |
| `close()` | 关闭连接 | 无 | `DMSCResult<()>` |
| `transaction()` | 开始事务 | 无 | `DMSCResult<Box<dyn DMSCDatabaseTransaction>>` |
| `execute_with_params(sql, params)` | 带参数执行SQL | `sql: &str`, `params: &[serde_json::Value]` | `DMSCResult<u64>` |
| `query_with_params(sql, params)` | 带参数执行查询 | `sql: &str`, `params: &[serde_json::Value]` | `DMSCResult<DMSCDBResult>` |
| `batch_execute(sql, params)` | 批量执行 | `sql: &str`, `params: &[Vec<serde_json::Value>]` | `DMSCResult<Vec<u64>>` |
| `batch_query(sql, params)` | 批量查询 | `sql: &str`, `params: &[Vec<serde_json::Value>]` | `DMSCResult<Vec<DMSCDBResult>>` |

#### 使用示例

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

    // 执行查询
    let results = db.query("SELECT id, name, email FROM users WHERE active = true").await?;

    for row in results {
        let id: i64 = row.get("id");
        let name: String = row.get("name");
        let email: String = row.get("email");
        
        println!("User {}: {} <{}>", id, name, email);
    }

    // 执行更新
    db.execute("UPDATE users SET last_login = NOW() WHERE id = 123").await?;

    // 获取单行结果
    let user = db.query_one("SELECT * FROM users WHERE id = 123").await?;
    if let Some(row) = user {
        let name: String = row.get("name")?;
    }

    // 带参数查询
    let rows = db.query_with_params(
        "SELECT * FROM users WHERE name = $1 AND active = $2",
        &[serde_json::json!("alice"), serde_json::json!(true)]
    ).await?;

    Ok(())
}
```

### DMSCDatabaseConfig

数据库配置构建器。

#### 静态方法

| 方法 | 描述 | 返回值 |
|:--------|:-------------|:--------|
| `postgres()` | 创建PostgreSQL默认配置 | `DMSCDatabaseConfig` |
| `mysql()` | 创建MySQL默认配置 | `DMSCDatabaseConfig` |
| `sqlite()` | 创建SQLite默认配置 | `DMSCDatabaseConfig` |

#### 构建器方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `host()` | 设置主机 | `host: &str` | `Self` |
| `port()` | 设置端口 | `port: u16` | `Self` |
| `database()` | 设置数据库名 | `database: &str` | `Self` |
| `user()` | 设置用户名 | `user: &str` | `Self` |
| `password()` | 设置密码 | `password: &str` | `Self` |
| `max_connections()` | 最大连接数 | `n: u32` | `Self` |
| `min_idle_connections()` | 最小空闲连接数 | `n: u32` | `Self` |
| `connection_timeout_secs()` | 连接超时(秒) | `secs: u64` | `Self` |
| `idle_timeout_secs()` | 空闲超时(秒) | `secs: u64` | `Self` |
| `max_lifetime_secs()` | 最大生命周期(秒) | `secs: u64` | `Self` |
| `build()` | 构建配置 | 无 | `DMSCDatabaseConfig` |

#### 配置示例

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

数据库类型枚举。

| 变体 | 描述 |
|:--------|:-------------|
| `Postgres` | PostgreSQL |
| `MySQL` | MySQL |
| `SQLite` | SQLite |
| `MongoDB` | MongoDB（计划中） |
| `Redis` | Redis（计划中） |

**注意**: MongoDB和Redis支持正在开发中，将在后续版本中提供。

<div align="center">

## 连接池管理

</div>

### DMSCDatabasePool

数据库连接池管理器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建连接池 | `config: DMSCDatabaseConfig` | `DMSCResult<Self>` |
| `get()` | 获取连接 | 无 | `DMSCResult<PooledDatabase>` |
| `close()` | 关闭连接池 | 无 | `DMSCResult<()>` |
| `metrics()` | 获取统计信息 | 无 | `DatabaseMetrics` |

#### 使用示例

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

    // 使用数据库连接...

    // 获取连接池统计
    let metrics = pool.metrics();
    println!("活跃连接: {}", metrics.active_connections);
    println!("空闲连接: {}", metrics.idle_connections);

    Ok(())
}
```

### PooledDatabase

池化数据库连接包装器。

实现DMSCDatabase trait，可直接用于数据库操作。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `id()` | 获取连接ID | 无 | `u32` |
| `execute(sql)` | 执行SQL | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | 执行查询 | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `query_one(sql)` | 查询单行 | `sql: &str` | `DMSCResult<Option<DMSCDBRow>>` |
| `ping()` | 测试连接 | 无 | `DMSCResult<bool>` |
| `is_connected()` | 检查连接 | 无 | `bool` |
| `pool_metrics()` | 获取池统计 | 无 | `DatabaseMetrics` |

### DatabaseMetrics

连接池统计信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `active_connections` | `u64` | 活跃连接数 |
| `idle_connections` | `u64` | 空闲连接数 |
| `total_connections` | `u64` | 总连接数 |
| `queries_executed` | `u64` | 执行查询数 |
| `query_duration_ms` | `f64` | 查询平均耗时(ms) |
| `errors` | `u64` | 错误数 |

<div align="center">

## ORM与查询构建器

</div>

### QueryBuilder

ORM查询构建器，提供类型安全的查询构建。

**注意**: QueryBuilder位于`dmsc::database::orm`模块中。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新实例 | 无 | `Self` |
| `table(name)` | 设置表名 | `name: &str` | `Self` |
| `select(columns)` | 选择列 | `columns: &[&str]` | `Self` |
| `and_where(condition)` | 添加AND条件 | `condition: &str` | `Self` |
| `or_where(condition)` | 添加OR条件 | `condition: &str` | `Self` |
| `where_eq(column, value)` | 等值条件 | `column: &str`, `value: serde_json::Value` | `Self` |
| `where_in(column, values)` | IN条件 | `column: &str`, `values: &[serde_json::Value]` | `Self` |
| `order_by(column, order)` | 排序 | `column: &str`, `order: SortOrder` | `Self` |
| `limit(n)` | 限制数量 | `n: u64` | `Self` |
| `offset(n)` | 偏移量 | `n: u64` | `Self` |
| `build()` | 构建SQL | 无 | `String` |

#### 使用示例

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

表定义结构，用于模式管理。

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

列定义结构。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `name` | `String` | 列名 |
| `column_type` | `ColumnType` | 列类型 |
| `is_nullable` | `bool` | 是否可为空 |
| `is_primary_key` | `bool` | 是否为主键 |
| `is_unique` | `bool` | 是否唯一 |
| `default_value` | `Option<String>` | 默认值 |

### Criteria

查询条件，用于构建WHERE子句。

```rust
use dmsc::database::orm::{Criteria, ComparisonOperator, LogicalOperator};

let criteria = Criteria::new("age", ComparisonOperator::GreaterThan, serde_json::json!(18));
let criteria2 = Criteria::new("status", ComparisonOperator::Equal, serde_json::json!("active"));
```

### JoinClause

连接子句，用于构建JOIN查询。

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

逻辑运算符，用于组合条件。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `And` | AND运算符 |
| `Or` | OR运算符 |

### ComparisonOperator

比较运算符，用于WHERE子句。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Equal` | 等于 |
| `NotEqual` | 不等于 |
| `GreaterThan` | 大于 |
| `LessThan` | 小于 |
| `GreaterThanOrEqual` | 大于等于 |
| `LessThanOrEqual` | 小于等于 |
| `Like` | LIKE |
| `In` | IN |
| `NotIn` | NOT IN |
| `IsNull` | IS NULL |
| `IsNotNull` | IS NOT NULL |

### Pagination

分页信息，用于查询结果分页。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `page` | `u64` | 当前页码（从1开始） |
| `page_size` | `u64` | 每页项目数 |

### SortOrder

排序顺序枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Asc` | 升序 |
| `Desc` | 降序 |

### DMSCORMSimpleRepository

简单ORM仓储，提供基础的CRUD操作。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(table_name)` | 创建实例 | `table_name: &str` | `Self` |
| `find_all()` | 查询所有 | 无 | `DMSCResult<Vec<serde_json::Value>>` |
| `find_by_id(id)` | 按ID查询 | `id: i64` | `DMSCResult<Option<serde_json::Value>>` |
| `find_by_where(where)` | 条件查询 | `where: &str` | `DMSCResult<Vec<serde_json::Value>>` |
| `create(data)` | 创建记录 | `data: &serde_json::Value` | `DMSCResult<serde_json::Value>` |
| `update(id, data)` | 更新记录 | `id: i64`, `data: &serde_json::Value` | `DMSCResult<serde_json::Value>` |
| `delete(id)` | 删除记录 | `id: i64` | `DMSCResult<()>` |

<div align="center">

## 事务管理

</div>

### DMSCDatabaseTransaction

事务接口trait。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `execute(sql)` | 执行SQL | `sql: &str` | `DMSCResult<u64>` |
| `query(sql)` | 执行查询 | `sql: &str` | `DMSCResult<DMSCDBResult>` |
| `commit()` | 提交事务 | 无 | `DMSCResult<()>` |
| `rollback()` | 回滚事务 | 无 | `DMSCResult<()>` |
| `close()` | 关闭事务 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres()
        .database("mydb")
        .build();

    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // 开始事务
    let mut tx = db.transaction().await?;

    // 在事务中执行操作
    tx.execute("UPDATE accounts SET balance = balance - 100 WHERE id = $1", &[&1]).await?;
    tx.execute("UPDATE accounts SET balance = balance + 100 WHERE id = $1", &[&2]).await?;

    // 提交事务
    tx.commit().await?;

    Ok(())
}
```

<div align="center">

## 数据库迁移

</div>

### DMSCDatabaseMigrator

数据库迁移管理器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新实例 | 无 | `Self` |
| `with_migrations_dir(dir)` | 设置迁移目录 | `dir: PathBuf` | `Self` |
| `add_migration(migration)` | 添加迁移 | `migration: DMSCDatabaseMigration` | `()` |
| `get_migrations()` | 获取所有迁移 | 无 | `&[DMSCDatabaseMigration]` |
| `load_migrations_from_dir(dir)` | 从目录加载 | `dir: &str` | `std::io::Result<()>` |

### DMSCDatabaseMigration

迁移定义结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `version` | `u32` | 版本号 |
| `name` | `String` | 迁移名称 |
| `sql_up` | `String` | 升级SQL |
| `sql_down` | `Option<String>` | 回滚SQL |
| `timestamp` | `DateTime<Utc>` | 创建时间 |

#### 创建迁移

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

迁移历史记录。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `version` | `u32` | 版本号 |
| `name` | `String` | 迁移名称 |
| `applied_at` | `DateTime<Utc>` | 执行时间 |
| `checksum` | `String` | 校验和 |

<div align="center">

## 数据行操作

</div>

### DMSCDBRow

数据库结果行。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get<T>(column)` | 获取列值 | `column: &str` | `T` |
| `get_opt<T>(column)` | 获取可选值 | `column: &str` | `DMSCResult<Option<T>>` |
| `len()` | 列数 | 无 | `usize` |
| `is_empty()` | 是否为空 | 无 | `bool` |
| `columns()` | 列名列表 | 无 | `Vec<String>` |

#### 使用示例

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

查询结果集。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `len()` | 行数 | 无 | `usize` |
| `is_empty()` | 是否为空 | 无 | `bool` |
| `columns()` | 列名列表 | 无 | `Vec<String>` |
| `iter()` | 迭代器 | 无 | `Iter<'_, DMSCDBRow>` |
| `into_iter()` | 转为迭代器 | 无 | `IntoIter<DMSCDBRow>` |

#### 实现 trait

- `IntoIterator` - 支持 `for row in results` 遍历

<div align="center">

## 批量操作

</div>

```rust
use dmsc::database::{DMSCDatabasePool, DMSCDatabaseConfig};

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let config = DMSCDatabaseConfig::postgres().build();
    let pool = DMSCDatabasePool::new(config).await?;
    let db = pool.get().await?;

    // 批量插入
    let users = vec![
        vec![serde_json::json!("Alice"), serde_json::json!("alice@example.com")],
        vec![serde_json::json!("Bob"), serde_json::json!("bob@example.com")],
        vec![serde_json::json!("Charlie"), serde_json::json!("charlie@example.com")],
    ];

    let results = db.batch_execute(
        "INSERT INTO users (name, email) VALUES ($1, $2)",
        &users
    ).await?;

    println!("插入 {} 行", results.len());

    Ok(())
}
```

<div align="center">

## 错误处理

</div>

### 错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `DATABASE_CONNECTION_ERROR` | 数据库连接错误 |
| `DATABASE_QUERY_ERROR` | 数据库查询错误 |
| `DATABASE_TRANSACTION_ERROR` | 数据库事务错误 |
| `DATABASE_MIGRATION_ERROR` | 数据库迁移错误 |
| `DATABASE_POOL_ERROR` | 连接池错误 |

### 错误处理示例

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
            eprintln!("数据库错误: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
```

<div align="center">

## 最佳实践

</div>

1. **使用连接池**: 避免频繁创建和销毁连接
2. **使用参数绑定**: 使用`query_with_params`防止SQL注入
3. **合理使用事务**: 保持事务简短，避免长时间锁定
4. **正确处理错误**: 区分可恢复和不可恢复的错误
5. **监控连接池**: 使用`metrics()`监控连接池使用情况
6. **使用迁移**: 使用DMSCDatabaseMigrator管理schema变化
7. **异步操作**: 所有数据库操作都是异步的，使用`.await`
8. **批量操作**: 使用`batch_execute`减少数据库往返

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [queue](./queue.md): 消息队列模块，提供消息队列支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
