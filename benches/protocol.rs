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
//! This module provides performance benchmarks for cryptographic operations and
//! protocol-level data transformations used by DMSC.
//!
//! ## Benchmark Categories
//!
//! 1. **AES-GCM Encryption**: Authenticated encryption performance
//!
//! 2. **Hash Functions**: SHA-256, SHA-384, SHA-512 throughput
//!
//! 3. **Base64 Encoding**: Binary-to-text encoding/decoding
//!
//! 4. **Hex Encoding**: Binary-to-hexadecimal conversion
//!
//! 5. **Frame Parsing**: Protocol frame building and parsing
//!
//! ## Cryptography Notes
//!
//! DMSC uses:
//! - AES-256-GCM for symmetric encryption
//! - SHA family for hashing (via ring crate)
//! - These are industry-standard cryptographic primitives
//!
//! ## Security Considerations
//!
//! - Benchmarks use all-zero keys for consistency only
//! - Production should use cryptographically random keys
//! - Nonces should be unique per encryption operation

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

/// Benchmark: AES-256-GCM authenticated encryption/decryption.
///
/// AES-GCM provides both:
/// - Confidentiality (encryption)
/// - Authenticity (HMAC-like authentication tag)
///
/// Used in DMSC for:
/// - Secure session storage
/// - Token encryption
/// - Sensitive data at rest
///
/// Performance scales with data size.
fn bench_aes_gcm_encryption(c: &mut Criterion) {
    use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
    use aes_gcm::aead::Aead;

    let key = aes_gcm::Key::<Aes256Gcm>::from_slice(&[0u8; 32]);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(b"unique nonce");

    let mut group = c.benchmark_group("aes_gcm");

    /// Test across different payload sizes: 16B to 4KB
    for size in [16, 64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        /// Encryption: Encrypts plaintext to ciphertext
        group.bench_with_input(BenchmarkId::new("encrypt", size), size, |b, _| {
            b.iter(|| {
                let result = cipher.encrypt(nonce, data.as_ref()).unwrap();
                black_box(result);
            });
        });

        /// Decryption: Decrypts ciphertext back to plaintext
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

/// Benchmark: SHA family hash function performance.
///
/// SHA (Secure Hash Algorithm) family:
/// - SHA-256: 256-bit output, good balance
/// - SHA-384: 384-bit output, stronger security
/// - SHA-512: 512-bit output, maximum strength
///
/// Used for:
/// - Data integrity verification
/// - Digital signatures
/// - HMAC construction
/// - Blockchain-style hashing
fn bench_hash_functions(c: &mut Criterion) {
    use ring::digest::{digest, SHA256, SHA384, SHA512};

    let mut group = c.benchmark_group("hash_functions");

    /// Test across different input sizes: 64B to 16KB
    for size in [64, 256, 1024, 4096, 16384].iter() {
        let data = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        /// SHA-256: Widely used, 128-bit security level
        group.bench_with_input(BenchmarkId::new("sha256", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA256, &data);
                black_box(result);
            });
        });

        /// SHA-384: NIST recommended, 192-bit security level
        group.bench_with_input(BenchmarkId::new("sha384", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA384, &data);
                black_box(result);
            });
        });

        /// SHA-512: Maximum security, faster on 64-bit platforms
        group.bench_with_input(BenchmarkId::new("sha512", size), size, |b, _| {
            b.iter(|| {
                let result = digest(&SHA512, &data);
                black_box(result);
            });
        });
    }

    group.finish();
}

/// Benchmark: Base64 encoding and decoding.
///
/// Base64 encodes binary data as ASCII text:
/// - 33% size overhead (3 bytes -> 4 chars)
/// - Safe for text-only channels (JSON, URLs, email)
/// - Used for: JWT tokens, email attachments, API keys
///
/// Performance depends on input size.
fn bench_base64_encoding(c: &mut Criterion) {
    use base64::{Engine, engine::general_purpose::STANDARD};

    let mut group = c.benchmark_group("base64");

    /// Test across different data sizes: 64B to 4KB
    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        /// Encoding: Binary to Base64 text
        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| {
                let result = STANDARD.encode(&data);
                black_box(result);
            });
        });

        /// Decoding: Base64 text back to binary
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

/// Benchmark: Hexadecimal encoding and decoding.
///
/// Hex encoding:
/// - 2x size overhead (1 byte -> 2 chars)
/// - Human readable
/// - Used for: debugging, checksums, UUID display
fn bench_hex_encoding(c: &mut Criterion) {
    let mut group = c.benchmark_group("hex_encoding");

    /// Test across different data sizes: 64B to 4KB
    for size in [64, 256, 1024, 4096].iter() {
        let data = vec![0u8; *size];

        group.throughput(Throughput::Bytes(*size as u64));

        /// Encoding: Binary to hex string
        group.bench_with_input(BenchmarkId::new("encode", size), size, |b, _| {
            b.iter(|| {
                let result = hex::encode(&data);
                black_box(result);
            });
        });

        /// Decoding: Hex string back to binary
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

/// Benchmark: Protocol frame parsing and building.
///
/// DMSC uses a custom framing protocol for messages:
/// - Frame header with metadata
/// - Length-prefixed payload
/// - Checksum for integrity
///
/// Frame operations are critical for:
/// - Network message parsing
/// - Stream processing
/// - Protocol compliance
fn bench_frame_parsing(c: &mut Criterion) {
    use dmsc::protocol::{DMSCFrameBuilder, DMSCFrameParser};

    let mut group = c.benchmark_group("frame_parsing");

    /// Create test frame with 256-byte payload
    let payload = vec![0u8; 256];
    let mut builder = DMSCFrameBuilder::new();
    let frame = builder.build_data_frame(payload.clone()).unwrap();
    let frame_bytes = frame.to_bytes().unwrap();

    group.throughput(Throughput::Bytes(frame_bytes.len() as u64));

    /// Parsing: Convert raw bytes to structured frame
    group.bench_function("parse_frame", |b| {
        b.iter(|| {
            let mut parser = DMSCFrameParser::new();
            parser.add_data(&frame_bytes);
            let result = parser.parse_frame().unwrap();
            black_box(result);
        });
    });

    /// Building: Convert structured frame to bytes
    group.bench_function("build_frame", |b| {
        b.iter(|| {
            let mut builder = DMSCFrameBuilder::new();
            let result = builder.build_data_frame(payload.clone()).unwrap();
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark group registration for protocol/crypto module benchmarks.
criterion_group!(
    protocol_benches,
    bench_aes_gcm_encryption,
    bench_hash_functions,
    bench_base64_encoding,
    bench_hex_encoding,
    bench_frame_parsing,
);

criterion_main!(protocol_benches);
