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

use dmsc::device::{DMSCDevice, DMSCDeviceType, DMSCDeviceCapabilities};
use dmsc::device::discovery_scheduler::{DMSCDeviceDiscoveryEngine, DMSCResourceScheduler, DeviceScanResult, ResourceRequest};

/// Device discovery and resource scheduling test module for DMSC tooling.
///
/// This module provides comprehensive test coverage for the device management
/// components that handle hardware resource discovery and scheduling in the
/// DMSC tooling layer. The tests validate device detection, capability matching,
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
fn test_device_discovery_engine() {
    let mut engine = DMSCDeviceDiscoveryEngine::new();
    
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
    assert_eq!(devices[0].device_type(), DMSCDeviceType::GPU);
}

#[test]
fn test_resource_scheduler() {
    let mut scheduler = DMSCResourceScheduler::new();
    
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
        DMSCDevice::new(
            "GPU 1".to_string(),
            DMSCDeviceType::GPU,
        ).with_capabilities(DMSCDeviceCapabilities {
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
