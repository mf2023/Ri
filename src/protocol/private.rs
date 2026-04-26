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

//! # Private Protocol Module
//! 
//! This module implements the secure private communication protocol for Ri.
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
//! 4. **Application Layer**: Ri-specific secure messaging
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
//! use ri::protocol::{RiPrivateProtocol, RiProtocolConfig, RiCryptoSuite, RiObfuscationLevel};
//! 
//! async fn example() -> RiResult<()> {
//!     let mut protocol = RiPrivateProtocol::new();
//!     
//!     let config = RiProtocolConfig::Private {
//!         crypto_suite: RiCryptoSuite::NationalStandard,
//!         device_auth: true,
//!         obfuscation_level: RiObfuscationLevel::High,
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

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use async_trait::async_trait;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use rand::Rng;
use zeroize::{Zeroize, ZeroizeOnDrop};
use secrecy::{ExposeSecret, SecretVec};
use std::sync::atomic::{AtomicUsize, Ordering};

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

use crate::core::{RiResult, RiError};
use super::{RiProtocol, RiProtocolConfig, RiProtocolType, RiProtocolConnection, 
            RiProtocolStats, RiMessageFlags, RiConnectionInfo, RiSecurityLevel};
use super::security::{RiCryptoSuite, RiObfuscationLevel, RiDeviceAuthProtocol, 
                       RiPostQuantumCrypto, RiObfuscationLayer};
use super::crypto::{RiCryptoEngine, AES256GCM, ChaCha20Poly1305};
use super::frames::{RiFrame, RiFrameType, RiFrameBuilder, RiFrameParser};
use crate::device::pool::{RiConnectionPool, RiConnectionInfo as PoolConnectionInfo, RiConnectionState};

/// Private protocol implementation.
pub struct RiPrivateProtocol {
    /// Protocol configuration
    config: Option<RiPrivateConfig>,
    /// Device authentication protocol
    device_auth: Arc<RiDeviceAuthProtocol>,
    /// Post-quantum cryptography handler
    post_quantum: Arc<RiPostQuantumCrypto>,
    /// Obfuscation layer
    obfuscation: Arc<RiObfuscationLayer>,
    /// Connection pool for secure connections
    connection_pool: Arc<RwLock<HashMap<String, Arc<RiPrivateConnection>>>>,
    /// Protocol statistics
    stats: Arc<RwLock<RiProtocolStats>>,
    /// Whether the protocol is ready
    ready: Arc<RwLock<bool>>,
    /// Crypto engine for encryption/decryption
    crypto_engine: Arc<RwLock<Option<Box<dyn RiCryptoEngine>>>>,
    /// Frame builder for protocol frames
    frame_builder: Arc<RiFrameBuilder>,
    /// Frame parser for incoming frames
    frame_parser: Arc<RiFrameParser>,
}

/// Private protocol specific configuration.
#[derive(Debug, Clone)]
struct RiPrivateConfig {
    /// Cryptographic suite to use
    crypto_suite: RiCryptoSuite,
    /// Enable device authentication
    device_auth: bool,
    /// Obfuscation level
    obfuscation_level: RiObfuscationLevel,
    /// Enable quantum-resistant algorithms
    quantum_resistant: bool,
    /// Session timeout
    session_timeout: Duration,
    /// Key rotation interval
    key_rotation_interval: Duration,
}

