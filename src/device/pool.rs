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

//! # Resource Pool Management
//! 
//! This file implements resource pool management for the DMS framework, providing a way to group
//! similar devices together for efficient resource allocation and management. It includes both
//! single resource pools and a resource pool manager for handling multiple pools.
//! 
//! ## Key Components
//! 
//! - **DMSResourcePool**: Manages a pool of similar devices
//! - **DMSResourcePoolConfig**: Configuration for resource pools
//! - **DMSResourcePoolStatistics**: Statistics for monitoring resource pools
//! - **DMSResourcePoolManager**: Manages multiple resource pools
//! 
//! ## Design Principles
//! 
//! 1. **Resource Grouping**: Groups similar devices together for efficient management
//! 2. **Capacity Tracking**: Tracks total, available, and allocated capacity
//! 3. **Statistics Collection**: Collects comprehensive statistics for monitoring
//! 4. **Device Filtering**: Filters devices by availability and allocation status
//! 5. **Health Monitoring**: Monitors pool health based on available devices
//! 6. **Utilization Tracking**: Tracks resource utilization rates
//! 7. **Multi-Pool Management**: Supports managing multiple pools through a central manager
//! 8. **Device Type Segregation**: Each pool contains devices of a single type
//! 9. **Arc-Based Sharing**: Uses Arc for safe concurrent access to devices
//! 10. **Serialization Support**: All structures support serialization/deserialization
//! 11. **Builder Pattern**: Configurable through DMSResourcePoolConfig
//! 12. **Resource Optimization**: Calculates total compute, memory, storage, and bandwidth
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::device::{DMSResourcePoolManager, DMSResourcePoolConfig, DMSDeviceType};
//! use dms::core::DMSResult;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a resource pool manager
//!     let mut manager = DMSResourcePoolManager::_Fnew();
//!     
//!     // Create a resource pool configuration
//!     let config = DMSResourcePoolConfig {
//!         name: "cpu-pool-1".to_string(),
//!         device_type: DMSDeviceType::CPU,
//!         max_concurrent_allocations: 10,
//!         allocation_timeout_secs: 60,
//!         health_check_interval_secs: 30,
//!     };
//!     
//!     // Create a resource pool
//!     let pool = manager._Fcreate_pool(config);
//!     
//!     // Get pool statistics
//!     let stats = pool._Fget_statistics();
//!     println!("Pool has {} devices, utilization: {:.2}%", 
//!              stats.total_devices, stats.utilization_rate * 100.0);
//!     
//!     // Get all pools by device type
//!     let cpu_pools = manager._Fget_pools_by_type(DMSDeviceType::CPU);
//!     println!("Found {} CPU pools", cpu_pools.len());
//!     
//!     // Get overall statistics
//!     let overall_stats = manager._Fget_overall_statistics();
//!     println!("Total devices across all pools: {}", overall_stats.total_devices);
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::device::{DMSDevice, DMSDeviceType};

/// Resource pool for managing multiple similar devices
/// 
/// This struct manages a pool of devices of the same type, tracking their availability,
/// allocation status, and capacity. It provides methods for adding/removing devices,
/// allocating/releasing devices, and collecting statistics.
pub struct DMSResourcePool {
    /// Name of the resource pool
    name: String,
    /// Type of devices in the pool
    device_type: DMSDeviceType,
    /// Map of device IDs to device instances
    devices: HashMap<String, Arc<DMSDevice>>,
    /// Total capacity of the pool (number of devices)
    total_capacity: usize,
    /// Available capacity (number of devices not allocated)
    available_capacity: usize,
    /// Allocated capacity (number of devices currently allocated)
    allocated_capacity: usize,
    /// Number of pending requests for devices
    pending_requests: usize,
    /// Total compute units across all devices in the pool
    total_compute_units: usize,
    /// Total memory in GB across all devices in the pool
    total_memory_gb: f64,
    /// Total storage in GB across all devices in the pool
    total_storage_gb: f64,
    /// Total bandwidth in Gbps across all devices in the pool
    total_bandwidth_gbps: f64,
    /// Available compute units (not allocated)
    available_compute_units: usize,
    /// Available memory in GB (not allocated)
    available_memory_gb: f64,
    /// Available storage in GB (not allocated)
    available_storage_gb: f64,
    /// Available bandwidth in Gbps (not allocated)
    available_bandwidth_gbps: f64,
}

