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

//! # Device Control Module
//! 
//! This module provides comprehensive smart device control functionality for DMS, including device
//! discovery, control, and resource scheduling. It enables efficient management of devices and
//! their resources across distributed environments.
//! 
//! ## Key Components
//! 
//! - **DMSDeviceControlModule**: Main device control module implementing service module traits
//! - **DMSDevice**: Device representation with type, status, and capabilities
//! - **DMSDeviceType**: Enum defining supported device types
//! - **DMSDeviceStatus**: Enum defining device statuses
//! - **DMSDeviceCapabilities**: Device capabilities structure
//! - **DMSDeviceController**: Device controller for managing devices
//! - **DMSDeviceScheduler**: Device scheduler for resource allocation
//! - **DMSResourcePool**: Resource pool for managing device resources
//! - **DMSResourcePoolManager**: Manager for multiple resource pools
//! - **DMSResourcePoolStatistics**: Statistics for resource pool monitoring
//! - **DMSDeviceControlConfig**: Configuration for device control behavior
//! - **DMSDiscoveryResult**: Result structure for device discovery
//! - **DMSResourceRequest**: Request structure for resource allocation
//! - **DMSResourceAllocation**: Result structure for resource allocation
//! - **DMSResourcePoolStatus**: Status structure for resource pools
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
//! use dms::prelude::*;
//! use dms::device::{DMSDeviceControlConfig, DMSResourceRequest, DMSDeviceType, DMSDeviceCapabilities};
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create device control configuration
//!     let device_config = DMSDeviceControlConfig {
//!         discovery_enabled: true,
//!         discovery_interval_secs: 30,
//!         auto_scheduling_enabled: true,
//!         max_concurrent_tasks: 100,
//!         resource_allocation_timeout_secs: 60,
//!     };
//!     
//!     // Create device control module
//!     let device_module = DMSDeviceControlModule::_Fnew()
//!         ._Fwith_config(device_config);
//!     
//!     // Discover devices
//!     let discovery_result = device_module._Fdiscover_devices().await?;
//!     println!("Discovered {} devices, total devices: {}", 
//!              discovery_result.discovered_devices.len(), 
//!              discovery_result.total_devices);
//!     
//!     // Get device status
//!     let devices = device_module._Fget_device_status().await?;
//!     println!("Current devices: {:?}", devices);
//!     
//!     // Create resource request
//!     let resource_request = DMSResourceRequest {
//!         request_id: "request-123".to_string(),
//!         device_type: DMSDeviceType::Compute,
//!         required_capabilities: DMSDeviceCapabilities {
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
//!     if let Some(allocation) = device_module._Fallocate_resource(resource_request).await? {
//!         println!("Allocated device: {} (ID: {})", 
//!                  allocation.device_name, 
//!                  allocation.device_id);
//!         
//!         // Release resource after use
//!         device_module._Frelease_resource(&allocation.allocation_id).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

pub mod device;
pub mod controller;
pub mod scheduler;
pub mod pool;
pub mod discovery_scheduler;

use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::observability::{DMSMetricsRegistry, DMSMetric, DMSMetricConfig, DMSMetricType};


pub use device::{DMSDevice, DMSDeviceType, DMSDeviceStatus, DMSDeviceCapabilities};
pub use controller::DMSDeviceController;
pub use pool::{DMSResourcePool, DMSResourcePoolManager, DMSResourcePoolStatistics};
pub use scheduler::DMSDeviceScheduler;

use crate::core::{DMSResult, DMSServiceContext};


/// Main device control module for DMS.
/// 
/// This module provides comprehensive smart device control functionality, including device discovery,
/// control, and resource scheduling. It manages devices and their resources across distributed environments.
pub struct DMSDeviceControlModule {
    /// Device controller for managing devices
    controller: Arc<RwLock<DMSDeviceController>>,
    /// Device scheduler for resource allocation
    #[allow(dead_code)]
    scheduler: Arc<DMSDeviceScheduler>,
    /// Map of resource pool names to resource pool instances
    resource_pools: HashMap<String, Arc<DMSResourcePool>>,
    /// Device control configuration
    config: DMSDeviceControlConfig,
    /// Metrics registry for device management metrics
    #[allow(dead_code)]
    metrics_registry: Arc<Mutex<Option<Arc<DMSMetricsRegistry>>>>,
}

