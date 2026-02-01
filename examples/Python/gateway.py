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
DMSC Gateway Module Example

This example demonstrates how to use the DMSC gateway module for API gateway
functionality including routing, rate limiting, circuit breaking, and load balancing.
"""

import asyncio
from dmsc import (
    DMSCGateway,
    DMSCGatewayConfig,
    DMSCRoute,
    DMSCRouter,
    DMSCRateLimiter,
    DMSCRateLimitConfig,
    DMSCRateLimitStats,
    DMSCSlidingWindowRateLimiter,
    DMSCCircuitBreaker,
    DMSCCircuitBreakerConfig,
    DMSCCircuitBreakerState,
    DMSCCircuitBreakerMetrics,
    DMSCBackendServer,
    DMSCLoadBalancerServerStats,
    DMSCLoadBalancer,
    DMSCLoadBalancerStrategy,
)


async def main():
    # Create gateway configuration
    config = DMSCGatewayConfig()
    config.host = "0.0.0.0"
    config.port = 8080
    config.enable_rate_limiting = True
    config.enable_circuit_breaker = True
    config.enable_load_balancing = True
    config.max_request_size_mb = 10
    config.timeout_seconds = 30

    # Initialize gateway
    gateway = DMSCGateway(config)

    # Create router
    router = DMSCRouter()

    # Define routes
    print("Defining routes...")

    # API routes
    user_route = DMSCRoute()
    user_route.path = "/api/users"
    user_route.methods = ["GET", "POST", "PUT", "DELETE"]
    user_route.handler = "user_service"
    user_route.middleware = ["auth", "logging"]

    order_route = DMSCRoute()
    order_route.path = "/api/orders"
    order_route.methods = ["GET", "POST"]
    order_route.handler = "order_service"
    order_route.middleware = ["auth", "rate_limit"]

    health_route = DMSCRoute()
    health_route.path = "/health"
    health_route.methods = ["GET"]
    health_route.handler = "health_check"
    health_route.middleware = []

    # Add routes to router
    router.add_route(user_route)
    router.add_route(order_route)
    router.add_route(health_route)

    print(f"Added {len(router.routes)} routes to router")

    # Configure rate limiting
    print("\nConfiguring rate limiting...")

    rate_limit_config = DMSCRateLimitConfig()
    rate_limit_config.requests_per_second = 100
    rate_limit_config.burst_size = 150
    rate_limit_config.window_size_seconds = 60

    rate_limiter = DMSCRateLimiter(rate_limit_config)

    # Check rate limit
    client_id = "client_123"
    is_allowed = rate_limiter.is_allowed(client_id)
    print(f"Rate limit check for {client_id}: {is_allowed}")

    # Get rate limit statistics
    rate_stats = DMSCRateLimitStats()
    print(f"Rate limit stats: {rate_stats.total_requests} total requests")

    # Configure circuit breaker
    print("\nConfiguring circuit breaker...")

    cb_config = DMSCCircuitBreakerConfig()
    cb_config.failure_threshold = 5
    cb_config.success_threshold = 3
    cb_config.timeout_seconds = 30
    cb_config.half_open_max_calls = 3

    circuit_breaker = DMSCCircuitBreaker(cb_config)

    # Record success/failure
    circuit_breaker.record_success()
    circuit_breaker.record_failure()

    # Get circuit breaker state
    cb_state = circuit_breaker.get_state()
    print(f"Circuit breaker state: {cb_state}")

    # Get circuit breaker metrics
    cb_metrics = DMSCCircuitBreakerMetrics()
    print(f"Circuit breaker metrics:")
    print(f"  Success count: {cb_metrics.success_count}")
    print(f"  Failure count: {cb_metrics.failure_count}")

    # Configure load balancer
    print("\nConfiguring load balancer...")

    # Create backend servers
    server1 = DMSCBackendServer()
    server1.id = "server_1"
    server1.host = "192.168.1.10"
    server1.port = 8081
    server1.weight = 3
    server1.is_healthy = True

    server2 = DMSCBackendServer()
    server2.id = "server_2"
    server2.host = "192.168.1.11"
    server2.port = 8081
    server2.weight = 2
    server2.is_healthy = True

    server3 = DMSCBackendServer()
    server3.id = "server_3"
    server3.host = "192.168.1.12"
    server3.port = 8081
    server3.weight = 1
    server3.is_healthy = False

    # Create load balancer
    lb = DMSCLoadBalancer()
    lb.strategy = DMSCLoadBalancerStrategy.WEIGHTED_ROUND_ROBIN

    # Add servers
    lb.add_server(server1)
    lb.add_server(server2)
    lb.add_server(server3)

    print(f"Load balancer configured with strategy: {lb.strategy}")
    print(f"Total servers: 3 (2 healthy)")

    # Get next server
    next_server = lb.get_next_server()
    if next_server:
        print(f"Next server: {next_server.id} ({next_server.host}:{next_server.port})")

    # Get load balancer statistics
    lb_stats = DMSCLoadBalancerServerStats()
    print(f"Load balancer stats:")
    print(f"  Active connections: {lb_stats.active_connections}")
    print(f"  Total requests: {lb_stats.total_requests}")

    # Route matching examples
    print("\nTesting route matching...")

    matched_route = router.match("/api/users", "GET")
    if matched_route:
        print(f"Matched route: {matched_route.path}")

    matched_route = router.match("/api/orders", "POST")
    if matched_route:
        print(f"Matched route: {matched_route.path}")

    matched_route = router.match("/health", "GET")
    if matched_route:
        print(f"Matched route: {matched_route.path}")

    print("\nGateway operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
