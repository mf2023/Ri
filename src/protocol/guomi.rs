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

use std::sync::Arc;
use ring::{digest, rand, signature};
use ring::rand::SecureRandom;
use ring::signature::EcdsaKeyPair;
use crate::core::{DMSCResult, DMSCError};

pub mod sm2;
pub mod sm3;

pub use sm2::{SM2Signer, SM2PublicKey, SM2SecretKey, SM2Signature};
pub use sm3::{SM3, SM3Digest};

const SM2_P: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
];

const SM2_A: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc,
];

const SM2_B: [u8; 32] = [
    0x28, 0xe9, 0xfa, 0x9e, 0x9d, 0x9f, 0x5e, 0x34,
    0x4d, 0x5a, 0x28, 0xe3, 0x4b, 0xc3, 0x1e, 0x9c,
    0xb6, 0xe0, 0xff, 0x4a, 0xda, 0x03, 0xfc, 0x30,
    0x39, 0xa1, 0x07, 0x37, 0xf8, 0x26, 0x96, 0x1a,
];

const SM2_GX: [u8; 32] = [
    0x6b, 0x17, 0xd1, 0xf2, 0xe1, 0x2c, 0x42, 0x47,
    0xf8, 0xbc, 0xe6, 0xe5, 0x63, 0xa4, 0x40, 0xf2,
    0x77, 0x03, 0x7d, 0x81, 0x2d, 0xeb, 0x33, 0xa0,
    0xf4, 0xa1, 0x39, 0x45, 0xd8, 0x98, 0xc2, 0x96,
];

const SM2_GY: [u8; 32] = [
    0x4f, 0xe3, 0x42, 0xc2, 0xea, 0x1b, 0x69, 0xc8,
    0x04, 0xe9, 0xfa, 0x9e, 0x7d, 0x48, 0x30, 0xc4,
    0x1c, 0x97, 0x7a, 0xcf, 0x72, 0x15, 0x6f, 0xe5,
    0xea, 0x59, 0x0a, 0xaf, 0xd2, 0x3e, 0x0b, 0x64,
];

const SM2_N: [u8; 32] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe,
    0xba, 0xae, 0xdc, 0xe6, 0xaf, 0x48, 0xa0, 0x3b,
    0xbf, 0xd2, 0x5e, 0x8c, 0xd0, 0x36, 0x41, 0x41,
];

const SM3_IV: [u32; 8] = [
    0x7380166F, 0x4914B2B9, 0x17DB4C8C, 0x27F83DDE,
    0xA0C2FAE1, 0xF0C8C0B5, 0x67D91940, 0xFAE3B82C,
];

const SM3_TJ1: u32 = 0x79CC4519;
const SM3_TJ2: u32 = 0x7A879D8A;

const SM3_ROL: fn(u32, usize) -> u32 = |x, n| x.rotate_left(n as u32);

fn sm3_ff_j(x: u32, y: u32, z: u32, j: usize) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | (x & z) | (y & z)
    }
}

fn sm3_gg_j(x: u32, y: u32, z: u32, j: usize) -> u32 {
    if j < 16 {
        x ^ y ^ z
    } else {
        (x & y) | ((!x) & z)
    }
}

fn sm3_p0(x: u32) -> u32 {
    x ^ SM3_ROL(x, 9) ^ SM3_ROL(x, 17)
}

fn sm3_p1(x: u32) -> u32 {
    x ^ SM3_ROL(x, 15) ^ SM3_ROL(x, 23)
}

fn sm3_expand_b0(b: &[u8; 64], j: usize) -> u32 {
    u32::from_be_bytes([b[j * 4], b[j * 4 + 1], b[j * 4 + 2], b[j * 4 + 3]])
}

pub struct SM3 {
    state: [u32; 8],
    length: u128,
    buffer: Vec<u8>,
}

