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

const FALCON_N: usize = 512;
const FALCON_Q: i32 = 12289;
const FALCON_LOGN: usize = 9;
const FALCON_SIGMA: f64 = 1.9205295783377863;
const FALCON_TAU: f64 = 1.4203930715656246;
const FALCON_B: f64 = 1.4203930715656246;
const FALCON_SIGMA_MIN: f64 = 1.4010926648502084;
const FALCON_SIGMA_MAX: f64 = 2.558629373688786;

#[derive(Debug, Clone)]
pub struct FalconPublicKey(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct FalconSecretKey(Vec<u8>);

#[derive(Debug, Clone)]
pub struct FalconSignature(pub Vec<u8>);

#[derive(Debug, Clone)]
pub struct FalconSigner {
    crypto: Arc<DMSCCrypto>,
    random: Arc<rand::rngs::OsRng>,
}

impl FalconSigner {
    pub fn new() -> Self {
        Self {
            crypto: Arc::new(DMSCCrypto::new()),
            random: Arc::new(rand::rngs::OsRng),
        }
    }

    pub fn keygen(&self) -> DMSCResult<(Vec<u8>, Vec<u8>)> {
        let mut seed = [0u8; 48];
        self.random.fill_bytes(&mut seed);

        let mut f = vec![0i32; FALCON_N];
        let mut g = vec![0i32; FALCON_N];
        self.generate_fg(&seed, &mut f, &mut g);

        let mut f_mod_q = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            f_mod_q[i] = ((f[i] % FALCON_Q as i32) + FALCON_Q as i32) % FALCON_Q as i32;
        }

        let mut g_mod_q = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            g_mod_q[i] = ((g[i] % FALCON_Q as i32) + FALCON_Q as i32) % FALCON_Q as i32;
        }

        let (f_inv, _) = self.extended_gcd(&f_mod_q, &g_mod_q);

        let mut h = self.poly_mul(&g_mod_q, &f_inv);
        self.poly_mod(&mut h, FALCON_Q);
        self.poly_reduce(&mut h, FALCON_Q);

        for i in 0..FALCON_N {
            h[i] = ((h[i] as i64 * 2) % FALCON_Q as i64) as i32;
        }
        self.poly_reduce(&mut h, FALCON_Q);

        let mut sk = Vec::with_capacity(2 * FALCON_N * 4 + FALCON_N * 4);
        for i in 0..FALCON_N {
            let mut bytes = [0u8; 4];
            let val = ((f[i] as i64) + 0x8000 as i64) as u32;
            bytes[0] = (val >> 0) as u8;
            bytes[1] = (val >> 8) as u8;
            bytes[2] = (val >> 16) as u8;
            bytes[3] = (val >> 24) as u8;
            sk.extend_from_slice(&bytes);
        }

        for i in 0..FALCON_N {
            let mut bytes = [0u8; 4];
            let val = ((g[i] as i64) + 0x8000 as i64) as u32;
            bytes[0] = (val >> 0) as u8;
            bytes[1] = (val >> 8) as u8;
            bytes[2] = (val >> 16) as u8;
            bytes[3] = (val >> 24) as u8;
            sk.extend_from_slice(&bytes);
        }

        let mut pk = Vec::with_capacity(FALCON_N * 2);
        for i in 0..FALCON_N {
            pk.push(((h[i] as i64) + 0x8000 as i64) as u8);
        }

        let mut pk_compressed = Vec::with_capacity(FALCON_N);
        for i in 0..FALCON_N {
            let val = (h[i] as i64) + FALCON_Q as i64;
            let compressed = ((val as u32) >> 13) as u8;
            pk_compressed.push(compressed);
        }

        let mut pk_final = Vec::with_capacity(FALCON_N);
        pk_final.extend_from_slice(&seed);
        pk_final.extend_from_slice(&pk_compressed);

        Ok((pk_final, sk))
    }

    pub fn sign(&self, sk: &[u8], msg: &[u8]) -> DMSCResult<Vec<u8>> {
        if sk.len() < 2 * FALCON_N * 4 {
            return Err(DMSCError::InvalidInput("Invalid secret key size".to_string()));
        }

        let mut f = vec![0i32; FALCON_N];
        let mut g = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            let offset = i * 4;
            let v0 = sk[offset] as i32;
            let v1 = sk[offset + 1] as i32;
            let v2 = sk[offset + 2] as i32;
            let v3 = sk[offset + 3] as i32;
            let val = ((v0 as u32) | ((v1 as u32) << 8) | ((v2 as u32) << 16) | ((v3 as u32) << 24)) as i32;
            f[i] = val as i32 - 0x8000 as i32;
        }

        for i in 0..FALCON_N {
            let offset = FALCON_N * 4 + i * 4;
            let v0 = sk[offset] as i32;
            let v1 = sk[offset + 1] as i32;
            let v2 = sk[offset + 2] as i32;
            let v3 = sk[offset + 3] as i32;
            let val = ((v0 as u32) | ((v1 as u32) << 8) | ((v2 as u32) << 16) | ((v3 as u32) << 24)) as i32;
            g[i] = val as i32 - 0x8000 as i32;
        }

        let mut f_mod_q = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            f_mod_q[i] = ((f[i] % FALCON_Q as i32) + FALCON_Q as i32) % FALCON_Q as i32;
        }

        let mut g_mod_q = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            g_mod_q[i] = ((g[i] % FALCON_Q as i32) + FALCON_Q as i32) % FALCON_Q as i32;
        }

        let (f_inv, _) = self.extended_gcd(&f_mod_q, &g_mod_q);

        let mut h = self.poly_mul(&g_mod_q, &f_inv);
        self.poly_mod(&mut h, FALCON_Q);
        self.poly_reduce(&mut h, FALCON_Q);

        for i in 0..FALCON_N {
            h[i] = ((h[i] as i64 * 2) % FALCON_Q as i64) as i32;
        }
        self.poly_reduce(&mut h, FALCON_Q);

        let mut nonce = 0u8;
        let mut signature = Vec::new();

        loop {
            let mut c = [0u8; 64];
            let mut seed = [0u8; 40];
            self.random.fill_bytes(&mut seed);

            let mut msg_hash = [0u8; 64];
            self.crypto.hash_sha3_512(msg, &mut msg_hash);

            let mut c_input = Vec::with_capacity(40 + 64);
            c_input.extend_from_slice(&seed);
            c_input.extend_from_slice(&msg_hash);
            self.crypto.hash_sha3_512(&c_input, &mut c);

            let m = self.sample_leading_bits(&c);

            let mut r = vec![0i32; FALCON_N];
            for i in 0..FALCON_N {
                let mut noise_seed = [0u8; 48];
                noise_seed[..40].copy_from_slice(&seed);
                noise_seed[40] = nonce;
                noise_seed[41] = (i >> 0) as u8;
                noise_seed[42] = (i >> 8) as u8;
                noise_seed[43] = 0;
                noise_seed[44] = 0;
                noise_seed[45] = 0;
                noise_seed[46] = 0;
                noise_seed[47] = 0;

                let mut expanded = [0u8; 64];
                self.crypto.hash_sha3_512(&noise_seed, &mut expanded);

                let val = ((expanded[i % 32] as u32) | ((expanded[(i + 1) % 32] as u32) << 8)) as f64;
                let gaussian = self.sample_gaussian(val / 65536.0, FALCON_SIGMA);
                r[i] = (gaussian * 256.0) as i32;
            }

            let r_fft = self.poly_fft(&r, false);

            let m_fft = self.poly_fft(&m, false);

            let mut t1_fft = vec![0f64; 2 * FALCON_N];
            let mut t2_fft = vec![0f64; 2 * FALCON_N];
            for i in 0..FALCON_N {
                t1_fft[2 * i] = r_fft[2 * i] * h[2 * i] - r_fft[2 * i + 1] * h[2 * i + 1];
                t1_fft[2 * i + 1] = r_fft[2 * i] * h[2 * i + 1] + r_fft[2 * i + 1] * h[2 * i];
                t2_fft[2 * i] = m_fft[2 * i];
                t2_fft[2 * i + 1] = m_fft[2 * i + 1];
            }

            let mut t_fft = vec![0f64; 2 * FALCON_N];
            for i in 0..2 * FALCON_N {
                t_fft[i] = t1_fft[i] + t2_fft[i];
            }

            let mut t = self.poly_ifft(&t_fft);

            let mut s = vec![0i32; FALCON_N];
            let mut s1 = vec![0i32; FALCON_N];
            let mut s2 = vec![0i32; FALCON_N];

            self.split_ternary(&f, &mut s1, &mut s2);
            self.split_ternary(&g, &mut s1, &mut s2);

            let mut norm_s = 0i64;
            for i in 0..FALCON_N {
                norm_s += (s1[i] as i64) * (s1[i] as i64);
            }
            let bound = (FALCON_TAU * FALCON_TAU * (FALCON_N as f64)) as i64;

            if norm_s < bound {
                for i in 0..FALCON_N {
                    signature.push(((t[i] as i64) + 0x8000 as i64) as u8);
                }
                return Ok(signature);
            }

            nonce += 1;
            if nonce > 100 {
                return Err(DMSCError::SigningFailed("Failed to generate valid signature".to_string()));
            }
        }
    }

    pub fn verify(&self, pk: &[u8], msg: &[u8], sig: &[u8]) -> DMSCResult<bool> {
        if pk.len() < 48 + FALCON_N {
            return Err(DMSCError::InvalidInput("Invalid public key size".to_string()));
        }
        if sig.len() < FALCON_N {
            return Err(DMSCError::InvalidInput("Invalid signature size".to_string()));
        }

        let seed = &pk[..48];
        let h_compressed = &pk[48..48 + FALCON_N];

        let mut h = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            let val = h_compressed[i] as i32;
            let high = (val << 13) as i32;
            let low = (val >> 3) as i32 & 0x1FFF;
            h[i] = high + low;
        }

        let mut s = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            let val = sig[i] as i32;
            s[i] = val as i32 - 0x8000 as i32;
        }

        let mut norm_s = 0i64;
        for i in 0..FALCON_N {
            norm_s += (s[i] as i64) * (s[i] as i64);
        }
        let bound = (FALCON_TAU * FALCON_TAU * (FALCON_N as f64)) as i64;

        if norm_s > bound {
            return Ok(false);
        }

        let mut msg_hash = [0u8; 64];
        self.crypto.hash_sha3_512(msg, &mut msg_hash);

        let mut c_input = Vec::with_capacity(48 + 64);
        c_input.extend_from_slice(seed);
        c_input.extend_from_slice(&msg_hash);
        let mut c = [0u8; 64];
        self.crypto.hash_sha3_512(&c_input, &mut c);

        let m = self.sample_leading_bits(&c);

        let s_fft = self.poly_fft(&s, false);
        let h_fft = self.poly_fft(&h, false);
        let m_fft = self.poly_fft(&m, false);

        let mut sh_fft = vec![0f64; 2 * FALCON_N];
        for i in 0..FALCON_N {
            sh_fft[2 * i] = s_fft[2 * i] * h_fft[2 * i] - s_fft[2 * i + 1] * h_fft[2 * i + 1];
            sh_fft[2 * i + 1] = s_fft[2 * i] * h_fft[2 * i + 1] + s_fft[2 * i + 1] * h_fft[2 * i];
        }

        let mut c_computed_fft = vec![0f64; 2 * FALCON_N];
        for i in 0..2 * FALCON_N {
            c_computed_fft[i] = sh_fft[i] - m_fft[i];
        }

        let mut c_computed = self.poly_ifft(&c_computed_fft);

        self.poly_reduce(&mut c_computed, FALCON_Q);

        let mut c_leading = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            let val = c_computed[i] as i64;
            let leading = ((val + FALCON_Q as i64 / 2) / FALCON_Q as i64) as i32;
            c_leading[i] = leading;
        }

        let c_original = self.sample_leading_bits(&c);

        let mut match_count = 0;
        for i in 0..FALCON_N {
            if c_leading[i] == c_original[i] {
                match_count += 1;
            }
        }

        Ok(match_count >= FALCON_N - 10)
    }

    fn generate_fg(&self, seed: &[u8], f: &mut [i32], g: &mut [i32]) {
        let mut ctr = 0usize;
        let mut j = 0usize;

        while ctr < FALCON_N {
            let mut expanded = [0u8; 64];
            let mut input = [0u8; 49];
            input[..48].copy_from_slice(seed);
            input[48] = ctr as u8;

            self.crypto.hash_sha3_512(&input, &mut expanded);

            for i in 0..64 {
                let val = (expanded[i] as u16) | ((expanded[i + 1] as u16) << 8);
                for _ in 0..2 {
                    let d = (val & 0xF) as i32;
                    if d < 9 {
                        f[ctr] = d as i32 - 1;
                        ctr += 1;
                        if ctr >= FALCON_N {
                            return;
                        }
                    }
                }
            }
        }
    }

    fn split_ternary(&self, poly: &[i32], s1: &mut [i32], s2: &mut [i32]) {
        for i in 0..FALCON_N {
            let val = poly[i];
            if val > 0 {
                let count = val.min(1) as i32;
                s1[i] = count;
                s2[i] = val - count;
            } else {
                let count = (-val).min(1) as i32;
                s1[i] = -count;
                s2[i] = val + count;
            }
        }
    }

    fn extended_gcd(&self, a: &[i32], b: &[i32]) -> (Vec<i32>, i32) {
        let mut old_r = a.to_vec();
        let mut r = b.to_vec();
        let mut old_s = vec![1i32; FALCON_N];
        let mut s = vec![0i32; FALCON_N];
        let mut old_t = vec![0i32; FALCON_N];
        let mut t = vec![1i32; FALCON_N];

        for _ in 0..100 {
            let q = self.poly_div(&old_r, &r);
            let temp_r = r.clone();
            r = self.poly_sub(&old_r, &self.poly_mul(&q, &r));
            old_r = temp_r;

            let temp_s = s.clone();
            s = self.poly_sub(&old_s, &self.poly_mul(&q, &s));
            old_s = temp_s;

            let temp_t = t.clone();
            t = self.poly_sub(&old_t, &self.poly_mul(&q, &t));
            old_t = temp_t;
        }

        for i in 0..FALCON_N {
            if old_r[i] < 0 {
                old_r[i] += FALCON_Q as i32;
            }
        }

        (old_r, 1)
    }

    fn sample_gaussian(&self, x: f64, sigma: f64) -> f64 {
        let u1 = x;
        let u2 = 1.0 - x;

        let mut z1 = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
        let z2 = (-2.0 * u2.ln()).sqrt() * (2.0 * std::f64::consts::PI * u1).sin();

        if z1 * z1 + z2 * z2 < 1.0 {
            return z1 * sigma;
        }

        self.sample_gaussian(x, sigma)
    }

    fn sample_leading_bits(&self, seed: &[u8]) -> Vec<i32> {
        let mut result = vec![0i32; FALCON_N];
        let mut j = 0usize;
        let mut bits = 0u32;
        let mut bit_count = 0usize;

        for i in 0..FALCON_N {
            while bit_count < 14 {
                bits = (bits << 8) | (seed[j] as u32);
                j += 1;
                bit_count += 8;
            }

            result[i] = ((bits >> (bit_count - 14)) as i32) & 0x3FFF;
            bit_count -= 14;
        }

        result
    }

    fn poly_mul(&self, a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut result = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            for j in 0..FALCON_N {
                let k = (i + j) % FALCON_N;
                result[k] = (result[k] as i64 + (a[i] as i64 * b[j] as i64)) as i32;
            }
        }
        result
    }

    fn poly_div(&self, a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut result = vec![0i32; FALCON_N];
        let mut remainder = a.to_vec();

        for i in (0..FALCON_N).rev() {
            if remainder[i] != 0 {
                let factor = (remainder[i] as i64 * self.mod_inverse(b[0] as i64, FALCON_Q as i64)) as i32;
                result[i] = factor;

                for j in 0..=i {
                    remainder[j] = (remainder[j] as i64 - (factor as i64 * b[(i - j) % FALCON_N] as i64)) as i32;
                    self.poly_reduce(&mut remainder, FALCON_Q);
                }
            }
        }

        result
    }

    fn poly_sub(&self, a: &[i32], b: &[i32]) -> Vec<i32> {
        let mut result = vec![0i32; FALCON_N];
        for i in 0..FALCON_N {
            result[i] = a[i] - b[i];
        }
        result
    }

    fn poly_mod(&self, a: &mut [i32], q: i32) {
        for i in 0..FALCON_N {
            a[i] = ((a[i] % q) + q) % q;
        }
    }

    fn poly_reduce(&self, a: &mut [i32], q: i32) {
        for i in 0..FALCON_N {
            a[i] = ((a[i] as i64 + q as i64 / 2) / q as i64) as i32;
            a[i] = ((a[i] as i64 * q as i64) % q as i64) as i32;
        }
    }

    fn mod_inverse(&self, a: i64, m: i64) -> i64 {
        let mut t = 0i64;
        let mut new_t = 1i64;
        let mut r = m;
        let mut new_r = a;

        while new_r != 0 {
            let quotient = r / new_r;
            let temp_t = t;
            t = new_t;
            new_t = temp_t - quotient as i64 * new_t;
            let temp_r = r;
            r = new_r;
            new_r = temp_r - quotient as i64 * new_r;
        }

        if r > 1 {
            return -1;
        }
        if t < 0 {
            t += m;
        }
        t
    }

    fn poly_fft(&self, a: &[i32], inverse: bool) -> Vec<f64> {
        let mut result = vec![0.0; 2 * FALCON_N];

        let n = FALCON_N;
        let sign = if inverse { 1.0 } else { -1.0 };

        for i in 0..n {
            let mut sum_real = 0.0f64;
            let mut sum_imag = 0.0f64;

            for j in 0..n {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) * (j as f64) / (n as f64) * sign;
                let cos_angle = angle.cos();
                let sin_angle = angle.sin();

                sum_real += (a[j] as f64) * cos_angle;
                sum_imag += (a[j] as f64) * sin_angle;
            }

            result[2 * i] = sum_real / (n as f64).sqrt();
            result[2 * i + 1] = sum_imag / (n as f64).sqrt();
        }

        result
    }

    fn poly_ifft(&self, a: &[f64]) -> Vec<i32> {
        let mut result = vec![0i32; FALCON_N];

        let n = FALCON_N;

        for i in 0..n {
            let mut sum_real = 0.0f64;
            let mut sum_imag = 0.0f64;

            for j in 0..n {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) * (j as f64) / (n as f64);
                let cos_angle = angle.cos();
                let sin_angle = angle.sin();

                sum_real += a[2 * j] * cos_angle - a[2 * j + 1] * sin_angle;
                sum_imag += a[2 * j] * sin_angle + a[2 * j + 1] * cos_angle;
            }

            result[i] = (sum_real / (n as f64).sqrt()) as i32;
        }

        result
    }
}

impl Default for FalconSigner {
    fn default() -> Self {
        Self::new()
    }
}
