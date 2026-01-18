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
//! ```rust
//! use dmsc::protocol::falcon::FalconSigner;
//!
//! let signer = FalconSigner::new();
//! let (public_key, secret_key) = signer.keygen()?;
//! let message = b"Hello, Post-Quantum World!";
//! let signature = signer.sign(&secret_key, message)?;
//! assert!(signer.verify(&public_key, message, &signature)?);
//! ```

use std::sync::Arc;
use crate::core::{DMSCResult, DMSCError};

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
pub struct FalconSigner;

impl FalconSigner {
    pub fn new() -> Self {
        Self
    }

    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let (pk, sk) = oqs::sig::Falcon512::keypair();
        Ok((pk, sk))
    }

    pub fn sign(&self, secret_key: &[u8], message: &[u8]) -> DMSCResult<Vec<u8>> {
        let signature = oqs::sig::Falcon512::sign(secret_key, message);
        Ok(signature)
    }

    pub fn verify(&self, public_key: &[u8], message: &[u8], signature: &[u8]) -> DMSCResult<bool> {
        let result = oqs::sig::Falcon512::verify(public_key, message, signature);
        Ok(result)
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
