<div align="center">

# Config API参考

**Version: 0.1.7**

**Last modified date: 2026-02-11**

config模块提供多源配置管理与热重载功能，支持文件、环境变量等多种配置来源。

## 模块概述

</div>

config模块包含以下子模块：

- **core**: 配置核心接口和类型定义
- **sources**: 配置源实现（文件、环境变量等）
- **validators**: 配置验证器
- **reload**: 配置热重载机制

<div align="center">

## 核心组件

</div>

### DMSCConfig

配置管理器主接口，提供统一的配置访问。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的配置 | 无 | `DMSCConfig` |
| `get(key)` | 获取配置值 | `key: &str` | `String` 或 `None` |
| `get_str(key)` | 获取字符串值 | `key: &str` | `&str` 或 `None` |
| `get_bool(key)` | 获取布尔值 | `key: &str` | `Option<bool>` |
| `get_i64(key)` | 获取 i64 值 | `key: &str` | `Option<i64>` |
| `get_u64(key)` | 获取 u64 值 | `key: &str` | `Option<u64>` |
| `get_f32(key)` | 获取 f32 值 | `key: &str` | `Option<f32>` |
| `get_f64(key)` | 获取 f64 值 | `key: &str` | `Option<f64>` |
| `get_usize(key)` | 获取 usize 值 | `key: &str` | `Option<usize>` |
| `get_i32(key)` | 获取 i32 值 | `key: &str` | `Option<i32>` |
| `get_u32(key)` | 获取 u32 值 | `key: &str` | `Option<u32>` |
| `set(key, value)` | 设置配置值 | `key: &str`, `value: &str` | 无 |
| `has_key(key)` | 检查键是否存在 | `key: &str` | `bool` |
| `keys()` | 获取所有键 | 无 | `Vec<&str>` |
| `all_values()` | 获取所有值 | 无 | `Vec<&str>` |
| `count()` | 获取配置数量 | 无 | `usize` |
| `merge(other)` | 合并配置 | `other: &DMSCConfig` | 无 |
| `clear()` | 清空配置 | 无 | 无 |

#### 使用示例

```rust
use dmsc::config::DMSCConfig;

// 创建配置
let config = DMSCConfig::new();

// 设置配置
config.set("service.port", "8080");
config.set("database.url", "postgres://localhost/mydb");

// 获取配置
let port = config.get("service.port");
let url = config.get("database.url");

// 检查配置
if config.contains("service.host") {
    let host = config.get("service.host");
}

// 获取所有键
let keys = config.keys();
for key in &keys {
    println!("Config key: {}", key);
}

// 合并配置
let other = DMSCConfig::new();
other.set("additional", "value");
config.merge(&other);
```

### DMSCConfigManager

配置管理器，提供多源配置管理。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的配置管理器 | 无 | `DMSCConfigManager` |
| `add_file_source(path)` | 添加文件配置源 | `path: &str` | 无 |
| `add_environment_source()` | 添加环境变量源 | 无 | 无 |
| `get(key)` | 获取配置值 | `key: &str` | `String` 或 `None` |

### DMSCConfigSource

配置源枚举类型。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `File(path)` | 文件配置源 |
| `Env(prefix)` | 环境变量配置源 |
| `Http(url)` | HTTP配置源 |
| `Database(connection)` | 数据库配置源 |
| `Custom(name, data)` | 自定义配置源 |

### DMSCConfigBuilder

配置构建器，用于构建配置管理器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的配置构建器 | 无 | `Self` |
| `add_source(source)` | 添加配置源 | `source: DMSCConfigSource` | `Self` |
| `set_default(key, value)` | 设置默认值 | `key: &str`, `value: impl Serialize` | `Self` |
| `add_validator(validator)` | 添加验证器 | `validator: impl ConfigValidator` | `Self` |
| `enable_hot_reload()` | 启用热重载 | 无 | `Self` |
| `set_reload_interval(seconds)` | 设置重载间隔 | `seconds: u64` | `Self` |
| `build()` | 构建配置管理器 | 无 | `DMSCResult<DMSCConfig>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))
    .set_default("service.port", 8080)
    .set_default("service.host", "localhost")
    .enable_hot_reload()
    .set_reload_interval(60)
    .build()?;
```

