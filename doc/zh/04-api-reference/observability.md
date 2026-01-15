<div align="center">

# Observability API参考

**Version: 0.1.4**

**Last modified date: 2026-01-15**

observability模块提供分布式追踪、指标收集、健康检查与性能监控功能，支持OpenTelemetry标准。

## 模块概述

</div>

observability模块包含以下子模块：

- **tracing**: 分布式追踪
- **metrics**: 指标收集
- **health**: 健康检查
- **profiling**: 性能分析
- **alerts**: 告警管理

<div align="center">

## 核心组件

</div>

### DMSCObservability

可观测性管理器主接口，提供统一的监控功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start_trace(name)` | 开始追踪 | `name: &str` | `DMSCTrace` |
| `record_metric(name, value)` | 记录指标 | `name: &str`, `value: f64` | `()` |
| `increment_counter(name)` | 增加计数器 | `name: &str` | `()` |
| `set_gauge(name, value)` | 设置计量器 | `name: &str`, `value: f64` | `()` |
| `record_histogram(name, value)` | 记录直方图 | `name: &str`, `value: f64` | `()` |
| `check_health()` | 执行健康检查 | 无 | `DMSCResult<HealthStatus>` |
| `start_profiling()` | 开始性能分析 | 无 | `DMSCResult<()>` |
| `stop_profiling()` | 停止性能分析 | 无 | `DMSCResult<Vec<ProfileData>>` |
| `get_metrics()` | 获取所有指标 | 无 | `HashMap<String, MetricValue>` |
| `export_metrics()` | 导出指标 | 无 | `DMSCResult<String>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 记录指标
ctx.observability().increment_counter("requests.total");
ctx.observability().record_metric("response.time", 125.5);
ctx.observability().set_gauge("active.connections", 42.0);

// 分布式追踪
let trace = ctx.observability().start_trace("user_request");
trace.with_tag("user_id", "12345");
trace.with_tag("endpoint", "/api/users");

// 记录追踪事件
trace.record_event("database_query", serde_json::json!({
    "query": "SELECT * FROM users",
    "duration_ms": 45.2
}));

// 结束追踪
trace.finish();
```

### DMSCTrace

分布式追踪接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `with_tag(key, value)` | 添加标签 | `key: &str`, `value: impl Serialize` | `&Self` |
| `with_tags(tags)` | 添加多个标签 | `tags: impl Serialize` | `&Self` |
| `record_event(name, attributes)` | 记录事件 | `name: &str`, `attributes: impl Serialize` | `()` |
| `start_span(name)` | 开始子跨度 | `name: &str` | `DMSCSpan` |
| `set_status(status)` | 设置状态 | `status: TraceStatus` | `()` |
| `finish()` | 结束追踪 | 无 | `()` |
| `get_trace_id()` | 获取追踪ID | 无 | `String` |
| `get_span_id()` | 获取跨度ID | 无 | `String` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 开始追踪
let trace = ctx.observability().start_trace("http_request");
trace.with_tags(serde_json::json!({
    "method": "GET",
    "path": "/api/users",
    "user_agent": "Mozilla/5.0"
}));

// 记录数据库查询
let db_span = trace.start_span("database_query");
db_span.with_tag("table", "users");
db_span.with_tag("operation", "SELECT");

// 模拟数据库操作
std::thread::sleep(std::time::Duration::from_millis(50));

db_span.finish();

// 设置追踪状态
trace.set_status(TraceStatus::Ok);
trace.finish();
```

### DMSCSpan

追踪跨度接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `with_tag(key, value)` | 添加标签 | `key: &str`, `value: impl Serialize` | `&Self` |
| `with_tags(tags)` | 添加多个标签 | `tags: impl Serialize` | `&Self` |
| `record_event(name, attributes)` | 记录事件 | `name: &str`, `attributes: impl Serialize` | `()` |
| `start_span(name)` | 开始子跨度 | `name: &str` | `DMSCSpan` |
| `set_status(status)` | 设置状态 | `status: TraceStatus` | `()` |
| `finish()` | 结束跨度 | 无 | `()` |
| `get_span_id()` | 获取跨度ID | 无 | `String` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建嵌套跨度
let parent_span = ctx.observability().start_trace("request_processing");

{
    let db_span = parent_span.start_span("database_operation");
    db_span.with_tag("database", "users");
    
    // 模拟数据库操作
    std::thread::sleep(std::time::Duration::from_millis(30));
    
    db_span.finish();
}

{
    let cache_span = parent_span.start_span("cache_operation");
    cache_span.with_tag("cache_type", "redis");
    
    // 模拟缓存操作
    std::thread::sleep(std::time::Duration::from_millis(10));
    
    cache_span.finish();
}

parent_span.finish();
```
<div align="center">

## 指标收集

</div>

### 指标类型

#### DMSCCounter

计数器类型，只能增加。

```rust
use dmsc::prelude::*;

// 创建计数器
let counter = ctx.observability().create_counter("requests.total");
counter.increment();
counter.increment_by(5);

// 获取当前值
let count = counter.get();
```

#### DMSCGauge

计量器类型，可以设置任意值。

```rust
use dmsc::prelude::*;

