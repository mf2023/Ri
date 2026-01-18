#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Service Mesh Module Example

This example demonstrates how to use the service mesh module in DMSC,
including service discovery, health checks, and traffic management.

Features Demonstrated:
- Service registration and discovery
- Health check configuration
- Instance management
- Service status monitoring
"""

import dmsc
from dmsc.service_mesh import (
    DMSCServiceMesh, DMSCServiceMeshConfig, 
    DMSCServiceDiscovery, DMSCServiceInstance, DMSCServiceStatus,
)
import asyncio


async def main():
    """
    Main async entry point for the service mesh module example.
    
    This function demonstrates the complete service mesh workflow including:
    - Service mesh configuration and initialization
    - Service instance registration with metadata
    - Service discovery and instance lookup
    - Instance health status management
    - Health check execution and reporting
    - Service mesh statistics and monitoring
    - Service deregistration and cleanup
    
    The example shows how DMSC handles service mesh functionality for
    microservices architecture with dynamic service discovery and health monitoring.
    """
    print("=== DMSC Service Mesh Module Example ===\n")
    
    # Configuration Setup: Create service mesh configuration
    # Parameters:
    # - service_name: Name of this service instance for mesh identification
    # - namespace: Logical grouping of services (e.g., Kubernetes namespace)
    # - instance_id: Unique identifier for this service instance
    # - host: Network host where this service is running
    # - port: Network port where this service listens
    # - health_check_interval_secs: How often to perform health checks (30 seconds)
    # - failure_threshold: Consecutive failures before marking unhealthy (3)
    # - recovery_threshold: Consecutive successes before marking healthy (2)
    config = DMSCServiceMeshConfig(
        service_name="dmsc-example",
        namespace="default",
        instance_id="instance-001",
        host="localhost",
        port=8080,
        health_check_interval_secs=30,
        failure_threshold=3,
        recovery_threshold=2,
    )
    
    # Module Initialization: Create service mesh instance
    # The mesh provides service discovery, load balancing, and health management
    print("1. Creating service mesh...")
    service_mesh = await DMSCServiceMesh.create(config)
    print("   Service mesh initialized\n")
    
    # Step 2: Register first service instance
    # Demonstrates service registration with full metadata
    # Services are registered to make them discoverable by other services
    print("2. Registering service instance...")
    
    # Create service instance with connection and version information
    # - service_name: Logical name of the service (user-service)
    # - host: Where the service is running (localhost)
    # - port: Service port (8081)
    # - version: Service version for routing (v1.0.0)
    instance = DMSCServiceInstance(
        service_name="user-service",
        host="localhost",
        port=8081,
        version="v1.0.0",
    )
    
    # Register the instance with the service mesh
    # Mesh will track this instance for discovery and health monitoring
    await service_mesh.register_instance(instance)
    print("   User service registered\n")
    
    # Step 3: Register additional service instance
    # Demonstrates registering multiple instances of same service
    # Multiple instances enable load balancing and high availability
    print("3. Registering another instance...")
    
    instance2 = DMSCServiceInstance(
        service_name="user-service",
        host="localhost",
        port=8082,
        version="v1.0.0",
    )
    await service_mesh.register_instance(instance2)
    print("   Second user service instance registered\n")
    
    # Step 4: Register different service type
    # Demonstrates registering completely separate service
    print("4. Registering order service...")
    
    order_instance = DMSCServiceInstance(
        service_name="order-service",
        host="localhost",
        port=8083,
        version="v1.0.0",
    )
    await service_mesh.register_instance(order_instance)
    print("   Order service registered\n")
    
    # Step 5: Service discovery
    # Demonstrates finding service instances by name
    # Discovery is fundamental to microservices communication
    print("5. Discovering services...")
    
    # Query for all instances of user-service
    # Returns list of available instances with their metadata
    instances = await service_mesh.discover_instances("user-service")
    print(f"   Found {len(instances)} user-service instance(s):")
    for instance in instances:
        print(f"   - {instance.host()}:{instance.port()} (status: {instance.status()})")
    print()
    
    # Step 6: List all registered services
    # Demonstrates enumeration of all services in the mesh
    print("6. Listing all registered services...")
    
    # Get list of unique service names in the mesh
    services = await service_mesh.list_services()
    print("   Registered services:")
    for service in services:
        # For each service, discover its instances
        instances = await service_mesh.discover_instances(service)
        print(f"   - {service}: {len(instances)} instance(s)")
    print()
    
    # Step 7: Update instance health
    # Demonstrates manual health status management
    # Health status affects load balancer routing decisions
    print("7. Updating instance health...")
    
    # Get instances and update first one's health
    instances = await service_mesh.discover_instances("user-service")
    if instances:
        first_instance = instances[0]
        
        # Update health status to indicate instance state
        # Status can be: Healthy, Unhealthy, Unknown, etc.
        await service_mesh.update_instance_health(
            first_instance.id(),
            DMSCServiceStatus.Healthy,
        )
        print(f"   Updated instance {first_instance.id()} to Healthy\n")
    
    # Step 8: Run health checks
    # Demonstrates automatic health monitoring
    # Health checks verify service instances are responding correctly
    print("8. Simulating health check...")
    
    # Execute health checks on all registered instances
    # Returns comprehensive report with pass/fail status
    health_report = await service_mesh.run_health_checks()
    print("   Health check completed:")
    print(f"   - Total checks: {health_report.total_checks()}")
    print(f"   - Passed: {health_report.passed_checks()}")
    print(f"   - Failed: {health_report.failed_checks()}\n")
    
    # Step 9: Get service mesh statistics
    # Demonstrates aggregate monitoring of mesh state
    print("9. Getting service mesh statistics...")
    
    # Get aggregated metrics about registered instances
    stats = await service_mesh.get_statistics()
    print("   Service mesh statistics:")
    print(f"   - Total instances: {stats.total_instances()}")
    print(f"   - Healthy instances: {stats.healthy_instances()}")
    print(f"   - Unhealthy instances: {stats.unhealthy_instances()}")
    print()
    
    # Step 10: Deregister service
    # Demonstrates removing service from mesh
    # Cleanup is important for scaling down or decommissioning services
    print("10. Deregistering service...")
    
    # Remove specific instance from mesh
    await service_mesh.deregister_instance("user-service", "instance-001")
    print("   Deregistered user-service instance-001\n")
    
    # Step 11: Final discovery
    # Verify instance was successfully removed
    print("11. Final service discovery...")
    
    instances = await service_mesh.discover_instances("user-service")
    print(f"   Remaining user-service instances: {len(instances)}\n")
    
    print("=== Service Mesh Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
