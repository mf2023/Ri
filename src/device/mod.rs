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

//! # Device Control Module
//! 
//! This module provides comprehensive smart device control functionality for DMSC, including device
//! discovery, control, and resource scheduling. It enables efficient management of devices and
//! their resources across distributed environments.
//! 
//! ## Key Components
//! 
//! - **DMSCDeviceControlModule**: Main device control module implementing service module traits
//! - **DMSCDevice**: Device representation with type, status, and capabilities
//! - **DMSCDeviceType**: Enum defining supported device types
//! - **DMSCDeviceStatus**: Enum defining device statuses
//! - **DMSCDeviceCapabilities**: Device capabilities structure
//! - **DMSCDeviceController**: Device controller for managing devices
//! - **DMSCDeviceScheduler**: Device scheduler for resource allocation
//! - **DMSCResourcePool**: Resource pool for managing device resources
//! - **DMSCResourcePoolManager**: Manager for multiple resource pools
//! - **DMSCResourcePoolStatistics**: Statistics for resource pool monitoring
//! - **DMSCDeviceControlConfig**: Configuration for device control behavior
//! - **DMSCDiscoveryResult**: Result structure for device discovery
//! - **DMSCResourceRequest**: Request structure for resource allocation
//! - **DMSCResourceAllocation**: Result structure for resource allocation
//! - **DMSCResourcePoolStatus**: Status structure for resource pools
//! 
//! ## Design Principles
//! 
//! 1. **Device Abstraction**: Unified interface for different device types
//! 2. **Auto Discovery**: Automatic device discovery in the network/environment
//! 3. **Resource Scheduling**: Intelligent resource allocation and scheduling
//! 4. **Configurable**: Highly configurable device control behavior
//! 5. **Async Support**: Full async/await compatibility
//! 6. **Resource Pooling**: Efficient management of device resources through pooling
//! 7. **Service Module Integration**: Implements service module traits for seamless integration
//! 8. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 9. **Non-critical**: Device control failures should not break the entire application
//! 10. **Monitoring**: Comprehensive statistics for device and resource monitoring
//! 11. **Scalable**: Designed to handle large numbers of devices and concurrent tasks
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! use dmsc::device::{DMSCDeviceControlConfig, DMSCResourceRequest, DMSCDeviceType, DMSCDeviceCapabilities};
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create device control configuration
//!     let device_config = DMSCDeviceControlConfig {
//!         discovery_enabled: true,
//!         discovery_interval_secs: 30,
//!         auto_scheduling_enabled: true,
//!         max_concurrent_tasks: 100,
//!         resource_allocation_timeout_secs: 60,
//!     };
//!     
//!     // Create device control module
//!     let device_module = DMSCDeviceControlModule::new()
//!         .with_config(device_config);
//!     
//!     // Discover devices
//!     let discovery_result = device_module.discover_devices().await?;
//!     println!("Discovered {} devices, total devices: {}", 
//!              discovery_result.discovered_devices.len(), 
//!              discovery_result.total_devices);
//!     
//!     // Get device status
//!     let devices = device_module.get_device_status().await?;
//!     println!("Current devices: {:?}", devices);
//!     
//!     // Create resource request
//!     let resource_request = DMSCResourceRequest {
//!         request_id: "request-123".to_string(),
//!         device_type: DMSCDeviceType::Compute,
//!         required_capabilities: DMSCDeviceCapabilities {
//!             cpu_cores: Some(4),
//!             memory_gb: Some(8.0),
//!             storage_gb: Some(100.0),
//!             gpu_enabled: Some(true),
//!             network_speed_mbps: Some(1000.0),
//!             extra: Default::default(),
//!         },
//!         priority: 5,
//!         timeout_secs: 30,
//!     };
//!     
//!     // Allocate resource
//!     if let Some(allocation) = device_module.allocate_resource(resource_request).await? {
//!         println!("Allocated device: {} (ID: {})", 
//!                  allocation.device_name, 
//!                  allocation.device_id);
//!         
//!         // Release resource after use
//!         device_module.release_resource(&allocation.allocation_id).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod core;
mod controller;
mod scheduler;
pub mod pool;
pub mod discovery_scheduler;
pub mod discovery;