impl SM3 {
    pub fn new() -> Self {
        SM3 {
            state: SM3_IV,
            length: 0,
            buffer: Vec::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.length += (data.len() * 8) as u128;
        self.buffer.extend_from_slice(data);
        while self.buffer.len() >= 64 {
            let block = self.buffer[..64].try_into().unwrap();
            self.buffer.drain(..64);
            self.compress(block);
        }
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        let mut padded = self.buffer.clone();

        padded.push(0x80);

        while (padded.len() + 8) % 64 != 0 {
            padded.push(0x00);
        }

        let bit_length_bits = self.length.to_be_bytes();
        padded.extend_from_slice(&bit_length_bits);

        self.buffer.clear();
        for chunk in padded.chunks(64) {
            let block = chunk.try_into().unwrap();
            self.compress(block);
        }

        let mut result = [0u8; 32];
        for (i, &word) in self.state.iter().enumerate() {
            result[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
        }
        result
    }

    fn compress(&mut self, block: [u8; 64]) {
        let mut w = [0u32; 68];
        let mut w1 = [0u32; 64];

        for j in 0..16 {
            w[j] = sm3_expand_b0(&block, j);
        }
        for j in 16..68 {
            w[j] = sm3_p1(
                w[j - 16] ^ w[j - 9] ^ SM3_ROL(w[j - 3], 15)
            ) ^ SM3_ROL(w[j - 13], 7) ^ w[j - 6];
        }

        for j in 0..64 {
            w1[j] = w[j] ^ w[j + 4];
        }

        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];
        let mut e = self.state[4];
        let mut f = self.state[5];
        let mut g = self.state[6];
        let mut h = self.state[7];

        for j in 0..64 {
            let tj = if j < 16 { SM3_TJ1 } else { SM3_TJ2 };
            let ff = sm3_ff_j(a, b, c, j);
            let gg = sm3_gg_j(e, f, g, j);

            let ss1 = SM3_ROL(a.wrapping_add(e).wrapping_add(tj), 7);
            let ss2 = ss1 ^ SM3_ROL(a, 12);

            let tt1 = ff.wrapping_add(e)
                .wrapping_add(ss2)
                .wrapping_add(w[j])
                .wrapping_add(w1[j]);
            let tt2 = gg.wrapping_add(h)
                .wrapping_add(ss1)
                .wrapping_add(w[j])
                .wrapping_add(w1[j]);

            d = c;
            c = SM3_ROL(b, 9);
            b = a;
            a = tt1;
            h = g;
            g = SM3_ROL(f, 19);
            f = e;
            e = sm3_p0(tt2);
        }

        self.state[0] ^= a;
        self.state[1] ^= b;
        self.state[2] ^= c;
        self.state[3] ^= d;
        self.state[4] ^= e;
        self.state[5] ^= f;
        self.state[6] ^= g;
        self.state[7] ^= h;
    }
}

impl Default for SM3 {
    fn default() -> Self {
        Self::new()
    }
}

impl digest::Digest for SM3 {
    fn update(&mut self, data: &[u8]) {
        Self::update(self, data);
    }

    fn finalize(self) -> digest::DigestBytes {
        let mut result = [0u8; 32];
        let mut hasher = self;
        let output = hasher.finalize();
        result.copy_from_slice(&output);
        digest::DigestBytes::from(&result[..])
    }

    fn digest_size() -> usize {
        32
    }
}

pub struct SM3Digest {
    hasher: SM3,
}

impl SM3Digest {
    pub fn new() -> Self {
        Self {
            hasher: SM3::new(),
        }
    }

    pub fn finalize(&mut self) -> [u8; 32] {
        self.hasher.finalize()
    }
}

impl digest::Digest for SM3Digest {
    fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    fn finalize(mut self) -> digest::DigestBytes {
        let result = self.hasher.finalize();
        digest::DigestBytes::from(&result[..])
    }

    fn digest_size() -> usize {
        32
    }
}

impl Default for SM3Digest {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SM2Signer {
    rng: Arc<dyn SecureRandom>,
}

impl SM2Signer {
    pub fn new() -> DMSCResult<Self> {
        Ok(Self {
            rng: Arc::new(rand::SystemRandom::new()),
        })
    }

