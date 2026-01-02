<div align="center">

# Gateway API参考

**Version: 0.0.3**

**Last modified date: 2026-01-01**

gateway模块提供API网关功能，包括路由、中间件、负载均衡、限流和熔断支持。

## 模块概述

</div>

gateway模块包含以下子模块：

- **routing**: 路由管理
- **middleware**: 中间件链
- **load_balancer**: 负载均衡
- **rate_limiter**: 限流器
- **circuit_breaker**: 熔断器

<div align="center">

## 核心组件

</div>

### DMSCGateway

API网关主接口，提供统一的网关功能。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建网关实例 | 无 | `Self` |
| `router()` | 获取路由器 | 无 | `Arc<DMSCRouter>` |
| `middleware_chain()` | 获取中间件链 | 无 | `Arc<DMSCMiddlewareChain>` |
| `handle_request(request)` | 处理请求 | `request: DMSCGatewayRequest` | `DMSCGatewayResponse` |

#### 使用示例

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

网关请求结构体。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 请求ID |
| `method` | `String` | HTTP方法 |
| `path` | `String` | 请求路径 |
| `headers` | `HashMap<String, String>` | HTTP头 |
| `query_params` | `HashMap<String, String>` | 查询参数 |
| `body` | `Option<Vec<u8>>` | 请求体 |
| `remote_addr` | `String` | 客户端地址 |
| `timestamp` | `Instant` | 请求时间戳 |

### DMSCGatewayResponse

网关响应结构体。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(status_code, body, request_id)` | 创建响应 | `status_code: u16`, `body: Vec<u8>`, `request_id: String` | `Self` |
| `json(status_code, data, request_id)` | 创建JSON响应 | `status_code: u16`, `data: &T`, `request_id: String` | `DMSCResult<Self>` |
| `error(status_code, message, request_id)` | 创建错误响应 | `status_code: u16`, `message: String`, `request_id: String` | `Self` |
| `with_header(key, value)` | 添加响应头 | `key: String`, `value: String` | `Self` |

<div align="center">

## 路由管理

</div>

### DMSCRoute

路由定义。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `path` | `String` | 路由路径 |
| `method` | `String` | HTTP方法 |
| `handler` | `Arc<RouteHandler>` | 处理器 |
| `middleware` | `Vec<Arc<DMSCMiddleware>>` | 中间件列表 |
| `timeout` | `Duration` | 超时时间 |

### DMSCRouter

路由器。

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

## 中间件

</div>

### DMSCMiddleware

中间件接口。

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

### 内置中间件

```rust
use dms::gateway::DMSCRateLimiter;

let rate_limiter = DMSCRateLimiter::new(DMSCRateLimitConfig::default());
gateway.set_rate_limiter(Some(Arc::new(rate_limiter)));

let circuit_breaker = DMSCCircuitBreaker::new(DMSCCircuitBreakerConfig::default());
gateway.set_circuit_breaker(Some(Arc::new(circuit_breaker)));
```

<div align="center">

## 负载均衡

</div>

### DMSCLoadBalancer

负载均衡器。

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

### 负载均衡策略

| 策略 | 描述 |
|:--------|:-------------|
| `RoundRobin` | 轮询 |
| `LeastConnections` | 最少连接 |
| `Weighted` | 加权 |
| `Random` | 随机 |
| `IPHash` | IP哈希 |

<div align="center">

## 限流

</div>

### DMSCRateLimiter

限流器。

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

限流配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `requests_per_second` | `u64` | 每秒请求数 | `100` |
| `burst_size` | `u64` | 突发大小 | `200` |
| `window_size` | `Duration` | 时间窗口 | `60s` |

<div align="center">

## 熔断器

</div>

### DMSCCircuitBreaker

熔断器。

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

熔断器配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `failure_threshold` | `u32` | 失败阈值 | `5` |
| `success_threshold` | `u32` | 成功阈值 | `2` |
| `timeout_duration` | `Duration` | 超时时间 | `30s` |

<div align="center">

## 配置

</div>

### DMSCGatewayConfig

网关配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `listen_address` | `String` | 监听地址 | `"0.0.0.0"` |
| `listen_port` | `u16` | 监听端口 | `8080` |
| `max_connections` | `usize` | 最大连接数 | `10000` |
| `request_timeout_seconds` | `u64` | 请求超时(秒) | `30` |
| `enable_rate_limiting` | `bool` | 启用限流 | `true` |
| `enable_circuit_breaker` | `bool` | 启用熔断 | `true` |
| `enable_load_balancing` | `bool` | 启用负载均衡 | `true` |
| `cors_enabled` | `bool` | 启用CORS | `true` |
| `cors_origins` | `Vec<String>` | CORS来源 | `["*"]` |
| `cors_methods` | `Vec<String>` | CORS方法 | `["GET","POST","PUT","DELETE","OPTIONS"]` |
| `cors_headers` | `Vec<String>` | CORS头 | `["Content-Type","Authorization","X-Requested-With"]` |
| `enable_logging` | `bool` | 启用日志 | `true` |
| `log_level` | `String` | 日志级别 | `"info"` |

<div align="center">

## 最佳实践

</div>

1. **合理配置超时**：根据业务需求设置合适的请求超时时间
2. **启用熔断保护**：防止级联故障，提高系统稳定性
3. **使用限流保护**：防止恶意请求和过载
4. **合理使用中间件**：按需添加中间件，避免过多影响性能
5. **配置CORS**：正确配置跨域资源共享策略
6. **启用日志**：记录请求日志便于问题排查

<div align="center">

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [log](./log.md): 日志模块，记录网关请求日志
- [config](./config.md): 配置模块，管理网关配置
- [service_mesh](./service_mesh.md): 服务网格模块，与网关配合实现服务治理
- [observability](./observability.md): 可观测性模块，监控网关性能
