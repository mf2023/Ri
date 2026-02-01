<div align="center">

# Cache API Reference

**Version: 0.1.6**

**Last modified date: 2026-02-01**

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

**Note**: This class provides access to the cache manager. For specific cache operations, use `cache_manager()` to get the `DMSCCacheManager`.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `cache_manager()` | Get cache manager | None | `Arc<DMSCCacheManager>` |
| `backend()` | Get current cache backend type | None | `DMSCCacheBackendType` |

#### Usage Example

```rust
use dmsc::prelude::*;

// Access cache manager through module
let cache_manager = ctx.module::<DMSCCacheModule>().await?
    .cache_manager();
    
// Set cache
cache_manager.set("user:1", &user, Some(3600)).await?;

// Get cache
let user: Option<User> = cache_manager.get("user:1").await?;

// Check if cache exists
let exists = cache_manager.exists("user:1").await;

// Delete cache
cache_manager.delete("user:1").await?;

// Get or set cache value
let user = cache_manager.get_or_set("user:1", Some(3600), || async {
    fetch_user_from_db().await
}).await?;
```

### DMSCCacheManager

Cache manager, responsible for specific cache operations.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `get(key)` | Get cache value | `key: &str` | `DMSCResult<Option<T>>` |
| `set(key, value, ttl_seconds)` | Set cache value | `key: &str`, `value: &T`, `ttl_seconds: Option<u64>` | `DMSCResult<()>` |
| `delete(key)` | Delete cache | `key: &str` | `DMSCResult<bool>` |
| `exists(key)` | Check if cache exists | `key: &str` | `bool` |
| `clear()` | Clear all cache | None | `DMSCResult<()>` |
| `stats()` | Get cache statistics | None | `DMSCCacheStats` |
| `cleanup_expired()` | Cleanup expired cache | None | `DMSCResult<usize>` |
| `get_or_set(key, ttl_seconds, factory)` | Get or set cache value | `key: &str`, `ttl_seconds: Option<u64>`, `factory: F` where `F: FnOnce() -> Fut`, `Fut: Future` | `DMSCResult<T>` |

### DMSCCacheConfig

Cache module configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:--------|:-------------|:--------|
| `enabled` | `bool` | Whether caching is enabled | `true` |
| `backend_type` | `DMSCCacheBackendType` | Cache backend type | `Memory` |
| `default_ttl_secs` | `u64` | Default expiration time (seconds) | 3600 |
| `max_memory_mb` | `u64` | Maximum memory size (MB) | 512 |
| `cleanup_interval_secs` | `u64` | Cleanup interval (seconds) | 300 |
| `redis_url` | `String` | Redis connection URL | `"redis://127.0.0.1:6379"` |
| `redis_pool_size` | `usize` | Redis connection pool size | 10 |

#### Usage Example

```rust
let cache_config = DMSCCacheConfig {
    enabled: true,
    backend_type: DMSCCacheBackendType::Redis,
    default_ttl_secs: 7200,
    max_memory_mb: 512,
    cleanup_interval_secs: 600,
    redis_url: "redis://localhost:6379".to_string(),
    redis_pool_size: 20,
};
```

### DMSCCachedValue

Cached value wrapper with TTL expiration and LRU eviction support.

#### Fields

| Field | Type | Description |
|:--------|:--------|:-------------|
| `value` | `String` | The actual cached data |
| `expires_at` | `Option<u64>` | TTL-based expiration timestamp (UNIX epoch seconds), None means never expires |
| `last_accessed` | `Option<u64>` | Last access timestamp (UNIX epoch seconds), used for LRU eviction policies |

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(value, ttl)` | Create cached value | `value: String`, `ttl: Option<u64>` | `Self` |
| `touch()` | Update last access timestamp | `()` | `()` |
| `is_expired()` | Check if expired | `()` | `bool` |
| `is_stale(max_idle_secs)` | Check if stale due to long idle time | `max_idle_secs: u64` | `bool` |
| `deserialize<T>()` | Deserialize to specified type | `()` | `DMSCResult<T>` |

#### Usage Example

```rust
use dmsc::cache::DMSCCachedValue;

// Create a cache value that expires in 1 hour
let cached = DMSCCachedValue::new("user_data".to_string(), Some(3600));

// Update last access time when accessed
cached.touch();

// Check if expired
if cached.is_expired() {
    println!("Cache expired");
}

// Check if stale due to long idle time (for LRU eviction)
if cached.is_stale(300) {
    println!("Cache is stale, may be evicted by LRU policy");
}

// Deserialize
let user: User = cached.deserialize()?;
```

#### LRU Eviction Policy Support

`DMSCCachedValue` provides the following features to support LRU cache eviction:

- **touch()**: Call each time the cached value is accessed to update the last access timestamp
- **is_stale(max_idle_secs)**: Determine if the cache item has exceeded the maximum idle time

```rust
// LRU eviction example
let max_idle_seconds = 300; // 5 minutes
for (_, cached) in cache.iter() {
    if cached.is_stale(max_idle_seconds) {
        // Remove cache items that haven't been accessed for a long time
        cache.remove(key);
    }
}
```

### DMSCCacheBackendType

Cache backend type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Memory` | Memory cache (default) |
| `Redis` | Redis distributed cache |
| `Hybrid` | Hybrid cache (Memory + Redis) |

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [log](./log.md): Logging module for protocol events
- [observability](./observability.md): Observability module for protocol performance monitoring
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [queue](./queue.md): Message queue module providing message queue support
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication

