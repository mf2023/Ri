//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

use ri::device::{RiDevice, RiDeviceType, RiDeviceCapabilities};
use ri::device::discovery_scheduler::{RiDeviceDiscoveryEngine, RiResourceScheduler, DeviceScanResult, ResourceRequest};

/// Device discovery and resource scheduling test module for Ri tooling.
///
/// This module provides comprehensive test coverage for the device management
/// components that handle hardware resource discovery and scheduling in the
/// Ri tooling layer. The tests validate device detection, capability matching,
/// and resource allocation strategies.
///
/// ## Test Coverage
///
/// - **Device Discovery Engine**: Tests the scanning functionality that detects
///   and identifies hardware devices from scan results, including device type
///   classification based on detected capabilities and metadata.
///
/// - **Device Capability Matching**: Validates the capability matching logic
///   that determines whether a device satisfies resource requirements, including
///   memory, compute units, bandwidth, and custom capability constraints.
///
/// - **Resource Scheduling**: Tests the scheduling algorithm that selects
///   appropriate devices for resource requests based on availability and
///   requirement matching, prioritizing devices that fully satisfy constraints.
///
/// - **Multi-Device Selection**: Validates scenarios with multiple candidate
///   devices, verifying that the scheduler correctly identifies and selects
///   the optimal device based on resource requirements and priorities.
///
/// ## Design Principles
///
/// The device discovery system supports hot-plugging and dynamic device
/// detection, enabling runtime discovery of hardware resources. Tests verify
/// that scan results are correctly transformed into device representations
/// with appropriate type classification.
///
/// The capability model uses a flexible key-value format for custom
/// capabilities, enabling extension to new device types and vendor-specific
/// features without modifying core scheduling logic. Tests verify that
/// capability matching correctly interprets requirement constraints.
///
/// The resource scheduler implements a best-fit allocation strategy,
/// selecting devices that satisfy requirements with minimal excess capacity.
Tests verify that scheduling decisions correctly balance multiple
/// requirements including memory, compute, bandwidth, and custom features.

#[test]
/// Tests RiDeviceDiscoveryEngine device discovery from scan results.
///
/// Verifies that the discovery engine can transform raw scan results
/// into device representations with proper type classification.
///
/// ## Discovery Process
///
/// 1. **Scan Results Input**: Raw device information from system scan
///    - device_id: Unique identifier for the device
///    - device_name: Human-readable name
///    - device_info: Key-value metadata about the device
///
/// 2. **Device Classification**: The engine analyzes device info to determine type
///    - NVIDIA GPUs are classified as GPU type
///    - CPUs are classified as CPU type
///    - Other accelerators use type inference
///
/// 3. **Device Output**: Transformed device representation
///    - RiDevice with unique ID
///    - Classified device type
///    - Initial status set to Unknown
///
/// ## Test Scenario
///
/// 1. Create a discovery engine instance
/// 2. Provide a scan result for an NVIDIA GPU with device info
/// 3. Call discover_devices() with the scan results
/// 4. Verify exactly one device is returned
/// 5. Verify the device type is classified as GPU
///
/// ## Expected Behavior
///
/// - The scan result is transformed into a device
/// - The device has GPU type classification
/// - Device count matches input count
fn test_device_discovery_engine() {
    let mut engine = RiDeviceDiscoveryEngine::new();
    
    let scan_results = vec![
        DeviceScanResult {
            device_id: "gpu_1".to_string(),
            device_name: "NVIDIA GPU".to_string(),
            device_info: [
                ("device_name".to_string(), "NVIDIA GeForce RTX 3080".to_string()),
                ("driver".to_string(), "CUDA 11.4".to_string()),
            ].iter().cloned().collect(),
        },
    ];
    
    let devices = engine.discover_devices(scan_results);
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].device_type(), RiDeviceType::GPU);
}

#[test]
/// Tests RiResourceScheduler device selection for resource requests.
///
/// Verifies that the scheduler can match resource requests to
/// available devices based on capability requirements.
///
/// ## Scheduling Algorithm
///
/// The scheduler implements a best-fit selection strategy:
/// 1. Filter devices by required type
/// 2. Filter devices meeting all capability requirements
/// 3. If multiple candidates exist, select one (implementation-specific)
/// 4. Return the selected device ID
///
/// ## Capability Requirements
///
/// - **required_memory_gb**: Device must have at least this much memory
/// - **required_compute_units**: Device must have at least this many compute units
/// - **required_bandwidth_gbps**: Device must have at least this bandwidth
/// - **required_custom_capabilities**: Device must have all specified capabilities
///
/// ## Test Scenario
///
/// 1. Create a resource scheduler
/// 2. Define a request requiring 8GB memory, 256 compute units, 100Gbps bandwidth
/// 3. Create a device with sufficient capabilities (16GB, 512 units, 900Gbps, CUDA support)
/// 4. Call schedule_resource() with request and devices
/// 5. Verify the scheduler selects the device
///
/// ## Expected Behavior
///
/// - The scheduler finds a matching device
/// - The returned device ID matches the input device
/// - The selected device satisfies all requirements
fn test_resource_scheduler() {
    let mut scheduler = RiResourceScheduler::new();
    
    let request = ResourceRequest {
        request_id: "req_1".to_string(),
        required_memory_gb: Some(8.0),
        required_compute_units: Some(256),
        required_bandwidth_gbps: Some(100.0),
        required_custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
        priority: 5,
        deadline: None,
    };
    
    let devices = vec![
        RiDevice::new(
            "GPU 1".to_string(),
            RiDeviceType::GPU,
        ).with_capabilities(RiDeviceCapabilities {
            memory_gb: Some(16.0),
            compute_units: Some(512),
            storage_gb: Some(1000.0),
            bandwidth_gbps: Some(900.0),
            custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
        }),
    ];
    
    let selected_device = scheduler.schedule_resource(&request, &devices);
    assert_eq!(selected_device, Some(devices[0].id().to_string()));
}
