<div align="center">

# Cache API参考

**Version: 0.1.4**

**Last modified date: 2026-01-15**

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
| `cache_manager()` | 获取缓存管理器 | 无 | `Arc<DMSCCacheManager>` |
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
let exists = cache_manager.exists("user:1").await?;

// 删除缓存
cache_manager.delete("user:1").await?;

// 数值操作
let count = cache_manager.increment("counter", 1).await?;
let count = cache_manager.decrement("counter", 5).await?;
```

### DMSCCacheManager

缓存管理器，负责具体的缓存操作。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `get(key)` | 获取缓存值 | `key: &str` | `DMSCResult<Option<T>>` |
| `set(key, value, ttl)` | 设置缓存值 | `key: &str`, `value: &T`, `ttl: Option<u64>` | `DMSCResult<()>` |
| `delete(key)` | 删除缓存 | `key: &str` | `DMSCResult<bool>` |
| `exists(key)` | 检查缓存是否存在 | `key: &str` | `DMSCResult<bool>` |
| `clear()` | 清空所有缓存 | 无 | `DMSCResult<()>` |
| `invalidate_pattern(pattern)` | 按模式失效缓存 | `pattern: &str` | `DMSCResult<()>` |
| `stats()` | 获取缓存统计 | 无 | `DMSCCacheStats` |
| `cleanup_expired()` | 清理过期缓存 | 无 | `DMSCResult<usize>` |

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
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能

