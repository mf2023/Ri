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

impl DMSCPerformanceCoordinator {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(DMSCPerformanceMetrics::new())),
            optimizations: Arc::new(RwLock::new(Vec::new())),
            monitor: Arc::new(DMSCPerformanceMonitor::new()),
        }
    }

    pub async fn collect_metrics(&self) -> DMSCPerformanceMetrics {
        let mut metrics = self.metrics.write().await;
        metrics.last_update = Instant::now();
        metrics.clone()
    }

    pub async fn add_optimization(&self, optimization: DMSCPerformanceOptimization) {
        let mut optimizations = self.optimizations.write().await;
        optimizations.push(optimization);
    }

    pub async fn get_optimizations(&self) -> Vec<DMSCPerformanceOptimization> {
        self.optimizations.read().await.clone()
    }

    pub async fn check_thresholds(&self) -> Vec<DMSCPerformanceThresholdViolation> {
        let metrics = self.metrics.read().await;
        let config = self.monitor.config.clone();
        let mut violations = Vec::new();

        for (_, protocol_metrics) in &metrics.protocol_metrics {
            if protocol_metrics.avg_latency > config.thresholds.max_latency {
                violations.push(DMSCPerformanceThresholdViolation {
                    threshold: "max_latency".to_string(),
                    actual_value: protocol_metrics.avg_latency.as_secs_f64(),
                    threshold_value: config.thresholds.max_latency.as_secs_f64(),
                    severity: DMSCAlertSeverityLevel::Warning,
                });
            }

            if protocol_metrics.throughput < config.thresholds.min_throughput {
                violations.push(DMSCPerformanceThresholdViolation {
                    threshold: "min_throughput".to_string(),
                    actual_value: protocol_metrics.throughput as f64,
                    threshold_value: config.thresholds.min_throughput as f64,
                    severity: DMSCAlertSeverityLevel::Warning,
                });
            }

            if protocol_metrics.error_rate > config.thresholds.max_error_rate {
                violations.push(DMSCPerformanceThresholdViolation {
                    threshold: "max_error_rate".to_string(),
                    actual_value: protocol_metrics.error_rate as f64,
                    threshold_value: config.thresholds.max_error_rate as f64,
                    severity: DMSCAlertSeverityLevel::Error,
                });
            }
        }

        if metrics.system_metrics.cpu_utilization > config.thresholds.max_cpu_utilization {
            violations.push(DMSCPerformanceThresholdViolation {
                threshold: "max_cpu_utilization".to_string(),
                actual_value: metrics.system_metrics.cpu_utilization as f64,
                threshold_value: config.thresholds.max_cpu_utilization as f64,
                severity: DMSCAlertSeverityLevel::Critical,
            });
        }

        if metrics.system_metrics.memory_utilization > config.thresholds.max_memory_utilization {
            violations.push(DMSCPerformanceThresholdViolation {
                threshold: "max_memory_utilization".to_string(),
                actual_value: metrics.system_metrics.memory_utilization as f64,
                threshold_value: config.thresholds.max_memory_utilization as f64,
                severity: DMSCAlertSeverityLevel::Critical,
            });
        }

        violations
    }

    pub async fn update_protocol_metrics(
        &self,
        protocol_type: DMSCProtocolType,
        metrics: DMSCProtocolPerformanceMetrics,
    ) {
        let mut perf_metrics = self.metrics.write().await;
        perf_metrics.protocol_metrics.insert(protocol_type, metrics);
        perf_metrics.last_update = Instant::now();
    }

    pub async fn update_system_metrics(&self, metrics: DMSCSystemPerformanceMetrics) {
        let mut perf_metrics = self.metrics.write().await;
        perf_metrics.system_metrics = metrics;
        perf_metrics.last_update = Instant::now();
    }

    pub async fn generate_recommendations(&self) -> Vec<String> {
        let metrics = self.metrics.read().await;
        let mut recommendations = Vec::new();

        for (_, protocol_metrics) in &metrics.protocol_metrics {
            if protocol_metrics.error_rate > 0.05 {
                recommendations.push(format!(
                    "Consider investigating high error rate ({:.2}%) for protocol {:?}",
                    protocol_metrics.error_rate * 100.0,
                    protocol_metrics.protocol_type
                ));
            }

            if protocol_metrics.avg_latency > Duration::from_millis(100) {
                recommendations.push(format!(
                    "High latency detected ({:.2}ms) for protocol {:?}. Consider optimizing connection pooling.",
                    protocol_metrics.avg_latency.as_secs_f64() * 1000.0,
                    protocol_metrics.protocol_type
                ));
            }
        }

        if metrics.system_metrics.cpu_utilization > 0.8 {
            recommendations.push("High CPU utilization detected. Consider scaling or optimizing CPU-intensive operations.".to_string());
        }

        if metrics.system_metrics.memory_utilization > 0.8 {
            recommendations.push("High memory utilization detected. Consider implementing caching strategies or increasing memory.".to_string());
        }

        recommendations
    }
}

impl Default for DMSCPerformanceCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            protocol_metrics: HashMap::new(),
            cross_protocol_metrics: DMSCCrossProtocolMetrics::default(),
            system_metrics: DMSCSystemPerformanceMetrics::default(),
            last_update: Instant::now(),
        }
    }
}

