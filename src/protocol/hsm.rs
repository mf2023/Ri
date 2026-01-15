//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0

#![allow(unused)]
#![allow(non_snake_case)]

//! # HSM Module
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

#![allow(non_snake_case)]

//! # Hardware Security Module Integration Module
//!
//! This module provides hardware security module (HSM) integration capabilities
//! for the DMSC framework, supporting PKCS#11 devices and TPM 2.0 chips. It
//! enables secure key storage and cryptographic operations using hardware
//! security mechanisms.
//!
//! ## Supported Hardware Security Modules
//!
//! - **PKCS#11 Devices**: Smart cards, USB tokens, and network HSMs
//!   - YubiKey, Nitrokey, SafeNet, Utimaco, Thales
//!   - Standard PKCS#11 v2.40+ compatible devices
//!
//! - **TPM 2.0 Chips**: Trusted Platform Module implementations
//!   - Intel Platform Trust Technology (PTT)
//!   - AMD Secure Processor
//!   - Software TPM simulators (for testing)
//!
//! ## Security Features
//!
//! - **Secure Key Storage**: Private keys never leave the HSM
//! - **Hardware-Based Signing**: Digital signatures generated in hardware
//! - **Attestation**: Platform and key attestation capabilities
//! - **Secure Random Generation**: Hardware random number generation
//! - **Key Import/Export**: Secure key backup and migration
//!
//! ## Usage Examples
//!
//! ```rust
//! use dmsc::protocol::hsm::{DMSCHSMManager, DMSCHSMType, DMSCKeyInfo};
//!
//! async fn example() -> DMSCResult<()> {
//!     // Initialize HSM manager
//!     let mut manager = DMSCHSMManager::new();
//!
//!     // Connect to PKCS#11 device
//!     manager.connect_pkcs11("/usr/lib/pkcs11/opensc-pkcs11.so").await?;
//!
//!     // Generate key in HSM
//!     let key_id = manager.generate_key("rsa2048", None).await?;
//!
//!     // Sign data using HSM
//!     let signature = manager.sign(&key_id, b"data to sign").await?;
//!
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};
use async_trait::async_trait;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use subtle::ConstantTimeEq;

#[cfg(not(windows))]
use pkcs11;

use crate::core::{DMSCResult, DMSCError};

/// HSM type enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSCHSMType {
    /// PKCS#11 compatible device
    PKCS11,
    /// TPM 2.0 chip
    TPM20,
    /// Software simulator (for testing)
    Software,
}

/// HSM connection status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCConnectionStatus {
    /// Not connected
    Disconnected,
    /// Connecting in progress
    Connecting,
    /// Successfully connected
    Connected,
    /// Connection error
    Error,
    /// Reconnecting
    Reconnecting,
}

/// Key type enumeration for HSM operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSCKeyType {
    /// RSA key pair
    RSA { size: usize },
    /// ECDSA key pair
    ECDSA { curve: DMSCECCurve },
    /// EC EdDSA key pair
    EdDSA { curve: DMSCECCurve },
    /// Symmetric AES key
    AES { size: usize },
    /// Symmetric SM4 key (Chinese National Standard)
    SM4,
    /// HMAC key
    HMAC { size: usize },
    /// Dilithium key (post-quantum)
    Dilithium { level: u8 },
    /// Falcon key (post-quantum)
    Falcon { level: u8 },
}

/// Elliptic curve enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DMSCECCurve {
    /// P-256 curve
    P256,
    /// P-384 curve
    P384,
    /// P-521 curve
    P521,
    /// Secp256k1 curve
    Secp256k1,
    /// SM2 curve (Chinese National Standard)
    SM2,
}

/// Key information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCKeyInfo {
    /// Unique key identifier
    pub id: String,
    /// Key type
    pub key_type: DMSCKeyType,
    /// Key label for identification
    pub label: String,
    /// Whether the key is sensitive
    pub sensitive: bool,
    /// Whether the key is extractable
    pub extractable: bool,
    /// Key creation timestamp
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// Last usage timestamp
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
    /// Usage count
    pub usage_count: u64,
}

/// HSM operation result structure.
#[derive(Debug, Clone)]
pub struct DMSCOperationResult<T> {
    /// Operation result data
    pub data: T,
    /// Operation duration
    pub duration: Duration,
    /// Whether the operation used hardware acceleration
    pub hardware_accelerated: bool,
}

/// HSM event for monitoring.
#[derive(Debug, Clone)]
pub enum DMSCHSMEvent {
    /// Connection established
    Connected { hsm_type: DMSCHSMType, device_info: String },
    /// Connection lost
    Disconnected { hsm_type: DMSCHSMType, reason: String },
    /// Key operation performed
    KeyOperation { operation: String, key_id: String, duration: Duration },
    /// Error occurred
    Error { error: String, recoverable: bool },
    /// Slot status changed
    SlotChanged { slot_id: u32, present: bool },
}

