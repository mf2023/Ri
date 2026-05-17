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

//! # Security Utilities Module
//!
//! This module provides security-related utilities for Ri, including:
//! - Configuration encryption and decryption using AES-256-GCM
//! - Sensitive data protection with HMAC-SHA256 signing
//! - Cryptographic key generation and management
//!
//! ## Encryption
//!
//! The module uses AES-256-GCM (Galois/Counter Mode) for symmetric encryption,
//! providing both confidentiality and authenticity. Nonce values are generated
//! randomly for each encryption operation.
//!
//! ## HMAC Signing
//!
//! HMAC-SHA256 is used for message authentication, ensuring data integrity
//! and authenticity. Both signing and verification functions are provided.
//!
//! ## Key Management
//!
//! Encryption and HMAC keys are loaded from environment variables:
//! - `Ri_ENCRYPTION_KEY`: 32-byte hex-encoded key for encryption
//! - `Ri_HMAC_KEY`: 32-byte hex-encoded key for HMAC
//!
//! If not set, keys are generated randomly using cryptographically secure
//! random number generators.
//!
//! ## Security Considerations
//!
//! - Keys should be stored securely in production environments
//! - Randomly generated keys are lost on application restart
//! - Consider using a secrets management solution for production
//! - Encrypted data includes a random nonce, so the same plaintext encrypts differently each time

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use generic_array::GenericArray;
use rand::RngCore;
use ring::hmac;
use std::env;

use crate::core::error::RiError;
use crate::core::error::RiResult;

#[cfg(feature = "pyo3")]
pub(crate) fn ri_error_to_py_err(e: RiError) -> pyo3::prelude::PyErr {
    use pyo3::exceptions::*;
    
    match e {
        RiError::InvalidInput(_) | RiError::InvalidState(_) | RiError::SecurityViolation(_)
        | RiError::TomlError(_) | RiError::YamlError(_) | RiError::FrameError(_) => {
            PyValueError::new_err(e.to_string())
        }
        RiError::DeviceNotFound { .. } | RiError::AllocationNotFound { .. }
        | RiError::ModuleNotFound { .. } | RiError::MissingDependency { .. } => {
            PyKeyError::new_err(e.to_string())
        }
        RiError::CircularDependency { .. } => {
            PyValueError::new_err(e.to_string())
        }
        RiError::Io(_) | RiError::Config(_) | RiError::Serde(_) | RiError::Hook(_)
        | RiError::Prometheus(_) | RiError::ServiceMesh(_) | RiError::DeviceAllocationFailed { .. }
        | RiError::ModuleInitFailed { .. } | RiError::ModuleStartFailed { .. } | RiError::ModuleShutdownFailed { .. }
        | RiError::Other(_) | RiError::ExternalError(_) | RiError::PoolError(_) | RiError::DeviceError(_)
        | RiError::RedisError(_) | RiError::HttpClientError(_) | RiError::Queue(_)
        | RiError::Database(_) => {
            PyRuntimeError::new_err(e.to_string())
        }
    }
}

const ENCRYPTION_KEY_ENV: &str = "Ri_ENCRYPTION_KEY";
const HMAC_KEY_ENV: &str = "Ri_HMAC_KEY";
const DEFAULT_KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

static ENCRYPTION_KEY_WARNED: std::sync::Once = std::sync::Once::new();
static HMAC_KEY_WARNED: std::sync::Once = std::sync::Once::new();

fn load_or_generate_key(env_var: &str, length: usize, key_name: &str, warned: &std::sync::Once) -> Vec<u8> {
    if let Ok(s) = env::var(env_var) {
        if let Ok(key) = hex::decode(&s) {
            if key.len() >= 16 {
                return key;
            }
            tracing::warn!(
                "{} from {} is too short ({} bytes), minimum 16 bytes required",
                key_name, env_var, key.len()
            );
        }
    }
    
    warned.call_once(|| {
        tracing::warn!(
            "SECURITY WARNING: {} not set or invalid. Using ephemeral random key. \
            Encrypted data will be lost on restart! Set {} environment variable.",
            key_name, env_var
        );
    });
    
    let mut key = vec![0u8; length];
    rand::thread_rng().fill_bytes(&mut key);
    key
}

fn load_encryption_key() -> Vec<u8> {
    load_or_generate_key(ENCRYPTION_KEY_ENV, DEFAULT_KEY_LENGTH, "Encryption key", &ENCRYPTION_KEY_WARNED)
}

fn load_hmac_key() -> Vec<u8> {
    load_or_generate_key(HMAC_KEY_ENV, DEFAULT_KEY_LENGTH, "HMAC key", &HMAC_KEY_WARNED)
}

