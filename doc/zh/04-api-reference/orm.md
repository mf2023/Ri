<div align="center">

# ORM API 参考

**Version: 0.1.5**

**Last modified date: 2026-01-18**

ORM 模块提供类型安全的对象关系映射层，包含查询构建器、基于条件的过滤、分页支持，以及 Python 绑定。

## 模块概述

</div>

ORM 模块包含以下子模块：

- **query_builder**：带流式 API 的 SQL 查询构建器
- **criteria**：基于条件的过滤和查询
- **pagination**：分页支持，适用于大数据集
- **entity**：实体映射和数据访问
- **repository**：仓储模式实现

<div align="center">

## 核心组件

</div>

### DMSCQueryBuilder

查询构建器，用于使用流式 API 构建类型安全的 SQL 查询。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的查询构建器 | 无 | `Self` |
| `select(columns)` | 设置查询列 | `columns: &[&str]` | `Self` |
| `from(table)` | 设置查询表 | `table: &str` | `Self` |
| `where(condition)` | 添加 WHERE 条件 | `condition: &str` | `Self` |
| `where_eq(column, value)` | 添加等于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_ne(column, value)` | 添加不等于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_gt(column, value)` | 添加大于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_gte(column, value)` | 添加大于等于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_lt(column, value)` | 添加小于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_lte(column, value)` | 添加小于等于条件 | `column: &str`, `value: impl ToSql` | `Self` |
| `where_like(column, pattern)` | 添加 LIKE 条件 | `column: &str`, `pattern: &str` | `Self` |
| `where_in(column, values)` | 添加 IN 条件 | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `where_not_in(column, values)` | 添加 NOT IN 条件 | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `where_between(column, start, end)` | 添加 BETWEEN 条件 | `column: &str`, `start: impl ToSql`, `end: impl ToSql` | `Self` |
| `where_is_null(column)` | 添加 IS NULL 条件 | `column: &str` | `Self` |
| `where_is_not_null(column)` | 添加 IS NOT NULL 条件 | `column: &str` | `Self` |
| `and_where(condition)` | 添加 AND 条件 | `condition: &str` | `Self` |
| `or_where(condition)` | 添加 OR 条件 | `condition: &str` | `Self` |
| `join(table, on)` | 添加 INNER JOIN | `table: &str`, `on: &str` | `Self` |
| `left_join(table, on)` | 添加 LEFT JOIN | `table: &str`, `on: &str` | `Self` |
| `right_join(table, on)` | 添加 RIGHT JOIN | `table: &str`, `on: &str` | `Self` |
| `order_by(column, direction)` | 添加 ORDER BY | `column: &str`, `direction: OrderDirection` | `Self` |
| `group_by(columns)` | 添加 GROUP BY | `columns: &[&str]` | `Self` |
| `having(condition)` | 添加 HAVING 条件 | `condition: &str` | `Self` |
| `limit(count)` | 设置 LIMIT | `count: i64` | `Self` |
| `offset(count)` | 设置 OFFSET | `count: i64` | `Self` |
| `build()` | 构建查询 | 无 | `DMSCResult<String>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let query = DMSCQueryBuilder::new()
    .select(&["id", "name", "email", "created_at"])
    .from("users")
    .where_eq("active", true)
    .where_gte("age", 18)
    .where_like("email", "%@example.com")
    .order_by("created_at", OrderDirection::Desc)
    .limit(10)
    .offset(0)
    .build()?;

