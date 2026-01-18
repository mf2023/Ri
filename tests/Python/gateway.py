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
        registered routes. A new router has zero routes.
        """
        router = DMSCRouter()
        count = router.get_route_count()
        self.assertEqual(count, 0)

    def test_router_clear_all_routes(self):
        """Test clearing routes.

        The clear_all_routes() method removes all registered
        routes, resetting the router to its initial state.
        """
        router = DMSCRouter()
        router.clear_all_routes()
        self.assertEqual(router.get_route_count(), 0)


class TestDMSCGateway(unittest.TestCase):
    """
    Test suite for DMSCGateway class.

    The DMSCGateway class serves as the main entry point for the
    gateway system, coordinating routing, rate limiting, and
    circuit breaker functionality.

    Gateway Responsibilities:
    - Request initialization and routing
    - Rate limit enforcement
    - Circuit breaker checks
    - Response processing

    Test Methods:
    - test_gateway_new: Verify gateway instantiation
    """

    def test_gateway_new(self):
        """Test creating a new gateway.

        A gateway is created with default configuration and
        is ready to accept routes and handle requests.
        """
        gateway = DMSCGateway()
        self.assertIsNotNone(gateway)


class TestDMSCRoute(unittest.TestCase):
    """
    Test suite for DMSCRoute class.

    The DMSCRoute class defines a single routing rule including
    the HTTP method, path pattern, and associated handler.

    Route Components:
    - Method: HTTP method (GET, POST, PUT, DELETE, etc.)
    - Path: URL path pattern with optional parameters
    - Handler: Function to process matching requests
    - Middleware: Pre/post-processing hooks

    Path Patterns:
    - Static: Exact path matching
    - Parameter: Named parameters (/users/:id)
    - Wildcard: Catch-all patterns (/*)

    Test Methods:
    - test_route_new: Verify route instantiation
    """

    def test_route_new(self):
        """Test creating a new route.

        A route is created with method and path, ready to
        be registered with a router.
        """
        route = DMSCRoute("GET", "/test")
        self.assertIsNotNone(route)


class TestDMSCRateLimitConfig(unittest.TestCase):
    """
    Test suite for DMSCRateLimitConfig class.

    The DMSCRateLimitConfig class configures rate limiting behavior
    including request rates and burst capacities.

    Rate Limit Parameters:
    - requests_per_second: Maximum average request rate
    - burst_size: Maximum burst above average rate

    Rate Limiting Strategies:
    - Token Bucket: Smooth rate with burst capacity
    - Leaky Bucket: Constant rate with queue
    - Sliding Window: Moving time window

    Test Methods:
    - test_rate_limit_config_setters: Test configuration updates
    """

    def test_rate_limit_config_setters(self):
        """Test rate limit configuration setters.

        Rate limit parameters can be configured for different
        endpoints and client types:
        - 50 requests per second average
        - 100 requests burst capacity
        """
        config = DMSCRateLimitConfig()
        config.set_requests_per_second(50)
        config.set_burst_size(100)
        self.assertEqual(config.get_requests_per_second(), 50)
        self.assertEqual(config.get_burst_size(), 100)


class TestDMSCCircuitBreakerConfig(unittest.TestCase):
    """
    Test suite for DMSCCircuitBreakerConfig class.

    The DMSCCircuitBreakerConfig class configures circuit breaker
    behavior including failure thresholds and recovery settings.

    Circuit Breaker Parameters:
    - failure_threshold: Failures before opening circuit
    - success_threshold: Successes in half-open to close
    - timeout: Time in open state before half-open

    State Transitions:
    - Closed -> Open: Exceeds failure threshold
    - Open -> Half-Open: Timeout expires
    - Half-Open -> Closed: Sufficient successes
    - Half-Open -> Open: Any failure

    Test Methods:
    - test_circuit_breaker_config_setters: Test configuration updates
    """

    def test_circuit_breaker_config_setters(self):
        """Test circuit breaker configuration setters.

        Circuit breaker thresholds control sensitivity:
        - Open after 10 consecutive failures
        - Close after 3 successes in half-open state
        """
        config = DMSCCircuitBreakerConfig()
        config.set_failure_threshold(10)
        config.set_success_threshold(3)
        self.assertEqual(config.get_failure_threshold(), 10)
        self.assertEqual(config.get_success_threshold(), 3)


class TestDMSCCircuitBreakerState(unittest.TestCase):
    """
    Test suite for DMSCCircuitBreakerState enum.

    The DMSCCircuitBreakerState enum represents the current state
    of a circuit breaker, determining how requests are handled.

    Circuit States:
    - Closed: Normal operation, requests allowed
    - Open: Blocking requests, failing fast
    - Half-Open: Testing recovery, limited requests

    State Purpose:
    States prevent cascading failures by stopping requests to
    failing services and allowing them time to recover.

    Test Methods:
    - test_circuit_breaker_state_values: Verify state values
    """

    def test_circuit_breaker_state_values(self):
        """Test circuit breaker state values.

        All circuit states should have string representations
        for logging and monitoring.
        """
        self.assertEqual(str(DMSCCircuitBreakerState.Closed), "DMSCCircuitBreakerState.Closed")
        self.assertEqual(str(DMSCCircuitBreakerState.Open), "DMSCCircuitBreakerState.Open")
        self.assertEqual(str(DMSCCircuitBreakerState.HalfOpen), "DMSCCircuitBreakerState.HalfOpen")


class TestRateLimitStats(unittest.TestCase):
    """
    Test suite for RateLimitStats class.

    The RateLimitStats class provides statistics about rate limiting
    including current token bucket state and request counts.

    Statistics Tracked:
    - current_tokens: Remaining tokens in bucket
    - total_requests: Total requests processed

    Metrics Usage:
    - Monitor rate limit effectiveness
    - Detect traffic anomalies
    - Plan capacity requirements

    Test Methods:
    - test_rate_limit_stats_new: Verify stats instantiation
    - test_rate_limit_stats_properties: Test property access
    """

    def test_rate_limit_stats_new(self):
        """Test creating rate limit stats.

        Rate limit statistics track the state of rate limiting
        for monitoring and analysis.
        """
        stats = RateLimitStats(100, 50)
        self.assertIsNotNone(stats)

    def test_rate_limit_stats_properties(self):
        """Test rate limit stats properties.

        Statistics provide access to current state:
        - 100 tokens available in bucket
        - 50 total requests processed
        """
        stats = RateLimitStats(100, 50)
        self.assertEqual(stats.get_current_tokens(), 100)
        self.assertEqual(stats.get_total_requests(), 50)


class TestCircuitBreakerMetrics(unittest.TestCase):
    """
    Test suite for CircuitBreakerMetrics class.

    The CircuitBreakerMetrics class tracks circuit breaker performance
    including state transitions and request counts.

    Metrics Tracked:
    - State: Current circuit state
    - Failure count: Recent failures
    - Success count: Recent successes
    - Request count: Total requests

    Performance Analysis:
    Metrics help identify problematic services and tune
    circuit breaker thresholds.

    Test Methods:
    - test_circuit_breaker_metrics_new: Verify metrics instantiation
    """

    def test_circuit_breaker_metrics_new(self):
        """Test creating circuit breaker metrics.

        Circuit breaker metrics track performance for monitoring
        and alerting on system health.
        """
        metrics = CircuitBreakerMetrics(
            "Closed", 0, 0, 0, 0
        )
        self.assertIsNotNone(metrics)


if __name__ == "__main__":
    unittest.main()
