//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # DMSC Protocol Module
//! 
//! This module provides the protocol abstraction layer for DMSC, supporting
//! both global and private communication protocols. It implements the core
//! protocol management, security features, and integration capabilities
//! required for the DMSC distributed system.
//! 
//! ## Architecture Overview
//! 
//! The protocol module implements a layered architecture:
//! 
//! - **Protocol Layer**: Core protocol implementations (Global, Private)
//! - **Security Layer**: Encryption, authentication, and security enforcement
//! - **Adapter Layer**: Protocol abstraction and unified interfaces
//! - **Integration Layer**: Cross-protocol coordination and state management
//! - **Global State Layer**: Distributed state management and synchronization
//! 
//! ## Key Features
//! 
//! - **Protocol Abstraction**: Unified interface for different protocols
//! - **Security Integration**: End-to-end encryption and authentication
//! - **State Management**: Distributed state synchronization
//! - **Performance Optimization**: Intelligent protocol selection and switching
//! - **Fault Tolerance**: Graceful handling of protocol failures
//! - **Monitoring**: Comprehensive monitoring and alerting
//! 
//! ## Protocol Types
//! 
//! The module supports two main protocol types:
//! 
//! - **Global Protocol**: Standard communication protocol for general use
//! - **Private Protocol**: Enhanced security protocol for sensitive operations
//! 
//! ## Usage Examples
//! 
//! ```rust
//! use dms::protocol::{DMSCProtocolManager, DMSCProtocolType, DMSCProtocolConfig};
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create protocol manager
//!     let mut manager = DMSCProtocolManager::new();
//!     
//!     // Configure protocols
//!     let config = DMSCProtocolConfig {
//!         default_protocol: DMSCProtocolType::Global,
//!         enable_security: true,
//!         enable_state_sync: true,
//!         performance_optimization: true,
//!     };
//!     
//!     // Initialize manager
//!     manager.initialize(config).await?;
//!     
//!     // Send message using default protocol
//!     let response = manager.send_message("target-device", b"Hello DMSC").await?;
//!     
//!     // Switch to private protocol for sensitive operations
//!     manager.switch_protocol(DMSCProtocolType::Private).await?;
//!     let secure_response = manager.send_message("secure-device", b"Secure message").await?;
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::core::{DMSCResult, DMSCError, DMSCServiceContext};

mod global;
mod private;
mod security;
mod adapter;
mod global_state;
mod integration;
mod crypto;
mod frames;

pub use private::{DMSCPrivateProtocol, DMSCPrivateProtocolConfig};
pub use security::{DMSCCryptoSuite, DMSCDeviceAuthProtocol, DMSCObfuscationLayer, DMSCPostQuantumCrypto, DMSCRandomPadding};
pub use crypto::{DMSCCryptoEngine, AES256GCM, ChaCha20Poly1305};
pub use frames::{DMSCFrame, DMSCFrameHeader, DMSCFrameType, DMSCFrameParser, DMSCFrameBuilder};
pub use integration::{
    DMSCGlobalSystemIntegration,
    DMSCIntegrationConfig,
    DMSCCrossProtocolConnection,
    DMSCCrossProtocolConnectionState,
    DMSCControlCenter,
    DMSCExternalControlAction,
    DMSCExternalControlResult,
};

/// Protocol type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCProtocolType {
    /// Global communication protocol
    Global = 0,
    /// Private communication protocol
    Private = 1,
}

/// Protocol trait defining the common interface for all protocols.
#[async_trait]
pub trait DMSCProtocol: Send + Sync {
    /// Initialize the protocol.
    async fn initialize(&mut self) -> DMSCResult<()>;
    
    /// Shutdown the protocol.
    async fn shutdown(&mut self) -> DMSCResult<()>;
    
    /// Connect to a target device.
    async fn connect(&self, target: &str) -> DMSCResult<Box<dyn DMSCProtocolConnection>>;
    
    /// Get protocol type.
    fn protocol_type(&self) -> DMSCProtocolType;
    
    /// Get protocol version.
    fn protocol_version(&self) -> String;
    
    /// Get protocol status.
    async fn status(&self) -> DMSCProtocolStatus;
    
    /// Get protocol statistics.
    async fn get_stats(&self) -> DMSCProtocolStats;
}

/// Protocol connection trait for managing individual connections.
#[async_trait]
pub trait DMSCProtocolConnection: Send + Sync {
    /// Send a message through the connection.
    async fn send_message(&self, message: &[u8]) -> DMSCResult<Vec<u8>>;
    
