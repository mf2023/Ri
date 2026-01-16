<div align="center">

# 缓存使用示例

**Version: 0.1.4**

**Last modified date: 2026-01-15**

本示例展示如何使用DMSC的cache模块进行多种缓存后端和高级缓存功能的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 内存缓存和Redis缓存的使用
- 缓存标签和原子操作
- 分布式锁实现
- 缓存健康检查和监控
- 数据序列化与压缩
- 错误处理和降级策略

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解缓存基本概念
- （可选）Redis服务器用于Redis缓存示例

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-cache-example
cd dms-cache-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-cache-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

cache:
  backend: "memory"  # 或 "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
  memory:
    max_size: 1000
    ttl: 3600
```

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_cache(DMSCCacheConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Cache Example started")?;
        
        // 基本缓存操作示例
        basic_cache_operations(&ctx).await?;
        
        // 高级缓存功能示例
        advanced_cache_features(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Cache Example completed")?;
        
        Ok(())
    }).await
}

async fn basic_cache_operations(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("cache", "Starting basic cache operations")?;
    
    // 设置缓存值
    ctx.cache().set("user:123", json!({
        "id": 123,
        "name": "John Doe",
        "email": "john@example.com"
    }), Some(Duration::from_secs(3600)))?;
    
    // 获取缓存值
    if let Some(user_data) = ctx.cache().get("user:123")? {
        ctx.logger().info("cache", &format!("Retrieved user: {}", user_data))?;
    }
    
    // 检查缓存是否存在
    if ctx.cache().exists("user:123")? {
        ctx.logger().info("cache", "User cache exists")?;
    }
    
    // 删除缓存
    ctx.cache().delete("user:123")?;
    ctx.logger().info("cache", "User cache deleted")?;
    
    Ok(())
}

async fn advanced_cache_features(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("cache", "Starting advanced cache features")?;
    
    // 使用缓存标签
    ctx.cache().set_with_tags(
        "article:123",
        json!({
            "id": 123,
            "title": "Rust Programming Guide",
            "author": "John Doe"
        }),
        Some(Duration::from_secs(86400)),
        vec!["articles", "programming", "rust"]
    )?;
    
    // 原子递增操作
    let new_value = ctx.cache().increment("counter:page_views", 1)?;
    ctx.logger().info("cache", &format!("Page views: {}", new_value))?;
    
    // 获取标签下的所有键
    let rust_keys = ctx.cache().get_keys_by_tag("rust")?;
    ctx.logger().info("cache", &format!("Found {} items with 'rust' tag", rust_keys.len()))?;
    
    Ok(())
}
```

<div align="center">

## 代码解析

</div>

## 基本缓存操作

### 内存缓存

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建内存缓存后端
let memory_backend = DMSCCacheBackend::Memory {
    max_size: 1000,
    ttl: Duration::from_secs(3600),
};

// 初始化缓存模块
let cache_config = DMSCCacheConfig {
    backend: memory_backend,
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,
    encryption: false,
};

ctx.cache().init(cache_config)?;

// 设置缓存值
ctx.cache().set("user:123", json!({
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com"
}), Some(Duration::from_secs(3600)))?;

// 获取缓存值
if let Some(user_data) = ctx.cache().get("user:123")? {
    ctx.log().info(format!("Retrieved user: {}", user_data));
}

// 删除缓存
ctx.cache().delete("user:123")?;

// 检查缓存是否存在
if ctx.cache().exists("user:123")? {
    ctx.log().info("User cache exists");
}

