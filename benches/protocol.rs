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

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

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

fn bench_frame_parsing(c: &mut Criterion) {
    use dmsc::protocol::{DMSCFrameBuilder, DMSCFrameParser};
    
    let mut group = c.benchmark_group("frame_parsing");
    
    let payload = vec![0u8; 256];
    let mut builder = DMSCFrameBuilder::new();
    let frame = builder.build_data_frame(payload.clone()).unwrap();
    let frame_bytes = frame.to_bytes().unwrap();
    
    group.throughput(Throughput::Bytes(frame_bytes.len() as u64));
    
    group.bench_function("parse_frame", |b| {
        b.iter(|| {
            let mut parser = DMSCFrameParser::new();
            parser.add_data(&frame_bytes);
            let result = parser.parse_frame().unwrap();
            black_box(result);
        });
    });
    
    group.bench_function("build_frame", |b| {
        b.iter(|| {
            let mut builder = DMSCFrameBuilder::new();
            let result = builder.build_data_frame(payload.clone()).unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

criterion_group!(
    protocol_benches,
    bench_aes_gcm_encryption,
    bench_hash_functions,
    bench_base64_encoding,
    bench_hex_encoding,
    bench_frame_parsing,
);

criterion_main!(protocol_benches);
