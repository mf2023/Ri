<div align="center">

# ServiceMesh API参考

**Version: 0.1.6**

**Last modified date: 2026-01-18**

service_mesh模块提供服务网格功能，包括服务发现、健康检查、流量管理和负载均衡。

## 模块概述

</div>

service_mesh模块包含以下子模块：

- **service_discovery**: 服务发现
- **health_check**: 健康检查
- **traffic_management**: 流量管理

<div align="center">

## 核心组件

</div>

### DMSCServiceMesh

服务网格主接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(config)` | 创建服务网格实例 | `config: DMSCServiceMeshConfig` | `DMSCResult<Self>` |
| `register_service(service_name, endpoint, weight, metadata)` | 注册服务（支持元数据） | `service_name: &str`, `endpoint: &str`, `weight: u32`, `metadata: Option<HashMap<String, String>>` | `DMSCResult<()>` |
| `register_versioned_service(service_name, version, endpoint, weight, metadata)` | 注册带版本的服务 | `service_name: &str`, `version: &str`, `endpoint: &str`, `weight: u32`, `metadata: Option<HashMap<String, String>>` | `DMSCResult<()>` |
| `unregister_service(service_name, endpoint)` | 注销服务 | `service_name: &str`, `endpoint: &str` | `DMSCResult<()>` |
| `discover_service(service_name)` | 发现健康的服务的端点 | `service_name: &str` | `DMSCResult<Vec<DMSCServiceEndpoint>>` |
| `get_all_endpoints(service_name)` | 获取所有端点（无论健康状态） | `service_name: &str` | `DMSCResult<Vec<DMSCServiceEndpoint>>` |
| `call_service(service_name, request_data)` | 调用服务 | `service_name: &str`, `request_data: Vec<u8>` | `DMSCResult<Vec<u8>>` |
| `update_service_health(service_name, endpoint, is_healthy)` | 更新服务健康状态 | `service_name: &str`, `endpoint: &str`, `is_healthy: bool` | `DMSCResult<()>` |
| `get_stats()` | 获取服务网格统计信息 | 无 | `DMSCResult<DMSCServiceMeshStats>` |
| `get_circuit_breaker()` | 获取熔断器 | 无 | `&DMSCCircuitBreaker` |
| `get_load_balancer()` | 获取负载均衡器 | 无 | `&DMSCLoadBalancer` |
| `get_health_checker()` | 获取健康检查器 | 无 | `&DMSCHealthChecker` |
| `get_traffic_manager()` | 获取流量管理器 | 无 | `&DMSCTrafficManager` |
| `get_service_discovery()` | 获取服务发现 | 无 | `&DMSCServiceDiscovery` |

#### 使用示例

```rust
use dmsc::prelude::*;
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
use std::collections::HashMap;

