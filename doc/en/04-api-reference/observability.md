<div align="center">

# Observability API Reference

**Version: 1.0.0**

**Last modified date: 2025-12-12**

The observability module provides distributed tracing, metrics collection, health checks, and performance monitoring functionality, supporting the OpenTelemetry standard.

## Module Overview

</div>

The observability module contains the following sub-modules:

- **tracing**: Distributed tracing
- **metrics**: Metrics collection
- **health**: Health checks
- **profiling**: Performance profiling
- **alerts**: Alert management

<div align="center">

## Core Components

</div>

### DMSCObservability

Main interface for the observability manager, providing unified monitoring functionality.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `start_trace(name)` | Start tracing | `name: &str` | `DMSCTrace` |
| `record_metric(name, value)` | Record metric | `name: &str`, `value: f64` | `()` |
| `increment_counter(name)` | Increment counter | `name: &str` | `()` |
| `set_gauge(name, value)` | Set gauge | `name: &str`, `value: f64` | `()` |
| `record_histogram(name, value)` | Record histogram | `name: &str`, `value: f64` | `()` |
| `check_health()` | Execute health check | None | `DMSCResult<HealthStatus>` |
| `start_profiling()` | Start performance profiling | None | `DMSCResult<()>` |
| `stop_profiling()` | Stop performance profiling | None | `DMSCResult<Vec<ProfileData>>` |
| `get_metrics()` | Get all metrics | None | `HashMap<String, MetricValue>` |
| `export_metrics()` | Export metrics | None | `DMSCResult<String>` |

#### Usage Example

```rust
use dms::prelude::*;

// Record metrics
ctx.observability().increment_counter("requests.total");
ctx.observability().record_metric("response.time", 125.5);
ctx.observability().set_gauge("active.connections", 42.0);

// Distributed tracing
let trace = ctx.observability().start_trace("user_request");
trace.with_tag("user_id", "12345");
trace.with_tag("endpoint", "/api/users");

// Record trace event
trace.record_event("database_query", serde_json::json!({
    "query": "SELECT * FROM users",
    "duration_ms": 45.2
}));

// End tracing
trace.finish();
```

### DMSCTrace

Distributed tracing interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `with_tag(key, value)` | Add tag | `key: &str`, `value: impl Serialize` | `&Self` |
| `with_tags(tags)` | Add multiple tags | `tags: impl Serialize` | `&Self` |
| `record_event(name, attributes)` | Record event | `name: &str`, `attributes: impl Serialize` | `()` |
| `start_span(name)` | Start child span | `name: &str` | `DMSCSpan` |
| `set_status(status)` | Set status | `status: TraceStatus` | `()` |
| `finish()` | End tracing | None | `()` |
| `get_trace_id()` | Get trace ID | None | `String` |
| `get_span_id()` | Get span ID | None | `String` |

#### Usage Example

```rust
use dms::prelude::*;

// Start tracing
let trace = ctx.observability().start_trace("http_request");
trace.with_tags(serde_json::json!({
    "method": "GET",
    "path": "/api/users",
    "user_agent": "Mozilla/5.0"
}));

// Record database query
let db_span = trace.start_span("database_query");
db_span.with_tag("table", "users");
db_span.with_tag("operation", "SELECT");

// Simulate database operation
std::thread::sleep(std::time::Duration::from_millis(50));

db_span.finish();

// Set trace status
trace.set_status(TraceStatus::Ok);
trace.finish();
```

### DMSCSpan

Trace span interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `with_tag(key, value)` | Add tag | `key: &str`, `value: impl Serialize` | `&Self` |
| `with_tags(tags)` | Add multiple tags | `tags: impl Serialize` | `&Self` |
| `record_event(name, attributes)` | Record event | `name: &str`, `attributes: impl Serialize` | `()` |
| `start_span(name)` | Start child span | `name: &str` | `DMSCSpan` |
| `set_status(status)` | Set status | `status: TraceStatus` | `()` |
| `finish()` | End span | None | `()` |
| `get_span_id()` | Get span ID | None | `String` |

#### Usage Example

