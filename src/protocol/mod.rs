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

/// Protocol type enumeration defining the available protocol variants.
///
/// The DMSC protocol module supports two distinct protocol types designed for
/// different communication requirements. The Global protocol handles standard
/// communications, while the Private protocol provides enhanced security features
/// for sensitive operations. Protocol type selection determines the encryption
/// algorithms, authentication mechanisms, and communication patterns used.
///
/// # Protocol Type Selection Guidelines
///
/// - Use **Global** protocol for general-purpose communications where standard
///   security requirements apply and performance is a priority
/// - Use **Private** protocol when enhanced confidentiality is required, including
///   sensitive data transmission, financial operations, or communications subject
///   to regulatory compliance requirements
///
/// # Python Bindings
///
/// When compiled with the `pyo3` feature, this enum provides Python static methods
/// for creating protocol type instances:
/// ```python
/// from dms import DMSCProtocolType
///
/// # Use global protocol for standard operations
/// protocol = DMSCProtocolType.Global()
///
/// # Use private protocol for sensitive operations
/// secure_protocol = DMSCProtocolType.Private()
/// ```
///
/// # Thread Safety
///
/// This enum is fully thread-safe and can be shared across concurrent contexts
/// without additional synchronization. The Copy trait enables efficient passing
/// of protocol type values through function arguments and return types.
///
/// # Storage and Transmission
///
/// Protocol type values are stored as single bytes (0 for Global, 1 for Private)
/// making them efficient for network transmission and compact storage. The Hash
/// trait enables protocol type usage as dictionary keys in collection types.
///
/// # Examples
///
/// Basic protocol type creation and comparison:
/// ```rust
/// use dms::protocol::DMSCProtocolType;
///
/// let global = DMSCProtocolType::Global;
/// let private = DMSCProtocolType::Private;
///
/// assert_eq!(global as u8, 0);
/// assert_eq!(private as u8, 1);
/// assert_ne!(global, private);
/// ```
///
/// Protocol type matching in conditional logic:
/// ```rust
/// use dms::protocol::DMSCProtocolType;
///
/// fn requires_enhanced_security(protocol: DMSCProtocolType) -> bool {
///     matches!(protocol, DMSCProtocolType::Private)
/// }
///
/// assert!(requires_enhanced_security(DMSCProtocolType::Private));
/// assert!(!requires_enhanced_security(DMSCProtocolType::Global));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCProtocolType {
    /// Global communication protocol - standard protocol for general use
    ///
    /// The Global protocol provides reliable, ordered message delivery with
    /// standard AES-256-GCM encryption. It is optimized for performance and
    /// is suitable for the majority of inter-service communications within
    /// the DMSC distributed system. This protocol type consumes minimal
    /// computational resources while maintaining industry-standard security.
    ///
    /// ## Protocol Characteristics
    ///
    /// - **Encryption**: AES-256-GCM with 256-bit keys
    /// - **Key Exchange**: ECDH with P-256 curve
    /// - **Digital Signatures**: ECDSA with P-256 curve
    /// - **Hash Functions**: SHA-256 for message integrity
    /// - **Connection Model**: Long-lived persistent connections
    /// - **Message Ordering**: Guaranteed in-order delivery
    /// - **Flow Control**: Sliding window-based congestion control
    ///
    /// ## Performance Characteristics
    ///
    /// - **Latency**: Low overhead, suitable for real-time communications
    /// - **Throughput**: Optimized for high message throughput
    /// - **Resource Usage**: Minimal CPU and memory footprint
    /// - **Scaling**: Efficient connection multiplexing
    ///
    /// ## Use Cases
    ///
    /// - General inter-service communication
    /// - Status updates and health monitoring
    /// - Non-sensitive data synchronization
    /// - Load balancing and service discovery
    Global = 0,
    /// Private communication protocol - enhanced security protocol
    ///
    /// The Private protocol implements advanced security features including
    /// post-quantum cryptography, traffic obfuscation, and enhanced device
    /// authentication. This protocol is designed for operations requiring
    /// maximum confidentiality and resistance to sophisticated attacks,
    /// including potential future quantum computing threats.
    ///
    /// ## Protocol Characteristics
    ///
    /// - **Encryption**: AES-256-GCM + ChaCha20-Poly1305 (dual encryption)
    /// - **Key Exchange**: Kyber-1024 (post-quantum KEM)
    /// - **Digital Signatures**: Dilithium-5 or Falcon-1024
    /// - **Traffic Obfuscation**: Polymorphic traffic patterns
    /// - **Anti-Forensic**: Metadata protection and timing randomization
    /// - **Device Authentication**: Hardware-based identity verification
    ///
    /// ## Security Level
    ///
    /// The Private protocol achieves Security Level 10 (maximum) under the
    /// DMSC security framework, providing protection against:
    ///
    /// - Classical cryptanalysis attacks
    /// - Quantum computing attacks (Shor's algorithm, Grover's algorithm)
    /// - Side-channel attacks (timing, power, electromagnetic)
    /// - Traffic analysis and pattern recognition
    /// - Replay and man-in-the-middle attacks
    ///
    /// ## Use Cases
    ///
    /// - Sensitive data transmission (financial, medical, legal)
    /// - Regulatory compliance (GDPR, HIPAA, PCI-DSS)
    /// - High-value transaction processing
    /// - Communications subject to advanced persistent threats
    Private = 1,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCProtocolType {
    #[staticmethod]
    fn Global() -> Self {
        DMSCProtocolType::Global
    }
    
    #[staticmethod]
    fn Private() -> Self {
        DMSCProtocolType::Private
    }
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

/// Protocol connection trait for managing individual protocol connections.
///
/// This trait defines the core operations available on a protocol connection,
/// enabling message transmission, reception, and lifecycle management. Connections
/// represent established communication channels to remote devices or services,
/// providing bidirectional data exchange with reliability guarantees.
///
/// ## Connection Lifecycle
///
/// 1. **Creation**: Connections are created through protocol's `connect()` method
/// 2. **Active State**: Ready for message exchange after successful establishment
/// 3. **Message Exchange**: Bidirectional communication using send/receive methods
/// 4. **Graceful Closure**: Connections should be closed via `close()` when done
///
/// ## Thread Safety
///
/// All trait methods are marked with async_trait and implement Send + Sync bounds,
/// enabling safe concurrent access from multiple tasks or threads. Multiple send
/// operations may be performed concurrently, though message ordering depends on
/// the underlying protocol implementation.
///
/// ## Error Handling
///
/// Operations return `DMSCResult` indicating success or specific error conditions:
/// - Connection errors during send/receive operations
/// - Timeout errors when operations exceed configured limits
/// - Resource exhaustion when connection limits are exceeded
/// - Protocol violations or malformed messages
///
/// # Examples
///
/// Basic connection usage pattern:
/// ```rust,ignore
/// async fn communicate(protocol: &dyn DMSCProtocol, target: &str) -> DMSCResult<Vec<u8>> {
///     // Establish connection to target device
///     let connection = protocol.connect(target).await?;
///
///     // Send request message
///     let request = b"Hello, target device!";
///     let response = connection.send_message(request).await?;
///
///     // Gracefully close connection
///     connection.close().await?;
///
///     Ok(response)
/// }
/// ```
///
/// Concurrent message handling:
/// ```rust,ignore
/// async fn parallel_communication(
///     protocol: &dyn DMSCProtocol,
///     targets: &[&str],
///     message: &[u8]
/// ) -> DMSCResult<Vec<Vec<u8>>> {
///     use futures::stream::FuturesUnordered;
///     use futures::TryStreamExt;
///
///     // Create connections to all targets
///     let connections: Vec<_> = targets
///         .iter()
///         .map(|t| protocol.connect(t))
///         .collect();
///
///     // Execute sends concurrently
///     let mut results = FuturesUnordered::new();
///     for (i, conn) in connections.into_iter().enumerate() {
///         let conn = conn.await?;
///         results.push(async move {
///             conn.send_message(message).await
///         });
///     }
///
///     // Collect all responses
///     let responses: Vec<Vec<u8>> = results.try_collect().await?;
///     Ok(responses)
/// }
/// ```
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
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

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCProtocolHealth {
    #[staticmethod]
    fn Healthy() -> Self {
        DMSCProtocolHealth::Healthy
    }
    
    #[staticmethod]
    fn Degraded() -> Self {
        DMSCProtocolHealth::Degraded
    }
    
    #[staticmethod]
    fn Unhealthy() -> Self {
        DMSCProtocolHealth::Unhealthy
    }
    
    #[staticmethod]
    fn Unknown() -> Self {
        DMSCProtocolHealth::Unknown
    }
}

/// Protocol statistics structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolStats {
    /// Total messages sent
    #[pyo3(get, set)]
    pub total_messages_sent: u64,
    /// Total messages received
    #[pyo3(get, set)]
    pub total_messages_received: u64,
    /// Total bytes sent
    #[pyo3(get, set)]
    pub total_bytes_sent: u64,
    /// Total bytes received
    #[pyo3(get, set)]
    pub total_bytes_received: u64,
    /// Average latency
    #[pyo3(get, set)]
    pub average_latency_ms: u64,
    /// Error count
    #[pyo3(get, set)]
    pub error_count: u64,
    /// Success rate
    #[pyo3(get, set)]
    pub success_rate: f32,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCProtocolStats {
    #[new]
    fn new() -> Self {
        Self {
            total_messages_sent: 0,
            total_messages_received: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            average_latency_ms: 0,
            error_count: 0,
            success_rate: 1.0,
        }
    }
}

/// Connection statistics structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCConnectionStats {
    /// Connection identifier
    #[pyo3(get, set)]
    pub connection_id: String,
    /// Target device
    #[pyo3(get, set)]
    pub target_device: String,
    /// Protocol type
    #[pyo3(get, set)]
    pub protocol_type: DMSCProtocolType,
    /// Connection state
    #[pyo3(get, set)]
    pub connection_state: DMSCConnectionState,
    /// Messages sent
    #[pyo3(get, set)]
    pub messages_sent: u64,
    /// Messages received
    #[pyo3(get, set)]
    pub messages_received: u64,
    /// Bytes sent
    #[pyo3(get, set)]
    pub bytes_sent: u64,
    /// Bytes received
    #[pyo3(get, set)]
    pub bytes_received: u64,
    /// Connection established time
    #[pyo3(get, set)]
    pub established_time: std::time::Instant,
    /// Last activity time
    #[pyo3(get, set)]
    pub last_activity: std::time::Instant,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCConnectionStats {
    #[new]
    fn new(connection_id: String, target_device: String, protocol_type: DMSCProtocolType) -> Self {
        Self {
            connection_id,
            target_device,
            protocol_type,
            connection_state: DMSCConnectionState::Connecting,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            established_time: std::time::Instant::now(),
            last_activity: std::time::Instant::now(),
        }
    }
}

/// Connection state enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
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

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCConnectionState {
    #[staticmethod]
    fn Connecting() -> Self {
        DMSCConnectionState::Connecting
    }
    
    #[staticmethod]
    fn Established() -> Self {
        DMSCConnectionState::Established
    }
    
    #[staticmethod]
    fn Active() -> Self {
        DMSCConnectionState::Active
    }
    
    #[staticmethod]
    fn Closing() -> Self {
        DMSCConnectionState::Closing
    }
    
    #[staticmethod]
    fn Closed() -> Self {
        DMSCConnectionState::Closed
    }
    
    #[staticmethod]
    fn Failed() -> Self {
        DMSCConnectionState::Failed
    }
}

/// Protocol configuration structure.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolConfig {
    /// Default protocol type
    #[pyo3(get, set)]
    pub default_protocol: DMSCProtocolType,
    /// Enable security features
    #[pyo3(get, set)]
    pub enable_security: bool,
    /// Enable state synchronization
    #[pyo3(get, set)]
    pub enable_state_sync: bool,
    /// Performance optimization enabled
    #[pyo3(get, set)]
    pub performance_optimization: bool,
    /// Connection timeout
    #[pyo3(get, set)]
    pub connection_timeout: std::time::Duration,
    /// Maximum connections per protocol
    #[pyo3(get, set)]
    pub max_connections_per_protocol: u32,
    /// Protocol switching enabled
    #[pyo3(get, set)]
    pub protocol_switching_enabled: bool,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCProtocolConfig {
    #[new]
    fn new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
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

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCProtocolManager {
    #[new]
    fn new() -> Self {
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
    
    /// Send a message using the current protocol (Python wrapper).
    #[pyo3(name = "send_message")]
    fn send_message_impl(&self, target: String, message: Vec<u8>) -> pyo3::PyResult<Vec<u8>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.send_message(&target, &message).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Protocol error: {}", e))
            })
        })
    }
    
    /// Send a message using a specific protocol (Python wrapper).
    #[pyo3(name = "send_message_with_protocol")]
    fn send_message_with_protocol_impl(&self, target: String, message: Vec<u8>, protocol_type: DMSCProtocolType) -> pyo3::PyResult<Vec<u8>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.send_message_with_protocol(&target, &message, protocol_type).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Protocol error: {}", e))
            })
        })
    }
    
    /// Switch to a different protocol (Python wrapper).
    #[pyo3(name = "switch_protocol")]
    fn switch_protocol_impl(&self, protocol_type: DMSCProtocolType) -> pyo3::PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.switch_protocol(protocol_type).await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Protocol error: {}", e))
            })
        })
    }
    
    /// Get current protocol type (Python wrapper).
    #[pyo3(name = "get_current_protocol")]
    fn get_current_protocol_impl(&self) -> DMSCProtocolType {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            self.get_current_protocol().await
        }).unwrap()
    }
    
    /// Get protocol statistics (Python wrapper).
    #[pyo3(name = "get_stats")]
    fn get_stats_impl(&self) -> pyo3::PyResult<DMSCProtocolStats> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.get_stats().await.map_err(|e| {
                pyo3::exceptions::PyRuntimeError::new_err(format!("Protocol error: {}", e))
            })
        })
    }
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
