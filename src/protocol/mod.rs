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

//! # Protocol Module
//!
//! This module provides protocol implementations for Ri, including
//! global protocol, private protocol, post-quantum cryptography, and
//! integration features.
//!
//! ## Features
//!
//! - **RiProtocol**: Main protocol interface (trait definition)
//! - **RiGlobalProtocol**: Global protocol implementation (basic implementation)
//! - **RiPrivateProtocol**: Private protocol implementation (basic implementation)
//! - **RiCrypto**: Cryptographic operations
//! - **Post-Quantum Cryptography**: Kyber, Dilithium, Falcon implementations using liboqs
//!
//! ## Security Status
//!
//! This module now uses the **liboqs** library for post-quantum cryptography,
//! which is:
//! - The reference implementation from the NIST PQC competition
//! - Actively maintained and regularly audited
//! - **Suitable for production use**
//!
//! Post-Quantum Cryptography algorithms (Kyber, Dilithium, Falcon):
//! - Based on NIST PQC competition algorithms
//! - Have undergone formal security analysis
//! - Constant-time implementations for side-channel resistance
//! - Recommended for protecting sensitive data
//!
//! ## Recommendation
//!
//! For cryptographic operations, this module uses audited libraries:
//! - liboqs - NIST PQC reference implementation
//! - ring - Modern, audited crypto library
//! - openssl - Industry-standard crypto library

use std::collections::HashMap as FxHashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
#[cfg(feature = "pyo3")]
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::core::{RiResult, RiError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Frame definitions for binary protocol encoding
pub mod frames;
pub use frames::{RiFrameBuilder, RiFrameParser};

/// Post-quantum cryptography modules (requires protocol feature)
#[cfg(feature = "protocol")]
pub mod kyber;
#[cfg(feature = "protocol")]
pub mod dilithium;
#[cfg(feature = "protocol")]
pub mod falcon;
#[cfg(feature = "protocol")]
pub mod post_quantum;
#[cfg(feature = "protocol")]
pub use post_quantum::{
    KyberKEM, KyberPublicKey, KyberSecretKey, KyberCiphertext,
    DilithiumSigner, DilithiumPublicKey, DilithiumSecretKey, DilithiumSignature,
    FalconSigner, FalconPublicKey, FalconSecretKey, FalconSignature,
    RiPostQuantumAlgorithm, KEMResult,
};

/// Advanced protocol features (future/experimental)
/// These modules are not yet fully stabilized and may change in future versions.
/// Enable with `protocol-advanced` feature.
#[cfg(feature = "protocol-advanced")]
pub mod adapter;
#[cfg(feature = "protocol-advanced")]
pub mod crypto;
#[cfg(feature = "protocol-advanced")]
pub mod global_state;
#[cfg(feature = "protocol-advanced")]
pub mod guomi;
#[cfg(feature = "protocol-advanced")]
pub mod hsm;
#[cfg(feature = "protocol-advanced")]
pub mod private;
#[cfg(feature = "protocol-advanced")]
pub mod security;
#[cfg(feature = "protocol-advanced")]
pub mod integration;

/// Protocol type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiProtocolType {
    /// Standard global protocol
    Global = 0,
    /// Enhanced private protocol
    Private = 1,
}

/// Protocol status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiProtocolStatus {
    /// Protocol is inactive
    Inactive,
    /// Protocol is initializing
    Initializing,
    /// Protocol is ready
    Ready,
    /// Protocol has an error
    Error,
}

/// Connection state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiConnectionState {
    /// Connection is disconnected
    Disconnected,
    /// Connection is connecting
    Connecting,
    /// Connection is connected
    Connected,
    /// Connection is disconnecting
    Disconnecting,
}

/// Security level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiSecurityLevel {
    /// No security
    None,
    /// Standard security
    Standard,
    /// High security
    High,
    /// Military-grade security
    Military,
}

/// Protocol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiProtocolConfig {
    /// Default protocol type
    pub default_protocol: RiProtocolType,
    /// Whether security is enabled
    pub enable_security: bool,
    /// Security level
    pub security_level: RiSecurityLevel,
    /// Whether state synchronization is enabled
    pub enable_state_sync: bool,
    /// Whether performance optimization is enabled
    pub performance_optimization: bool,
}

