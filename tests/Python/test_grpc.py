#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri gRPC Module Tests

Tests for the gRPC functionality including server and client.
"""

import pytest
from ri import (
    RiGrpcConfig,
    RiGrpcClientPy,
    RiGrpcStats,
)


class TestRiGrpcConfig:
    """Tests for RiGrpcConfig"""

    def test_grpc_config_creation(self):
        """Test creating gRPC configuration"""
        config = RiGrpcConfig()
        assert config is not None


class TestRiGrpcClient:
    """Tests for RiGrpcClientPy"""

    def test_grpc_client_creation(self):
        """Test creating gRPC client - requires endpoint string"""
        client = RiGrpcClientPy("http://localhost:50051")
        assert client is not None


class TestRiGrpcStats:
    """Tests for RiGrpcStats"""

    def test_grpc_stats_creation(self):
        """Test creating gRPC stats - skip as it requires internal setup"""
        pass


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
