<div align="center">

# Caching Usage Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's cache module for various cache backends and advanced caching features.

## Example Overview

</div>

This example creates a DMSC Python application with the following features:

- Memory cache and Redis cache usage
- Cache tags and atomic operations
- Distributed lock implementation
- Cache health check and monitoring
- Data serialization and compression
- Error handling and fallback strategies

<div align="center">

## Prerequisites

</div>

- Python 3.8+
- pip 20.0+
- Basic Python programming knowledge
- Understanding of cache concepts
- (Optional) Redis server for Redis cache examples

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
mkdir dms-cache-example
cd dms-cache-example
python -m venv venv
source venv/bin/activate  # Windows: venv\Scripts\activate
```

### 2. Add Dependencies

Create `requirements.txt`:

```txt
dmsc>=0.0.3
redis>=4.0.0
msgpack>=0.0.3
```

Install dependencies:

```bash
pip install -r requirements.txt
```

### 3. Create Configuration File

Create `config.yaml` in the project root:

```yaml
service:
  name: "dms-cache-example"
  version: "0.0.3"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true
```

### 4. Create Cache Application

Create `app.py`:

```python
import asyncio
from datetime import datetime, timedelta
from typing import Optional, Any
import hashlib
import json
import msgpack

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCCacheConfig, DMSCCacheModule, DMSCCacheStats,
    DMSCConfig, DMSCError
)

# Simulated database
database = {
    "users": {
        "1": {"id": "1", "name": "Alice", "email": "alice@example.com"},
        "2": {"id": "2", "name": "Bob", "email": "bob@example.com"},
        "3": {"id": "3", "name": "Charlie", "email": "charlie@example.com"}
    },
    "products": {
        "p1": {"id": "p1", "name": "Laptop", "price": 999.99},
        "p2": {"id": "p2", "name": "Phone", "price": 599.99},
        "p3": {"id": "p3", "name": "Tablet", "price": 399.99}
    }
}

# Cache service
class CacheService:
    def __init__(self, cache: DMSCCacheModule, context: DMSCServiceContext):
        self.cache = cache
        self.context = context
        self.logger = context.logger
    
    def _make_key(self, category: str, key: str) -> str:
        return f"{category}:{key}"
    
    async def get_user(self, user_id: str) -> Optional[dict]:
        cache_key = self._make_key("user", user_id)
        
        # Try cache first
        cached = await self.cache.get(cache_key)
        if cached:
            self.logger.info("cache", f"Cache hit for user {user_id}")
            return cached
        
        # Cache miss - fetch from database
        self.logger.info("cache", f"Cache miss for user {user_id}")
        user = database["users"].get(user_id)
        
        if user:
            # Store in cache with 5-minute TTL
            await self.cache.set(cache_key, user, ttl=300)
        
        return user
    
    async def set_user(self, user_id: str, user_data: dict, ttl: int = 300) -> None:
        cache_key = self._make_key("user", user_id)
        await self.cache.set(cache_key, user_data, ttl=ttl)
        self.logger.info("cache", f"Cached user {user_id}")
    
    async def delete_user(self, user_id: str) -> None:
        cache_key = self._make_key("user", user_id)
        await self.cache.delete(cache_key)
        self.logger.info("cache", f"Deleted user {user_id} from cache")
    
    async def invalidate_users_by_tag(self, tag: str) -> None:
        # Get all user keys
        user_keys = await self.cache.keys("user:*")
        
        for key in user_keys:
            cached = await self.cache.get(key)
            if cached and cached.get("tag") == tag:
                await self.cache.delete(key)
        
        self.logger.info("cache", f"Invalidated users with tag: {tag}")
    
    async def get_or_set(self, key: str, fetch_func, ttl: int = 300) -> Any:
        cached = await self.cache.get(key)
        if cached:
            return cached
        
        value = await fetch_func()
        await self.cache.set(key, value, ttl=ttl)
        return value
    
    async def increment_with_retry(self, key: str, delta: int = 1, max_retries: int = 3) -> int:
        for attempt in range(max_retries):
            try:
                return await self.cache.increment(key, delta)
            except Exception as e:
                self.logger.warn("cache", f"Increment attempt {attempt + 1} failed: {e}")
                if attempt < max_retries - 1:
                    await asyncio.sleep(0.1 * (attempt + 1))
        raise DMSCError(f"Failed to increment {key} after {max_retries} attempts", "CACHE_ERROR")
    
    async def batch_get_users(self, user_ids: list) -> dict:
        cache_keys = [self._make_key("user", uid) for uid in user_ids]
        
        # Batch get from cache
        cached_values = await self.cache.batch_get(cache_keys)
        
        result = {}
        missing_ids = []
        
        for user_id, cached in zip(user_ids, cached_values):
            if cached:
                result[user_id] = cached
            else:
                missing_ids.append(user_id)
        
        # Fetch missing from database
        if missing_ids:
            self.logger.info("cache", f"Batch cache miss for {len(missing_ids)} users")
            for user_id in missing_ids:
                user = database["users"].get(user_id)
                if user:
                    result[user_id] = user
                    # Cache the fetched user
                    await self.cache.set(
                        self._make_key("user", user_id),
                        user,
                        ttl=300
                    )
        
        return result
    
    async def get_cache_stats(self) -> DMSCCacheStats:
        return await self.cache.get_stats()