/// Checks if encryption keys are properly configured.
///
/// Returns `Ok(())` if both encryption and HMAC keys are set via environment variables.
/// Returns an error if any key is missing, with instructions on how to set them.
#[allow(dead_code)]
pub fn check_encryption_keys() -> RiResult<()> {
    let encryption_key_set = env::var(ENCRYPTION_KEY_ENV)
        .ok()
        .and_then(|s| hex::decode(&s).ok())
        .map(|k| k.len() >= 16)
        .unwrap_or(false);
    
    let hmac_key_set = env::var(HMAC_KEY_ENV)
        .ok()
        .and_then(|s| hex::decode(&s).ok())
        .map(|k| k.len() >= 16)
        .unwrap_or(false);
    
    if !encryption_key_set || !hmac_key_set {
        let mut missing = Vec::new();
        if !encryption_key_set {
            missing.push(ENCRYPTION_KEY_ENV);
        }
        if !hmac_key_set {
            missing.push(HMAC_KEY_ENV);
        }
        
        return Err(RiError::SecurityViolation(format!(
            "Encryption keys not configured: {}. \
            Generate keys using RiSecurityManager::generate_encryption_key() and \
            RiSecurityManager::generate_hmac_key(), then set them as environment variables. \
            WARNING: Without proper keys, encrypted data will be lost on restart!",
            missing.join(", ")
        )));
    }
    
    Ok(())
}

/// Security utilities manager for Ri.
///
/// This struct provides static methods for encryption, decryption, HMAC signing,
/// and key management operations. It is designed as a singleton utility class
/// with no instance state.
///
/// ## Thread Safety
///
/// All methods are stateless and can be safely called concurrently from multiple threads.
///
/// ## Usage
///
/// ```rust,ignore
/// use ri::auth::security::RiSecurityManager;
///
/// // Encrypt sensitive data
/// let encrypted = RiSecurityManager::encrypt("secret data");
///
/// // Decrypt data
/// let decrypted = RiSecurityManager::decrypt(&encrypted);
///
/// // Sign data with HMAC
/// let signature = RiSecurityManager::hmac_sign("data to sign");
///
/// // Verify HMAC signature
/// let is_valid = RiSecurityManager::hmac_verify("data to verify", &signature);
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiSecurityManager;

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiSecurityManager {
    #[new]
    fn py_new() -> Self {
        Self
    }

    #[staticmethod]
    fn encrypt_py(plaintext: &str) -> pyo3::prelude::PyResult<String> {
        Self::encrypt(plaintext).map_err(ri_error_to_py_err)
    }

    #[staticmethod]
    fn decrypt_py(encrypted: &str) -> pyo3::prelude::PyResult<String> {
        Self::decrypt(encrypted).map_err(ri_error_to_py_err)
    }

    #[staticmethod]
    fn hmac_sign_py(data: &str) -> String {
        Self::hmac_sign(data)
    }

    #[staticmethod]
    fn hmac_verify_py(data: &str, signature: &str) -> bool {
        Self::hmac_verify(data, signature)
    }

    #[staticmethod]
    fn generate_encryption_key_py() -> String {
        Self::generate_encryption_key()
    }

    #[staticmethod]
    fn generate_hmac_key_py() -> String {
        Self::generate_hmac_key()
    }
}

impl RiSecurityManager {
    /// Encrypts plaintext data using AES-256-GCM.
    ///
    /// This method encrypts the input string using AES-256-GCM (Galois/Counter Mode),
    /// which provides both confidentiality and authenticity. A random nonce is generated
    /// for each encryption operation, so the same plaintext produces different ciphertext
    /// each time it is encrypted.
    ///
    /// ## Output Format
    ///
    /// The output is Base64-encoded and contains:
    /// - 12-byte nonce (randomly generated)
    /// - Encrypted data with authentication tag
    ///
    /// # Parameters
    ///
    /// - `plaintext`: The text string to encrypt
    ///
    /// # Returns
    ///
    /// `RiResult<String>` containing Base64-encoded encrypted data on success
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let encrypted = RiSecurityManager::encrypt("sensitive data");
    /// println!("Encrypted: {}", encrypted);
    /// ```
    pub fn encrypt(plaintext: &str) -> RiResult<String> {
        let key = load_encryption_key();
        let nonce = {
            let mut n = [0u8; NONCE_LENGTH];
            rand::thread_rng().fill_bytes(&mut n);
            n
        };

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
            .map_err(|e| RiError::SecurityViolation(format!("encryption failed: {}", e)))?;

        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        Ok(STANDARD.encode(result))
    }

