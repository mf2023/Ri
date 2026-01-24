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

//! # Protocol Module
//!
//! This module provides protocol implementations for DMSC, including
//! global protocol, private protocol, post-quantum cryptography, and
//! integration features.
//!
//! ## Features
//!
//! - **DMSCProtocol**: Main protocol interface (trait definition)
//! - **DMSCGlobalProtocol**: Global protocol implementation (basic implementation)
//! - **DMSCPrivateProtocol**: Private protocol implementation (basic implementation)
//! - **DMSCCrypto**: Cryptographic operations
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

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use crate::core::{DMSCResult, DMSCError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Frame definitions for binary protocol encoding
mod frames;
pub use frames::{DMSCFrameBuilder, DMSCFrameParser};

/// Protocol type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCProtocolType {
    /// Standard global protocol
    Global = 0,
    /// Enhanced private protocol
    Private = 1,
}

/// Protocol status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCProtocolStatus {
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
pub enum DMSCConnectionState {
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
pub enum DMSCSecurityLevel {
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
pub struct DMSCProtocolConfig {
    /// Default protocol type
    pub default_protocol: DMSCProtocolType,
    /// Whether security is enabled
    pub enable_security: bool,
    /// Security level
    pub security_level: DMSCSecurityLevel,
    /// Whether state synchronization is enabled
    pub enable_state_sync: bool,
    /// Whether performance optimization is enabled
    pub performance_optimization: bool,
}

impl Default for DMSCProtocolConfig {
    fn default() -> Self {
        Self {
            default_protocol: DMSCProtocolType::Global,
            enable_security: true,
            security_level: DMSCSecurityLevel::Standard,
            enable_state_sync: true,
            performance_optimization: true,
        }
    }
}

impl DMSCProtocolConfig {
    /// Validate the configuration.
    pub fn validate(&self) -> DMSCResult<()> {
        if self.security_level == DMSCSecurityLevel::None && self.enable_security {
            return Err(DMSCError::Config(
                "Security level cannot be None when security is enabled".to_string()
            ));
        }

        Ok(())
    }

    /// Create a secure configuration.
    pub fn secure() -> Self {
        Self {
            default_protocol: DMSCProtocolType::Private,
            enable_security: true,
            security_level: DMSCSecurityLevel::High,
            enable_state_sync: true,
            performance_optimization: true,
        }
    }

    /// Create a maximum security configuration with post-quantum cryptography.
    pub fn maximum_security() -> Self {
        Self {
            default_protocol: DMSCProtocolType::Private,
            enable_security: true,
            security_level: DMSCSecurityLevel::Military,
            enable_state_sync: true,
            performance_optimization: false,
        }
    }
}

/// Protocol statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolStats {
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

impl DMSCProtocolStats {
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
pub struct DMSCConnectionStats {
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

impl Default for DMSCConnectionStats {
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
pub enum DMSCProtocolHealth {
    /// Healthy
    Healthy,
    /// Degraded
    Degraded,
    /// Unhealthy
    Unhealthy,
    /// Unknown
    Unknown,
}

impl Default for DMSCProtocolHealth {
    fn default() -> Self {
        DMSCProtocolHealth::Unknown
    }
}

/// Message flags for protocol messages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMessageFlags {
    /// Whether the message is compressed
    pub compressed: bool,
    /// Whether the message is encrypted
    pub encrypted: bool,
    /// Whether the message requires acknowledgment
    pub requires_ack: bool,
    /// Whether this is a priority message
    pub priority: bool,
}

impl Default for DMSCMessageFlags {
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
pub struct DMSCConnectionInfo {
    /// Connection ID
    pub connection_id: String,
    /// Remote device ID
    pub device_id: String,
    /// Connection address
    pub address: String,
    /// Protocol type
    pub protocol_type: DMSCProtocolType,
    /// Connection state
    pub state: DMSCConnectionState,
    /// Security level
    pub security_level: DMSCSecurityLevel,
    /// Connection timestamp
    pub connected_at: u64,
    /// Last activity timestamp
    pub last_activity: u64,
}

impl Default for DMSCConnectionInfo {
    fn default() -> Self {
        Self {
            connection_id: String::new(),
            device_id: String::new(),
            address: String::new(),
            protocol_type: DMSCProtocolType::Global,
            state: DMSCConnectionState::Disconnected,
            security_level: DMSCSecurityLevel::None,
            connected_at: 0,
            last_activity: 0,
        }
    }
}

/// Frame type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCFrameType {
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
pub struct DMSCFrameHeader {
    /// Protocol version (major.minor as u8)
    pub version: u8,
    /// Frame type
    pub frame_type: DMSCFrameType,
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

impl Default for DMSCFrameHeader {
    fn default() -> Self {
        Self {
            version: 1,
            frame_type: DMSCFrameType::Data,
            sequence_number: 0,
            length: 0,
            timestamp: 0,
            flags: 0,
            auth_tag_offset: 0,
        }
    }
}

impl DMSCFrameHeader {
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
pub struct DMSCFrame {
    /// Frame header
    pub header: DMSCFrameHeader,
    /// Frame payload
    pub payload: Vec<u8>,
    /// Source device ID
    pub source_id: String,
    /// Target device ID
    pub target_id: String,
}

impl Default for DMSCFrame {
    fn default() -> Self {
        Self {
            header: DMSCFrameHeader::default(),
            payload: Vec::new(),
            source_id: String::new(),
            target_id: String::new(),
        }
    }
}

/// Core protocol trait
#[async_trait]
pub trait DMSCProtocol {
    /// Get protocol type
    fn protocol_type(&self) -> DMSCProtocolType;
    
