<div align="center">

# Caching Usage Examples

**Version: 0.1.4**

**Last modified date: 2026-01-15**

This example demonstrates how to use DMSC's cache module for multiple cache backends and advanced caching features.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- Memory cache and Redis cache usage
- Cache tags and atomic operations
- Distributed lock implementation
- Cache health checks and monitoring
- Data serialization and compression
- Error handling and fallback strategies

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of basic caching concepts
- (Optional) Redis server for Redis cache examples

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-cache-example
cd dms-cache-example
```

### 2. Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

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
  backend: "memory"  # or "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
  memory:
    max_size: 1000
    ttl: 3600
```

### 4. Write Main Code

Replace the contents of `src/main.rs` file with the following:

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_cache(DMSCCacheConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Cache Example started")?;
        
        // Basic cache operations examples
        basic_cache_operations(&ctx).await?;
        
        // Advanced cache features examples
        advanced_cache_features(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Cache Example completed")?;
        
        Ok(())
    }).await
}

async fn basic_cache_operations(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("cache", "Starting basic cache operations")?;
    
    // Set cache value
    ctx.cache().set("user:123", json!({
        "id": 123,
        "name": "John Doe",
        "email": "john@example.com"
    }), Some(Duration::from_secs(3600)))?;
    
    // Get cache value
    if let Some(user_data) = ctx.cache().get("user:123")? {
        ctx.logger().info("cache", &format!("Retrieved user: {}", user_data))?;
    }
    
    // Check if cache exists
    if ctx.cache().exists("user:123")? {
        ctx.logger().info("cache", "User cache exists")?;
    }
    
    // Delete cache
    ctx.cache().delete("user:123")?;
    ctx.logger().info("cache", "User cache deleted")?;
    
    Ok(())
}

async fn advanced_cache_features(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("cache", "Starting advanced cache features")?;
    
    // Use cache tags
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
    
    // Atomic increment operation
    let new_value = ctx.cache().increment("counter:page_views", 1)?;
    ctx.logger().info("cache", &format!("Page views: {}", new_value))?;
    
    // Get all keys by tag
    let rust_keys = ctx.cache().get_keys_by_tag("rust")?;
    ctx.logger().info("cache", &format!("Found {} items with 'rust' tag", rust_keys.len()))?;
    
    Ok(())
}
```

<div align="center">

## Code Analysis

</div>

## Basic Cache Operations

### Memory Cache

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create memory cache backend
let memory_backend = DMSCCacheBackend::Memory {
    max_size: 1000,
    ttl: Duration::from_secs(3600),
};

// Initialize cache module
let cache_config = DMSCCacheConfig {
    backend: memory_backend,
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,
    encryption: false,
};

ctx.cache().init(cache_config)?;

// Set cache value
ctx.cache().set("user:123", json!({
    "id": 123,
    "name": "John Doe",
    "email": "john@example.com"
}), Some(Duration::from_secs(3600)))?;

// Get cache value
if let Some(user_data) = ctx.cache().get("user:123")? {
    ctx.log().info(format!("Retrieved user: {}", user_data));
}

// Delete cache
ctx.cache().delete("user:123")?;

// Check if cache exists
if ctx.cache().exists("user:123")? {
    ctx.log().info("User cache exists");
}

// Get cache TTL
if let Some(ttl) = ctx.cache().ttl("user:123")? {
    ctx.log().info(format!("Cache expires in {} seconds", ttl.as_secs()));
}
```

### Redis Cache

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create Redis cache backend
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

// Use Redis cache
ctx.cache().set("session:abc123", json!({
    "user_id": 123,
    "role": "admin",
    "permissions": ["read", "write", "delete"],
    "login_time": "2024-01-15T10:30:00Z"
}), Some(Duration::from_secs(7200)))?;

// Batch operations
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

## Advanced Cache Features

### Cache Tags

```rust
use dmsc::prelude::*;
use serde_json::json;

// Set cache with tags
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

// Batch delete by tags
ctx.cache().delete_by_tags(vec!["rust"])?;

// Get all keys by tag
let rust_keys = ctx.cache().get_keys_by_tag("rust")?;
ctx.log().info(format!("Found {} items with 'rust' tag", rust_keys.len()));
```

### Atomic Operations

