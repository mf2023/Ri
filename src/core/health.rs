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

//! # Health Check System
//!
//! This module provides comprehensive health checking functionality for DMS modules and services.
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

use crate::core::DMSResult;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

/// Health status enumeration representing the state of a component or service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthStatus {
    /// Component is functioning normally
    Healthy,
    /// Component is experiencing issues but still operational
    Degraded,
    /// Component is not functioning and requires attention
    Unhealthy,
    /// Health status is unknown (check failed or not performed)
    Unknown,
}

impl HealthStatus {
    /// Returns true if the status is considered healthy (Healthy or Degraded).
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy | HealthStatus::Degraded)
    }

    /// Returns true if the status requires immediate attention.
    pub fn requires_attention(&self) -> bool {
        matches!(self, HealthStatus::Unhealthy)
    }

    /// Merges multiple health statuses into a single status.
    /// The most severe status takes precedence: Unhealthy > Degraded > Unknown > Healthy
    pub fn merge(statuses: &[HealthStatus]) -> HealthStatus {
        if statuses.is_empty() {
            return HealthStatus::Unknown;
        }

        let mut has_unhealthy = false;
        let mut has_degraded = false;
        let mut has_unknown = false;

        for status in statuses {
            match status {
                HealthStatus::Unhealthy => has_unhealthy = true,
                HealthStatus::Degraded => has_degraded = true,
                HealthStatus::Unknown => has_unknown = true,
                HealthStatus::Healthy => {}
            }
        }

        if has_unhealthy {
            HealthStatus::Unhealthy
        } else if has_degraded {
            HealthStatus::Degraded
        } else if has_unknown {
            HealthStatus::Unknown
        } else {
            HealthStatus::Healthy
        }
    }
}

impl std::fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HealthStatus::Healthy => write!(f, "healthy"),
            HealthStatus::Degraded => write!(f, "degraded"),
            HealthStatus::Unhealthy => write!(f, "unhealthy"),
            HealthStatus::Unknown => write!(f, "unknown"),
        }
    }
}

/// Result of a health check execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckResult {
    /// Name of the health check
    pub name: String,
    /// Health status
    pub status: HealthStatus,
    /// Optional message providing additional context
    pub message: Option<String>,
    /// Timestamp when the check was performed
    pub timestamp: SystemTime,
    /// Duration of the health check execution
    pub duration: Duration,
}