// 创建计量器
let gauge = ctx.observability().create_gauge("connections.active");
gauge.set(42.0);
gauge.increment_by(1.0);
gauge.decrement_by(2.0);

// 获取当前值
let value = gauge.get();
```

#### DMSCHistogram

直方图类型，记录数值分布。

```rust
use dmsc::prelude::*;

// 创建直方图
let histogram = ctx.observability().create_histogram("response.duration");
histogram.record(125.5);
histogram.record(98.3);
histogram.record(156.7);

// 获取统计信息
let stats = histogram.get_stats();
println!("Count: {}", stats.count);
println!("Mean: {}", stats.mean);
println!("P95: {}", stats.percentile_95);
```

### 指标标签

```rust
use dmsc::prelude::*;

// 带标签的指标
let counter = ctx.observability()
    .create_counter_with_labels("requests.total", vec![
        ("method", "GET"),
        ("endpoint", "/api/users"),
        ("status", "200")
    ]);

counter.increment();

// 动态标签
let endpoint = get_current_endpoint();
let status = get_response_status();

ctx.observability()
    .increment_counter_with_labels("requests.total", vec![
        ("endpoint", endpoint.as_str()),
        ("status", status.as_str())
    ]);
```

### 指标导出

#### Prometheus格式

```rust
use dmsc::prelude::*;

// 导出Prometheus格式的指标
let prometheus_metrics = ctx.observability().export_prometheus()?;
println!("{}", prometheus_metrics);
```

#### StatsD格式

```rust
use dmsc::prelude::*;

// 导出StatsD格式的指标
let statsd_metrics = ctx.observability().export_statsd()?;
println!("{}", statsd_metrics);
```

<div align="center">

## 健康检查

</div>

### DMSCHealthCheck

健康检查接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `add_check(name, check)` | 添加健康检查 | `name: &str`, `check: impl HealthCheck` | `()` |
| `remove_check(name)` | 移除健康检查 | `name: &str` | `()` |
| `run_checks()` | 执行所有检查 | 无 | `DMSCResult<HealthReport>` |
| `get_status()` | 获取健康状态 | 无 | `HealthStatus` |

#### 内置健康检查

```rust
use dmsc::prelude::*;

// 数据库健康检查
let db_health = DatabaseHealthCheck::new("postgres://localhost/mydb");
ctx.observability().health().add_check("database", db_health);

// Redis健康检查
let redis_health = RedisHealthCheck::new("redis://localhost:6379");
ctx.observability().health().add_check("redis", redis_health);

// HTTP端点健康检查
let http_health = HttpHealthCheck::new("https://api.example.com/health");
ctx.observability().health().add_check("external_api", http_health);

// 执行健康检查
let health_report = ctx.observability().health().run_checks()?;

for (name, result) in health_report.results {
    match result.status {
        HealthStatus::Healthy => println!("{}: ✓", name),
        HealthStatus::Degraded => println!("{}: ⚠ ({})", name, result.message),
        HealthStatus::Unhealthy => println!("{}: ✗ ({})", name, result.message),
    }
}
```

### 自定义健康检查

```rust
use dmsc::prelude::*;

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
        // 实现具体的检查逻辑
        Ok(0.5)
    }
}

// 使用自定义健康检查
let custom_check = CustomHealthCheck { threshold: 0.8 };
ctx.observability().health().add_check("custom", custom_check);
```

<div align="center">

## 性能分析

</div>

### CPU分析

```rust
use dmsc::prelude::*;

// 开始CPU分析
ctx.observability().profiling().start_cpu_profiling()?;

// 执行需要分析的操作
perform_expensive_operation();

// 停止分析并获取结果
let profile_data = ctx.observability().profiling().stop_cpu_profiling()?;

// 导出分析结果
let flame_graph = profile_data.generate_flame_graph()?;
std::fs::write("cpu_profile.svg", flame_graph)?;
```

### 内存分析

```rust
use dmsc::prelude::*;

// 开始内存分析
ctx.observability().profiling().start_memory_profiling()?;

// 执行需要分析的操作
allocate_memory_intensive_operation();

// 停止分析并获取结果
let memory_profile = ctx.observability().profiling().stop_memory_profiling()?;

// 分析内存使用模式
for allocation in memory_profile.allocations {
    println!("Allocation: {} bytes at {:?}", allocation.size, allocation.timestamp);
}
```

### 性能指标

```rust
use dmsc::prelude::*;

// 记录性能指标
ctx.observability().profiling().record_performance_metric("cpu.usage", 45.2);
ctx.observability().profiling().record_performance_metric("memory.usage", 1024.0);
ctx.observability().profiling().record_performance_metric("disk.io", 125.5);

// 获取性能报告
let performance_report = ctx.observability().profiling().get_performance_report()?;
println!("CPU Usage: {:.1}%", performance_report.cpu_usage);
println!("Memory Usage: {:.1}MB", performance_report.memory_usage);
println!("Disk I/O: {:.1}MB/s", performance_report.disk_io);
```

<div align="center">

## 告警管理

</div>  

### DMSCAlert

告警接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `create_alert(name, condition)` | 创建告警 | `name: &str`, `condition: AlertCondition` | `DMSCAlert` |
| `enable_alert(name)` | 启用告警 | `name: &str` | `()` |
| `disable_alert(name)` | 禁用告警 | `name: &str` | `()` |
| `get_alerts()` | 获取所有告警 | 无 | `Vec<DMSCAlert>` |

### 告警条件

```rust
use dmsc::prelude::*;