## 配置源

### 文件配置

#### YAML文件

```yaml
# config.yaml
service:
  name: "my-service"
  version: "1.0.0"
  port: 8080
  host: "0.0.0.0"

database:
  url: "postgres://localhost/mydb"
  max_connections: 100
  timeout: 30

logging:
  level: "info"
  file_format: "json"
```

#### JSON文件

```json
{
  "service": {
    "name": "my-service",
    "version": "1.0.0",
    "port": 8080,
    "host": "0.0.0.0"
  },
  "database": {
    "url": "postgres://localhost/mydb",
    "max_connections": 100,
    "timeout": 30
  }
}
```

#### TOML文件

```toml
[service]
name = "my-service"
version = "1.0.0"
port = 8080
host = "0.0.0.0"

[database]
url = "postgres://localhost/mydb"
max_connections = 100
timeout = 30
```

### 环境变量配置

```bash
# 基本环境变量
export SERVICE_NAME=my-service
export SERVICE_PORT=8080
export DATABASE_URL=postgres://localhost/mydb

# 带前缀的环境变量
export DMSC_SERVICE_NAME=my-service
export DMSC_SERVICE_PORT=8080
export DMSC_DATABASE_URL=postgres://localhost/mydb
```

### 配置优先级

配置源的优先级从高到低：

1. **环境变量** (最高优先级)
2. **配置文件** (中等优先级)
3. **默认值** (最低优先级)

```rust
let config = DMSCConfigBuilder::new()
    .set_default("service.port", 3000)                    // 默认值
    .add_source(DMSCConfigSource::File("config.yaml".to_string())) // 配置文件
    .add_source(DMSCConfigSource::Env("DMSC".to_string()))        // 环境变量
    .build()?;

// 优先级：环境变量 > 配置文件 > 默认值
```
<div align="center">

## 类型安全的配置访问

</div>

### 基本类型

```rust
// 字符串
let name: String = ctx.config().get_typed("service.name")?;

// 数字类型
let port: u16 = ctx.config().get_typed("service.port")?;
let max_connections: usize = ctx.config().get_typed("database.max_connections")?;
let timeout: f64 = ctx.config().get_typed("service.timeout")?;

// 布尔类型
let debug_mode: bool = ctx.config().get_typed("service.debug")?;
let enable_feature: bool = ctx.config().get_typed("feature.enabled")?;
```

### 复杂类型

```rust
#[derive(Debug, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: usize,
    timeout: u64,
}

// 获取结构体配置
let db_config: DatabaseConfig = ctx.config().get_typed("database")?;

// 获取数组配置
let allowed_hosts: Vec<String> = ctx.config().get_typed("security.allowed_hosts")?;

// 获取映射配置
let feature_flags: HashMap<String, bool> = ctx.config().get_typed("features")?;
```

### 可选类型

```rust
// 获取可选配置
let port: Option<u16> = ctx.config().get_typed("service.port").ok();
let timeout: Option<u64> = ctx.config().get_typed("service.timeout").ok();

// 使用unwrap_or提供默认值
let port = ctx.config().get_typed("service.port").unwrap_or(8080);
let timeout = ctx.config().get_typed("service.timeout").unwrap_or(30);
```
<div align="center">

## 配置验证

</div>  

### DMSCConfigValidator

配置验证器，用于验证配置的完整性和有效性。

