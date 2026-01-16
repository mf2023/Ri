<div align="center">

# 可观测性使用示例

**Version: 0.1.4**

**Last modified date: 2026-01-15**

本示例展示如何使用DMSC的observability模块进行分布式追踪、指标收集、健康检查、性能分析和告警管理的使用。

## 示例概述

</div>

本示例将创建一个DMSC应用，实现以下功能：

- 分布式追踪和span管理
- 指标收集和性能监控
- 健康检查和状态监控
- 日志聚合和分析
- 告警管理和通知
- 性能分析和优化

<div align="center">

## 前置要求

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的Rust编程知识
- 了解可观测性基本概念
- （可选）Jaeger、Prometheus等监控工具

<div align="center">

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-observability-example
cd dms-observability-example
```

### 2. 添加依赖

在`Cargo.toml`文件中添加以下依赖：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
futures = "0.3"
chrono = "0.4"
```

### 3. 创建配置文件

在项目根目录创建`config.yaml`文件：

```yaml
service:
  name: "dms-observability-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

observability:
  tracing:
    enabled: true
    service_name: "dms-observability-example"
    sampling_rate: 1.0
    jaeger_endpoint: "http://localhost:14268/api/traces"
  metrics:
    enabled: true
    prometheus_endpoint: "0.0.0.0:9090"
    collect_interval: 15
  health_check:
    enabled: true
    endpoint: "/health"
    interval: 30
  alerting:
    enabled: true
    webhook_url: "http://localhost:8080/alerts"
    thresholds:
      error_rate: 0.05
      response_time: 1000
      cpu_usage: 0.8
      memory_usage: 0.85
```

### 4. 编写主代码

将`src/main.rs`文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde_json::json;
use futures::future::join_all;
use chrono::Utc;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // 构建服务运行时
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // 运行业务逻辑
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Observability Example started")?;
        
        // 初始化可观测性
        initialize_observability(&ctx).await?;
        
        // 演示分布式追踪
        demonstrate_distributed_tracing(&ctx).await?;
        
        // 演示指标收集
        demonstrate_metrics_collection(&ctx).await?;
        
        // 演示健康检查
        demonstrate_health_checks(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Observability Example completed")?;
        
        Ok(())
    }).await
}

async fn initialize_observability(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Initializing observability")?;
    
    // 创建根span
    let root_span = ctx.observability().start_span("initialize_observability", json!({
        "service_name": "dms-observability-example",
        "timestamp": Utc::now().to_rfc3339(),
    }));
    
    // 初始化追踪
    ctx.observability().initialize_tracing()?;
    ctx.logger().info("observability", "Tracing initialized")?;
    
    // 初始化指标
    ctx.observability().initialize_metrics()?;
    ctx.logger().info("observability", "Metrics initialized")?;
    
    // 初始化健康检查
    ctx.observability().initialize_health_check()?;
    ctx.logger().info("observability", "Health check initialized")?;
    
    root_span.end();
    
    Ok(())
}

async fn demonstrate_distributed_tracing(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Demonstrating distributed tracing")?;
    
    // 创建根span
    let root_span = ctx.observability().start_span("process_order", json!({
        "order_id": "ORD-12345",
        "user_id": "user-67890",
        "total_amount": 99.99,
    }));
    
    // 在span中执行操作
    match process_order_example("ORD-12345", ctx).await {
        Ok(result) => {
            root_span.set_attribute("status", "success");
            root_span.set_attribute("result", json!(result));
            ctx.logger().info("observability", "Order processed successfully")?;
        }
        Err(e) => {
            root_span.set_attribute("status", "error");
            root_span.set_attribute("error.message", e.to_string());
            root_span.record_exception(&e);
            ctx.logger().error("observability", &format!("Order processing failed: {}", e))?;
        }
    }
    
    root_span.end();
    
    Ok(())
}

async fn process_order_example(order_id: &str, ctx: &DMSCServiceContext) -> DMSCResult<serde_json::Value> {
    let order_span = ctx.observability().start_span("fetch_order", json!({
        "order_id": order_id,
    }));
    
    // 模拟订单获取
    tokio::time::sleep(Duration::from_millis(100)).await;
    let order = json!({
        "id": order_id,
        "user_id": "user-67890",
        "total_amount": 99.99,
        "status": "pending",
    });
    
    order_span.end();
    
    // 验证订单
    let validation_span = ctx.observability().start_span("validate_order", json!({
        "order_id": order_id,
        "user_id": order["user_id"],
    }));
    
    tokio::time::sleep(Duration::from_millis(50)).await;
    validation_span.end();
    
    Ok(json!({
        "order_id": order_id,
        "status": "completed",
        "processed_at": Utc::now().to_rfc3339(),
    }))
}

