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

//! # Global System Integration Module
//! 
//! This module provides the integration layer between the global system and
//! private communication protocols. It implements the coordination mechanisms
//! that enable seamless interaction between different protocol implementations
//! while maintaining security and performance requirements.

// Re-export submodules
mod core;
mod config;
mod connection;
mod security;
mod performance;
mod events;

// Re-export public API
pub use self::core::{RiGlobalSystemIntegration, RiControlCenter, RiExternalControlAction, RiExternalControlResult};
pub use self::config::{RiIntegrationConfig, RiSecurityEnforcementLevel};
pub use self::connection::{RiCrossProtocolConnection, RiCrossProtocolConnectionState, 
                           RiConnectionRoutingTable, RiRoutingEntry, RiRoutingPolicy, 
                           RiRoutingCondition, RiRoutingAction, RiPerformanceCondition, 
                           RiDeviceType};
pub use self::security::{RiEnforcementRule, RiEnforcementCondition, RiEnforcementAction, 
                         RiEnforcementRuleStatus, RiThreatCondition, RiThreatLevel, 
                         RiThreatType, RiSecurityEvent, RiSecurityEventType, 
                         RiSecurityEventSeverity, RiEnforcementStats, RiSecurityEventStats};
pub use self::performance::{RiPerformanceMetrics, RiProtocolPerformanceMetrics, 
                           RiCrossProtocolMetrics, RiSystemPerformanceMetrics, 
                           RiPerformanceOptimization, RiPerformanceOptimizationType, 
                           RiImplementationStatus, RiPerformanceMonitoringConfig, 
                           RiPerformanceThresholds, RiPerformanceAlertConfig, 
                           RiAlertSeverityLevel, RiPerformanceMonitoringResult, 
                           RiPerformanceThresholdViolation, RiPerformanceAlert, 
                           RiPerformanceAlertType};
pub use self::events::{RiIntegrationEvent, RiIntegrationEventType, RiIntegrationEventStats, 
                       RiIntegrationStats};
