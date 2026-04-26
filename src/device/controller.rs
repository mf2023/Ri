//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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
//! This file implements the device controller for the Ri framework, responsible for managing the
//! lifecycle and state of devices in the system. It provides functionality for device discovery,
//! allocation, health monitoring, and state management.
//!
//! ## Key Components
//!
//! - **RiDeviceController**: Main device controller struct
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
//! use ri::device::{RiDeviceController, RiDeviceType, RiDeviceCapabilities};
//! use ri::core::RiResult;
//!
//! async fn example() -> RiResult<()> {
//!     // Create a new device controller
//!     let mut controller = RiDeviceController::new();
//!     
//!     // Add mock devices for testing
//!     controller.add_mock_devices()?;
//!     
//!     // Discover devices in the system
//!     let discovery_result = controller.discover_devices().await?;
//!     println!("Discovered {} devices", discovery_result.total_devices);
//!     
//!     // Find a suitable CPU device
//!     let requirements = RiDeviceCapabilities::new()
//!         .with_compute_units(8)
//!         .with_memory_gb(16.0);
//!     
//!     if let Some(device) = controller.find_suitable_device(&RiDeviceType::CPU, &requirements).await? {
//!         println!("Found suitable device: {}", device.id());
//!         
//!         // Allocate the device
//!         controller.allocate_device(device.id(), "allocation-1").await?;
//!         println!("Allocated device: {}", device.id());
//!         
//!         // Release the device
//!         controller.release_device_by_allocation("allocation-1").await?;
//!         println!("Released device: {}", device.id());
//!     }
//!     
//!     // Perform health checks
//!     let health_results = controller.perform_health_checks().await?;
//!     for (device_id, health_score) in health_results {
//!         println!("Device {} health score: {}", device_id, health_score);
//!     }
//!     
//!     Ok(())
//! }
//! ```

use chrono::Utc;
use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

use super::core::{RiDevice, RiDeviceCapabilities, RiDeviceStatus, RiDeviceType, RiDeviceControlConfig, RiNetworkDeviceInfo};
use super::discovery::{RiDeviceDiscovery, DiscoveryConfig};
use crate::core::RiResult;
use crate::prelude::RiMetricsRegistry;
#[cfg(not(target_os = "macos"))]
use crate::prelude::RiError;
// use super::scheduler::RiDeviceScheduler;

/// Device controller - manages device lifecycle and state
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiDeviceController {
    devices: FxHashMap<String, Arc<RwLock<RiDevice>>>,
    device_type_index: FxHashMap<RiDeviceType, Vec<String>>,
    allocation_map: FxHashMap<String, String>,
    discovery: Option<Arc<RiDeviceDiscovery>>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiDeviceController {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    #[staticmethod]
    fn default_controller() -> Self {
        Self::default()
    }
    
    #[pyo3(name = "discover_devices")]
    fn discover_devices_impl(&mut self) -> PyResult<super::RiDiscoveryResult> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.discover_devices()).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Device discovery failed: {}", e))
        })
    }
    
    #[pyo3(name = "discover_system_devices")]
    fn discover_system_devices_impl(&mut self, config: &RiDeviceControlConfig) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.discover_system_devices(config)).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("System device discovery failed: {}", e))
        })
    }
    
    #[pyo3(name = "find_suitable_device")]
    fn find_suitable_device_impl(&self, device_type: &RiDeviceType, requirements: &RiDeviceCapabilities) -> PyResult<Option<RiDevice>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.find_suitable_device(device_type, requirements)).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to find suitable device: {}", e))
        })
    }
    
    #[pyo3(name = "allocate_device")]
    fn allocate_device_impl(&mut self, device_id: &str, allocation_id: &str) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.allocate_device(device_id, allocation_id)).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Device allocation failed: {}", e))
        })
    }
    
    #[pyo3(name = "release_device_by_allocation")]
    fn release_device_by_allocation_impl(&mut self, allocation_id: &str) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.release_device_by_allocation(allocation_id)).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Device release failed: {}", e))
        })
    }
    
    #[pyo3(name = "get_all_devices")]
    fn get_all_devices_impl(&self) -> Vec<RiDevice> {
        self.get_all_devices()
    }
    
    #[pyo3(name = "release_all_devices")]
    fn release_all_devices_impl(&mut self) -> PyResult<()> {
        self.release_all_devices().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to release all devices: {}", e))
        })
    }
    
    #[pyo3(name = "perform_health_checks")]
    fn perform_health_checks_impl(&mut self) -> PyResult<Vec<(String, u8)>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.perform_health_checks()).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Health checks failed: {}", e))
        })
    }
    
    #[pyo3(name = "get_device_health")]
    fn get_device_health_impl(&self, device_id: &str) -> PyResult<super::core::RiDeviceHealthMetrics> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.get_device_health(device_id)).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get device health: {}", e))
        })
    }
    
    #[pyo3(name = "get_all_device_health")]
    fn get_all_device_health_impl(&self) -> PyResult<FxHashMap<String, super::core::RiDeviceHealthMetrics>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(self.get_all_device_health()).map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get all device health: {}", e))
        })
    }
    
    #[pyo3(name = "device_count")]
    fn device_count_impl(&self) -> usize {
        self.devices.len()
    }
    
    #[pyo3(name = "get_devices_by_type")]
    fn get_devices_by_type_impl(&self, device_type: &RiDeviceType) -> Vec<RiDevice> {
        let device_ids = match self.device_type_index.get(device_type) {
            Some(ids) => ids.clone(),
            None => return Vec::new(),
        };
        
        let mut devices = Vec::with_capacity(4);
        for device_id in device_ids {
            if let Some(device_lock) = self.devices.get(&device_id) {
                if let Ok(device) = device_lock.try_read() {
                    devices.push(device.clone());
                }
            }
        }
        devices
    }
    
    #[pyo3(name = "start_health_checks")]
    fn start_health_checks_impl(&self, interval_secs: u64) -> PyResult<String> {
        let _handle = self.start_health_checks(interval_secs);
        Ok(format!("Health check task started with interval {} seconds", interval_secs))
    }
}

