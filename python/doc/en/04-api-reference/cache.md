<div align="center">

# Cache API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The cache module provides high-performance caching functionality, supporting multiple cache backends and caching strategies.

## Module Overview

</div>

The cache module contains the following core components:

- **DMSCCacheModule**: Cache module main interface
- **DMSCCacheConfig**: Cache configuration
- **DMSCCacheManager**: Cache manager

<div align="center">

## Core Components

</div>

### DMSCCacheModule

Cache module main interface, providing unified cache service access.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `get(key)` | Get cache value | `key: str` | `Optional[Any]` |
| `set(key, value, ttl)` | Set cache value | `key: str`, `value: Any`, `ttl: int` | `None` |
| `delete(key)` | Delete cache | `key: str` | `None` |
| `exists(key)` | Check if cache exists | `key: str` | `bool` |
| `clear()` | Clear cache | `None` | `None` |
| `keys(pattern)` | Get matching keys | `pattern: str` | `List[str]` |
| `ttl(key)` | Get remaining TTL | `key: str` | `Optional[int]` |
| `expire(key, ttl)` | Set expiration time | `key: str`, `ttl: int` | `None` |
| `increment(key, delta)` | Increment value | `key: str`, `delta: int` | `int` |
| `decrement(key, delta)` | Decrement value | `key: str`, `delta: int` | `int` |

#### Usage Example

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

# Initialize cache module
config = DMSCCacheConfig(
    backend="memory",
    default_ttl=3600,
    max_memory_size=100000000
)
cache_module = DMSCCacheModule(config)

# Set cache value
cache_module.set("user:123", {"name": "John", "email": "john@example.com"})

# Get cache value
user_data = cache_module.get("user:123")
print(f"User data: {user_data}")

# Check if cache exists
exists = cache_module.exists("user:123")
print(f"Cache exists: {exists}")

# Delete cache
cache_module.delete("user:123")

# Batch operations
cache_module.set("user:1", {"name": "Alice"})
cache_module.set("user:2", {"name": "Bob"})

users = cache_module.keys("user:*")
print(f"All users: {users}")

# Counter operations
```

### DMSCCacheConfig

Cache module configuration.

```python
from dmsc import DMSCCacheConfig

# Memory cache configuration
memory_config = DMSCCacheConfig(
    backend="memory",
    default_ttl=3600,
    max_memory_size=100000000,
    eviction_policy="lru"
)

# Redis cache configuration
redis_config = DMSCCacheConfig(
    backend="redis",
    default_ttl=3600,
    host="localhost",
    port=6379,
    password=None,
    db=0,
    max_connections=50,
    connection_timeout=5,
    command_timeout=3
)

# Hybrid cache configuration
hybrid_config = DMSCCacheConfig(
    backend="hybrid",
    default_ttl=3600,
    local_cache_size=10000,
    local_cache_ttl=300,
    remote_backend="redis"
)
```

## Cache Backends

### Memory Cache

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

# Simple in-memory cache
cache = DMSCCacheModule(
    DMSCCacheConfig(
        backend="memory",
        default_ttl=300,
        max_memory_size=10000000
    )
)

# Store data
cache.set("session:123", {"user_id": 1, "role": "admin"}, ttl=1800)

# Retrieve data
session = cache.get("session:123")
if session:
    print(f"Session found: {session}")
else:
    print("Session expired or not found")
```

### Redis Cache

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

# Redis cache with connection pool
redis_cache = DMSCCacheModule(
    DMSCCacheConfig(
        backend="redis",
        host="localhost",
        port=6379,
        password="your-password",
        db=0,
        default_ttl=3600,
        max_connections=100
    )
)

# Store JSON data
import json
user_data = {"id": 1, "name": "John", "email": "john@example.com"}
redis_cache.set("user:1", json.dumps(user_data), ttl=3600)

# Retrieve and parse
data = redis_cache.get("user:1")
if data:
    user = json.loads(data)
    print(f"User: {user['name']}")
