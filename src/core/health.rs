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

//! # Health Check System
//!
//! This module provides comprehensive health checking functionality for Ri modules and services.
//! It supports both active health checks (proactive monitoring) and passive health indicators
//! (reactive status reporting).
//!
//! ## Key Components
//!
//! - **HealthStatus**: Enum representing the health state of a component
//! - **HealthCheck**: Trait for implementing custom health checks
//! - **HealthChecker**: Service for managing and executing health checks
//! - **HealthReport**: Comprehensive health status report
//!
//! ## Design Principles
//!
//! 1. **Non-Intrusive**: Health checks can be added without modifying existing code
//! 2. **Configurable**: Check intervals, timeouts, and thresholds are configurable
//! 3. **Comprehensive**: Supports multiple health indicators and aggregation
//! 4. **Performance-Aware**: Minimal impact on system performance
//! 5. **Extensible**: Easy to add new health check types

use crate::core::RiResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap as FxHashMap;
use std::time::{Duration, SystemTime};

/// Health status enumeration representing the state of a component or service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiHealthStatus {
    /// Component is functioning normally
    Healthy,
    /// Component is experiencing issues but still operational
    Degraded,
    /// Component is not functioning and requires attention
    Unhealthy,
    /// Health status is unknown (check failed or not performed)
    Unknown,
}

impl RiHealthStatus {
    /// Returns true if the status is considered healthy (Healthy or Degraded).
    pub fn is_healthy(&self) -> bool {
        matches!(self, RiHealthStatus::Healthy | RiHealthStatus::Degraded)
    }

    /// Returns true if the status requires immediate attention.
    pub fn requires_attention(&self) -> bool {
        matches!(self, RiHealthStatus::Unhealthy)
    }

    /// Merges multiple health statuses into a single status.
    /// The most severe status takes precedence: Unhealthy > Degraded > Unknown > Healthy
    pub fn merge(statuses: &[RiHealthStatus]) -> RiHealthStatus {
        if statuses.is_empty() {
            return RiHealthStatus::Unknown;
        }

        let mut has_unhealthy = false;
        let mut has_degraded = false;
        let mut has_unknown = false;

        for status in statuses {
            match status {
                RiHealthStatus::Unhealthy => has_unhealthy = true,
                RiHealthStatus::Degraded => has_degraded = true,
                RiHealthStatus::Unknown => has_unknown = true,
                RiHealthStatus::Healthy => {}
            }
        }

        if has_unhealthy {
            RiHealthStatus::Unhealthy
        } else if has_degraded {
            RiHealthStatus::Degraded
        } else if has_unknown {
            RiHealthStatus::Unknown
        } else {
            RiHealthStatus::Healthy
        }
    }
}

impl std::fmt::Display for RiHealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiHealthStatus::Healthy => write!(f, "healthy"),
            RiHealthStatus::Degraded => write!(f, "degraded"),
            RiHealthStatus::Unhealthy => write!(f, "unhealthy"),
            RiHealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiHealthStatus {
    fn __str__(&self) -> String {
        self.to_string()
    }

    fn __repr__(&self) -> String {
        format!("RiHealthStatus::{}", self)
    }

    #[staticmethod]
    fn merge_statuses(statuses: Vec<RiHealthStatus>) -> Self {
        RiHealthStatus::merge(&statuses)
    }
}

/// Result of a health check execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiHealthCheckResult {
    /// Name of the health check
    pub name: String,
    /// Health status
    pub status: RiHealthStatus,
    /// Optional message providing additional context
    pub message: Option<String>,
    /// Timestamp when the check was performed
    pub timestamp: SystemTime,
    /// Duration of the health check execution
    pub duration: Duration,
}