```rust
use dmsc::config::DMSCConfigValidator;

let mut validator = DMSCConfigValidator::new();
validator.add_required("service.name".to_string());
validator.add_port_check("service.port".to_string());
validator.add_timeout_check("server.timeout".to_string());
validator.add_secret_check("auth.jwt.secret".to_string());
validator.add_url_check("database.url".to_string());
validator.add_positive_int_check("pool.size".to_string());

let config = DMSCConfigManager::new().config();
validator.validate_config(&config)?;
```

#### 方法

| 方法 | 描述 |
|:--------|:-------------|
| `add_required(key)` | 添加必需配置项检查 |
| `add_port_check(key)` | 添加端口号验证 (1-65535) |
| `add_timeout_check(key)` | 添加超时时间验证 (1-86400秒) |
| `add_secret_check(key)` | 添加密钥强度检查 (最小8字符) |
| `add_url_check(key)` | 添加URL格式验证 |
| `add_positive_int_check(key)` | 添加正整数验证 |
| `validate_config(config)` | 验证配置完整性 |

<div align="center">

## 热重载

</div>  

### 启用热重载

```rust
let config = DMSCConfigBuilder::new()
    .add_source(DMSCConfigSource::File("config.yaml".to_string()))
    .enable_hot_reload()
    .set_reload_interval(60) // 每60秒检查一次
    .build()?;

// 手动触发重载
config.reload().await?;
```

### 监听配置变化

```rust
// 监听特定配置变化
config.watch("service.port", |new_value| {
    println!("Port changed to: {}", new_value);
    // 重启服务或更新配置
}).await?;

// 监听所有配置变化
config.watch_all(|changes| {
    for (key, old_value, new_value) in changes {
        println!("Config {} changed from {:?} to {:?}", key, old_value, new_value);
    }
}).await?;
```

<div align="center">

## 配置模板

</div>  

### 环境特定配置

```rust
// config.dev.yaml
service:
  name: "my-service-dev"
  port: 3000
  debug: true

// config.prod.yaml  
service:
  name: "my-service"
  port: 8080
  debug: false
```

### 配置继承

```yaml
# base.yaml
service:
  name: "my-service"
  version: "1.0.0"

# config.yaml
import: "base.yaml"
service:
  port: 8080  # 覆盖基础配置
```
<div align="center">

## 最佳实践

</div>

### 配置错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `CONFIG_FILE_NOT_FOUND` | 配置文件未找到 |
| `CONFIG_PARSE_ERROR` | 配置解析错误 |
| `CONFIG_VALIDATION_FAILED` | 配置验证失败 |
| `CONFIG_TYPE_ERROR` | 配置类型错误 |
| `CONFIG_SOURCE_ERROR` | 配置源错误 |

### 错误处理示例

