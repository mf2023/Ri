// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dms::core::health::*;
use dms::core::{DMSCResult, DMSCError};
use async_trait::async_trait;
use std::time::{Duration, SystemTime};

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