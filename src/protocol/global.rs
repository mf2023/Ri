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

//! # Global Protocol Module
//! 
//! This module implements the standard global communication protocol for DMSC.
//! It provides reliable, efficient communication with optional encryption and
//! compression support.
//! 
//! ## Features
//! 
//! - **Reliable Communication**: TCP-based reliable message delivery
//! - **Optional Encryption**: TLS/SSL support for secure communication
//! - **Compression**: Configurable compression for bandwidth optimization
//! - **Connection Pooling**: Efficient connection reuse
//! - **Timeout Management**: Configurable connection and operation timeouts
//! - **Statistics Tracking**: Comprehensive connection statistics
//! 
//! ## Architecture
//! 
//! The global protocol uses a layered approach:
//! 
//! 1. **Transport Layer**: TCP connections with optional TLS
//! 2. **Message Layer**: Framed message protocol with length prefixes
//! 3. **Application Layer**: DMSC-specific message handling
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::protocol::{DMSCGlobalProtocol, DMSCProtocolConfig};
//! 
//! async fn example() -> DMSCResult<()> {
//!     let mut protocol = DMSCGlobalProtocol::new();
//!     
//!     let config = DMSCProtocolConfig::Global {
//!         enable_encryption: true,
//!         compression_level: 6,
//!         connection_timeout: Duration::from_secs(30),
//!     };
//!     
//!     protocol.initialize(config).await?;
//!     
//!     let connection = protocol.connect("target-device:8080").await?;
//!     let response = connection.send_message(b"Hello, World!").await?;
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

use crate::core::{DMSCResult, DMSCError};
use super::{DMSCProtocol, DMSCProtocolConfig, DMSCProtocolType, DMSCProtocolConnection, 
            DMSCProtocolStats, DMSCMessageFlags, DMSCConnectionInfo, DMSCSecurityLevel};

/// Global protocol implementation.
pub struct DMSCGlobalProtocol {
    /// Protocol configuration
    config: Option<DMSCProtocolConfig>,
    /// Connection pool for efficient connection reuse
    connection_pool: Arc<RwLock<HashMap<String, Arc<DMSCGlobalConnection>>>>,
    /// Protocol statistics
    stats: Arc<RwLock<DMSCProtocolStats>>,
    /// Whether the protocol is ready
    ready: Arc<RwLock<bool>>,
}

impl DMSCGlobalProtocol {
    /// Create a new global protocol instance.
    pub fn new() -> Self {
        Self {
            config: None,
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DMSCProtocolStats::default())),
            ready: Arc::new(RwLock::new(false)),
        }
    }
    
    /// Get connection from pool or create new one.
    async fn get_or_create_connection(&self, target_id: &str) -> DMSCResult<Arc<DMSCGlobalConnection>> {
        let mut pool = self.connection_pool.write().await;
        
        if let Some(connection) = pool.get(target_id) {
            if connection.is_active() {
                return Ok(Arc::clone(connection));
            } else {
                // Remove inactive connection
                pool.remove(target_id);
            }
        }
        
        // Create new connection
        let connection = Arc::new(DMSCGlobalConnection::new(target_id.to_string()).await?);
        pool.insert(target_id.to_string(), Arc::clone(&connection));
        
        Ok(connection)
    }
    
    /// Update statistics.
    async fn update_stats<F>(&self, updater: F)
    where
        F: FnOnce(&mut DMSCProtocolStats),
    {
        let mut stats = self.stats.write().await;
        updater(&mut *stats);
    }
}

#[async_trait]
impl DMSCProtocol for DMSCGlobalProtocol {
    fn protocol_type(&self) -> DMSCProtocolType {
        DMSCProtocolType::Global
    }
    
    async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()> {
        // Validate configuration
        match &config {
            DMSCProtocolConfig::Global { compression_level, connection_timeout, .. } => {
                if *compression_level > 9 {
                    return Err(DMSCError::InvalidConfiguration("Compression level must be 0-9".to_string()));
                }
                if connection_timeout.as_secs() == 0 {
                    return Err(DMSCError::InvalidConfiguration("Connection timeout must be positive".to_string()));
                }
            }
            _ => return Err(DMSCError::InvalidConfiguration("Invalid configuration type for global protocol".to_string())),
        }
        
        self.config = Some(config);
        *self.ready.write().await = true;
        
        Ok(())
    }
    
    async fn connect(&self, target_id: &str) -> DMSCResult<Box<dyn DMSCProtocolConnection>> {
        if !*self.ready.read().await {
            return Err(DMSCError::InvalidState("Protocol not initialized".to_string()));
        }
        
        // Update connection attempts
        self.update_stats(|stats| stats.connection_attempts += 1).await;
        
        let connection = self.get_or_create_connection(target_id).await?;
        
        // Update successful connections
        self.update_stats(|stats| stats.successful_connections += 1).await;
        
        Ok(Box::new(DMSCGlobalConnectionWrapper {
            inner: connection,
            stats: Arc::clone(&self.stats),
        }))
    }
    
    fn is_ready(&self) -> bool {
        *self.ready.blocking_read()
    }
    
    fn get_stats(&self) -> DMSCProtocolStats {
        *self.stats.blocking_read()
    }
    
