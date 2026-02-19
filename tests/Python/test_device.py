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
DMSC Device Module Tests

Tests for the device management functionality.
"""

import pytest
from dmsc import (
    DMSCDevice,
    DMSCDeviceType,
    DMSCDeviceStatus,
    DMSCDeviceCapabilities,
    DMSCDeviceHealthMetrics,
)


class TestDMSCDevice:
    """Tests for DMSCDevice"""

    def test_device_creation(self):
        """Test creating a device"""
        device = DMSCDevice("Test Device", DMSCDeviceType.CPU)
        
        assert device.id() is not None
        assert device.name() == "Test Device"
        assert str(device.device_type()) == "CPU"

    def test_device_status(self):
        """Test device status operations"""
        device = DMSCDevice("Test Device", DMSCDeviceType.GPU)
        
        device.set_status(DMSCDeviceStatus.Available)
        assert "Available" in str(device.status())


class TestDMSCDeviceCapabilities:
    """Tests for DMSCDeviceCapabilities"""

    def test_capabilities_creation(self):
        """Test creating device capabilities"""
        caps = DMSCDeviceCapabilities()
        caps.compute_units = 16
        caps.memory_gb = 32.0
        caps.storage_gb = 512.0
        caps.bandwidth_gbps = 10.0

        assert caps.compute_units == 16
        assert caps.memory_gb == 32.0
        assert caps.storage_gb == 512.0
        assert caps.bandwidth_gbps == 10.0


class TestDMSCDeviceHealthMetrics:
    """Tests for DMSCDeviceHealthMetrics"""

    def test_health_metrics_creation(self):
        """Test creating health metrics"""
        metrics = DMSCDeviceHealthMetrics()
        metrics.cpu_usage_percent = 45.5
        metrics.memory_usage_percent = 60.0
        metrics.temperature_celsius = 55.0
        metrics.error_count = 0
        metrics.throughput = 1000

        assert metrics.cpu_usage_percent == 45.5
        assert metrics.memory_usage_percent == 60.0
        assert metrics.temperature_celsius == 55.0


class TestDMSCDeviceType:
    """Tests for DMSCDeviceType enum"""

    def test_device_types(self):
        """Test device type enum values"""
        assert DMSCDeviceType.CPU is not None
        assert DMSCDeviceType.GPU is not None
        assert DMSCDeviceType.Memory is not None
        assert DMSCDeviceType.Storage is not None
        assert DMSCDeviceType.Network is not None
        assert DMSCDeviceType.Custom is not None
        assert DMSCDeviceType.Sensor is not None
        assert DMSCDeviceType.Actuator is not None


class TestDMSCDeviceStatus:
    """Tests for DMSCDeviceStatus enum"""

    def test_device_statuses(self):
        """Test device status enum values"""
        assert DMSCDeviceStatus.Unknown is not None
        assert DMSCDeviceStatus.Available is not None
        assert DMSCDeviceStatus.Busy is not None
        assert DMSCDeviceStatus.Error is not None
        assert DMSCDeviceStatus.Offline is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