use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::collections::HashMap;

use crate::observability::{DMSCMetricsRegistry, DMSCMetric, DMSCMetricConfig, DMSCMetricType};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;


pub use core::{DMSCDevice, DMSCDeviceType, DMSCDeviceStatus, DMSCDeviceCapabilities, DMSCDeviceControlConfig, DMSCDeviceConfig, NetworkDeviceInfo, DMSCDeviceHealthMetrics};
pub use controller::DMSCDeviceController;
pub use pool::{DMSCResourcePool, DMSCResourcePoolManager, DMSCConnectionPoolStatistics};
pub use scheduler::DMSCDeviceScheduler;
pub use discovery_scheduler::{DMSCDeviceDiscoveryEngine, DMSCResourceScheduler};

// Re-export discovery module types
pub use discovery::{
    DMSCDeviceDiscovery,
    DiscoveryConfig,
    DiscoveryStats,
    DiscoveryStrategy,
    HardwareCategory,
    PlatformInfo,
    PlatformType,
    Architecture,
    PlatformCompatibility,
    ProviderRegistry,
    DMSCHardwareProvider,
    PluginRegistry,
    DMSCHardwareDiscoveryPlugin,
    PluginMetadata,
    PluginStatus,
    PluginError,
    AsyncDiscovery,
};

use crate::core::{DMSCResult, DMSCServiceContext};


/// Main device control module for DMSC.
/// 
/// This module provides comprehensive smart device control functionality, including device discovery,
/// control, and resource scheduling. It manages devices and their resources across distributed environments.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceControlModule {
    /// Device controller for managing devices
    controller: Arc<RwLock<DMSCDeviceController>>,
    /// Device scheduler for resource allocation
    scheduler: Arc<RwLock<DMSCDeviceScheduler>>,
    /// Discovery engine for device discovery
    discovery_engine: Arc<RwLock<DMSCResourceScheduler>>,
    /// Map of resource pool names to resource pool instances
    resource_pools: HashMap<String, Arc<DMSCResourcePool>>,
    /// Device control configuration
    config: DMSCDeviceControlConfig,
}

/// Scheduling configuration for device control module
///
/// This configuration contains scheduling and resource management settings
/// for device control operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDeviceSchedulingConfig {
    /// Whether device discovery is enabled
    pub discovery_enabled: bool,
    /// Interval between device discovery scans in seconds
    pub discovery_interval_secs: u64,
    /// Whether automatic resource scheduling is enabled
    pub auto_scheduling_enabled: bool,
    /// Maximum number of concurrent tasks
    pub max_concurrent_tasks: usize,
    /// Timeout for resource allocation in seconds
    pub resource_allocation_timeout_secs: u64,
}

