<div align="center">

# Database API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

database模块提供统一的数据库访问接口，支持多种数据库和ORM框架。

## 模块概述

</div>

database模块包含以下子模块：

- **connections**: 数据库连接管理
- **pools**: 连接池管理
- **transactions**: 事务管理
- **migrations**: 数据库迁移
- **query**: 查询构建器
- **orm**: ORM集成
- **sharding**: 分库分表
- **monitoring**: 数据库监控

<div align="center">

## 核心组件

</div>

### DMSCDatabaseConfig

数据库配置类，用于配置数据库连接。

#### 构造函数

```python
DMSCDatabaseConfig(
    db_type: str = "mysql",
    host: str = "localhost",
    port: int = 3306,
    database: str = "",
    username: str = "",
    password: str = "",
    charset: str = "utf8mb4",
    collation: str = "utf8mb4_unicode_ci",
    pool_size: int = 10,
    pool_max_size: int = 20,
    pool_min_size: int = 5,
    pool_timeout: int = 30,
    pool_recycle: int = 3600,
    pool_pre_ping: bool = True,
    ssl_enabled: bool = False,
    ssl_ca: str = "",
    ssl_cert: str = "",
    ssl_key: str = "",
    ssl_verify: bool = True,
    read_timeout: int = 30,
    write_timeout: int = 30,
    connect_timeout: int = 10,
    retry_attempts: int = 3,
    retry_delay: float = 1.0,
    enable_logging: bool = True,
    log_level: str = "INFO",
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    read_replicas: List[str] = None,
    write_nodes: List[str] = None,
    sharding_enabled: bool = False,
    shard_count: int = 4,
    shard_key: str = "id",
    backup_enabled: bool = False,
    backup_interval: int = 86400,
    backup_retention: int = 30
)
```

### DMSCDatabaseManager

数据库管理器，提供统一的数据库接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `connect()` | 建立数据库连接 | `None` | `bool` |
| `disconnect()` | 断开数据库连接 | `None` | `bool` |
| `execute(query, params=None)` | 执行SQL查询 | `query: str`, `params: Dict` | `List[Dict]` |
| `execute_one(query, params=None)` | 执行查询返回单行 | `query: str`, `params: Dict` | `Dict` |
| `execute_many(query, params=None)` | 执行查询返回多行 | `query: str`, `params: Dict` | `List[Dict]` |
| `insert(table, data)` | 插入数据 | `table: str`, `data: Dict` | `int` |
| `insert_many(table, data)` | 批量插入 | `table: str`, `data: List[Dict]` | `List[int]` |
| `update(table, data, where)` | 更新数据 | `table: str`, `data: Dict`, `where: Dict` | `int` |
| `delete(table, where)` | 删除数据 | `table: str`, `where: Dict` | `int` |
| `select(table, columns=None, where=None, order=None, limit=None)` | 查询数据 | `table: str`, `columns: List[str]`, `where: Dict`, `order: str`, `limit: int` | `List[Dict]` |
| `count(table, where=None)` | 统计行数 | `table: str`, `where: Dict` | `int` |
| `exists(table, where)` | 检查存在 | `table: str`, `where: Dict` | `bool` |
| `begin_transaction()` | 开始事务 | `None` | `DMSCTransaction` |
| `commit()` | 提交事务 | `None` | `bool` |
| `rollback()` | 回滚事务 | `None` | `bool` |
| `ping()` | 检查连接 | `None` | `bool` |
| `get_tables()` | 获取所有表 | `None` | `List[str]` |
| `get_table_info(table)` | 获取表信息 | `table: str` | `Dict` |
| `create_table(table, schema)` | 创建表 | `table: str`, `schema: Dict` | `bool` |
| `drop_table(table)` | 删除表 | `table: str` | `bool` |
| `get_stats()` | 获取统计信息 | `None` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCDatabaseManager, DMSCDatabaseConfig

# 初始化数据库管理器
config = DMSCDatabaseConfig(
    db_type="postgresql",
    host="localhost",
    port=5432,
    database="myapp",
    username="admin",
    password="secret"
)

db_manager = DMSCDatabaseManager(config)

# 连接数据库
db_manager.connect()

# 执行查询
users = db_manager.execute("SELECT * FROM users WHERE age > %s", {"age": 18})
print(f"Found {len(users)} users")

# 插入数据
user_id = db_manager.insert("users", {
    "name": "John Doe",
    "email": "john@example.com",
    "age": 25
})
print(f"Created user with ID: {user_id}")

# 更新数据
updated = db_manager.update("users", 
    {"name": "Jane Doe"}, 
    {"id": user_id}
)
print(f"Updated {updated} rows")

# 删除数据
deleted = db_manager.delete("users", {"id": user_id})
print(f"Deleted {deleted} rows")

# 使用事务
transaction = db_manager.begin_transaction()
try:
    db_manager.insert("users", {"name": "User 1"})
    db_manager.insert("users", {"name": "User 2"})
    db_manager.commit()
except Exception as e:
    db_manager.rollback()
    print(f"Transaction failed: {e}")
```

### DMSCTransaction

事务管理器，提供事务支持。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `begin()` | 开始事务 | `None` | `bool` |
| `commit()` | 提交事务 | `None` | `bool` |
| `rollback()` | 回滚事务 | `None` | `bool` |
| `savepoint(name)` | 创建保存点 | `name: str` | `bool` |
| `rollback_to_savepoint(name)` | 回滚到保存点 | `name: str` | `bool` |
| `release_savepoint(name)` | 释放保存点 | `name: str` | `bool` |
| `is_active()` | 检查事务是否活跃 | `None` | `bool` |
| `get_level()` | 获取隔离级别 | `None` | `str` |
| `set_level(level)` | 设置隔离级别 | `level: str` | `bool` |

#### 使用示例

```python
from dmsc import DMSCTransaction

