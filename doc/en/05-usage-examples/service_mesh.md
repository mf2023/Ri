# Service Mesh Usage Guide

This document provides comprehensive usage examples for the DMSC Service Mesh, demonstrating how to leverage service discovery, health checking, traffic management, load balancing, and circuit breaker capabilities for distributed systems.

## Table of Contents

1. [Service Mesh Basics](#service-mesh-basics)
2. [Service Registration](#service-registration)
3. [Service Discovery](#service-discovery)
4. [Health Checking](#health-checking)
5. [Traffic Management](#traffic-management)
6. [Circuit Breaker](#circuit-breaker)
7. [Load Balancing](#load-balancing)
8. [Complete Example](#complete-example)

---

## Service Mesh Basics

The service mesh is a core component of DMSC, providing a complete solution for inter-service communication. It integrates service discovery, health checking, traffic management, load balancing, and circuit breaker functionality.

### Creating a Service Mesh

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
use dmsc::prelude::*;

async fn create_service_mesh() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    Ok(())
}
```

### Custom Configuration

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

### Default Configuration

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

## Service Registration

Service registration is the process of adding service instances to the service mesh, enabling other services to discover and invoke them.

### Basic Service Registration

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn register_basic_service() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Register user service
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    
    // Register order service
    mesh.register_service("order-service", "http://order-service:8080", 100).await?;
    
    // Register payment service
    mesh.register_service("payment-service", "http://payment-service:8080", 100).await?;
    
    Ok(())
}
```

### Weighted Service Registration

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn register_weighted_services() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Register primary service instance (weight 100)
    mesh.register_service(
        "api-service",
        "http://api-primary:8080",
        100,
    ).await?;
    
    // Register backup service instance (weight 50)
    mesh.register_service(
        "api-service",
        "http://api-backup:8080",
        50,
    ).await?;
    
    // Register development environment instance (weight 10)
    mesh.register_service(
        "api-service",
        "http://api-dev:8080",
        10,
    ).await?;
    
    Ok(())
}
```

### Batch Registration

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
    
    println!("Registered {} services", services.len());
    
    Ok(())
}
```

---

## Service Discovery

Service discovery allows the service mesh to automatically find available service instances and return a list of healthy instances.

### Discovering Service Instances

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn discover_services() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // First register some services
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    mesh.register_service("user-service", "http://user-service-backup:8080", 50).await?;
    
    // Discover user service
    let endpoints = mesh.discover_service("user-service").await?;
    
    println!("Found {} user service instances", endpoints.len());
    for ep in &endpoints {
        println!("  - {} (weight: {})", ep.endpoint, ep.weight);
    }
    
    Ok(())
}
```

### Handling Unavailable Services

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn handle_service_not_found() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Try to discover a non-existent service
    match mesh.discover_service("non-existent-service").await {
        Ok(endpoints) => {
            println!("Found service instances: {:?}", endpoints);
        }
        Err(e) => {
            println!("Service discovery failed: {}", e);
        }
    }
    
    Ok(())
}
```

### Discovering Healthy Instances

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceHealthStatus};
use dmsc::prelude::*;

async fn discover_healthy_instances() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Register multiple service instances
    mesh.register_service("api-service", "http://api-1:8080", 100).await?;
    mesh.register_service("api-service", "http://api-2:8080", 100).await?;
    mesh.register_service("api-service", "http://api-3:8080", 100).await?;
    
    // Mark some instances as unhealthy
    mesh.update_service_health("api-service", "http://api-2:8080", false).await?;
    
    // Discovery will only return healthy instances
    let endpoints = mesh.discover_service("api-service").await?;
    
    println!("Found {} healthy instances", endpoints.len());
    for ep in &endpoints {
        println!("  - {} (status: {:?})", ep.endpoint, ep.health_status);
    }
    
    Ok(())
}
```

---

## Health Checking

Health checking continuously monitors the status of service instances, ensuring only healthy instances receive traffic.

### Manual Health Status Update

```rust
use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceHealthStatus};
use dmsc::prelude::*;

async fn manual_health_update() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Register service
    mesh.register_service("database-service", "http://db:5432", 100).await?;
    
    // Simulate health check results
    mesh.update_service_health("database-service", "http://db:5432", true).await?;
    
    // Service becomes unhealthy
    mesh.update_service_health("database-service", "http://db:5432", false).await?;
    
    // Service recovers health
    mesh.update_service_health("database-service", "http://db:5432", true).await?;
    
    Ok(())
}
```

### Health Status Reference

| Status | Description |
|--------|-------------|
| `Healthy` | Service is healthy and available |
| `Unhealthy` | Service is unhealthy and unavailable |
| `Unknown` | Service health status is unknown |

---

## Traffic Management

Traffic management provides intelligent routing and request forwarding, supporting traffic splitting, timeout control, and retry mechanisms.

### Calling a Service

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn call_service() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Register service
    mesh.register_service("user-service", "http://user-service:8080", 100).await?;
    
    // Call user service
    let request_data = r#"{"user_id": "123"}"#.as_bytes().to_vec();
    let response = mesh.call_service("user-service", request_data).await?;
    
    println!("Response: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

### Calling with Retry

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
        max_retry_attempts: 5, // Retry up to 5 times
        retry_timeout: std::time::Duration::from_secs(10),
    };
    
    let mesh = DMSCServiceMesh::new(config)?;
    mesh.register_service("unreliable-service", "http://unreliable:8080", 100).await?;
    
    let request = b"Test request".to_vec();
    
    // Automatically retry failed requests
    let response = mesh.call_service("unreliable-service", request).await?;
    
    println!("Final response: {}", String::from_utf8_lossy(&response));
    
    Ok(())
}
```

---

## Circuit Breaker

The circuit breaker is a critical component that protects the system from cascading failures. When the service failure rate exceeds the threshold, the circuit breaker opens, failing fast and preventing failure propagation.

### Circuit Breaker States

| State | Description |
|-------|-------------|
| `Closed` | Normal operation, requests are allowed |
| `Open` | Circuit is open, all requests are rejected |
| `HalfOpen` | Half-open state, allows some requests to test service recovery |

### Accessing the Circuit Breaker

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn access_circuit_breaker() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Get circuit breaker reference
    let circuit_breaker = mesh.get_circuit_breaker();
    
    // Check circuit breaker state
    let state = circuit_breaker.get_state().await;
    println!("Circuit breaker state: {:?}", state);
    
    Ok(())
}
```

### Circuit Breaker Configuration

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
            failure_threshold: 5, // Open after 5 failures
            success_threshold: 2, // Close after 2 successes
            timeout_duration: std::time::Duration::from_secs(30), // Try half-open after 30 seconds
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

## Load Balancing

Load balancing distributes requests across multiple service instances, ensuring efficient resource utilization and high availability.

### Load Balancing Strategies

| Strategy | Description |
|----------|-------------|
| `RoundRobin` | Round-robin, each request goes to the next instance |
| `Random` | Randomly select an instance |
| `LeastConnections` | Select the instance with fewest connections |
| `WeightedRoundRobin` | Weighted round-robin based on instance weights |
| `IPHash` | Hash based on client IP address |

### Accessing the Load Balancer

```rust
use dmsc::service_mesh::DMSCServiceMesh;
use dmsc::prelude::*;