async fn demonstrate_metrics_collection(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Demonstrating metrics collection")?;
    
    // 创建计数器
    let request_counter = ctx.observability().create_counter("http_requests_total", "Total HTTP requests", "requests");
    let order_counter = ctx.observability().create_counter("orders_processed_total", "Total orders processed", "orders");
    
    // 模拟HTTP请求
    for i in 0..10 {
        request_counter.increment(1, json!({
            "method": "GET",
            "path": "/api/orders",
            "status_code": 200,
        }))?;
        
        // 模拟订单处理
        match process_order_example(&format!("ORD-{}", i), ctx).await {
            Ok(_) => {
                order_counter.increment(1, json!({
                    "status": "success",
                    "payment_method": "credit_card",
                }))?;
            }
            Err(_) => {
                order_counter.increment(1, json!({
                    "status": "failed",
                    "payment_method": "credit_card",
                }))?;
            }
        }
    }
    
    ctx.logger().info("observability", "Metrics collection demonstration completed")?;
    
    Ok(())
}

async fn demonstrate_health_checks(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Demonstrating health checks")?;
    
    // 执行健康检查
    let health_status = ctx.observability().perform_health_check().await?;
    
    ctx.logger().info("observability", &format!("Health check status: {:?}", health_status))?;
    
    Ok(())
}
```

<div align="center">

## 代码解析

</div>

observability模块提供分布式追踪、指标收集、健康检查、性能分析和告警管理的实际使用示例。

## 分布式追踪

### 基本追踪

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建根span
let root_span = ctx.observability().start_span("process_order", json!({
    "order_id": "ORD-12345",
    "user_id": "user-67890",
    "total_amount": 99.99,
}));

// 在span中执行操作
match process_order("ORD-12345").await {
    Ok(result) => {
        root_span.set_attribute("status", "success");
        root_span.set_attribute("result", json!(result));
        ctx.log().info("Order processed successfully");
    }
    Err(e) => {
        root_span.set_attribute("status", "error");
        root_span.set_attribute("error.message", e.to_string());
        root_span.record_exception(&e);
        ctx.log().error(format!("Order processing failed: {}", e));
    }
}

// 结束span
root_span.end();
```

### 嵌套span

```rust
use dmsc::prelude::*;

async fn process_order(order_id: &str) -> DMSCResult<OrderResult> {
    let order_span = ctx.observability().start_span("fetch_order", json!({
        "order_id": order_id,
    }));
    
    // 获取订单数据
    let order = fetch_order_from_db(order_id).await?;
    order_span.end();
    
    // 验证订单
    let validation_span = ctx.observability().start_span("validate_order", json!({
        "order_id": order_id,
        "user_id": order.user_id,
    }));
    
    validate_order(&order).await?;
    validation_span.end();
    
    // 处理支付
    let payment_span = ctx.observability().start_span("process_payment", json!({
        "order_id": order_id,
        "amount": order.total_amount,
        "currency": order.currency,
    }));
    
    let payment_result = process_payment(&order).await?;
    payment_span.set_attribute("payment_id", payment_result.payment_id.clone());
    payment_span.end();
    
    // 更新库存
    let inventory_span = ctx.observability().start_span("update_inventory", json!({
        "order_id": order_id,
        "items_count": order.items.len(),
    }));
    
    update_inventory(&order.items).await?;
    inventory_span.end();
    
    Ok(OrderResult {
        order_id: order_id.to_string(),
        payment_id: payment_result.payment_id,
        status: "completed".to_string(),
    })
}
```

### 异步追踪

```rust
use dmsc::prelude::*;
use futures::future::join_all;

async fn batch_process_orders(order_ids: Vec<String>) -> DMSCResult<Vec<OrderResult>> {
    let batch_span = ctx.observability().start_span("batch_process_orders", json!({
        "order_count": order_ids.len(),
        "orders": order_ids.clone(),
    }));
    
    // 创建异步任务
    let tasks: Vec<_> = order_ids.into_iter().map(|order_id| {
        let span = ctx.observability().start_span("process_single_order", json!({
            "order_id": order_id.clone(),
        }));
        
        // 将span上下文传递给异步任务
        let span_context = span.get_context();
        
        async move {
            // 在异步任务中恢复span上下文
            let task_span = ctx.observability().span_from_context(&span_context);
            
            let result = process_order(&order_id).await;
            
            match &result {
                Ok(_) => task_span.set_attribute("status", "success"),
                Err(e) => {
                    task_span.set_attribute("status", "error");
                    task_span.set_attribute("error.message", e.to_string());
                }
            }
            
            task_span.end();
            result
        }
    }).collect();
    
    // 并行执行所有任务
    let results = join_all(tasks).await;
    
    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    let failed_count = results.len() - successful_count;
    
    batch_span.set_attribute("successful_count", successful_count as i64);
    batch_span.set_attribute("failed_count", failed_count as i64);
    batch_span.end();
    
    results.into_iter().collect()
}
```

## 指标收集

### 计数器指标

