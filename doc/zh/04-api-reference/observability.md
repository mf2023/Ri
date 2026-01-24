<div align="center">

# Observability API参考

**Version: 0.1.6**

**Last modified date: 2026-01-16**

observability模块提供分布式追踪、指标收集、Prometheus集成和Grafana仪表板生成功能。

## 模块概述

</div>

observability模块包含以下子模块：

- **tracing**: 分布式追踪
- **metrics**: 指标收集
- **propagation**: 追踪上下文传播
- **prometheus**: Prometheus指标导出
- **grafana**: Grafana仪表板生成
- **metrics_collector**: 系统指标收集器

<div align="center">

## 核心组件

</div>

### DMSCObservabilityModule

可观测性管理器主接口，提供统一的监控功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建可观测性模块 | `config: DMSCObservabilityConfig` | `Self` |
| `start_trace(name)` | 开始追踪 | `name: String` | `Option<DMSCTraceId>` |
| `start_span(...)` | 开始子跨度 | 多种参数 | `Option<DMSCSpanId>` |
| `end_span(id, status)` | 结束跨度 | `id: &DMSCSpanId`, `status: DMSCSpanStatus` | `DMSCResult<()>` |
| `get_tracer()` | 获取追踪器引用 | 无 | `Option<Arc<DMSCTracer>>` |
| `get_metrics_registry()` | 获取指标注册表 | 无 | `Option<Arc<DMSCMetricsRegistry>>` |

#### 使用示例

```rust
use dmsc::prelude::*;

// 开始追踪
let trace_id = observability.start_trace("user_request".to_string());

// 开始子跨度
let span_id = observability.start_span(
    None, // 使用当前追踪ID
    None, // 无父跨度
    "database_query".to_string(),
    DMSCSpanKind::Client
);

// 结束跨度
observability.end_span(&span_id, DMSCSpanStatus::Ok)?;
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

### DMSCMetricsRegistry

指标注册表，用于管理应用指标。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的指标注册表 | 无 | `Self` |
| `register(metric)` | 注册指标 | `metric: Arc<DMSCMetric>` | `DMSCResult<()>` |
| `get_metric(name)` | 获取指标 | `name: &str` | `Option<Arc<DMSCMetric>>` |
| `get_all_metrics()` | 获取所有指标 | 无 | `HashMap<String, Arc<DMSCMetric>>` |
| `export_prometheus()` | 导出Prometheus格式 | 无 | `String` |

#### 使用示例

```rust
use dmsc::prelude::*;
use std::sync::Arc;

let registry = DMSCMetricsRegistry::new();

// 创建并注册计数器指标
let counter = Arc::new(DMSCMetric::new(DMSCMetricConfig {
    name: "requests_total".to_string(),
    metric_type: DMSCMetricType::Counter,
    description: "Total number of requests".to_string(),
    buckets: None,
}));

registry.register(counter.clone())?;

// 记录指标值
counter.record(1.0);

// 导出Prometheus格式
let prometheus_output = registry.export_prometheus();
```

### DMSCMetric

指标类型，支持计数器、计量器和直方图。

#### DMSCMetricType

| 类型 | 描述 |
|:--------|:-------------|
| `Counter` | 计数器，只增不减 |
| `Gauge` | 计量器，可增可减 |
| `Histogram` | 直方图，记录分布 |

#### 使用示例

```rust
use dmsc::prelude::*;

// 创建计数器
let counter = DMSCMetric::new(DMSCMetricConfig {
    name: "http_requests_total".to_string(),
    metric_type: DMSCMetricType::Counter,
    description: "Total HTTP requests".to_string(),
    buckets: None,
});

// 记录值
counter.record(1.0);

// 创建直方图
let histogram = DMSCMetric::new(DMSCMetricConfig {
    name: "http_request_duration_seconds".to_string(),
    metric_type: DMSCMetricType::Histogram,
    description: "HTTP request duration".to_string(),
    buckets: Some(vec![0.1, 0.5, 1.0, 5.0]),
});

// 记录直方图值
histogram.record(0.125);
histogram.record(0.456);
```

### DMSCWindowStats

滑动窗口统计信息。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `count` | `u64` | 样本数量 |
| `sum` | `f64` | 值总和 |
| `min` | `f64` | 最小值 |
| `max` | `f64` | 最大值 |
| `mean` | `f64` | 平均值 |
| `std_dev` | `f64` | 标准差 |

### 指标导出

#### Prometheus格式

```rust
use dmsc::prelude::*;

let registry = DMSCMetricsRegistry::new();
// ... 注册指标