// 获取缓存TTL
if let Some(ttl) = ctx.cache().ttl("user:123")? {
    ctx.log().info(format!("Cache expires in {} seconds", ttl.as_secs()));
}
```

### Redis缓存

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建Redis缓存后端
let redis_backend = DMSCCacheBackend::Redis {
    url: "redis://localhost:6379".to_string(),
    pool_size: 10,
    key_prefix: "app".to_string(),
    connection_timeout: Duration::from_secs(10),
    operation_timeout: Duration::from_secs(5),
};

let cache_config = DMSCCacheConfig {
    backend: redis_backend,
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,
    encryption: false,
};

ctx.cache().init(cache_config)?;

// 使用Redis缓存
ctx.cache().set("session:abc123", json!({
    "user_id": 123,
    "role": "admin",
    "permissions": ["read", "write", "delete"],
    "login_time": "2024-01-15T10:30:00Z"
}), Some(Duration::from_secs(7200)))?;

// 批量操作
let items = vec![
    ("key1", json!("value1")),
    ("key2", json!("value2")),
    ("key3", json!("value3")),
];
ctx.cache().set_many(items, Some(Duration::from_secs(3600)))?;

let keys = vec!["key1", "key2", "key3"];
let values = ctx.cache().get_many(&keys)?;
for (key, value) in values {
    ctx.log().info(format!("{}: {}", key, value.unwrap_or(json!(null))));
}
```

## 高级缓存功能

### 缓存标签

```rust
use dmsc::prelude::*;
use serde_json::json;

// 设置带标签的缓存
ctx.cache().set_with_tags(
    "article:123",
    json!({
        "id": 123,
        "title": "Rust Programming Guide",
        "author": "John Doe",
        "tags": ["rust", "programming", "tutorial"],
        "created_at": "2024-01-15T10:30:00Z"
    }),
    Some(Duration::from_secs(86400)),
    vec!["articles", "programming", "rust"]
)?;

// 通过标签批量删除
ctx.cache().delete_by_tags(vec!["rust"])?;

// 获取标签下的所有键
let rust_keys = ctx.cache().get_keys_by_tag("rust")?;
ctx.log().info(format!("Found {} items with 'rust' tag", rust_keys.len()));
```

### 原子操作

```rust
use dmsc::prelude::*;

// 原子递增
let new_value = ctx.cache().increment("counter:page_views", 1)?;
ctx.log().info(format!("Page views: {}", new_value));

// 原子递减
let new_value = ctx.cache().decrement("counter:inventory:123", 5)?;
ctx.log().info(format!("Inventory: {}", new_value));

// 比较并设置 (CAS)
let success = ctx.cache().compare_and_set("config:feature_flag", "old_value", "new_value")?;
if success {
    ctx.log().info("Feature flag updated successfully");
}

// 获取并设置
let old_value = ctx.cache().get_and_set("session:last_activity", json!("2024-01-15T11:00:00Z"))?;
ctx.log().info(format!("Previous activity: {:?}", old_value));
```

### 分布式锁

```rust
use dmsc::prelude::*;
use tokio::time::{sleep, Duration};

// 获取分布式锁
let lock_key = "lock:resource:123";
let lock_value = "worker-1";
let ttl = Duration::from_secs(30);

match ctx.cache().acquire_lock(lock_key, lock_value, ttl)? {
    Some(lock) => {
        ctx.log().info("Lock acquired successfully");
        
        // 执行临界区操作
        perform_critical_operation().await?;
        
        // 释放锁
        ctx.cache().release_lock(lock_key, lock_value)?;
        ctx.log().info("Lock released");
    }
    None => {
        ctx.log().warn("Failed to acquire lock");
        return Err(DMSCError::resource_busy("Resource is locked"));
    }
}

// 使用锁的便捷方法
ctx.cache().with_lock("lock:report_generation", "worker-1", Duration::from_secs(60), || async {
    ctx.log().info("Generating report...");
    generate_report().await?;
    ctx.log().info("Report generated successfully");
    Ok(())
}).await?;
```

## 缓存策略

### 缓存穿透保护

