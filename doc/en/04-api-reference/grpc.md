<div align="center">

# gRPC API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-16**

The gRPC module provides high-performance RPC (Remote Procedure Call) functionality with service registration, discovery, and load balancing. Supports both server and client modes with Python bindings.

## Module Overview

</div>

The gRPC module includes the following sub-modules:

- **server**: gRPC server implementation and service registration
- **client**: gRPC client implementation with load balancing
- **discovery**: Service discovery and registration
- **load_balancer**: Client-side load balancing
- **interceptor**: Request/response interceptors

<div align="center">

## Core Components

</div>

### DMSCGrpcServiceRegistry

Service registry for managing gRPC services with Python bindings support.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `register(name, handler)` | Register a new service | `name: &str`, `handler: PyCallable` | `DMSCResult<()>` |
| `unregister(name)` | Unregister a service | `name: &str` | `DMSCResult<()>` |
| `get(name)` | Get service handler | `name: &str` | `DMSCResult<Option<PyObject>>` |
| `list_services()` | List all registered services | None | `DMSCResult<Vec<String>>` |
| `register_with_config(name, handler, config)` | Register with config | `name: &str`, `handler: PyCallable`, `config: PyDict` | `DMSCResult<()>` |

#### Python Usage Example

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

registry = DMSCGrpcServiceRegistryPy()

def my_handler(method: str, data: bytes) -> bytes:
    print(f"Received request: {method}")
    return b"Response from Python handler"

# Register a service
registry.register("my-service", my_handler)

# List all services
services = registry.list_services()
print(f"Registered services: {services}")

# Get specific service
service = registry.get("my-service")
```

### DMSCGrpcConfig

gRPC server configuration struct.

#### Fields

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `host` | `String` | Server host | `"0.0.0.0"` |
| `port` | `u16` | Server port | `50051` |
| `max_concurrent_rpcs` | `u32` | Maximum concurrent RPCs | `100` |
| `max_receive_message_size` | `u32` | Max receive message size (bytes) | `4194304` |
| `max_send_message_size` | `u32` | Max send message size (bytes) | `4194304` |
| `keepalive_time` | `Duration` | Keep-alive time | `30000ms` |
| `keepalive_timeout` | `Duration` | Keep-alive timeout | `5000ms` |
| `tls_enabled` | `bool` | Enable TLS | `false` |
| `tls_cert_path` | `Option<String>` | TLS certificate path | `None` |
| `tls_key_path` | `Option<String>` | TLS key path | `None` |

#### Configuration Example

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

gRPC server main interface.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `start()` | Start the gRPC server | None | `DMSCResult<()>` |
| `stop()` | Stop the gRPC server | None | `DMSCResult<()>` |
| `add_service<S>(&mut self, service: S)` | Add a service | `service: S` | `DMSCResult<()>` |
| `is_running(&self)` | Check if server is running | None | `bool` |

#### Usage Example

```rust
use dmsc::prelude::*;

let config = DMSCGrpcConfig::default();
let mut server = DMSCGrpcServer::new(config)?;

server.start()?;
println!("gRPC server started on {}", config.port);

// Server runs until stopped
server.stop()?;
println!("gRPC server stopped");
```

<div align="center">

## Service Discovery

</div>

### DMSCServiceDiscovery

Service discovery interface for dynamic service registration and lookup.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `register(service)` | Register a service | `service: DMSCServiceInfo` | `DMSCResult<()>` |
| `deregister(service_id)` | Deregister a service | `service_id: &str` | `DMSCResult<()>` |
| `lookup(service_name)` | Lookup services by name | `service_name: &str` | `DMSCResult<Vec<DMSCServiceInfo>>` |
| `get_all()` | Get all registered services | None | `DMSCResult<Vec<DMSCServiceInfo>>` |
| `watch()` | Watch for service changes | None | `DMSCResult<DMSCServiceWatcher>` |

#### Usage Example

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
println!("Found {} user services", services.len());
```

### DMSCServiceInfo

Service information struct for registration.

#### Fields

| Field | Type | Description |
|:--------|:-----|:-------------|
| `id` | `String` | Unique service identifier |
| `name` | `String` | Service name |
| `host` | `String` | Service host |
| `port` | `u16` | Service port |
| `metadata` | `HashMap<String, String>` | Additional metadata |

<div align="center">

## Load Balancing

</div>

### DMSCLoadBalancer

Client-side load balancer for distributing requests across multiple service instances.

#### Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `new(strategy)` | Create load balancer | `strategy: LoadBalancingStrategy` | `Self` |
| `add_endpoint(&mut self, endpoint)` | Add endpoint | `endpoint: DMSCLoadBalancerEndpoint` | `()` |
| `remove_endpoint(&mut self, id)` | Remove endpoint | `id: &str` | `()` |
| `select(&self)` | Select an endpoint | None | `Option<DMSCLoadBalancerEndpoint>` |
| `get_all(&self)` | Get all endpoints | None | `Vec<DMSCLoadBalancerEndpoint>` |

#### Load Balancing Strategies

