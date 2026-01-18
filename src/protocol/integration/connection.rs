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

use crate::core::{DMSCResult};
use super::super::{DMSCProtocolType};

/// Connection coordinator for managing cross-protocol connections.
pub struct DMSCConnectionCoordinator {
    /// Active cross-protocol connections
    pub connections: Arc<RwLock<HashMap<String, DMSCCrossProtocolConnection>>>,
    /// Connection routing table
    pub routing_table: Arc<RwLock<DMSCConnectionRoutingTable>>,
    /// Connection health monitor
    pub health_monitor: Arc<DMSCConnectionHealthMonitor>,
}

/// Cross-protocol connection structure.
#[derive(Debug, Clone)]
pub struct DMSCCrossProtocolConnection {
    /// Connection identifier
    pub connection_id: String,
    /// Source protocol
    pub source_protocol: DMSCProtocolType,
    /// Target protocol
    pub target_protocol: DMSCProtocolType,
    /// Target device
    pub target_device: String,
    /// Connection state
    pub state: DMSCCrossProtocolConnectionState,
    /// Connection metadata
    pub metadata: HashMap<String, String>,
    /// Established timestamp
    pub established_at: Instant,
    /// Last activity
    pub last_activity: Instant,
}

/// Cross-protocol connection state enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCCrossProtocolConnectionState {
    /// Connection is initializing
    Initializing,
    /// Connection is established
    Established,
    /// Connection is active
    Active,
    /// Connection is degraded
    Degraded,
    /// Connection is failed
    Failed,
    /// Connection is closing
    Closing,
    /// Connection is closed
    Closed,
}

/// Connection routing table structure.
#[derive(Debug, Clone)]
pub struct DMSCConnectionRoutingTable {
    /// Protocol routing entries
    pub entries: HashMap<String, DMSCRoutingEntry>,
    /// Default protocol
    pub default_protocol: DMSCProtocolType,
    /// Routing policies
    pub routing_policies: Vec<DMSCRoutingPolicy>,
}

/// Routing entry structure.
#[derive(Debug, Clone)]
pub struct DMSCRoutingEntry {
    /// Target device
    pub target_device: String,
    /// Preferred protocol
    pub preferred_protocol: DMSCProtocolType,
    /// Alternative protocols
    pub alternative_protocols: Vec<DMSCProtocolType>,
    /// Routing priority
    pub priority: u32,
    /// Route cost
    pub cost: u32,
}

/// Routing policy structure.
#[derive(Debug, Clone)]
pub struct DMSCRoutingPolicy {
    /// Policy name
    pub name: String,
    /// Policy condition
    pub condition: DMSCRoutingCondition,
    /// Policy action
    pub action: DMSCRoutingAction,
    /// Policy priority
    pub priority: u32,
}

/// Routing condition enumeration.
#[derive(Debug, Clone)]
pub enum DMSCRoutingCondition {
    /// Device type condition
    DeviceType(DMSCDeviceType),
    /// Protocol availability condition
    ProtocolAvailability(DMSCProtocolType),
    /// Security level condition
    SecurityLevel(super::super::DMSCSecurityLevel),
    /// Performance condition
    Performance(DMSCPerformanceCondition),
    /// Custom condition
    Custom(String),
}

/// Routing action enumeration.
#[derive(Debug, Clone)]
pub enum DMSCRoutingAction {
    /// Use protocol
    UseProtocol(DMSCProtocolType),
    /// Load balance
    LoadBalance(Vec<DMSCProtocolType>),
    /// Failover
    Failover(Vec<DMSCProtocolType>),
    /// Block connection
    Block,
    /// Custom action
    Custom(String),
}

/// Performance condition structure.
#[derive(Debug, Clone)]
pub struct DMSCPerformanceCondition {
    /// Maximum latency
    pub max_latency: Duration,
    /// Minimum throughput
    pub min_throughput: u64,
    /// Maximum error rate
    pub max_error_rate: f32,
}

/// Device type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCDeviceType {
    /// Server device
    Server,
    /// Client device
    Client,
    /// Gateway device
    Gateway,
    /// IoT device
    IoT,
    /// Mobile device
    Mobile,
    /// Unknown device type
    Unknown,
}

/// Connection health monitor structure.
pub struct DMSCConnectionHealthMonitor {
    /// Health check results
    pub health_results: Arc<RwLock<HashMap<String, DMSCConnectionHealthResult>>>,
    /// Health check configuration
    pub config: Arc<DMSCHealthCheckConfig>,
}

/// Connection health result structure.
#[derive(Debug, Clone)]
pub struct DMSCConnectionHealthResult {
    /// Connection identifier
    pub connection_id: String,
    /// Health status
    pub health_status: DMSCHealthStatus,
    /// Response time
    pub response_time: Duration,
    /// Error count
    pub error_count: u32,
    /// Last check time
    pub last_check: Instant,
    /// Health metrics
    pub metrics: HashMap<String, f64>,
}

/// Health status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCHealthStatus {
    /// Healthy status
    Healthy,
    /// Degraded status
    Degraded,
    /// Unhealthy status
    Unhealthy,
    /// Unknown status
    Unknown,
}

/// Health check configuration structure.
#[derive(Debug, Clone)]
pub struct DMSCHealthCheckConfig {
    /// Check interval
    pub check_interval: Duration,
    /// Timeout duration
    pub timeout: Duration,
    /// Retry attempts
    pub retry_attempts: u32,
    /// Healthy threshold
    pub healthy_threshold: u32,
    /// Unhealthy threshold
    pub unhealthy_threshold: u32,
}