async fn access_load_balancer() -> DMSCResult<()> {
    let mesh = DMSCServiceMesh::new(DMSCServiceMeshConfig::default())?;
    
    // Get load balancer reference
    let load_balancer = mesh.get_load_balancer();
    
    // Get list of healthy servers
    let servers = load_balancer.get_healthy_servers().await;
    
    println!("Healthy servers: {}", servers.len());
    
    Ok(())
}
```

### Custom Load Balancing Strategy

```rust
use dmsc::service_mesh::DMSCServiceMeshConfig;
use dmsc::gateway::DMSCLoadBalancerStrategy;

async fn custom_load_balancing() -> DMSCResult<()> {
    // Use least connections strategy
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
    
    // Register multiple service instances
    mesh.register_service("api-service", "http://api-1:8080", 100).await?;
    mesh.register_service("api-service", "http://api-2:8080", 100).await?;
    mesh.register_service("api-service", "http://api-3:8080", 100).await?;
    
    Ok(())
}
```

---

## Complete Example

The following example demonstrates a complete integration of the service mesh, including coordinated use of all core features:

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
        println!("Initializing services...");
        
        // Core services
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
        
        // External services
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
        
        // High availability services (multiple instances)
        self.mesh.register_service("api-gateway", "http://api-gateway-1:8080", 100).await?;
        self.mesh.register_service("api-gateway", "http://api-gateway-2:8080", 100).await?;
        
        println!("Service initialization complete");
        
        Ok(())
    }
    
    async fn process_user_order(&self, user_id: &str, order_data: &[u8]) -> DMSCResult<Vec<u8>> {
        println!("Processing order for user {}...", user_id);
        
        // Get user information
        let user_request = format!(r#"{{"user_id": "{}"}}"#, user_id).as_bytes().to_vec();
        let user_response = self.mesh.call_service("user-service", user_request).await?;
        println!("User information retrieved successfully");
        
        // Create order
        let order_response = self.mesh.call_service("order-service", order_data.to_vec()).await?;
        println!("Order created successfully");
        
        // Process payment
        let payment_request = order_response.clone();
        let payment_response = self.mesh.call_service("payment-service", payment_request).await?;
        println!("Payment processed successfully");
        
        // Send notification
        let notification = format!(r#"{{"user_id": "{}", "message": "Order processed"}}"#, user_id);
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
        
        println!("=== Service Health Status ===");
        for service in services {
            match self.mesh.discover_service(service).await {
                Ok(endpoints) => {
                    println!("{}: {} healthy instances", service, endpoints.len());
                    for ep in endpoints {
                        println!("  - {} (weight: {})", ep.endpoint, ep.weight);
                    }
                }
                Err(e) => {
                    println!("{}: Service unavailable - {}", service, e);
                }
            }
        }
        
        Ok(())
    }
    
    async fn get_mesh_components(&self) {
        println!("=== Service Mesh Components ===");
        
        // Get components
        let _ = self.mesh.get_circuit_breaker();
        let _ = self.mesh.get_load_balancer();
        let _ = self.mesh.get_health_checker();
        let _ = self.mesh.get_traffic_manager();
        let _ = self.mesh.get_service_discovery();
        
        println!("All components ready");
    }
}

#[tokio::main]
async fn main() -> DMSCResult<()> {
    println!("Starting microservices platform...");
    
    let platform = MicroservicesPlatform::new().await?;
    
    platform.initialize_services().await?;
    
    platform.get_mesh_components().await;
    
    platform.get_service_health().await?;
    
    println!("\nProcessing sample order...");
    let order_data = r#"{"items": [{"id": 1, "quantity": 2}], "total": 99.99}"#;
    let _result = platform.process_user_order("user-123", order_data.as_bytes()).await;
    
    println!("\nDemonstration complete");
    
    Ok(())
}
```

