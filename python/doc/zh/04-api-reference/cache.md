<div align="center">

# Cache API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

cache模块提供高性能缓存功能，支持多种缓存后端和缓存策略。

## 模块概述

</div>

cache模块包含以下子模块：

- **memory**: 内存缓存实现
- **redis**: Redis缓存后端
- **memcached**: Memcached缓存后端
- **strategies**: 缓存策略实现
- **invalidation**: 缓存失效机制
- **distributed**: 分布式缓存支持

<div align="center">

## 核心组件

</div>

### DMSCCacheConfig

缓存配置类，用于配置缓存行为。

#### 构造函数

```python
DMSCCacheConfig(
    backend: str = "memory",
    ttl: int = 3600,
    max_size: int = 1000,
    enable_compression: bool = True,
    compression_threshold: int = 1024,
    enable_encryption: bool = False,
    encryption_key: str = "",
    enable_stats: bool = True,
    stats_interval: int = 60,
    enable_metrics: bool = True,
    enable_tracing: bool = True,
    circuit_breaker_enabled: bool = True,
    circuit_breaker_threshold: int = 5,
    circuit_breaker_timeout: int = 60,
    retry_attempts: int = 3,
    retry_delay: float = 0.1,
    connection_pool_size: int = 10,
    connection_timeout: int = 5,
    read_timeout: int = 2,
    write_timeout: int = 2
)
```

### DMSCCacheManager

缓存管理器，提供统一的缓存接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key, default=None)` | 获取缓存值 | `key: str`, `default: Any` | `Any` |
| `set(key, value, ttl=None)` | 设置缓存值 | `key: str`, `value: Any`, `ttl: int` | `bool` |
| `delete(key)` | 删除缓存 | `key: str` | `bool` |
| `exists(key)` | 检查缓存是否存在 | `key: str` | `bool` |
| `clear()` | 清空缓存 | `None` | `bool` |
| `get_many(keys)` | 批量获取 | `keys: List[str]` | `Dict[str, Any]` |
| `set_many(mapping, ttl=None)` | 批量设置 | `mapping: Dict[str, Any]`, `ttl: int` | `bool` |
| `delete_many(keys)` | 批量删除 | `keys: List[str]` | `bool` |
| `increment(key, delta=1)` | 递增 | `key: str`, `delta: int` | `int` |
| `decrement(key, delta=1)` | 递减 | `key: str`, `delta: int` | `int` |
| `expire(key, ttl)` | 设置过期时间 | `key: str`, `ttl: int` | `bool` |
| `ttl(key)` | 获取剩余过期时间 | `key: str` | `int` |
| `keys(pattern="*")` | 获取匹配的键 | `pattern: str` | `List[str]` |
| `stats()` | 获取缓存统计 | `None` | `Dict` |
| `ping()` | 检查连接 | `None` | `bool` |

#### 使用示例

```python
from dmsc import DMSCCacheManager, DMSCCacheConfig

# 初始化缓存管理器
config = DMSCCacheConfig(
    backend="redis",
    ttl=3600,
    max_size=1000,
    enable_compression=True
)

cache_manager = DMSCCacheManager(config)

# 设置缓存值
cache_manager.set("user:123", {"name": "John", "email": "john@example.com"})

# 获取缓存值
user_data = cache_manager.get("user:123")
print(f"User data: {user_data}")

# 批量操作
users = {
    "user:123": {"name": "John", "email": "john@example.com"},
    "user:456": {"name": "Jane", "email": "jane@example.com"}
}
cache_manager.set_many(users)

retrieved_users = cache_manager.get_many(["user:123", "user:456"])
print(f"Retrieved users: {retrieved_users}")

# 计数器操作
cache_manager.set("counter", 0)
cache_manager.increment("counter")  # 值变为1
cache_manager.increment("counter", 5)  # 值变为6

# 检查统计信息
stats = cache_manager.stats()
print(f"Cache stats: {stats}")
```

### DMSCCacheAsideStrategy

Cache-Aside策略实现，最常用的缓存策略。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get_or_set(key, func, ttl=None)` | 获取或设置 | `key: str`, `func: Callable`, `ttl: int` | `Any` |
| `get_or_set_many(keys, func, ttl=None)` | 批量获取或设置 | `keys: List[str]`, `func: Callable`, `ttl: int` | `Dict[str, Any]` |
| `invalidate(key)` | 失效缓存 | `key: str` | `bool` |
| `invalidate_many(keys)` | 批量失效 | `keys: List[str]` | `bool` |
| `invalidate_pattern(pattern)` | 按模式失效 | `pattern: str` | `int` |

#### 使用示例

```python
from dmsc import DMSCCacheAsideStrategy

# 初始化Cache-Aside策略
cache_aside = DMSCCacheAsideStrategy(cache_manager)

# 获取或设置（如果不存在则调用函数获取数据并缓存）
def get_user_from_db(user_id):
    # 模拟数据库查询
    return {"id": user_id, "name": f"User {user_id}", "email": f"user{user_id}@example.com"}

user = cache_aside.get_or_set("user:123", lambda: get_user_from_db(123))
print(f"User: {user}")

# 失效缓存
cache_aside.invalidate("user:123")

# 按模式失效（失效所有用户缓存）
invalidated_count = cache_aside.invalidate_pattern("user:*")
print(f"Invalidated {invalidated_count} user caches")
```