```rust
use dmsc::prelude::*;
use serde_json::json;

// 获取用户数据，带缓存穿透保护
async fn get_user_with_cache_through_protection(user_id: u64) -> DMSCResult<Option<Value>> {
    let cache_key = format!("user:{}", user_id);
    
    // 先尝试从缓存获取
    if let Some(cached_data) = ctx.cache().get(&cache_key)? {
        return Ok(Some(cached_data));
    }
    
    // 检查布隆过滤器（防止缓存穿透）
    if !ctx.cache().bloom_filter_might_contain("users", &user_id.to_string())? {
        ctx.log().info(format!("User {} definitely does not exist", user_id));
        return Ok(None);
    }
    
    // 从数据库获取
    match fetch_user_from_database(user_id).await? {
        Some(user_data) => {
            // 设置缓存
            ctx.cache().set(&cache_key, user_data.clone(), Some(Duration::from_secs(3600)))?;
            Ok(Some(user_data))
        }
        None => {
            // 设置空值缓存（防止缓存穿透）
            ctx.cache().set(&cache_key, json!(null), Some(Duration::from_secs(300)))?;
            Ok(None)
        }
    }
}

// 使用布隆过滤器
ctx.cache().bloom_filter_add("users", "123")?;
ctx.cache().bloom_filter_add("users", "456")?;

if ctx.cache().bloom_filter_might_contain("users", "123")? {
    ctx.log().info("User 123 might exist");
}
```

### 缓存雪崩保护

```rust
use dmsc::prelude::*;
use serde_json::json;
use rand::Rng;

// 设置缓存时添加随机TTL偏移，防止缓存雪崩
fn set_cache_with_jitter(key: &str, value: Value, base_ttl: Duration) -> DMSCResult<()> {
    let jitter = rand::thread_rng().gen_range(0..300); // 0-5分钟随机偏移
    let ttl = base_ttl + Duration::from_secs(jitter);
    
    ctx.cache().set(key, value, Some(ttl))?;
    Ok(())
}

// 批量设置缓存时使用不同的TTL
let articles = vec![
    ("article:1", json!({"id": 1, "title": "Article 1"}), Duration::from_secs(3600)),
    ("article:2", json!({"id": 2, "title": "Article 2"}), Duration::from_secs(3600 + 60)),
    ("article:3", json!({"id": 3, "title": "Article 3"}), Duration::from_secs(3600 + 120)),
];

for (key, value, ttl) in articles {
    set_cache_with_jitter(key, value, ttl)?;
}
```

### 缓存预热

```rust
use dmsc::prelude::*;
use serde_json::json;

// 系统启动时预热热门数据
async fn warmup_cache() -> DMSCResult<()> {
    ctx.log().info("Starting cache warmup...");
    
    // 预热热门文章
    let popular_articles = fetch_popular_articles().await?;
    for article in popular_articles {
        let key = format!("article:{}", article["id"]);
        ctx.cache().set(&key, article, Some(Duration::from_secs(3600)))?;
    }
    
    // 预热用户会话
    let active_sessions = fetch_active_sessions().await?;
    for session in active_sessions {
        let key = format!("session:{}", session["id"]);
        ctx.cache().set(&key, session, Some(Duration::from_secs(7200)))?;
    }
    
    // 预热配置数据
    let config_data = fetch_configuration().await?;
    ctx.cache().set("config:app", config_data, Some(Duration::from_secs(86400)))?;
    
    ctx.log().info("Cache warmup completed");
    Ok(())
}
```

## 缓存监控

### 缓存统计

```rust
use dmsc::prelude::*;

// 获取缓存统计信息
let stats = ctx.cache().get_stats()?;
ctx.log().info(format!("Cache stats: {:?}", stats));

// 监控特定键的命中率
let key_stats = ctx.cache().get_key_stats("user:123")?;
ctx.log().info(format!("Key stats: {:?}", key_stats));

// 获取缓存大小
let cache_size = ctx.cache().get_cache_size()?;
ctx.log().info(format!("Cache size: {} bytes", cache_size));

// 清理过期缓存
let cleaned_count = ctx.cache().cleanup_expired()?;
ctx.log().info(format!("Cleaned {} expired entries", cleaned_count));
```