println!("生成的查询: {}", query);
// SELECT id, name, email, created_at FROM users WHERE active = true AND age >= 18 AND email LIKE '%@example.com' ORDER BY created_at DESC LIMIT 10 OFFSET 0
```

### DMSCCriteria

基于条件的过滤，用于复杂查询条件，支持 Python 绑定。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建空条件组 | 无 | `Self` |
| `with_entity(entity)` | 设置实体类型 | `entity: &str` | `Self` |
| `add_condition(condition)` | 添加条件 | `condition: DMSCCondition` | `Self` |
| `and_condition(condition)` | 添加 AND 条件 | `condition: DMSCCondition` | `Self` |
| `or_condition(condition)` | 添加 OR 条件 | `condition: DMSCCondition` | `Self` |
| `order_by(field, direction)` | 添加排序 | `field: &str`, `direction: OrderDirection` | `Self` |
| `limit(count)` | 设置限制 | `count: u64` | `Self` |
| `offset(count)` | 设置偏移 | `count: u64` | `Self` |
| `build()` | 构建条件组 | 无 | `DMSCResult<DMSCCriteriaBuilder>` |

#### 条件类型

| 条件 | 描述 |
|:--------|:-------------|
| `Eq(field, value)` | 等于条件 |
| `Ne(field, value)` | 不等于条件 |
| `Gt(field, value)` | 大于 |
| `Gte(field, value)` | 大于等于 |
| `Lt(field, value)` | 小于 |
| `Lte(field, value)` | 小于等于 |
| `Like(field, pattern)` | LIKE 模式匹配 |
| `ILike(field, pattern)` | 不区分大小写 LIKE |
| `In(field, values)` | IN 列表匹配 |
| `NotIn(field, values)` | NOT IN 列表匹配 |
| `Between(field, start, end)` | 介于两个值之间 |
| `IsNull(field)` | IS NULL 检查 |
| `IsNotNull(field)` | IS NOT NULL 检查 |

#### Python 使用示例

```python
from dmsc.orm import DMSCCriteriaPy, DMSCCondition

# 创建带条件的条件组
criteria = DMSCCriteriaPy.with_entity("User")

# 添加条件
criteria.add_condition(DMSCCondition.eq("active", True))
criteria.add_condition(DMSCCondition.gte("age", 18))
criteria.add_condition(DMSCCondition.like("email", "%@company.com"))

# 构建并执行
results = user_repository.find(criteria)
```

#### Rust 使用示例

```rust
use dmsc::prelude::*;

let criteria = DMSCCriteria::new()
    .with_entity("users")
    .add_condition(DMSCCondition::eq("status", "active"))
    .add_condition(DMSCCondition::gte("created_at", "2024-01-01"))
    .order_by("name", OrderDirection::Asc)
    .limit(50)
    .build()?;

let results = repository.find_by_criteria(&criteria)?;
```

### DMSCPagination

分页支持，用于高效检索数据，支持 Python 绑定。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `page` | `u64` | 当前页码 | `1` |
| `page_size` | `u64` | 每页项目数 | `20` |
| `total_items` | `u64` | 总项目数 | `0` |
| `total_pages` | `u64` | 总页数 | `0` |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(page, page_size)` | 创建分页 | `page: u64`, `page_size: u64` | `Self` |
| `with_page(page)` | 设置当前页 | `page: u64` | `Self` |
| `with_page_size(size)` | 设置每页大小 | `size: u64` | `Self` |
| `calc_offset(&self)` | 计算偏移量 | 无 | `u64` |
| `calc_total_pages(&self, total: u64)` | 计算总页数 | `total: u64` | `Self` |
| `has_next(&self)` | 检查是否有下一页 | 无 | `bool` |
| `has_previous(&self)` | 检查是否有上一页 | 无 | `bool` |

#### Python 使用示例

```python
from dmsc.orm import DMSCPaginationPy

# 创建分页
pagination = DMSCPaginationPy(page=1, page_size=20)

# 计算查询偏移量
offset = pagination.calc_offset()
print(f"查询偏移量: {offset}")

# 更新总项目数后计算总页数
pagination.calc_total_pages(100)
print(f"总页数: {pagination.total_pages}")
print(f"有下一页: {pagination.has_next()}")
print(f"有上一页: {pagination.has_previous()}")
```

#### Rust 使用示例

```rust
use dmsc::prelude::*;

let mut pagination = DMSCPagination::new(1, 20);

// 在查询中使用
let offset = pagination.calc_offset();
let query = format!("SELECT * FROM users LIMIT {} OFFSET {}", pagination.page_size, offset);

// 获取总数后计算总页数
let total_items = 100;
pagination.calc_total_pages(total_items);

println!("第 {} 页，共 {} 页", pagination.page, pagination.total_pages);
println!("有下一页: {}", pagination.has_next());
println!("有上一页: {}", pagination.has_previous());
```

<div align="center>

## 仓储模式

</div>

### DMSCRepository