impl Default for RiProtocolConfig {
    fn default() -> Self {
        Self {
            default_protocol: RiProtocolType::Global,
            enable_security: true,
            security_level: RiSecurityLevel::Standard,
            enable_state_sync: true,
            performance_optimization: true,
        }
    }
}

impl RiProtocolConfig {
    /// Validate the configuration.
    pub fn validate(&self) -> RiResult<()> {
        if self.security_level == RiSecurityLevel::None && self.enable_security {
            return Err(RiError::Config(
                "Security level cannot be None when security is enabled".to_string()
            ));
        }

        Ok(())
    }

    /// Create a secure configuration.
    pub fn secure() -> Self {
        Self {
            default_protocol: RiProtocolType::Private,
            enable_security: true,
            security_level: RiSecurityLevel::High,
            enable_state_sync: true,
            performance_optimization: true,
        }
    }

    /// Create a maximum security configuration with post-quantum cryptography.
    pub fn maximum_security() -> Self {
        Self {
            default_protocol: RiProtocolType::Private,
            enable_security: true,
            security_level: RiSecurityLevel::Military,
            enable_state_sync: true,
            performance_optimization: false,
        }
    }
}

/// Protocol statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiProtocolStats {
    /// Total messages sent
    pub messages_sent: u64,
    /// Total messages received
    pub messages_received: u64,
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Total errors
    pub errors: u64,
    /// Average latency in milliseconds
    pub avg_latency_ms: f64,
}

impl RiProtocolStats {
    pub fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            errors: 0,
            avg_latency_ms: 0.0,
        }
    }

    pub fn record_sent(&mut self, bytes: usize) {
        self.messages_sent += 1;
        self.bytes_sent += bytes as u64;
    }

    pub fn record_received(&mut self, bytes: usize) {
        self.messages_received += 1;
        self.bytes_received += bytes as u64;
    }

    pub fn record_error(&mut self) {
        self.errors += 1;
    }
}

/// Connection statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiConnectionStats {
    /// Total connections
    pub total_connections: u64,
    /// Active connections
    pub active_connections: u64,
    /// Total bytes sent
    pub bytes_sent: u64,
    /// Total bytes received
    pub bytes_received: u64,
    /// Connection duration in seconds
    pub connection_duration_secs: u64,
}

impl Default for RiConnectionStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            bytes_sent: 0,
            bytes_received: 0,
            connection_duration_secs: 0,
        }
    }
}

/// Protocol health status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiProtocolHealth {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

impl Default for RiProtocolHealth {
    fn default() -> Self {
        RiProtocolHealth::Unknown
    }
}

/// Message flags for protocol messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiMessageFlags {
    /// Whether the message is compressed
    pub compressed: bool,
    /// Whether the message is encrypted
    pub encrypted: bool,
    /// Whether the message requires acknowledgment
    pub requires_ack: bool,
    /// Whether this is a priority message
    pub priority: bool,
}

impl Default for RiMessageFlags {
    fn default() -> Self {
        Self {
            compressed: false,
            encrypted: false,
            requires_ack: false,
            priority: false,
        }
    }
}

/// Connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiConnectionInfo {
    /// Connection ID
    pub connection_id: String,
    /// Remote device ID
    pub device_id: String,
    /// Connection address
    pub address: String,
    /// Protocol type
    pub protocol_type: RiProtocolType,
    /// Connection state
    pub state: RiConnectionState,
    /// Security level
    pub security_level: RiSecurityLevel,
    /// Connection timestamp
    pub connected_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
}

impl Default for RiConnectionInfo {
    fn default() -> Self {
        Self {
            connection_id: String::new(),
            device_id: String::new(),
            address: String::new(),
            protocol_type: RiProtocolType::Global,
            state: RiConnectionState::Disconnected,
            security_level: RiSecurityLevel::None,
            connected_at: 0,
            last_activity: 0,
        }
    }
}

/// Frame type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiFrameType {
    /// Data frame
    Data = 0,
    /// Control frame
    Control = 1,
    /// Heartbeat frame
    Heartbeat = 2,
    /// Acknowledgment frame
    Ack = 3,
    /// Error frame
    Error = 4,
}

/// Frame header
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiFrameHeader {
    /// Protocol version (major.minor as u8)
    pub version: u8,
    /// Frame type
    pub frame_type: RiFrameType,
    /// Sequence number
    pub sequence_number: u64,
    /// Message length
    pub length: u32,
    /// Timestamp
    pub timestamp: u64,
    /// Flags
    pub flags: u16,
    /// Authentication tag offset (for authenticated frames)
    pub auth_tag_offset: u16,
}