### 缓存健康检查

```rust
use dmsc::prelude::*;
use serde_json::json;

// 检查缓存健康状态
fn check_cache_health() -> DMSCResult<Value> {
    let health = ctx.cache().health_check()?;
    
    let status = if health.is_healthy {
        "healthy"
    } else {
        "unhealthy"
    };
    
    Ok(json!({
        "status": status,
        "backend": health.backend,
        "response_time_ms": health.response_time.as_millis(),
        "error_rate": health.error_rate,
        "memory_usage": health.memory_usage,
        "connection_pool": {
            "active": health.active_connections,
            "idle": health.idle_connections,
            "max": health.max_connections,
        },
        "last_check": health.last_check,
        "errors": health.recent_errors,
    }))
}

// 定期健康检查
ctx.observability().register_health_check("cache", || async {
    match check_cache_health() {
        Ok(health) => Ok(health),
        Err(e) => Err(DMSCError::internal(format!("Cache health check failed: {}", e))),
    }
}).await?;
```

## 序列化与压缩

### 数据序列化

```rust
use dmsc::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    id: u64,
    name: String,
    email: String,
    role: String,
    created_at: String,
}

// 存储序列化数据
let user = User {
    id: 123,
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    role: "admin".to_string(),
    created_at: "2024-01-15T10:30:00Z".to_string(),
};

// 使用MessagePack序列化（更紧凑）
let cache_config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Memory {
        max_size: 1000,
        ttl: Duration::from_secs(3600),
    },
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,  // 启用压缩
    encryption: false,
    serializer: DMSCSerializer::MessagePack, // 使用MessagePack
};

ctx.cache().init(cache_config)?;
ctx.cache().set("user:123", json!(user), None)?;

// 获取并反序列化
if let Some(cached_data) = ctx.cache().get("user:123")? {
    let cached_user: User = serde_json::from_value(cached_data)?;
    ctx.log().info(format!("Retrieved user: {:?}", cached_user));
}
```

### 数据压缩

```rust
use dmsc::prelude::*;
use serde_json::json;

// 存储大型数据时启用压缩
let large_data = json!({
    "articles": (0..1000).map(|i| {
        json!({
            "id": i,
            "title": format!("Article {}", i),
            "content": "Lorem ipsum dolor sit amet, consectetur adipiscing elit. ".repeat(100),
            "tags": ["tag1", "tag2", "tag3"],
            "metadata": {
                "author": format!("Author {}", i % 10),
                "views": i * 100,
                "likes": i * 10,
                "comments": i * 5,
            }
        })
    }).collect::<Vec<_>>()
});

// 启用压缩存储
let cache_config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Redis {
        url: "redis://localhost:6379".to_string(),
        pool_size: 10,
        key_prefix: "app".to_string(),
        connection_timeout: Duration::from_secs(10),
        operation_timeout: Duration::from_secs(5),
    },
    default_ttl: Duration::from_secs(3600),
    key_prefix: "app".to_string(),
    compression: true,  // 启用压缩
    compression_threshold: 1024, // 大于1KB的数据才压缩
    compression_level: 6, // 压缩级别 (1-9)
    encryption: false,
};

ctx.cache().init(cache_config)?;

let start = std::time::Instant::now();
ctx.cache().set("large_dataset", large_data.clone(), None)?;
let store_time = start.elapsed();

ctx.log().info(format!("Stored large dataset in {:?}", store_time));

// 验证压缩效果
let uncompressed_size = serde_json::to_vec(&large_data)?.len();
ctx.log().info(format!("Uncompressed size: {} bytes", uncompressed_size));
```

## 错误处理