// 创建告警条件
let cpu_alert_condition = AlertCondition::threshold(
    "cpu.usage",
    ThresholdCondition::GreaterThan(80.0),
    Duration::from_secs(300)  // 持续5分钟
);

let memory_alert_condition = AlertCondition::threshold(
    "memory.usage",
    ThresholdCondition::GreaterThan(90.0),
    Duration::from_secs(600)  // 持续10分钟
);

// 创建告警
ctx.observability().alerts().create_alert("high_cpu_usage", cpu_alert_condition)?;
ctx.observability().alerts().create_alert("high_memory_usage", memory_alert_condition)?;
```

### 告警通知

```rust
use dmsc::prelude::*;

// 配置告警通知
let email_notification = EmailNotification::new(
    "admin@example.com",
    "System Alert",
    "CPU usage exceeded threshold"
);

let slack_notification = SlackNotification::new(
    "https://hooks.slack.com/services/xxx/yyy/zzz",
    "#alerts"
);

// 添加通知渠道
ctx.observability().alerts().add_notification_channel("email", email_notification)?;
ctx.observability().alerts().add_notification_channel("slack", slack_notification)?;
```

<div align="center">

## OpenTelemetry集成

</div>

### 导出器配置

```rust
use dmsc::prelude::*;

// 配置Jaeger导出器
let jaeger_config = JaegerExporterConfig {
    endpoint: "http://localhost:14268/api/traces".to_string(),
    service_name: "my-service".to_string(),
    ..Default::default()
};

ctx.observability().set_trace_exporter(TraceExporter::Jaeger(jaeger_config))?;

// 配置Prometheus导出器
let prometheus_config = PrometheusExporterConfig {
    endpoint: "0.0.0.0:9090".to_string(),
    ..Default::default()
};

ctx.observability().set_metric_exporter(MetricExporter::Prometheus(prometheus_config))?;
```

### 上下文传播

```rust
use dmsc::prelude::*;

// 注入追踪上下文到HTTP请求头
let mut headers = HashMap::new();
ctx.observability().inject_context(&mut headers)?;

// 从HTTP请求头提取追踪上下文
let extracted_context = ctx.observability().extract_context(&headers)?;
let trace = ctx.observability().start_trace_with_context("http_request", extracted_context);
```

## 配置

### DMSCObservabilityConfig

可观测性配置结构体。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `tracing_enabled` | `bool` | 启用追踪 | `true` |
| `metrics_enabled` | `bool` | 启用指标 | `true` |
| `health_checks_enabled` | `bool` | 启用健康检查 | `true` |
| `profiling_enabled` | `bool` | 启用性能分析 | `false` |
| `sampling_rate` | `f64` | 追踪采样率 | `0.1` |
| `export_interval` | `Duration` | 导出间隔 | `60s` |

#### 配置示例

```rust
use dmsc::prelude::*;

let observability_config = DMSCObservabilityConfig {
    tracing_enabled: true,
    metrics_enabled: true,
    health_checks_enabled: true,
    profiling_enabled: cfg!(debug_assertions),  // 只在调试模式下启用
    sampling_rate: 0.1,
    export_interval: Duration::from_secs(60),
};

ctx.observability().configure(observability_config)?;
```

<div align="center">

## 错误处理

</div>  

### 可观测性错误码

| 错误码 | 描述 |
|:--------|:-------------|
| `TRACE_EXPORT_ERROR` | 追踪导出错误 |
| `METRIC_EXPORT_ERROR` | 指标导出错误 |
| `HEALTH_CHECK_ERROR` | 健康检查错误 |
| `PROFILING_ERROR` | 性能分析错误 |
| `ALERT_CONFIG_ERROR` | 告警配置错误 |

### 错误处理示例

```rust
use dmsc::prelude::*;

match ctx.observability().export_metrics() {
    Ok(metrics) => {
        // 指标导出成功
        println!("Exported metrics: {}", metrics);
    }
    Err(DMSCError { code, .. }) if code == "METRIC_EXPORT_ERROR" => {
        // 指标导出错误，记录警告
        ctx.log().warn("Failed to export metrics, continuing without metrics");
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

1. **合理设置采样率**: 生产环境使用较低的采样率(0.1-0.01)
2. **使用有意义的指标名称**: 遵循命名约定，使用描述性名称
3. **添加适当的标签**: 为指标和追踪添加有用的标签
4. **监控关键路径**: 重点监控业务关键路径的性能
5. **设置合理的告警阈值**: 避免过多的误报和漏报
6. **定期审查健康检查**: 确保健康检查反映真实的系统状态
7. **使用异步导出**: 避免阻塞主业务流程
8. **保护敏感信息**: 不要在追踪和指标中包含敏感数据
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
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能