### DMSCWriteThroughStrategy

Write-Through策略实现，写操作同时更新缓存和数据库。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `set(key, value, write_func, ttl=None)` | 设置值 | `key: str`, `value: Any`, `write_func: Callable`, `ttl: int` | `bool` |
| `set_many(mapping, write_func, ttl=None)` | 批量设置 | `mapping: Dict[str, Any]`, `write_func: Callable`, `ttl: int` | `bool` |
| `delete(key, delete_func)` | 删除值 | `key: str`, `delete_func: Callable` | `bool` |
| `delete_many(keys, delete_func)` | 批量删除 | `keys: List[str]`, `delete_func: Callable` | `bool` |

#### 使用示例

```python
from dmsc import DMSCWriteThroughStrategy

# 初始化Write-Through策略
write_through = DMSCWriteThroughStrategy(cache_manager)

# 写操作（同时更新缓存和数据库）
def save_user_to_db(user_data):
    # 模拟保存到数据库
    print(f"Saving user to database: {user_data}")
    return True

user_data = {"id": 123, "name": "John Doe", "email": "john@example.com"}
write_through.set("user:123", user_data, save_user_to_db)

# 删除操作（同时从缓存和数据库删除）
def delete_user_from_db(user_id):
    # 模拟从数据库删除
    print(f"Deleting user from database: {user_id}")
    return True

write_through.delete("user:123", lambda: delete_user_from_db(123))
```

### DMSCWriteBehindStrategy

Write-Behind策略实现，异步批量写入数据库。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `set(key, value, ttl=None)` | 设置值 | `key: str`, `value: Any`, `ttl: int` | `bool` |
| `set_many(mapping, ttl=None)` | 批量设置 | `mapping: Dict[str, Any]`, `ttl: int` | `bool` |
| `delete(key)` | 删除值 | `key: str` | `bool` |
| `delete_many(keys)` | 批量删除 | `keys: List[str]` | `bool` |
| `flush()` | 刷新待写入数据 | `None` | `bool` |
| `start_batch_processor(interval=5)` | 启动批处理器 | `interval: int` | `None` |
| `stop_batch_processor()` | 停止批处理器 | `None` | `None` |

#### 使用示例

```python
from dmsc import DMSCWriteBehindStrategy

# 初始化Write-Behind策略
write_behind = DMSCWriteBehindStrategy(cache_manager)

# 启动批处理器
write_behind.start_batch_processor(interval=10)  # 每10秒批量写入一次

# 写操作（先写缓存，异步批量写数据库）
def batch_save_users(user_batch):
    # 模拟批量保存到数据库
    print(f"Batch saving {len(user_batch)} users to database")
    return True

# 注册批量写入函数
write_behind.register_batch_writer(batch_save_users)

# 多次写操作
for i in range(10):
    user_data = {"id": i, "name": f"User {i}", "email": f"user{i}@example.com"}
    write_behind.set(f"user:{i}", user_data)

# 手动刷新（可选）
write_behind.flush()

# 停止批处理器
write_behind.stop_batch_processor()
```

### DMSCDistributedCache

分布式缓存管理器，支持多节点缓存同步。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `join_cluster(node_id, cluster_nodes)` | 加入集群 | `node_id: str`, `cluster_nodes: List[str]` | `bool` |
| `leave_cluster()` | 离开集群 | `None` | `bool` |
| `sync_cache(key, value, ttl=None)` | 同步缓存 | `key: str`, `value: Any`, `ttl: int` | `bool` |
| `sync_cache_many(mapping, ttl=None)` | 批量同步缓存 | `mapping: Dict[str, Any]`, `ttl: int` | `bool` |
| `invalidate_cluster_cache(key)` | 失效集群缓存 | `key: str` | `bool` |
| `invalidate_cluster_cache_many(keys)` | 批量失效集群缓存 | `keys: List[str]` | `bool` |
| `get_cluster_nodes()` | 获取集群节点 | `None` | `List[str]` |
| `get_cluster_status()` | 获取集群状态 | `None` | `Dict` |

#### 使用示例

```python
from dmsc import DMSCDistributedCache

# 初始化分布式缓存
distributed_cache = DMSCDistributedCache(
    node_id="node-1",
    cache_manager=cache_manager
)

# 加入集群
cluster_nodes = ["node-1", "node-2", "node-3"]
distributed_cache.join_cluster("node-1", cluster_nodes)

# 同步缓存到所有节点
distributed_cache.sync_cache("global:config", {"version": "1.0.0", "features": ["auth", "cache"]})

# 批量同步
global_data = {
    "config:1": {"key": "value1"},
    "config:2": {"key": "value2"},
    "config:3": {"key": "value3"}
}
distributed_cache.sync_cache_many(global_data)

# 失效集群缓存
distributed_cache.invalidate_cluster_cache("user:123")

# 获取集群状态
cluster_status = distributed_cache.get_cluster_status()
print(f"Cluster status: {cluster_status}")
```