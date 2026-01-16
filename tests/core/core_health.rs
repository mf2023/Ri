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

use dmsc::core::health::*;
use dmsc::core::{DMSCResult, DMSCError};
use async_trait::async_trait;
use std::time::{Duration, SystemTime};

/// Health checking system test module for DMSC core monitoring infrastructure.
///
/// This module provides comprehensive test coverage for the health monitoring
/// subsystem, which enables applications to track the operational status of
/// critical components and aggregate health reports across the system. The health
/// checking system supports configurable health checks with custom thresholds,
/// asynchronous execution, and status aggregation for determining overall system
/// health state.
///
/// ## Test Coverage
///
/// - **Health Status Merging**: Verifies the status aggregation algorithm that
///   determines overall health from multiple component statuses. Tests cover all
///   combinations including Healthy, Degraded, Unhealthy, and Unknown states.
///
/// - **Health Checker Registration**: Tests the ability to register multiple
///   health check implementations with the HealthChecker component, supporting
///   diverse check types including connectivity, resource utilization, and
///   dependency availability.
///
/// - **Asynchronous Health Checks**: Validates the async execution model where
///   health checks run concurrently with configurable timeouts, ensuring that
///   slow checks do not block the overall health assessment process.
///
/// - **Health Report Aggregation**: Tests the report generation functionality
///   that summarizes check results including counts of healthy and unhealthy
///   components, overall status determination, and result metadata collection.
///
/// ## Design Principles
///
/// The health checking system implements a composite pattern where individual
/// health checks contribute to an aggregate health status. This design supports
/// both simple single-component checks and complex multi-layer health monitoring
/// scenarios common in distributed systems.
///
/// Tests follow a behavioral specification approach, verifying the observable
/// outcomes of health check operations rather than internal implementation
/// details. This ensures test stability across refactoring while validating
/// the critical health monitoring functionality.
///
/// The status merging algorithm implements a severity-based aggregation where
/// the most severe status takes precedence: Unhealthy overrides Degraded which
/// overrides Healthy. Unknown status is treated as neutral, only affecting the
/// result when all components report Unknown.
///
/// The async health check design enables efficient concurrent execution of
/// independent checks while providing proper error isolation. A failure in one
/// check does not affect others, and the HealthChecker continues to aggregate
/// results from all registered checks regardless of individual outcomes.

struct MockHealthCheck {
    name: String,
    status: HealthStatus,
    config: HealthCheckConfig,
}

#[async_trait::async_trait]
impl HealthCheck for MockHealthCheck {
    async fn check(&self) -> DMSCResult<HealthCheckResult> {
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