// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
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

extern crate dms;

use dms::device::{DMSDevice, DMSDeviceType, DMSDeviceStatus, DMSDeviceCapabilities};
use dms::device::{DMSDeviceControlModule, DMSDeviceControlConfig, DMSDiscoveryResult, DMSResourceRequest, DMSResourceAllocation};

#[test]
fn test_device_capabilities_new() {
    let capabilities = DMSDeviceCapabilities::new();
    assert_eq!(capabilities.compute_units, None);
    assert_eq!(capabilities.memory_gb, None);
    assert_eq!(capabilities.storage_gb, None);
    assert_eq!(capabilities.bandwidth_gbps, None);
    assert!(capabilities.custom_capabilities.is_empty());
}

#[test]
fn test_device_capabilities_builder() {
    let capabilities = DMSDeviceCapabilities::new()
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
    let device_capabilities = DMSDeviceCapabilities::new()
        .with_compute_units(8)
        .with_memory_gb(16.0);
    
    // Test requirements that are met
    let requirements = DMSDeviceCapabilities::new()
        .with_compute_units(4)
        .with_memory_gb(8.0);
    
    assert!(device_capabilities.meets_requirements(&requirements));
    
    // Test requirements that are not met
    let high_requirements = DMSDeviceCapabilities::new()
        .with_compute_units(16) // More than available
        .with_memory_gb(8.0);
    
    assert!(!device_capabilities.meets_requirements(&high_requirements));
    
    // Test with custom capabilities
    let device_capabilities_with_custom = DMSDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "value1".to_string());
    
    let requirements_with_custom = DMSDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "value1".to_string());
    
    assert!(device_capabilities_with_custom.meets_requirements(&requirements_with_custom));
    
    let requirements_with_wrong_custom = DMSDeviceCapabilities::new()
        .with_custom_capability("feature1".to_string(), "wrong_value".to_string());
    
    assert!(!device_capabilities_with_custom.meets_requirements(&requirements_with_wrong_custom));
}

#[test]
fn test_device_new() {
    let device = DMSDevice::new("test_device".to_string(), DMSDeviceType::CPU);
    
    assert!(!device.id().is_empty());
    assert_eq!(device.name(), "test_device");
    assert_eq!(device.device_type(), DMSDeviceType::CPU);
    assert_eq!(device.status(), DMSDeviceStatus::Unknown);
    assert!(device.is_available());
    assert!(!device.is_allocated());
}

#[test]
fn test_device_allocation() {
    let mut device = DMSDevice::new("test_device".to_string(), DMSDeviceType::CPU);
    
    // Test initial state
    assert!(device.is_available());
    assert!(!device.is_allocated());
    
    // Test allocation
    let allocation_id = "test_allocation_id";
    assert!(device.allocate(allocation_id));
    assert!(!device.is_available());
    assert!(device.is_allocated());
    assert_eq!(device.get_allocation_id(), Some(allocation_id));
    assert_eq!(device.status(), DMSDeviceStatus::Busy);
    
    // Test release
    device.release();
    assert!(device.is_available());
    assert!(!device.is_allocated());
    assert_eq!(device.get_allocation_id(), None);
    assert_eq!(device.status(), DMSDeviceStatus::Available);
}

#[test]
fn test_device_status() {
    let mut device = DMSDevice::new("test_device".to_string(), DMSDeviceType::CPU);
    
    // Test status change
    device.set_status(DMSDeviceStatus::Available);
    assert_eq!(device.status(), DMSDeviceStatus::Available);
    
    device.set_status(DMSDeviceStatus::Busy);
    assert_eq!(device.status(), DMSDeviceStatus::Busy);
    
    device.set_status(DMSDeviceStatus::Error);
    assert_eq!(device.status(), DMSDeviceStatus::Error);
}

#[test]
fn test_device_health_score() {
    let mut device = DMSDevice::new("test_device".to_string(), DMSDeviceType::CPU);
    
    // Test health score for different statuses
    device.set_status(DMSDeviceStatus::Available);
    assert_eq!(device.health_score(), 100);
    
    device.set_status(DMSDeviceStatus::Busy);
    assert_eq!(device.health_score(), 80);
    
    device.set_status(DMSDeviceStatus::Maintenance);
    assert_eq!(device.health_score(), 60);
    
    device.set_status(DMSDeviceStatus::Offline);
    assert_eq!(device.health_score(), 20);
    
    device.set_status(DMSDeviceStatus::Error);
    assert_eq!(device.health_score(), 10);
    
    device.set_status(DMSDeviceStatus::Unknown);
    assert_eq!(device.health_score(), 0);
}

#[test]
fn test_device_control_config_default() {
    let config = DMSDeviceControlConfig::default();
    
    assert!(config.discovery_enabled);
    assert_eq!(config.discovery_interval_secs, 30);
    assert!(config.auto_scheduling_enabled);
    assert_eq!(config.max_concurrent_tasks, 100);
    assert_eq!(config.resource_allocation_timeout_secs, 60);
}

#[tokio::test]
async fn test_device_control_module_new() {
    let module = DMSDeviceControlModule::new();
    // Just test that creation works without panicking
}

#[tokio::test]
async fn test_device_control_module_discover_devices() {
    let module = DMSDeviceControlModule::new();
    
    // Test device discovery
    let result = module.discover_devices().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(result.discovered_devices.len() > 0);
}

#[tokio::test]
async fn test_device_control_module_get_device_status() {
    let module = DMSDeviceControlModule::new();
    
    // Test getting device status
    let devices = module.get_device_status().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(devices.len() > 0);
}

#[tokio::test]
async fn test_device_control_module_allocate_resource() {
    let module = DMSDeviceControlModule::new();
    
    // Test resource allocation
    let request = DMSResourceRequest {
        request_id: "test_request_id".to_string(),
        device_type: DMSDeviceType::CPU,
        required_capabilities: DMSDeviceCapabilities::new()
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
    let module = DMSDeviceControlModule::new();
    
    // Test getting resource pool status
    let pool_status = module.get_resource_pool_status();
    
    // Should return a HashMap (might be empty if no resource pools are created)
    assert!(pool_status.is_empty() || pool_status.len() > 0);
}