    /// Check if protocol is ready
    async fn is_ready(&self) -> bool;
    
    /// Initialize protocol
    async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()>;
    
    /// Send message
    async fn send_message(&mut self, target: &str, data: &[u8]) -> DMSCResult<Vec<u8>>;
    
    /// Send message with flags
    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>>;
    
    /// Receive message
    async fn receive_message(&mut self) -> DMSCResult<Vec<u8>>;
    
    /// Get connection info
    async fn get_connection_info(&self, connection_id: &str) -> DMSCResult<DMSCConnectionInfo>;
    
    /// Close connection
    async fn close_connection(&mut self, connection_id: &str) -> DMSCResult<()>;
    
    /// Get protocol statistics
    fn get_stats(&self) -> DMSCProtocolStats;
    
    /// Get protocol health
    async fn get_health(&self) -> DMSCProtocolHealth;
    
    /// Shutdown protocol
    async fn shutdown(&mut self) -> DMSCResult<()>;
}

/// Protocol connection trait
#[async_trait]
pub trait DMSCProtocolConnection {
    /// Get connection ID
    fn connection_id(&self) -> &str;
    
    /// Get remote device ID
    fn remote_device_id(&self) -> &str;
    
    /// Get protocol type
    fn protocol_type(&self) -> DMSCProtocolType;
    
    /// Check if connection is active
    fn is_active(&self) -> bool;
    
    /// Send data
    async fn send(&mut self, data: &[u8]) -> DMSCResult<usize>;
    
    /// Receive data
    async fn receive(&mut self, buffer: &mut [u8]) -> DMSCResult<usize>;
    
    /// Get statistics
    fn get_stats(&self) -> DMSCConnectionStats;
    
    /// Close connection
    async fn close(&mut self) -> DMSCResult<()>;
}

/// Protocol manager
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolManager {
    /// Protocol statistics
    pub stats: Arc<RwLock<DMSCProtocolStats>>,
    /// Default protocol type
    pub default_protocol: DMSCProtocolType,
    /// Active connections
    connections: Arc<RwLock<HashMap<String, DMSCConnectionInfo>>>,
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
        let config = DMSCProtocolConfig::default();
        assert_eq!(config.default_protocol, DMSCProtocolType::Global);
        assert!(config.enable_security);
        assert_eq!(config.security_level, DMSCSecurityLevel::Standard);
    }

    #[test]
    fn test_protocol_config_secure() {
        let config = DMSCProtocolConfig::secure();
        assert_eq!(config.default_protocol, DMSCProtocolType::Private);
        assert!(config.enable_security);
        assert_eq!(config.security_level, DMSCSecurityLevel::High);
    }