// 导出Prometheus格式的指标
let prometheus_metrics = registry.export_prometheus();
println!("{}", prometheus_metrics);
```

<div align="center">

## 分布式追踪

</div>

### DMSCTracer

分布式追踪器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的追踪器 | 无 | `Self` |
| `start_trace(name)` | 开始追踪 | `name: String` | `Option<DMSCTraceId>` |
| `start_span(...)` | 开始跨度 | 多种参数 | `Option<DMSCSpanId>` |
| `end_span(id, status)` | 结束跨度 | `id: &DMSCSpanId`, `status: DMSCSpanStatus` | `DMSCResult<()>` |
| `span_mut(id, f)` | 修改跨度 | `id: &DMSCSpanId`, `f: F` | `DMSCResult<()>` |
| `export_traces()` | 导出追踪 | 无 | `HashMap<DMSCTraceId, Vec<DMSCSpan>>` |
| `active_trace_count()` | 活跃追踪数 | 无 | `usize` |
| `active_span_count()` | 活跃跨度数 | 无 | `usize` |

#### 使用示例

```rust
use dmsc::prelude::*;

let tracer = DMSCTracer::new();

// 开始追踪
let trace_id = tracer.start_trace("HTTP request".to_string())?;
assert!(trace_id.is_some());

// 开始子跨度
let span_id = tracer.start_span(
    trace_id.as_ref(),
    None,
    "database_query".to_string(),
    DMSCSpanKind::Client
)?;
assert!(span_id.is_some());

// 结束跨度
tracer.end_span(&span_id, DMSCSpanStatus::Ok)?;
```

### DMSCTraceId

追踪ID类型。

```rust
/// 追踪ID类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSCTraceId(pub String);
```

### DMSCSpanId

跨度ID类型。

```rust
/// 跨度ID类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DMSCSpanId(pub String);
```

### DMSCSpanKind

跨度类型。

| 变体 | 描述 |
|:--------|:-------------|
| `Server` | 服务器端处理 |
| `Client` | 客户端请求 |
| `Producer` | 消息生产者 |
| `Consumer` | 消息消费者 |
| `Internal` | 内部操作 |

### DMSCSpanStatus

跨度状态。

| 变体 | 描述 |
|:--------|:-------------|
| `Unset` | 未设置 |
| `Ok` | 成功 |
| `Error` | 错误 |
| `DeadlineExceeded` | 超时 |

### DMSCSamplingStrategy

追踪采样策略。

| 变体 | 描述 |
|:--------|:-------------|
| `Rate(rate)` | 固定采样率 |
| `Deterministic(rate)` | 确定性采样 |
| `Adaptive(target_rate)` | 自适应采样 |

<div align="center">

## 追踪传播

</div>

### DMSCTraceContext

追踪上下文。

```rust
use dmsc::prelude::*;

let context = DMSCTraceContext::new()
    .with_trace_id("trace-123".to_string())
    .with_span_id("span-456".to_string());

// 设置为当前上下文
context.set_as_current();

// 获取当前上下文
if let Some(current) = DMSCTraceContext::current() {
    let trace_id = current.trace_id();
    let span_id = current.span_id();
}
```

### W3CTracePropagator

W3C追踪传播器。

```rust
use dmsc::prelude::*;

let propagator = W3CTracePropagator::new();

// 注入到载体
let mut carrier = HashMap::new();
propagator.inject(&context, &mut carrier);

// 从载体提取
let extracted = propagator.extract(&carrier);
```

<div align="center">

## Prometheus集成

</div>

### DMSCPrometheusExporter

Prometheus指标导出器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建新的导出器 | 无 | `Self` |
| `register_counter(name, help)` | 注册计数器 | `name: &str`, `help: &str` | `DMSCResult<()>` |
| `register_gauge(name, help)` | 注册计量器 | `name: &str`, `help: &str` | `DMSCResult<()>` |
| `register_histogram(name, help, buckets)` | 注册直方图 | 多种参数 | `DMSCResult<()>` |
| `increment_counter(name)` | 增加计数器 | `name: &str` | `DMSCResult<()>` |
| `set_gauge(name, value)` | 设置计量器 | `name: &str`, `value: f64` | `DMSCResult<()>` |
| `observe_histogram(name, value)` | 观察直方图 | `name: &str`, `value: f64` | `DMSCResult<()>` |
| `render()` | 渲染指标 | 无 | `DMSCResult<String>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let exporter = DMSCPrometheusExporter::new();

