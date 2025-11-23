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

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DMSDeviceType {
    CPU,
    GPU,
    Memory,
    Storage,
    Network,
    Sensor,
    Actuator,
    Custom,
}

/// Device capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceCapabilities {
    pub compute_units: Option<usize>,
    pub memory_gb: Option<f64>,
    pub storage_gb: Option<f64>,
    pub bandwidth_gbps: Option<f64>,
    pub custom_capabilities: HashMap<String, String>,
}

impl DMSDeviceCapabilities {
    pub fn new() -> Self {
        Self {
            compute_units: None,
            memory_gb: None,
            storage_gb: None,
            bandwidth_gbps: None,
            custom_capabilities: HashMap::new(),
        }
    }
    
    pub fn _Fwith_compute_units(mut self, units: usize) -> Self {
        self.compute_units = Some(units);
        self
    }
    
    pub fn _Fwith_memory_gb(mut self, memory: f64) -> Self {
        self.memory_gb = Some(memory);
        self
    }
    
    pub fn _Fwith_storage_gb(mut self, storage: f64) -> Self {
        self.storage_gb = Some(storage);
        self
    }
    
    pub fn _Fwith_bandwidth_gbps(mut self, bandwidth: f64) -> Self {
        self.bandwidth_gbps = Some(bandwidth);
        self
    }
    
    pub fn _Fwith_custom_capability(mut self, key: String, value: String) -> Self {
        self.custom_capabilities.insert(key, value);
        self
    }
    
    /// Check if this device meets the required capabilities
    pub fn meets_requirements(&self, requirements: &DMSDeviceCapabilities) -> bool {
        if let Some(required_units) = requirements.compute_units {
            if let Some(available_units) = self.compute_units {
                if available_units < required_units {
                    return false;
                }
            } else {
                return false; // No compute units available
            }
        }
        
        if let Some(required_memory) = requirements.memory_gb {
            if let Some(available_memory) = self.memory_gb {
                if available_memory < required_memory {
                    return false;
                }
            } else {
                return false; // No memory available
            }
        }
        
        if let Some(required_storage) = requirements.storage_gb {
            if let Some(available_storage) = self.storage_gb {
                if available_storage < required_storage {
                    return false;
                }
            } else {
                return false; // No storage available
            }
        }
        
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

/// Device status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSDeviceStatus {
    Unknown,
    Available,
    Busy,
    Error,
    Offline,
    Maintenance,
}

/// Smart device representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDevice {
    id: String,
    name: String,
    device_type: DMSDeviceType,
    status: DMSDeviceStatus,
    capabilities: DMSDeviceCapabilities,
    location: Option<String>,
    metadata: HashMap<String, String>,
    last_seen: chrono::DateTime<chrono::Utc>,
    current_allocation_id: Option<String>,
}

impl DMSDevice {
    /// Create a new device with given name and type
    pub fn new(name: String, device_type: DMSDeviceType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            device_type,
            status: DMSDeviceStatus::Unknown,
            capabilities: DMSDeviceCapabilities::new(),
            location: None,
            metadata: HashMap::new(),
            last_seen: chrono::Utc::now(),
            current_allocation_id: None,
        }
    }
    
    pub fn _Fid(&self) -> &str {
        &self.id
    }
    
    pub fn _Fname(&self) -> &str {
        &self.name
    }
    
    pub fn _Fdevice_type(&self) -> DMSDeviceType {
        self.device_type
    }
    
    pub fn _Fstatus(&self) -> DMSDeviceStatus {
        self.status
    }
    
    pub fn capabilities(&self) -> &DMSDeviceCapabilities {
        &self.capabilities
    }
    
    pub fn _Fset_status(&mut self, status: DMSDeviceStatus) {
        self.status = status;
        self.last_seen = chrono::Utc::now();
    }
    
    pub fn with_capabilities(mut self, capabilities: DMSDeviceCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }
    
    pub fn _Fset_location(&mut self, location: String) {
        self.location = Some(location);
    }
    
    pub fn _Fadd_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
    
    pub fn _Fupdate_last_seen(&mut self) {
        self.last_seen = chrono::Utc::now();
    }
    
    pub fn _Flast_seen(&self) -> chrono::DateTime<chrono::Utc> {
        self.last_seen
    }
    
    pub fn _Fis_available(&self) -> bool {
        self.status == DMSDeviceStatus::Available && self.current_allocation_id.is_none()
    }
    
    pub fn _Fis_allocated(&self) -> bool {
        self.current_allocation_id.is_some()
    }
    
    pub fn _Fallocate(&mut self, allocation_id: &str) -> bool {
        if self._Fis_available() {
            self.current_allocation_id = Some(allocation_id.to_string());
            self.status = DMSDeviceStatus::Busy;
            true
        } else {
            false
        }
    }
    
    pub fn _Frelease(&mut self) {
        self.current_allocation_id = None;
        if self.status == DMSDeviceStatus::Busy {
            self.status = DMSDeviceStatus::Available;
        }
    }
    
    pub fn _Fget_allocation_id(&self) -> Option<&str> {
        self.current_allocation_id.as_deref()
    }
    
    /// Get device health score (0-100)
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
    
    /// Check if device is still responsive (last seen within timeout)
    pub fn _Fis_responsive(&self, timeout_secs: i64) -> bool {
        let elapsed = chrono::Utc::now() - self.last_seen;
        elapsed.num_seconds() < timeout_secs
    }
}