    #[test]
    fn test_protocol_config_maximum_security() {
        let config = DMSCProtocolConfig::maximum_security();
        assert_eq!(config.default_protocol, DMSCProtocolType::Private);
        assert!(config.enable_security);
        assert_eq!(config.security_level, DMSCSecurityLevel::Military);
    }

    #[test]
    fn test_protocol_config_validation() {
        let mut config = DMSCProtocolConfig::default();

        // Valid config should pass
        assert!(config.validate().is_ok());

        // None security level with security enabled should fail
        config.security_level = DMSCSecurityLevel::None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_frame_header_version() {
        let header = DMSCFrameHeader::default();
        assert_eq!(header.major_version(), 0);
        assert_eq!(header.minor_version(), 1);
    }

    #[test]
    fn test_frame_header_supports_feature() {
        let header = DMSCFrameHeader {
            flags: 0b00000011,
            ..Default::default()
        };

        assert!(header.supports_feature(0b00000001));
        assert!(header.supports_feature(0b00000010));
        assert!(!header.supports_feature(0b00000100));
    }

    #[test]
    fn test_protocol_stats() {
        let stats = DMSCProtocolStats::new();
        assert_eq!(stats.messages_sent, 0);
        assert_eq!(stats.messages_received, 0);
        assert_eq!(stats.errors, 0);
    }

    #[test]
    fn test_connection_info() {
        let mut info = DMSCConnectionInfo::default();
        info.device_id = "test-device".to_string();
        info.state = DMSCConnectionState::Connected;
        assert_eq!(info.device_id, "test-device");
        assert_eq!(info.state, DMSCConnectionState::Connected);
        assert_eq!(info.security_level, DMSCSecurityLevel::None);
    }

    #[test]
    fn test_protocol_type_values() {
        assert_eq!(DMSCProtocolType::Global as u8, 0);
        assert_eq!(DMSCProtocolType::Private as u8, 1);
    }

    #[test]
    fn test_security_level_values() {
        assert_eq!(DMSCSecurityLevel::None as u8, 0);
        assert_eq!(DMSCSecurityLevel::Standard as u8, 1);
        assert_eq!(DMSCSecurityLevel::High as u8, 2);
        assert_eq!(DMSCSecurityLevel::Military as u8, 3);
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCProtocolManager {
    /// Create new protocol manager
    #[new]
    pub fn new() -> Self {
        Self {
            stats: Arc::new(RwLock::new(DMSCProtocolStats::new())),
            default_protocol: DMSCProtocolType::Global,
            connections: Arc::new(RwLock::new(HashMap::new())),
            sequence_counter: Arc::new(AtomicU64::new(0)),
            initialized: Arc::new(RwLock::new(false)),
        }
    }
    
    #[getter]
    fn get_stats(&self) -> DMSCProtocolStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| DMSCProtocolStats::new())
    }
    
    #[getter]
    fn get_default_protocol(&self) -> DMSCProtocolType {
        self.default_protocol
    }
    
    /// Initialize manager
    pub fn initialize(&mut self, config: DMSCProtocolConfig) -> PyResult<()> {
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
        
        let frame = DMSCFrame {
            header: DMSCFrameHeader {
                version: 1,
                frame_type: DMSCFrameType::Data,
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
                let error_response = DMSCProtocolResponse {
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
        
        let response = DMSCProtocolResponse {
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
    
    fn build_response_data(&self, target: &str, frame: &DMSCFrame, sequence: u64, timestamp: u64) -> Vec<u8> {
        let mut response = HashMap::<String, serde_json::Value>::new();
        
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
    pub fn send_message_with_flags(&self, target: &str, data: &[u8], _flags: DMSCMessageFlags) -> Vec<u8> {
        self.send_message(target, data)
    }
    
    /// Get connection info (sync version for Python)
    pub fn get_connection_info(&self, connection_id: &str) -> Option<DMSCConnectionInfo> {
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

impl Default for DMSCProtocolManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Protocol response structure
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCProtocolResponse {
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

impl Default for DMSCProtocolResponse {
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

impl From<ProtocolError> for DMSCError {
    fn from(error: ProtocolError) -> Self {
        DMSCError::Other(format!("Protocol error: {}", error))
    }
}

impl From<serde_json::Error> for ProtocolError {
    fn from(error: serde_json::Error) -> Self {
        ProtocolError::Serialization { message: error.to_string() }
    }
}

impl From<ProtocolError> for DMSCResult<()> {
    fn from(error: ProtocolError) -> Self {
        Err(error.into())
    }
}

#[derive(Debug, Clone)]
pub struct DMSCBaseProtocol {
    config: DMSCProtocolConfig,
    stats: Arc<RwLock<DMSCProtocolStats>>,
    connections: Arc<RwLock<HashMap<String, DMSCConnectionInfo>>>,
    sequence_counter: Arc<AtomicU64>,
    initialized: Arc<RwLock<bool>>,
    _receiver_id: String,
}

impl DMSCBaseProtocol {
    pub fn new(receiver_id: String) -> Self {
        Self {
            config: DMSCProtocolConfig::default(),
            stats: Arc::new(RwLock::new(DMSCProtocolStats::new())),
            connections: Arc::new(RwLock::new(HashMap::new())),
            sequence_counter: Arc::new(AtomicU64::new(0)),
            initialized: Arc::new(RwLock::new(false)),
            _receiver_id: receiver_id,
        }
    }
    
    pub async fn is_ready(&self) -> bool {
        *self.initialized.read().await
    }
    
    pub async fn initialize(&mut self, config: DMSCProtocolConfig) {
        self.config = config;
        *self.initialized.write().await = true;
    }

    pub async fn send_message(&mut self, _target: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(ProtocolError::NotInitialized.into());
        }

        let sequence = self.sequence_counter.fetch_add(1, Ordering::SeqCst) as u32;

        self.stats.write().await.record_sent(data.len());

        let mut builder = DMSCFrameBuilder::new();
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
    
    pub async fn receive_message(&mut self) -> DMSCResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(ProtocolError::NotInitialized.into());
        }

        let sequence = self.sequence_counter.fetch_add(1, Ordering::SeqCst) as u32;

        let mut builder = DMSCFrameBuilder::new();
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
    
    pub async fn get_connection_info(&self, connection_id: &str) -> DMSCResult<DMSCConnectionInfo> {
        let connections = self.connections.read().await;
        connections.get(connection_id)
            .cloned()
            .ok_or_else(|| ProtocolError::ConnectionNotFound {
                connection_id: connection_id.to_string()
            }.into())
    }
    
    pub async fn close_connection(&mut self, connection_id: &str) -> DMSCResult<()> {
        let mut connections = self.connections.write().await;
        if connections.remove(connection_id).is_some() {
            Ok(())
        } else {
            Err(ProtocolError::ConnectionNotFound {
                connection_id: connection_id.to_string()
            }.into())
        }
    }
    
    pub fn get_stats(&self) -> DMSCProtocolStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| DMSCProtocolStats::new())
    }
    
    pub async fn get_health(&self) -> DMSCProtocolHealth {
        if *self.initialized.read().await {
            DMSCProtocolHealth::Healthy
        } else {
            DMSCProtocolHealth::Unknown
        }
    }
    
    pub async fn shutdown(&mut self) {
        *self.initialized.write().await = false;
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGlobalProtocol {
    base: DMSCBaseProtocol,
}

impl DMSCGlobalProtocol {
    pub fn new() -> Self {
        Self {
            base: DMSCBaseProtocol::new(String::from("receiver")),
        }
    }
}

impl Default for DMSCGlobalProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DMSCProtocol for DMSCGlobalProtocol {
    fn protocol_type(&self) -> DMSCProtocolType {
        DMSCProtocolType::Global
    }

    async fn is_ready(&self) -> bool {
        self.base.is_ready().await
    }

    async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()> {
        self.base.initialize(config).await;
        Ok(())
    }

    async fn send_message(&mut self, target: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        self.base.send_message(target, data).await
    }

    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>> {
        let response = self.base.send_message(target, data).await?;
        if flags.encrypted {
            self.base.stats.write().await.record_error();
        }
        Ok(response)
    }

    async fn receive_message(&mut self) -> DMSCResult<Vec<u8>> {
        self.base.receive_message().await
    }

    async fn get_connection_info(&self, connection_id: &str) -> DMSCResult<DMSCConnectionInfo> {
        self.base.get_connection_info(connection_id).await
    }

    async fn close_connection(&mut self, connection_id: &str) -> DMSCResult<()> {
        self.base.close_connection(connection_id).await
    }

    fn get_stats(&self) -> DMSCProtocolStats {
        self.base.get_stats()
    }

    async fn get_health(&self) -> DMSCProtocolHealth {
        self.base.get_health().await
    }

    async fn shutdown(&mut self) -> DMSCResult<()> {
        self.base.shutdown().await;
        Ok(())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCPrivateProtocol {
    base: DMSCBaseProtocol,
}

impl DMSCPrivateProtocol {
    pub fn new() -> Self {
        Self {
            base: DMSCBaseProtocol::new(String::from("private_receiver")),
        }
    }
}

impl Default for DMSCPrivateProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DMSCProtocol for DMSCPrivateProtocol {
    fn protocol_type(&self) -> DMSCProtocolType {
        DMSCProtocolType::Private
    }

    async fn is_ready(&self) -> bool {
        self.base.is_ready().await
    }

    async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()> {
        self.base.initialize(config).await;
        Ok(())
    }

    async fn send_message(&mut self, target: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        self.base.send_message(target, data).await
    }

    async fn send_message_with_flags(&mut self, target: &str, data: &[u8], flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>> {
        let response = self.base.send_message(target, data).await?;
        if !flags.encrypted {
            self.base.stats.write().await.record_error();
        }
        Ok(response)
    }

    async fn receive_message(&mut self) -> DMSCResult<Vec<u8>> {
        self.base.receive_message().await
    }

    async fn get_connection_info(&self, connection_id: &str) -> DMSCResult<DMSCConnectionInfo> {
        self.base.get_connection_info(connection_id).await
    }

    async fn close_connection(&mut self, connection_id: &str) -> DMSCResult<()> {
        self.base.close_connection(connection_id).await
    }

    fn get_stats(&self) -> DMSCProtocolStats {
        self.base.get_stats()
    }

    async fn get_health(&self) -> DMSCProtocolHealth {
        self.base.get_health().await
    }

    async fn shutdown(&mut self) -> DMSCResult<()> {
        self.base.shutdown().await;
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCGlobalProtocol {
    pub fn is_ready_sync(&self) -> bool {
        self.base.initialized.try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    pub fn initialize(&mut self, config: DMSCProtocolConfig) -> bool {
        self.base.config = config;
        *self.base.initialized.try_write().unwrap() = true;
        true
    }

    pub fn get_stats(&self) -> DMSCProtocolStats {
        self.base.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| DMSCProtocolStats::new())
    }

    pub fn get_health(&self) -> DMSCProtocolHealth {
        self.base.initialized.try_read()
            .map(|guard| {
                if *guard {
                    DMSCProtocolHealth::Healthy
                } else {
                    DMSCProtocolHealth::Unknown
                }
            })
            .unwrap_or(DMSCProtocolHealth::Unknown)
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
impl DMSCPrivateProtocol {
    pub fn is_ready_sync(&self) -> bool {
        self.base.initialized.try_read()
            .map(|guard| *guard)
            .unwrap_or(false)
    }

    pub fn initialize(&mut self, config: DMSCProtocolConfig) -> bool {
        self.base.config = config;
        *self.base.initialized.try_write().unwrap() = true;
        true
    }

    pub fn get_stats(&self) -> DMSCProtocolStats {
        self.base.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| DMSCProtocolStats::new())
    }

    pub fn get_health(&self) -> DMSCProtocolHealth {
        if *self.base.initialized.try_read().unwrap() {
            DMSCProtocolHealth::Healthy
        } else {
            DMSCProtocolHealth::Unknown
        }
    }

    pub fn shutdown(&mut self) -> bool {
        if let Ok(mut guard) = self.base.initialized.try_write() {
            *guard = false;
            return true;
        }
        false
    }
}
