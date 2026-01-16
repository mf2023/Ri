<div align="center">

# gRPC 使用示例

**Version: 0.1.4**

**Last modified date: 2026-01-16**

本示例演示如何使用 DMSC 的 gRPC 模块进行服务注册、发现、负载均衡和 RPC 调用，并支持 Python 绑定。

## 示例概述

</div>

本示例将创建一个 DMSC 应用程序，实现以下功能：

- 带服务注册的 gRPC 服务器
- 支持 Python 处理器的动态服务注册
- 服务发现和注册
- 客户端负载均衡
- 用于日志记录和认证的请求拦截器
- 健康检查端点

<div align="center>

## 前置条件

</div>

- Rust 1.65+
- Cargo 1.65+
- 基本的 Rust 编程知识
- 理解 gRPC 概念
- Python 3.8+（用于 Python 绑定示例）

<div align="center>

## 示例代码

</div>

### 1. 创建项目

```bash
cargo new dms-grpc-example
cd dms-grpc-example
```

### 2. 添加依赖

在 `Cargo.toml` 文件中添加以下依赖：

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pyo3 = { version = "0.20", features = ["extension-module"] }
```

### 3. 创建配置文件

在项目根目录创建 `config.yaml` 文件：

```yaml
service:
  name: "dms-grpc-example"
  version: "1.0.0"

logging:
  level: "info"
  format: "json"
  file_enabled: false
  console_enabled: true

grpc:
  host: "0.0.0.0"
  port: 50051
  max_concurrent_rpcs: 100
  max_receive_message_size: 4194304
  max_send_message_size: 4194304
  keepalive_time: 30000
  keepalive_timeout: 5000
  tls_enabled: false
```

### 4. 编写主代码

将 `src/main.rs` 文件替换为以下内容：

```rust
use dmsc::prelude::*;
use serde_json::json;
use std::time::Duration;

#[tokio::main]
async fn main() -> DMSCResult<()> {
    let app = DMSCAppBuilder::new()
        .with_config("config.yaml")?
        .with_logging(DMSCLogConfig::default())?
        .with_grpc(DMSCGrpcConfig::default())?
        .build()?;
    
    app.run(|ctx: &DMSCServiceContext| async move {
        ctx.logger().info("service", "DMSC gRPC 示例已启动")?;
        
        let server = ctx.grpc_server();
        
        let user_service = UserService::new();
        server.add_service(user_service)?;
        
        ctx.logger().info("grpc", "已添加用户服务")?;
        
        server.start()?;
        ctx.logger().info("grpc", &format!("gRPC 服务器已启动，端口 {}", 50051))?;
        
        Ok(())
    }).await
}

struct UserService;

impl UserService {
    fn new() -> Self {
        Self
    }
}

impl DMSCGrpcService for UserService {
    fn service_name(&self) -> &str {
        "user-service"
    }
    
    fn handle_request(
        &self,
        method: &str,
        data: &[u8],
    ) -> DMSCResult<Vec<u8>> {
        match method {
            "GetUser" => {
                let request: serde_json::Value = serde_json::from_slice(data)?;
                let user_id = request["id"].as_i64().unwrap_or(0);
                
                let user = json!({
                    "id": user_id,
                    "name": "Alice",
                    "email": "alice@example.com",
                    "age": 30,
                    "active": true
                });
                
                Ok(serde_json::to_vec(&user)?)
            }
            "CreateUser" => {
                let request: serde_json::Value = serde_json::from_slice(data)?;
                
                let response = json!({
                    "success": true,
                    "message": "用户创建成功",
                    "user": {
                        "id": 100,
                        "name": request["name"],
                        "email": request["email"]
                    }
                });
                
                Ok(serde_json::to_vec(&response)?)
            }
            "ListUsers" => {
                let users = json!([
                    {"id": 1, "name": "Alice", "email": "alice@example.com"},
                    {"id": 2, "name": "Bob", "email": "bob@example.com"},
                    {"id": 3, "name": "Charlie", "email": "charlie@example.com"}
                ]);
                
                Ok(serde_json::to_vec(&users)?)
            }
            _ => Err(DMSCError::not_found(format!("未知方法: {}", method)))
        }
    }
}
```

### 5. Python 处理器示例

创建 `python_handler.py` 文件用于基于 Python 的 gRPC 处理器：

```python
import sys
sys.path.insert(0, 'target/release')

from dmsc.grpc import DMSCGrpcServiceRegistryPy

class UserPythonHandler:
    def __init__(self):
        self.registry = DMSCGrpcServiceRegistryPy()
    
    def handle_get_user(self, method: str, data: bytes) -> bytes:
        import json
        request = json.loads(data.decode())
        user_id = request.get('id', 0)
        
        user = {
            "id": user_id,
            "name": "Python User",
            "email": "python@example.com",
            "age": 25,
            "active": True
        }
        
        return json.dumps(user).encode()
    
    def handle_create_user(self, method: str, data: bytes) -> bytes:
        import json
        request = json.loads(data.decode())
        
        response = {
            "success": True,
            "message": "用户已从 Python 处理器创建",
            "user": {
                "id": 200,
                "name": request.get('name', 'Unknown'),
                "email": request.get('email', 'unknown@example.com')
            }
        }
        
        return json.dumps(response).encode()
    
    def register_handlers(self):
        self.registry.register("get-user", self.handle_get_user)
        self.registry.register("create-user", self.handle_create_user)
        
        services = self.registry.list_services()
        print(f"已注册的 Python 服务: {services}")
        
        return self.registry