/// Configuration for a resource pool
/// 
/// This struct defines the configuration options for creating a resource pool, including
/// name, device type, and various operational parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolConfig {
    /// Name of the resource pool
    pub name: String,
    /// Type of devices that will be in the pool
    pub device_type: DMSDeviceType,
    /// Maximum number of concurrent allocations allowed
    pub max_concurrent_allocations: usize,
    /// Timeout for device allocation in seconds
    pub allocation_timeout_secs: u64,
    /// Interval for health checks in seconds
    pub health_check_interval_secs: u64,
}

impl DMSResourcePool {
    /// Creates a new resource pool with the given configuration
    /// 
    /// # Parameters
    /// 
    /// - `config`: The configuration for the resource pool
    /// 
    /// # Returns
    /// 
    /// A new `DMSResourcePool` instance with the specified configuration
    pub fn _Fnew(config: DMSResourcePoolConfig) -> Self {
        Self {
            name: config.name,
            device_type: config.device_type,
            devices: HashMap::new(),
            total_capacity: 0,
            available_capacity: 0,
            allocated_capacity: 0,
            pending_requests: 0,
            total_compute_units: 0,
            total_memory_gb: 0.0,
            total_storage_gb: 0.0,
            total_bandwidth_gbps: 0.0,
            available_compute_units: 0,
            available_memory_gb: 0.0,
            available_storage_gb: 0.0,
            available_bandwidth_gbps: 0.0,
        }
    }
    
    /// Adds a device to the pool
    /// 
    /// This method adds a device to the pool if it matches the pool's device type
    /// and is not already in the pool.
    /// 
    /// # Parameters
    /// 
    /// - `device`: The device to add to the pool
    /// 
    /// # Returns
    /// 
    /// `true` if the device was successfully added, `false` otherwise
    pub fn _Fadd_device(&mut self, device: Arc<DMSDevice>) -> bool {
        // Check if device type matches pool device type
        if device._Fdevice_type() != self.device_type {
            return false;
        }
        
        let device_id = device._Fid().to_string();
        // Check if device is already in the pool
        if self.devices.contains_key(&device_id) {
            return false;
        }
        
        // Get device capabilities and extract values before inserting the device
        let compute_units = device.capabilities().compute_units.unwrap_or(0);
        let memory_gb = device.capabilities().memory_gb.unwrap_or(0.0);
        let storage_gb = device.capabilities().storage_gb.unwrap_or(0.0);
        let bandwidth_gbps = device.capabilities().bandwidth_gbps.unwrap_or(0.0);
        
        // Add device to the pool
        self.devices.insert(device_id, device);
        
        // Update capacity counters
        self.total_capacity += 1;
        self.available_capacity += 1;
        
        // Update total resource counters
        self.total_compute_units += compute_units;
        self.total_memory_gb += memory_gb;
        self.total_storage_gb += storage_gb;
        self.total_bandwidth_gbps += bandwidth_gbps;
        
        // Update available resource counters (device is available initially)
        self.available_compute_units += compute_units;
        self.available_memory_gb += memory_gb;
        self.available_storage_gb += storage_gb;
        self.available_bandwidth_gbps += bandwidth_gbps;
        
        true
    }
    
    /// Removes a device from the pool
    /// 
    /// This method removes a device from the pool by its ID, updating capacity counters
    /// based on the device's status.
    /// 
    /// # Parameters
    /// 
    /// - `device_id`: The ID of the device to remove
    /// 
    /// # Returns
    /// 
    /// `true` if the device was successfully removed, `false` otherwise
    pub fn _Fremove_device(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.remove(device_id) {
            // Get device capabilities
            let capabilities = device.capabilities();
            
            // Decrement total capacity
            self.total_capacity -= 1;
            
            // Update available or allocated capacity based on device status
            if device._Fis_available() {
                self.available_capacity -= 1;
                
                // Update available resource counters
                self.available_compute_units -= capabilities.compute_units.unwrap_or(0);
                self.available_memory_gb -= capabilities.memory_gb.unwrap_or(0.0);
                self.available_storage_gb -= capabilities.storage_gb.unwrap_or(0.0);
                self.available_bandwidth_gbps -= capabilities.bandwidth_gbps.unwrap_or(0.0);
            } else if device._Fis_allocated() {
                self.allocated_capacity -= 1;
                
                // Update allocated resource counters indirectly by updating total
                // Available resources don't change when removing an allocated device
            }
            
            // Update total resource counters
            self.total_compute_units -= capabilities.compute_units.unwrap_or(0);
            self.total_memory_gb -= capabilities.memory_gb.unwrap_or(0.0);
            self.total_storage_gb -= capabilities.storage_gb.unwrap_or(0.0);
            self.total_bandwidth_gbps -= capabilities.bandwidth_gbps.unwrap_or(0.0);
            
            true
        } else {
            false
        }
    }
    