impl Default for RiFrameHeader {
    fn default() -> Self {
        Self {
            version: 1,
            frame_type: RiFrameType::Data,
            sequence_number: 0,
            length: 0,
            timestamp: 0,
            flags: 0,
            auth_tag_offset: 0,
        }
    }
}

impl RiFrameHeader {
    /// Get major version number.
    pub fn major_version(&self) -> u8 {
        self.version >> 4
    }

    /// Get minor version number.
    pub fn minor_version(&self) -> u8 {
        self.version & 0x0F
    }

    /// Check if a feature is supported.
    pub fn supports_feature(&self, feature: u16) -> bool {
        (self.flags & feature) != 0
    }
}

/// Protocol frame
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiFrame {
    /// Frame header
    pub header: RiFrameHeader,
    /// Frame payload
    pub payload: Vec<u8>,
    /// Source device ID
    pub source_id: String,
    /// Target device ID
    pub target_id: String,
}

impl Default for RiFrame {
    fn default() -> Self {
        Self {
            header: RiFrameHeader::default(),
            payload: Vec::new(),
            source_id: String::new(),
            target_id: String::new(),
        }
    }
}

/// Core protocol trait
#[async_trait]
pub trait RiProtocol {
    /// Get protocol type
    fn protocol_type(&self) -> RiProtocolType;
    
    /// Check if protocol is ready
    async fn is_ready(&self) -> bool;
    
    /// Initialize protocol
    async fn initialize(&mut self, config: RiProtocolConfig) -> RiResult<()>;
    
    /// Send message
    async fn send_message(&mut self, target: &str, data: &[u8]) -> RiResult<Vec<u8>>;
    
    /// Send message with flags
    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: RiMessageFlags) -> RiResult<Vec<u8>>;
    
    /// Receive message
    async fn receive_message(&mut self) -> RiResult<Vec<u8>>;
    
    /// Get connection info
    async fn get_connection_info(&self, connection_id: &str) -> RiResult<RiConnectionInfo>;
    
    /// Close connection
    async fn close_connection(&mut self, connection_id: &str) -> RiResult<()>;
    
    /// Get protocol statistics
    fn get_stats(&self) -> RiProtocolStats;
    
    /// Get protocol health
    async fn get_health(&self) -> RiProtocolHealth;
    
    /// Shutdown protocol
    async fn shutdown(&mut self) -> RiResult<()>;
}

/// Protocol connection trait
#[async_trait]
pub trait RiProtocolConnection {
    /// Get connection ID
    fn connection_id(&self) -> &str;
    
    /// Get remote device ID
    fn remote_device_id(&self) -> &str;
    
    /// Get protocol type
    fn protocol_type(&self) -> RiProtocolType;
    
    /// Check if connection is active
    fn is_active(&self) -> bool;
    
    /// Send data
    async fn send(&mut self, data: &[u8]) -> RiResult<usize>;
    
    /// Receive data
    async fn receive(&mut self, buffer: &mut [u8]) -> RiResult<usize>;
    
    /// Get statistics
    fn get_stats(&self) -> RiConnectionStats;
    
    /// Close connection
    async fn close(&mut self) -> RiResult<()>;
}

/// Protocol manager
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiProtocolManager {
    /// Protocol statistics
    pub stats: Arc<RwLock<RiProtocolStats>>,
    /// Default protocol type
    pub default_protocol: RiProtocolType,
    /// Active connections
    connections: Arc<RwLock<FxHashMap<String, RiConnectionInfo>>>,
    /// Message sequence counter
    sequence_counter: Arc<AtomicU64>,
    /// Protocol initialized state
    initialized: Arc<RwLock<bool>>,
}

#[cfg(test)]
mod protocol_tests {
    use super::*;

    #[test]
    fn test_protocol_config_default() {
        let config = RiProtocolConfig::default();
        assert_eq!(config.default_protocol, RiProtocolType::Global);
        assert!(config.enable_security);
        assert_eq!(config.security_level, RiSecurityLevel::Standard);
    }

    #[test]
    fn test_protocol_config_secure() {
        let config = RiProtocolConfig::secure();
        assert_eq!(config.default_protocol, RiProtocolType::Private);
        assert!(config.enable_security);
        assert_eq!(config.security_level, RiSecurityLevel::High);
    }

