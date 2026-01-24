<div align="center">

# gRPC API参考

**版本: 0.1.6**

**最后修改日期: 2026-01-16**

gRPC模块提供高性能RPC（远程过程调用）功能，支持服务注册、发现和负载均衡。支持服务器和客户端模式，并提供Python绑定。

## 模块概述

</div>

gRPC模块包含以下子模块：

- **server**: gRPC服务器实现和服务注册
- **client**: 带负载均衡的gRPC客户端实现
- **discovery**: 服务发现和注册
- **load_balancer**: 客户端负载均衡
- **interceptor**: 请求/响应拦截器

<div align="center">

## 核心组件

</div>

### DMSCGrpcServiceRegistry

用于管理gRPC服务的服务注册表，支持Python绑定。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `register(name, handler)` | 注册新服务 | `name: &str`, `handler: PyCallable` | `DMSCResult<()>` |
| `unregister(name)` | 取消注册服务 | `name: &str` | `DMSCResult<()>` |
| `get(name)` | 获取服务处理器 | `name: &str` | `DMSCResult<Option<PyObject>>` |
| `list_services()` | 列出所有已注册服务 | 无 | `DMSCResult<Vec<String>>` |
| `register_with_config(name, handler, config)` | 带配置注册 | `name: &str`, `handler: PyCallable`, `config: PyDict` | `DMSCResult<()>` |

#### Python使用示例

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

registry = DMSCGrpcServiceRegistryPy()

def my_handler(method: str, data: bytes) -> bytes:
    print(f"收到请求: {method}")
    return b"来自Python处理器的响应"

# 注册服务
registry.register("my-service", my_handler)

# 列出所有服务
services = registry.list_services()
print(f"已注册服务: {services}")

# 获取特定服务
service = registry.get("my-service")
```

### DMSCGrpcConfig

gRPC服务器配置结构。

#### 字段

| 字段 | 类型 | 描述 | 默认值 |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | 服务器主机 | `"0.0.0.0"` |
| `port` | `u16` | 服务器端口 | `50051` |
| `max_concurrent_rpcs` | `u32` | 最大并发RPC数 | `100` |
| `max_receive_message_size` | `u32` | 最大接收消息大小(字节) | `4194304` |
| `max_send_message_size` | `u32` | 最大发送消息大小(字节) | `4194304` |
| `keepalive_time` | `Duration` | 保活时间 | `30000ms` |
| `keepalive_timeout` | `Duration` | 保活超时 | `5000ms` |
| `tls_enabled` | `bool` | 启用TLS | `false` |
| `tls_cert_path` | `Option<String>` | TLS证书路径 | `None` |
| `tls_key_path` | `Option<String>` | TLS密钥路径 | `None` |

#### 配置示例

```rust
use dmsc::prelude::*;

let config = DMSCGrpcConfig {
    host: "0.0.0.0".to_string(),
    port: 50051,
    max_concurrent_rpcs: 100,
    max_receive_message_size: 4 * 1024 * 1024,
    max_send_message_size: 4 * 1024 * 1024,
    keepalive_time: Duration::from_secs(30),
    keepalive_timeout: Duration::from_secs(5),
    tls_enabled: false,
    tls_cert_path: None,
    tls_key_path: None,
};
```

### DMSCGrpcServer

gRPC服务器主接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `start()` | 启动gRPC服务器 | 无 | `DMSCResult<()>` |
| `stop()` | 停止gRPC服务器 | 无 | `DMSCResult<()>` |
| `add_service<S>(&mut self, service: S)` | 添加服务 | `service: S` | `DMSCResult<()>` |
| `is_running(&self)` | 检查服务器是否运行 | 无 | `bool` |

#### 使用示例

```rust
use dmsc::prelude::*;

let config = DMSCGrpcConfig::default();
let mut server = DMSCGrpcServer::new(config)?;

server.start()?;
println!("gRPC服务器已启动在端口 {}", config.port);