    /// Allocates a device from the pool
    /// 
    /// This method allocates the first available device from the pool, updating capacity counters.
    /// 
    /// # Parameters
    /// 
    /// - `_allocation_id`: The ID of the allocation (currently unused)
    /// 
    /// # Returns
    /// 
    /// An `Option<Arc<DMSDevice>>` containing the allocated device if successful, `None` otherwise
    pub fn _Fallocate(&mut self, _allocation_id: &str) -> Option<Arc<DMSDevice>> {
        // Check if there's available capacity
        if self.available_capacity == 0 {
            return None;
        }
        
        // Find the first available device
        for device in self.devices.values() {
            // This is a simplified allocation - in a real implementation, 
            // we'd need to lock the device and check its status atomically
            if device._Fis_available() {
                // Get device capabilities
                let capabilities = device.capabilities();
                
                // Note: In a real implementation, we'd need to modify the device
                // to mark it as allocated. This is simplified for demonstration.
                self.available_capacity -= 1;
                self.allocated_capacity += 1;
                
                // Update available resource counters
                self.available_compute_units -= capabilities.compute_units.unwrap_or(0);
                self.available_memory_gb -= capabilities.memory_gb.unwrap_or(0.0);
                self.available_storage_gb -= capabilities.storage_gb.unwrap_or(0.0);
                self.available_bandwidth_gbps -= capabilities.bandwidth_gbps.unwrap_or(0.0);
                
                return Some(device.clone());
            }
        }
        
        None
    }
    
