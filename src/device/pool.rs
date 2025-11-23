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

use std::sync::Arc;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

use super::device::{DMSDevice, DMSDeviceType};

/// Resource pool for managing multiple similar devices
pub struct DMSResourcePool {
    name: String,
    device_type: DMSDeviceType,
    devices: HashMap<String, Arc<DMSDevice>>,
    total_capacity: usize,
    available_capacity: usize,
    allocated_capacity: usize,
    pending_requests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolConfig {
    pub name: String,
    pub device_type: DMSDeviceType,
    pub max_concurrent_allocations: usize,
    pub allocation_timeout_secs: u64,
    pub health_check_interval_secs: u64,
}

impl DMSResourcePool {
    pub fn _Fnew(config: DMSResourcePoolConfig) -> Self {
        Self {
            name: config.name,
            device_type: config.device_type,
            devices: HashMap::new(),
            total_capacity: 0,
            available_capacity: 0,
            allocated_capacity: 0,
            pending_requests: 0,
        }
    }
    
    /// Add a device to the pool
    pub fn _Fadd_device(&mut self, device: Arc<DMSDevice>) -> bool {
        if device._Fdevice_type() != self.device_type {
            return false;
        }
        
        let device_id = device._Fid().to_string();
        if self.devices.contains_key(&device_id) {
            return false;
        }
        
        self.devices.insert(device_id, device);
        self.total_capacity += 1;
        self.available_capacity += 1;
        
        true
    }
    
    /// Remove a device from the pool
    pub fn _Fremove_device(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.remove(device_id) {
            self.total_capacity -= 1;
            
            if device._Fis_available() {
                self.available_capacity -= 1;
            } else if device._Fis_allocated() {
                self.allocated_capacity -= 1;
            }
            
            true
        } else {
            false
        }
    }
    
    /// Allocate a device from the pool
    pub fn _Fallocate(&mut self, _allocation_id: &str) -> Option<Arc<DMSDevice>> {
        if self.available_capacity == 0 {
            return None;
        }
        
        // Find the first available device
        for device in self.devices.values() {
            // This is a simplified allocation - in a real implementation, 
            // we'd need to lock the device and check its status atomically
            if device._Fis_available() {
                // Note: In a real implementation, we'd need to modify the device
                // to mark it as allocated. This is simplified for demonstration.
                self.available_capacity -= 1;
                self.allocated_capacity += 1;
                return Some(device.clone());
            }
        }
        
        None
    }
    