impl Default for DMSCPerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCProtocolPerformanceMetrics {
    pub fn new(protocol_type: DMSCProtocolType) -> Self {
        Self {
            protocol_type,
            avg_latency: Duration::from_millis(0),
            throughput: 0,
            error_rate: 0.0,
            connection_count: 0,
            success_rate: 1.0,
        }
    }

    pub fn update_from_measurements(&mut self, latencies: &[Duration], success_count: u64, error_count: u64) {
        if !latencies.is_empty() {
            let total: Duration = latencies.iter().sum();
            self.avg_latency = total / latencies.len() as u32;
        }

        let total_requests = success_count + error_count;
        if total_requests > 0 {
            self.success_rate = success_count as f32 / total_requests as f32;
            self.error_rate = error_count as f32 / total_requests as f32;
        }
    }
}

impl Default for DMSCCrossProtocolMetrics {
    fn default() -> Self {
        Self {
            cross_protocol_latency: Duration::from_millis(0),
            protocol_switching_time: Duration::from_millis(0),
            state_sync_time: Duration::from_millis(0),
            message_routing_efficiency: 1.0,
        }
    }
}

impl Default for DMSCSystemPerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            network_utilization: 0.0,
            disk_utilization: 0.0,
        }
    }
}

impl DMSCPerformanceOptimization {
    pub fn new(
        optimization_type: DMSCPerformanceOptimizationType,
        description: String,
        performance_impact: f32,
    ) -> Self {
        Self {
            optimization_id: uuid::Uuid::new_v4().to_string(),
            optimization_type,
            description,
            performance_impact,
            implementation_status: DMSCImplementationStatus::NotImplemented,
            parameters: HashMap::new(),
        }
    }

    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }

    pub fn mark_implemented(&mut self) {
        self.implementation_status = DMSCImplementationStatus::Implemented;
    }

    pub fn mark_tested(&mut self) {
        self.implementation_status = DMSCImplementationStatus::Tested;
    }
}

impl DMSCPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            config: Arc::new(DMSCPerformanceMonitoringConfig::default()),
            results: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn with_config(config: DMSCPerformanceMonitoringConfig) -> Self {
        Self {
            config: Arc::new(config),
            results: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_result(&self, result: DMSCPerformanceMonitoringResult) {
        let mut results = self.results.write().await;
        results.push(result);
        
        if results.len() > 1000 {
            results.remove(0);
        }
    }

    pub async fn create_alert(&self, alert_type: DMSCPerformanceAlertType, message: String, severity: DMSCAlertSeverityLevel) {
        let alert = DMSCPerformanceAlert {
            alert_id: uuid::Uuid::new_v4().to_string(),
            alert_type,
            message,
            severity,
            alert_time: Instant::now(),
            alert_data: HashMap::new(),
        };

        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
    }

    pub async fn get_alerts(&self) -> Vec<DMSCPerformanceAlert> {
        self.alerts.read().await.clone()
    }

    pub async fn clear_alerts(&self) {
        self.alerts.write().await.clear();
    }

    pub async fn get_latest_results(&self, count: usize) -> Vec<DMSCPerformanceMonitoringResult> {
        let results = self.results.read().await;
        results.iter().rev().take(count).cloned().collect()
    }
}

impl Default for DMSCPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for DMSCPerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(60),
            thresholds: DMSCPerformanceThresholds::default(),
            alert_config: DMSCPerformanceAlertConfig::default(),
        }
    }
}

impl Default for DMSCPerformanceThresholds {
    fn default() -> Self {
        Self {
            max_latency: Duration::from_millis(1000),
            min_throughput: 100,
            max_error_rate: 0.05,
            max_cpu_utilization: 0.8,
            max_memory_utilization: 0.8,
        }
    }
}

impl Default for DMSCPerformanceAlertConfig {
    fn default() -> Self {
        Self {
            alert_enabled: true,
            alert_severity_levels: vec![
                DMSCAlertSeverityLevel::Information,
                DMSCAlertSeverityLevel::Warning,
                DMSCAlertSeverityLevel::Error,
                DMSCAlertSeverityLevel::Critical,
            ],
            alert_destinations: vec!["log".to_string()],
        }
    }
}

impl DMSCPerformanceMonitoringResult {
    pub fn new(metrics: DMSCPerformanceMetrics) -> Self {
        Self {
            result_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Instant::now(),
            metrics,
            threshold_violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn with_violations(mut self, violations: Vec<DMSCPerformanceThresholdViolation>) -> Self {
        self.threshold_violations = violations;
        self
    }

    pub fn with_recommendations(mut self, recommendations: Vec<String>) -> Self {
        self.recommendations = recommendations;
        self
    }
}

impl DMSCPerformanceThresholdViolation {
    pub fn new(threshold: String, actual_value: f64, threshold_value: f64, severity: DMSCAlertSeverityLevel) -> Self {
        Self {
            threshold,
            actual_value,
            threshold_value,
            severity,
        }
    }

    pub fn is_critical(&self) -> bool {
        matches!(self.severity, DMSCAlertSeverityLevel::Critical)
    }
}

impl DMSCPerformanceAlert {
    pub fn new(alert_type: DMSCPerformanceAlertType, message: String, severity: DMSCAlertSeverityLevel) -> Self {
        Self {
            alert_id: uuid::Uuid::new_v4().to_string(),
            alert_type,
            message,
            severity,
            alert_time: Instant::now(),
            alert_data: HashMap::new(),
        }
    }

    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.alert_data.insert(key, value);
        self
    }
}

use crate::core::{DMSCResult, DMSCError};