async fn example() -> DMSCResult<()> {
    let mesh_config = DMSCServiceMeshConfig::default();
    
    let service_mesh = DMSCServiceMesh::new(mesh_config)?;
    
    // 带元数据注册服务
    let mut metadata = HashMap::new();
    metadata.insert("region".to_string(), "us-east-1".to_string());
    service_mesh.register_service("user-service", "http://user-service:8080", 100, Some(metadata)).await?;
    
    // 注册带版本的服务
    service_mesh.register_versioned_service("api-service", "v2.0", "http://api-service-v2:8080", 100, None).await?;
    
    service_mesh.register_service("order-service", "http://order-service:8080", 100, None).await?;
    service_mesh.register_service("payment-service", "http://payment-service:8080", 100, None).await?;
    
    // 发现健康的端点
    let user_service_endpoints = service_mesh.discover_service("user-service").await?;
    println!("User service endpoints: {:?}", user_service_endpoints);
    
    // 获取所有端点
    let all_endpoints = service_mesh.get_all_endpoints("user-service").await?;
    
    // 获取统计信息
    let stats = service_mesh.get_stats().await?;
    println!("Total services: {}", stats.total_services);
    println!("Healthy endpoints: {}", stats.healthy_endpoints);
    
    let request_data = r#"{ "user_id": "123" }"#.as_bytes().to_vec();
    let response = service_mesh.call_service("user-service", request_data).await?;
    println!("Service response: {}", String::from_utf8_lossy(&response));
    
    // 注销服务
    service_mesh.unregister_service("user-service", "http://user-service:8080").await?;
    
    let health_checker = service_mesh.get_health_checker();
    let traffic_manager = service_mesh.get_traffic_manager();
    let circuit_breaker = service_mesh.get_circuit_breaker();
    let load_balancer = service_mesh.get_load_balancer();
    
    Ok(())
}
```

### DMSCServiceMeshStats

服务网格统计信息。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `total_services` | `usize` | 已注册服务总数 |
| `total_endpoints` | `usize` | 已注册端点总数 |
| `healthy_endpoints` | `usize` | 健康端点数量 |
| `unhealthy_endpoints` | `usize` | 不健康端点数量 |

### DMSCServiceMeshConfig

服务网格配置。

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `enable_service_discovery` | `bool` | 启用服务发现 | `true` |
| `enable_health_check` | `bool` | 启用健康检查 | `true` |
| `enable_traffic_management` | `bool` | 启用流量管理 | `true` |
| `health_check_interval` | `Duration` | 健康检查间隔 | `30s` |
| `circuit_breaker_config` | `DMSCCircuitBreakerConfig` | 熔断器配置 | 默认配置 |
| `load_balancer_strategy` | `DMSCLoadBalancerStrategy` | 负载均衡策略 | `RoundRobin` |
| `max_retry_attempts` | `u32` | 最大重试次数 | `3` |
| `retry_timeout` | `Duration` | 重试超时 | `5s` |

<div align="center">

## 服务发现

</div>

### DMSCServiceDiscovery

服务发现组件。

```rust
use dmsc::service_mesh::DMSCServiceDiscovery;

let discovery = DMSCServiceDiscovery::new(true);

discovery.start_background_tasks().await?;

let endpoints = discovery.discover("user-service").await?;

discovery.stop_background_tasks().await?;
```

### DMSCServiceEndpoint

服务端点。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `service_name` | `String` | 服务名称 |
| `endpoint` | `String` | 端点URL |
| `weight` | `u32` | 负载均衡权重 |
| `metadata` | `HashMap<String, String>` | 元数据 |
| `health_status` | `DMSCServiceHealthStatus` | 健康状态 |
| `last_health_check` | `SystemTime` | 最后健康检查时间 |

### DMSCServiceHealthStatus

服务健康状态。

| 变体 | 描述 |
|:--------|:-------------|
| `Healthy` | 健康 |
| `Unhealthy` | 不健康 |
| `Unknown` | 未知 |

<div align="center">

## 健康检查

</div>

### DMSCHealthChecker

健康检查器。

```rust
use dmsc::service_mesh::{DMSCHealthChecker, DMSCHealthStatus};

let health_checker = DMSCHealthChecker::new(Duration::from_secs(30));

health_checker.start_health_check("user-service", "http://user-service:8080/health").await?;

// 停止特定服务的健康检查
health_checker.stop_health_check("user-service", "http://user-service:8080/health").await?;

let summary = health_checker.get_health_summary().await?;
println!("Healthy services: {}", summary.healthy_count);
println!("Unhealthy services: {}", summary.unhealthy_count);

health_checker.stop_background_tasks().await?;
```

### DMSCHealthCheckResult

健康检查结果。

```rust
let result = health_checker.check_health("http://user-service:8080").await?;