    pub fn keygen(&self) -> DMSCResult<(SM2PublicKey, SM2SecretKey)> {
        let private_key = self.generate_private_key()?;
        let public_key = self.derive_public_key(&private_key)?;

        let mut sk_bytes = [0u8; 32];
        sk_bytes.copy_from_slice(&private_key[..32]);

        let mut pk_bytes = Vec::with_capacity(65);
        pk_bytes.push(0x04);
        pk_bytes.extend_from_slice(&public_key[..32]);
        pk_bytes.extend_from_slice(&public_key[32..64]);

        Ok((SM2PublicKey(pk_bytes), SM2SecretKey(sk_bytes)))
    }

    pub fn sign(&self, sk: &SM2SecretKey, data: &[u8]) -> DMSCResult<SM2Signature> {
        let private_key_bytes = &sk.0[..];

        let mut za = [0u8; 32];
        self.compute_za(&[], &mut za)?;

        let mut message = Vec::with_capacity(32 + data.len());
        message.extend_from_slice(&za);
        message.extend_from_slice(data);

        let message_hash = digest::digest(&digest::SHA256, &message);
        let e = self.bits_to_int(message_hash.as_ref())?;

        let d = self.bytes_to_int(private_key_bytes)?;
        let (x, y) = self.bytes_to_point(&self.derive_public_key(private_key_bytes)?)?;

        loop {
            let k = self.generate_private_key()?;
            let k_int = self.bytes_to_int(&k)?;

            let x1_int = self.mul_base_point(&k_int)?.0;

            let r = (e + x1_int) % self.get_n()?;
            if r == 0 || r + self.get_n()? == 0 {
                continue;
            }

            let s = self.mod_inverse(
                &(1 + d),
                &self.get_n()?,
            )? * (k_int - r * d % self.get_n()? % self.get_n()?) % self.get_n()?;

            if s == 0 {
                continue;
            }

            let mut signature_bytes = Vec::with_capacity(64);
            let r_bytes = self.int_to_bytes(r);
            let s_bytes = self.int_to_bytes(s);
            signature_bytes.extend_from_slice(&r_bytes[..32]);
            signature_bytes.extend_from_slice(&s_bytes[..32]);

            return Ok(SM2Signature(signature_bytes));
        }
    }

    pub fn verify(&self, pk: &SM2PublicKey, data: &[u8], sig: &SM2Signature) -> DMSCResult<bool> {
        if sig.0.len() != 64 {
            return Ok(false);
        }

        let r = self.bytes_to_int(&sig.0[..32])?;
        let s = self.bytes_to_int(&sig.0[32..])?;

        let n = self.get_n()?;
        if r < 1 || r >= n || s < 1 || s >= n {
            return Ok(false);
        }

        let mut za = [0u8; 32];
        self.compute_za(&[], &mut za)?;

        let mut message = Vec::with_capacity(32 + data.len());
        message.extend_from_slice(&za);
        message.extend_from_slice(data);

        let message_hash = digest::digest(&digest::SHA256, &message);
        let e = self.bits_to_int(message_hash.as_ref())?;

        let t = (r + s) % n;
        if t == 0 {
            return Ok(false);
        }

        let (x1, y1) = self.mul_point(&self.bytes_to_point(&pk.0[1..])?, &t)?;

        let mut x1_bytes = [0u8; 32];
        let x1_int = self.bytes_to_int(&self.int_to_bytes(x1))?;
        x1_bytes.copy_from_slice(&self.int_to_bytes(x1_int)[..32]);

        let point_r = self.add_points(
            &(x1, y1),
            &(e, self.bytes_to_int(&self.int_to_bytes(self.mul_base_point(&s)?.1)?)?),
        )?;

        let r_computed = (e + point_r.0) % n;

        Ok(r_computed == r)
    }

