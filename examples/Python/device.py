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
DMSC Device Module Example

This example demonstrates how to use the DMSC device module for device
management, discovery, and resource scheduling.
"""

import asyncio
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
    DMSCDeviceConfig,
    DMSCNetworkDeviceInfo,
    DMSCDiscoveryResult,
    DMSCResourceRequest,
    DMSCResourceAllocation,
    DMSCRequestSlaClass,
    DMSCResourceWeights,
    DMSCAffinityRules,
    DMSCResourcePoolStatus,
    DMSCResourcePool,
    DMSCResourcePoolConfig,
    DMSCResourcePoolStatistics,
    DMSCResourcePoolManager,
    DMSCConnectionPoolStatistics,
    DMSCResourceScheduler,
    DMSCDeviceScheduler,
    DMSCSchedulingPolicy,
    DMSCAllocationRecord,
    DMSCAllocationRequest,
    DMSCAllocationStatistics,
    DMSCDeviceTypeStatistics,
    DMSCSchedulingRecommendation,
    DMSCSchedulingRecommendationType,
    DMSCDeviceDiscoveryEngine,
)


async def main():
    # Create device control configuration
    control_config = DMSCDeviceControlConfig()
    control_config.enable_discovery = True
    control_config.discovery_interval_seconds = 60
    control_config.enable_health_check = True
    control_config.health_check_interval_seconds = 30

    # Create scheduling configuration
    scheduling_config = DMSCDeviceSchedulingConfig()
    scheduling_config.default_policy = DMSCSchedulingPolicy.BEST_FIT
    scheduling_config.enable_preemption = False
    scheduling_config.max_scheduling_latency_ms = 100

    # Initialize device control module
    device_module = DMSCDeviceControlModule(control_config, scheduling_config)

    # Create device discovery engine
    discovery_engine = DMSCDeviceDiscoveryEngine()

    # Create devices
    print("Creating devices...")

    # IoT Device
    iot_device = DMSCDevice()
    iot_device.device_id = "iot_sensor_001"
    iot_device.device_type = DMSCDeviceType.IOT
    iot_device.name = "Temperature Sensor"
    iot_device.status = DMSCDeviceStatus.ONLINE

    iot_capabilities = DMSCDeviceCapabilities()
    iot_capabilities.can_read = True
    iot_capabilities.can_write = False
    iot_capabilities.supported_protocols = ["MQTT", "HTTP"]
    iot_capabilities.max_concurrent_connections = 100
    iot_device.capabilities = iot_capabilities

    iot_health = DMSCDeviceHealthMetrics()
    iot_health.cpu_usage_percent = 15.5
    iot_health.memory_usage_percent = 30.2
    iot_health.network_latency_ms = 25.0
    iot_health.is_healthy = True
    iot_device.health = iot_health

    # Edge Device
    edge_device = DMSCDevice()
    edge_device.device_id = "edge_gateway_001"
    edge_device.device_type = DMSCDeviceType.EDGE
    edge_device.name = "Edge Gateway"
    edge_device.status = DMSCDeviceStatus.ONLINE

    edge_capabilities = DMSCDeviceCapabilities()
    edge_capabilities.can_read = True
    edge_capabilities.can_write = True
    edge_capabilities.supported_protocols = ["HTTP", "gRPC", "WebSocket"]
    edge_capabilities.max_concurrent_connections = 1000
    edge_device.capabilities = edge_capabilities

    edge_health = DMSCDeviceHealthMetrics()
    edge_health.cpu_usage_percent = 45.0
    edge_health.memory_usage_percent = 60.5
    edge_health.network_latency_ms = 10.0
    edge_health.is_healthy = True
    edge_device.health = edge_health

    print(f"Created {DMSCDeviceType.IOT} device: {iot_device.name}")
    print(f"Created {DMSCDeviceType.EDGE} device: {edge_device.name}")

    # Create device controller
    controller = DMSCDeviceController()

    # Register devices
    print("\nRegistering devices...")
    controller.register_device(iot_device)
    controller.register_device(edge_device)

    # Get device status
    print("\nDevice Status:")
    iot_status = controller.get_device_status("iot_sensor_001")
    edge_status = controller.get_device_status("edge_gateway_001")
    print(f"IoT Sensor: {iot_status}")
    print(f"Edge Gateway: {edge_status}")

    # Create resource scheduler
    scheduler = DMSCResourceScheduler()

    # Create resource request
    print("\nCreating resource request...")
    resource_request = DMSCResourceRequest()
    resource_request.request_id = "req_001"
    resource_request.cpu_cores = 2
    resource_request.memory_mb = 4096
    resource_request.storage_gb = 50
    resource_request.sla_class = DMSCRequestSlaClass.GOLD

    resource_weights = DMSCResourceWeights()
    resource_weights.cpu_weight = 0.4
    resource_weights.memory_weight = 0.3
    resource_weights.storage_weight = 0.2
    resource_weights.network_weight = 0.1
    resource_request.weights = resource_weights

    affinity_rules = DMSCAffinityRules()
    affinity_rules.preferred_devices = ["edge_gateway_001"]
    affinity_rules.anti_affinity_devices = []
    resource_request.affinity = affinity_rules

    # Create allocation request
    allocation_request = DMSCAllocationRequest()
    allocation_request.request = resource_request
    allocation_request.priority = 10
    allocation_request.timeout_ms = 5000

    print(f"Resource request created: {resource_request.request_id}")
    print(f"  CPU: {resource_request.cpu_cores} cores")
    print(f"  Memory: {resource_request.memory_mb} MB")
    print(f"  Storage: {resource_request.storage_gb} GB")
    print(f"  SLA Class: {resource_request.sla_class}")

    # Create resource pool
    print("\nCreating resource pool...")
    pool_config = DMSCResourcePoolConfig()
    pool_config.pool_name = "default_pool"
    pool_config.max_devices = 100
    pool_config.enable_auto_scaling = True

    resource_pool = DMSCResourcePool(pool_config)

    # Add devices to pool
    resource_pool.add_device(iot_device)
    resource_pool.add_device(edge_device)

    # Get pool statistics
    pool_stats = DMSCResourcePoolStatistics()
    print(f"Pool statistics: {pool_stats.total_devices} devices")

    # Create pool manager
    pool_manager = DMSCResourcePoolManager()

    # Create allocation record
    print("\nCreating allocation record...")
    allocation = DMSCResourceAllocation()
    allocation.allocation_id = "alloc_001"
    allocation.device_id = "edge_gateway_001"
    allocation.request_id = "req_001"
    allocation.allocated_at = 0
    allocation.expires_at = 3600

    allocation_record = DMSCAllocationRecord()
    allocation_record.allocation = allocation
    allocation_record.status = "active"

    print(f"Allocation created: {allocation.allocation_id}")
    print(f"  Device: {allocation.device_id}")
    print(f"  Expires at: {allocation.expires_at}")

    # Get allocation statistics
    alloc_stats = DMSCAllocationStatistics()
    print(f"\nAllocation statistics:")
    print(f"  Total allocations: {alloc_stats.total_allocations}")
    print(f"  Active allocations: {alloc_stats.active_allocations}")

    # Create scheduling recommendation
    recommendation = DMSCSchedulingRecommendation()
    recommendation.recommendation_type = DMSCSchedulingRecommendationType.OPTIMAL
    recommendation.device_id = "edge_gateway_001"
    recommendation.confidence = 0.95
    recommendation.reason = "Best resource match"

    print(f"\nScheduling recommendation:")
    print(f"  Type: {recommendation.recommendation_type}")
    print(f"  Device: {recommendation.device_id}")
    print(f"  Confidence: {recommendation.confidence}")

    # Device discovery simulation
    print("\nSimulating device discovery...")
    discovery_result = DMSCDiscoveryResult()
    discovery_result.devices_found = [iot_device, edge_device]
    discovery_result.discovery_duration_ms = 150.0

    print(f"Discovery completed in {discovery_result.discovery_duration_ms}ms")
    print(f"Devices found: {len(discovery_result.devices_found)}")

    print("\nDevice operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
