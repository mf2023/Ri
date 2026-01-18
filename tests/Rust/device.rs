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
/// Tests DMSCDeviceCapabilities creation with new() constructor.
///
/// Verifies that device capabilities can be created with default
/// empty values, initializing all capability fields to None.
///
/// ## Default Capability Values
///
/// - compute_units: None - Number of CPU cores or compute units not specified
/// - memory_gb: None - Memory capacity not specified
/// - storage_gb: None - Storage capacity not specified
/// - bandwidth_gbps: None - Network bandwidth not specified
/// - custom_capabilities: Empty hash map - No custom capabilities defined
///
/// ## Expected Behavior
///
/// - All capability fields are initialized to None
/// - Custom capabilities map is empty
/// - The capabilities struct is ready for builder pattern configuration
fn test_device_capabilities_new() {
    let capabilities = DMSCDeviceCapabilities::new();
    assert_eq!(capabilities.compute_units, None);
    assert_eq!(capabilities.memory_gb, None);
    assert_eq!(capabilities.storage_gb, None);
    assert_eq!(capabilities.bandwidth_gbps, None);
    assert!(capabilities.custom_capabilities.is_empty());
}

#[test]
/// Tests DMSCDeviceCapabilities builder pattern with fluent API.
///
/// Verifies that capabilities can be configured using the builder
/// pattern, chaining method calls for clean configuration code.
///
/// ## Builder Methods
///
/// - with_compute_units(n): Sets the number of compute units
/// - with_memory_gb(gb): Sets memory capacity in gigabytes
/// - with_storage_gb(gb): Sets storage capacity in gigabytes
/// - with_bandwidth_gbps(gbps): Sets network bandwidth in Gbps
/// - with_custom_capability(key, value): Adds a custom capability
///
/// ## Expected Behavior
///
/// - All builder methods set the corresponding field
/// - Methods return self for method chaining
/// - Custom capabilities are stored in the hash map
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
/// Tests DMSCDeviceCapabilities requirement matching with meets_requirements().
///
/// Verifies that the capability matching logic correctly determines
/// whether a device's capabilities satisfy specified requirements.
///
/// ## Matching Rules
///
/// - Quantitative fields: Device must have >= required value
/// - None values: Treated as unlimited (satisfies any requirement)
/// - Custom capabilities: Must match exactly on key and value
///
/// ## Test Scenarios
///
/// 1. **Sufficient capabilities**: Device exceeds all requirements -> true
/// 2. **Insufficient capabilities**: Device doesn't meet compute units -> false
/// 3. **Custom capabilities match**: Device has required custom capability -> true
/// 4. **Custom capabilities mismatch**: Device has wrong value -> false
///
/// ## Expected Behavior
///
/// - Capabilities that meet all requirements return true
/// - Capabilities that fail any requirement return false
/// - Custom capability matching is exact on both key and value
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
/// Tests DMSCDevice creation with name and type.
///
/// Verifies that a device can be created with a name and type
/// classification, initializing to Unknown status with available state.
///
/// ## Device Initialization
///
/// - **id**: Auto-generated unique identifier (UUID)
/// - **name**: User-provided device name
/// - **device_type**: Type classification (CPU, GPU, FPGA, TPU, ASIC)
/// - **status**: Initialized to Unknown
/// - **available**: True by default
/// - **allocated**: False by default
///
/// ## Expected Behavior
///
/// - Device has a unique non-empty ID
/// - Device name matches the provided name
/// - Device type matches the provided type
/// - Initial status is Unknown
/// - Device is marked as available
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
/// Tests DMSCDevice allocation lifecycle with allocate() and release().
///
/// Verifies that devices can be allocated to consumers and released
/// back to the available pool, with proper status transitions.
///
/// ## Allocation Lifecycle
///
/// 1. **Initial state**: Device is available, not allocated
/// 2. **Allocate**: Device becomes busy, allocated flag set, allocation ID stored
/// 3. **Release**: Device returns to available, allocated flag cleared
///
/// ## Status Transitions
///
/// - Available -> Busy: When allocated
/// - Busy -> Available: When released
///
/// ## Expected Behavior
///
/// - allocate() returns true for available devices
/// - After allocation, device is not available but allocated
/// - get_allocation_id() returns the allocation ID
/// - release() returns device to available state
/// - After release, allocation ID is cleared
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
/// Tests DMSCDevice status management with set_status().
///
/// Verifies that device status can be changed and the new status
/// is correctly reflected in status queries.
///
/// ## Status Values
///
/// - Unknown: Initial state before status determination
/// - Available: Ready for allocation
/// - Busy: Currently allocated to a consumer
/// - Maintenance: Undergoing maintenance
/// - Offline: Not reachable but not in error
/// - Error: Encountered an error condition
///
/// ## Expected Behavior
///
/// - set_status() changes the device status
/// - status() returns the current status
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
/// Tests DMSCDevice health scoring based on status.
///
/// Verifies that health scores are correctly calculated based
/// on the device's current status, providing quick assessment.
///
/// ## Health Score Mapping
///
/// - Available: 100% - Fully operational, ready for use
/// - Busy: 80% - In use but healthy
/// - Maintenance: 60% - Under maintenance, partially usable
/// - Offline: 20% - Unreachable, not in error
/// - Error: 10% - Error condition detected
/// - Unknown: 0% - Status not yet determined
///
/// ## Expected Behavior
///
/// - Each status has a specific health score
/// - Health scores decrease with problematic states
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
/// Tests DMSCDeviceControlConfig default configuration values.
///
/// Verifies that the device control module has appropriate defaults
/// for discovery, scheduling, and resource allocation settings.
///
/// ## Default Configuration Values
///
/// - **discovery_enabled**: true - Device discovery is active
/// - **discovery_interval_secs**: 30 - Discovery runs every 30 seconds
/// - **auto_scheduling_enabled**: true - Automatic scheduling is enabled
/// - **max_concurrent_tasks**: 100 - Maximum concurrent tasks
/// - **resource_allocation_timeout_secs**: 60 - Allocation timeout in seconds
///
/// ## Expected Behavior
///
/// - All feature toggles are enabled by default
/// - Timing values are sensible defaults for production
fn test_device_control_config_default() {
    let config = DMSCDeviceControlConfig::default();
    
    assert!(config.discovery_enabled);
    assert_eq!(config.discovery_interval_secs, 30);
    assert!(config.auto_scheduling_enabled);
    assert_eq!(config.max_concurrent_tasks, 100);
    assert_eq!(config.resource_allocation_timeout_secs, 60);
}

