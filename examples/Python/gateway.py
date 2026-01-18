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
DMSC Gateway Module Example

This example demonstrates how to use the API gateway module in DMSC,
including routing, rate limiting, and circuit breaker patterns.

Features Demonstrated:
- Route configuration and management
- Rate limiting policies
- Circuit breaker patterns
- Load balancing strategies
"""

import dmsc
from dmsc.gateway import DMSCGateway, DMSCRoute, DMSCRouteTarget
import asyncio


def main():
    """
    Main entry point for the gateway module example.
    
    This function demonstrates the complete API gateway workflow including:
    - Gateway module initialization
    - Route configuration and management
    - Route lookup and matching
    - Rate limiting configuration and simulation
    - Circuit breaker configuration and state management
    - Gateway statistics and monitoring
    
    The example shows how DMSC handles API gateway functionality with
    features like request routing, traffic control, and fault tolerance.
    """
    print("=== DMSC Gateway Module Example ===\n")
    
    # Module Initialization: Create gateway instance
    # The gateway provides unified API entry point with:
    # - Request routing to backend services
    # - Rate limiting to prevent abuse
    # - Circuit breaker for fault tolerance
    # - Load balancing across service instances
    gateway = DMSCGateway()
    
    # Get sub-modules for configuration and operations
    # - Router: Manages route definitions and lookup
    # - RateLimiter: Controls request rate limits
    # - CircuitBreaker: Handles fault tolerance
    router = gateway.get_router()
    rate_limiter = gateway.get_rate_limiter()
    circuit_breaker = gateway.get_circuit_breaker()
    
    # Section 1: Route Management
    # Demonstrates API route configuration and registration
    # Routes define how incoming requests are routed to backend services
    print("1. Route Management")
    print("   -----------------")
    
    # Route 1: User API endpoint
    # Supports GET and POST methods
    # Routes to user service at localhost:8081
    router.add_route(DMSCRoute(
        "/api/users",
        ["GET", "POST"],
        DMSCRouteTarget("http://localhost:8081"),
    ))
    
    # Route 2: Product API endpoint
    # Read-only endpoint (GET only)
    # Routes to product service at localhost:8082
    router.add_route(DMSCRoute(
        "/api/products",
        ["GET"],
        DMSCRouteTarget("http://localhost:8082"),
    ))
    
    # Route 3: Admin API endpoint with wildcard
    # Catches all admin sub-paths (* acts as wildcard)
    # Supports full CRUD operations
    # Routes to admin service at localhost:8080
    router.add_route(DMSCRoute(
        "/api/admin/*",
        ["GET", "POST", "PUT", "DELETE"],
        DMSCRouteTarget("http://localhost:8080"),
    ))
    
    # Route 4: Health check endpoint
    # Lightweight endpoint for service health monitoring
    # Routes to health check handler
    router.add_route(DMSCRoute(
        "/health",
        ["GET"],
        DMSCRouteTarget("http://localhost:8080/health"),
    ))
    
    # Retrieve and display all configured routes
    routes = router.get_routes()
    print(f"   Total routes configured: {len(routes)}\n")
    
    # Display route details
    for route in routes:
        print(f"   Route: {route.path()} -> {route.target().url()}")
        print(f"   Methods: {route.methods()}\n")
    
    # Section 2: Route Lookup
    # Demonstrates how the router matches incoming requests to routes
    # Uses path and HTTP method for matching
    print("2. Route Lookup")
    print("   -------------")
    
    # Lookup exact match for user API
    # find_route() searches for matching route by path and method
    user_route = router.find_route("/api/users", "GET")
    if user_route:
        print(f"   ✓ Found route for /api/users (GET)")
        print(f"   Target: {user_route.target().url()}\n")
    else:
        print("   ✗ No route found for /api/users (GET)\n")
    
    # Lookup with wildcard path matching
    # /api/admin/users should match /api/admin/* pattern
    admin_route = router.find_route("/api/admin/users", "GET")
    if admin_route:
        print(f"   ✓ Found route for /api/admin/users (GET)")
        print(f"   Target: {admin_route.target().url()}\n")
    else:
        print("   ✗ No route found for /api/admin/users (GET)\n")
    
    # Section 3: Rate Limiting
    # Demonstrates request rate control to prevent service overload
    # Rate limiting protects backend services from excessive traffic
    print("3. Rate Limiting")
    print("   --------------")
    
    # Get current rate limiter configuration
    # - requests_per_second: Maximum allowed requests per second
    # - burst_size: Maximum burst capacity above normal rate
    config = rate_limiter.get_config()
    print(f"   Rate limiter configuration:")
    print(f"   - Requests per second: {config.requests_per_second()}")
    print(f"   - Burst size: {config.burst_size()}\n")
    
    # Simulate rate-limited requests
    # try_acquire() returns True if request allowed, False if rate limited
    print("   Simulating request rate check...")
    for i in range(1, 6):
        allowed = rate_limiter.try_acquire()
        print(f"   Request {i}: {'✓ Allowed' if allowed else '✗ Rate limited'}")
    print()
    
    # Section 4: Circuit Breaker
    # Demonstrates fault tolerance pattern for handling downstream failures
    # Circuit breaker prevents cascade failures in distributed systems
    print("4. Circuit Breaker")
    print("   ----------------")
    
    # Get circuit breaker configuration
    # - failure_threshold: Number of failures before opening circuit
    # - recovery_timeout: Time to wait before trying again
    breaker_config = circuit_breaker.get_config()
    print(f"   Circuit breaker configuration:")
    print(f"   - Failure threshold: {breaker_config.failure_threshold()}")
    print(f"   - Recovery timeout: {breaker_config.recovery_timeout()}\n")
    
    # Get current circuit breaker status
    # States: CLOSED (normal), OPEN (blocking), HALF_OPEN (testing)
    status = circuit_breaker.get_status()
    print(f"   Circuit breaker status: {status}\n")
    
    # Simulate circuit breaker operations
    # Record successes and failures to demonstrate state changes
    print("   Simulating circuit breaker operations...")
    
    # Record successful operations
    # Successes help close the circuit after it opens
    for _ in range(3):
        circuit_breaker.record_success()
    print("   Recorded 3 successes")
    
    # Record failures
    # Accumulated failures can trigger circuit opening
    for _ in range(2):
        circuit_breaker.record_failure()
    print("   Recorded 2 failures")
    
    # Final status after operations
    final_status = circuit_breaker.get_status()
    print(f"   Final circuit breaker status: {final_status}\n")
    
    # Section 5: Gateway Statistics
    # Demonstrates gateway metrics and monitoring
    # Statistics track request volume, success/failure rates, and performance
    print("5. Gateway Statistics")
    print("   -------------------")
    
    # Get gateway statistics
    stats = gateway.get_stats()
    print(f"   Gateway statistics:")
    print(f"   - Total requests: {stats.total_requests()}")
    print(f"   - Successful requests: {stats.successful_requests()}")
    print(f"   - Failed requests: {stats.failed_requests()}")
    print(f"   - Average response time: {stats.avg_response_time()}\n")
    
    print("=== Gateway Example Completed ===")


if __name__ == "__main__":
    main()
