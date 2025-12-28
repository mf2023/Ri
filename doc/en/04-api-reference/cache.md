<div align="center">

# Cache API Reference

**Version: 1.0.0**

**Last modified date: 2025-12-12**

The cache module provides multi-backend cache abstraction, supporting memory, Redis, hybrid, and other cache backends.

## Module Overview

</div>

The cache module includes the following sub-modules:

- **core**: Cache core interfaces and type definitions
- **manager**: Cache manager, unified management of multiple cache backends
- **backends**: Various cache backend implementations
- **config**: Cache configuration

<div align="center">

## Core Components

</div>

### DMSCCacheModule

The main interface for the cache module, providing unified access to cache services.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `get(key)` | Get cache value | `key: &str` | `DMSCResult<Option<String>>` |
| `set(key, value, ttl)` | Set cache value | `key: &str`, `value: impl Serialize`, `ttl: Option<u64>` | `DMSCResult<()>` |
| `delete(key)` | Delete cache | `key: &str` | `DMSCResult<()>` |
| `exists(key)` | Check if cache exists | `key: &str` | `DMSCResult<bool>` |
| `clear()` | Clear all cache | None | `DMSCResult<()>` |
| `keys(pattern)` | Get matching keys | `pattern: &str` | `DMSCResult<Vec<String>>` |
| `ttl(key)` | Get cache expiration time | `key: &str` | `DMSCResult<Option<u64>>` |
| `expire(key, ttl)` | Set cache expiration time | `key: &str`, `ttl: u64` | `DMSCResult<()>` |
| `increment(key, delta)` | Numeric increment | `key: &str`, `delta: i64` | `DMSCResult<i64>` |
| `decrement(key, delta)` | Numeric decrement | `key: &str`, `delta: i64` | `DMSCResult<i64>` |

#### Usage Example

```rust
use dms::prelude::*;

// Set cache
ctx.cache().set("user:1", &user, Some(3600)).await?;

// Get cache
let user: Option<User> = ctx.cache().get("user:1").await?;

// Check if cache exists
let exists = ctx.cache().exists("user:1").await?;

// Delete cache
ctx.cache().delete("user:1").await?;

// Numeric operations
let count = ctx.cache().increment("counter", 1).await?;
let count = ctx.cache().decrement("counter", 5).await?;
```

### DMSCCacheConfig

Cache module configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:--------|:-------------|:--------|
| `backend` | `DMSCCacheBackend` | Cache backend type | `Memory` |
| `default_ttl` | `u64` | Default expiration time (seconds) | 3600 |
| `max_memory_size` | `usize` | Maximum memory size (bytes) | 100MB |
| `redis_url` | `Option<String>` | Redis connection URL | `None` |
| `redis_pool_size` | `u32` | Redis connection pool size | 10 |
| `cleanup_interval` | `u64` | Cleanup interval (seconds) | 300 |
| `compression` | `bool` | Whether to enable compression | false |

#### Usage Example

```rust
let cache_config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis,
    default_ttl: 7200,
    max_memory_size: 200 * 1024 * 1024, // 200MB
    redis_url: Some("redis://localhost:6379".to_string()),
    redis_pool_size: 20,
    cleanup_interval: 600,
    compression: true,
};
```

### DMSCCacheBackend

Cache backend enum type.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Memory` | Memory cache |
| `Redis` | Redis cache |
| `Hybrid` | Hybrid cache (Memory + Redis) |
| `Custom` | Custom cache backend |

## Cache Backends

### Memory Cache

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Memory,
    max_memory_size: 100 * 1024 * 1024, // 100MB
    ..Default::default()
};
```

### Redis Cache

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis,
    redis_url: Some("redis://localhost:6379".to_string()),
    redis_pool_size: 10,
    ..Default::default()
};
```

### Hybrid Cache

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Hybrid,
    max_memory_size: 50 * 1024 * 1024, // 50MB memory cache
    redis_url: Some("redis://localhost:6379".to_string()),
    ..Default::default()
};
```

<div align="center">

## Advanced Features

</div>

### Batch Operations

```rust
// Batch get
let keys = vec!["key1", "key2", "key3"];
let values = ctx.cache().get_multi(&keys).await?;

// Batch set
let items = vec![
    ("key1", "value1"),
    ("key2", "value2"),
    ("key3", "value3"),
];
ctx.cache().set_multi(&items, Some(3600)).await?;

// Batch delete
ctx.cache().delete_multi(&keys).await?;
```

### Atomic Operations

```rust
// Atomic increment and return new value
let new_value = ctx.cache().increment_and_get("counter", 1).await?;

// Atomic decrement and return new value
let new_value = ctx.cache().decrement_and_get("counter", 5).await?;

// Compare and set
let success = ctx.cache().compare_and_set("key", "old_value", "new_value").await?;
```

