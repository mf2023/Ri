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
DMSC gRPC Module Tests

Tests for the gRPC functionality including server, client, and service registry.
"""

import pytest
from dmsc import (
    DMSCGrpcConfig,
    DMSCGrpcStats,
    DMSCGrpcServiceRegistry,
    DMSCGrpcPythonService,
    DMSCGrpcServiceRegistryPy,
    DMSCGrpcServer,
    DMSCGrpcClient,
)


class TestDMSCGrpcConfig:
    """Tests for DMSCGrpcConfig"""

    def test_grpc_config_creation(self):
        """Test creating gRPC configuration"""
        config = DMSCGrpcConfig()
        config.host = "0.0.0.0"
        config.port = 50051
        config.max_concurrent_streams = 100
        config.enable_reflection = True

        assert config.host == "0.0.0.0"
        assert config.port == 50051
        assert config.max_concurrent_streams == 100


class TestDMSCGrpcServer:
    """Tests for DMSCGrpcServer"""

    def test_grpc_server_creation(self):
        """Test creating gRPC server"""
        config = DMSCGrpcConfig()
        server = DMSCGrpcServer(config)
        assert server is not None


class TestDMSCGrpcClient:
    """Tests for DMSCGrpcClient"""

    def test_grpc_client_creation(self):
        """Test creating gRPC client"""
        config = DMSCGrpcConfig()
        config.host = "localhost"
        config.port = 50051

        client = DMSCGrpcClient(config)
        assert client is not None


class TestDMSCGrpcServiceRegistryPy:
    """Tests for DMSCGrpcServiceRegistryPy"""

    def test_registry_creation(self):
        """Test creating service registry"""
        registry = DMSCGrpcServiceRegistryPy()
        assert registry is not None


class TestDMSCGrpcPythonService:
    """Tests for DMSCGrpcPythonService"""

    def test_python_service_creation(self):
        """Test creating Python gRPC service"""
        service = DMSCGrpcPythonService()
        service.service_name = "TestService"
        service.methods = ["Method1", "Method2"]
        service.proto_file = "test.proto"

        assert service.service_name == "TestService"
        assert len(service.methods) == 2


class TestDMSCGrpcStats:
    """Tests for DMSCGrpcStats"""

    def test_grpc_stats_creation(self):
        """Test creating gRPC statistics"""
        stats = DMSCGrpcStats()
        stats.total_requests = 1000
        stats.active_connections = 10
        stats.average_latency_ms = 25.5

        assert stats.total_requests == 1000
        assert stats.active_connections == 10


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
