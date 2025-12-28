//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

#![allow(non_snake_case)]

//! # Device Core Structures
//! 
//! This file defines the core data structures for device management in DMSC, including device types,
//! capabilities, status, health metrics, and the device representation itself. These structures form
//! the foundation for device discovery, scheduling, and management.
//! 
//! ## Key Components
//! 
//! - **DMSCDeviceType**: Enum defining supported device types
//! - **DMSCDeviceCapabilities**: Device capabilities structure
//! - **DMSCDeviceStatus**: Enum defining device statuses
//! - **DMSCDeviceHealthMetrics**: Device health metrics structure
//! - **DMSCDevice**: Main device representation with status, capabilities, and health metrics
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Coverage**: Covers all aspects of device management
//! 2. **Flexibility**: Supports custom device types and capabilities
//! 3. **Health Monitoring**: Built-in health metrics for device monitoring
//! 4. **Resource Management**: Capabilities structure for resource allocation
//! 5. **Status Tracking**: Clear status definitions for device lifecycle management
//! 6. **Serialization Support**: All structures support serialization/deserialization
//! 7. **Builder Pattern**: Capabilities support a fluent builder API
//! 8. **Health Scoring**: Built-in health scoring for device selection
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::device::{DMSCDevice, DMSCDeviceType, DMSCDeviceCapabilities};
//! 
//! // Create a new device
//! let mut device = DMSCDevice::new("server-1".to_string(), DMSCDeviceType::CPU);
//! 
//! // Configure device capabilities
//! let capabilities = DMSCDeviceCapabilities::new()
//!     .with_compute_units(16)
//!     .with_memory_gb(32.0)
//!     .with_storage_gb(1024.0)
//!     .with_bandwidth_gbps(10.0);
//! 
//! // Set device capabilities and status
//! device = device.with_capabilities(capabilities);
//! device.set_status(dms::device::DMSCDeviceStatus::Available);
//! 
//! // Check if device meets requirements
//! let requirements = DMSCDeviceCapabilities::new().with_compute_units(8);
//! if device.capabilities().meets_requirements(&requirements) {
//!     println!("Device meets requirements");
//! }
//! ```

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use pyo3::pymethods;

/// Configuration for device control module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceControlConfig {
    /// Enable CPU discovery
    pub enable_cpu_discovery: bool,
    /// Enable GPU discovery  
    pub enable_gpu_discovery: bool,
    /// Enable memory discovery
    pub enable_memory_discovery: bool,
    /// Enable storage discovery
    pub enable_storage_discovery: bool,
    /// Enable network discovery
    pub enable_network_discovery: bool,
    /// Network discovery timeout in seconds
    pub discovery_timeout_secs: u64,
    /// Maximum number of devices to discover per type
    pub max_devices_per_type: usize,
}

