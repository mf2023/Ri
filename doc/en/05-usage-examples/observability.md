<div align="center">

# Observability Usage Examples

**Version: 0.1.6**

**Last modified date: 2026-01-30**

This example demonstrates how to use DMSC's observability module for distributed tracing, metrics collection, health checks, performance analysis, and alerting management.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- Distributed tracing and span management
- Metrics collection and performance monitoring
- Health checks and status monitoring
- Log aggregation and analysis
- Alerting management and notifications
- Performance analysis and optimization

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of observability basic concepts
- (Optional) Jaeger, Prometheus and other monitoring tools

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-observability-example
cd dms-observability-example
```

### 2. Add Dependencies

Add the following dependencies to your `Cargo.toml` file:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
futures = "0.3"
chrono = "0.4"
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root:

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

### 4. Write Main Code

Replace the content of `src/main.rs` with the following:

```rust
use dmsc::prelude::*;
use serde_json::json;
use futures::future::join_all;
use chrono::Utc;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    // Build service runtime
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_observability(DMSCObservabilityConfig::default())?
        .build()?;
    
    // Run business logic
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC Observability Example started")?;
        
        // Initialize observability
        initialize_observability(&ctx).await?;
        
        // Demonstrate distributed tracing
        demonstrate_distributed_tracing(&ctx).await?;
        
        // Demonstrate metrics collection
        demonstrate_metrics_collection(&ctx).await?;
        
        // Demonstrate health checks
        demonstrate_health_checks(&ctx).await?;
        
        ctx.logger().info("service", "DMSC Observability Example completed")?;
        
        Ok(())
    }).await
}

async fn initialize_observability(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Initializing observability")?;
    
    // Create root span
    let root_span = ctx.observability().start_span("initialize_observability", json!({
        "service_name": "dms-observability-example",
        "timestamp": Utc::now().to_rfc3339(),
    }));
    
    // Initialize tracing
    ctx.observability().initialize_tracing()?;
    ctx.logger().info("observability", "Tracing initialized")?;
    
    // Initialize metrics
    ctx.observability().initialize_metrics()?;
    ctx.logger().info("observability", "Metrics initialized")?;
    
    // Initialize health check
    ctx.observability().initialize_health_check()?;
    ctx.logger().info("observability", "Health check initialized")?;
    
    root_span.end();
    
    Ok(())
}