    fn generate_private_key(&self) -> DMSCResult<Vec<u8>> {
        let mut private_key = vec![0u8; 32];
        self.rng.fill(&mut private_key)
            .map_err(|e| DMSCError::CryptoError(format!("Failed to generate private key: {}", e)))?;
        Ok(private_key)
    }

    fn derive_public_key(&self, private_key: &[u8]) -> DMSCResult<Vec<u8>> {
        let d = self.bytes_to_int(private_key)?;
        let (x, y) = self.mul_base_point(&d)?;

        let mut public_key = vec![0u8; 64];
        let x_bytes = self.int_to_bytes(x);
        let y_bytes = self.int_to_bytes(y);
        public_key[..32].copy_from_slice(&x_bytes[..32]);
        public_key[32..].copy_from_slice(&y_bytes[..32]);

        Ok(public_key)
    }

    fn compute_za(&self, identity: &[u8], za: &mut [u8; 32]) -> DMSCResult<()> {
        let mut input = Vec::new();

        let id_bytes = identity.as_bytes();
        let id_len = id_bytes.len();
        let id_len_bytes = (id_len as u16).to_be_bytes();
        input.extend_from_slice(&id_len_bytes);
        input.extend_from_slice(id_bytes);

        input.extend_from_slice(&SM2_P);
        input.extend_from_slice(&SM2_A);
        input.extend_from_slice(&SM2_B);
        input.extend_from_slice(&SM2_GX);
        input.extend_from_slice(&SM2_GY);

        let za_input_hash = digest::digest(&digest::SHA256, &input);

        input.clear();
        input.extend_from_slice(&za_input_hash);
        input.extend_from_slice(&SM2_P);
        input.extend_from_slice(&SM2_A);
        input.extend_from_slice(&SM2_B);
        input.extend_from_slice(&SM2_GX);
        input.extend_from_slice(&SM2_GY);

        let mut sm3_hasher = SM3::new();
        sm3_hasher.update(&input);
        let za_result = sm3_hasher.finalize();
        za.copy_from_slice(&za_result);

        Ok(())
    }

    fn mul_base_point(&self, k: &num_bigint::BigUint) -> DMSCResult<(num_bigint::BigUint, num_bigint::BigUint)> {
        let gx = self.bytes_to_int(&SM2_GX)?;
        let gy = self.bytes_to_int(&SM2_GY)?;
        self.mul_point(&(gx, gy), k)
    }

    fn mul_point(&self, point: &(num_bigint::BigUint, num_bigint::BigUint), k: &num_bigint::BigUint) -> DMSCResult<(num_bigint::BigUint, num_bigint::BigUint)> {
        let n = self.get_n()?;
        let p = self.bytes_to_int(&SM2_P)?;
        let a = self.bytes_to_int(&SM2_A)?;

        let mut result = None;
        let mut current = point.clone();
        let mut k = k.clone();

        while k > num_bigint::BigUint::from(0u32) {
            if k.clone() & num_bigint::BigUint::from(1u32) == num_bigint::BigUint::from(1u32) {
                result = Some(match result {
                    None => current.clone(),
                    Some(r) => self.add_points(&r, &current)?,
                });
            }
            current = self.double_point(&current, &p, &a)?;
            k >>= 1;
        }

        match result {
            Some(r) => Ok(r),
            None => Err(DMSCError::CryptoError("Point multiplication failed".to_string())),
        }
    }

    fn add_points(&self, p1: &(num_bigint::BigUint, num_bigint::BigUint), p2: &(num_bigint::BigUint, num_bigint::BigUint)) -> DMSCResult<(num_bigint::BigUint, num_bigint::BigUint)> {
        let p = self.bytes_to_int(&SM2_P)?;
        let a = self.bytes_to_int(&SM2_A)?;

        if p1.0 == p2.0 && p1.1 == p2.1 {
            return self.double_point(p1, &p, &a);
        }

        if p1.0 == p2.0 {
            return Err(DMSCError::CryptoError("Point at infinity".to_string()));
        }

        let dx = (p2.0.clone() + p.clone() - p1.0.clone()) % &p;
        let dy = (p2.1.clone() + p.clone() - p1.1.clone()) % &p;

        let slope = self.mod_inverse(&dx, &p)? * dy % &p;
        let x3 = (slope.clone() * slope.clone() + p.clone() - a - p1.0.clone() - p2.0.clone()) % &p;
        let y3 = (slope * (p1.0.clone() - x3.clone()) + p.clone() - p1.1.clone()) % &p;

        Ok((x3, y3))
    }

