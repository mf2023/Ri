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
use chrono::{DateTime, Utc};

use super::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities};
use super::pool::DMSResourcePoolManager;
use std::sync::{Arc, Mutex};

/// # Device Scheduler
/// 
/// This file implements a comprehensive device scheduler for DMS, responsible for:
/// 1. Managing device resource allocation and scheduling
/// 2. Implementing multiple scheduling algorithms
/// 3. Recording allocation history and statistics
/// 4. Providing scheduling recommendations based on historical data
/// 
/// ## Design Principles
/// 
/// 1. **Multiple Scheduling Algorithms**: Supports FirstFit, BestFit, WorstFit, RoundRobin, PriorityBased, and LoadBalanced policies
/// 2. **Policy Per Device Type**: Different device types can have different scheduling policies
/// 3. **Allocation History**: Maintains detailed records of all allocations
/// 4. **Statistics and Recommendations**: Provides insights into scheduling effectiveness and recommendations for optimization
/// 5. **Resource Pool Integration**: Works closely with the resource pool manager
/// 6. **Thread Safety**: Uses Arc and Mutex for thread-safe operations
/// 7. **Scalability**: Designed to handle large numbers of devices and allocations
/// 
/// ## Usage Examples
/// 
/// ```rust
/// use dms::device::{DMSDeviceScheduler, DMSAllocationRequest, DMSDeviceType, DMSDeviceCapabilities};
/// use dms::device::pool::DMSResourcePoolManager;
/// use std::sync::{Arc, Mutex};
/// 
/// fn example() {
///     // Create resource pool manager
///     let pool_manager = Arc::new(Mutex::new(DMSResourcePoolManager::_Fnew()));
///     
///     // Create device scheduler
///     let mut scheduler = DMSDeviceScheduler::_Fnew(pool_manager);
///     
///     // Set scheduling policy for GPUs
///     scheduler._Fset_policy(DMSDeviceType::GPU, dms::device::DMSSchedulingPolicy::PriorityBased);
///     
///     // Create allocation request
///     let request = DMSAllocationRequest {
///         device_type: DMSDeviceType::GPU,
///         capabilities: DMSDeviceCapabilities {
///             memory_gb: Some(16.0),
///             compute_units: Some(512),
///             storage_gb: None,
///             bandwidth_gbps: None,
///             custom_capabilities: HashMap::new(),
///         },
///         priority: 5,
///         timeout_secs: 30,
///     };
///     
///     // Allocate device
///     if let Some(allocation_id) = scheduler._Fallocate(&request) {
///         println!("Allocated device with ID: {}", allocation_id);
///         
///         // Record release when done
///         scheduler._Frecord_release(&allocation_id);
///     }
///     
///     // Get statistics
///     let stats = scheduler._Fget_statistics();
///     println!("Total allocations: {}", stats.total_allocations);
/// }
/// ```

/// Device scheduler - manages device resource allocation and scheduling
/// 
/// This struct implements a comprehensive device scheduling system that manages
/// resource allocation using various scheduling algorithms. It works closely with
/// the resource pool manager to access available devices and implements multiple
/// scheduling policies per device type.
pub struct DMSDeviceScheduler {
    /// Scheduling policies per device type
    scheduling_policies: HashMap<DMSDeviceType, DMSSchedulingPolicy>,
    /// History of all allocations
    allocation_history: Vec<DMSAllocationRecord>,
    /// Resource pool manager for accessing available devices
    resource_pool_manager: Arc<Mutex<DMSResourcePoolManager>>,
    /// Round-robin counters per device type
    round_robin_counters: HashMap<DMSDeviceType, usize>,
}

/// Scheduling policy enum - defines different algorithms for device selection
/// 
/// This enum defines the available scheduling policies that can be applied to different
/// device types. Each policy implements a different algorithm for selecting devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSSchedulingPolicy {
    /// FirstFit: Select the first device that meets requirements
    FirstFit,
    /// BestFit: Select the device that best matches the requirements
    BestFit,
    /// WorstFit: Select the device with the most remaining capacity
    WorstFit,
    /// RoundRobin: Select devices in rotation
    RoundRobin,
    /// PriorityBased: Select device based on request priority and device health
    PriorityBased,
    /// LoadBalanced: Select device with lowest current load
    LoadBalanced,
}