```rust
use dmsc::prelude::*;

// Atomic increment
let new_value = ctx.cache().increment("counter:page_views", 1)?;
ctx.log().info(format!("Page views: {}", new_value));

// Atomic decrement
let new_value = ctx.cache().decrement("counter:inventory:123", 5)?;
ctx.log().info(format!("Inventory: {}", new_value));

// Compare and set (CAS)
let success = ctx.cache().compare_and_set("config:feature_flag", "old_value", "new_value")?;
if success {
    ctx.log().info("Feature flag updated successfully");
}

// Get and set
let old_value = ctx.cache().get_and_set("session:last_activity", json!("2024-01-15T11:00:00Z"))?;
ctx.log().info(format!("Previous activity: {:?}", old_value));
```

### Distributed Lock

```rust
use dmsc::prelude::*;
use tokio::time::{sleep, Duration};

// Acquire distributed lock
let lock_key = "lock:resource:123";
let lock_value = "worker-1";
let ttl = Duration::from_secs(30);

match ctx.cache().acquire_lock(lock_key, lock_value, ttl)? {
    Some(lock) => {
        ctx.log().info("Lock acquired successfully");
        
        // Execute critical section operations
        perform_critical_operation().await?;
        
        // Release lock
        ctx.cache().release_lock(lock_key, lock_value)?;
        ctx.log().info("Lock released");
    }
    None => {
        ctx.log().warn("Failed to acquire lock");
        return Err(DMSCError::resource_busy("Resource is locked"));
    }
}

// Convenient method to use lock
ctx.cache().with_lock("lock:report_generation", "worker-1", Duration::from_secs(60), || async {
    ctx.log().info("Generating report...");
    generate_report().await?;
    ctx.log().info("Report generated successfully");
    Ok(())
}).await?;
```

## Cache Strategies

### Cache Penetration Protection

```rust
use dmsc::prelude::*;
use serde_json::json;

// Get user data with cache penetration protection
async fn get_user_with_cache_through_protection(user_id: u64) -> DMSCResult<Option<Value>> {
    let cache_key = format!("user:{}", user_id);
    
    // Try to get from cache first
    if let Some(cached_data) = ctx.cache().get(&cache_key)? {
        return Ok(Some(cached_data));
    }
    
    // Check bloom filter (prevent cache penetration)
    if !ctx.cache().bloom_filter_might_contain("users", &user_id.to_string())? {
        ctx.log().info(format!("User {} definitely does not exist", user_id));
        return Ok(None);
    }
    
    // Get from database
    match fetch_user_from_database(user_id).await? {
        Some(user_data) => {
            // Set cache
            ctx.cache().set(&cache_key, user_data.clone(), Some(Duration::from_secs(3600)))?;
            Ok(Some(user_data))
        }
        None => {
            // Set empty value cache (prevent cache penetration)
            ctx.cache().set(&cache_key, json!(null), Some(Duration::from_secs(300)))?;
            Ok(None)
        }
    }
}

// Use bloom filter
ctx.cache().bloom_filter_add("users", "123")?;
ctx.cache().bloom_filter_add("users", "456")?;

if ctx.cache().bloom_filter_might_contain("users", "123")? {
    ctx.log().info("User 123 might exist");
}
```

### Cache Avalanche Protection

```rust
use dmsc::prelude::*;
use serde_json::json;
use rand::Rng;

// Add random TTL offset when setting cache to prevent cache avalanche
fn set_cache_with_jitter(key: &str, value: Value, base_ttl: Duration) -> DMSCResult<()> {
    let jitter = rand::thread_rng().gen_range(0..300); // 0-5 minutes random offset
    let ttl = base_ttl + Duration::from_secs(jitter);
    
    ctx.cache().set(key, value, Some(ttl))?;
    Ok(())
}

// Use different TTL when batch setting cache
let articles = vec![
    ("article:1", json!({"id": 1, "title": "Article 1"}), Duration::from_secs(3600)),
    ("article:2", json!({"id": 2, "title": "Article 2"}), Duration::from_secs(3600 + 60)),
    ("article:3", json!({"id": 3, "title": "Article 3"}), Duration::from_secs(3600 + 120)),
];

for (key, value, ttl) in articles {
    set_cache_with_jitter(key, value, ttl)?;
}
```

### Cache Preheating

