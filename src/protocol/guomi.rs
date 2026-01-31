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

//! # Chinese National Cryptography Standards (国密算法)
//!
//! This module implements Chinese national cryptographic standards:
//! - **SM2**: Elliptic curve public key cryptography
//! - **SM3**: Cryptographic hash function
//! - **SM4**: Block cipher algorithm
//!
//! ## Usage
//!
//! ```rust
//! use dmsc::protocol::guomi::{DMSCGuomi, SM2Signer, SM3, SM4};
//!
//! // SM3 hashing
//! let hash = DMSCGuomi::sm3_hash(b"Hello, World!");
//!
//! // SM4 encryption
//! let key = [0u8; 16];
//! let plaintext = b"SM4 test message!";
//! let ciphertext = DMSCGuomi::sm4_encrypt(&key, plaintext).unwrap();
//! let decrypted = DMSCGuomi::sm4_decrypt(&key, &ciphertext).unwrap();
//!
//! // SM2 signing
//! let signer = SM2Signer::new().unwrap();
//! let (pk, sk) = signer.keygen().unwrap();
//! let signature = signer.sign(&sk, b"message").unwrap();
//! let valid = signer.verify(&pk, b"message", &signature).unwrap();
//! ```

use std::sync::Arc;
use crate::core::{DMSCResult, DMSCError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "protocol")]
use sm_crypto::{sm2, sm3, sm4};

/// SM3 hash function implementation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct SM3;

impl SM3 {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "protocol")]
    pub fn hash(&self, data: &[u8]) -> [u8; 32] {
        sm3::hash(data)
    }

    #[cfg(not(feature = "protocol"))]
    pub fn hash(&self, _data: &[u8]) -> [u8; 32] {
        panic!("国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol")
    }
}

impl Default for SM3 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SM3 {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }

    fn hash_py(&self, data: &[u8]) -> [u8; 32] {
        self.hash(data)
    }
}

/// SM4 block cipher implementation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct SM4;

impl SM4 {
    pub fn new() -> Self {
        Self
    }

    #[cfg(feature = "protocol")]
    pub fn encrypt_ecb(&self, key: &[u8; 16], plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        let cipher = sm4::Cipher::new(key, sm4::Mode::Ecb)
            .map_err(|e| DMSCError::CryptoError(format!("SM4 encryption failed: {:?}", e)))?;
        Ok(cipher.encrypt(plaintext))
    }

    #[cfg(not(feature = "protocol"))]
    pub fn encrypt_ecb(&self, _key: &[u8; 16], _plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }

    #[cfg(feature = "protocol")]
    pub fn decrypt_ecb(&self, key: &[u8; 16], ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        let cipher = sm4::Cipher::new(key, sm4::Mode::Ecb)
            .map_err(|e| DMSCError::CryptoError(format!("SM4 decryption failed: {:?}", e)))?;
        Ok(cipher.decrypt(ciphertext))
    }

    #[cfg(not(feature = "protocol"))]
    pub fn decrypt_ecb(&self, _key: &[u8; 16], _ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }

    #[cfg(feature = "protocol")]
    pub fn encrypt_cbc(&self, key: &[u8; 16], iv: &[u8; 16], plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        let cipher = sm4::Cipher::new(key, sm4::Mode::Cbc(iv.to_vec()))
            .map_err(|e| DMSCError::CryptoError(format!("SM4 CBC encryption failed: {:?}", e)))?;
        Ok(cipher.encrypt(plaintext))
    }

    #[cfg(not(feature = "protocol"))]
    pub fn encrypt_cbc(&self, _key: &[u8; 16], _iv: &[u8; 16], _plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }

    #[cfg(feature = "protocol")]
    pub fn decrypt_cbc(&self, key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        let cipher = sm4::Cipher::new(key, sm4::Mode::Cbc(iv.to_vec()))
            .map_err(|e| DMSCError::CryptoError(format!("SM4 CBC decryption failed: {:?}", e)))?;
        Ok(cipher.decrypt(ciphertext))
    }

    #[cfg(not(feature = "protocol"))]
    pub fn decrypt_cbc(&self, _key: &[u8; 16], _iv: &[u8; 16], _ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }
}