通用仓储接口，用于数据访问操作。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `find_all()` | 查询所有实体 | 无 | `DMSCResult<Vec<Entity>>` |
| `find_by_id(id)` | 按主键查询 | `id: impl ToSql` | `DMSCResult<Option<Entity>>` |
| `find_by_criteria(criteria)` | 按条件查询 | `criteria: &DMSCCriteria` | `DMSCResult<Vec<Entity>>` |
| `find_one_by_criteria(criteria)` | 查询单个结果 | `criteria: &DMSCCriteria` | `DMSCResult<Option<Entity>>` |
| `find_paginated(criteria, pagination)` | 分页查询 | `criteria: &DMSCCriteria`, `pagination: &mut DMSCPagination` | `DMSCResult<Vec<Entity>>` |
| `save(entity)` | 保存实体 | `entity: &Entity` | `DMSCResult<Entity>` |
| `save_many(entities)` | 保存多个实体 | `entities: &[Entity]` | `DMSCResult<Vec<Entity>>` |
| `update(entity)` | 更新实体 | `entity: &Entity` | `DMSCResult<Entity>` |
| `delete(id)` | 按 ID 删除 | `id: impl ToSql` | `DMSCResult<()>` |
| `delete_by_criteria(criteria)` | 按条件删除 | `criteria: &DMSCCriteria` | `DMSCResult<u64>` |
| `count()` | 统计总数 | 无 | `DMSCResult<u64>` |
| `count_by_criteria(criteria)` | 按条件统计 | `criteria: &DMSCCriteria` | `DMSCResult<u64>` |
| `exists_by_id(id)` | 按 ID 检查存在 | `id: impl ToSql` | `DMSCResult<bool>` |
| `exists_by_criteria(criteria)` | 按条件检查存在 | `criteria: &DMSCCriteria` | `DMSCResult<bool>` |
| `batch_insert(entities, batch_size)` | 批量插入实体 | `entities: &[Entity]`, `batch_size: usize` | `DMSCResult<Vec<Entity>>` |
| `upsert(entity, conflict_columns)` | 插入或更新实体 | `entity: &Entity`, `conflict_columns: &[&str]` | `DMSCResult<Entity>` |

#### 批量插入示例

```rust
use dmsc::prelude::*;

let users = vec![
    User { name: "Alice".to_string(), email: "alice@example.com".to_string() },
    User { name: "Bob".to_string(), email: "bob@example.com".to_string() },
    User { name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
];

// 批量插入（自定义批次大小）
let inserted = repository.batch_insert(&users, 100)?;
println!("插入 {} 个用户", inserted.len());
```

#### Upsert示例

```rust
use dmsc::prelude::*;

let user = User { id: Some(1), name: "Alice Updated".to_string(), email: "alice.new@example.com".to_string() };

// 冲突时按email列进行upsert
let upserted = repository.upsert(&user, &["email"])?;
println!("Upserted用户ID: {}", upserted.id);
```

#### 使用示例

```rust
use dmsc::prelude::*;

let repository = DMSCRepository::<User>::new(pool);

// 查询所有
let all_users = repository.find_all()?;

// 按 ID 查询
let user = repository.find_by_id(1)?;

// 按条件查询
let criteria = DMSCCriteria::new()
    .with_entity("users")
    .add_condition(DMSCCondition::eq("active", true))
    .limit(10)
    .build()?;

let active_users = repository.find_by_criteria(&criteria)?;

// 分页查询
let mut pagination = DMSCPagination::new(1, 20);
let paginated_users = repository.find_paginated(&criteria, &mut pagination)?;

// 统计
let count = repository.count()?;
let active_count = repository.count_by_criteria(&criteria)?;

// 检查存在
let exists = repository.exists_by_id(1)?;

// 保存
let new_user = User { name: "Alice".to_string(), email: "alice@example.com".to_string() };
let saved_user = repository.save(&new_user)?;

// 更新
let mut user = repository.find_by_id(1)?;
user.name = "Alice Smith";
let updated_user = repository.update(&user)?;

// 删除
repository.delete(1)?;
```

<div align="center">

## 实体映射

</div>

### DMSCEntity

实体特征，用于将 Rust 结构体映射到数据库表。

