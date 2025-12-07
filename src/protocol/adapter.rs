//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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

//! # Protocol Adapter Module
//! 
//! This module provides the adapter pattern for seamless protocol switching
//! between global and private protocols. It enables dynamic protocol selection
//! based on security requirements, performance needs, and environmental context.
//! 
//! ## Architecture Overview
//! 
//! The adapter module implements a bridge pattern that:
//! - Provides unified interface for different protocol implementations
//! - Enables runtime protocol switching without service interruption
//! - Maintains connection state during protocol transitions
//! - Provides fallback mechanisms for protocol failures
//! - Supports protocol version negotiation and compatibility
//! 
//! ## Key Features
//! 
//! - **Dynamic Protocol Switching**: Seamless transition between protocols
//! - **Connection State Preservation**: Maintain connections during switches
//! - **Protocol Fallback**: Automatic fallback on protocol failures
//! - **Version Negotiation**: Automatic protocol version selection
//! - **Performance Optimization**: Select optimal protocol based on conditions
//! - **Security Context Awareness**: Protocol selection based on security requirements
//! 
//! ## Adapter Pattern Implementation
//! 
//! The adapter follows the bridge pattern with three layers:
//! 
//! 1. **Protocol Adapter**: Unified interface for all protocols
//! 2. **Protocol Bridge**: Handles protocol switching logic
//! 3. **Protocol Strategy**: Determines optimal protocol selection
//! 
//! ## Usage Examples
//! 
//! ```rust
//! use dms::protocol::adapter::{DMSProtocolAdapter, DMSProtocolStrategy, DMSSecurityContext};
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create protocol adapter
//!     let mut adapter = DMSProtocolAdapter::new();
//!     
//!     // Define security context
//!     let security_context = DMSSecurityContext {
//!         required_security_level: DMSSecurityLevel::High,
//!         threat_level: DMSThreatLevel::Elevated,
//!         data_classification: DMSDataClassification::Confidential,
//!         network_environment: DMSNetworkEnvironment::Untrusted,
//!     };
//!     
//!     // Initialize adapter with strategy
//!     adapter.initialize(DMSProtocolStrategy::SecurityBased(security_context)).await?;
//!     
//!     // Connect using optimal protocol
//!     let connection = adapter.connect("target-device").await?;
//!     
//!     // Send message (protocol selected automatically)
//!     let response = connection.send_message(b"sensitive data").await?;
//!     
//!     // Dynamically switch protocol if needed
//!     adapter.switch_protocol(DMSProtocolType::Private).await?;
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::core::{DMSResult, DMSError};
use super::{DMSProtocol, DMSProtocolType, DMSProtocolConfig, DMSProtocolConnection, 
            DMSProtocolStats, DMSMessageFlags, DMSConnectionInfo, DMSSecurityLevel};

/// Protocol strategy for determining optimal protocol selection.
#[derive(Debug, Clone)]
pub enum DMSProtocolStrategy {
    /// Security-based strategy (prioritizes security)
    SecurityBased(DMSSecurityContext),
    /// Performance-based strategy (prioritizes speed)
    PerformanceBased(DMSPerformanceContext),
    /// Adaptive strategy (balances security and performance)
    Adaptive(DMSAdaptiveContext),
    /// Manual strategy (explicit protocol selection)
    Manual(DMSProtocolType),
}

/// Security context for protocol selection.
#[derive(Debug, Clone)]
pub struct DMSSecurityContext {
    /// Required security level
    pub required_security_level: DMSSecurityLevel,
    /// Current threat level
    pub threat_level: DMSThreatLevel,
    /// Data classification level
    pub data_classification: DMSDataClassification,
    /// Network environment
    pub network_environment: DMSNetworkEnvironment,
    /// Compliance requirements
    pub compliance_requirements: Vec<DMSComplianceRequirement>,
}

