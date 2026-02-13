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

//! # Protocol/Cryptography Module Benchmarks
//!
//! This benchmark suite measures the performance of DMSC protocol and crypto operations.
//! It tests various cryptographic components including:
//! - Post-quantum cryptography (Kyber KEM)
//! - Digital signatures (Dilithium, Falcon)
//! - Symmetric encryption (AES-GCM)
//! - Hash functions
//!
//! ## Running Benchmarks
//!
//! ```bash
//! cargo bench --bench protocol_benchmark
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

#[cfg(feature = "protocol")]
fn bench_kyber_kem(c: &mut Criterion) {
    use dmsc::protocol::kyber::KyberKEM;
    use dmsc::protocol::DMSCPostQuantumAlgorithm;
    
    let mut group = c.benchmark_group("kyber_kem");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("kyber512_keygen", |b| {
        let kem = KyberKEM::with_algorithm(DMSCPostQuantumAlgorithm::Kyber512);
        b.iter(|| {
            let result = kem.keygen().unwrap();
            black_box(result);
        });
    });
    
    group.bench_function("kyber768_keygen", |b| {
        let kem = KyberKEM::with_algorithm(DMSCPostQuantumAlgorithm::Kyber768);
        b.iter(|| {
            let result = kem.keygen().unwrap();
            black_box(result);
        });
    });
    
    group.bench_function("kyber1024_keygen", |b| {
        let kem = KyberKEM::with_algorithm(DMSCPostQuantumAlgorithm::Kyber1024);
        b.iter(|| {
            let result = kem.keygen().unwrap();
            black_box(result);
        });
    });
    
    let kem = KyberKEM::new();
    let (pk, sk) = kem.keygen().unwrap();
    
    group.bench_function("kyber_encapsulate", |b| {
        b.iter(|| {
            let result = kem.encapsulate(&pk).unwrap();
            black_box(result);
        });
    });
    
    let kem_result = kem.encapsulate(&pk).unwrap();
    
    group.bench_function("kyber_decapsulate", |b| {
        b.iter(|| {
            let result = kem.decapsulate(&kem_result.ciphertext, &sk).unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

#[cfg(not(feature = "protocol"))]
fn bench_kyber_kem(_c: &mut Criterion) {}

#[cfg(feature = "protocol")]
fn bench_dilithium_signature(c: &mut Criterion) {
    use dmsc::protocol::dilithium::DilithiumSigner;
    use dmsc::protocol::DMSCPostQuantumAlgorithm;
    
    let mut group = c.benchmark_group("dilithium_signature");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("dilithium2_keygen", |b| {
        let signer = DilithiumSigner::with_algorithm(DMSCPostQuantumAlgorithm::Dilithium2);
        b.iter(|| {
            let result = signer.keygen().unwrap();
            black_box(result);
        });
    });
    
    let signer = DilithiumSigner::new();
    let (pk, sk) = signer.keygen().unwrap();
    let message = b"Test message for benchmarking";
    
    group.bench_function("dilithium_sign", |b| {
        b.iter(|| {
            let result = signer.sign(message, &sk).unwrap();
            black_box(result);
        });
    });
    
    let signature = signer.sign(message, &sk).unwrap();
    
    group.bench_function("dilithium_verify", |b| {
        b.iter(|| {
            let result = signer.verify(message, &signature, &pk).unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

#[cfg(not(feature = "protocol"))]
fn bench_dilithium_signature(_c: &mut Criterion) {}

#[cfg(feature = "protocol")]
fn bench_falcon_signature(c: &mut Criterion) {
    use dmsc::protocol::falcon::FalconSigner;
    use dmsc::protocol::DMSCPostQuantumAlgorithm;
    
    let mut group = c.benchmark_group("falcon_signature");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("falcon512_keygen", |b| {
        let signer = FalconSigner::with_algorithm(DMSCPostQuantumAlgorithm::Falcon512);
        b.iter(|| {
            let result = signer.keygen().unwrap();
            black_box(result);
        });
    });
    
    let signer = FalconSigner::new();
    let (pk, sk) = signer.keygen().unwrap();
    let message = b"Test message for benchmarking";
    
    group.bench_function("falcon_sign", |b| {
        b.iter(|| {
            let result = signer.sign(message, &sk).unwrap();
            black_box(result);
        });
    });
    
    let signature = signer.sign(message, &sk).unwrap();
    
    group.bench_function("falcon_verify", |b| {
        b.iter(|| {
            let result = signer.verify(message, &signature, &pk).unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

#[cfg(not(feature = "protocol"))]
fn bench_falcon_signature(_c: &mut Criterion) {}

fn bench_aes_gcm_encryption(c: &mut Criterion) {
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
    use aes_gcm::aead::Aead;
    
    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");
    
    let mut group = c.benchmark_group("aes_gcm");
    
    for size in [16, 64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("encrypt", size), size, |b, _| {
            b.iter(|| {
                let result = cipher.encrypt(nonce, data.as_ref()).unwrap();
                black_box(result);
            });
        });
        
        let ciphertext = cipher.encrypt(nonce, data.as_ref()).unwrap();
        
        group.bench_with_input(BenchmarkId::new("decrypt", size), size, |b, _| {
            b.iter(|| {
                let result = cipher.decrypt(nonce, ciphertext.as_ref()).unwrap();
                black_box(result);
            });
        });
    }
    
    group.finish();
}

fn bench_hash_functions(c: &mut Criterion) {
    use ring::digest::{digest, SHA256, SHA384, SHA512};
    
    let mut group = c.benchmark_group("hash_functions");
    
    for size in [64, 256, 1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("sha256", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA256, &data);
                black_box(result);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("sha384", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA384, &data);
                black_box(result);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("sha512", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA512, &data);
                black_box(result);
            });
        });
    }
    
    group.finish();
}

fn bench_base64_encoding(c: &mut Criterion) {
    use base64::{Engine, engine::general_purpose::STANDARD};
    
    let mut group = c.benchmark_group("base64");
    
    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| {
                let result = STANDARD.encode(&data);
                black_box(result);
            });
        });
        
        let encoded = STANDARD.encode(&data);
        
        group.bench_with_input(BenchmarkId::new("decode", size), size, |b, _| {
            b.iter(|| {
                let result = STANDARD.decode(&encoded).unwrap();
                black_box(result);
            });
        });
    }
    
    group.finish();
}

fn bench_hex_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("hex_encoding");
    
    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| {
                let result = hex::encode(&data);
                black_box(result);
            });
        });
        
        let encoded = hex::encode(&data);
        
        group.bench_with_input(BenchmarkId::new("decode", size), size, |b, _| {
            b.iter(|| {
                let result = hex::decode(&encoded).unwrap();
                black_box(result);
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    protocol_benches,
    bench_kyber_kem,
    bench_dilithium_signature,
    bench_falcon_signature,
    bench_aes_gcm_encryption,
    bench_hash_functions,
    bench_base64_encoding,
    bench_hex_encoding,
);

criterion_main!(protocol_benches);
