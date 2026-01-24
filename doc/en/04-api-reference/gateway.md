<div align="center">

# Gateway API Reference

**Version: 0.1.5**

**Last modified date: 2026-01-24**

The gateway module provides API gateway functionality, including routing, middleware, load balancing, rate limiting, and circuit breaking support.

## Module Overview

</div>

The gateway module contains the following sub-modules:

- **routing**: Route management
- **middleware**: Middleware chain
- **load_balancer**: Load balancing
- **rate_limiter**: Rate limiter
- **circuit_breaker**: Circuit breaker

<div align="center">

## Core Components

</div>

### DMSCGateway

The main API gateway interface, providing unified gateway functionality.

**Note**: The `handle_request` method is an internal method, only available within the crate.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create gateway instance | None | `Self` |
| `router()` | Get router | None | `Arc<DMSCRouter>` |
| `middleware_chain()` | Get middleware chain | None | `Arc<DMSCMiddlewareChain>` |
| `handle_request(request)` | Handle request (internal method) | `request: DMSCGatewayRequest` | `DMSCGatewayResponse` |
| `after_shutdown(ctx)` | Cleanup operation after gateway shutdown (implements DMSCModule trait) | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` |

#### Usage Example

```rust
use dmsc::prelude::*;
use dmsc::gateway::{DMSCGateway, DMSCRoute};
use std::sync::Arc;

async fn example() -> DMSCResult<()> {
    let gateway = DMSCGateway::new();
    
    let router = gateway.router();
    
    router.add_route(DMSCRoute::new(
        "GET".to_string(),
        "/api/v1/health".to_string(),
        Arc::new(|req| Box::pin(async move {
            Ok(DMSCGatewayResponse::json(200, &serde_json::json!({ "status": "ok" }), req.id.clone())?)
        })),
    ));
    
    Ok(())
}
```

### DMSCGatewayRequest

Gateway request struct.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Request ID |
| `method` | `String` | HTTP method |
| `path` | `String` | Request path |
| `headers` | `HashMap<String, String>` | HTTP headers |
| `query_params` | `HashMap<String, String>` | Query parameters |
| `body` | `Option<Vec<u8>>` | Request body |
| `remote_addr` | `String` | Client address |
| `timestamp` | `Instant` | Request timestamp |

### DMSCGatewayResponse

Gateway response struct.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(status_code, body, request_id)` | Create response | `status_code: u16`, `body: Vec<u8>`, `request_id: String` | `Self` |
| `json(status_code, data, request_id)` | Create JSON response | `status_code: u16`, `data: &T`, `request_id: String` | `DMSCResult<Self>` |
| `error(status_code, message, request_id)` | Create error response | `status_code: u16`, `message: String`, `request_id: String` | `Self` |
| `with_header(key, value)` | Add response header | `key: String`, `value: String` | `Self` |

<div align="center">

## Route Management

</div>

### DMSCRoute

Route definition.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `path` | `String` | Route path |
| `method` | `String` | HTTP method |
| `handler` | `Arc<RouteHandler>` | Handler |
| `middleware` | `Vec<Arc<DMSCMiddleware>>` | Middleware list |
| `timeout` | `Duration` | Timeout |

### DMSCRouter

Router.

```rust
use dmsc::gateway::{DMSCRoute, DMSCRouter};

let router = DMSCRouter::new();

router.add_route(DMSCRoute {
    path: "/api/users".to_string(),
    method: "GET".to_string(),
    handler: Arc::new(|req| Box::pin(async move {
        let users = vec![
            serde_json::json!({"id": 1, "name": "Alice"}),
            serde_json::json!({"id": 2, "name": "Bob"}),
        ];
        Ok(DMSCGatewayResponse::json(200, &users, req.id.clone())?)
    })),
    ..Default::default()
});

// Or use shorthand methods
router.get("/api/users", Arc::new(|req| Box::pin(async move {
    let users = vec![
        serde_json::json!({"id": 1, "name": "Alice"}),
        serde_json::json!({"id": 2, "name": "Bob"}),
    ];
    Ok(DMSCGatewayResponse::json(200, &users, req.id.clone())?)
})));
```

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create router | None | `Self` |
| `add_route(route)` | Add route | `route: DMSCRoute` | `()` |
| `get(path, handler)` | Add GET route | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `post(path, handler)` | Add POST route | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `put(path, handler)` | Add PUT route | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `delete(path, handler)` | Add DELETE route | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `patch(path, handler)` | Add PATCH route | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `clear_routes()` | Clear all routes | None | `()` |
| `route_count()` | Get route count | None | `usize` |

<div align="center">

## Middleware

</div>

### DMSCMiddleware

Middleware interface.

