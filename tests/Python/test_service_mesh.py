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
DMSC Service Mesh Module Tests

Tests for the service mesh functionality including service discovery,
health checking, traffic management, and load balancing integration.
"""

import pytest
from dmsc import (
    DMSCServiceMesh,
    DMSCServiceMeshConfig,
    DMSCServiceEndpoint,
    DMSCServiceHealthStatus,
    DMSCServiceMeshStats,
    DMSCServiceDiscovery,
    DMSCServiceInstance,
    DMSCServiceStatus,
    DMSCHealthChecker,
    DMSCHealthCheckResult,
    DMSCHealthSummary,
    DMSCHealthStatus,
    DMSCTrafficManager,
    DMSCTrafficRoute,
    DMSCMatchCriteria,
    DMSCRouteAction,
    DMSCWeightedDestination,
    DMSCCircuitBreaker,
    DMSCCircuitBreakerConfig,
    DMSCLoadBalancer,
    DMSCLoadBalancerStrategy,
)


class TestDMSCServiceMesh:
    """Tests for DMSCServiceMesh"""

    def test_service_mesh_creation(self):
        """Test creating service mesh"""
        config = DMSCServiceMeshConfig()
        config.enable_service_discovery = True
        config.enable_health_check = True
        config.enable_traffic_management = True

        mesh = DMSCServiceMesh(config)
        assert mesh is not None

    def test_service_mesh_register_service(self):
        """Test registering service"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        mesh.register_service("user-service", "http://localhost:8080", 100)

    def test_service_mesh_discover_service(self):
        """Test discovering service"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        mesh.register_service("user-service", "http://localhost:8080", 100)
        endpoints = mesh.discover_service("user-service")
        assert endpoints is not None

    def test_service_mesh_get_stats(self):
        """Test getting service mesh stats"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        mesh.register_service("user-service", "http://localhost:8080", 100)
        stats = mesh.get_stats()
        assert stats is not None

    def test_service_mesh_get_circuit_breaker(self):
        """Test getting circuit breaker"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        cb = mesh.get_circuit_breaker()
        assert cb is not None

    def test_service_mesh_get_load_balancer(self):
        """Test getting load balancer"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        lb = mesh.get_load_balancer()
        assert lb is not None

    def test_service_mesh_get_health_checker(self):
        """Test getting health checker"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        hc = mesh.get_health_checker()
        assert hc is not None

    def test_service_mesh_get_traffic_manager(self):
        """Test getting traffic manager"""
        config = DMSCServiceMeshConfig()
        mesh = DMSCServiceMesh(config)

        tm = mesh.get_traffic_manager()
        assert tm is not None


class TestDMSCServiceMeshConfig:
    """Tests for DMSCServiceMeshConfig"""

    def test_service_mesh_config_creation(self):
        """Test creating service mesh configuration"""
        config = DMSCServiceMeshConfig()
        config.enable_service_discovery = True
        config.enable_health_check = True
        config.enable_traffic_management = True
        config.health_check_interval = 30
        config.max_retry_attempts = 3

        assert config.enable_service_discovery is True
        assert config.enable_health_check is True
        assert config.enable_traffic_management is True


class TestDMSCServiceEndpoint:
    """Tests for DMSCServiceEndpoint"""

    def test_service_endpoint_creation(self):
        """Test creating service endpoint"""
        endpoint = DMSCServiceEndpoint()
        endpoint.service_name = "user-service"
        endpoint.endpoint = "http://localhost:8080"
        endpoint.weight = 100

        assert endpoint.service_name == "user-service"
        assert endpoint.endpoint == "http://localhost:8080"
        assert endpoint.weight == 100


class TestDMSCServiceHealthStatus:
    """Tests for DMSCServiceHealthStatus"""

    def test_health_status_values(self):
        """Test health status enum values"""
        assert DMSCServiceHealthStatus.Healthy is not None
        assert DMSCServiceHealthStatus.Unhealthy is not None
        assert DMSCServiceHealthStatus.Unknown is not None


class TestDMSCHealthChecker:
    """Tests for DMSCHealthChecker"""

    def test_health_checker_creation(self):
        """Test creating health checker"""
        hc = DMSCHealthChecker(30)
        assert hc is not None

    def test_health_checker_start(self):
        """Test starting health check"""
        hc = DMSCHealthChecker(30)
        hc.start_health_check("user-service", "http://localhost:8080/health")

    def test_health_checker_get_summary(self):
        """Test getting health summary"""
        hc = DMSCHealthChecker(30)
        summary = hc.get_service_health_summary("user-service")
        assert summary is not None


class TestDMSCHealthSummary:
    """Tests for DMSCHealthSummary"""

    def test_health_summary_properties(self):
        """Test health summary properties"""
        hc = DMSCHealthChecker(30)
        summary = hc.get_service_health_summary("test-service")

        assert summary.get_service_name() == "test-service"
        assert summary.get_total_checks() >= 0
        assert summary.get_success_rate() >= 0.0


class TestDMSCTrafficManager:
    """Tests for DMSCTrafficManager"""

    def test_traffic_manager_creation(self):
        """Test creating traffic manager"""
        tm = DMSCTrafficManager(True)
        assert tm is not None

    def test_traffic_manager_add_route(self):
        """Test adding traffic route"""
        tm = DMSCTrafficManager(True)
        route = DMSCTrafficRoute("api-route", "gateway", "backend")
        tm.add_traffic_route(route)

    def test_traffic_manager_get_routes(self):
        """Test getting traffic routes"""
        tm = DMSCTrafficManager(True)
        route = DMSCTrafficRoute("api-route", "gateway", "backend")
        tm.add_traffic_route(route)

        routes = tm.get_traffic_routes("gateway")
        assert routes is not None


class TestDMSCTrafficRoute:
    """Tests for DMSCTrafficRoute"""

    def test_traffic_route_creation(self):
        """Test creating traffic route"""
        route = DMSCTrafficRoute("api-route", "gateway", "backend")

        assert route.get_name() == "api-route"
        assert route.get_source_service() == "gateway"
        assert route.get_destination_service() == "backend"


class TestDMSCMatchCriteria:
    """Tests for DMSCMatchCriteria"""

    def test_match_criteria_creation(self):
        """Test creating match criteria"""
        criteria = DMSCMatchCriteria()

        assert criteria is not None
        assert criteria.get_path_prefix() is None
        assert criteria.get_method() is None


class TestDMSCWeightedDestination:
    """Tests for DMSCWeightedDestination"""

    def test_weighted_destination_creation(self):
        """Test creating weighted destination"""
        dest = DMSCWeightedDestination("backend-v1", 80)

        assert dest.get_service() == "backend-v1"
        assert dest.get_weight() == 80


class TestDMSCServiceDiscovery:
    """Tests for DMSCServiceDiscovery"""

    def test_service_discovery_creation(self):
        """Test creating service discovery"""
        sd = DMSCServiceDiscovery(True)
        assert sd is not None


class TestDMSCServiceInstance:
    """Tests for DMSCServiceInstance"""

    def test_service_instance_creation(self):
        """Test creating service instance"""
        instance = DMSCServiceInstance()
        instance.service_name = "user-service"
        instance.instance_id = "instance-1"
        instance.address = "192.168.1.10"
        instance.port = 8080

        assert instance.service_name == "user-service"
        assert instance.instance_id == "instance-1"


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