    /// Receive a message from the connection.
    async fn receive_message(&self) -> DMSCResult<Vec<u8>>;
    
    /// Close the connection.
    async fn close(&mut self) -> DMSCResult<()>;
    
    /// Check if connection is active.
    async fn is_active(&self) -> bool;
    
    /// Get connection statistics.
    async fn get_connection_stats(&self) -> DMSCConnectionStats;
}

/// Protocol status structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolStatus {
    /// Protocol is initialized
    pub initialized: bool,
    /// Protocol is active
    pub active: bool,
    /// Number of active connections
    pub active_connections: u32,
    /// Protocol health
    pub health: DMSCProtocolHealth,
    /// Last activity timestamp
    pub last_activity: std::time::Instant,
}

/// Protocol health enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCProtocolHealth {
    /// Protocol is healthy
    Healthy,
    /// Protocol is degraded
    Degraded,
    /// Protocol is unhealthy
    Unhealthy,
    /// Protocol status is unknown
    Unknown,
}

/// Protocol statistics structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolStats {
    /// Total messages sent
    pub total_messages_sent: u64,
    /// Total messages received
    pub total_messages_received: u64,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_received: u64,
    /// Average latency
    pub average_latency_ms: u64,
    /// Error count
    pub error_count: u64,
    /// Success rate
    pub success_rate: f32,
}

/// Connection statistics structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCConnectionStats {
    /// Connection identifier
    pub connection_id: String,
    /// Target device
    pub target_device: String,
    /// Protocol type
    pub protocol_type: DMSCProtocolType,
    /// Connection state
    pub connection_state: DMSCConnectionState,
    /// Messages sent
    pub messages_sent: u64,
    /// Messages received
    pub messages_received: u64,
    /// Bytes sent
    pub bytes_sent: u64,
    /// Bytes received
    pub bytes_received: u64,
    /// Connection established time
    pub established_time: std::time::Instant,
    /// Last activity time
    pub last_activity: std::time::Instant,
}

/// Connection state enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCConnectionState {
    /// Connection is connecting
    Connecting,
    /// Connection is established
    Established,
    /// Connection is active
    Active,
    /// Connection is closing
    Closing,
    /// Connection is closed
    Closed,
    /// Connection failed
    Failed,
}

/// Protocol configuration structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolConfig {
    /// Default protocol type
    pub default_protocol: DMSCProtocolType,
    /// Enable security features
    pub enable_security: bool,
    /// Enable state synchronization
    pub enable_state_sync: bool,
    /// Performance optimization enabled
    pub performance_optimization: bool,
    /// Connection timeout
    pub connection_timeout: std::time::Duration,
    /// Maximum connections per protocol
    pub max_connections_per_protocol: u32,
    /// Protocol switching enabled
    pub protocol_switching_enabled: bool,
}

/// Protocol manager for managing multiple protocols.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolManager {
    /// Protocol configuration
    config: Arc<RwLock<DMSCProtocolConfig>>,
    /// Protocol adapter
    adapter: Arc<adapter::DMSCProtocolAdapter>,
    /// Global state manager
    state_manager: Arc<global_state::DMSCGlobalStateManager>,
    /// Global system integration
    integration: Arc<integration::DMSCGlobalSystemIntegration>,
    /// Current protocol type
    current_protocol: Arc<RwLock<DMSCProtocolType>>,
    /// Initialization status
    initialized: Arc<RwLock<bool>>,
}

impl DMSCProtocolManager {
    /// Create a new protocol manager.
    pub fn new() -> Self {
        Self {
            config: Arc::new(RwLock::new(DMSCProtocolConfig::default())),
            adapter: Arc::new(adapter::DMSCProtocolAdapter::new()),
            state_manager: Arc::new(global_state::DMSCGlobalStateManager::new()),
            integration: Arc::new(integration::DMSCGlobalSystemIntegration::new(
                integration::DMSCIntegrationConfig::default()
            )),
            current_protocol: Arc::new(RwLock::new(DMSCProtocolType::Global)),
            initialized: Arc::new(RwLock::new(false)),
        }
    }
    /// Initialize the protocol manager.
    pub async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()> {
        if *self.initialized.read().await {
            return Ok(());
        }
        
        *self.config.write().await = config;
        
        // Initialize protocol adapter
        let security_context = adapter::DMSCSecurityContext {
            required_security_level: security::DMSCSecurityLevel::Standard,
            threat_level: adapter::DMSCThreatLevel::Normal,
            data_classification: adapter::DMSCDataClassification::Internal,
            network_environment: adapter::DMSCNetworkEnvironment::Trusted,
            compliance_requirements: vec![],
        };
        
        let strategy = adapter::DMSCProtocolStrategy::SecurityBased(security_context);
        self.adapter.initialize(strategy).await?;
        
        // Initialize state manager
        self.state_manager.initialize().await?;
        
        // Initialize global system integration
        self.integration.initialize().await?;
        
        // Register protocols
        self.integration.register_protocol(DMSCProtocolType::Global).await?;
        self.integration.register_protocol(DMSCProtocolType::Private).await?;
        
        // Start coordination
        self.integration.start_coordination().await?;
        
        *self.initialized.write().await = true;
        Ok(())
    }
    
