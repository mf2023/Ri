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

pub mod device;
pub mod controller;
pub mod scheduler;
pub mod pool;
pub mod discovery_scheduler;

use std::sync::Arc;

use serde::{Serialize, Deserialize};
use tokio::sync::RwLock;
use std::collections::HashMap;


pub use device::{DMSDevice, DMSDeviceType, DMSDeviceStatus, DMSDeviceCapabilities};
pub use controller::DMSDeviceController;
pub use pool::{DMSResourcePool, DMSResourcePoolManager, DMSResourcePoolStatistics};
pub use scheduler::DMSDeviceScheduler;

use crate::core::{DMSResult, DMSServiceContext};


/// Smart device control module for DMS - provides device discovery, control and resource scheduling
pub struct DMSDeviceControlModule {
    controller: Arc<RwLock<DMSDeviceController>>,
    #[allow(dead_code)]
    scheduler: Arc<DMSDeviceScheduler>,
    resource_pools: HashMap<String, Arc<DMSResourcePool>>,
    config: DMSDeviceControlConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceControlConfig {
    pub discovery_enabled: bool,
    pub discovery_interval_secs: u64,
    pub auto_scheduling_enabled: bool,
    pub max_concurrent_tasks: usize,
    pub resource_allocation_timeout_secs: u64,
}

impl Default for DMSDeviceControlConfig {
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

/// Device discovery result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDiscoveryResult {
    pub discovered_devices: Vec<DMSDevice>,
    pub updated_devices: Vec<DMSDevice>,
    pub removed_devices: Vec<String>, // device IDs
    pub total_devices: usize,
}

/// Resource allocation request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourceRequest {
    pub request_id: String,
    pub device_type: DMSDeviceType,
    pub required_capabilities: DMSDeviceCapabilities,
    pub priority: u8, // 1-10, higher is more important
    pub timeout_secs: u64,
}

/// Resource allocation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourceAllocation {
    pub allocation_id: String,
    pub device_id: String,
    pub device_name: String,
    pub allocated_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub request: DMSResourceRequest,
}

impl DMSDeviceControlModule {
    pub fn _Fnew() -> Self {
        let controller = Arc::new(RwLock::new(DMSDeviceController::_Fnew()));
        let scheduler = Arc::new(DMSDeviceScheduler::_Fnew());
        
        Self {
            controller,
            scheduler,
            resource_pools: HashMap::new(),
            config: DMSDeviceControlConfig::default(),
        }
    }
    
    pub fn _Fwith_config(mut self, config: DMSDeviceControlConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Discover devices in the network/environment
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
    
    /// Allocate a device resource
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
    
    /// Release a device resource
    pub async fn _Frelease_resource(&self, allocation_id: &str) -> DMSResult<()> {
        let mut controller = self.controller.write().await;
        controller._Frelease_device_by_allocation(allocation_id).await
    }
    
    /// Get current device status
    pub async fn _Fget_device_status(&self) -> DMSResult<Vec<DMSDevice>> {
        let controller = self.controller.read().await;
        Ok(controller._Fget_all_devices())
    }
    
    /// Get resource pool status
    pub fn _Fget_resource_pool_status(&self) -> HashMap<String, DMSResourcePoolStatus> {
        let mut status = HashMap::new();
        for (pool_name, pool) in &self.resource_pools {
            status.insert(pool_name.clone(), pool._Fget_status());
        }
        status
    }
    

}

/// Resource pool status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSResourcePoolStatus {
    pub total_capacity: usize,
    pub available_capacity: usize,
    pub allocated_capacity: usize,
    pub pending_requests: usize,
    pub utilization_rate: f64,
}

#[async_trait::async_trait]
impl crate::core::_CAsyncServiceModule for DMSDeviceControlModule {
    fn _Fname(&self) -> &str {
        "DMS.DeviceControl"
    }
    
    fn _Fis_critical(&self) -> bool {
        false // Non-critical, should not break the app if device control fails
    }
    
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
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module initialized")?;
        
        Ok(())
    }
    
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
    fn _Fname(&self) -> &str {
        "DMS.DeviceControl"
    }
    
    fn _Fis_critical(&self) -> bool {
        false // Non-critical, should not break the app if device control fails
    }
    
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
    
    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Release all allocated resources
        let mut controller = self.controller.blocking_write();
        controller._Frelease_all_devices()?;
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.DeviceControl", "Device control module shutdown completed")?;
        
        Ok(())
    }
}