/// Allocation record - details of a device allocation
/// 
/// This struct records detailed information about each device allocation, including
/// the device used, capabilities required, allocation and release times, and success status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSAllocationRecord {
    /// Unique allocation identifier
    pub allocation_id: String,
    /// ID of the allocated device
    pub device_id: String,
    /// Type of the allocated device
    pub device_type: DMSDeviceType,
    /// Time when the device was allocated
    pub allocated_at: DateTime<Utc>,
    /// Time when the device was released (if applicable)
    pub released_at: Option<DateTime<Utc>>,
    /// Duration of the allocation in seconds (if completed)
    pub duration_seconds: Option<f64>,
    /// Whether the allocation was successful
    pub success: bool,
    /// Capabilities required for this allocation
    pub capabilities_required: DMSDeviceCapabilities,
}

/// Allocation request - request for device resources
/// 
/// This struct represents a request for device resources, including the device type,
/// required capabilities, priority, and timeout.
#[derive(Debug, Clone)]
pub struct DMSAllocationRequest {
    /// Type of device requested
    pub device_type: DMSDeviceType,
    /// Capabilities required from the device
    pub capabilities: DMSDeviceCapabilities,
    /// Priority of this request (0-255, higher = higher priority)
    pub priority: u32,
    /// Timeout in seconds for this request
    pub timeout_secs: u64,
}

impl DMSDeviceScheduler {
    /// Create a new device scheduler with default policies and empty allocation history.
    /// 
    /// This method initializes the scheduler with default scheduling policies for each device type
    /// and connects it to the provided resource pool manager.
    /// 
    /// # Parameters
    /// 
    /// - `resource_pool_manager`: Resource pool manager for accessing available devices
    /// 
    /// # Returns
    /// 
    /// A new `DMSDeviceScheduler` instance with default policies and settings.
    pub fn _Fnew(resource_pool_manager: Arc<Mutex<DMSResourcePoolManager>>) -> Self {
        let mut scheduling_policies = HashMap::new();
        
        // Set default policies for different device types
        scheduling_policies.insert(DMSDeviceType::CPU, DMSSchedulingPolicy::LoadBalanced);
        scheduling_policies.insert(DMSDeviceType::GPU, DMSSchedulingPolicy::PriorityBased);
        scheduling_policies.insert(DMSDeviceType::Memory, DMSSchedulingPolicy::BestFit);
        scheduling_policies.insert(DMSDeviceType::Storage, DMSSchedulingPolicy::FirstFit);
        scheduling_policies.insert(DMSDeviceType::Network, DMSSchedulingPolicy::RoundRobin);
        scheduling_policies.insert(DMSDeviceType::Sensor, DMSSchedulingPolicy::FirstFit);
        scheduling_policies.insert(DMSDeviceType::Actuator, DMSSchedulingPolicy::PriorityBased);
        scheduling_policies.insert(DMSDeviceType::Custom, DMSSchedulingPolicy::LoadBalanced);
        
        Self {
            scheduling_policies,
            allocation_history: Vec::new(),
            resource_pool_manager,
            round_robin_counters: HashMap::new(),
        }
    }
    