    /// Send a message using the current protocol.
    pub async fn send_message(&self, target: &str, message: &[u8]) -> DMSCResult<Vec<u8>> {
        let current_protocol = *self.current_protocol.read().await;
        self.send_message_with_protocol(target, message, current_protocol).await
    }
    
    /// Send a message using a specific protocol.
    pub async fn send_message_with_protocol(
        &self,
        target: &str,
        message: &[u8],
        protocol_type: DMSCProtocolType,
    ) -> DMSCResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Protocol manager not initialized".to_string()));
        }
        
        let current_protocol = *self.current_protocol.read().await;
        
        if protocol_type == current_protocol {
            // Use current protocol directly
            let connection = self.adapter.connect(target).await?;
            connection.send_message(message).await
        } else {
            // Use cross-protocol integration
            self.integration.send_cross_protocol_message(
                target, current_protocol, protocol_type, message
            ).await
        }
    }
    
    /// Switch to a different protocol.
    pub async fn switch_protocol(&self, protocol_type: DMSCProtocolType) -> DMSCResult<()> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Protocol manager not initialized".to_string()));
        }
        
        let current_protocol = *self.current_protocol.read().await;
        
        if current_protocol == protocol_type {
            return Ok(()); // Already using this protocol
        }
        
        // Check if protocol switching is enabled
        if !self.config.read().await.protocol_switching_enabled {
            return Err(DMSCError::InvalidState("Protocol switching is disabled".to_string()));
        }
        
        // Update current protocol
        *self.current_protocol.write().await = protocol_type;
        
        // Notify integration about protocol switch
        let mut event_data = std::collections::HashMap::new();
        event_data.insert("from_protocol".to_string(), format!("{:?}", current_protocol));
        event_data.insert("to_protocol".to_string(), format!("{:?}", protocol_type));
        
        // Note: In real implementation, we would publish this event through the integration event bus
        
        Ok(())
    }
    
    /// Get current protocol type.
    pub async fn get_current_protocol(&self) -> DMSCProtocolType {
        *self.current_protocol.read().await
    }
    
    /// Create a control center bound to this protocol manager.
    pub fn create_control_center(&self, service_context: DMSCServiceContext) -> DMSCControlCenter {
        DMSCControlCenter::new(self.state_manager.clone(), service_context)
    }
    
    /// Get protocol statistics.
    pub async fn get_stats(&self) -> DMSCResult<DMSCProtocolStats> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Protocol manager not initialized".to_string()));
        }
        
        // Get stats from current protocol
        let current_protocol = *self.current_protocol.read().await;
        
        // This would get stats from the actual protocol implementation
        // For now, return default stats
        Ok(DMSCProtocolStats {
            total_messages_sent: 0,
            total_messages_received: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            average_latency_ms: 0,
            error_count: 0,
            success_rate: 1.0,
        })
    }
    
    /// Shutdown the protocol manager.
    pub async fn shutdown(&mut self) -> DMSCResult<()> {
        if !*self.initialized.read().await {
            return Ok(());
        }
        
        // Shutdown integration
        let mut integration = self.integration.clone();
        integration.shutdown().await?;
        
        // Shutdown state manager
        let mut state_manager = self.state_manager.clone();
        state_manager.shutdown().await?;
        
        // Shutdown adapter
        let mut adapter = self.adapter.clone();
        adapter.shutdown().await?;
        
        *self.initialized.write().await = false;
        Ok(())
    }
}

impl Default for DMSCProtocolConfig {
    fn default() -> Self {
        Self {
            default_protocol: DMSCProtocolType::Global,
            enable_security: true,
            enable_state_sync: true,
            performance_optimization: true,
            connection_timeout: std::time::Duration::from_secs(30),
            max_connections_per_protocol: 1000,
            protocol_switching_enabled: true,
        }
    }
}

impl Default for DMSCProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}
