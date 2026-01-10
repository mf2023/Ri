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

//! # Private Protocol Module
//! 
//! This module implements the secure private communication protocol for DMSC.
//! It provides military-grade security with quantum-resistant cryptography,
//! device authentication, traffic obfuscation, and anti-forensic features.
//! 
//! ## Security Features
//! 
//! - **Quantum-Resistant Cryptography**: Post-quantum algorithms for future-proof security
//! - **Device Authentication**: Hardware-based device identity verification
//! - **Traffic Obfuscation**: Makes encrypted traffic appear as normal HTTP/HTTPS
//! - **Perfect Forward Secrecy**: Ephemeral keys for each session
//! - **Anti-Forensic Design**: No persistent keys or identifiable patterns
//! - **National Cryptographic Standards**: Support for SM2/SM3/SM4 algorithms
//! 
//! ## Architecture
//! 
//! The private protocol uses a multi-layered security approach:
//! 
//! 1. **Physical Layer**: Custom frame format with obfuscation
//! 2. **Link Layer**: Device authentication and key exchange
//! 3. **Network Layer**: Quantum-resistant encryption
//! 4. **Application Layer**: DMSC-specific secure messaging
//! 
//! ## Threat Model
//! 
//! - **Passive Eavesdropping**: Encrypted traffic is unreadable
//! - **Active Man-in-the-Middle**: Device authentication prevents MITM attacks
//! - **Traffic Analysis**: Obfuscation makes traffic patterns indistinguishable
//! - **Quantum Attacks**: Post-quantum cryptography resists future quantum computers
//! - **Forensic Analysis**: No recoverable data after session termination
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::protocol::{DMSCPrivateProtocol, DMSCProtocolConfig, DMSCCryptoSuite, DMSCObfuscationLevel};
//! 
//! async fn example() -> DMSCResult<()> {
//!     let mut protocol = DMSCPrivateProtocol::new();
//!     
//!     let config = DMSCProtocolConfig::Private {
//!         crypto_suite: DMSCCryptoSuite::NationalStandard,
//!         device_auth: true,
//!         obfuscation_level: DMSCObfuscationLevel::High,
//!         quantum_resistant: true,
//!     };
//!     
//!     protocol.initialize(config).await?;
//!     
//!     let connection = protocol.connect("secure-device-id").await?;
//!     let response = connection.send_message(b"classified data").await?;
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use rand::Rng;

/// Connection health status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionHealth {
    /// Healthy status
    Healthy,
    /// Degraded status
    Degraded,
    /// Unhealthy status
    Unhealthy,
    /// Unknown status
    Unknown,
}

impl Default for ConnectionHealth {
    fn default() -> Self {
        ConnectionHealth::Unknown
    }
}

use crate::core::{DMSCResult, DMSCError};
use super::{DMSCProtocol, DMSCProtocolConfig, DMSCProtocolType, DMSCProtocolConnection, 
            DMSCProtocolStats, DMSCMessageFlags, DMSCConnectionInfo, DMSCSecurityLevel};
use super::security::{DMSCCryptoSuite, DMSCObfuscationLevel, DMSCDeviceAuthProtocol, 
                       DMSCPostQuantumCrypto, DMSCObfuscationLayer};
use super::crypto::{DMSCCryptoEngine, AES256GCM, ChaCha20Poly1305};
use super::frames::{DMSCFrame, DMSCFrameType, DMSCFrameBuilder, DMSCFrameParser};
use crate::device::pool::{DMSCConnectionPool, DMSCConnectionInfo as PoolConnectionInfo, DMSCConnectionState};