match result.status {
    DMSCHealthStatus::Healthy => println!("Service is healthy"),
    DMSCHealthStatus::Unhealthy => println!("Service is unhealthy: {:?}", result.error),
    DMSCHealthStatus::Unknown => println!("Service health unknown"),
}
```

### DMSCHealthSummary

健康检查汇总。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `healthy_count` | `usize` | 健康服务数 |
| `unhealthy_count` | `usize` | 不健康服务数 |
| `unknown_count` | `usize` | 未知服务数 |
| `total_services` | `usize` | 服务总数 |

<div align="center">

## 流量管理

</div>

### DMSCTrafficManager

流量管理器。

```rust
use dmsc::service_mesh::{DMSCTrafficManager, DMSCTrafficRoute, DMSCMatchCriteria, DMSCRouteAction};

let traffic_manager = DMSCTrafficManager::new(true);

let route = DMSCTrafficRoute {
    name: "api-route".to_string(),
    match_criteria: DMSCMatchCriteria {
        path_prefix: Some("/api/".to_string()),
        headers: HashMap::new(),
        methods: vec!["GET".to_string()],
    },
    action: DMSCRouteAction::RouteTo {
        service_name: "api-service".to_string(),
        weight: 100,
    },
    timeout: Duration::from_secs(30),
    retry_count: 3,
};

traffic_manager.add_route(route).await?;

traffic_manager.start_background_tasks().await?;
traffic_manager.stop_background_tasks().await?;
```

### DMSCTrafficRoute

流量路由。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `name` | `String` | 路由名称 |
| `match_criteria` | `DMSCMatchCriteria` | 匹配条件 |
| `action` | `DMSCRouteAction` | 路由动作 |
| `timeout` | `Duration` | 超时时间 |
| `retry_count` | `u32` | 重试次数 |

### DMSCMatchCriteria

匹配条件。

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `path_prefix` | `Option<String>` | 路径前缀 |
| `headers` | `HashMap<String, String>` | 请求头 |
| `methods` | `Vec<String>` | HTTP方法 |

### DMSCRouteAction

路由动作。

| 变体 | 描述 |
|:--------|:-------------|
| `RouteTo { service_name, weight }` | 路由到服务 |
| `Redirect { url }` | 重定向 |
| `Rewrite { path }` | 路径重写 |
| `CircuitBreak` | 熔断 |

<div align="center">

## 熔断器

</div>

```rust
use dmsc::gateway::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig};

let circuit_breaker = service_mesh.get_circuit_breaker();

if circuit_breaker.allow_request().await {
    println!("Request allowed");
} else {
    println!("Circuit breaker is open");
}

circuit_breaker.record_success().await;
circuit_breaker.record_failure().await;
```

<div align="center>

## 负载均衡

</div>

```rust
use dmsc::gateway::DMSCLoadBalancer;

let load_balancer = service_mesh.get_load_balancer();

let selected_server = load_balancer.select_server(None).await?;
println!("Selected server: {}", selected_server.url);

load_balancer.add_server(DMSCBackendServer {
    id: "new-server".to_string(),
    url: "http://new-server:8080".to_string(),
    weight: 100,
    max_connections: 1000,
    health_check_path: "/health".to_string(),
    is_healthy: true,
}).await;
```

<div align="center>

## 最佳实践

</div>

1. **启用健康检查**：定期检查服务健康状态，自动移除不健康实例
2. **配置合理的超时**：根据服务响应时间设置合适的超时
3. **使用重试机制**：对瞬时故障进行自动重试
4. **启用熔断保护**：防止级联故障
5. **配置负载均衡**：合理分配请求到不同实例
6. **监控服务状态**：定期检查服务网格状态

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
- [grpc](./grpc.md): gRPC 模块，带服务注册和 Python 绑定
- [hooks](./hooks.md): 钩子模块，提供生命周期钩子支持
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [log](./log.md): 日志模块，记录协议事件
- [mq](./mq.md): 消息队列模块，提供消息队列支持
- [observability](./observability.md): 可观测性模块，监控协议性能
- [orm](./orm.md): ORM 模块，带查询构建器和分页支持
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [storage](./storage.md): 存储模块，提供云存储支持
- [validation](./validation.md): 验证模块，提供数据验证功能
- [ws](./ws.md): WebSocket 模块，带 Python 绑定的实时通信
