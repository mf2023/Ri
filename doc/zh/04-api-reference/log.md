<div align="center">

# Log API参考

**Version: 0.1.6**

**Last modified date: 2026-01-30**

log模块提供结构化日志记录与多后端支持，支持日志级别、格式化、采样和日志分析等功能。

## 模块概述

</div>

log模块包含以下子模块：

- **core**: 日志核心接口和类型定义
- **formatters**: 日志格式化器
- **backends**: 日志后端实现
- **sampling**: 日志采样机制
- **analytics**: 日志分析功能

<div align="center">

## 核心组件

</div>

### DMSCLogger

日志记录器主接口，提供统一的日志记录功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `trace(message)` | 记录跟踪日志 | `message: impl Display` | `()` |
| `debug(message)` | 记录调试日志 | `message: impl Display` | `()` |
| `info(message)` | 记录信息日志 | `message: impl Display` | `()` |
| `warn(message)` | 记录警告日志 | `message: impl Display` | `()` |
| `error(message)` | 记录错误日志 | `message: impl Display` | `()` |
| `fatal(message)` | 记录致命日志 | `message: impl Display` | `()` |
| `log(level, message)` | 记录指定级别日志 | `level: LogLevel`, `message: impl Display` | `()` |
| `with_field(key, value)` | 添加字段到日志上下文 | `key: &str`, `value: impl Serialize` | `DMSCLogger` |
| `with_fields(fields)` | 添加多个字段 | `fields: impl Serialize` | `DMSCLogger` |
| `with_span(name)` | 创建日志跨度 | `name: &str` | `LogSpan` |
| `flush()` | 刷新日志缓冲区 | 无 | `DMSCResult<()>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 基本日志记录
ctx.log().info("Application started");
ctx.log().warn("Configuration file not found, using defaults");
ctx.log().error("Database connection failed");

// 带字段的日志记录
ctx.log()
    .with_field("user_id", 12345)
    .with_field("action", "login")
    .info("User login successful");

// 结构化日志
let user_data = serde_json::json!({
    "id": 12345,
    "name": "John Doe",
    "email": "john@example.com"
});

ctx.log()
    .with_fields(user_data)
    .info("User profile updated");
```

### DMSCLogLevel

日志级别枚举类型。

#### 变体

| 变体 | 描述 | 数值 |
|:--------|:-------------|:-----|
| `Trace` | 最详细的调试信息 | 0 |
| `Debug` | 调试信息 | 1 |
| `Info` | 一般信息 | 2 |
| `Warn` | 警告信息 | 3 |
| `Error` | 错误信息 | 4 |
| `Fatal` | 致命错误 | 5 |

#### 使用示例

```rust
use dmsc::prelude::*;

// 设置日志级别
ctx.log().set_level(DMSCLogLevel::Info);

// 检查日志级别
if ctx.log().is_enabled(DMSCLogLevel::Debug) {
    ctx.log().debug("This is a debug message");
}

// 动态调整日志级别
ctx.log().set_level(DMSCLogLevel::Debug);
```

### DMSCLogConfig

日志配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `level` | `DMSCLogLevel` | 日志级别 | `Info` |
| `console_enabled` | `bool` | 启用控制台输出 | `true` |
| `file_enabled` | `bool` | 启用文件输出 | `false` |
| `sampling_default` | `f32` | 默认采样率 (0.0-1.0) | `1.0` |
| `file_name` | `String` | 日志文件名 | `""` |
| `json_format` | `bool` | 使用JSON格式 | `false` |
| `rotate_when` | `String` | 何时轮转 ("size" 或 "none") | `"size"` |
| `max_bytes` | `u64` | 轮转前最大文件大小(字节) | `10485760` |

#### 配置示例

```rust
use dmsc::prelude::*;

let log_config = DMSCLogConfig {
    level: DMSCLogLevel::Info,
    console_enabled: true,
    file_enabled: true,
    sampling_default: 1.0,
    file_name: "myapp.log".to_string(),
    json_format: true,
    rotate_when: "size".to_string(),
    max_bytes: 10 * 1024 * 1024, // 10MB
    ..Default::default()
};
```

<div align="center">

## 日志格式化

</div>

### DMSCLogFormat

日志格式枚举类型。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Text` | 纯文本格式 |
| `Json` | JSON格式 |
| `Pretty` | 美化格式 |
| `Structured` | 结构化格式 |

#### 格式示例

**Text格式:**
```
2024-01-15 10:30:45 [INFO] Application started
2024-01-15 10:30:45 [WARN] Configuration file not found, using defaults
```

**JSON格式:**
```json
{
  "timestamp": "2024-01-15T10:30:45.123Z",
  "level": "INFO",
  "message": "Application started",
  "fields": {
    "user_id": 12345,
    "action": "login"
  },
  "span": {
    "name": "user_operation",
    "id": "span_123"
  }
}
```

**Pretty格式:**
```
┌─ 2024-01-15 10:30:45.123 ─────────────────────────────┐
│ INFO  Application started                             │
│                                                       │
│ Fields:                                               │
│   • user_id: 12345                                    │
│   • action: login                                     │
└───────────────────────────────────────────────────────┘
```

### 自定义格式化器

```rust
use dmsc::prelude::*;