```rust
use dmsc::prelude::*;

// 创建计数器
let request_counter = ctx.observability().create_counter("http_requests_total", "Total HTTP requests", "requests");

// 在HTTP中间件中使用
async fn http_middleware(req: DMSCHttpRequest, next: Next) -> DMSCResult<DMSCHttpResponse> {
    // 记录请求计数
    request_counter.increment(1, json!({
        "method": req.method.clone(),
        "path": req.path.clone(),
        "user_agent": req.headers.get("user-agent").unwrap_or(&"unknown".to_string()).clone(),
    }));
    
    let start_time = std::time::Instant::now();
    let response = next.run(req).await?;
    let duration = start_time.elapsed();
    
    // 记录响应时间
    let histogram = ctx.observability().create_histogram("http_request_duration_seconds", "HTTP request duration", "seconds");
    histogram.record(duration.as_secs_f64(), json!({
        "method": response.method.clone(),
        "path": response.path.clone(),
        "status_code": response.status_code,
    }));
    
    Ok(response)
}

// 业务指标计数
let order_counter = ctx.observability().create_counter("orders_processed_total", "Total orders processed", "orders");
let user_counter = ctx.observability().create_counter("users_registered_total", "Total users registered", "users");

// 在业务逻辑中使用
match process_order(&order).await {
    Ok(_) => {
        order_counter.increment(1, json!({
            "status": "success",
            "payment_method": order.payment_method.clone(),
            "currency": order.currency.clone(),
        }));
    }
    Err(_) => {
        order_counter.increment(1, json!({
            "status": "failed",
            "payment_method": order.payment_method.clone(),
            "currency": order.currency.clone(),
        }));
    }
}
```

### 直方图指标

```rust
use dmsc::prelude::*;

// 创建直方图
let request_size_histogram = ctx.observability().create_histogram("http_request_size_bytes", "HTTP request size", "bytes");
let response_size_histogram = ctx.observability().create_histogram("http_response_size_bytes", "HTTP response size", "bytes");
let db_query_histogram = ctx.observability().create_histogram("database_query_duration_seconds", "Database query duration", "seconds");

// 记录请求大小
request_size_histogram.record(req.body.len() as f64, json!({
    "method": req.method.clone(),
    "path": req.path.clone(),
}));

// 记录数据库查询时间
async fn execute_query(query: &str) -> DMSCResult<QueryResult> {
    let start_time = std::time::Instant::now();
    
    let result = ctx.database().query(query, vec![]).await?;
    
    let duration = start_time.elapsed();
    db_query_histogram.record(duration.as_secs_f64(), json!({
        "query_type": get_query_type(query),
        "table": extract_table_name(query),
        "success": true,
    }));
    
    Ok(result)
}

// 自定义直方图桶
let custom_histogram = ctx.observability().create_histogram_with_buckets(
    "custom_metric_seconds",
    "Custom metric with custom buckets",
    "seconds",
    vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0] // 自定义桶
);
```

### 仪表盘指标

```rust
use dmsc::prelude::*;

// 创建仪表盘
let active_users_gauge = ctx.observability().create_gauge("active_users_current", "Current active users", "users");
let system_load_gauge = ctx.observability().create_gauge("system_load_average", "System load average", "load");
let memory_usage_gauge = ctx.observability().create_gauge("memory_usage_bytes", "Memory usage", "bytes");

// 定期更新仪表盘
async fn update_system_metrics() -> DMSCResult<()> {
    loop {
        // 获取活跃用户数
        let active_users = get_active_user_count().await?;
        active_users_gauge.set(active_users as f64, json!({
            "source": "websocket_connections",
        }));
        
        // 获取系统负载
        let load_avg = get_system_load_average()?;
        system_load_gauge.set(load_avg, json!({
            "time_window": "1m",
        }));
        
        // 获取内存使用
        let memory_usage = get_memory_usage()?;
        memory_usage_gauge.set(memory_usage as f64, json!({
            "type": "heap",
        }));
        
        sleep(Duration::from_secs(10)).await;
    }
}

// 业务仪表盘
let order_queue_gauge = ctx.observability().create_gauge("order_queue_size", "Order queue size", "orders");
let processing_rate_gauge = ctx.observability().create_gauge("order_processing_rate_per_second", "Order processing rate", "orders/s");

// 更新业务指标
order_queue_gauge.set(get_order_queue_size() as f64, json!({
    "priority": "high",
}));

processing_rate_gauge.set(calculate_processing_rate(), json!({
    "time_window": "1m",
}));
```

## 健康检查

### 系统健康检查

```rust
use dmsc::prelude::*;
use serde_json::json;

// 注册健康检查
ctx.observability().register_health_check("database", || async {
    match ctx.database().ping().await {
        Ok(_) => Ok(json!({
            "status": "healthy",
            "response_time_ms": 15,
            "connections": {
                "active": 10,
                "idle": 5,
                "max": 20,
            }
        })),
        Err(e) => Err(DMSCError::internal(format!("Database ping failed: {}", e))),
    }
}).await?;

ctx.observability().register_health_check("cache", || async {
    match ctx.cache().health_check() {
        Ok(health) => Ok(json!({
            "status": if health.is_healthy { "healthy" } else { "unhealthy" },
            "backend": health.backend,
            "memory_usage": health.memory_usage,
            "hit_rate": health.hit_rate,
        })),
        Err(e) => Err(DMSCError::internal(format!("Cache health check failed: {}", e))),
    }
}).await?;

ctx.observability().register_health_check("external_api", || async {
    match check_external_api_health().await {
        Ok(response_time) => Ok(json!({
            "status": "healthy",
            "response_time_ms": response_time,
            "api_endpoint": "https://api.example.com/health",
        })),
        Err(e) => Err(DMSCError::internal(format!("External API health check failed: {}", e))),
    }
}).await?;

// 获取整体健康状态
let health_status = ctx.observability().get_health_status().await?;
ctx.log().info(format!("System health: {:?}", health_status));

// 健康检查端点
async fn health_check_handler() -> DMSCResult<DMSCHttpResponse> {
    let health = ctx.observability().get_health_status().await?;
    
    let status_code = if health.overall_status == "healthy" {
        200
    } else if health.overall_status == "degraded" {
        503
    } else {
        503
    };
    
    Ok(DMSCHttpResponse {
        status_code,
        headers: json!({
            "Content-Type": "application/json",
        }),
        body: json!(health).to_string(),
    })
}
```

