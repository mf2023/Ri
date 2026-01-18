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
- Device discovery and registration
- Device type management
- Resource allocation and scheduling
- Device status monitoring
- Resource pool management
"""

import dmsc
from dmsc.device import (
    DMSCDeviceControlModule, DMSCDeviceControlConfig,
    DMSCDevice, DMSCDeviceType, DMSCDeviceStatus, DMSCDeviceCapabilities,
)
import asyncio


async def main():
    """
    Main async entry point for the device management module example.
    
    This function demonstrates the complete device management workflow including:
    - Device module initialization and configuration
    - Device registration with various types (sensors, cameras, actuators)
    - Device capabilities definition and assignment
    - Device listing and filtering by type
    - Device status updates and monitoring
    - Device discovery simulation
    - Resource allocation requests
    - Device deregistration
    
    The example shows how DMSC handles IoT device management with support
    for different device types, capabilities, and lifecycle management.
    """
    print("=== DMSC Device Management Module Example ===\n")
    
    # Configuration Setup: Create device control module configuration
    # Parameters:
    # - scan_interval_secs: How often to scan for new devices (60 seconds)
    # - max_concurrent_operations: Maximum parallel device operations (10)
    # - connection_timeout_secs: Timeout for device connections (30 seconds)
    config = DMSCDeviceControlConfig(
        scan_interval_secs=60,
        max_concurrent_operations=10,
        connection_timeout_secs=30,
    )
    
    # Module Initialization: Create device control module instance
    # The module provides device management capabilities including
    # registration, discovery, status monitoring, and resource allocation
    print("1. Creating device control module...")
    device_module = DMSCDeviceControlModule(config)
    print("   Device control module initialized\n")
    
    # Step 2: Register sensor devices
    # Demonstrates device registration with specific capabilities
    # Sensors typically collect data from environment (temperature, humidity, etc.)
    print("2. Registering sensors...")
    
    # Define sensor device capabilities
    # Capabilities describe what operations a device can perform
    # - operations: List of supported operations (read, calibrate, etc.)
    # - max_sampling_rate: Maximum data points per second (100 Hz)
    # - supports_batching: Whether device can batch data samples
    sensor_caps = DMSCDeviceCapabilities(
        operations=["read_temperature", "read_humidity", "calibrate"],
        max_sampling_rate=100.0,
        supports_batching=True,
    )
    
    # Register multiple sensor devices with sequential IDs
    # Each device gets unique ID, type classification, status, and capabilities
    for i in range(1, 4):
        device = DMSCDevice(
            id=f"sensor-{i:03d}",
            device_type=DMSCDeviceType.SENSOR,
            status=DMSCDeviceStatus.ONLINE,
            capabilities=sensor_caps,
        )
        await device_module.register_device(device)
        print(f"   Registered sensor-{i:03d}")
    print()
    
    # Step 3: Register camera devices
    # Demonstrates different device type with specific capabilities
    # Cameras typically capture video/images and support streaming
    print("3. Registering cameras...")
    
    # Define camera device capabilities
    # - operations: Capture, stream, record, pan/tilt controls
    # - supports_streaming: Real-time video streaming capability
    # - max_resolution: Maximum video resolution supported
    camera_caps = DMSCDeviceCapabilities(
        operations=["capture", "stream", "record", "pan_tilt"],
        supports_streaming=True,
        max_resolution="4K",
    )
    
    # Register multiple camera devices
    for i in range(1, 3):
        device = DMSCDevice(
            id=f"camera-{i:02d}",
            device_type=DMSCDeviceType.CAMERA,
            status=DMSCDeviceStatus.ONLINE,
            capabilities=camera_caps,
        )
        await device_module.register_device(device)
        print(f"   Registered camera-{i:02d}")
    print()
    
    # Step 4: Register actuator devices
    # Demonstrates actuator device type for physical control
    # Actuators perform actions based on commands (motor control, valves, etc.)
    print("4. Registering actuators...")
    
    # Define actuator device capabilities
    # - operations: activate, deactivate, position control, status queries
    # - supports_feedback: Can report back position/state information
    actuator_caps = DMSCDeviceCapabilities(
        operations=["activate", "deactivate", "set_position", "get_status"],
        supports_feedback=True,
    )
    
    # Register a single actuator device
    actuator = DMSCDevice(
        id="actuator-01",
        device_type=DMSCDeviceType.ACTUATOR,
        status=DMSCDeviceStatus.ONLINE,
        capabilities=actuator_caps,
    )
    await device_module.register_device(actuator)
    print("   Registered actuator-01\n")
    
    # Step 5: List all registered devices
    # Demonstrates device enumeration and metadata access
    print("5. Listing all devices...")
    devices = await device_module.list_devices()
    print(f"   Total devices: {len(devices)}")
    for device in devices:
        # Access device properties:
        # - id(): Unique device identifier
        # - device_type(): Classification (SENSOR, CAMERA, ACTUATOR, etc.)
        # - status(): Current operational status
        # - capabilities(): Device capability object
        caps_count = len(device.capabilities().operations) if device.capabilities() else 0
        print(f"   - {device.id()}: type={device.device_type()}, "
              f"status={device.status()}, caps={caps_count}")
    print()
    
    # Step 6: Filter devices by type
    # Demonstrates device querying with type-based filtering
    print("6. Filtering devices by type...")
    sensors = await device_module.get_devices_by_type(DMSCDeviceType.SENSOR)
    print(f"   Found {len(sensors)} sensor(s):")
    for sensor in sensors:
        print(f"   - {sensor.id()}")
    print()
    
    # Step 7: Get device by ID
    # Demonstrates individual device lookup
    print("7. Getting device by ID...")
    device = await device_module.get_device("sensor-001")
    if device:
        print(f"   Found device: {device.id()} (type: {device.device_type()})")
    else:
        print("   Device not found")
    print()
    
    # Step 8: Update device status
    # Demonstrates status management for devices
    # Status changes can indicate: maintenance, offline, error, etc.
    print("8. Updating device status...")
    await device_module.update_device_status("camera-01", DMSCDeviceStatus.MAINTENANCE)
    print("   Set camera-01 to Maintenance status\n")
    
    # Step 9: Device discovery simulation
    # Demonstrates automatic device detection capabilities
    # Discovery can find new devices, detect removed devices, and update existing ones
    print("9. Device discovery simulation...")
    discovery_result = await device_module.discover_devices()
    print("   Discovery completed:")
    print(f"   - New devices found: {len(discovery_result.new_devices())}")
    print(f"   - Updated devices: {len(discovery_result.updated_devices())}")
    print(f"   - Removed devices: {len(discovery_result.removed_devices())}\n")
    
    # Step 10: Get device statistics
    # Demonstrates aggregate device monitoring and reporting
    print("10. Getting device statistics...")
    stats = await device_module.get_statistics()
    print("   Device statistics:")
    print(f"   - Total devices: {stats.total_devices()}")
    print(f"   - Online devices: {stats.online_devices()}")
    print(f"   - Offline devices: {stats.offline_devices()}")
    print(f"   - Maintenance devices: {stats.maintenance_devices()}")
    print()
    
    # Step 11: Create resource allocation request
    # Demonstrates resource scheduling for device usage
    # Used when applications need specific device resources
    print("11. Creating resource request...")
    request = await device_module.create_resource_request(
        request_id="high-performance-sensor",
        device_type=DMSCDeviceType.SENSOR,
        count=4,
        requirements=["high_precision", "fast_sampling"],
    )
    print(f"   Resource request created: {request.request_id()}\n")
    
    # Step 12: Deregister a device
    # Demonstrates device removal from management
    print("12. Deregistering test device...")
    await device_module.deregister_device("actuator-01")
    print("   Deregistered actuator-01\n")
    
    # Step 13: Verify final device count
    # Demonstrates device count after removal
    print("13. Final device count...")
    devices = await device_module.list_devices()
    print(f"   Remaining devices: {len(devices)}\n")
    
    print("=== Device Management Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
