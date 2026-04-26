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

use std::time::Duration;

/// Integration configuration structure.
#[derive(Debug, Clone)]
pub struct RiIntegrationConfig {
    /// Enable protocol coordination
    pub enable_protocol_coordination: bool,
    /// Enable state synchronization
    pub enable_state_sync: bool,
    /// Security enforcement level
    pub security_enforcement_level: RiSecurityEnforcementLevel,
    /// Performance optimization enabled
    pub performance_optimization: bool,
    /// Fault tolerance enabled
    pub fault_tolerance: bool,
    /// Cross-protocol timeout
    pub cross_protocol_timeout: Duration,
    /// State sync interval
    pub state_sync_interval: Duration,
    /// Health check interval
    pub health_check_interval: Duration,
    /// Maximum retry attempts
    pub max_retry_attempts: u32,
}

/// Security enforcement level enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiSecurityEnforcementLevel {
    /// No security enforcement
    None,
    /// Basic security enforcement
    Basic,
    /// Standard security enforcement
    Standard,
    /// High security enforcement
    High,
    /// Maximum security enforcement
    Maximum,
}

impl Default for RiIntegrationConfig {
    fn default() -> Self {
        Self {
            enable_protocol_coordination: true,
            enable_state_sync: true,
            security_enforcement_level: RiSecurityEnforcementLevel::Standard,
            performance_optimization: true,
            fault_tolerance: true,
            cross_protocol_timeout: Duration::from_secs(30),
            state_sync_interval: Duration::from_secs(60),
            health_check_interval: Duration::from_secs(30),
            max_retry_attempts: 3,
        }
    }
}