```

### Hybrid Cache

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

# L1 (memory) + L2 (redis) cache
hybrid_cache = DMSCCacheModule(
    DMSCCacheConfig(
        backend="hybrid",
        local_cache_size=10000,
        local_cache_ttl=60,
        remote_backend="redis",
        remote_ttl=3600
    )
)

# Fast local cache with persistent backup
hybrid_cache.set("config:app", {"setting": "value"}, ttl=1800)
```

## Advanced Operations

### Pattern Matching

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

cache = DMSCCacheModule(DMSCCacheConfig(backend="memory"))

# Store many keys
for i in range(100):
    cache.set(f"user:{i}", {"id": i, "name": f"User {i}"})

# Find all user keys
user_keys = cache.keys("user:*")
print(f"Found {len(user_keys)} user keys")

# Delete all matching keys
for key in cache.keys("user:5*"):
    cache.delete(key)
```

### TTL Operations

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

cache = DMSCCacheModule(DMSCCacheConfig(backend="memory"))

# Set with specific TTL
cache.set("temp:data", "value", ttl=60)

# Check remaining TTL
remaining = cache.ttl("temp:data")
print(f"TTL remaining: {remaining} seconds")

# Extend TTL
cache.expire("temp:data", ttl=300)

# Delete expired keys automatically
# (Redis handles this, memory may need cleanup)
```

### Counter Operations

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

cache = DMSCCacheModule(DMSCCacheConfig(backend="memory"))

# Create counter
cache.set("visits:page1", 0)

# Increment
for _ in range(10):
    count = cache.increment("visits:page1")
    print(f"Visit count: {count}")

# Decrement
count = cache.decrement("visits:page1", 2)
print(f"Count after decrement: {count}")
```

## Cache Strategies

### Cache-Aside Pattern

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig

class UserRepository:
    def __init__(self, cache, db):
        self.cache = cache
        self.db = db
    
    async def get_user(self, user_id: str) -> dict:
        # Check cache first
        cached = await self.cache.get(f"user:{user_id}")
        if cached:
            return cached
        
        # Fetch from database
        user = await self.db.query(f"SELECT * FROM users WHERE id = {user_id}")
        
        # Cache for next time
        if user:
            await self.cache.set(f"user:{user_id}", user, ttl=3600)
        
        return user
```

### Write-Through Pattern

```python
class WriteThroughCache:
    def __init__(self, cache, db):
        self.cache = cache
        self.db = db
    
    async def update_user(self, user_id: str, data: dict):
        # Write to database first
        await self.db.update(f"UPDATE users SET ... WHERE id = {user_id}")
        
        # Then update cache
        await self.cache.set(f"user:{user_id}", data)
```

### Time-Based Expiration

```python
from dmsc import DMSCCacheModule, DMSCCacheConfig
import time

cache = DMSCCacheModule(DMSCCacheConfig(backend="memory"))

# Session cache - short TTL
cache.set("session:abc", {"user": "john"}, ttl=1800)  # 30 minutes

# Configuration cache - longer TTL
cache.set("config:app", {"theme": "dark"}, ttl=86400)  # 24 hours

# Real-time data - very short TTL
cache.set("metrics:current", {"users": 123}, ttl=5)  # 5 seconds
```

## Best Practices

1. **Choose Right TTL**: Set appropriate TTL based on data freshness requirements
2. **Monitor Memory Usage**: Monitor cache memory consumption in production
3. **Use Namespaces**: Use prefixes for different data types (e.g., `user:`, `session:`)
4. **Handle Cache Misses**: Always handle cases where data is not in cache
5. **Avoid Large Objects**: Don't cache very large objects (>1MB)
6. **Use Serialization**: Use efficient serialization (JSON, msgpack) for complex objects
7. **Implement Fallbacks**: Have fallback strategies when cache is unavailable
8. **Warm Up Cache**: Pre-populate cache on application startup for frequently accessed data