async fn demonstrate_distributed_tracing(ctx: &DMSCServiceContext) -> DMSCResult<()> {
    ctx.logger().info("observability", "Demonstrating distributed tracing")?;
    
    // Create root span
    let root_span = ctx.observability().start_span("process_order", json!({
        "order_id": "ORD-12345",
        "user_id": "user-67890",
        "total_amount": 99.99,
    }));
    
    // Execute operations in span
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
    
    // Simulate order fetching
    tokio::time::sleep(Duration::from_millis(100)).await;
    let order = json!({
        "id": order_id,
        "user_id": "user-67890",
        "total_amount": 99.99,
        "status": "pending",
    });
    
    order_span.end();
    
    // Validate order
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
    
    // Create counters
    let request_counter = ctx.observability().create_counter("http_requests_total", "Total HTTP requests", "requests");
    let order_counter = ctx.observability().create_counter("orders_processed_total", "Total orders processed", "orders");
    
    // Simulate HTTP requests
    for i in 0..10 {
        request_counter.increment(1, json!({
            "method": "GET",
            "path": "/api/orders",
            "status_code": 200,
        }))?;
        
        // Simulate order processing
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
    
    // Execute health check
    let health_status = ctx.observability().perform_health_check().await?;
    
    ctx.logger().info("observability", &format!("Health check status: {:?}", health_status))?;
    
    Ok(())
}
```

<div align="center">

## Code Explanation

</div>

The observability module provides practical usage examples for distributed tracing, metrics collection, health checks, performance analysis, and alerting management.

## Distributed Tracing

### Basic Tracing

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create root span
let root_span = ctx.observability().start_span("process_order", json!({
    "order_id": "ORD-12345",
    "user_id": "user-67890",
    "total_amount": 99.99,
}));

// Execute operations in span
match process_order("ORD-12345").await {
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

// End span
root_span.end();
```

### Nested Spans

```rust
use dmsc::prelude::*;

async fn process_order(order_id: &str) -> DMSCResult<OrderResult> {
    let order_span = ctx.observability().start_span("fetch_order", json!({
        "order_id": order_id,
    }));
    
    // Fetch order data
    let order = fetch_order_from_db(order_id).await?;
    order_span.end();
    
    // Validate order
    let validation_span = ctx.observability().start_span("validate_order", json!({
        "order_id": order_id,
        "user_id": order.user_id,
    }));
    
    validate_order(&order).await?;
    validation_span.end();
    
    // Process payment
    let payment_span = ctx.observability().start_span("process_payment", json!({
        "order_id": order_id,
        "amount": order.total_amount,
        "currency": order.currency,
    }));
    
    let payment_result = process_payment(&order).await?;
    payment_span.set_attribute("payment_id", payment_result.payment_id.clone());
    payment_span.end();
    
    // Update inventory
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

### Asynchronous Tracing

```rust
use dmsc::prelude::*;
use futures::future::join_all;

async fn batch_process_orders(order_ids: Vec<String>) -> DMSCResult<Vec<OrderResult>> {
    let batch_span = ctx.observability().start_span("batch_process_orders", json!({
        "order_count": order_ids.len(),
        "orders": order_ids.clone(),
    }));
    
    // Create async tasks
    let tasks: Vec<_> = order_ids.into_iter().map(|order_id| {
        let span = ctx.observability().start_span("process_single_order", json!({
            "order_id": order_id.clone(),
        }));
        
        // Pass span context to async task
        let span_context = span.get_context();
        
        async move {
            // Restore span context in async task
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
    
    // Execute all tasks in parallel
    let results = join_all(tasks).await;
    
    let successful_count = results.iter().filter(|r| r.is_ok()).count();
    let failed_count = results.len() - successful_count;
    
    batch_span.set_attribute("successful_count", successful_count as i64);
    batch_span.set_attribute("failed_count", failed_count as i64);
    batch_span.end();
    
    results.into_iter().collect()
}
```

## Metrics Collection

### Counter Metrics

```rust
use dmsc::prelude::*;

// Create counters
let request_counter = ctx.observability().create_counter("http_requests_total", "Total HTTP requests", "requests");

// Use in HTTP middleware
async fn http_middleware(req: DMSCHttpRequest, next: Next) -> DMSCResult<DMSCHttpResponse> {
    // Record request count
    request_counter.increment(1, json!({
        "method": req.method.clone(),
        "path": req.path.clone(),
        "user_agent": req.headers.get("user-agent").unwrap_or(&"unknown".to_string()).clone(),
    }));
    
    let start_time = std::time::Instant::now();
    let response = next.run(req).await?;
    let duration = start_time.elapsed();
    
    // Record response time
    let histogram = ctx.observability().create_histogram("http_request_duration_seconds", "HTTP request duration", "seconds");
    histogram.record(duration.as_secs_f64(), json!({
        "method": response.method.clone(),
        "path": response.path.clone(),
        "status_code": response.status_code,
    }));
    
    Ok(response)
}

// Business metrics counting
let order_counter = ctx.observability().create_counter("orders_processed_total", "Total orders processed", "orders");
let user_counter = ctx.observability().create_counter("users_registered_total", "Total users registered", "users");

// Use in business logic
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

### Histogram Metrics

```rust
use dmsc::prelude::*;

// Create histograms
let request_size_histogram = ctx.observability().create_histogram("http_request_size_bytes", "HTTP request size", "bytes");
let response_size_histogram = ctx.observability().create_histogram("http_response_size_bytes", "HTTP response size", "bytes");
let db_query_histogram = ctx.observability().create_histogram("database_query_duration_seconds", "Database query duration", "seconds");

// Record request size
request_size_histogram.record(req.body.len() as f64, json!({
    "method": req.method.clone(),
    "path": req.path.clone(),
}));

// Record database query time
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

// Custom histogram buckets
let custom_histogram = ctx.observability().create_histogram_with_buckets(
    "custom_metric_seconds",
    "Custom metric with custom buckets",
    "seconds",
    vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0] // Custom buckets
);
```

### Gauge Metrics

```rust
use dmsc::prelude::*;

// Create gauges
let active_users_gauge = ctx.observability().create_gauge("active_users_current", "Current active users", "users");
let system_load_gauge = ctx.observability().create_gauge("system_load_average", "System load average", "load");
let memory_usage_gauge = ctx.observability().create_gauge("memory_usage_bytes", "Memory usage", "bytes");

// Update gauges periodically
async fn update_system_metrics() -> DMSCResult<()> {
    loop {
        // Get active user count
        let active_users = get_active_user_count().await?;
        active_users_gauge.set(active_users as f64, json!({
            "source": "websocket_connections",
        }));
        
        // Get system load
        let load_avg = get_system_load_average()?;
        system_load_gauge.set(load_avg, json!({
            "time_window": "1m",
        }));
        
        // Get memory usage
        let memory_usage = get_memory_usage()?;
        memory_usage_gauge.set(memory_usage as f64, json!({
            "type": "heap",
        }));
        
        sleep(Duration::from_secs(10)).await;
    }
}

// Business gauges
let order_queue_gauge = ctx.observability().create_gauge("order_queue_size", "Order queue size", "orders");
let processing_rate_gauge = ctx.observability().create_gauge("order_processing_rate_per_second", "Order processing rate", "orders/s");

// Update business metrics
order_queue_gauge.set(get_order_queue_size() as f64, json!({
    "priority": "high",
}));

processing_rate_gauge.set(calculate_processing_rate(), json!({
    "time_window": "1m",
}));
```

## Health Checks

### System Health Checks

```rust
use dmsc::prelude::*;
use serde_json::json;

// Register health checks
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

// Get overall health status
let health_status = ctx.observability().get_health_status().await?;
ctx.log().info(format!("System health: {:?}", health_status));

// Health check endpoint
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

### Custom Health Checks

```rust
use dmsc::prelude::*;
use serde_json::json;

// Business logic health check
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

// Disk space health check
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

// Memory usage health check
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

## Performance Analysis

### Performance Profiling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Start performance profiling
let profile_id = ctx.observability().start_profiling("order_processing_profile", json!({
    "order_id": "ORD-12345",
    "profile_type": "cpu",
    "duration_seconds": 30,
}));

// Execute operation to be profiled
let result = process_order("ORD-12345").await?;

// End performance profiling
let profile_data = ctx.observability().stop_profiling(profile_id)?;

// Analyze profiling results
ctx.log().info(format!("Profile completed: {:?}", profile_data));

// Get performance hotspots
let hotspots = ctx.observability().get_performance_hotspots()?;
for hotspot in hotspots {
    ctx.log().info(format!("Hotspot: {} - {} samples ({}%)", 
        hotspot.function, 
        hotspot.sample_count, 
        hotspot.percentage
    ));
}
```

### Memory Analysis

```rust
use dmsc::prelude::*;
use serde_json::json;

// Start memory profiling
let memory_profile_id = ctx.observability().start_memory_profiling("order_processing_memory", json!({
    "order_id": "ORD-12345",
    "profile_type": "heap",
    "track_allocations": true,
}));

// Execute memory-intensive operations
let large_dataset = generate_large_dataset().await?;
process_large_dataset(large_dataset).await?;

// End memory profiling
let memory_profile = ctx.observability().stop_memory_profiling(memory_profile_id)?;

// Analyze memory usage
ctx.log().info(format!("Memory profile: {:?}", memory_profile));

// Get memory leak detection
let leaks = ctx.observability().detect_memory_leaks()?;
if !leaks.is_empty() {
    ctx.log().warn(format!("Detected {} potential memory leaks", leaks.len()));
    for leak in leaks {
        ctx.log().warn(format!("Potential leak: {} - {} bytes", leak.location, leak.size));
    }
}
```

### Performance Benchmarking

```rust
use dmsc::prelude::*;
use serde_json::json;

// Run performance benchmark
let benchmark_config = DMSCBenchmarkConfig {
    name: "order_processing_benchmark".to_string(),
    iterations: 1000,
    warmup_iterations: 100,
    concurrent_requests: 10,
    metrics: vec!["latency".to_string(), "throughput".to_string(), "cpu_usage".to_string()],
};

let benchmark_result = ctx.observability().run_benchmark(benchmark_config, || async {
    // Benchmark test code
    let order = create_test_order().await?;
    process_order(&order).await
}).await?;

// Analyze benchmark results
ctx.log().info(format!("Benchmark completed: {:?}", benchmark_result));

// Get performance statistics
let stats = benchmark_result.statistics;
ctx.log().info(format!("Average latency: {:.2}ms", stats.average_latency_ms));
ctx.log().info(format!("P95 latency: {:.2}ms", stats.p95_latency_ms));
ctx.log().info(format!("P99 latency: {:.2}ms", stats.p99_latency_ms));
ctx.log().info(format!("Throughput: {:.2} req/s", stats.throughput_rps));
ctx.log().info(format!("Error rate: {:.2}%", stats.error_rate * 100.0));
```

## Alert Management

### Creating Alert Rules

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create CPU usage alert
let cpu_alert_rule = DMAlertRule {
    name: "high_cpu_usage".to_string(),
    metric: "cpu_usage_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(80.0),
    duration: Duration::from_secs(300), // Duration: 5 minutes
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["email".to_string(), "slack".to_string()],
    labels: json!({
        "component": "application_server",
        "environment": "production",
    }),
};

ctx.observability().create_alert_rule(cpu_alert_rule).await?;

// Create memory usage alert
let memory_alert_rule = DMAlertRule {
    name: "high_memory_usage".to_string(),
    metric: "memory_usage_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(85.0),
    duration: Duration::from_secs(600), // Duration: 10 minutes
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["email".to_string(), "pagerduty".to_string()],
    labels: json!({
        "component": "application_server",
        "environment": "production",
    }),
};

ctx.observability().create_alert_rule(memory_alert_rule).await?;

// Create error rate alert
let error_rate_alert_rule = DMAlertRule {
    name: "high_error_rate".to_string(),
    metric: "error_rate_percentage".to_string(),
    condition: DMAlertCondition::GreaterThan(5.0),
    duration: Duration::from_secs(180), // Duration: 3 minutes
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["slack".to_string()],
    labels: json!({
        "component": "api",
        "service": "order_service",
    }),
};

ctx.observability().create_alert_rule(error_rate_alert_rule).await?;
```

### Business Metrics Alerts

```rust
use dmsc::prelude::*;
use serde_json::json;

// Order processing latency alert
let order_latency_alert = DMAlertRule {
    name: "order_processing_high_latency".to_string(),
    metric: "order_processing_duration_seconds".to_string(),
    condition: DMAlertCondition::GreaterThan(30.0), // Exceeds 30 seconds
    duration: Duration::from_secs(600), // Duration: 10 minutes
    severity: DMAlertSeverity::Warning,
    notification_channels: vec!["email".to_string(), "slack".to_string()],
    labels: json!({
        "component": "order_service",
        "metric_type": "latency",
    }),
};

ctx.observability().create_alert_rule(order_latency_alert).await?;

// Queue backlog alert
let queue_backlog_alert = DMAlertRule {
    name: "order_queue_backlog".to_string(),
    metric: "order_queue_size".to_string(),
    condition: DMAlertCondition::GreaterThan(1000), // Exceeds 1000 orders
    duration: Duration::from_secs(300), // Duration: 5 minutes
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["pagerduty".to_string(), "email".to_string()],
    labels: json!({
        "component": "order_service",
        "queue": "order_processing",
    }),
};

ctx.observability().create_alert_rule(queue_backlog_alert).await?;

// Custom business logic alert
let fraud_detection_alert = DMAlertRule {
    name: "suspicious_fraud_pattern".to_string(),
    metric: "fraud_detection_score".to_string(),
    condition: DMAlertCondition::GreaterThan(0.8), // Fraud score exceeds 0.8
    duration: Duration::from_secs(120), // Duration: 2 minutes
    severity: DMAlertSeverity::Critical,
    notification_channels: vec!["email".to_string(), "security_team".to_string()],
    labels: json!({
        "component": "fraud_detection",
        "type": "security",
    }),
};

ctx.observability().create_alert_rule(fraud_detection_alert).await?;

### Alert Notifications

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure notification channels
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

// Handle alert notifications
async fn handle_alert_notification(alert: DMAlert) -> DMSCResult<()> {
    ctx.log().warn(format!("Alert triggered: {} - {}", alert.name, alert.severity));
    
    // Send email notification
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
    
    // Send Slack notification
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

## Monitoring Integration

### Prometheus Integration

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure Prometheus exporter
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

// Custom Prometheus metrics
let custom_metric = ctx.observability().create_prometheus_counter(
    "custom_business_metric_total",
    "Custom business metric",
    vec!["label1".to_string(), "label2".to_string()]
);

custom_metric.increment(1.0, vec!["value1".to_string(), "value2".to_string()]);
```

### Grafana Integration

```rust
use dmsc::prelude::*;
use serde_json::json;

// Create Grafana dashboard configuration
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

## Error Handling

### Observability Error Handling

```rust
use dmsc::prelude::*;
use serde_json::json;

// Handle tracing errors
match ctx.observability().start_span("operation", json!({"key": "value"})) {
    Ok(span) => {
        // Normal processing
        span.end();
    }
    Err(DMSCError::ObservabilityTracingError(e)) => {
        ctx.log().error(format!("Tracing error: {}", e));
        // Degraded handling: operate without tracing
        perform_operation_without_tracing().await?;
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected error: {}", e));
        return Err(e);
    }
}

// Handle metrics collection errors
match ctx.observability().create_counter("metric_name", "description", "unit") {
    Ok(counter) => {
        counter.increment(1.0, json!({"label": "value"}));
    }
    Err(DMSCError::ObservabilityMetricsError(e)) => {
        ctx.log().warn(format!("Metrics collection error: {}", e));
        // Degraded: log instead of metrics
        ctx.log().info("Metric: metric_name increment by 1");
    }
    Err(e) => {
        ctx.log().error(format!("Unexpected error: {}", e));
        return Err(e);
    }
}

// Health check degradation
ctx.observability().register_health_check("critical_service", || async {
    match check_critical_service().await {
        Ok(health) => Ok(health),
        Err(DMSCError::ObservabilityHealthCheckError(e)) => {
            ctx.log().warn(format!("Health check error: {}", e));
            // Return degraded status instead of error
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

## Best Practices

1. **Use Span Reasonably**: Create spans for important operations, avoid excessive tracing
2. **Add Meaningful Attributes**: Include business data in spans that helps with debugging
3. **Consistent Metric Naming**: Use consistent naming conventions for easy querying and aggregation
4. **Set Appropriate Alert Thresholds**: Avoid overly sensitive or sluggish alerts
5. **Regularly Review Alerts**: Remove unnecessary alerts and adjust thresholds
6. **Layered Health Checks**: Distinguish between system-level, application-level, and business-level health checks
7. **Performance Profiling Sampling**: Use sampling in production to avoid excessive overhead
8. **Monitor Observability Itself**: Monitor the health of tracing, metrics, and alerting systems
9. **Document Metrics**: Record the meaning and purpose of all custom metrics
10. **Test Alerts**: Regularly test the alerting system to ensure it works properly
11. **Use Structured Logging**: Standardize log formats for easy log analysis and search
12. **Monitor Key Business Metrics**: Monitor not only technical metrics but also business KPIs
13. **Set Alert Escalation Strategies**: Configure alert escalation paths to ensure important issues are handled promptly
14. **Use Labels and Dimensions**: Add rich labels to metrics to support multi-dimensional analysis
15. **Regular Performance Tuning**: Continuously optimize system performance based on monitoring data

<div align="center">

## Running Steps

</div>

1. **Environment Preparation**: Ensure Rust development environment is installed
2. **Create Project**: Create new project using `cargo new observability-example`
3. **Add Dependencies**: Add dms dependency in `Cargo.toml`
4. **Create Configuration**: Copy the above configuration code to `src/config.rs`
5. **Run Example**: Execute `cargo run` to start the application

<div align="center">

## Expected Results

</div>

After successful execution, you will see the following output:

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

## Extension Features

</div>

### Distributed Tracing Enhancement

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure advanced tracing options
let advanced_tracing = DMTracingConfig {
    service_name: "order_service".to_string(),
    jaeger_endpoint: "http://localhost:14268/api/traces".to_string(),
    sampling_rate: 0.1, // 10% sampling rate
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

// Create cross-service tracing context
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

### Smart Alerting System

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure intelligent alerting rules
let intelligent_alerts = vec![
    DMAlertRule {
        name: "anomaly_detection".to_string(),
        condition: DMAlertCondition::Anomaly {
            metric: "request_rate".to_string(),
            algorithm: "ewma".to_string(), // Exponentially Weighted Moving Average
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

// Configure alert escalation policy
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

### Performance Profiling Integration

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure continuous performance profiling
let profiling_config = DMProfilingConfig {
    enabled: true,
    endpoint: "http://localhost:4040".to_string(),
    sample_rate: 0.01, // 1% sampling rate
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

// Create custom performance metrics
let custom_profile = ctx.observability().create_custom_profile(
    "business_logic_performance",
    json!({
        "function_name": "process_order",
        "threshold_ms": 100,
    })
).await?;

custom_profile.start_timer();
// Execute business logic
process_order(order_data).await?;
custom_profile.end_timer();
```

### Business Metrics Monitoring

```rust
use dmsc::prelude::*;
use serde_json::json;

// Configure business KPI monitoring
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

// Record business event
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

## Summary

</div>

This example demonstrates the powerful observability features of the DMSC framework, helping you build highly monitorable and debuggable applications. Through distributed tracing, metrics collection, health checks, log aggregation, and alerting systems, you can gain comprehensive insights into your application's operational status.

### Core Features

1. **Distributed Tracing**: Cross-service call chain tracing, supporting Jaeger and OpenTelemetry
2. **Metrics Collection**: Prometheus metrics export, supporting custom business metrics
3. **Health Checks**: Multi-level health checks, including system, application, and business layers
4. **Log Aggregation**: Structured log collection, supporting multiple log levels and formats
5. **Performance Profiling**: Runtime performance analysis, identifying performance bottlenecks
6. **Alerting System**: Intelligent alerting rules, supporting multiple notification channels
7. **Grafana Integration**: Visual monitoring dashboards, displaying key metrics in real-time
8. **Error Handling**: Comprehensive error handling and degradation mechanisms

### Advanced Features

1. **Intelligent Alerts**: Machine learning-based anomaly detection and predictive alerting
2. **Business Metrics**: Business KPI monitoring, linking technical metrics to business objectives
3. **Performance Profiling**: Continuous performance monitoring, automatically identifying performance issues
4. **Cross-Service Tracing**: Supporting call chain tracing for complex microservice architectures
5. **Alert Escalation**: Multi-level alert escalation strategies, ensuring important issues are handled
6. **Sampling Strategy**: Configurable tracing and performance profiling sampling rates, balancing overhead and accuracy

### Best Practices

- Add distributed tracing for critical business operations
- Set reasonable alert thresholds and escalation strategies
- Monitor business metrics, not just technical metrics
- Regularly review and optimize monitoring configurations
- Use structured logs for easy analysis
- Implement layered design for health checks
- Test the effectiveness of alerting systems
- Document the meaning of all custom metrics

<div align="center">

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation to all usage examples
- [authentication](./authentication.md): Authentication examples, learning JWT, OAuth2 and RBAC authentication and authorization
- [basic-app](./basic-app.md): Basic application example, learning how to create and run your first DMSC application
- [caching](./caching.md): Caching examples, understanding how to use caching modules to improve application performance
- [database](./database.md): Database examples, learning database connections and query operations
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [http](./http.md): HTTP service examples, building web applications and RESTful APIs
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples, implementing asynchronous message processing and event-driven architecture

- [security](./security.md): Security examples, encryption, hashing and security best practices
- [storage](./storage.md): Storage examples, file upload/download and storage management
- [validation](./validation.md): Validation examples, data validation and sanitization operations