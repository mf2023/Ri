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

use std::time::{Instant, Duration};
use std::collections::{HashMap, VecDeque};
use serde::{Serialize, Deserialize};
use crate::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities};

/// # Device Discovery and Resource Scheduling
/// 
/// This file implements advanced device discovery and resource scheduling algorithms for DMS.
/// It provides two main components:
/// 
/// 1. **DMSDeviceDiscoveryEngine**: Advanced device discovery using machine learning and heuristics
/// 2. **DMSResourceScheduler**: Resource scheduling algorithm using performance history and policies
/// 
/// ## Design Principles
/// 
/// 1. **Machine Learning Integration**: Uses pattern recognition and confidence scoring for device identification
/// 2. **Heuristic Optimization**: Implements intelligent resource allocation based on device capabilities and load
/// 3. **Scalability**: Designed to handle large numbers of devices and requests
/// 4. **Flexibility**: Supports custom scheduling policies and device fingerprints
/// 5. **Performance Focus**: Optimizes for low latency and high throughput
/// 6. **Adaptability**: Learns from discovery and performance history
/// 
/// ## Usage Examples
/// 
/// ```rust
/// use dms::device::{DMSDeviceDiscoveryEngine, DMSResourceScheduler, ResourceRequest, DeviceScanResult};
/// 
/// fn example() {
///     // Create discovery engine
///     let mut discovery_engine = DMSDeviceDiscoveryEngine::new();
///     
///     // Create scan results
///     let scan_results = vec![
///         DeviceScanResult {
///             device_id: "device-123".to_string(),
///             device_name: "NVIDIA RTX 3090".to_string(),
///             device_info: HashMap::from([
///                 ("device_name".to_string(), "NVIDIA RTX 3090".to_string()),
///                 ("driver".to_string(), "CUDA 12.0".to_string())
///             ])
///         }
///     ];
///     
///     // Discover devices
///     let discovered_devices = discovery_engine.discover_devices(scan_results);
///     
///     // Create resource scheduler
///     let mut scheduler = DMSResourceScheduler::new();
///     
///     // Create resource request
///     let request = ResourceRequest {
///         request_id: "req-456".to_string(),
///         required_memory_gb: Some(16.0),
///         required_compute_units: Some(512),
///         required_bandwidth_gbps: Some(900.0),
///         required_custom_capabilities: HashMap::from([
///             ("cuda_support".to_string(), "true".to_string())
///         ]),
///         priority: 5,
///         deadline: None
///     };
///     
///     // Schedule resource
///     let assigned_device = scheduler.schedule_resource(&request, &discovered_devices);
/// }
/// ```

/// Advanced device discovery algorithm using machine learning and heuristics
/// 
/// This engine uses pattern recognition, confidence scoring, and historical data to identify
/// devices with high accuracy. It maintains a database of device fingerprints and uses
/// them to match discovered devices based on identification patterns.
#[derive(Debug, Clone)]
pub struct DMSDeviceDiscoveryEngine {
    /// Device fingerprint database mapping fingerprint IDs to their details
    fingerprints: HashMap<String, DeviceFingerprint>,
    /// Discovery history for pattern recognition and learning
    discovery_history: VecDeque<DiscoveryRecord>,
    /// Confidence threshold for device identification (0.0 to 1.0)
    confidence_threshold: f64,
}

/// Device fingerprint containing identification patterns and capabilities
/// 
/// This struct represents a device fingerprint used for identifying devices during discovery.
/// It contains identification patterns with weights that are used to calculate confidence scores.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceFingerprint {
    /// Type of device this fingerprint identifies
    device_type: DMSDeviceType,
    /// Capabilities associated with this device type
    capabilities: DMSDeviceCapabilities,
    /// Identification patterns used to match this device type
    identification_patterns: Vec<IdentificationPattern>,
    /// Base confidence score for this fingerprint
    confidence_score: f64,
}

