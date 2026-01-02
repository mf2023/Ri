<div align="center">

# Service Mesh Example

**Version: 0.0.3**

**Last modified date: 2026-01-01**

This example demonstrates how to use DMSC Python's service mesh module for service discovery, health checks, traffic management, load balancing, and circuit breaker functionality.

## Example Overview

This example creates a DMSC Python application with the following features:

- Service registration and discovery
- Health checks for services
- Load balancing across service instances
- Circuit breaker pattern implementation
- Traffic splitting for canary deployments
- Service mesh statistics and monitoring

## Prerequisites

- Python 3.8+
- Understanding of service mesh concepts
- Knowledge of microservices patterns

## Complete Code Example

```python
import asyncio
import random
from datetime import datetime
from typing import Dict, List, Optional, Any
from enum import Enum
from dataclasses import dataclass
from collections import deque

from dmsc import (
    DMSCAppBuilder, DMSCServiceContext, DMSCLogConfig,
    DMSCServiceMesh, DMSCServiceMeshConfig,
    DMSCServiceRegistry, DMSCServiceInstance,
    DMSCHealthChecker, DMSCHealthStatus,
    DMSCCircuitBreaker, DMSCCircuitBreakerConfig,
    DMSCCircuitBreakerState, DMSCLoadBalancer, DMSCLoadBalancerStrategy,
    DMSCTrafficConfig, DMSCTrafficSplit,
    DMSCConfig, DMSCError
)

# Service status
class ServiceStatus(Enum):
    HEALTHY = "healthy"
    UNHEALTHY = "unhealthy"
    UNKNOWN = "unknown"

# Instance status
class InstanceStatus(Enum):
    ONLINE = "online"
    OFFLINE = "offline"
    BUSY = "busy"
    CHECKING = "checking"

# Service instance data class
@dataclass
class ServiceInstance:
    instance_id: str
    service_name: str
    address: str
    port: int
    weight: int
    status: InstanceStatus
    metadata: Dict[str, Any]
    last_health_check: Optional[datetime]
    current_requests: int
    registered_at: datetime

# Service mesh service
class ServiceMeshService:
    def __init__(self, mesh: DMSCServiceMesh, context: DMSCServiceContext):
        self.mesh = mesh
        self.context = context
        self.logger = context.logger
        self.instances: Dict[str, List[ServiceInstance]] = {}
        self.circuit_breakers: Dict[str, DMSCCircuitBreaker] = {}
        self.load_balancers: Dict[str, DMSCLoadBalancer] = {}
        self.traffic_splits: Dict[str, DMSCTrafficConfig] = {}
        self.request_history: deque = deque(maxlen=10000)
        self.health_checker = DMSCHealthChecker()
    
    async def register_service(
        self,
        service_name: str,
        address: str,
        port: int,
        weight: int = 100,
        metadata: Optional[Dict] = None
    ) -> ServiceInstance:
        """Register a service instance"""
        instance_id = f"{service_name}-{address}-{port}"
        
        # Check if already registered
        if service_name in self.instances:
            for instance in self.instances[service_name]:
                if instance.instance_id == instance_id:
                    self.logger.warn("mesh", f"Instance already registered: {instance_id}")
                    return instance
        
        instance = ServiceInstance(
            instance_id=instance_id,
            service_name=service_name,
            address=address,
            port=port,
            weight=weight,
            status=InstanceStatus.ONLINE,
            metadata=metadata or {},
            last_health_check=None,
            current_requests=0,
            registered_at=datetime.now()
        )
        
        if service_name not in self.instances:
            self.instances[service_name] = []
        
        self.instances[service_name].append(instance)
        
        # Create circuit breaker for service
        self._create_circuit_breaker(service_name)
        
        # Create load balancer for service
        self._create_load_balancer(service_name)
        
        self.logger.info("mesh", f"Service registered: {service_name} at {address}:{port}")
        
        return instance
    
    def _create_circuit_breaker(self, service_name: str):
        """Create a circuit breaker for a service"""
        if service_name not in self.circuit_breakers:
            config = DMSCCircuitBreakerConfig(
                failure_threshold=5,
                success_threshold=2,
                timeout_seconds=30
            )
            self.circuit_breakers[service_name] = DMSCCircuitBreaker(config)
            self.logger.info("mesh", f"Circuit breaker created: {service_name}")
    
    def _create_load_balancer(self, service_name: str):
        """Create a load balancer for a service"""
        if service_name not in self.load_balancers:
            lb = DMSCLoadBalancer(strategy=DMSCLoadBalancerStrategy.WEIGHTED)
            self.load_balancers[service_name] = lb
            self.logger.info("mesh", f"Load balancer created: {service_name}")
    
    async def discover_services(
        self,
        service_name: str,
        healthy_only: bool = True
    ) -> List[ServiceInstance]:
        """Discover service instances"""
        if service_name not in self.instances:
            return []
        
        instances = self.instances[service_name]
        
        if healthy_only:
            instances = [i for i in instances if i.status == InstanceStatus.ONLINE]
        
        return instances
    
    async def select_instance(
        self,
        service_name: str,
        strategy: DMSCLoadBalancerStrategy = None
    ) -> Optional[ServiceInstance]:
        """Select a service instance using load balancing"""
        instances = await self.discover_services(service_name)
        
        if not instances:
            return None
        
        if strategy is None:
            if service_name in self.load_balancers:
                lb = self.load_balancers[service_name]
                selected = await lb.select(instances)
                if selected:
                    return selected
        
        # Default: weighted random selection
        total_weight = sum(i.weight for i in instances)
        r = random.uniform(0, total_weight)
        
        cumulative = 0
        for instance in instances:
            cumulative += instance.weight
            if r <= cumulative:
                return instance
        
        return instances[0]
    
    async def route_request(
        self,
        service_name: str,
        path: str,
        method: str,
        headers: Dict,
        body: Optional[bytes]
    ) -> Dict:
        """Route a request to a service instance with load balancing and circuit breaker"""
        
        # Check circuit breaker
        if service_name in self.circuit_breakers:
            cb = self.circuit_breakers[service_name]
            if cb.get_state() == DMSCCircuitBreakerState.OPEN:
                return {
                    "status_code": 503,
                    "body": b'{"error": "Service unavailable"}'
                }
        
        # Select instance using load balancer
        instance = await self.select_instance(service_name)
        
        if not instance:
            return {
                "status_code": 503,
                "body": b'{"error": "No healthy instances available"}'
            }
        
        # Check circuit breaker before request
        if service_name in self.circuit_breakers:
            cb = self.circuit_breakers[service_name]
            if not await cb.allow_request():
                return {
                    "status_code": 503,
                    "body": b'{"error": "Circuit breaker open"}'
                }
        
        # Track request
        instance.current_requests += 1
        self.request_history.append({
            "timestamp": datetime.now(),
            "service": service_name,
            "instance": instance.instance_id,
            "path": path
        })
        
        try:
            # In a real implementation, this would make an HTTP request
            # For this example, we simulate a successful response
            response_body = f'{{"message": "Response from {instance.instance_id}"}}'
            
            # Record success
            if service_name in self.circuit_breakers:
                cb = self.circuit_breakers[service_name]
                await cb.record_success()
            
            return {
                "status_code": 200,
                "body": response_body.encode()
            }
        
        except Exception as e:
            # Record failure
            if service_name in self.circuit_breakers:
                cb = self.circuit_breakers[service_name]
                await cb.record_failure()
            
            self.logger.error("mesh", f"Request failed: {e}")
            raise
        
        finally:
            instance.current_requests -= 1
    
    def configure_traffic_split(
        self,
        service_name: str,
        splits: List[Dict]
    ) -> DMSCTrafficConfig:
        """Configure traffic splitting for canary deployments"""
        traffic_config = DMSCTrafficConfig(
            service_name=service_name,
            splits=[
                DMSCTrafficSplit(
                    version=split["version"],
                    weight=split["weight"]
                )
                for split in splits
            ]
        )
        
        self.traffic_splits[service_name] = traffic_config
        self.logger.info("mesh", f"Traffic split configured for {service_name}: {splits}")
        
        return traffic_config
    
    def get_circuit_breaker_state(self, service_name: str) -> str:
        """Get circuit breaker state for a service"""
        if service_name in self.circuit_breakers:
            return self.circuit_breakers[service_name].get_state().value
        return "not_configured"
    
    def get_all_instances(self) -> Dict[str, List[Dict]]:
        """Get all registered service instances"""
        result = {}
        
        for service_name, instances in self.instances.items():
            result[service_name] = [
                {
                    "instance_id": i.instance_id,
                    "address": i.address,
                    "port": i.port,
                    "weight": i.weight,
                    "status": i.status.value,
                    "current_requests": i.current_requests,
                    "registered_at": i.registered_at.isoformat()
                }
                for i in instances
            ]
        
        return result
    
    def get_mesh_stats(self) -> Dict:
        """Get service mesh statistics"""
        total_instances = sum(len(instances) for instances in self.instances.values())
        healthy_instances = sum(
            len([i for i in instances if i.status == InstanceStatus.ONLINE])
            for instances in self.instances.values()
        )
        
        return {
            "total_services": len(self.instances),
            "total_instances": total_instances,
            "healthy_instances": healthy_instances,
            "circuit_breakers": {
                name: cb.get_state().value
                for name, cb in self.circuit_breakers.items()
            },
            "traffic_splits": {
                name: [{"version": s.version, "weight": s.weight} for s in config.splits]
                for name, config in self.traffic_splits.items()
            }
        }

# Request handlers
async def handle_register_service(context: DMSCServiceContext):
    """Register a service instance"""
    data = await context.http.request.json()
    
    service_name = data.get("service_name")
    address = data.get("address")
    port = data.get("port", 8080)
    weight = data.get("weight", 100)
    metadata = data.get("metadata", {})
    
    if not service_name or not address:
        return {"status": "error", "message": "service_name and address required"}, 400
    
    mesh_service = context.mesh_service
    instance = await mesh_service.register_service(
        service_name=service_name,
        address=address,
        port=port,
        weight=weight,
        metadata=metadata
    )
    
    return {
        "status": "success",
        "data": {
            "instance_id": instance.instance_id,
            "service_name": instance.service_name,
            "address": f"{instance.address}:{instance.port}",
            "weight": instance.weight
        }
    }

async def handle_discover_services(context: DMSCServiceContext):
    """Discover service instances"""
    data = await context.http.request.json()
    
    service_name = data.get("service_name")
    healthy_only = data.get("healthy_only", True)
    
    if not service_name:
        return {"status": "error", "message": "service_name required"}, 400
    
    mesh_service = context.mesh_service
    instances = await mesh_service.discover_services(service_name, healthy_only)
    
    return {
        "status": "success",
        "data": {
            "service_name": service_name,
            "count": len(instances),
            "instances": [
                {
                    "instance_id": i.instance_id,
                    "address": i.address,
                    "port": i.port,
                    "weight": i.weight,
                    "status": i.status.value
                }
                for i in instances
            ]
        }
    }

async def handle_route_request(context: DMSCServiceContext):
    """Route a request to a service"""
    request = context.http.request
    
    path_parts = request.path.split("/")
    if len(path_parts) < 3:
        return {"status": "error", "message": "Invalid path"}, 400
    
    service_name = path_parts[2]  # /mesh/{service_name}/...
    
    mesh_service = context.mesh_service
    response = await mesh_service.route_request(
        service_name=service_name,
        path=request.path,
        method=request.method,
        headers=dict(request.headers),
        body=await request.body() if request.body else None
    )
    
    return response

async def handle_configure_traffic_split(context: DMSCServiceContext):
    """Configure traffic splitting for a service"""
    data = await context.http.request.json()
    
    service_name = data.get("service_name")
    splits = data.get("splits", [])
    
    if not service_name or not splits:
        return {"status": "error", "message": "service_name and splits required"}, 400
    
    mesh_service = context.mesh_service
    config = mesh_service.configure_traffic_split(service_name, splits)
    
    return {
        "status": "success",
        "data": {
            "service_name": service_name,
            "splits": [{"version": s.version, "weight": s.weight} for s in config.splits]
        }
    }

async def handle_get_circuit_state(context: DMSCServiceContext):
    """Get circuit breaker state"""
    data = await context.http.request.json()
    
    service_name = data.get("service_name")
    
    if not service_name:
        return {"status": "error", "message": "service_name required"}, 400
    
    mesh_service = context.mesh_service
    state = mesh_service.get_circuit_breaker_state(service_name)
    
    return {
        "status": "success",
        "data": {
            "service_name": service_name,
            "circuit_breaker_state": state
        }
    }

async def handle_get_instances(context: DMSCServiceContext):
    """Get all registered instances"""
    mesh_service = context.mesh_service
    instances = mesh_service.get_all_instances()
    
    return {"status": "success", "data": instances}

async def handle_get_stats(context: DMSCServiceContext):
    """Get service mesh statistics"""
    mesh_service = context.mesh_service
    stats = mesh_service.get_mesh_stats()
    
    return {"status": "success", "data": stats}

# Main application
async def main():
    app = DMSCAppBuilder()
    
    app.with_logging(DMSCLogConfig(level="INFO", format="json"))
    
    app.with_config(DMSCConfig.from_file("config.yaml"))
    
    # Configure service mesh
    app.with_service_mesh(DMSCServiceMeshConfig(
        enable_service_discovery=True,
        enable_health_check=True,
        enable_traffic_management=True
    ))
    
    app.with_http()
    
    dms_app = app.build()
    
    # Initialize service mesh
    mesh_config = DMSCServiceMeshConfig(
        enable_service_discovery=True,
        enable_health_check=True,
        enable_traffic_management=True,
        health_check_interval=30
    )
    mesh = DMSCServiceMesh(mesh_config)
    
    # Initialize service mesh service
    mesh_service = ServiceMeshService(mesh, dms_app.context)
    dms_app.context.mesh_service = mesh_service
    
    # Add routes
    dms_app.router.add_route("POST", "/mesh/register", handle_register_service)
    dms_app.router.add_route("POST", "/mesh/discover", handle_discover_services)
    dms_app.router.add_route("GET", "/mesh/{service_name}/**", handle_route_request)
    dms_app.router.add_route("POST", "/mesh/traffic-split", handle_configure_traffic_split)
    dms_app.router.add_route("POST", "/mesh/circuit-state", handle_get_circuit_state)
    dms_app.router.add_route("GET", "/mesh/instances", handle_get_instances)
    dms_app.router.add_route("GET", "/mesh/stats", handle_get_stats)
    
    await dms_app.run_async()

if __name__ == "__main__":
    asyncio.run(main())
```

