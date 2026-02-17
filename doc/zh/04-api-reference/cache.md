<div align="center">

# Cache API参考

**Version: 0.1.7**

**Last modified date: 2026-02-17**

cache模块提供多后端缓存抽象，支持内存、Redis、混合等多种缓存后端。

## 模块概述

</div>

cache模块包含以下子模块：

- **core**: 缓存核心接口和类型定义
- **manager**: 缓存管理器，统一管理多个缓存后端
- **backends**: 各种缓存后端实现
- **config**: 缓存配置

<div align="center">

## 核心组件

</div>

### DMSCCacheModule

缓存模块主接口，提供统一的缓存服务访问。

**注意**：此类提供对缓存管理器的访问入口，具体的缓存操作请通过`cache_manager()`方法获取`DMSCCacheManager`。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `cache_manager()` | 获取缓存管理器 | 无 | `Arc<RwLock<DMSCCacheManager>>` |
| `backend()` | 获取当前使用的缓存后端类型 | 无 | `DMSCCacheBackendType` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 通过module访问缓存管理器
let cache_manager = ctx.module::<DMSCCacheModule>().await?
    .cache_manager();
    
// 设置缓存
cache_manager.set("user:1", &user, Some(3600)).await?;

// 获取缓存
let user: Option<User> = cache_manager.get("user:1").await?;

// 检查缓存是否存在
let exists = cache_manager.exists("user:1").await;

// 删除缓存
cache_manager.delete("user:1").await?;

// 获取或设置缓存值
let user = cache_manager.get_or_set("user:1", Some(3600), || async {
    fetch_user_from_db().await
}).await?;
```

### DMSCCacheManager

缓存管理器，负责具体的缓存操作。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key)` | 获取缓存值 | `key: &str` | `DMSCResult<Option<T>>` |
| `set(key, value, ttl_seconds)` | 设置缓存值 | `key: &str`, `value: &T`, `ttl_seconds: Option<u64>` | `DMSCResult<()>` |
| `delete(key)` | 删除缓存 | `key: &str` | `DMSCResult<bool>` |
| `exists(key)` | 检查缓存是否存在 | `key: &str` | `bool` |
| `clear()` | 清空所有缓存 | 无 | `DMSCResult<()>` |
| `stats()` | 获取缓存统计 | 无 | `DMSCCacheStats` |
| `cleanup_expired()` | 清理过期缓存 | 无 | `DMSCResult<usize>` |
| `get_or_set(key, ttl_seconds, factory)` | 获取或设置缓存值 | `key: &str`, `ttl_seconds: Option<u64>`, `factory: F` where `F: FnOnce() -> Fut`, `Fut: Future` | `DMSCResult<T>` |

### DMSCCacheStats

缓存统计数据结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `hits` | `u64` | 缓存命中次数 |
| `misses` | `u64` | 缓存未命中次数 |
| `entries` | `usize` | 缓存中的条目数 |
| `memory_usage_bytes` | `usize` | 内存使用量（字节） |
| `avg_hit_rate` | `f64` | 平均命中率 (0.0-1.0) |
| `hit_count` | `u64` | 命中计数 |
| `miss_count` | `u64` | 未命中计数 |
| `eviction_count` | `u64` | 驱逐条目数 |

#### 使用示例

```rust
use dmsc::cache::DMSCCacheModule;

let cache_module = DMSCCacheModule::new(config).await?;
let cache_manager = cache_module.cache_manager();

// 设置缓存
cache_manager.set("key", "value", Some(3600)).await?;

// 获取缓存
let result = cache_manager.get("key").await?;
match result {
    Some(value) => println!("Cache hit: {}", value),
    None => println!("Cache miss"),
}

// 检查缓存是否存在
if cache_manager.exists("key") {
    println!("Key exists");
}

// 删除缓存
cache_manager.delete("key").await?;
```

### DMSCCacheConfig

缓存模块配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `enabled` | `bool` | 是否启用缓存 | `true` |
| `backend_type` | `DMSCCacheBackendType` | 缓存后端类型 | `Memory` |
| `default_ttl_secs` | `u64` | 默认过期时间（秒） | 3600 |
| `max_memory_mb` | `u64` | 最大内存大小（MB） | 512 |
| `cleanup_interval_secs` | `u64` | 清理间隔（秒） | 300 |
| `redis_url` | `String` | Redis连接URL | `"redis://127.0.0.1:6379"` |
| `redis_pool_size` | `usize` | Redis连接池大小 | 10 |

#### 使用示例

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

缓存值包装器，支持TTL过期和LRU淘汰策略。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:--------|:-------------|
| `value` | `String` | 缓存的实际数据 |
| `expires_at` | `Option<u64>` | 基于TTL的过期时间戳（UNIX epoch秒），None表示永不过期 |
| `last_accessed` | `Option<u64>` | 最后访问时间戳（UNIX epoch秒），用于LRU淘汰策略 |

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(value, ttl)` | 创建缓存值 | `value: String`, `ttl: Option<u64>` | `Self` |
| `touch()` | 更新最后访问时间 | 无 | `()` |
| `is_expired()` | 检查是否过期 | 无 | `bool` |
| `is_stale(max_idle_secs)` | 检查是否因长时间未访问而变陈旧 | `max_idle_secs: u64` | `bool` |
| `deserialize<T>()` | 反序列化为指定类型 | 无 | `DMSCResult<T>` |

#### 使用示例

```rust
use dmsc::cache::DMSCCachedValue;

// 创建缓存值，1小时后过期
let cached = DMSCCachedValue::new("user_data".to_string(), Some(3600));

// 访问时更新最后访问时间
cached.touch();

// 检查是否过期
if cached.is_expired() {
    println!("Cache expired");
}

// 检查是否因长时间未访问而变陈旧（用于LRU淘汰）
if cached.is_stale(300) {
    println!("Cache is stale, may be evicted by LRU policy");
}

// 反序列化
let user: User = cached.deserialize()?;
```

#### LRU淘汰策略支持

`DMSCCachedValue`提供以下功能支持LRU缓存淘汰：

- **touch()**: 每次访问缓存时调用，更新最后访问时间
- **is_stale(max_idle_secs)**: 判断缓存项是否超过最大空闲时间

```rust
// LRU淘汰示例
let max_idle_seconds = 300; // 5分钟
for (_, cached) in cache.iter() {
    if cached.is_stale(max_idle_seconds) {
        // 移除长时间未访问的缓存项
        cache.remove(key);
    }
}
```

### DMSCCacheBackendType

缓存后端类型枚举。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Memory` | 内存缓存（默认） |
| `Redis` | Redis分布式缓存 |
| `Hybrid` | 混合缓存（内存+Redis） |

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [log](./log.md): 日志模块，记录协议事件
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信