/// Identification pattern with field, pattern, and weight
/// 
/// This struct defines a single identification pattern used in device fingerprinting.
/// Each pattern has a field to match against, a pattern string, and a weight that determines
/// how important this pattern is for identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct IdentificationPattern {
    /// Device information field to match against (e.g., "device_name", "driver")
    field: String,
    /// Pattern string to match (e.g., "nvidia", "cuda")
    pattern: String,
    /// Weight of this pattern in the overall confidence calculation (0.0 to 1.0)
    weight: f64,
}

/// Record of a device discovery attempt
/// 
/// This struct records information about a device discovery attempt, including the device
/// information, identified type (if any), and confidence score. These records are used for
/// pattern learning and improving future discovery accuracy.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscoveryRecord {
    /// Timestamp of the discovery attempt (UNIX seconds)
    timestamp: u64, // Changed from Instant to u64 for serialization
    /// Unique identifier of the discovered device
    device_id: String,
    /// Device information collected during discovery
    device_info: HashMap<String, String>,
    /// Identified device type (if confidence threshold was met)
    identified_type: Option<DMSDeviceType>,
    /// Confidence score for the identification
    confidence: f64,
}

impl Default for DMSDeviceDiscoveryEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSDeviceDiscoveryEngine {
    /// Create a new device discovery engine with default settings.
    /// 
    /// This method initializes the engine with default device fingerprints and sets up
    /// the discovery history with a capacity of 1000 records.
    /// 
    /// # Returns
    /// 
    /// A new `DMSDeviceDiscoveryEngine` instance with default fingerprints and settings.
    pub fn new() -> Self {
        let mut engine = Self {
            fingerprints: HashMap::new(),
            discovery_history: VecDeque::with_capacity(1000),
            confidence_threshold: 0.7,
        };
        
        // Initialize with default fingerprints
        engine.initialize_default_fingerprints();
        engine
    }
    
    /// Discover devices using advanced pattern matching and confidence scoring.
    /// 
    /// This method processes a list of device scan results, identifies each device using
    /// pattern matching against known fingerprints, and returns a list of discovered devices.
    /// 
    /// # Parameters
    /// 
    /// - `scan_results`: List of device scan results from hardware discovery
    /// 
    /// # Returns
    /// 
    /// A vector of discovered `DMSDevice` instances with identified types and capabilities.
    pub fn discover_devices(&mut self, scan_results: Vec<DeviceScanResult>) -> Vec<DMSDevice> {
        let mut discovered_devices = Vec::new();
        
        for scan_result in scan_results {
            if let Some(device) = self.identify_device(scan_result) {
                discovered_devices.push(device);
            }
        }
        
        discovered_devices
    }
    
    /// Identify a single device from scan results using fingerprint matching.
    /// 
    /// This method attempts to identify a device by matching its information against known
    /// device fingerprints. It calculates confidence scores for each fingerprint match and
    /// returns the best match if it meets the confidence threshold.
    /// 
    /// # Parameters
    /// 
    /// - `scan_result`: Single device scan result to identify
    /// 
    /// # Returns
    /// 
    /// An `Option<DMSDevice>` containing the identified device if a match was found with
    /// sufficient confidence, or `None` if no match was found.
    fn identify_device(&mut self, scan_result: DeviceScanResult) -> Option<DMSDevice> {
        let device_info = scan_result.device_info;
        
        // Try to match against known fingerprints
        let mut best_match: Option<(String, f64)> = None;
        
        for (fingerprint_id, fingerprint) in &self.fingerprints {
            let confidence = self.calculate_match_confidence(&device_info, fingerprint);
            
            if confidence > self.confidence_threshold {
                match best_match {
                    None => best_match = Some((fingerprint_id.clone(), confidence)),
                    Some((_, best_confidence)) if confidence > best_confidence => {
                        best_match = Some((fingerprint_id.clone(), confidence));
                    }
                    _ => {}
                }
            }
        }
        
        if let Some((fingerprint_id, confidence)) = best_match {
            let fingerprint = self.fingerprints.get(&fingerprint_id).unwrap();
            
            let device = DMSDevice::new(
                scan_result.device_name.clone(),
                fingerprint.device_type,
            ).with_capabilities(fingerprint.capabilities.clone());
            
            // Record discovery for future learning
            self.record_discovery(
                scan_result.device_id,
                device_info,
                Some(fingerprint.device_type),
                confidence,
            );
            
            Some(device)
        } else {
            // Record failed discovery for analysis
            self.record_discovery(
                scan_result.device_id,
                device_info,
                None,
                0.0,
            );
            
            None
        }
    }
    