### 自定义健康检查

```rust
use dmsc::prelude::*;
use serde_json::json;

// 业务逻辑健康检查
ctx.observability().register_health_check("order_processing", || async {
    let queue_size = get_order_queue_size().await?;
    let processing_rate = get_order_processing_rate().await?;
    
    let status = if queue_size > 1000 {
        "unhealthy"
    } else if queue_size > 500 {
        "degraded"
    } else {
        "healthy"
    };
    
    Ok(json!({
        "status": status,
        "queue_size": queue_size,
        "processing_rate_per_minute": processing_rate,
        "thresholds": {
            "healthy": "< 500",
            "degraded": "500-1000",
            "unhealthy": "> 1000",
        }
    }))
}).await?;

// 磁盘空间健康检查
ctx.observability().register_health_check("disk_space", || async {
    let disk_usage = get_disk_usage()?;
    let free_space_gb = disk_usage.free_gb;
    let used_percentage = disk_usage.used_percentage;
    
    let status = if used_percentage > 95 {
        "unhealthy"
    } else if used_percentage > 85 {
        "degraded"
    } else {
        "healthy"
    };
    
    Ok(json!({
        "status": status,
        "free_space_gb": free_space_gb,
        "used_percentage": used_percentage,
        "total_space_gb": disk_usage.total_gb,
        "thresholds": {
            "healthy": "< 85%",
            "degraded": "85-95%",
            "unhealthy": "> 95%",
        }
    }))
}).await?;

// 内存使用健康检查
ctx.observability().register_health_check("memory_usage", || async {
    let memory_stats = get_memory_stats()?;
    let used_percentage = (memory_stats.used as f64 / memory_stats.total as f64) * 100.0;
    
    let status = if used_percentage > 90 {
        "unhealthy"
    } else if used_percentage > 75 {
        "degraded"
    } else {
        "healthy"
    };
    
    Ok(json!({
        "status": status,
        "used_mb": memory_stats.used / 1024 / 1024,
        "total_mb": memory_stats.total / 1024 / 1024,
        "used_percentage": used_percentage,
        "thresholds": {
            "healthy": "< 75%",
            "degraded": "75-90%",
            "unhealthy": "> 90%",
        }
    }))
}).await?;
```

## 性能分析

### 性能剖析

```rust
use dmsc::prelude::*;
use serde_json::json;

// 开始性能剖析
let profile_id = ctx.observability().start_profiling("order_processing_profile", json!({
    "order_id": "ORD-12345",
    "profile_type": "cpu",
    "duration_seconds": 30,
}));

// 执行需要剖析的操作
let result = process_order("ORD-12345").await?;

// 结束性能剖析
let profile_data = ctx.observability().stop_profiling(profile_id)?;

// 分析剖析结果
ctx.log().info(format!("Profile completed: {:?}", profile_data));

// 获取性能热点
let hotspots = ctx.observability().get_performance_hotspots()?;
for hotspot in hotspots {
    ctx.log().info(format!("Hotspot: {} - {} samples ({}%)", 
        hotspot.function, 
        hotspot.sample_count, 
        hotspot.percentage
    ));
}
```

### 内存分析

```rust
use dmsc::prelude::*;
use serde_json::json;

// 开始内存分析
let memory_profile_id = ctx.observability().start_memory_profiling("order_processing_memory", json!({
    "order_id": "ORD-12345",
    "profile_type": "heap",
    "track_allocations": true,
}));

// 执行内存密集型操作
let large_dataset = generate_large_dataset().await?;
process_large_dataset(large_dataset).await?;

// 结束内存分析
let memory_profile = ctx.observability().stop_memory_profiling(memory_profile_id)?;

// 分析内存使用
ctx.log().info(format!("Memory profile: {:?}", memory_profile));

// 获取内存泄漏检测
let leaks = ctx.observability().detect_memory_leaks()?;
if !leaks.is_empty() {
    ctx.log().warn(format!("Detected {} potential memory leaks", leaks.len()));
    for leak in leaks {
        ctx.log().warn(format!("Potential leak: {} - {} bytes", leak.location, leak.size));
    }
}
```

### 性能基准测试

