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

const KYBER_N: usize = 256;
const KYBER_Q: i32 = 3329;
const KYBER_ETA1: usize = 3;
const KYBER_ETA2: usize = 2;
const KYBER_DU: usize = 3;
const KYBER_DV: usize = 4;
const KYBER_POLYBYTES: usize = 384;
const KYBER_POLYVECBYTES: usize = KYBER_K * KYBER_POLYBYTES;

const KYBER_K: usize = 4;
const KYBER_SeedSize: usize = 32;
const KYBER_CiphertextSize: usize = KYBER_POLYVECBYTES + KYBER_DU * KYBER_N / 8;

#[derive(Debug, Clone)]
pub struct KyberPublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct KyberSecretKey(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct KyberCiphertext(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct KyberKEM {
    crypto: Arc<DMSCCrypto>,
    random: Arc<rand::rngs::OsRng>,
}

impl KyberKEM {
    pub fn new() -> Self {
        Self {
            crypto: Arc::new(DMSCCrypto::new()),
            random: Arc::new(rand::rngs::OsRng),
        }
    }

    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let mut seed = [0u8; KYBER_SeedSize];
        self.random.fill_bytes(&mut seed);

        let mut seed_ext = [0u8; 64];
        self.crypto.hash_sha3_512(&seed, &mut seed_ext);

        let rho = &seed_ext[32..];
        let sigma = &seed_ext[..32];

        let mut sk = Vec::with_capacity(2 * KYBER_SeedSize);
        let mut pk = Vec::with_capacity(KYBER_K * KYBER_POLYBYTES);

        sk.extend_from_slice(&seed);
        pk.extend_from_slice(rho);

        let mut mat = self.generate_matrix(rho, true);

        let mut s = self.sample_polynomial_vec(sigma, KYBER_ETA1);
        let mut e = self.sample_polynomial_vec(sigma, KYBER_ETA1);
        let mut a_transposed = Vec::new();
        for i in 0..KYBER_K {
            let mut row = Vec::new();
            for j in 0..KYBER_K {
                row.push(mat[j * KYBER_K + i].clone());
            }
            a_transposed.push(row);
        }

        let mut s_times_a = self.polyvec_matrix_mul(&a_transposed, &s);
        self.polyvec_add_inplace(&mut s_times_a, &e);
        self.polyvec_csubq(&mut s_times_a);

        for i in 0..KYBER_K {
            self.poly_compress(&s_times_a[i], &mut pk[i * KYBER_POLYBYTES..(i + 1) * KYBER_POLYBYTES]);
        }

        let mut nonce = 0u8;
        let mut e1 = self.sample_polynomial_vec_from_nonce(sigma, nonce, KYBER_ETA2);
        nonce += 1;

        let mut e2 = self.sample_polynomial(sigma, KYBER_ETA2);

        let mut b = self.polyvec_matrix_mul(&mat, &s);
        self.polyvec_add_inplace(&mut b, &e);

        for i in 0..KYBER_K {
            self.poly_to_bytes(&b[i], &mut sk[KYBER_SeedSize + i * KYBER_POLYBYTES..KYBER_SeedSize + (i + 1) * KYBER_POLYBYTES]);
        }

        let pk_hash = {
            let mut hash_input = pk.clone();
            hash_input.extend_from_slice(rho);
            let mut hash_output = [0u8; 32];
            self.crypto.hash_sha3_256(&hash_input, &mut hash_output);
            hash_output
        };

        sk.extend_from_slice(&pk_hash);

        let mut c = self.polyvec_matrix_mul(&a_transposed, &s);
        self.polyvec_add_inplace(&mut c, &e1);
        self.poly_csubq(&mut c[0]);
        for i in 1..KYBER_K {
            self.poly_csubq(&mut c[i]);
        }

        let mut v = self.polyvec_dot(&s, &c);
        self.poly_add(&mut v, &e2);
        self.poly_csubq(&mut v);

        let mut v_compressed = vec![0u8; 32];
        self.poly_compress(&v, &mut v_compressed);

        let mut ct = Vec::with_capacity(KYBER_CiphertextSize);
        for i in 0..KYBER_K {
            let mut compressed = vec![0u8; KYBER_DU * KYBER_N / 8];
            self.poly_compress(&c[i], &mut compressed);
            ct.extend_from_slice(&compressed);
        }
        ct.extend_from_slice(&v_compressed);

        Ok((pk, sk))
    }

    pub fn encapsulate(&self, pk: &[u8]) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        if pk.len() != KYBER_K * KYBER_POLYBYTES {
            return Err(DMSCError::InvalidInput("Invalid public key size".to_string()));
        }

        let mut m = [0u8; 32];
        self.random.fill_bytes(&mut m);

        let mut kr = [0u8; 64];
        let mut m_hash = [0u8; 32];
        self.crypto.hash_sha3_256(&m, &mut m_hash);
        kr[..32].copy_from_slice(&m_hash);

        let pk_hash = {
            let mut hash_input = pk.to_vec();
            hash_input.extend_from_slice(&m_hash);
            let mut hash_output = [0u8; 32];
            self.crypto.hash_sha3_256(&hash_input, &mut hash_output);
            hash_output
        };
        kr[32..].copy_from_slice(&pk_hash);

        let mut rho = [0u8; 32];
        self.crypto.hash_sha3_256(&m, &mut rho);

        let mut nonce = 0u8;
        let mut r = self.sample_polynomial_vec_from_nonce(&kr, nonce, KYBER_ETA1);
        nonce += 1;

        let mut e1 = self.sample_polynomial_vec_from_nonce(&kr, nonce, KYBER_ETA2);
        nonce += 1;

        let mut e2 = self.sample_polynomial(&kr, KYBER_ETA2);

        let mut mat = self.generate_matrix(&rho, false);

        let mut t = self.polyvec_matrix_mul(&mat, &r);
        self.polyvec_add_inplace(&mut t, &e1);
        self.polyvec_csubq(&mut t);

        let mut u = Vec::new();
        for i in 0..KYBER_K {
            let mut compressed = vec![0u8; KYBER_DU * KYBER_N / 8];
            self.poly_compress(&t[i], &mut compressed);
            u.push(compressed);
        }

        let mut m_compressed = vec![0u8; 32];
        let m_poly = self.poly_decompress(&m);
        self.poly_compress(&m_poly, &mut m_compressed);

        let mut v = self.polyvec_dot(&t, &r);
        self.poly_add(&mut v, &e2);
        self.poly_add(&mut v, &m_poly);
        self.poly_csubq(&mut v);

        let mut v_compressed = vec![0u8; 32];
        self.poly_compress(&v, &mut v_compressed);

        let mut ct = Vec::new();
        for i in 0..KYBER_K {
            ct.extend_from_slice(&u[i]);
        }
        ct.extend_from_slice(&v_compressed);

        let mut ss = [0u8; 32];
        self.crypto.hash_sha3_256(&ct, &mut ss);

        Ok((ct, ss.to_vec()))
    }

    pub fn decapsulate(&self, ct: &[u8], sk: &[u8]) -> DMSCResult<Vec<u8>> {
        if ct.len() != KYBER_CiphertextSize || sk.len() < 64 {
            return Err(DMSCError::InvalidInput("Invalid ciphertext or secret key size".to_string()));
        }

        let seed = &sk[..32];
        let pk_hash = &sk[64..96];

        let mut rho = [0u8; 32];
        self.crypto.hash_sha3_256(seed, &mut rho);

        let mut nonce = 0u8;
        let s = self.sample_polynomial_vec(seed, KYBER_ETA1);

        let mut mat = self.generate_matrix(&rho, true);

        let mut u = Vec::new();
        let mut pos = 0;
        for _ in 0..KYBER_K {
            let chunk = &ct[pos..pos + KYBER_DU * KYBER_N / 8];
            pos += KYBER_DU * KYBER_N / 8;
            let poly = self.poly_decompress_compressed(chunk, KYBER_DU);
            u.push(poly);
        }

        let v_compressed = &ct[pos..pos + 32];
        let v = self.poly_decompress_compressed(v_compressed, 4);

        let mut m = self.polyvec_dot(&u, &s);
        self.poly_csubq(&mut m);
        self.poly_sub(&mut m, &v);
        self.poly_csubq(&mut m);

        let mut m_compressed = vec![0u8; 32];
        self.poly_compress(&m, &mut m_compressed);

        let mut rho_prime = [0u8; 32];
        let mut kr = [0u8; 64];
        let m_hash = {
            let mut h = [0u8; 32];
            self.crypto.hash_sha3_256(&m_compressed, &mut h);
            h
        };
        kr[..32].copy_from_slice(&m_hash);

        let pk_input = {
            let mut input = Vec::with_capacity(KYBER_K * KYBER_POLYBYTES + 32);
            input.extend_from_slice(&s);
            input.extend_from_slice(&rho);
            input
        };
        let pk_computed = {
            let mut hash_output = [0u8; 32];
            self.crypto.hash_sha3_256(&pk_input, &mut hash_output);
            hash_output
        };

        kr[32..].copy_from_slice(&pk_computed);

        let mut nonce = 0u8;
        let r_prime = self.sample_polynomial_vec_from_nonce(&kr, nonce, KYBER_ETA1);
        nonce += 1;

        let mut e1_prime = self.sample_polynomial_vec_from_nonce(&kr, nonce, KYBER_ETA2);
        nonce += 1;

        let mut e2_prime = self.sample_polynomial(&kr, KYBER_ETA2);

        let mut u_prime = self.polyvec_matrix_mul(&mat, &r_prime);
        self.polyvec_add_inplace(&mut u_prime, &e1_prime);
        self.polyvec_csubq(&mut u_prime);

        let mut v_prime = self.polyvec_dot(&r_prime, &u);
        self.poly_add(&mut v_prime, &e2_prime);
        self.poly_add(&mut v_prime, &m);
        self.poly_csubq(&mut v_prime);

        let mut u_compressed = Vec::new();
        for i in 0..KYBER_K {
            let mut compressed = vec![0u8; KYBER_DU * KYBER_N / 8];
            self.poly_compress(&u_prime[i], &mut compressed);
            u_compressed.extend_from_slice(&compressed);
        }

        let mut v_compressed_prime = vec![0u8; 32];
        self.poly_compress(&v_prime, &mut v_compressed_prime);

        let mut ct_prime = u_compressed;
        ct_prime.extend_from_slice(&v_compressed_prime);

        let (ct_check, ss) = if ct[..32] == ct_prime[..32] && ct[32..] == ct_prime[32..] {
            let mut ss_input = Vec::with_capacity(32 + 32);
            ss_input.extend_from_slice(&m_hash);
            ss_input.extend_from_slice(&ct);
            let mut ss = [0u8; 32];
            self.crypto.hash_sha3_256(&ss_input, &mut ss);
            (true, ss.to_vec())
        } else {
            let mut fail_ss = [0u8; 32];
            self.crypto.hash_sha3_256(&kr, &mut fail_ss);
            (false, fail_ss.to_vec())
        };

        Ok(ss)
    }

    fn generate_matrix(&self, rho: &[u8], transposed: bool) -> Vec<Vec<i16>> {
        let mut mat = Vec::with_capacity(KYBER_K * KYBER_K);
        for i in 0..KYBER_K {
            for j in 0..KYBER_K {
                let mut seed = [0u8; 33];
                seed[0] = (i as u8) | ((j as u8) << 4);
                if transposed {
                    seed[0] = (j as u8) | ((i as u8) << 4);
                }
                seed[1..].copy_from_slice(&rho[..32]);

                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&seed, &mut expanded);

                let coeffs = self.sample_polynomial_from_xof(&expanded);
                mat.push(coeffs);
            }
        }
        mat
    }

    fn sample_polynomial_vec(&self, seed: &[u8], eta: usize) -> Vec<Vec<i16>> {
        let mut nonce = 0u8;
        let mut result = Vec::with_capacity(KYBER_K);
        for _ in 0..KYBER_K {
            result.push(self.sample_polynomial_from_nonce(seed, eta, nonce));
            nonce += 1;
        }
        result
    }

    fn sample_polynomial_vec_from_nonce(&self, seed: &[u8], nonce: u8, eta: usize) -> Vec<Vec<i16>> {
        let mut result = Vec::with_capacity(KYBER_K);
        for i in 0..KYBER_K {
            let mut seed_with_nonce = [0u8; 33];
            seed_with_nonce[..32].copy_from_slice(seed);
            seed_with_nonce[32] = nonce + (i as u8);
            result.push(self.sample_polynomial_from_xof_eta(&seed_with_nonce, eta));
        }
        result
    }

    fn sample_polynomial(&self, seed: &[u8], eta: usize) -> Vec<i16> {
        self.sample_polynomial_from_nonce(seed, eta, 0)
    }

    fn sample_polynomial_from_nonce(&self, seed: &[u8], eta: usize, nonce: u8) -> Vec<i16> {
        let mut extended = [0u8; 168];
        let mut seed_nonce = [0u8; 33];
        seed_nonce[..32].copy_from_slice(seed);
        seed_nonce[32] = nonce;

        self.crypto.hash_sha3_256(&seed_nonce, &mut extended[..64]);

        self.crypto.hash_sha3_512(&seed_nonce, &mut extended);

        self.sample_polynomial_from_xof_eta(&extended, eta)
    }

    fn sample_polynomial_from_xof(&self, seed: &[u8]) -> Vec<i16> {
        let mut coeffs = vec![0i16; KYBER_N];
        let mut j = 0;
        let mut d1 = 0i32;
        let mut d2 = 0i32;

        for i in 0..(504 / 3) {
            let mut buf = [0u8; 3];
            buf.copy_from_slice(&seed[j..j + 3]);
            j += 3;

            let b1 = buf[0] as i32;
            let b2 = (buf[0] >> 8) as i32 | ((buf[1] as i32) << 8);
            let b3 = (buf[1] >> 6) as i32 | ((buf[2] as i32) << 2);

            d1 = b1 + ((b2 & 0xF) as i32) * 256 - KYBER_Q as i32;
            d2 = (b2 >> 4) as i32 + ((b3 & 0x3) as i32) * 16 - KYBER_Q as i32;

            let i1 = (d1 as i32) < 0i32;
            let i2 = (d2 as i32) < 0i32;

            coeffs[3 * i] = if i1 { -d1 } else { d1 };
            coeffs[3 * i + 1] = if i2 { -d2 } else { d2 };

            let b1_next = (buf[2] >> 4) as i32;
            let b2_next = (buf[2] >> 12) as i32 | ((0u8::wrapping_sub(0)) << 8);

            d1 = b1_next as i32 - KYBER_Q as i32;
            coeffs[3 * i + 2] = if d1 < 0 { -d1 } else { d1 };
        }

        coeffs
    }

    fn sample_polynomial_from_xof_eta(&self, seed: &[u8], eta: usize) -> Vec<i16> {
        let mut coeffs = vec![0i16; KYBER_N];

        if eta == 2 {
            let mut ctr = 0usize;
            let mut j = 0usize;

            while ctr < KYBER_N && j < seed.len() {
                let d1 = (seed[j] as u16) | ((seed[j + 1] as u16) << 8);
                let d2 = (seed[j + 1] as u16 >> 6) | ((seed[j + 2] as u16) << 2);

                for s in 0..4 {
                    let t = (d1 >> (2 * s)) as i16 & 0x3;
                    if t < 5 {
                        coeffs[ctr] = 2 - t;
                        ctr += 1;
                        if ctr >= KYBER_N {
                            break;
                        }
                    }
                    let t2 = (d2 >> (2 * s)) as i16 & 0x3;
                    if t2 < 5 {
                        coeffs[ctr] = t2 - 2;
                        ctr += 1;
                        if ctr >= KYBER_N {
                            break;
                        }
                    }
                }
                j += 3;
            }
        } else {
            let mut ctr = 0usize;
            let mut j = 0usize;

            while ctr < KYBER_N && j < seed.len() {
                let d1 = (seed[j] as u16) | ((seed[j + 1] as u16) << 8);
                let d2 = (seed[j + 1] as u16 >> 5) | ((seed[j + 2] as u16) << 3);

                for s in 0..3 {
                    let t = (d1 >> (3 * s)) as i16 & 0x7;
                    if t < 7 {
                        coeffs[ctr] = 3 - t;
                        ctr += 1;
                        if ctr >= KYBER_N {
                            break;
                        }
                    }
                    let t2 = (d2 >> (3 * s)) as i16 & 0x7;
                    if t2 < 7 {
                        coeffs[ctr] = t2 - 3;
                        ctr += 1;
                        if ctr >= KYBER_N {
                            break;
                        }
                    }
                }
                j += 3;
            }
        }

        coeffs
    }

    fn polyvec_matrix_mul(&self, mat: &[Vec<Vec<i16>>], vec: &[Vec<i16>]) -> Vec<Vec<i16>> {
        let mut result = Vec::with_capacity(KYBER_K);
        for i in 0..KYBER_K {
            let mut sum = vec![0i16; KYBER_N];
            for j in 0..KYBER_K {
                let prod = self.poly_mul(&mat[i * KYBER_K + j], &vec[j]);
                for k in 0..KYBER_N {
                    sum[k] = (sum[k] + prod[k]) % KYBER_Q as i16;
                }
            }
            result.push(sum);
        }
        result
    }

    fn polyvec_dot(&self, a: &[Vec<i16>], b: &[Vec<i16>]) -> Vec<i16> {
        let mut result = vec![0i16; KYBER_N];
        for i in 0..KYBER_K {
            let prod = self.poly_mul(&a[i], &b[i]);
            for j in 0..KYBER_N {
                result[j] = (result[j] + prod[j]) % KYBER_Q as i16;
            }
        }
        result
    }

    fn poly_mul(&self, a: &[i16], b: &[i16]) -> Vec<i16> {
        let mut result = vec![0i16; 2 * KYBER_N];
        for i in 0..KYBER_N {
            for j in 0..KYBER_N {
                let idx = i + j;
                if idx < 2 * KYBER_N {
                    let val = (a[i] as i32 * b[j] as i32) % (2 * KYBER_Q as i32);
                    result[idx] = ((result[idx] as i32 + val) / 2) as i16;
                }
            }
        }
        let mut reduced = vec![0i16; KYBER_N];
        for i in 0..KYBER_N {
            let val = result[i] as i32;
            let reduced_val = ((val + KYBER_Q as i32 / 2) / KYBER_Q as i32) as i16;
            reduced[i] = val as i16 - reduced_val * (KYBER_Q as i16);
        }
        reduced
    }

    fn polyvec_add_inplace(&self, a: &mut Vec<Vec<i16>>, b: &Vec<Vec<i16>>) {
        for i in 0..KYBER_K {
            for j in 0..KYBER_N {
                a[i][j] = (a[i][j] + b[i][j]) % KYBER_Q as i16;
            }
        }
    }

    fn polyvec_csubq(&self, a: &mut Vec<Vec<i16>>) {
        for i in 0..KYBER_K {
            self.poly_csubq(&mut a[i]);
        }
    }

    fn poly_csubq(&self, a: &mut Vec<i16>) {
        for i in 0..KYBER_N {
            a[i] = ((a[i] as i32 + KYBER_Q as i32 / 2) % KYBER_Q as i32) as i16;
            if a[i] >= KYBER_Q as i16 / 2 {
                a[i] -= KYBER_Q as i16;
            }
        }
    }

    fn poly_add(&self, a: &mut Vec<i16>, b: &Vec<i16>) {
        for i in 0..KYBER_N {
            a[i] = (a[i] + b[i]) % KYBER_Q as i16;
        }
    }

    fn poly_sub(&self, a: &mut Vec<i16>, b: &Vec<i16>) {
        for i in 0..KYBER_N {
            a[i] = (a[i] - b[i]) % KYBER_Q as i16;
        }
    }

    fn poly_compress(&self, a: &[i16], out: &mut [u8]) {
        for i in 0..(out.len() * 8 / KYBER_DU) {
            let val = ((a[i] as i32 * 8) / KYBER_Q as i32) as u8;
            out[i * KYBER_DU / 8] |= val << ((i * KYBER_DU) % 8);
        }
    }

    fn poly_decompress(&self, a: &[u8]) -> Vec<i16> {
        let mut result = vec![0i16; KYBER_N];
        for i in 0..32 {
            for j in 0..8 {
                let val = (a[i] >> (j * 3)) & 0x7;
                result[i * 8 + j] = ((val as i32 * KYBER_Q as i32 + 4) / 8) as i16;
            }
        }
        result
    }

    fn poly_decompress_compressed(&self, a: &[u8], d: usize) -> Vec<i16> {
        let mut result = vec![0i16; KYBER_N];
        for i in 0..(a.len() * 8 / d) {
            let mut val = 0u16;
            for j in 0..d {
                val |= ((a[i * d / 8 + j] as u16) << (j * 8)) & (0xFFu16 << (j * 8));
            }
            result[i] = ((val as i32 * KYBER_Q as i32 + (1 << (d - 1)) - 1) / (1 << d)) as i16;
        }
        result
    }

    fn poly_to_bytes(&self, a: &[i16], out: &mut [u8]) {
        for i in 0..(out.len() / 3) {
            let idx = i * 3;
            let t0 = ((a[idx] as i32 + 0x800) % KYBER_Q as i32) >> 0;
            let t1 = ((a[idx] as i32 + 0x800) % KYBER_Q as i32) >> 8;
            let t2 = ((a[idx] as i32 + 0x800) % KYBER_Q as i32) >> 16;
            out[idx] = (t0 | (t1 << 8) | (t2 << 16)) as u8;
        }
    }
}

impl Default for KyberKEM {
    fn default() -> Self {
        Self::new()
    }
}