```rust
use dmsc::prelude::*;
use serde_json::json;

// Preheat hot data when system starts
async fn warmup_cache() -> DMSCResult<()> {
    ctx.log().info("Starting cache warmup...");
    
    // Preheat popular articles
    let popular_articles = fetch_popular_articles().await?;
    for article in popular_articles {
        let key = format!("article:{}", article["id"]);
        ctx.cache().set(&key, article, Some(Duration::from_secs(3600)))?;
    }
    
    // Preheat user sessions
    let active_sessions = fetch_active_sessions().await?;
    for session in active_sessions {
        let key = format!("session:{}", session["id"]);
        ctx.cache().set(&key, session, Some(Duration::from_secs(7200)))?;
    }
    
    // Preheat configuration data
    let config_data = fetch_configuration().await?;
    ctx.cache().set("config:app", config_data, Some(Duration::from_secs(86400)))?;
    
    ctx.log().info("Cache warmup completed");
    Ok(())
}
```

## Cache Monitoring

### Cache Statistics

```rust
use dmsc::prelude::*;

// Get cache statistics
let stats = ctx.cache().get_stats()?;
ctx.log().info(format!("Cache stats: {:?}", stats));

// Monitor hit rate for specific keys
let key_stats = ctx.cache().get_key_stats("user:123")?;
ctx.log().info(format!("Key stats: {:?}", key_stats));

// Get cache size
let cache_size = ctx.cache().get_cache_size()?;
ctx.log().info(format!("Cache size: {} bytes", cache_size));

// Clean expired cache
let cleaned_count = ctx.cache().cleanup_expired()?;
ctx.log().info(format!("Cleaned {} expired entries", cleaned_count));
```

### Cache Health Check

```rust
use dmsc::prelude::*;
use serde_json::json;

// Check cache health status
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

// Periodic health check
ctx.observability().register_health_check("cache", || async {
    match check_cache_health() {
        Ok(health) => Ok(health),
        Err(e) => Err(DMSCError::internal(format!("Cache health check failed: {}", e))),
    }
}).await?;
```

## Serialization and Compression

### Data Serialization

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

// Store serialized data
let user = User {
    id: 123,
    name: "John Doe".to_string(),
    email: "john@example.com".to_string(),
    role: "admin".to_string(),
    created_at: "2024-01-15T10:30:00Z".to_string(),
};

// Use MessagePack serialization (more compact)
let cache_config = DMSCCacheConfig {
    backend: DMSCCacheBackend::Memory {
        max_size: 1000,
        ttl: Duration::from_secs(3600),
    },
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,  // Enable compression
    encryption: false,
    serializer: DMSCSerializer::MessagePack, // Use MessagePack
};

ctx.cache().init(cache_config)?;
ctx.cache().set("user:123", json!(user), None)?;

// Get and deserialize
if let Some(cached_data) = ctx.cache().get("user:123")? {
    let cached_user: User = serde_json::from_value(cached_data)?;
    ctx.log().info(format!("Retrieved user: {:?}", cached_user));
}
```

### Data Compression

```rust
use dmsc::prelude::*;
use serde_json::json;

// Enable compression when storing large data
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

// Enable compression storage
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
    compression: true,  // Enable compression
    compression_threshold: 1024, // Only compress data larger than 1KB
    compression_level: 6, // Compression level (1-9)
    encryption: false,
};

ctx.cache().init(cache_config)?;

let start = std::time::Instant::now();
ctx.cache().set("large_dataset", large_data.clone(), None)?;
let store_time = start.elapsed();

ctx.log().info(format!("Stored large dataset in {:?}", store_time));

// Verify compression effect
let uncompressed_size = serde_json::to_vec(&large_data)?.len();
ctx.log().info(format!("Uncompressed size: {} bytes", uncompressed_size));
```

## Error Handling

### Cache Error Handling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Handle cache errors
match ctx.cache().set("key", json!("value"), None) {
    Ok(_) => ctx.log().info("Cache set successfully"),
    Err(DMSCError::CacheConnectionError(e)) => {
        ctx.log().error(format!("Cache connection failed: {}", e));
        // Fallback to database or other backup storage
        fallback_to_database("key", json!("value")).await?;
    }
    Err(DMSCError::CacheTimeoutError(e)) => {
        ctx.log().warn(format!("Cache operation timed out: {}", e));
        // Retry or fallback
        retry_cache_operation().await?;
    }
    Err(DMSCError::CacheFullError) => {
        ctx.log().warn("Cache is full");
        // Clean expired cache or increase capacity
        ctx.cache().cleanup_expired()?;
    }
    Err(e) => {
        ctx.log().error(format!("Cache error: {}", e));
        return Err(e);
    }
}

// Cache fallback strategy
async fn get_data_with_fallback(key: &str) -> DMSCResult<Value> {
    // Try cache first
    if let Ok(Some(cached)) = ctx.cache().get(key) {
        return Ok(cached);
    }
    
    // Cache failed, try database
    match fetch_from_database(key).await {
        Ok(data) => {
            // Update cache asynchronously (doesn't block main flow)
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

## Running Steps

</div>

### 1. Environment Preparation

Ensure the following components are installed:
- Rust 1.65+ and Cargo
- (Optional) Redis server (for Redis cache examples)

### 2. Create Project

```bash
cargo new dms-cache-example
cd dms-cache-example
```

### 3. Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### 4. Create Configuration

Create `config.yaml` in the project root directory:

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
  backend: "memory"  # or "redis"
  redis:
    url: "redis://localhost:6379"
    pool_size: 10
  memory:
    max_size: 1000
    ttl: 3600
```

