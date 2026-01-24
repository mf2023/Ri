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
DMSC Device Management Module Example

This example demonstrates how to use the device management module in DMSC,
including device discovery, resource allocation, and device control.

Features Demonstrated:
- Device discovery
- Resource allocation and scheduling
- Device status monitoring
- Resource pool management
"""

from dmsc import (
    DMSCDeviceControlModule, DMSCDeviceControlConfig,
    DMSCDevice, DMSCDeviceType, DMSCDeviceStatus, DMSCDeviceCapabilities,
    DMSCResourceRequest, DMSCResourceAllocation, DMSCRequestSlaClass,
    DMSCResourceWeights, DMSCAffinityRules
)
import asyncio


async def main():
    """
    Main async entry point for the device management module example.
    
    This function demonstrates the complete device management workflow including:
    - Device module initialization and configuration
    - Device discovery
    - Resource allocation requests
    - Device status monitoring
    - Resource pool management
    
    The example shows how DMSC handles IoT device management with support
    for different device types, capabilities, and lifecycle management.
    """
    print("=== DMSC Device Management Module Example ===\n")
    
    print("1. Creating device control module...")
    device_module = DMSCDeviceControlModule.new()
    print("   Device control module initialized\n")
    
    print("2. Discovering devices...")
    try:
        discovery_result = await device_module.discover_devices()
        print(f"   Discovered {len(discovery_result.discovered_devices_impl())} new devices")
        print(f"   Total devices: {discovery_result.total_devices_impl()}\n")
    except Exception as e:
        print(f"   Discovery failed: {e}\n")
    
    print("3. Getting device status...")
    try:
        devices = await device_module.get_device_status()
        print(f"   Total devices: {len(devices)}")
        for device in devices:
            print(f"   - {device.id()}: type={device.device_type()}, status={device.status()}")
        print()
    except Exception as e:
        print(f"   Failed to get device status: {e}\n")
    
    print("4. Creating resource request...")
    resource_request = DMSCResourceRequest(
        request_id="request-123",
        device_type=DMSCDeviceType.Compute,
        required_capabilities=DMSCDeviceCapabilities(
            cpu_cores=4,
            memory_gb=8.0,
            storage_gb=100.0,
            gpu_enabled=True,
            network_speed_mbps=1000.0,
        ),
        priority=5,
        timeout_secs=30,
    )
    print(f"   Resource request created: {resource_request.request_id()}\n")
    
    print("5. Allocating resource...")
    try:
        allocation = await device_module.allocate_resource(resource_request)
        if allocation:
            print(f"   Allocated device: {allocation.device_name()} (ID: {allocation.device_id()})")
            print(f"   Allocation ID: {allocation.allocation_id()}")
            print(f"   Expires at: {allocation.expires_at()}\n")
            
            print("6. Releasing resource...")
            try:
                await device_module.release_resource(allocation.allocation_id())
                print("   Resource released successfully\n")
            except Exception as e:
                print(f"   Failed to release resource: {e}\n")
        else:
            print("   No suitable device found for allocation\n")
    except Exception as e:
        print(f"   Allocation failed: {e}\n")
    
    print("7. Getting resource pool status...")
    pool_status = device_module.get_resource_pool_status()
    print("   Resource pool status:")
    for name, status in pool_status.items():
        print(f"   - {name}: total={status.total_capacity}, available={status.available_capacity}, utilization={status.utilization_rate * 100.0:.1f}%")
    print()
    
    print("=== Device Management Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
