//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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
//! This file defines the core data structures for device management in DMS, including device types,
//! capabilities, status, health metrics, and the device representation itself. These structures form
//! the foundation for device discovery, scheduling, and management.
//! 
//! ## Key Components
//! 
//! - **DMSDeviceType**: Enum defining supported device types
//! - **DMSDeviceCapabilities**: Device capabilities structure
//! - **DMSDeviceStatus**: Enum defining device statuses
//! - **DMSDeviceHealthMetrics**: Device health metrics structure
//! - **DMSDevice**: Main device representation with status, capabilities, and health metrics
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
//! use dms::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities};
//! 
//! // Create a new device
//! let mut device = DMSDevice::new("server-1".to_string(), DMSDeviceType::CPU);
//! 
//! // Configure device capabilities
//! let capabilities = DMSDeviceCapabilities::new()
//!     ._Fwith_compute_units(16)
//!     ._Fwith_memory_gb(32.0)
//!     ._Fwith_storage_gb(1024.0)
//!     ._Fwith_bandwidth_gbps(10.0);
//! 
//! // Set device capabilities and status
//! device = device.with_capabilities(capabilities);
//! device._Fset_status(dms::device::DMSDeviceStatus::Available);
//! 
//! // Check if device meets requirements
//! let requirements = DMSDeviceCapabilities::new()._Fwith_compute_units(8);
//! if device.capabilities().meets_requirements(&requirements) {
//!     println!("Device meets requirements");
//! }
//! ```

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Device type enumeration
/// 
/// This enum defines the different types of devices supported by DMS. Each device type
/// has specific capabilities and use cases in the system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DMSDeviceType {
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

/// Device capabilities structure
/// 
/// This struct defines the capabilities of a device, including compute power, memory, storage,
/// bandwidth, and custom capabilities. It supports a fluent builder API for easy configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceCapabilities {
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

impl Default for DMSDeviceCapabilities {
    /// Returns the default device capabilities (empty capabilities)
    fn default() -> Self {
        Self::new()
    }
}