### 5. Run Example

```bash
cargo run
```

<div align="center">

## Expected Results

</div>

After successful execution, you will see output similar to the following:

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

## Extended Features

</div>

### Cache Cluster Support

```rust
use dmsc::prelude::*;

// Configure multi-node Redis cluster
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
    read_from_replicas: true, // Read from replicas to improve performance
};

let cache_config = DMSCCacheConfig {
    backend: cluster_backend,
    default_ttl: Duration::from_secs(1800),
    key_prefix: "app".to_string(),
    compression: true,
    encryption: false,
};

ctx.cache().init(cache_config)?;

// Cluster-aware routing
ctx.cache().set_with_shard_key("user:123", json!({"name": "John"}), Some(Duration::from_secs(3600)), "shard_1")?;
```

### Intelligent Cache Preheating

```rust
use dmsc::prelude::*;
use machine_learning::cache::CachePredictor;

// Predict cache requirements based on machine learning
async fn intelligent_cache_warmup() -> DMSCResult<()> {
    let predictor = CachePredictor::new();
    
    // Analyze historical access patterns
    let access_patterns = ctx.cache().get_access_history(30)?; // 30 days historical data
    let predictions = predictor.predict_hot_data(&access_patterns)?;
    
    // Preheat predicted hot data
    for predicted_key in predictions.hot_keys {
        if let Ok(data) = fetch_data_from_source(&predicted_key).await {
            ctx.cache().set(&predicted_key, data, Some(Duration::from_secs(3600)))?;
        }
    }
    
    // Preheat based on time patterns
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

### Adaptive Cache Strategy

```rust
use dmsc::prelude::*;
use std::collections::HashMap;

// Dynamically adjust cache strategy
pub struct AdaptiveCacheManager {
    hit_rate_threshold: f64,
    miss_rate_threshold: f64,
    adjustment_interval: Duration,
}

impl AdaptiveCacheManager {
    pub async fn optimize_cache_strategy(&self) -> DMSCResult<()> {
        let stats = ctx.cache().get_detailed_stats()?;
        
        // Analyze cache performance metrics
        let hit_rate = stats.hit_rate;
        let miss_rate = stats.miss_rate;
        let avg_response_time = stats.avg_response_time;
        
        // Dynamically adjust TTL
        if hit_rate < self.hit_rate_threshold {
            // Low hit rate, increase TTL
            ctx.cache().adjust_global_ttl(Duration::from_secs(3600))?;
        } else if miss_rate > self.miss_rate_threshold {
            // High miss rate, decrease TTL and optimize prefetch
            ctx.cache().adjust_global_ttl(Duration::from_secs(900))?;
            self.optimize_prefetch_strategy().await?;
        }
        
        // Dynamically adjust cache size
        if stats.memory_usage > 0.8 {
            // High memory usage, enable more aggressive cleanup policy
            ctx.cache().set_eviction_policy(DMSCEvictionPolicy::LFU)?;
        } else {
            ctx.cache().set_eviction_policy(DMSCEvictionPolicy::LRU)?;
        }
        
        // Adjust compression strategy
        if avg_response_time > Duration::from_millis(100) {
            ctx.cache().enable_compression(true)?;
            ctx.cache().set_compression_threshold(512)?; // Lower compression threshold
        }
        
        Ok(())
    }
    