// 服务器持续运行直到停止
server.stop()?;
println!("gRPC服务器已停止");
```

<div align="center">

## 服务发现

</div>

### DMSCServiceDiscovery

用于动态服务注册和查找的服务发现接口。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `register(service)` | 注册服务 | `service: DMSCServiceInfo` | `DMSCResult<()>` |
| `deregister(service_id)` | 取消注册服务 | `service_id: &str` | `DMSCResult<()>` |
| `lookup(service_name)` | 按名称查找服务 | `service_name: &str` | `DMSCResult<Vec<DMSCServiceInfo>>` |
| `get_all()` | 获取所有已注册服务 | 无 | `DMSCResult<Vec<DMSCServiceInfo>>` |
| `watch()` | 监视服务变化 | 无 | `DMSCResult<DMSCServiceWatcher>` |

#### 使用示例

```rust
use dmsc::prelude::*;

let discovery = DMSCServiceDiscovery::new()?;

let service = DMSCServiceInfo {
    id: "user-service-1".to_string(),
    name: "user-service".to_string(),
    host: "localhost".to_string(),
    port: 8080,
    metadata: HashMap::new(),
};

discovery.register(service)?;

let services = discovery.lookup("user-service")?;
println!("找到 {} 个用户服务", services.len());
```

### DMSCServiceInfo

用于注册的服务信息结构。

#### 字段

| 字段 | 类型 | 描述 |
|:--------|:-----|:-------------|
| `id` | `String` | 唯一服务标识符 |
| `name` | `String` | 服务名称 |
| `host` | `String` | 服务主机 |
| `port` | `u16` | 服务端口 |
| `metadata` | `HashMap<String, String>` | 附加元数据 |

<div align="center">

## 负载均衡

</div>

### DMSCLoadBalancer

用于在多个服务实例之间分配请求的客户端负载均衡器。

#### 方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `new(strategy)` | 创建负载均衡器 | `strategy: LoadBalancingStrategy` | `Self` |
| `add_endpoint(&mut self, endpoint)` | 添加端点 | `endpoint: DMSCLoadBalancerEndpoint` | `()` |
| `remove_endpoint(&mut self, id)` | 移除端点 | `id: &str` | `()` |
| `select(&self)` | 选择一个端点 | 无 | `Option<DMSCLoadBalancerEndpoint>` |
| `get_all(&self)` | 获取所有端点 | 无 | `Vec<DMSCLoadBalancerEndpoint>` |

#### 负载均衡策略

| 策略 | 描述 |
|:--------|:-------------|
| `RoundRobin` | 在所有端点之间均匀分配请求 |
| `LeastConnections` | 选择活动连接数最少的端点 |
| `Random` | 随机选择一个端点 |
| `Weighted` | 根据端点权重选择 |

#### 使用示例

```rust
use dmsc::prelude::*;

let mut lb = DMSCLoadBalancer::new(LoadBalancingStrategy::RoundRobin);

lb.add_endpoint(DMSCLoadBalancerEndpoint {
    id: "endpoint-1".to_string(),
    host: "localhost".to_string(),
    port: 8080,
    weight: 10,
    metadata: HashMap::new(),
});

lb.add_endpoint(DMSCLoadBalancerEndpoint {
    id: "endpoint-2".to_string(),
    host: "localhost".to_string(),
    port: 8081,
    weight: 10,
    metadata: HashMap::new(),
});

if let Some(endpoint) = lb.select() {
    println!("选择的端点: {}:{}", endpoint.host, endpoint.port);
}
```

<div align="center">

## 拦截器

</div>

### DMSCGrpcInterceptor

用于向请求处理添加自定义逻辑的拦截器trait。

#### Trait方法

| 方法 | 描述 | 参数 | 返回值 |
|:--------|:-------------|:--------|:--------|
| `intercept(&self, request, next)` | 拦截请求 | `request: DMSCGrpcRequest`, `next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>` | `DMSCResult<DMSCGrpcResponse>` |

#### 内置拦截器

| 拦截器 | 描述 |
|:--------|:-------------|
| `LoggingInterceptor` | 记录所有传入请求和响应 |
| `AuthInterceptor` | 验证认证令牌 |
| `RateLimitInterceptor` | 限制每个客户端的请求速率 |
| `MetricsInterceptor` | 收集请求指标 |
| `TracingInterceptor` | 添加分布式追踪头 |

#### 使用示例

```rust
use dmsc::prelude::*;

