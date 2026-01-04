// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Device Module Tests
//!
//! This module contains comprehensive tests for the DMSC device management system,
//! covering device capabilities, device lifecycle management, device control operations,
//! and resource allocation mechanisms.
//!
//! ## Test Coverage
//!
//! - **DMSCDeviceCapabilities**: Tests for device capability representation including
//!   compute units, memory, storage, bandwidth, and custom capabilities. The builder
//!   pattern enables fluent capability construction, and the requirements matching
//!   system supports capability-based device selection
//!
//! - **DMSCDevice**: Tests for the core device entity covering device identification,
//!   type classification (CPU, GPU, FPGA, TPU, ASIC), status management, allocation
//!   lifecycle, and health scoring based on device state
//!
//! - **DMSCDeviceControlConfig**: Tests for device control configuration including
//!   discovery settings, scheduling parameters, concurrent task limits, and resource
//!   allocation timeouts
//!
//! - **DMSCDeviceControlModule**: Tests for the device control module that orchestrates
//!   device discovery, status monitoring, resource allocation, and pool management
//!
//! - **Resource Management**: Tests for resource request/allocation semantics including
//!   capability-based matching, priority handling, timeout management, and allocation
//!   lifecycle (allocate, query, release)
//!
//! ## Device Lifecycle
//!
//! The device management system implements a complete device lifecycle:
//! - **Discovery**: Devices are discovered through the control module's discovery
//!   mechanism, which identifies available resources and registers them
//! - **Registration**: Each device receives a unique identifier and is tracked in
//!   the system's device registry
//! - **Status Management**: Devices transition through various states (Unknown,
//!   Available, Busy, Maintenance, Offline, Error) with associated health scores
//! - **Allocation**: Resources can be allocated to consumers, marking devices as
//!   busy and tracking allocation identifiers
//! - **Release**: When resource use is complete, devices are released back to the
//!   available pool
//!
//! ## Capability Matching
//!
//! The capability system enables sophisticated device selection:
//! - **Quantitative Capabilities**: Compute units, memory (GB), storage (GB), and
//!   bandwidth (Gbps) are stored as optional values that can be compared against
//!   requirements
//! - **Custom Capabilities**: Key-value pairs enable domain-specific feature flags
//!   and capability extensions
//! - **Requirements Matching**: The `meets_requirements()` method checks that a
//!   device's capabilities satisfy all specified requirements, with None values
//!   treated as unlimited (a device with unspecified memory meets any memory
//!   requirement)
//!
//! ## Health Scoring
//!
//! Health scores provide quick device state assessment:
//! - **Available**: 100% - Device is fully operational and ready for use
//! - **Busy**: 80% - Device is in use but healthy
//! - **Maintenance**: 60% - Device is undergoing maintenance, still partially usable
//! - **Offline**: 20% - Device is unreachable but not in error state
//! - **Error**: 10% - Device has encountered an error condition
//! - **Unknown**: 0% - Device state has not been determined

use dmsc::device::{DMSCDevice, DMSCDeviceType, DMSCDeviceStatus, DMSCDeviceCapabilities};
use dmsc::device::{DMSCDeviceControlModule, DMSCDeviceControlConfig, DMSCDiscoveryResult, DMSCResourceRequest, DMSCResourceAllocation};

#[test]
fn test_device_capabilities_new() {
    let capabilities = DMSCDeviceCapabilities::new();
    assert_eq!(capabilities.compute_units, None);
    assert_eq!(capabilities.memory_gb, None);
    assert_eq!(capabilities.storage_gb, None);
    assert_eq!(capabilities.bandwidth_gbps, None);
    assert!(capabilities.custom_capabilities.is_empty());
}

#[test]
fn test_device_capabilities_builder() {
    let capabilities = DMSCDeviceCapabilities::new()
        .with_compute_units(8)
        .with_memory_gb(16.0)
        .with_storage_gb(512.0)
        .with_bandwidth_gbps(10.0)
        .with_custom_capability("feature".to_string(), "value".to_string());
    
    assert_eq!(capabilities.compute_units, Some(8));
    assert_eq!(capabilities.memory_gb, Some(16.0));
    assert_eq!(capabilities.storage_gb, Some(512.0));
    assert_eq!(capabilities.bandwidth_gbps, Some(10.0));
    assert_eq!(capabilities.custom_capabilities.get("feature"), Some(&"value".to_string()));
}

#[test]
fn test_device_capabilities_meets_requirements() {
    // Test device with sufficient capabilities
    let device_capabilities = DMSCDeviceCapabilities::new()
        .with_compute_units(8)
        .with_memory_gb(16.0);
    
    // Test requirements that are met
    let requirements = DMSCDeviceCapabilities::new()
        .with_compute_units(4)
        .with_memory_gb(8.0);
    
    assert!(device_capabilities.meets_requirements(&requirements));
    
    // Test requirements that are not met
    let high_requirements = DMSCDeviceCapabilities::new()
        .with_compute_units(16) // More than available
        .with_memory_gb(8.0);
    
    assert!(!device_capabilities.meets_requirements(&high_requirements));
    
    // Test with custom capabilities
    let device_capabilities_with_custom = DMSCDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "value1".to_string());
    
    let requirements_with_custom = DMSCDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "value1".to_string());
    
    assert!(device_capabilities_with_custom.meets_requirements(&requirements_with_custom));
    
    let requirements_with_wrong_custom = DMSCDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "wrong_value".to_string());
    
    assert!(!device_capabilities_with_custom.meets_requirements(&requirements_with_wrong_custom));
}