struct CustomFormatter;

impl LogFormatter for CustomFormatter {
    fn format(&self, record: &LogRecord) -> String {
        format!(
            "[{}] {} - {}",
            record.timestamp.format("%H:%M:%S"),
            record.level,
            record.message
        )
    }
}

// 使用自定义格式化器
ctx.log().set_formatter(Box::new(CustomFormatter));
```

<div align="center">

## 日志后端

</div>

### DMSCLogBackend

日志后端枚举类型。

#### 变体

| 变体 | 描述 |
|:--------|:-------------|
| `Stdout` | 标准输出 |
| `Stderr` | 标准错误 |
| `File(path)` | 文件输出 |
| `Syslog` | 系统日志 |
| `Http(url)` | HTTP输出 |
| `Custom(name)` | 自定义后端 |

### 文件日志

```rust
use dmsc::prelude::*;

// 基本文件日志
let file_backend = DMSCLogBackend::File("/var/log/myapp.log".to_string());
ctx.log().set_backend(file_backend);

// 带轮转的文件日志
let rotating_backend = RotatingFileBackend::new(
    "/var/log/myapp.log",
    100,  // 最大文件大小(MB)
    10    // 最大文件数量
);
ctx.log().set_backend(DMSCLogBackend::Custom("rotating_file".to_string()));
```

### 多后端输出

```rust
use dmsc::prelude::*;

// 同时输出到文件和控制台
let multi_backend = MultiLogBackend::new(vec![
    DMSCLogBackend::Stdout,
    DMSCLogBackend::File("/var/log/myapp.log".to_string()),
]);

ctx.log().set_backend(DMSCLogBackend::Custom("multi".to_string()));
```

### 远程日志

```rust
use dmsc::prelude::*;

// HTTP日志后端
let http_backend = HttpLogBackend::new(
    "https://logs.example.com/api/v1/logs",
    Some("api-key-12345".to_string())
);

ctx.log().set_backend(DMSCLogBackend::Custom("http".to_string()));
```

<div align="center">

## 日志采样

</div>

### 采样配置

```rust
use dmsc::prelude::*;

let sampling_config = DMSCLogSamplingConfig {
    enable_sampling: true,
    sampling_rate: 0.1,        // 10%采样率
    burst_threshold: 1000,     // 突发阈值
    time_window: 60,           // 时间窗口(秒)
    ..Default::default()
};

ctx.log().set_sampling_config(sampling_config);
```

### 条件采样

```rust
use dmsc::prelude::*;

// 基于日志级别的采样
let level_sampling = LevelBasedSampling::new()
    .set_rate(DMSCLogLevel::Debug, 0.01)    // Debug日志1%采样
    .set_rate(DMSCLogLevel::Info, 0.1)     // Info日志10%采样
    .set_rate(DMSCLogLevel::Warn, 1.0)     // Warn日志100%采样
    .set_rate(DMSCLogLevel::Error, 1.0);   // Error日志100%采样

ctx.log().set_sampling_strategy(DMSCLogSamplingStrategy::LevelBased(level_sampling));
```

### 自适应采样

```rust
use dmsc::prelude::*;

// 自适应采样，根据日志量动态调整采样率
let adaptive_sampling = AdaptiveSampling::new()
    .set_min_rate(0.01)      // 最小采样率1%
    .set_max_rate(1.0)       // 最大采样率100%
    .set_target_rate(1000);  // 目标日志率(条/秒)

ctx.log().set_sampling_strategy(DMSCLogSamplingStrategy::Adaptive(adaptive_sampling));
```

<div align="center">

## 日志跨度

</div>

### 创建日志跨度

```rust
use dmsc::prelude::*;

// 创建日志跨度
let span = ctx.log().with_span("user_operation");

// 在跨度内记录日志
span.info("Starting user operation");
span.with_field("user_id", 12345)
    .debug("Processing user data");
span.info("User operation completed");

// 自动关闭跨度
drop(span);
```

### 嵌套跨度

```rust
use dmsc::prelude::*;

let outer_span = ctx.log().with_span("request_processing");
outer_span.info("Processing HTTP request");

{
    let inner_span = ctx.log().with_span("database_query");
    inner_span.with_field("query", "SELECT * FROM users")
        .debug("Executing database query");
    
    // 模拟数据库操作
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    inner_span.info("Database query completed");
}

outer_span.info("Request processing completed");
```

<div align="center">

## 日志分析

</div>

### 日志指标

```rust
use dmsc::prelude::*;

// 启用日志指标收集
ctx.log().enable_metrics(true);