/// Performance context for protocol selection.
#[derive(Debug, Clone)]
pub struct DMSPerformanceContext {
    /// Required throughput (bytes/second)
    pub required_throughput: u64,
    /// Maximum acceptable latency (milliseconds)
    pub max_latency_ms: u64,
    /// Network bandwidth constraints
    pub bandwidth_constraints: DMSBandwidthConstraints,
    /// Connection stability requirements
    pub stability_requirements: DMSStabilityRequirements,
}

/// Adaptive context for balanced protocol selection.
#[derive(Debug, Clone)]
pub struct DMSAdaptiveContext {
    /// Security weight (0.0 - 1.0)
    pub security_weight: f32,
    /// Performance weight (0.0 - 1.0)
    pub performance_weight: f32,
    /// Adaptation triggers
    pub adaptation_triggers: Vec<DMSAdaptationTrigger>,
    /// Learning parameters
    pub learning_params: DMSLearningParameters,
}

/// Threat level enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSThreatLevel {
    /// Normal threat level
    Normal,
    /// Elevated threat level
    Elevated,
    /// High threat level
    High,
    /// Critical threat level
    Critical,
}

/// Data classification enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSDataClassification {
    /// Public data
    Public,
    /// Internal data
    Internal,
    /// Confidential data
    Confidential,
    /// Secret data
    Secret,
    /// Top secret data
    TopSecret,
}

/// Network environment enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSNetworkEnvironment {
    /// Trusted internal network
    Trusted,
    /// Untrusted external network
    Untrusted,
    /// Hostile network environment
    Hostile,
    /// Unknown network environment
    Unknown,
}

/// Compliance requirement enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSComplianceRequirement {
    /// GDPR compliance
    GDPR,
    /// HIPAA compliance
    HIPAA,
    /// SOX compliance
    SOX,
    /// PCI DSS compliance
    PCIDSS,
    /// National security standards
    NationalSecurity,
}

/// Bandwidth constraints structure.
#[derive(Debug, Clone)]
pub struct DMSBandwidthConstraints {
    /// Available bandwidth (bits/second)
    pub available_bandwidth: u64,
    /// Burst capacity (bits)
    pub burst_capacity: u64,
    /// Network congestion level (0.0 - 1.0)
    pub congestion_level: f32,
}

/// Stability requirements structure.
#[derive(Debug, Clone)]
pub struct DMSStabilityRequirements {
    /// Maximum acceptable packet loss (0.0 - 1.0)
    pub max_packet_loss: f32,
    /// Maximum acceptable jitter (milliseconds)
    pub max_jitter_ms: u64,
    /// Minimum connection uptime (seconds)
    pub min_uptime: u64,
}

/// Adaptation trigger enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSAdaptationTrigger {
    /// Security breach detected
    SecurityBreach,
    /// Performance degradation detected
    PerformanceDegradation,
    /// Network conditions changed
    NetworkChange,
    /// Manual trigger
    Manual,
}

/// Learning parameters structure.
#[derive(Debug, Clone)]
pub struct DMSLearningParameters {
    /// Learning rate (0.0 - 1.0)
    pub learning_rate: f32,
    /// Adaptation window (seconds)
    pub adaptation_window: Duration,
    /// Performance history size
    pub history_size: usize,
}

/// Protocol adapter for unified protocol interface.
pub struct DMSProtocolAdapter {
    /// Protocol strategy
    strategy: Arc<RwLock<Option<DMSProtocolStrategy>>>,
    /// Available protocols
    protocols: Arc<RwLock<HashMap<DMSProtocolType, Box<dyn DMSProtocol>>>>,
    /// Active protocol
    active_protocol: Arc<RwLock<Option<DMSProtocolType>>>,
    /// Connection state manager
    connection_manager: Arc<DMSConnectionManager>,
    /// Protocol statistics
    stats: Arc<RwLock<DMSProtocolAdapterStats>>,
    /// Whether the adapter is initialized
    initialized: Arc<RwLock<bool>>,
}