### 缓存错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 处理缓存错误
match ctx.cache().set("key", json!("value"), None) {
    Ok(_) => ctx.log().info("Cache set successfully"),
    Err(DMSCError::CacheConnectionError(e)) => {
        ctx.log().error(format!("Cache connection failed: {}", e));
        // 降级到数据库或其他后备存储
        fallback_to_database("key", json!("value")).await?;
    }
    Err(DMSCError::CacheTimeoutError(e)) => {
        ctx.log().warn(format!("Cache operation timed out: {}", e));
        // 重试或降级
        retry_cache_operation().await?;
    }
    Err(DMSCError::CacheFullError) => {
        ctx.log().warn("Cache is full");
        // 清理过期缓存或增加容量
        ctx.cache().cleanup_expired()?;
    }
    Err(e) => {
        ctx.log().error(format!("Cache error: {}", e));
        return Err(e);
    }
}

// 缓存降级策略
async fn get_data_with_fallback(key: &str) -> DMSCResult<Value> {
    // 首先尝试缓存
    if let Ok(Some(cached)) = ctx.cache().get(key) {
        return Ok(cached);
    }
    
    // 缓存失败，尝试数据库
    match fetch_from_database(key).await {
        Ok(data) => {
            // 异步更新缓存（不阻塞主流程）
            let key = key.to_string();
            let data_clone = data.clone();
            tokio::spawn(async move {
                if let Err(e) = ctx.cache().set(&key, data_clone, Some(Duration::from_secs(3600))) {
                    ctx.log().warn(format!("Failed to update cache: {}", e));
                }
            });
            
            Ok(data)
        }
        Err(e) => {
            ctx.log().error(format!("Database fallback failed: {}", e));
            Err(e)
        }
    }
}
```

<div align="center">

## 运行步骤

</div>

### 1. 环境准备

确保已安装以下组件：
- Rust 1.65+ 和 Cargo
- （可选）Redis 服务器（用于Redis缓存示例）

### 2. 创建项目

```bash
cargo new dms-cache-example
cd dms-cache-example
```

### 3. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 4. 创建配置

在项目根目录创建 `config.yaml`：

```yaml
service:
  name: "dms-cache-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

cache:
  backend: "memory"  # 或 "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
  memory:
    max_size: 1000
    ttl: 3600
```

### 5. 运行示例

```bash
cargo run
```

<div align="center">

## 预期结果

</div>

运行成功后，您将看到类似以下输出：

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "dms_cache_example",
  "message": "DMSC Cache Example started"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "Starting basic cache operations"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "Retrieved user: {\"id\":123,\"name\":\"John Doe\",\"email\":\"john@example.com\"}"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "User cache exists"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "User cache deleted"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "Starting advanced cache features"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "Page views: 1"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "cache",
  "message": "Found 1 items with 'rust' tag"
}
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "target": "service",
  "message": "DMSC Cache Example completed"
}
```

<div align="center">

## 扩展功能

</div>

### 缓存集群支持

```rust
use dmsc::prelude::*;

// 配置多节点Redis集群
let cluster_backend = DMSCCacheBackend::RedisCluster {
    nodes: vec![
        "redis://node1:6379".to_string(),
        "redis://node2:6379".to_string(),
        "redis://node3:6379".to_string(),
    ],
    pool_size: 20,
    key_prefix: "app".to_string(),
    connection_timeout: Duration::from_secs(10),
    operation_timeout: Duration::from_secs(5),
    read_from_replicas: true, // 从副本读取以提高性能
};

let cache_config = DMSCCacheConfig {
    backend: cluster_backend,
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,
    encryption: false,
};

ctx.cache().init(cache_config)?;

// 集群感知的路由
ctx.cache().set_with_shard_key("user:123", json!({"name": "John"}), Some(Duration::from_secs(3600)), "shard_1")?;
```

### 智能缓存预热