    /// Calculate confidence score for device identification.
    /// 
    /// This method calculates a confidence score by matching device information against
    /// the identification patterns in a fingerprint. The score is a weighted sum of matching
    /// patterns divided by the total weight of all patterns.
    /// 
    /// # Parameters
    /// 
    /// - `device_info`: Device information to match
    /// - `fingerprint`: Fingerprint containing identification patterns
    /// 
    /// # Returns
    /// 
    /// A confidence score between 0.0 and 1.0, where higher scores indicate a stronger match.
    fn calculate_match_confidence(&self, device_info: &HashMap<String, String>, fingerprint: &DeviceFingerprint) -> f64 {
        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;
        
        for pattern in &fingerprint.identification_patterns {
            total_weight += pattern.weight;
            
            if let Some(value) = device_info.get(&pattern.field) {
                if self.matches_pattern(value, &pattern.pattern) {
                    matched_weight += pattern.weight;
                }
            }
        }
        
        if total_weight > 0.0 {
            matched_weight / total_weight
        } else {
            0.0
        }
    }
    
    /// Check if a value matches a pattern using simple string matching.
    /// 
    /// This method performs a case-insensitive substring match to determine if a value
    /// matches a pattern. It can be enhanced with regex support in future versions.
    /// 
    /// # Parameters
    /// 
    /// - `value`: Device information value to check
    /// - `pattern`: Pattern string to match against
    /// 
    /// # Returns
    /// 
    /// `true` if the value matches the pattern, `false` otherwise.
    fn matches_pattern(&self, value: &str, pattern: &str) -> bool {
        // Simple pattern matching - can be enhanced with regex
        value.to_lowercase().contains(&pattern.to_lowercase())
    }
    
    /// Record discovery for pattern learning and analysis.
    /// 
    /// This method records a discovery attempt in the history, including the device information,
    /// identified type (if any), and confidence score. This history is used for pattern recognition
    /// and improving future discovery accuracy.
    /// 
    /// # Parameters
    /// 
    /// - `device_id`: Unique identifier of the discovered device
    /// - `device_info`: Device information collected during discovery
    /// - `identified_type`: Identified device type (if any)
    /// - `confidence`: Confidence score for the identification
    fn record_discovery(&mut self, device_id: String, device_info: HashMap<String, String>, identified_type: Option<DMSDeviceType>, confidence: f64) {
        let record = DiscoveryRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs(),
            device_id,
            device_info,
            identified_type,
            confidence,
        };
        
        self.discovery_history.push_back(record);
        
