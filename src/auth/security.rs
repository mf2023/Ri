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
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # Security Utilities Module
//!
//! This module provides security-related utilities for DMSC, including:
//! - Configuration encryption and decryption
//! - Sensitive data protection
//! - Cryptographic utilities

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose::STANDARD, Engine as _};
use generic_array::GenericArray;
use rand::RngCore;
use ring::hmac;
use std::env;

const ENCRYPTION_KEY_ENV: &str = "DMSC_ENCRYPTION_KEY";
const HMAC_KEY_ENV: &str = "DMSC_HMAC_KEY";
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

pub struct DMSCSecurityManager;

impl DMSCSecurityManager {
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

    pub fn hmac_sign(data: &str) -> String {
        let key = load_hmac_key();
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, &key);
        let signature = hmac::sign(&signing_key, data.as_bytes());
        hex::encode(signature)
    }

    pub fn hmac_verify(data: &str, signature: &str) -> bool {
        let expected = hex::decode(signature).ok().unwrap_or_default();
        let key = load_hmac_key();
        let signing_key = hmac::Key::new(hmac::HMAC_SHA256, &key);
        hmac::verify(&signing_key, data.as_bytes(), &expected).is_ok()
    }

    pub fn generate_encryption_key() -> String {
        let mut key = vec![0u8; DEFAULT_KEY_LENGTH];
        rand::thread_rng().fill_bytes(&mut key);
        hex::encode(key)
    }

    pub fn generate_hmac_key() -> String {
        let mut key = vec![0u8; DEFAULT_KEY_LENGTH];
        rand::thread_rng().fill_bytes(&mut key);
        hex::encode(key)
    }
}
