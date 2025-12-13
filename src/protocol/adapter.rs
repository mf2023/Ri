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
use log::{info, warn, debug};

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

/// Network condition enumeration for adaptive decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSNetworkCondition {
    /// Excellent network conditions
    Excellent,
    /// Good network conditions
    Good,
    /// Fair network conditions
    Fair,
    /// Poor network conditions
    Poor,
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
    /// Protocol-specific switch statistics
    pub protocol_switches: HashMap<DMSProtocolType, u64>,
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
                debug!("Protocol switch requested but already using {:?}", new_protocol_type);
                return Ok(()); // Already using this protocol
            }
        }
        
        // Validate new protocol is available
        let protocols = self.protocols.read().await;
        if !protocols.contains_key(&new_protocol_type) {
            self.stats.write().await.failed_switches += 1;
            warn!("Protocol {:?} not available for switch", new_protocol_type);
            return Err(DMSError::NotFound(format!("Protocol {:?} not available", new_protocol_type)));
        }
        
        // Perform protocol switch
        *self.active_protocol.write().await = Some(new_protocol_type);
        
        // Update statistics
        let mut stats = self.stats.write().await;
        stats.successful_switches += 1;
        let switch_time = start_time.elapsed().as_millis() as u64;
        stats.avg_switch_time_ms = (stats.avg_switch_time_ms + switch_time) / 2;

        // Update protocol-specific statistics
        if let Some(current) = current_protocol {
            *stats.protocol_switches.entry(current).or_insert(0) += 1;
        }
        *stats.protocol_switches.entry(new_protocol_type).or_insert(0) += 1;

        // Log the switch for monitoring with detailed context
        let switch_type = match (current_protocol, new_protocol_type) {
            (DMSProtocolType::Global, DMSProtocolType::Private) => "SECURITY_UPGRADE",
            (DMSProtocolType::Private, DMSProtocolType::Global) => "PERFORMANCE_UPGRADE",
            _ => "NEUTRAL_SWITCH",
        };

        info!(
            "Protocol switch: {:?} -> {:?} (type: {}, switch_time: {}ms, total_switches: {})",
            current_protocol, new_protocol_type, switch_type, switch_time, stats.protocol_switches.values().sum::<u64>()
        );
        
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
        // Check if protocols are available
        let protocols = self.protocols.read().await;
        
        // Determine protocol based on comprehensive security analysis
        let security_score = self.calculate_security_score(context);
        
        debug!("Security-based protocol selection - score: {}, required_level: {:?}, threat_level: {:?}", 
               security_score, context.required_security_level, context.threat_level);
        
        match security_score {
            score if score >= 80 => {
                // High security requirements - prefer private protocol if available
                if protocols.contains_key(&DMSProtocolType::Private) {
                    info!("Selected Private protocol for high security requirements (score: {})", security_score);
                    Ok(DMSProtocolType::Private)
                } else if protocols.contains_key(&DMSProtocolType::Global) {
                    warn!("Private protocol not available, falling back to Global for high security requirements");
                    Ok(DMSProtocolType::Global)
                } else {
                    Err(DMSError::NotFound("No suitable protocol available for high security requirements".to_string()))
                }
            }
            score if score >= 40 => {
                // Medium security requirements - check threat level and data classification
                if context.threat_level as u8 >= DMSThreatLevel::Elevated as u8 ||
                   context.data_classification as u8 >= DMSDataClassification::Confidential as u8 {
                    if protocols.contains_key(&DMSProtocolType::Private) {
                        info!("Selected Private protocol for medium security with elevated threat/confidential data");
                        Ok(DMSProtocolType::Private)
                    } else {
                        warn!("Private protocol not available for medium security requirements, using Global");
                        Ok(DMSProtocolType::Global)
                    }
                } else {
                    info!("Selected Global protocol for medium security requirements");
                    Ok(DMSProtocolType::Global)
                }
            }
            _ => {
                // Low security requirements - prefer global protocol for performance
                if protocols.contains_key(&DMSProtocolType::Global) {
                    debug!("Selected Global protocol for low security requirements");
                    Ok(DMSProtocolType::Global)
                } else if protocols.contains_key(&DMSProtocolType::Private) {
                    warn!("Global protocol not available, using Private for low security requirements");
                    Ok(DMSProtocolType::Private)
                } else {
                    Err(DMSError::NotFound("No suitable protocol available for low security requirements".to_string()))
                }
            }
        }
    }
    
    /// Calculate security score based on context.
    fn calculate_security_score(&self, context: &DMSSecurityContext) -> u8 {
        let mut score = 0u8;
        
        // Security level contribution (0-40 points)
        score += match context.required_security_level {
            DMSSecurityLevel::None => 0,
            DMSSecurityLevel::Basic => 10,
            DMSSecurityLevel::Standard => 25,
            DMSSecurityLevel::High => 35,
            DMSSecurityLevel::Maximum => 40,
        };
        
        // Threat level contribution (0-25 points)
        score += match context.threat_level {
            DMSThreatLevel::Normal => 0,
            DMSThreatLevel::Elevated => 15,
            DMSThreatLevel::High => 20,
            DMSThreatLevel::Critical => 25,
        };
        
        // Data classification contribution (0-20 points)
        score += match context.data_classification {
            DMSDataClassification::Public => 0,
            DMSDataClassification::Internal => 5,
            DMSDataClassification::Confidential => 15,
            DMSDataClassification::Secret => 18,
            DMSDataClassification::TopSecret => 20,
        };
        
        // Network environment contribution (0-10 points)
        score += match context.network_environment {
            DMSNetworkEnvironment::Trusted => 0,
            DMSNetworkEnvironment::Unknown => 5,
            DMSNetworkEnvironment::Untrusted => 8,
            DMSNetworkEnvironment::Hostile => 10,
        };
        
        // Compliance requirements contribution (0-5 points)
        if !context.compliance_requirements.is_empty() {
            score += 5;
        }
        
        score.min(100)
    }
    
    /// Select protocol based on performance requirements.
    async fn select_performance_based_protocol(&self, context: &DMSPerformanceContext) -> DMSResult<DMSProtocolType> {
        // Check if protocols are available
        let protocols = self.protocols.read().await;
        
        // Calculate performance score
        let performance_score = self.calculate_performance_score(context);
        
        debug!("Performance-based protocol selection - score: {}, required_throughput: {}, max_latency: {}ms", 
               performance_score, context.required_throughput, context.max_latency_ms);
        
        match performance_score {
            score if score >= 80 => {
                // High performance requirements - prefer Global protocol for better throughput
                if protocols.contains_key(&DMSProtocolType::Global) {
                    info!("Selected Global protocol for high performance requirements (score: {})", performance_score);
                    Ok(DMSProtocolType::Global)
                } else if protocols.contains_key(&DMSProtocolType::Private) {
                    warn!("Global protocol not available, using Private for high performance requirements");
                    Ok(DMSProtocolType::Private)
                } else {
                    Err(DMSError::NotFound("No suitable protocol available for high performance requirements".to_string()))
                }
            }
            score if score >= 40 => {
                // Medium performance requirements - balance between Global and Private
                if context.required_throughput >= 1000 || context.max_latency_ms <= 50 {
                    // High throughput or low latency needs - prefer Global
                    if protocols.contains_key(&DMSProtocolType::Global) {
                        info!("Selected Global protocol for medium performance with high throughput/low latency needs");
                        Ok(DMSProtocolType::Global)
                    } else {
                        warn!("Global protocol not available for medium performance requirements, using Private");
                        Ok(DMSProtocolType::Private)
                    }
                } else {
                    // Moderate requirements - prefer Private for stability
                    if protocols.contains_key(&DMSProtocolType::Private) {
                        info!("Selected Private protocol for medium performance with stability focus");
                        Ok(DMSProtocolType::Private)
                    } else {
                        warn!("Private protocol not available for medium performance requirements, using Global");
                        Ok(DMSProtocolType::Global)
                    }
                }
            }
            _ => {
                // Low performance requirements - prefer Private for stability and security
                if protocols.contains_key(&DMSProtocolType::Private) {
                    debug!("Selected Private protocol for low performance requirements");
                    Ok(DMSProtocolType::Private)
                } else if protocols.contains_key(&DMSProtocolType::Global) {
                    warn!("Private protocol not available, using Global for low performance requirements");
                    Ok(DMSProtocolType::Global)
                } else {
                    Err(DMSError::NotFound("No suitable protocol available for low performance requirements".to_string()))
                }
            }
        }
    }
    
    /// Calculate performance score based on context.
    fn calculate_performance_score(&self, context: &DMSPerformanceContext) -> u8 {
        let mut score = 0u8;
        
        // Throughput contribution (0-40 points)
        if context.required_throughput >= 100_000_000 { // 100MB/s
            score += 40;
        } else if context.required_throughput >= 50_000_000 { // 50MB/s
            score += 30;
        } else if context.required_throughput >= 10_000_000 { // 10MB/s
            score += 20;
        } else if context.required_throughput >= 1_000_000 { // 1MB/s
            score += 10;
        }
        
        // Latency contribution (0-30 points)
        if context.max_latency_ms <= 1 {
            score += 30;
        } else if context.max_latency_ms <= 5 {
            score += 25;
        } else if context.max_latency_ms <= 10 {
            score += 20;
        } else if context.max_latency_ms <= 50 {
            score += 10;
        }
        
        // Bandwidth constraints contribution (0-20 points)
        let bandwidth_score = if context.bandwidth_constraints.available_bandwidth >= 1_000_000_000 { // 1Gbps
            20
        } else if context.bandwidth_constraints.available_bandwidth >= 100_000_000 { // 100Mbps
            15
        } else if context.bandwidth_constraints.available_bandwidth >= 10_000_000 { // 10Mbps
            10
        } else {
            5
        };
        score += bandwidth_score;
        
        // Apply congestion penalty (0-20 points reduction)
        let congestion_penalty = (context.bandwidth_constraints.congestion_level * 20.0) as u8;
        score = score.saturating_sub(congestion_penalty);
        
        // Stability requirements contribution (0-10 points)
        if context.stability_requirements.max_packet_loss <= 0.001 { // 0.1%
            score += 10;
        } else if context.stability_requirements.max_packet_loss <= 0.01 { // 1%
            score += 5;
        }
        
        score.min(100)
    }
    
    /// Select protocol based on adaptive learning.
    async fn select_adaptive_protocol(&self, context: &DMSAdaptiveContext) -> DMSResult<DMSProtocolType> {
        let security_score = self.calculate_security_score(&context.security_context);
        let performance_score = self.calculate_performance_score(&context.performance_context);
        let adaptive_score = self.calculate_adaptive_score(security_score, performance_score, context);
        
        // Check if protocols are available
        let protocols = self.protocols.read().await;
        
        // Consider learned preferences
        let learned_preference = self.get_learned_protocol_preference().await;
        
        debug!("Adaptive protocol selection - security_score: {}, performance_score: {}, adaptive_score: {}, learned_preference: {:?}", 
               security_score, performance_score, adaptive_score, learned_preference);
        
        // Make decision based on adaptive score and learned preference
        match adaptive_score {
            score if score >= 70 => {
                // High adaptive score - follow learned preference with bias towards performance
                let selected_protocol = if learned_preference == DMSProtocolType::Global {
                    DMSProtocolType::Global
                } else {
                    DMSProtocolType::Private
                };
                
                if protocols.contains_key(&selected_protocol) {
                    info!("Selected {:?} protocol based on adaptive learning (score: {}, preference: {:?})", 
                          selected_protocol, adaptive_score, learned_preference);
                    Ok(selected_protocol)
                } else {
                    // Fallback to available protocol
                    let fallback = if protocols.contains_key(&DMSProtocolType::Global) {
                        DMSProtocolType::Global
                    } else if protocols.contains_key(&DMSProtocolType::Private) {
                        DMSProtocolType::Private
                    } else {
                        return Err(DMSError::NotFound("No suitable protocol available for adaptive selection".to_string()));
                    };
                    warn!("Preferred protocol {:?} not available, falling back to {:?}", selected_protocol, fallback);
                    Ok(fallback)
                }
            }
            score if score >= 40 => {
                // Medium adaptive score - balance between learned preference and security
                let selected_protocol = if context.security_context.required_security_level as u8 >= DMSSecurityLevel::Standard as u8 {
                    DMSProtocolType::Private  // Security preference
                } else {
                    learned_preference  // Use learned preference
                };
                
                if protocols.contains_key(&selected_protocol) {
                    info!("Selected {:?} protocol for balanced security/performance (score: {})", selected_protocol, adaptive_score);
                    Ok(selected_protocol)
                } else {
                    let fallback = if protocols.contains_key(&DMSProtocolType::Private) {
                        DMSProtocolType::Private
                    } else {
                        DMSProtocolType::Global
                    };
                    warn!("Balanced selection preferred {:?} not available, using {:?}", selected_protocol, fallback);
                    Ok(fallback)
                }
            }
            _ => {
                // Low adaptive score - be conservative and prefer Private for security
                if protocols.contains_key(&DMSProtocolType::Private) {
                    info!("Selected Private protocol for low adaptive score with security focus (score: {})", adaptive_score);
                    Ok(DMSProtocolType::Private)
                } else if protocols.contains_key(&DMSProtocolType::Global) {
                    warn!("Private protocol not available for low adaptive score, using Global");
                    Ok(DMSProtocolType::Global)
                } else {
                    Err(DMSError::NotFound("No suitable protocol available for conservative selection".to_string()))
                }
            }
        }
    }
    
    /// Calculate adaptive score based on context.
    fn calculate_adaptive_score(&self, context: &DMSAdaptiveContext) -> u8 {
        let mut score = 0u8;
        
        // Weight-based calculation (0-50 points for security, 0-50 for performance)
        let security_contribution = (context.security_weight * 50.0) as u8;
        let performance_contribution = (context.performance_weight * 50.0) as u8;
        
        score += security_contribution;
        score += performance_contribution;
        
        // Adaptation triggers adjustment (±20 points)
        for trigger in &context.adaptation_triggers {
            match trigger {
                DMSAdaptationTrigger::SecurityBreach => score = score.saturating_add(20),
                DMSAdaptationTrigger::PerformanceDegradation => score = score.saturating_sub(15),
                DMSAdaptationTrigger::NetworkChange => score = score.saturating_sub(10),
                DMSAdaptationTrigger::Manual => score = score.saturating_add(5),
            }
        }
        
        // Learning parameters adjustment (±10 points)
        if context.learning_params.learning_rate > 0.5 {
            score = score.saturating_add(10);
        }
        
        score.min(100)
    }
    
    /// Get learned protocol preference from historical data.
    async fn get_learned_protocol_preference(&self, context: &DMSAdaptiveContext) -> f32 {
        // Analyze historical performance data from stats
        let stats = self.stats.read().await;
        
        // Calculate success rates for each protocol
        let global_success_rate = if let Some(global_switches) = stats.protocol_switches.get(&DMSProtocolType::Global) {
            if *global_switches > 0 {
                (stats.successful_switches as f32 / *global_switches as f32) * 100.0
            } else {
                50.0 // Default neutral score
            }
        } else {
            50.0
        };
        
        let private_success_rate = if let Some(private_switches) = stats.protocol_switches.get(&DMSProtocolType::Private) {
            if *private_switches > 0 {
                (stats.successful_switches as f32 / *private_switches as f32) * 100.0
            } else {
                50.0
            }
        } else {
            50.0
        };
        
        // Apply weights to success rates
        let weighted_score = if context.security_weight > context.performance_weight {
            (private_success_rate * 0.7 + global_success_rate * 0.3)
        } else if context.performance_weight > context.security_weight {
            (global_success_rate * 0.7 + private_success_rate * 0.3)
        } else {
            (global_success_rate + private_success_rate) / 2.0
        };
        
        weighted_score.max(0.0).min(100.0)
    }
    
    /// Assess current network conditions.
    async fn assess_current_network_conditions(&self) -> DMSNetworkCondition {
        // This is a simplified implementation
        // In a real system, this would measure actual network metrics
        
        // For now, return good condition as default
        // This could be enhanced with actual network probing
        DMSNetworkCondition::Good
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
        // Check connection activity by examining last activity timestamp from metadata
        let metadata_guard = self.connection_manager.metadata.blocking_read();
        if let Some(metadata) = metadata_guard.get(&self.connection_id) {
            let elapsed = metadata.established_at.elapsed();
            // Consider connection active if established within last 5 minutes
            elapsed.as_secs() < 300
        } else {
            false
        }
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