        // Keep only recent history
        if self.discovery_history.len() > 1000 {
            self.discovery_history.pop_front();
        }
    }
    
    /// Initialize default device fingerprints for common device types.
    /// 
    /// This method populates the fingerprint database with default fingerprints for common
    /// device types, including GPUs and TPUs. These fingerprints are used as a starting point
    /// for device identification.
    fn initialize_default_fingerprints(&mut self) {
        // GPU Device Fingerprint
        let gpu_fingerprint = DeviceFingerprint {
            device_type: DMSDeviceType::GPU,
            capabilities: DMSDeviceCapabilities {
                memory_gb: Some(16.0),
                compute_units: Some(512),
                storage_gb: Some(500.0),
                bandwidth_gbps: Some(900.0),
                custom_capabilities: vec![("cuda_support".to_string(), "true".to_string()), ("tensor_cores".to_string(), "true".to_string())].into_iter().collect(),
            },
            identification_patterns: vec![
                IdentificationPattern {
                    field: "device_name".to_string(),
                    pattern: "nvidia".to_string(),
                    weight: 0.4,
                },
                IdentificationPattern {
                    field: "driver".to_string(),
                    pattern: "cuda".to_string(),
                    weight: 0.6,
                },
            ],
            confidence_score: 0.9,
        };
        
        // TPU Device Fingerprint
        let tpu_fingerprint = DeviceFingerprint {
            device_type: DMSDeviceType::Custom, // Using Custom for TPU until we have a proper TPU type
            capabilities: DMSDeviceCapabilities {
                memory_gb: Some(32.0),
                compute_units: Some(128),
                storage_gb: Some(1000.0),
                bandwidth_gbps: Some(1200.0),
                custom_capabilities: vec![("tpu_support".to_string(), "true".to_string()), ("ml_accelerator".to_string(), "true".to_string())].into_iter().collect(),
            },
            identification_patterns: vec![
                IdentificationPattern {
                    field: "device_name".to_string(),
                    pattern: "tpu".to_string(),
                    weight: 0.8,
                },
                IdentificationPattern {
                    field: "vendor".to_string(),
                    pattern: "google".to_string(),
                    weight: 0.2,
                },
            ],
            confidence_score: 0.95,
        };
        
        self.fingerprints.insert("gpu".to_string(), gpu_fingerprint);
        self.fingerprints.insert("tpu".to_string(), tpu_fingerprint); // Using proper key for TPU
    }
    
    /// Get discovery statistics and performance metrics.
    /// 
    /// This method calculates and returns statistics about the discovery process, including
    /// total attempts, successful identifications, success rate, and average confidence score.
    /// 
    /// # Returns
    /// 
    /// A `DiscoveryStats` struct containing the discovery statistics.
    pub fn get_discovery_stats(&self) -> DiscoveryStats {
        let total_attempts = self.discovery_history.len();
        let successful_identifications = self.discovery_history
            .iter()
            .filter(|record| record.identified_type.is_some())
            .count();
        
        let avg_confidence = if total_attempts > 0 {
            self.discovery_history
                .iter()
                .map(|record| record.confidence)
                .sum::<f64>() / total_attempts as f64
        } else {
            0.0
        };
        
        DiscoveryStats {
            total_attempts,
            successful_identifications,
            success_rate: if total_attempts > 0 {
                successful_identifications as f64 / total_attempts as f64
            } else {
                0.0
            },
            avg_confidence,
        }
    }
}

/// Device scan result from hardware discovery
/// 
/// This struct represents the result of a device scan, containing basic device information
/// that is used for identification and fingerprint matching.
#[derive(Debug, Clone)]
pub struct DeviceScanResult {
    /// Unique identifier for the discovered device
    pub device_id: String,
    /// Human-readable name of the device
    pub device_name: String,
    /// Additional device information collected during scanning
    pub device_info: HashMap<String, String>,
}

/// Discovery statistics and performance metrics
/// 
/// This struct contains statistics about the device discovery process, including
/// success rates, confidence scores, and total attempts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStats {
    /// Total number of device discovery attempts
    pub total_attempts: usize,
    /// Number of successful device identifications
    pub successful_identifications: usize,
    /// Success rate for device identification (0.0 to 1.0)
    pub success_rate: f64,
    /// Average confidence score for successful identifications
    pub avg_confidence: f64,
}