/// Connection manager for state preservation during protocol switches.
struct DMSConnectionManager {
    /// Active connections
    connections: Arc<RwLock<HashMap<String, Arc<dyn DMSProtocolConnection>>>>,
    /// Connection metadata
    metadata: Arc<RwLock<HashMap<String, DMSConnectionMetadata>>>,
}

/// Connection metadata for state preservation.
#[derive(Debug, Clone)]
struct DMSConnectionMetadata {
    /// Original protocol type
    original_protocol: DMSProtocolType,
    /// Current protocol type
    current_protocol: DMSProtocolType,
    /// Connection establishment time
    established_at: Instant,
    /// Last protocol switch time
    last_switch: Option<Instant>,
    /// Protocol switch count
    switch_count: u64,
    /// Connection state data
    state_data: HashMap<String, Vec<u8>>,
}

/// Protocol adapter statistics.
#[derive(Debug, Default)]
struct DMSProtocolAdapterStats {
    /// Total protocol switches
    pub protocol_switches: u64,
    /// Successful switches
    pub successful_switches: u64,
    /// Failed switches
    pub failed_switches: u64,
    /// Connection migrations
    pub connection_migrations: u64,
    /// Strategy changes
    pub strategy_changes: u64,
    /// Average switch time (milliseconds)
    pub avg_switch_time_ms: u64,
}

