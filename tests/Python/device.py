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
    DMSCDeviceControlModule,
    DMSCDeviceControlConfig,
    DMSCDeviceSchedulingConfig,
    DMSCDevice,
    DMSCDeviceType,
    DMSCDeviceStatus,
    DMSCDeviceCapabilities,
    DMSCDeviceHealthMetrics,
    DMSCDeviceController,
    DMSCResourceRequest,
    DMSCResourceAllocation,
    DMSCRequestSlaClass,
    DMSCResourceWeights,
    DMSCAffinityRules,
    DMSCResourcePool,
    DMSCResourcePoolConfig,
    DMSCResourcePoolStatistics,
    DMSCResourcePoolManager,
    DMSCResourceScheduler,
    DMSCDeviceScheduler,
    DMSCSchedulingPolicy,
    DMSCAllocationRecord,
    DMSCAllocationRequest,
    DMSCAllocationStatistics,
    DMSCSchedulingRecommendation,
    DMSCSchedulingRecommendationType,
    DMSCDeviceDiscoveryEngine,
)


class TestDMSCDeviceControlModule:
    """Tests for DMSCDeviceControlModule"""

    def test_device_module_creation(self):
        """Test creating device control module"""
        control_config = DMSCDeviceControlConfig()
        scheduling_config = DMSCDeviceSchedulingConfig()

        module = DMSCDeviceControlModule(control_config, scheduling_config)
        assert module is not None


class TestDMSCDevice:
    """Tests for DMSCDevice"""

    def test_device_creation(self):
        """Test creating a device"""
        device = DMSCDevice()
        device.device_id = "device_001"
        device.device_type = DMSCDeviceType.IoT
        device.name = "Test Device"
        device.status = DMSCDeviceStatus.Online

        assert device.device_id == "device_001"
        assert device.device_type == DMSCDeviceType.IoT
        assert device.name == "Test Device"


class TestDMSCDeviceCapabilities:
    """Tests for DMSCDeviceCapabilities"""

    def test_capabilities_creation(self):
        """Test creating device capabilities"""
        caps = DMSCDeviceCapabilities()
        caps.can_read = True
        caps.can_write = True
        caps.supported_protocols = ["MQTT", "HTTP"]
        caps.max_concurrent_connections = 100

        assert caps.can_read is True
        assert caps.can_write is True
        assert "MQTT" in caps.supported_protocols


class TestDMSCDeviceHealthMetrics:
    """Tests for DMSCDeviceHealthMetrics"""

    def test_health_metrics_creation(self):
        """Test creating health metrics"""
        metrics = DMSCDeviceHealthMetrics()
        metrics.cpu_usage_percent = 45.5
        metrics.memory_usage_percent = 60.0
        metrics.network_latency_ms = 25.0
        metrics.is_healthy = True

        assert metrics.cpu_usage_percent == 45.5
        assert metrics.is_healthy is True


class TestDMSCResourceRequest:
    """Tests for DMSCResourceRequest"""

    def test_resource_request_creation(self):
        """Test creating resource request"""
        request = DMSCResourceRequest()
        request.request_id = "req_001"
        request.cpu_cores = 2
        request.memory_mb = 4096
        request.storage_gb = 50
        request.sla_class = DMSCRequestSlaClass.Gold

        assert request.request_id == "req_001"
        assert request.cpu_cores == 2
        assert request.sla_class == DMSCRequestSlaClass.Gold


class TestDMSCResourcePool:
    """Tests for DMSCResourcePool"""

    def test_resource_pool_creation(self):
        """Test creating resource pool"""
        config = DMSCResourcePoolConfig()
        config.pool_name = "test_pool"
        config.max_devices = 100

        pool = DMSCResourcePool(config)
        assert pool is not None


class TestDMSCSchedulingPolicy:
    """Tests for DMSCSchedulingPolicy"""

    def test_scheduling_policies(self):
        """Test scheduling policy types"""
        assert DMSCSchedulingPolicy.BestFit is not None
        assert DMSCSchedulingPolicy.WorstFit is not None
        assert DMSCSchedulingPolicy.FirstFit is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
