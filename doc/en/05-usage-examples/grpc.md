<div align="center">

# gRPC Usage Examples

**Version: 0.1.4**

**Last modified date: 2026-01-16**

This example demonstrates how to use DMSC's gRPC module for service registration, discovery, load balancing, and RPC calls with Python bindings support.

## Example Overview

</div>

This example will create a DMSC application that implements the following features:

- gRPC server with service registration
- Python handler support for dynamic service registration
- Service discovery and registration
- Client-side load balancing
- Request interceptors for logging and authentication
- Health check endpoints

<div align="center">

## Prerequisites

</div>

- Rust 1.65+
- Cargo 1.65+
- Basic Rust programming knowledge
- Understanding of gRPC concepts
- Python 3.8+ (for Python bindings examples)

<div align="center">

## Example Code

</div>

### 1. Create Project

```bash
cargo new dms-grpc-example
cd dms-grpc-example
```

### 2. Add Dependencies

Add the following dependencies to the `Cargo.toml` file:

```toml
[dependencies]
dmsc = { git = "https://github.com/mf2023/DMSC" }
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
pyo3 = { version = "0.20", features = ["extension-module"] }
```

### 3. Create Configuration File

Create a `config.yaml` file in the project root directory:

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

### 4. Write Main Code

Replace the `src/main.rs` file with the following content:

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
        ctx.logger().info("service", "DMSC gRPC Example started")?;
        
        let server = ctx.grpc_server();
        
        let user_service = UserService::new();
        server.add_service(user_service)?;
        
        ctx.logger().info("grpc", "Added user service")?;
        
        server.start()?;
        ctx.logger().info("grpc", &format!("gRPC server started on port {}", 50051))?;
        
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
                    "message": "User created successfully",
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
            _ => Err(DMSCError::not_found(format!("Unknown method: {}", method)))
        }
    }
}
```

### 5. Python Handler Example

Create a `python_handler.py` file for Python-based gRPC handlers:

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
            "message": "User created from Python handler",
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
        print(f"Registered Python services: {services}")
        
        return self.registry

if __name__ == "__main__":
    handler = UserPythonHandler()
    registry = handler.register_handlers()
    
    import time
    try:
        while True:
            time.sleep(1)
    except KeyboardInterrupt:
        print("Shutting down Python handler")
```

<div align="center">

## Code Analysis

</div>

### Service Registration

```rust
use dmsc::prelude::*;

// Create service instance
let user_service = UserService::new();

// Register with gRPC server
server.add_service(user_service)?;

println!("User service registered successfully");
```

### Python Handler Registration

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy

registry = DMSCGrpcServiceRegistryPy()

def my_handler(method: str, data: bytes) -> bytes:
    print(f"Received: {method}")
    return b"Response"

registry.register("my-service", my_handler)

# List all registered services
services = registry.list_services()
```

### Service Discovery

```rust
use dmsc::prelude::*;

// Create service discovery
let discovery = DMSCServiceDiscovery::new()?;

// Register service
let service = DMSCServiceInfo {
    id: "user-service-1".to_string(),
    name: "user-service".to_string(),
    host: "localhost".to_string(),
    port: 50051,
    metadata: HashMap::new(),
};

discovery.register(service)?;

// Lookup services
let services = discovery.lookup("user-service")?;
println!("Found {} services", services.len());
```

### Load Balancing

```rust
use dmsc::prelude::*;

// Create load balancer with round-robin strategy
let mut lb = DMSCLoadBalancer::new(LoadBalancingStrategy::RoundRobin);

// Add endpoints
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

// Select endpoint
if let Some(endpoint) = lb.select() {
    println!("Selected endpoint: {}:{}", endpoint.host, endpoint.port);
}
```

### Interceptors

```rust
use dmsc::prelude::*;

// Logging interceptor
struct LoggingInterceptor;

impl DMSCGrpcInterceptor for LoggingInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        println!("[{}] {} {}", request.method, request.path, request.timestamp);
        let response = next(request)?;
        println!("Response: {} - {:?}", response.status_code, response.duration);
        Ok(response)
    }
}

// Add interceptor
server.add_interceptor(LoggingInterceptor)?;
```

### Health Checks

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
        DMSCHealthCheckResult::Unhealthy("Database connection failed".to_string())
    }
});

server.add_service(health_service)?;
```

<div align="center>

## Running Steps

</div>

### 1. Build Project

```bash
cargo build --release
```

### 2. Run Project

```bash
cargo run
```

### 3. Test with Python Client

```python
import grpc
import json

# Create gRPC channel
channel = grpc.insecure_channel('localhost:50051')
stub = UserServiceStub(channel)

# Call GetUser
request = json.dumps({"id": 1}).encode()
response = stub.GetUser(request)
print(f"GetUser response: {response}")

# Call CreateUser
request = json.dumps({
    "name": "New User",
    "email": "newuser@example.com"
}).encode()
response = stub.CreateUser(request)
print(f"CreateUser response: {response}")
```