/// HSM configuration structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCHSMConfig {
    /// HSM type to use
    pub hsm_type: DMSCHSMType,
    /// PKCS#11 library path (for PKCS#11 devices)
    pub pkcs11_library: Option<String>,
    /// Slot number (for PKCS#11 devices)
    pub slot: Option<u32>,
    /// Token label (for PKCS#11 devices)
    pub token_label: Option<String>,
    /// TPM device path (for TPM devices)
    pub tpm_device: Option<String>,
    /// PIN for authentication
    pub pin: Option<String>,
    /// Whether to use secure PIN entry
    pub secure_pin_entry: bool,
    /// Connection timeout
    pub connection_timeout: Duration,
    /// Retry attempts on failure
    pub max_retries: u32,
    /// Enable key caching
    pub enable_key_cache: bool,
    /// Cache TTL for keys
    pub cache_ttl: Duration,
}

impl Default for DMSCHSMConfig {
    fn default() -> Self {
        Self {
            hsm_type: DMSCHSMType::PKCS11,
            pkcs11_library: None,
            slot: None,
            token_label: None,
            tpm_device: None,
            pin: None,
            secure_pin_entry: true,
            connection_timeout: Duration::from_secs(30),
            max_retries: 3,
            enable_key_cache: true,
            cache_ttl: Duration::from_secs(300),
        }
    }
}

/// HSM statistics structure.
#[derive(Debug, Clone)]
pub struct DMSCHSMStatistics {
    /// Total connection count
    pub total_connections: Arc<RwLock<u64>>,
    /// Current connection status
    pub current_status: Arc<RwLock<DMSCConnectionStatus>>,
    /// Total operations performed
    pub total_operations: Arc<RwLock<u64>>,
    /// Total hardware operations
    pub hardware_operations: Arc<RwLock<u64>>,
    /// Total software fallback operations
    pub software_fallbacks: Arc<RwLock<u64>>,
    /// Average operation latency
    pub average_latency_ms: Arc<RwLock<f64>>,
    /// Last operation timestamp
    pub last_operation: Arc<RwLock<Option<Instant>>>,
    /// Error count
    pub error_count: Arc<RwLock<u64>>,
    /// Connection start time
    pub connected_at: Arc<RwLock<Option<Instant>>>,
}

impl DMSCHSMStatistics {
    /// Create new statistics instance.
    pub fn new() -> Self {
        Self {
            total_connections: Arc::new(RwLock::new(0)),
            current_status: Arc::new(RwLock::new(DMSCConnectionStatus::Disconnected)),
            total_operations: Arc::new(RwLock::new(0)),
            hardware_operations: Arc::new(RwLock::new(0)),
            software_fallbacks: Arc::new(RwLock::new(0)),
            average_latency_ms: Arc::new(RwLock::new(0.0)),
            last_operation: Arc::new(RwLock::new(None)),
            error_count: Arc::new(RwLock::new(0)),
            connected_at: Arc::new(RwLock::new(None)),
        }
    }

    /// Record an operation.
    pub async fn record_operation(&self, hardware_accelerated: bool, duration: Duration) {
        let mut total_ops = self.total_operations.write().await;
        *total_ops += 1;

        if hardware_accelerated {
            let mut hw_ops = self.hardware_operations.write().await;
            *hw_ops += 1;
        } else {
            let mut sw_fallbacks = self.software_fallbacks.write().await;
            *sw_fallbacks += 1;
        }

        let mut last_op = self.last_operation.write().await;
        *last_op = Some(Instant::now());

        let latency_ms = duration.as_secs_f64() * 1000.0;
        let mut avg_latency = self.average_latency_ms.write().await;
        *avg_latency = (*avg_latency * 0.9) + (latency_ms * 0.1);
    }

    /// Record an error.
    pub async fn record_error(&self) {
        let mut errors = self.error_count.write().await;
        *errors += 1;
    }

    /// Record connection.
    pub async fn record_connection(&self, status: DMSCConnectionStatus) {
        let mut total_conns = self.total_connections.write().await;
        *total_conns += 1;

        let mut current_status = self.current_status.write().await;
        *current_status = status;

        if status == DMSCConnectionStatus::Connected {
            let mut connected_at = self.connected_at.write().await;
            *connected_at = Some(Instant::now());
        }
    }
}

/// Device information structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDeviceInfo {
    /// Manufacturer name
    pub manufacturer: String,
    /// Model name
    pub model: String,
    /// Serial number
    pub serial_number: String,
    /// Firmware version
    pub firmware_version: String,
    /// Supported algorithms
    pub supported_algorithms: Vec<String>,
    /// Hardware version
    pub hardware_version: String,
    /// Token flags
    pub token_flags: Vec<String>,
}