    /// Get the scheduling policy for a specific device type.
    /// 
    /// This method returns the scheduling policy configured for the specified device type.
    /// If no policy is configured, it returns the default FirstFit policy.
    /// 
    /// # Parameters
    /// 
    /// - `device_type`: Device type to get the policy for
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSSchedulingPolicy` for the specified device type.
    pub fn _Fget_policy(&self, device_type: &DMSDeviceType) -> &DMSSchedulingPolicy {
        self.scheduling_policies.get(device_type).unwrap_or(&DMSSchedulingPolicy::FirstFit)
    }
    
    /// Set the scheduling policy for a specific device type.
    /// 
    /// This method configures the scheduling policy for the specified device type.
    /// 
    /// # Parameters
    /// 
    /// - `device_type`: Device type to set the policy for
    /// - `policy`: Scheduling policy to use for this device type
    pub fn _Fset_policy(&mut self, device_type: DMSDeviceType, policy: DMSSchedulingPolicy) {
        self.scheduling_policies.insert(device_type, policy);
    }
    
    /// Select a device based on the scheduling policy for the requested device type.
    /// 
    /// This method selects the best device for the allocation request by:
    /// 1. Getting the appropriate scheduling policy for the device type
    /// 2. Collecting available devices from the resource pool manager
    /// 3. Filtering devices that meet the requirements
    /// 4. Applying the scheduling policy to select the best device
    /// 
    /// # Parameters
    /// 
    /// - `request`: Allocation request with device type, capabilities, priority, and timeout
    /// 
    /// # Returns
    /// 
    /// An `Arc<DMSDevice>` if a suitable device was found, or `None` if no device meets the requirements.
    pub fn _Fselect_device(&mut self, request: &DMSAllocationRequest) -> Option<Arc<DMSDevice>> {
        // Get policy first to avoid borrow conflicts
        let policy = self._Fget_policy(&request.device_type);
        
        // Collect available devices while holding the lock
        let available_devices = {
            let pool_manager = self.resource_pool_manager.lock().unwrap();
            let pools = pool_manager._Fget_pools_by_type(request.device_type);
            
            if pools.is_empty() {
                return None;
            }
            
            let mut devices: Vec<Arc<DMSDevice>> = Vec::new();
            
            // Collect all available devices from all pools
            for pool in &pools {
                let pool_devices = pool._Fget_available_devices();
                devices.extend(pool_devices.into_iter()
                    .filter(|device| device.capabilities().meets_requirements(&request.capabilities)));
            }
            
            devices
        };
        
        if available_devices.is_empty() {
            return None;
        }
        
        // Apply scheduling policy after releasing the lock
        match policy {
            DMSSchedulingPolicy::FirstFit => self._Ffirst_fit(&available_devices),
            DMSSchedulingPolicy::BestFit => self._Fbest_fit(&available_devices, &request.capabilities),
            DMSSchedulingPolicy::WorstFit => self._Fworst_fit(&available_devices, &request.capabilities),
            DMSSchedulingPolicy::RoundRobin => self._Fround_robin(&available_devices, request.device_type),
            DMSSchedulingPolicy::PriorityBased => self._Fpriority_based(&available_devices, request.priority),
            DMSSchedulingPolicy::LoadBalanced => self._Fload_balanced(&available_devices),
        }
    }
    
    /// First Fit algorithm: select the first device that meets requirements.
    /// 
    /// This algorithm selects the first device in the list that meets the requirements.
    /// It's simple and fast, but may not be the most efficient in terms of resource utilization.
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// 
    /// # Returns
    /// 
    /// The first device in the list, or `None` if the list is empty.
    fn _Ffirst_fit(&self, devices: &[Arc<DMSDevice>]) -> Option<Arc<DMSDevice>> {
        devices.first().cloned()
    }
    
    /// Best Fit algorithm: select the device that best matches the requirements.
    /// 
    /// This algorithm selects the device that best matches the requirements by calculating
    /// a fitness score based on the ratio of required resources to available resources.
    /// It aims to minimize wasted resources.
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// - `requirements`: Required capabilities for the allocation
    /// 
    /// # Returns
    /// 
    /// The device with the best fitness score, or `None` if no devices meet the requirements.
    fn _Fbest_fit(&self, devices: &[Arc<DMSDevice>], requirements: &DMSDeviceCapabilities) -> Option<Arc<DMSDevice>> {
        devices.iter()
            .filter(|device| device.capabilities().meets_requirements(requirements))
            .min_by(|a, b| {
                // Calculate fitness score based on resource usage ratios
                let a_fitness = self._Fcalculate_fitness_score(a.capabilities(), requirements);
                let b_fitness = self._Fcalculate_fitness_score(b.capabilities(), requirements);
                a_fitness.partial_cmp(&b_fitness).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }
    
    /// Worst Fit algorithm: select the device with the most remaining capacity.
    /// 
    /// This algorithm selects the device with the most remaining capacity, which can help
    /// prevent fragmentation of resources. It's useful when dealing with varying allocation sizes.
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// - `requirements`: Required capabilities for the allocation
    /// 
    /// # Returns
    /// 
    /// The device with the highest remaining capacity score, or `None` if no devices meet the requirements.
    fn _Fworst_fit(&self, devices: &[Arc<DMSDevice>], requirements: &DMSDeviceCapabilities) -> Option<Arc<DMSDevice>> {
        devices.iter()
            .filter(|device| device.capabilities().meets_requirements(requirements))
            .max_by(|a, b| {
                // Calculate remaining capacity score
                let a_score = self._Fcalculate_remaining_capacity_score(a.capabilities());
                let b_score = self._Fcalculate_remaining_capacity_score(b.capabilities());
                a_score.partial_cmp(&b_score).unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }
    
    /// Round Robin algorithm: select devices in rotation.
    /// 
    /// This algorithm selects devices in rotation, distributing allocations evenly across all
    /// available devices. It's simple and ensures fair distribution of work.
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// - `device_type`: Type of device being allocated
    /// 
    /// # Returns
    /// 
    /// The next device in the rotation, or `None` if no devices meet the requirements.
    fn _Fround_robin(&mut self, devices: &[Arc<DMSDevice>], device_type: DMSDeviceType) -> Option<Arc<DMSDevice>> {
        let counter = self.round_robin_counters.entry(device_type)
            .or_insert(0);
        
        let index = *counter % devices.len();
        *counter += 1;
        
        devices.get(index).cloned()
    }
    
    /// Priority Based algorithm: select device based on request priority and device health.
    /// 
    /// This algorithm selects devices based on a combination of request priority and device health.
    /// Higher priority requests get higher quality devices (higher health scores).
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// - `priority`: Priority of the allocation request
    /// 
    /// # Returns
    /// 
    /// The device with the highest priority-weighted health score, or `None` if no devices meet the requirements.
    fn _Fpriority_based(&self, devices: &[Arc<DMSDevice>], priority: u32) -> Option<Arc<DMSDevice>> {
        // For higher priority requests, select devices with higher health scores
        devices.iter()
            .max_by(|a, b| {
                let a_score = a._Fhealth_score() as u32 * priority;
                let b_score = b._Fhealth_score() as u32 * priority;
                a_score.cmp(&b_score)
            })
            .cloned()
    }
    
    /// Load Balanced algorithm: select device with lowest current load.
    /// 
    /// This algorithm selects the device with the lowest current load, using health score as a proxy for load.
    /// It aims to distribute work evenly based on device capacity.
    /// 
    /// # Parameters
    /// 
    /// - `devices`: List of devices that meet the requirements
    /// 
    /// # Returns
    /// 
    /// The device with the lowest load (highest health score), or `None` if no devices meet the requirements.
    fn _Fload_balanced(&self, devices: &[Arc<DMSDevice>]) -> Option<Arc<DMSDevice>> {
        devices.iter()
            .min_by(|a, b| {
                // Use health score as a proxy for load (inverse relationship)
                a._Fhealth_score().cmp(&b._Fhealth_score()).reverse()
            })
            .cloned()
    }
    
    /// Calculate fitness score for Best Fit algorithm.
    /// 
    /// This method calculates a fitness score based on the ratio of required resources to available resources.
    /// Lower scores indicate better fit (more efficient resource utilization).
    /// 
    /// # Parameters
    /// 
    /// - `device_cap`: Device capabilities
    /// - `requirements`: Required capabilities
    /// 
    /// # Returns
    /// 
    /// A fitness score between 0.0 and potentially over 1.0, where lower scores indicate better fit.
    fn _Fcalculate_fitness_score(&self, device_cap: &DMSDeviceCapabilities, requirements: &DMSDeviceCapabilities) -> f64 {
        let mut score = 0.0;
        
        // Calculate memory fitness
        if let (Some(req_mem), Some(avail_mem)) = (requirements.memory_gb, device_cap.memory_gb) {
            let used_ratio = req_mem / avail_mem;
            score += used_ratio;
        }
        
        // Calculate compute units fitness
        if let (Some(req_units), Some(avail_units)) = (requirements.compute_units, device_cap.compute_units) {
            let used_ratio = req_units as f64 / avail_units as f64;
            score += used_ratio;
        }
        
        // Calculate storage fitness
        if let (Some(req_storage), Some(avail_storage)) = (requirements.storage_gb, device_cap.storage_gb) {
            let used_ratio = req_storage / avail_storage;
            score += used_ratio;
        }
        
        score
    }
    
    /// Calculate remaining capacity score for Worst Fit algorithm.
    /// 
    /// This method calculates a score based on the remaining capacity of a device.
    /// Higher scores indicate more remaining capacity.
    /// 
    /// # Parameters
    /// 
    /// - `device_cap`: Device capabilities
    /// 
    /// # Returns
    /// 
    /// A score representing the remaining capacity, where higher scores indicate more capacity.
    fn _Fcalculate_remaining_capacity_score(&self, device_cap: &DMSDeviceCapabilities) -> f64 {
        let mut score = 0.0;
        
        // Add memory capacity
        if let Some(mem) = device_cap.memory_gb {
            score += mem;
        }
        
        // Add compute units (weighted by 100 to balance with memory)
        if let Some(units) = device_cap.compute_units {
            score += units as f64 * 100.0;
        }
        
        // Add storage capacity
        if let Some(storage) = device_cap.storage_gb {
            score += storage;
        }
        
        score
    }
    
    /// Allocate a device based on the scheduling policy.
    /// 
    /// This method selects a device using the appropriate scheduling policy and records the allocation.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Allocation request with device type, capabilities, priority, and timeout
    /// 
    /// # Returns
    /// 
    /// An allocation ID if successful, or `None` if no suitable device was found.
    pub fn _Fallocate(&mut self, request: &DMSAllocationRequest) -> Option<String> {
        if let Some(device) = self._Fselect_device(request) {
            // Generate unique allocation ID
            let allocation_id = uuid::Uuid::new_v4().to_string();
            
            // Note: In a real implementation, we'd need to lock the device and update its status
            // This is simplified for demonstration
            
            // Record the allocation
            self._Frecord_allocation(allocation_id.clone(), device._Fid().to_string(), device._Fdevice_type(), request.capabilities.clone());
            
            Some(allocation_id)
        } else {
            None
        }
    }
    
    /// Record an allocation in the history.
    /// 
    /// This method records a new allocation in the history, including device information,
    /// capabilities required, and allocation time.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: Unique allocation identifier
    /// - `device_id`: ID of the allocated device
    /// - `device_type`: Type of the allocated device
    /// - `capabilities_required`: Capabilities required for this allocation
    pub fn _Frecord_allocation(&mut self, allocation_id: String, device_id: String, device_type: DMSDeviceType, capabilities_required: DMSDeviceCapabilities) {
        let record = DMSAllocationRecord {
            allocation_id,
            device_id,
            device_type,
            allocated_at: Utc::now(),
            released_at: None,
            duration_seconds: None,
            success: true,
            capabilities_required,
        };
        
        self.allocation_history.push(record);
        
        // Keep only recent history (last 1000 allocations)
        if self.allocation_history.len() > 1000 {
            self.allocation_history.remove(0);
        }
    }
    
    /// Record the release of an allocation.
    /// 
    /// This method updates an allocation record to mark it as released, including the release time
    /// and duration of the allocation.
    /// 
    /// # Parameters
    /// 
    /// - `allocation_id`: ID of the allocation to release
    pub fn _Frecord_release(&mut self, allocation_id: &str) {
        if let Some(record) = self.allocation_history.iter_mut().find(|r| r.allocation_id == allocation_id) {
            record.released_at = Some(Utc::now());
            
            if let Ok(duration) = record.released_at.unwrap().signed_duration_since(record.allocated_at).to_std() {
                record.duration_seconds = Some(duration.as_secs_f64());
            }
        }
    }
    
    /// Get allocation statistics and metrics.
    /// 
    /// This method calculates comprehensive statistics about allocations, including:
    /// - Total and successful allocations
    /// - Success rate
    /// - Average allocation duration
    /// - Statistics by device type
    /// 
    /// # Returns
    /// 
    /// A `DMSAllocationStatistics` struct containing comprehensive allocation statistics.
    pub fn _Fget_statistics(&self) -> DMSAllocationStatistics {
        let total_allocations = self.allocation_history.len();
        let successful_allocations = self.allocation_history.iter().filter(|r| r.success).count();
        let failed_allocations = total_allocations - successful_allocations;
        
        let completed_allocations: Vec<&DMSAllocationRecord> = self.allocation_history.iter()
            .filter(|r| r.released_at.is_some())
            .collect();
        
        let total_duration_seconds: f64 = completed_allocations.iter()
            .filter_map(|r| r.duration_seconds)
            .sum();
        
        let average_duration_seconds = if !completed_allocations.is_empty() {
            total_duration_seconds / completed_allocations.len() as f64
        } else {
            0.0
        };
        
        // Statistics by device type
        let mut by_device_type = HashMap::new();
        for device_type in [DMSDeviceType::CPU, DMSDeviceType::GPU, DMSDeviceType::Memory, 
            DMSDeviceType::Storage, DMSDeviceType::Network, DMSDeviceType::Sensor, 
            DMSDeviceType::Actuator, DMSDeviceType::Custom] {
            let type_allocations = self.allocation_history.iter()
                .filter(|r| r.device_type == device_type)
                .count();
            
            if type_allocations > 0 {
                let type_completed = self.allocation_history.iter()
                    .filter(|r| r.device_type == device_type && r.released_at.is_some())
                    .count();
                
                let type_duration: f64 = self.allocation_history.iter()
                    .filter(|r| r.device_type == device_type)
                    .filter_map(|r| r.duration_seconds)
                    .sum();
                
                let type_avg_duration = if type_completed > 0 {
                    type_duration / type_completed as f64
                } else {
                    0.0
                };
                
                by_device_type.insert(device_type, DMSDeviceTypeStatistics {
                    total_allocations: type_allocations,
                    completed_allocations: type_completed,
                    average_duration_seconds: type_avg_duration,
                });
            }
        }
        
        DMSAllocationStatistics {
            total_allocations,
            successful_allocations,
            failed_allocations,
            success_rate: if total_allocations > 0 {
                (successful_allocations as f64 / total_allocations as f64) * 100.0
            } else {
                0.0
            },
            average_duration_seconds,
            by_device_type,
        }
    }
    
    /// Get scheduling recommendations based on historical data.
    /// 
    /// This method analyzes recent allocation patterns for the specified device type and generates
    /// recommendations for optimizing scheduling policies. Recommendations are based on success rates,
    /// allocation durations, and frequency.
    /// 
    /// # Parameters
    /// 
    /// - `device_type`: Device type to get recommendations for
    /// 
    /// # Returns
    /// 
    /// A vector of `DMSSchedulingRecommendation` sorted by priority (highest first).
    pub fn _Fget_recommendations(&self, device_type: &DMSDeviceType) -> Vec<DMSSchedulingRecommendation> {
        let mut recommendations = Vec::new();
        
        // Analyze recent allocation patterns for this device type
        let recent_allocations: Vec<&DMSAllocationRecord> = self.allocation_history.iter()
            .filter(|r| r.device_type == *device_type)
            .rev()
            .take(100) // Last 100 allocations
            .collect();
        
        if recent_allocations.is_empty() {
            recommendations.push(DMSSchedulingRecommendation {
                recommendation_type: DMSSchedulingRecommendationType::UseDefaultPolicy,
                description: format!("No recent allocation data for {device_type:?}, using default policy"),
                priority: 1,
                confidence: 0.5,
            });
            return recommendations;
        }
        
        let avg_duration = recent_allocations.iter()
            .filter_map(|r| r.duration_seconds)
            .sum::<f64>() / recent_allocations.len() as f64;
        
        let success_rate = recent_allocations.iter().filter(|r| r.success).count() as f64 
            / recent_allocations.len() as f64;
        
        // Generate recommendations based on patterns
        if success_rate < 0.8 {
            recommendations.push(DMSSchedulingRecommendation {
                recommendation_type: DMSSchedulingRecommendationType::ConsiderPolicyChange,
                description: format!("Low success rate ({:.1}%) for {:?}, consider changing scheduling policy", 
                    success_rate * 100.0, device_type),
                priority: 3,
                confidence: 0.8,
            });
        }
        
        if avg_duration > 300.0 { // 5 minutes
            recommendations.push(DMSSchedulingRecommendation {
                recommendation_type: DMSSchedulingRecommendationType::OptimizeForLongRunning,
                description: format!("Average allocation duration is {avg_duration:.1} seconds for {device_type:?}, consider load balancing"),
                priority: 2,
                confidence: 0.7,
            });
        }
        
        if recent_allocations.len() > 50 && avg_duration < 60.0 {
            recommendations.push(DMSSchedulingRecommendation {
                recommendation_type: DMSSchedulingRecommendationType::OptimizeForShortRunning,
                description: format!("High frequency of short allocations for {device_type:?}, consider round-robin scheduling"),
                priority: 2,
                confidence: 0.6,
            });
        }
        
        recommendations.push(DMSSchedulingRecommendation {
            recommendation_type: DMSSchedulingRecommendationType::ContinueCurrentPolicy,
            description: format!("Current scheduling policy appears effective for {device_type:?}"),
            priority: 1,
            confidence: 0.9,
        });
        
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }
}

/// Allocation statistics - comprehensive metrics about device allocations
/// 
/// This struct contains detailed statistics about device allocations, including success rates,
/// durations, and breakdowns by device type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSAllocationStatistics {
    /// Total number of allocations
    pub total_allocations: usize,
    /// Number of successful allocations
    pub successful_allocations: usize,
    /// Number of failed allocations
    pub failed_allocations: usize,
    /// Success rate as a percentage (0.0-100.0)
    pub success_rate: f64,
    /// Average duration of completed allocations in seconds
    pub average_duration_seconds: f64,
    /// Statistics broken down by device type
    pub by_device_type: HashMap<DMSDeviceType, DMSDeviceTypeStatistics>,
}

/// Device type statistics - metrics for a specific device type
/// 
/// This struct contains allocation statistics for a specific device type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceTypeStatistics {
    /// Total number of allocations for this device type
    pub total_allocations: usize,
    /// Number of completed allocations for this device type
    pub completed_allocations: usize,
    /// Average duration of completed allocations for this device type in seconds
    pub average_duration_seconds: f64,
}

/// Scheduling recommendation types - categories of scheduling recommendations
/// 
/// This enum defines the different types of scheduling recommendations that can be generated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSSchedulingRecommendationType {
    /// Use the default policy for this device type
    UseDefaultPolicy,
    /// Continue using the current policy
    ContinueCurrentPolicy,
    /// Consider changing the scheduling policy
    ConsiderPolicyChange,
    /// Optimize for long-running allocations
    OptimizeForLongRunning,
    /// Optimize for short-running allocations
    OptimizeForShortRunning,
    /// Use load balancing
    LoadBalance,
    /// Use priority-based scheduling
    Prioritize,
}

/// Scheduling recommendation - suggestion for optimizing scheduling
/// 
/// This struct represents a recommendation for optimizing scheduling, including the recommendation type,
/// description, priority, and confidence level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSSchedulingRecommendation {
    /// Type of recommendation
    pub recommendation_type: DMSSchedulingRecommendationType,
    /// Human-readable description of the recommendation
    pub description: String,
    /// Priority of the recommendation (1-10, higher = more important)
    pub priority: u8, // 1-10, higher is more important
    /// Confidence in the recommendation (0.0-1.0)
    pub confidence: f64, // 0.0-1.0, confidence in this recommendation
}