// 获取日志统计
let stats = ctx.log().get_stats()?;
println!("Total logs: {}", stats.total_logs);
println!("By level: {:?}", stats.by_level);
println!("Error rate: {:.2}%", stats.error_rate * 100.0);
```

### 日志查询

```rust
use dmsc::prelude::*;

// 查询最近100条日志
let recent_logs = ctx.log().query()
    .limit(100)
    .level(DMSCLogLevel::Error)
    .execute()?;

for log in recent_logs {
    println!("[{}] {} - {}", log.timestamp, log.level, log.message);
}
```

### 日志聚合

```rust
use dmsc::prelude::*;

// 按级别聚合日志
let level_aggregation = ctx.log().aggregate()
    .by_level()
    .time_range(chrono::Utc::now() - chrono::Duration::hours(1), chrono::Utc::now())
    .execute()?;

for (level, count) in level_aggregation {
    println!("{}: {} logs", level, count);
}
```

<div align="center">

## 日志上下文

</div>

### 全局上下文

```rust
use dmsc::prelude::*;

// 设置全局上下文字段
ctx.log().set_global_context(serde_json::json!({
    "service": "my-service",
    "version": "1.0.0",
    "environment": "production"
}));

// 所有日志都会包含这些字段
ctx.log().info("This log includes global context");
```

### 请求上下文

```rust
use dmsc::prelude::*;

// 为特定请求设置上下文
let request_context = serde_json::json!({
    "request_id": "req_12345",
    "user_id": 12345,
    "ip_address": "192.168.1.1"
});

ctx.log().set_request_context(request_context);

// 在该请求处理过程中记录的所有日志都会包含请求上下文
ctx.log().info("Processing user request");
```

<div align="center">

## 日志过滤

</div>

### 级别过滤

```rust
use dmsc::prelude::*;

// 设置全局日志级别
ctx.log().set_level(DMSCLogLevel::Warn);

// 为特定模块设置日志级别
ctx.log().set_module_level("database", DMSCLogLevel::Debug);
ctx.log().set_module_level("http", DMSCLogLevel::Info);
```

### 内容过滤

```rust
use dmsc::prelude::*;

// 基于关键字的过滤
let keyword_filter = KeywordFilter::new()
    .exclude("password")
    .exclude("secret")
    .exclude("token");

ctx.log().add_filter(Box::new(keyword_filter));
```

### 速率限制

```rust
use dmsc::prelude::*;

// 设置日志速率限制
let rate_limit = LogRateLimit::new()
    .set_max_logs_per_second(100)
    .set_burst_size(1000);

ctx.log().set_rate_limit(rate_limit);
```

<div align="center">

## 错误处理

</div>

### 日志错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `LOG_FILE_NOT_FOUND` | 日志文件未找到 |
| `LOG_FILE_PERMISSION_DENIED` | 日志文件权限不足 |
| `LOG_FORMAT_ERROR` | 日志格式错误 |
| `LOG_BACKEND_ERROR` | 日志后端错误 |
| `LOG_ROTATION_ERROR` | 日志轮转错误 |

### 错误处理示例

```rust
use dmsc::prelude::*;

match ctx.log().flush() {
    Ok(_) => {
        // 日志刷新成功
    }
    Err(DMSCError { code, .. }) if code == "LOG_FILE_PERMISSION_DENIED" => {
        // 文件权限错误，回退到标准输出
        ctx.log().set_backend(DMSCLogBackend::Stdout);
        ctx.log().warn("Falling back to stdout logging due to file permission error");
    }
    Err(e) => {
        // 其他错误
        return Err(e);
    }
}
```

<div align="center">

## 性能优化

</div>

### 异步日志

```rust
use dmsc::prelude::*;

// 启用异步日志
ctx.log().enable_async(true);
ctx.log().set_async_buffer_size(10000);  // 设置缓冲区大小

// 异步记录日志
ctx.log().info_async("This is an async log message").await?;
```

### 批量日志

```rust
use dmsc::prelude::*;

// 批量记录日志
let batch = vec![
    LogEntry::info("Message 1"),
    LogEntry::warn("Message 2"),
    LogEntry::error("Message 3"),
];

ctx.log().log_batch(batch).await?;
```

### 内存优化

```rust
use dmsc::prelude::*;

// 优化内存使用
ctx.log().set_max_memory_usage(100 * 1024 * 1024);  // 100MB
ctx.log().enable_memory_compression(true);
ctx.log().set_compression_threshold(1024);  // 1KB以上启用压缩
```

<div align="center">

## 最佳实践

</div>

1. **使用结构化日志**: 使用字段而不是字符串拼接
2. **适当的日志级别**: 根据重要性选择合适的日志级别
3. **避免记录敏感信息**: 不要记录密码、密钥等敏感信息
4. **使用日志跨度**: 为相关操作创建日志跨度
5. **启用采样**: 对于高频日志启用采样机制
6. **定期轮转日志**: 使用日志轮转避免磁盘空间耗尽
7. **监控日志性能**: 监控日志系统的性能指标
8. **使用异步日志**: 对于性能敏感的应用使用异步日志

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信