## Code Analysis

### Service Mesh Architecture

1. **Service Registry**: Register and manage service instances
2. **Service Discovery**: Discover healthy service instances
3. **Load Balancing**: Route requests using weighted load balancing
4. **Circuit Breaker**: Protect services from cascading failures
5. **Traffic Splitting**: Support canary deployments

### Key Components

- **DMSCServiceMesh**: Main service mesh interface
- **DMSCServiceMeshConfig**: Service mesh configuration
- **DMSCCircuitBreaker**: Circuit breaker pattern implementation
- **DMSCLoadBalancer**: Load balancing across instances
- **DMSCTrafficConfig**: Traffic splitting configuration

## Running Steps

1. Save the code to `service_mesh_app.py`
2. Install DMSC Python:
   ```bash
   pip install dmsc
   ```
3. Run the application:
   ```bash
   python service_mesh_app.py
   ```
4. Test the API endpoints:

   ```bash
   # Register a service instance
   curl -X POST http://localhost:8080/mesh/register \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service", "address": "user-service", "port": 8080, "weight": 100}'
   
   # Register multiple instances for load balancing
   curl -X POST http://localhost:8080/mesh/register \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service", "address": "user-service-2", "port": 8080, "weight": 100}'
   
   # Discover services
   curl -X POST http://localhost:8080/mesh/discover \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service", "healthy_only": true}'
   
   # Route request to service
   curl http://localhost:8080/mesh/user-service/api/users
   
   # Configure traffic split (canary deployment)
   curl -X POST http://localhost:8080/mesh/traffic-split \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service", "splits": [{"version": "v1", "weight": 90}, {"version": "v2", "weight": 10}]}'
   
   # Get circuit breaker state
   curl -X POST http://localhost:8080/mesh/circuit-state \
     -H "Content-Type: application/json" \
     -d '{"service_name": "user-service"}'
   
   # Get all instances
   curl http://localhost:8080/mesh/instances
   
   # Get mesh statistics
   curl http://localhost:8080/mesh/stats
   ```

## Expected Output

### Register Service Response

```json
{
  "status": "success",
  "data": {
    "instance_id": "user-service-user-service-8080",
    "service_name": "user-service",
    "address": "user-service:8080",
    "weight": 100
  }
}
```

### Discover Services Response

```json
{
  "status": "success",
  "data": {
    "service_name": "user-service",
    "count": 2,
    "instances": [
      {
        "instance_id": "user-service-user-service-8080",
        "address": "user-service",
        "port": 8080,
        "weight": 100,
        "status": "online"
      }
    ]
  }
}
```

### Mesh Stats Response

```json
{
  "status": "success",
  "data": {
    "total_services": 3,
    "total_instances": 7,
    "healthy_instances": 6,
    "circuit_breakers": {
      "user-service": "closed",
      "order-service": "closed",
      "payment-service": "closed"
    },
    "traffic_splits": {
      "user-service": [{"version": "v1", "weight": 90}, {"version": "v2", "weight": 10}]
    }
  }
}
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