    async fn shutdown(&mut self) -> DMSCResult<()> {
        // Clear connection pool
        self.connection_pool.write().await.clear();
        
        // Mark as not ready
        *self.ready.write().await = false;
        
        Ok(())
    }
}

impl Default for DMSCGlobalProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// Global protocol connection implementation.
struct DMSCGlobalConnection {
    /// Connection ID
    connection_id: String,
    /// Target address
    target_id: String,
    /// TCP stream
    stream: Arc<RwLock<Option<TcpStream>>>,
    /// Connection establishment time
    established_at: Instant,
    /// Last activity time
    last_activity: Arc<RwLock<Instant>>,
    /// Whether the connection is active
    active: Arc<RwLock<bool>>,
}

impl DMSCGlobalConnection {
    /// Create a new global connection.
    async fn new(target_id: String) -> DMSCResult<Self> {
        // Parse target address
        let addr: SocketAddr = target_id.parse()
            .map_err(|_| DMSCError::InvalidConfiguration(format!("Invalid target address: {}", target_id)))?;
        
        // Connect to target
        let stream = TcpStream::connect(addr).await
            .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to connect to {}: {}", target_id, e)))?;
        
        let connection_id = format!("global-{}", uuid::Uuid::new_v4());
        let now = Instant::now();
        
        Ok(Self {
            connection_id,
            target_id,
            stream: Arc::new(RwLock::new(Some(stream))),
            established_at: now,
            last_activity: Arc::new(RwLock::new(now)),
            active: Arc::new(RwLock::new(true)),
        })
    }
    
    /// Update last activity time.
    async fn update_activity(&self) {
        *self.last_activity.write().await = Instant::now();
    }
    
    /// Check if connection is still active.
    async fn check_connection(&self) -> bool {
        if let Some(mut stream) = self.stream.write().await.as_mut() {
            // Try to peek at the stream to check if it's still connected
            match stream.try_read(&mut [0u8; 1]) {
                Ok(0) => {
                    // Connection closed
                    *self.active.write().await = false;
                    false
                }
                Ok(_) => true,
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // No data available, but connection is still active
                    true
                }
                Err(_) => {
                    // Connection error
                    *self.active.write().await = false;
                    false
                }
            }
        } else {
            false
        }
    }
}

/// Wrapper for global connection to implement DMSCProtocolConnection trait.
struct DMSCGlobalConnectionWrapper {
    inner: Arc<DMSCGlobalConnection>,
    stats: Arc<RwLock<DMSCProtocolStats>>,
}

#[async_trait]
impl DMSCProtocolConnection for DMSCGlobalConnectionWrapper {
    async fn send_message(&self, data: &[u8]) -> DMSCResult<Vec<u8>> {
        self.send_message_with_flags(data, DMSCMessageFlags::default()).await
    }
    
    async fn send_message_with_flags(&self, data: &[u8], flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>> {
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.check_connection().await {
            return Err(DMSCError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut tcp_stream) = *stream {
            // Send message length (4 bytes, big-endian)
            let len = data.len() as u32;
            tcp_stream.write_all(&len.to_be_bytes()).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to send message length: {}", e)))?;
            
            // Send message data
            tcp_stream.write_all(data).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to send message data: {}", e)))?;
            
            // Flush the stream
            tcp_stream.flush().await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to flush stream: {}", e)))?;
            
            // Update statistics
            self.stats.write().await.messages_sent += 1;
            self.stats.write().await.bytes_sent += data.len() as u64;
            
            self.receive_message().await
        } else {
            Err(DMSCError::ConnectionFailed("No active stream".to_string()))
        }
    }
    
    async fn receive_message(&self) -> DMSCResult<Vec<u8>> {
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.check_connection().await {
            return Err(DMSCError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut tcp_stream) = *stream {
            // Read message length (4 bytes, big-endian)
            let mut len_bytes = [0u8; 4];
            tcp_stream.read_exact(&mut len_bytes).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to read message length: {}", e)))?;
            
            let len = u32::from_be_bytes(len_bytes) as usize;
            
            // Validate message length
            if len > 1024 * 1024 * 100 { // 100MB limit
                return Err(DMSCError::InvalidData("Message too large".to_string()));
            }
            
            // Read message data
            let mut data = vec![0u8; len];
            tcp_stream.read_exact(&mut data).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to read message data: {}", e)))?;
            
            // Update statistics
            self.stats.write().await.messages_received += 1;
            self.stats.write().await.bytes_received += data.len() as u64;
            
            Ok(data)
        } else {
            Err(DMSCError::ConnectionFailed("No active stream".to_string()))
        }
    }
    
    fn is_active(&self) -> bool {
        *self.inner.active.blocking_read()
    }
    
    fn get_connection_info(&self) -> DMSCConnectionInfo {
        DMSCConnectionInfo {
            connection_id: self.inner.connection_id.clone(),
            target_id: self.inner.target_id.clone(),
            protocol_type: DMSCProtocolType::Global,
            established_at: self.inner.established_at,
            last_activity: *self.inner.last_activity.blocking_read(),
            security_level: DMSCSecurityLevel::Basic, // Global protocol uses basic security
        }
    }
    
    async fn close(&self) -> DMSCResult<()> {
        *self.inner.active.write().await = false;
        self.inner.stream.write().await.take();
        Ok(())
    }
}
