<div align="center">

# ORM API Reference

**Version: 0.1.5**

**Last modified date: 2026-01-18**

The ORM module provides a type-safe object-relational mapping layer with query builders, criteria-based filtering, pagination support, and Python bindings.

## Module Overview

</div>

The ORM module includes the following sub-modules:

- **query_builder**: SQL query builder with fluent API
- **criteria**: Criteria-based filtering and conditions
- **pagination**: Pagination support for large datasets
- **entity**: Entity mapping and data access
- **repository**: Repository pattern implementation

<div align="center">

## Core Components

</div>

### DMSCQueryBuilder

Query builder for constructing type-safe SQL queries with a fluent API.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create a new query builder | None | `Self` |
| `select(columns)` | Set select columns | `columns: &[&str]` | `Self` |
| `from(table)` | Set from table | `table: &str` | `Self` |
| `where(condition)` | Add where condition | `condition: &str` | `Self` |
| `where_eq(column, value)` | Add equality condition | `column: &str`, `value: impl ToSql` | `Self` |
| `where_ne(column, value)` | Add inequality condition | `column: &str`, `value: impl ToSql` | `Self` |
| `where_gt(column, value)` | Add greater than condition | `column: &str`, `value: impl ToSql` | `Self` |
| `where_gte(column, value)` | Add greater than or equal | `column: &str`, `value: impl ToSql` | `Self` |
| `where_lt(column, value)` | Add less than condition | `column: &str`, `value: impl ToSql` | `Self` |
| `where_lte(column, value)` | Add less than or equal | `column: &str`, `value: impl ToSql` | `Self` |
| `where_like(column, pattern)` | Add LIKE condition | `column: &str`, `pattern: &str` | `Self` |
| `where_in(column, values)` | Add IN condition | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `where_not_in(column, values)` | Add NOT IN condition | `column: &str`, `values: &[impl ToSql]` | `Self` |
| `where_between(column, start, end)` | Add BETWEEN condition | `column: &str`, `start: impl ToSql`, `end: impl ToSql` | `Self` |
| `where_is_null(column)` | Add IS NULL condition | `column: &str` | `Self` |
| `where_is_not_null(column)` | Add IS NOT NULL condition | `column: &str` | `Self` |
| `and_where(condition)` | Add AND condition | `condition: &str` | `Self` |
| `or_where(condition)` | Add OR condition | `condition: &str` | `Self` |
| `join(table, on)` | Add INNER JOIN | `table: &str`, `on: &str` | `Self` |
| `left_join(table, on)` | Add LEFT JOIN | `table: &str`, `on: &str` | `Self` |
| `right_join(table, on)` | Add RIGHT JOIN | `table: &str`, `on: &str` | `Self` |
| `order_by(column, direction)` | Add ORDER BY | `column: &str`, `direction: OrderDirection` | `Self` |
| `group_by(columns)` | Add GROUP BY | `columns: &[&str]` | `Self` |
| `having(condition)` | Add HAVING condition | `condition: &str` | `Self` |
| `limit(count)` | Set LIMIT | `count: i64` | `Self` |
| `offset(count)` | Set OFFSET | `count: i64` | `Self` |
| `build()` | Build the query | None | `DMSCResult<String>` |

#### Usage Example

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

println!("Generated query: {}", query);
// SELECT id, name, email, created_at FROM users WHERE active = true AND age >= 18 AND email LIKE '%@example.com' ORDER BY created_at DESC LIMIT 10 OFFSET 0
```

### DMSCCriteria

Criteria-based filtering for complex query conditions with Python bindings.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new()` | Create empty criteria | None | `Self` |
| `with_entity(entity)` | Set entity type | `entity: &str` | `Self` |
| `add_condition(condition)` | Add a condition | `condition: DMSCCondition` | `Self` |
| `and_condition(condition)` | Add AND condition | `condition: DMSCCondition` | `Self` |
| `or_condition(condition)` | Add OR condition | `condition: DMSCCondition` | `Self` |
| `order_by(field, direction)` | Add ordering | `field: &str`, `direction: OrderDirection` | `Self` |
| `limit(count)` | Set limit | `count: u64` | `Self` |
| `offset(count)` | Set offset | `count: u64` | `Self` |
| `build()` | Build the criteria | None | `DMSCResult<DMSCCriteriaBuilder>` |

#### Condition Types

| Condition | Description |
|:--------|:-------------|
| `Eq(field, value)` | Equals condition |
| `Ne(field, value)` | Not equals condition |
| `Gt(field, value)` | Greater than |
| `Gte(field, value)` | Greater than or equal |
| `Lt(field, value)` | Less than |
| `Lte(field, value)` | Less than or equal |
| `Like(field, pattern)` | LIKE pattern match |
| `ILike(field, pattern)` | Case-insensitive LIKE |
| `In(field, values)` | IN list match |
| `NotIn(field, values)` | NOT IN list match |
| `Between(field, start, end)` | Between two values |
| `IsNull(field)` | IS NULL check |
| `IsNotNull(field)` | IS NOT NULL check |

