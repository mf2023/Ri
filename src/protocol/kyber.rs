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

//! # Kyber KEM
//!
//! This module implements the Kyber Key Encapsulation Mechanism (KEM)
//! using liboqs. Kyber is IND-CCA2 secure and is based on the hardness
//! of the Module-LWE (Learning With Errors over Modules) problem.
//!
//! ## Security Level
//!
//! - **Kyber512**: NIST Level 1 ≈ AES-128
//! - **Kyber768**: NIST Level 3 ≈ AES-192
//! - **Kyber1024**: NIST Level 5 ≈ AES-256
//!
//! ## Usage
//!
//! ```rust
//! use dmsc::protocol::kyber::KyberKEM;
//!
//! let kem = KyberKEM::new();
//! let (public_key, secret_key) = kem.keygen()?;
//! let (ciphertext, shared_secret_1) = kem.encapsulate(&public_key)?;
//! let shared_secret_2 = kem.decapsulate(&ciphertext, &secret_key)?;
//! assert_eq!(shared_secret_1, shared_secret_2);
//! ```

use std::sync::Arc;
use crate::core::{DMSCResult, DMSCError};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct KyberPublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct KyberSecretKey(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct KyberCiphertext(pub Vec<u8>);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct KyberKEM;

impl KyberKEM {
    pub fn new() -> Self {
        Self
    }

    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let (pk, sk) = oqs::kem::Kyber512::keypair();
        Ok((pk, sk))
    }

    pub fn encapsulate(&self, public_key: &[u8]) -> DMSCResult<super::KEMResult> {
        let (ciphertext, shared_secret) = oqs::kem::Kyber512::encapsulate(public_key);
        Ok(super::KEMResult { ciphertext, shared_secret })
    }

    pub fn decapsulate(&self, ciphertext: &[u8], secret_key: &[u8]) -> DMSCResult<Vec<u8>> {
        let shared_secret = oqs::kem::Kyber512::decapsulate(ciphertext, secret_key);
        Ok(shared_secret)
    }
}

impl Default for KyberKEM {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl KyberKEM {
    #[new]
    pub fn new_py() -> Self {
        Self::new()
    }

    pub fn keygen_py(&self) -> Option<(Vec<u8>, Vec<u8>)> {
        self.keygen().ok()
    }

    pub fn encapsulate_py(&self, public_key: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
        self.encapsulate(public_key).ok().map(|r| (r.ciphertext, r.shared_secret))
    }

    pub fn decapsulate_py(&self, ciphertext: &[u8], secret_key: &[u8]) -> Option<Vec<u8>> {
        self.decapsulate(ciphertext, secret_key).ok()
    }
}