```rust
match ctx.config().get_typed::<u16>("service.port") {
    Ok(port) => {
        // 配置正确
        println!("Service port: {}", port);
    }
    Err(DMSCError { code, .. }) if code == "CONFIG_TYPE_ERROR" => {
        // 类型错误，使用默认值
        let port: u16 = 8080;
        println!("Using default port: {}", port);
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

1. **使用类型安全的配置访问**: 避免手动类型转换
2. **提供合理的默认值**: 确保应用在缺少配置时能正常运行
3. **验证配置完整性**: 在应用启动时验证所有必需的配置
4. **使用环境变量覆盖**: 在不同环境中使用环境变量覆盖配置文件
5. **启用热重载**: 对于需要动态调整的配置启用热重载
6. **加密敏感信息**: 对密码、密钥等敏感信息进行加密
7. **使用配置模板**: 为不同环境创建配置模板
8. **记录配置变化**: 监听和记录配置变化，便于审计
9. **注意配置修改的安全时机**:
   - **启动阶段**: 所有配置都可以安全修改
   - **运行阶段**: 只有标记为"可动态修改"的配置才能安全修改
   - **敏感模块**: 网关、认证、服务网格等核心模块的配置修改需要特别谨慎
   - **重启要求**: 某些配置修改后需要重启服务才能生效

<div align="center">

## 配置修改的安全时机

</div>  

### 可安全动态修改的配置

以下类型的配置通常可以安全地在运行时修改：

- **日志级别**: 可以动态调整日志输出级别
- **监控配置**: 可以动态调整监控采样率和告警阈值
- **超时设置**: 可以动态调整请求超时时间
- **限流配置**: 可以动态调整速率限制
- **缓存配置**: 可以动态调整缓存大小和TTL
- **功能开关**: 可以动态启用或禁用功能

### 需要谨慎修改的配置

以下类型的配置修改需要特别谨慎，可能影响系统稳定性：

- **认证配置**: 可能导致用户无法登录或权限失效
- **数据库配置**: 可能导致数据库连接中断
- **网络配置**: 可能导致服务间通信中断
- **安全配置**: 可能导致安全漏洞
- **核心组件配置**: 可能导致系统崩溃

### 修改配置的最佳时机

1. **应用启动前**: 修改所有需要重启才能生效的配置
2. **低峰期**: 在系统负载较低时修改配置
3. **逐步修改**: 对于关键配置，先在非生产环境测试，然后逐步推广到生产环境
4. **监控修改**: 修改配置后密切监控系统指标
5. **回滚机制**: 准备好配置回滚方案，以便在出现问题时快速恢复

### 配置修改的影响范围

| 配置类型 | 影响范围 | 是否需要重启 |
|:--------|:-------------|:--------|
| 日志级别 | 全局 | 否 |
| 监控配置 | 全局 | 否 |
| 超时设置 | 局部 | 否 |
| 限流配置 | 局部 | 否 |
| 缓存配置 | 局部 | 否 |
| 功能开关 | 局部/全局 | 否 |
| 认证配置 | 全局 | 是 |
| 数据库配置 | 全局 | 是 |
| 网络配置 | 全局 | 是 |
| 安全配置 | 全局 | 是 |
| 核心组件配置 | 全局 | 是 |

<div align="center">

## 配置文件参考

</div>

本节提供DMSC应用程序中使用的所有配置结构的详细文档。

### 完整配置文件示例

```yaml
# config.yaml - 完整DMSC配置文件

# =============================================================================
# 认证配置
# =============================================================================
auth:
  enabled: true                          # 启用认证
  jwt_secret: "your-secret-key"          # JWT密钥（生产环境请使用环境变量）
  jwt_expiry_secs: 3600                  # JWT令牌过期时间（1小时）
  session_timeout_secs: 86400            # 会话超时（24小时）
  oauth_providers: []                    # OAuth提供商：["google", "github"]
  enable_api_keys: true                  # 启用API密钥认证
  enable_session_auth: true              # 启用会话认证
  oauth_cache_backend_type: "Memory"     # 缓存后端：Memory, Redis, Hybrid
  oauth_cache_redis_url: "redis://127.0.0.1:6379"

# =============================================================================
# 缓存配置
# =============================================================================
cache:
  enabled: true                          # 启用缓存
  default_ttl_secs: 3600                 # 默认TTL（1小时）
  max_memory_mb: 512                     # 最大内存使用（MB）
  cleanup_interval_secs: 300             # 清理间隔（5分钟）
  backend_type: "Memory"                 # 后端类型：Memory, Redis, Hybrid
  redis_url: "redis://127.0.0.1:6379"    # Redis连接URL
  redis_pool_size: 10                    # Redis连接池大小

# =============================================================================
# 日志配置
# =============================================================================
logging:
  level: "INFO"                          # 日志级别：DEBUG, INFO, WARN, ERROR
  console_enabled: true                  # 启用控制台输出
  file_enabled: true                     # 启用文件输出
  sampling_default: 1.0                  # 采样率（0.0-1.0）
  file_name: "app.log"                   # 日志文件名
  json_format: false                     # 使用JSON格式
  rotate_when: "size"                    # 轮转触发：size 或 none
  max_bytes: 10485760                    # 轮转前最大文件大小（10MB）
  color_blocks: true                     # 使用彩色块