```rust
use dmsc::prelude::*;
use serde_json::json;

// 运行性能基准测试
let benchmark_config = DMSCBenchmarkConfig {
    name: "order_processing_benchmark".to_string(),
    iterations: 1000,
    warmup_iterations: 100,
    concurrent_requests: 10,
    metrics: vec!["latency".to_string(), "throughput".to_string(), "cpu_usage".to_string()],
};

let benchmark_result = ctx.observability().run_benchmark(benchmark_config, || async {
    // 基准测试代码
    let order = create_test_order().await?;
    process_order(&order).await
}).await?;

// 分析基准测试结果
ctx.log().info(format!("Benchmark completed: {:?}", benchmark_result));

// 获取性能统计
let stats = benchmark_result.statistics;
ctx.log().info(format!("Average latency: {:.2}ms", stats.average_latency_ms));
ctx.log().info(format!("P95 latency: {:.2}ms", stats.p95_latency_ms));
ctx.log().info(format!("P99 latency: {:.2}ms", stats.p99_latency_ms));
ctx.log().info(format!("Throughput: {:.2} req/s", stats.throughput_rps));
ctx.log().info(format!("Error rate: {:.2}%", stats.error_rate * 100.0));
```

## 告警管理

### 创建告警规则

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建CPU使用率告警
let cpu_alert_rule = DMAlertRule {
    name: "high_cpu_usage".to_string(),
    metric: "cpu_usage_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(80.0),
    duration: Duration::from_secs(300), // 持续5分钟
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["email".to_string(), "slack".to_string()],
    labels: json!({
        "component": "application_server",
        "environment": "production",
    }),
};

ctx.observability().create_alert_rule(cpu_alert_rule).await?;

// 创建内存使用率告警
let memory_alert_rule = DMAlertRule {
    name: "high_memory_usage".to_string(),
    metric: "memory_usage_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(85.0),
    duration: Duration::from_secs(600), // 持续10分钟
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["email".to_string(), "pagerduty".to_string()],
    labels: json!({
        "component": "application_server",
        "environment": "production",
    }),
};

ctx.observability().create_alert_rule(memory_alert_rule).await?;

// 创建错误率告警
let error_rate_alert_rule = DMAlertRule {
    name: "high_error_rate".to_string(),
    metric: "error_rate_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(5.0),
    duration: Duration::from_secs(180), // 持续3分钟
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["slack".to_string()],
    labels: json!({
        "component": "api",
        "service": "order_service",
    }),
};

ctx.observability().create_alert_rule(error_rate_alert_rule).await?;
```

### 业务指标告警

```rust
use dmsc::prelude::*;
use serde_json::json;

// 订单处理延迟告警
let order_latency_alert = DMAlertRule {
    name: "order_processing_high_latency".to_string(),
    metric: "order_processing_duration_seconds".to_string(),
    condition: DMAlertCondition::GreaterThan(30.0), // 超过30秒
    duration: Duration::from_secs(600), // 持续10分钟
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["email".to_string(), "slack".to_string()],
    labels: json!({
        "component": "order_service",
        "metric_type": "latency",
    }),
};

ctx.observability().create_alert_rule(order_latency_alert).await?;

// 队列积压告警
let queue_backlog_alert = DMAlertRule {
    name: "order_queue_backlog".to_string(),
    metric: "order_queue_size".to_string(),
    condition: DMAlertCondition::GreaterThan(1000), // 超过1000个订单
    duration: Duration::from_secs(300), // 持续5分钟
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["pagerduty".to_string(), "email".to_string()],
    labels: json!({
        "component": "order_service",
        "queue": "order_processing",
    }),
};

ctx.observability().create_alert_rule(queue_backlog_alert).await?;

// 自定义业务逻辑告警
let fraud_detection_alert = DMAlertRule {
    name: "suspicious_fraud_pattern".to_string(),
    metric: "fraud_detection_score".to_string(),
    condition: DMAlertCondition::GreaterThan(0.8), // 欺诈分数超过0.8
    duration: Duration::from_secs(120), // 持续2分钟
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["email".to_string(), "security_team".to_string()],
    labels: json!({
        "component": "fraud_detection",
        "type": "security",
    }),
};

ctx.observability().create_alert_rule(fraud_detection_alert).await?;
```

### 告警通知

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置通知渠道
let email_config = DMNotificationChannelConfig {
    channel_type: "email".to_string(),
    name: "operations_team".to_string(),
    config: json!({
        "smtp_server": "smtp.company.com",
        "smtp_port": 587,
        "username": "alerts@company.com",
        "password": "smtp_password",
        "recipients": ["ops-team@company.com", "oncall@company.com"],
    }),
};

ctx.observability().create_notification_channel(email_config).await?;

let slack_config = DMNotificationChannelConfig {
    channel_type: "slack".to_string(),
    name: "engineering_alerts".to_string(),
    config: json!({
        "webhook_url": "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK",
        "channel": "#engineering-alerts",
        "username": "DMSC-AlertBot",
        "icon_emoji": ":warning:",
    }),
};

ctx.observability().create_notification_channel(slack_config).await?;

// 处理告警通知
async fn handle_alert_notification(alert: DMAlert) -> DMSCResult<()> {
    ctx.log().warn(format!("Alert triggered: {} - {}", alert.name, alert.severity));
    
    // 发送邮件通知
    if alert.notification_channels.contains(&"email".to_string()) {
        let email_body = format!(
            "Alert: {}\nSeverity: {}\nDescription: {}\nTime: {}\nLabels: {:?}",
            alert.name,
            alert.severity,
            alert.description,
            alert.triggered_at,
            alert.labels
        );
        
        send_email_notification("alerts@company.com", email_body).await?;
    }
    
    // 发送Slack通知
    if alert.notification_channels.contains(&"slack".to_string()) {
        let slack_message = json!({
            "text": format!("🚨 Alert: {} - {}", alert.name, alert.severity),
            "attachments": [{
                "color": get_alert_color(&alert.severity),
                "fields": [
                    {"title": "Description", "value": alert.description, "short": true},
                    {"title": "Time", "value": alert.triggered_at.to_string(), "short": true},
                    {"title": "Labels", "value": format!("{:?}", alert.labels), "short": false}
                ]
            }]
        });
        
        send_slack_notification(slack_message).await?;
    }
    
    Ok(())
}
```