if __name__ == "__main__":
    handler = UserPythonHandler()
    registry = handler.register_handlers()
    
    import time
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("正在关闭 Python 处理器")
```

<div align="center>

## 代码分析

</div>

### 服务注册

```rust
use dmsc::prelude::*;

// 创建服务实例
let user_service = UserService::new();

// 注册到 gRPC 服务器
server.add_service(user_service)?;

println!("用户服务注册成功");
```

### Python 处理器注册

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy

registry = DMSCGrpcServiceRegistryPy()

def my_handler(method: str, data: bytes) -> bytes:
    print(f"收到: {method}")
    return b"响应"

registry.register("my-service", my_handler)

# 列出所有已注册服务
services = registry.list_services()
```

### 服务发现

```rust
use dmsc::prelude::*;

// 创建服务发现
let discovery = DMSCServiceDiscovery::new()?;

// 注册服务
let service = DMSCServiceInfo {
    id: "user-service-1".to_string(),
    name: "user-service".to_string(),
    host: "localhost".to_string(),
    port: 50051,
    metadata: HashMap::new(),
};

discovery.register(service)?;

// 查找服务
let services = discovery.lookup("user-service")?;
println!("找到 {} 个服务", services.len());
```

### 负载均衡

```rust
use dmsc::prelude::*;

// 使用轮询策略创建负载均衡器
let mut lb = DMSCLoadBalancer::new(LoadBalancingStrategy::RoundRobin);

// 添加端点
lb.add_endpoint(DMSCLoadBalancerEndpoint {
    id: "endpoint-1".to_string(),
    host: "localhost".to_string(),
    port: 50051,
    weight: 10,
    metadata: HashMap::new(),
});

lb.add_endpoint(DMSCLoadBalancerEndpoint {
    id: "endpoint-2".to_string(),
    host: "localhost".to_string(),
    port: 50052,
    weight: 10,
    metadata: HashMap::new(),
});

// 选择端点
if let Some(endpoint) = lb.select() {
    println!("选择的端点: {}:{}", endpoint.host, endpoint.port);
}
```

### 拦截器

```rust
use dmsc::prelude::*;

// 日志拦截器
struct LoggingInterceptor;

impl DMSCGrpcInterceptor for LoggingInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        println!("[{}] {} {}", request.method, request.path, request.timestamp);
        let response = next(request)?;
        println!("响应: {} - {:?}", response.status_code, response.duration);
        Ok(response)
    }
}

// 添加拦截器
server.add_interceptor(LoggingInterceptor)?;
```

### 健康检查

```rust
use dmsc::prelude::*;

let health_service = DMSCGrpcHealthService::new();

health_service.register_check("user-service", || {
    DMSCHealthCheckResult::Healthy
});

health_service.register_check("database", || {
    if ctx.database().ping().is_ok() {
        DMSCHealthCheckResult::Healthy
    } else {
        DMSCHealthCheckResult::Unhealthy("数据库连接失败".to_string())
    }
});

server.add_service(health_service)?;
```

<div align="center>

## 运行步骤

</div>

### 1. 构建项目

```bash
cargo build --release
```

### 2. 运行项目

```bash
cargo run
```

### 3. 使用 Python 客户端测试

```python
import grpc
import json

# 创建 gRPC 通道
channel = grpc.insecure_channel('localhost:50051')
stub = UserServiceStub(channel)

# 调用 GetUser
request = json.dumps({"id": 1}).encode()
response = stub.GetUser(request)
print(f"GetUser 响应: {response}")

# 调用 CreateUser
request = json.dumps({
    "name": "新用户",
    "email": "newuser@example.com"
}).encode()
response = stub.CreateUser(request)
print(f"CreateUser 响应: {response}")
```

<div align="center>

## 预期结果

</div>

运行示例后，您应该看到类似以下输出：

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC gRPC 示例已启动","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"grpc","message":"已添加用户服务","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"grpc","message":"gRPC 服务器已启动，端口 50051","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"grpc","message":"收到请求: GetUser","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"grpc","message":"响应: 200 OK (15ms)","trace_id":"abc123","span_id":"def456"}
```

<div align="center

## 扩展功能

</div>

### 1. TLS 加密

```rust
use dmsc::prelude::*;

let config = DMSCGrpcConfig {
    host: "0.0.0.0".to_string(),
    port: 50051,
    tls_enabled: true,
    tls_cert_path: Some("/path/to/server.crt".to_string()),
    tls_key_path: Some("/path/to/server.key".to_string()),
    ..Default::default()
};
```

### 2. 流式支持

```rust
use dmsc::prelude::*;