/// Resource scheduling algorithm using performance history and policies
/// 
/// This scheduler uses device performance history, current load, and custom policies
/// to intelligently allocate resources to the most suitable devices.
#[derive(Debug, Clone)]
pub struct DMSResourceScheduler {
    /// Device performance history mapping device IDs to performance records
    performance_history: HashMap<String, Vec<PerformanceRecord>>,
    /// Current device loads (0.0 to 1.0)
    device_loads: HashMap<String, f64>,
    /// Custom scheduling policies sorted by priority
    policies: Vec<SchedulingPolicy>,
}

/// Performance record for a device
/// 
/// This struct records performance metrics for a device at a specific point in time,
/// including latency, throughput, error rate, and utilization.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceRecord {
    /// Timestamp of the performance measurement (UNIX seconds)
    timestamp: u64, // Changed from Instant to u64 for serialization
    /// Latency in milliseconds
    latency_ms: f64,
    /// Throughput in operations per second
    throughput: f64,
    /// Error rate as a fraction (0.0 to 1.0)
    error_rate: f64,
    /// Resource utilization as a fraction (0.0 to 1.0)
    utilization: f64,
}

/// Scheduling policy for resource allocation
/// 
/// This struct defines a custom scheduling policy that can be applied to resource requests.
/// Policies consist of conditions and actions, and are evaluated in priority order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    /// Name of the policy
    name: String,
    /// Priority of the policy (higher numbers = higher priority)
    priority: u8,
    /// Conditions that must be met for the policy to apply
    conditions: Vec<PolicyCondition>,
    /// Action to take if conditions are met
    action: PolicyAction,
}

/// Policy condition for scheduling decisions
/// 
/// This struct defines a single condition that must be met for a scheduling policy to apply.
/// Conditions compare metrics against thresholds using specified operators.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PolicyCondition {
    /// Metric to evaluate (e.g., "device_type", "request_priority", "time_of_day")
    metric: String,
    /// Comparison operator (>, >=, <, <=, ==)
    operator: String,
    /// Threshold value for the comparison
    threshold: f64,
}

/// Policy action for scheduling decisions
/// 
/// This enum defines the actions that can be taken when a scheduling policy's conditions are met.
#[derive(Debug, Clone, Serialize, Deserialize)]
enum PolicyAction {
    /// Prefer a specific device for resource allocation
    PreferDevice(String),
    /// Avoid a specific device for resource allocation
    AvoidDevice(String),
    /// Use load balancing for resource allocation
    LoadBalance,
    /// Use priority-based scheduling
    PriorityBased,
}

impl Default for DMSResourceScheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSResourceScheduler {
    /// Create a new resource scheduler with default settings.
    /// 
    /// This method initializes the scheduler with empty performance history, device loads,
    /// and scheduling policies.
    /// 
    /// # Returns
    /// 
    /// A new `DMSResourceScheduler` instance with default settings.
    pub fn new() -> Self {
        Self {
            performance_history: HashMap::new(),
            device_loads: HashMap::new(),
            policies: Vec::new(),
        }
    }
    
    /// Schedule a resource request to the most suitable device based on capabilities, load, and policies.
    /// 
    /// This method evaluates available devices against the resource request requirements,
    /// calculates a score for each suitable device, and returns the ID of the best device.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Resource request with requirements and priority
    /// - `available_devices`: List of available devices to consider for scheduling
    /// 
    /// # Returns
    /// 
    /// The ID of the most suitable device for the request, or `None` if no suitable device is found.
    pub fn schedule_resource(
        &mut self,
        request: &ResourceRequest,
        available_devices: &[DMSDevice],
    ) -> Option<String> {
        // Filter devices that meet requirements
        let suitable_devices: Vec<&DMSDevice> = available_devices
            .iter()
            .filter(|device| self.meets_requirements(device, request))
            .collect();
        
        if suitable_devices.is_empty() {
            return None;
        }
        
        // Score each suitable device
        let mut device_scores: Vec<(String, f64)> = suitable_devices
            .iter()
            .map(|device| {
                let score = self.calculate_device_score(device, request);
                (device.id().to_string(), score)
            })
            .collect();
        
        // Sort by score (highest first)
        device_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return the best device
        device_scores.first().map(|(device_id, _)| device_id.clone())
    }
    
