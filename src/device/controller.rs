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
use tokio::sync::RwLock;
use std::collections::HashMap;
use chrono::Utc;

use crate::core::DMSResult;
use super::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities, DMSDeviceStatus};
// use super::scheduler::DMSDeviceScheduler;

/// Device controller - manages device lifecycle and state
pub struct DMSDeviceController {
    devices: HashMap<String, Arc<RwLock<DMSDevice>>>,
    device_type_index: HashMap<DMSDeviceType, Vec<String>>,
    allocation_map: HashMap<String, String>, // allocation_id -> device_id
}

impl DMSDeviceController {
    pub fn _Fnew() -> Self {
        Self {
            devices: HashMap::new(),
            device_type_index: HashMap::new(),
            allocation_map: HashMap::new(),
        }
    }
    
    /// Discover devices in the system
    pub async fn _Fdiscover_devices(&mut self) -> DMSResult<super::DMSDiscoveryResult> {
        // In a real implementation, this would scan the system/network for devices
        // For now, we'll simulate discovery and update existing mock devices
        
        let mut discovered_devices = Vec::new();
        let mut updated_devices = Vec::new();
        let mut removed_devices = Vec::new();
        
        // Update existing devices
        for device_lock in self.devices.values() {
            let mut device = device_lock.write().await;
            device._Fupdate_last_seen();
            
            // Simulate some devices going offline randomly
            if rand::random::<f64>() < 0.05 { // 5% chance
                device._Fset_status(DMSDeviceStatus::Offline);
            } else if device._Fstatus() == DMSDeviceStatus::Offline {
                device._Fset_status(DMSDeviceStatus::Available);
            }
            
            updated_devices.push(device.clone());
        }
        
        // Occasionally add new mock devices
        if rand::random::<f64>() < 0.1 { // 10% chance
            let new_device = self._Fcreate_mock_device();
             let device_id = new_device._Fid().to_string();
             
             self.devices.insert(device_id.clone(), Arc::new(RwLock::new(new_device.clone())));
             self.device_type_index.entry(new_device._Fdevice_type()).or_default().push(device_id);
             
             discovered_devices.push(new_device);
        }
        
        // Remove devices that haven't been seen for a while
        let timeout = chrono::TimeDelta::minutes(5);
        let now = Utc::now();
        
        let mut to_remove = Vec::new();
        for (device_id, device_lock) in &self.devices {
            let device = device_lock.read().await;
            if now.signed_duration_since(device._Flast_seen()) > timeout {
                to_remove.push(device_id.clone());
            }
        }
        
        for device_id in &to_remove {
            self._Fremove_device(device_id).await?;
            removed_devices.push(device_id.clone());
        }
        
        Ok(super::DMSDiscoveryResult {
            discovered_devices,
            updated_devices,
            removed_devices,
            total_devices: self.devices.len(),
        })
    }
    
    /// Add mock devices for demonstration
    pub fn _Fadd_mock_devices(&mut self) -> DMSResult<()> {
        // CPU devices
        let cpu1 = DMSDevice::new("CPU-1".to_string(), DMSDeviceType::CPU)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_compute_units(8)
                ._Fwith_memory_gb(16.0));
        