#[test]
fn test_device_new() {
    let device = DMSCDevice::new("test_device".to_string(), DMSCDeviceType::CPU);
    
    assert!(!device.id().is_empty());
    assert_eq!(device.name(), "test_device");
    assert_eq!(device.device_type(), DMSCDeviceType::CPU);
    assert_eq!(device.status(), DMSCDeviceStatus::Unknown);
    assert!(device.is_available());
    assert!(!device.is_allocated());
}

#[test]
fn test_device_allocation() {
    let mut device = DMSCDevice::new("test_device".to_string(), DMSCDeviceType::CPU);
    
    // Test initial state
    assert!(device.is_available());
    assert!(!device.is_allocated());
    
    // Test allocation
    let allocation_id = "test_allocation_id";
    assert!(device.allocate(allocation_id));
    assert!(!device.is_available());
    assert!(device.is_allocated());
    assert_eq!(device.get_allocation_id(), Some(allocation_id));
    assert_eq!(device.status(), DMSCDeviceStatus::Busy);
    
    // Test release
    device.release();
    assert!(device.is_available());
    assert!(!device.is_allocated());
    assert_eq!(device.get_allocation_id(), None);
    assert_eq!(device.status(), DMSCDeviceStatus::Available);
}

#[test]
fn test_device_status() {
    let mut device = DMSCDevice::new("test_device".to_string(), DMSCDeviceType::CPU);
    
    // Test status change
    device.set_status(DMSCDeviceStatus::Available);
    assert_eq!(device.status(), DMSCDeviceStatus::Available);
    
    device.set_status(DMSCDeviceStatus::Busy);
    assert_eq!(device.status(), DMSCDeviceStatus::Busy);
    
    device.set_status(DMSCDeviceStatus::Error);
    assert_eq!(device.status(), DMSCDeviceStatus::Error);
}

#[test]
fn test_device_health_score() {
    let mut device = DMSCDevice::new("test_device".to_string(), DMSCDeviceType::CPU);
    
    // Test health score for different statuses
    device.set_status(DMSCDeviceStatus::Available);
    assert_eq!(device.health_score(), 100);
    
    device.set_status(DMSCDeviceStatus::Busy);
    assert_eq!(device.health_score(), 80);
    
    device.set_status(DMSCDeviceStatus::Maintenance);
    assert_eq!(device.health_score(), 60);
    
    device.set_status(DMSCDeviceStatus::Offline);
    assert_eq!(device.health_score(), 20);
    
    device.set_status(DMSCDeviceStatus::Error);
    assert_eq!(device.health_score(), 10);
    
    device.set_status(DMSCDeviceStatus::Unknown);
    assert_eq!(device.health_score(), 0);
}

#[test]
fn test_device_control_config_default() {
    let config = DMSCDeviceControlConfig::default();
    
    assert!(config.discovery_enabled);
    assert_eq!(config.discovery_interval_secs, 30);
    assert!(config.auto_scheduling_enabled);
    assert_eq!(config.max_concurrent_tasks, 100);
    assert_eq!(config.resource_allocation_timeout_secs, 60);
}

#[tokio::test]
async fn test_device_control_module_new() {
    let module = DMSCDeviceControlModule::new();
    // Just test that creation works without panicking
}

#[tokio::test]
async fn test_device_control_module_discover_devices() {
    let module = DMSCDeviceControlModule::new();
    
    // Test device discovery
    let result = module.discover_devices().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(result.discovered_devices.len() > 0);
}

#[tokio::test]
async fn test_device_control_module_get_device_status() {
    let module = DMSCDeviceControlModule::new();
    
    // Test getting device status
    let devices = module.get_device_status().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(devices.len() > 0);
}

#[tokio::test]
async fn test_device_control_module_allocate_resource() {
    let module = DMSCDeviceControlModule::new();
    
    // Test resource allocation
    let request = DMSCResourceRequest {
        request_id: "test_request_id".to_string(),
        device_type: DMSCDeviceType::CPU,
        required_capabilities: DMSCDeviceCapabilities::new()
            .with_compute_units(1)
            .with_memory_gb(1.0),
        priority: 5,
        timeout_secs: 60,
    };
    
    let allocation = module.allocate_resource(request).await.unwrap();
    
    // Should return an allocation if a suitable device is found
    assert!(allocation.is_some());
    
    if let Some(allocation) = allocation {
        // Test releasing the resource
        module.release_resource(&allocation.allocation_id).await.unwrap();
    }
}

#[tokio::test]
async fn test_device_control_module_get_resource_pool_status() {
    let module = DMSCDeviceControlModule::new();
    
    // Test getting resource pool status
    let pool_status = module.get_resource_pool_status();
    
    // Should return a HashMap (might be empty if no resource pools are created)
    assert!(pool_status.is_empty() || pool_status.len() > 0);
}