impl Default for DMSCDeviceControlConfig {
    fn default() -> Self {
        Self {
            enable_cpu_discovery: true,
            enable_gpu_discovery: true,
            enable_memory_discovery: true,
            enable_storage_discovery: true,
            enable_network_discovery: true,
            discovery_timeout_secs: 30,
            max_devices_per_type: 100,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_config() -> Self {
        Self::default()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceControlConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_config() -> Self {
        Self::default()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceHealthMetrics {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_metrics() -> Self {
        Self::default()
    }
}

/// Configuration for device module
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceConfig {
    /// Enable CPU discovery
    pub enable_cpu_discovery: bool,
    /// Enable GPU discovery  
    pub enable_gpu_discovery: bool,
    /// Enable memory discovery
    pub enable_memory_discovery: bool,
    /// Enable storage discovery
    pub enable_storage_discovery: bool,
    /// Enable network discovery
    pub enable_network_discovery: bool,
    /// Network discovery timeout in seconds
    pub discovery_timeout_secs: u64,
    /// Maximum number of devices to discover per type
    pub max_devices_per_type: usize,
}

impl Default for DMSCDeviceConfig {
    fn default() -> Self {
        Self {
            enable_cpu_discovery: true,
            enable_gpu_discovery: true,
            enable_memory_discovery: true,
            enable_storage_discovery: true,
            enable_network_discovery: true,
            discovery_timeout_secs: 30,
            max_devices_per_type: 100,
        }
    }
}

/// Network device information for remote device discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct NetworkDeviceInfo {
    /// Unique device identifier
    pub id: String,
    /// Device type (CPU, GPU, Memory, Storage, Network)
    pub device_type: String,
    /// Source system identifier
    pub source: String,
    /// Number of compute units (for CPU/GPU)
    pub compute_units: Option<usize>,
    /// Memory capacity in GB
    pub memory_gb: Option<f64>,
    /// Storage capacity in GB
    pub storage_gb: Option<f64>,
    /// Bandwidth in Gbps
    pub bandwidth_gbps: Option<f64>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl NetworkDeviceInfo {
    #[new]
    fn py_new(id: String, device_type: String, source: String) -> Self {
        Self {
            id,
            device_type,
            source,
            compute_units: None,
            memory_gb: None,
            storage_gb: None,
            bandwidth_gbps: None,
        }
    }
    
    #[staticmethod]
    fn default_info(id: String, device_type: String, source: String) -> Self {
        Self::py_new(id, device_type, source)
    }
}

/// Device type enumeration
/// 
/// This enum defines the different types of devices supported by DMSC. Each device type
/// has specific capabilities and use cases in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCDeviceType {
    /// Central Processing Unit - General purpose computing
    CPU,
    /// Graphics Processing Unit - Parallel computing and graphics
    GPU,
    /// Memory - RAM and other memory devices
    Memory,
    /// Storage - Hard drives, SSDs, and other storage devices
    Storage,
    /// Network - Network interfaces and devices
    Network,
    /// Sensor - Devices that collect data from the environment
    Sensor,
    /// Actuator - Devices that perform physical actions
    Actuator,
    /// Custom - User-defined device types
    Custom,
}

impl std::fmt::Display for DMSCDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMSCDeviceType::CPU => write!(f, "CPU"),
            DMSCDeviceType::GPU => write!(f, "GPU"),
            DMSCDeviceType::Memory => write!(f, "Memory"),
            DMSCDeviceType::Storage => write!(f, "Storage"),
            DMSCDeviceType::Network => write!(f, "Network"),
            DMSCDeviceType::Sensor => write!(f, "Sensor"),
            DMSCDeviceType::Actuator => write!(f, "Actuator"),
            DMSCDeviceType::Custom => write!(f, "Custom"),
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceType {
    #[staticmethod]
    fn new_cpu() -> Self {
        DMSCDeviceType::CPU
    }
    
    #[staticmethod]
    fn new_gpu() -> Self {
        DMSCDeviceType::GPU
    }
    
    #[staticmethod]
    fn new_memory() -> Self {
        DMSCDeviceType::Memory
    }
    
    #[staticmethod]
    fn new_storage() -> Self {
        DMSCDeviceType::Storage
    }
    
    #[staticmethod]
    fn new_network() -> Self {
        DMSCDeviceType::Network
    }
    
    #[staticmethod]
    fn new_sensor() -> Self {
        DMSCDeviceType::Sensor
    }
    
    #[staticmethod]
    fn new_actuator() -> Self {
        DMSCDeviceType::Actuator
    }
    
    #[staticmethod]
    fn new_custom() -> Self {
        DMSCDeviceType::Custom
    }
    
    fn __str__(&self) -> String {
        self.to_string()
    }
}

/// Device capabilities structure
/// 
/// This struct defines the capabilities of a device, including compute power, memory, storage,
/// bandwidth, and custom capabilities. It supports a fluent builder API for easy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceCapabilities {
    /// Number of compute units (e.g., CPU cores, GPU CUDA cores)
    pub compute_units: Option<usize>,
    /// Memory capacity in gigabytes
    pub memory_gb: Option<f64>,
    /// Storage capacity in gigabytes
    pub storage_gb: Option<f64>,
    /// Bandwidth in gigabits per second
    pub bandwidth_gbps: Option<f64>,
    /// Custom capabilities as key-value pairs
    pub custom_capabilities: HashMap<String, String>,
}

impl Default for DMSCDeviceCapabilities {
    /// Returns the default device capabilities (empty capabilities)
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCDeviceCapabilities {
    /// Creates a new empty device capabilities structure
    /// 
    /// # Returns
    /// 
    /// A new `DMSCDeviceCapabilities` instance with no capabilities set
    pub fn new() -> Self {
        Self {
            compute_units: None,
            memory_gb: None,
            storage_gb: None,
            bandwidth_gbps: None,
            custom_capabilities: HashMap::new(),
        }
    }
    
    /// Sets the number of compute units
    /// 
    /// # Parameters
    /// 
    /// - `units`: Number of compute units
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceCapabilities` instance
    pub fn with_compute_units(mut self, units: usize) -> Self {
        self.compute_units = Some(units);
        self
    }
    
    /// Sets the memory capacity in gigabytes
    /// 
    /// # Parameters
    /// 
    /// - `memory`: Memory capacity in GB
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceCapabilities` instance
    pub fn with_memory_gb(mut self, memory: f64) -> Self {
        self.memory_gb = Some(memory);
        self
    }
    
    /// Sets the storage capacity in gigabytes
    /// 
    /// # Parameters
    /// 
    /// - `storage`: Storage capacity in GB
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceCapabilities` instance
    pub fn with_storage_gb(mut self, storage: f64) -> Self {
        self.storage_gb = Some(storage);
        self
    }
    
    /// Sets the bandwidth in gigabits per second
    /// 
    /// # Parameters
    /// 
    /// - `bandwidth`: Bandwidth in Gbps
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceCapabilities` instance
    pub fn with_bandwidth_gbps(mut self, bandwidth: f64) -> Self {
        self.bandwidth_gbps = Some(bandwidth);
        self
    }
    
    /// Adds a custom capability
    /// 
    /// # Parameters
    /// 
    /// - `key`: Custom capability key
    /// - `value`: Custom capability value
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceCapabilities` instance
    pub fn with_custom_capability(mut self, key: String, value: String) -> Self {
        self.custom_capabilities.insert(key, value);
        self
    }
    
    /// Checks if this device meets the required capabilities
    /// 
    /// This method compares the device's capabilities with the required capabilities and
    /// returns true if all required capabilities are met or exceeded.
    /// 
    /// # Parameters
    /// 
    /// - `requirements`: The required capabilities to check against
    /// 
    /// # Returns
    /// 
    /// `true` if the device meets all requirements, `false` otherwise
    pub fn meets_requirements(&self, requirements: &DMSCDeviceCapabilities) -> bool {
        // Check compute units requirement
        if let Some(required_units) = requirements.compute_units {
            if let Some(available_units) = self.compute_units {
                if available_units < required_units {
                    return false;
                }
            } else {
                return false; // No compute units available
            }
        }
        
        // Check memory requirement
        if let Some(required_memory) = requirements.memory_gb {
            if let Some(available_memory) = self.memory_gb {
                if available_memory < required_memory {
                    return false;
                }
            } else {
                return false; // No memory available
            }
        }
        
        // Check storage requirement
        if let Some(required_storage) = requirements.storage_gb {
            if let Some(available_storage) = self.storage_gb {
                if available_storage < required_storage {
                    return false;
                }
            } else {
                return false; // No storage available
            }
        }
        
        // Check bandwidth requirement
        if let Some(required_bandwidth) = requirements.bandwidth_gbps {
            if let Some(available_bandwidth) = self.bandwidth_gbps {
                if available_bandwidth < required_bandwidth {
                    return false;
                }
            } else {
                return false; // No bandwidth available
            }
        }
        
        // Check custom capabilities
        for (key, required_value) in &requirements.custom_capabilities {
            match self.custom_capabilities.get(key) {
                Some(available_value) => {
                    // Simple string comparison for now
                    if available_value != required_value {
                        return false;
                    }
                }
                None => return false, // Required capability not available
            }
        }
        
        true
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceCapabilities {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    #[staticmethod]
    fn default_capabilities() -> Self {
        Self::default()
    }
    
    fn with_compute_units_py(&self, units: usize) -> Self {
        let mut new = self.clone();
        new.compute_units = Some(units);
        new
    }
    
    fn with_memory_gb_py(&self, memory: f64) -> Self {
        let mut new = self.clone();
        new.memory_gb = Some(memory);
        new
    }
    
    fn with_storage_gb_py(&self, storage: f64) -> Self {
        let mut new = self.clone();
        new.storage_gb = Some(storage);
        new
    }
    
    fn with_bandwidth_gbps_py(&self, bandwidth: f64) -> Self {
        let mut new = self.clone();
        new.bandwidth_gbps = Some(bandwidth);
        new
    }
    
    fn with_custom_capability_py(&self, key: String, value: String) -> Self {
        let mut new = self.clone();
        new.custom_capabilities.insert(key, value);
        new
    }
    
    // Getter methods for Python
    fn get_compute_units_py(&self) -> Option<usize> { self.compute_units }
    fn get_memory_gb_py(&self) -> Option<f64> { self.memory_gb }
    fn get_storage_gb_py(&self) -> Option<f64> { self.storage_gb }
    fn get_bandwidth_gbps_py(&self) -> Option<f64> { self.bandwidth_gbps }
    fn get_custom_capabilities_py(&self) -> HashMap<String, String> { self.custom_capabilities.clone() }
    
    fn meets_requirements_py(&self, requirements: &DMSCDeviceCapabilities) -> bool {
        self.meets_requirements(requirements)
    }
}

/// Device status enumeration
/// 
/// This enum defines the different statuses a device can have during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCDeviceStatus {
    /// Device status is unknown
    Unknown,
    /// Device is available for use
    Available,
    /// Device is currently in use
    Busy,
    /// Device has encountered an error
    Error,
    /// Device is offline or unreachable
    Offline,
    /// Device is under maintenance
    Maintenance,
    /// Device is degraded but still operational
    Degraded,
    /// Device is allocated to a specific task
    Allocated,
}



/// Device health metrics structure
/// 
/// This struct contains health metrics for monitoring device performance and reliability.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceHealthMetrics {
    /// CPU usage percentage (0-100)
    pub cpu_usage_percent: f64,
    /// Memory usage percentage (0-100)
    pub memory_usage_percent: f64,
    /// Device temperature in Celsius
    pub temperature_celsius: f64,
    /// Number of errors encountered
    pub error_count: u32,
    /// Throughput in operations per second
    pub throughput: u64, // operations per second
    /// Network latency in milliseconds (for network devices)
    pub network_latency_ms: f64,
    /// Disk I/O operations per second (for storage devices)
    pub disk_iops: u64,
    /// Battery level percentage (for battery-powered devices, 0-100)
    pub battery_level_percent: f64,
    /// Response time in milliseconds
    pub response_time_ms: f64,
    /// Uptime in seconds
    pub uptime_seconds: u64,
}

impl Default for DMSCDeviceHealthMetrics {
    /// Returns default health metrics (all zero values)
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            temperature_celsius: 0.0,
            error_count: 0,
            throughput: 0,
            network_latency_ms: 0.0,
            disk_iops: 0,
            battery_level_percent: 0.0,
            response_time_ms: 0.0,
            uptime_seconds: 0,
        }
    }
}

/// Smart device representation
/// 
/// This struct represents a smart device in the DMSC system, including its status, capabilities,
/// health metrics, and lifecycle information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDevice {
    /// Unique device ID
    id: String,
    /// Device name
    name: String,
    /// Device type
    device_type: DMSCDeviceType,
    /// Current device status
    status: DMSCDeviceStatus,
    /// Device capabilities for resource allocation
    capabilities: DMSCDeviceCapabilities,
    /// Current health metrics
    health_metrics: DMSCDeviceHealthMetrics,
    /// Physical location of the device (optional)
    location: Option<String>,
    /// Device group (for grouping devices)
    group: Option<String>,
    /// Device tags (for filtering and selection)
    tags: Vec<String>,
    /// Additional metadata as key-value pairs
    metadata: HashMap<String, String>,
    /// Last time the device was seen/updated
    last_seen: chrono::DateTime<chrono::Utc>,
    /// ID of the current allocation using this device (if any)
    current_allocation_id: Option<String>,
}

impl DMSCDevice {
    /// Creates a new device with the given name and type
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the device
    /// - `device_type`: The type of the device
    /// 
    /// # Returns
    /// 
    /// A new `DMSCDevice` instance with default capabilities and health metrics
    pub fn new(name: String, device_type: DMSCDeviceType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            device_type,
            status: DMSCDeviceStatus::Unknown,
            capabilities: DMSCDeviceCapabilities::new(),
            health_metrics: DMSCDeviceHealthMetrics::default(),
            location: None,
            group: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
            last_seen: chrono::Utc::now(),
            current_allocation_id: None,
        }
    }
    
    /// Gets the device ID
    /// 
    /// # Returns
    /// 
    /// The device ID as a string slice
    pub fn id(&self) -> &str {
        &self.id
    }
    
    /// Gets the device name
    /// 
    /// # Returns
    /// 
    /// The device name as a string slice
    pub fn name(&self) -> &str {
        &self.name
    }
    
    /// Gets the device type
    /// 
    /// # Returns
    /// 
    /// The device type as a `DMSCDeviceType` enum
    pub fn device_type(&self) -> DMSCDeviceType {
        self.device_type
    }
    
    /// Gets the current device status
    /// 
    /// # Returns
    /// 
    /// The device status as a `DMSCDeviceStatus` enum
    pub fn status(&self) -> DMSCDeviceStatus {
        self.status
    }
    
    /// Gets a reference to the device capabilities
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCDeviceCapabilities` structure
    pub fn capabilities(&self) -> &DMSCDeviceCapabilities {
        &self.capabilities
    }
    
    /// Gets a reference to the device health metrics
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCDeviceHealthMetrics` structure
    pub fn health_metrics(&self) -> &DMSCDeviceHealthMetrics {
        &self.health_metrics
    }
    
    /// Sets the device status and updates the last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `status`: The new status to set
    pub fn set_status(&mut self, status: DMSCDeviceStatus) {
        self.status = status;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Updates the device health metrics and last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `metrics`: The new health metrics to set
    pub fn update_health_metrics(&mut self, metrics: DMSCDeviceHealthMetrics) {
        self.health_metrics = metrics;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Increments the device error count and updates the last seen timestamp
    pub fn increment_error_count(&mut self) {
        self.health_metrics.error_count += 1;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Updates the device throughput and last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `throughput`: The new throughput value in operations per second
    pub fn update_throughput(&mut self, throughput: u64) {
        self.health_metrics.throughput = throughput;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Sets the device capabilities using the builder pattern
    /// 
    /// # Parameters
    /// 
    /// - `capabilities`: The new capabilities to set
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDevice` instance
    pub fn with_capabilities(mut self, capabilities: DMSCDeviceCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }
    
    /// Sets the device location
    /// 
    /// # Parameters
    /// 
    /// - `location`: The physical location of the device
    pub fn set_location(&mut self, location: String) {
        self.location = Some(location);
    }
    
    /// Adds a metadata key-value pair to the device
    /// 
    /// # Parameters
    /// 
    /// - `key`: Metadata key
    /// - `value`: Metadata value
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Updates the last seen timestamp to the current time
    pub fn update_last_seen(&mut self) {
        self.last_seen = chrono::Utc::now();
    }
    
    /// Gets the last seen timestamp
    /// 
    /// # Returns
    /// 
    /// The last seen timestamp as a `chrono::DateTime<chrono::Utc>`
    pub fn last_seen(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_seen
    }
    
    /// Checks if the device is available for allocation
    /// 
    /// A device is available if its status is Available and it has no current allocation.
    /// 
    /// # Returns
    /// 
    /// `true` if the device is available, `false` otherwise
    pub fn is_available(&self) -> bool {
        self.status == DMSCDeviceStatus::Available && self.current_allocation_id.is_none()
    }
    
    /// Checks if the device is currently allocated
    /// 
    /// # Returns
    /// 
    /// `true` if the device is allocated, `false` otherwise
    pub fn is_allocated(&self) -> bool {
        self.current_allocation_id.is_some()
    }
    
    /// Allocates the device to a specific allocation ID
    /// 
    /// This method marks the device as busy and associates it with the given allocation ID.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: The ID of the allocation using this device
    /// 
    /// # Returns
    /// 
    /// `true` if the device was successfully allocated, `false` if it was already in use
    pub fn allocate(&mut self, allocation_id: &str) -> bool {
        if self.is_available() {
            self.current_allocation_id = Some(allocation_id.to_string());
            self.status = DMSCDeviceStatus::Busy;
            true
        } else {
            false
        }
    }
    
    /// Releases the device from its current allocation
    /// 
    /// This method clears the allocation ID and sets the device status to Available if it was Busy.
    pub fn release(&mut self) {
        self.current_allocation_id = None;
        if self.status == DMSCDeviceStatus::Busy {
            self.status = DMSCDeviceStatus::Available;
        }
    }
    
    /// Gets the device group
    /// 
    /// # Returns
    /// 
    /// The device group as an `Option<&str>`
    pub fn group(&self) -> Option<&str> {
        self.group.as_deref()
    }
    
    /// Sets the device group
    /// 
    /// # Parameters
    /// 
    /// - `group`: The new group for the device
    pub fn set_group(&mut self, group: Option<String>) {
        self.group = group;
    }
    
    /// Gets the device tags
    /// 
    /// # Returns
    /// 
    /// A reference to the device tags vector
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }
    
    /// Adds a tag to the device
    /// 
    /// # Parameters
    /// 
    /// - `tag`: The tag to add to the device
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }
    
    /// Removes a tag from the device
    /// 
    /// # Parameters
    /// 
    /// - `tag`: The tag to remove from the device
    /// 
    /// # Returns
    /// 
    /// `true` if the tag was removed, `false` if the tag was not found
    pub fn remove_tag(&mut self, tag: &str) -> bool {
        let initial_len = self.tags.len();
        self.tags.retain(|t| t != tag);
        self.tags.len() < initial_len
    }
    
    /// Checks if the device has a specific tag
    /// 
    /// # Parameters
    /// 
    /// - `tag`: The tag to check for
    /// 
    /// # Returns
    /// 
    /// `true` if the device has the tag, `false` otherwise
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
    
    /// Gets the current allocation ID if the device is allocated
    /// 
    /// # Returns
    /// 
    /// An `Option<&str>` containing the allocation ID if the device is allocated, `None` otherwise
    pub fn get_allocation_id(&self) -> Option<&str> {
        self.current_allocation_id.as_deref()
    }
    
    /// Calculates a simple health score based on device status (0-100)
    /// 
    /// This method returns a basic health score based solely on the device status.
    /// 
    /// # Returns
    /// 
    /// A health score between 0 (worst) and 100 (best)
    pub fn health_score(&self) -> u8 {
        match self.status {
            DMSCDeviceStatus::Available => 100,
            DMSCDeviceStatus::Busy => 80,
            DMSCDeviceStatus::Allocated => 80,
            DMSCDeviceStatus::Maintenance => 60,
            DMSCDeviceStatus::Degraded => 40,
            DMSCDeviceStatus::Offline => 20,
            DMSCDeviceStatus::Error => 10,
            DMSCDeviceStatus::Unknown => 0,
        }
    }
    
    /// Checks if the device is still responsive (last seen within the timeout)
    /// 
    /// # Parameters
    /// 
    /// - `timeout_secs`: Maximum number of seconds since last seen for the device to be considered responsive
    /// 
    /// # Returns
    /// 
    /// `true` if the device is responsive, `false` otherwise
    pub fn is_responsive(&self, timeout_secs: i64) -> bool {
        let elapsed = chrono::Utc::now() - self.last_seen;
        elapsed.num_seconds() < timeout_secs
    }
    
    /// Calculates a dynamic health score based on multiple factors (0-100)
    /// 
    /// This method calculates a comprehensive health score based on device status, CPU usage,
    /// memory usage, temperature, and error count.
    /// 
    /// # Parameters
    /// 
    /// - `health_metrics`: Current health metrics for the device
    /// 
    /// # Returns
    /// 
    /// A dynamic health score between 0 (worst) and 100 (best)
    pub fn dynamic_health_score(&self, health_metrics: &DMSCDeviceHealthMetrics) -> u8 {
        let mut score = self.health_score() as f64;
        
        // Adjust score based on CPU usage
        let cpu_penalty = (health_metrics.cpu_usage_percent / 100.0) * 20.0;
        score -= cpu_penalty;
        
        // Adjust score based on memory usage
        let memory_penalty = (health_metrics.memory_usage_percent / 100.0) * 15.0;
        score -= memory_penalty;
        
        // Adjust score based on temperature
        let temp_penalty = if health_metrics.temperature_celsius > 80.0 {
            (health_metrics.temperature_celsius - 80.0) * 2.0
        } else {
            0.0
        };
        score -= temp_penalty;
        
        // Adjust score based on error count
        let error_penalty = (health_metrics.error_count as f64) * 5.0;
        score -= error_penalty;
        
        // Adjust score based on network latency (for network devices)
        if matches!(self.device_type, DMSCDeviceType::Network) {
            let latency_penalty = if health_metrics.network_latency_ms > 100.0 {
                (health_metrics.network_latency_ms - 100.0) * 0.5
            } else {
                0.0
            };
            score -= latency_penalty;
        }
        
        // Adjust score based on disk IOPS (for storage devices)
        if matches!(self.device_type, DMSCDeviceType::Storage) {
            let iops_penalty = if health_metrics.disk_iops < 100 {
                (100.0 - health_metrics.disk_iops as f64) * 0.3
            } else {
                0.0
            };
            score -= iops_penalty;
        }
        
        // Adjust score based on battery level (for mobile/portable devices)
        let battery_penalty = if health_metrics.battery_level_percent < 20.0 {
            (20.0 - health_metrics.battery_level_percent) * 2.0
        } else {
            0.0
        };
        score -= battery_penalty;
        
        // Adjust score based on response time
        let response_time_penalty = if health_metrics.response_time_ms > 50.0 {
            (health_metrics.response_time_ms - 50.0) * 1.0
        } else {
            0.0
        };
        score -= response_time_penalty;
        
        // Ensure score is within 0-100 range
        score.clamp(0.0, 100.0) as u8
    }
    
    /// Checks if the device is healthy based on multiple criteria
    /// 
    /// A device is considered healthy if it is responsive, has a good health score,
    /// and is not in an error or offline state.
    /// 
    /// # Parameters
    /// 
    /// - `health_metrics`: Current health metrics for the device
    /// - `timeout_secs`: Maximum number of seconds since last seen for the device to be considered responsive
    /// 
    /// # Returns
    /// 
    /// `true` if the device is healthy, `false` otherwise
    pub fn is_healthy(&self, health_metrics: &DMSCDeviceHealthMetrics, timeout_secs: i64) -> bool {
        self.is_responsive(timeout_secs) && 
        self.dynamic_health_score(health_metrics) > 50 && 
        self.status != DMSCDeviceStatus::Error && 
        self.status != DMSCDeviceStatus::Offline
    }

    /// Gets a reference to the device metadata
    /// 
    /// # Returns
    /// 
    /// A reference to the metadata HashMap
    pub fn metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDevice {
    #[new]
    fn py_new(name: String, device_type: DMSCDeviceType) -> Self {
        Self::new(name, device_type)
    }
    
    #[staticmethod]
    fn default_device(name: String, device_type: DMSCDeviceType) -> Self {
        Self::new(name, device_type)
    }
    
    fn id_py(&self) -> String {
        self.id().to_string()
    }
    
    fn name_py(&self) -> String {
        self.name().to_string()
    }
    
    fn device_type_py(&self) -> DMSCDeviceType {
        self.device_type()
    }
    
    fn status_py(&self) -> DMSCDeviceStatus {
        self.status()
    }
    
    fn capabilities_py(&self) -> DMSCDeviceCapabilities {
        self.capabilities().clone()
    }
    
    fn health_metrics_py(&self) -> DMSCDeviceHealthMetrics {
        self.health_metrics().clone()
    }
    
    fn set_status_py(&mut self, status: DMSCDeviceStatus) {
        self.set_status(status)
    }
    
    fn update_health_metrics_py(&mut self, metrics: DMSCDeviceHealthMetrics) {
        self.update_health_metrics(metrics)
    }
    
    fn increment_error_count_py(&mut self) {
        self.increment_error_count()
    }
    
    fn update_throughput_py(&mut self, throughput: u64) {
        self.update_throughput(throughput)
    }
    
    fn with_capabilities_py(&self, capabilities: DMSCDeviceCapabilities) -> Self {
        self.clone().with_capabilities(capabilities)
    }
    
    fn set_location_py(&mut self, location: String) {
        self.set_location(location)
    }
    
    fn add_metadata_py(&mut self, key: String, value: String) {
        self.add_metadata(key, value)
    }
    
    fn update_last_seen_py(&mut self) {
        self.update_last_seen()
    }
    
    fn is_available_py(&self) -> bool {
        self.is_available()
    }
    
    fn is_allocated_py(&self) -> bool {
        self.is_allocated()
    }
    
    fn allocate_py(&mut self, allocation_id: String) -> bool {
        self.allocate(&allocation_id)
    }
    
    fn release_py(&mut self) {
        self.release()
    }
    
    fn group_py(&self) -> Option<String> {
        self.group().map(|s| s.to_string())
    }
    
    fn set_group_py(&mut self, group: Option<String>) {
        self.set_group(group)
    }
    
    fn tags_py(&self) -> Vec<String> {
        self.tags().clone()
    }
    
    fn add_tag_py(&mut self, tag: String) {
        self.add_tag(tag)
    }
    
    fn remove_tag_py(&mut self, tag: String) -> bool {
        self.remove_tag(&tag)
    }
    
    fn has_tag_py(&self, tag: String) -> bool {
        self.has_tag(&tag)
    }
    
    fn get_allocation_id_py(&self) -> Option<String> {
        self.get_allocation_id().map(|s| s.to_string())
    }
    
    fn health_score_py(&self) -> u8 {
        self.health_score()
    }
    
    fn is_responsive_py(&self, timeout_secs: i64) -> bool {
        self.is_responsive(timeout_secs)
    }
    
    fn dynamic_health_score_py(&self, health_metrics: DMSCDeviceHealthMetrics) -> u8 {
        self.dynamic_health_score(&health_metrics)
    }
    
    fn is_healthy_py(&self, health_metrics: DMSCDeviceHealthMetrics, timeout_secs: i64) -> bool {
        self.is_healthy(&health_metrics, timeout_secs)
    }
    
    fn metadata_py(&self) -> HashMap<String, String> {
        self.metadata().clone()
    }
}
