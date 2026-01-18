//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use super::super::{DMSCProtocolType};

/// Performance coordinator for cross-protocol performance optimization.
pub struct DMSCPerformanceCoordinator {
    /// Performance metrics
    pub metrics: Arc<RwLock<DMSCPerformanceMetrics>>,
    /// Performance optimizations
    pub optimizations: Arc<RwLock<Vec<DMSCPerformanceOptimization>>>,
    /// Performance monitoring
    pub monitor: Arc<DMSCPerformanceMonitor>,
}

/// Performance metrics structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceMetrics {
    /// Protocol performance metrics
    pub protocol_metrics: HashMap<DMSCProtocolType, DMSCProtocolPerformanceMetrics>,
    /// Cross-protocol metrics
    pub cross_protocol_metrics: DMSCCrossProtocolMetrics,
    /// System performance metrics
    pub system_metrics: DMSCSystemPerformanceMetrics,
    /// Last update time
    pub last_update: Instant,
}

/// Protocol performance metrics structure.
#[derive(Debug, Clone)]
pub struct DMSCProtocolPerformanceMetrics {
    /// Protocol type
    pub protocol_type: DMSCProtocolType,
    /// Average latency
    pub avg_latency: Duration,
    /// Throughput
    pub throughput: u64,
    /// Error rate
    pub error_rate: f32,
    /// Connection count
    pub connection_count: u32,
    /// Success rate
    pub success_rate: f32,
}

/// Cross-protocol metrics structure.
#[derive(Debug, Clone)]
pub struct DMSCCrossProtocolMetrics {
    /// Cross-protocol latency
    pub cross_protocol_latency: Duration,
    /// Protocol switching time
    pub protocol_switching_time: Duration,
    /// State synchronization time
    pub state_sync_time: Duration,
    /// Message routing efficiency
    pub message_routing_efficiency: f32,
}

/// System performance metrics structure.
#[derive(Debug, Clone)]
pub struct DMSCSystemPerformanceMetrics {
    /// CPU utilization
    pub cpu_utilization: f32,
    /// Memory utilization
    pub memory_utilization: f32,
    /// Network utilization
    pub network_utilization: f32,
    /// Disk utilization
    pub disk_utilization: f32,
}

/// Performance optimization structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceOptimization {
    /// Optimization identifier
    pub optimization_id: String,
    /// Optimization type
    pub optimization_type: DMSCPerformanceOptimizationType,
    /// Optimization description
    pub description: String,
    /// Performance impact
    pub performance_impact: f32,
    /// Implementation status
    pub implementation_status: DMSCImplementationStatus,
    /// Optimization parameters
    pub parameters: HashMap<String, String>,
}

/// Performance optimization type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCPerformanceOptimizationType {
    /// Protocol optimization
    Protocol,
    /// Connection optimization
    Connection,
    /// Message routing optimization
    MessageRouting,
    /// State synchronization optimization
    StateSync,
    /// Security optimization
    Security,
}

/// Implementation status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCImplementationStatus {
    /// Not implemented
    NotImplemented,
    /// In progress
    InProgress,
    /// Implemented
    Implemented,
    /// Tested
    Tested,
}

/// Performance monitor structure.
pub struct DMSCPerformanceMonitor {
    /// Monitoring configuration
    pub config: Arc<DMSCPerformanceMonitoringConfig>,
    /// Monitoring results
    pub results: Arc<RwLock<Vec<DMSCPerformanceMonitoringResult>>>,
    /// Performance alerts
    pub alerts: Arc<RwLock<Vec<DMSCPerformanceAlert>>>,
}

/// Performance monitoring configuration structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceMonitoringConfig {
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Performance thresholds
    pub thresholds: DMSCPerformanceThresholds,
    /// Alert configuration
    pub alert_config: DMSCPerformanceAlertConfig,
}

/// Performance thresholds structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceThresholds {
    /// Maximum latency threshold
    pub max_latency: Duration,
    /// Minimum throughput threshold
    pub min_throughput: u64,
    /// Maximum error rate threshold
    pub max_error_rate: f32,
    /// Maximum CPU utilization threshold
    pub max_cpu_utilization: f32,
    /// Maximum memory utilization threshold
    pub max_memory_utilization: f32,
}

/// Performance alert configuration structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceAlertConfig {
    /// Alert enabled
    pub alert_enabled: bool,
    /// Alert severity levels
    pub alert_severity_levels: Vec<DMSCAlertSeverityLevel>,
    /// Alert destinations
    pub alert_destinations: Vec<String>,
}

/// Alert severity level enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCAlertSeverityLevel {
    /// Information severity
    Information,
    /// Warning severity
    Warning,
    /// Error severity
    Error,
    /// Critical severity
    Critical,
}

/// Performance monitoring result structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceMonitoringResult {
    /// Result identifier
    pub result_id: String,
    /// Monitoring timestamp
    pub timestamp: Instant,
    /// Performance metrics
    pub metrics: DMSCPerformanceMetrics,
    /// Threshold violations
    pub threshold_violations: Vec<DMSCPerformanceThresholdViolation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Performance threshold violation structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceThresholdViolation {
    /// Violated threshold
    pub threshold: String,
    /// Actual value
    pub actual_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Violation severity
    pub severity: DMSCAlertSeverityLevel,
}

/// Performance alert structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceAlert {
    /// Alert identifier
    pub alert_id: String,
    /// Alert type
    pub alert_type: DMSCPerformanceAlertType,
    /// Alert message
    pub message: String,
    /// Alert severity
    pub severity: DMSCAlertSeverityLevel,
    /// Alert time
    pub alert_time: Instant,
    /// Alert data
    pub alert_data: HashMap<String, String>,
}

/// Performance alert type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCPerformanceAlertType {
    /// High latency alert
    HighLatency,
    /// Low throughput alert
    LowThroughput,
    /// High error rate alert
    HighErrorRate,
    /// High resource utilization alert
    HighResourceUtilization,
    /// Protocol switching performance alert
    ProtocolSwitchingPerformance,
}