impl Default for RiDeviceController {
    fn default() -> Self {
        Self::new()
    }
}

impl RiDeviceController {
    pub fn new() -> Self {
        Self {
            devices: FxHashMap::default(),
            device_type_index: FxHashMap::default(),
            allocation_map: FxHashMap::default(),
            discovery: None,
        }
    }

    /// Creates a new controller with the discovery engine
    pub async fn with_discovery(discovery: Arc<RiDeviceDiscovery>) -> Self {
        Self {
            devices: FxHashMap::default(),
            device_type_index: FxHashMap::default(),
            allocation_map: FxHashMap::default(),
            discovery: Some(discovery),
        }
    }

    /// Initializes the discovery engine
    pub async fn init_discovery(&mut self) -> RiResult<()> {
        let config = DiscoveryConfig::default();
        let discovery = Arc::new(RiDeviceDiscovery::new(config).await?);
        self.discovery = Some(discovery);
        Ok(())
    }
    
    /// Discover devices in the system
    pub async fn discover_devices(&mut self) -> RiResult<super::RiDiscoveryResult> {
        // In a real implementation, this would scan the system/network for devices
        // For now, we'll simulate discovery and update existing mock devices
        
        // Retry mechanism for device discovery
        let max_retries = 3;
        let retry_delay = std::time::Duration::from_millis(500);
        
        for attempt in 0..max_retries {
            match self.perform_device_discovery().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_retries - 1 {
                        // Last attempt failed, return error
                        return Err(e);
                    }
                    
                    // Log retry attempt
                    let error_msg = format!("Device discovery attempt {} failed: {}, retrying in {}ms", 
                                          attempt + 1, e, retry_delay.as_millis());
                    log::warn!("{error_msg}");
                    
                    // Wait before retrying
                    tokio::time::sleep(retry_delay).await;
                }
            }
        }
        
        // This line should never be reached due to the retry loop
        Err(crate::core::RiError::Other("Device discovery failed after maximum retries".to_string()))
    }
    
    /// Performs the actual device discovery logic with proper error handling.
    /// 
    /// # Returns
    /// 
    /// A `RiResult<super::RiDiscoveryResult>` containing the discovery result if successful.
    async fn perform_device_discovery(&mut self) -> RiResult<super::RiDiscoveryResult> {
        let mut discovered_devices = Vec::with_capacity(4);
        let mut updated_devices = Vec::with_capacity(4);
        let mut removed_devices = Vec::with_capacity(4);

        // Update existing devices with error handling
        for device_lock in self.devices.values() {
            match device_lock.try_write() {
                Ok(mut device) => {
                    device.update_last_seen();

                    // Improved device status detection based on health metrics
                    let health_metrics = device.health_metrics().clone();
                    let device_type = device.device_type();
                    
                    // Realistic device status update based on health metrics
                    match device_type {
                        RiDeviceType::CPU => {
                            // CPU devices are affected by high CPU usage and temperature
                            if health_metrics.cpu_usage_percent > 95.0 || health_metrics.temperature_celsius > 90.0 {
                                device.set_status(RiDeviceStatus::Degraded);
                            } else if health_metrics.cpu_usage_percent > 80.0 || health_metrics.temperature_celsius > 80.0 {
                                device.set_status(RiDeviceStatus::Busy);
                            } else if device.status() != RiDeviceStatus::Allocated {
                                device.set_status(RiDeviceStatus::Available);
                            }
                        },
                        RiDeviceType::GPU => {
                            // GPU devices are affected by high usage, temperature, and memory usage
                            if health_metrics.cpu_usage_percent > 95.0 || health_metrics.temperature_celsius > 95.0 {
                                device.set_status(RiDeviceStatus::Degraded);
                            } else if health_metrics.cpu_usage_percent > 85.0 || health_metrics.temperature_celsius > 85.0 {
                                device.set_status(RiDeviceStatus::Busy);
                            } else if device.status() != RiDeviceStatus::Allocated {
                                device.set_status(RiDeviceStatus::Available);
                            }
                        },
                        RiDeviceType::Network => {
                            // Network devices are affected by high latency
                            if health_metrics.network_latency_ms > 200.0 {
                                device.set_status(RiDeviceStatus::Degraded);
                            } else if health_metrics.network_latency_ms > 100.0 {
                                device.set_status(RiDeviceStatus::Busy);
                            } else if device.status() != RiDeviceStatus::Allocated {
                                device.set_status(RiDeviceStatus::Available);
                            }
                        },
                        RiDeviceType::Storage => {
                            // Storage devices are affected by high response time
                            if health_metrics.response_time_ms > 100.0 {
                                device.set_status(RiDeviceStatus::Degraded);
                            } else if health_metrics.response_time_ms > 50.0 {
                                device.set_status(RiDeviceStatus::Busy);
                            } else if device.status() != RiDeviceStatus::Allocated {
                                device.set_status(RiDeviceStatus::Available);
                            }
                        },
                        _ => {
                            // Default status update for other device types
                            if health_metrics.error_count > 5 {
                                device.set_status(RiDeviceStatus::Degraded);
                            } else if device.status() != RiDeviceStatus::Allocated {
                                device.set_status(RiDeviceStatus::Available);
                            }
                        }
                    }

                    updated_devices.push(device.clone());
                },
                Err(_) => {
                    // Failed to acquire write lock, skip this device for now
                    continue;
                }
            }
        }

        // Discover real hardware devices
        let new_hardware_devices = self.discover_hardware_devices().await?;
        
        // Add discovered hardware devices
        for device in new_hardware_devices {
            let device_id = device.id().to_string();
            
            // Check if device already exists
            if !self.devices.contains_key(&device_id) {
                self.devices.insert(device_id.clone(), Arc::new(RwLock::new(device.clone())));
                self.device_type_index
                    .entry(device.device_type())
                    .or_default()
                    .push(device_id);
                
                discovered_devices.push(device);
            }
        }
        
        // Occasionally add new mock devices for testing and demonstration
        if rand::random::<f64>() < 0.05 {
            // 5% chance
            let new_device = self.create_mock_device_for_discovery();
            let device_id = new_device.id().to_string();

            self.devices
                .insert(device_id.clone(), Arc::new(RwLock::new(new_device.clone())));
            self.device_type_index
                .entry(new_device.device_type())
                .or_default()
                .push(device_id);

            discovered_devices.push(new_device);
        }

        // Remove devices that haven't been seen for a while
        let timeout = chrono::TimeDelta::minutes(5);
        let now = Utc::now();

        let mut to_remove = Vec::with_capacity(4);
        for (device_id, device_lock) in &self.devices {
            match device_lock.try_read() {
                Ok(device) => {
                    if now.signed_duration_since(device.last_seen()) > timeout {
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
            self.remove_device(device_id).await?;
            removed_devices.push(device_id.to_string());
        }

        Ok(super::RiDiscoveryResult {
            discovered_devices,
            updated_devices,
            removed_devices,
            total_devices: self.devices.len(),
        })
    }

    /// Discover real system devices based on configuration
    pub async fn discover_system_devices(&mut self, config: &RiDeviceControlConfig) -> RiResult<()> {
        // Discover CPU devices
        self.discover_cpu_devices(config).await?;
        
        // Discover GPU devices
        self.discover_gpu_devices(config).await?;
        
        // Discover memory devices
        self.discover_memory_devices(config).await?;
        
        // Discover storage devices
        self.discover_storage_devices(config).await?;
        
        // Discover network devices
        self.discover_network_devices(config).await?;
        
        Ok(())
    }
    
    /// Discover GPU devices from system
    async fn discover_gpu_devices(&mut self, _config: &RiDeviceControlConfig) -> RiResult<()> {
        #[cfg(target_os = "windows")]
        {
            // Try NVIDIA GPU discovery first
            if let Ok(nvidia_output) = std::process::Command::new("nvidia-smi")
                .args(["--query-gpu=name,memory.total", "--format=csv,noheader"])
                .output() 
            {
                let gpu_info = String::from_utf8_lossy(&nvidia_output.stdout);
                
                for (index, line) in gpu_info.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let memory_mb = parts[1].trim().replace(" MiB", "").parse::<f64>().unwrap_or(0.0);
                        let memory_gb = memory_mb / 1024.0;
                        
                        let gpu_device = RiDevice::new(
                            format!("GPU-{}-{}", index + 1, name), 
                            RiDeviceType::GPU
                        ).with_capabilities(
                            RiDeviceCapabilities::new()
                                .with_compute_units(1000) // Estimate
                                .with_memory_gb(memory_gb)
                        );
                        
                        self.add_device(gpu_device, "NVIDIA GPU".to_string()).await?;
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Try NVIDIA GPU discovery
            if let Ok(nvidia_output) = std::process::Command::new("nvidia-smi")
                .args(&["--query-gpu=name,memory.total", "--format=csv,noheader"])
                .output()
            {
                let gpu_info = String::from_utf8_lossy(&nvidia_output.stdout);
                
                for (index, line) in gpu_info.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let memory_mb = parts[1].trim().replace(" MiB", "").parse::<f64>().unwrap_or(0.0);
                        let memory_gb = memory_mb / 1024.0;
                        
                        let gpu_device = RiDevice::new(
                            format!("GPU-{}-{}", index + 1, name), 
                            RiDeviceType::GPU
                        ).with_capabilities(
                            RiDeviceCapabilities::new()
                                .with_compute_units(1000) // Estimate
                                .with_memory_gb(memory_gb)
                        );
                        
                        self.add_device(gpu_device, "NVIDIA GPU".to_string()).await?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Discover storage devices from system
    #[allow(dead_code)]
    async fn discover_storage_devices_impl(&mut self, _config: &RiDeviceControlConfig) -> RiResult<Vec<RiDevice>> {
        #[cfg(target_os = "windows")]
        {
            // Try NVIDIA GPU discovery first
            if let Ok(nvidia_output) = std::process::Command::new("nvidia-smi")
                .args(["--query-gpu=name,memory.total", "--format=csv,noheader"])
                .output() 
            {
                let gpu_info = String::from_utf8_lossy(&nvidia_output.stdout);
                
                for (index, line) in gpu_info.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let memory_mb = parts[1].trim().replace(" MiB", "").parse::<f64>().unwrap_or(0.0);
                        let memory_gb = memory_mb / 1024.0;
                        
                        let gpu_device = RiDevice::new(
                            format!("GPU-{}-{}", index + 1, name), 
                            RiDeviceType::GPU
                        ).with_capabilities(
                            RiDeviceCapabilities::new()
                                .with_compute_units(1000) // Estimate
                                .with_memory_gb(memory_gb)
                        );
                        
                        self.add_device(gpu_device, "NVIDIA GPU".to_string()).await?;
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Try NVIDIA GPU discovery
            if let Ok(nvidia_output) = std::process::Command::new("nvidia-smi")
                .args(&["--query-gpu=name,memory.total", "--format=csv,noheader"])
                .output()
            {
                let gpu_info = String::from_utf8_lossy(&nvidia_output.stdout);
                
                for (index, line) in gpu_info.lines().enumerate() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if parts.len() >= 2 {
                        let name = parts[0].trim();
                        let memory_mb = parts[1].trim().replace(" MiB", "").parse::<f64>().unwrap_or(0.0);
                        let memory_gb = memory_mb / 1024.0;
                        
                        let gpu_device = RiDevice::new(
                            format!("GPU-{}-{}", index + 1, name), 
                            RiDeviceType::GPU
                        ).with_capabilities(
                            RiDeviceCapabilities::new()
                                .with_compute_units(1000) // Estimate
                                .with_memory_gb(memory_gb)
                        );
                        
                        self.add_device(gpu_device, "NVIDIA GPU".to_string()).await?;
                    }
                }
            }
        }
        
        Ok(vec![])
    }
    
    /// Discover memory devices from system
    async fn discover_memory_devices(&mut self, _config: &RiDeviceControlConfig) -> RiResult<()> {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("wmic")
                .args(["memorychip", "get", "Capacity,Speed", "/format:list"])
                .output()
                .map_err(|e| RiError::DeviceError(format!("Failed to query memory info: {e}")))?;
                
            let memory_info = String::from_utf8_lossy(&output.stdout);
            
            let mut total_capacity_gb = 0.0;
            let mut memory_modules = 0;
            
            for line in memory_info.lines() {
                if line.starts_with("Capacity=") {
                    if let Some(capacity_bytes) = line.split('=').nth(1).and_then(|s| s.trim().parse::<u64>().ok()) {
                        total_capacity_gb += capacity_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                        memory_modules += 1;
                    }
                }
            }
            
            if memory_modules > 0 {
                let memory_device = RiDevice::new(
                    format!("Memory-{}GB-total", total_capacity_gb.round() as u32), 
                    RiDeviceType::Memory
                ).with_capabilities(
                    RiDeviceCapabilities::new()
                        .with_memory_gb(total_capacity_gb)
                        .with_bandwidth_gbps(25.6) // Estimate for DDR4
                );
                
                self.add_device(memory_device, "System Memory".to_string()).await?;
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Ok(meminfo) = std::fs::read_to_string("/proc/meminfo") {
                for line in meminfo.lines() {
                    if line.starts_with("MemTotal:") {
                        if let Some(kb_str) = line.split_whitespace().nth(1) {
                            if let Ok(kb) = kb_str.parse::<f64>() {
                                let total_gb = kb / (1024.0 * 1024.0);
                                
                                let memory_device = RiDevice::new(
                                    format!("Memory-{}GB-total", total_gb.round() as u32), 
                                    RiDeviceType::Memory
                                ).with_capabilities(
                                    RiDeviceCapabilities::new()
                                        .with_memory_gb(total_gb)
                                        .with_bandwidth_gbps(25.6) // Estimate for DDR4
                                );
                                
                                self.add_device(memory_device, "System Memory".to_string()).await?;
                                break;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Discover CPU devices from system
    async fn discover_cpu_devices(&mut self, _config: &RiDeviceControlConfig) -> RiResult<()> {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("wmic")
                .args(["cpu", "get", "Name,NumberOfCores,NumberOfLogicalProcessors", "/format:list"])
                .output()
                .map_err(|e| RiError::DeviceError(format!("Failed to query CPU info: {e}")))?;
                
            let cpu_info = String::from_utf8_lossy(&output.stdout);
            
            let mut cpu_count = 0;
            let mut total_cores = 0;
            let mut total_threads = 0;
            
            for line in cpu_info.lines() {
                if line.starts_with("Name=") {
                    cpu_count += 1;
                } else if line.starts_with("NumberOfCores=") {
                    if let Some(cores) = line.split('=').nth(1).and_then(|s| s.trim().parse::<usize>().ok()) {
                        total_cores += cores;
                    }
                } else if line.starts_with("NumberOfLogicalProcessors=") {
                    if let Some(threads) = line.split('=').nth(1).and_then(|s| s.trim().parse::<usize>().ok()) {
                        total_threads += threads;
                    }
                }
            }
            
            if cpu_count > 0 {
                let cpu_device = RiDevice::new(
                    format!("CPU-{total_cores}-cores-{total_threads}-threads"), 
                    RiDeviceType::CPU
                ).with_capabilities(
                    RiDeviceCapabilities::new()
                        .with_compute_units(total_cores)
                        .with_memory_gb(0.0)
                );
                
                self.add_device(cpu_device, "System Hardware".to_string()).await?;
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Linux CPU discovery using /proc/cpuinfo
            let cpu_info = std::fs::read_to_string("/proc/cpuinfo")
                .map_err(|e| RiError::DeviceError(format!("Failed to read cpuinfo: {}", e)))?;
                
            let mut cpu_count = 0;
            let mut total_cores = 0;
            
            for line in cpu_info.lines() {
                if line.starts_with("processor\t") {
                    cpu_count += 1;
                } else if line.starts_with("cpu cores\t") {
                    if let Some(cores) = line.split(':').nth(1).and_then(|s| s.trim().parse::<usize>().ok()) {
                        total_cores = cores;
                    }
                }
            }
            
            let total_threads = cpu_count; // In Linux, processor count equals thread count
            
            if cpu_count > 0 {
                let cpu_device = RiDevice::new(
                    format!("CPU-{}-cores-{}-threads", total_cores, total_threads), 
                    RiDeviceType::CPU
                ).with_capabilities(
                    RiDeviceCapabilities::new()
                        .with_compute_units(total_cores)
                        .with_memory_gb(0.0)
                );
                
                self.add_device(cpu_device, "System Hardware".to_string()).await?;
            }
        }
        
        Ok(())
    }
    
    /// Discover storage devices from system
    async fn discover_storage_devices(&mut self, config: &RiDeviceControlConfig) -> RiResult<()> {
        // Call the implementation
        self.discover_storage_devices_impl2(config).await
    }
    
    /// Discover storage devices from system (implementation)
    async fn discover_storage_devices_impl2(&mut self, _config: &RiDeviceControlConfig) -> RiResult<()> {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("wmic")
                .args(["diskdrive", "get", "Model,Size", "/format:list"])
                .output()
                .map_err(|e| RiError::DeviceError(format!("Failed to query disk info: {e}")))?;
                
            let disk_info = String::from_utf8_lossy(&output.stdout);
            
            let mut disk_counter = 0;
            // Store lines in a vector for easier access
            let lines: Vec<&str> = disk_info.lines().collect();
            
            for (disk_index, line) in lines.iter().enumerate() {
                if line.starts_with("Model=") {
                    let model = line.split('=').nth(1).unwrap_or("Unknown").trim();
                    disk_counter += 1;
                    
                    // Look for the size in the next line
                    if disk_index + 1 < lines.len() && lines[disk_index + 1].starts_with("Size=") {
                        let size_line = lines[disk_index + 1];
                        if let Some(size_bytes) = size_line.split('=').nth(1).and_then(|s| s.trim().parse::<u64>().ok()) {
                            let size_gb = size_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                            
                            let storage_device = RiDevice::new(
                                format!("Storage-{disk_counter}-{model}"), 
                                RiDeviceType::Storage
                            ).with_capabilities(
                                RiDeviceCapabilities::new()
                                    .with_storage_gb(size_gb)
                                    .with_bandwidth_gbps(6.0) // SATA III estimate
                            );
                            
                            self.add_device(storage_device, "System Storage".to_string()).await?;
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Read block devices from /sys/block
            if let Ok(entries) = std::fs::read_dir("/sys/block") {
                for (index, entry) in entries.enumerate() {
                    if let Ok(entry) = entry {
                        let device_name = entry.file_name().to_string_lossy().to_string();
                        
                        // Skip loop devices and ram disks
                        if device_name.starts_with("loop") || device_name.starts_with("ram") {
                            continue;
                        }
                        
                        // Try to read size
                        let size_path = entry.path().join("size");
                        if let Ok(size_str) = std::fs::read_to_string(&size_path) {
                            if let Ok(size_sectors) = size_str.trim().parse::<u64>() {
                                let size_gb = (size_sectors * 512) as f64 / (1024.0 * 1024.0 * 1024.0);
                                
                                let storage_device = RiDevice::new(
                                    format!("Storage-{}-{}", index + 1, device_name), 
                                    RiDeviceType::Storage
                                ).with_capabilities(
                                    RiDeviceCapabilities::new()
                                        .with_storage_gb(size_gb)
                                        .with_bandwidth_gbps(6.0) // SATA III estimate
                                );
                                
                                self.add_device(storage_device, "System Storage".to_string()).await?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Discover real hardware devices using the new discovery engine
    async fn discover_hardware_devices(&mut self) -> RiResult<Vec<RiDevice>> {
        // Use the new discovery engine if available
        if let Some(discovery) = &self.discovery {
            let devices = discovery.discover_all().await?;
            return Ok(devices);
        }

        // Fallback to old discovery if no engine initialized
        let mut temp_controller = RiDeviceController::new();
        let config = RiDeviceControlConfig::default();
        temp_controller.discover_system_devices(&config).await?;
        Ok(temp_controller.get_all_devices())
    }
    
    /// Discover network devices from system
    async fn discover_network_devices(&mut self, _config: &RiDeviceControlConfig) -> RiResult<()> {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("wmic")
                .args(["nic", "where", "NetEnabled=true", "get", "Name,Speed", "/format:list"])
                .output()
                .map_err(|e| RiError::DeviceError(format!("Failed to query network info: {e}")))?;
                
            let network_info = String::from_utf8_lossy(&output.stdout);
            
            let mut network_counter = 0;
            // Store lines in a vector for easier access
            let lines: Vec<&str> = network_info.lines().collect();
            
            for (network_index, line) in lines.iter().enumerate() {
                if line.starts_with("Name=") {
                    let name = line.split('=').nth(1).unwrap_or("Unknown").trim();
                    network_counter += 1;
                    
                    // Look for speed in next lines
                    if let Some(speed_line) = lines.iter().skip(network_index + 1).find(|l| l.starts_with("Speed=")) {
                        if let Some(speed_bps) = speed_line.split('=').nth(1).and_then(|s| s.trim().parse::<u64>().ok()) {
                            let speed_gbps = speed_bps as f64 / (1000.0 * 1000.0 * 1000.0);
                            
                            let network_device = RiDevice::new(
                                format!("Network-{network_counter}-{name}"), 
                                RiDeviceType::Network
                            ).with_capabilities(
                                RiDeviceCapabilities::new()
                                    .with_bandwidth_gbps(speed_gbps)
                            );
                            
                            self.add_device(network_device, "System Network".to_string()).await?;
                        }
                    }
                }
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // Read network interfaces from /sys/class/net
            if let Ok(entries) = std::fs::read_dir("/sys/class/net") {
                for (index, entry) in entries.enumerate() {
                    if let Ok(entry) = entry {
                        let interface_name = entry.file_name().to_string_lossy().to_string();
                        
                        // Skip loopback interface
                        if interface_name == "lo" {
                            continue;
                        }
                        
                        // Try to read speed
                        let speed_path = entry.path().join("speed");
                        if let Ok(speed_str) = std::fs::read_to_string(&speed_path) {
                            if let Ok(speed_mbps) = speed_str.trim().parse::<f64>() {
                                let speed_gbps = speed_mbps / 1000.0;
                                
                                let network_device = RiDevice::new(
                                    format!("Network-{}-{}", index + 1, interface_name), 
                                    RiDeviceType::Network
                                ).with_capabilities(
                                    RiDeviceCapabilities::new()
                                        .with_bandwidth_gbps(speed_gbps)
                                );
                                
                                self.add_device(network_device, "System Network".to_string()).await?;
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Helper method to add a discovered device
    #[allow(dead_code)]
    async fn add_device(&mut self, mut device: RiDevice, location: String) -> RiResult<()> {
        device.set_status(RiDeviceStatus::Available);
        device.set_location(location);
        
        let device_id = device.id().to_string();
        let device_type = device.device_type();
        
        self.devices.insert(device_id.clone(), Arc::new(RwLock::new(device)));
        self.device_type_index
            .entry(device_type)
            .or_default()
            .push(device_id);
            
        Ok(())
    }

    /// Create a device from network discovery (for remote devices)
    #[allow(dead_code)]
    fn create_discovered_device(&self, device_info: &RiNetworkDeviceInfo) -> RiDevice {
        let device_type_enum = match device_info.device_type.as_str() {
            "CPU" => RiDeviceType::CPU,
            "GPU" => RiDeviceType::GPU,
            "Memory" => RiDeviceType::Memory,
            "Storage" => RiDeviceType::Storage,
            "Network" => RiDeviceType::Network,
            _ => RiDeviceType::Custom,
        };
        
        let name = format!("Discovered-{}-{}", device_info.device_type, device_info.id);
        let mut device = RiDevice::new(name, device_type_enum);

        // Add discovered capabilities
        let mut capabilities = RiDeviceCapabilities::new();

        match device_type_enum {
            RiDeviceType::CPU => {
                capabilities = capabilities
                    .with_compute_units(device_info.compute_units.unwrap_or(8))
                    .with_memory_gb(device_info.memory_gb.unwrap_or(16.0));
            }
            RiDeviceType::GPU => {
                capabilities = capabilities
                    .with_compute_units(device_info.compute_units.unwrap_or(1000))
                    .with_memory_gb(device_info.memory_gb.unwrap_or(8.0));
            }
            RiDeviceType::Memory => {
                capabilities = capabilities
                    .with_memory_gb(device_info.memory_gb.unwrap_or(64.0))
                    .with_bandwidth_gbps(device_info.bandwidth_gbps.unwrap_or(25.6));
            }
            RiDeviceType::Storage => {
                capabilities = capabilities
                    .with_storage_gb(device_info.storage_gb.unwrap_or(1000.0))
                    .with_bandwidth_gbps(device_info.bandwidth_gbps.unwrap_or(6.0));
            }
            RiDeviceType::Network => {
                capabilities = capabilities
                    .with_bandwidth_gbps(device_info.bandwidth_gbps.unwrap_or(1.0));
            }
            _ => {}
        }

        device = device.with_capabilities(capabilities);
        device.set_status(RiDeviceStatus::Available);
        device.set_location(format!("Network Discovery: {}", device_info.source));

        device
    }

    /// Find a suitable device for the given requirements
    pub async fn find_suitable_device(
        &self,
        device_type: &RiDeviceType,
        requirements: &RiDeviceCapabilities,
    ) -> RiResult<Option<RiDevice>> {
        let device_ids = match self.device_type_index.get(device_type) {
            Some(ids) => ids.clone(),
            None => return Ok(None),
        };

        // Find the best available device
        let mut best_device: Option<RiDevice> = None;
        let mut best_score = 0u32;

        for device_id in device_ids {
            if let Some(device_lock) = self.devices.get(&device_id) {
                let device = device_lock.read().await;

                if device.is_available() && device.capabilities().meets_requirements(requirements)
                {
                    let score = self.calculate_device_score(&device);

                    if score > best_score || best_device.is_none() {
                        best_device = Some(device.clone());
                        best_score = score;
                    }
                }
            }
        }

        Ok(best_device)
    }
    
    /// Initialize metrics for device monitoring
    pub fn initialize_metrics(&mut self, metrics_registry: &RiMetricsRegistry) -> RiResult<()> {
        use crate::observability::{RiMetric, RiMetricConfig, RiMetricType};
        use std::sync::Arc;
        
        // Register device count metric
        let device_total_config = RiMetricConfig {
            metric_type: RiMetricType::Gauge,
            name: "dms_devices_total".to_string(),
            help: "Total number of discovered devices".to_string(),
            buckets: vec![],
            quantiles: vec![],
            max_age: std::time::Duration::from_secs(300),
            age_buckets: 5,
        };
        let device_total_metric = Arc::new(RiMetric::new(device_total_config));
        metrics_registry.register(device_total_metric.clone())?;
        
        // Register device type metrics
        for device_type in self.device_type_index.keys() {
            let device_type_config = RiMetricConfig {
                metric_type: RiMetricType::Gauge,
                name: format!("dms_devices_{}_total", device_type.to_string().to_lowercase()),
                help: format!("Total number of {device_type} devices"),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            let device_type_metric = Arc::new(RiMetric::new(device_type_config));
            metrics_registry.register(device_type_metric.clone())?;
        }
        
        Ok(())
    }

    fn calculate_device_score(&self, device: &RiDevice) -> u32 {
        let mut score = device.health_score() as u32 * 100;

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
    pub async fn allocate_device(
        &mut self,
        device_id: &str,
        allocation_id: &str,
    ) -> RiResult<()> {
        if let Some(device_lock) = self.devices.get(device_id) {
            let mut device = device_lock.write().await;

            if device.allocate(allocation_id) {
                self.allocation_map
                    .insert(allocation_id.to_string(), device_id.to_string());
                Ok(())
            } else {
                Err(crate::core::RiError::DeviceAllocationFailed {
                    device_id: device_id.to_string(),
                    reason: "Device not available".to_string(),
                })
            }
        } else {
            Err(crate::core::RiError::DeviceNotFound {
                device_id: device_id.to_string(),
            })
        }
    }

    /// Release a device by allocation ID
    pub async fn release_device_by_allocation(&mut self, allocation_id: &str) -> RiResult<()> {
        if let Some(device_id) = self.allocation_map.remove(allocation_id) {
            if let Some(device_lock) = self.devices.get(&device_id) {
                let mut device = device_lock.write().await;
                device.release();
                Ok(())
            } else {
                Err(crate::core::RiError::DeviceNotFound { device_id })
            }
        } else {
            Err(crate::core::RiError::AllocationNotFound {
                allocation_id: allocation_id.to_string(),
            })
        }
    }

    /// Remove a device
    async fn remove_device(&mut self, device_id: &str) -> RiResult<()> {
        if let Some(device_lock) = self.devices.remove(device_id) {
            let device = device_lock.read().await;
            let device_type = device.device_type();

            // Remove from type index
            if let Some(type_devices) = self.device_type_index.get_mut(&device_type) {
                type_devices.retain(|id| id != device_id);
            }

            // Remove any allocations
            if let Some(allocation_id) = device.get_allocation_id() {
                self.allocation_map.remove(allocation_id);
            }
        }

        Ok(())
    }

    /// Get all devices
    pub fn get_all_devices(&self) -> Vec<RiDevice> {
        let mut devices = Vec::with_capacity(4);

        // This is a blocking operation - in a real implementation, we'd use async
        for device_lock in self.devices.values() {
            if let Ok(device) = device_lock.try_read() {
                devices.push(device.clone());
            }
        }

        devices
    }

    /// Release all devices (shutdown)
    pub fn release_all_devices(&mut self) -> RiResult<()> {
        // Clear all allocations
        self.allocation_map.clear();

        // Release all devices
        for device_lock in self.devices.values() {
            if let Ok(mut device) = device_lock.try_write() {
                device.release();
            }
        }

        Ok(())
    }

    /// Perform health check on all devices
    pub async fn perform_health_checks(&mut self) -> RiResult<Vec<(String, u8)>> {
        let mut results = Vec::with_capacity(4);

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
            
            // Simulate network latency (for network devices)
            health_metrics.network_latency_ms = rand::random::<f64>() * 200.0;
            
            // Simulate disk IOPS (for storage devices)
            health_metrics.disk_iops = (rand::random::<f64>() * 500.0) as u64;
            
            // Simulate battery level
            health_metrics.battery_level_percent = rand::random::<f64>() * 100.0;
            
            // Simulate response time
            health_metrics.response_time_ms = rand::random::<f64>() * 150.0;
            
            // Simulate uptime (increment by 30 seconds each check)
            health_metrics.uptime_seconds += 30;

            // Update device health metrics
            device.update_health_metrics(health_metrics);

            // Calculate health score
            let health_score = device.dynamic_health_score(device.health_metrics());

            // Update device status based on health score
            if health_score < 20 {
                device.set_status(RiDeviceStatus::Error);
            } else if health_score < 50 {
                device.set_status(RiDeviceStatus::Maintenance);
            } else if health_score < 70 {
                device.set_status(RiDeviceStatus::Degraded);
            } else if device.status() == RiDeviceStatus::Error
                || device.status() == RiDeviceStatus::Maintenance
                || device.status() == RiDeviceStatus::Degraded
            {
                device.set_status(RiDeviceStatus::Available);
            }

            results.push((device_id.to_string(), health_score));
        }

        Ok(results)
    }

    /// Start periodic health checks
    pub async fn start_health_checks(&self, interval_secs: u64) -> tokio::task::JoinHandle<()> {
        let devices = self.devices.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));

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
                    
                    // Simulate network latency (for network devices)
                    health_metrics.network_latency_ms = rand::random::<f64>() * 200.0;
                    
                    // Simulate disk IOPS (for storage devices)
                    health_metrics.disk_iops = (rand::random::<f64>() * 500.0) as u64;
                    
                    // Simulate battery level
                    health_metrics.battery_level_percent = rand::random::<f64>() * 100.0;
                    
                    // Simulate response time
                    health_metrics.response_time_ms = rand::random::<f64>() * 150.0;
                    
                    // Simulate uptime (increment by the interval each check)
                    health_metrics.uptime_seconds += interval_secs as u64;

                    // Update device health metrics
                    device.update_health_metrics(health_metrics);

                    // Calculate health score
                    let health_score = device.dynamic_health_score(device.health_metrics());

                    // Update device status based on health score
                    if health_score < 20 {
                        device.set_status(RiDeviceStatus::Error);
                    } else if health_score < 50 {
                        device.set_status(RiDeviceStatus::Maintenance);
                    } else if health_score < 70 {
                        device.set_status(RiDeviceStatus::Degraded);
                    } else if device.status() == RiDeviceStatus::Error
                        || device.status() == RiDeviceStatus::Maintenance
                        || device.status() == RiDeviceStatus::Degraded
                    {
                        device.set_status(RiDeviceStatus::Available);
                    }
                }
            }
        })
    }

    /// Get device health metrics
    pub async fn get_device_health(
        &self,
        device_id: &str,
    ) -> RiResult<super::core::RiDeviceHealthMetrics> {
        if let Some(device_lock) = self.devices.get(device_id) {
            let device = device_lock.read().await;
            Ok(device.health_metrics().clone())
        } else {
            Err(crate::core::RiError::DeviceNotFound {
                device_id: device_id.to_string(),
            })
        }
    }

    /// Get all device health metrics
    pub async fn get_all_device_health(
        &self,
    ) -> RiResult<FxHashMap<String, super::core::RiDeviceHealthMetrics>> {
        let mut health_map = FxHashMap::default();

        for (device_id, device_lock) in &self.devices {
            let device = device_lock.read().await;
            health_map.insert(device_id.to_string(), device.health_metrics().clone());
        }

        Ok(health_map)
    }

    /// Create a mock device for discovery simulation
    fn create_mock_device_for_discovery(&self) -> RiDevice {
        use super::core::{RiDeviceCapabilities, RiDeviceType};
        
        let device_types = [RiDeviceType::CPU,
            RiDeviceType::GPU,
            RiDeviceType::Memory,
            RiDeviceType::Storage,
            RiDeviceType::Network];
        
        let device_type = device_types[rand::random::<usize>() % device_types.len()];
        
        let device_name = match device_type {
            RiDeviceType::CPU => format!("CPU-{}-cores", rand::random::<usize>() % 32 + 1),
            RiDeviceType::GPU => format!("GPU-{}-GB", rand::random::<usize>() % 24 + 1),
            RiDeviceType::Memory => format!("Memory-{}-GB", rand::random::<usize>() % 64 + 1),
            RiDeviceType::Storage => format!("Storage-{}-TB", rand::random::<usize>() % 10 + 1),
            RiDeviceType::Network => format!("Network-{}-Gbps", rand::random::<usize>() % 100 + 1),
            RiDeviceType::Sensor => format!("Sensor-{}-units", rand::random::<usize>() % 100 + 1),
            RiDeviceType::Actuator => format!("Actuator-{}-actions", rand::random::<usize>() % 50 + 1),
            RiDeviceType::Custom => format!("Custom-{}-device", rand::random::<usize>() % 1000 + 1),
        };
        
        let capabilities = RiDeviceCapabilities::new()
            .with_compute_units(rand::random::<usize>() % 1000 + 100)
            .with_memory_gb(rand::random::<f64>() * 64.0 + 1.0);
        
        RiDevice::new(device_name, device_type)
            .with_capabilities(capabilities)
    }
}