impl DMSDeviceCapabilities {
    /// Creates a new empty device capabilities structure
    /// 
    /// # Returns
    /// 
    /// A new `DMSDeviceCapabilities` instance with no capabilities set
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
    /// The updated `DMSDeviceCapabilities` instance
    pub fn _Fwith_compute_units(mut self, units: usize) -> Self {
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
    /// The updated `DMSDeviceCapabilities` instance
    pub fn _Fwith_memory_gb(mut self, memory: f64) -> Self {
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
    /// The updated `DMSDeviceCapabilities` instance
    pub fn _Fwith_storage_gb(mut self, storage: f64) -> Self {
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
    /// The updated `DMSDeviceCapabilities` instance
    pub fn _Fwith_bandwidth_gbps(mut self, bandwidth: f64) -> Self {
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
    /// The updated `DMSDeviceCapabilities` instance
    pub fn _Fwith_custom_capability(mut self, key: String, value: String) -> Self {
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
    pub fn meets_requirements(&self, requirements: &DMSDeviceCapabilities) -> bool {
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

/// Device status enumeration
/// 
/// This enum defines the different statuses a device can have during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSDeviceStatus {
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
}

/// Device health metrics structure
/// 
/// This struct contains health metrics for monitoring device performance and reliability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceHealthMetrics {
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
}

impl Default for DMSDeviceHealthMetrics {
    /// Returns default health metrics (all zero values)
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            memory_usage_percent: 0.0,
            temperature_celsius: 0.0,
            error_count: 0,
            throughput: 0,
        }
    }
}

/// Smart device representation
/// 
/// This struct represents a smart device in the DMS system, including its status, capabilities,
/// health metrics, and lifecycle information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDevice {
    /// Unique device ID
    id: String,
    /// Device name
    name: String,
    /// Device type
    device_type: DMSDeviceType,
    /// Current device status
    status: DMSDeviceStatus,
    /// Device capabilities for resource allocation
    capabilities: DMSDeviceCapabilities,
    /// Current health metrics
    health_metrics: DMSDeviceHealthMetrics,
    /// Physical location of the device (optional)
    location: Option<String>,
    /// Additional metadata as key-value pairs
    metadata: HashMap<String, String>,
    /// Last time the device was seen/updated
    last_seen: chrono::DateTime<chrono::Utc>,
    /// ID of the current allocation using this device (if any)
    current_allocation_id: Option<String>,
}

impl DMSDevice {
    /// Creates a new device with the given name and type
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the device
    /// - `device_type`: The type of the device
    /// 
    /// # Returns
    /// 
    /// A new `DMSDevice` instance with default capabilities and health metrics
    pub fn new(name: String, device_type: DMSDeviceType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            device_type,
            status: DMSDeviceStatus::Unknown,
            capabilities: DMSDeviceCapabilities::new(),
            health_metrics: DMSDeviceHealthMetrics::default(),
            location: None,
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
    pub fn _Fid(&self) -> &str {
        &self.id
    }
    
    /// Gets the device name
    /// 
    /// # Returns
    /// 
    /// The device name as a string slice
    pub fn _Fname(&self) -> &str {
        &self.name
    }
    
    /// Gets the device type
    /// 
    /// # Returns
    /// 
    /// The device type as a `DMSDeviceType` enum
    pub fn _Fdevice_type(&self) -> DMSDeviceType {
        self.device_type
    }
    
    /// Gets the current device status
    /// 
    /// # Returns
    /// 
    /// The device status as a `DMSDeviceStatus` enum
    pub fn _Fstatus(&self) -> DMSDeviceStatus {
        self.status
    }
    
    /// Gets a reference to the device capabilities
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSDeviceCapabilities` structure
    pub fn capabilities(&self) -> &DMSDeviceCapabilities {
        &self.capabilities
    }
    
    /// Gets a reference to the device health metrics
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSDeviceHealthMetrics` structure
    pub fn health_metrics(&self) -> &DMSDeviceHealthMetrics {
        &self.health_metrics
    }
    
    /// Sets the device status and updates the last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `status`: The new status to set
    pub fn _Fset_status(&mut self, status: DMSDeviceStatus) {
        self.status = status;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Updates the device health metrics and last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `metrics`: The new health metrics to set
    pub fn _Fupdate_health_metrics(&mut self, metrics: DMSDeviceHealthMetrics) {
        self.health_metrics = metrics;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Increments the device error count and updates the last seen timestamp
    pub fn _Fincrement_error_count(&mut self) {
        self.health_metrics.error_count += 1;
        self.last_seen = chrono::Utc::now();
    }
    
    /// Updates the device throughput and last seen timestamp
    /// 
    /// # Parameters
    /// 
    /// - `throughput`: The new throughput value in operations per second
    pub fn _Fupdate_throughput(&mut self, throughput: u64) {
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
    /// The updated `DMSDevice` instance
    pub fn with_capabilities(mut self, capabilities: DMSDeviceCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }
    
    /// Sets the device location
    /// 
    /// # Parameters
    /// 
    /// - `location`: The physical location of the device
    pub fn _Fset_location(&mut self, location: String) {
        self.location = Some(location);
    }
    
    /// Adds a metadata key-value pair to the device
    /// 
    /// # Parameters
    /// 
    /// - `key`: Metadata key
    /// - `value`: Metadata value
    pub fn _Fadd_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    /// Updates the last seen timestamp to the current time
    pub fn _Fupdate_last_seen(&mut self) {
        self.last_seen = chrono::Utc::now();
    }
    
    /// Gets the last seen timestamp
    /// 
    /// # Returns
    /// 
    /// The last seen timestamp as a `chrono::DateTime<chrono::Utc>`
    pub fn _Flast_seen(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_seen
    }
    
    /// Checks if the device is available for allocation
    /// 
    /// A device is available if its status is Available and it has no current allocation.
    /// 
    /// # Returns
    /// 
    /// `true` if the device is available, `false` otherwise
    pub fn _Fis_available(&self) -> bool {
        self.status == DMSDeviceStatus::Available && self.current_allocation_id.is_none()
    }
    
    /// Checks if the device is currently allocated
    /// 
    /// # Returns
    /// 
    /// `true` if the device is allocated, `false` otherwise
    pub fn _Fis_allocated(&self) -> bool {
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
    pub fn _Fallocate(&mut self, allocation_id: &str) -> bool {
        if self._Fis_available() {
            self.current_allocation_id = Some(allocation_id.to_string());
            self.status = DMSDeviceStatus::Busy;
            true
        } else {
            false
        }
    }
    
    /// Releases the device from its current allocation
    /// 
    /// This method clears the allocation ID and sets the device status to Available if it was Busy.
    pub fn _Frelease(&mut self) {
        self.current_allocation_id = None;
        if self.status == DMSDeviceStatus::Busy {
            self.status = DMSDeviceStatus::Available;
        }
    }
    
    /// Gets the current allocation ID if the device is allocated
    /// 
    /// # Returns
    /// 
    /// An `Option<&str>` containing the allocation ID if the device is allocated, `None` otherwise
    pub fn _Fget_allocation_id(&self) -> Option<&str> {
        self.current_allocation_id.as_deref()
    }
    
    /// Calculates a simple health score based on device status (0-100)
    /// 
    /// This method returns a basic health score based solely on the device status.
    /// 
    /// # Returns
    /// 
    /// A health score between 0 (worst) and 100 (best)
    pub fn _Fhealth_score(&self) -> u8 {
        match self.status {
            DMSDeviceStatus::Available => 100,
            DMSDeviceStatus::Busy => 80,
            DMSDeviceStatus::Maintenance => 60,
            DMSDeviceStatus::Offline => 20,
            DMSDeviceStatus::Error => 10,
            DMSDeviceStatus::Unknown => 0,
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
    pub fn _Fis_responsive(&self, timeout_secs: i64) -> bool {
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
    pub fn _Fdynamic_health_score(&self, health_metrics: &DMSDeviceHealthMetrics) -> u8 {
        let mut score = self._Fhealth_score() as f64;
        
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
    pub fn _Fis_healthy(&self, health_metrics: &DMSDeviceHealthMetrics, timeout_secs: i64) -> bool {
        self._Fis_responsive(timeout_secs) && 
        self._Fdynamic_health_score(health_metrics) > 50 && 
        self.status != DMSDeviceStatus::Error && 
        self.status != DMSDeviceStatus::Offline
    }
}