### Expected Output

```
Starting microservices platform...
Initializing services...
Service initialization complete
=== Service Mesh Components ===
All components ready
=== Service Health Status ===
user-service: 1 healthy instances
  - http://user-service:8080 (weight: 100)
order-service: 1 healthy instances
  - http://order-service:8080 (weight: 100)
payment-service: 1 healthy instances
  - http://payment-service:8080 (weight: 100)
notification-service: 1 healthy instances
  - http://notification-service:8080 (weight: 80)
analytics-service: 1 healthy instances
  - http://analytics-service:8080 (weight: 60)
api-gateway: 2 healthy instances
  - http://api-gateway-1:8080 (weight: 100)
  - http://api-gateway-2:8080 (weight: 100)

Processing sample order...
Processing order for user user-123...
User information retrieved successfully
Order created successfully
Payment processed successfully

Demonstration complete
```

<div align="center">

## Related Modules

</div>

- [README](./README.md): Module overview with usage examples summary and quick navigation
- [authentication](./authentication.md): Authentication examples, including JWT, OAuth2, and MFA
- [basic-app](./basic-app.md): Basic application examples
- [caching](./caching.md): Caching examples, including memory and distributed caching
- [database](./database.md): Database operation examples
- [device](./device.md): Device control examples
- [fs](./fs.md): Filesystem operation examples
- [gateway](./gateway.md): API gateway examples
- [hooks](./hooks.md): Hook system examples
- [http](./http.md): HTTP server and client examples
- [grpc](./grpc.md): gRPC examples, implement high-performance RPC calls
- [websocket](./websocket.md): WebSocket examples, implement real-time bidirectional communication
- [mq](./mq.md): Message queue examples
- [observability](./observability.md): Observability examples
- [protocol](./protocol.md): Protocol module examples
- [security](./security.md): Security and encryption examples
- [storage](./storage.md): Cloud storage examples
- [validation](./validation.md): Data validation examples