## 监控集成

### Prometheus集成

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置Prometheus导出器
let prometheus_config = DMPrometheusConfig {
    enabled: true,
    port: 9090,
    path: "/metrics".to_string(),
    namespace: "dms_app".to_string(),
    labels: json!({
        "environment": "production",
        "service": "order_service",
        "version": "1.0.0",
    }),
};

ctx.observability().configure_prometheus(prometheus_config).await?;

// 自定义Prometheus指标
let custom_metric = ctx.observability().create_prometheus_counter(
    "custom_business_metric_total",
    "Custom business metric",
    vec!["label1".to_string(), "label2".to_string()]
);

custom_metric.increment(1.0, vec!["value1".to_string(), "value2".to_string()]);
```

### Grafana集成

```rust
use dmsc::prelude::*;
use serde_json::json;

// 创建Grafana仪表盘配置
let grafana_dashboard = DMGrafanaDashboard {
    title: "DMSC Application Dashboard".to_string(),
    description: "Main application monitoring dashboard".to_string(),
    tags: vec!["dms".to_string(), "production".to_string()],
    panels: vec![
        DMGrafanaPanel {
            title: "Request Rate".to_string(),
            panel_type: "graph".to_string(),
            targets: vec![
                DMGrafanaTarget {
                    expr: "rate(http_requests_total[5m])".to_string(),
                    legend_format: "{{method}} {{status_code}}".to_string(),
                }
            ],
        },
        DMGrafanaPanel {
            title: "Error Rate".to_string(),
            panel_type: "singlestat".to_string(),
            targets: vec![
                DMGrafanaTarget {
                    expr: "rate(http_requests_total{status_code=~\"5..\"}[5m]) / rate(http_requests_total[5m]) * 100".to_string(),
                    legend_format: "Error Rate %".to_string(),
                }
            ],
        },
        DMGrafanaPanel {
            title: "Response Time".to_string(),
            panel_type: "heatmap".to_string(),
            targets: vec![
                DMGrafanaTarget {
                    expr: "http_request_duration_seconds_bucket".to_string(),
                    legend_format: "".to_string(),
                }
            ],
        },
    ],
};

ctx.observability().create_grafana_dashboard(grafana_dashboard).await?;
```

## 错误处理

### 可观测性错误处理

```rust
use dmsc::prelude::*;
use serde_json::json;

// 处理追踪错误
match ctx.observability().start_span("operation", json!({"key": "value"})) {
    Ok(span) => {
        // 正常处理
        span.end();
    }
    Err(DMSCError::ObservabilityTracingError(e)) => {
        ctx.log().error(format!("Tracing error: {}", e));
        // 降级处理：不使用追踪
        perform_operation_without_tracing().await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected error: {}", e));
        return Err(e);
    }
}

// 处理指标收集错误
match ctx.observability().create_counter("metric_name", "description", "unit") {
    Ok(counter) => {
        counter.increment(1.0, json!({"label": "value"}));
    }
    Err(DMSCError::ObservabilityMetricsError(e)) => {
        ctx.log().warn(format!("Metrics collection error: {}", e));
        // 降级：记录日志而不是指标
        ctx.log().info("Metric: metric_name increment by 1");
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected error: {}", e));
        return Err(e);
    }
}