    async fn optimize_prefetch_strategy(&self) -> DMSCResult<()> {
        // Optimize prefetch strategy based on access patterns
        let patterns = ctx.cache().analyze_access_patterns()?;
        
        for pattern in patterns {
            if pattern.confidence > 0.8 {
                // High confidence pattern, prefetch related data
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

### Distributed Cache Consistency

```rust
use dmsc::prelude::*;
use distributed_consensus::raft::RaftNode;

// Implement distributed cache consistency
pub struct DistributedCacheConsistency {
    raft_node: RaftNode,
    cache_instances: Vec<String>,
}

impl DistributedCacheConsistency {
    pub async fn maintain_consistency(&self) -> DMSCResult<()> {
        // Use Raft protocol to ensure cache update consistency
        let update_command = CacheUpdateCommand {
            key: "user:123".to_string(),
            value: json!({"name": "John", "updated": chrono::Utc::now()}),
            ttl: Duration::from_secs(3600),
        };
        
        // Replicate update through Raft protocol
        match self.raft_node.propose(update_command).await? {
            ConsensusResult::Committed => {
                // Update committed, apply to all nodes
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

## Best Practices

</div>

1. **Choose appropriate cache backend**: Memory cache is suitable for single-machine applications, Redis is suitable for distributed environments
2. **Set reasonable TTL**: Set cache expiration time based on data update frequency
3. **Use cache tags**: Facilitate batch management and cleanup of related caches
4. **Implement cache degradation**: Have fallback solutions when cache is unavailable
5. **Monitor cache performance**: Regularly check hit rate, response time and other metrics
6. **Prevent cache penetration**: Use bloom filters or empty value caching
7. **Avoid cache avalanche**: Add random offset to TTL
8. **Serialize reasonably**: Choose efficient serialization formats like MessagePack
9. **Compress large data**: Compress large data to save storage space
10. **Clean regularly**: Clean expired caches and release resources
11. **Use distributed locks**: Ensure mutual exclusion of critical operations
12. **Implement cache preheating**: Load hot data during system startup
13. **Monitor memory usage**: Avoid cache occupying too much memory
14. **Configure connection pool**: Set reasonable connection pool size and timeout
15. **Implement consistent hashing**: Maintain balanced data distribution in distributed environments

<div align="center">

## Summary

</div>

This example comprehensively demonstrates the core functionality and advanced features of the DMSC cache module, covering the following key capabilities:

### 🚀 Core Features
- **Multi-backend Support**: Seamless integration of memory cache, Redis cache, and Redis cluster
- **Basic Cache Operations**: Basic functions like set, get, delete, and existence check
- **Advanced Cache Features**: Tag management, atomic operations, distributed lock implementation
- **Cache Strategies**: Penetration protection, avalanche protection, preheating mechanisms
- **Serialization and Compression**: Support for multiple serialization formats and data compression
- **Health Monitoring**: Real-time cache status monitoring and performance statistics

### 🔧 Advanced Features
- **Cache Cluster**: Multi-node Redis cluster support and intelligent routing
- **Intelligent Preheating**: Machine learning-based cache prediction and preheating
- **Adaptive Strategy**: Dynamic adjustment of TTL, compression, and cleanup strategies
- **Distributed Consistency**: Using Raft protocol to ensure cache consistency
- **Performance Optimization**: Connection pool management, batch operations, asynchronous processing
- **Error Handling**: Comprehensive degradation strategies and exception handling

### 💡 Best Practices
- Choose appropriate cache backend and optimize configuration based on application scenarios
- Set reasonable TTL to balance data freshness and cache hit rate
- Use cache tags for convenient batch management and cleanup
- Implement cache degradation to ensure system high availability
- Monitor key metrics and continuously optimize cache performance
- Prevent cache penetration and avalanche to ensure system stability
- Configure serialization and compression reasonably to optimize storage efficiency
- Clean and maintain regularly to keep cache healthy

Through this example, you can build high-performance, highly available distributed cache systems, significantly improving application response speed and user experience.

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication and authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication

- [database](./database.md): Database examples, learn database connection and query operations
- [http](./http.md): HTTP service examples, build web applications and RESTful APIs
- [mq](./mq.md): Message queue examples, implement asynchronous message processing and event-driven architecture
- [observability](./observability.md): Observability examples, monitor application performance and health status
- [security](./security.md): Security examples, encryption, hashing and security best practices
- [storage](./storage.md): Storage examples, file upload/download and storage management
- [validation](./validation.md): Validation examples, data validation and cleanup operations