```rust
use dmsc::prelude::*;
use machine_learning::cache::CachePredictor;

// 基于机器学习预测缓存需求
async fn intelligent_cache_warmup() -> DMSCResult<()> {
    let predictor = CachePredictor::new();
    
    // 分析历史访问模式
    let access_patterns = ctx.cache().get_access_history(30)?; // 30天历史数据
    let predictions = predictor.predict_hot_data(&access_patterns)?;
    
    // 预热预测的热门数据
    for predicted_key in predictions.hot_keys {
        if let Ok(data) = fetch_data_from_source(&predicted_key).await {
            ctx.cache().set(&predicted_key, data, Some(Duration::from_secs(3600)))?;
        }
    }
    
    // 基于时间模式预热
    let time_based_keys = predictor.predict_by_time_pattern(chrono::Local::now())?;
    for key in time_based_keys {
        if let Ok(data) = fetch_data_from_source(&key).await {
            ctx.cache().set(&key, data, Some(Duration::from_secs(1800)))?;
        }
    }
    
    ctx.logger().info("Intelligent cache warmup completed");
    Ok(())
}
```

### 自适应缓存策略

```rust
use dmsc::prelude::*;
use std::collections::HashMap;

// 动态调整缓存策略
pub struct AdaptiveCacheManager {
    hit_rate_threshold: f64,
    miss_rate_threshold: f64,
    adjustment_interval: Duration,
}

impl AdaptiveCacheManager {
    pub async fn optimize_cache_strategy(&self) -> DMSCResult<()> {
        let stats = ctx.cache().get_detailed_stats()?;
        
        // 分析缓存性能指标
        let hit_rate = stats.hit_rate;
        let miss_rate = stats.miss_rate;
        let avg_response_time = stats.avg_response_time;
        
        // 动态调整TTL
        if hit_rate < self.hit_rate_threshold {
            // 命中率低，增加TTL
            ctx.cache().adjust_global_ttl(Duration::from_secs(3600))?;
        } else if miss_rate > self.miss_rate_threshold {
            // 未命中率高，减少TTL并优化预取
            ctx.cache().adjust_global_ttl(Duration::from_secs(900))?;
            self.optimize_prefetch_strategy().await?;
        }
        
        // 动态调整缓存大小
        if stats.memory_usage > 0.8 {
            // 内存使用率高，启用更激进的清理策略
            ctx.cache().set_eviction_policy(DMSCEvictionPolicy::LFU)?;
        } else {
            ctx.cache().set_eviction_policy(DMSCEvictionPolicy::LRU)?;
        }
        
        // 调整压缩策略
        if avg_response_time > Duration::from_millis(100) {
            ctx.cache().enable_compression(true)?;
            ctx.cache().set_compression_threshold(512)?; // 降低压缩阈值
        }
        
        Ok(())
    }
    
    async fn optimize_prefetch_strategy(&self) -> DMSCResult<()> {
        // 基于访问模式优化预取策略
        let patterns = ctx.cache().analyze_access_patterns()?;
        
        for pattern in patterns {
            if pattern.confidence > 0.8 {
                // 高置信度模式，预取相关数据
                for related_key in pattern.related_keys {
                    if let Ok(data) = fetch_data_from_source(&related_key).await {
                        ctx.cache().set(&related_key, data, Some(pattern.suggested_ttl))?;
                    }
                }
            }
        }
        
        Ok(())
    }
}
```

### 分布式缓存一致性