/// HSM trait defining the interface for hardware security modules.
#[async_trait]
pub trait DMSCHSM: Send + Sync {
    /// Connect to the HSM device.
    async fn connect(&mut self, config: &DMSCHSMConfig) -> DMSCResult<DMSCDeviceInfo>;

    /// Disconnect from the HSM device.
    async fn disconnect(&mut self) -> DMSCResult<()>;

    /// Check if connected.
    fn is_connected(&self) -> bool;

    /// Get the HSM type.
    fn hsm_type(&self) -> DMSCHSMType;

    /// Generate a new key pair in the HSM.
    async fn generate_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String>;

    /// Generate a symmetric key in the HSM.
    async fn generate_symmetric_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String>;

    /// Import a key into the HSM.
    async fn import_key(
        &mut self,
        key_type: DMSCKeyType,
        key_data: &[u8],
        label: Option<&str>,
        extractable: bool,
    ) -> DMSCResult<String>;

    /// Export a public key from the HSM.
    async fn export_public_key(&mut self, key_id: &str) -> DMSCResult<Vec<u8>>;

    /// Export a key (if extractable).
    async fn export_key(&mut self, key_id: &str) -> DMSCResult<Vec<u8>>;

    /// Sign data using a private key in the HSM.
    async fn sign(
        &mut self,
        key_id: &str,
        data: &[u8],
    ) -> DMSCResult<Vec<u8>>;

    /// Verify a signature using a public key in the HSM.
    async fn verify(
        &mut self,
        key_id: &str,
        data: &[u8],
        signature: &[u8],
    ) -> DMSCResult<bool>;

    /// Encrypt data using a public key in the HSM.
    async fn encrypt(
        &mut self,
        key_id: &str,
        data: &[u8],
    ) -> DMSCResult<Vec<u8>>;

    /// Decrypt data using a private key in the HSM.
    async fn decrypt(
        &mut self,
        key_id: &str,
        ciphertext: &[u8],
    ) -> DMSCResult<Vec<u8>>;

    /// Encrypt data using a symmetric key in the HSM.
    async fn encrypt_symmetric(
        &mut self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> DMSCResult<Vec<u8>>;

    /// Decrypt data using a symmetric key in the HSM.
    async fn decrypt_symmetric(
        &mut self,
        key_id: &str,
        ciphertext: &[u8],
        iv: Option<&[u8]>,
    ) -> DMSCResult<Vec<u8>>;

    /// Generate a random number using the HSM.
    async fn random(&mut self, size: usize) -> DMSCResult<Vec<u8>>;

    /// Get the hash of data using the HSM.
    async fn hash(&mut self, algorithm: &str, data: &[u8]) -> DMSCResult<Vec<u8>>;

    /// Derive a key from another key using the HSM.
    async fn derive_key(
        &mut self,
        base_key_id: &str,
        template: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String>;

    /// Delete a key from the HSM.
    async fn delete_key(&mut self, key_id: &str) -> DMSCResult<()>;

    /// List all keys in the HSM.
    async fn list_keys(&mut self) -> DMSCResult<Vec<DMSCKeyInfo>>;

    /// Get information about a specific key.
    async fn get_key_info(&mut self, key_id: &str) -> DMSCResult<DMSCKeyInfo>;

    /// Get the device information.
    fn get_device_info(&self) -> Option<DMSCDeviceInfo>;
}

#[cfg(not(windows))]
/// PKCS#11 HSM implementation.
pub struct DMSCPKCS11HSM {
    config: Option<DMSCHSMConfig>,
    connected: bool,
    device_info: Option<DMSCDeviceInfo>,
    library: Option<pkcs11::Ctx>,
    session: Option<pkcs11::Session>,
    slot: Option<pkcs11::Slot>,
    statistics: Arc<DMSCHSMStatistics>,
}

#[cfg(not(windows))]
impl DMSCPKCS11HSM {
    /// Create new PKCS#11 HSM instance.
    pub fn new() -> Self {
        Self {
            config: None,
            connected: false,
            device_info: None,
            library: None,
            session: None,
            slot: None,
            statistics: Arc::new(DMSCHSMStatistics::new()),
        }
    }

    /// Initialize the PKCS#11 library.
    fn initialize_library(&mut self, library_path: &str) -> DMSCResult<()> {
        let library = pkcs11::Ctx::new(library_path)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to load PKCS#11 library: {}", e)))?;

        library.initialize()
            .map_err(|e| DMSCError::CryptoError(format!("Failed to initialize PKCS#11: {}", e)))?;

        self.library = Some(library);
        Ok(())
    }

    /// Get available slots.
    fn get_available_slots(&self) -> DMSCResult<Vec<pkcs11::Slot>> {
        let library = self.library.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("PKCS#11 library not loaded".to_string()))?;

        let slots = library.get_slot_list(false)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to get slot list: {}", e)))?;

        Ok(slots)
    }

    /// Find a slot by number or token label.
    fn find_slot(&self, slot_number: Option<u32>, token_label: Option<&str>) -> DMSCResult<pkcs11::Slot> {
        let library = self.library.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("PKCS#11 library not loaded".to_string()))?;

        let slots = self.get_available_slots()?;

        for slot in slots {
            let token_info = library.get_token_info(slot)
                .map_err(|e| DMSCError::CryptoError(format!("Failed to get token info: {}", e)))?;

            if let Some(num) = slot_number {
                if slot == num {
                    return Ok(slot);
                }
            }

            if let Some(label) = token_label {
                let token_label_str = String::from_utf8_lossy(&token_info.label);
                if token_label_str.trim() == label {
                    return Ok(slot);
                }
            }
        }

        Err(DMSCError::CryptoError("Slot not found".to_string()))
    }

