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

This example demonstrates how to use the DMSC service mesh module for service
discovery, traffic management, and health checking.
"""

import asyncio
from dmsc import (
    DMSCServiceMesh,
    DMSCServiceMeshConfig,
    DMSCServiceDiscovery,
    DMSCServiceInstance,
    DMSCServiceStatus,
    DMSCServiceMeshStats,
    DMSCServiceEndpoint,
    DMSCServiceHealthStatus,
    DMSCHealthChecker,
    DMSCHealthSummary,
    DMSCHealthCheckType,
    DMSCTrafficManager,
    DMSCTrafficRoute,
    DMSCMatchCriteria,
    DMSCRouteAction,
    DMSCWeightedDestination,
)


async def main():
    # Create service mesh configuration
    config = DMSCServiceMeshConfig()
    config.service_name = "dmsc-example-service"
    config.namespace = "production"
    config.enable_service_discovery = True
    config.enable_health_check = True
    config.enable_traffic_management = True
    config.discovery_interval_seconds = 30
    config.health_check_interval_seconds = 10

    # Initialize service mesh
    service_mesh = DMSCServiceMesh(config)

    # Create service discovery
    print("Creating service discovery...")
    discovery = DMSCServiceDiscovery()

    # Register service instances
    print("\nRegistering service instances...")

    # User service instances
    user_service_1 = DMSCServiceInstance()
    user_service_1.instance_id = "user-service-1"
    user_service_1.service_name = "user-service"
    user_service_1.host = "192.168.1.10"
    user_service_1.port = 8080
    user_service_1.status = DMSCServiceStatus.HEALTHY
    user_service_1.weight = 3
    user_service_1.metadata = {"version": "1.0.0", "region": "us-east-1"}

    user_service_2 = DMSCServiceInstance()
    user_service_2.instance_id = "user-service-2"
    user_service_2.service_name = "user-service"
    user_service_2.host = "192.168.1.11"
    user_service_2.port = 8080
    user_service_2.status = DMSCServiceStatus.HEALTHY
    user_service_2.weight = 2
    user_service_2.metadata = {"version": "1.0.0", "region": "us-east-2"}

    # Order service instances
    order_service_1 = DMSCServiceInstance()
    order_service_1.instance_id = "order-service-1"
    order_service_1.service_name = "order-service"
    order_service_1.host = "192.168.1.20"
    order_service_1.port = 8081
    order_service_1.status = DMSCServiceStatus.HEALTHY
    order_service_1.weight = 1
    order_service_1.metadata = {"version": "1.1.0", "region": "us-east-1"}

    # Register services
    discovery.register_service(user_service_1)
    discovery.register_service(user_service_2)
    discovery.register_service(order_service_1)

    print(f"Registered 3 service instances")

    # Discover services
    print("\nDiscovering services...")
    user_services = discovery.discover_services("user-service")
    print(f"Found {len(user_services)} instances of user-service")

    order_services = discovery.discover_services("order-service")
    print(f"Found {len(order_services)} instances of order-service")

    # Create service endpoints
    print("\nCreating service endpoints...")

    user_endpoint = DMSCServiceEndpoint()
    user_endpoint.service_name = "user-service"
    user_endpoint.address = "192.168.1.10:8080"
    user_endpoint.protocol = "http"
    user_endpoint.health_status = DMSCServiceHealthStatus.HEALTHY

    order_endpoint = DMSCServiceEndpoint()
    order_endpoint.service_name = "order-service"
    order_endpoint.address = "192.168.1.20:8081"
    order_endpoint.protocol = "http"
    order_endpoint.health_status = DMSCServiceHealthStatus.HEALTHY

    print(f"Created endpoints for user-service and order-service")

    # Configure health checking
    print("\nConfiguring health checking...")

    health_checker = DMSCHealthChecker()
    health_checker.check_type = DMSCHealthCheckType.HTTP
    health_checker.interval_seconds = 10
    health_checker.timeout_seconds = 5
    health_checker.path = "/health"

    # Perform health check
    health_status = health_checker.check(user_endpoint)
    print(f"Health check for user-service: {health_status}")

    # Get health summary
    health_summary = DMSCHealthSummary()
    health_summary.total_services = 3
    health_summary.healthy_services = 3
    health_summary.unhealthy_services = 0
    health_summary.unknown_services = 0

    print(f"\nHealth summary:")
    print(f"  Total: {health_summary.total_services}")
    print(f"  Healthy: {health_summary.healthy_services}")
    print(f"  Unhealthy: {health_summary.unhealthy_services}")

    # Configure traffic management
    print("\nConfiguring traffic management...")

    traffic_manager = DMSCTrafficManager()

    # Create traffic routes
    route1 = DMSCTrafficRoute()
    route1.name = "user-service-route"
    route1.priority = 100

    # Match criteria
    match_criteria = DMSCMatchCriteria()
    match_criteria.path = "/api/users/*"
    match_criteria.methods = ["GET", "POST", "PUT", "DELETE"]
    route1.match = match_criteria

    # Route action
    route_action = DMSCRouteAction()
    route_action.route_to = "user-service"
    route1.action = route_action

    # Weighted destinations for canary deployment
    canary_route = DMSCTrafficRoute()
    canary_route.name = "canary-route"
    canary_route.priority = 90

    canary_match = DMSCMatchCriteria()
    canary_match.headers = {"x-canary": "true"}
    canary_route.match = canary_match

    # Weighted destinations
    stable_dest = DMSCWeightedDestination()
    stable_dest.service = "user-service"
    stable_dest.version = "1.0.0"
    stable_dest.weight = 90

    canary_dest = DMSCWeightedDestination()
    canary_dest.service = "user-service"
    canary_dest.version = "1.1.0"
    canary_dest.weight = 10

    canary_action = DMSCRouteAction()
    canary_action.weighted_destinations = [stable_dest, canary_dest]
    canary_route.action = canary_action

    # Add routes to traffic manager
    traffic_manager.add_route(route1)
    traffic_manager.add_route(canary_route)

    print(f"Added {len(traffic_manager.routes)} traffic routes")

    # Get service mesh statistics
    print("\nService mesh statistics:")
    mesh_stats = DMSCServiceMeshStats()
    mesh_stats.total_services = 2
    mesh_stats.total_instances = 3
    mesh_stats.healthy_instances = 3
    mesh_stats.unhealthy_instances = 0
    mesh_stats.total_requests = 10000
    mesh_stats.average_latency_ms = 25.5

    print(f"  Total services: {mesh_stats.total_services}")
    print(f"  Total instances: {mesh_stats.total_instances}")
    print(f"  Healthy instances: {mesh_stats.healthy_instances}")
    print(f"  Total requests: {mesh_stats.total_requests}")
    print(f"  Average latency: {mesh_stats.average_latency_ms}ms")

    # Deregister a service
    print("\nDeregistering service...")
    discovery.deregister_service("user-service-2")
    print("Deregistered user-service-2")

    # List all services
    print("\nAll registered services:")
    all_services = discovery.list_services()
    for service_name in all_services:
        instances = discovery.discover_services(service_name)
        print(f"  {service_name}: {len(instances)} instances")

    print("\nService mesh operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
