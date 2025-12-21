<div align="center">

# Cache API参考

**Version: 1.0.0**

**Last modified date: 2025-12-12**

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

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key)` | 获取缓存值 | `key: &str` | `DMSCResult<Option<String>>` |
| `set(key, value, ttl)` | 设置缓存值 | `key: &str`, `value: impl Serialize`, `ttl: Option<u64>` | `DMSCResult<()>` |
| `delete(key)` | 删除缓存 | `key: &str` | `DMSCResult<()>` |
| `exists(key)` | 检查缓存是否存在 | `key: &str` | `DMSCResult<bool>` |
| `clear()` | 清空所有缓存 | 无 | `DMSCResult<()>` |
| `keys(pattern)` | 获取匹配的键 | `pattern: &str` | `DMSCResult<Vec<String>>` |
| `ttl(key)` | 获取缓存过期时间 | `key: &str` | `DMSCResult<Option<u64>>` |
| `expire(key, ttl)` | 设置缓存过期时间 | `key: &str`, `ttl: u64` | `DMSCResult<()>` |
| `increment(key, delta)` | 数值递增 | `key: &str`, `delta: i64` | `DMSCResult<i64>` |
| `decrement(key, delta)` | 数值递减 | `key: &str`, `delta: i64` | `DMSCResult<i64>` |

#### 使用示例

```rust
use dms::prelude::*;

// 设置缓存
ctx.cache().set("user:1", &user, Some(3600)).await?;

// 获取缓存
let user: Option<User> = ctx.cache().get("user:1").await?;

// 检查缓存是否存在
let exists = ctx.cache().exists("user:1").await?;

// 删除缓存
ctx.cache().delete("user:1").await?;

// 数值操作
let count = ctx.cache().increment("counter", 1).await?;
let count = ctx.cache().decrement("counter", 5).await?;
```

### DMSCCacheConfig

缓存模块配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:--------|:-------------|:--------|
| `backend` | `DMSCCacheBackend` | 缓存后端类型 | `Memory` |
| `default_ttl` | `u64` | 默认过期时间（秒） | 3600 |
| `max_memory_size` | `usize` | 最大内存大小（字节） | 100MB |
| `redis_url` | `Option<String>` | Redis连接URL | `None` |
| `redis_pool_size` | `u32` | Redis连接池大小 | 10 |
| `cleanup_interval` | `u64` | 清理间隔（秒） | 300 |
| `compression` | `bool` | 是否启用压缩 | false |

#### 使用示例

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

缓存后端枚举类型。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Memory` | 内存缓存 |
| `Redis` | Redis缓存 |
| `Hybrid` | 混合缓存（内存+Redis） |
| `Custom` | 自定义缓存后端 |

## 缓存后端

### 内存缓存

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Memory,
    max_memory_size: 100 * 1024 * 1024, // 100MB
    ..Default::default()
};
```

### Redis缓存

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis,
    redis_url: Some("redis://localhost:6379".to_string()),
    redis_pool_size: 10,
    ..Default::default()
};
```

### 混合缓存

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Hybrid,
    max_memory_size: 50 * 1024 * 1024, // 50MB内存缓存
    redis_url: Some("redis://localhost:6379".to_string()),
    ..Default::default()
};
```

<div align="center">

## 高级功能

</div>

### 批量操作

```rust
// 批量获取
let keys = vec!["key1", "key2", "key3"];
let values = ctx.cache().get_multi(&keys).await?;

// 批量设置
let items = vec![
    ("key1", "value1"),
    ("key2", "value2"),
    ("key3", "value3"),
];
ctx.cache().set_multi(&items, Some(3600)).await?;

// 批量删除
ctx.cache().delete_multi(&keys).await?;
```

### 原子操作

```rust
// 原子递增并返回新值
let new_value = ctx.cache().increment_and_get("counter", 1).await?;

// 原子递减并返回新值
let new_value = ctx.cache().decrement_and_get("counter", 5).await?;

// 比较并设置
let success = ctx.cache().compare_and_set("key", "old_value", "new_value").await?;
```

### 分布式锁

```rust
// 获取分布式锁
let lock = ctx.cache().acquire_lock("resource_lock", 30).await?;

// 执行业务逻辑
// ...

// 释放锁
ctx.cache().release_lock("resource_lock", &lock).await?;
```
<div align="center">

## 缓存策略

</div>  

### TTL策略

```rust
// 设置相对过期时间
ctx.cache().set("key", &value, Some(3600)).await?; // 1小时

// 设置绝对过期时间
ctx.cache().set_at("key", &value, timestamp).await?;