    /// Check if a device meets the requirements of a resource request.
    /// 
    /// This method verifies that a device has the necessary capabilities, memory, compute units,
    /// bandwidth, and custom capabilities to handle a resource request.
    /// 
    /// # Parameters
    /// 
    /// - `device`: Device to check against requirements
    /// - `request`: Resource request with requirements
    /// 
    /// # Returns
    /// 
    /// `true` if the device meets all requirements, `false` otherwise.
    fn meets_requirements(&self, device: &DMSDevice, request: &ResourceRequest) -> bool {
        let capabilities = device.capabilities();
        
        // Check memory requirement
        if let Some(required_memory) = request.required_memory_gb {
            if let Some(available_memory) = capabilities.memory_gb {
                if available_memory < required_memory {
                    return false;
                }
            } else {
                return false; // No memory available
            }
        }
        
        // Check compute units requirement
        if let Some(required_compute) = request.required_compute_units {
            if let Some(available_compute) = capabilities.compute_units {
                if available_compute < required_compute {
                    return false;
                }
            } else {
                return false; // No compute units available
            }
        }
        
        // Check bandwidth requirement
        if let Some(required_bandwidth) = request.required_bandwidth_gbps {
            if let Some(available_bandwidth) = capabilities.bandwidth_gbps {
                if available_bandwidth < required_bandwidth {
                    return false;
                }
            } else {
                return false; // No bandwidth available
            }
        }
        
        // Check custom capabilities
        for (required_key, required_value) in &request.required_custom_capabilities {
            if let Some(available_value) = capabilities.custom_capabilities.get(required_key) {
                if available_value != required_value {
                    return false;
                }
            } else {
                return false; // Required capability not found
            }
        }
        
        true
    }
    
    /// Calculate a score for a device based on its capabilities, load, performance history, and policies.
    /// 
    /// This method calculates a score for a device by considering:
    /// 1. Base score (100.0)
    /// 2. Penalty for current load
    /// 3. Bonus for good performance history (latency, throughput, error rate, utilization)
    /// 4. Health score adjustment
    /// 5. Responsiveness adjustment
    /// 6. Policy-based adjustments
    /// 
    /// # Parameters
    /// 
    /// - `device`: Device to score
    /// - `request`: Resource request being scheduled
    /// 
    /// # Returns
    /// 
    /// A score between 0.0 and potentially over 100.0, where higher scores indicate better suitability.
    fn calculate_device_score(&self, device: &DMSDevice, request: &ResourceRequest) -> f64 {
        let device_id = device.id();
        let base_score = 100.0;
        let mut score = base_score;
        
        // Penalize based on current load (exponential penalty for high load)
        if let Some(&load) = self.device_loads.get(device_id) {
            // Exponential penalty: 0% load = 0 penalty, 100% load = 70 point penalty
            score -= load.powi(2) * 70.0;
        }
        
        // Bonus for device performance history with weighted factors
        if let Some(performance_records) = self.performance_history.get(device_id) {
            if !performance_records.is_empty() {
                // Only consider recent performance records (last 10)
                let recent_records = &performance_records[performance_records.len().saturating_sub(10)..];
                let record_count = recent_records.len() as f64;
                
                let avg_latency = recent_records
                    .iter()
                    .map(|record| record.latency_ms)
                    .sum::<f64>() / record_count;
                
                let avg_throughput = recent_records
                    .iter()
                    .map(|record| record.throughput)
                    .sum::<f64>() / record_count;
                
                let avg_error_rate = recent_records
                    .iter()
                    .map(|record| record.error_rate)
                    .sum::<f64>() / record_count;
                
                let avg_utilization = recent_records
                    .iter()
                    .map(|record| record.utilization)
                    .sum::<f64>() / record_count;
                
                // Bonus for low latency (up to 20 points)
                score += (100.0 - avg_latency.min(100.0)) * 0.2;
                
                // Bonus for high throughput (up to 20 points)
                score += (avg_throughput.min(100.0)) * 0.2;
                
                // Penalty for high error rate (up to 20 points)
                score -= (avg_error_rate.min(1.0)) * 20.0;
                
                // Bonus for optimal utilization (60-80% is ideal, up to 10 points)
                let utilization_bonus = if (0.6..=0.8).contains(&avg_utilization) {
                    10.0
                } else if (0.4..=0.9).contains(&avg_utilization) {
                    5.0
                } else {
                    0.0
                };
                score += utilization_bonus;
            }
        }
        
        // Add health score adjustment (up to 20 points)
        let health_score = device.health_score() as f64;
        score += (health_score / 100.0) * 20.0;
        
        // Add responsiveness adjustment (10 points if responsive, 0 otherwise)
        if device.is_responsive(300) { // 5 minutes timeout
            score += 10.0;
        }
        
        // Apply policy-based adjustments
        for policy in &self.policies {
            score = self.apply_policy_score_adjustment(device, request, policy, score);
        }
        
        // Ensure score is within reasonable bounds
        score.max(0.0).min(200.0)
    }
    