impl HealthCheckResult {
    /// Creates a new successful health check result.
    pub fn healthy(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Healthy,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new degraded health check result.
    pub fn degraded(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Degraded,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new unhealthy health check result.
    pub fn unhealthy(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Unhealthy,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }

    /// Creates a new unknown health check result.
    pub fn unknown(name: String, message: Option<String>) -> Self {
        Self {
            name,
            status: HealthStatus::Unknown,
            message,
            timestamp: SystemTime::now(),
            duration: Duration::ZERO,
        }
    }
}

/// Configuration for health checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
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

impl Default for HealthCheckConfig {
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

/// Trait for implementing custom health checks.
#[async_trait::async_trait]
pub trait HealthCheck: Send + Sync {
    /// Performs the health check and returns the result.
    async fn check(&self) -> DMSResult<HealthCheckResult>;

    /// Returns the name of this health check.
    fn name(&self) -> &str;

    /// Returns the configuration for this health check.
    fn config(&self) -> &HealthCheckConfig;
}

/// Comprehensive health report containing status of all components.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    /// Overall system health status
    pub overall_status: HealthStatus,
    /// Individual component health results
    pub components: HashMap<String, HealthCheckResult>,
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

impl HealthReport {
    /// Creates a new empty health report.
    pub fn new() -> Self {
        Self {
            overall_status: HealthStatus::Unknown,
            components: HashMap::new(),
            timestamp: SystemTime::now(),
            total_components: 0,
            healthy_count: 0,
            degraded_count: 0,
            unhealthy_count: 0,
            unknown_count: 0,
        }
    }

    /// Adds a health check result to the report.
    pub fn add_result(&mut self, result: HealthCheckResult) {
        match result.status {
            HealthStatus::Healthy => self.healthy_count += 1,
            HealthStatus::Degraded => self.degraded_count += 1,
            HealthStatus::Unhealthy => self.unhealthy_count += 1,
            HealthStatus::Unknown => self.unknown_count += 1,
        }
        self.total_components += 1;
        self.components.insert(result.name.clone(), result);
        self.update_overall_status();
    }

    /// Updates the overall health status based on component statuses.
    fn update_overall_status(&mut self) {
        let statuses: Vec<HealthStatus> = self.components.values().map(|r| r.status).collect();
        self.overall_status = HealthStatus::merge(&statuses);
    }
}

impl Default for HealthReport {
    fn default() -> Self {
        Self::new()
    }
}

/// Health checker service that manages and executes health checks.
pub struct HealthChecker {
    /// Registered health checks
    checks: Vec<Box<dyn HealthCheck>>,
    /// Global configuration
    _config: HealthCheckConfig,
}

impl HealthChecker {
    /// Creates a new health checker with default configuration.
    pub fn new() -> Self {
        Self {
            checks: Vec::new(),
            _config: HealthCheckConfig::default(),
        }
    }

    /// Creates a new health checker with custom configuration.
    pub fn with_config(config: HealthCheckConfig) -> Self {
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
    pub async fn check_all(&self) -> HealthReport {
        let mut report = HealthReport::new();

        for check in &self.checks {
            if !check.config().enabled {
                continue;
            }

            let start_time = SystemTime::now();
            let result = match tokio::time::timeout(check.config().timeout, check.check()).await {
                Ok(Ok(result)) => result,
                Ok(Err(err)) => HealthCheckResult::unknown(
                    check.name().to_string(),
                    Some(format!("Check failed: {err}")),
                ),
                Err(_) => HealthCheckResult::unknown(
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

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockHealthCheck {
        name: String,
        status: HealthStatus,
        config: HealthCheckConfig,
    }

    #[async_trait::async_trait]
    impl HealthCheck for MockHealthCheck {
        async fn check(&self) -> DMSResult<HealthCheckResult> {
            Ok(HealthCheckResult {
                name: self.name.clone(),
                status: self.status,
                message: None,
                timestamp: SystemTime::now(),
                duration: Duration::from_millis(100),
            })
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn config(&self) -> &HealthCheckConfig {
            &self.config
        }
    }

    #[tokio::test]
    async fn test_health_status_merge() {
        assert_eq!(HealthStatus::merge(&[]), HealthStatus::Unknown);
        assert_eq!(HealthStatus::merge(&[HealthStatus::Healthy]), HealthStatus::Healthy);
        assert_eq!(HealthStatus::merge(&[HealthStatus::Healthy, HealthStatus::Degraded]), HealthStatus::Degraded);
        assert_eq!(HealthStatus::merge(&[HealthStatus::Healthy, HealthStatus::Unhealthy]), HealthStatus::Unhealthy);
        assert_eq!(HealthStatus::merge(&[HealthStatus::Unknown, HealthStatus::Healthy]), HealthStatus::Unknown);
    }

    #[tokio::test]
    async fn test_health_checker() {
        let mut checker = HealthChecker::new();
        
        checker.register_check(Box::new(MockHealthCheck {
            name: "test_healthy".to_string(),
            status: HealthStatus::Healthy,
            config: HealthCheckConfig::default(),
        }));

        checker.register_check(Box::new(MockHealthCheck {
            name: "test_unhealthy".to_string(),
            status: HealthStatus::Unhealthy,
            config: HealthCheckConfig::default(),
        }));

        let report = checker.check_all().await;
        assert_eq!(report.total_components, 2);
        assert_eq!(report.healthy_count, 1);
        assert_eq!(report.unhealthy_count, 1);
        assert_eq!(report.overall_status, HealthStatus::Unhealthy);
    }
}