### Distributed Lock

```rust
// Acquire distributed lock
let lock = ctx.cache().acquire_lock("resource_lock", 30).await?;

// Execute business logic
// ...

// Release lock
ctx.cache().release_lock("resource_lock", &lock).await?;
```
<div align="center">

## Cache Strategies

</div>  

### TTL Strategy

```rust
// Set relative expiration time
ctx.cache().set("key", &value, Some(3600)).await?; // 1 hour

// Set absolute expiration time
ctx.cache().set_at("key", &value, timestamp).await?;

// Get remaining expiration time
let ttl = ctx.cache().ttl("key").await?;

// Extend expiration time
ctx.cache().expire("key", 7200).await?;
```

### Cache Penetration Protection

```rust
// Use bloom filter to prevent cache penetration
ctx.cache().set_with_bloom_filter("key", &value, Some(3600)).await?;

// Check bloom filter
let might_exist = ctx.cache().bloom_filter_might_contain("key").await?;
```

### Cache Avalanche Protection

```rust
// Set random expiration time to avoid simultaneous expiration
ctx.cache().set_with_jitter("key", &value, 3600, 300).await?; // ±5 minutes random
```

<div align="center">

## Serialization Support

</div>  

### JSON Serialization

```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// Store struct
let user = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
ctx.cache().set("user:1", &user, Some(3600)).await?;

// Get struct
let user: Option<User> = ctx.cache().get("user:1").await?;
```

### Binary Serialization

```rust
// Store in binary format
ctx.cache().set_binary("binary_key", &binary_data, Some(3600)).await?;

// Get binary data
let data = ctx.cache().get_binary("binary_key").await?;
```
<div align="center">

## Performance Optimization

</div>      

### Connection Pool

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis,
    redis_pool_size: 50, // Increase connection pool
    ..Default::default()
};
```

### Compression

```rust
let config = DMSCCacheConfig {
    compression: true, // Enable compression
    ..Default::default()
};
```

### Batching

```rust
// Use pipeline for batch operations
let pipeline = ctx.cache().pipeline();
pipeline.set("key1", "value1", Some(3600));
pipeline.set("key2", "value2", Some(3600));
pipeline.execute().await?;
```
<div align="center">

## Monitoring and Statistics

</div>

### Cache Statistics

```rust
// Get cache statistics
let stats = ctx.cache().get_stats().await?;
println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
println!("Hit rate: {:.2}%", stats.hit_rate * 100.0);
```

### Keyspace Notifications

```rust
// Listen to key expiration events
ctx.cache().subscribe_key_events("expired", |event| {
    println!("Key expired: {}", event.key);
}).await?;
```
<div align="center">

## Error Handling

</div>

### Cache Error Codes

| Error Code | Description |
|:--------|:-------------|
| `CACHE_CONNECTION_FAILED` | Cache connection failed |
| `CACHE_OPERATION_FAILED` | Cache operation failed |
| `CACHE_SERIALIZATION_ERROR` | Cache serialization error |
| `CACHE_KEY_NOT_FOUND` | Cache key not found |
| `CACHE_LOCK_ACQUISITION_FAILED` | Distributed lock acquisition failed |

### Error Handling Example

```rust
match ctx.cache().get::<User>("user:1").await {
    Ok(Some(user)) => {
        // Cache hit
        println!("User from cache: {:?}", user);
    }
    Ok(None) => {
        // Cache miss
        let user = load_user_from_database(1).await?;
        ctx.cache().set("user:1", &user, Some(3600)).await?;
    }
    Err(DMSCError { code, .. }) if code == "CACHE_CONNECTION_FAILED" => {
        // Cache connection failed, fallback to database
        let user = load_user_from_database(1).await?;
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

1. **Reasonable TTL**: Set appropriate expiration times based on data update frequency
2. **Use Batch Operations**: Reduce network round trips and improve performance
3. **Implement Cache Warm-up**: Load hot data at application startup
4. **Handle Cache Penetration**: Use bloom filters or null value caching
5. **Monitor Cache Hit Rate**: Adjust cache strategies in time
6. **Use Connection Pools**: Avoid frequent connection creation
7. **Enable Compression**: Enable compression for large value data to reduce memory usage

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, providing JWT, OAuth2 and RBAC authentication and authorization functionality
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [config](./config.md): Configuration module, managing authentication configuration and key settings
- [database](./database.md): Database module, providing user data persistence and query functionality
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [observability](./observability.md): Observability module, monitoring authentication performance and security events
- [security](./security.md): Security module, providing encryption, hashing, and verification functionality
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