// 健康检查降级
ctx.observability().register_health_check("critical_service", || async {
    match check_critical_service().await {
        Ok(health) => Ok(health),
        Err(DMSCError::ObservabilityHealthCheckError(e)) => {
            ctx.log().warn(format!("Health check error: {}", e));
            // 返回降级状态而不是错误
            Ok(json!({
                "status": "unknown",
                "error": e.to_string(),
                "degraded": true,
            }))
        }
        Err(e) => {
            ctx.log().error(format!("Service check failed: {}", e));
            Err(e)
        }
    }
}).await?;
```

## 最佳实践

1. **合理使用Span**: 为重要操作创建span，避免过度追踪
2. **添加有意义的属性**: 在span中包含有助于调试的业务数据
3. **统一指标命名**: 使用一致的命名约定，便于查询和聚合
4. **设置合适的告警阈值**: 避免过于敏感或迟钝的告警
5. **定期审查告警**: 移除不再需要的告警，调整阈值
6. **健康检查分层**: 区分系统级、应用级和业务级健康检查
7. **性能剖析采样**: 在生产环境中使用采样，避免过度开销
8. **监控可观测性本身**: 监控追踪、指标和告警系统的健康状况
9. **文档化指标**: 记录所有自定义指标的含义和用途
10. **测试告警**: 定期测试告警系统，确保其正常工作
11. **使用结构化日志**: 统一日志格式，便于日志分析和搜索
12. **监控关键业务指标**: 不仅监控技术指标，还要监控业务KPI
13. **设置告警升级策略**: 配置告警升级路径，确保重要问题得到及时处理
14. **使用标签和维度**: 为指标添加丰富的标签，支持多维度分析
15. **定期性能调优**: 基于监控数据持续优化系统性能

<div align="center">

## 运行步骤

</div>

1. **环境准备**: 确保已安装Rust开发环境
2. **创建项目**: 使用 `cargo new observability-example` 创建新项目
3. **添加依赖**: 在 `Cargo.toml` 中添加 dms 依赖
4. **创建配置**: 复制上述配置代码到 `src/config.rs`
5. **运行示例**: 执行 `cargo run` 启动应用

<div align="center">

## 预期结果

</div>

运行成功后，您将看到以下输出：

```
[2024-01-01 12:00:00] INFO: DMSC Observability Example starting...
[2024-01-01 12:00:00] INFO: Jaeger tracing enabled on http://localhost:14268
[2024-01-01 12:00:00] INFO: Prometheus metrics exposed on http://localhost:9090/metrics
[2024-01-01 12:00:00] INFO: Health check endpoint available at http://localhost:8080/health
[2024-01-01 12:00:00] INFO: Starting distributed trace example...
[2024-01-01 12:00:00] INFO: Span created: user_registration
[2024-01-01 12:00:00] INFO: Metric recorded: user_registered_total{source="web"}
[2024-01-01 12:00:00] INFO: Health check passed: database
[2024-01-01 12:00:00] INFO: Alert triggered: high_error_rate
[2024-01-01 12:00:00] INFO: Email notification sent to alerts@company.com
[2024-01-01 12:00:00] INFO: Application running successfully!
```

<div align="center">

## 扩展功能

</div>

### 分布式追踪增强

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置高级追踪选项
let advanced_tracing = DMTracingConfig {
    service_name: "order_service".to_string(),
    jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
    sampling_rate: 0.1, // 10%采样率
    max_spans_per_second: 1000,
    span_buffer_size: 10000,
    enable_baggage: true,
    baggage_restriction_config: json!({
        "max_value_length": 2048,
        "max_entries": 64,
    }),
    tags: json!({
        "environment": "production",
        "region": "us-east-1",
        "cluster": "prod-cluster-1",
    }),
};

ctx.observability().configure_advanced_tracing(advanced_tracing).await?;

// 创建跨服务追踪上下文
let cross_service_context = ctx.observability().create_cross_service_context(
    "user_service",
    json!({
        "trace_id": "abc123",
        "span_id": "def456",
        "baggage": {
            "user_id": "12345",
            "request_id": "req-789",
        }
    })
).await?;
```

### 智能告警系统

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置智能告警规则
let intelligent_alerts = vec![
    DMAlertRule {
        name: "anomaly_detection".to_string(),
        condition: DMAlertCondition::Anomaly {
            metric: "request_rate".to_string(),
            algorithm: "ewma".to_string(), // 指数加权移动平均
            sensitivity: 0.8,
            window: "10m".to_string(),
        },
        severity: DMAlertSeverity::Warning,
        notification_channels: vec!["slack".to_string(), "email".to_string()],
        cooldown: "30m".to_string(),
        auto_resolve: true,
        resolve_after: "15m".to_string(),
    },
    DMAlertRule {
        name: "predictive_alert".to_string(),
        condition: DMAlertCondition::Predictive {
            metric: "memory_usage".to_string(),
            algorithm: "linear_regression".to_string(),
            threshold: 0.85,
            prediction_window: "2h".to_string(),
        },
        severity: DMAlertSeverity::Info,
        notification_channels: vec!["slack".to_string()],
        cooldown: "1h".to_string(),
        auto_resolve: true,
        resolve_after: "30m".to_string(),
    },
];

ctx.observability().configure_intelligent_alerts(intelligent_alerts).await?;

// 配置告警升级策略
let escalation_policy = DMEscalationPolicy {
    name: "critical_service_escalation".to_string(),
    rules: vec![
        DMEscalationRule {
            severity: DMAlertSeverity::Critical,
            initial_delay: "0m".to_string(),
            escalation_steps: vec![
                DMEscalationStep {
                    delay: "5m".to_string(),
                    notification_channels: vec!["slack".to_string()],
                },
                DMEscalationStep {
                    delay: "15m".to_string(),
                    notification_channels: vec!["email".to_string(), "sms".to_string()],
                },
                DMEscalationStep {
                    delay: "30m".to_string(),
                    notification_channels: vec!["pagerduty".to_string()],
                },
            ],
        },
    ],
};