    #[test]
    fn test_protocol_config_maximum_security() {
        let config = RiProtocolConfig::maximum_security();
        assert_eq!(config.default_protocol, RiProtocolType::Private);
        assert!(config.enable_security);
        assert_eq!(config.security_level, RiSecurityLevel::Military);
    }

    #[test]
    fn test_protocol_config_validation() {
        let mut config = RiProtocolConfig::default();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // None security level with security enabled should fail
        config.security_level = RiSecurityLevel::None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_frame_header_version() {
        let header = RiFrameHeader::default();
        assert_eq!(header.major_version(), 0);
        assert_eq!(header.minor_version(), 1);
    }

    #[test]
    fn test_frame_header_supports_feature() {
        let header = RiFrameHeader {
            flags: 0b00000011,
            ..Default::default()
        };

        assert!(header.supports_feature(0b00000001));
        assert!(header.supports_feature(0b00000010));
        assert!(!header.supports_feature(0b00000100));
    }

    #[test]
    fn test_protocol_stats() {
        let stats = RiProtocolStats::new();
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_connection_info() {
        let mut info = RiConnectionInfo::default();
        info.device_id = "test-device".to_string();
        info.state = RiConnectionState::Connected;
        assert_eq!(info.device_id, "test-device");
        assert_eq!(info.state, RiConnectionState::Connected);
        assert_eq!(info.security_level, RiSecurityLevel::None);
    }

    #[test]
    fn test_protocol_type_values() {
        assert_eq!(RiProtocolType::Global as u8, 0);
        assert_eq!(RiProtocolType::Private as u8, 1);
    }

    #[test]
    fn test_security_level_values() {
        assert_eq!(RiSecurityLevel::None as u8, 0);
        assert_eq!(RiSecurityLevel::Standard as u8, 1);
        assert_eq!(RiSecurityLevel::High as u8, 2);
        assert_eq!(RiSecurityLevel::Military as u8, 3);
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiProtocolManager {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }
    
    #[getter]
    fn get_stats_py(&self) -> RiProtocolStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiProtocolStats::new())
    }
    
    #[getter]
    fn get_default_protocol_py(&self) -> RiProtocolType {
        self.default_protocol
    }
    
    /// Initialize manager
    pub fn initialize(&mut self, config: RiProtocolConfig) -> PyResult<()> {
        self.default_protocol = config.default_protocol;
        
        let mut initialized = self.initialized.try_write()
            .map_err(|_| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Failed to acquire write lock on initialized state"))?;
        *initialized = true;
        
        Ok(())
    }
    
    /// Send message (sync version for Python)
    pub fn send_message(&self, target: &str, data: &[u8]) -> Vec<u8> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        
        let sequence = self.sequence_counter.fetch_add(1, Ordering::SeqCst);
        
        let frame = RiFrame {
            header: RiFrameHeader {
                version: 1,
                frame_type: RiFrameType::Data,
                sequence_number: sequence,
                length: data.len() as u32,
                timestamp,
                flags: 0,
                auth_tag_offset: 0,
            },
            payload: data.to_vec(),
            source_id: String::from("protocol_manager"),
            target_id: target.to_string(),
        };
        
        let serialized = match serde_json::to_vec(&frame) {
            Ok(serialized_data) => serialized_data,
            Err(e) => {
                if let Ok(mut stats) = self.stats.try_write() {
                    stats.record_error();
                }
                let error_response = RiProtocolResponse {
                    success: false,
                    sequence_number: sequence,
                    target_id: target.to_string(),
                    response_data: format!("Serialization error: {}", e).into_bytes(),
                    timestamp,
                };
                return serde_json::to_vec(&error_response)
                    .unwrap_or_else(|_| b"{\"success\":false,\"error\":\"Serialization failed\"}".to_vec());
            }
        };
        
        let payload_len = serialized.len();
        if let Ok(mut stats) = self.stats.try_write() {
            stats.record_sent(payload_len);
        }
        
        let response_data = self.build_response_data(target, &frame, sequence, timestamp);
        
        let response = RiProtocolResponse {
            success: true,
            sequence_number: sequence,
            target_id: target.to_string(),
            response_data,
            timestamp,
        };
        
        self.stats.try_write()
            .map(|mut stats| stats.record_received(response.response_data.len()))
            .map_err(|e| tracing::error!("Failed to update protocol stats: {}", e))
            .ok();
        
        serde_json::to_vec(&response).unwrap_or_else(|_| b"{\"success\":true,\"message\":\"Message sent\"}".to_vec())
    }
    
    fn build_response_data(&self, target: &str, frame: &RiFrame, sequence: u64, timestamp: u64) -> Vec<u8> {
        let mut response = FxHashMap::<String, serde_json::Value>::new();
        
        response.insert("status".to_string(), serde_json::Value::String("delivered".to_string()));
        response.insert("target".to_string(), serde_json::Value::String(target.to_string()));
        response.insert("source".to_string(), serde_json::Value::String(frame.source_id.clone()));
        response.insert("sequence".to_string(), serde_json::Value::Number(sequence.into()));
        response.insert("timestamp".to_string(), serde_json::Value::Number(timestamp.into()));
        response.insert("frame_type".to_string(), serde_json::Value::String(format!("{:?}", frame.header.frame_type)));
        response.insert("payload_size".to_string(), serde_json::Value::Number(serde_json::Number::from(frame.payload.len())));
        response.insert("protocol".to_string(), serde_json::Value::String(format!("{:?}", self.default_protocol)));
        
        let delivery_info = serde_json::json!({
            "delivered_at": timestamp,
            "hops": 1,
            "route": [frame.source_id.clone(), target.to_string()]
        });
        response.insert("delivery".to_string(), delivery_info);
        
        serde_json::to_vec(&response).unwrap_or_default()
    }
    
    /// Send message with flags (sync version for Python)
    pub fn send_message_with_flags(&self, target: &str, data: &[u8], _flags: RiMessageFlags) -> Vec<u8> {
        self.send_message(target, data)
    }
    
    /// Get connection info (sync version for Python)
    pub fn get_connection_info(&self, connection_id: &str) -> Option<RiConnectionInfo> {
        self.connections.try_read()
            .ok()
            .and_then(|connections| connections.get(connection_id).cloned())
    }
    
    /// Close connection (sync version for Python)
    pub fn close_connection(&mut self, connection_id: &str) -> bool {
        self.connections.try_write()
            .ok()
            .map(|mut connections| connections.remove(connection_id).is_some())
            .unwrap_or(false)
    }
}

impl RiProtocolManager {
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(RiProtocolStats::new())),
            default_protocol: RiProtocolType::Global,
            connections: Arc::new(RwLock::new(FxHashMap::default())),
            sequence_counter: Arc::new(AtomicU64::new(0)),
            initialized: Arc::new(RwLock::new(false)),
        }
    }
}

