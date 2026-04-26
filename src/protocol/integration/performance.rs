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

use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

use super::super::{RiProtocolType};

/// Performance coordinator for cross-protocol performance optimization.
pub struct RiPerformanceCoordinator {
    /// Performance metrics
    pub metrics: Arc<RwLock<RiPerformanceMetrics>>,
    /// Performance optimizations
    pub optimizations: Arc<RwLock<Vec<RiPerformanceOptimization>>>,
    /// Performance monitoring
    pub monitor: Arc<RiPerformanceMonitor>,
}

/// Performance metrics structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceMetrics {
    /// Protocol performance metrics
    pub protocol_metrics: FxHashMap<RiProtocolType, RiProtocolPerformanceMetrics>,
    /// Cross-protocol metrics
    pub cross_protocol_metrics: RiCrossProtocolMetrics,
    /// System performance metrics
    pub system_metrics: RiSystemPerformanceMetrics,
    /// Last update time
    pub last_update: Instant,
}

/// Protocol performance metrics structure.
#[derive(Debug, Clone)]
pub struct RiProtocolPerformanceMetrics {
    /// Protocol type
    pub protocol_type: RiProtocolType,
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
pub struct RiCrossProtocolMetrics {
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
pub struct RiSystemPerformanceMetrics {
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
pub struct RiPerformanceOptimization {
    /// Optimization identifier
    pub optimization_id: String,
    /// Optimization type
    pub optimization_type: RiPerformanceOptimizationType,
    /// Optimization description
    pub description: String,
    /// Performance impact
    pub performance_impact: f32,
    /// Implementation status
    pub implementation_status: RiImplementationStatus,
    /// Optimization parameters
    pub parameters: FxHashMap<String, String>,
}

/// Performance optimization type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiPerformanceOptimizationType {
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
pub enum RiImplementationStatus {
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
pub struct RiPerformanceMonitor {
    /// Monitoring configuration
    pub config: Arc<RiPerformanceMonitoringConfig>,
    /// Monitoring results
    pub results: Arc<RwLock<Vec<RiPerformanceMonitoringResult>>>,
    /// Performance alerts
    pub alerts: Arc<RwLock<Vec<RiPerformanceAlert>>>,
}

/// Performance monitoring configuration structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceMonitoringConfig {
    /// Monitoring interval
    pub monitoring_interval: Duration,
    /// Performance thresholds
    pub thresholds: RiPerformanceThresholds,
    /// Alert configuration
    pub alert_config: RiPerformanceAlertConfig,
}

/// Performance thresholds structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceThresholds {
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
pub struct RiPerformanceAlertConfig {
    /// Alert enabled
    pub alert_enabled: bool,
    /// Alert severity levels
    pub alert_severity_levels: Vec<RiAlertSeverityLevel>,
    /// Alert destinations
    pub alert_destinations: Vec<String>,
}

/// Alert severity level enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiAlertSeverityLevel {
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
pub struct RiPerformanceMonitoringResult {
    /// Result identifier
    pub result_id: String,
    /// Monitoring timestamp
    pub timestamp: Instant,
    /// Performance metrics
    pub metrics: RiPerformanceMetrics,
    /// Threshold violations
    pub threshold_violations: Vec<RiPerformanceThresholdViolation>,
    /// Recommendations
    pub recommendations: Vec<String>,
}

/// Performance threshold violation structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceThresholdViolation {
    /// Violated threshold
    pub threshold: String,
    /// Actual value
    pub actual_value: f64,
    /// Threshold value
    pub threshold_value: f64,
    /// Violation severity
    pub severity: RiAlertSeverityLevel,
}

/// Performance alert structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceAlert {
    /// Alert identifier
    pub alert_id: String,
    /// Alert type
    pub alert_type: RiPerformanceAlertType,
    /// Alert message
    pub message: String,
    /// Alert severity
    pub severity: RiAlertSeverityLevel,
    /// Alert time
    pub alert_time: Instant,
    /// Alert data
    pub alert_data: FxHashMap<String, String>,
}

/// Performance alert type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiPerformanceAlertType {
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

impl RiPerformanceCoordinator {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(RiPerformanceMetrics::new())),
            optimizations: Arc::new(RwLock::new(Vec::new())),
            monitor: Arc::new(RiPerformanceMonitor::new()),
        }
    }

    pub async fn collect_metrics(&self) -> RiPerformanceMetrics {
        let mut metrics = self.metrics.write().await;
        metrics.last_update = Instant::now();
        metrics.clone()
    }

    pub async fn add_optimization(&self, optimization: RiPerformanceOptimization) {
        let mut optimizations = self.optimizations.write().await;
        optimizations.push(optimization);
    }

    pub async fn get_optimizations(&self) -> Vec<RiPerformanceOptimization> {
        self.optimizations.read().await.clone()
    }

    pub async fn check_thresholds(&self) -> Vec<RiPerformanceThresholdViolation> {
        let metrics = self.metrics.read().await;
        let config = self.monitor.config.clone();
        let mut violations = Vec::with_capacity(4);

        for (_, protocol_metrics) in &metrics.protocol_metrics {
            if protocol_metrics.avg_latency > config.thresholds.max_latency {
                violations.push(RiPerformanceThresholdViolation {
                    threshold: "max_latency".to_string(),
                    actual_value: protocol_metrics.avg_latency.as_secs_f64(),
                    threshold_value: config.thresholds.max_latency.as_secs_f64(),
                    severity: RiAlertSeverityLevel::Warning,
                });
            }

            if protocol_metrics.throughput < config.thresholds.min_throughput {
                violations.push(RiPerformanceThresholdViolation {
                    threshold: "min_throughput".to_string(),
                    actual_value: protocol_metrics.throughput as f64,
                    threshold_value: config.thresholds.min_throughput as f64,
                    severity: RiAlertSeverityLevel::Warning,
                });
            }

            if protocol_metrics.error_rate > config.thresholds.max_error_rate {
                violations.push(RiPerformanceThresholdViolation {
                    threshold: "max_error_rate".to_string(),
                    actual_value: protocol_metrics.error_rate as f64,
                    threshold_value: config.thresholds.max_error_rate as f64,
                    severity: RiAlertSeverityLevel::Error,
                });
            }
        }

        if metrics.system_metrics.cpu_utilization > config.thresholds.max_cpu_utilization {
            violations.push(RiPerformanceThresholdViolation {
                threshold: "max_cpu_utilization".to_string(),
                actual_value: metrics.system_metrics.cpu_utilization as f64,
                threshold_value: config.thresholds.max_cpu_utilization as f64,
                severity: RiAlertSeverityLevel::Critical,
            });
        }

        if metrics.system_metrics.memory_utilization > config.thresholds.max_memory_utilization {
            violations.push(RiPerformanceThresholdViolation {
                threshold: "max_memory_utilization".to_string(),
                actual_value: metrics.system_metrics.memory_utilization as f64,
                threshold_value: config.thresholds.max_memory_utilization as f64,
                severity: RiAlertSeverityLevel::Critical,
            });
        }

        violations
    }

    pub async fn update_protocol_metrics(
        &self,
        protocol_type: RiProtocolType,
        metrics: RiProtocolPerformanceMetrics,
    ) {
        let mut perf_metrics = self.metrics.write().await;
        perf_metrics.protocol_metrics.insert(protocol_type, metrics);
        perf_metrics.last_update = Instant::now();
    }

    pub async fn update_system_metrics(&self, metrics: RiSystemPerformanceMetrics) {
        let mut perf_metrics = self.metrics.write().await;
        perf_metrics.system_metrics = metrics;
        perf_metrics.last_update = Instant::now();
    }

    pub async fn generate_recommendations(&self) -> Vec<String> {
        let metrics = self.metrics.read().await;
        let mut recommendations = Vec::with_capacity(4);

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

impl Default for RiPerformanceCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl RiPerformanceMetrics {
    pub fn new() -> Self {
        Self {
            protocol_metrics: FxFxHashMap::default(),
            cross_protocol_metrics: RiCrossProtocolMetrics::default(),
            system_metrics: RiSystemPerformanceMetrics::default(),
            last_update: Instant::now(),
        }
    }
}

impl Default for RiPerformanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RiProtocolPerformanceMetrics {
    pub fn new(protocol_type: RiProtocolType) -> Self {
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

impl Default for RiCrossProtocolMetrics {
    fn default() -> Self {
        Self {
            cross_protocol_latency: Duration::from_millis(0),
            protocol_switching_time: Duration::from_millis(0),
            state_sync_time: Duration::from_millis(0),
            message_routing_efficiency: 1.0,
        }
    }
}

impl Default for RiSystemPerformanceMetrics {
    fn default() -> Self {
        Self {
            cpu_utilization: 0.0,
            memory_utilization: 0.0,
            network_utilization: 0.0,
            disk_utilization: 0.0,
        }
    }
}

impl RiPerformanceOptimization {
    pub fn new(
        optimization_type: RiPerformanceOptimizationType,
        description: String,
        performance_impact: f32,
    ) -> Self {
        Self {
            optimization_id: uuid::Uuid::new_v4().to_string(),
            optimization_type,
            description,
            performance_impact,
            implementation_status: RiImplementationStatus::NotImplemented,
            parameters: FxFxHashMap::default(),
        }
    }

    pub fn with_parameter(mut self, key: String, value: String) -> Self {
        self.parameters.insert(key, value);
        self
    }

    pub fn mark_implemented(&mut self) {
        self.implementation_status = RiImplementationStatus::Implemented;
    }

    pub fn mark_tested(&mut self) {
        self.implementation_status = RiImplementationStatus::Tested;
    }
}

impl RiPerformanceMonitor {
    pub fn new() -> Self {
        Self {
            config: Arc::new(RiPerformanceMonitoringConfig::default()),
            results: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub fn with_config(config: RiPerformanceMonitoringConfig) -> Self {
        Self {
            config: Arc::new(config),
            results: Arc::new(RwLock::new(Vec::new())),
            alerts: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn record_result(&self, result: RiPerformanceMonitoringResult) {
        let mut results = self.results.write().await;
        results.push(result);
        
        if results.len() > 1000 {
            results.remove(0);
        }
    }

    pub async fn create_alert(&self, alert_type: RiPerformanceAlertType, message: String, severity: RiAlertSeverityLevel) {
        let alert = RiPerformanceAlert {
            alert_id: uuid::Uuid::new_v4().to_string(),
            alert_type,
            message,
            severity,
            alert_time: Instant::now(),
            alert_data: FxFxHashMap::default(),
        };

        let mut alerts = self.alerts.write().await;
        alerts.push(alert);
    }

    pub async fn get_alerts(&self) -> Vec<RiPerformanceAlert> {
        self.alerts.read().await.clone()
    }

    pub async fn clear_alerts(&self) {
        self.alerts.write().await.clear();
    }

    pub async fn get_latest_results(&self, count: usize) -> Vec<RiPerformanceMonitoringResult> {
        let results = self.results.read().await;
        results.iter().rev().take(count).cloned().collect()
    }
}

impl Default for RiPerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RiPerformanceMonitoringConfig {
    fn default() -> Self {
        Self {
            monitoring_interval: Duration::from_secs(60),
            thresholds: RiPerformanceThresholds::default(),
            alert_config: RiPerformanceAlertConfig::default(),
        }
    }
}

impl Default for RiPerformanceThresholds {
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

impl Default for RiPerformanceAlertConfig {
    fn default() -> Self {
        Self {
            alert_enabled: true,
            alert_severity_levels: vec![
                RiAlertSeverityLevel::Information,
                RiAlertSeverityLevel::Warning,
                RiAlertSeverityLevel::Error,
                RiAlertSeverityLevel::Critical,
            ],
            alert_destinations: vec!["log".to_string()],
        }
    }
}

impl RiPerformanceMonitoringResult {
    pub fn new(metrics: RiPerformanceMetrics) -> Self {
        Self {
            result_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Instant::now(),
            metrics,
            threshold_violations: Vec::new(),
            recommendations: Vec::new(),
        }
    }

    pub fn with_violations(mut self, violations: Vec<RiPerformanceThresholdViolation>) -> Self {
        self.threshold_violations = violations;
        self
    }

    pub fn with_recommendations(mut self, recommendations: Vec<String>) -> Self {
        self.recommendations = recommendations;
        self
    }
}

impl RiPerformanceThresholdViolation {
    pub fn new(threshold: String, actual_value: f64, threshold_value: f64, severity: RiAlertSeverityLevel) -> Self {
        Self {
            threshold,
            actual_value,
            threshold_value,
            severity,
        }
    }

    pub fn is_critical(&self) -> bool {
        matches!(self.severity, RiAlertSeverityLevel::Critical)
    }
}

impl RiPerformanceAlert {
    pub fn new(alert_type: RiPerformanceAlertType, message: String, severity: RiAlertSeverityLevel) -> Self {
        Self {
            alert_id: uuid::Uuid::new_v4().to_string(),
            alert_type,
            message,
            severity,
            alert_time: Instant::now(),
            alert_data: FxFxHashMap::default(),
        }
    }

    pub fn with_data(mut self, key: String, value: String) -> Self {
        self.alert_data.insert(key, value);
        self
    }
}

use crate::core::{RiResult, RiError};