# 开始事务
transaction = db_manager.begin_transaction()

try:
    # 执行多个操作
    db_manager.insert("users", {"name": "User 1"})
    db_manager.insert("users", {"name": "User 2"})
    
    # 创建保存点
    transaction.savepoint("sp1")
    
    # 执行更多操作
    db_manager.insert("users", {"name": "User 3"})
    
    # 如果需要回滚到保存点
    # transaction.rollback_to_savepoint("sp1")
    
    # 提交事务
    transaction.commit()
    print("Transaction committed successfully")
    
except Exception as e:
    # 回滚事务
    transaction.rollback()
    print(f"Transaction rolled back: {e}")
```

### DMSCQueryBuilder

查询构建器，提供链式查询接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `select(columns)` | 选择列 | `columns: List[str]` | `DMSCQueryBuilder` |
| `from_table(table)` | 指定表 | `table: str` | `DMSCQueryBuilder` |
| `where(condition)` | 添加条件 | `condition: str` | `DMSCQueryBuilder` |
| `where_in(column, values)` | 添加IN条件 | `column: str`, `values: List` | `DMSCQueryBuilder` |
| `where_between(column, min_val, max_val)` | 添加BETWEEN条件 | `column: str`, `min_val: Any`, `max_val: Any` | `DMSCQueryBuilder` |
| `join(table, on)` | 添加连接 | `table: str`, `on: str` | `DMSCQueryBuilder` |
| `left_join(table, on)` | 添加左连接 | `table: str`, `on: str` | `DMSCQueryBuilder` |
| `right_join(table, on)` | 添加右连接 | `table: str`, `on: str` | `DMSCQueryBuilder` |
| `inner_join(table, on)` | 添加内连接 | `table: str`, `on: str` | `DMSCQueryBuilder` |
| `order_by(column, direction="ASC")` | 添加排序 | `column: str`, `direction: str` | `DMSCQueryBuilder` |
| `group_by(columns)` | 添加分组 | `columns: List[str]` | `DMSCQueryBuilder` |
| `having(condition)` | 添加HAVING条件 | `condition: str` | `DMSCQueryBuilder` |
| `limit(count)` | 添加限制 | `count: int` | `DMSCQueryBuilder` |
| `offset(count)` | 添加偏移 | `count: int` | `DMSCQueryBuilder` |
| `build()` | 构建查询 | `None` | `str` |
| `execute()` | 执行查询 | `None` | `List[Dict]` |
| `execute_one()` | 执行查询返回单行 | `None` | `Dict` |
| `count()` | 执行计数查询 | `None` | `int` |

#### 使用示例

```python
from dmsc import DMSCQueryBuilder

# 创建查询构建器
query_builder = DMSCQueryBuilder(db_manager)

# 构建复杂查询
results = (query_builder
    .select(["users.id", "users.name", "COUNT(orders.id) as order_count"])
    .from_table("users")
    .left_join("orders", "users.id = orders.user_id")
    .where("users.age > 18")
    .where_in("users.status", ["active", "premium"])
    .group_by(["users.id", "users.name"])
    .having("COUNT(orders.id) > 5")
    .order_by("order_count", "DESC")
    .limit(10)
    .execute()
)

print(f"Found {len(results)} users with more than 5 orders")

# 简单查询
user = (query_builder
    .select(["*"])
    .from_table("users")
    .where("id = %s")
    .execute_one()
)
print(f"User: {user}")
```

### DMSCMigrationManager

迁移管理器，管理数据库模式变更。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `create_migration(name)` | 创建迁移 | `name: str` | `str` |
| `run_migration(migration_id)` | 运行迁移 | `migration_id: str` | `bool` |
| `rollback_migration(migration_id)` | 回滚迁移 | `migration_id: str` | `bool` |
| `get_pending_migrations()` | 获取待处理迁移 | `None` | `List[Dict]` |
| `get_applied_migrations()` | 获取已应用迁移 | `None` | `List[Dict]` |
| `get_migration_history()` | 获取迁移历史 | `None` | `List[Dict]` |
| `apply_all()` | 应用所有待处理迁移 | `None` | `bool` |
| `rollback_all()` | 回滚所有迁移 | `None` | `bool` |
| `create_table_migration(table, schema)` | 创建表迁移 | `table: str`, `schema: Dict` | `str` |
| `drop_table_migration(table)` | 删除表迁移 | `table: str` | `str` |
| `add_column_migration(table, column, definition)` | 添加列迁移 | `table: str`, `column: str`, `definition: str` | `str` |
| `drop_column_migration(table, column)` | 删除列迁移 | `table: str`, `column: str` | `str` |

#### 使用示例

```python
from dmsc import DMSCMigrationManager

# 初始化迁移管理器
migration_manager = DMSCMigrationManager(db_manager)

# 创建新迁移
migration_id = migration_manager.create_migration("add_user_profile_table")
print(f"Created migration: {migration_id}")

# 创建表迁移
schema = {
    "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
    "username": "VARCHAR(50) NOT NULL UNIQUE",
    "email": "VARCHAR(100) NOT NULL UNIQUE",
    "created_at": "TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
}
table_migration = migration_manager.create_table_migration("users", schema)

# 应用所有待处理迁移
success = migration_manager.apply_all()
if success:
    print("All migrations applied successfully")

# 获取迁移历史
history = migration_manager.get_migration_history()
print(f"Migration history: {history}")
```