// 获取剩余过期时间
let ttl = ctx.cache().ttl("key").await?;

// 延长过期时间
ctx.cache().expire("key", 7200).await?;
```

### 缓存穿透保护

```rust
// 使用布隆过滤器防止缓存穿透
ctx.cache().set_with_bloom_filter("key", &value, Some(3600)).await?;

// 检查布隆过滤器
let might_exist = ctx.cache().bloom_filter_might_contain("key").await?;
```

### 缓存雪崩保护

```rust
// 设置随机过期时间，避免同时过期
ctx.cache().set_with_jitter("key", &value, 3600, 300).await?; // ±5分钟随机
```

<div align="center">

## 序列化支持

</div>  

### JSON序列化

```rust
#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

// 存储结构体
let user = User { id: 1, name: "Alice".to_string(), email: "alice@example.com".to_string() };
ctx.cache().set("user:1", &user, Some(3600)).await?;

// 获取结构体
let user: Option<User> = ctx.cache().get("user:1").await?;
```

### 二进制序列化

```rust
// 使用二进制格式存储
ctx.cache().set_binary("binary_key", &binary_data, Some(3600)).await?;

// 获取二进制数据
let data = ctx.cache().get_binary("binary_key").await?;
```
<div align="center">

## 性能优化

</div>      

### 连接池

```rust
let config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis,
    redis_pool_size: 50, // 增大连接池
    ..Default::default()
};
```

### 压缩

```rust
let config = DMSCCacheConfig {
    compression: true, // 启用压缩
    ..Default::default()
};
```

### 批处理

```rust
// 使用管道批量操作
let pipeline = ctx.cache().pipeline();
pipeline.set("key1", "value1", Some(3600));
pipeline.set("key2", "value2", Some(3600));
pipeline.execute().await?;
```
<div align="center">

## 监控和统计

</div>

### 缓存统计

```rust
// 获取缓存统计信息
let stats = ctx.cache().get_stats().await?;
println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
println!("Hit rate: {:.2}%", stats.hit_rate * 100.0);
```

### 键空间通知

```rust
// 监听键过期事件
ctx.cache().subscribe_key_events("expired", |event| {
    println!("Key expired: {}", event.key);
}).await?;
```
<div align="center">

## 错误处理

</div>

### 缓存错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `CACHE_CONNECTION_FAILED` | 缓存连接失败 |
| `CACHE_OPERATION_FAILED` | 缓存操作失败 |
| `CACHE_SERIALIZATION_ERROR` | 缓存序列化错误 |
| `CACHE_KEY_NOT_FOUND` | 缓存键不存在 |
| `CACHE_LOCK_ACQUISITION_FAILED` | 分布式锁获取失败 |

### 错误处理示例

```rust
match ctx.cache().get::<User>("user:1").await {
    Ok(Some(user)) => {
        // 缓存命中
        println!("User from cache: {:?}", user);
    }
    Ok(None) => {
        // 缓存未命中
        let user = load_user_from_database(1).await?;
        ctx.cache().set("user:1", &user, Some(3600)).await?;
    }
    Err(DMSCError { code, .. }) if code == "CACHE_CONNECTION_FAILED" => {
        // 缓存连接失败，回退到数据库
        let user = load_user_from_database(1).await?;
    }
    Err(e) => {
        // 其他错误
        return Err(e);
    }
}
```
<div align="center">

## 最佳实践

</div>

1. **合理设置TTL**: 根据数据更新频率设置合适的过期时间
2. **使用批量操作**: 减少网络往返，提高性能
3. **实现缓存预热**: 在应用启动时加载热点数据
4. **处理缓存穿透**: 使用布隆过滤器或空值缓存
5. **监控缓存命中率**: 及时调整缓存策略
6. **使用连接池**: 避免频繁创建连接
7. **启用压缩**: 对于大值数据启用压缩减少内存占用

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，提供JWT、OAuth2和RBAC认证授权功能
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录认证事件和安全日志
- [config](./config.md): 配置模块，管理认证配置和密钥设置
- [database](./database.md): 数据库模块，提供用户数据持久化和查询功能
- [http](./http.md): HTTP模块，提供Web认证接口和中间件支持
- [mq](./mq.md): 消息队列模块，处理认证事件和异步通知
- [observability](./observability.md): 可观测性模块，监控认证性能和安全事件
- [security](./security.md): 安全模块，提供加密、哈希和验证功能
- [storage](./storage.md): 存储模块，管理认证文件、密钥和证书
- [validation](./validation.md): 验证模块，验证用户输入和表单数据