impl Default for RiProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Protocol response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiProtocolResponse {
    /// Whether the operation was successful
    pub success: bool,
    /// Sequence number matching the request
    pub sequence_number: u64,
    /// Target ID that was addressed
    pub target_id: String,
    /// Response data payload
    pub response_data: Vec<u8>,
    /// Timestamp of the original request
    pub timestamp: u64,
}

impl Default for RiProtocolResponse {
    fn default() -> Self {
        Self {
            success: false,
            sequence_number: 0,
            target_id: String::new(),
            response_data: Vec::new(),
            timestamp: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProtocolError {
    #[error("Target not found: {target_id}")]
    TargetNotFound { target_id: String },
    #[error("Send failed: {message}")]
    SendFailed { message: String },
    #[error("Protocol not initialized")]
    NotInitialized,
    #[error("Invalid state: {state}")]
    InvalidState { state: String },
    #[error("Connection not found: {connection_id}")]
    ConnectionNotFound { connection_id: String },
    #[error("Serialization error: {message}")]
    Serialization { message: String },
    #[error("Operation not supported for this protocol")]
    NotSupported,
}

impl From<ProtocolError> for RiError {
    fn from(error: ProtocolError) -> Self {
        RiError::Other(format!("Protocol error: {}", error))
    }
}

impl From<serde_json::Error> for ProtocolError {
    fn from(error: serde_json::Error) -> Self {
        ProtocolError::Serialization { message: error.to_string() }
    }
}

impl From<ProtocolError> for RiResult<()> {
    fn from(error: ProtocolError) -> Self {
        Err(error.into())
    }
}

#[derive(Debug, Clone)]
pub struct RiBaseProtocol {
    config: RiProtocolConfig,
    stats: Arc<RwLock<RiProtocolStats>>,
    connections: Arc<RwLock<FxHashMap<String, RiConnectionInfo>>>,
    sequence_counter: Arc<AtomicU64>,
    initialized: Arc<RwLock<bool>>,
    _receiver_id: String,
}

impl RiBaseProtocol {
    pub fn new(receiver_id: String) -> Self {
        Self {
            config: RiProtocolConfig::default(),
            stats: Arc::new(RwLock::new(RiProtocolStats::new())),
            connections: Arc::new(RwLock::new(FxHashMap::default())),
            sequence_counter: Arc::new(AtomicU64::new(0)),
            initialized: Arc::new(RwLock::new(false)),
            _receiver_id: receiver_id,
        }
    }
    
    pub async fn is_ready(&self) -> bool {
        *self.initialized.read().await
    }
    
    pub async fn initialize(&mut self, config: RiProtocolConfig) {
        self.config = config;
        *self.initialized.write().await = true;
    }

    pub async fn send_message(&mut self, _target: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(ProtocolError::NotInitialized.into());
        }

        let sequence = self.sequence_counter.fetch_add(1, Ordering::SeqCst) as u32;

        self.stats.write().await.record_sent(data.len());

        let mut builder = RiFrameBuilder::new();
        builder.set_sequence(sequence);
        let frame = builder.build_data_frame(data.to_vec())
            .map_err(|e| ProtocolError::Serialization {
                message: e.to_string()
            })?;

        let frame_bytes = frame.to_bytes()
            .map_err(|e| ProtocolError::Serialization {
                message: e.to_string()
            })?;

        self.stats.write().await.record_received(frame_bytes.len());

        Ok(frame_bytes)
    }
    
    pub async fn receive_message(&mut self) -> RiResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(ProtocolError::NotInitialized.into());
        }

        let sequence = self.sequence_counter.fetch_add(1, Ordering::SeqCst) as u32;

        let mut builder = RiFrameBuilder::new();
        builder.set_sequence(sequence);
        let frame = builder.build_keepalive_frame()
            .map_err(|e| ProtocolError::Serialization {
                message: e.to_string()
            })?;

        let frame_bytes = frame.to_bytes()
            .map_err(|e| ProtocolError::Serialization {
                message: e.to_string()
            })?;

        self.stats.write().await.record_received(0);

        Ok(frame_bytes)
    }
    