#### Python Usage Example

```python
from dmsc.orm import DMSCCriteriaPy, DMSCCondition

# Create criteria with conditions
criteria = DMSCCriteriaPy.with_entity("User")

# Add conditions
criteria.add_condition(DMSCCondition.eq("active", True))
criteria.add_condition(DMSCCondition.gte("age", 18))
criteria.add_condition(DMSCCondition.like("email", "%@company.com"))

# Build and execute
results = user_repository.find(criteria)
```

#### Rust Usage Example

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

Pagination support for efficient data retrieval with Python bindings.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `page` | `u64` | Current page number | `1` |
| `page_size` | `u64` | Items per page | `20` |
| `total_items` | `u64` | Total item count | `0` |
| `total_pages` | `u64` | Total page count | `0` |

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(page, page_size)` | Create pagination | `page: u64`, `page_size: u64` | `Self` |
| `with_page(page)` | Set current page | `page: u64` | `Self` |
| `with_page_size(size)` | Set page size | `size: u64` | `Self` |
| `calc_offset(&self)` | Calculate offset | None | `u64` |
| `calc_total_pages(&self, total: u64)` | Calculate total pages | `total: u64` | `Self` |
| `has_next(&self)` | Check if next page exists | None | `bool` |
| `has_previous(&self)` | Check if previous page exists | None | `bool` |

#### Python Usage Example

```python
from dmsc.orm import DMSCPaginationPy

# Create pagination
pagination = DMSCPaginationPy(page=1, page_size=20)

# Calculate offset for query
offset = pagination.calc_offset()
print(f"Query offset: {offset}")

# Update with total items
pagination.calc_total_pages(100)
print(f"Total pages: {pagination.total_pages}")
print(f"Has next: {pagination.has_next()}")
print(f"Has previous: {pagination.has_previous()}")
```

#### Rust Usage Example

```rust
use dmsc::prelude::*;

let mut pagination = DMSCPagination::new(1, 20);

// Use in query
let offset = pagination.calc_offset();
let query = format!("SELECT * FROM users LIMIT {} OFFSET {}", pagination.page_size, offset);

// Calculate total pages after getting count
let total_items = 100;
pagination.calc_total_pages(total_items);

println!("Page {} of {}", pagination.page, pagination.total_pages);
println!("Has next: {}", pagination.has_next());
println!("Has previous: {}", pagination.has_previous());
```

<div align="center">

## Repository Pattern

</div>

### DMSCRepository

Generic repository interface for data access operations.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `find_all()` | Find all entities | None | `DMSCResult<Vec<Entity>>` |
| `find_by_id(id)` | Find by primary key | `id: impl ToSql` | `DMSCResult<Option<Entity>>` |
| `find_by_criteria(criteria)` | Find by criteria | `criteria: &DMSCCriteria` | `DMSCResult<Vec<Entity>>` |
| `find_one_by_criteria(criteria)` | Find single result | `criteria: &DMSCCriteria` | `DMSCResult<Option<Entity>>` |
| `find_paginated(criteria, pagination)` | Find with pagination | `criteria: &DMSCCriteria`, `pagination: &mut DMSCPagination` | `DMSCResult<Vec<Entity>>` |
| `save(entity)` | Save entity | `entity: &Entity` | `DMSCResult<Entity>` |
| `save_many(entities)` | Save multiple entities | `entities: &[Entity]` | `DMSCResult<Vec<Entity>>` |
| `update(entity)` | Update entity | `entity: &Entity` | `DMSCResult<Entity>` |
| `delete(id)` | Delete by ID | `id: impl ToSql` | `DMSCResult<()>` |
| `delete_by_criteria(criteria)` | Delete by criteria | `criteria: &DMSCCriteria` | `DMSCResult<u64>` |
| `count()` | Count all entities | None | `DMSCResult<u64>` |
| `count_by_criteria(criteria)` | Count by criteria | `criteria: &DMSCCriteria` | `DMSCResult<u64>` |
| `exists_by_id(id)` | Check existence by ID | `id: impl ToSql` | `DMSCResult<bool>` |
| `exists_by_criteria(criteria)` | Check existence by criteria | `criteria: &DMSCCriteria` | `DMSCResult<bool>` |
| `batch_insert(entities, batch_size)` | Batch insert entities | `entities: &[Entity]`, `batch_size: usize` | `DMSCResult<Vec<Entity>>` |
| `upsert(entity, conflict_columns)` | Insert or update entity | `entity: &Entity`, `conflict_columns: &[&str]` | `DMSCResult<Entity>` |

#### Batch Insert Example

```rust
use dmsc::prelude::*;

let users = vec![
    User { name: "Alice".to_string(), email: "alice@example.com".to_string() },
    User { name: "Bob".to_string(), email: "bob@example.com".to_string() },
    User { name: "Charlie".to_string(), email: "charlie@example.com".to_string() },
];