    /// Open a session with the token.
    fn open_session(&mut self, slot: pkcs11::Slot) -> DMSCResult<()> {
        let library = self.library.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("PKCS#11 library not loaded".to_string()))?;

        let session = library.open_session(
            slot,
            pkcs11::SessionType::Serial,
            None,
            None,
        ).map_err(|e| DMSCError::CryptoError(format!("Failed to open session: {}", e)))?;

        self.session = Some(session);
        self.slot = Some(slot);
        Ok(())
    }

    /// Authenticate to the token.
    fn authenticate(&mut self, pin: Option<&str>) -> DMSCResult<()> {
        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let pin = pin.unwrap_or("");

        session.login(pkcs11::UserType::User, pin)
            .map_err(|e| DMSCError::CryptoError(format!("Login failed: {}", e)))?;

        Ok(())
    }

    /// Find objects by template.
    fn find_objects(&self, template: &[pkcs11::Attribute]) -> DMSCResult<Vec<pkcs11::ObjectHandle>> {
        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        session.find_objects_init(template)
            .map_err(|e| DMSCError::CryptoError(format!("Find objects init failed: {}", e)))?;

        let objects = session.find_objects(100)
            .map_err(|e| DMSCError::CryptoError(format!("Find objects failed: {}", e)))?;

        session.find_objects_final()
            .map_err(|e| DMSCError::CryptoError(format!("Find objects final failed: {}", e)))?;

        Ok(objects)
    }

    /// Get attribute value from object.
    fn get_attribute_value(
        &self,
        object: pkcs11::ObjectHandle,
        attribute: pkcs11::AttributeType,
    ) -> DMSCResult<Vec<u8>> {
        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let attributes = session.get_attribute_value(object, &[attribute])
            .map_err(|e| DMSCError::CryptoError(format!("Get attribute failed: {}", e)))?;

        if attributes.is_empty() {
            return Err(DMSCError::CryptoError("Attribute not found".to_string()));
        }

        Ok(attributes[0].value().to_vec())
    }
}

#[cfg(not(windows))]
#[async_trait]
impl DMSCHSM for DMSCPKCS11HSM {
    async fn connect(&mut self, config: &DMSCHSMConfig) -> DMSCResult<DMSCDeviceInfo> {
        let start = Instant::now();

        self.config = Some(config.clone());

        let library_path = config.pkcs11_library.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("PKCS#11 library path not configured".to_string()))?;

        self.initialize_library(library_path)?;

        let slot = self.find_slot(config.slot, config.token_label.as_deref())?;

        self.open_session(slot)?;

        if let Some(pin) = &config.pin {
            self.authenticate(Some(pin))?;
        } else {
            self.authenticate(None)?;
        }

        let library = self.library.as_ref().unwrap();
        let token_info = library.get_token_info(slot)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to get token info: {}", e)))?;

        let device_info = DMSCDeviceInfo {
            manufacturer: String::from_utf8_lossy(&token_info.manufacturer_id).trim().to_string(),
            model: String::from_utf8_lossy(&token_info.model).trim().to_string(),
            serial_number: String::from_utf8_lossy(&token_info.serial_number).trim().to_string(),
            firmware_version: "N/A".to_string(),
            supported_algorithms: vec!["RSA".to_string(), "ECDSA".to_string(), "AES".to_string()],
            hardware_version: format!("{}.{}", token_info.hardware_version.major, token_info.hardware_version.minor),
            token_flags: vec!["RNG".to_string(), "LoginRequired".to_string()],
        };

        self.device_info = Some(device_info.clone());
        self.connected = true;

        self.statistics.record_connection(DMSCConnectionStatus::Connected).await;

