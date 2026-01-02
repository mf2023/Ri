<div align="center">

# Gateway API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

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

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new()` | Create gateway instance | None | `Self` |
| `router()` | Get router | None | `Arc<DMSCRouter>` |
| `middleware_chain()` | Get middleware chain | None | `Arc<DMSCMiddlewareChain>` |
| `handle_request(request)` | Handle request | `request: DMSCGatewayRequest` | `DMSCGatewayResponse` |

#### Usage Example

```rust
use dms::prelude::*;
use dms::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCRoute};
use std::collections::HashMap;

async fn example() -> DMSCResult<()> {
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
        cors_methods: vec!["GET".to_string(), "POST".to_string()],
        cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
        enable_logging: true,
        log_level: "info".to_string(),
    };
    
    let gateway = DMSCGateway::new();
    
    let router = gateway.router();
    
    router.add_route(DMSCRoute {
        path: "/api/v1/health".to_string(),
        method: "GET".to_string(),
        handler: Arc::new(|req| Box::pin(async move {
            Ok(DMSCGatewayResponse::json(200, &serde_json::json!({ "status": "ok" }), req.id.clone())?)
        })),
        ..Default::default()
    }).await?;
    
    let middleware_chain = gateway.middleware_chain();
    middleware_chain.add_middleware(Arc::new(|req, next| Box::pin(async move {
        println!("Request: {} {}", req.method, req.path);
        next(req).await
    }))).await;
    
    let sample_request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/api/v1/health".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let response = gateway.handle_request(sample_request).await;
    println!("Response: {} {}", response.status_code, String::from_utf8_lossy(&response.body));
    
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
use dms::gateway::{DMSCRoute, DMSCRouter};

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
}).await?;

let route = router.route(&request).await?;
```

<div align="center">

## Middleware

</div>

### DMSCMiddleware

Middleware interface.

```rust
use dms::gateway::{DMSCMiddleware, DMSCGatewayRequest, DMSCGatewayResponse};

struct LoggingMiddleware;

#[async_trait]
impl DMSCMiddleware for LoggingMiddleware {
    async fn handle(&self, req: &mut DMSCGatewayRequest, next: RouteNext) -> Result<DMSCGatewayResponse, String> {
        println!("Before: {} {}", req.method, req.path);
        
        let response = next.run(req).await?;
        
        println!("After: {} {}", response.status_code, req.path);
        
        Ok(response)
    }
}

gateway.middleware_chain().add_middleware(Arc::new(LoggingMiddleware)).await;
```

### Built-in Middleware

```rust
use dms::gateway::DMSCRateLimiter;

let rate_limiter = DMSCRateLimiter::new(DMSCRateLimitConfig::default());
gateway.set_rate_limiter(Some(Arc::new(rate_limiter)));

let circuit_breaker = DMSCCircuitBreaker::new(DMSCCircuitBreakerConfig::default());
gateway.set_circuit_breaker(Some(Arc::new(circuit_breaker)));
```

<div align="center">

## Load Balancing

</div>

### DMSCLoadBalancer

Load balancer.

```rust
use dms::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};

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
use dms::gateway::{DMSCRateLimiter, DMSCRateLimitConfig};

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
use dms::gateway::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig};

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
- [core](./core.md): Core module providing error handling and service context
- [log](./log.md): Logging module for gateway request logging
- [config](./config.md): Configuration module for gateway settings
- [service_mesh](./service_mesh.md): Service mesh module working with gateway for service governance
- [observability](./observability.md): Observability module for gateway performance monitoring
