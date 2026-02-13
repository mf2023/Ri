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
//! ## Security Status
//!
//! ⚠️ **IMPORTANT**: This module provides the API structure for post-quantum cryptography.
//! For production use, integrate an audited library:
//!
//! - **liboqs** (Recommended): https://github.com/open-quantum-safe/liboqs
//!   - NIST PQC competition reference implementation
//!   - Actively maintained and audited
//!   - Supports all major platforms including Windows, Linux, macOS
//!
//! - **pqm4**: https://github.com/mupq/pqm4
//!   - For ARM Cortex-M4 microcontrollers
//!
//! ## Integration Example
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! liboqs = "0.7"
//! ```
//!
//! Then enable the protocol feature:
//! ```bash
//! cargo build --features protocol
//! ```
//!
//! ## API Structure
//!
//! ```rust,ignore
//! use dmsc::protocol::post_quantum::{KyberKEM, DilithiumSigner};
//!
//! // Kyber key encapsulation (requires liboqs integration)
//! let kem = KyberKEM::new();
//! let (public_key, secret_key) = kem.keygen()?;
//! let (ciphertext, shared_secret_1) = kem.encapsulate(&public_key)?;
//! let shared_secret_2 = kem.decapsulate(&ciphertext, &secret_key)?;
//!
//! // Dilithium signing (requires liboqs integration)
//! let signer = DilithiumSigner::new();
//! let (pk, sk) = signer.keygen()?;
//! let message = b"Hello, Post-Quantum World!";
//! let signature = signer.sign(&sk, message)?;
//! assert!(signer.verify(&pk, message, &signature));
//! ```

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use rand::RngCore;
use subtle::ConstantTimeEq;
use crate::core::{DMSCResult, DMSCError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub mod kyber;
pub mod dilithium;
pub mod falcon;

pub use kyber::{KyberKEM, KyberPublicKey, KyberSecretKey, KyberCiphertext};
pub use dilithium::{DilithiumSigner, DilithiumPublicKey, DilithiumSecretKey, DilithiumSignature};
pub use falcon::{FalconSigner, FalconPublicKey, FalconSecretKey, FalconSignature};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCPostQuantumAlgorithm {
    Kyber512,
    Kyber768,
    Kyber1024,
    Dilithium2,
    Dilithium3,
    Dilithium5,
    Falcon512,
    Falcon1024,
}

pub struct KEMResult {
    pub ciphertext: Vec<u8>,
    pub shared_secret: Vec<u8>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCPostQuantumManager {
    algorithm: Arc<RwLock<DMSCPostQuantumAlgorithm>>,
    initialized_at: Arc<RwLock<Instant>>,
    initialized: Arc<RwLock<bool>>,
}

impl DMSCPostQuantumManager {
    pub fn new() -> Self {
        Self {
            algorithm: Arc::new(RwLock::new(DMSCPostQuantumAlgorithm::Kyber512)),
            initialized_at: Arc::new(RwLock::new(Instant::now())),
            initialized: Arc::new(RwLock::new(false)),
        }
    }

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

    pub async fn algorithm(&self) -> DMSCPostQuantumAlgorithm {
        *self.algorithm.read().await
    }

    pub fn security_level(&self) -> u8 {
        match *self.algorithm.read() {
            DMSCPostQuantumAlgorithm::Kyber512 => 1,
            DMSCPostQuantumAlgorithm::Kyber768 => 3,
            DMSCPostQuantumAlgorithm::Kyber1024 => 5,
            DMSCPostQuantumAlgorithm::Dilithium2 => 1,
            DMSCPostQuantumAlgorithm::Dilithium3 => 3,
            DMSCPostQuantumAlgorithm::Dilithium5 => 5,
            DMSCPostQuantumAlgorithm::Falcon512 => 1,
            DMSCPostQuantumAlgorithm::Falcon1024 => 5,
        }
    }
}

impl Default for DMSCPostQuantumManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCPostQuantumManager {
    #[new]
    pub fn new_py() -> Self {
        Self::new()
    }

    pub fn initialize_sync(&mut self, algorithm: DMSCPostQuantumAlgorithm) -> bool {
        if let Ok(mut guard) = self.initialized.try_write() {
            *guard = true;
            if let Ok(mut algo_guard) = self.algorithm.try_write() {
                *algo_guard = algorithm;
                return true;
            }
        }
        false
    }

    pub fn get_stats(&self) -> String {
        format!("Post-Quantum Manager: API structure ready. Integrate liboqs for production use.")
    }
}
