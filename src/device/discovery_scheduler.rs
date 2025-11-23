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

use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use serde::{Serialize, Deserialize};
use crate::device::{DMSDevice, DMSDeviceType, DMSDeviceCapabilities};

/// Advanced device discovery algorithm using machine learning and heuristics
#[derive(Debug, Clone)]
pub struct DMSDeviceDiscoveryEngine {
    /// Device fingerprint database
    fingerprints: HashMap<String, DeviceFingerprint>,
    /// Discovery history for pattern recognition
    discovery_history: VecDeque<DiscoveryRecord>,
    /// Confidence threshold for device identification
    confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DeviceFingerprint {
    device_type: DMSDeviceType,
    capabilities: DMSDeviceCapabilities,
    identification_patterns: Vec<IdentificationPattern>,
    confidence_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct IdentificationPattern {
    field: String,
    pattern: String,
    weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DiscoveryRecord {
    timestamp: u64, // Changed from Instant to u64 for serialization
    device_id: String,
    device_info: HashMap<String, String>,
    identified_type: Option<DMSDeviceType>,
    confidence: f64,
}

impl DMSDeviceDiscoveryEngine {
    pub fn _Fnew() -> Self {
        let mut engine = Self {
            fingerprints: HashMap::new(),
            discovery_history: VecDeque::with_capacity(1000),
            confidence_threshold: 0.7,
        };
        
        // Initialize with default fingerprints
        engine._Finitialize_default_fingerprints();
        engine
    }
    
    /// Discover devices using advanced pattern matching
    pub fn _Fdiscover_devices(&mut self, scan_results: Vec<DeviceScanResult>) -> Vec<DMSDevice> {
        let mut discovered_devices = Vec::new();
        
        for scan_result in scan_results {
            if let Some(device) = self._Fidentify_device(scan_result) {
                discovered_devices.push(device);
            }
        }
        
        discovered_devices
    }
    
    /// Identify a single device from scan results
    fn _Fidentify_device(&mut self, scan_result: DeviceScanResult) -> Option<DMSDevice> {
        let device_info = scan_result.device_info;
        
        // Try to match against known fingerprints
        let mut best_match: Option<(String, f64)> = None;
        
        for (fingerprint_id, fingerprint) in &self.fingerprints {
            let confidence = self._Fcalculate_match_confidence(&device_info, fingerprint);
            
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
                fingerprint.device_type.clone(),
            ).with_capabilities(fingerprint.capabilities.clone());
            
            // Record discovery for future learning
            self._Frecord_discovery(
                scan_result.device_id,
                device_info,
                Some(fingerprint.device_type.clone()),
                confidence,
            );
            
            Some(device)
        } else {
            // Record failed discovery for analysis
            self._Frecord_discovery(
                scan_result.device_id,
                device_info,
                None,
                0.0,
            );
            
            None
        }
    }
    
    /// Calculate confidence score for device identification
    fn _Fcalculate_match_confidence(&self, device_info: &HashMap<String, String>, fingerprint: &DeviceFingerprint) -> f64 {
        let mut total_weight = 0.0;
        let mut matched_weight = 0.0;
        
        for pattern in &fingerprint.identification_patterns {
            total_weight += pattern.weight;
            
            if let Some(value) = device_info.get(&pattern.field) {
                if self._Fmatches_pattern(value, &pattern.pattern) {
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
    
    /// Check if a value matches a pattern
    fn _Fmatches_pattern(&self, value: &str, pattern: &str) -> bool {
        // Simple pattern matching - can be enhanced with regex
        value.to_lowercase().contains(&pattern.to_lowercase())
    }
    
    /// Record discovery for pattern learning
    fn _Frecord_discovery(&mut self, device_id: String, device_info: HashMap<String, String>, identified_type: Option<DMSDeviceType>, confidence: f64) {
        let record = DiscoveryRecord {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
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
    
    /// Initialize default device fingerprints
    fn _Finitialize_default_fingerprints(&mut self) {
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
            device_type: DMSDeviceType::Memory, // Using Memory as substitute for TPU
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
        self.fingerprints.insert("memory".to_string(), tpu_fingerprint); // Using memory as TPU substitute
    }
    
    /// Get discovery statistics
    pub fn _Fget_discovery_stats(&self) -> DiscoveryStats {
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
#[derive(Debug, Clone)]
pub struct DeviceScanResult {
    pub device_id: String,
    pub device_name: String,
    pub device_info: HashMap<String, String>,
}

/// Discovery statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveryStats {
    pub total_attempts: usize,
    pub successful_identifications: usize,
    pub success_rate: f64,
    pub avg_confidence: f64,
}

/// Resource scheduling algorithm using machine learning and optimization
#[derive(Debug, Clone)]
pub struct DMSResourceScheduler {
    /// Device performance history
    performance_history: HashMap<String, Vec<PerformanceRecord>>,
    /// Current device loads
    device_loads: HashMap<String, f64>,
    /// Scheduling policies
    policies: Vec<SchedulingPolicy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PerformanceRecord {
    timestamp: u64, // Changed from Instant to u64 for serialization
    latency_ms: f64,
    throughput: f64,
    error_rate: f64,
    utilization: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingPolicy {
    name: String,
    priority: u8,
    conditions: Vec<PolicyCondition>,
    action: PolicyAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PolicyCondition {
    metric: String,
    operator: String,
    threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum PolicyAction {
    PreferDevice(String),
    AvoidDevice(String),
    LoadBalance,
    PriorityBased,
}

impl DMSResourceScheduler {
    pub fn _Fnew() -> Self {
        Self {
            performance_history: HashMap::new(),
            device_loads: HashMap::new(),
            policies: Vec::new(),
        }
    }
    
    /// Schedule a resource request to the most suitable device
    pub fn _Fschedule_resource(
        &mut self,
        request: &ResourceRequest,
        available_devices: &[DMSDevice],
    ) -> Option<String> {
        // Filter devices that meet requirements
        let suitable_devices: Vec<&DMSDevice> = available_devices
            .iter()
            .filter(|device| self._Fmeets_requirements(device, request))
            .collect();
        
        if suitable_devices.is_empty() {
            return None;
        }
        
        // Score each suitable device
        let mut device_scores: Vec<(String, f64)> = suitable_devices
            .iter()
            .map(|device| {
                let score = self._Fcalculate_device_score(device, request);
                (device._Fid().to_string(), score)
            })
            .collect();
        
        // Sort by score (highest first)
        device_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        // Return the best device
        device_scores.first().map(|(device_id, _)| device_id.clone())
    }
    
    /// Check if device meets request requirements
    fn _Fmeets_requirements(&self, device: &DMSDevice, request: &ResourceRequest) -> bool {
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
    
    /// Calculate device score for scheduling
    fn _Fcalculate_device_score(&self, device: &DMSDevice, request: &ResourceRequest) -> f64 {
        let device_id = device._Fid();
        let base_score = 100.0;
        let mut score = base_score;
        
        // Penalize based on current load
        if let Some(&load) = self.device_loads.get(device_id) {
            score -= load * 50.0; // Max 50 point penalty for load
        }
        
        // Bonus for device performance history
        if let Some(performance_records) = self.performance_history.get(device_id) {
            if !performance_records.is_empty() {
                let avg_latency = performance_records
                    .iter()
                    .map(|record| record.latency_ms)
                    .sum::<f64>() / performance_records.len() as f64;
                
                let avg_throughput = performance_records
                    .iter()
                    .map(|record| record.throughput)
                    .sum::<f64>() / performance_records.len() as f64;
                
                // Bonus for low latency (up to 20 points)
                score += (100.0 - avg_latency.min(100.0)) * 0.2;
                
                // Bonus for high throughput (up to 20 points)
                score += (avg_throughput.min(100.0)) * 0.2;
            }
        }
        
        // Apply policy-based adjustments
        for policy in &self.policies {
            score = self._Fapply_policy_score_adjustment(device, request, policy, score);
        }
        
        score.max(0.0) // Ensure non-negative score
    }
    
    /// Apply policy-based score adjustments
    fn _Fapply_policy_score_adjustment(
        &self,
        device: &DMSDevice,
        request: &ResourceRequest,
        policy: &SchedulingPolicy,
        current_score: f64,
    ) -> f64 {
        // Check if policy conditions are met
        let conditions_met = policy.conditions.iter().all(|condition| {
            self._Fevaluate_condition(device, request, condition)
        });
        
        if conditions_met {
            match &policy.action {
                PolicyAction::PreferDevice(preferred_device) => {
                    if device._Fid() == preferred_device {
                        current_score + 30.0 // Bonus for preferred device
                    } else {
                        current_score
                    }
                }
                PolicyAction::AvoidDevice(avoided_device) => {
                    if device._Fid() == avoided_device {
                        current_score - 30.0 // Penalty for avoided device
                    } else {
                        current_score
                    }
                }
                PolicyAction::LoadBalance => {
                    // Penalty based on current load for load balancing
                    let load_penalty = self.device_loads.get(device._Fid()).unwrap_or(&0.0) * 20.0;
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
    
    /// Evaluate a policy condition
    fn _Fevaluate_condition(
        &self,
        device: &DMSDevice,
        request: &ResourceRequest,
        condition: &PolicyCondition,
    ) -> bool {
        let value = match condition.metric.as_str() {
            "device_type" => {
                // Convert device type to numeric value for comparison
                match device._Fdevice_type() {
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
    
    /// Record device performance after task completion
    pub fn _Frecord_performance(
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
            .or_insert_with(Vec::new)
            .push(record);
        
        // Keep only recent performance records (last 100)
        if let Some(history) = self.performance_history.get_mut(device_id) {
            if history.len() > 100 {
                history.remove(0);
            }
        }
    }
    
    /// Update device load
    pub fn _Fupdate_device_load(&mut self, device_id: &str, load: f64) {
        self.device_loads.insert(device_id.to_string(), load.clamp(0.0, 1.0));
    }
    
    /// Add a scheduling policy
    pub fn _Fadd_policy(&mut self, policy: SchedulingPolicy) {
        self.policies.push(policy);
        // Sort by priority (highest first)
        self.policies.sort_by(|a, b| b.priority.cmp(&a.priority));
    }
}

/// Resource request for scheduling
#[derive(Debug, Clone)]
pub struct ResourceRequest {
    pub request_id: String,
    pub required_memory_gb: Option<f64>,
    pub required_compute_units: Option<usize>,
    pub required_bandwidth_gbps: Option<f64>,
    pub required_custom_capabilities: HashMap<String, String>,
    pub priority: u8,
    pub deadline: Option<Instant>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_device_discovery_engine() {
        let mut engine = DMSDeviceDiscoveryEngine::_Fnew();
        
        let scan_results = vec![
            DeviceScanResult {
                device_id: "gpu_1".to_string(),
                device_name: "NVIDIA GPU".to_string(),
                device_info: [
                    ("device_name".to_string(), "NVIDIA GeForce RTX 3080".to_string()),
                    ("driver".to_string(), "CUDA 11.4".to_string()),
                ].iter().cloned().collect(),
            },
        ];
        
        let devices = engine._Fdiscover_devices(scan_results);
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0]._Fdevice_type(), DMSDeviceType::GPU);
    }
    
    #[test]
    fn test_resource_scheduler() {
        let mut scheduler = DMSResourceScheduler::_Fnew();
        
        let request = ResourceRequest {
            request_id: "req_1".to_string(),
            required_memory_gb: Some(8.0),
            required_compute_units: Some(256),
            required_bandwidth_gbps: Some(100.0),
            required_custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
            priority: 5,
            deadline: None,
        };
        
        let devices = vec![
            DMSDevice::new(
                "GPU 1".to_string(),
                DMSDeviceType::GPU,
            ).with_capabilities(DMSDeviceCapabilities {
                memory_gb: Some(16.0),
                compute_units: Some(512),
                storage_gb: Some(1000.0),
                bandwidth_gbps: Some(900.0),
                custom_capabilities: [("cuda_support".to_string(), "true".to_string())].iter().cloned().collect(),
            }),
        ];
        
        let selected_device = scheduler._Fschedule_resource(&request, &devices);
        assert_eq!(selected_device, Some(devices[0]._Fid().to_string()));
    }
}