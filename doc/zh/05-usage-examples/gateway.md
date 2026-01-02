<div align="center">

# API网关使用示例

**Version: 0.0.3**

**Last modified date: 2026-01-01**

本示例展示如何使用 gateway 模块构建 API 网关，包括路由、中间件、负载均衡、限流和熔断功能。

## 前置要求

</div>

- DMSC Rust SDK
- tokio 异步运行时

<div align="center">

## 示例代码

</div>

```rust
use dms::prelude::*;
use dms::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCRoute, DMSCRouter};
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    println!("=== DMSC Gateway Example ===\n");
    
    let gateway_config = DMSCGatewayConfig {
        listen_address: "0.0.0.0".to_string(),
        listen_port: 8080,
        max_connections: 10000,
        request_timeout_seconds: 30,
        enable_rate_limiting: true,
        enable_circuit_breaker: true,
        enable_load_balancing: true,
        cors_enabled: true,
        cors_origins: vec!["*".to_string()],
        cors_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string()],
        cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        enable_logging: true,
        log_level: "info".to_string(),
    };
    
    println!("1. Creating Gateway");
    println!("--------------------");
    
    let gateway = DMSCGateway::new();
    println!("Gateway created successfully");
    
    let router = gateway.router();
    
    println!("\n2. Adding Routes");
    println!("-----------------");
    
    router.add_route(DMSCRoute {
        path: "/api/health".to_string(),
        method: "GET".to_string(),
        handler: Arc::new(|req| {
            Box::pin(async move {
                Ok(DMSCGatewayResponse::json(200, &serde_json::json!({
                    "status": "healthy",
                    "service": "DMSC Gateway",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }), req.id.clone())?)
            })
        }),
        ..Default::default()
    }).await?;
    println!("Route added: GET /api/health");
    
    router.add_route(DMSCRoute {
        path: "/api/users".to_string(),
        method: "GET".to_string(),
        handler: Arc::new(|req| {
            Box::pin(async move {
                let users = vec![
                    serde_json::json!({"id": 1, "name": "Alice", "email": "alice@example.com"}),
                    serde_json::json!({"id": 2, "name": "Bob", "email": "bob@example.com"}),
                    serde_json::json!({"id": 3, "name": "Charlie", "email": "charlie@example.com"}),
                ];
                Ok(DMSCGatewayResponse::json(200, &users, req.id.clone())?)
            })
        }),
        ..Default::default()
    }).await?;
    println!("Route added: GET /api/users");
    
    router.add_route(DMSCRoute {
        path: "/api/users".to_string(),
        method: "POST".to_string(),
        handler: Arc::new(|req| {
            Box::pin(async move {
                let user_data = match &req.body {
                    Some(body) => {
                        serde_json::from_slice(body).unwrap_or_else(|_| serde_json::json!({}))
                    }
                    None => serde_json::json!({}),
                };
                Ok(DMSCGatewayResponse::json(201, &serde_json::json!({
                    "message": "User created successfully",
                    "user": user_data
                }), req.id.clone())?)
            })
        }),
        ..Default::default()
    }).await?;
    println!("Route added: POST /api/users");
    
    router.add_route(DMSCRoute {
        path: "/api/users/:id".to_string(),
        method: "GET".to_string(),
        handler: Arc::new(|req| {
            Box::pin(async move {
                let user_id = req.params.get("id").cloned().unwrap_or_default();
                Ok(DMSCGatewayResponse::json(200, &serde_json::json!({
                    "id": user_id,
                    "name": "User ".to_string() + &user_id,
                    "email": format!("user{}@example.com", user_id)
                }), req.id.clone())?)
            })
        }),
        ..Default::default()
    }).await?;
    println!("Route added: GET /api/users/:id");
    
    println!("\n3. Testing Request Handling");
    println!("----------------------------");
    
    let health_request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/api/health".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let health_response = gateway.handle_request(health_request).await;
    println!("Health check response: {} - {}", health_response.status_code, 
             String::from_utf8_lossy(&health_response.body));
    
    let users_request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/api/users".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let users_response = gateway.handle_request(users_request).await;
    println!("Users list response: {} - {}", users_response.status_code,
             String::from_utf8_lossy(&users_response.body));
    
    let user_request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/api/users/42".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let user_response = gateway.handle_request(user_request).await;
    println!("Single user response: {} - {}", user_response.status_code,
             String::from_utf8_lossy(&user_response.body));
    
    println!("\n4. Rate Limiting");
    println!("-----------------");
    
    let rate_limiter = gateway.get_rate_limiter().cloned().unwrap_or_default();
    if let Some(limiter) = rate_limiter {
        println!("Rate limiter is enabled");
        println!("  Requests per second: {}", limiter.config.requests_per_second);
        println!("  Burst size: {}", limiter.config.burst_size);
    }
    
    println!("\n5. Circuit Breaker");
    println!("-------------------");
    
    let circuit_breaker = gateway.get_circuit_breaker().cloned().unwrap_or_default();
    if let Some(breaker) = circuit_breaker {
        println!("Circuit breaker is enabled");
        println!("  Failure threshold: {}", breaker.config.failure_threshold);
        println!("  Success threshold: {}", breaker.config.success_threshold);
        println!("  Timeout duration: {:?}", breaker.config.timeout_duration);
    }
    
    println!("\n6. Error Handling");
    println!("------------------");
    
    let not_found_request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/api/nonexistent".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let not_found_response = gateway.handle_request(not_found_request).await;
    println!("Not found response: {} - {}", not_found_response.status_code,
             String::from_utf8_lossy(&not_found_response.body));
    
    println!("\n=== Example Completed ===");
    Ok(())
}
```