        let cpu2 = DMSDevice::new("CPU-2".to_string(), DMSDeviceType::CPU)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_compute_units(16)
                ._Fwith_memory_gb(32.0));
        
        // GPU devices
        let gpu1 = DMSDevice::new("GPU-1".to_string(), DMSDeviceType::GPU)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_compute_units(2560)
                ._Fwith_memory_gb(24.0)
                ._Fwith_custom_capability("cuda_version".to_string(), "11.8".to_string()));
        
        // Memory devices
        let memory1 = DMSDevice::new("Memory-1".to_string(), DMSDeviceType::Memory)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_memory_gb(128.0)
                ._Fwith_bandwidth_gbps(25.6));
        
        // Storage devices
        let storage1 = DMSDevice::new("Storage-1".to_string(), DMSDeviceType::Storage)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_storage_gb(1000.0)
                ._Fwith_bandwidth_gbps(6.0));
        
        // Network devices
        let network1 = DMSDevice::new("Network-1".to_string(), DMSDeviceType::Network)
            .with_capabilities(DMSDeviceCapabilities::new()
                ._Fwith_bandwidth_gbps(10.0)
                ._Fwith_custom_capability("protocol".to_string(), "ethernet".to_string()));
        
        // Add devices to controller
        let devices = vec![cpu1, cpu2, gpu1, memory1, storage1, network1];
        for mut device in devices {
            device._Fset_status(DMSDeviceStatus::Available);
            device._Fset_location("Local System".to_string());
            
            let device_id = device._Fid().to_string();
            let device_type = device._Fdevice_type();
            
            self.devices.insert(device_id.clone(), Arc::new(RwLock::new(device)));
            self.device_type_index.entry(device_type).or_default().push(device_id);
        }
        
        Ok(())
    }
    
    fn _Fcreate_mock_device(&self) -> DMSDevice {
        // Create a random mock device
        let device_types = vec!["CPU".to_string(), "GPU".to_string(), "Memory".to_string(), "Storage".to_string(), "Network".to_string()];
        let device_type = device_types[rand::random::<usize>() % device_types.len()].clone();
        
        let device_type_enum = match device_type.as_str() {
            "CPU" => DMSDeviceType::CPU,
            "GPU" => DMSDeviceType::GPU,
            "Memory" => DMSDeviceType::Memory,
            "Storage" => DMSDeviceType::Storage,
            "Network" => DMSDeviceType::Network,
            _ => DMSDeviceType::Custom,
        };
        let name = format!("Mock-{}-{}", device_type, rand::random::<u16>());
        let mut device = DMSDevice::new(name, device_type_enum);
        
        // Add random capabilities
        let mut capabilities = DMSDeviceCapabilities::new();
        
        match device_type_enum {
            DMSDeviceType::CPU => {
                capabilities = capabilities
                    ._Fwith_compute_units(rand::random::<usize>() % 32 + 1)
                    ._Fwith_memory_gb(rand::random::<f64>() * 64.0 + 4.0);
            }
            DMSDeviceType::GPU => {
                capabilities = capabilities
                    ._Fwith_compute_units(rand::random::<usize>() % 10000 + 100)
                    ._Fwith_memory_gb(rand::random::<f64>() * 32.0 + 2.0);
            }
            DMSDeviceType::Memory => {
                capabilities = capabilities
                    ._Fwith_memory_gb(rand::random::<f64>() * 256.0 + 8.0)
                    ._Fwith_bandwidth_gbps(rand::random::<f64>() * 50.0 + 5.0);
            }
            DMSDeviceType::Storage => {
                capabilities = capabilities
                    ._Fwith_storage_gb(rand::random::<f64>() * 10000.0 + 100.0)
                    ._Fwith_bandwidth_gbps(rand::random::<f64>() * 20.0 + 1.0);
            }
            DMSDeviceType::Network => {
                capabilities = capabilities
                    ._Fwith_bandwidth_gbps(rand::random::<f64>() * 100.0 + 1.0);
            }
            _ => {}
        }
        
        device = device.with_capabilities(capabilities);
        device._Fset_status(DMSDeviceStatus::Available);
        device._Fset_location("Discovered Network".to_string());
        
        device
    }
    
    /// Find a suitable device for the given requirements
    pub async fn _Ffind_suitable_device(
        &self, 
        device_type: &DMSDeviceType, 
        requirements: &DMSDeviceCapabilities
    ) -> DMSResult<Option<DMSDevice>> {
        let device_ids = match self.device_type_index.get(device_type) {
            Some(ids) => ids.clone(),
            None => return Ok(None),
        };
        
        // Find the best available device
        let mut best_device: Option<DMSDevice> = None;
        let mut best_score = 0u32;
        
        for device_id in device_ids {
            if let Some(device_lock) = self.devices.get(&device_id) {
                let device = device_lock.read().await;
                
                if device._Fis_available() && device.capabilities().meets_requirements(requirements) {
                    let score = self._Fcalculate_device_score(&device);
                    
                    if score > best_score || best_device.is_none() {
                        best_device = Some(device.clone());
                        best_score = score;
                    }
                }
            }
        }
        
        Ok(best_device)
    }
    
    fn _Fcalculate_device_score(&self, device: &DMSDevice) -> u32 {
        let mut score = device._Fhealth_score() as u32 * 100;
        
        // Add capability-based scoring
        let capabilities = device.capabilities();
        
        if let Some(compute_units) = capabilities.compute_units {
            score += compute_units as u32;
        }
        
        if let Some(memory_gb) = capabilities.memory_gb {
            score += (memory_gb * 10.0) as u32;
        }
        
        if let Some(storage_gb) = capabilities.storage_gb {
            score += (storage_gb * 5.0) as u32;
        }
        
        if let Some(bandwidth_gbps) = capabilities.bandwidth_gbps {
            score += (bandwidth_gbps * 20.0) as u32;
        }
        
        score
    }
    
    /// Allocate a device
    pub async fn _Fallocate_device(&mut self, device_id: &str, allocation_id: &str) -> DMSResult<()> {
        if let Some(device_lock) = self.devices.get(device_id) {
            let mut device = device_lock.write().await;
            
            if device._Fallocate(allocation_id) {
                self.allocation_map.insert(allocation_id.to_string(), device_id.to_string());
                Ok(())
            } else {
                Err(crate::core::DMSError::DeviceAllocationFailed {
                    device_id: device_id.to_string(),
                    reason: "Device not available".to_string(),
                })
            }
        } else {
            Err(crate::core::DMSError::DeviceNotFound {
                device_id: device_id.to_string(),
            })
        }
    }
    
    /// Release a device by allocation ID
    pub async fn _Frelease_device_by_allocation(&mut self, allocation_id: &str) -> DMSResult<()> {
        if let Some(device_id) = self.allocation_map.remove(allocation_id) {
            if let Some(device_lock) = self.devices.get(&device_id) {
                let mut device = device_lock.write().await;
                device._Frelease();
                Ok(())
            } else {
                Err(crate::core::DMSError::DeviceNotFound {
                    device_id,
                })
            }
        } else {
            Err(crate::core::DMSError::AllocationNotFound {
                allocation_id: allocation_id.to_string(),
            })
        }
    }
    
    /// Remove a device
    async fn _Fremove_device(&mut self, device_id: &str) -> DMSResult<()> {
        if let Some(device_lock) = self.devices.remove(device_id) {
            let device = device_lock.read().await;
            let device_type = device._Fdevice_type();
            
            // Remove from type index
            if let Some(type_devices) = self.device_type_index.get_mut(&device_type) {
                type_devices.retain(|id| id != device_id);
            }
            
            // Remove any allocations
            if let Some(allocation_id) = device._Fget_allocation_id() {
                self.allocation_map.remove(allocation_id);
            }
        }
        
        Ok(())
    }
    
    /// Get all devices
    pub fn _Fget_all_devices(&self) -> Vec<DMSDevice> {
        let mut devices = Vec::new();
        
        // This is a blocking operation - in a real implementation, we'd use async
        for device_lock in self.devices.values() {
            if let Ok(device) = device_lock.try_read() {
                devices.push(device.clone());
            }
        }
        
        devices
    }
    
    /// Release all devices (shutdown)
    pub fn _Frelease_all_devices(&mut self) -> DMSResult<()> {
        // Clear all allocations
        self.allocation_map.clear();
        
        // Release all devices
        for device_lock in self.devices.values() {
            if let Ok(mut device) = device_lock.try_write() {
                device._Frelease();
            }
        }
        
        Ok(())
    }
}