ctx.observability().configure_escalation_policy(escalation_policy).await?;
```

### 性能剖析集成

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置持续性能剖析
let profiling_config = DMProfilingConfig {
    enabled: true,
    endpoint: "http://localhost:4040".to_string(),
    sample_rate: 0.01, // 1%采样率
    profile_types: vec![
        "cpu".to_string(),
        "heap".to_string(),
        "goroutine".to_string(),
        "mutex".to_string(),
        "block".to_string(),
    ],
    upload_interval: "10s".to_string(),
    labels: json!({
        "service": "order_service",
        "environment": "production",
        "version": "1.0.0",
    }),
};

ctx.observability().configure_profiling(profiling_config).await?;

// 创建自定义性能指标
let custom_profile = ctx.observability().create_custom_profile(
    "business_logic_performance",
    json!({
        "function_name": "process_order",
        "threshold_ms": 100,
    })
).await?;

custom_profile.start_timer();
// 执行业务逻辑
process_order(order_data).await?;
custom_profile.end_timer();
```

### 业务指标监控

```rust
use dmsc::prelude::*;
use serde_json::json;

// 配置业务KPI监控
let business_metrics = vec![
    DMBusinessMetric {
        name: "revenue_per_minute".to_string(),
        metric_type: DMBusinessMetricType::Gauge,
        unit: "USD".to_string(),
        labels: vec!["product_category".to_string(), "region".to_string()],
        aggregation: "sum".to_string(),
        alert_threshold: Some(DMMetricThreshold {
            min_value: Some(1000.0),
            max_value: None,
            anomaly_detection: true,
        }),
    },
    DMBusinessMetric {
        name: "user_conversion_rate".to_string(),
        metric_type: DMBusinessMetricType::Histogram,
        unit: "percentage".to_string(),
        labels: vec!["traffic_source".to_string(), "device_type".to_string()],
        aggregation: "avg".to_string(),
        alert_threshold: Some(DMMetricThreshold {
            min_value: Some(0.02),
            max_value: None,
            anomaly_detection: true,
        }),
    },
];

ctx.observability().configure_business_metrics(business_metrics).await?;

// 记录业务事件
let business_event = DMBusinessEvent {
    event_type: "order_completed".to_string(),
    event_data: json!({
        "order_id": "ORD-12345",
        "user_id": "USER-67890",
        "amount": 299.99,
        "currency": "USD",
        "products": [
            {"id": "PROD-1", "name": "Premium Plan", "price": 199.99},
            {"id": "PROD-2", "name": "Add-on Service", "price": 100.00}
        ],
        "payment_method": "credit_card",
        "completion_time": "2024-01-01T12:00:00Z",
    }),
    correlation_id: "trace-abc123".to_string(),
};

ctx.observability().record_business_event(business_event).await?;

<div align="center">

## 总结

</div>

本示例展示了DMSC框架强大的可观测性功能，帮助您构建高度可监控和可调试的应用程序。通过分布式追踪、指标收集、健康检查、日志聚合和告警系统，您可以全面了解应用的运行状况。

### 核心功能

1. **分布式追踪**: 跨服务调用链追踪，支持Jaeger和OpenTelemetry
2. **指标收集**: Prometheus指标导出，支持自定义业务指标
3. **健康检查**: 多层次健康检查，包括系统、应用和业务层
4. **日志聚合**: 结构化日志收集，支持多种日志级别和格式
5. **性能剖析**: 运行时性能分析，识别性能瓶颈
6. **告警系统**: 智能告警规则，支持多种通知渠道
7. **Grafana集成**: 可视化监控仪表盘，实时展示关键指标
8. **错误处理**: 完善的错误处理和降级机制

### 高级特性

1. **智能告警**: 基于机器学习的异常检测和预测性告警
2. **业务指标**: 业务KPI监控，将技术指标与业务目标关联
3. **性能剖析**: 持续性能监控，自动识别性能问题
4. **跨服务追踪**: 支持复杂微服务架构的调用链追踪
5. **告警升级**: 多级告警升级策略，确保重要问题得到处理
6. **采样策略**: 可配置的追踪和性能剖析采样率，平衡开销和精度

### 最佳实践

- 为关键业务操作添加分布式追踪
- 设置合理的告警阈值和升级策略
- 监控业务指标而不仅仅是技术指标
- 定期审查和优化监控配置
- 使用结构化日志便于分析
- 实施健康检查的分层设计
- 测试告警系统的有效性
- 文档化所有自定义指标的含义

<div align="center">

## 相关模块

</div>

- [README](./README.md): 使用示例概览，提供所有使用示例的快速导航
- [authentication](./authentication.md): 认证示例，学习JWT、OAuth2和RBAC认证授权
- [basic-app](./basic-app.md): 基础应用示例，学习如何创建和运行第一个DMSC应用
- [caching](./caching.md): 缓存示例，了解如何使用缓存模块提升应用性能
- [database](./database.md): 数据库示例，学习数据库连接和查询操作
- [grpc](./grpc.md): gRPC 示例，实现高性能 RPC 调用
- [http](./http.md): HTTP服务示例，构建Web应用和RESTful API
- [mq](./mq.md): 消息队列示例，实现异步消息处理和事件驱动架构

- [security](./security.md): 安全示例，加密、哈希和安全最佳实践
- [storage](./storage.md): 存储示例，文件上传下载和存储管理
- [validation](./validation.md): 验证示例，数据验证和清理操作
- [websocket](./websocket.md): WebSocket 示例，实现实时双向通信