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
DMSC Protocol Module Tests

Tests for the protocol functionality including frame handling and connection management.
"""

import pytest
from dmsc import (
    DMSCProtocolManager,
    DMSCProtocolType,
    DMSCProtocolConfig,
    DMSCProtocolStatus,
    DMSCProtocolStats,
    DMSCConnectionState,
    DMSCConnectionStats,
    DMSCProtocolHealth,
    DMSCFrame,
    DMSCFrameHeader,
    DMSCFrameType,
    DMSCConnectionInfo,
    DMSCSecurityLevel,
)


class TestDMSCProtocolManager:
    """Tests for DMSCProtocolManager"""

    def test_protocol_manager_creation(self):
        """Test creating protocol manager"""
        config = DMSCProtocolConfig()
        manager = DMSCProtocolManager(config)
        assert manager is not None


class TestDMSCProtocolConfig:
    """Tests for DMSCProtocolConfig"""

    def test_protocol_config_creation(self):
        """Test creating protocol configuration"""
        config = DMSCProtocolConfig()
        config.default_protocol = DMSCProtocolType.Private
        config.enable_security = True
        config.security_level = DMSCSecurityLevel.High
        config.enable_state_sync = True
        config.performance_optimization = True

        assert config.default_protocol == DMSCProtocolType.Private
        assert config.enable_security is True
        assert config.security_level == DMSCSecurityLevel.High


class TestDMSCFrame:
    """Tests for DMSCFrame"""

    def test_frame_creation(self):
        """Test creating frame"""
        header = DMSCFrameHeader()
        header.frame_type = DMSCFrameType.Data
        header.sequence_number = 1

        frame = DMSCFrame()
        frame.header = header
        frame.payload = b"test data"
        frame.source_id = "source_device"
        frame.target_id = "target_device"

        assert frame.header.frame_type == DMSCFrameType.Data
        assert frame.payload == b"test data"


class TestDMSCFrameHeader:
    """Tests for DMSCFrameHeader"""

    def test_frame_header_creation(self):
        """Test creating frame header"""
        header = DMSCFrameHeader()
        header.frame_type = DMSCFrameType.Data
        header.version = 1
        header.sequence_number = 1
        header.flags = 0
        header.length = 100

        assert header.frame_type == DMSCFrameType.Data
        assert header.version == 1
        assert header.sequence_number == 1


class TestDMSCConnectionInfo:
    """Tests for DMSCConnectionInfo"""

    def test_connection_info_creation(self):
        """Test creating connection info"""
        conn_info = DMSCConnectionInfo()
        conn_info.connection_id = "conn_001"
        conn_info.device_id = "device_001"
        conn_info.address = "192.168.1.100:8080"
        conn_info.protocol_type = DMSCProtocolType.Global
        conn_info.state = DMSCConnectionState.Connected
        conn_info.security_level = DMSCSecurityLevel.High

        assert conn_info.connection_id == "conn_001"
        assert conn_info.state == DMSCConnectionState.Connected


class TestDMSCConnectionStats:
    """Tests for DMSCConnectionStats"""

    def test_connection_stats_creation(self):
        """Test creating connection statistics"""
        stats = DMSCConnectionStats()
        stats.total_connections = 10
        stats.active_connections = 5
        stats.bytes_sent = 1024000
        stats.bytes_received = 980000
        stats.connection_duration_secs = 3600

        assert stats.total_connections == 10
        assert stats.active_connections == 5


class TestDMSCProtocolStats:
    """Tests for DMSCProtocolStats"""

    def test_protocol_stats_creation(self):
        """Test creating protocol statistics"""
        stats = DMSCProtocolStats()
        stats.messages_sent = 1000
        stats.messages_received = 950
        stats.bytes_sent = 1024000
        stats.bytes_received = 980000
        stats.errors = 5
        stats.avg_latency_ms = 25.0

        assert stats.messages_sent == 1000
        assert stats.messages_received == 950


class TestDMSCProtocolHealth:
    """Tests for DMSCProtocolHealth"""

    def test_protocol_health_creation(self):
        """Test creating protocol health"""
        health = DMSCProtocolHealth.Healthy
        assert health == DMSCProtocolHealth.Healthy

if __name__ == "__main__":
    pytest.main([__file__, "-v"])
