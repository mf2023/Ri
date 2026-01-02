<div align="center">

# Service Mesh API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The service_mesh module provides service discovery, health checks, traffic management, load balancing, and circuit breaker capabilities for building distributed systems.

## Module Overview

</div>

The service_mesh module contains the following core components:

- **DMSCServiceMesh**: Service mesh main interface
- **DMSCServiceMeshConfig**: Service mesh configuration
- **DMSCServiceRegistry**: Service registry
- **DMSCServiceDiscovery**: Service discovery
- **DMSCHealthChecker**: Health checker
- **DMSCCircuitBreaker**: Circuit breaker
- **DMSCLoadBalancer**: Load balancer
- **DMSCTrafficManager**: Traffic manager

<div align="center">

## Core Components

</div>

### DMSCServiceMesh

Service mesh main interface.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `initialize(config)` | Initialize service mesh | `config: DMSCServiceMeshConfig` | `None` |
| `register_service(name, address, port, metadata)` | Register service | `name: str`, `address: str`, `port: int`, `metadata: dict` | `str` |
| `deregister_service(service_id)` | Deregister service | `service_id: str` | `bool` |
| `discover_services(service_name)` | Discover services | `service_name: str` | `List[ServiceInstance]` |
| `get_health_check_url(service_id)` | Get health check URL | `service_id: str` | `str` |
| `perform_health_check(service_id)` | Perform health check | `service_id: str` | `HealthCheckResult` |
| `get_statistics()` | Get mesh statistics | None | `ServiceMeshStats` |
| `shutdown()` | Shutdown service mesh | None | `None` |

#### Usage Example

```python
from dmsc import (
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCServiceRegistry
)

# Initialize service mesh
config = DMSCServiceMeshConfig(
    enable_service_discovery=True,
    enable_health_check=True,
    enable_traffic_management=True,
    health_check_interval=30,
    circuit_breaker_enabled=True
)
mesh = DMSCServiceMesh()
await mesh.initialize(config)

# Register services
service_id = mesh.register_service(
    name="user-service",
    address="user-service.internal",
    port=8080,
    metadata={"version": "1.0.0", "region": "us-east"}
)

# Discover services
services = await mesh.discover_services("user-service")
print(f"Found {len(services)} instances")

# Perform health check
result = await mesh.perform_health_check(service_id)
print(f"Health status: {result.status}")
```

### DMSCServiceMeshConfig

Service mesh configuration.

```python
from dmsc import (
    DMSCServiceMeshConfig,
    DMSCLoadBalancerStrategy,
    DMSCCircuitBreakerConfig
)

config = DMSCServiceMeshConfig(
    enable_service_discovery=True,
    enable_health_check=True,
    enable_traffic_management=True,
    health_check_interval=30,
    health_check_timeout=10,
    health_check_path="/health",
    circuit_breaker_config=DMSCCircuitBreakerConfig(
        failure_threshold=5,
        success_threshold=2,
        timeout_seconds=30
    ),
    load_balancer_strategy=DMSCLoadBalancerStrategy.ROUND_ROBIN,
    max_retry_attempts=3,
    retry_timeout=5,
    connection_timeout=10
)
```

## Service Registry

### Register Services

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Register a service
service_id = mesh.register_service(
    name="order-service",
    address="order-service.internal",
    port=8082,
    metadata={
        "version": "1.0.0",
        "environment": "production",
        "region": "us-west"
    }
)

# Register with custom health check
mesh.register_service(
    name="payment-service",
    address="payment-service.internal",
    port=8083,
    metadata={"version": "2.0.0"},
    health_check_url="http://payment-service:8083/healthz",
    health_check_interval=15,
    deregister_on_failure=True
)
```

### Service Discovery

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Discover all instances of a service
instances = await mesh.discover_services("user-service")

# Filter by metadata
filtered = await mesh.discover_services(
    "user-service",
    filters={
        "version": "1.0.0",
        "region": "us-east"
    }
)

# Get healthy instances only
healthy = await mesh.discover_services(
    "user-service",
    healthy_only=True
)

# Select specific instance
instance = await mesh.select_instance(
    "user-service",
    strategy="round_robin",
    exclude_instance_id="instance-123"
)
```

## Health Checks

### Built-in Health Checks

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Get service health status
health = await mesh.get_service_health("user-service")
print(f"Service health: {health.status}")
print(f"Healthy instances: {health.healthy_count}")
print(f"Unhealthy instances: {health.unhealthy_count}")

# Detailed health information
for instance_health in health.instances:
    print(f"Instance {instance_health.instance_id}: {instance_health.status}")
    print(f"  Last check: {instance_health.last_check_time}")
    print(f"  Response time: {instance_health.response_time_ms}ms")
```

### Custom Health Checks

```python
from dmsc import DMSCCustomHealthCheck

