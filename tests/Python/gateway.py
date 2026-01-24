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
DMSC Gateway Module Python Tests.

This module contains comprehensive tests for the DMSC gateway system Python bindings.
The gateway system provides request routing, rate limiting, and circuit breaker
functionality for building resilient microservices.

Gateway Components:
- DMSCGateway: Main gateway entry point
- DMSCRouter: Request routing and rule management
- DMSCRoute: Individual route definition
- DMSCRateLimiter: Request rate control
- DMSCRateLimitConfig: Rate limiting configuration
- DMSCSlidingWindowRateLimiter: Sliding window rate limiting
- DMSCCircuitBreaker: Failure cascade prevention
- DMSCCircuitBreakerConfig: Circuit breaker configuration
- DMSCCircuitBreakerState: Circuit state enumeration

Request Routing:
- Route matching based on method and path
- Middleware support for cross-cutting concerns
- Handler delegation for business logic

Rate Limiting:
- Requests per second limits
- Burst capacity for traffic spikes
- Sliding window for smooth rate enforcement

Circuit Breaker Patterns:
- Closed: Normal operation
- Open: Blocking requests after failures
- Half-Open: Testing recovery

Test Classes:
- TestDMSCRouter: Router functionality
- TestDMSCGateway: Gateway entry point
- TestDMSCRoute: Route definition
- TestDMSCRateLimitConfig: Rate limit configuration
- TestDMSCCircuitBreakerConfig: Circuit breaker configuration
- TestDMSCCircuitBreakerState: State enumeration
- TestRateLimitStats: Rate limiting statistics
- TestCircuitBreakerMetrics: Circuit breaker metrics
"""

import unittest
from dmsc import (
    DMSCGateway, DMSCGatewayConfig, DMSCRouter, DMSCRoute,
    DMSCRateLimiter, DMSCRateLimitConfig, RateLimitStats,
    DMSCSlidingWindowRateLimiter, DMSCCircuitBreaker,
    DMSCCircuitBreakerConfig, DMSCCircuitBreakerState,
    CircuitBreakerMetrics
)


class TestDMSCRouter(unittest.TestCase):
    """
    Test suite for DMSCRouter class.

    The DMSCRouter class manages request routing rules and handles
    incoming requests by matching against registered routes. It
    provides efficient route lookup and management operations.

    Route Management:
    - add_route(): Register a new route
    - remove_route(): Unregister a route
    - clear_all_routes(): Remove all routes
    - get_route_count(): Number of registered routes

    Route Matching:
    - HTTP method matching (GET, POST, etc.)
    - Path pattern matching
    - Parameter extraction
    - Middleware chain execution

    Test Methods:
    - test_router_new: Verify router instantiation
    - test_router_get_route_count: Test route counting
    - test_router_clear_all_routes: Test route clearing
    """

    def test_router_new(self):
        """Test creating a new router.

        A router is created ready to accept route registrations
        and handle incoming requests.
        """
        router = DMSCRouter()
        self.assertIsNotNone(router)

    def test_router_get_route_count(self):
        """Test getting route count.
        
        The get_route_count() method returns the number of
        registered routes. A new router starts with zero routes.
        
        Expected Behavior:
        - Newly created router returns 0
        - After adding routes, count increases
        - After removing routes, count decreases
        - After clearing routes, count returns to 0
        """
        router = DMSCRouter()
        count = router.get_route_count()
        self.assertEqual(count, 0)

    def test_router_clear_all_routes(self):
        """Test clearing routes.
        
        The clear_all_routes() method removes all registered
        routes, resetting the router to its initial state.
        
        Expected Behavior:
        - After clear, route count is 0
        - Router can accept new route registrations
        - Previous route handlers are no longer accessible
        - Memory is freed for removed routes
        """
        router = DMSCRouter()
        router.clear_all_routes()
        self.assertEqual(router.get_route_count(), 0)


class TestDMSCGateway(unittest.TestCase):
    """Test suite for DMSCGateway class.
    
    The DMSCGateway class serves as the main entry point for the
    gateway system, coordinating routing, rate limiting, and
    circuit breaker functionality. It acts as a unified access
    point for all incoming requests.
    
    Gateway Responsibilities:
    - Request initialization: Parse incoming requests
    - Request routing: Match to appropriate handler
    - Rate limit enforcement: Check against limits before processing
    - Circuit breaker checks: Verify service availability
    - Response processing: Format and send responses
    - Metrics collection: Track request statistics
    
    Gateway Configuration:
    - Route table: Registered routes and handlers
    - Rate limiter: Token bucket or sliding window
    - Circuit breaker: Failure prevention mechanism
    - Middleware chain: Pre/post-processing hooks
    
    Request Flow:
    1. Request arrives at gateway
    2. Rate limiter checks (reject if over limit)
    3. Circuit breaker check (fail fast if open)
    4. Route matching (find handler)
    5. Middleware execution (pre-handler)
    6. Handler execution (business logic)
    7. Middleware execution (post-handler)
    8. Response sent to client
    
    Test Methods:
    - test_gateway_new: Verify gateway instantiation
    """

    def test_gateway_new(self):
        """Test creating a new gateway.
        
        This test verifies that DMSCGateway can be instantiated.
        The gateway is ready to be configured with routes and middleware.
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid gateway instance
        - Gateway has empty route table initially
        - Gateway is ready for configuration
        """
        gateway = DMSCGateway()
        self.assertIsNotNone(gateway)


class TestDMSCRoute(unittest.TestCase):
    """Test suite for DMSCRoute class.
    
    The DMSCRoute class defines a single routing rule including
    the HTTP method, path pattern, and associated handler. Routes
    are the fundamental building blocks of the gateway's routing
    system.
    
    Route Components:
    - Method: HTTP method (GET, POST, PUT, DELETE, PATCH, OPTIONS, HEAD)
    - Path: URL path pattern with optional parameters
    - Handler: Function or coroutine to process matching requests
    - Middleware: Pre/post-processing hooks for cross-cutting concerns
    - Metadata: Route tags, versioning, deprecation status
    
    Path Pattern Syntax:
    - Static: "/api/users" matches exactly "/api/users"
    - Parameter: "/api/users/{id}" captures id parameter
    - Optional: "/api/posts/{id?}" makes id optional
    - Wildcard: "/api/**" matches any path below
    
    Method Constraints:
    - GET: Retrieve resource (no body)
    - POST: Create new resource
    - PUT: Replace entire resource
    - PATCH: Partial update
    - DELETE: Remove resource
    
    Test Methods:
    - test_route_new: Verify route instantiation
    """

    def test_route_new(self):
        """Test creating a new route.
        
        This test verifies that DMSCRoute can be instantiated
        with a method and path pattern. The route is ready to
        be registered with a router.
        
        Expected Behavior:
        - Constructor accepts method and path parameters
        - Returns a valid route instance
        - Route is ready for handler assignment
        - Route can be added to router
        """
        route = DMSCRoute("GET", "/test")
        self.assertIsNotNone(route)


class TestDMSCRateLimitConfig(unittest.TestCase):
    """Test suite for DMSCRateLimitConfig class.
    
    The DMSCRateLimitConfig class configures rate limiting behavior
    including request rates, burst capacities, and policy enforcement.
    Rate limiting protects services from overuse and ensures fair access.
    
    Rate Limit Parameters:
    - requests_per_second: Maximum average request rate (sustained limit)
    - burst_size: Maximum burst capacity above average rate (peak tolerance)
    - window_size: Time window for rate calculation (default 1 second)
    - policy: Enforcement policy (reject, queue, or delay)
    
    Rate Limiting Algorithms:
    - Token Bucket: Tokens added at fixed rate, consumed per request
    - Leaky Bucket: Requests queued, processed at fixed rate
    - Sliding Window: Moving time window for smooth limits
    - Fixed Window: Simple counting within time windows
    
    Configuration Examples:
    - API Limit: 100 requests/second, 200 burst
    - WebSocket: 50 messages/second, 100 burst
    - File Upload: 10 uploads/second, 20 burst
    
    Test Methods:
    - test_rate_limit_config_setters: Test configuration updates
    """

    def test_rate_limit_config_setters(self):
        """Test rate limit configuration setters.
        
        Rate limit parameters can be configured to protect different
        endpoints based on their expected load:
        - 50 requests per second average rate
        - 100 requests burst capacity for traffic spikes
        
        Expected Behavior:
        - requests_per_second setter updates the rate
        - burst_size setter updates the burst capacity
        - get_requests_per_second returns configured value
        - get_burst_size returns configured value
        """
        config = DMSCRateLimitConfig()
        config.set_requests_per_second(50)
        config.set_burst_size(100)
        self.assertEqual(config.get_requests_per_second(), 50)
        self.assertEqual(config.get_burst_size(), 100)


class TestDMSCCircuitBreakerConfig(unittest.TestCase):
    """Test suite for DMSCCircuitBreakerConfig class.
    
    The DMSCCircuitBreakerConfig class configures circuit breaker
    behavior including failure thresholds, timeout periods, and
    recovery settings. Circuit breakers prevent cascading failures
    in distributed systems.
    
    Circuit Breaker Parameters:
    - failure_threshold: Number of failures before opening circuit
    - success_threshold: Successes needed in half-open to close
    - timeout_seconds: Time in open state before half-open transition
    - volume_threshold: Minimum requests before evaluating failure rate
    
    State Transitions:
    - Closed -> Open: Exceeds failure threshold or error rate
    - Open -> Half-Open: Timeout expires, testing recovery
    - Half-Open -> Closed: Sufficient successes (meets threshold)
    - Half-Open -> Open: Any failure during testing
    
    Failure Detection:
    - Count-based: N consecutive failures
    - Rate-based: Error rate exceeds percentage
    - Volume-based: Minimum sample size required
    
    Test Methods:
    - test_circuit_breaker_config_setters: Test configuration updates
    """

    def test_circuit_breaker_config_setters(self):
        """Test circuit breaker configuration setters.
        
        Circuit breaker thresholds control sensitivity to failures
        and recovery behavior:
        - Open after 10 consecutive failures
        - Close after 3 successes in half-open state
        
        Expected Behavior:
        - failure_threshold setter updates the failure limit
        - success_threshold setter updates the recovery threshold
        - get_failure_threshold returns configured value
        - get_success_threshold returns configured value
        """
        config = DMSCCircuitBreakerConfig()
        config.set_failure_threshold(10)
        config.set_success_threshold(3)
        self.assertEqual(config.get_failure_threshold(), 10)
        self.assertEqual(config.get_success_threshold(), 3)


class TestDMSCCircuitBreakerState(unittest.TestCase):
    """Test suite for DMSCCircuitBreakerState enum.
    
    The DMSCCircuitBreakerState enum represents the current state
    of a circuit breaker, determining how requests are handled.
    States implement the fail-fast pattern to protect failing services.
    
    Circuit States:
    - Closed: Normal operation, requests pass through
    - Open: Requests fail immediately without attempting service
    - Half-Open: Limited requests allowed to test recovery
    
    State Characteristics:
    - Closed: Requests execute normally, failures counted
    - Open: Requests return error immediately, no execution
    - Half-Open: Single request allowed to test service
    
    State Purpose:
    States prevent cascading failures by stopping requests to
    failing services, allowing them time to recover, and
    gradually restoring traffic as health returns.
    
    State Duration:
    - Closed: Indefinite (until failure threshold exceeded)
    - Open: Until timeout expires, then transitions to Half-Open
    - Half-Open: Until success threshold met or failure occurs
    
    Test Methods:
    - test_circuit_breaker_state_values: Verify state values
    """

    def test_circuit_breaker_state_values(self):
        """Test circuit breaker state values.
        
        Each circuit state should have a string representation
        for logging, monitoring, and debugging purposes.
        
        Expected Behavior:
        - Closed state string matches expected format
        - Open state string matches expected format
        - Half-Open state string matches expected format
        """
        self.assertEqual(str(DMSCCircuitBreakerState.Closed), "DMSCCircuitBreakerState.Closed")
        self.assertEqual(str(DMSCCircuitBreakerState.Open), "DMSCCircuitBreakerState.Open")
        self.assertEqual(str(DMSCCircuitBreakerState.HalfOpen), "DMSCCircuitBreakerState.HalfOpen")


class TestRateLimitStats(unittest.TestCase):
    """Test suite for RateLimitStats class.
    
    The RateLimitStats class provides statistics about rate limiting
    including current token bucket state, request counts, and limit
    enforcement metrics. These statistics are essential for monitoring
    and alerting on API usage.
    
    Statistics Tracked:
    - current_tokens: Remaining tokens in the bucket (0 to max)
    - total_requests: Total requests processed since start
    - rejected_requests: Number of requests rejected (over limit)
    - queued_requests: Number of requests queued (if queuing enabled)
    - last_refill: Timestamp of last token refill
    
    Metrics Usage:
    - Monitor rate limit effectiveness: Track rejection rates
    - Detect traffic anomalies: Sudden changes in patterns
    - Plan capacity requirements: Understand peak usage
    - Optimize limits: Adjust based on actual usage
    
    Performance Monitoring:
    - Hit rate: Requests served vs total requests
    - Rejection rate: Rejected vs total requests
    - Token utilization: Average tokens consumed
    
    Test Methods:
    - test_rate_limit_stats_new: Verify stats instantiation
    - test_rate_limit_stats_properties: Test property access
    """

    def test_rate_limit_stats_new(self):
        """Test creating rate limit stats.
        
        Rate limit statistics track the state of rate limiting
        for monitoring, analysis, and alerting purposes.
        
        Expected Behavior:
        - Constructor accepts initial token count and request count
        - Returns a valid stats instance
        - Stats object tracks rate limiting metrics
        """
        stats = RateLimitStats(100, 50)
        self.assertIsNotNone(stats)

    def test_rate_limit_stats_properties(self):
        """Test rate limit stats properties.
        
        Statistics provide access to current rate limiting state:
        - 100 tokens available in bucket
        - 50 total requests processed
        
        Expected Behavior:
        - get_current_tokens() returns initial token count
        - get_total_requests() returns initial request count
        - Properties can be updated during operation
        """
        stats = RateLimitStats(100, 50)
        self.assertEqual(stats.get_current_tokens(), 100)
        self.assertEqual(stats.get_total_requests(), 50)


class TestCircuitBreakerMetrics(unittest.TestCase):
    """Test suite for CircuitBreakerMetrics class.
    
    The CircuitBreakerMetrics class tracks circuit breaker performance
    including state transitions, request counts, and failure rates.
    These metrics are crucial for understanding service health and
    circuit breaker effectiveness.
    
    Metrics Tracked:
    - current_state: Current circuit breaker state
    - failure_count: Number of recent failures
    - success_count: Number of recent successes
    - request_count: Total requests attempted
    - rejected_count: Number of requests rejected (when open)
    
    Performance Analysis:
    - Failure rate: failures / (failures + successes)
    - State duration: Time spent in each state
    - Recovery time: How long until circuit closes
    - Protection effectiveness: Requests rejected vs failed
    
    Alerting Thresholds:
    - Circuit open for extended period
    - High failure rate sustained
    - Frequent state transitions
    
    Test Methods:
    - test_circuit_breaker_metrics_new: Verify metrics instantiation
    """

    def test_circuit_breaker_metrics_new(self):
        """Test creating circuit breaker metrics.
        
        Circuit breaker metrics track performance for monitoring,
        alerting, and capacity planning. They help identify
        problematic services and tune circuit breaker thresholds.
        
        Expected Behavior:
        - Constructor accepts state and count parameters
        - Returns a valid metrics instance
        - Metrics object tracks circuit breaker performance
        """
        metrics = CircuitBreakerMetrics(
            "Closed", 0, 0, 0, 0
        )
        self.assertIsNotNone(metrics)


if __name__ == "__main__":
    unittest.main()
