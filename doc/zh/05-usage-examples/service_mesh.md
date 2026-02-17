<div align="center">

# Service Mesh 使用指南

**Version: 0.1.7**

**Last modified date: 2026-02-17**

本文档提供 DMSC 服务网格的完整使用示例，演示如何利用服务发现、健康检查、流量管理、负载均衡和熔断器等功能构建分布式系统。

## 目录

</div>

1. [服务网格基础](#服务网格基础)
2. [服务注册](#服务注册)
3. [服务发现](#服务发现)
4. [健康检查](#健康检查)
5. [流量管理](#流量管理)
6. [熔断器](#熔断器)
7. [负载均衡](#负载均衡)
8. [完整示例](#完整示例)

</div>

---

## 服务网格基础

服务网格是 DMSC 的核心组件，提供服务间通信的完整解决方案。它集成了服务发现、健康检查、流量管理、负载均衡和熔断器等功能。

### 创建服务网格

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
use dmsc::prelude::*;

async fn create_service_mesh() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    Ok(())
}
```

### 自定义配置

```rust
use dmsc::service_mesh::DMSCServiceMeshConfig;
use dmsc::gateway::{DMSCCircuitBreakerConfig, DMSCLoadBalancerStrategy};
use std::time::Duration;

async fn create_custom_mesh() -> DMSCResult<()> {
    let config = DMSCServiceMeshConfig {
        enable_service_discovery: true,
        enable_health_check: true,
        enable_traffic_management: true,
        health_check_interval: Duration::from_secs(30),
        circuit_breaker_config: DMSCCircuitBreakerConfig::default(),
        load_balancer_strategy: DMSCLoadBalancerStrategy::RoundRobin,
        max_retry_attempts: 3,
        retry_timeout: Duration::from_secs(5),
    };
    
    let mesh = DMSCServiceMesh::new(config)?;
    Ok(())
}
```

### 默认配置

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn use_default_config() -> DMSCResult<()> {
    let mesh_config = DMSCServiceMeshConfig::default();
    let mesh = DMSCServiceMesh::new(mesh_config)?;
    Ok(())
}
```

---

## 服务注册

服务注册是将服务实例添加到服务网格的过程，使其他服务能够发现并调用该服务。

### 基本服务注册

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn register_basic_service() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 注册用户服务
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    
    // 注册订单服务
    mesh.register_service("order-service", "http://order-service:8080", 100).await?;
    
    // 注册支付服务
    mesh.register_service("payment-service", "http://payment-service:8080", 100).await?;
    
    Ok(())
}
```

### 带权重的服务注册

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn register_weighted_services() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 注册主服务实例（权重100）
    mesh.register_service(
        "api-service",
        "http://api-primary:8080",
        100,
    ).await?;
    
    // 注册备份服务实例（权重50）
    mesh.register_service(
        "api-service",
        "http://api-backup:8080",
        50,
    ).await?;
    
    // 注册开发环境服务实例（权重10）
    mesh.register_service(
        "api-service",
        "http://api-dev:8080",
        10,
    ).await?;
    
    Ok(())
}
```

### 批量服务注册

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn batch_registration() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    let services = vec![
        ("user-service", "http://user-service:8080", 100),
        ("order-service", "http://order-service:8080", 100),
        ("payment-service", "http://payment-service:8080", 100),
        ("notification-service", "http://notification-service:8080", 80),
        ("analytics-service", "http://analytics-service:8080", 60),
    ];
    
    for (name, endpoint, weight) in services {
        mesh.register_service(name, endpoint, weight).await?;
    }
    
    println!("已注册 {} 个服务", services.len());
    
    Ok(())
}
```

---

## 服务发现

服务发现允许服务网格自动查找可用的服务实例，并返回健康的实例列表。

### 发现服务实例

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn discover_services() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 先注册一些服务
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    mesh.register_service("user-service", "http://user-service-backup:8080", 50).await?;
    
    // 发现用户服务
    let endpoints = mesh.discover_service("user-service").await?;
    
    println!("发现 {} 个用户服务实例", endpoints.len());
    for ep in &endpoints {
        println!("  - {} (权重: {})", ep.endpoint, ep.weight);
    }
    
    Ok(())
}
```

### 处理服务不可用

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn handle_service_not_found() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 尝试发现不存在的服务
    match mesh.discover_service("non-existent-service").await {
        Ok(endpoints) => {
            println!("找到服务实例: {:?}", endpoints);
        }
        Err(e) => {
            println!("服务发现失败: {}", e);
        }
    }
    
    Ok(())
}
```

### 发现健康实例

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceHealthStatus};
use dmsc::prelude::*;

async fn discover_healthy_instances() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 注册多个服务实例
    mesh.register_service("api-service", "http://api-1:8080", 100).await?;
    mesh.register_service("api-service", "http://api-2:8080", 100).await?;
    mesh.register_service("api-service", "http://api-3:8080", 100).await?;
    
    // 标记部分实例为不健康
    mesh.update_service_health("api-service", "http://api-2:8080", false).await?;
    
    // 发现服务只会返回健康实例
    let endpoints = mesh.discover_service("api-service").await?;
    
    println!("找到 {} 个健康实例", endpoints.len());
    for ep in &endpoints {
        println!("  - {} (状态: {:?})", ep.endpoint, ep.health_status);
    }
    
    Ok(())
}
```

---

## 健康检查

健康检查机制持续监控服务实例的状态，确保只有健康的实例接收流量。

### 手动更新健康状态

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceHealthStatus};
use dmsc::prelude::*;

async fn manual_health_update() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 注册服务
    mesh.register_service("database-service", "http://db:5432", 100).await?;
    
    // 模拟健康检查结果
    mesh.update_service_health("database-service", "http://db:5432", true).await?;
    
    // 服务变为不健康
    mesh.update_service_health("database-service", "http://db:5432", false).await?;
    
    // 服务恢复健康
    mesh.update_service_health("database-service", "http://db:5432", true).await?;
    
    Ok(())
}
```

### 健康状态参考

| 状态 | 描述 |
|------|------|
| `Healthy` | 服务健康且可用 |
| `Unhealthy` | 服务不健康且不可用 |
| `Unknown` | 服务健康状态未知 |

---

## 流量管理

流量管理提供智能路由和请求转发功能，支持流量分割、超时控制和重试机制。

### 调用服务

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn call_service() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 注册服务
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    
    // 调用用户服务
    let request_data = r#"{"user_id": "123"}"#.as_bytes().to_vec();
    let response = mesh.call_service("user-service", request_data).await?;
    
    println!("响应: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

### 带重试的调用

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn call_with_retry() -> DMSCResult<()> {
    let config = DMSCServiceMeshConfig {
        enable_service_discovery: true,
        enable_health_check: true,
        enable_traffic_management: true,
        health_check_interval: std::time::Duration::from_secs(30),
        circuit_breaker_config: dmsc::gateway::DMSCCircuitBreakerConfig::default(),
        load_balancer_strategy: dmsc::gateway::DMSCLoadBalancerStrategy::RoundRobin,
        max_retry_attempts: 5, // 最多重试5次
        retry_timeout: std::time::Duration::from_secs(10),
    };
    
    let mesh = DMSCServiceMesh::new(config)?;
    mesh.register_service("unreliable-service", "http://unreliable:8080", 100).await?;
    
    let request = b"Test request".to_vec();
    
    // 自动重试失败的请求
    let response = mesh.call_service("unreliable-service", request).await?;
    
    println!("最终响应: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

---

## 熔断器

熔断器是保护系统免受级联故障影响的关键组件。当服务故障率超过阈值时，熔断器会打开，快速失败并防止故障扩散。

### 熔断器状态

| 状态 | 描述 |
|------|------|
| `Closed` | 正常运行，允许请求通过 |
| `Open` | 熔断打开，拒绝所有请求 |
| `HalfOpen` | 半开状态，允许部分请求测试服务恢复 |

### 获取熔断器实例

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn access_circuit_breaker() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 获取熔断器引用
    let circuit_breaker = mesh.get_circuit_breaker();
    
    // 检查熔断器状态
    let state = circuit_breaker.get_state().await;
    println!("熔断器状态: {:?}", state);
    
    Ok(())
}
```

### 熔断器配置

```rust
use dmsc::service_mesh::DMSCServiceMeshConfig;
use dmsc::gateway::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig};

async fn configure_circuit_breaker() -> DMSCResult<()> {
    let config = DMSCServiceMeshConfig {
        enable_service_discovery: true,
        enable_health_check: true,
        enable_traffic_management: true,
        health_check_interval: std::time::Duration::from_secs(30),
        circuit_breaker_config: DMSCCircuitBreakerConfig {
            failure_threshold: 5, // 5次失败后打开
            success_threshold: 2, // 2次成功后关闭
            timeout_duration: std::time::Duration::from_secs(30), // 30秒后尝试半开
            half_open_max_requests: 10,
        },
        load_balancer_strategy: dmsc::gateway::DMSCLoadBalancerStrategy::RoundRobin,
        max_retry_attempts: 3,
        retry_timeout: std::time::Duration::from_secs(5),
    };
    
    let mesh = DMSCServiceMesh::new(config)?;
    Ok(())
}
```

---

## 负载均衡

负载均衡在多个服务实例之间分配请求，确保资源合理利用和高可用性。

### 负载均衡策略

| 策略 | 描述 |
|------|------|
| `RoundRobin` | 轮询，每个请求依次分配到不同实例 |
| `Random` | 随机选择实例 |
| `LeastConnections` | 选择连接数最少的实例 |
| `WeightedRoundRobin` | 基于权重的轮询 |
| `IPHash` | 基于客户端IP的哈希 |

### 获取负载均衡器

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn access_load_balancer() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // 获取负载均衡器引用
    let load_balancer = mesh.get_load_balancer();
    
    // 获取健康服务器列表
    let servers = load_balancer.get_healthy_servers().await;
    
    println!("健康服务器数量: {}", servers.len());
    
    Ok(())
}
```

### 自定义负载均衡策略

```rust
use dmsc::service_mesh::DMSCServiceMeshConfig;
use dmsc::gateway::DMSCLoadBalancerStrategy;

async fn custom_load_balancing() -> DMSCResult<()> {
    // 使用最少连接策略
    let config = DMSCServiceMeshConfig {
        enable_service_discovery: true,
        enable_health_check: true,
        enable_traffic_management: true,
        health_check_interval: std::time::Duration::from_secs(30),
        circuit_breaker_config: dmsc::gateway::DMSCCircuitBreakerConfig::default(),
        load_balancer_strategy: DMSCLoadBalancerStrategy::LeastConnections,
        max_retry_attempts: 3,
        retry_timeout: std::time::Duration::from_secs(5),
    };
    
    let mesh = DMSCServiceMesh::new(config)?;
    
    // 注册多个服务实例
    mesh.register_service("api-service", "http://api-1:8080", 100).await?;
    mesh.register_service("api-service", "http://api-2:8080", 100).await?;
    mesh.register_service("api-service", "http://api-3:8080", 100).await?;
    
    Ok(())
}
```

---

## 完整示例

以下示例展示服务网格的完整集成，包括所有核心功能的协调使用：

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
use dmsc::gateway::{DMSCCircuitBreakerConfig, DMSCLoadBalancerStrategy};
use dmsc::prelude::*;
use std::time::Duration;

struct MicroservicesPlatform {
    mesh: DMSCServiceMesh,
}

impl MicroservicesPlatform {
    async fn new() -> DMSCResult<Self> {
        let config = DMSCServiceMeshConfig {
            enable_service_discovery: true,
            enable_health_check: true,
            enable_traffic_management: true,
            health_check_interval: Duration::from_secs(30),
            circuit_breaker_config: DMSCCircuitBreakerConfig {
                failure_threshold: 5,
                success_threshold: 2,
                timeout_duration: Duration::from_secs(30),
                half_open_max_requests: 10,
            },
            load_balancer_strategy: DMSCLoadBalancerStrategy::RoundRobin,
            max_retry_attempts: 3,
            retry_timeout: Duration::from_secs(5),
        };
        
        let mesh = DMSCServiceMesh::new(config)?;
        
        Ok(Self { mesh })
    }
    
    async fn initialize_services(&self) -> DMSCResult<()> {
        println!("正在初始化服务...");
        
        // 核心服务
        self.mesh.register_service(
            "user-service",
            "http://user-service:8080",
            100,
        ).await?;
        
        self.mesh.register_service(
            "order-service",
            "http://order-service:8080",
            100,
        ).await?;
        
        self.mesh.register_service(
            "payment-service",
            "http://payment-service:8080",
            100,
        ).await?;
        
        // 外部服务
        self.mesh.register_service(
            "notification-service",
            "http://notification-service:8080",
            80,
        ).await?;
        
        self.mesh.register_service(
            "analytics-service",
            "http://analytics-service:8080",
            60,
        ).await?;
        
        // 高可用服务（多个实例）
        self.mesh.register_service("api-gateway", "http://api-gateway-1:8080", 100).await?;
        self.mesh.register_service("api-gateway", "http://api-gateway-2:8080", 100).await?;
        
        println!("服务初始化完成");
        
        Ok(())
    }
    
    async fn process_user_order(&self, user_id: &str, order_data: &[u8]) -> DMSCResult<Vec<u8>> {
        println!("正在处理用户 {} 的订单...", user_id);
        
        // 获取用户信息
        let user_request = format!(r#"{{"user_id": "{}"}}"#, user_id).as_bytes().to_vec();
        let user_response = self.mesh.call_service("user-service", user_request).await?;
        println!("用户信息获取成功");
        
        // 创建订单
        let order_response = self.mesh.call_service("order-service", order_data.to_vec()).await?;
        println!("订单创建成功");
        
        // 处理支付
        let payment_request = order_response.clone();
        let payment_response = self.mesh.call_service("payment-service", payment_request).await?;
        println!("支付处理成功");
        
        // 发送通知
        let notification = format!(r#"{{"user_id": "{}", "message": "订单已处理"}}"#, user_id);
        let _ = self.mesh.call_service(
            "notification-service",
            notification.as_bytes().to_vec(),
        ).await;
        
        Ok(order_response)
    }
    
    async fn get_service_health(&self) -> DMSCResult<()> {
        let services = vec![
            "user-service",
            "order-service",
            "payment-service",
            "notification-service",
            "analytics-service",
            "api-gateway",
        ];
        
        println!("=== 服务健康状态 ===");
        for service in services {
            match self.mesh.discover_service(service).await {
                Ok(endpoints) => {
                    println!("{}: {} 个健康实例", service, endpoints.len());
                    for ep in endpoints {
                        println!("  - {} (权重: {})", ep.endpoint, ep.weight);
                    }
                }
                Err(e) => {
                    println!("{}: 服务不可用 - {}", service, e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn get_mesh_components(&self) {
        println!("=== 服务网格组件状态 ===");
        
        // 获取各组件
        let _ = self.mesh.get_circuit_breaker();
        let _ = self.mesh.get_load_balancer();
        let _ = self.mesh.get_health_checker();
        let _ = self.mesh.get_traffic_manager();
        let _ = self.mesh.get_service_discovery();
        
        println!("所有组件已就绪");
    }
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    println!("启动微服务平台...");
    
    let platform = MicroservicesPlatform::new().await?;
    
    platform.initialize_services().await?;
    
    platform.get_mesh_components().await;
    
    platform.get_service_health().await?;
    
    println!("\n处理示例订单...");
    let order_data = r#"{"items": [{"id": 1, "quantity": 2}], "total": 99.99}"#;
    let _result = platform.process_user_order("user-123", order_data.as_bytes()).await;
    
    println!("\n演示完成");
    
    Ok(())
}
```

### 预期输出

```
启动微服务平台...
正在初始化服务...
服务初始化完成
=== 服务网格组件状态 ===
所有组件已就绪
=== 服务健康状态 ===
user-service: 1 个健康实例
  - http://user-service:8080 (权重: 100)
order-service: 1 个健康实例
  - http://order-service:8080 (权重: 100)
payment-service: 1 个健康实例
  - http://payment-service:8080 (权重: 100)
notification-service: 1 个健康实例
  - http://notification-service:8080 (权重: 80)
analytics-service: 1 个健康实例
  - http://analytics-service:8080 (权重: 60)
api-gateway: 2 个健康实例
  - http://api-gateway-1:8080 (权重: 100)
  - http://api-gateway-2:8080 (权重: 100)

处理示例订单...
正在处理用户 user-123 的订单...
用户信息获取成功
订单创建成功
支付处理成功

演示完成
```

<div align="center">

## 相关模块

</div>

- [README](./README.md)：使用示例总览，提供快速导航
- [authentication](./authentication.md)：认证示例，包括JWT、OAuth2和多因素认证
- [basic-app](./basic-app.md)：基础应用示例
- [caching](./caching.md)：缓存示例，包括内存缓存和分布式缓存
- [database](./database.md)：数据库操作示例
- [device](./device.md)：设备控制示例
- [fs](./fs.md)：文件系统操作示例
- [gateway](./gateway.md)：API网关示例
- [grpc](./grpc.md)：gRPC 示例，实现高性能 RPC 调用
- [hooks](./hooks.md)：钩子系统示例
- [observability](./observability.md)：可观测性示例
- [protocol](./protocol.md)：协议模块示例
- [validation](./validation.md)：数据验证示例
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信
