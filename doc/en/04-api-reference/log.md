<div align="center">

# Log API Reference

**Version: 0.1.4**

**Last modified date: 2026-01-15**

The log module provides structured logging with multi-backend support, supporting log levels, formatting, sampling, and log analysis features.

## Module Overview

</div>

The log module contains the following sub-modules:

- **core**: Log core interfaces and type definitions
- **formatters**: Log formatters
- **backends**: Log backend implementations
- **sampling**: Log sampling mechanisms
- **analytics**: Log analysis features

<div align="center">

## Core Components

</div>

### DMSCLogger

The main logger interface providing unified logging functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `trace(message)` | Record trace log | `message: impl Display` | `()` |
| `debug(message)` | Record debug log | `message: impl Display` | `()` |
| `info(message)` | Record info log | `message: impl Display` | `()` |
| `warn(message)` | Record warning log | `message: impl Display` | `()` |
| `error(message)` | Record error log | `message: impl Display` | `()` |
| `fatal(message)` | Record fatal log | `message: impl Display` | `()` |
| `log(level, message)` | Record log at specified level | `level: LogLevel`, `message: impl Display` | `()` |
| `with_field(key, value)` | Add field to log context | `key: &str`, `value: impl Serialize` | `DMSCLogger` |
| `with_fields(fields)` | Add multiple fields | `fields: impl Serialize` | `DMSCLogger` |
| `with_span(name)` | Create log span | `name: &str` | `LogSpan` |
| `flush()` | Flush log buffer | None | `DMSCResult<()>` |

#### Usage Examples

```rust
use dmsc::prelude::*;

// Basic logging
ctx.log().info("Application started");
ctx.log().warn("Configuration file not found, using defaults");
ctx.log().error("Database connection failed");

// Logging with fields
ctx.log()
    .with_field("user_id", 12345)
    .with_field("action", "login")
    .info("User login successful");

// Structured logging
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

Log level enumeration type.

#### Variants

| Variant | Description | Value |
|:--------|:-------------|:-----|
| `Trace` | Most detailed debug information | 0 |
| `Debug` | Debug information | 1 |
| `Info` | General information | 2 |
| `Warn` | Warning information | 3 |
| `Error` | Error information | 4 |
| `Fatal` | Fatal error | 5 |

#### Usage Examples

```rust
use dmsc::prelude::*;

// Set log level
ctx.log().set_level(DMSCLogLevel::Info);

// Check log level
if ctx.log().is_enabled(DMSCLogLevel::Debug) {
    ctx.log().debug("This is a debug message");
}

// Dynamically adjust log level
ctx.log().set_level(DMSCLogLevel::Debug);
```

### DMSCLogConfig

Log configuration structure.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `level` | `DMSCLogLevel` | Log level | `Info` |
| `console_enabled` | `bool` | Enable console output | `true` |
| `file_enabled` | `bool` | Enable file output | `false` |
| `sampling_default` | `f32` | Default sampling rate (0.0-1.0) | `1.0` |
| `file_name` | `String` | Log file name | `""` |
| `json_format` | `bool` | Use JSON format | `false` |
| `rotate_when` | `String` | When to rotate ("size" or "none") | `"size"` |
| `max_bytes` | `u64` | Max file size before rotation (bytes) | `10485760` |

#### Configuration Example

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

## Log Formatting

</div>

### DMSCLogFormat

Log format enumeration type.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Text` | Plain text format |
| `Json` | JSON format |
| `Pretty` | Pretty-printed format |
| `Structured` | Structured format |

#### Format Examples

**Text Format:**
```
2024-01-15 10:30:45 [INFO] Application started
2024-01-15 10:30:45 [WARN] Configuration file not found, using defaults
```

**JSON Format:**
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

**Pretty Format:**
```
┌─ 2024-01-15 10:30:45.123 ─────────────────────────────┐
│ INFO  Application started                             │
│                                                       │
│ Fields:                                               │
│   • user_id: 12345                                    │
│   • action: login                                     │
└───────────────────────────────────────────────────────┘
```

### Custom Formatter

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

// Use custom formatter
ctx.log().set_formatter(Box::new(CustomFormatter));
```

<div align="center">

## Log Backends

</div>

### DMSCLogBackend

Log backend enumeration type.

#### Variants

| Variant | Description |
|:--------|:-------------|
| `Stdout` | Standard output |
| `Stderr` | Standard error |
| `File(path)` | File output |
| `Syslog` | System log |
| `Http(url)` | HTTP output |
| `Custom(name)` | Custom backend |

### File Logging

```rust
use dmsc::prelude::*;