```rust
use dms::prelude::*;

// Create nested spans
let parent_span = ctx.observability().start_trace("request_processing");

{
    let db_span = parent_span.start_span("database_operation");
    db_span.with_tag("database", "users");
    
    // Simulate database operation
    std::thread::sleep(std::time::Duration::from_millis(30));
    
    db_span.finish();
}

{
    let cache_span = parent_span.start_span("cache_operation");
    cache_span.with_tag("cache_type", "redis");
    
    // Simulate cache operation
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    cache_span.finish();
}

parent_span.finish();
```
<div align="center">

## Metrics Collection

</div>

### Metric Types

#### DMSCCounter

Counter type, can only be incremented.

```rust
use dms::prelude::*;

// Create counter
let counter = ctx.observability().create_counter("requests.total");
counter.increment();
counter.increment_by(5);

// Get current value
let count = counter.get();
```

#### DMSCGauge

Gauge type, can be set to any value.

```rust
use dms::prelude::*;

// Create gauge
let gauge = ctx.observability().create_gauge("connections.active");
gauge.set(42.0);
gauge.increment_by(1.0);
gauge.decrement_by(2.0);

// Get current value
let value = gauge.get();
```

#### DMSCHistogram

Histogram type, records value distribution.

```rust
use dms::prelude::*;

// Create histogram
let histogram = ctx.observability().create_histogram("response.duration");
histogram.record(125.5);
histogram.record(98.3);
histogram.record(156.7);

// Get statistics
let stats = histogram.get_stats();
println!("Count: {}", stats.count);
println!("Mean: {}", stats.mean);
println!("P95: {}", stats.percentile_95);
```

### Metric Labels

```rust
use dms::prelude::*;

// Metrics with labels
let counter = ctx.observability()
    .create_counter_with_labels("requests.total", vec![
        ("method", "GET"),
        ("endpoint", "/api/users"),
        ("status", "200")
    ]);

counter.increment();

// Dynamic labels
let endpoint = get_current_endpoint();
let status = get_response_status();

ctx.observability()
    .increment_counter_with_labels("requests.total", vec![
        ("endpoint", endpoint.as_str()),
        ("status", status.as_str())
    ]);
```

### Metrics Export

#### Prometheus Format

```rust
use dms::prelude::*;

// Export metrics in Prometheus format
let prometheus_metrics = ctx.observability().export_prometheus()?;
println!("{}", prometheus_metrics);
```

#### StatsD Format

```rust
use dms::prelude::*;

// Export metrics in StatsD format
let statsd_metrics = ctx.observability().export_statsd()?;
println!("{}", statsd_metrics);
```

<div align="center">

## Health Checks

</div>

### DMSCHealthCheck

Health check interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `add_check(name, check)` | Add health check | `name: &str`, `check: impl HealthCheck` | `()` |
| `remove_check(name)` | Remove health check | `name: &str` | `()` |
| `run_checks()` | Execute all checks | None | `DMSCResult<HealthReport>` |
| `get_status()` | Get health status | None | `HealthStatus` |

#### Built-in Health Checks

```rust
use dms::prelude::*;

// Database health check
let db_health = DatabaseHealthCheck::new("postgres://localhost/mydb");
ctx.observability().health().add_check("database", db_health);

// Redis health check
let redis_health = RedisHealthCheck::new("redis://localhost:6379");
ctx.observability().health().add_check("redis", redis_health);

// HTTP endpoint health check
let http_health = HttpHealthCheck::new("https://api.example.com/health");
ctx.observability().health().add_check("external_api", http_health);

// Execute health checks
let health_report = ctx.observability().health().run_checks()?;

for (name, result) in health_report.results {
    match result.status {
        HealthStatus::Healthy => println!("{}: ✓", name),
        HealthStatus::Degraded => println!("{}: ⚠ ({})", name, result.message),
        HealthStatus::Unhealthy => println!("{}: ✗ ({})", name, result.message),
    }
}
```

### Custom Health Checks

