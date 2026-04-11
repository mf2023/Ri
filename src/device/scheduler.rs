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

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use crate::device::RiResourceAllocation;
use tokio::sync::RwLock;

use super::core::{RiDevice, RiDeviceType, RiDeviceCapabilities};
use super::pool::RiResourcePoolManager;
use std::sync::Arc;

/// Resource scheduler for device management
#[allow(dead_code)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiResourceScheduler {
    /// Active allocations
    allocations: Arc<RwLock<HashMap<String, RiResourceAllocation>>>,
    /// Allocation history for analytics
    allocation_history: Arc<RwLock<Vec<RiResourceAllocation>>>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pymethods)]
impl RiResourceScheduler {
    #[cfg(feature = "pyo3")]
    #[new]
    fn new() -> Self {
        Self {
            allocations: Arc::new(RwLock::new(HashMap::new())),
            allocation_history: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

/// # Device Scheduler
/// 
/// This file implements a comprehensive device scheduler for Ri, responsible for:
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
/// use ri::device::{RiDeviceScheduler, RiAllocationRequest, RiDeviceType, RiDeviceCapabilities};
/// use ri::device::pool::RiResourcePoolManager;
/// use std::sync::{Arc, Mutex};
/// 
/// fn example() {
///     // Create resource pool manager
///     let pool_manager = Arc::new(Mutex::new(RiResourcePoolManager::new()));
///     
///     // Create device scheduler
///     let mut scheduler = RiDeviceScheduler::new(pool_manager);
///     
///     // Set scheduling policy for GPUs
///     scheduler.set_policy(RiDeviceType::GPU, ri::device::RiSchedulingPolicy::PriorityBased);
///     
///     // Create allocation request
///     let request = RiAllocationRequest {
///         device_type: RiDeviceType::GPU,
///         capabilities: RiDeviceCapabilities {
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
///     if let Some(allocation_id) = scheduler.allocate(&request) {
///         println!("Allocated device with ID: {}", allocation_id);
///         
///         // Record release when done
///         scheduler.record_release(&allocation_id);
///     }
///     
///     // Get statistics
///     let stats = scheduler.get_statistics();
///     println!("Total allocations: {}", stats.total_allocations);
/// }
/// ```
/// Device scheduler - manages device resource allocation and scheduling
///
/// This struct implements a comprehensive device scheduling system that manages
/// resource allocation using various scheduling algorithms. It works closely with
/// the resource pool manager to access available devices and implements multiple
/// scheduling policies per device type.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiDeviceScheduler {
    scheduling_policies: HashMap<RiDeviceType, RiSchedulingPolicy>,
    allocation_history: Arc<RwLock<Vec<RiAllocationRecord>>>,
    resource_pool_manager: Arc<RwLock<RiResourcePoolManager>>,
    round_robin_counters: Arc<RwLock<HashMap<RiDeviceType, usize>>>,
}

/// Scheduling policy enum - defines different algorithms for device selection
///
/// This enum defines the available scheduling policies that can be applied to different
/// device types. Each policy implements a different algorithm for selecting devices.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiSchedulingPolicy {
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiAllocationRecord {
    /// Unique allocation identifier
    pub allocation_id: String,
    /// ID of the allocated device
    pub device_id: String,
    /// Type of the allocated device
    pub device_type: RiDeviceType,
    /// Time when the device was allocated
    pub allocated_at: DateTime<Utc>,
    /// Time when the device was released (if applicable)
    pub released_at: Option<DateTime<Utc>>,
    /// Duration of the allocation in seconds (if completed)
    pub duration_seconds: Option<f64>,
    /// Whether the allocation was successful
    pub success: bool,
    /// Capabilities required for this allocation
    pub capabilities_required: RiDeviceCapabilities,
}

/// Allocation request - request for device resources
///
/// This struct represents a request for device resources, including the device type,
/// required capabilities, priority, timeout, and additional scheduling hints such as
/// SLA class, resource weights, and affinity rules. These extra fields are optional
/// and are used only by advanced scheduling logic.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiAllocationRequest {
    /// Type of device requested
    pub device_type: RiDeviceType,
    /// Capabilities required from the device
    pub capabilities: RiDeviceCapabilities,
    /// Priority of this request (0-255, higher = higher priority)
    pub priority: u32,
    /// Timeout in seconds for this request
    pub timeout_secs: u64,
    /// Optional SLA class propagated from the external resource request
    pub sla_class: Option<super::RiRequestSlaClass>,
    /// Optional resource weights propagated from the external resource request
    pub resource_weights: Option<super::RiResourceWeights>,
    /// Optional affinity rules propagated from the external resource request
    pub affinity: Option<super::RiAffinityRules>,
    /// Optional anti-affinity rules propagated from the external resource request
    pub anti_affinity: Option<super::RiAffinityRules>,
}

impl RiDeviceScheduler {
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
    /// A new `RiDeviceScheduler` instance with default policies and settings.
    pub fn new(resource_pool_manager: Arc<RwLock<RiResourcePoolManager>>) -> Self {
        let mut scheduling_policies = HashMap::new();
        
        // Set default policies for different device types
        scheduling_policies.insert(RiDeviceType::CPU, RiSchedulingPolicy::LoadBalanced);
        scheduling_policies.insert(RiDeviceType::GPU, RiSchedulingPolicy::PriorityBased);
        scheduling_policies.insert(RiDeviceType::Memory, RiSchedulingPolicy::BestFit);
        scheduling_policies.insert(RiDeviceType::Storage, RiSchedulingPolicy::FirstFit);
        scheduling_policies.insert(RiDeviceType::Network, RiSchedulingPolicy::RoundRobin);
        scheduling_policies.insert(RiDeviceType::Sensor, RiSchedulingPolicy::FirstFit);
        scheduling_policies.insert(RiDeviceType::Actuator, RiSchedulingPolicy::PriorityBased);
        scheduling_policies.insert(RiDeviceType::Custom, RiSchedulingPolicy::LoadBalanced);
        
        Self {
            scheduling_policies,
            allocation_history: Arc::new(RwLock::new(Vec::new())),
            resource_pool_manager,
            round_robin_counters: Arc::new(RwLock::new(HashMap::new())),
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
    /// A reference to the `RiSchedulingPolicy` for the specified device type.
    pub fn get_policy(&self, device_type: &RiDeviceType) -> &RiSchedulingPolicy {
        self.scheduling_policies.get(device_type).unwrap_or(&RiSchedulingPolicy::FirstFit)
    }
    
    /// Set the scheduling policy for a specific device type.
    /// 
    /// This method configures the scheduling policy for the specified device type.
    /// 
    /// # Parameters
    /// 
    /// - `device_type`: Device type to set the policy for
    /// - `policy`: Scheduling policy to use for this device type
    pub fn set_policy(&mut self, device_type: RiDeviceType, policy: RiSchedulingPolicy) {
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
    /// An `Arc<RiDevice>` if a suitable device was found, or `None` if no device meets the requirements.
    pub async fn select_device(&self, request: &RiAllocationRequest) -> Option<Arc<RiDevice>> {
        let policy = self.get_policy(&request.device_type);

        let available_devices = {
            let pool_manager = self.resource_pool_manager.read().await;
            let pools = pool_manager.get_pools_by_type(request.device_type);

            if pools.is_empty() {
                return None;
            }

            let mut devices: Vec<Arc<RiDevice>> = Vec::new();

            for pool in &pools {
                let pool_devices = pool.get_available_devices();
                devices.extend(pool_devices.into_iter());
            }

            devices
        };

        // Stage 1: filter candidates according to basic requirements.
        // This currently replicates the original capabilities-based filtering
        // and will be extended with SLA / affinity rules in future iterations.
        let filtered = self.filter_candidates(available_devices, request);
        if filtered.is_empty() {
            return None;
        }

        // Stage 2: scoring hook. For now this is effectively a no-op that
        // preserves the original behavior. Later we will use this hook to
        // incorporate SLA, resource weights and affinity into scoring.
        let scored = self.score_candidates(&filtered, request);

        // Stage 3: apply scheduling policy using the (optionally) scored list.
        match policy {
            RiSchedulingPolicy::FirstFit => self.first_fit(&scored),
            RiSchedulingPolicy::BestFit => self.best_fit(&scored, &request.capabilities),
            RiSchedulingPolicy::WorstFit => self.worst_fit(&scored, &request.capabilities),
            RiSchedulingPolicy::RoundRobin => self.round_robin(&scored, request.device_type).await,
            RiSchedulingPolicy::PriorityBased => self.priority_based(&scored, request.priority),
            RiSchedulingPolicy::LoadBalanced => self.load_balanced(&scored),
        }
    }

    /// Filters raw available devices into scheduling candidates.
    ///
    /// Currently this only applies basic capability checks to preserve the
    /// original behavior. In future iterations, SLA and affinity rules from
    /// the allocation request can be incorporated here.
    fn filter_candidates(
        &self,
        devices: Vec<Arc<RiDevice>>,
        request: &RiAllocationRequest,
    ) -> Vec<Arc<RiDevice>> {
        devices
            .into_iter()
            .filter(|device| device.capabilities().meets_requirements(&request.capabilities))
            .filter(|device| {
                // Apply hard affinity / anti-affinity rules when present
                if let Some(rules) = &request.affinity {
                    // required_labels: all must match
                    for (key, val) in &rules.required_labels {
                        match device.metadata().get(key) {
                            Some(v) if v == val => {}
                            _ => return false,
                        }
                    }
                }

                if let Some(rules) = &request.anti_affinity {
                    // forbidden_labels: none may match
                    for (key, val) in &rules.forbidden_labels {
                        if let Some(v) = device.metadata().get(key) {
                            if v == val {
                                return false;
                            }
                        }
                    }
                }

                true
            })
            .collect()
    }

    /// Scores candidates for advanced scheduling.
    ///
    /// This function computes a composite score per device based on:
    /// - resource fitness (how well the device matches requested capabilities)
    /// - remaining capacity
    /// - device health
    /// - optional multi-dimensional resource weights
    /// - optional SLA class
    ///
    /// The list is then sorted in descending order of score so that subsequent
    /// policy functions (FirstFit/BestFit/etc.) operate on a preference-ordered
    /// candidate set.
    fn score_candidates(
        &self,
        candidates: &[Arc<RiDevice>],
        request: &RiAllocationRequest,
    ) -> Vec<Arc<RiDevice>> {
        if candidates.is_empty() {
            return Vec::new();
        }

        // Derive SLA multiplier
        let sla_multiplier: f64 = match request.sla_class {
            Some(super::RiRequestSlaClass::Critical) => 1.5,
            Some(super::RiRequestSlaClass::High) => 1.2,
            Some(super::RiRequestSlaClass::Medium) => 1.0,
            Some(super::RiRequestSlaClass::Low) => 0.8,
            None => 1.0,
        };

        // Resource dimension weights (fallback to 1.0 if not provided)
        let (compute_weight, memory_weight, storage_weight, bandwidth_weight) =
            match &request.resource_weights {
                Some(w) => (w.compute_weight, w.memory_weight, w.storage_weight, w.bandwidth_weight),
                None => (1.0, 1.0, 1.0, 1.0),
            };

        let mut scored: Vec<(Arc<RiDevice>, f64)> = candidates
            .iter()
            .cloned()
            .map(|device| {
                let caps = device.capabilities();

                // Base fitness: lower is better; convert to [0,1] where 1 is best.
                let fitness = self.calculate_fitness_score(caps, &request.capabilities);
                let fitness_score = 1.0 / (1.0 + fitness.max(0.0));

                // Remaining capacity score: already higher-is-better.
                let remaining = self.calculate_remaining_capacity_score(caps);

                // Health score normalized to [0,1].
                let health = device.health_score() as f64 / 100.0;

                // Dimension-specific ratios for weighting.
                let mut dim_score = 0.0;

                if let (Some(req), Some(avail)) = (request.capabilities.compute_units, caps.compute_units) {
                    if avail > 0 {
                        let ratio = req as f64 / avail as f64;
                        dim_score += compute_weight * (1.0 / (1.0 + ratio.max(0.0)));
                    }
                }

                if let (Some(req), Some(avail)) = (request.capabilities.memory_gb, caps.memory_gb) {
                    if avail > 0.0 {
                        let ratio = req / avail;
                        dim_score += memory_weight * (1.0 / (1.0 + ratio.max(0.0)));
                    }
                }

                if let (Some(req), Some(avail)) = (request.capabilities.storage_gb, caps.storage_gb) {
                    if avail > 0.0 {
                        let ratio = req / avail;
                        dim_score += storage_weight * (1.0 / (1.0 + ratio.max(0.0)));
                    }
                }

                if let (Some(req), Some(avail)) = (request.capabilities.bandwidth_gbps, caps.bandwidth_gbps) {
                    if avail > 0.0 {
                        let ratio = req / avail;
                        dim_score += bandwidth_weight * (1.0 / (1.0 + ratio.max(0.0)));
                    }
                }

                // Normalize remaining capacity to a softer influence.
                let remaining_score = (remaining / (1.0 + remaining.abs())).max(0.0);

                // Affinity preference bonus: reward preferred label matches when defined
                let mut affinity_bonus = 0.0;
                if let Some(rules) = &request.affinity {
                    for (key, val) in &rules.preferred_labels {
                        if let Some(v) = device.metadata().get(key) {
                            if v == val {
                                affinity_bonus += 0.05;
                            }
                        }
                    }
                }

                // Composite score
                let score = sla_multiplier
                    * (
                        0.4 * fitness_score   // how tightly it matches requirements
                        + 0.3 * dim_score     // per-dimension weighted match
                        + 0.2 * remaining_score
                        + 0.1 * health
                        + affinity_bonus
                    );

                (device, score)
            })
            .collect();

        // Sort descending by score (best first)
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        scored.into_iter().map(|(device, _)| device).collect()
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
    fn first_fit(&self, devices: &[Arc<RiDevice>]) -> Option<Arc<RiDevice>> {
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
    fn best_fit(&self, devices: &[Arc<RiDevice>], requirements: &RiDeviceCapabilities) -> Option<Arc<RiDevice>> {
        devices.iter()
            .filter(|device| device.capabilities().meets_requirements(requirements))
            .min_by(|a, b| {
                // Calculate fitness score based on resource usage ratios
                let a_fitness = self.calculate_fitness_score(a.capabilities(), requirements);
                let b_fitness = self.calculate_fitness_score(b.capabilities(), requirements);
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
    fn worst_fit(&self, devices: &[Arc<RiDevice>], requirements: &RiDeviceCapabilities) -> Option<Arc<RiDevice>> {
        devices.iter()
            .filter(|device| device.capabilities().meets_requirements(requirements))
            .max_by(|a, b| {
                // Calculate remaining capacity score
                let a_score = self.calculate_remaining_capacity_score(a.capabilities());
                let b_score = self.calculate_remaining_capacity_score(b.capabilities());
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
    async fn round_robin(&self, devices: &[Arc<RiDevice>], device_type: RiDeviceType) -> Option<Arc<RiDevice>> {
        let mut counters = self.round_robin_counters.write().await;
        let counter = counters.entry(device_type)
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
    fn priority_based(&self, devices: &[Arc<RiDevice>], priority: u32) -> Option<Arc<RiDevice>> {
        // For higher priority requests, select devices with higher health scores
        devices.iter()
            .max_by(|a, b| {
                let a_score = a.health_score() as u32 * priority;
                let b_score = b.health_score() as u32 * priority;
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
    fn load_balanced(&self, devices: &[Arc<RiDevice>]) -> Option<Arc<RiDevice>> {
        devices.iter()
            .min_by(|a, b| {
                // Use health score as a proxy for load (inverse relationship)
                a.health_score().cmp(&b.health_score()).reverse()
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
    fn calculate_fitness_score(&self, device_cap: &RiDeviceCapabilities, requirements: &RiDeviceCapabilities) -> f64 {
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
    fn calculate_remaining_capacity_score(&self, device_cap: &RiDeviceCapabilities) -> f64 {
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
    pub async fn allocate(&self, request: &RiAllocationRequest) -> Option<String> {
        if let Some(device) = self.select_device(request).await {
            // Generate unique allocation ID
            let allocation_id = uuid::Uuid::new_v4().to_string();
            
            // Note: In a real implementation, we'd need to lock the device and update its status
            // This is simplified for demonstration
            
            // Record the allocation
            self.record_allocation(allocation_id.clone(), device.id().to_string(), device.device_type(), request.capabilities.clone()).await;
            
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
    pub async fn record_allocation(&self, allocation_id: String, device_id: String, device_type: RiDeviceType, capabilities_required: RiDeviceCapabilities) {
        let record = RiAllocationRecord {
            allocation_id,
            device_id,
            device_type,
            allocated_at: Utc::now(),
            released_at: None,
            duration_seconds: None,
            success: true,
            capabilities_required,
        };
        
        let mut history = self.allocation_history.write().await;
        history.push(record);
        
        if history.len() > 1000 {
            history.remove(0);
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
    pub async fn record_release(&self, allocation_id: &str) {
        let mut history = self.allocation_history.write().await;
        if let Some(record) = history.iter_mut().find(|r| r.allocation_id == allocation_id) {
            record.released_at = Some(Utc::now());
            
            if let Some(released_at) = record.released_at {
                if let Ok(duration) = released_at.signed_duration_since(record.allocated_at).to_std() {
                    record.duration_seconds = Some(duration.as_secs_f64());
                }
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
    /// A `RiAllocationStatistics` struct containing comprehensive allocation statistics.
    pub async fn get_statistics(&self) -> RiAllocationStatistics {
        let history = self.allocation_history.read().await;
        let total_allocations = history.len();
        let successful_allocations = history.iter().filter(|r| r.success).count();
        let failed_allocations = total_allocations - successful_allocations;
        
        let completed_allocations: Vec<&RiAllocationRecord> = history.iter()
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
        
        let mut by_device_type = HashMap::new();
        for device_type in [RiDeviceType::CPU, RiDeviceType::GPU, RiDeviceType::Memory, 
            RiDeviceType::Storage, RiDeviceType::Network, RiDeviceType::Sensor, 
            RiDeviceType::Actuator, RiDeviceType::Custom] {
            let type_allocations = history.iter()
                .filter(|r| r.device_type == device_type)
                .count();
            
            if type_allocations > 0 {
                let type_completed = history.iter()
                    .filter(|r| r.device_type == device_type && r.released_at.is_some())
                    .count();
                
                let type_duration: f64 = history.iter()
                    .filter(|r| r.device_type == device_type)
                    .filter_map(|r| r.duration_seconds)
                    .sum();
                
                let type_avg_duration = if type_completed > 0 {
                    type_duration / type_completed as f64
                } else {
                    0.0
                };
                
                by_device_type.insert(device_type, RiDeviceTypeStatistics {
                    total_allocations: type_allocations,
                    completed_allocations: type_completed,
                    average_duration_seconds: type_avg_duration,
                });
            }
        }
        
        RiAllocationStatistics {
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
    pub async fn get_recommendations(&self, device_type: &RiDeviceType) -> Vec<RiSchedulingRecommendation> {
        let mut recommendations = Vec::new();
        
        let history = self.allocation_history.read().await;
        let recent_allocations: Vec<&RiAllocationRecord> = history.iter()
            .filter(|r| r.device_type == *device_type)
            .rev()
            .take(100)
            .collect();
        
        if recent_allocations.is_empty() {
            recommendations.push(RiSchedulingRecommendation {
                recommendation_type: RiSchedulingRecommendationType::UseDefaultPolicy,
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
        
        if success_rate < 0.8 {
            recommendations.push(RiSchedulingRecommendation {
                recommendation_type: RiSchedulingRecommendationType::ConsiderPolicyChange,
                description: format!("Low success rate ({:.1}%) for {:?}, consider changing scheduling policy", 
                    success_rate * 100.0, device_type),
                priority: 3,
                confidence: 0.8,
            });
        }
        
        if avg_duration > 300.0 {
            recommendations.push(RiSchedulingRecommendation {
                recommendation_type: RiSchedulingRecommendationType::OptimizeForLongRunning,
                description: format!("Average allocation duration is {avg_duration:.1} seconds for {device_type:?}, consider load balancing"),
                priority: 2,
                confidence: 0.7,
            });
        }
        
        if recent_allocations.len() > 50 && avg_duration < 60.0 {
            recommendations.push(RiSchedulingRecommendation {
                recommendation_type: RiSchedulingRecommendationType::OptimizeForShortRunning,
                description: format!("High frequency of short allocations for {device_type:?}, consider round-robin scheduling"),
                priority: 2,
                confidence: 0.6,
            });
        }
        
        recommendations.push(RiSchedulingRecommendation {
            recommendation_type: RiSchedulingRecommendationType::ContinueCurrentPolicy,
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiAllocationStatistics {
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
    pub by_device_type: HashMap<RiDeviceType, RiDeviceTypeStatistics>,
}

/// Device type statistics - metrics for a specific device type
///
/// This struct contains allocation statistics for a specific device type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiDeviceTypeStatistics {
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiSchedulingRecommendationType {
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiSchedulingRecommendation {
    /// Type of recommendation
    pub recommendation_type: RiSchedulingRecommendationType,
    /// Human-readable description of the recommendation
    pub description: String,
    /// Priority of the recommendation (1-10, higher = more important)
    pub priority: u8,
    /// Confidence in the recommendation (0.0-1.0)
    pub confidence: f64,
}