# =============================================================================
# 网关配置
# =============================================================================
gateway:
  listen_address: "0.0.0.0"              # 监听地址
  listen_port: 8080                      # 监听端口
  max_connections: 1000                  # 最大并发连接数
  request_timeout_seconds: 30            # 请求超时
  enable_rate_limiting: true             # 启用限流
  enable_circuit_breaker: true           # 启用熔断器
  enable_load_balancing: true            # 启用负载均衡
  cors_enabled: true                     # 启用CORS
  cors_origins: ["*"]                    # 允许的来源
  cors_methods: ["GET", "POST", "PUT", "DELETE"]
  cors_headers: ["Content-Type", "Authorization"]
  enable_logging: true                   # 启用请求日志
  log_level: "INFO"                      # 网关日志级别

# =============================================================================
# 数据库配置
# =============================================================================
database:
  database_type: "Postgres"              # 数据库类型：Postgres, MySQL, SQLite
  host: "localhost"                      # 数据库主机
  port: 5432                             # 数据库端口
  database: "mydb"                       # 数据库名称
  username: "user"                       # 数据库用户名
  password: "password"                   # 数据库密码
  max_connections: 10                    # 最大连接数
  min_idle_connections: 1                # 最小空闲连接数
  connection_timeout_secs: 30            # 连接超时
  idle_timeout_secs: 600                 # 空闲超时（10分钟）
  max_lifetime_secs: 1800                # 最大连接生命周期（30分钟）
  ssl_mode: "Prefer"                     # SSL模式：Disable, Prefer, Require

# =============================================================================
# 队列配置
# =============================================================================
queue:
  enabled: true                          # 启用队列
  backend_type: "Memory"                 # 后端类型：Memory, RabbitMQ, Kafka, Redis
  connection_string: "memory://localhost"
  max_connections: 10                    # 最大连接数
  message_max_size: 1048576              # 最大消息大小（1MB）
  consumer_timeout_ms: 30000             # 消费者超时（30秒）
  producer_timeout_ms: 5000              # 生产者超时（5秒）
  retry_policy:
    max_retries: 3                       # 最大重试次数
    initial_delay_ms: 100                # 初始延迟
    max_delay_ms: 5000                   # 最大延迟
    multiplier: 2.0                      # 延迟乘数
  dead_letter_config:
    enabled: true                        # 启用死信队列
    queue_name: "dead_letter"            # 死信队列名称
    max_retention_secs: 86400            # 保留时间（24小时）

# =============================================================================
# 可观测性配置
# =============================================================================
observability:
  tracing_enabled: true                  # 启用分布式追踪
  metrics_enabled: true                  # 启用指标收集
  tracing_sampling_rate: 0.1             # 采样率（10%）
  tracing_sampling_strategy: "rate"      # 策略：rate, probabilistic
  metrics_window_size_secs: 300          # 指标窗口（5分钟）
  metrics_bucket_size_secs: 10           # 桶大小（10秒）