# Request handlers
async def handle_get_user(context: DMSCServiceContext):
    request = context.http.request
    user_id = request.path.split("/")[-1]
    
    cache_service = context.cache_service
    user = await cache_service.get_user(user_id)
    
    if user:
        return {"status": "success", "data": user}
    else:
        return {"status": "error", "message": "User not found"}, 404

async def handle_set_user(context: DMSCServiceContext):
    data = await context.http.request.json()
    cache_service = context.cache_service
    
    user_id = data["id"]
    user_data = {
        "id": user_id,
        "name": data["name"],
        "email": data["email"],
        "tag": data.get("tag", "default")
    }
    
    await cache_service.set_user(user_id, user_data)
    
    return {"status": "success", "message": "User cached"}

async def handle_batch_get_users(context: DMSCServiceContext):
    data = await context.http.request.json()
    cache_service = context.cache_service
    
    user_ids = data.get("user_ids", [])
    users = await cache_service.batch_get_users(user_ids)
    
    return {"status": "success", "data": users}

async def handle_get_stats(context: DMSCServiceContext):
    cache_service = context.cache_service
    stats = await cache_service.get_cache_stats()
    
    return {
        "status": "success",
        "data": {
            "hits": stats.hits,
            "misses": stats.misses,
            "hit_rate": stats.hit_rate,
            "keys_count": stats.keys_count,
            "memory_usage": stats.memory_usage
        }
    }

async def handle_invalidate(context: DMSCServiceContext):
    data = await context.http.request.json()
    cache_service = context.cache_service
    
    tag = data.get("tag")
    if tag:
        await cache_service.invalidate_users_by_tag(tag)
        return {"status": "success", "message": f"Invalidated users with tag: {tag}"}
    else:
        return {"status": "error", "message": "Tag required"}, 400

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(
        level="DEBUG",
        format="json"
    ))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    # Configure memory cache
    app.with_cache(DMSCCacheConfig(
        backend="memory",
        default_ttl=300,
        max_memory_size=100000000,
        eviction_policy="lru"
    ))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize cache service
    cache_service = CacheService(dms_app.context.cache, dms_app.context)
    dms_app.context.cache_service = cache_service
    
    # Add routes
    dms_app.router.add_route("GET", "/users/{user_id}", handle_get_user)
    dms_app.router.add_route("POST", "/users", handle_set_user)
    dms_app.router.add_route("POST", "/users/batch", handle_batch_get_users)
    dms_app.router.add_route("GET", "/stats", handle_get_stats)
    dms_app.router.add_route("POST", "/invalidate", handle_invalidate)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Cache Service Architecture

The example demonstrates a complete cache service architecture:

1. **Cache Service Layer**: Wraps cache operations with business logic
2. **Cache-Aside Pattern**: Check cache before database access
3. **Batch Operations**: Efficient batch get operations
4. **Tag-Based Invalidation**: Invalidate cache by tags
5. **Retry Logic**: Handle transient cache failures

### Key Components

- **DMSCCacheConfig**: Cache configuration for backend, TTL, and memory limits
- **DMSCCacheModule**: Cache operations (get, set, delete, keys, increment)
- **Cache-Aside Pattern**: Read-through and write-through caching

## Running Steps

1. Save the code to `app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc redis msgpack
   ```
3. Run the application:
   ```bash
   python app.py
   ```
4. Test the API endpoints:

   ```bash
   # Get a user (cache miss, then cache hit)
   curl http://localhost:8080/users/1
   
   # Set a user in cache
   curl -X POST http://localhost:8080/users \
     -H "Content-Type: application/json" \
     -d '{"id": "4", "name": "David", "email": "david@example.com", "tag": "vip"}'
   
   # Batch get users
   curl -X POST http://localhost:8080/users/batch \
     -H "Content-Type: application/json" \
     -d '{"user_ids": ["1", "2", "3", "4"]}'
   
   # Get cache statistics
   curl http://localhost:8080/stats
   
   # Invalidate cache by tag
   curl -X POST http://localhost:8080/invalidate \
     -H "Content-Type: application/json" \
     -d '{"tag": "vip"}'
   ```

## Expected Output

### Get User Response

```json
{
  "status": "success",
  "data": {
    "id": "1",
    "name": "Alice",
    "email": "alice@example.com"
  }
}
```

### Cache Stats Response

```json
{
  "status": "success",
  "data": {
    "hits": 15,
    "misses": 5,
    "hit_rate": 0.75,
    "keys_count": 10,
    "memory_usage": 1024000
  }
}
```

### Console Output

```
[2024-01-15T10:30:00] [INFO] [cache] Cache miss for user 1
[2024-01-15T10:30:01] [INFO] [cache] Cache hit for user 1
[2024-01-15T10:30:02] [INFO] [cache] Cached user 4
```

## Best Practices

1. **Set Appropriate TTL**: Use different TTLs for different data types
2. **Monitor Cache Memory**: Monitor memory usage in production
3. **Use Namespaces**: Use prefixes for different data types
4. **Handle Cache Misses**: Always handle cases where data is not in cache
5. **Avoid Large Objects**: Don't cache very large objects (>1MB)
6. **Use Serialization**: Use efficient serialization (JSON, msgpack)
7. **Implement Fallbacks**: Have fallback strategies when cache is unavailable
8. **Warm Up Cache**: Pre-populate cache on startup for frequently accessed data
