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

#![allow(non_snake_case)]

use std::sync::Arc;
use rand::RngCore;
use subtle::ConstantTimeEq;
use crate::core::{DMSCResult, DMSCError};
use crate::crypto::DMSCCrypto;

const DILITHIUM_Q: i32 = 8380417;
const DILITHIUM_N: usize = 256;
const DILITHIUM_K: usize = 4;
const DILITHIUM_L: usize = 4;
const DILITHIUM_ETA: usize = 2;
const DILITHIUM_GAMMA1: i32 = 1 << 17;
const DILITHIUM_GAMMA2: i32 = (DILITHIUM_Q as i32 - 1) / 32;
const DILITHIUM_ALPHA: i32 = DILITHIUM_GAMMA2 * 2;
const DILITHIUM_BETA: i32 = 78;
const DILITHIUM_OMEGA: usize = 80;
const DILITHIUM_POLYBYTES: usize = 640;
const DILITHIUM_POLYVECBYTES: usize = DILITHIUM_K * DILITHIUM_POLYBYTES;

#[derive(Debug, Clone)]
pub struct DilithiumPublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct DilithiumSecretKey(Vec<u8>);

#[derive(Debug, Clone)]
pub struct DilithiumSignature(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct DilithiumSigner {
    crypto: Arc<DMSCCrypto>,
    random: Arc<rand::rngs::OsRng>,
}

impl DilithiumSigner {
    pub fn new() -> Self {
        Self {
            crypto: Arc::new(DMSCCrypto::new()),
            random: Arc::new(rand::rngs::OsRng),
        }
    }

    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let mut rho = [0u8; 32];
        let mut seed = [0u8; 32];
        self.random.fill_bytes(&mut rho);
        self.random.fill_bytes(&mut seed);

        let mut seed_ext = [0u8; 128];
        self.crypto.hash_sha3_512(&seed, &mut seed_ext);

        let key = &seed_ext[64..];
        let mut mat = Vec::with_capacity(DILITHIUM_K * DILITHIUM_L);
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_L {
                let mut seed_ij = [0u8; 33];
                seed_ij[0] = (i as u8) | ((j as u8) << 4);
                seed_ij[1..].copy_from_slice(&rho[..32]);
                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&seed_ij, &mut expanded);
                mat.push(self.expand_a(&expanded));
            }
        }

        let mut s1 = Vec::with_capacity(DILITHIUM_L);
        for i in 0..DILITHIUM_L {
            s1.push(self.sample_polynomial(&key[i * 32..(i + 1) * 32], DILITHIUM_ETA));
        }

        let mut s2 = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            s2.push(self.sample_polynomial(&key[32 + i * 32..32 + (i + 1) * 32], DILITHIUM_ETA));
        }

        let mut t = self.mat_vec_mul(&mat, &s1);
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                t[i][j] += s2[i][j];
            }
        }
        self.polyvec_reduce(&mut t);
        let mut t1 = vec![vec![0i32; DILITHIUM_N]; DILITHIUM_K];
        self.polyvec_power2round(&mut t, &mut t1);

        let mut pk = Vec::with_capacity(32 + DILITHIUM_POLYVECBYTES);
        pk.extend_from_slice(&rho);
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                let val = t[i][j] >> 2;
                pk[32 + i * DILITHIUM_N / 8 + j / 8] |= ((val as u8) & 0xFF) << ((j % 8) * 8);
            }
        }

        let mut sk = Vec::with_capacity(2 * 32 + 32 + DILITHIUM_L * DILITHIUM_POLYBYTES + DILITHIUM_K * DILITHIUM_POLYBYTES);
        sk.extend_from_slice(&seed);
        sk.extend_from_slice(&rho);
        sk.extend_from_slice(&key[64..96]);

        for i in 0..DILITHIUM_L {
            for j in 0..DILITHIUM_N {
                let mut bytes = [0u8; 4];
                let val = ((s1[i][j] as i32) + 0x8000) as u16;
                bytes[0] = (val >> 0) as u8;
                bytes[1] = (val >> 8) as u8;
                sk.extend_from_slice(&bytes);
            }
        }

        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                let mut bytes = [0u8; 4];
                let val = ((s2[i][j] as i32) + 0x8000) as u16;
                bytes[0] = (val >> 0) as u8;
                bytes[1] = (val >> 8) as u8;
                sk.extend_from_slice(&bytes);
            }
        }

        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                let val = (t[i][j] as i32) >> 2;
                let mut bytes = [0u8; 4];
                bytes[0] = (val >> 0) as u8;
                bytes[1] = (val >> 8) as u8;
                sk.extend_from_slice(&bytes);
            }
        }

        Ok((pk, sk))
    }

    pub fn sign(&self, sk: &[u8], msg: &[u8]) -> DMSCResult<Vec<u8>> {
        if sk.len() < 2 * 32 + 32 {
            return Err(DMSCError::InvalidInput("Invalid secret key size".to_string()));
        }

        let seed = &sk[..32];
        let rho = &sk[32..64];
        let key = &sk[64..96];
        let s1_offset = 96;
        let s2_offset = s1_offset + DILITHIUM_L * DILITHIUM_N * 4;
        let t1_offset = s2_offset + DILITHIUM_K * DILITHIUM_N * 4;

        let mut s1 = Vec::with_capacity(DILITHIUM_L);
        for i in 0..DILITHIUM_L {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                let offset = s1_offset + i * DILITHIUM_N * 4 + j * 4;
                let v0 = sk[offset] as i32;
                let v1 = sk[offset + 1] as i32;
                poly[j] = (v0 | (v1 << 8)) as i16 as i32;
            }
            s1.push(poly);
        }

        let mut s2 = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                let offset = s2_offset + i * DILITHIUM_N * 4 + j * 4;
                let v0 = sk[offset] as i32;
                let v1 = sk[offset + 1] as i32;
                poly[j] = (v0 | (v1 << 8)) as i16 as i32;
            }
            s2.push(poly);
        }

        let mut t1 = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                let offset = t1_offset + i * DILITHIUM_N * 4 + j * 4;
                let v0 = sk[offset] as i32;
                let v1 = sk[offset + 1] as i32;
                poly[j] = (v0 | (v1 << 8)) as i16 as i32;
            }
            t1.push(poly);
        }

        let mut mat = Vec::with_capacity(DILITHIUM_K * DILITHIUM_L);
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_L {
                let mut seed_ij = [0u8; 33];
                seed_ij[0] = (i as u8) | ((j as u8) << 4);
                seed_ij[1..].copy_from_slice(rho);
                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&seed_ij, &mut expanded);
                mat.push(self.expand_a(&expanded));
            }
        }

        let mut nonce = 0u16;
        let mut signature = Vec::new();

        loop {
            let mut mu = [0u8; 64];
            let mut rho_prime = [0u8; 32];
            self.random.fill_bytes(&mut rho_prime);

            let mut msg_hash = [0u8; 64];
            self.crypto.hash_sha3_512(msg, &mut msg_hash);

            let mut mu_input = Vec::with_capacity(32 + 64);
            mu_input.extend_from_slice(&rho_prime);
            mu_input.extend_from_slice(&msg_hash);
            self.crypto.hash_sha3_512(&mu_input, &mut mu);

            let mut w1_input = Vec::with_capacity(64 + 32);
            w1_input.extend_from_slice(&mu);
            w1_input.extend_from_slice(rho_prime);
            let mut w1_bytes = [0u8; 64];
            self.crypto.hash_sha3_512(&w1_input, &mut w1_bytes);

            let mut y = Vec::with_capacity(DILITHIUM_L);
            for i in 0..DILITHIUM_L {
                let mut poly = vec![0i32; DILITHIUM_N];
                let mut seed_i = [0u8; 32];
                seed_i.copy_from_slice(&key[i * 32..(i + 1) * 32]);
                seed_i[0] ^= (nonce >> 0) as u8;
                seed_i[1] ^= (nonce >> 8) as u8;

                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&seed_i, &mut expanded);
                for j in 0..DILITHIUM_N {
                    let val = ((expanded[j * 2] as u16) | ((expanded[j * 2 + 1] as u16) << 8)) as i32;
                    poly[j] = ((val as i64 % (2 * DILITHIUM_GAMMA1 as i64)) as i32) - DILITHIUM_GAMMA1 as i32;
                }
                y.push(poly);
            }
            nonce += 1;

            let mut w = self.mat_vec_mul(&mat, &y);
            self.polyvec_add_inplace(&mut w, &s2);
            self.polyvec_reduce(&mut w);

            let mut w1_vec = self.decompose_vector(&w);
            let mut sign_input = Vec::with_capacity(64 + DILITHIUM_L * DILITHIUM_N * 4);
            sign_input.extend_from_slice(&mu);
            for i in 0..DILITHIUM_L {
                for j in 0..DILITHIUM_N {
                    let mut bytes = [0u8; 4];
                    let val = ((y[i][j] as i64) + DILITHIUM_GAMMA1 as i64) as u64;
                    bytes[0] = (val >> 0) as u8;
                    bytes[1] = (val >> 8) as u8;
                    bytes[2] = (val >> 16) as u8;
                    bytes[3] = (val >> 24) as u8;
                    sign_input.extend_from_slice(&bytes);
                }
            }

            let mut challenge = [0u8; 64];
            self.crypto.hash_sha3_512(&sign_input, &mut challenge);
            let c = self.sample_in_ball(&challenge);

            let mut z = Vec::with_capacity(DILITHIUM_L);
            for i in 0..DILITHIUM_L {
                let mut poly = vec![0i32; DILITHIUM_N];
                for j in 0..DILITHIUM_N {
                    poly[j] = y[i][j] + c * s1[i][j];
                }
                z.push(poly);
            }

            let mut t0 = Vec::with_capacity(DILITHIUM_K);
            for i in 0..DILITHIUM_K {
                let mut poly = vec![0i32; DILITHIUM_N];
                for j in 0..DILITHIUM_N {
                    poly[j] = t1[i][j] & 0x3;
                }
                t0.push(poly);
            }

            let mut w0 = self.polyvec_sub(&w, &w1_vec);
            self.polyvec_reduce(&mut w0);
            self.polyvec_csubq(&mut w0);

            let mut h = self.polyvec_matrix_mul(&mat, &z);
            self.polyvec_sub_inplace(&mut h, &s2);
            self.polyvec_add_inplace(&mut h, &w0);

            let mut valid = true;
            for i in 0..DILITHIUM_K {
                for j in 0..DILITHIUM_N {
                    if h[i][j] < -(DILITHIUM_GAMMA2 as i32) || h[i][j] > DILITHIUM_GAMMA2 as i32 {
                        valid = false;
                        break;
                    }
                }
                if !valid {
                    break;
                }
            }

            if valid {
                signature.extend_from_slice(&rho_prime);
                for i in 0..DILITHIUM_L {
                    for j in 0..DILITHIUM_N {
                        let mut bytes = [0u8; 4];
                        let val = ((z[i][j] as i64 + DILITHIUM_GAMMA1 as i64) as u64);
                        bytes[0] = (val >> 0) as u8;
                        bytes[1] = (val >> 8) as u8;
                        bytes[2] = (val >> 16) as u8;
                        bytes[3] = (val >> 24) as u8;
                        signature.extend_from_slice(&bytes);
                    }
                }
                signature.extend_from_slice(&c);

                return Ok(signature);
            }
        }
    }

    pub fn verify(&self, pk: &[u8], msg: &[u8], sig: &[u8]) -> DMSCResult<bool> {
        if pk.len() < 32 + DILITHIUM_POLYVECBYTES {
            return Err(DMSCError::InvalidInput("Invalid public key size".to_string()));
        }
        if sig.len() < 32 + DILITHIUM_L * DILITHIUM_N * 4 + 32 {
            return Err(DMSCError::InvalidInput("Invalid signature size".to_string()));
        }

        let rho = &pk[..32];
        let t1 = {
            let mut poly_vec = Vec::with_capacity(DILITHIUM_K);
            for i in 0..DILITHIUM_K {
                let mut poly = vec![0i32; DILITHIUM_N];
                for j in 0..DILITHIUM_N {
                    let offset = 32 + i * DILITHIUM_N / 8 + j / 8;
                    let val = ((pk[offset] as u32) >> ((j % 8) * 8)) as u32;
                    poly[j] = (val as i32) << 2;
                }
                poly_vec.push(poly);
            }
            poly_vec
        };

        let rho_prime = &sig[..32];
        let z = {
            let mut poly_vec = Vec::with_capacity(DILITHIUM_L);
            let mut pos = 32;
            for i in 0..DILITHIUM_L {
                let mut poly = vec![0i32; DILITHIUM_N];
                for j in 0..DILITHIUM_N {
                    let offset = pos + i * DILITHIUM_N * 4 + j * 4;
                    let v0 = sig[offset] as i32;
                    let v1 = sig[offset + 1] as i32;
                    let v2 = sig[offset + 2] as i32;
                    let v3 = sig[offset + 3] as i32;
                    let val = ((v0 as u32) | ((v1 as u32) << 8) | ((v2 as u32) << 16) | ((v3 as u32) << 24)) as i32;
                    poly[j] = val as i32 - DILITHIUM_GAMMA1 as i32;
                }
                poly_vec.push(poly);
                pos += DILITHIUM_L * DILITHIUM_N * 4;
            }
            poly_vec
        };
        let c_offset = 32 + DILITHIUM_L * DILITHIUM_N * 4;
        let c = &sig[c_offset..c_offset + 32];

        for i in 0..DILITHIUM_L {
            for j in 0..DILITHIUM_N {
                if z[i][j] < -(DILITHIUM_GAMMA1 as i32) || z[i][j] > DILITHIUM_GAMMA1 as i32 {
                    return Ok(false);
                }
            }
        }

        let mut mu = [0u8; 64];
        let mut msg_hash = [0u8; 64];
        self.crypto.hash_sha3_512(msg, &mut msg_hash);

        let mut mu_input = Vec::with_capacity(32 + 64);
        mu_input.extend_from_slice(rho_prime);
        mu_input.extend_from_slice(&msg_hash);
        self.crypto.hash_sha3_512(&mu_input, &mut mu);

        let mut mat = Vec::with_capacity(DILITHIUM_K * DILITHIUM_L);
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_L {
                let mut seed_ij = [0u8; 33];
                seed_ij[0] = (i as u8) | ((j as u8) << 4);
                seed_ij[1..].copy_from_slice(rho);
                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&seed_ij, &mut expanded);
                mat.push(self.expand_a(&expanded));
            }
        }

        let mut h = self.polyvec_matrix_mul(&mat, &z);
        let mut t0 = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                poly[j] = t1[i][j] & 0x3;
            }
            t0.push(poly);
        }

        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                h[i][j] -= t0[i][j];
            }
        }
        self.polyvec_reduce(&mut h);

        let mut w1_vec = self.decompose_vector(&h);

        let mut sign_input = Vec::with_capacity(64 + DILITHIUM_L * DILITHIUM_N * 4);
        sign_input.extend_from_slice(&mu);
        for i in 0..DILITHIUM_L {
            for j in 0..DILITHIUM_N {
                let mut bytes = [0u8; 4];
                let val = ((z[i][j] as i64 + DILITHIUM_GAMMA1 as i64) as u64);
                bytes[0] = (val >> 0) as u8;
                bytes[1] = (val >> 8) as u8;
                bytes[2] = (val >> 16) as u8;
                bytes[3] = (val >> 24) as u8;
                sign_input.extend_from_slice(&bytes);
            }
        }

        let mut c_prime = [0u8; 64];
        self.crypto.hash_sha3_512(&sign_input, &mut c_prime);

        let result = c.ct_eq(&c_prime);

        let mut valid_w = true;
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                if w1_vec[i][j] != self.decompose(h[i][j]) {
                    valid_w = false;
                    break;
                }
            }
            if !valid_w {
                break;
            }
        }

        Ok(result && valid_w)
    }

    fn expand_a(&self, seed: &[u8]) -> Vec<i32> {
        let mut coeffs = vec![0i32; DILITHIUM_N];
        let mut j = 0usize;
        while j < DILITHIUM_N && j + 3 < seed.len() {
            let d1 = (seed[j] as u16) | ((seed[j + 1] as u16) << 8);
            let d2 = (seed[j + 1] as u16 >> 6) | ((seed[j + 2] as u16) << 2);
            let d3 = (seed[j + 2] as u16 >> 4) | ((seed[j + 3] as u16) << 4);

            for s in 0..4 {
                let t = (d1 >> (2 * s)) as i32 & 0x3;
                if t < 4 {
                    coeffs[j + s] = t - if t >= 2 { 4 } else { 0 };
                }
                let t2 = (d2 >> (2 * s)) as i32 & 0x3;
                if t2 < 4 {
                    coeffs[j + s + 4] = t2 - if t2 >= 2 { 4 } else { 0 };
                }
                let t3 = (d3 >> (2 * s)) as i32 & 0x3;
                if t3 < 4 {
                    coeffs[j + s + 8] = t3 - if t3 >= 2 { 4 } else { 0 };
                }
            }
            j += 12;
        }
        coeffs
    }

    fn sample_polynomial(&self, seed: &[u8], eta: usize) -> Vec<i32> {
        let mut coeffs = vec![0i32; DILITHIUM_N];
        let mut ctr = 0usize;
        let mut j = 0usize;

        while ctr < DILITHIUM_N && j < seed.len() {
            let d1 = (seed[j] as u16) | ((seed[j + 1] as u16) << 8);
            let d2 = (seed[j + 1] as u16 >> 5) | ((seed[j + 2] as u16) << 3);

            for s in 0..3 {
                let t = (d1 >> (3 * s)) as i32 & 0x7;
                if t < 2 * eta as i32 {
                    coeffs[ctr] = (t as i32) - eta as i32;
                    ctr += 1;
                    if ctr >= DILITHIUM_N {
                        break;
                    }
                }
                let t2 = (d2 >> (3 * s)) as i32 & 0x7;
                if t2 < 2 * eta as i32 {
                    coeffs[ctr] = (t2 as i32) - eta as i32;
                    ctr += 1;
                    if ctr >= DILITHIUM_N {
                        break;
                    }
                }
            }
            j += 3;
        }
        coeffs
    }

    fn mat_vec_mul(&self, mat: &[Vec<Vec<i32>>], vec: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut sum = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_L {
                let a = &mat[i * DILITHIUM_L + j];
                let b = &vec[j];
                for k in 0..DILITHIUM_N {
                    sum[k] = (sum[k] + a[k] * b[k]) % DILITHIUM_Q as i32;
                }
            }
            result.push(sum);
        }
        result
    }

    fn polyvec_add_inplace(&self, a: &mut Vec<Vec<i32>>, b: &Vec<Vec<i32>>) {
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                a[i][j] = (a[i][j] + b[i][j]) % DILITHIUM_Q as i32;
            }
        }
    }

    fn polyvec_sub_inplace(&self, a: &mut Vec<Vec<i32>>, b: &Vec<Vec<i32>>) {
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                a[i][j] = (a[i][j] - b[i][j]) % DILITHIUM_Q as i32;
            }
        }
    }

    fn polyvec_reduce(&self, a: &mut Vec<Vec<i32>>) {
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                a[i][j] = ((a[i][j] as i64 + DILITHIUM_Q as i64 / 2) % DILITHIUM_Q as i64) as i32;
                if a[i][j] >= DILITHIUM_Q as i32 / 2 {
                    a[i][j] -= DILITHIUM_Q as i32;
                }
            }
        }
    }

    fn polyvec_sub(&self, a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                poly[j] = (a[i][j] - b[i][j]) % DILITHIUM_Q as i32;
            }
            result.push(poly);
        }
        result
    }

    fn polyvec_csubq(&self, a: &mut Vec<Vec<i32>>) {
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                a[i][j] = ((a[i][j] as i64 + DILITHIUM_Q as i64 / 2) % DILITHIUM_Q as i64) as i32;
                if a[i][j] >= DILITHIUM_Q as i32 / 2 {
                    a[i][j] -= DILITHIUM_Q as i32;
                }
            }
        }
    }

    fn polyvec_power2round(&self, a: &mut Vec<Vec<i32>>, t1: &mut Vec<Vec<i32>>) {
        for i in 0..DILITHIUM_K {
            for j in 0..DILITHIUM_N {
                let r = a[i][j] % (1 << 2);
                let q = (a[i][j] - r) >> 2;
                a[i][j] = r;
                t1[i][j] = q;
            }
        }
    }

    fn decompose(&self, a: i32) -> i32 {
        let r = a % (1 << 2);
        let q = (a - r) >> 2;
        q
    }

    fn decompose_vector(&self, a: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::with_capacity(DILITHIUM_K);
        for i in 0..DILITHIUM_K {
            let mut poly = vec![0i32; DILITHIUM_N];
            for j in 0..DILITHIUM_N {
                let r = a[i][j] % (1 << 2);
                let q = (a[i][j] - r) >> 2;
                let high = (q % (DILITHIUM_GAMMA2 as i32 * 2)) as i32;
                if high >= DILITHIUM_GAMMA2 as i32 {
                    poly[j] = high - DILITHIUM_GAMMA2 as i32 * 2;
                } else {
                    poly[j] = high;
                }
            }
            result.push(poly);
        }
        result
    }

    fn sample_in_ball(&self, seed: &[u8]) -> Vec<u8> {
        let mut signs = Vec::with_capacity(32);
        let mut pos = 0usize;
        let mut idx = 0usize;

        let mut expanded = [0u8; 64];
        self.crypto.hash_sha3_512(seed, &mut expanded);

        let mut coeffs = vec![0i32; DILITHIUM_N];
        let mut ctr = 0usize;
        let mut j = 0usize;

        while ctr < DILITHIUM_N && j < expanded.len() {
            let d1 = (expanded[j] as u16) | ((expanded[j + 1] as u16) << 8);
            let d2 = (expanded[j + 1] as u16 >> 6) | ((expanded[j + 2] as u16) << 2);

            for s in 0..4 {
                let t = (d1 >> (2 * s)) as i32 & 0x3;
                if t < 4 {
                    coeffs[ctr] = t as i32 - if t >= 2 { 4 } else { 0 };
                    ctr += 1;
                    if ctr >= DILITHIUM_N {
                        break;
                    }
                }
            }
            j += 3;
        }

        for i in 0..DILITHIUM_N {
            if coeffs[i] != 0 {
                if pos < 32 {
                    let sign_byte = expanded[32 + pos / 8];
                    let bit = (sign_byte >> (pos % 8)) & 1;
                    signs.push(bit as u8);
                }
                pos += 1;
                if pos >= 32 {
                    break;
                }
            }
        }

        while signs.len() < 32 {
            signs.push(0);
        }

        signs
    }
}

impl Default for DilithiumSigner {
    fn default() -> Self {
        Self::new()
    }
}