struct LoggingInterceptor;

impl DMSCGrpcInterceptor for LoggingInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        println!("收到请求: {} {}", request.method, request.path);
        let start = std::time::Instant::now();
        let response = next(request)?;
        println!("响应完成于 {:?}", start.elapsed());
        Ok(response)
    }
}

// 向服务器添加拦截器
server.add_interceptor(LoggingInterceptor)?;
```

<div align="center>

## 高级功能

</div>

### 流式支持

```rust
use dmsc::prelude::*;

// 服务器端流式
async fn stream_users(request: DMSCGrpcRequest) -> DMSCResult<DMSCGrpcStreamingResponse<User>> {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];
    
    let stream = futures::stream::iter(users);
    Ok(DMSCGrpcStreamingResponse::new(stream))
}

// 客户端流式
let request = DMSCGrpcStreamingRequest::new(
    std::iter::repeat(User { id: 0, name: "Test".to_string() }).take(100)
);
let response = client.bidirectional_stream(request).await?;
```

### 健康检查

```rust
use dmsc::prelude::*;

let health_service = DMSCGrpcHealthService::new();

// 注册健康检查处理器
health_service.register_check("service-name", move || {
    DMSCHealthCheckResult::Healthy
});

// 启动健康服务
server.add_service(health_service)?;
```

### 错误处理

```rust
use dmsc::prelude::*;

match client.call_method("user-service", "GetUser", request).await {
    Ok(response) => {
        println!("响应: {:?}", response);
    }
    Err(DMSCGrpcError::ServiceNotFound(name)) => {
        println!("服务 {} 未找到", name);
    }
    Err(DMSCGrpcError::MethodNotFound(method)) => {
        println!("方法 {} 未找到", method);
    }
    Err(DMSCGrpcError::DeadlineExceeded) => {
        println!("请求超时");
    }
    Err(e) => {
        println!("gRPC错误: {}", e);
    }
}
```

<div align="center>

## 最佳实践

</div>

1. **使用服务发现**：为可扩展部署启用动态服务注册
2. **配置超时**：设置适当的请求和截止时间
3. **启用负载均衡**：使用客户端负载均衡实现高可用性
4. **使用拦截器**：添加日志、认证和指标拦截器
5. **监控健康状况**：实现健康检查以确保服务可靠性
6. **优雅处理错误**：实现适当的错误处理和重试逻辑
7. **对大数据使用流式**：对大型数据集使用服务器/客户端流式
8. **在生产环境中启用TLS**：生产部署始终使用TLS

<div align="center>

## Python支持

</div>

gRPC模块通过PyO3提供完整的Python绑定：

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# 创建服务注册表
registry = DMSCGrpcServiceRegistryPy()

# 注册Python处理器
def user_handler(method: str, data: bytes) -> bytes:
    if method == "GetUser":
        return b'{"id": 1, "name": "Alice"}'
    return b'{"error": "unknown method"}'

registry.register("user-service", user_handler)

# 列出服务
print(registry.list_services())
```

<div align="center>

## 相关模块

</div>

- [README](./README.md): 模块概览，提供API参考文档总览和快速导航
- [auth](./auth.md): 认证模块，处理用户认证和授权
- [cache](./cache.md): 缓存模块，提供内存缓存和分布式缓存支持
- [config](./config.md): 配置模块，管理应用程序配置
- [core](./core.md): 核心模块，提供错误处理和服务上下文
- [database](./database.md): 数据库模块，提供数据库访问层
- [http](./http.md): HTTP模块，提供HTTP服务器和客户端功能
- [protocol](./protocol.md): 协议模块，提供通信协议支持
- [security](./security.md): 安全模块，提供加密和解密功能
- [service_mesh](./service_mesh.md): 服务网格模块，使用协议进行服务间通信