    /// Decrypts encrypted data using AES-256-GCM.
    ///
    /// This method decrypts data that was encrypted using the `encrypt` method.
    /// It verifies the authentication tag and returns the original plaintext.
    ///
    /// ## Failure Conditions
    ///
    /// Returns `Err(RiError::SecurityViolation(...))` if:
    /// - The input is not valid Base64
    /// - The input is shorter than the nonce length
    /// - The authentication tag verification fails (wrong key or tampered data)
    /// - UTF-8 decoding of the decrypted data fails
    ///
    /// # Parameters
    ///
    /// - `encrypted`: Base64-encoded encrypted data
    ///
    /// # Returns
    ///
    /// `RiResult<String>` containing the decrypted plaintext on success
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let encrypted = RiSecurityManager::encrypt("secret")?;
    /// let decrypted = RiSecurityManager::decrypt(&encrypted)?;
    /// println!("Decrypted: {}", decrypted);
    /// ```
    pub fn decrypt(encrypted: &str) -> RiResult<String> {
        let key = load_encryption_key();
        let data = STANDARD.decode(encrypted)
            .map_err(|e| RiError::SecurityViolation(format!("Base64 decode failed: {}", e)))?;

        if data.len() < NONCE_LENGTH {
            return Err(RiError::SecurityViolation(
                format!("Encrypted data too short: expected at least {} bytes, got {}",
                    NONCE_LENGTH, data.len())
            ));
        }

        let (nonce, ciphertext) = data.split_at(NONCE_LENGTH);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

        let plaintext = cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .map_err(|e| RiError::SecurityViolation(format!("Decryption failed: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| RiError::SecurityViolation(format!("UTF-8 decode failed: {}", e)))
    }

    /// Signs data using HMAC-SHA256.
    ///
    /// This method creates an HMAC signature using the configured HMAC key
    /// and SHA-256 hash algorithm. The signature is returned as a hex-encoded string.
    ///
    /// ## Security
    ///
    /// HMAC provides message integrity and authenticity verification. Only parties
    /// with access to the HMAC key can create or verify signatures.
    ///
    /// # Parameters
    ///
    /// - `data`: The data string to sign
    ///
    /// # Returns
    ///
    /// Hex-encoded HMAC signature
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let data = "important message";
    /// let signature = RiSecurityManager::hmac_sign(data);
    /// println!("Signature: {}", signature);
    /// ```
    pub fn hmac_sign(data: &str) -> String {
        let key = load_hmac_key();
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, &key);
        let signature = hmac::sign(&signing_key, data.as_bytes());
        hex::encode(signature)
    }

    /// Verifies an HMAC-SHA256 signature.
    ///
    /// This method verifies that the provided signature matches the data using
    /// constant-time comparison to prevent timing attacks.
    ///
    /// ## Signature Format
    ///
    /// The signature must be a valid hex-encoded string as produced by `hmac_sign`.
    ///
    /// # Parameters
    ///
    /// - `data`: The original data that was signed
    /// - `signature`: The hex-encoded signature to verify
    ///
    /// # Returns
    ///
    /// `true` if the signature is valid, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let data = "important message";
    /// let signature = RiSecurityManager::hmac_sign(data);
    ///
    /// if RiSecurityManager::hmac_verify(data, &signature) {
    ///     println!("Signature is valid!");
    /// } else {
    ///     println!("Signature is invalid!");
    /// }
    /// ```
    pub fn hmac_verify(data: &str, signature: &str) -> bool {
        let expected = match hex::decode(signature) {
            Ok(sig) => sig,
            Err(_) => {
                log::warn!("[Ri.Security] Invalid hex signature format");
                return false;
            }
        };
        let key = load_hmac_key();
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, &key);
        hmac::verify(&signing_key, data.as_bytes(), &expected).is_ok()
    }

    /// Generates a new encryption key.
    ///
    /// This method generates a cryptographically secure random 32-byte (256-bit) key
    /// suitable for AES-256 encryption. The key is returned as a hex-encoded string.
    ///
    /// ## Usage
    ///
    /// This method can be used to generate keys for initial configuration or key rotation.
    /// Store the generated key securely and set it via the `Ri_ENCRYPTION_KEY` environment variable.
    ///
    /// # Returns
    ///
    /// Hex-encoded 32-byte encryption key
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let key = RiSecurityManager::generate_encryption_key();
    /// println!("New encryption key: {}", key);
    /// ```
    pub fn generate_encryption_key() -> String {
        let mut key = vec![0u8; DEFAULT_KEY_LENGTH];
        rand::thread_rng().fill_bytes(&mut key);
        hex::encode(key)
    }

    /// Generates a new HMAC key.
    ///
    /// This method generates a cryptographically secure random 32-byte (256-bit) key
    /// suitable for HMAC-SHA256 signing. The key is returned as a hex-encoded string.
    ///
    /// ## Usage
    ///
    /// This method can be used to generate keys for initial configuration or key rotation.
    /// Store the generated key securely and set it via the `Ri_HMAC_KEY` environment variable.
    ///
    /// # Returns
    ///
    /// Hex-encoded 32-byte HMAC key
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let key = RiSecurityManager::generate_hmac_key();
    /// println!("New HMAC key: {}", key);
    /// ```
    pub fn generate_hmac_key() -> String {
        let mut key = vec![0u8; DEFAULT_KEY_LENGTH];
        rand::thread_rng().fill_bytes(&mut key);
        hex::encode(key)
    }
}