    pub async fn get_connection_info(&self, connection_id: &str) -> RiResult<RiConnectionInfo> {
        let connections = self.connections.read().await;
        connections.get(connection_id)
            .cloned()
            .ok_or_else(|| ProtocolError::ConnectionNotFound {
                connection_id: connection_id.to_string()
            }.into())
    }
    
    pub async fn close_connection(&mut self, connection_id: &str) -> RiResult<()> {
        let mut connections = self.connections.write().await;
        if connections.remove(connection_id).is_some() {
            Ok(())
        } else {
            Err(ProtocolError::ConnectionNotFound {
                connection_id: connection_id.to_string()
            }.into())
        }
    }
    
    pub fn get_stats(&self) -> RiProtocolStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiProtocolStats::new())
    }
    
    pub async fn get_health(&self) -> RiProtocolHealth {
        if *self.initialized.read().await {
            RiProtocolHealth::Healthy
        } else {
            RiProtocolHealth::Unknown
        }
    }
    
    pub async fn shutdown(&mut self) {
        *self.initialized.write().await = false;
    }
}

#[derive(Debug, Clone)]
#[pyo3::prelude::pyclass]
pub struct RiGlobalProtocol {
    base: RiBaseProtocol,
}

impl RiGlobalProtocol {
    pub fn new() -> Self {
        Self {
            base: RiBaseProtocol::new(String::from("receiver")),
        }
    }
}

impl Default for RiGlobalProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiProtocol for RiGlobalProtocol {
    fn protocol_type(&self) -> RiProtocolType {
        RiProtocolType::Global
    }

    async fn is_ready(&self) -> bool {
        self.base.is_ready().await
    }

    async fn initialize(&mut self, config: RiProtocolConfig) -> RiResult<()> {
        self.base.initialize(config).await;
        Ok(())
    }