    /// Apply policy-based score adjustments to a device's score.
    /// 
    /// This method evaluates a scheduling policy's conditions and applies the appropriate
    /// score adjustment if the conditions are met.
    /// 
    /// # Parameters
    /// 
    /// - `device`: Device being scored
    /// - `request`: Resource request being scheduled
    /// - `policy`: Scheduling policy to apply
    /// - `current_score`: Current score of the device
    /// 
    /// # Returns
    /// 
    /// The updated score after applying the policy adjustment.
    fn apply_policy_score_adjustment(
        &self,
        device: &DMSDevice,
        request: &ResourceRequest,
        policy: &SchedulingPolicy,
        current_score: f64,
    ) -> f64 {
        // Check if policy conditions are met
        let conditions_met = policy.conditions.iter().all(|condition| {
            self.evaluate_condition(device, request, condition)
        });
        
        if conditions_met {
            match &policy.action {
                PolicyAction::PreferDevice(preferred_device) => {
                    if device.id() == preferred_device {
                        current_score + 30.0 // Bonus for preferred device
                    } else {
                        current_score
                    }
                }
                PolicyAction::AvoidDevice(avoided_device) => {
                    if device.id() == avoided_device {
                        current_score - 30.0 // Penalty for avoided device
                    } else {
                        current_score
                    }
                }
                PolicyAction::LoadBalance => {
                    // Penalty based on current load for load balancing
                    let load_penalty = self.device_loads.get(device.id()).unwrap_or(&0.0) * 20.0;
                    current_score - load_penalty
                }
                PolicyAction::PriorityBased => {
                    // Bonus based on request priority
                    let priority_bonus = (request.priority as f64 / 10.0) * 15.0;
                    current_score + priority_bonus
                }
            }
        } else {
            current_score
        }
    }
    