    fn double_point(&self, point: &(num_bigint::BigUint, num_bigint::BigUint), p: &num_bigint::BigUint, a: &num_bigint::BigUint) -> DMSCResult<(num_bigint::BigUint, num_bigint::BigUint)> {
        if point.1 == num_bigint::BigUint::from(0u32) {
            return Err(DMSCError::CryptoError("Point at infinity".to_string()));
        }

        let three_x2 = (point.0.clone() * point.0.clone() * num_bigint::BigUint::from(3u32)) % p;
        let two_y = (point.1.clone() * num_bigint::BigUint::from(2u32)) % p;

        let slope = self.mod_inverse(&two_y, p)? * (three_x2 + a) % p;
        let x3 = (slope.clone() * slope.clone() + p.clone() - num_bigint::BigUint::from(2u32) * point.0.clone()) % p;
        let y3 = (slope * (point.0.clone() - x3.clone()) + p.clone() - point.1.clone()) % p;

        Ok((x3, y3))
    }

    fn mod_inverse(a: &num_bigint::BigUint, m: &num_bigint::BigUint) -> DMSCResult<num_bigint::BigUint> {
        let extended_gcd = |a: &num_bigint::BigUint, b: &num_bigint::BigUint| -> (num_bigint::BigInt, num_bigint::BigInt, num_bigint::BigInt) {
            if b == &num_bigint::BigUint::from(0u32) {
                return (num_bigint::BigInt::from(a.clone()), num_bigint::BigInt::from(1), num_bigint::BigInt::from(0));
            }
            let (gcd, x1, y1) = extended_gcd(b, &(a % b));
            let x = y1.clone();
            let y = x1 - (a / b) * y1;
            (gcd, x, y)
        };

        let (gcd, x, _) = extended_gcd(a, m);
        if gcd != num_bigint::BigInt::from(1) {
            return Err(DMSCError::CryptoError("No modular inverse exists".to_string()));
        }

        let result = (x % m + m) % m;
        Ok(result.try_into().unwrap())
    }

    fn bytes_to_int(&self, bytes: &[u8]) -> DMSCResult<num_bigint::BigUint> {
        Ok(num_bigint::BigUint::from_bytes_be(bytes))
    }

    fn bytes_to_point(&self, bytes: &[u8]) -> DMSCResult<(num_bigint::BigUint, num_bigint::BigUint)> {
        if bytes.len() != 64 {
            return Err(DMSCError::CryptoError("Invalid point size".to_string()));
        }
        let x = self.bytes_to_int(&bytes[..32])?;
        let y = self.bytes_to_int(&bytes[32..])?;
        Ok((x, y))
    }

    fn bits_to_int(&self, bits: &[u8]) -> DMSCResult<num_bigint::BigUint> {
        let n = self.get_n()?;
        let mut truncated = Vec::from(bits);
        while truncated.len() * 8 > 256 {
            truncated.pop();
        }
        let val = self.bytes_to_int(&truncated)?;
        Ok(val % n)
    }

    fn int_to_bytes(&self, val: num_bigint::BigUint) -> Vec<u8> {
        val.to_bytes_be()
    }

    fn get_n(&self) -> DMSCResult<num_bigint::BigUint> {
        self.bytes_to_int(&SM2_N)
    }
}

impl Default for SM2Signer {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct SM2PublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct SM2SecretKey(pub [u8; 32]);

#[derive(Debug, Clone)]
pub struct SM2Signature(pub Vec<u8>);

#[cfg(test)]
mod guomi_tests {
    use super::*;