// Basic file logging
let file_backend = DMSCLogBackend::File("/var/log/myapp.log".to_string());
ctx.log().set_backend(file_backend);

// File logging with rotation
let rotating_backend = RotatingFileBackend::new(
    "/var/log/myapp.log",
    100,  // Maximum file size (MB)
    10    // Maximum number of files
);
ctx.log().set_backend(DMSCLogBackend::Custom("rotating_file".to_string()));
```

### Multi-Backend Output

```rust
use dmsc::prelude::*;

// Output to both file and console
let multi_backend = MultiLogBackend::new(vec![
    DMSCLogBackend::Stdout,
    DMSCLogBackend::File("/var/log/myapp.log".to_string()),
]);

ctx.log().set_backend(DMSCLogBackend::Custom("multi".to_string()));
```

### Remote Logging

```rust
use dmsc::prelude::*;

// HTTP log backend
let http_backend = HttpLogBackend::new(
    "https://logs.example.com/api/v1/logs",
    Some("api-key-12345".to_string())
);

ctx.log().set_backend(DMSCLogBackend::Custom("http".to_string()));
```

<div align="center">

## Log Sampling

</div>

### Sampling Configuration

```rust
use dmsc::prelude::*;

let sampling_config = DMSCLogSamplingConfig {
    enable_sampling: true,
    sampling_rate: 0.1,        // 10% sampling rate
    burst_threshold: 1000,     // Burst threshold
    time_window: 60,           // Time window (seconds)
    ..Default::default()
};

ctx.log().set_sampling_config(sampling_config);
```

### Conditional Sampling

```rust
use dmsc::prelude::*;

// Level-based sampling
let level_sampling = LevelBasedSampling::new()
    .set_rate(DMSCLogLevel::Debug, 0.01)    // Debug logs 1% sampling
    .set_rate(DMSCLogLevel::Info, 0.1)     // Info logs 10% sampling
    .set_rate(DMSCLogLevel::Warn, 1.0)     // Warn logs 100% sampling
    .set_rate(DMSCLogLevel::Error, 1.0);   // Error logs 100% sampling

ctx.log().set_sampling_strategy(DMSCLogSamplingStrategy::LevelBased(level_sampling));
```

### Adaptive Sampling

```rust
use dmsc::prelude::*;

// Adaptive sampling, dynamically adjusts sampling rate based on log volume
let adaptive_sampling = AdaptiveSampling::new()
    .set_min_rate(0.01)      // Minimum sampling rate 1%
    .set_max_rate(1.0)       // Maximum sampling rate 100%
    .set_target_rate(1000);  // Target log rate (logs/second)

ctx.log().set_sampling_strategy(DMSCLogSamplingStrategy::Adaptive(adaptive_sampling));
```

<div align="center">

## Log Spans

</div>

### Creating Log Spans

```rust
use dmsc::prelude::*;

// Create log span
let span = ctx.log().with_span("user_operation");

// Log within span
span.info("Starting user operation");
span.with_field("user_id", 12345)
    .debug("Processing user data");
span.info("User operation completed");

// Auto-close span
drop(span);
```

### Nested Spans

```rust
use dmsc::prelude::*;

let outer_span = ctx.log().with_span("request_processing");
outer_span.info("Processing HTTP request");

{
    let inner_span = ctx.log().with_span("database_query");
    inner_span.with_field("query", "SELECT * FROM users")
        .debug("Executing database query");
    
    // Simulate database operation
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    inner_span.info("Database query completed");
}

outer_span.info("Request processing completed");
```

<div align="center">

## Log Analysis

</div>

### Log Metrics

```rust
use dmsc::prelude::*;

// Enable log metrics collection
ctx.log().enable_metrics(true);

// Get log statistics
let stats = ctx.log().get_stats()?;
println!("Total logs: {}", stats.total_logs);
println!("By level: {:?}", stats.by_level);
println!("Error rate: {:.2}%", stats.error_rate * 100.0);
```

### Log Query

```rust
use dmsc::prelude::*;

// Query last 100 logs
let recent_logs = ctx.log().query()
    .limit(100)
    .level(DMSCLogLevel::Error)
    .execute()?;

for log in recent_logs {
    println!("[{}] {} - {}", log.timestamp, log.level, log.message);
}
```

### Log Aggregation

```rust
use dmsc::prelude::*;

