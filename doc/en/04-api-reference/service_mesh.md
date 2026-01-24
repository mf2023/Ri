<div align="center">

# ServiceMesh API Reference

**Version: 0.1.6**

**Last modified date: 2026-01-18**

The service_mesh module provides service mesh functionality, including service discovery, health checking, traffic management, and load balancing.

## Module Overview

</div>

The service_mesh module contains the following sub-modules:

- **service_discovery**: Service discovery
- **health_check**: Health checking
- **traffic_management**: Traffic management

<div align="center">

## Core Components

</div>

### DMSCServiceMesh

The main service mesh interface.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `new(config)` | Create service mesh instance | `config: DMSCServiceMeshConfig` | `DMSCResult<Self>` |
| `register_service(service_name, endpoint, weight, metadata)` | Register service with optional metadata | `service_name: &str`, `endpoint: &str`, `weight: u32`, `metadata: Option<HashMap<String, String>>` | `DMSCResult<()>` |
| `register_versioned_service(service_name, version, endpoint, weight, metadata)` | Register versioned service | `service_name: &str`, `version: &str`, `endpoint: &str`, `weight: u32`, `metadata: Option<HashMap<String, String>>` | `DMSCResult<()>` |
| `unregister_service(service_name, endpoint)` | Unregister service | `service_name: &str`, `endpoint: &str` | `DMSCResult<()>` |
| `discover_service(service_name)` | Discover healthy service endpoints | `service_name: &str` | `DMSCResult<Vec<DMSCServiceEndpoint>>` |
| `get_all_endpoints(service_name)` | Get all endpoints regardless of health | `service_name: &str` | `DMSCResult<Vec<DMSCServiceEndpoint>>` |
| `call_service(service_name, request_data)` | Call service | `service_name: &str`, `request_data: Vec<u8>` | `DMSCResult<Vec<u8>>` |
| `update_service_health(service_name, endpoint, is_healthy)` | Update service health | `service_name: &str`, `endpoint: &str`, `is_healthy: bool` | `DMSCResult<()>` |
| `get_stats()` | Get service mesh statistics | None | `DMSCResult<ServiceMeshStats>` |
| `get_circuit_breaker()` | Get circuit breaker | None | `&DMSCCircuitBreaker` |
| `get_load_balancer()` | Get load balancer | None | `&DMSCLoadBalancer` |
| `get_health_checker()` | Get health checker | None | `&DMSCHealthChecker` |
| `get_traffic_manager()` | Get traffic manager | None | `&DMSCTrafficManager` |
| `get_service_discovery()` | Get service discovery | None | `&DMSCServiceDiscovery` |

#### Usage Example

```rust
use dmsc::prelude::*;
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
use std::collections::HashMap;

async fn example() -> DMSCResult<()> {
    let mesh_config = DMSCServiceMeshConfig::default();
    
    let service_mesh = DMSCServiceMesh::new(mesh_config)?;
    
    // Register service with metadata
    let mut metadata = HashMap::new();
    metadata.insert("region".to_string(), "us-east-1".to_string());
    service_mesh.register_service("user-service", "http://user-service:8080", 100, Some(metadata)).await?;
    
    // Register versioned service
    service_mesh.register_versioned_service("api-service", "v2.0", "http://api-service-v2:8080", 100, None).await?;
    
    service_mesh.register_service("order-service", "http://order-service:8080", 100, None).await?;
    service_mesh.register_service("payment-service", "http://payment-service:8080", 100, None).await?;
    
    // Discover healthy endpoints
    let user_service_endpoints = service_mesh.discover_service("user-service").await?;
    println!("User service endpoints: {:?}", user_service_endpoints);
    
    // Get all endpoints
    let all_endpoints = service_mesh.get_all_endpoints("user-service").await?;
    
    // Get statistics
    let stats = service_mesh.get_stats().await?;
    println!("Total services: {}", stats.total_services);
    println!("Healthy endpoints: {}", stats.healthy_endpoints);
    
    let request_data = r#"{ "user_id": "123" }"#.as_bytes().to_vec();
    let response = service_mesh.call_service("user-service", request_data).await?;
    println!("Service response: {}", String::from_utf8_lossy(&response));
    
    // Unregister service
    service_mesh.unregister_service("user-service", "http://user-service:8080").await?;
    
    let health_checker = service_mesh.get_health_checker();
    let traffic_manager = service_mesh.get_traffic_manager();
    let circuit_breaker = service_mesh.get_circuit_breaker();
    let load_balancer = service_mesh.get_load_balancer();
    
    Ok(())
}
```

### DMSCServiceMeshStats

Service mesh statistics.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `total_services` | `usize` | Total registered services |
| `total_endpoints` | `usize` | Total registered endpoints |
| `healthy_endpoints` | `usize` | Number of healthy endpoints |
| `unhealthy_endpoints` | `usize` | Number of unhealthy endpoints |

### DMSCServiceMeshConfig

Service mesh configuration.