    #[test]
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
    fn test_sm4_encrypt_decrypt() {
        let key = [0u8; 16];
        let plaintext = b"SM4 test message!";

        let ciphertext = DMSCGuomi::sm4_encrypt(&key, plaintext).unwrap();
        let decrypted = DMSCGuomi::sm4_decrypt(&key, &ciphertext).unwrap();

        assert_eq!(&decrypted, plaintext);
    }

    #[test]
    fn test_sm4_different_modes() {
        let key = [0u8; 16];
        let plaintext = b"Test data for SM4";

        // ECB mode
        let ecb = DMSCGuomi::sm4_encrypt_ecb(&key, plaintext).unwrap();
        let decrypted_ecb = DMSCGuomi::sm4_decrypt_ecb(&key, &ecb).unwrap();
        assert_eq!(&decrypted_ecb, plaintext);

        // CBC mode
        let iv = [0u8; 16];
        let cbc = DMSCGuomi::sm4_encrypt_cbc(&key, &iv, plaintext).unwrap();
        let decrypted_cbc = DMSCGuomi::sm4_decrypt_cbc(&key, &iv, &cbc).unwrap();
        assert_eq!(&decrypted_cbc, plaintext);
    }

    #[test]
    fn test_sm2_key_generation() {
        let private_key = DMSCGuomi::sm2_generate_private_key().unwrap();
        assert_eq!(private_key.len(), 32);

        let public_key = DMSCGuomi::sm2_derive_public_key(&private_key).unwrap();
        assert_eq!(public_key.len(), 64);
    }

    #[test]
    fn test_sm2_sign_verify() {
        let private_key = DMSCGuomi::sm2_generate_private_key().unwrap();
        let public_key = DMSCGuomi::sm2_derive_public_key(&private_key).unwrap();
        let signer = DMSCGuomi::sm2_signer(&private_key).unwrap();

        let message = b"Message to sign";
        let signature = signer.sign(message).unwrap();
        assert_eq!(signature.len(), 64);

        let verifier = DMSCGuomi::sm2_verifier(&public_key);
        let is_valid = verifier.verify(message, &signature).unwrap();
        assert!(is_valid);

        // Verify that different message fails
        let wrong_message = b"Different message";
        let is_valid_wrong = verifier.verify(wrong_message, &signature).unwrap();
        assert!(!is_valid_wrong);
    }

    #[test]
    fn test_sm2_sign_different_keys() {
        let private_key1 = DMSCGuomi::sm2_generate_private_key().unwrap();
        let private_key2 = DMSCGuomi::sm2_generate_private_key().unwrap();
        let public_key1 = DMSCGuomi::sm2_derive_public_key(&private_key1).unwrap();

        let signer1 = DMSCGuomi::sm2_signer(&private_key1).unwrap();
        let signature = signer1.sign(b"test").unwrap();

        let verifier1 = DMSCGuomi::sm2_verifier(&public_key1);
        assert!(verifier1.verify(b"test", &signature).unwrap());

        // Signature from different key should not verify
        let signer2 = DMSCGuomi::sm2_signer(&private_key2);
        let signature2 = signer2.sign(b"test").unwrap();
        assert!(!verifier1.verify(b"test", &signature2).unwrap());
    }

    #[test]
    fn test_sm2_signature_format() {
        let private_key = DMSCGuomi::sm2_generate_private_key().unwrap();
        let signer = DMSCGuomi::sm2_signer(&private_key).unwrap();

        let signature = signer.sign(b"test").unwrap();

        // SM2 signature should be 64 bytes (r || s)
        assert_eq!(signature.len(), 64);

        // Both r and s should be non-zero
        let r = &signature[..32];
        let s = &signature[32..];
        assert!(r.iter().any(|&x| x != 0));
        assert!(s.iter().any(|&x| x != 0));
    }
}