#### 必需方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `table_name(&self)` | 获取表名 | 无 | `&str` |
| `primary_key(&self)` | 获取主键字段 | 无 | `&str` |
| `columns(&self)` | 获取所有列 | 无 | `Vec<&str>` |
| `get_id(&self)` | 获取 ID 值 | 无 | `Option<DMSCSqlValue>` |
| `set_id(&mut self, id)` | 设置 ID 值 | `id: DMSCSqlValue` | `()` |

#### 可派生宏

```rust
use dmsc::prelude::*;

#[derive(DMSCEntity)]
#[entity(table_name = "users")]
struct User {
    #[dmsc(id)]
    id: i32,
    name: String,
    email: String,
    age: i32,
    active: bool,
    created_at: chrono::NaiveDateTime,
}

impl User {
    // 可以在此处添加其他方法
}
```

<div align="center

## 高级功能

</div>

### 预加载

```rust
use dmsc::prelude::*;

// 预加载关联实体
let users = repository
    .find_all()?
    .into_iter()
    .map(|user| {
        let posts = post_repository.find_by_criteria(
            &DMSCCriteria::new()
                .with_entity("posts")
                .add_condition(DMSCCondition::eq("user_id", user.id))
                .build()?
        )?;
        Ok((user, posts))
    })
    .collect::<DMSCResult<Vec<_>>>()?;
```

### 软删除

```rust
use dmsc::prelude::*;

#[derive(DMSCEntity)]
#[entity(table_name = "users", soft_delete = "deleted_at")]
struct User {
    #[dmsc(id)]
    id: i32,
    name: String,
    deleted_at: Option<chrono::NaiveDateTime>,
}

// 软删除自动过滤已删除的记录
let active_users = repository.find_all()?; // 只返回未删除的用户

// 包含已删除的记录
let criteria = DMSCCriteria::new()
    .with_entity("users")
    .with_deleted(true) // 包含软删除的记录
    .build()?;
let all_users = repository.find_by_criteria(&criteria)?;
```

### 事务

```rust
use dmsc::prelude::*;

let transaction = repository.transaction()?;

try {
    // 创建用户
    let user = repository.save_in_transaction(&transaction, &new_user)?;
    
    // 创建关联记录
    for post in posts {
        post_repository.save_in_transaction(&transaction, &post)?;
    }
    
    transaction.commit()?;
} catch {
    transaction.rollback()?;
    return Err(e);
}
```

<div align="center

## Python 支持

</div>

ORM 模块通过 PyO3 提供完整的 Python 绑定：

```python
from dmsc.orm import (
    DMSCCriteriaPy, 
    DMSCCondition, 
    DMSCPaginationPy,
    DMSCRepositoryPy
)

# 创建条件组
criteria = DMSCCriteriaPy.with_entity("User")
criteria.add_condition(DMSCCondition.eq("active", True))
criteria.add_condition(DMSCCondition.gte("age", 18))

# 创建分页
pagination = DMSCPaginationPy(page=1, page_size=20)

# 使用仓储
repository = DMSCRepositoryPy("User")
users = repository.find_by_criteria(criteria)
paginated_users = repository.find_paginated(criteria, pagination)

# 统计
count = repository.count()
```

<div align="center">

## 最佳实践

</div>

1. **对复杂查询使用条件组**：构建可复用的条件组用于复杂查询
2. **实现分页**：对大数据集始终使用分页以提高性能
3. **使用仓储模式**：封装数据访问逻辑以提高可维护性
4. **利用类型安全**：使用实体派生和类型安全的查询构建器
5. **合理使用预加载**：平衡 N+1 查询和过度获取
6. **实现软删除**：对重要数据使用软删除防止意外丢失
7. **使用事务**：将相关操作包装在事务中以保持数据一致性
8. **索引优化**：为常用查询列创建数据库索引

<div align="center

## 相关模块

</div>

- [README](./README.md)：模块概览，提供 API 参考文档总览和快速导航
- [auth](./auth.md)：认证模块，处理用户认证和授权
- [cache](./cache.md)：缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md)：配置模块，管理应用程序配置
- [core](./core.md)：核心模块，提供错误处理和服务上下文
- [database](./database.md)：数据库模块，提供数据库访问层
- [http](./http.md)：HTTP 模块，提供 HTTP 服务器和客户端功能
- [protocol](./protocol.md)：协议模块，提供通信协议支持
- [validation](./validation.md)：验证模块，提供数据验证功能