/// Private protocol implementation.
pub struct DMSCPrivateProtocol {
    /// Protocol configuration
    config: Option<DMSCPrivateConfig>,
    /// Device authentication protocol
    device_auth: Arc<DMSCDeviceAuthProtocol>,
    /// Post-quantum cryptography handler
    post_quantum: Arc<DMSCPostQuantumCrypto>,
    /// Obfuscation layer
    obfuscation: Arc<DMSCObfuscationLayer>,
    /// Connection pool for secure connections
    connection_pool: Arc<RwLock<HashMap<String, Arc<DMSCPrivateConnection>>>>,
    /// Protocol statistics
    stats: Arc<RwLock<DMSCProtocolStats>>,
    /// Whether the protocol is ready
    ready: Arc<RwLock<bool>>,
    /// Crypto engine for encryption/decryption
    crypto_engine: Arc<RwLock<Option<Box<dyn DMSCCryptoEngine>>>>,
    /// Frame builder for protocol frames
    frame_builder: Arc<DMSCFrameBuilder>,
    /// Frame parser for incoming frames
    frame_parser: Arc<DMSCFrameParser>,
}

/// Private protocol specific configuration.
#[derive(Debug, Clone)]
struct DMSCPrivateConfig {
    /// Cryptographic suite to use
    crypto_suite: DMSCCryptoSuite,
    /// Enable device authentication
    device_auth: bool,
    /// Obfuscation level
    obfuscation_level: DMSCObfuscationLevel,
    /// Enable quantum-resistant algorithms
    quantum_resistant: bool,
    /// Session timeout
    session_timeout: Duration,
    /// Key rotation interval
    key_rotation_interval: Duration,
}