    /// Releases a device back to the pool
    /// 
    /// This method releases a device back to the pool by its allocation ID, updating capacity counters.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: The ID of the allocation to release
    /// 
    /// # Returns
    /// 
    /// `true` if the device was successfully released, `false` otherwise
    pub fn _Frelease(&mut self, allocation_id: &str) -> bool {
        // Find the allocated device by allocation ID
        for device in self.devices.values() {
            if let Some(current_allocation) = device._Fget_allocation_id() {
                if current_allocation == allocation_id {
                    // Get device capabilities
                    let capabilities = device.capabilities();
                    
                    // Note: In a real implementation, we'd need to modify the device
                    // to mark it as released. This is simplified for demonstration.
                    self.allocated_capacity -= 1;
                    self.available_capacity += 1;
                    
                    // Update available resource counters
                    self.available_compute_units += capabilities.compute_units.unwrap_or(0);
                    self.available_memory_gb += capabilities.memory_gb.unwrap_or(0.0);
                    self.available_storage_gb += capabilities.storage_gb.unwrap_or(0.0);
                    self.available_bandwidth_gbps += capabilities.bandwidth_gbps.unwrap_or(0.0);
                    
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Gets the current status of the pool
    /// 
    /// This method returns a DMSResourcePoolStatus struct containing information about the pool's
    /// capacity, allocation, and utilization.
    /// 
    /// # Returns
    /// 
    /// A `DMSResourcePoolStatus` struct with the current pool status
    pub fn _Fget_status(&self) -> super::DMSResourcePoolStatus {
        super::DMSResourcePoolStatus {
            total_capacity: self.total_capacity,
            available_capacity: self.available_capacity,
            allocated_capacity: self.allocated_capacity,
            pending_requests: self.pending_requests,
            utilization_rate: if self.total_capacity > 0 {
                (self.allocated_capacity as f64 / self.total_capacity as f64) * 100.0
            } else {
                0.0
            },
        }
    }
    
    /// Gets the name of the pool
    /// 
    /// # Returns
    /// 
    /// The pool name as a string slice
    pub fn _Fname(&self) -> &str {
        &self.name
    }
    
    /// Gets the device type of the pool
    /// 
    /// # Returns
    /// 
    /// The device type as a `DMSDeviceType` enum
    pub fn _Fdevice_type(&self) -> DMSDeviceType {
        self.device_type
    }
    
    /// Gets all devices in the pool
    /// 
    /// # Returns
    /// 
    /// A vector of `Arc<DMSDevice>` containing all devices in the pool
    pub fn _Fget_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values().cloned().collect()
    }
    
    /// Gets available devices in the pool
    /// 
    /// # Returns
    /// 
    /// A vector of `Arc<DMSDevice>` containing only available devices
    pub fn _Fget_available_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values()
            .filter(|device| device._Fis_available())
            .cloned()
            .collect()
    }
    
    /// Gets allocated devices in the pool
    /// 
    /// # Returns
    /// 
    /// A vector of `Arc<DMSDevice>` containing only allocated devices
    pub fn _Fget_allocated_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values()
            .filter(|device| device._Fis_allocated())
            .cloned()
            .collect()
    }
    
    /// Checks if the pool has available capacity
    /// 
    /// # Returns
    /// 
    /// `true` if the pool has available devices, `false` otherwise
    pub fn _Fhas_available_capacity(&self) -> bool {
        self.available_capacity > 0
    }
    
    /// Gets the utilization rate of the pool (0.0 - 1.0)
    /// 
    /// # Returns
    /// 
    /// The utilization rate as a floating-point number between 0.0 and 1.0
    pub fn _Futilization_rate(&self) -> f64 {
        if self.total_capacity > 0 {
            self.allocated_capacity as f64 / self.total_capacity as f64
        } else {
            0.0
        }
    }
    
    /// Checks if the pool is healthy
    /// 
    /// A pool is considered healthy if it has available devices or allocated devices.
    /// 
    /// # Returns
    /// 
    /// `true` if the pool is healthy, `false` otherwise
    pub fn _Fis_healthy(&self) -> bool {
        self.available_capacity > 0 || self.allocated_capacity > 0
    }
    
    /// Gets comprehensive statistics for the pool
    /// 
    /// This method calculates and returns comprehensive statistics for the pool, including
    /// device counts, utilization, total compute units, memory, storage, bandwidth, and average health score.
    /// 
    /// # Returns
    /// 
    /// A `DMSResourcePoolStatistics` struct with comprehensive pool statistics
    pub fn _Fget_statistics(&self) -> DMSResourcePoolStatistics {
        let devices = self._Fget_devices();
        let available_devices = self._Fget_available_devices();
        let allocated_devices = self._Fget_allocated_devices();
        
        // Calculate total compute units across all devices
        let total_compute_units: usize = devices.iter()
            .filter_map(|d| d.capabilities().compute_units)
            .sum();
        
        // Calculate total memory across all devices
        let total_memory_gb: f64 = devices.iter()
            .filter_map(|d| d.capabilities().memory_gb)
            .sum();
        
        // Calculate total storage across all devices
        let total_storage_gb: f64 = devices.iter()
            .filter_map(|d| d.capabilities().storage_gb)
            .sum();
        
        // Calculate total bandwidth across all devices
        let total_bandwidth_gbps: f64 = devices.iter()
            .filter_map(|d| d.capabilities().bandwidth_gbps)
            .sum();
        
        // Calculate average health score across all devices
        let average_health_score: f64 = if !devices.is_empty() {
            devices.iter()
                .map(|d| d._Fhealth_score() as f64)
                .sum::<f64>() / devices.len() as f64
        } else {
            0.0
        };
        
        DMSResourcePoolStatistics {
            total_devices: devices.len(),
            available_devices: available_devices.len(),
            allocated_devices: allocated_devices.len(),
            utilization_rate: self._Futilization_rate(),
            total_compute_units,
            total_memory_gb,
            total_storage_gb,
            total_bandwidth_gbps,
            average_health_score,
            device_type: self.device_type,
        }
    }
}

/// Resource pool statistics structure
/// 
/// This struct contains comprehensive statistics for a resource pool, including device counts,
/// utilization, and total resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolStatistics {
    /// Total number of devices in the pool
    pub total_devices: usize,
    /// Number of available devices in the pool
    pub available_devices: usize,
    /// Number of allocated devices in the pool
    pub allocated_devices: usize,
    /// Utilization rate of the pool (0.0 - 1.0)
    pub utilization_rate: f64,
    /// Total compute units across all devices
    pub total_compute_units: usize,
    /// Total memory in gigabytes across all devices
    pub total_memory_gb: f64,
    /// Total storage in gigabytes across all devices
    pub total_storage_gb: f64,
    /// Total bandwidth in gigabits per second across all devices
    pub total_bandwidth_gbps: f64,
    /// Average health score across all devices
    pub average_health_score: f64,
    /// Type of devices in the pool
    pub device_type: DMSDeviceType,
}

/// Resource pool manager for managing multiple resource pools
/// 
/// This struct manages multiple resource pools, providing methods for creating, retrieving,
/// and removing pools, as well as getting overall statistics.
pub struct DMSResourcePoolManager {
    /// Map of pool names to resource pools
    pools: HashMap<String, Arc<DMSResourcePool>>,
}