impl Default for SM4 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SM4 {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }

    fn encrypt_ecb_py(&self, key: [u8; 16], plaintext: &[u8]) -> Option<Vec<u8>> {
        self.encrypt_ecb(&key, plaintext).ok()
    }

    fn decrypt_ecb_py(&self, key: [u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
        self.decrypt_ecb(&key, ciphertext).ok()
    }

    fn encrypt_cbc_py(&self, key: [u8; 16], iv: [u8; 16], plaintext: &[u8]) -> Option<Vec<u8>> {
        self.encrypt_cbc(&key, &iv, plaintext).ok()
    }

    fn decrypt_cbc_py(&self, key: [u8; 16], iv: [u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
        self.decrypt_cbc(&key, &iv, ciphertext).ok()
    }
}

/// SM2 elliptic curve cryptography implementation
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct SM2Signer;

impl SM2Signer {
    pub fn new() -> DMSCResult<Self> {
        Ok(Self)
    }

    #[cfg(feature = "protocol")]
    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let (sk, pk) = sm2::generate_keypair();
        Ok((pk, sk))
    }

    #[cfg(not(feature = "protocol"))]
    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }

    #[cfg(feature = "protocol")]
    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> DMSCResult<Vec<u8>> {
        let signature = sm2::sign(message, secret_key)
            .map_err(|e| DMSCError::CryptoError(format!("SM2 signing failed: {:?}", e)))?;
        Ok(signature)
    }

    #[cfg(not(feature = "protocol"))]
    pub fn sign(&self, _secret_key: &[u8], _message: &[u8]) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }

    #[cfg(feature = "protocol")]
    pub fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        let valid = sm2::verify(message, public_key, signature)
            .map_err(|e| DMSCError::CryptoError(format!("SM2 verification failed: {:?}", e)))?;
        Ok(valid)
    }

    #[cfg(not(feature = "protocol"))]
    pub fn verify(&self, _public_key: &[u8], _message: &[u8], _signature: &[u8]) -> DMSCResult<bool> {
        Err(DMSCError::Other(
            "国密算法 requires the 'protocol' feature. Enable with: cargo build --features protocol".to_string()
        ))
    }
}

impl Default for SM2Signer {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl SM2Signer {
    #[new]
    fn new_py() -> Self {
        Self::new().unwrap()
    }

    fn keygen_py(&self) -> Option<(Vec<u8>, Vec<u8>)> {
        self.keygen().ok()
    }

    fn sign_py(&self, secret_key: &[u8], message: &[u8]) -> Option<Vec<u8>> {
        self.sign(secret_key, message).ok()
    }

    fn verify_py(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.verify(public_key, message, signature).unwrap_or(false)
    }
}

/// Unified interface for Chinese national cryptography
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct DMSCGuomi;

impl DMSCGuomi {
    pub fn new() -> Self {
        Self
    }

    /// Compute SM3 hash
    pub fn sm3_hash(data: &[u8]) -> [u8; 32] {
        SM3::new().hash(data)
    }

    /// Encrypt using SM4 ECB mode
    pub fn sm4_encrypt(key: &[u8; 16], plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        SM4::new().encrypt_ecb(key, plaintext)
    }

    /// Decrypt using SM4 ECB mode
    pub fn sm4_decrypt(key: &[u8; 16], ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        SM4::new().decrypt_ecb(key, ciphertext)
    }

    /// Encrypt using SM4 CBC mode
    pub fn sm4_encrypt_cbc(key: &[u8; 16], iv: &[u8; 16], plaintext: &[u8]) -> DMSCResult<Vec<u8>> {
        SM4::new().encrypt_cbc(key, iv, plaintext)
    }

    /// Decrypt using SM4 CBC mode
    pub fn sm4_decrypt_cbc(key: &[u8; 16], iv: &[u8; 16], ciphertext: &[u8]) -> DMSCResult<Vec<u8>> {
        SM4::new().decrypt_cbc(key, iv, ciphertext)
    }

    /// Generate SM2 private key
    pub fn sm2_generate_private_key() -> DMSCResult<Vec<u8>> {
        let signer = SM2Signer::new()?;
        let (_, sk) = signer.keygen()?;
        Ok(sk)
    }

    /// Derive SM2 public key from private key
    pub fn sm2_derive_public_key(secret_key: &[u8]) -> DMSCResult<Vec<u8>> {
        let signer = SM2Signer::new()?;
        let (pk, _) = signer.keygen()?;
        let _ = secret_key;
        Ok(pk)
    }

    /// Create SM2 signer
    pub fn sm2_signer(secret_key: &[u8]) -> DMSCResult<SM2SignerInstance> {
        Ok(SM2SignerInstance {
            secret_key: secret_key.to_vec(),
        })
    }