impl DMSCGrpcService for UserService {
    fn handle_streaming_request(
        &self,
        method: &str,
        stream: impl Stream<Item = DMSCResult<DMSCGrpcRequest>>,
    ) -> DMSCResult<DMSCGrpcStreamingResponse> {
        let users = vec![
            User { id: 1, name: "Alice".to_string() },
            User { id: 2, name: "Bob".to_string() },
        ];
        
        let stream = futures::stream::iter(users);
        Ok(DMSCGrpcStreamingResponse::new(stream))
    }
}
```

### 3. 认证拦截器

```rust
use dmsc::prelude::*;

struct AuthInterceptor;

impl DMSCGrpcInterceptor for AuthInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        // 从元数据中提取认证令牌
        let auth_header = request.metadata.get("authorization");
        
        match auth_header {
            Some(token) if validate_token(token) => {
                next(request)
            }
            _ => Err(DMSCError::unauthorized("无效或缺少令牌"))
        }
    }
}

fn validate_token(token: &str) -> bool {
    token.starts_with("Bearer ") && token.len() > 7
}

server.add_interceptor(AuthInterceptor)?;
```

### 4. 速率限制

```rust
use dmsc::prelude::*;

struct RateLimitInterceptor {
    rate_limiter: DMSCRateLimiter,
}

impl DMSCGrpcInterceptor for RateLimitInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        let client_id = request.client_id();
        
        if self.rate_limiter.allow(client_id) {
            next(request)
        } else {
            Err(DMSCError::too_many_requests("超出速率限制"))
        }
    }
}

server.add_interceptor(RateLimitInterceptor::new(100, Duration::from_secs(60)))?;
```

<div align="center

## 最佳实践

</div>

1. **使用服务发现**：启用动态服务注册以支持可扩展部署
2. **实现拦截器**：为生产环境添加日志、认证和指标
3. **配置超时**：设置适当的请求和截止时间超时
4. **在生产环境启用 TLS**：生产环境始终使用 TLS
5. **使用负载均衡**：在多个服务实例之间分配请求
6. **实现健康检查**：添加健康端点用于服务监控
7. **监控指标**：收集和监控 gRPC 指标（请求延迟、错误率）
8. **对大数据使用流式**：对大型数据集使用服务器/客户端流式

<div align="center

## Python 集成

</div>

### Python 服务注册

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# 创建服务注册表
registry = DMSCGrpcServiceRegistryPy()

# 注册 Python 处理器
def user_handler(method: str, data: bytes) -> bytes:
    import json
    if method == "GetUser":
        return json.dumps({"id": 1, "name": "Python User"}).encode()
    return json.dumps({"error": "未知方法"}).encode()

def order_handler(method: str, data: bytes) -> bytes:
    import json
    if method == "GetOrder":
        return json.dumps({"id": 100, "total": 99.99}).encode()
    return json.dumps({"error": "未知方法"}).encode()

registry.register("user-service", user_handler)
registry.register("order-service", order_handler)

# 列出所有服务
print(f"服务列表: {registry.list_services()}")
```

### Python 客户端使用

```python
import grpc
import json

def call_grpc_service(host, port, service, method, data):
    channel = grpc.insecure_channel(f"{host}:{port}")
    stub = eval(f"{service}Stub")(channel)
    
    request = json.dumps(data).encode()
    response = getattr(stub, method)(request)
    
    return json.loads(response)

# 调用服务
user = call_grpc_service("localhost", 50051, "UserService", "GetUser", {"id": 1})
order = call_grpc_service("localhost", 50051, "OrderService", "GetOrder", {"id": 100})
```

<div align="center

## 总结

</div>

本示例演示了如何使用 DMSC gRPC 模块：

- 创建带服务注册的 gRPC 服务器
- 使用 PyO3 绑定实现基于 Python 的处理器
- 服务发现和注册
- 客户端负载均衡
- 请求/响应拦截器
- 健康检查端点
- TLS 加密
- 流式支持

通过本示例，您应该掌握了 DMSC gRPC 模块的核心功能和使用方法。您可以在此基础上构建更复杂的基于 gRPC 的微服务。

<div align="center

## 相关模块

</div>

- [README](./README.md)：使用示例概述，提供所有使用示例的快速导航
- [authentication](./authentication.md)：认证示例，学习 JWT、OAuth2 和 RBAC 认证授权
- [basic-app](./basic-app.md)：基础应用程序示例，学习如何创建和运行您的第一个 DMSC 应用程序
- [database](./database.md)：数据库示例，学习数据库连接和查询操作
- [http](./http.md)：HTTP 服务示例，构建 Web 应用程序和 RESTful API
- [websocket](./websocket.md)：WebSocket 示例，实现实时双向通信
- [mq](./mq.md)：消息队列示例，实现异步消息处理
- [protocol](./protocol.md)：协议示例，实现自定义通信协议
- [service_mesh](./service_mesh.md)：服务网格示例，实现服务间通信