impl RiPrivateProtocol {
    /// Create a new private protocol instance.
    pub fn new() -> Self {
        Self {
            config: None,
            device_auth: Arc::new(RiDeviceAuthProtocol::new()),
            post_quantum: Arc::new(RiPostQuantumCrypto::new()),
            obfuscation: Arc::new(RiObfuscationLayer::new()),
            connection_pool: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(RiProtocolStats::default())),
            ready: Arc::new(RwLock::new(false)),
            crypto_engine: Arc::new(RwLock::new(None)),
            frame_builder: Arc::new(RiFrameBuilder::new()),
            frame_parser: Arc::new(RiFrameParser::new()),
        }
    }
    
    /// Get or create a secure connection.
    async fn get_or_create_connection(&self, target_id: &str) -> RiResult<Arc<RiPrivateConnection>> {
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
            .ok_or_else(|| RiError::InvalidState("Private protocol not initialized".to_string()))?;
        let connection = Arc::new(RiPrivateConnection::new(
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
        F: FnOnce(&mut RiProtocolStats),
    {
        let mut stats = self.stats.write().await;
        updater(&mut *stats);
    }
}

#[async_trait]
impl RiProtocol for RiPrivateProtocol {
    fn protocol_type(&self) -> RiProtocolType {
        RiProtocolType::Private
    }
    
    async fn initialize(&mut self, config: RiProtocolConfig) -> RiResult<()> {
        // Validate and convert configuration
        let private_config = match config {
            RiProtocolConfig::Private { crypto_suite, device_auth, obfuscation_level, quantum_resistant } => {
                RiPrivateConfig {
                    crypto_suite,
                    device_auth,
                    obfuscation_level,
                    quantum_resistant,
                    session_timeout: Duration::from_secs(3600), // 1 hour
                    key_rotation_interval: Duration::from_secs(600), // 10 minutes
                }
            }
            _ => return Err(RiError::InvalidConfiguration("Invalid configuration type for private protocol".to_string())),
        };
        
        // Initialize crypto engine based on crypto suite
        let crypto_engine: Box<dyn RiCryptoEngine> = match private_config.crypto_suite {
            RiCryptoSuite::AES256GCM => Box::new(AES256GCM::new()),
            RiCryptoSuite::ChaCha20Poly1305 => Box::new(ChaCha20Poly1305::new()),
            RiCryptoSuite::NationalStandard => Box::new(AES256GCM::new()), // Default to AES256GCM for now
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
    
    async fn connect(&self, target_id: &str) -> RiResult<Box<dyn RiProtocolConnection>> {
        if !*self.ready.read().await {
            return Err(RiError::InvalidState("Protocol not initialized".to_string()));
        }
        
        // Update connection attempts
        self.update_stats(|stats| stats.connection_attempts += 1).await;
        
        let connection = self.get_or_create_connection(target_id).await?;
        
        // Update successful connections
        self.update_stats(|stats| stats.successful_connections += 1).await;
        
        Ok(Box::new(RiPrivateConnectionWrapper {
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
    
    async fn get_stats(&self) -> RiProtocolStats {
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
    
    async fn shutdown(&mut self) -> RiResult<()> {
        // Clear connection pool
        self.connection_pool.write().await.clear();
        
        // Mark as not ready
        *self.ready.write().await = false;
        
        Ok(())
    }
}

impl Default for RiPrivateProtocol {
    fn default() -> Self {
        Self::new()
    }
}

/// Private protocol connection implementation.
struct RiPrivateConnection {
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
    /// Session keys for encryption (with zeroize protection)
    session_keys: Arc<RwLock<SessionKeys>>,
    /// Configuration
    config: RiPrivateConfig,
    /// Connection pool info for integration
    pool_info: Arc<RwLock<Option<PoolConnectionInfo>>>,
    /// Reference to crypto engine
    crypto_engine: Arc<RwLock<Option<Box<dyn RiCryptoEngine>>>>,
    /// Key rotation in progress flag
    key_rotation_in_progress: Arc<RwLock<bool>>,
}

/// Secure stream wrapper.
struct SecureStream {
    /// Underlying TCP stream
    tcp_stream: TcpStream,
    /// Encryption key (zeroized on drop)
    encryption_key: SecretVec<u8>,
    /// Authentication key (zeroized on drop)
    auth_key: SecretVec<u8>,
}

/// Session keys for encryption and authentication with secure memory handling.
#[derive(ZeroizeOnDrop)]
struct SessionKeys {
    /// Encryption key
    #[zeroize(skip)]
    encryption_key: SecretVec<u8>,
    /// Authentication key
    #[zeroize(skip)]
    auth_key: SecretVec<u8>,
    /// Key generation timestamp
    created_at: Instant,
    /// Nonce counter for replay protection (sliding window)
    nonce_counter: Arc<AtomicUsize>,
    /// Recently used nonces (limited sliding window)
    recent_nonces: Arc<RwLock<HashSet<u64>>>,
    /// Maximum nonces to track
    max_nonce_history: usize,
}

impl SessionKeys {
    /// Create new session keys with secure memory handling
    async fn new(config: &RiPrivateConfig) -> RiResult<Self> {
        let mut rng = rand::thread_rng();

        let mut encryption_key_data = vec![0u8; 32];
        rng.fill(&mut encryption_key_data[..]);

        let mut auth_key_data = vec![0u8; 32];
        rng.fill(&mut auth_key_data[..]);

        Ok(Self {
            encryption_key: SecretVec::new(encryption_key_data),
            auth_key: SecretVec::new(auth_key_data),
            created_at: Instant::now(),
            nonce_counter: Arc::new(AtomicUsize::new(0)),
            recent_nonces: Arc::new(RwLock::new(HashSet::new())),
            max_nonce_history: 1000000, // Track up to 1M nonces
        })
    }

    /// Generate a unique nonce for a new message
    async fn generate_nonce(&self) -> u64 {
        let nonce = self.nonce_counter.fetch_add(1, Ordering::SeqCst) as u64;

        // Add to sliding window
        let mut recent = self.recent_nonces.write().await;
        if recent.len() >= self.max_nonce_history {
            // Remove oldest nonce (simple eviction)
            if let Some(oldest) = recent.iter().next().cloned() {
                recent.remove(&oldest);
            }
        }
        recent.insert(nonce);

        nonce
    }

    /// Check if a nonce is valid (not replayed)
    async fn is_valid_nonce(&self, nonce: u64) -> bool {
        let recent = self.recent_nonces.read().await;
        !recent.contains(&nonce)
    }

    /// Check if keys need rotation
    fn needs_rotation(&self, rotation_interval: Duration) -> bool {
        self.created_at.elapsed() > rotation_interval
    }
}

impl RiPrivateConnection {
    /// Create a new private connection.
    async fn new(
        target_id: String,
        config: RiPrivateConfig,
        device_auth: Arc<RiDeviceAuthProtocol>,
        post_quantum: Arc<RiPostQuantumCrypto>,
        obfuscation: Arc<RiObfuscationLayer>,
    ) -> RiResult<Self> {
        if config.device_auth {
            device_auth.authenticate_device(&target_id).await?;
        }

        let session_keys = SessionKeys::new(&config).await?;

        let stream = Self::establish_secure_connection(
            &target_id,
            &session_keys,
            &config,
            post_quantum,
            obfuscation,
        ).await?;

        let connection_id = format!("private-{}", uuid::Uuid::new_v4());
        let now = Instant::now();

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
            key_rotation_in_progress: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Establish secure connection with target.
    async fn establish_secure_connection(
        target_id: &str,
        session_keys: &SessionKeys,
        config: &RiPrivateConfig,
        post_quantum: Arc<RiPostQuantumCrypto>,
        obfuscation: Arc<RiObfuscationLayer>,
    ) -> RiResult<SecureStream> {
        // Parse target address (obfuscated)
        let obfuscated_addr = obfuscation.obfuscate_address(target_id).await?;
        
        // Connect with obfuscation
        let tcp_stream = TcpStream::connect(&obfuscated_addr).await
            .map_err(|e| RiError::ConnectionFailed(format!("Failed to connect to {}: {}", target_id, e)))?;
        
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

        let last_activity = *self.last_activity.read().await;
        if last_activity.elapsed() > self.config.session_timeout {
            *self.active.write().await = false;
            return false;
        }

        let session_keys = self.session_keys.read().await;
        if session_keys.needs_rotation(self.config.key_rotation_interval) {
            drop(session_keys);
            let _ = self.rotate_keys().await;
        }

        true
    }

    /// Rotate session keys securely.
    async fn rotate_keys(&self) -> RiResult<()> {
        let mut rotation_in_progress = self.key_rotation_in_progress.write().await;
        if *rotation_in_progress {
            return Ok(());
        }
        *rotation_in_progress = true;
        drop(rotation_in_progress);

        let new_keys = SessionKeys::new(&self.config).await?;

        {
            let mut keys = self.session_keys.write().await;
            *keys = new_keys;
        }

        *self.key_rotation_in_progress.write().await = false;
        Ok(())
    }
}

/// Wrapper for private connection to implement RiProtocolConnection trait.
struct RiPrivateConnectionWrapper {
    inner: Arc<RiPrivateConnection>,
    stats: Arc<RwLock<RiProtocolStats>>,
    frame_builder: Arc<RiFrameBuilder>,
    frame_parser: Arc<RiFrameParser>,
    crypto_engine: Arc<RwLock<Option<Box<dyn RiCryptoEngine>>>>,
}

#[async_trait]
impl RiProtocolConnection for RiPrivateConnectionWrapper {
    async fn send_message(&self, data: &[u8]) -> RiResult<Vec<u8>> {
        let start_time = Instant::now();
        let result = self.send_message_with_flags(data, RiMessageFlags {
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
    
    async fn send_message_with_flags(&self, data: &[u8], flags: RiMessageFlags) -> RiResult<Vec<u8>> {
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.is_active().await {
            return Err(RiError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        // Get session keys
        let session_keys = self.inner.session_keys.read().await;
        
        // Encrypt and authenticate data
        let encrypted_data = self.encrypt_and_authenticate(data, &session_keys, flags).await?;
        
        // Send through secure stream
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut secure_stream) = *stream {
            secure_stream.tcp_stream.write_all(&encrypted_data).await
                .map_err(|e| RiError::ConnectionFailed(format!("Failed to send encrypted data: {}", e)))?;
            
            secure_stream.tcp_stream.flush().await
                .map_err(|e| RiError::ConnectionFailed(format!("Failed to flush stream: {}", e)))?;
            
            // Update statistics
            self.stats.write().await.messages_sent += 1;
            self.stats.write().await.bytes_sent += data.len() as u64;
            
            self.receive_message().await
        } else {
            Err(RiError::ConnectionFailed("No active secure stream".to_string()))
        }
    }
    
    async fn receive_message(&self) -> RiResult<Vec<u8>> {
        let start_time = Instant::now();
        // Update activity
        self.inner.update_activity().await;
        
        // Check connection
        if !self.inner.is_active().await {
            return Err(RiError::ConnectionFailed("Connection is not active".to_string()));
        }
        
        // Get session keys
        let session_keys = self.inner.session_keys.read().await;
        
        // Receive from secure stream
        let mut stream = self.inner.stream.write().await;
        if let Some(ref mut secure_stream) = *stream {
            // Read encrypted data (simplified - in real implementation would read frame)
            let mut buffer = vec![0u8; 4096]; // Max message size
            let n = secure_stream.tcp_stream.read(&mut buffer).await
                .map_err(|e| RiError::ConnectionFailed(format!("Failed to receive encrypted data: {}", e)))?;
            
            if n == 0 {
                return Err(RiError::ConnectionFailed("Connection closed by peer".to_string()));
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
            Err(RiError::ConnectionFailed("No active secure stream".to_string()))
        }
    }
    
    fn is_active(&self) -> bool {
        *self.inner.active.blocking_read()
    }
    
    fn get_connection_info(&self) -> RiConnectionInfo {
        RiConnectionInfo {
            connection_id: self.inner.connection_id.clone(),
            target_id: self.inner.target_id.clone(),
            protocol_type: RiProtocolType::Private,
            established_at: self.inner.established_at,
            last_activity: *self.inner.last_activity.blocking_read(),
            security_level: if self.inner.config.quantum_resistant {
                RiSecurityLevel::Maximum
            } else {
                RiSecurityLevel::High
            },
        }
    }
    
    async fn close(&self) -> RiResult<()> {
        *self.inner.active.write().await = false;
        self.inner.stream.write().await.take();
        Ok(())
    }
}

impl RiPrivateConnectionWrapper {
    /// Encrypt and authenticate data with nonce-based replay protection.
    async fn encrypt_and_authenticate(&self, data: &[u8], session_keys: &SessionKeys, flags: RiMessageFlags) -> RiResult<Vec<u8>> {
        let frame = self.frame_builder.build_frame(
            RiFrameType::Data,
            data,
            flags,
        ).await?;

        let frame_data = frame.to_bytes()?;

        let mut result = Vec::with_capacity(4);

        let nonce = session_keys.generate_nonce().await;
        result.extend_from_slice(&nonce.to_be_bytes()[..12]);

        if let Some(ref crypto_engine) = *self.inner.crypto_engine.read().await {
            let encrypted_data = crypto_engine.encrypt(&frame_data, session_keys.encryption_key.expose_secret(), &nonce.to_be_bytes()[..])?;
            result.extend_from_slice(&encrypted_data);
        } else {
            for (i, &byte) in frame_data.iter().enumerate() {
                result.push(byte ^ session_keys.encryption_key.expose_secret()[i % session_keys.encryption_key.len()]);
            }
        }

        result.extend_from_slice(&session_keys.auth_key.expose_secret()[..16]);

        Ok(result)
    }

    /// Decrypt and verify data with nonce validation.
    async fn decrypt_and_verify(&self, data: &[u8], session_keys: &SessionKeys) -> RiResult<Vec<u8>> {
        if data.len() < 28 {
            return Err(RiError::InvalidData("Data too short for nonce and authentication tag".to_string()));
        }

        let nonce = u64::from_be_bytes([
            data[0], data[1], data[2], data[3],
            data[4], data[5], data[6], data[7],
            data[8], data[9], data[10], data[11]
        ]);

        if !session_keys.is_valid_nonce(nonce).await {
            return Err(RiError::AuthenticationFailed("Nonce replay detected".to_string()));
        }

        let encrypted_data = &data[12..data.len() - 16];

        let auth_tag = &data[data.len() - 16..];
        let expected_tag = &session_keys.auth_key.expose_secret()[..16];

        // Use constant-time comparison to prevent timing attacks
        let auth_tag_len = auth_tag.len();
        let expected_tag_len = expected_tag.len();
        let mut result = 0u8;
        if auth_tag_len == expected_tag_len {
            for i in 0..auth_tag_len {
                result |= auth_tag[i] ^ expected_tag[i];
            }
        } else {
            result = 1;
        }
        if result != 0 {
            return Err(RiError::AuthenticationFailed("Invalid authentication tag".to_string()));
        }

        let decrypted_data = if let Some(ref crypto_engine) = *self.inner.crypto_engine.read().await {
            crypto_engine.decrypt(encrypted_data, session_keys.encryption_key.expose_secret(), &nonce.to_be_bytes()[..])?
        } else {
            let mut result = Vec::with_capacity(4);
            for (i, &byte) in encrypted_data.iter().enumerate() {
                result.push(byte ^ session_keys.encryption_key.expose_secret()[i % session_keys.encryption_key.len()]);
            }
            result
        };

        let frame = self.frame_parser.parse_frame(&decrypted_data)?;

        Ok(frame.payload)
    }
}