```rust
use dmsc::prelude::*;
use distributed_consensus::raft::RaftNode;

// 实现分布式缓存一致性
pub struct DistributedCacheConsistency {
    raft_node: RaftNode,
    cache_instances: Vec<String>,
}

impl DistributedCacheConsistency {
    pub async fn maintain_consistency(&self) -> DMSCResult<()> {
        // 使用Raft协议确保缓存更新的一致性
        let update_command = CacheUpdateCommand {
            key: "user:123".to_string(),
            value: json!({"name": "John", "updated": chrono::Utc::now()}),
            ttl: Duration::from_secs(3600),
        };
        
        // 通过Raft协议复制更新
        match self.raft_node.propose(update_command).await? {
            ConsensusResult::Committed => {
                // 更新已提交，应用到所有节点
                self.apply_to_all_nodes(&update_command).await?;
                ctx.logger().info("Cache update committed and replicated");
            }
            ConsensusResult::Rejected => {
                ctx.logger().warn("Cache update rejected by consensus");
                return Err(DMSCError::consensus_error("Update rejected"));
            }
        }
        
        Ok(())
    }
    
    async fn apply_to_all_nodes(&self, command: &CacheUpdateCommand) -> DMSCResult<()> {
        for instance in &self.cache_instances {
            match self.update_remote_cache(instance, command).await {
                Ok(_) => ctx.logger().info(format!("Updated cache node: {}", instance)),
                Err(e) => ctx.logger().error(format!("Failed to update node {}: {}", instance, e)),
            }
        }
        Ok(())
    }
}
```

<div align="center">

## 最佳实践

</div>

1. **选择合适的缓存后端**: 内存缓存适合单机应用，Redis适合分布式环境
2. **设置合理的TTL**: 根据数据更新频率设置缓存过期时间
3. **使用缓存标签**: 便于批量管理和清理相关缓存
4. **实现缓存降级**: 当缓存不可用时，有后备方案
5. **监控缓存性能**: 定期检查命中率、响应时间等指标
6. **防止缓存穿透**: 使用布隆过滤器或空值缓存
7. **避免缓存雪崩**: 为TTL添加随机偏移
8. **合理序列化**: 选择高效的序列化格式，如MessagePack
9. **压缩大数据**: 对大型数据进行压缩，节省存储空间
10. **定期清理**: 清理过期缓存，释放资源
11. **使用分布式锁**: 确保关键操作的互斥性
12. **实现缓存预热**: 系统启动时加载热门数据
13. **监控内存使用**: 避免缓存占用过多内存
14. **配置连接池**: 合理设置连接池大小和超时时间
15. **实现一致性哈希**: 在分布式环境中保持数据分布均衡

<div align="center">

## 总结

</div>

本示例全面展示了 DMSC 缓存模块的核心功能和高级特性，涵盖以下关键能力：

### 🚀 核心功能
- **多后端支持**: 内存缓存、Redis缓存和Redis集群的无缝集成
- **基本缓存操作**: 设置、获取、删除、存在检查等基础功能
- **高级缓存功能**: 标签管理、原子操作、分布式锁实现
- **缓存策略**: 穿透保护、雪崩保护、预热机制
- **序列化与压缩**: 支持多种序列化格式和数据压缩
- **健康监控**: 实时缓存状态监控和性能统计

### 🔧 高级特性
- **缓存集群**: 多节点Redis集群支持和智能路由
- **智能预热**: 基于机器学习的缓存预测和预热
- **自适应策略**: 动态调整TTL、压缩和清理策略
- **分布式一致性**: 使用Raft协议确保缓存一致性
- **性能优化**: 连接池管理、批量操作、异步处理
- **错误处理**: 完善的降级策略和异常处理

### 💡 最佳实践
- 选择合适的缓存后端，根据应用场景优化配置
- 设置合理的TTL，平衡数据新鲜度和缓存命中率
- 使用缓存标签，便于批量管理和清理
- 实现缓存降级，确保系统高可用性
- 监控关键指标，持续优化缓存性能
- 防止缓存穿透和雪崩，保障系统稳定性
- 合理配置序列化和压缩，优化存储效率
- 定期清理和维护，保持缓存健康状态

通过本示例，您可以构建高性能、高可用的分布式缓存系统，显著提升应用响应速度和用户体验。

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用

- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构
- [observability](./observability.md): 可观测性示例，监控应用性能和健康状况
- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信