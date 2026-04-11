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

const ENCRYPTION_KEY_ENV: &str = "Ri_ENCRYPTION_KEY";
const HMAC_KEY_ENV: &str = "Ri_HMAC_KEY";
const DEFAULT_KEY_LENGTH: usize = 32;
const NONCE_LENGTH: usize = 12;

fn load_or_generate_key(env_var: &str, length: usize) -> Vec<u8> {
    env::var(env_var)
        .ok()
        .and_then(|s| hex::decode(s).ok())
        .unwrap_or_else(|| {
            let mut key = vec![0u8; length];
            rand::thread_rng().fill_bytes(&mut key);
            key
        })
}

fn load_encryption_key() -> Vec<u8> {
    load_or_generate_key(ENCRYPTION_KEY_ENV, DEFAULT_KEY_LENGTH)
}

fn load_hmac_key() -> Vec<u8> {
    load_or_generate_key(HMAC_KEY_ENV, DEFAULT_KEY_LENGTH)
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
pub struct RiSecurityManager;

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
    /// Base64-encoded encrypted data
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let encrypted = RiSecurityManager::encrypt("sensitive data");
    /// println!("Encrypted: {}", encrypted);
    /// ```
    pub fn encrypt(plaintext: &str) -> String {
        let key = load_encryption_key();
        let nonce = {
            let mut n = [0u8; NONCE_LENGTH];
            rand::thread_rng().fill_bytes(&mut n);
            n
        };

        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));
        let ciphertext = cipher
            .encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes())
            .expect("encryption failure");

        let mut result = Vec::with_capacity(nonce.len() + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);

        STANDARD.encode(result)
    }

    /// Decrypts encrypted data using AES-256-GCM.
    ///
    /// This method decrypts data that was encrypted using the `encrypt` method.
    /// It verifies the authentication tag and returns the original plaintext.
    ///
    /// ## Failure Conditions
    ///
    /// Returns `None` if:
    /// - The input is not valid Base64
    /// - The input is shorter than the nonce length
    /// - The authentication tag verification fails (wrong key or tampered data)
    ///
    /// # Parameters
    ///
    /// - `encrypted`: Base64-encoded encrypted data
    ///
    /// # Returns
    ///
    /// `Some(String)` containing the decrypted plaintext, or `None` if decryption fails
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use ri::auth::security::RiSecurityManager;
    ///
    /// let encrypted = RiSecurityManager::encrypt("secret");
    /// let decrypted = RiSecurityManager::decrypt(&encrypted);
    ///
    /// match decrypted {
    ///     Some(text) => println!("Decrypted: {}", text),
    ///     None => println!("Decryption failed!"),
    /// }
    /// ```
    pub fn decrypt(encrypted: &str) -> Option<String> {
        let key = load_encryption_key();
        let data = STANDARD.decode(encrypted).ok()?;

        if data.len() < NONCE_LENGTH {
            return None;
        }

        let (nonce, ciphertext) = data.split_at(NONCE_LENGTH);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&key));

        cipher
            .decrypt(Nonce::from_slice(nonce), ciphertext)
            .ok()
            .map(|v| String::from_utf8(v).ok())?
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
        let expected = hex::decode(signature).ok().unwrap_or_default();
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
    /// Store the generated key securely and set it via theRi_HMAC_KEY` environment variable.
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
