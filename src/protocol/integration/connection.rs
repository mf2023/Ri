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

use crate::core::{RiResult};
use super::super::{RiProtocolType};

/// Connection coordinator for managing cross-protocol connections.
pub struct RiConnectionCoordinator {
    /// Active cross-protocol connections
    pub connections: Arc<RwLock<FxHashMap<String, RiCrossProtocolConnection>>>,
    /// Connection routing table
    pub routing_table: Arc<RwLock<RiConnectionRoutingTable>>,
    /// Connection health monitor
    pub health_monitor: Arc<RiConnectionHealthMonitor>,
}

/// Cross-protocol connection structure.
#[derive(Debug, Clone)]
pub struct RiCrossProtocolConnection {
    /// Connection identifier
    pub connection_id: String,
    /// Source protocol
    pub source_protocol: RiProtocolType,
    /// Target protocol
    pub target_protocol: RiProtocolType,
    /// Target device
    pub target_device: String,
    /// Connection state
    pub state: RiCrossProtocolConnectionState,
    /// Connection metadata
    pub metadata: FxHashMap<String, String>,
    /// Established timestamp
    pub established_at: Instant,
    /// Last activity
    pub last_activity: Instant,
}

/// Cross-protocol connection state enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiCrossProtocolConnectionState {
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
pub struct RiConnectionRoutingTable {
    /// Protocol routing entries
    pub entries: FxHashMap<String, RiRoutingEntry>,
    /// Default protocol
    pub default_protocol: RiProtocolType,
    /// Routing policies
    pub routing_policies: Vec<RiRoutingPolicy>,
}

/// Routing entry structure.
#[derive(Debug, Clone)]
pub struct RiRoutingEntry {
    /// Target device
    pub target_device: String,
    /// Preferred protocol
    pub preferred_protocol: RiProtocolType,
    /// Alternative protocols
    pub alternative_protocols: Vec<RiProtocolType>,
    /// Routing priority
    pub priority: u32,
    /// Route cost
    pub cost: u32,
}

/// Routing policy structure.
#[derive(Debug, Clone)]
pub struct RiRoutingPolicy {
    /// Policy name
    pub name: String,
    /// Policy condition
    pub condition: RiRoutingCondition,
    /// Policy action
    pub action: RiRoutingAction,
    /// Policy priority
    pub priority: u32,
}

/// Routing condition enumeration.
#[derive(Debug, Clone)]
pub enum RiRoutingCondition {
    /// Device type condition
    DeviceType(RiDeviceType),
    /// Protocol availability condition
    ProtocolAvailability(RiProtocolType),
    /// Security level condition
    SecurityLevel(super::super::RiSecurityLevel),
    /// Performance condition
    Performance(RiPerformanceCondition),
    /// Custom condition
    Custom(String),
}

/// Routing action enumeration.
#[derive(Debug, Clone)]
pub enum RiRoutingAction {
    /// Use protocol
    UseProtocol(RiProtocolType),
    /// Load balance
    LoadBalance(Vec<RiProtocolType>),
    /// Failover
    Failover(Vec<RiProtocolType>),
    /// Block connection
    Block,
    /// Custom action
    Custom(String),
}

/// Performance condition structure.
#[derive(Debug, Clone)]
pub struct RiPerformanceCondition {
    /// Maximum latency
    pub max_latency: Duration,
    /// Minimum throughput
    pub min_throughput: u64,
    /// Maximum error rate
    pub max_error_rate: f32,
}

/// Device type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiDeviceType {
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
pub struct RiConnectionHealthMonitor {
    /// Health check results
    pub health_results: Arc<RwLock<FxHashMap<String, RiConnectionHealthResult>>>,
    /// Health check configuration
    pub config: Arc<RiHealthCheckConfig>,
}

/// Connection health result structure.
#[derive(Debug, Clone)]
pub struct RiConnectionHealthResult {
    /// Connection identifier
    pub connection_id: String,
    /// Health status
    pub health_status: RiHealthStatus,
    /// Response time
    pub response_time: Duration,
    /// Error count
    pub error_count: u32,
    /// Last check time
    pub last_check: Instant,
    /// Health metrics
    pub metrics: FxHashMap<String, f64>,
}

/// Health status enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiHealthStatus {
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
pub struct RiHealthCheckConfig {
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
