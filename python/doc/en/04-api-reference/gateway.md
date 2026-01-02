<div align="center">

# Gateway API Reference

**Version: 0.0.3**

**Last modified date: 2026-01-01**

The gateway module provides API gateway functionality, including routing, load balancing, rate limiting, and circuit breaker capabilities.

## Module Overview

</div>

The gateway module contains the following core components:

- **DMSCGateway**: API gateway main interface
- **DMSCGatewayConfig**: Gateway configuration
- **DMSCRoute**: Route definition
- **DMSCRouter**: Route manager
- **DMSCGatewayRequest**: Gateway request
- **DMSCGatewayResponse**: Gateway response

<div align="center">

## Core Components

</div>

### DMSCGateway

API gateway main interface.

#### Methods

| Method | Description | Parameters | Returns |
|:--------|:-------------|:--------|:--------|
| `start()` | Start gateway | None | `None` |
| `stop()` | Stop gateway | None | `None` |
| `add_route(route)` | Add route | `route: DMSCRoute` | `None` |
| `remove_route(path)` | Remove route | `path: str` | `None` |
| `get_statistics()` | Get statistics | None | `dict` |

#### Usage Example

```python
from dmsc import DMSCGateway, DMSCGatewayConfig, DMSCRoute

config = DMSCGatewayConfig(
    host="0.0.0.0",
    port=8080,
    num_workers=4
)
gateway = DMSCGateway(config)

# Add routes
gateway.add_route(DMSCRoute(
    path="/api/users",
    target="http://user-service:8081",
    methods=["GET", "POST"]
))

gateway.add_route(DMSCRoute(
    path="/api/orders",
    target="http://order-service:8082",
    methods=["GET", "POST", "DELETE"]
))

# Start gateway
gateway.start()
```

### DMSCGatewayConfig

Gateway configuration.

```python
from dmsc import DMSCGatewayConfig

config = DMSCGatewayConfig(
    host="0.0.0.0",
    port=8080,
    num_workers=4,
    request_timeout=30,
    max_connections=10000,
    enable_rate_limiting=True,
    rate_limit_requests=1000,
    rate_limit_window=60,
    enable_circuit_breaker=True,
    circuit_breaker_threshold=5,
    circuit_breaker_timeout=30,
    load_balancer_strategy="round_robin"
)
```

### DMSCRoute

Route definition.

```python
from dmsc import DMSCRoute, DMSCRouteMethod

# Create route
route = DMSCRoute(
    path="/api/users/{id}",
    target="http://user-service:8081",
    methods=[DMSCRouteMethod.GET, DMSCRouteMethod.PUT, DMSCRouteMethod.DELETE],
    strip_path_prefix=False,
    preserve_host_header=False,
    timeout=30,
    retry_count=3
)

# Add middleware
route.add_middleware("auth")
route.add_middleware("logging")
route.add_filter("header_filter")
```

## Routing

### Basic Routing

```python
from dmsc import DMSCGateway, DMSCRoute

gateway = DMSCGateway()

# Simple route
gateway.add_route(DMSCRoute(
    path="/api/health",
    target="http://health-service:8080/health",
    methods=["GET"]
))

# Dynamic route with path parameter
gateway.add_route(DMSCRoute(
    path="/api/users/{userId}",
    target="http://user-service:8080/users/{userId}",
    methods=["GET"]
))

# Route with query parameters
gateway.add_route(DMSCRoute(
    path="/api/search",
    target="http://search-service:8080/search",
    methods=["GET"]
))
```

### Route Priority

```python
from dmsc import DMSCRoute

# Routes are matched in order of priority
# More specific routes should have higher priority

# High priority - specific route
specific_route = DMSCRoute(
    path="/api/users/123",
    target="http://user-service:8080/users/123",
    priority=100
)

# Low priority - wildcard route
wildcard_route = DMSCRoute(
    path="/api/*",
    target="http://api-service:8080/",
    priority=1
)
```

## Load Balancing

```python
from dmsc import DMSCGateway, DMSCRoute

gateway = DMSCGateway()

# Round-robin load balancing
gateway.add_route(DMSCRoute(
    path="/api/users",
    target=["http://user-1:8080", "http://user-2:8080", "http://user-3:8080"],
    load_balancer_strategy="round_robin"
))

# Weighted load balancing
gateway.add_route(DMSCRoute(
    path="/api/orders",
    target=[
        ("http://order-primary:8080", 3),  # 3x weight
        ("http://order-secondary:8080", 1)  # 1x weight
    ],
    load_balancer_strategy="weighted"
))

# Least connections load balancing
gateway.add_route(DMSCRoute(
    path="/api/products",
    target="http://product-service:8080",
    load_balancer_strategy="least_connections"
))
```

## Rate Limiting

```python
from dmsc import DMSCGateway, DMSCRateLimitConfig

gateway = DMSCGateway()

# Configure rate limiting
gateway.configure_rate_limiting(
    DMSCRateLimitConfig(
        requests_per_minute=100,
        requests_per_hour=1000,
        burst_size=20,
        strategy="sliding_window"
    )
)

# Per-user rate limiting
gateway.configure_rate_limiting(
    DMSCRateLimitConfig(
        requests_per_minute=100,
        per_user=True,
        key_generator="user_id"
    )
)
```

## Circuit Breaker

```python
from dmsc import DMSCGateway, DMSCCircuitBreakerConfig

gateway = DMSCGateway()

# Configure circuit breaker
gateway.configure_circuit_breaker(
    DMSCCircuitBreakerConfig(
        failure_threshold=5,        # Open after 5 failures
        success_threshold=2,        # Close after 2 successes
        timeout_seconds=30,         # Try again after 30 seconds
        half_open_requests=3        # Allow 3 requests in half-open state
    )
)
```

## Middleware

```python
from dmsc import DMSCGateway, DMSCMiddleware

gateway = DMSCGateway()

# Add authentication middleware
gateway.use_middleware("auth", DMSCMiddleware(
    type="authentication",
    config={"jwt_secret": "your-secret"}
))

# Add logging middleware
gateway.use_middleware("logging", DMSCMiddleware(
    type="logging",
    config={"level": "INFO", "format": "json"}
))

# Add compression middleware
gateway.use_middleware("compression", DMSCMiddleware(
    type="compression",
    config={"algorithms": ["gzip", "br"]}
))
```

## Statistics and Monitoring

```python
from dmsc import DMSCGateway

gateway = DMSCGateway()

# Get gateway statistics
stats = gateway.get_statistics()
print(f"Total requests: {stats.total_requests}")
print(f"Successful requests: {stats.successful_requests}")
print(f"Failed requests: {stats.failed_requests}")
print(f"Average latency: {stats.avg_latency_ms}ms")
print(f"Active connections: {stats.active_connections}")

# Get route-specific statistics
route_stats = gateway.get_route_statistics("/api/users")
print(f"Route requests: {route_stats.request_count}")
print(f"Route errors: {route_stats.error_count}")
```

## Best Practices

1. **Use Health Checks**: Implement health checks for backend services
2. **Configure Timeouts**: Set appropriate timeouts for all routes
3. **Implement Rate Limiting**: Protect services from overload
4. **Use Circuit Breakers**: Prevent cascading failures
5. **Monitor Performance**: Track latency and error rates
6. **Use HTTPS**: Always use HTTPS in production
7. **Implement Caching**: Cache responses when appropriate
8. **Log All Requests**: Log requests for debugging and auditing
9. **Handle Errors Gracefully**: Return meaningful error messages
10. **Test Failover**: Test circuit breaker and failover behavior
