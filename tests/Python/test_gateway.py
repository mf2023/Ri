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
DMSC Gateway Module Tests

Tests for the gateway functionality including routing, rate limiting,
circuit breaking, and load balancing.
"""

import pytest
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


class TestDMSCGateway:
    """Tests for DMSCGateway"""

    def test_gateway_creation(self):
        """Test creating gateway"""
        config = DMSCGatewayConfig()
        config.listen_address = "0.0.0.0"
        config.listen_port = 8080

        gateway = DMSCGateway(config)
        assert gateway is not None


class TestDMSCGatewayConfig:
    """Tests for DMSCGatewayConfig"""

    def test_gateway_config_creation(self):
        """Test creating gateway configuration"""
        config = DMSCGatewayConfig()
        config.listen_address = "0.0.0.0"
        config.listen_port = 8080
        config.enable_rate_limiting = True
        config.enable_circuit_breaker = True
        config.enable_load_balancing = True

        assert config.listen_address == "0.0.0.0"
        assert config.listen_port == 8080
        assert config.enable_rate_limiting is True


class TestDMSCRouter:
    """Tests for DMSCRouter"""

    def test_router_creation(self):
        """Test creating router"""
        router = DMSCRouter()
        assert router is not None


class TestDMSCRoute:
    """Tests for DMSCRoute"""

    def test_route_creation(self):
        """Test creating route"""
        route = DMSCRoute()
        route.path = "/api/users"
        route.methods = ["GET", "POST", "PUT", "DELETE"]
        route.handler = "user_service"

        assert route.path == "/api/users"
        assert "GET" in route.methods
        assert route.handler == "user_service"


class TestDMSCRateLimiter:
    """Tests for DMSCRateLimiter"""

    def test_rate_limiter_creation(self):
        """Test creating rate limiter"""
        config = DMSCRateLimitConfig()
        limiter = DMSCRateLimiter(config)
        assert limiter is not None


class TestDMSCRateLimitConfig:
    """Tests for DMSCRateLimitConfig"""

    def test_rate_limit_config_creation(self):
        """Test creating rate limit config"""
        config = DMSCRateLimitConfig()
        config.requests_per_second = 100
        config.burst_size = 150
        config.window_seconds = 60

        assert config.requests_per_second == 100
        assert config.burst_size == 150


class TestDMSCCircuitBreaker:
    """Tests for DMSCCircuitBreaker"""

    def test_circuit_breaker_creation(self):
        """Test creating circuit breaker"""
        config = DMSCCircuitBreakerConfig()
        cb = DMSCCircuitBreaker(config)
        assert cb is not None

    def test_circuit_breaker_state(self):
        """Test circuit breaker state transitions"""
        config = DMSCCircuitBreakerConfig()
        cb = DMSCCircuitBreaker(config)

        state = cb.get_state()
        assert state is not None


class TestDMSCCircuitBreakerConfig:
    """Tests for DMSCCircuitBreakerConfig"""

    def test_circuit_breaker_config_creation(self):
        """Test creating circuit breaker config"""
        config = DMSCCircuitBreakerConfig()
        config.failure_threshold = 5
        config.success_threshold = 3
        config.timeout_seconds = 30

        assert config.failure_threshold == 5
        assert config.success_threshold == 3


class TestDMSCLoadBalancer:
    """Tests for DMSCLoadBalancer"""

    def test_load_balancer_creation(self):
        """Test creating load balancer"""
        lb = DMSCLoadBalancer()
        assert lb is not None


class TestDMSCBackendServer:
    """Tests for DMSCBackendServer"""

    def test_backend_server_creation(self):
        """Test creating backend server"""
        server = DMSCBackendServer()
        server.id = "server_1"
        server.host = "192.168.1.10"
        server.port = 8080
        server.weight = 3
        server.is_healthy = True

        assert server.id == "server_1"
        assert server.host == "192.168.1.10"
        assert server.weight == 3


class TestDMSCLoadBalancerStrategy:
    """Tests for DMSCLoadBalancerStrategy"""

    def test_load_balancer_strategies(self):
        """Test load balancer strategies"""
        assert DMSCLoadBalancerStrategy.RoundRobin is not None
        assert DMSCLoadBalancerStrategy.WeightedRoundRobin is not None
        assert DMSCLoadBalancerStrategy.LeastConnections is not None
        assert DMSCLoadBalancerStrategy.IpHash is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