<div align="center>

## Expected Results

</div>

After running the example, you should see output similar to the following:

```json
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"service","message":"DMSC gRPC Example started","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"grpc","message":"Added user service","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:00Z","level":"info","module":"grpc","message":"gRPC server started on port 50051","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"grpc","message":"Received request: GetUser","trace_id":"abc123","span_id":"def456"}
{"timestamp":"2025-12-12T15:30:01Z","level":"info","module":"grpc","message":"Response: 200 OK (15ms)","trace_id":"abc123","span_id":"def456"}
```

<div align="center>

## Extended Features

</div>

### 1. TLS Encryption

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

### 2. Streaming Support

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

### 3. Authentication Interceptor

```rust
use dmsc::prelude::*;

struct AuthInterceptor;

impl DMSCGrpcInterceptor for AuthInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        // Extract auth token from metadata
        let auth_header = request.metadata.get("authorization");
        
        match auth_header {
            Some(token) if validate_token(token) => {
                next(request)
            }
            _ => Err(DMSCError::unauthorized("Invalid or missing token"))
        }
    }
}

fn validate_token(token: &str) -> bool {
    token.starts_with("Bearer ") && token.len() > 7
}

server.add_interceptor(AuthInterceptor)?;
```

### 4. Rate Limiting

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
            Err(DMSCError::too_many_requests("Rate limit exceeded"))
        }
    }
}

server.add_interceptor(RateLimitInterceptor::new(100, Duration::from_secs(60)))?;
```

<div align="center>

## Best Practices

</div>

1. **Use service discovery**: Enable dynamic service registration for scalable deployments
2. **Implement interceptors**: Add logging, authentication, and metrics for production use
3. **Configure timeouts**: Set appropriate request and deadline timeouts
4. **Enable TLS in production**: Always use TLS for production deployments
5. **Use load balancing**: Distribute requests across multiple service instances
6. **Implement health checks**: Add health endpoints for service monitoring
7. **Monitor metrics**: Collect and monitor gRPC metrics (request latency, error rates)
8. **Use streaming for large data**: Use server/client streaming for large datasets

<div align="center>

## Python Integration

</div>

### Python Service Registration

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# Create service registry
registry = DMSCGrpcServiceRegistryPy()

# Register Python handlers
def user_handler(method: str, data: bytes) -> bytes:
    import json
    if method == "GetUser":
        return json.dumps({"id": 1, "name": "Python User"}).encode()
    return json.dumps({"error": "unknown method"}).encode()

def order_handler(method: str, data: bytes) -> bytes:
    import json
    if method == "GetOrder":
        return json.dumps({"id": 100, "total": 99.99}).encode()
    return json.dumps({"error": "unknown method"}).encode()

registry.register("user-service", user_handler)
registry.register("order-service", order_handler)

# List all services
print(f"Services: {registry.list_services()}")
```

### Python Client Usage

```python
import grpc
import json

def call_grpc_service(host, port, service, method, data):
    channel = grpc.insecure_channel(f"{host}:{port}")
    stub = eval(f"{service}Stub")(channel)
    
    request = json.dumps(data).encode()
    response = getattr(stub, method)(request)
    
    return json.loads(response)

# Call services
user = call_grpc_service("localhost", 50051, "UserService", "GetUser", {"id": 1})
order = call_grpc_service("localhost", 50051, "OrderService", "GetOrder", {"id": 100})
```

<div align="center>

## Summary

</div>

This example demonstrates how to use the DMSC gRPC module for:

- Creating gRPC servers with service registration
- Implementing Python-based handlers with PyO3 bindings
- Service discovery and registration
- Client-side load balancing
- Request/response interceptors
- Health check endpoints
- TLS encryption
- Streaming support

Through this example, you should have mastered the core functions and usage methods of the DMSC gRPC module. You can build more complex gRPC-based microservices based on this foundation.

<div align="center>

## Related Modules

</div>

- [README](./README.md): Usage examples overview, providing quick navigation for all usage examples
- [authentication](./authentication.md): Authentication examples, learn JWT, OAuth2 and RBAC authentication authorization
- [basic-app](./basic-app.md): Basic application example, learn how to create and run your first DMSC application
- [database](./database.md): Database examples, learn database connection and query operations
- [http](./http.md): HTTP service examples, build Web applications and RESTful APIs
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples, implement asynchronous message processing
- [protocol](./protocol.md): Protocol examples, implement custom communication protocols
- [service_mesh](./service_mesh.md): Service mesh examples, implement inter-service communication