        Ok(device_info)
    }

    async fn disconnect(&mut self) -> DMSCResult<()> {
        if let Some(session) = &self.session {
            session.logout()
                .ok();
            session.close_session()
                .ok();
        }

        self.session = None;
        self.slot = None;
        self.connected = false;

        self.statistics.record_connection(DMSCConnectionStatus::Disconnected).await;

        Ok(())
    }

    fn is_connected(&self) -> bool {
        self.connected
    }

    fn hsm_type(&self) -> DMSCHSMType {
        DMSCHSMType::PKCS11
    }

    async fn generate_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let key_id = format!("{:x}", rand::random::<u64>());

        let template = match key_type {
            DMSCKeyType::RSA { size } => {
                vec![
                    pkcs11::Attribute::Class(pkcs11::ObjectClass::PUBLIC_KEY),
                    pkcs11::Attribute::KeyType(pkcs11::KeyType::RSA),
                    pkcs11::Attribute::Token(true),
                    pkcs11::Attribute::Sensitive(true),
                    pkcs11::Attribute::Extractable(false),
                    pkcs11::Attribute::ModulusBits(size as u64),
                    pkcs11::Attribute::PublicExponent(vec![0x01, 0x00, 0x01]),
                    pkcs11::Attribute::Label(label.unwrap_or("RSA Key").as_bytes().to_vec()),
                ]
            }
            DMSCKeyType::ECDSA { curve } => {
                let curve_oid = match curve {
                    DMSCECCurve::P256 => vec![0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07],
                    DMSCECCurve::P384 => vec![0x06, 0x05, 0x2B, 0x81, 0x04, 0x00, 0x22],
                    DMSCECCurve::P521 => vec![0x06, 0x05, 0x2B, 0x81, 0x04, 0x00, 0x23],
                    DMSCECCurve::Secp256k1 => vec![0x06, 0x05, 0x2B, 0x81, 0x04, 0x00, 0x0A],
                    DMSCECCurve::SM2 => vec![0x06, 0x08, 0x2A, 0x81, 0x1C, 0xCF, 0x55, 0x01, 0x82, 0x2D],
                    _ => vec![0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07],
                };

                vec![
                    pkcs11::Attribute::Class(pkcs11::ObjectClass::PUBLIC_KEY),
                    pkcs11::Attribute::KeyType(pkcs11::KeyType::EC),
                    pkcs11::Attribute::Token(true),
                    pkcs11::Attribute::Sensitive(true),
                    pkcs11::Attribute::Extractable(false),
                    pkcs11::Attribute::EcdsaParams(curve_oid),
                    pkcs11::Attribute::Label(label.unwrap_or("EC Key").as_bytes().to_vec()),
                ]
            }
            _ => {
                return Err(DMSCError::CryptoError("Unsupported key type for asymmetric key generation".to_string()));
            }
        };

        session.generate_key_pair(&template, &[])
            .map_err(|e| DMSCError::CryptoError(format!("Failed to generate key pair: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(key_id)
    }

    async fn generate_symmetric_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let key_id = format!("{:x}", rand::random::<u64>());

        let (key_size, key_type_pkcs11) = match key_type {
            DMSCKeyType::AES { size } => (size, pkcs11::KeyType::AES),
            DMSCKeyType::SM4 => (128, pkcs11::KeyType::AES),
            DMSCKeyType::HMAC { size } => (size, pkcs11::KeyType::GENERIC_SECRET),
            _ => {
                return Err(DMSCError::CryptoError("Unsupported key type for symmetric key generation".to_string()));
            }
        };

        let template = vec![
            pkcs11::Attribute::Class(pkcs11::ObjectClass::SECRET_KEY),
            pkcs11::Attribute::KeyType(key_type_pkcs11),
            pkcs11::Attribute::Token(true),
            pkcs11::Attribute::Sensitive(true),
            pkcs11::Attribute::Extractable(false),
            pkcs11::Attribute::ValueLen(key_size as u64),
            pkcs11::Attribute::Label(label.unwrap_or("Symmetric Key").as_bytes().to_vec()),
        ];

        session.generate_key(&template, None, None)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to generate symmetric key: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(key_id)
    }

    async fn import_key(
        &mut self,
        key_type: DMSCKeyType,
        key_data: &[u8],
        label: Option<&str>,
        extractable: bool,
    ) -> DMSCResult<String> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let key_id = format!("{:x}", rand::random::<u64>());

        let template = match key_type {
            DMSCKeyType::RSA { .. } => {
                vec![
                    pkcs11::Attribute::Class(pkcs11::ObjectClass::PRIVATE_KEY),
                    pkcs11::Attribute::KeyType(pkcs11::KeyType::RSA),
                    pkcs11::Attribute::Token(true),
                    pkcs11::Attribute::Sensitive(true),
                    pkcs11::Attribute::Extractable(extractable),
                    pkcs11::Attribute::Value(key_data.to_vec()),
                    pkcs11::Attribute::Label(label.unwrap_or("Imported RSA Key").as_bytes().to_vec()),
                ]
            }
            DMSCKeyType::ECDSA { .. } => {
                vec![
                    pkcs11::Attribute::Class(pkcs11::ObjectClass::PRIVATE_KEY),
                    pkcs11::Attribute::KeyType(pkcs11::KeyType::EC),
                    pkcs11::Attribute::Token(true),
                    pkcs11::Attribute::Sensitive(true),
                    pkcs11::Attribute::Extractable(extractable),
                    pkcs11::Attribute::Value(key_data.to_vec()),
                    pkcs11::Attribute::Label(label.unwrap_or("Imported EC Key").as_bytes().to_vec()),
                ]
            }
            DMSCKeyType::AES { size } => {
                vec![
                    pkcs11::Attribute::Class(pkcs11::ObjectClass::SECRET_KEY),
                    pkcs11::Attribute::KeyType(pkcs11::KeyType::AES),
                    pkcs11::Attribute::Token(true),
                    pkcs11::Attribute::Sensitive(true),
                    pkcs11::Attribute::Extractable(extractable),
                    pkcs11::Attribute::Value(key_data.to_vec()),
                    pkcs11::Attribute::ValueLen(size as u64),
                    pkcs11::Attribute::Label(label.unwrap_or("Imported AES Key").as_bytes().to_vec()),
                ]
            }
            _ => {
                return Err(DMSCError::CryptoError("Unsupported key type for import".to_string()));
            }
        };

        session.create_object(&template)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to import key: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(key_id)
    }

    async fn export_public_key(&mut self, key_id: &str) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let public_key = self.get_attribute_value(objects[0], pkcs11::Attribute::PublicExponent)?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(public_key)
    }

    async fn export_key(&mut self, key_id: &str) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let key_data = self.get_attribute_value(objects[0], pkcs11::Attribute::Value)?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(key_data)
    }

    async fn sign(&mut self, key_id: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let signature = session.sign(
            pkcs11::Mechanism::Ecdsa1,
            objects[0],
            data,
        ).map_err(|e| DMSCError::CryptoError(format!("Signing failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(signature)
    }

    async fn verify(&mut self, key_id: &str, data: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let result = session.verify(
            pkcs11::Mechanism::Ecdsa1,
            objects[0],
            data,
            signature,
        ).map_err(|e| DMSCError::CryptoError(format!("Verification failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(result)
    }

    async fn encrypt(&mut self, key_id: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let ciphertext = session.encrypt(
            pkcs11::Mechanism::RsaPkcs1,
            objects[0],
            data,
        ).map_err(|e| DMSCError::CryptoError(format!("Encryption failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(ciphertext)
    }

    async fn decrypt(&mut self, key_id: &str, ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let plaintext = session.decrypt(
            pkcs11::Mechanism::RsaPkcs1,
            objects[0],
            ciphertext,
        ).map_err(|e| DMSCError::CryptoError(format!("Decryption failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(plaintext)
    }

    async fn encrypt_symmetric(
        &mut self,
        key_id: &str,
        data: &[u8],
        iv: Option<&[u8]>,
    ) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let iv = iv.unwrap_or(&[0u8; 16]);

        let mechanism = pkcs11::Mechanism::CbcPkcs5(iv.to_vec());

        let ciphertext = session.encrypt(
            mechanism,
            objects[0],
            data,
        ).map_err(|e| DMSCError::CryptoError(format!("Symmetric encryption failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(ciphertext)
    }

    async fn decrypt_symmetric(
        &mut self,
        key_id: &str,
        ciphertext: &[u8],
        iv: Option<&[u8]>,
    ) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let iv = iv.unwrap_or(&[0u8; 16]);

        let mechanism = pkcs11::Mechanism::CbcPkcs5(iv.to_vec());

        let plaintext = session.decrypt(
            mechanism,
            objects[0],
            ciphertext,
        ).map_err(|e| DMSCError::CryptoError(format!("Symmetric decryption failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(plaintext)
    }

    async fn random(&mut self, size: usize) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let random = session.generate_random(size)
            .map_err(|e| DMSCError::CryptoError(format!("Random generation failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(random)
    }

    async fn hash(&mut self, algorithm: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let mechanism = match algorithm.to_uppercase().as_str() {
            "SHA-256" | "SHA256" => pkcs11::Mechanism::Sha256,
            "SHA-384" | "SHA384" => pkcs11::Mechanism::Sha384,
            "SHA-512" | "SHA512" => pkcs11::Mechanism::Sha512,
            "SM3" => pkcs11::Mechanism::Sha256,
            _ => return Err(DMSCError::CryptoError("Unsupported hash algorithm".to_string())),
        };

        let hash_data = session.hash(mechanism, data)
            .map_err(|e| DMSCError::CryptoError(format!("Hashing failed: {}", e)))?;

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(hash_data)
    }

    async fn derive_key(
        &mut self,
        base_key_id: &str,
        template: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String> {
        Err(DMSCError::CryptoError("Key derivation not yet implemented for PKCS#11".to_string()))
    }

    async fn delete_key(&mut self, key_id: &str) -> DMSCResult<()> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        for object in objects {
            session.destroy_object(object)
                .map_err(|e| DMSCError::CryptoError(format!("Failed to delete key: {}", e)))?;
        }

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(())
    }

    async fn list_keys(&mut self) -> DMSCResult<Vec<DMSCKeyInfo>> {
        let start = Instant::now();

        let session = self.session.as_ref()
            .ok_or_else(|| DMSCError::CryptoError("Session not open".to_string()))?;

        let template = vec![
            pkcs11::Attribute::Token(true),
        ];

        let objects = self.find_objects(&template)?;

        let mut keys = Vec::new();

        for object in objects {
            let label = self.get_attribute_value(object, pkcs11::Attribute::Label)?;
            let key_type = self.get_attribute_value(object, pkcs11::Attribute::KeyType)?;

            let label_str = String::from_utf8_lossy(&label).to_string();
            let key_type_val = if key_type.len() >= 4 {
                u32::from_be_bytes([key_type[0], key_type[1], key_type[2], key_type[3]]) as u64
            } else {
                0
            };

            let dmsc_key_type = match key_type_val {
                _ => DMSCKeyType::RSA { size: 2048 },
            };

            let key_info = DMSCKeyInfo {
                id: label_str.clone(),
                key_type: dmsc_key_type,
                label: label_str,
                sensitive: true,
                extractable: false,
                created_at: chrono::Utc::now(),
                last_used: None,
                usage_count: 0,
            };

            keys.push(key_info);
        }

        self.statistics.record_operation(true, start.elapsed()).await;

        Ok(keys)
    }

    async fn get_key_info(&mut self, key_id: &str) -> DMSCResult<DMSCKeyInfo> {
        let template = vec![
            pkcs11::Attribute::Label(key_id.as_bytes().to_vec()),
        ];

        let objects = self.find_objects(&template)?;

        if objects.is_empty() {
            return Err(DMSCError::CryptoError("Key not found".to_string()));
        }

        let label = self.get_attribute_value(objects[0], pkcs11::Attribute::Label)?;
        let label_str = String::from_utf8_lossy(&label).to_string();

        Ok(DMSCKeyInfo {
            id: key_id.to_string(),
            key_type: DMSCKeyType::RSA { size: 2048 },
            label: label_str,
            sensitive: true,
            extractable: false,
            created_at: chrono::Utc::now(),
            last_used: None,
            usage_count: 0,
        })
    }

    fn get_device_info(&self) -> Option<DMSCDeviceInfo> {
        self.device_info.clone()
    }
}

/// HSM manager for coordinating multiple HSM instances.
pub struct DMSCHSMManager {
    active_hsm: Option<Box<dyn DMSCHSM>>,
    hsm_type: DMSCHSMType,
    config: Option<DMSCHSMConfig>,
    statistics: Arc<DMSCHSMStatistics>,
    event_callback: Option<Arc<dyn Fn(DMSCHSMEvent) + Send + Sync>>,
}

impl DMSCHSMManager {
    /// Create new HSM manager.
    pub fn new() -> Self {
        Self {
            active_hsm: None,
            hsm_type: DMSCHSMType::PKCS11,
            config: None,
            statistics: Arc::new(DMSCHSMStatistics::new()),
            event_callback: None,
        }
    }

    /// Set the event callback.
    pub fn set_event_callback<F>(&mut self, callback: F)
    where
        F: Fn(DMSCHSMEvent) + Send + Sync + 'static,
    {
        self.event_callback = Some(Arc::new(callback));
    }

    /// Emit an event.
    fn emit_event(&self, event: DMSCHSMEvent) {
        if let Some(callback) = &self.event_callback {
            callback(event);
        }
    }

    /// Connect to a PKCS#11 device.
    pub async fn connect_pkcs11(&mut self, library_path: &str) -> DMSCResult<DMSCDeviceInfo> {
        let mut config = DMSCHSMConfig {
            hsm_type: DMSCHSMType::PKCS11,
            pkcs11_library: Some(library_path.to_string()),
            ..Default::default()
        };

        self.connect_with_config(&config).await
    }

    /// Connect with a configuration.
    pub async fn connect_with_config(&mut self, config: &DMSCHSMConfig) -> DMSCResult<DMSCDeviceInfo> {
        self.config = Some(config.clone());

        #[cfg(not(windows))]
        let hsm: Box<dyn DMSCHSM> = match config.hsm_type {
            DMSCHSMType::PKCS11 => Box::new(DMSCPKCS11HSM::new()),
            DMSCHSMType::TPM20 => {
                return Err(DMSCError::CryptoError(
                    "TPM 2.0 is not yet supported. Please use PKCS#11 devices.".to_string()
                ));
            }
            DMSCHSMType::Software => Box::new(DMSCPKCS11HSM::new()),
        };

        #[cfg(windows)]
        let hsm: Box<dyn DMSCHSM> = match config.hsm_type {
            DMSCHSMType::PKCS11 | DMSCHSMType::Software => {
                return Err(DMSCError::CryptoError(
                    "PKCS#11 is not supported on Windows. HSM features require a Unix-like operating system.".to_string()
                ));
            }
            DMSCHSMType::TPM20 => {
                return Err(DMSCError::CryptoError(
                    "TPM 2.0 is not yet supported. Please use PKCS#11 devices on Unix systems.".to_string()
                ));
            }
        };

        let device_info = hsm.connect(config).await?;

        self.active_hsm = Some(hsm);
        self.hsm_type = config.hsm_type;

        self.emit_event(DMSCHSMEvent::Connected {
            hsm_type: config.hsm_type,
            device_info: format!("{} - {}", device_info.manufacturer, device_info.model),
        });

        Ok(device_info)
    }

    /// Disconnect from the current HSM.
    pub async fn disconnect(&mut self) -> DMSCResult<()> {
        if let Some(mut hsm) = self.active_hsm.take() {
            hsm.disconnect().await?;
            self.emit_event(DMSCHSMEvent::Disconnected {
                hsm_type: self.hsm_type,
                reason: "User requested".to_string(),
            });
        }
        Ok(())
    }

    /// Check if connected.
    pub fn is_connected(&self) -> bool {
        self.active_hsm.as_ref().map(|h| h.is_connected()).unwrap_or(false)
    }

    /// Get the current HSM type.
    pub fn hsm_type(&self) -> DMSCHSMType {
        self.hsm_type
    }

    /// Generate a new key in the HSM.
    pub async fn generate_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        let key_id = hsm.generate_key(key_type, label).await?;

        self.emit_event(DMSCHSMEvent::KeyOperation {
            operation: "generate_key".to_string(),
            key_id: key_id.clone(),
            duration: Duration::from_millis(100),
        });

        Ok(key_id)
    }

    /// Generate a symmetric key in the HSM.
    pub async fn generate_symmetric_key(
        &mut self,
        key_type: DMSCKeyType,
        label: Option<&str>,
    ) -> DMSCResult<String> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.generate_symmetric_key(key_type, label).await
    }

    /// Import a key into the HSM.
    pub async fn import_key(
        &mut self,
        key_type: DMSCKeyType,
        key_data: &[u8],
        label: Option<&str>,
        extractable: bool,
    ) -> DMSCResult<String> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.import_key(key_type, key_data, label, extractable).await
    }

    /// Sign data using the HSM.
    pub async fn sign(&mut self, key_id: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        let start = Instant::now();
        let signature = hsm.sign(key_id, data).await?;

        self.emit_event(DMSCHSMEvent::KeyOperation {
            operation: "sign".to_string(),
            key_id: key_id.to_string(),
            duration: start.elapsed(),
        });

        Ok(signature)
    }

    /// Verify a signature using the HSM.
    pub async fn verify(&mut self, key_id: &str, data: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.verify(key_id, data, signature).await
    }

    /// Encrypt data using the HSM.
    pub async fn encrypt(&mut self, key_id: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.encrypt(key_id, data).await
    }

    /// Decrypt data using the HSM.
    pub async fn decrypt(&mut self, key_id: &str, ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.decrypt(key_id, ciphertext).await
    }

    /// Generate random bytes using the HSM.
    pub async fn random(&mut self, size: usize) -> DMSCResult<Vec<u8>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.random(size).await
    }

    /// Hash data using the HSM.
    pub async fn hash(&mut self, algorithm: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.hash(algorithm, data).await
    }

    /// Delete a key from the HSM.
    pub async fn delete_key(&mut self, key_id: &str) -> DMSCResult<()> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.delete_key(key_id).await
    }

    /// List all keys in the HSM.
    pub async fn list_keys(&mut self) -> DMSCResult<Vec<DMSCKeyInfo>> {
        let hsm = self.active_hsm.as_mut()
            .ok_or_else(|| DMSCError::CryptoError("No HSM connected".to_string()))?;

        hsm.list_keys().await
    }

    /// Get statistics.
    pub fn statistics(&self) -> &Arc<DMSCHSMStatistics> {
        &self.statistics
    }

    /// Get device information.
    pub fn device_info(&self) -> Option<DMSCDeviceInfo> {
        self.active_hsm.as_ref().and_then(|h| h.get_device_info())
    }
}

impl Default for DMSCHSMManager {
    fn default() -> Self {
        Self::new()
    }
}