impl RiHealthCheckResult {
    /// Creates a new successful health check result.
    pub fn healthy(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: RiHealthStatus::Healthy,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new degraded health check result.
    pub fn degraded(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: RiHealthStatus::Degraded,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new unhealthy health check result.
    pub fn unhealthy(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: RiHealthStatus::Unhealthy,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new unknown health check result.
    pub fn unknown(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: RiHealthStatus::Unknown,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiHealthCheckResult {
    #[new]
    fn new_py(name: String, status: RiHealthStatus, message: Option<String>) -> Self {
        Self {
            name,
            status,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    #[staticmethod]
    fn create_healthy(name: String, message: Option<String>) -> Self {
        Self::healthy(name, message)
    }

    #[staticmethod]
    fn create_degraded(name: String, message: Option<String>) -> Self {
        Self::degraded(name, message)
    }

    #[staticmethod]
    fn create_unhealthy(name: String, message: Option<String>) -> Self {
        Self::unhealthy(name, message)
    }

    #[staticmethod]
    fn create_unknown(name: String, message: Option<String>) -> Self {
        Self::unknown(name, message)
    }

    #[getter]
    fn name(&self) -> String {
        self.name.clone()
    }

    #[getter]
    fn status(&self) -> RiHealthStatus {
        self.status
    }

    #[getter]
    fn message(&self) -> Option<String> {
        self.message.clone()
    }

    fn __str__(&self) -> String {
        format!("{}: {}", self.name, self.status)
    }

    fn __repr__(&self) -> String {
        format!("RiHealthCheckResult {{ name: {:?}, status: {:?} }}", self.name, self.status)
    }
}

/// Configuration for health checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiHealthCheckConfig {
    /// Interval between health checks
    pub check_interval: Duration,
    /// Timeout for individual health checks
    pub timeout: Duration,
    /// Number of consecutive failures before marking as unhealthy
    pub failure_threshold: u32,
    /// Number of consecutive successes before marking as healthy
    pub success_threshold: u32,
    /// Whether the health check is enabled
    pub enabled: bool,
}

impl Default for RiHealthCheckConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(30),
            timeout: Duration::from_secs(5),
            failure_threshold: 3,
            success_threshold: 2,
            enabled: true,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiHealthCheckConfig {
    #[new]
    fn new_py(check_interval: u64, timeout: u64, failure_threshold: u32, success_threshold: u32, enabled: bool) -> Self {
        Self {
            check_interval: Duration::from_secs(check_interval),
            timeout: Duration::from_secs(timeout),
            failure_threshold,
            success_threshold,
            enabled,
        }
    }

    #[staticmethod]
    fn default_config() -> Self {
        Self::default()
    }

    #[getter]
    fn check_interval(&self) -> u64 {
        self.check_interval.as_secs()
    }

    #[setter]
    fn set_check_interval(&mut self, value: u64) {
        self.check_interval = Duration::from_secs(value);
    }

    #[getter]
    fn timeout(&self) -> u64 {
        self.timeout.as_secs()
    }

    #[setter]
    fn set_timeout(&mut self, value: u64) {
        self.timeout = Duration::from_secs(value);
    }

    #[getter]
    fn failure_threshold(&self) -> u32 {
        self.failure_threshold
    }

    #[getter]
    fn success_threshold(&self) -> u32 {
        self.success_threshold
    }

    #[getter]
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn __repr__(&self) -> String {
        format!("RiHealthCheckConfig {{ check_interval: {}, timeout: {}, failure_threshold: {}, success_threshold: {}, enabled: {} }}",
            self.check_interval.as_secs(), self.timeout.as_secs(), self.failure_threshold, self.success_threshold, self.enabled)
    }
}

/// Trait for implementing custom health checks.
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns the result.
    async fn check(&self) -> RiResult<RiHealthCheckResult>;

    /// Returns the name of this health check.
    fn name(&self) -> &str;

    /// Returns the configuration for this health check.
    fn config(&self) -> &RiHealthCheckConfig;
}

/// Comprehensive health report containing status of all components.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiHealthReport {
    /// Overall system health status
    pub overall_status: RiHealthStatus,
    /// Individual component health results
    pub components: FxHashMap<String, RiHealthCheckResult>,
    /// Timestamp when the report was generated
    pub timestamp: SystemTime,
    /// Total number of components checked
    pub total_components: usize,
    /// Number of healthy components
    pub healthy_count: usize,
    /// Number of degraded components
    pub degraded_count: usize,
    /// Number of unhealthy components
    pub unhealthy_count: usize,
    /// Number of unknown components
    pub unknown_count: usize,
}

impl RiHealthReport {
    /// Creates a new empty health report.
    pub fn new() -> Self {
        Self {
            overall_status: RiHealthStatus::Unknown,
            components: FxHashMap::default(),
            timestamp: SystemTime::now(),
            total_components: 0,
            healthy_count: 0,
            degraded_count: 0,
            unhealthy_count: 0,
            unknown_count: 0,
        }
    }

    /// Adds a health check result to the report.
    pub fn add_result(&mut self, result: RiHealthCheckResult) {
        match result.status {
            RiHealthStatus::Healthy => self.healthy_count += 1,
            RiHealthStatus::Degraded => self.degraded_count += 1,
            RiHealthStatus::Unhealthy => self.unhealthy_count += 1,
            RiHealthStatus::Unknown => self.unknown_count += 1,
        }
        self.total_components += 1;
        self.components.insert(result.name.clone(), result);
        self.update_overall_status();
    }

    /// Updates the overall health status based on component statuses.
    fn update_overall_status(&mut self) {
        let statuses: Vec<RiHealthStatus> = self.components.values().map(|r| r.status).collect();
        self.overall_status = RiHealthStatus::merge(&statuses);
    }
}

impl Default for RiHealthReport {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiHealthReport {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }

    #[staticmethod]
    fn create() -> Self {
        Self::new()
    }

    #[staticmethod]
    fn from_results(results: Vec<RiHealthCheckResult>) -> Self {
        let mut report = Self::new();
        for result in results {
            report.add_result(result);
        }
        report
    }

    #[getter]
    fn overall_status(&self) -> RiHealthStatus {
        self.overall_status
    }

    #[getter]
    fn total_components(&self) -> usize {
        self.total_components
    }

    #[getter]
    fn healthy_count(&self) -> usize {
        self.healthy_count
    }

    #[getter]
    fn degraded_count(&self) -> usize {
        self.degraded_count
    }

    #[getter]
    fn unhealthy_count(&self) -> usize {
        self.unhealthy_count
    }

    #[getter]
    fn unknown_count(&self) -> usize {
        self.unknown_count
    }

    fn __str__(&self) -> String {
        format!("RiHealthReport: {} ({}/{} healthy, {} degraded, {} unhealthy, {} unknown)",
            self.overall_status, self.healthy_count, self.total_components,
            self.degraded_count, self.unhealthy_count, self.unknown_count)
    }

    fn __repr__(&self) -> String {
        format!("RiHealthReport {{ overall_status: {:?}, total_components: {} }}", self.overall_status, self.total_components)
    }
}

/// Health checker service that manages and executes health checks.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiHealthChecker {
    /// Registered health checks
    checks: Vec<Box<dyn HealthCheck>>,
    /// Global configuration
    _config: RiHealthCheckConfig,
}

impl RiHealthChecker {
    /// Creates a new health checker with default configuration.
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            _config: RiHealthCheckConfig::default(),
        }
    }

    /// Creates a new health checker with custom configuration.
    pub fn with_config(config: RiHealthCheckConfig) -> Self {
        Self {
            checks: Vec::new(),
            _config: config,
        }
    }

    /// Registers a health check.
    pub fn register_check(&mut self, check: Box<dyn HealthCheck>) {
        self.checks.push(check);
    }

    /// Performs all health checks and returns a comprehensive report.
    pub async fn check_all(&self) -> RiHealthReport {
        let mut report = RiHealthReport::new();

        for check in &self.checks {
            if !check.config().enabled {
                continue;
            }

            let start_time = SystemTime::now();
            let result = match tokio::time::timeout(check.config().timeout, check.check()).await {
                Ok(Ok(result)) => result,
                Ok(Err(err)) => RiHealthCheckResult::unknown(
                    check.name().to_string(),
                    Some(format!("Check failed: {err}")),
                ),
                Err(_) => RiHealthCheckResult::unknown(
                    check.name().to_string(),
                    Some("Check timed out".to_string()),
                ),
            };

            let duration = SystemTime::now()
                .duration_since(start_time)
                .unwrap_or(Duration::ZERO);

            let mut result_with_duration = result;
            result_with_duration.duration = duration;
            report.add_result(result_with_duration);
        }

        report
    }

    /// Gets the number of registered health checks.
    pub fn check_count(&self) -> usize {
        self.checks.len()
    }
}

impl Default for RiHealthChecker {
    fn default() -> Self {
        Self::new()
    }
}


