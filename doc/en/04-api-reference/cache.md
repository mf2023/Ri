<div align="center">

# Cache API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

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
| `backend()` | Get current cache backend type | None | `DMSCCacheBackend` |

#### Usage Example

```rust
use dms::prelude::*;

// Access cache manager through module
let cache_manager = ctx.module::<DMSCCacheModule>().await?
    .cache_manager();
    
// Set cache
cache_manager.set("user:1", &user, Some(3600)).await?;

// Get cache
let user: Option<User> = cache_manager.get("user:1").await?;

// Check if cache exists
let exists = cache_manager.exists("user:1").await?;

// Delete cache
cache_manager.delete("user:1").await?;

// Numeric operations
let count = cache_manager.increment("counter", 1).await?;
let count = cache_manager.decrement("counter", 5).await?;
```

### DMSCCacheManager

Cache manager, responsible for specific cache operations.

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

Cache backend type enum.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Memory` | Memory cache (default) |
| `Redis` | Redis distributed cache |
| `Hybrid` | Hybrid cache (Memory + Redis) |