| Field | Type | Description | Default |
|:--------|:-----|:-------------|:-------|
| `enable_service_discovery` | `bool` | Enable service discovery | `true` |
| `enable_health_check` | `bool` | Enable health checking | `true` |
| `enable_traffic_management` | `bool` | Enable traffic management | `true` |
| `health_check_interval` | `Duration` | Health check interval | `30s` |
| `circuit_breaker_config` | `DMSCCircuitBreakerConfig` | Circuit breaker config | Default |
| `load_balancer_strategy` | `DMSCLoadBalancerStrategy` | Load balancing strategy | `RoundRobin` |
| `max_retry_attempts` | `u32` | Max retry attempts | `3` |
| `retry_timeout` | `Duration` | Retry timeout | `5s` |

<div align="center">

## Service Discovery

</div>

### DMSCServiceDiscovery

Service discovery component.

```rust
use dmsc::service_mesh::DMSCServiceDiscovery;

let discovery = DMSCServiceDiscovery::new(true);

discovery.start_background_tasks().await?;

let endpoints = discovery.discover("user-service").await?;

discovery.stop_background_tasks().await?;
```

### DMSCServiceEndpoint

Service endpoint.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `service_name` | `String` | Service name |
| `endpoint` | `String` | Endpoint URL |
| `weight` | `u32` | Load balancing weight |
| `metadata` | `HashMap<String, String>` | Metadata |
| `health_status` | `DMSCServiceHealthStatus` | Health status |
| `last_health_check` | `SystemTime` | Last health check time |

### DMSCServiceHealthStatus

Service health status.

| Variant | Description |
|:--------|:-------------|
| `Healthy` | Healthy |
| `Unhealthy` | Unhealthy |
| `Unknown` | Unknown |

<div align="center">

## Health Checking

</div>

### DMSCHealthChecker

Health checker.

```rust
use dmsc::service_mesh::{DMSCHealthChecker, DMSCHealthStatus};

let health_checker = DMSCHealthChecker::new(Duration::from_secs(30));

health_checker.start_health_check("user-service", "http://user-service:8080/health").await?;

// Stop health check for a specific service
health_checker.stop_health_check("user-service", "http://user-service:8080/health").await?;

let summary = health_checker.get_health_summary().await?;
println!("Healthy services: {}", summary.healthy_count);
println!("Unhealthy services: {}", summary.unhealthy_count);

health_checker.stop_background_tasks().await?;
```

### DMSCHealthCheckResult

Health check result.

```rust
let result = health_checker.check_health("http://user-service:8080").await?;

match result.status {
    DMSCHealthStatus::Healthy => println!("Service is healthy"),
    DMSCHealthStatus::Unhealthy => println!("Service is unhealthy: {:?}", result.error),
    DMSCHealthStatus::Unknown => println!("Service health unknown"),
}
```

### DMSCHealthSummary

Health check summary.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `healthy_count` | `usize` | Healthy services count |
| `unhealthy_count` | `usize` | Unhealthy services count |
| `unknown_count` | `usize` | Unknown services count |
| `total_services` | `usize` | Total services count |

<div align="center">

## Traffic Management

</div>

### DMSCTrafficManager

Traffic manager.

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

Traffic route.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `name` | `String` | Route name |
| `match_criteria` | `DMSCMatchCriteria` | Match criteria |
| `action` | `DMSCRouteAction` | Route action |
| `timeout` | `Duration` | Timeout |
| `retry_count` | `u32` | Retry count |

### DMSCMatchCriteria

Match criteria.

| Field | Type | Description |
|:--------|:-----|:-------------|
| `path_prefix` | `Option<String>` | Path prefix |
| `headers` | `HashMap<String, String>` | Request headers |
| `methods` | `Vec<String>` | HTTP methods |

### DMSCRouteAction

Route action.

| Variant | Description |
|:--------|:-------------|
| `RouteTo { service_name, weight }` | Route to service |
| `Redirect { url }` | Redirect |
| `Rewrite { path }` | Path rewrite |
| `CircuitBreak` | Circuit break |

<div align="center">

## Circuit Breaking

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

<div align="center">

## Load Balancing

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

<div align="center">

## Best Practices

</div>

1. **Enable health checking**: Regularly check service health status and automatically remove unhealthy instances
2. **Configure appropriate timeouts**: Set appropriate timeouts based on service response time
3. **Use retry mechanism**: Automatically retry transient failures
4. **Enable circuit breaker protection**: Prevent cascading failures
5. **Configure load balancing**: Properly distribute requests to different instances
6. **Monitor service status**: Regularly check service mesh status

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
- [gateway](./gateway.md): Gateway module providing API gateway functionality
- [grpc](./grpc.md): gRPC module with service registry and Python bindings
- [hooks](./hooks.md): Hooks module providing lifecycle hook support
- [http](./http.md): HTTP module providing HTTP server and client functionality
- [log](./log.md): Logging module for protocol events
- [mq](./mq.md): Message queue module providing message queue support
- [observability](./observability.md): Observability module for protocol performance monitoring
- [orm](./orm.md): ORM module with query builder and pagination support
- [protocol](./protocol.md): Protocol module providing communication protocol support
- [security](./security.md): Security module providing encryption and decryption functions
- [storage](./storage.md): Storage module providing cloud storage support
- [validation](./validation.md): Validation module providing data validation functions
- [ws](./ws.md): WebSocket module with Python bindings for real-time communication