# Define custom health check
custom_check = DMSCCustomHealthCheck(
    name="database_connectivity",
    check_fn=async lambda: await check_database_connection(),
    timeout=10,
    interval=30
)

# Register custom health check
mesh.register_health_check(
    service_id="user-service",
    health_check=custom_check
)
```

## Circuit Breaker

### Configuration

```python
from dmsc import (
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCCircuitBreakerConfig, DMSCCircuitBreakerState
)

# Configure circuit breaker
config = DMSCServiceMeshConfig(
    circuit_breaker_config=DMSCCircuitBreakerConfig(
        failure_threshold=5,        # Open after 5 failures
        success_threshold=2,        # Close after 2 successes
        timeout_seconds=30,         # Try again after 30 seconds
        half_open_requests=3        # Allow 3 requests in half-open state
    )
)
mesh = DMSCServiceMesh()
await mesh.initialize(config)
```

### Circuit Breaker States

```python
from dmsc import DMSCCircuitBreakerState

# Circuit breaker states
closed = DMSCCircuitBreakerState.CLOSED     # Normal operation
open_ = DMSCCircuitBreakerState.OPEN         # Blocking requests
half_open = DMSCCircuitBreakerState.HALF_OPEN  # Testing recovery

# Get circuit breaker state
state = mesh.get_circuit_breaker_state("user-service")
print(f"Circuit breaker state: {state}")

# Manual control
mesh.open_circuit("user-service")
mesh.close_circuit("user-service")
```

## Load Balancing

### Load Balancing Strategies

```python
from dmsc import (
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCLoadBalancerStrategy
)

config = DMSCServiceMeshConfig(
    load_balancer_strategy=DMSCLoadBalancerStrategy.ROUND_ROBIN
)
mesh = DMSCServiceMesh()
await mesh.initialize(config)

# Available strategies
strategies = {
    "round_robin": DMSCLoadBalancerStrategy.ROUND_ROBIN,
    "least_connections": DMSCLoadBalancerStrategy.LEAST_CONNECTIONS,
    "weighted": DMSCLoadBalancerStrategy.WEIGHTED,
    "random": DMSCLoadBalancerStrategy.RANDOM,
    "ip_hash": DMSCLoadBalancerStrategy.IP_HASH,
    "consistent_hash": DMSCLoadBalancerStrategy.CONSISTENT_HASH
}
```

### Weighted Load Balancing

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Register instances with weights
mesh.register_service(
    name="api-service",
    address="api-v1.internal",
    port=8080,
    metadata={"version": "1.0.0"},
    weight=3  # 3x traffic
)

mesh.register_service(
    name="api-service",
    address="api-v2.internal",
    port=8080,
    metadata={"version": "2.0.0"},
    weight=1  # 1x traffic (canary)
)
```

## Traffic Management

### Traffic Splitting

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Configure traffic splitting (canary deployment)
await mesh.configure_traffic_split(
    service="api-service",
    splits=[
        {"version": "1.0.0", "weight": 90},
        {"version": "2.0.0", "weight": 10}
    ]
)

# Update traffic split
await mesh.configure_traffic_split(
    service="api-service",
    splits=[
        {"version": "1.0.0", "weight": 50},
        {"version": "2.0.0", "weight": 50}
    ]
)
```

### Rate Limiting

```python
from dmsc import DMSCRateLimitConfig

# Configure rate limiting per service
await mesh.configure_rate_limiting(
    service="api-service",
    config=DMSCRateLimitConfig(
        requests_per_minute=1000,
        requests_per_hour=10000,
        burst_size=100,
        strategy="token_bucket"
    )
)
```

## Statistics and Monitoring

```python
from dmsc import DMSCServiceMesh

mesh = DMSCServiceMesh()
await mesh.initialize()

# Get mesh statistics
stats = mesh.get_statistics()

print(f"Total services: {stats.total_services}")
print(f"Total instances: {stats.total_instances}")
print(f"Healthy instances: {stats.healthy_instances}")
print(f"Unhealthy instances: {stats.unhealthy_instances}")
print(f"Total requests: {stats.total_requests}")
print(f"Failed requests: {stats.failed_requests}")
print(f"Average latency: {stats.average_latency_ms}ms")
print(f"Circuit breaker trips: {stats.circuit_breaker_trips}")
```

## Best Practices

1. **Register Health Checks**: Always implement health checks for services
2. **Use Circuit Breakers**: Protect services from cascading failures
3. **Monitor Metrics**: Track service mesh metrics in production
4. **Use Traffic Splitting**: Use canary deployments for new versions
5. **Implement Retries**: Configure retries with appropriate backoff
6. **Set Timeouts**: Set appropriate timeouts for all operations
7. **Use Load Balancing**: Distribute traffic across healthy instances
8. **Regular Health Checks**: Run health checks frequently
9. **Document Services**: Document all registered services
10. **Test Failover**: Test circuit breaker and failover behavior
