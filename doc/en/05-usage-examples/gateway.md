<div align="center">

# API Gateway Usage Example

**Version: 0.1.6**

**Last modified date: 2026-01-30**

This example demonstrates how to use the gateway module to build an API gateway, including routing, middleware, load balancing, rate limiting, and circuit breaking functionality.

## Prerequisites

</div>

- DMSC Rust SDK
- tokio async runtime

<div align="center">

## Example Code

</div>

```rust
use dmsc::prelude::*;
use dmsc::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCRoute, DMSCRouter};
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
    
    router.add_route(DMSCRoute::new(
        "GET".to_string(),
        "/api/health".to_string(),
        Arc::new(|req| {
            Box::pin(async move {
                Ok(DMSCGatewayResponse::json(200, &serde_json::json!({
                    "status": "healthy",
                    "service": "DMSC Gateway",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }), req.id.clone())?)
            })
        }),
    ));
    println!("Route added: GET /api/health");
    
    router.add_route(DMSCRoute::new(
        "GET".to_string(),
        "/api/users".to_string(),
        Arc::new(|req| {
            Box::pin(async move {
                let users = vec![
                    serde_json::json!({"id": 1, "name": "Alice", "email": "alice@example.com"}),
                    serde_json::json!({"id": 2, "name": "Bob", "email": "bob@example.com"}),
                    serde_json::json!({"id": 3, "name": "Charlie", "email": "charlie@example.com"}),
                ];
                Ok(DMSCGatewayResponse::json(200, &users, req.id.clone())?)
            })
        }),
    ));
    println!("Route added: GET /api/users");
    
    router.add_route(DMSCRoute::new(
        "POST".to_string(),
        "/api/users".to_string(),
        Arc::new(|req| {
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
    ));
    println!("Route added: POST /api/users");
    
    router.add_route(DMSCRoute::new(
        "GET".to_string(),
        "/api/users/:id".to_string(),
        Arc::new(|req| {
            Box::pin(async move {
                let user_id = req.path.split("/").last().unwrap_or("unknown").to_string();
                Ok(DMSCGatewayResponse::json(200, &serde_json::json!({
                    "id": user_id,
                    "name": "User ".to_string() + &user_id,
                    "email": format!("user{}@example.com", user_id)
                }), req.id.clone())?)
            })
        }),
    ));
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
    
    println!("Rate limiting is configured in DMSCGatewayConfig");
    println!("  Requests per second: 100");
    println!("  Burst size: 200");
    
    println!("\n5. Circuit Breaker");
    println!("-------------------");
    
    println!("Circuit breaker is configured in DMSCGatewayConfig");
    println!("  Failure threshold: 5");
    println!("  Success threshold: 2");
    println!("  Timeout duration: 30s");
    
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

## Running Steps

</div>

### 1. Create Example Project

```bash
cargo new dms-gateway-example
cd dms-gateway-example
```

### 2. Add Dependencies

Add to `Cargo.toml`:

```toml
[dependencies]
dms = { git = "https://github.com/mf2023/DMSC.git" }
tokio = { version = "1.0", features = ["full"] }
serde_json = "1.0"
```

### 3. Run Example

```bash
cargo run
```

<div align="center>

## Expected Output

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

## Advanced Features

</div>

### Adding Custom Middleware

```rust
fn add_custom_middleware(gateway: &DMSCGateway) {
    let mut middleware_chain = gateway.middleware_chain();
    
    middleware_chain.add(Arc::new(|req, next| {
        Box::pin(async move {
            println!("Request received: {} {}", req.method, req.path);
            next.execute(req).await?;
            println!("Response sent");
            Ok(())
        })
    }));
    
    println!("Custom logging middleware added");
}
```

### Configuring Load Balancing

```rust
use dmsc::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy, DMSCBackendServer};

async fn configure_load_balancing() {
    let load_balancer = DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin);
    
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
    
    println!("Backend servers added for load balancing");
}
```

<div align="center>

## Best Practices

</div>

1. **Configure timeouts appropriately**: Set appropriate request timeout values based on business requirements
2. **Enable circuit breaker protection**: Prevent cascading failures and improve system stability
3. **Use rate limiting**: Prevent malicious requests and overload
4. **Configure CORS properly**: Properly configure cross-origin resource sharing policies
5. **Enable request logging**: Log requests for easier troubleshooting
6. **Use route parameters**: Use route parameters for dynamic path handling

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with usage examples summary and quick navigation
- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [basic-app](./basic-app.md): Basic application examples
- [caching](./caching.md): Caching examples, including memory and distributed caching
- [database](./database.md): Database operation examples
- [device](./device.md): Device control examples
- [fs](./fs.md): Filesystem operation examples
- [hooks](./hooks.md): Hook system examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [observability](./observability.md): Observability examples
- [protocol](./protocol.md): Protocol module examples
- [service_mesh](./service_mesh.md): Service mesh examples
- [validation](./validation.md): Data validation examples

<div align="center">

## Related Documentation

</div>

- [Gateway API Reference](../04-api-reference/gateway.md): Detailed API documentation
- [Service Mesh](../04-api-reference/service_mesh.md): Service discovery and traffic management
- [Best Practices](../06-best-practices.md): More best practice suggestions