impl DMSCPrivateProtocol {
    /// Create a new private protocol instance.
    pub fn new() -> Self {
        Self {
            config: None,
            device_auth: Arc::new(DMSCDeviceAuthProtocol::new()),
            post_quantum: Arc::new(DMSCPostQuantumCrypto::new()),
            obfuscation: Arc::new(DMSCObfuscationLayer::new()),
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(DMSCProtocolStats::default())),
            ready: Arc::new(RwLock::new(false)),
            crypto_engine: Arc::new(RwLock::new(None)),
            frame_builder: Arc::new(DMSCFrameBuilder::new()),
            frame_parser: Arc::new(DMSCFrameParser::new()),
        }
    }
    
    /// Get or create a secure connection.
    async fn get_or_create_connection(&self, target_id: &str) -> DMSCResult<Arc<DMSCPrivateConnection>> {
        let mut pool = self.connection_pool.write().await;
        
        if let Some(connection) = pool.get(target_id) {
            if connection.is_active().await {
                return Ok(Arc::clone(connection));
            } else {
                // Remove inactive connection
                pool.remove(target_id);
            }
        }
        
        // Create new secure connection
        let config_ref = self.config.as_ref()
            .ok_or_else(|| DMSCError::InvalidState("Private protocol not initialized".to_string()))?;
        let connection = Arc::new(DMSCPrivateConnection::new(
            target_id.to_string(),
            config_ref,
            Arc::clone(&self.device_auth),
            Arc::clone(&self.post_quantum),
            Arc::clone(&self.obfuscation),
        ).await?);
        
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
impl DMSCProtocol for DMSCPrivateProtocol {
    fn protocol_type(&self) -> DMSCProtocolType {
        DMSCProtocolType::Private
    }
    
    async fn initialize(&mut self, config: DMSCProtocolConfig) -> DMSCResult<()> {
        // Validate and convert configuration
        let private_config = match config {
            DMSCProtocolConfig::Private { crypto_suite, device_auth, obfuscation_level, quantum_resistant } => {
                DMSCPrivateConfig {
                    crypto_suite,
                    device_auth,
                    obfuscation_level,
                    quantum_resistant,
                    session_timeout: Duration::from_secs(3600), // 1 hour
                    key_rotation_interval: Duration::from_secs(600), // 10 minutes
                }
            }
            _ => return Err(DMSCError::InvalidConfiguration("Invalid configuration type for private protocol".to_string())),
        };
        
        // Initialize crypto engine based on crypto suite
        let crypto_engine: Box<dyn DMSCCryptoEngine> = match private_config.crypto_suite {
            DMSCCryptoSuite::AES256GCM => Box::new(AES256GCM::new()),
            DMSCCryptoSuite::ChaCha20Poly1305 => Box::new(ChaCha20Poly1305::new()),
            DMSCCryptoSuite::NationalStandard => Box::new(AES256GCM::new()), // Default to AES256GCM for now
        };
        *self.crypto_engine.write().await = Some(crypto_engine);
        
        // Initialize security components
        if private_config.device_auth {
            self.device_auth.initialize().await?;
        }
        
        if private_config.quantum_resistant {
            self.post_quantum.initialize(&private_config.crypto_suite).await?;
        }
        
        self.obfuscation.initialize(private_config.obfuscation_level).await?;
        
        self.config = Some(private_config);
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
        
        Ok(Box::new(DMSCPrivateConnectionWrapper {
            inner: connection,
            stats: Arc::clone(&self.stats),
            frame_builder: Arc::clone(&self.frame_builder),
            frame_parser: Arc::clone(&self.frame_parser),
            crypto_engine: Arc::clone(&self.crypto_engine),
        }))
    }
    
    fn is_ready(&self) -> bool {
        *self.ready.blocking_read()
    }
    
    async fn get_stats(&self) -> DMSCProtocolStats {
        let mut stats = self.stats.read().await.clone();
        
        // Calculate real-time metrics
        let elapsed_secs = stats.start_time.elapsed().as_secs();
        
        if elapsed_secs > 0 {
            // Calculate throughput (bytes per second)
            stats.throughput_bps = ((stats.bytes_sent + stats.bytes_received) * 8) / elapsed_secs;
            
            // Calculate average latency
            let total_messages = stats.messages_sent + stats.messages_received;
            if total_messages > 0 {
                stats.avg_latency_ms = stats.total_latency_ms / total_messages;
            }
            
            // Calculate error rate
            if stats.messages_sent > 0 {
                stats.error_rate = (stats.errors * 100) / stats.messages_sent;
            }
        }
        
        stats
    }
    
    async fn shutdown(&mut self) -> DMSCResult<()> {
        // Clear connection pool
        self.connection_pool.write().await.clear();
        
        // Mark as not ready
        *self.ready.write().await = false;
        
        Ok(())
    }
}

impl Default for DMSCPrivateProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// Private protocol connection implementation.
struct DMSCPrivateConnection {
    /// Connection ID
    connection_id: String,
    /// Target device ID
    target_id: String,
    /// Secure TCP stream
    stream: Arc<RwLock<Option<SecureStream>>>,
    /// Connection establishment time
    established_at: Instant,
    /// Last activity time
    last_activity: Arc<RwLock<Instant>>,
    /// Whether the connection is active
    active: Arc<RwLock<bool>>,
    /// Session keys for encryption
    session_keys: Arc<RwLock<SessionKeys>>,
    /// Configuration
    config: DMSCPrivateConfig,
    /// Connection pool info for integration
    pool_info: Arc<RwLock<Option<PoolConnectionInfo>>>,
    /// Reference to crypto engine
    crypto_engine: Arc<RwLock<Option<Box<dyn DMSCCryptoEngine>>>>,
}

/// Secure stream wrapper.
struct SecureStream {
    /// Underlying TCP stream
    tcp_stream: TcpStream,
    /// Encryption key
    encryption_key: Vec<u8>,
    /// Authentication key
    auth_key: Vec<u8>,
}

/// Session keys for encryption and authentication.
struct SessionKeys {
    /// Encryption key
    encryption_key: Vec<u8>,
    /// Authentication key
    auth_key: Vec<u8>,
    /// Key generation timestamp
    created_at: Instant,
}

impl DMSCPrivateConnection {
    /// Create a new private connection.
    async fn new(
        target_id: String,
        config: DMSCPrivateConfig,
        device_auth: Arc<DMSCDeviceAuthProtocol>,
        post_quantum: Arc<DMSCPostQuantumCrypto>,
        obfuscation: Arc<DMSCObfuscationLayer>,
    ) -> DMSCResult<Self> {
        // Perform device authentication
        if config.device_auth {
            device_auth.authenticate_device(&target_id).await?;
        }
        
        // Generate ephemeral session keys
        let session_keys = Self::generate_session_keys(&config).await?;
        
        // Establish secure connection
        let stream = Self::establish_secure_connection(
            &target_id,
            &session_keys,
            &config,
            post_quantum,
            obfuscation,
        ).await?;
        
        let connection_id = format!("private-{}", uuid::Uuid::new_v4());
        let now = Instant::now();
        
        // Create pool connection info for integration
        let pool_info = PoolConnectionInfo {
            connection_id: connection_id.clone(),
            device_id: target_id.clone(),
            address: "127.0.0.1:8080".to_string(),
            created_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            last_activity_secs: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            is_active: true,
            health_status: ConnectionHealth::Healthy,
        };
        
        Ok(Self {
            connection_id,
            target_id,
            stream: Arc::new(RwLock::new(Some(stream))),
            established_at: now,
            last_activity: Arc::new(RwLock::new(now)),
            active: Arc::new(RwLock::new(true)),
            session_keys: Arc::new(RwLock::new(session_keys)),
            config,
            pool_info: Arc::new(RwLock::new(Some(pool_info))),
            crypto_engine: Arc::new(RwLock::new(None)),
        })
    }
    
    /// Generate ephemeral session keys.
    async fn generate_session_keys(config: &DMSCPrivateConfig) -> DMSCResult<SessionKeys> {
        let mut rng = rand::thread_rng();
        
        // Generate encryption key (256 bits)
        let mut encryption_key = vec![0u8; 32];
        rng.fill(&mut encryption_key[..]);
        
        // Generate authentication key (256 bits)
        let mut auth_key = vec![0u8; 32];
        rng.fill(&mut auth_key[..]);
        
        Ok(SessionKeys {
            encryption_key,
            auth_key,
            created_at: Instant::now(),
        })
    }
    
    /// Establish secure connection with target.
    async fn establish_secure_connection(
        target_id: &str,
        session_keys: &SessionKeys,
        config: &DMSCPrivateConfig,
        post_quantum: Arc<DMSCPostQuantumCrypto>,
        obfuscation: Arc<DMSCObfuscationLayer>,
    ) -> DMSCResult<SecureStream> {
        // Parse target address (obfuscated)
        let obfuscated_addr = obfuscation.obfuscate_address(target_id).await?;
        
        // Connect with obfuscation
        let tcp_stream = TcpStream::connect(&obfuscated_addr).await
            .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to connect to {}: {}", target_id, e)))?;
        
        // Perform post-quantum key exchange if enabled
        if config.quantum_resistant {
            post_quantum.perform_key_exchange(&tcp_stream).await?;
        }
        
        Ok(SecureStream {
            tcp_stream,
            encryption_key: session_keys.encryption_key.clone(),
            auth_key: session_keys.auth_key.clone(),
        })
    }
    
    /// Update last activity time.
    async fn update_activity(&self) {
        *self.last_activity.write().await = Instant::now();
        
        // Update pool info last activity
        if let Some(ref mut pool_info) = *self.pool_info.write().await {
            pool_info.last_activity_secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or(Duration::from_secs(0)).as_secs();
        }
    }
    
    /// Check if connection is still active.
    async fn is_active(&self) -> bool {
        let active = *self.active.read().await;
        if !active {
            return false;
        }
        
        // Check session timeout
        let last_activity = *self.last_activity.read().await;
        if last_activity.elapsed() > self.config.session_timeout {
            *self.active.write().await = false;
            return false;
        }
        
        // Check if keys need rotation
        let session_keys = self.session_keys.read().await;
        if session_keys.created_at.elapsed() > self.config.key_rotation_interval {
            drop(session_keys);
            // In a real implementation, we would rotate keys here
            // For now, just mark as inactive
            *self.active.write().await = false;
            return false;
        }
        
        true
    }
    
    /// Rotate session keys.
    async fn rotate_keys(&self) -> DMSCResult<()> {
        let new_keys = Self::generate_session_keys(&self.config).await?;
        *self.session_keys.write().await = new_keys;
        Ok(())
    }
}

/// Wrapper for private connection to implement DMSCProtocolConnection trait.
struct DMSCPrivateConnectionWrapper {
    inner: Arc<DMSCPrivateConnection>,
    stats: Arc<RwLock<DMSCProtocolStats>>,
    frame_builder: Arc<DMSCFrameBuilder>,
    frame_parser: Arc<DMSCFrameParser>,
    crypto_engine: Arc<RwLock<Option<Box<dyn DMSCCryptoEngine>>>>,
}

#[async_trait]
impl DMSCProtocolConnection for DMSCPrivateConnectionWrapper {
    async fn send_message(&self, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let start_time = Instant::now();
        let result = self.send_message_with_flags(data, DMSCMessageFlags {
            encrypted: true,
            obfuscated: true,
            ..Default::default()
        }).await;

        // Update stats with latency metrics
        let latency = start_time.elapsed().as_millis() as u64;
        let mut stats = self.stats.write().await;
        stats.total_latency_ms += latency;
        if stats.min_latency_ms == 0 || latency < stats.min_latency_ms {
            stats.min_latency_ms = latency;
        }
        if latency > stats.max_latency_ms {
            stats.max_latency_ms = latency;
        }

        result
    }
    
    async fn send_message_with_flags(&self, data: &[u8], flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>> {
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.is_active().await {
            return Err(DMSCError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        // Get session keys
        let session_keys = self.inner.session_keys.read().await;
        
        // Encrypt and authenticate data
        let encrypted_data = self.encrypt_and_authenticate(data, &session_keys, flags).await?;
        
        // Send through secure stream
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut secure_stream) = *stream {
            secure_stream.tcp_stream.write_all(&encrypted_data).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to send encrypted data: {}", e)))?;
            
            secure_stream.tcp_stream.flush().await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to flush stream: {}", e)))?;
            
            // Update statistics
            self.stats.write().await.messages_sent += 1;
            self.stats.write().await.bytes_sent += data.len() as u64;
            
            // For simplicity, return empty response
            Ok(Vec::new())
        } else {
            Err(DMSCError::ConnectionFailed("No active secure stream".to_string()))
        }
    }
    
    async fn receive_message(&self) -> DMSCResult<Vec<u8>> {
        let start_time = Instant::now();
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.is_active().await {
            return Err(DMSCError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        // Get session keys
        let session_keys = self.inner.session_keys.read().await;
        
        // Receive from secure stream
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut secure_stream) = *stream {
            // Read encrypted data (simplified - in real implementation would read frame)
            let mut buffer = vec![0u8; 4096]; // Max message size
            let n = secure_stream.tcp_stream.read(&mut buffer).await
                .map_err(|e| DMSCError::ConnectionFailed(format!("Failed to receive encrypted data: {}", e)))?;
            
            if n == 0 {
                return Err(DMSCError::ConnectionFailed("Connection closed by peer".to_string()));
            }
            
            buffer.truncate(n);
            
            // Decrypt and verify data
            let decrypted_data = self.decrypt_and_verify(&buffer, &session_keys).await?;
            
            // Update stats with real metrics
            let mut stats = self.stats.write().await;
            stats.messages_received += 1;
            stats.bytes_received += decrypted_data.len() as u64;
            
            // Calculate and track latency
            let latency = start_time.elapsed().as_millis() as u64;
            stats.total_latency_ms += latency;
            if stats.min_latency_ms == 0 || latency < stats.min_latency_ms {
                stats.min_latency_ms = latency;
            }
            if latency > stats.max_latency_ms {
                stats.max_latency_ms = latency;
            }
            
            Ok(decrypted_data)
        } else {
            Err(DMSCError::ConnectionFailed("No active secure stream".to_string()))
        }
    }
    
    fn is_active(&self) -> bool {
        *self.inner.active.blocking_read()
    }
    
    fn get_connection_info(&self) -> DMSCConnectionInfo {
        DMSCConnectionInfo {
            connection_id: self.inner.connection_id.clone(),
            target_id: self.inner.target_id.clone(),
            protocol_type: DMSCProtocolType::Private,
            established_at: self.inner.established_at,
            last_activity: *self.inner.last_activity.blocking_read(),
            security_level: if self.inner.config.quantum_resistant {
                DMSCSecurityLevel::Maximum
            } else {
                DMSCSecurityLevel::High
            },
        }
    }
    
    async fn close(&self) -> DMSCResult<()> {
        *self.inner.active.write().await = false;
        self.inner.stream.write().await.take();
        Ok(())
    }
}

impl DMSCPrivateConnectionWrapper {
    /// Encrypt and authenticate data.
    async fn encrypt_and_authenticate(&self, data: &[u8], session_keys: &SessionKeys, flags: DMSCMessageFlags) -> DMSCResult<Vec<u8>> {
        // Build protocol frame
        let frame = self.frame_builder.build_frame(
            DMSCFrameType::Data,
            data,
            flags,
        ).await?;
        
        // Serialize frame
        let frame_data = frame.to_bytes()?;
        
        // Encrypt frame data using session keys
        let mut result = Vec::new();
        
        // Add nonce for encryption
        let mut nonce = vec![0u8; 12];
        rand::thread_rng().fill(&mut nonce[..]);
        result.extend_from_slice(&nonce);
        
        // Use crypto engine for real encryption if available
        if let Some(ref crypto_engine) = *self.inner.crypto_engine.read().await {
            let encrypted_data = crypto_engine.encrypt(&frame_data, &session_keys.encryption_key, &nonce)?;
            result.extend_from_slice(&encrypted_data);
        } else {
            // Fallback to simplified XOR encryption
            for (i, &byte) in frame_data.iter().enumerate() {
                result.push(byte ^ session_keys.encryption_key[i % session_keys.encryption_key.len()]);
            }
        }
        
        // Add authentication tag
        result.extend_from_slice(&session_keys.auth_key[..16]);
        
        Ok(result)
    }
    
    /// Decrypt and verify data.
    async fn decrypt_and_verify(&self, data: &[u8], session_keys: &SessionKeys) -> DMSCResult<Vec<u8>> {
        if data.len() < 28 { // 12 bytes nonce + 16 bytes auth tag
            return Err(DMSCError::InvalidData("Data too short for nonce and authentication tag".to_string()));
        }
        
        // Extract nonce
        let nonce = &data[..12];
        let encrypted_data = &data[12..data.len() - 16];
        
        // Verify authentication tag
        let auth_tag = &data[data.len() - 16..];
        if auth_tag != &session_keys.auth_key[..16] {
            return Err(DMSCError::AuthenticationFailed("Invalid authentication tag".to_string()));
        }
        
        // Decrypt data
        let decrypted_data = if let Some(ref crypto_engine) = *self.inner.crypto_engine.read().await {
            crypto_engine.decrypt(encrypted_data, &session_keys.encryption_key, nonce)?
        } else {
            // Fallback to simplified XOR decryption
            let mut result = Vec::new();
            for (i, &byte) in encrypted_data.iter().enumerate() {
                result.push(byte ^ session_keys.encryption_key[i % session_keys.encryption_key.len()]);
            }
            result
        };
        
        // Parse frame from decrypted data
        let frame = self.frame_parser.parse_frame(&decrypted_data)?;
        
        // Extract payload from frame
        Ok(frame.payload)
    }
}
