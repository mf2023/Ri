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

//! # Falcon Signature
//!
//! This module implements the Falcon digital signature algorithm
//! using liboqs. Falcon is EUF-CMA secure and is based on the hardness
//! of the Shortest Integer Solution (SIS) problem over NTRU lattices.
//!
//! ## Security Level
//!
//! - **Falcon512**: NIST Level 1 ≈ AES-128 (compact signatures)
//! - **Falcon1024**: NIST Level 5 ≈ AES-256 (compact signatures)
//!
//! ## Advantages
//!
//! Falcon provides the smallest signature sizes among NIST PQC finalists,
//! making it ideal for bandwidth-constrained scenarios.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use ri::protocol::falcon::FalconSigner;
//!
//! let signer = FalconSigner::new();
//! let (public_key, secret_key) = signer.keygen()?;
//! let message = b"Hello, Post-Quantum World!";
//! let signature = signer.sign(&secret_key, message)?;
//! assert!(signer.verify(&public_key, message, &signature)?);
//! ```

use std::sync::Arc;
use crate::core::{RiResult, RiError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct FalconPublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct FalconSecretKey(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct FalconSignature(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct FalconSigner {
    algorithm: Arc<std::sync::RwLock<FalconAlgorithm>>,
}

#[derive(Debug, Clone, Copy)]
enum FalconAlgorithm {
    Falcon512,
    Falcon1024,
}

impl FalconSigner {
    pub fn new() -> Self {
        Self {
            algorithm: Arc::new(std::sync::RwLock::new(FalconAlgorithm::Falcon512)),
        }
    }

    pub fn with_algorithm(algorithm: super::RiPostQuantumAlgorithm) -> Self {
        let algo = match algorithm {
            super::RiPostQuantumAlgorithm::Falcon512 => FalconAlgorithm::Falcon512,
            super::RiPostQuantumAlgorithm::Falcon1024 => FalconAlgorithm::Falcon1024,
            _ => FalconAlgorithm::Falcon512,
        };
        Self {
            algorithm: Arc::new(std::sync::RwLock::new(algo)),
        }
    }

    #[cfg(feature = "oqs")]
    pub fn keygen(&self) -> RiResult<(Vec<u8>, Vec<u8>)> {
        use oqs::sig::Sig;

        let algo = *self.algorithm.read().map_err(|e| 
            RiError::InvalidState(format!("Lock error: {}", e))
        )?;
        let sig = match algo {
            FalconAlgorithm::Falcon512 => Sig::new(oqs::sig::Algorithm::Falcon512),
            FalconAlgorithm::Falcon1024 => Sig::new(oqs::sig::Algorithm::Falcon1024),
        }.map_err(|e| RiError::Other(format!("Failed to initialize Falcon: {:?}", e)))?;

        let (pk, sk) = sig.keypair();
        Ok((pk.into_vec(), sk.into_vec()))
    }

    #[cfg(not(feature = "oqs"))]
    pub fn keygen(&self) -> RiResult<(Vec<u8>, Vec<u8>)> {
        Err(RiError::Other(
            "Post-quantum cryptography requires the 'oqs' feature. \
             Enable with: cargo build --features oqs".to_string()
        ))
    }

    #[cfg(feature = "oqs")]
    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> RiResult<Vec<u8>> {
        use oqs::sig::Sig;

        let algo = *self.algorithm.read().map_err(|e| 
            RiError::InvalidState(format!("Lock error: {}", e))
        )?;
        let sig = match algo {
            FalconAlgorithm::Falcon512 => Sig::new(oqs::sig::Algorithm::Falcon512),
            FalconAlgorithm::Falcon1024 => Sig::new(oqs::sig::Algorithm::Falcon1024),
        }.map_err(|e| RiError::Other(format!("Failed to initialize Falcon: {:?}", e)))?;

        let sk = sig.secret_key_from_bytes(secret_key)
            .ok_or_else(|| RiError::Other("Invalid secret key".to_string()))?;
        let signature = sig.sign(message, &sk);
        Ok(signature.into_vec())
    }

    #[cfg(not(feature = "oqs"))]
    pub fn sign(&self, _secret_key: &[u8], _message: &[u8]) -> RiResult<Vec<u8>> {
        Err(RiError::Other(
            "Post-quantum cryptography requires the 'oqs' feature. \
             Enable with: cargo build --features oqs".to_string()
        ))
    }

    #[cfg(feature = "oqs")]
    pub fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> RiResult<bool> {
        use oqs::sig::Sig;

        let algo = *self.algorithm.read().map_err(|e| 
            RiError::InvalidState(format!("Lock error: {}", e))
        )?;
        let sig = match algo {
            FalconAlgorithm::Falcon512 => Sig::new(oqs::sig::Algorithm::Falcon512),
            FalconAlgorithm::Falcon1024 => Sig::new(oqs::sig::Algorithm::Falcon1024),
        }.map_err(|e| RiError::Other(format!("Failed to initialize Falcon: {:?}", e)))?;

        let pk = sig.public_key_from_bytes(public_key)
            .ok_or_else(|| RiError::Other("Invalid public key".to_string()))?;
        let sig_bytes = sig.signature_from_bytes(signature)
            .ok_or_else(|| RiError::Other("Invalid signature".to_string()))?;
        let result = sig.verify(message, &sig_bytes, &pk);
        Ok(result.is_ok())
    }

    #[cfg(not(feature = "oqs"))]
    pub fn verify(&self, _public_key: &[u8], _message: &[u8], _signature: &[u8]) -> RiResult<bool> {
        Err(RiError::Other(
            "Post-quantum cryptography requires the 'oqs' feature. \
             Enable with: cargo build --features oqs".to_string()
        ))
    }
}

impl Default for FalconSigner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl FalconSigner {
    #[new]
    pub fn new_py() -> Self {
        Self::new()
    }

    pub fn keygen_py(&self) -> Option<(Vec<u8>, Vec<u8>)> {
        self.keygen().ok()
    }

    pub fn sign_py(&self, secret_key: &[u8], message: &[u8]) -> Option<Vec<u8>> {
        self.sign(secret_key, message).ok()
    }

    pub fn verify_py(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> bool {
        self.verify(public_key, message, signature).unwrap_or(false)
    }
}