```rust
use dms::prelude::*;

struct CustomHealthCheck {
    threshold: f64,
}

impl HealthCheck for CustomHealthCheck {
    fn check(&self) -> DMSCResult<HealthCheckResult> {
        let current_value = self.get_current_value()?;
        
        if current_value < self.threshold {
            Ok(HealthCheckResult::healthy())
        } else {
            Ok(HealthCheckResult::unhealthy(format!(
                "Value {} exceeds threshold {}",
                current_value, self.threshold
            )))
        }
    }
    
    fn name(&self) -> &str {
        "custom_check"
    }
}

impl CustomHealthCheck {
    fn get_current_value(&self) -> DMSCResult<f64> {
        // Implement specific check logic
        Ok(0.5)
    }
}

// Use custom health check
let custom_check = CustomHealthCheck { threshold: 0.8 };
ctx.observability().health().add_check("custom", custom_check);
```

<div align="center">

## Performance Profiling

</div>

### CPU Profiling

```rust
use dms::prelude::*;

// Start CPU profiling
ctx.observability().profiling().start_cpu_profiling()?;

// Perform operations to profile
perform_expensive_operation();

// Stop profiling and get results
let profile_data = ctx.observability().profiling().stop_cpu_profiling()?;

// Export profiling results
let flame_graph = profile_data.generate_flame_graph()?;
std::fs::write("cpu_profile.svg", flame_graph)?;
```

### Memory Profiling

```rust
use dms::prelude::*;

// Start memory profiling
ctx.observability().profiling().start_memory_profiling()?;

// Perform operations to profile
allocate_memory_intensive_operation();

// Stop profiling and get results
let memory_profile = ctx.observability().profiling().stop_memory_profiling()?;

// Analyze memory usage patterns
for allocation in memory_profile.allocations {
    println!("Allocation: {} bytes at {:?}", allocation.size, allocation.timestamp);
}
```

### Performance Metrics

```rust
use dms::prelude::*;

// Record performance metrics
ctx.observability().profiling().record_performance_metric("cpu.usage", 45.2);
ctx.observability().profiling().record_performance_metric("memory.usage", 1024.0);
ctx.observability().profiling().record_performance_metric("disk.io", 125.5);

// Get performance report
let performance_report = ctx.observability().profiling().get_performance_report()?;
println!("CPU Usage: {:.1}%", performance_report.cpu_usage);
println!("Memory Usage: {:.1}MB", performance_report.memory_usage);
println!("Disk I/O: {:.1}MB/s", performance_report.disk_io);
```

<div align="center">

## Alert Management

</div>  

### DMSCAlert

Alert interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `create_alert(name, condition)` | Create alert | `name: &str`, `condition: AlertCondition` | `DMSCAlert` |
| `enable_alert(name)` | Enable alert | `name: &str` | `()` |
| `disable_alert(name)` | Disable alert | `name: &str` | `()` |
| `get_alerts()` | Get all alerts | None | `Vec<DMSCAlert>` |

### Alert Conditions

```rust
use dms::prelude::*;

// Create alert conditions
let cpu_alert_condition = AlertCondition::threshold(
    "cpu.usage",
    ThresholdCondition::GreaterThan(80.0),
    Duration::from_secs(300)  // Lasts for 5 minutes
);

let memory_alert_condition = AlertCondition::threshold(
    "memory.usage",
    ThresholdCondition::GreaterThan(90.0),
    Duration::from_secs(600)  // Lasts for 10 minutes
);

// Create alerts
ctx.observability().alerts().create_alert("high_cpu_usage", cpu_alert_condition)?;
ctx.observability().alerts().create_alert("high_memory_usage", memory_alert_condition)?;
```

### Alert Notifications

```rust
use dms::prelude::*;

// Configure alert notifications
let email_notification = EmailNotification::new(
    "admin@example.com",
    "System Alert",
    "CPU usage exceeded threshold"
);

let slack_notification = SlackNotification::new(
    "https://hooks.slack.com/services/xxx/yyy/zzz",
    "#alerts"
);

// Add notification channels
ctx.observability().alerts().add_notification_channel("email", email_notification)?;
ctx.observability().alerts().add_notification_channel("slack", slack_notification)?;
```

<div align="center">

## OpenTelemetry Integration

</div>

### Exporter Configuration

```rust
use dms::prelude::*;

// Configure Jaeger exporter
let jaeger_config = JaegerExporterConfig {
    endpoint: "http://localhost:14268/api/traces".to_string(),
    service_name: "my-service".to_string(),
    ..Default::default()
};

ctx.observability().set_trace_exporter(TraceExporter::Jaeger(jaeger_config))?;

// Configure Prometheus exporter
let prometheus_config = PrometheusExporterConfig {
    endpoint: "0.0.0.0:9090".to_string(),
    ..Default::default()
};

ctx.observability().set_metric_exporter(MetricExporter::Prometheus(prometheus_config))?;
```