/// Configuration for the device control module.
/// 
/// This struct defines the configuration options for device control behavior, including discovery,
/// scheduling, and resource allocation settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceControlConfig {
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

impl Default for DMSDeviceControlConfig {
    /// Returns the default configuration for device control.
    /// 
    /// Default values:
    /// - discovery_enabled: true
    /// - discovery_interval_secs: 30
    /// - auto_scheduling_enabled: true
    /// - max_concurrent_tasks: 100
    /// - resource_allocation_timeout_secs: 60
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

/// Result structure for device discovery operations.
/// 
/// This struct contains information about the results of a device discovery scan, including
/// discovered, updated, and removed devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDiscoveryResult {
    /// Newly discovered devices
    pub discovered_devices: Vec<DMSDevice>,
    /// Devices with updated information
    pub updated_devices: Vec<DMSDevice>,
    /// IDs of removed devices
    pub removed_devices: Vec<String>, // device IDs
    /// Total number of devices after discovery
    pub total_devices: usize,
}

/// Request structure for resource allocation.
/// 
/// This struct defines the requirements for resource allocation, including device type, capabilities,
/// priority, and timeout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourceRequest {
    /// Unique request ID
    pub request_id: String,
    /// Required device type
    pub device_type: DMSDeviceType,
    /// Required device capabilities
    pub required_capabilities: DMSDeviceCapabilities,
    /// Request priority (1-10, higher is more important)
    pub priority: u8, // 1-10, higher is more important
    /// Request timeout in seconds
    pub timeout_secs: u64,
}

/// Result structure for resource allocation.
/// 
/// This struct contains information about a successful resource allocation, including the allocated
/// device, allocation time, and expiration time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourceAllocation {
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
    pub request: DMSResourceRequest,
}