impl Default for DMSCDeviceSchedulingConfig {
    fn default() -> Self {
        Self {
            discovery_enabled: true,
            discovery_interval_secs: 30,
            auto_scheduling_enabled: true,
            max_concurrent_tasks: 100,
            resource_allocation_timeout_secs: 60,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDeviceSchedulingConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_config() -> Self {
        Self::default()
    }
}

/// Result structure for device discovery operations.
/// 
/// This struct contains information about the results of a device discovery scan, including
/// discovered, updated, and removed devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCDiscoveryResult {
    /// Newly discovered devices
    pub discovered_devices: Vec<DMSCDevice>,
    /// Devices with updated information
    pub updated_devices: Vec<DMSCDevice>,
    /// IDs of removed devices
    pub removed_devices: Vec<String>, // device IDs
    /// Total number of devices after discovery
    pub total_devices: usize,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCDiscoveryResult {
    #[new]
    fn py_new() -> Self {
        Self {
            discovered_devices: Vec::new(),
            updated_devices: Vec::new(),
            removed_devices: Vec::new(),
            total_devices: 0,
        }
    }
    
    #[staticmethod]
    fn default_result() -> Self {
        Self::default()
    }
    
    fn discovered_devices_impl(&self) -> Vec<DMSCDevice> {
        self.discovered_devices.clone()
    }
    
    fn updated_devices_impl(&self) -> Vec<DMSCDevice> {
        self.updated_devices.clone()
    }
    
    fn removed_devices_impl(&self) -> Vec<String> {
        self.removed_devices.clone()
    }
    
    fn total_devices_impl(&self) -> usize {
        self.total_devices
    }
    
    fn __str__(&self) -> String {
        format!("DMSCDiscoveryResult(discovered: {}, updated: {}, removed: {}, total: {})", 
                self.discovered_devices.len(), self.updated_devices.len(), 
                self.removed_devices.len(), self.total_devices)
    }
}

impl Default for DMSCDiscoveryResult {
    fn default() -> Self {
        Self {
            discovered_devices: Vec::new(),
            updated_devices: Vec::new(),
            removed_devices: Vec::new(),
            total_devices: 0,
        }
    }
}

/// Request structure for resource allocation.
/// 
/// This struct defines the requirements for resource allocation, including device type, capabilities,
/// priority, timeout, and advanced scheduling preferences such as SLA, resource weights,
/// and affinity rules. New fields are optional to preserve backward compatibility.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCResourceRequest {
    /// Unique request ID
    pub request_id: String,
    /// Required device type
    pub device_type: DMSCDeviceType,
    /// Required device capabilities
    pub required_capabilities: DMSCDeviceCapabilities,
    /// Request priority (1-10, higher is more important)
    pub priority: u8, // 1-10, higher is more important
    /// Request timeout in seconds
    pub timeout_secs: u64,
    /// Optional SLA class for this request (e.g. Critical / High / Medium / Low)
    pub sla_class: Option<DMSCRequestSlaClass>,
    /// Optional multi-dimensional resource weights to influence scheduling decisions
    pub resource_weights: Option<DMSCResourceWeights>,
    /// Optional affinity rules describing preferred/required device labels
    pub affinity: Option<DMSCAffinityRules>,
    /// Optional anti-affinity rules describing labels or devices to avoid
    pub anti_affinity: Option<DMSCAffinityRules>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCResourceRequest {
    #[new]
    #[pyo3(signature = (request_id, device_type, required_capabilities, priority=5, timeout_secs=60))]
    fn py_new(request_id: String, device_type: DMSCDeviceType, required_capabilities: DMSCDeviceCapabilities, priority: u8, timeout_secs: u64) -> Self {
        Self {
            request_id,
            device_type,
            required_capabilities,
            priority,
            timeout_secs,
            sla_class: None,
            resource_weights: None,
            affinity: None,
            anti_affinity: None,
        }
    }
    
    #[pyo3(name = "request_id")]
    fn request_id_impl(&self) -> String {
        self.request_id.clone()
    }
    
    #[pyo3(name = "device_type")]
    fn device_type_impl(&self) -> DMSCDeviceType {
        self.device_type
    }
    
    #[pyo3(name = "required_capabilities")]
    fn required_capabilities_impl(&self) -> DMSCDeviceCapabilities {
        self.required_capabilities.clone()
    }
    
    #[pyo3(name = "priority")]
    fn priority_impl(&self) -> u8 {
        self.priority
    }
    
    #[pyo3(name = "timeout_secs")]
    fn timeout_secs_impl(&self) -> u64 {
        self.timeout_secs
    }
    
    #[pyo3(name = "sla_class")]
    fn sla_class_impl(&self) -> Option<DMSCRequestSlaClass> {
        self.sla_class
    }
    
    #[pyo3(name = "resource_weights")]
    fn resource_weights_impl(&self) -> Option<DMSCResourceWeights> {
        self.resource_weights.clone()
    }
    
    #[pyo3(name = "affinity")]
    fn affinity_impl(&self) -> Option<DMSCAffinityRules> {
        self.affinity.clone()
    }
    
    #[pyo3(name = "anti_affinity")]
    fn anti_affinity_impl(&self) -> Option<DMSCAffinityRules> {
        self.anti_affinity.clone()
    }
    
    #[pyo3(name = "set_priority")]
    fn set_priority_impl(&mut self, priority: u8) {
        self.priority = priority;
    }
    
    #[pyo3(name = "set_timeout_secs")]
    fn set_timeout_secs_impl(&mut self, timeout_secs: u64) {
        self.timeout_secs = timeout_secs;
    }
    
    #[pyo3(name = "set_sla_class")]
    fn set_sla_class_impl(&mut self, sla_class: Option<DMSCRequestSlaClass>) {
        self.sla_class = sla_class;
    }
    
    #[pyo3(name = "set_resource_weights")]
    fn set_resource_weights_impl(&mut self, resource_weights: Option<DMSCResourceWeights>) {
        self.resource_weights = resource_weights;
    }
    
    #[pyo3(name = "set_affinity")]
    fn set_affinity_impl(&mut self, affinity: Option<DMSCAffinityRules>) {
        self.affinity = affinity;
    }
    
    #[pyo3(name = "set_anti_affinity")]
    fn set_anti_affinity_impl(&mut self, anti_affinity: Option<DMSCAffinityRules>) {
        self.anti_affinity = anti_affinity;
    }
    
    fn __str__(&self) -> String {
        format!("DMSCResourceRequest(id: {}, type: {:?}, priority: {}, timeout: {}s)", 
                self.request_id, self.device_type, self.priority, self.timeout_secs)
    }
}

/// SLA class for a resource request.
/// 
/// This enum describes the service level expectations for a request. Schedulers can
/// use this information to trade off between latency, availability, and resource usage.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCRequestSlaClass {
    /// Mission critical requests that should be served with the highest priority
    Critical,
    /// High priority requests
    High,
    /// Normal priority requests
    Medium,
    /// Low priority / best-effort requests
    Low,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCRequestSlaClass {
    fn __str__(&self) -> String {
        match self {
            DMSCRequestSlaClass::Critical => "Critical".to_string(),
            DMSCRequestSlaClass::High => "High".to_string(),
            DMSCRequestSlaClass::Medium => "Medium".to_string(),
            DMSCRequestSlaClass::Low => "Low".to_string(),
        }
    }
}

/// Multi-dimensional resource weights for scheduling.
/// 
/// This struct allows callers to express how important different resource dimensions are
/// (compute, memory, storage, bandwidth) for a specific request. Schedulers can use these
/// weights when computing fitness scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCResourceWeights {
    /// Weight for compute resources (e.g. CPU cores, GPU units)
    pub compute_weight: f64,
    /// Weight for memory capacity
    pub memory_weight: f64,
    /// Weight for storage capacity
    pub storage_weight: f64,
    /// Weight for network bandwidth
    pub bandwidth_weight: f64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCResourceWeights {
    #[new]
    #[pyo3(signature = (compute_weight=1.0, memory_weight=1.0, storage_weight=1.0, bandwidth_weight=1.0))]
    fn py_new(compute_weight: f64, memory_weight: f64, storage_weight: f64, bandwidth_weight: f64) -> Self {
        Self {
            compute_weight,
            memory_weight,
            storage_weight,
            bandwidth_weight,
        }
    }
    
    #[staticmethod]
    fn default_weights() -> Self {
        Self::default()
    }
    
    #[pyo3(name = "compute_weight")]
    fn compute_weight_impl(&self) -> f64 { self.compute_weight }
    #[pyo3(name = "memory_weight")]
    fn memory_weight_impl(&self) -> f64 { self.memory_weight }
    #[pyo3(name = "storage_weight")]
    fn storage_weight_impl(&self) -> f64 { self.storage_weight }
    #[pyo3(name = "bandwidth_weight")]
    fn bandwidth_weight_impl(&self) -> f64 { self.bandwidth_weight }
    
    #[pyo3(name = "set_compute_weight")]
    fn set_compute_weight_impl(&mut self, weight: f64) { self.compute_weight = weight; }
    #[pyo3(name = "set_memory_weight")]
    fn set_memory_weight_impl(&mut self, weight: f64) { self.memory_weight = weight; }
    #[pyo3(name = "set_storage_weight")]
    fn set_storage_weight_impl(&mut self, weight: f64) { self.storage_weight = weight; }
    #[pyo3(name = "set_bandwidth_weight")]
    fn set_bandwidth_weight_impl(&mut self, weight: f64) { self.bandwidth_weight = weight; }
    
    fn __str__(&self) -> String {
        format!("DMSCResourceWeights(compute: {}, memory: {}, storage: {}, bandwidth: {})", 
                self.compute_weight, self.memory_weight, self.storage_weight, self.bandwidth_weight)
    }
}

impl Default for DMSCResourceWeights {
    fn default() -> Self {
        Self {
            compute_weight: 1.0,
            memory_weight: 1.0,
            storage_weight: 1.0,
            bandwidth_weight: 1.0,
        }
    }
}

/// Affinity and anti-affinity rules for device selection.
/// 
/// Rules are expressed as label key/value pairs. Implementations can interpret labels
/// using device metadata such as location, zone, rack, tenant, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCAffinityRules {
    /// Labels that must be present with matching values
    pub required_labels: HashMap<String, String>,
    /// Labels that are preferred (but not strictly required)
    pub preferred_labels: HashMap<String, String>,
    /// Labels that must not be present with matching values
    pub forbidden_labels: HashMap<String, String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCAffinityRules {
    #[new]
    fn py_new() -> Self {
        Self {
            required_labels: HashMap::new(),
            preferred_labels: HashMap::new(),
            forbidden_labels: HashMap::new(),
        }
    }
    
    #[staticmethod]
    fn default_rules() -> Self {
        Self::default()
    }
    
    #[pyo3(name = "required_labels")]
    fn required_labels_impl(&self) -> HashMap<String, String> {
        self.required_labels.clone()
    }
    
    #[pyo3(name = "preferred_labels")]
    fn preferred_labels_impl(&self) -> HashMap<String, String> {
        self.preferred_labels.clone()
    }
    
    #[pyo3(name = "forbidden_labels")]
    fn forbidden_labels_impl(&self) -> HashMap<String, String> {
        self.forbidden_labels.clone()
    }
    
    fn __str__(&self) -> String {
        format!("DMSCAffinityRules(required: {}, preferred: {}, forbidden: {})", 
                self.required_labels.len(), self.preferred_labels.len(), self.forbidden_labels.len())
    }
}

impl Default for DMSCAffinityRules {
    fn default() -> Self {
        Self {
            required_labels: HashMap::new(),
            preferred_labels: HashMap::new(),
            forbidden_labels: HashMap::new(),
        }
    }
}

/// Result structure for resource allocation.
/// 
/// This struct contains information about a successful resource allocation, including the allocated
/// device, allocation time, and expiration time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCResourceAllocation {
    /// Unique allocation ID
    pub allocation_id: String,
    /// ID of the allocated device
    pub device_id: String,
    /// Name of the allocated device
    pub device_name: String,
    /// Time when the resource was allocated
    pub allocated_at: chrono::DateTime<chrono::Utc>,
    /// Time when the allocation expires
    pub expires_at: chrono::DateTime<chrono::Utc>,
    /// Original resource request
    pub request: DMSCResourceRequest,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCResourceAllocation {
    #[new]
    fn py_new(allocation_id: String, device_id: String, device_name: String, request: DMSCResourceRequest) -> Self {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::TimeDelta::seconds(request.timeout_secs as i64);
        
        Self {
            allocation_id,
            device_id,
            device_name,
            allocated_at: now,
            expires_at,
            request,
        }
    }
    
    #[pyo3(name = "allocation_id")]
    fn allocation_id_impl(&self) -> String {
        self.allocation_id.clone()
    }
    
    #[pyo3(name = "device_id")]
    fn device_id_impl(&self) -> String {
        self.device_id.clone()
    }
    
    #[pyo3(name = "device_name")]
    fn device_name_impl(&self) -> String {
        self.device_name.clone()
    }
    
    #[pyo3(name = "allocated_at")]
    fn allocated_at_impl(&self) -> String {
        self.allocated_at.to_rfc3339()
    }
    
    #[pyo3(name = "expires_at")]
    fn expires_at_impl(&self) -> String {
        self.expires_at.to_rfc3339()
    }
    
    #[pyo3(name = "request")]
    fn request_impl(&self) -> DMSCResourceRequest {
        self.request.clone()
    }
    
    #[pyo3(name = "is_expired")]
    fn is_expired_impl(&self) -> bool {
        chrono::Utc::now() > self.expires_at
    }
    
    #[pyo3(name = "remaining_time")]
    fn remaining_time_impl(&self) -> i64 {
        (self.expires_at - chrono::Utc::now()).num_seconds()
    }
    
    fn __str__(&self) -> String {
        format!("DMSCResourceAllocation(id: {}, device: {} ({}), expires: {})", 
                self.allocation_id, self.device_name, self.device_id, self.expires_at)
    }
}

impl Default for DMSCDeviceControlModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCDeviceControlModule {
    /// Creates a new device control module with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSCDeviceControlModule` instance with default configuration
    pub fn new() -> Self {
        let controller = Arc::new(RwLock::new(DMSCDeviceController::new()));
        let resource_pool_manager = Arc::new(RwLock::new(DMSCResourcePoolManager::new()));
        let scheduler = Arc::new(RwLock::new(DMSCDeviceScheduler::new(resource_pool_manager)));
        let discovery_engine = Arc::new(RwLock::new(DMSCResourceScheduler::new()));
        
        Self {
            controller,
            scheduler,
            discovery_engine,
            resource_pools: HashMap::new(),
            config: crate::device::core::DMSCDeviceControlConfig::default(),
        }
    }
    
    /// Configures the device control module with custom settings.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The custom configuration to apply
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCDeviceControlModule` instance
    pub fn with_config(mut self, config: crate::device::core::DMSCDeviceControlConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Discovers devices in the network/environment.
    /// 
    /// This method performs a device discovery scan if discovery is enabled, returning information
    /// about discovered, updated, and removed devices.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<DMSCDiscoveryResult>` containing the discovery results
    pub async fn discover_devices(&self) -> DMSCResult<DMSCDiscoveryResult> {
        if !self.config.enable_cpu_discovery && !self.config.enable_gpu_discovery && 
           !self.config.enable_memory_discovery && !self.config.enable_storage_discovery && 
           !self.config.enable_network_discovery {
            return Ok(DMSCDiscoveryResult {
                discovered_devices: vec![],
                updated_devices: vec![],
                removed_devices: vec![],
                total_devices: 0,
            });
        }
        
        let mut controller = self.controller.write().await;
        controller.discover_devices().await
    }
    
    /// Allocates a device resource based on the given request.
    /// 
    /// This method finds a suitable device based on the requested device type and capabilities,
    /// allocates it using the device scheduler, and returns an allocation result if successful.
    /// 
    /// # Parameters
    /// 
    /// - `request`: The resource allocation request
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Option<DMSCResourceAllocation>>` containing the allocation result if successful,
    /// or None if allocation failed or auto-scheduling is disabled
    pub async fn allocate_resource(&self, request: DMSCResourceRequest) -> DMSCResult<Option<DMSCResourceAllocation>> {
        // Check if any device type scheduling is enabled
        let scheduling_enabled = match request.device_type {
            DMSCDeviceType::CPU => self.config.enable_cpu_discovery,
            DMSCDeviceType::GPU => self.config.enable_gpu_discovery,
            DMSCDeviceType::Memory => self.config.enable_memory_discovery,
            DMSCDeviceType::Storage => self.config.enable_storage_discovery,
            DMSCDeviceType::Network => self.config.enable_network_discovery,
            _ => true, // Default to enabled for unknown types
        };
        
        if !scheduling_enabled {
            return Ok(None);
        }

        let allocation_request = crate::device::scheduler::DMSCAllocationRequest {
            device_type: request.device_type,
            capabilities: request.required_capabilities,
            priority: request.priority as u32,
            timeout_secs: request.timeout_secs,
            sla_class: request.sla_class,
            resource_weights: request.resource_weights,
            affinity: request.affinity,
            anti_affinity: request.anti_affinity,
        };

        let scheduler = self.scheduler.write().await;
        let device = scheduler.select_device(&allocation_request).await;

        if let Some(device) = device {
            let allocation = DMSCResourceAllocation {
                allocation_id: uuid::Uuid::new_v4().to_string(),
                device_id: device.id().to_string(),
                device_name: device.name().to_string(),
                allocated_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::seconds(allocation_request.timeout_secs as i64),
                request: DMSCResourceRequest {
                    request_id: request.request_id,
                    device_type: allocation_request.device_type,
                    required_capabilities: allocation_request.capabilities,
                    priority: request.priority,
                    timeout_secs: allocation_request.timeout_secs,
                    sla_class: allocation_request.sla_class,
                    resource_weights: allocation_request.resource_weights,
                    affinity: allocation_request.affinity,
                    anti_affinity: allocation_request.anti_affinity,
                },
            };

            // Mark device as busy via controller
            let mut controller = self.controller.write().await;
            controller.allocate_device(&allocation.device_id, &allocation.allocation_id).await?;

            Ok(Some(allocation))
        } else {
            Ok(None)
        }
    }
    
    /// Releases a previously allocated device resource.
    /// 
    /// This method releases a device resource that was allocated with `allocate_resource`.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: The ID of the allocation to release
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn release_resource(&self, allocation_id: &str) -> DMSCResult<()> {
        let mut controller = self.controller.write().await;
        controller.release_device_by_allocation(allocation_id).await
    }
    
    /// Gets the current status of all devices.
    /// 
    /// This method returns a list of all devices currently managed by the device controller.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Vec<DMSCDevice>>` containing all managed devices
    pub async fn get_device_status(&self) -> DMSCResult<Vec<DMSCDevice>> {
        let controller = self.controller.read().await;
        Ok(controller.get_all_devices())
    }
    
    /// Gets the status of all resource pools.
    /// 
    /// This method returns a map of resource pool names to their current status.
    /// 
    /// # Returns
    /// 
    /// A `HashMap<String, DMSCResourcePoolStatus>` containing the status of all resource pools
    pub fn get_resource_pool_status(&self) -> HashMap<String, DMSCResourcePoolStatus> {
        let mut status = HashMap::new();
        for (pool_name, pool) in &self.resource_pools {
            status.insert(pool_name.clone(), pool.get_status());
        }
        status
    }
    
    /// Creates device management metrics and registers them with the metrics registry.
    /// 
    /// This method creates and registers the following metrics:
    /// - dms_device_total: Total number of devices by type and status
    /// - dms_device_allocation_attempts_total: Total number of allocation attempts
    /// - dms_device_allocation_success_total: Total number of successful allocations
    /// - dms_device_allocation_failure_total: Total number of failed allocations
    /// - dms_device_discovery_attempts_total: Total number of device discovery attempts
    /// - dms_device_discovery_success_total: Total number of successful device discoveries
    /// - dms_device_resource_utilization: Resource utilization by device type
    /// 
    /// # Parameters
    /// 
    /// - `registry`: The metrics registry to register the metrics with
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    #[allow(dead_code)]
    fn create_device_metrics(&self, registry: Arc<DMSCMetricsRegistry>) -> DMSCResult<()> {
        // Device total metric (Gauge)
        let device_total_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Gauge,
            name: "dms_device_total".to_string(),
            help: "Total number of devices by type and status".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(300),
            age_buckets: 5,
        };
        let device_total_metric = Arc::new(DMSCMetric::new(device_total_config));
        registry.register(device_total_metric)?;
        
        // Allocation attempts metric (Counter)
        let allocation_attempts_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Counter,
            name: "dms_device_allocation_attempts_total".to_string(),
            help: "Total number of device allocation attempts".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_attempts_metric = Arc::new(DMSCMetric::new(allocation_attempts_config));
        registry.register(allocation_attempts_metric)?;
        
        // Allocation success metric (Counter)
        let allocation_success_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Counter,
            name: "dms_device_allocation_success_total".to_string(),
            help: "Total number of successful device allocations".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_success_metric = Arc::new(DMSCMetric::new(allocation_success_config));
        registry.register(allocation_success_metric)?;
        
        // Allocation failure metric (Counter)
        let allocation_failure_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Counter,
            name: "dms_device_allocation_failure_total".to_string(),
            help: "Total number of failed device allocations".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_failure_metric = Arc::new(DMSCMetric::new(allocation_failure_config));
        registry.register(allocation_failure_metric)?;
        
        // Discovery attempts metric (Counter)
        let discovery_attempts_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Counter,
            name: "dms_device_discovery_attempts_total".to_string(),
            help: "Total number of device discovery attempts".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let discovery_attempts_metric = Arc::new(DMSCMetric::new(discovery_attempts_config));
        registry.register(discovery_attempts_metric)?;
        
        // Discovery success metric (Counter)
        let discovery_success_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Counter,
            name: "dms_device_discovery_success_total".to_string(),
            help: "Total number of successful device discoveries".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let discovery_success_metric = Arc::new(DMSCMetric::new(discovery_success_config));
        registry.register(discovery_success_metric)?;
        
        // Resource utilization metric (Gauge)
        let resource_utilization_config = DMSCMetricConfig {
            metric_type: DMSCMetricType::Gauge,
            name: "dms_device_resource_utilization".to_string(),
            help: "Resource utilization by device type".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(300),
            age_buckets: 5,
        };
        let resource_utilization_metric = Arc::new(DMSCMetric::new(resource_utilization_config));
        registry.register(resource_utilization_metric)?;
        
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCDeviceControlModule {
    fn get_config(&self) -> String {
        format!("{:?}", self.config)
    }
}

/// Status structure for resource pools.
/// 
/// This struct contains information about the current status of a resource pool, including capacity,
/// allocation, and utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCResourcePoolStatus {
    /// Total capacity of the resource pool
    pub total_capacity: usize,
    /// Available capacity in the resource pool
    pub available_capacity: usize,
    /// Allocated capacity in the resource pool
    pub allocated_capacity: usize,
    /// Number of pending resource requests
    pub pending_requests: usize,
    /// Resource utilization rate (0.0 to 1.0)
    pub utilization_rate: f64,
}

#[async_trait::async_trait]
impl crate::core::DMSCModule for DMSCDeviceControlModule {
    /// Returns the name of the device control module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.DeviceControl"
    }
    
    /// Indicates whether the device control module is critical.
    /// 
    /// The device control module is non-critical, meaning that if it fails to initialize or operate,
    /// it should not break the entire application. This allows the core functionality to continue
    /// even if device control features are unavailable.
    /// 
    /// # Returns
    /// 
    /// `false` since device control is non-critical
    fn is_critical(&self) -> bool {
        false // Non-critical, should not break the app if device control fails
    }
    
    /// Initializes the device control module asynchronously.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Initializes real device discovery based on system hardware
    /// 3. Sets up resource scheduling and management
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        let binding = ctx.config();
        let cfg = binding.config();
        let device_config = parse_device_config(cfg.get("device"));
        
        let mut controller = self.controller.write().await;
        controller.discover_system_devices(&device_config).await?;
        drop(controller);
        
        let discovery_engine = DMSCResourceScheduler::new();
        
        let mut discovery_guard = self.discovery_engine.write().await;
        *discovery_guard = discovery_engine;
        
        if let Some(metrics_registry) = ctx.metrics_registry() {
            let mut controller = self.controller.write().await;
            controller.initialize_metrics(&metrics_registry)?;
            drop(controller);
        }
        
        let logger = ctx.logger();
        logger.info("DMSC.DeviceControl", "Device control module initialized with real hardware discovery")?;
        Ok(())
    }
}

fn parse_device_config(config_str: Option<&String>) -> crate::device::core::DMSCDeviceControlConfig {
    match config_str {
        Some(config) => {
            let trimmed = config.trim();
            if trimmed.starts_with('{') {
                if let Ok(result) = serde_json::from_str::<crate::device::core::DMSCDeviceControlConfig>(trimmed) {
                    return result;
                }
            }
            if trimmed.starts_with("---") || trimmed.contains("discovery_enabled:") {
                if let Ok(result) = serde_yaml::from_str::<crate::device::core::DMSCDeviceControlConfig>(trimmed) {
                    return result;
                }
            }
            if trimmed.contains('[') || trimmed.contains("discovery_enabled") {
                if let Ok(result) = toml::from_str::<crate::device::core::DMSCDeviceControlConfig>(trimmed) {
                    return result;
                }
            }
            serde_json::from_str::<crate::device::core::DMSCDeviceControlConfig>(trimmed)
                .unwrap_or_default()
        }
        None => crate::device::core::DMSCDeviceControlConfig::default(),
    }
}