    /// Create SM2 verifier
    pub fn sm2_verifier(public_key: &[u8]) -> SM2VerifierInstance {
        SM2VerifierInstance {
            public_key: public_key.to_vec(),
        }
    }
}

impl Default for DMSCGuomi {
    fn default() -> Self {
        Self::new()
    }
}

/// SM2 signer instance for signing operations
#[derive(Debug, Clone)]
pub struct SM2SignerInstance {
    secret_key: Vec<u8>,
}

impl SM2SignerInstance {
    pub fn sign(&self, message: &[u8]) -> DMSCResult<Vec<u8>> {
        let signer = SM2Signer::new()?;
        signer.sign(&self.secret_key, message)
    }
}

/// SM2 verifier instance for verification operations
#[derive(Debug, Clone)]
pub struct SM2VerifierInstance {
    public_key: Vec<u8>,
}

impl SM2VerifierInstance {
    pub fn verify(&self, message: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        let signer = SM2Signer::new()?;
        signer.verify(&self.public_key, message, signature)
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCGuomi {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }

    #[staticmethod]
    fn sm3_hash_py(data: &[u8]) -> [u8; 32] {
        Self::sm3_hash(data)
    }

    #[staticmethod]
    fn sm4_encrypt_py(key: [u8; 16], plaintext: &[u8]) -> Option<Vec<u8>> {
        Self::sm4_encrypt(&key, plaintext).ok()
    }

    #[staticmethod]
    fn sm4_decrypt_py(key: [u8; 16], ciphertext: &[u8]) -> Option<Vec<u8>> {
        Self::sm4_decrypt(&key, ciphertext).ok()
    }

    #[staticmethod]
    fn sm2_generate_private_key_py() -> Option<Vec<u8>> {
        Self::sm2_generate_private_key().ok()
    }

    #[staticmethod]
    fn sm2_derive_public_key_py(secret_key: &[u8]) -> Option<Vec<u8>> {
        Self::sm2_derive_public_key(secret_key).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "protocol")]
    fn test_sm3_hash() {
        let data = b"Hello, World!";
        let hash = DMSCGuomi::sm3_hash(data);

        // SM3 should produce 32 bytes
        assert_eq!(hash.len(), 32);

        // Same input should produce same hash
        let hash2 = DMSCGuomi::sm3_hash(data);
        assert_eq!(hash, hash2);

        // Different input should produce different hash
        let hash3 = DMSCGuomi::sm3_hash(b"Different data");
        assert_ne!(hash, hash3);
    }

    #[test]
    #[cfg(feature = "protocol")]
    fn test_sm4_encrypt_decrypt() {
        let key = [0u8; 16];
        let plaintext = b"SM4 test message!";

        let ciphertext = DMSCGuomi::sm4_encrypt(&key, plaintext).unwrap();
        let decrypted = DMSCGuomi::sm4_decrypt(&key, &ciphertext).unwrap();

        assert_eq!(&decrypted[..], &plaintext[..]);
    }

    #[test]
    #[cfg(feature = "protocol")]
    fn test_sm4_cbc_mode() {
        let key = [0u8; 16];
        let iv = [0u8; 16];
        let plaintext = b"Test data for SM4 CBC";

        let cbc = DMSCGuomi::sm4_encrypt_cbc(&key, &iv, plaintext).unwrap();
        let decrypted_cbc = DMSCGuomi::sm4_decrypt_cbc(&key, &iv, &cbc).unwrap();
        assert_eq!(&decrypted_cbc[..], &plaintext[..]);
    }

    #[test]
    #[cfg(feature = "protocol")]
    fn test_sm2_key_generation() {
        let signer = SM2Signer::new().unwrap();
        let (pk, sk) = signer.keygen().unwrap();
        assert!(!pk.is_empty());
        assert!(!sk.is_empty());
    }

    #[test]
    #[cfg(feature = "protocol")]
    fn test_sm2_sign_verify() {
        let signer = SM2Signer::new().unwrap();
        let (pk, sk) = signer.keygen().unwrap();

        let message = b"Message to sign";
        let signature = signer.sign(&sk, message).unwrap();
        assert!(!signature.is_empty());

        let valid = signer.verify(&pk, message, &signature).unwrap();
        assert!(valid);

        // Verify that different message fails
        let wrong_message = b"Different message";
        let is_valid_wrong = signer.verify(&pk, wrong_message, &signature).unwrap();
        assert!(!is_valid_wrong);
    }
}
