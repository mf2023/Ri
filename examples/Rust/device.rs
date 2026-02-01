//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # DMSC Device Management Module Example
//!
//! This example demonstrates how to use the device management module in DMSC,
//! including device discovery, resource allocation, and device control.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example device
//! ```
//!
//! ## Features Demonstrated
//!
//! - Device discovery and registration
//! - Device type management
//! - Resource allocation and scheduling
//! - Device status monitoring
//! - Resource pool management

use dmsc::device::{DMSCDeviceControlModule, DMSCDeviceControlConfig, DMSCDevice, DMSCDeviceType, DMSCDeviceStatus, DMSCDeviceCapabilities, DMSCResourcePoolStatus};
use dmsc::core::DMSCResult;

/// Main entry point for the device management module example.
///
/// This function demonstrates the complete device management workflow including:
/// - Device module initialization with configuration settings
/// - Device registration with various types (sensors, cameras, actuators)
/// - Device capabilities definition and assignment
/// - Device listing and filtering by type
/// - Device status updates and monitoring
/// - Device discovery simulation
/// - Resource allocation requests
/// - Device deregistration
///
/// The example shows how DMSC handles IoT device management with support
/// for different device types, capabilities, and lifecycle management
/// in a Rust async runtime environment.
fn main() -> DMSCResult<()> {
    println!("=== DMSC Device Management Module Example ===\n");

    // Create async runtime for handling asynchronous device operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute all async device management operations within the runtime
    rt.block_on(async {
        // Configuration Setup: Create device control module configuration
        // Using builder pattern for configuration parameters:
        // - scan_interval_secs: How often to scan for new devices (60 seconds)
        // - max_concurrent_operations: Maximum parallel device operations (10)
        // - connection_timeout_secs: Timeout for device connections (30 seconds)
        // - build(): Finalizes configuration into DMSCDeviceControlConfig struct
        let config = DMSCDeviceControlConfig::new()
            .with_scan_interval_secs(60)
            .with_max_concurrent_operations(10)
            .with_connection_timeout_secs(30)
            .build();

        // Module Initialization: Create device control module instance
        // The module provides device management capabilities including
        // registration, discovery, status monitoring, and resource allocation
        println!("1. Creating device control module...");
        let device_module = DMSCDeviceControlModule::new(config).await?;
        println!("   Device control module initialized\n");

        // Step 2: Register sensor devices
        // Demonstrates device registration with specific capabilities
        // Sensors typically collect data from environment (temperature, humidity, etc.)
        println!("2. Registering sensors...");

        // Define sensor device capabilities
        // Capabilities describe what operations a device can perform
        // Builder pattern for creating capabilities struct:
        // - operations: List of supported operations (read, calibrate, etc.)
        // - max_sampling_rate: Maximum data points per second (100 Hz)
        // - supports_batching: Whether device can batch data samples
        let sensor_caps = DMSCDeviceCapabilities::new()
            .with_operations(vec!["read_temperature", "read_humidity", "calibrate"])
            .with_max_sampling_rate(100.0)
            .with_supports_batching(true)
            .build();
        
        // Register multiple sensor devices with sequential IDs
        // Each device gets unique ID, type classification, status, and capabilities
        // Device creation uses new() with all required fields:
        // - id: Unique identifier formatted with zero-padding (sensor-001, sensor-002, etc.)
        // - device_type: Classification enum (Sensor, Camera, Actuator, etc.)
        // - status: Current operational status (Online, Offline, Maintenance, etc.)
        // - capabilities: Optional device capabilities struct
        for i in 1..=3 {
            let device = DMSCDevice::new(
                format!("sensor-{:03}", i),
                DMSCDeviceType::Sensor,
                DMSCDeviceStatus::Online,
                Some(sensor_caps.clone()),
            );
            // Register device with the control module
            // This makes the device available for discovery and operations
            device_module.register_device(device).await?;
            println!("   Registered sensor-{:03}", i);
        }
        println!();

        // Step 3: Register camera devices
        // Demonstrates different device type with specific capabilities
        // Cameras typically capture video/images and support streaming
        println!("3. Registering cameras...");

        // Define camera device capabilities
        // Camera-specific features:
        // - operations: Capture, stream, record, pan/tilt controls
        // - supports_streaming: Real-time video streaming capability
        // - max_resolution: Maximum video resolution supported (4K)
        let camera_caps = DMSCDeviceCapabilities::new()
            .with_operations(vec!["capture", "stream", "record", "pan_tilt"])
            .with_supports_streaming(true)
            .with_max_resolution("4K")
            .build();
        
        // Register multiple camera devices
        for i in 1..=2 {
            let device = DMSCDevice::new(
                format!("camera-{:02}", i),
                DMSCDeviceType::Camera,
                DMSCDeviceStatus::Online,
                Some(camera_caps.clone()),
            );
            device_module.register_device(device).await?;
            println!("   Registered camera-{:02}", i);
        }
        println!();

        // Step 4: Register actuator devices
        // Demonstrates actuator device type for physical control
        // Actuators perform actions based on commands (motor control, valves, etc.)
        println!("4. Registering actuators...");

        // Define actuator device capabilities
        // Actuator-specific features:
        // - operations: activate, deactivate, position control, status queries
        // - supports_feedback: Can report back position/state information
        let actuator_caps = DMSCDeviceCapabilities::new()
            .with_operations(vec!["activate", "deactivate", "set_position", "get_status"])
            .with_supports_feedback(true)
            .build();
        
        // Register a single actuator device
        let actuator = DMSCDevice::new(
            "actuator-01".to_string(),
            DMSCDeviceType::Actuator,
            DMSCDeviceStatus::Online,
            Some(actuator_caps),
        );
        device_module.register_device(actuator).await?;
        println!("   Registered actuator-01\n");

        // Step 5: List all registered devices
        // Demonstrates device enumeration and metadata access
        println!("5. Listing all devices...");
        let devices = device_module.list_devices().await?;
        println!("   Total devices: {}", devices.len());
        for device in &devices {
            // Access device properties using getter methods:
            // - id(): Unique device identifier
            // - device_type(): Classification (SENSOR, CAMERA, ACTUATOR, etc.)
            // - status(): Current operational status
            // - capabilities(): Optional device capability object
            println!("   - {}: type={:?}, status={:?}, caps={}",
                device.id(),
                device.device_type(),
                device.status(),
                device.capabilities().map(|c| c.operations().len()).unwrap_or(0)
            );
        }
        println!();

        // Step 6: Filter devices by type
        // Demonstrates device querying with type-based filtering
        println!("6. Filtering devices by type...");
        let sensors = device_module.get_devices_by_type(DMSCDeviceType::Sensor).await?;
        println!("   Found {} sensor(s):", sensors.len());
        for sensor in &sensors {
            // Display filtered device IDs
            println!("   - {}\n", sensor.id());
        }

        // Step 7: Get device by ID
        // Demonstrates individual device lookup
        println!("7. Getting device by ID...");
        if let Some(device) = device_module.get_device("sensor-001").await? {
            println!("   Found device: {} (type: {:?})", device.id(), device.device_type());
        } else {
            println!("   Device not found");
        }
        println!();

        // Step 8: Update device status
        // Demonstrates status management for devices
        // Status changes can indicate: maintenance, offline, error, etc.
        println!("8. Updating device status...");
        device_module.update_device_status("camera-01", DMSCDeviceStatus::Maintenance).await?;
        println!("   Set camera-01 to Maintenance status\n");

        // Step 9: Device discovery simulation
        // Demonstrates automatic device detection capabilities
        // Discovery can find new devices, detect removed devices, and update existing ones
        println!("9. Device discovery simulation...");
        let discovery_result = device_module.discover_devices().await?;
        println!("   Discovery completed:");
        println!("   - New devices found: {}", discovery_result.new_devices().len());
        println!("   - Updated devices: {}", discovery_result.updated_devices().len());
        println!("   - Removed devices: {}\n", discovery_result.removed_devices().len());

        // Step 10: Get device statistics
        // Demonstrates aggregate device monitoring and reporting
        println!("10. Getting device statistics...");
        let stats = device_module.get_statistics().await?;
        println!("   Device statistics:");
        println!("   - Total devices: {}", stats.total_devices());
        println!("   - Online devices: {}", stats.online_devices());
        println!("   - Offline devices: {}", stats.offline_devices());
        println!("   - Maintenance devices: {}", stats.maintenance_devices());
        println!();

        // Step 11: Create resource allocation request
        // Demonstrates resource scheduling for device usage
        // Used when applications need specific device resources
        println!("11. Creating resource request...");
        let request = device_module.create_resource_request(
            "high-performance-sensor",
            DMSCDeviceType::Sensor,
            4,
            vec!["high_precision", "fast_sampling"],
        ).await?;
        println!("   Resource request created: {}\n", request.request_id());

        // Step 12: Resource pool status management
        // Demonstrates resource pool monitoring and status tracking
        println!("12. Resource pool status management...");
        let pool_status = DMSCResourcePoolStatus::new(
            100,   // total_capacity
            75,    // available_capacity
            25,    // allocated_capacity
            5,     // pending_requests
            0.25,  // utilization_rate
        );
        println!("   Resource pool status:");
        println!("   - Total capacity: {}", pool_status.total_capacity());
        println!("   - Available capacity: {}", pool_status.available_capacity());
        println!("   - Allocated capacity: {}", pool_status.allocated_capacity());
        println!("   - Pending requests: {}", pool_status.pending_requests());
        println!("   - Utilization rate: {:.2}%\n", pool_status.utilization_rate() * 100.0);

        // Step 13: Deregister a device
        // Demonstrates device removal from management
        println!("13. Deregistering test device...");
        device_module.deregister_device("actuator-01").await?;
        println!("   Deregistered actuator-01\n");

        // Step 14: Verify final device count
        // Demonstrates device count after removal
        println!("14. Final device count...");
        let devices = device_module.list_devices().await?;
        println!("   Remaining devices: {}\n", devices.len());

        println!("=== Device Management Example Completed ===");
        Ok::<(), dmsc::DMSCError>(())
    })?
}
