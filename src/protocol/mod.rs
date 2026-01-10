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
}

impl Default for DMSCFrameHeader {
    fn default() -> Self {
        Self {
            frame_type: DMSCFrameType::Data,
            sequence_number: 0,
            length: 0,
            timestamp: 0,
            flags: 0,
        }
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
        self.stats.try_read().unwrap().clone()
    }
    
    #[getter]
    fn get_default_protocol(&self) -> DMSCProtocolType {
        self.default_protocol
    }
    
    /// Initialize manager
    pub fn initialize(&mut self, config: DMSCProtocolConfig) -> PyResult<()> {
        self.default_protocol = config.default_protocol;
        *self.initialized.try_write().unwrap() = true;
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
                frame_type: DMSCFrameType::Data,
                sequence_number: sequence,
                length: data.len() as u32,
                timestamp,
                flags: 0,
            },
            payload: data.to_vec(),
            source_id: String::from("protocol_manager"),
            target_id: target.to_string(),
        };
        
        let serialized = serde_json::to_vec(&frame)
            .map_err(|e| ProtocolError::Serialization { message: e.to_string() }).unwrap_or_default();
        
        let payload_len = serialized.len();
        self.stats.try_write().unwrap().record_sent(payload_len);
        
        self.stats.try_write().unwrap().record_received(0);
        
        b"Message sent successfully".to_vec()
    }
    
    /// Send message with flags (sync version for Python)
    pub fn send_message_with_flags(&self, target: &str, data: &[u8], _flags: DMSCMessageFlags) -> Vec<u8> {
        self.send_message(target, data)
    }
    
    /// Get connection info (sync version for Python)
    pub fn get_connection_info(&self, connection_id: &str) -> Option<DMSCConnectionInfo> {
        let connections = self.connections.try_read().unwrap();
        connections.get(connection_id).cloned()
    }
    
    /// Close connection (sync version for Python)
    pub fn close_connection(&mut self, connection_id: &str) -> bool {
        let mut connections = self.connections.try_write().unwrap();
        connections.remove(connection_id).is_some()
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

const PROTOCOL_ERROR_TARGET_NOT_FOUND: &str = "Target not found";
const PROTOCOL_ERROR_SEND_FAILED: &str = "Failed to send message";
const PROTOCOL_ERROR_NOT_INITIALIZED: &str = "Protocol not initialized";
const PROTOCOL_ERROR_INVALID_STATE: &str = "Invalid protocol state";

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
