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

use crate::device::device::DMSDeviceType;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

// No device imports needed for scheduler

/// Device scheduler - manages device resource allocation and scheduling
pub struct DMSDeviceScheduler {
    scheduling_policies: HashMap<DMSDeviceType, DMSSchedulingPolicy>,
    allocation_history: Vec<DMSAllocationRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSSchedulingPolicy {
    FirstFit,
    BestFit,
    WorstFit,
    RoundRobin,
    PriorityBased,
    LoadBalanced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSAllocationRecord {
    pub allocation_id: String,
    pub device_id: String,
    pub device_type: DMSDeviceType,
    pub allocated_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<f64>,
    pub success: bool,
}

impl DMSDeviceScheduler {
    pub fn _Fnew() -> Self {
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
        }
    }
    
    /// Get scheduling policy for a device type
    pub fn _Fget_policy(&self, device_type: &DMSDeviceType) -> &DMSSchedulingPolicy {
        self.scheduling_policies.get(device_type).unwrap_or(&DMSSchedulingPolicy::FirstFit)
    }
    
    /// Set scheduling policy for a device type
    pub fn _Fset_policy(&mut self, device_type: DMSDeviceType, policy: DMSSchedulingPolicy) {
        self.scheduling_policies.insert(device_type, policy);
    }
    
    /// Record an allocation
    pub fn _Frecord_allocation(&mut self, allocation_id: String, device_id: String, device_type: DMSDeviceType) {
        let record = DMSAllocationRecord {
            allocation_id,
            device_id,
            device_type,
            allocated_at: Utc::now(),
            released_at: None,
            duration_seconds: None,
            success: true,
        };
        
        self.allocation_history.push(record);
        
        // Keep only recent history (last 1000 allocations)
        if self.allocation_history.len() > 1000 {
            self.allocation_history.remove(0);
        }
    }
    
    /// Record a release
    pub fn _Frecord_release(&mut self, allocation_id: &str) {
        if let Some(record) = self.allocation_history.iter_mut().find(|r| r.allocation_id == allocation_id) {
            record.released_at = Some(Utc::now());
            
            if let Ok(duration) = record.released_at.unwrap().signed_duration_since(record.allocated_at).to_std() {
                record.duration_seconds = Some(duration.as_secs_f64());
            }
        }
    }
    
    /// Get allocation statistics
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
        for device_type in vec![
            DMSDeviceType::CPU, DMSDeviceType::GPU, DMSDeviceType::Memory, 
            DMSDeviceType::Storage, DMSDeviceType::Network, DMSDeviceType::Sensor, 
            DMSDeviceType::Actuator, DMSDeviceType::Custom
        ] {
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
    
    /// Get scheduling recommendations
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
                description: format!("No recent allocation data for {:?}, using default policy", device_type),
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
                description: format!("Average allocation duration is {:.1} seconds for {:?}, consider load balancing", 
                    avg_duration, device_type),
                priority: 2,
                confidence: 0.7,
            });
        }
        
        if recent_allocations.len() > 50 && avg_duration < 60.0 {
            recommendations.push(DMSSchedulingRecommendation {
                recommendation_type: DMSSchedulingRecommendationType::OptimizeForShortRunning,
                description: format!("High frequency of short allocations for {:?}, consider round-robin scheduling", device_type),
                priority: 2,
                confidence: 0.6,
            });
        }
        
        recommendations.push(DMSSchedulingRecommendation {
            recommendation_type: DMSSchedulingRecommendationType::ContinueCurrentPolicy,
            description: format!("Current scheduling policy appears effective for {:?}", device_type),
            priority: 1,
            confidence: 0.9,
        });
        
        recommendations.sort_by(|a, b| b.priority.cmp(&a.priority));
        recommendations
    }
}

/// Allocation statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSAllocationStatistics {
    pub total_allocations: usize,
    pub successful_allocations: usize,
    pub failed_allocations: usize,
    pub success_rate: f64,
    pub average_duration_seconds: f64,
    pub by_device_type: HashMap<DMSDeviceType, DMSDeviceTypeStatistics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDeviceTypeStatistics {
    pub total_allocations: usize,
    pub completed_allocations: usize,
    pub average_duration_seconds: f64,
}

/// Scheduling recommendation types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSSchedulingRecommendationType {
    UseDefaultPolicy,
    ContinueCurrentPolicy,
    ConsiderPolicyChange,
    OptimizeForLongRunning,
    OptimizeForShortRunning,
    LoadBalance,
    Prioritize,
}

/// Scheduling recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSSchedulingRecommendation {
    pub recommendation_type: DMSSchedulingRecommendationType,
    pub description: String,
    pub priority: u8, // 1-10, higher is more important
    pub confidence: f64, // 0.0-1.0, confidence in this recommendation
}