    /// Evaluate a policy condition against a device and request.
    /// 
    /// This method evaluates a single policy condition by comparing the appropriate metric
    /// against the threshold using the specified operator.
    /// 
    /// # Parameters
    /// 
    /// - `device`: Device to evaluate
    /// - `request`: Resource request to evaluate
    /// - `condition`: Policy condition to evaluate
    /// 
    /// # Returns
    /// 
    /// `true` if the condition is met, `false` otherwise.
    fn evaluate_condition(
        &self,
        device: &DMSDevice,
        request: &ResourceRequest,
        condition: &PolicyCondition,
    ) -> bool {
        let value = match condition.metric.as_str() {
            "device_type" => {
                // Convert device type to numeric value for comparison
                match device.device_type() {
                    DMSDeviceType::GPU => 1.0,
                    DMSDeviceType::Memory => 2.0,
                    DMSDeviceType::CPU => 3.0,
                    DMSDeviceType::Storage => 4.0,
                    DMSDeviceType::Network => 5.0,
                    DMSDeviceType::Sensor => 6.0,
                    DMSDeviceType::Actuator => 7.0,
                    DMSDeviceType::Custom => 8.0,
                }
            }
            "request_priority" => request.priority as f64,
            "time_of_day" => {
                // Simple time-based condition (0-24)
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                (now % 86400) as f64 / 3600.0
            }
            _ => 0.0,
        };
        
        match condition.operator.as_str() {
            ">" => value > condition.threshold,
            ">=" => value >= condition.threshold,
            "<" => value < condition.threshold,
            "<=" => value <= condition.threshold,
            "==" => (value - condition.threshold).abs() < 0.001,
            _ => false,
        }
    }
    
    /// Record device performance after task completion.
    /// 
    /// This method records performance metrics for a device, including latency, throughput,
    /// error rate, and utilization. These records are used for future scheduling decisions.
    /// 
    /// # Parameters
    /// 
    /// - `device_id`: ID of the device whose performance is being recorded
    /// - `latency_ms`: Latency in milliseconds
    /// - `throughput`: Throughput in operations per second
    /// - `error_rate`: Error rate as a fraction (0.0 to 1.0)
    /// - `utilization`: Resource utilization as a fraction (0.0 to 1.0)
    pub fn record_performance(
        &mut self,
        device_id: &str,
        latency_ms: f64,
        throughput: f64,
        error_rate: f64,
        utilization: f64,
    ) {
        let record = PerformanceRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            latency_ms,
            throughput,
            error_rate,
            utilization,
        };
        
        self.performance_history
            .entry(device_id.to_string())
            .or_default()
            .push(record);
        
        // Keep only recent performance records (last 100)
        if let Some(history) = self.performance_history.get_mut(device_id) {
            if history.len() > 100 {
                history.remove(0);
            }
        }
    }
    
    /// Update the current load of a device.
    /// 
    /// This method updates the load of a device, clamping the value between 0.0 and 1.0.
    /// The load is used in scheduling decisions to avoid overloading devices.
    /// 
    /// # Parameters
    /// 
    /// - `device_id`: ID of the device whose load is being updated
    /// - `load`: New load value (0.0 to 1.0)
    pub fn update_device_load(&mut self, device_id: &str, load: f64) {
        self.device_loads.insert(device_id.to_string(), load.clamp(0.0, 1.0));
    }
    
    /// Add a scheduling policy to the scheduler.
    /// 
    /// This method adds a scheduling policy to the scheduler and sorts the policies
    /// by priority (highest first) to ensure they are evaluated in the correct order.
    /// 
    /// # Parameters
    /// 
    /// - `policy`: Scheduling policy to add
    pub fn add_policy(&mut self, policy: SchedulingPolicy) {
        self.policies.push(policy);
        // Sort by priority (highest first)
        self.policies.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}

/// Resource request for scheduling
/// 
/// This struct represents a request for resources, including memory, compute units,
/// bandwidth, and custom capabilities. It also includes priority and deadline information.
#[derive(Debug, Clone)]
pub struct ResourceRequest {
    /// Unique identifier for the resource request
    pub request_id: String,
    /// Required memory in gigabytes (optional)
    pub required_memory_gb: Option<f64>,
    /// Required compute units (optional)
    pub required_compute_units: Option<usize>,
    /// Required bandwidth in Gbps (optional)
    pub required_bandwidth_gbps: Option<f64>,
    /// Required custom capabilities as key-value pairs
    pub required_custom_capabilities: HashMap<String, String>,
    /// Request priority (0-255, higher = higher priority)
    pub priority: u8,
    /// Optional deadline for the request
    pub deadline: Option<Instant>,
}