impl DMSResourcePoolManager {
    /// Creates a new resource pool manager
    /// 
    /// # Returns
    /// 
    /// A new `DMSResourcePoolManager` instance
    pub fn _Fnew() -> Self {
        Self {
            pools: HashMap::new(),
        }
    }
    
    /// Creates a new resource pool
    /// 
    /// This method creates a new resource pool with the given configuration and adds it to the manager.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The configuration for the new resource pool
    /// 
    /// # Returns
    /// 
    /// An `Arc<DMSResourcePool>` to the newly created pool
    pub fn _Fcreate_pool(&mut self, config: DMSResourcePoolConfig) -> Arc<DMSResourcePool> {
        let pool = Arc::new(DMSResourcePool::_Fnew(config));
        self.pools.insert(pool._Fname().to_string(), pool.clone());
        pool
    }
    
    /// Gets a resource pool by name
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the resource pool to get
    /// 
    /// # Returns
    /// 
    /// An `Option<Arc<DMSResourcePool>>` containing the pool if found, `None` otherwise
    pub fn _Fget_pool(&self, name: &str) -> Option<Arc<DMSResourcePool>> {
        self.pools.get(name).cloned()
    }
    
    /// Removes a resource pool by name
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the resource pool to remove
    /// 
    /// # Returns
    /// 
    /// An `Option<Arc<DMSResourcePool>>` containing the removed pool if found, `None` otherwise
    pub fn _Fremove_pool(&mut self, name: &str) -> Option<Arc<DMSResourcePool>> {
        self.pools.remove(name)
    }
    
    /// Gets all resource pools
    /// 
    /// # Returns
    /// 
    /// A vector of `Arc<DMSResourcePool>` containing all resource pools
    pub fn _Fget_all_pools(&self) -> Vec<Arc<DMSResourcePool>> {
        self.pools.values().cloned().collect()
    }
    
    /// Gets all resource pools of a specific device type
    /// 
    /// # Parameters
    /// 
    /// - `device_type`: The device type to filter pools by
    /// 
    /// # Returns
    /// 
    /// A vector of `Arc<DMSResourcePool>` containing all pools of the specified device type
    pub fn _Fget_pools_by_type(&self, device_type: DMSDeviceType) -> Vec<Arc<DMSResourcePool>> {
        self.pools.values()
            .filter(|pool| pool._Fdevice_type() == device_type)
            .cloned()
            .collect()
    }
    
    /// Gets overall statistics for all resource pools
    /// 
    /// This method calculates and returns overall statistics for all resource pools, including
    /// total devices, utilization, and total resources across all pools.
    /// 
    /// # Returns
    /// 
    /// A `DMSResourcePoolStatistics` struct with overall statistics for all pools
    pub fn _Fget_overall_statistics(&self) -> DMSResourcePoolStatistics {
        let pools = self._Fget_all_pools();
        
        // Calculate total devices and allocated devices across all pools
        let total_devices: usize = pools.iter().map(|p| p._Fget_statistics().total_devices).sum();
        let allocated_devices: usize = pools.iter().map(|p| p._Fget_statistics().allocated_devices).sum();
        
        // Calculate total compute units across all devices in all pools
        let total_compute_units: usize = pools.iter()
            .flat_map(|p| p._Fget_devices())
            .filter_map(|d| d.capabilities().compute_units)
            .sum();
        
        // Calculate total memory across all devices in all pools
        let total_memory_gb: f64 = pools.iter()
            .flat_map(|p| p._Fget_devices())
            .filter_map(|d| d.capabilities().memory_gb)
            .sum();
        
        // Calculate overall utilization rate
        let overall_utilization = if total_devices > 0 {
            allocated_devices as f64 / total_devices as f64
        } else {
            0.0
        };
        
        DMSResourcePoolStatistics {
            total_devices,
            available_devices: total_devices - allocated_devices,
            allocated_devices,
            utilization_rate: overall_utilization,
            total_compute_units,
            total_memory_gb,
            total_storage_gb: 0.0, // Simplified: would sum across all pools in a real implementation
            total_bandwidth_gbps: 0.0, // Simplified: would sum across all pools in a real implementation
            average_health_score: 0.0, // Simplified: would calculate across all pools in a real implementation
            device_type: DMSDeviceType::Custom, // Multiple device types across pools
        }
    }
}