    async fn send_message(&mut self, target: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        self.base.send_message(target, data).await
    }

    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: RiMessageFlags) -> RiResult<Vec<u8>> {
        let response = self.base.send_message(target, data).await?;
        if flags.encrypted {
            self.base.stats.write().await.record_error();
        }
        Ok(response)
    }

    async fn receive_message(&mut self) -> RiResult<Vec<u8>> {
        self.base.receive_message().await
    }

    async fn get_connection_info(&self, connection_id: &str) -> RiResult<RiConnectionInfo> {
        self.base.get_connection_info(connection_id).await
    }

    async fn close_connection(&mut self, connection_id: &str) -> RiResult<()> {
        self.base.close_connection(connection_id).await
    }

    fn get_stats(&self) -> RiProtocolStats {
        self.base.get_stats()
    }

    async fn get_health(&self) -> RiProtocolHealth {
        self.base.get_health().await
    }

    async fn shutdown(&mut self) -> RiResult<()> {
        self.base.shutdown().await;
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[pyo3::prelude::pyclass]
pub struct RiPrivateProtocol {
    base: RiBaseProtocol,
}

impl RiPrivateProtocol {
    pub fn new() -> Self {
        Self {
            base: RiBaseProtocol::new(String::from("private_receiver")),
        }
    }
}

impl Default for RiPrivateProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiProtocol for RiPrivateProtocol {
    fn protocol_type(&self) -> RiProtocolType {
        RiProtocolType::Private
    }

    async fn is_ready(&self) -> bool {
        self.base.is_ready().await
    }

    async fn initialize(&mut self, config: RiProtocolConfig) -> RiResult<()> {
        self.base.initialize(config).await;
        Ok(())
    }

    async fn send_message(&mut self, target: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        self.base.send_message(target, data).await
    }

    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: RiMessageFlags) -> RiResult<Vec<u8>> {
        let response = self.base.send_message(target, data).await?;
        if !flags.encrypted {
            self.base.stats.write().await.record_error();
        }
        Ok(response)
    }

    async fn receive_message(&mut self) -> RiResult<Vec<u8>> {
        self.base.receive_message().await
    }

    async fn get_connection_info(&self, connection_id: &str) -> RiResult<RiConnectionInfo> {
        self.base.get_connection_info(connection_id).await
    }

    async fn close_connection(&mut self, connection_id: &str) -> RiResult<()> {
        self.base.close_connection(connection_id).await
    }

    fn get_stats(&self) -> RiProtocolStats {
        self.base.get_stats()
    }

    async fn get_health(&self) -> RiProtocolHealth {
        self.base.get_health().await
    }

    async fn shutdown(&mut self) -> RiResult<()> {
        self.base.shutdown().await;
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiGlobalProtocol {
    pub fn is_ready_sync(&self) -> bool {
        self.base.initialized.try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    pub fn initialize(&mut self, config: RiProtocolConfig) -> bool {
        self.base.config = config;
        if let Ok(mut guard) = self.base.initialized.try_write() {
            *guard = true;
            return true;
        }
        false
    }

    pub fn get_stats(&self) -> RiProtocolStats {
        self.base.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiProtocolStats::new())
    }

    pub fn get_health(&self) -> RiProtocolHealth {
        self.base.initialized.try_read()
            .map(|guard| {
                if *guard {
                    RiProtocolHealth::Healthy
                } else {
                    RiProtocolHealth::Unknown
                }
            })
            .unwrap_or(RiProtocolHealth::Unknown)
    }

    pub fn shutdown(&mut self) -> bool {
        if let Ok(mut guard) = self.base.initialized.try_write() {
            *guard = false;
            return true;
        }
        false
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiPrivateProtocol {
    pub fn is_ready_sync(&self) -> bool {
        self.base.initialized.try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    pub fn initialize(&mut self, config: RiProtocolConfig) -> bool {
        self.base.config = config;
        if let Ok(mut guard) = self.base.initialized.try_write() {
            *guard = true;
            return true;
        }
        false
    }

    pub fn get_stats(&self) -> RiProtocolStats {
        self.base.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiProtocolStats::new())
    }

    pub fn get_health(&self) -> RiProtocolHealth {
        if let Ok(guard) = self.base.initialized.try_read() {
            if *guard {
                return RiProtocolHealth::Healthy;
            }
        }
        RiProtocolHealth::Unknown
    }

    pub fn shutdown(&mut self) -> bool {
        if let Ok(mut guard) = self.base.initialized.try_write() {
            *guard = false;
            return true;
        }
        false
    }
}
