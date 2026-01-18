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
pub use self::core::{DMSCGlobalSystemIntegration, DMSCControlCenter, DMSCExternalControlAction, DMSCExternalControlResult};
pub use self::config::{DMSCIntegrationConfig, DMSCSecurityEnforcementLevel};
pub use self::connection::{DMSCCrossProtocolConnection, DMSCCrossProtocolConnectionState, 
                           DMSCConnectionRoutingTable, DMSCRoutingEntry, DMSCRoutingPolicy, 
                           DMSCRoutingCondition, DMSCRoutingAction, DMSCPerformanceCondition, 
                           DMSCDeviceType};
pub use self::security::{DMSCEnforcementRule, DMSCEnforcementCondition, DMSCEnforcementAction, 
                         DMSCEnforcementRuleStatus, DMSCThreatCondition, DMSCThreatLevel, 
                         DMSCThreatType, DMSCSecurityEvent, DMSCSecurityEventType, 
                         DMSCSecurityEventSeverity, DMSCEnforcementStats, DMSCSecurityEventStats};
pub use self::performance::{DMSCPerformanceMetrics, DMSCProtocolPerformanceMetrics, 
                           DMSCCrossProtocolMetrics, DMSCSystemPerformanceMetrics, 
                           DMSCPerformanceOptimization, DMSCPerformanceOptimizationType, 
                           DMSCImplementationStatus, DMSCPerformanceMonitoringConfig, 
                           DMSCPerformanceThresholds, DMSCPerformanceAlertConfig, 
                           DMSCAlertSeverityLevel, DMSCPerformanceMonitoringResult, 
                           DMSCPerformanceThresholdViolation, DMSCPerformanceAlert, 
                           DMSCPerformanceAlertType};
pub use self::events::{DMSCIntegrationEvent, DMSCIntegrationEventType, DMSCIntegrationEventStats, 
                       DMSCIntegrationStats};