impl DMSDeviceControlModule {
    /// Creates a new device control module with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSDeviceControlModule` instance with default configuration
    pub fn _Fnew() -> Self {
        let controller = Arc::new(RwLock::new(DMSDeviceController::_Fnew()));
        let resource_pool_manager = Arc::new(Mutex::new(DMSResourcePoolManager::_Fnew()));
        let scheduler = Arc::new(DMSDeviceScheduler::_Fnew(resource_pool_manager));
        
        Self {
            controller,
            scheduler,
            resource_pools: HashMap::new(),
            config: DMSDeviceControlConfig::default(),
            metrics_registry: Arc::new(Mutex::new(None)),
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
    /// The updated `DMSDeviceControlModule` instance
    pub fn _Fwith_config(mut self, config: DMSDeviceControlConfig) -> Self {
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
    /// A `DMSResult<DMSDiscoveryResult>` containing the discovery results
    pub async fn _Fdiscover_devices(&self) -> DMSResult<DMSDiscoveryResult> {
        if !self.config.discovery_enabled {
            return Ok(DMSDiscoveryResult {
                discovered_devices: vec![],
                updated_devices: vec![],
                removed_devices: vec![],
                total_devices: 0,
            });
        }
        
        let mut controller = self.controller.write().await;
        controller._Fdiscover_devices().await
    }
    
    /// Allocates a device resource based on the given request.
    /// 
    /// This method finds a suitable device based on the requested device type and capabilities,
    /// allocates it, and returns an allocation result if successful.
    /// 
    /// # Parameters
    /// 
    /// - `request`: The resource allocation request
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Option<DMSResourceAllocation>>` containing the allocation result if successful,
    /// or None if allocation failed or auto-scheduling is disabled
    pub async fn _Fallocate_resource(&self, request: DMSResourceRequest) -> DMSResult<Option<DMSResourceAllocation>> {
        if !self.config.auto_scheduling_enabled {
            return Ok(None);
        }
        
        let controller = self.controller.read().await;
        let device = controller._Ffind_suitable_device(&request.device_type, &request.required_capabilities).await?;
        
        if let Some(device) = device {
            let allocation = DMSResourceAllocation {
                allocation_id: uuid::Uuid::new_v4().to_string(),
                device_id: device._Fid().to_string(),
                device_name: device._Fname().to_string(),
                allocated_at: chrono::Utc::now(),
                expires_at: chrono::Utc::now() + chrono::Duration::seconds(request.timeout_secs as i64),
                request,
            };
            
            // Mark device as busy
            drop(controller);
            let mut controller = self.controller.write().await;
            controller._Fallocate_device(&allocation.device_id, &allocation.allocation_id).await?;
            
            Ok(Some(allocation))
        } else {
            Ok(None)
        }
    }
    
    /// Releases a previously allocated device resource.
    /// 
    /// This method releases a device resource that was allocated with `_Fallocate_resource`.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: The ID of the allocation to release
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub async fn _Frelease_resource(&self, allocation_id: &str) -> DMSResult<()> {
        let mut controller = self.controller.write().await;
        controller._Frelease_device_by_allocation(allocation_id).await
    }
    
    /// Gets the current status of all devices.
    /// 
    /// This method returns a list of all devices currently managed by the device controller.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Vec<DMSDevice>>` containing all managed devices
    pub async fn _Fget_device_status(&self) -> DMSResult<Vec<DMSDevice>> {
        let controller = self.controller.read().await;
        Ok(controller._Fget_all_devices())
    }
    
    /// Gets the status of all resource pools.
    /// 
    /// This method returns a map of resource pool names to their current status.
    /// 
    /// # Returns
    /// 
    /// A `HashMap<String, DMSResourcePoolStatus>` containing the status of all resource pools
    pub fn _Fget_resource_pool_status(&self) -> HashMap<String, DMSResourcePoolStatus> {
        let mut status = HashMap::new();
        for (pool_name, pool) in &self.resource_pools {
            status.insert(pool_name.clone(), pool._Fget_status());
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
    /// A `DMSResult<()>` indicating success or failure
    fn _Fcreate_device_metrics(&self, registry: Arc<DMSMetricsRegistry>) -> DMSResult<()> {
        // Device total metric (Gauge)
        let device_total_config = DMSMetricConfig {
            metric_type: DMSMetricType::Gauge,
            name: "dms_device_total".to_string(),
            help: "Total number of devices by type and status".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(300),
            age_buckets: 5,
        };
        let device_total_metric = Arc::new(DMSMetric::_Fnew(device_total_config));
        registry._Fregister(device_total_metric)?;
        
        // Allocation attempts metric (Counter)
        let allocation_attempts_config = DMSMetricConfig {
            metric_type: DMSMetricType::Counter,
            name: "dms_device_allocation_attempts_total".to_string(),
            help: "Total number of device allocation attempts".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_attempts_metric = Arc::new(DMSMetric::_Fnew(allocation_attempts_config));
        registry._Fregister(allocation_attempts_metric)?;
        
        // Allocation success metric (Counter)
        let allocation_success_config = DMSMetricConfig {
            metric_type: DMSMetricType::Counter,
            name: "dms_device_allocation_success_total".to_string(),
            help: "Total number of successful device allocations".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_success_metric = Arc::new(DMSMetric::_Fnew(allocation_success_config));
        registry._Fregister(allocation_success_metric)?;
        
        // Allocation failure metric (Counter)
        let allocation_failure_config = DMSMetricConfig {
            metric_type: DMSMetricType::Counter,
            name: "dms_device_allocation_failure_total".to_string(),
            help: "Total number of failed device allocations".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let allocation_failure_metric = Arc::new(DMSMetric::_Fnew(allocation_failure_config));
        registry._Fregister(allocation_failure_metric)?;
        
        // Discovery attempts metric (Counter)
        let discovery_attempts_config = DMSMetricConfig {
            metric_type: DMSMetricType::Counter,
            name: "dms_device_discovery_attempts_total".to_string(),
            help: "Total number of device discovery attempts".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let discovery_attempts_metric = Arc::new(DMSMetric::_Fnew(discovery_attempts_config));
        registry._Fregister(discovery_attempts_metric)?;
        
        // Discovery success metric (Counter)
        let discovery_success_config = DMSMetricConfig {
            metric_type: DMSMetricType::Counter,
            name: "dms_device_discovery_success_total".to_string(),
            help: "Total number of successful device discoveries".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(0),
            age_buckets: 0,
        };
        let discovery_success_metric = Arc::new(DMSMetric::_Fnew(discovery_success_config));
        registry._Fregister(discovery_success_metric)?;
        
        // Resource utilization metric (Gauge)
        let resource_utilization_config = DMSMetricConfig {
            metric_type: DMSMetricType::Gauge,
            name: "dms_device_resource_utilization".to_string(),
            help: "Resource utilization by device type".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(300),
            age_buckets: 5,
        };
        let resource_utilization_metric = Arc::new(DMSMetric::_Fnew(resource_utilization_config));
        registry._Fregister(resource_utilization_metric)?;
        
        Ok(())
    }
}

/// Status structure for resource pools.
/// 
/// This struct contains information about the current status of a resource pool, including capacity,
/// allocation, and utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolStatus {
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
impl crate::core::_CAsyncServiceModule for DMSDeviceControlModule {
    /// Returns the name of the device control module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
        "DMS.DeviceControl"
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
    fn _Fis_critical(&self) -> bool {
        false // Non-critical, should not break the app if device control fails
    }
    
    /// Initializes the device control module asynchronously.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Initializes the device controller with mock devices for demonstration
    /// 3. Logs initialization completion
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration and logger
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        self.config = DMSDeviceControlConfig {
            discovery_enabled: cfg._Fget_bool("device_control.discovery_enabled").unwrap_or(true),
            discovery_interval_secs: cfg._Fget_u64("device_control.discovery_interval_secs").unwrap_or(30),
            auto_scheduling_enabled: cfg._Fget_bool("device_control.auto_scheduling_enabled").unwrap_or(true),
            max_concurrent_tasks: cfg._Fget_i64("device_control.max_concurrent_tasks").unwrap_or(100) as usize,
            resource_allocation_timeout_secs: cfg._Fget_u64("device_control.resource_allocation_timeout_secs").unwrap_or(60),
        };
        
        // Initialize device controller with mock devices for demonstration
        let mut controller = self.controller.write().await;
        controller._Fadd_mock_devices()?;
        drop(controller);
        
        // Note: Metrics registry integration will be added in a future update
        // The _Fget_metrics_registry method is not available yet in the service context
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module initialized")?;
        
        Ok(())
    }
    
    /// Performs asynchronous cleanup after the application has shut down.
    /// 
    /// This method releases all allocated device resources and logs shutdown completion.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing the logger
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Release all allocated resources
        let mut controller = self.controller.write().await;
        controller._Frelease_all_devices()?;
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module shutdown completed")?;
        
        Ok(())
    }
}

impl crate::core::_CServiceModule for DMSDeviceControlModule {
    /// Returns the name of the device control module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
        "DMS.DeviceControl"
    }
    
    /// Indicates whether the device control module is critical.
    /// 
    /// # Returns
    /// 
    /// `false` since device control is non-critical
    fn _Fis_critical(&self) -> bool {
        false // Non-critical, should not break the app if device control fails
    }
    
    /// Initializes the device control module synchronously.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Initializes the device controller with mock devices for demonstration
    /// 3. Logs initialization completion
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration and logger
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        self.config = DMSDeviceControlConfig {
            discovery_enabled: cfg._Fget_bool("device_control.discovery_enabled").unwrap_or(true),
            discovery_interval_secs: cfg._Fget_u64("device_control.discovery_interval_secs").unwrap_or(30),
            auto_scheduling_enabled: cfg._Fget_bool("device_control.auto_scheduling_enabled").unwrap_or(true),
            max_concurrent_tasks: cfg._Fget_i64("device_control.max_concurrent_tasks").unwrap_or(100) as usize,
            resource_allocation_timeout_secs: cfg._Fget_u64("device_control.resource_allocation_timeout_secs").unwrap_or(60),
        };
        
        // Initialize device controller with mock devices for demonstration
        let mut controller = self.controller.blocking_write();
        controller._Fadd_mock_devices()?;
        drop(controller);
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module initialized")?;
        
        Ok(())
    }
    
    /// Performs synchronous cleanup after the application has shut down.
    /// 
    /// This method releases all allocated device resources and logs shutdown completion.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing the logger
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Release all allocated resources
        let mut controller = self.controller.blocking_write();
        controller._Frelease_all_devices()?;
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module shutdown completed")?;
        
        Ok(())
    }
}