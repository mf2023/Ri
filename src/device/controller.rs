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

//! # Device Controller
//!
//! This file implements the device controller for the DMS framework, responsible for managing the
//! lifecycle and state of devices in the system. It provides functionality for device discovery,
//! allocation, health monitoring, and state management.
//!
//! ## Key Components
//!
//! - **DMSDeviceController**: Main device controller struct
//! - **Device Discovery**: Scans the system/network for devices
//! - **Device Allocation**: Manages device allocation and deallocation
//! - **Health Monitoring**: Performs periodic health checks on devices
//! - **Device State Management**: Tracks device status and capabilities
//!
//! ## Design Principles
//!
//! 1. **Centralized Management**: Single point of control for all devices
//! 2. **Async-First**: All device operations are asynchronous
//! 3. **Thread Safety**: Uses Arc and RwLock for safe concurrent access
//! 4. **Indexing**: Maintains indexes for efficient device lookup by type
//! 5. **Health Monitoring**: Periodic health checks to ensure device reliability
//! 6. **Mock Support**: Built-in mock device generation for testing and demonstration
//! 7. **State Tracking**: Tracks device allocation and status changes
//! 8. **Scalability**: Efficiently handles large numbers of devices
//! 9. **Fault Tolerance**: Handles device failures gracefully
//! 10. **Resource Optimization**: Scores devices to find the best fit for requirements
//!
//! ## Usage
//!
//! ```rust
//! use dms::device::{DMSDeviceController, DMSDeviceType, DMSDeviceCapabilities};
//! use dms::core::DMSResult;
//!
//! async fn example() -> DMSResult<()> {
//!     // Create a new device controller
//!     let mut controller = DMSDeviceController::_Fnew();
//!     
//!     // Add mock devices for testing
//!     controller._Fadd_mock_devices()?;
//!     
//!     // Discover devices in the system
//!     let discovery_result = controller._Fdiscover_devices().await?;
//!     println!("Discovered {} devices", discovery_result.total_devices);
//!     
//!     // Find a suitable CPU device
//!     let requirements = DMSDeviceCapabilities::new()
//!         ._Fwith_compute_units(8)
//!         ._Fwith_memory_gb(16.0);
//!     
//!     if let Some(device) = controller._Ffind_suitable_device(&DMSDeviceType::CPU, &requirements).await? {
//!         println!("Found suitable device: {}", device._Fid());
//!         
//!         // Allocate the device
//!         controller._Fallocate_device(device._Fid(), "allocation-1").await?;
//!         println!("Allocated device: {}", device._Fid());
//!         
//!         // Release the device
//!         controller._Frelease_device_by_allocation("allocation-1").await?;
//!         println!("Released device: {}", device._Fid());
//!     }
//!     
//!     // Perform health checks
//!     let health_results = controller._Fperform_health_checks().await?;
//!     for (device_id, health_score) in health_results {
//!         println!("Device {} health score: {}", device_id, health_score);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::device::{DMSDevice, DMSDeviceCapabilities, DMSDeviceStatus, DMSDeviceType};
use crate::core::DMSResult;
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
        
        // Retry mechanism for device discovery
        let max_retries = 3;
        let retry_delay = std::time::Duration::from_millis(500);
        
        for attempt in 0..max_retries {
            match self._Fperform_device_discovery().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries - 1 {
                        // Last attempt failed, return error
                        return Err(e);
                    }
                    
                    // Log retry attempt
                    let error_msg = format!("Device discovery attempt {} failed: {}, retrying in {}ms", 
                                          attempt + 1, e, retry_delay.as_millis());
                    eprintln!("{}", error_msg);
                    
                    // Wait before retrying
                    tokio::time::sleep(retry_delay).await;
                }
            }
        }
        
        // This line should never be reached due to the retry loop
        Err(crate::core::DMSError::Other("Device discovery failed after maximum retries".to_string()))
    }
    
    /// Performs the actual device discovery logic with proper error handling.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<super::DMSDiscoveryResult>` containing the discovery result if successful.
    async fn _Fperform_device_discovery(&mut self) -> DMSResult<super::DMSDiscoveryResult> {
        let mut discovered_devices = Vec::new();
        let mut updated_devices = Vec::new();
        let mut removed_devices = Vec::new();

        // Update existing devices with error handling
        for device_lock in self.devices.values() {
            match device_lock.try_write() {
                Ok(mut device) => {
                    device._Fupdate_last_seen();

                    // Simulate some devices going offline randomly
                    if rand::random::<f64>() < 0.05 {
                        // 5% chance
                        device._Fset_status(DMSDeviceStatus::Offline);
                    } else if device._Fstatus() == DMSDeviceStatus::Offline {
                        device._Fset_status(DMSDeviceStatus::Available);
                    }

                    updated_devices.push(device.clone());
                },
                Err(_) => {
                    // Failed to acquire write lock, skip this device for now
                    continue;
                }
            }
        }

        // Occasionally add new mock devices
        if rand::random::<f64>() < 0.1 {
            // 10% chance
            let new_device = self._Fcreate_mock_device();
            let device_id = new_device._Fid().to_string();

            self.devices
                .insert(device_id.clone(), Arc::new(RwLock::new(new_device.clone())));
            self.device_type_index
                .entry(new_device._Fdevice_type())
                .or_default()
                .push(device_id);

            discovered_devices.push(new_device);
        }

        // Remove devices that haven't been seen for a while
        let timeout = chrono::TimeDelta::minutes(5);
        let now = Utc::now();

        let mut to_remove = Vec::new();
        for (device_id, device_lock) in &self.devices {
            match device_lock.try_read() {
                Ok(device) => {
                    if now.signed_duration_since(device._Flast_seen()) > timeout {
                        to_remove.push(device_id.clone());
                    }
                },
                Err(_) => {
                    // Failed to acquire read lock, skip this device for now
                    continue;
                }
            }
        }

        for device_id in &to_remove {
            self._Fremove_device(device_id).await?;
            removed_devices.push(device_id.to_string());
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
        let cpu1 = DMSDevice::new("CPU-1".to_string(), DMSDeviceType::CPU).with_capabilities(
            DMSDeviceCapabilities::new()
                ._Fwith_compute_units(8)
                ._Fwith_memory_gb(16.0),
        );

        let cpu2 = DMSDevice::new("CPU-2".to_string(), DMSDeviceType::CPU).with_capabilities(
            DMSDeviceCapabilities::new()
                ._Fwith_compute_units(16)
                ._Fwith_memory_gb(32.0),
        );

        // GPU devices
        let gpu1 = DMSDevice::new("GPU-1".to_string(), DMSDeviceType::GPU).with_capabilities(
            DMSDeviceCapabilities::new()
                ._Fwith_compute_units(2560)
                ._Fwith_memory_gb(24.0)
                ._Fwith_custom_capability("cuda_version".to_string(), "11.8".to_string()),
        );

        // Memory devices
        let memory1 = DMSDevice::new("Memory-1".to_string(), DMSDeviceType::Memory)
            .with_capabilities(
                DMSDeviceCapabilities::new()
                    ._Fwith_memory_gb(128.0)
                    ._Fwith_bandwidth_gbps(25.6),
            );

        // Storage devices
        let storage1 = DMSDevice::new("Storage-1".to_string(), DMSDeviceType::Storage)
            .with_capabilities(
                DMSDeviceCapabilities::new()
                    ._Fwith_storage_gb(1000.0)
                    ._Fwith_bandwidth_gbps(6.0),
            );

        // Network devices
        let network1 = DMSDevice::new("Network-1".to_string(), DMSDeviceType::Network)
            .with_capabilities(
                DMSDeviceCapabilities::new()
                    ._Fwith_bandwidth_gbps(10.0)
                    ._Fwith_custom_capability("protocol".to_string(), "ethernet".to_string()),
            );

        // Add devices to controller
        let devices = vec![cpu1, cpu2, gpu1, memory1, storage1, network1];
        for mut device in devices {
            device._Fset_status(DMSDeviceStatus::Available);
            device._Fset_location("Local System".to_string());

            let device_id = device._Fid().to_string();
            let device_type = device._Fdevice_type();

            self.devices
                .insert(device_id.clone(), Arc::new(RwLock::new(device)));
            self.device_type_index
                .entry(device_type)
                .or_default()
                .push(device_id);
        }

        Ok(())
    }

    fn _Fcreate_mock_device(&self) -> DMSDevice {
        // Create a random mock device
        let device_types = [
            "CPU".to_string(),
            "GPU".to_string(),
            "Memory".to_string(),
            "Storage".to_string(),
            "Network".to_string(),
        ];
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
                capabilities =
                    capabilities._Fwith_bandwidth_gbps(rand::random::<f64>() * 100.0 + 1.0);
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
        requirements: &DMSDeviceCapabilities,
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

                if device._Fis_available() && device.capabilities().meets_requirements(requirements)
                {
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
    pub async fn _Fallocate_device(
        &mut self,
        device_id: &str,
        allocation_id: &str,
    ) -> DMSResult<()> {
        if let Some(device_lock) = self.devices.get(device_id) {
            let mut device = device_lock.write().await;

            if device._Fallocate(allocation_id) {
                self.allocation_map
                    .insert(allocation_id.to_string(), device_id.to_string());
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
                Err(crate::core::DMSError::DeviceNotFound { device_id })
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

    /// Perform health check on all devices
    pub async fn _Fperform_health_checks(&mut self) -> DMSResult<Vec<(String, u8)>> {
        let mut results = Vec::new();

        for (device_id, device_lock) in &self.devices {
            let mut device = device_lock.write().await;

            // Simulate health check by updating health metrics
            let mut health_metrics = device.health_metrics().clone();

            // Simulate CPU and memory usage
            health_metrics.cpu_usage_percent = rand::random::<f64>() * 100.0;
            health_metrics.memory_usage_percent = rand::random::<f64>() * 100.0;

            // Simulate temperature
            health_metrics.temperature_celsius = rand::random::<f64>() * 50.0 + 30.0;

            // Simulate error count (occasionally increment)
            if rand::random::<f64>() < 0.01 {
                // 1% chance
                health_metrics.error_count += 1;
            }

            // Simulate throughput
            health_metrics.throughput = rand::random::<u64>() % 1000;

            // Update device health metrics
            device._Fupdate_health_metrics(health_metrics);

            // Calculate health score
            let health_score = device._Fdynamic_health_score(device.health_metrics());

            // Update device status based on health score
            if health_score < 20 {
                device._Fset_status(DMSDeviceStatus::Error);
            } else if health_score < 50 {
                device._Fset_status(DMSDeviceStatus::Maintenance);
            } else if device._Fstatus() == DMSDeviceStatus::Error
                || device._Fstatus() == DMSDeviceStatus::Maintenance
            {
                device._Fset_status(DMSDeviceStatus::Available);
            }

            results.push((device_id.to_string(), health_score));
        }

        Ok(results)
    }

    /// Start periodic health checks
    pub async fn _Fstart_health_checks(&self, interval_secs: u64) -> tokio::task::JoinHandle<()> {
        let devices = self.devices.clone();

        tokio::spawn(async move {
            let mut interval =
                tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));

            loop {
                interval.tick().await;

                for device_lock in devices.values() {
                    let mut device = device_lock.write().await;

                    // Simulate health check by updating health metrics
                    let mut health_metrics = device.health_metrics().clone();

                    // Simulate CPU and memory usage
                    health_metrics.cpu_usage_percent = rand::random::<f64>() * 100.0;
                    health_metrics.memory_usage_percent = rand::random::<f64>() * 100.0;

                    // Simulate temperature
                    health_metrics.temperature_celsius = rand::random::<f64>() * 50.0 + 30.0;

                    // Simulate error count (occasionally increment)
                    if rand::random::<f64>() < 0.01 {
                        // 1% chance
                        health_metrics.error_count += 1;
                    }

                    // Simulate throughput
                    health_metrics.throughput = rand::random::<u64>() % 1000;

                    // Update device health metrics
                    device._Fupdate_health_metrics(health_metrics);

                    // Calculate health score
                    let health_score = device._Fdynamic_health_score(device.health_metrics());

                    // Update device status based on health score
                    if health_score < 20 {
                        device._Fset_status(DMSDeviceStatus::Error);
                    } else if health_score < 50 {
                        device._Fset_status(DMSDeviceStatus::Maintenance);
                    } else if device._Fstatus() == DMSDeviceStatus::Error
                        || device._Fstatus() == DMSDeviceStatus::Maintenance
                    {
                        device._Fset_status(DMSDeviceStatus::Available);
                    }
                }
            }
        })
    }

    /// Get device health metrics
    pub async fn _Fget_device_health(
        &self,
        device_id: &str,
    ) -> DMSResult<super::device::DMSDeviceHealthMetrics> {
        if let Some(device_lock) = self.devices.get(device_id) {
            let device = device_lock.read().await;
            Ok(device.health_metrics().clone())
        } else {
            Err(crate::core::DMSError::DeviceNotFound {
                device_id: device_id.to_string(),
            })
        }
    }

    /// Get all device health metrics
    pub async fn _Fget_all_device_health(
        &self,
    ) -> DMSResult<HashMap<String, super::device::DMSDeviceHealthMetrics>> {
        let mut health_map = HashMap::new();

        for (device_id, device_lock) in &self.devices {
            let device = device_lock.read().await;
            health_map.insert(device_id.to_string(), device.health_metrics().clone());
        }

        Ok(health_map)
    }
}
