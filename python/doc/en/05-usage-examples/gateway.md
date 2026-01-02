<div align="center">

# Gateway Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's gateway module for API routing, load balancing, rate limiting, and circuit breaker functionality.

## Example Overview

This example creates a DMSC Python application with the following features:

- Basic API gateway setup and configuration
- Route management and request routing
- Load balancing across backend services
- Circuit breaker pattern implementation
- Rate limiting for API protection
- Request/response transformation
- Health checks for backend services

## Prerequisites

- Python 3.8+
- Understanding of API gateway concepts
- Knowledge of load balancing patterns
- Basic understanding of circuit breaker pattern

## Complete Code Example

```python
import asyncio
from datetime import datetime
from typing import Dict, List, Optional
from enum import Enum
from dataclasses import dataclass
from collections import deque

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCGateway, DMSCGatewayConfig, DMSCRoute,
    DMSCRouteMethod, DMSCCircuitBreaker, DMSCCircuitBreakerConfig,
    DMSCCircuitBreakerState, DMSCLoadBalancer, DMSCLoadBalancerStrategy,
    DMSCRateLimitConfig, DMSCRateLimiter, DMSCConfig, DMSCError
)

# Circuit breaker states
class CircuitState(Enum):
    CLOSED = "closed"
    OPEN = "open"
    HALF_OPEN = "half_open"

# Rate limit response
@dataclass
class RateLimitInfo:
    limit: int
    remaining: int
    reset_time: int

# Backend service representation
@dataclass
class BackendServer:
    id: str
    url: str
    weight: int
    healthy: bool
    current_requests: int
    last_health_check: datetime

# Gateway service
class GatewayService:
    def __init__(self, gateway: DMSCGateway, context: DMSCServiceContext):
        self.gateway = gateway
        self.context = context
        self.logger = context.logger
        self.circuit_breakers: Dict[str, DMSCCircuitBreaker] = {}
        self.load_balancers: Dict[str, DMSCLoadBalancer] = {}
        self.rate_limiter = DMSCRateLimiter()
        self.backend_servers: Dict[str, List[BackendServer]] = {}
        self.request_history: deque = deque(maxlen=1000)
    
    def create_circuit_breaker(self, name: str, config: DMSCCircuitBreakerConfig) -> DMSCCircuitBreaker:
        """Create a circuit breaker for a backend service"""
        cb = DMSCCircuitBreaker(config)
        self.circuit_breakers[name] = cb
        self.logger.info("gateway", f"Circuit breaker created: {name}")
        return cb
    
    def create_load_balancer(self, name: str, strategy: DMSCLoadBalancerStrategy) -> DMSCLoadBalancer:
        """Create a load balancer for a backend service"""
        lb = DMSCLoadBalancer(strategy=strategy)
        self.load_balancers[name] = lb
        self.logger.info("gateway", f"Load balancer created: {name}")
        return lb
    
    def add_backend_servers(self, service_name: str, servers: List[Dict]):
        """Add backend servers to a load balancer"""
        if service_name not in self.backend_servers:
            self.backend_servers[service_name] = []
        
        for server in servers:
            backend = BackendServer(
                id=server["id"],
                url=server["url"],
                weight=server.get("weight", 100),
                healthy=True,
                current_requests=0,
                last_health_check=datetime.now()
            )
            self.backend_servers[service_name].append(backend)
        
        self.logger.info("gateway", f"Added {len(servers)} servers to {service_name}")
    
    async def route_request(
        self,
        path: str,
        method: str,
        headers: Dict,
        body: Optional[bytes],
        service_name: str
    ) -> Dict:
        """Route request to backend service with load balancing and circuit breaker"""
        
        # Check rate limit
        client_ip = headers.get("X-Forwarded-For", "unknown")
        if not self.rate_limiter.allow_request(client_ip):
            return {
                "status_code": 429,
                "headers": {"Content-Type": "application/json"},
                "body": b'{"error": "Rate limit exceeded"}'
            }
        
        # Check circuit breaker
        if service_name in self.circuit_breakers:
            cb = self.circuit_breakers[service_name]
            if cb.get_state() == CircuitState.OPEN:
                return {
                    "status_code": 503,
                    "headers": {"Content-Type": "application/json"},
                    "body": b'{"error": "Service unavailable"}'
                }
        
        # Select server using load balancer
        if service_name in self.load_balancers:
            lb = self.load_balancers[service_name]
            server = await lb.select_server(self.backend_servers.get(service_name, []))
            
            if not server:
                return {
                    "status_code": 503,
                    "headers": {"Content-Type": "application/json"},
                    "body": b'{"error": "No healthy servers available"}'
                }
            
            # Record request start
            server.current_requests += 1
            self.request_history.append({
                "timestamp": datetime.now(),
                "service": service_name,
                "server": server.id
            })
            
            try:
                # Forward request to backend
                response = await self._forward_request(server.url, method, headers, body)
                
                # Record success
                if service_name in self.circuit_breakers:
                    await self.circuit_breakers[service_name].record_success()
                
                return response
            
            except Exception as e:
                # Record failure
                if service_name in self.circuit_breakers:
                    await self.circuit_breakers[service_name].record_failure()
                
                self.logger.error("gateway", f"Request failed: {e}")
                raise
            
            finally:
                server.current_requests -= 1
        else:
            return {
                "status_code": 404,
                "headers": {"Content-Type": "application/json"},
                "body": b'{"error": "Service not found"}'
            }
    
    async def _forward_request(
        self,
        server_url: str,
        method: str,
        headers: Dict,
        body: Optional[bytes]
    ) -> Dict:
        """Forward request to backend server (simplified)"""
        # In a real implementation, this would make an HTTP request
        # For this example, we simulate a successful response
        return {
            "status_code": 200,
            "headers": {"Content-Type": "application/json"},
            "body": b'{"message": "Success from backend"}'
        }
    
    async def get_rate_limit_status(self, client_ip: str) -> RateLimitInfo:
        """Get rate limit status for a client"""
        return self.rate_limiter.get_status(client_ip)
    
    def get_circuit_breaker_state(self, service_name: str) -> Optional[CircuitState]:
        """Get circuit breaker state for a service"""
        if service_name in self.circuit_breakers:
            return self.circuit_breakers[service_name].get_state()
        return None
    
    def get_statistics(self) -> Dict:
        """Get gateway statistics"""
        total_requests = len(self.request_history)
        
        service_stats = {}
        for service_name, servers in self.backend_servers.items.items():
            service_stats[service_name] = {
                "total_requests": sum(1 for r in self.request_history if r["service"] == service_name),
                "server_count": len(servers),
                "healthy_servers": sum(1 for s in servers if s.healthy)
            }
        
        return {
            "total_requests": total_requests,
            "services": service_stats,
            "circuit_breakers": {
                name: cb.get_state().value
                for name, cb in self.circuit_breakers.items()
            }
        }

# Request handlers
async def handle_route_request(context: DMSCServiceContext):
    """Handle incoming API request"""
    request = context.http.request
    
    # Extract service name from path
    path_parts = request.path.split("/")
    if len(path_parts) < 3:
        return {"status": "error", "message": "Invalid path"}, 400
    
    service_name = path_parts[2]  # /api/{service_name}/...
    
    gateway_service = context.gateway_service
    response = await gateway_service.route_request(
        path=request.path,
        method=request.method,
        headers=dict(request.headers),
        body=await request.body() if request.body else None,
        service_name=service_name
    )
    
    return response

async def handle_get_stats(context: DMSCServiceContext):
    """Get gateway statistics"""
    gateway_service = context.gateway_service
    stats = gateway_service.get_statistics()
    
    return {"status": "success", "data": stats}

async def handle_get_circuit_state(context: DMSCServiceContext):
    """Get circuit breaker state"""
    data = await context.http.request.json()
    service_name = data.get("service_name")
    
    if not service_name:
        return {"status": "error", "message": "service_name required"}, 400
    
    gateway_service = context.gateway_service
    state = gateway_service.get_circuit_breaker_state(service_name)
    
    if state:
        return {"status": "success", "data": {"service": service_name, "state": state.value}}
    else:
        return {"status": "error", "message": "Circuit breaker not found"}, 404

async def handle_get_rate_limit(context: DMSCServiceContext):
    """Get rate limit status"""
    client_ip = context.http.request.headers.get("X-Forwarded-For", "unknown")
    
    gateway_service = context.gateway_service
    rate_limit = await gateway_service.get_rate_limit_status(client_ip)
    
    return {
        "status": "success",
        "data": {
            "limit": rate_limit.limit,
            "remaining": rate_limit.remaining,
            "reset_time": rate_limit.reset_time
        }
    }

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize gateway
    gateway = DMSCGateway(DMSCGatewayConfig(
        host="0.0.0.0",
        port=8080,
        workers=4
    ))
    
    gateway_service = GatewayService(gateway, dms_app.context)
    dms_app.context.gateway_service = gateway_service
    
    # Configure circuit breaker
    gateway_service.create_circuit_breaker(
        "user-service",
        DMSCCircuitBreakerConfig(
            failure_threshold=5,
            success_threshold=2,
            timeout_seconds=30
        )
    )
    
    # Configure load balancer
    gateway_service.create_load_balancer(
        "user-service",
        DMSCLoadBalancerStrategy.ROUND_ROBIN
    )
    
    # Add backend servers
    gateway_service.add_backend_servers("user-service", [
        {"id": "user-1", "url": "http://user-service-1:8081", "weight": 100},
        {"id": "user-2", "url": "http://user-service-2:8081", "weight": 100},
        {"id": "user-3", "url": "http://user-service-3:8081", "weight": 50}
    ])
    
    # Configure rate limiter
    gateway_service.rate_limiter = DMSCRateLimiter(
        DMSCRateLimitConfig(
            requests_per_minute=1000,
            burst_size=100
        )
    )
    
    # Add routes
    dms_app.router.add_route("GET", "/api/{service_name}/**", handle_route_request)
    dms_app.router.add_route("POST", "/api/{service_name}/**", handle_route_request)
    dms_app.router.add_route("PUT", "/api/{service_name}/**", handle_route_request)
    dms_app.router.add_route("DELETE", "/api/{service_name}/**", handle_route_request)
    dms_app.router.add_route("GET", "/stats", handle_get_stats)
    dms_app.router.add_route("POST", "/circuit-state", handle_get_circuit_state)
    dms_app.router.add_route("GET", "/rate-limit", handle_get_rate_limit)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Gateway Architecture

1. **Route Management**: Dynamic routing based on URL paths
2. **Load Balancing**: Round-robin, weighted, and other strategies
3. **Circuit Breaker**: Protection against cascading failures
4. **Rate Limiting**: API protection from overuse
5. **Health Checks**: Backend service monitoring

### Key Components

- **DMSCGateway**: Main gateway interface
- **DMSCRoute**: Route definition and configuration
- **DMSCCircuitBreaker**: Circuit breaker pattern implementation
- **DMSCLoadBalancer**: Load balancing across backend servers
- **DMSCRateLimiter**: Rate limiting for API protection

## Running Steps

1. Save the code to `gateway_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python gateway_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Route request to user service
   curl http://localhost:8080/api/users/profile
   
   # Get gateway statistics
   curl http://localhost:8080/stats
   
   # Get circuit breaker state
   curl -X POST http://localhost:8080/circuit-state \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service"}'
   
   # Get rate limit status
   curl http://localhost:8080/rate-limit
   ```

## Expected Output

### Gateway Stats Response

```json
{
  "status": "success",
  "data": {
    "total_requests": 1500,
    "services": {
      "user-service": {
        "total_requests": 800,
        "server_count": 3,
        "healthy_servers": 3
      }
    },
    "circuit_breakers": {
      "user-service": "closed"
    }
  }
}
```

### Circuit Breaker State Response

```json
{
  "status": "success",
  "data": {
    "service": "user-service",
    "state": "closed"
  }
}
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