### Context Propagation

```rust
use dms::prelude::*;

// Inject trace context into HTTP request headers
let mut headers = HashMap::new();
ctx.observability().inject_context(&mut headers)?;

// Extract trace context from HTTP request headers
let extracted_context = ctx.observability().extract_context(&headers)?;
let trace = ctx.observability().start_trace_with_context("http_request", extracted_context);
```

## Configuration

### DMSCObservabilityConfig

Observability configuration struct.

#### Fields

| Field | Type | Description | Default Value |
|:--------|:-----|:-------------|:-------|
| `tracing_enabled` | `bool` | Enable tracing | `true` |
| `metrics_enabled` | `bool` | Enable metrics | `true` |
| `health_checks_enabled` | `bool` | Enable health checks | `true` |
| `profiling_enabled` | `bool` | Enable performance profiling | `false` |
| `sampling_rate` | `f64` | Trace sampling rate | `0.1` |
| `export_interval` | `Duration` | Export interval | `60s` |

#### Configuration Example

```rust
use dms::prelude::*;

let observability_config = DMSCObservabilityConfig {
    tracing_enabled: true,
    metrics_enabled: true,
    health_checks_enabled: true,
    profiling_enabled: cfg!(debug_assertions),  // Enable only in debug mode
    sampling_rate: 0.1,
    export_interval: Duration::from_secs(60),
};

ctx.observability().configure(observability_config)?;
```

<div align="center">

## Error Handling

</div>  

### Observability Error Codes

| Error Code | Description |
|:--------|:-------------|
| `TRACE_EXPORT_ERROR` | Trace export error |
| `METRIC_EXPORT_ERROR` | Metric export error |
| `HEALTH_CHECK_ERROR` | Health check error |
| `PROFILING_ERROR` | Performance profiling error |
| `ALERT_CONFIG_ERROR` | Alert configuration error |

### Error Handling Example

```rust
use dms::prelude::*;

match ctx.observability().export_metrics() {
    Ok(metrics) => {
        // Metrics export successful
        println!("Exported metrics: {}", metrics);
    }
    Err(DMSCError { code, .. }) if code == "METRIC_EXPORT_ERROR" => {
        // Metrics export error, log warning
        ctx.log().warn("Failed to export metrics, continuing without metrics");
    }
    Err(e) => {
        // Other errors
        return Err(e);
    }
}
```
<div align="center">

## Best Practices

</div>  

1. **Set appropriate sampling rate**: Use lower sampling rates in production (0.1-0.01)
2. **Use meaningful metric names**: Follow naming conventions and use descriptive names
3. **Add appropriate labels**: Add useful labels to metrics and traces
4. **Monitor critical paths**: Focus on monitoring performance of business-critical paths
5. **Set reasonable alert thresholds**: Avoid excessive false positives and false negatives
6. **Regularly review health checks**: Ensure health checks reflect actual system status
7. **Use asynchronous export**: Avoid blocking main business processes
8. **Protect sensitive information**: Do not include sensitive data in traces and metrics
<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview, providing API reference documentation overview and quick navigation
- [auth](./auth.md): Authentication module, providing JWT, OAuth2, and RBAC authentication and authorization functionality
- [core](./core.md): Core module, providing error handling and service context
- [log](./log.md): Logging module, recording authentication events and security logs
- [config](./config.md): Configuration module, managing authentication configuration and key settings
- [cache](./cache.md): Cache module, providing multi-backend cache abstraction, caching user sessions and permission data
- [database](./database.md): Database module, providing user data persistence and query functionality
- [http](./http.md): HTTP module, providing web authentication interfaces and middleware support
- [mq](./mq.md): Message queue module, handling authentication events and asynchronous notifications
- [security](./security.md): Security module, providing encryption, hashing, and verification functionality
- [storage](./storage.md): Storage module, managing authentication files, keys, and certificates
- [validation](./validation.md): Validation module, validating user input and form data