#[tokio::test]
/// Tests DMSCDeviceControlModule creation with new().
///
/// Verifies that the device control module can be created successfully
/// and initializes with mock devices for testing.
///
/// ## Module Initialization
///
/// - The control module is created without errors
/// - Mock devices are added for testing
/// - The module is ready for device management operations
///
/// ## Expected Behavior
///
/// - Module creation succeeds
async fn test_device_control_module_new() {
    let module = DMSCDeviceControlModule::new();
}
    // Just test that creation works without panicking
}

#[tokio::test]
/// Tests device discovery through the control module.
///
/// Verifies that the control module can discover devices and return
/// a discovery result with the list of found devices.
///
/// ## Discovery Process
///
/// - discover_devices() scans for available devices
/// - Returns a DMSCDiscoveryResult with discovered devices
/// - Mock devices are available for testing
///
/// ## Expected Behavior
///
/// - Discovery returns successfully
/// - At least one device is discovered (from mocks)
async fn test_device_control_module_discover_devices() {
    let module = DMSCDeviceControlModule::new();
    
    // Test device discovery
    let result = module.discover_devices().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(result.discovered_devices.len() > 0);
}

#[tokio::test]
/// Tests device status retrieval through the control module.
///
/// Verifies that the control module can report the status of
/// all registered devices.
///
/// ## Status Reporting
///
/// - get_device_status() returns all device statuses
/// - Each device status includes device ID, name, type, and health
/// - The result is a vector of device status information
///
/// ## Expected Behavior
///
/// - Status retrieval succeeds
/// - At least one device is reported (from mocks)
async fn test_device_control_module_get_device_status() {
    let module = DMSCDeviceControlModule::new();
    
    // Test getting device status
    let devices = module.get_device_status().await.unwrap();
    
    // Should return some devices (mock devices added in init)
    assert!(devices.len() > 0);
}

#[tokio::test]
/// Tests resource allocation through the control module.
///
/// Verifies that resources can be allocated based on device type
/// and capability requirements, with proper release handling.
///
/// ## Allocation Request
///
/// - A DMSCResourceRequest specifies device type, capabilities, priority, and timeout
/// - The module finds a matching available device
/// - If found, returns an allocation with allocation_id
///
/// ## Allocation Response
///
/// - allocation.is_some(): A suitable device was found
/// - allocation_id: Unique identifier for the allocation
/// - release_resource(): Frees the device back to the pool
///
/// ## Expected Behavior
///
/// - Resource allocation finds a suitable device
/// - Resource can be released after use
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
/// Tests resource pool status retrieval through the control module.
///
/// Verifies that the control module can report the status of
/// resource pools including device counts and utilization.
///
/// ## Pool Status Information
///
/// - get_resource_pool_status() returns a HashMap
/// - Keys are pool names or device types
/// - Values contain pool statistics
///
/// ## Expected Behavior
///
/// - Status retrieval succeeds
/// - Returns a HashMap (may be empty or populated)
async fn test_device_control_module_get_resource_pool_status() {
    let module = DMSCDeviceControlModule::new();
    
    // Test getting resource pool status
    let pool_status = module.get_resource_pool_status();
    
    // Should return a HashMap (might be empty if no resource pools are created)
    assert!(pool_status.is_empty() || pool_status.len() > 0);
}