<div align="center>

## 运行步骤

</div>

### 1. 创建示例项目

```bash
cargo new dms-gateway-example
cd dms-gateway-example
```

### 2. 添加依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
dms = { git = "https://gitee.com/dunimd/dmsc.git" }
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### 3. 运行示例

```bash
cargo run
```

<div align="center>

## 预期输出

</div>

```
=== DMSC Gateway Example ===

1. Creating Gateway
--------------------
Gateway created successfully

2. Adding Routes
-----------------
Route added: GET /api/health
Route added: GET /api/users
Route added: POST /api/users
Route added: GET /api/users/:id

3. Testing Request Handling
----------------------------
Health check response: 200 - {"status":"healthy","service":"DMSC Gateway","timestamp":"2024-01-15T10:30:00Z"}
Users list response: 200 - [{"email":"alice@example.com","id":1,"name":"Alice"},{"email":"bob@example.com","id":2,"name":"Bob"},{"email":"charlie@example.com","id":3,"name":"Charlie"}]
Single user response: 200 - {"email":"user42@example.com","id":"42","name":"User 42"}

4. Rate Limiting
-----------------
Rate limiter is enabled
  Requests per second: 100
  Burst size: 200

5. Circuit Breaker
-------------------
Circuit breaker is enabled
  Failure threshold: 5
  Success threshold: 2
  Timeout duration: 30s

6. Error Handling
------------------
Not found response: 404 - {"error":"Route not found"}

=== Example Completed ===
```

<div align="center>

## 高级功能

</div>

### 添加自定义中间件

```rust
fn add_custom_middleware(gateway: &DMSCGateway) {
    let middleware_chain = gateway.middleware_chain();
    
    middleware_chain.add_middleware(Arc::new(|req, next| {
        Box::pin(async move {
            println!("Request received: {} {}", req.method, req.path);
            let response = next.run(req).await?;
            println!("Response sent: {}", response.status_code);
            Ok(response)
        })
    })).await;
    
    println!("Custom logging middleware added");
}
```

### 配置负载均衡

```rust
fn configure_load_balancing(gateway: &DMSCGateway) {
    let load_balancer = gateway.get_load_balancer().unwrap();
    
    load_balancer.add_server(DMSCBackendServer {
        id: "backend-1".to_string(),
        url: "http://localhost:8081".to_string(),
        weight: 100,
        max_connections: 1000,
        health_check_path: "/health".to_string(),
        is_healthy: true,
    }).await;
    
    load_balancer.add_server(DMSCBackendServer {
        id: "backend-2".to_string(),
        url: "http://localhost:8082".to_string(),
        weight: 100,
        max_connections: 1000,
        health_check_path: "/health".to_string(),
        is_healthy: true,
    }).await;
    
    println!("Backend servers added for load balancing");
}
```

<div align="center>

## 最佳实践

</div>

1. **合理配置超时**：根据业务需求设置合适的请求超时
2. **启用熔断保护**：防止级联故障，提高系统稳定性
3. **使用限流保护**：防止恶意请求和过载
4. **正确配置CORS**：正确配置跨域资源共享策略
5. **启用请求日志**：记录请求日志便于问题排查
6. **使用路由参数**：使用路由参数处理动态路径

<div align="center">

## 相关文档

</div>

- [网关API参考](../04-api-reference/gateway.md)：详细的API文档
- [服务网格](../04-api-reference/service_mesh.md)：服务发现和流量管理
- [最佳实践](../06-best-practices.md)：更多最佳实践建议