// Batch insert with custom batch size
let inserted = repository.batch_insert(&users, 100)?;
println!("Inserted {} users", inserted.len());
```

#### Upsert Example

```rust
use dmsc::prelude::*;

let user = User { id: Some(1), name: "Alice Updated".to_string(), email: "alice.new@example.com".to_string() };

// Upsert on conflict with email column
let upserted = repository.upsert(&user, &["email"])?;
println!("Upserted user with ID: {}", upserted.id);
```

#### Usage Example

```rust
use dmsc::prelude::*;

let repository = DMSCRepository::<User>::new(pool);

// Find all
let all_users = repository.find_all()?;

// Find by ID
let user = repository.find_by_id(1)?;

// Find with criteria
let criteria = DMSCCriteria::new()
    .with_entity("users")
    .add_condition(DMSCCondition::eq("active", true))
    .limit(10)
    .build()?;

let active_users = repository.find_by_criteria(&criteria)?;

// Find paginated
let mut pagination = DMSCPagination::new(1, 20);
let paginated_users = repository.find_paginated(&criteria, &mut pagination)?;

// Count
let count = repository.count()?;
let active_count = repository.count_by_criteria(&criteria)?;

// Exists
let exists = repository.exists_by_id(1)?;

// Save
let new_user = User { name: "Alice".to_string(), email: "alice@example.com".to_string() };
let saved_user = repository.save(&new_user)?;

// Update
let mut user = repository.find_by_id(1)?;
user.name = "Alice Smith";
let updated_user = repository.update(&user)?;

// Delete
repository.delete(1)?;
```

<div align="center>

## Entity Mapping

</div>

### DMSCEntity

Entity trait for mapping Rust structs to database tables.

#### Required Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `table_name(&self)` | Get table name | None | `&str` |
| `primary_key(&self)` | Get primary key field | None | `&str` |
| `columns(&self)` | Get all columns | None | `Vec<&str>` |
| `get_id(&self)` | Get ID value | None | `Option<DMSCSqlValue>` |
| `set_id(&mut self, id)` | Set ID value | `id: DMSCSqlValue` | `()` |

#### Derivable Macro

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
    created_at:chrono::NaiveDateTime,
}

impl User {
    // Additional methods can be added here
}
```

<div align="center>

## Advanced Features

</div>

### Eager Loading

```rust
use dmsc::prelude*;

// Eager load related entities
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

### Soft Delete

```rust
use dmsc::prelude*;

#[derive(DMSCEntity)]
#[entity(table_name = "users", soft_delete = "deleted_at")]
struct User {
    #[dmsc(id)]
    id: i32,
    name: String,
    deleted_at: Option<chrono::NaiveDateTime>,
}

// Soft delete automatically filters out deleted records
let active_users = repository.find_all()?; // Only returns non-deleted users

// To include deleted records
let criteria = DMSCCriteria::new()
    .with_entity("users")
    .with_deleted(true) // Include soft-deleted records
    .build()?;
let all_users = repository.find_by_criteria(&criteria)?;
```

### Transactions

```rust
use dmsc::prelude*;

let transaction = repository.transaction()?;

try {
    // Create user
    let user = repository.save_in_transaction(&transaction, &new_user)?;
    
    // Create related records
    for post in posts {
        post_repository.save_in_transaction(&transaction, &post)?;
    }
    
    transaction.commit()?;
} catch {
    transaction.rollback()?;
    return Err(e);
}
```

<div align="center>

## Python Support

</div>

The ORM module provides full Python bindings through PyO3:

```python
from dmsc.orm import (
    DMSCCriteriaPy, 
    DMSCCondition, 
    DMSCPaginationPy,
    DMSCRepositoryPy
)

# Create criteria
criteria = DMSCCriteriaPy.with_entity("User")
criteria.add_condition(DMSCCondition.eq("active", True))
criteria.add_condition(DMSCCondition.gte("age", 18))

# Create pagination
pagination = DMSCPaginationPy(page=1, page_size=20)

# Use repository
repository = DMSCRepositoryPy("User")
users = repository.find_by_criteria(criteria)
paginated_users = repository.find_paginated(criteria, pagination)

# Count
count = repository.count()
```

<div align="center>

## Best Practices

</div>

1. **Use criteria for complex queries**: Build reusable criteria objects for complex conditions
2. **Implement pagination**: Always use pagination for large datasets to improve performance
3. **Use repository pattern**: Encapsulate data access logic for better maintainability
4. **Leverage type safety**: Use entity derives and type-safe query builders
5. **Use eager loading wisely**: Balance between N+1 queries and over-fetching
6. **Implement soft delete**: Use soft delete for important data to prevent accidental loss
7. **Use transactions**: Wrap related operations in transactions for data consistency
8. **Index optimization**: Create database indexes for frequently queried columns

<div align="center>

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database access layer
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [validation](./validation.md): Validation module providing data validation functions