| Strategy | Description |
|:--------|:-------------|
| `RoundRobin` | Distribute requests evenly across all endpoints |
| `LeastConnections` | Select endpoint with fewest active connections |
| `Random` | Randomly select an endpoint |
| `Weighted` | Select based on endpoint weights |

#### Usage Example

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
    println!("Selected endpoint: {}:{}", endpoint.host, endpoint.port);
}
```

<div align="center">

## Interceptors

</div>

### DMSCGrpcInterceptor

Interceptor trait for adding custom logic to request processing.

#### Trait Methods

| Method | Description | Parameters | Return Value |
|:--------|:-------------|:--------|:--------|
| `intercept(&self, request, next)` | Intercept request | `request: DMSCGrpcRequest`, `next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>` | `DMSCResult<DMSCGrpcResponse>` |

#### Built-in Interceptors

| Interceptor | Description |
|:--------|:-------------|
| `LoggingInterceptor` | Logs all incoming requests and responses |
| `AuthInterceptor` | Validates authentication tokens |
| `RateLimitInterceptor` | Limits request rate per client |
| `MetricsInterceptor` | Collects request metrics |
| `TracingInterceptor` | Adds distributed tracing headers |

#### Usage Example

```rust
use dmsc::prelude::*;

struct LoggingInterceptor;

impl DMSCGrpcInterceptor for LoggingInterceptor {
    fn intercept(
        &self,
        request: DMSCGrpcRequest,
        next: impl FnOnce(DMSCGrpcRequest) -> DMSCResult<DMSCGrpcResponse>,
    ) -> DMSCResult<DMSCGrpcResponse> {
        println!("Received request: {} {}", request.method, request.path);
        let start = std::time::Instant::now();
        let response = next(request)?;
        println!("Response completed in {:?}", start.elapsed());
        Ok(response)
    }
}

// Add interceptor to server
server.add_interceptor(LoggingInterceptor)?;
```

<div align="center>

## Advanced Features

</div>

### Streaming Support

```rust
use dmsc::prelude::*;

// Server-side streaming
async fn stream_users(request: DMSCGrpcRequest) -> DMSCResult<DMSCGrpcStreamingResponse<User>> {
    let users = vec![
        User { id: 1, name: "Alice".to_string() },
        User { id: 2, name: "Bob".to_string() },
    ];
    
    let stream = futures::stream::iter(users);
    Ok(DMSCGrpcStreamingResponse::new(stream))
}

// Client-side streaming
let request = DMSCGrpcStreamingRequest::new(
    std::iter::repeat(User { id: 0, name: "Test".to_string() }).take(100)
);
let response = client.bidirectional_stream(request).await?;
```

### Health Checks

```rust
use dmsc::prelude::*;

let health_service = DMSCGrpcHealthService::new();

// Register health check handler
health_service.register_check("service-name", move || {
    DMSCHealthCheckResult::Healthy
});

// Start health service
server.add_service(health_service)?;
```

### Error Handling

```rust
use dmsc::prelude::*;

match client.call_method("user-service", "GetUser", request).await {
    Ok(response) => {
        println!("Response: {:?}", response);
    }
    Err(DMSCGrpcError::ServiceNotFound(name)) => {
        println!("Service {} not found", name);
    }
    Err(DMSCGrpcError::MethodNotFound(method)) => {
        println!("Method {} not found", method);
    }
    Err(DMSCGrpcError::DeadlineExceeded) => {
        println!("Request timed out");
    }
    Err(e) => {
        println!("gRPC error: {}", e);
    }
}
```

<div align="center">

## Best Practices

</div>

1. **Use service discovery**: Enable dynamic service registration for scalable deployments
2. **Configure timeouts**: Set appropriate request and deadline timeouts
3. **Enable load balancing**: Use client-side load balancing for high availability
4. **Use interceptors**: Add logging, authentication, and metrics interceptors
5. **Monitor health**: Implement health checks for service reliability
6. **Handle errors gracefully**: Implement proper error handling and retry logic
7. **Use streaming for large data**: Use server/client streaming for large datasets
8. **Enable TLS in production**: Always use TLS for production deployments

<div align="center>

## Python Support

</div>

The gRPC module provides full Python bindings through PyO3:

```python
from dmsc.grpc import DMSCGrpcServiceRegistryPy, DMSCGrpcConfig

# Create service registry
registry = DMSCGrpcServiceRegistryPy()

# Register Python handler
def user_handler(method: str, data: bytes) -> bytes:
    if method == "GetUser":
        return b'{"id": 1, "name": "Alice"}'
    return b'{"error": "unknown method"}'

registry.register("user-service", user_handler)

# List services
print(registry.list_services())
```

<div align="center>

## Related Modules

</div>

- [README](./README.md): Module overview with API reference summary and quick navigation
- [auth](./auth.md): Authentication module handling user authentication and authorization
- [cache](./cache.md): Cache module providing in-memory and distributed cache support
- [config](./config.md): Configuration module managing application configuration
- [core](./core.md): Core module providing error handling and service context
- [database](./database.md): Database module providing database access layer
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [service_mesh](./service_mesh.md): Service mesh module using protocols for inter-service communication
