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

//! # Post-Quantum Cryptography Module
//!
//! This module provides post-quantum cryptographic algorithms that are resistant
//! to attacks from both classical and quantum computers. It implements:
//!
//! - **Kyber**: IND-CCA2 secure key encapsulation mechanism (KEM)
//! - **Dilithium**: Strongly secure digital signature algorithm
//! - **Falcon**: Compact digital signature algorithm
//!
//! These algorithms are based on hard problems in lattice theory and have been
//! selected by NIST for standardization in the post-quantum cryptography competition.
//!
//! ## Security Properties
//!
//! - **Kyber**: Provides IND-CCA2 security, suitable for key encapsulation
//! - **Dilithium**: Provides EUF-CMA security, suitable for digital signatures
//! - **Falcon**: Provides small signature sizes for bandwidth-constrained scenarios
//!
//! ## Usage
//!
//! ```rust
//! use dmsc::protocol::post_quantum::{KyberKEM, DilithiumSigner};
//!
//! // Kyber key encapsulation
//! let (public_key, secret_key) = KyberKEM::new().keygen()?;
//! let (ciphertext, shared_secret_1) = KyberKEM::new().encapsulate(&public_key)?;
//! let shared_secret_2 = KyberKEM::new().decapsulate(&ciphertext, &secret_key)?;
//! assert_eq!(shared_secret_1, shared_secret_2);
//!
//! // Dilithium signing
//! let (pk, sk) = DilithiumSigner::new().keygen()?;
//! let message = b"Hello, Post-Quantum World!";
//! let signature = DilithiumSigner::new().sign(&sk, message)?;
//! assert!(DilithiumSigner::new().verify(&pk, message, &signature));
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use rand::RngCore;
use subtle::ConstantTimeEq;
use crate::core::{DMSCResult, DMSCError};

pub mod kyber;
pub mod dilithium;
pub mod falcon;

pub use kyber::{KyberKEM, KyberPublicKey, KyberSecretKey, KyberCiphertext};
pub use dilithium::{DilithiumSigner, DilithiumPublicKey, DilithiumSecretKey, DilithiumSignature};
pub use falcon::{FalconSigner, FalconPublicKey, FalconSecretKey, FalconSignature};

/// Post-quantum algorithm type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DMSCPostQuantumAlgorithm {
    /// Kyber KEM
    Kyber512,
    /// Dilithium signature
    Dilithium5,
    /// Falcon signature
    Falcon512,
}

/// Key encapsulation result
pub struct KEMResult {
    /// Ciphertext to send to receiver
    pub ciphertext: Vec<u8>,
    /// Shared secret
    pub shared_secret: Vec<u8>,
}

/// Post-quantum crypto manager
pub struct DMSCPostQuantumManager {
    /// Algorithm selection
    algorithm: Arc<RwLock<DMSCPostQuantumAlgorithm>>,
    /// Kyber KEM instance
    kyber: Arc<RwLock<KyberKEM>>,
    /// Dilithium signer instance
    dilithium: Arc<RwLock<DilithiumSigner>>,
    /// Falcon signer instance
    falcon: Arc<RwLock<FalconSigner>>,
    /// Initialization time
    initialized_at: Arc<RwLock<Instant>>,
    /// Whether initialized
    initialized: Arc<RwLock<bool>>,
}

impl DMSCPostQuantumManager {
    /// Create new post-quantum manager
    pub fn new() -> Self {
        Self {
            algorithm: Arc::new(RwLock::new(DMSCPostQuantumAlgorithm::Kyber512)),
            kyber: Arc::new(RwLock::new(KyberKEM::new())),
            dilithium: Arc::new(RwLock::new(DilithiumSigner::new())),
            falcon: Arc::new(RwLock::new(FalconSigner::new())),
            initialized_at: Arc::new(RwLock::new(Instant::now())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

    /// Initialize the post-quantum manager
    pub async fn initialize(&self, algorithm: DMSCPostQuantumAlgorithm) -> DMSCResult<()> {
        let mut init = self.initialized.write().await;
        if *init {
            return Ok(());
        }

        *self.algorithm.write().await = algorithm;
        *self.initialized_at.write().await = Instant::now();
        *init = true;

        Ok(())
    }

    /// Get current algorithm
    pub async fn algorithm(&self) -> DMSCPostQuantumAlgorithm {
        *self.algorithm.read().await
    }

    /// Generate key pair for key encapsulation
    pub async fn generate_kem_keypair(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let kyber = self.kyber.read().await;
        kyber.keygen()
    }

    /// Encapsulate key using public key
    pub async fn encapsulate(&self, public_key: &[u8]) -> DMSCResult<KEMResult> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let kyber = self.kyber.read().await;
        kyber.encapsulate(public_key)
    }

    /// Decapsulate key using secret key
    pub async fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8]) -> DMSCResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let kyber = self.kyber.read().await;
        kyber.decapsulate(ciphertext, secret_key)
    }

    /// Generate signing key pair
    pub async fn generate_signing_keypair(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let alg = self.algorithm.read().await;
        match *alg {
            DMSCPostQuantumAlgorithm::Kyber512 | DMSCPostQuantumAlgorithm::Dilithium5 => {
                let dilithium = self.dilithium.read().await;
                dilithium.keygen()
            }
            DMSCPostQuantumAlgorithm::Falcon512 => {
                let falcon = self.falcon.read().await;
                falcon.keygen()
            }
        }
    }

    /// Sign data
    pub async fn sign(&self, secret_key: &[u8], data: &[u8]) -> DMSCResult<Vec<u8>> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let alg = self.algorithm.read().await;
        match *alg {
            DMSCPostQuantumAlgorithm::Kyber512 | DMSCPostQuantumAlgorithm::Dilithium5 => {
                let dilithium = self.dilithium.read().await;
                dilithium.sign(secret_key, data)
            }
            DMSCPostQuantumAlgorithm::Falcon512 => {
                let falcon = self.falcon.read().await;
                falcon.sign(secret_key, data)
            }
        }
    }

    /// Verify signature
    pub async fn verify(&self, public_key: &[u8], data: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        if !*self.initialized.read().await {
            return Err(DMSCError::InvalidState("Post-quantum crypto not initialized".to_string()));
        }

        let alg = self.algorithm.read().await;
        match *alg {
            DMSCPostQuantumAlgorithm::Kyber512 | DMSCPostQuantumAlgorithm::Dilithium5 => {
                let dilithium = self.dilithium.read().await;
                dilithium.verify(public_key, data, signature)
            }
            DMSCPostQuantumAlgorithm::Falcon512 => {
                let falcon = self.falcon.read().await;
                falcon.verify(public_key, data, signature)
            }
        }
    }

    /// Get algorithm security level
    pub fn security_level(&self) -> u8 {
        match *self.algorithm.read().await {
            DMSCPostQuantumAlgorithm::Kyber512 => 5,
            DMSCPostQuantumAlgorithm::Dilithium5 => 5,
            DMSCPostQuantumAlgorithm::Falcon512 => 4,
        }
    }
}

impl Default for DMSCPostQuantumManager {
    fn default() -> Self {
        Self::new()
    }
}