// Aggregate logs by level
let level_aggregation = ctx.log().aggregate()
    .by_level()
    .time_range(chrono::Utc::now() - chrono::Duration::hours(1), chrono::Utc::now())
    .execute()?;

for (level, count) in level_aggregation {
    println!("{}: {} logs", level, count);
}
```

<div align="center">

## Log Context

</div>

### Global Context

```rust
use dmsc::prelude::*;

// Set global context fields
ctx.log().set_global_context(serde_json::json!({
    "service": "my-service",
    "version": "1.0.0",
    "environment": "production"
}));

// All logs will include these fields
ctx.log().info("This log includes global context");
```

### Request Context

```rust
use dmsc::prelude::*;

// Set context for specific request
let request_context = serde_json::json!({
    "request_id": "req_12345",
    "user_id": 12345,
    "ip_address": "192.168.1.1"
});

ctx.log().set_request_context(request_context);

// All logs recorded during this request will include request context
ctx.log().info("Processing user request");
```

<div align="center">

## Log Filtering

</div>

### Level Filtering

```rust
use dmsc::prelude::*;

// Set global log level
ctx.log().set_level(DMSCLogLevel::Warn);

// Set log level for specific modules
ctx.log().set_module_level("database", DMSCLogLevel::Debug);
ctx.log().set_module_level("http", DMSCLogLevel::Info);
```

### Content Filtering

```rust
use dmsc::prelude::*;

// Keyword-based filtering
let keyword_filter = KeywordFilter::new()
    .exclude("password")
    .exclude("secret")
    .exclude("token");

ctx.log().add_filter(Box::new(keyword_filter));
```

### Rate Limiting

```rust
use dmsc::prelude::*;

// Set log rate limit
let rate_limit = LogRateLimit::new()
    .set_max_logs_per_second(100)
    .set_burst_size(1000);

ctx.log().set_rate_limit(rate_limit);
```

<div align="center">

## Error Handling

</div>

### Log Error Codes

| Error Code | Description |
|:--------|:-------------|
| `LOG_FILE_NOT_FOUND` | Log file not found |
| `LOG_FILE_PERMISSION_DENIED` | Log file permission denied |
| `LOG_FORMAT_ERROR` | Log format error |
| `LOG_BACKEND_ERROR` | Log backend error |
| `LOG_ROTATION_ERROR` | Log rotation error |

### Error Handling Example

```rust
use dmsc::prelude::*;

match ctx.log().flush() {
    Ok(_) => {
        // Log flush successful
    }
    Err(DMSCError { code, .. }) if code == "LOG_FILE_PERMISSION_DENIED" => {
        // File permission error, fallback to stdout
        ctx.log().set_backend(DMSCLogBackend::Stdout);
        ctx.log().warn("Falling back to stdout logging due to file permission error");
    }
    Err(e) => {
        // Other errors
        return Err(e);
    }
}
```

<div align="center">

## Performance Optimization

</div>

### Async Logging

```rust
use dmsc::prelude::*;

// Enable async logging
ctx.log().enable_async(true);
ctx.log().set_async_buffer_size(10000);  // Set buffer size

// Async log recording
ctx.log().info_async("This is an async log message").await?;
```

### Batch Logging

```rust
use dmsc::prelude::*;

// Batch log recording
let batch = vec![
    LogEntry::info("Message 1"),
    LogEntry::warn("Message 2"),
    LogEntry::error("Message 3"),
];

ctx.log().log_batch(batch).await?;
```

### Memory Optimization

```rust
use dmsc::prelude::*;

// Optimize memory usage
ctx.log().set_max_memory_usage(100 * 1024 * 1024);  // 100MB
ctx.log().enable_memory_compression(true);
ctx.log().set_compression_threshold(1024);  // Enable compression for logs above 1KB
```

<div align="center">

## Best Practices

</div>

1. **Use Structured Logging**: Use fields instead of string concatenation
2. **Appropriate Log Levels**: Choose appropriate log levels based on importance
3. **Avoid Logging Sensitive Information**: Do not log passwords, keys, or other sensitive information
4. **Use Log Spans**: Create log spans for related operations
5. **Enable Sampling**: Enable sampling mechanism for high-frequency logs
6. **Regular Log Rotation**: Use log rotation to avoid disk space exhaustion
7. **Monitor Log Performance**: Monitor performance metrics of the logging system
8. **Use Async Logging**: Use async logging for performance-sensitive applications

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database operation support
- [device](./device.md): Device module using protocols for device communication
- [fs](./fs.md): Filesystem module providing file operation functions
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