    /// Release a device back to the pool
    pub fn _Frelease(&mut self, allocation_id: &str) -> bool {
        // Find the allocated device
        for device in self.devices.values() {
            if let Some(current_allocation) = device._Fget_allocation_id() {
                if current_allocation == allocation_id {
                    // Note: In a real implementation, we'd need to modify the device
                    // to mark it as released. This is simplified for demonstration.
                    self.allocated_capacity -= 1;
                    self.available_capacity += 1;
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Get pool status
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
    
    /// Get pool name
    pub fn _Fname(&self) -> &str {
        &self.name
    }
    
    /// Get device type
    pub fn _Fdevice_type(&self) -> DMSDeviceType {
        self.device_type
    }
    
    /// Get all devices in the pool
    pub fn _Fget_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values().cloned().collect()
    }
    
    /// Get available devices
    pub fn _Fget_available_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values()
            .filter(|device| device._Fis_available())
            .cloned()
            .collect()
    }
    
    /// Get allocated devices
    pub fn _Fget_allocated_devices(&self) -> Vec<Arc<DMSDevice>> {
        self.devices.values()
            .filter(|device| device._Fis_allocated())
            .cloned()
            .collect()
    }
    
    /// Check if pool has available capacity
    pub fn _Fhas_available_capacity(&self) -> bool {
        self.available_capacity > 0
    }
    
    /// Get utilization rate (0.0 - 1.0)
    pub fn _Futilization_rate(&self) -> f64 {
        if self.total_capacity > 0 {
            self.allocated_capacity as f64 / self.total_capacity as f64
        } else {
            0.0
        }
    }
    
    /// Check if pool is healthy (has available devices)
    pub fn _Fis_healthy(&self) -> bool {
        self.available_capacity > 0 || self.allocated_capacity > 0
    }
    
    /// Get pool statistics
    pub fn _Fget_statistics(&self) -> DMSResourcePoolStatistics {
        let devices = self._Fget_devices();
        let available_devices = self._Fget_available_devices();
        let allocated_devices = self._Fget_allocated_devices();
        
        let total_compute_units: usize = devices.iter()
            .filter_map(|d| d.capabilities().compute_units)
            .sum();
        
        let total_memory_gb: f64 = devices.iter()
            .filter_map(|d| d.capabilities().memory_gb)
            .sum();
        
        let total_storage_gb: f64 = devices.iter()
            .filter_map(|d| d.capabilities().storage_gb)
            .sum();
        
        let total_bandwidth_gbps: f64 = devices.iter()
            .filter_map(|d| d.capabilities().bandwidth_gbps)
            .sum();
        
        let average_health_score: f64 = devices.iter()
            .map(|d| d._Fhealth_score() as f64)
            .sum::<f64>() / devices.len() as f64;
        
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

/// Resource pool statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolStatistics {
    pub total_devices: usize,
    pub available_devices: usize,
    pub allocated_devices: usize,
    pub utilization_rate: f64,
    pub total_compute_units: usize,
    pub total_memory_gb: f64,
    pub total_storage_gb: f64,
    pub total_bandwidth_gbps: f64,
    pub average_health_score: f64,
    pub device_type: DMSDeviceType,
}

/// Resource pool manager - manages multiple resource pools
pub struct DMSResourcePoolManager {
    pools: HashMap<String, Arc<DMSResourcePool>>,
}

impl DMSResourcePoolManager {
    pub fn _Fnew() -> Self {
        Self {
            pools: HashMap::new(),
        }
    }
    
    /// Create a new resource pool
    pub fn _Fcreate_pool(&mut self, config: DMSResourcePoolConfig) -> Arc<DMSResourcePool> {
        let pool = Arc::new(DMSResourcePool::_Fnew(config));
        self.pools.insert(pool._Fname().to_string(), pool.clone());
        pool
    }
    
    /// Get a resource pool by name
    pub fn _Fget_pool(&self, name: &str) -> Option<Arc<DMSResourcePool>> {
        self.pools.get(name).cloned()
    }
    
    /// Remove a resource pool
    pub fn _Fremove_pool(&mut self, name: &str) -> Option<Arc<DMSResourcePool>> {
        self.pools.remove(name)
    }
    
    /// Get all resource pools
    pub fn _Fget_all_pools(&self) -> Vec<Arc<DMSResourcePool>> {
        self.pools.values().cloned().collect()
    }
    
    /// Get pools by device type
    pub fn _Fget_pools_by_type(&self, device_type: DMSDeviceType) -> Vec<Arc<DMSResourcePool>> {
        self.pools.values()
            .filter(|pool| pool._Fdevice_type() == device_type)
            .cloned()
            .collect()
    }
    
    /// Get overall resource pool statistics
    pub fn _Fget_overall_statistics(&self) -> DMSResourcePoolStatistics {
        let pools = self._Fget_all_pools();
        
        let _total_pools = pools.len();
        let _healthy_pools = pools.iter().filter(|p| p._Fis_healthy()).count();
        
        let total_devices: usize = pools.iter().map(|p| p._Fget_statistics().total_devices).sum();
        let allocated_devices: usize = pools.iter().map(|p| p._Fget_statistics().allocated_devices).sum();
        
        let total_compute_units: usize = pools.iter()
            .flat_map(|p| p._Fget_devices())
            .filter_map(|d| d.capabilities().compute_units)
            .sum();
        
        let total_memory_gb: f64 = pools.iter()
            .flat_map(|p| p._Fget_devices())
            .filter_map(|d| d.capabilities().memory_gb)
            .sum();
        
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
            total_storage_gb: 0.0,
            total_bandwidth_gbps: 0.0,
            average_health_score: 0.0,
            device_type: DMSDeviceType::Custom,
        }
    }
}