```

### DMSCAuthConfig

认证配置，用于JWT、OAuth和会话管理。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | 启用认证 |
| `jwt_secret` | `String` | 自动生成 | JWT令牌密钥 |
| `jwt_expiry_secs` | `u64` | `3600` | JWT令牌过期时间（秒） |
| `session_timeout_secs` | `u64` | `86400` | 会话超时时间（秒） |
| `oauth_providers` | `Vec<String>` | `[]` | OAuth提供商列表 |
| `enable_api_keys` | `bool` | `true` | 启用API密钥认证 |
| `enable_session_auth` | `bool` | `true` | 启用会话认证 |
| `oauth_cache_backend_type` | `DMSCCacheBackendType` | `Memory` | OAuth令牌缓存后端 |
| `oauth_cache_redis_url` | `String` | `"redis://127.0.0.1:6379"` | OAuth缓存的Redis URL |

**环境变量：**
- `DMSC_JWT_SECRET`: 覆盖JWT密钥（生产环境推荐使用）

### DMSCCacheConfig

缓存系统配置，用于内存和Redis后端。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | 启用缓存 |
| `default_ttl_secs` | `u64` | `3600` | 默认TTL（秒） |
| `max_memory_mb` | `u64` | `512` | 最大内存（MB） |
| `cleanup_interval_secs` | `u64` | `300` | 清理间隔（秒） |
| `backend_type` | `DMSCCacheBackendType` | `Memory` | 缓存后端类型 |
| `redis_url` | `String` | `"redis://127.0.0.1:6379"` | Redis连接URL |
| `redis_pool_size` | `usize` | `10` | Redis连接池大小 |

**DMSCCacheBackendType 取值：**
- `Memory`: 内存缓存（快速，非持久化）
- `Redis`: Redis缓存（持久化，分布式）
- `Hybrid`: 内存+Redis（性能与持久化兼顾）

### DMSCLogConfig

日志配置，用于控制台和文件输出。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `level` | `DMSCLogLevel` | `INFO` | 最低日志级别 |
| `console_enabled` | `bool` | `true` | 启用控制台输出 |
| `file_enabled` | `bool` | `true` | 启用文件输出 |
| `sampling_default` | `f32` | `1.0` | 默认采样率（0.0-1.0） |
| `file_name` | `String` | `"app.log"` | 日志文件名 |
| `json_format` | `bool` | `false` | 使用JSON格式 |
| `rotate_when` | `String` | `"size"` | 轮转触发："size" 或 "none" |
| `max_bytes` | `u64` | `10485760` | 轮转前最大文件大小 |
| `color_blocks` | `bool` | `true` | 使用彩色块输出 |

**DMSCLogLevel 取值：**
- `DEBUG`: 调试级别消息
- `INFO`: 信息级别消息
- `WARN`: 警告级别消息
- `ERROR`: 错误级别消息

### DMSCGatewayConfig

API网关配置，用于HTTP路由和CORS。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `listen_address` | `String` | `"0.0.0.0"` | 监听地址 |
| `listen_port` | `u16` | `8080` | 监听端口 |
| `max_connections` | `usize` | `1000` | 最大并发连接数 |
| `request_timeout_seconds` | `u64` | `30` | 请求超时（秒） |
| `enable_rate_limiting` | `bool` | `true` | 启用限流 |
| `enable_circuit_breaker` | `bool` | `true` | 启用熔断器 |
| `enable_load_balancing` | `bool` | `true` | 启用负载均衡 |
| `cors_enabled` | `bool` | `true` | 启用CORS |
| `cors_origins` | `Vec<String>` | `["*"]` | 允许的CORS来源 |
| `cors_methods` | `Vec<String>` | `["GET", "POST", ...]` | 允许的CORS方法 |
| `cors_headers` | `Vec<String>` | `["Content-Type", ...]` | 允许的CORS头 |
| `enable_logging` | `bool` | `true` | 启用请求日志 |
| `log_level` | `String` | `"INFO"` | 网关日志级别 |

### DMSCDatabaseConfig

数据库连接配置，用于SQL数据库。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `database_type` | `DatabaseType` | `Postgres` | 数据库后端类型 |
| `host` | `String` | `"localhost"` | 数据库主机 |
| `port` | `u16` | `5432` | 数据库端口 |
| `database` | `String` | 必填 | 数据库名称 |
| `username` | `String` | 必填 | 数据库用户名 |
| `password` | `String` | 必填 | 数据库密码 |
| `max_connections` | `u32` | `10` | 最大连接池大小 |
| `min_idle_connections` | `u32` | `1` | 最小空闲连接数 |
| `connection_timeout_secs` | `u64` | `30` | 连接超时 |
| `idle_timeout_secs` | `u64` | `600` | 空闲连接超时 |
| `max_lifetime_secs` | `u64` | `1800` | 最大连接生命周期 |
| `ssl_mode` | `SslMode` | `Prefer` | SSL/TLS模式 |

**DatabaseType 取值：**
- `Postgres`: PostgreSQL数据库
- `MySQL`: MySQL数据库
- `SQLite`: SQLite数据库（基于文件）

**SslMode 取值：**
- `Disable`: 禁用SSL
- `Prefer`: 优先使用SSL但不强制
- `Require`: 强制使用SSL

### DMSCQueueConfig

消息队列配置，用于异步处理。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `enabled` | `bool` | `true` | 启用队列系统 |
| `backend_type` | `DMSCQueueBackendType` | `Memory` | 队列后端类型 |
| `connection_string` | `String` | `"memory://localhost"` | 后端连接字符串 |
| `max_connections` | `u32` | `10` | 最大连接数 |
| `message_max_size` | `usize` | `1048576` | 最大消息大小（1MB） |
| `consumer_timeout_ms` | `u64` | `30000` | 消费者超时（30秒） |
| `producer_timeout_ms` | `u64` | `5000` | 生产者超时（5秒） |
| `retry_policy` | `DMSCRetryPolicy` | 见下文 | 重试配置 |
| `dead_letter_config` | `Option<DMSCDeadLetterConfig>` | `None` | 死信队列配置 |

**DMSCQueueBackendType 取值：**
- `Memory`: 内存队列（开发/测试）
- `RabbitMQ`: RabbitMQ后端
- `Kafka`: Apache Kafka后端
- `Redis`: 基于Redis的队列

### DMSCObservabilityConfig

可观测性配置，用于追踪和指标。

| 字段 | 类型 | 默认值 | 描述 |
|:------|:-----|:--------|:------------|
| `tracing_enabled` | `bool` | `true` | 启用分布式追踪 |
| `metrics_enabled` | `bool` | `true` | 启用指标收集 |
| `tracing_sampling_rate` | `f64` | `0.1` | 采样率（0.0-1.0） |
| `tracing_sampling_strategy` | `String` | `"rate"` | 采样策略 |
| `metrics_window_size_secs` | `u64` | `300` | 指标窗口（5分钟） |
| `metrics_bucket_size_secs` | `u64` | `10` | 桶大小（10秒） |

### 配置文件位置

DMSC使用以下目录结构：

```
project_root/
├── config.yaml              # 主配置文件
├── .dms/                    # 应用数据目录
│   ├── logs/                # 日志文件
│   │   └── app.log
│   ├── cache/               # 缓存文件
│   ├── reports/             # 生成的报告
│   ├── observability/       # 追踪/指标数据
│   └── tmp/                 # 临时文件
```

### 加载配置

**Rust:**
```rust
use dmsc::prelude::*;

let app = DMSCAppBuilder::new()
    .with_config("config.yaml")
    .with_logging(DMSCLogConfig::default())
    .build()?;

// 访问配置
let port: u16 = app.context().config().get_typed("gateway.listen_port")?;
```

**Python:**
```python
from dmsc import DMSCAppBuilder, DMSCLogConfig

app = (DMSCAppBuilder()
    .with_config("config.yaml")
    .with_logging(DMSCLogConfig())
    .build())
```

### 环境变量覆盖

可以使用环境变量覆盖配置值：

```bash
# 使用 DMSC_ 前缀覆盖
export DMSC_AUTH_JWT_SECRET="production-secret"
export DMSC_CACHE_BACKEND_TYPE="Redis"
export DMSC_DATABASE_HOST="prod-db.example.com"
export DMSC_GATEWAY_LISTEN_PORT="443"
```

环境变量命名规范：`DMSC_<节>_<键>`（大写，下划线分隔）

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
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