// 注册指标
exporter.register_counter("http_requests_total", "Total HTTP requests")?;
exporter.register_gauge("http_active_connections", "Active HTTP connections")?;
exporter.register_histogram(
    "http_request_duration_seconds",
    "HTTP request duration in seconds",
    vec![0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
)?;

// 记录指标
exporter.increment_counter("http_requests_total")?;
exporter.set_gauge("http_active_connections", 42.0)?;
exporter.observe_histogram("http_request_duration_seconds", 0.256)?;

// 导出Prometheus格式
let output = exporter.render()?;
println!("{}", output);
```

<div align="center">

## Grafana集成

</div>

### DMSCGrafanaDashboard

Grafana仪表板。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(title)` | 创建新仪表板 | `title: &str` | `Self` |
| `add_panel(panel)` | 添加面板 | `panel: DMSCGrafanaPanel` | `DMSCResult<()>` |
| `to_json()` | 导出JSON | 无 | `DMSCResult<String>` |

### DMSCGrafanaPanel

Grafana面板。

```rust
use dmsc::prelude::*;

let panel = DMSCGrafanaPanel {
    title: "Request Rate".to_string(),
    query: "rate(http_requests_total[5m])".to_string(),
    panel_type: "timeseries".to_string(),
    grid_pos: DMSCGridPos { h: 8, w: 12, x: 0, y: 0 },
};
```

### DMSCGrafanaDashboardGenerator

Grafana仪表板生成器。

```rust
use dmsc::prelude::*;

let mut generator = DMSCGrafanaDashboardGenerator::new();

// 生成默认仪表板
let dashboard = generator.generate_default_dashboard()?;

// 生成自动仪表板
let metrics = vec!["http_requests_total", "http_request_duration_seconds"];
let auto_dashboard = generator.generate_auto_dashboard(metrics, "Auto Dashboard")?;

// 导出JSON
let json = auto_dashboard.to_json()?;
```

<div align="center">

## 配置

</div>

### DMSCObservabilityConfig

可观测性模块配置。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `tracing_enabled` | `bool` | 启用追踪 | `true` |
| `metrics_enabled` | `bool` | 启用指标 | `true` |
| `tracing_sampling_rate` | `f64` | 追踪采样率 | `1.0` |
| `tracing_sampling_strategy` | `String` | 采样策略 | `"rate"` |
| `metrics_window_size_secs` | `u64` | 指标窗口大小(秒) | `60` |
| `metrics_bucket_size_secs` | `u64` | 指标桶大小(秒) | `10` |

#### 使用示例

```rust
use dmsc::prelude::*;

let config = DMSCObservabilityConfig {
    tracing_enabled: true,
    metrics_enabled: true,
    tracing_sampling_rate: 0.1,
    tracing_sampling_strategy: "adaptive".to_string(),
    metrics_window_size_secs: 300,
    metrics_bucket_size_secs: 30,
};

let observability = DMSCObservabilityModule::new(config);
```;

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
| `run_checks()` | 执行所有检查 | 无 | `DMSCResult<DMSCHealthReport>` |
| `get_status()` | 获取健康状态 | 无 | `DMSCHealthStatus` |

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
        DMSCHealthStatus::Healthy => println!("{}: ✓", name),
        DMSCHealthStatus::Degraded => println!("{}: ⚠ ({})", name, result.message),
        DMSCHealthStatus::Unhealthy => println!("{}: ✗ ({})", name, result.message),
    }
}
```

### 自定义健康检查

```rust
use dmsc::prelude::*;

struct CustomHealthCheck {
    threshold: f64,
}

impl DMSCHealthCheck for CustomHealthCheck {
    fn check(&self) -> DMSCResult<DMSCHealthCheckResult> {
        let current_value = self.get_current_value()?;
        
        if current_value < self.threshold {
            Ok(DMSCHealthCheckResult::healthy())
        } else {
            Ok(DMSCHealthCheckResult::unhealthy(format!(
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
    tracing_sampling_rate: 0.1,
    tracing_sampling_strategy: "adaptive".to_string(),
    metrics_window_size_secs: 300,
    metrics_bucket_size_secs: 30,
};

let observability = DMSCObservabilityModule::new(observability_config);
```

<div align="center">

## 最佳实践

</div>

1. **合理配置采样率**：根据流量大小调整追踪采样率，避免过多开销
2. **使用合适的指标类型**：根据指标特性选择Counter、Gauge或Histogram
3. **添加有意义的标签**：使用标签区分不同维度，但避免基数过高的标签
4. **定期导出和清理**：配置指标导出间隔，避免内存占用过大
5. **使用上下文传播**：在分布式系统中正确传播追踪上下文
6. **使用安全的锁获取方法**：使用 `read_safe()` 和 `write_safe()` 替代 `.read()` 和 `.write()`

<div align="center">

## 相关模块

</div>

- [core](./core.md): 核心模块，提供错误处理和运行时支持
- [gateway](./gateway.md): 网关模块，提供API网关功能
- [database](./database.md): 数据库模块，提供数据库操作支持
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信