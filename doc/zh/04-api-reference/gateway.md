<div align="center">

# Gateway API参考

**Version: 0.1.6**

**Last modified date: 2026-01-24**

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

**注意**：`handle_request`方法为内部方法，仅在crate内部使用。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建网关实例 | 无 | `Self` |
| `router()` | 获取路由器 | 无 | `Arc<DMSCRouter>` |
| `middleware_chain()` | 获取中间件链 | 无 | `Arc<DMSCMiddlewareChain>` |
| `handle_request(request)` | 处理请求（内部方法） | `request: DMSCGatewayRequest` | `DMSCGatewayResponse` |
| `after_shutdown(ctx)` | 网关关闭后的清理操作（实现DMSCModule trait） | `ctx: &mut DMSCServiceContext` | `DMSCResult<()>` |

#### 使用示例

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

// 或者使用简写方法
router.get("/api/users", Arc::new(|req| Box::pin(async move {
    let users = vec![
        serde_json::json!({"id": 1, "name": "Alice"}),
        serde_json::json!({"id": 2, "name": "Bob"}),
    ];
    Ok(DMSCGatewayResponse::json(200, &users, req.id.clone())?)
})));
```

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new()` | 创建路由器 | 无 | `Self` |
| `add_route(route)` | 添加路由 | `route: DMSCRoute` | `()` |
| `get(path, handler)` | 添加GET路由 | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `post(path, handler)` | 添加POST路由 | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `put(path, handler)` | 添加PUT路由 | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `delete(path, handler)` | 添加DELETE路由 | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `patch(path, handler)` | 添加PATCH路由 | `path: &str`, `handler: DMSCRouteHandler` | `()` |
| `clear_routes()` | 清除所有路由 | 无 | `()` |
| `route_count()` | 获取路由数量 | 无 | `usize` |

<div align="center">

## 中间件

</div>

### DMSCMiddleware

中间件接口。

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

### 内置中间件

```rust
use dmsc::gateway::DMSCRateLimiter;

let rate_limiter = Arc::new(DMSCRateLimiter::new(DMSCRateLimitConfig::default()));
```

<div align="center">

## 负载均衡

</div>

### DMSCLoadBalancer

负载均衡器。

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

熔断器配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `failure_threshold` | `u32` | 失败阈值 | `5` |
| `success_threshold` | `u32` | 成功阈值 | `2` |
| `timeout_duration` | `Duration` | 超时时间 | `30s` |

<div align="center">

## 负载均衡

</div>

### DMSCBackendServer

后端服务器配置和状态，用于负载均衡。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 服务器唯一标识符 |
| `url` | `String` | 服务器基础URL（如"http://localhost:8080"） |
| `weight` | `u32` | 加权负载均衡的权重 |
| `max_connections` | `usize` | 最大并发连接数 |
| `health_check_path` | `String` | 健康检查路径 |
| `is_healthy` | `bool` | 当前健康状态 |

```rust
use dmsc::gateway::load_balancer::DMSCBackendServer;

let server = DMSCBackendServer::new("server-1".to_string(), "http://localhost:8080".to_string())
    .with_weight(2)
    .with_max_connections(100)
    .with_health_check_path("/health".to_string());
```

### LoadBalancerServerStats

负载均衡服务器统计信息，用于监控。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `active_connections` | `usize` | 当前活跃连接数 |
| `total_requests` | `usize` | 自添加以来的总请求数 |
| `failed_requests` | `usize` | 失败请求数 |
| `response_time_ms` | `usize` | 最近响应时间（毫秒） |

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
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库操作支持
- [device](./device.md): 设备模块，使用协议进行设备通信
- [fs](./fs.md): 文件系统模块，提供文件操作功能
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