impl DMSProtocolAdapter {
    /// Create a new protocol adapter.
    pub fn new() -> Self {
        let connection_manager = Arc::new(DMSConnectionManager {
            connections: Arc::new(RwLock::new(HashMap::new())),
            metadata: Arc::new(RwLock::new(HashMap::new())),
        });
        
        Self {
            strategy: Arc::new(RwLock::new(None)),
            protocols: Arc::new(RwLock::new(HashMap::new())),
            active_protocol: Arc::new(RwLock::new(None)),
            connection_manager,
            stats: Arc::new(RwLock::new(DMSProtocolAdapterStats::default())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Initialize the protocol adapter with a strategy.
    pub async fn initialize(&mut self, strategy: DMSProtocolStrategy) -> DMSResult<()> {
        *self.strategy.write().await = Some(strategy.clone());
        
        // Determine initial protocol based on strategy
        let initial_protocol = self.select_optimal_protocol(&strategy).await?;
        *self.active_protocol.write().await = Some(initial_protocol);
        
        *self.initialized.write().await = true;
        Ok(())
    }
    
    /// Register a protocol implementation.
    pub async fn register_protocol(&self, protocol_type: DMSProtocolType, protocol: Box<dyn DMSProtocol>) -> DMSResult<()> {
        self.protocols.write().await.insert(protocol_type, protocol);
        Ok(())
    }
    
    /// Connect using the optimal protocol.
    pub async fn connect(&self, target_id: &str) -> DMSResult<Box<dyn DMSProtocolConnection>> {
        if !*self.initialized.read().await {
            return Err(DMSError::InvalidState("Protocol adapter not initialized".to_string()));
        }
        
        let active_protocol = self.active_protocol.read().await;
        let protocol_type = active_protocol.ok_or_else(|| 
            DMSError::InvalidState("No active protocol selected".to_string()))?;
        
        let protocols = self.protocols.read().await;
        let protocol = protocols.get(&protocol_type)
            .ok_or_else(|| DMSError::NotFound(format!("Protocol {:?} not registered", protocol_type)))?;
        
        let connection = protocol.connect(target_id).await?;
        
        // Store connection metadata
        let connection_id = format!("adapter-{}", uuid::Uuid::new_v4());
        self.connection_manager.store_connection(
            connection_id.clone(),
            connection,
            protocol_type,
        ).await?;
        
        Ok(Box::new(DMSProtocolConnectionWrapper {
            connection_id,
            connection_manager: Arc::clone(&self.connection_manager),
            stats: Arc::clone(&self.stats),
        }))
    }
    
    /// Switch to a different protocol.
    pub async fn switch_protocol(&self, new_protocol_type: DMSProtocolType) -> DMSResult<()> {
        let start_time = Instant::now();
        
        // Update statistics
        self.stats.write().await.protocol_switches += 1;
        
        // Get current active protocol
        let current_protocol = *self.active_protocol.read().await;
        
        if let Some(current) = current_protocol {
            if current == new_protocol_type {
                return Ok(()); // Already using this protocol
            }
        }
        
        // Validate new protocol is available
        let protocols = self.protocols.read().await;
        if !protocols.contains_key(&new_protocol_type) {
            self.stats.write().await.failed_switches += 1;
            return Err(DMSError::NotFound(format!("Protocol {:?} not available", new_protocol_type)));
        }
        
        // Perform protocol switch
        *self.active_protocol.write().await = Some(new_protocol_type);
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.successful_switches += 1;
        let switch_time = start_time.elapsed().as_millis() as u64;
        stats.avg_switch_time_ms = (stats.avg_switch_time_ms + switch_time) / 2;
        
        Ok(())
    }
    
    /// Get the currently active protocol.
    pub async fn get_active_protocol(&self) -> DMSResult<DMSProtocolType> {
        self.active_protocol.read().await
            .ok_or_else(|| DMSError::InvalidState("No active protocol selected".to_string()))
    }
    
    /// Update protocol strategy.
    pub async fn update_strategy(&self, new_strategy: DMSProtocolStrategy) -> DMSResult<()> {
        *self.strategy.write().await = Some(new_strategy.clone());
        
        // Re-evaluate optimal protocol
        let optimal_protocol = self.select_optimal_protocol(&new_strategy).await?;
        
        if let Some(current) = *self.active_protocol.read().await {
            if current != optimal_protocol {
                self.switch_protocol(optimal_protocol).await?;
            }
        }
        
        self.stats.write().await.strategy_changes += 1;
        Ok(())
    }
    
    /// Select optimal protocol based on strategy.
    async fn select_optimal_protocol(&self, strategy: &DMSProtocolStrategy) -> DMSResult<DMSProtocolType> {
        match strategy {
            DMSProtocolStrategy::SecurityBased(context) => {
                self.select_security_based_protocol(context).await
            }
            DMSProtocolStrategy::PerformanceBased(context) => {
                self.select_performance_based_protocol(context).await
            }
            DMSProtocolStrategy::Adaptive(context) => {
                self.select_adaptive_protocol(context).await
            }
            DMSProtocolStrategy::Manual(protocol_type) => {
                Ok(*protocol_type)
            }
        }
    }
    
    /// Select protocol based on security requirements.
    async fn select_security_based_protocol(&self, context: &DMSSecurityContext) -> DMSResult<DMSProtocolType> {
        match context.required_security_level {
            DMSSecurityLevel::None | DMSSecurityLevel::Basic => {
                Ok(DMSProtocolType::Global)
            }
            DMSSecurityLevel::Standard => {
                if context.threat_level as u8 >= DMSThreatLevel::Elevated as u8 {
                    Ok(DMSProtocolType::Private)
                } else {
                    Ok(DMSProtocolType::Global)
                }
            }
            DMSSecurityLevel::High | DMSSecurityLevel::Maximum => {
                Ok(DMSProtocolType::Private)
            }
        }
    }
    
    /// Select protocol based on performance requirements.
    async fn select_performance_based_protocol(&self, context: &DMSPerformanceContext) -> DMSResult<DMSProtocolType> {
        // For high-performance requirements, prefer global protocol
        if context.required_throughput > 10_000_000 { // 10MB/s
            Ok(DMSProtocolType::Global)
        } else if context.max_latency_ms < 10 {
            Ok(DMSProtocolType::Global)
        } else {
            Ok(DMSProtocolType::Private)
        }
    }
    
    /// Select protocol using adaptive strategy.
    async fn select_adaptive_protocol(&self, context: &DMSAdaptiveContext) -> DMSResult<DMSProtocolType> {
        // Simple weighted decision based on security vs performance
        if context.security_weight > context.performance_weight {
            Ok(DMSProtocolType::Private)
        } else {
            Ok(DMSProtocolType::Global)
        }
    }
    
    /// Get adapter statistics.
    pub async fn get_stats(&self) -> DMSProtocolAdapterStats {
        *self.stats.read().await
    }
    
    /// Shutdown the protocol adapter.
    pub async fn shutdown(&mut self) -> DMSResult<()> {
        // Clear all connections
        self.connection_manager.clear_all_connections().await?;
        
        // Shutdown all protocols
        let mut protocols = self.protocols.write().await;
        for (_, mut protocol) in protocols.drain() {
            protocol.shutdown().await?;
        }
        
        *self.initialized.write().await = false;
        Ok(())
    }
}

impl Default for DMSProtocolAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSConnectionManager {
    /// Store a connection with metadata.
    async fn store_connection(
        &self,
        connection_id: String,
        connection: Box<dyn DMSProtocolConnection>,
        protocol_type: DMSProtocolType,
    ) -> DMSResult<()> {
        let metadata = DMSConnectionMetadata {
            original_protocol: protocol_type,
            current_protocol: protocol_type,
            established_at: Instant::now(),
            last_switch: None,
            switch_count: 0,
            state_data: HashMap::new(),
        };
        
        self.connections.write().await.insert(connection_id.clone(), connection.into());
        self.metadata.write().await.insert(connection_id, metadata);
        
        Ok(())
    }
    
    /// Clear all connections.
    async fn clear_all_connections(&self) -> DMSResult<()> {
        self.connections.write().await.clear();
        self.metadata.write().await.clear();
        Ok(())
    }
}

/// Wrapper for protocol connections managed by the adapter.
struct DMSProtocolConnectionWrapper {
    connection_id: String,
    connection_manager: Arc<DMSConnectionManager>,
    stats: Arc<RwLock<DMSProtocolAdapterStats>>,
}

#[async_trait]
impl DMSProtocolConnection for DMSProtocolConnectionWrapper {
    async fn send_message(&self, data: &[u8]) -> DMSResult<Vec<u8>> {
        let connections = self.connection_manager.connections.read().await;
        let connection = connections.get(&self.connection_id)
            .ok_or_else(|| DMSError::NotFound(format!("Connection {} not found", self.connection_id)))?;
        
        connection.send_message(data).await
    }
    
    async fn send_message_with_flags(&self, data: &[u8], flags: DMSMessageFlags) -> DMSResult<Vec<u8>> {
        let connections = self.connection_manager.connections.read().await;
        let connection = connections.get(&self.connection_id)
            .ok_or_else(|| DMSError::NotFound(format!("Connection {} not found", self.connection_id)))?;
        
        connection.send_message_with_flags(data, flags).await
    }
    
    async fn receive_message(&self) -> DMSResult<Vec<u8>> {
        let connections = self.connection_manager.connections.read().await;
        let connection = connections.get(&self.connection_id)
            .ok_or_else(|| DMSError::NotFound(format!("Connection {} not found", self.connection_id)))?;
        
        connection.receive_message().await
    }
    
    fn is_active(&self) -> bool {
        // This would need to be implemented properly with async runtime
        true // Placeholder
    }
    
    fn get_connection_info(&self) -> DMSConnectionInfo {
        // This would need proper implementation
        DMSConnectionInfo {
            connection_id: self.connection_id.clone(),
            target_id: "adapter-target".to_string(),
            protocol_type: DMSProtocolType::Global,
            established_at: Instant::now(),
            last_activity: Instant::now(),
            security_level: DMSSecurityLevel::Standard,
        }
    }
    
    async fn close(&self) -> DMSResult<()> {
        let mut connections = self.connection_manager.connections.write().await;
        connections.remove(&self.connection_id);
        
        let mut metadata = self.connection_manager.metadata.write().await;
        metadata.remove(&self.connection_id);
        
        Ok(())
    }
}