```rust
use dmsc::gateway::{DMSCMiddleware, DMSCGatewayRequest, DMSCGatewayResponse};

struct LoggingMiddleware;

#[async_trait]
impl DMSCMiddleware for LoggingMiddleware {
    async fn execute(&self, req: &mut DMSCGatewayRequest, next: &dyn DMSCMiddleware) -> DMSCResult<()> {
        println!("Before: {} {}", req.method, req.path);
        
        next.execute(req).await?;
        
        println!("After: {} {}", req.path, req.id);
        
        Ok(())
    }

    fn name(&self) -> &'static str {
        "LoggingMiddleware"
    }
}

let middleware = Arc::new(LoggingMiddleware);
let mut middleware_chain = gateway.middleware_chain();
middleware_chain.add(middleware);
```

### Built-in Middleware

```rust
use dmsc::gateway::DMSCRateLimiter;

let rate_limiter = Arc::new(DMSCRateLimiter::new(DMSCRateLimitConfig::default()));
```

<div align="center">

## Load Balancing

</div>

### DMSCLoadBalancer

Load balancer.

```rust
use dmsc::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};

let load_balancer = DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin);

load_balancer.add_server(DMSCBackendServer {
    id: "server-1".to_string(),
    url: "http://localhost:8081".to_string(),
    weight: 100,
    max_connections: 1000,
    health_check_path: "/health".to_string(),
    is_healthy: true,
}).await;

let server = load_balancer.select_server(None).await?;
```

### Load Balancing Strategies

| Strategy | Description |
|:--------|:-------------|
| `RoundRobin` | Round robin |
| `LeastConnections` | Least connections |
| `Weighted` | Weighted |
| `Random` | Random |
| `IPHash` | IP hash |

<div align="center">

## Rate Limiting

</div>

### DMSCRateLimiter

Rate limiter.

```rust
use dmsc::gateway::{DMSCRateLimiter, DMSCRateLimitConfig};

let rate_limit_config = DMSCRateLimitConfig {
    requests_per_second: 100,
    burst_size: 200,
    window_size: Duration::from_secs(60),
};

let rate_limiter = DMSCRateLimiter::new(rate_limit_config);

if rate_limiter.check_request(&request).await {
    println!("Request allowed");
} else {
    println!("Rate limit exceeded");
}
```

### DMSCRateLimitConfig

Rate limit configuration.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `requests_per_second` | `u64` | Requests per second | `100` |
| `burst_size` | `u64` | Burst size | `200` |
| `window_size` | `Duration` | Time window | `60s` |

<div align="center">

## Circuit Breaking

</div>

### DMSCCircuitBreaker

Circuit breaker.

```rust
use dmsc::gateway::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig};

let circuit_breaker_config = DMSCCircuitBreakerConfig {
    failure_threshold: 5,
    success_threshold: 2,
    timeout_duration: Duration::from_secs(30),
};

let circuit_breaker = DMSCCircuitBreaker::new(circuit_breaker_config);

if circuit_breaker.allow_request().await {
    println!("Request allowed by circuit breaker");
} else {
    println!("Circuit breaker is open, request rejected");
}

circuit_breaker.record_success().await;
circuit_breaker.record_failure().await;
```

### DMSCCircuitBreakerConfig

Circuit breaker configuration.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `failure_threshold` | `u32` | Failure threshold | `5` |
| `success_threshold` | `u32` | Success threshold | `2` |
| `timeout_duration` | `Duration` | Timeout duration | `30s` |

<div align="center">

## Configuration

</div>

### DMSCGatewayConfig

Gateway configuration.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `listen_address` | `String` | Listen address | `"0.0.0.0"` |
| `listen_port` | `u16` | Listen port | `8080` |
| `max_connections` | `usize` | Max connections | `10000` |
| `request_timeout_seconds` | `u64` | Request timeout (seconds) | `30` |
| `enable_rate_limiting` | `bool` | Enable rate limiting | `true` |
| `enable_circuit_breaker` | `bool` | Enable circuit breaker | `true` |
| `enable_load_balancing` | `bool` | Enable load balancing | `true` |
| `cors_enabled` | `bool` | Enable CORS | `true` |
| `cors_origins` | `Vec<String>` | CORS origins | `["*"]` |
| `cors_methods` | `Vec<String>` | CORS methods | `["GET","POST","PUT","DELETE","OPTIONS"]` |
| `cors_headers` | `Vec<String>` | CORS headers | `["Content-Type","Authorization","X-Requested-With"]` |
| `enable_logging` | `bool` | Enable logging | `true` |
| `log_level` | `String` | Log level | `"info"` |

<div align="center">

## Best Practices

</div>

1. **Configure timeouts appropriately**: Set appropriate request timeout values based on business requirements
2. **Enable circuit breaker protection**: Prevent cascading failures and improve system stability
3. **Use rate limiting**: Prevent malicious requests and overload
4. **Use middleware appropriately**: Add middleware as needed to avoid affecting performance
5. **Configure CORS**: Properly configure cross-origin resource sharing policies
6. **Enable logging**: Log requests for easier troubleshooting

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
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
