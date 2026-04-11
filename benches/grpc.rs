//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

//! # gRPC Module Benchmarks
//!
//! This module provides performance benchmarks for the Ri gRPC subsystem,
//! measuring the creation and cloning operations of core gRPC components.
//!
//! ## Benchmark Categories
//!
//! 1. **Client Operations**: gRPC client creation and cloning
//!
//! 2. **Server Operations**: gRPC server creation and cloning
//!
//! 3. **Statistics**: gRPC stats tracking object overhead
//!
//! 4. **Configuration**: gRPC config object creation
//!
//! ## Feature Flag
//!
//! These benchmarks are conditionally compiled based on the `grpc` feature flag.
//! They will only run when the `grpc` feature is enabled in Cargo.toml.
//!
//! ## gRPC Architecture
//!
//! Ri's gRPC support provides:
//! - High-performance RPC communication
//! - Protocol Buffer serialization
//! - Streaming support
//! - Interceptors for middleware
//!
//! ## Testing Notes
//!
//! Benchmarks measure object allocation and cloning overhead.
//! Actual RPC performance depends on network and serialization costs.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

#[cfg(feature = "grpc")]
fn bench_grpc_client_creation(c: &mut Criterion) {
    use ri::grpc::RiGrpcClient;

    let mut group = c.benchmark_group("grpc_client");
    group.throughput(Throughput::Elements(1));

    /// Client creation: Creates new gRPC client instance
    /// Involves channel creation and connection setup
    group.bench_function("client_creation", |b| {
        b.iter(|| {
            let client = RiGrpcClient::default();
            black_box(client);
        });
    });

    /// Client cloning: Shares underlying channel
    /// In gRPC, cloning is typically cheap (reference counted)
    group.bench_function("client_clone", |b| {
        let client = RiGrpcClient::default();
        b.iter(|| {
            let cloned = client.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

#[cfg(not(feature = "grpc"))]
fn bench_grpc_client_creation(_c: &mut Criterion) {}

#[cfg(feature = "grpc")]
fn bench_grpc_server_creation(c: &mut Criterion) {
    use ri::grpc::{RiGrpcServer, RiGrpcConfig};

    let mut group = c.benchmark_group("grpc_server");
    group.throughput(Throughput::Elements(1));

    /// Server creation: Allocates server instance with default config
    group.bench_function("server_creation", |b| {
        b.iter(|| {
            let config = RiGrpcConfig::default();
            let server = RiGrpcServer::default();
            black_box((server, config));
        });
    });

    /// Server cloning: Shares underlying listener/port
    group.bench_function("server_clone", |b| {
        let server = RiGrpcServer::default();
        b.iter(|| {
            let cloned = server.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

#[cfg(not(feature = "grpc"))]
fn bench_grpc_server_creation(_c: &mut Criterion) {}

#[cfg(feature = "grpc")]
fn bench_grpc_stats(c: &mut Criterion) {
    use ri::grpc::RiGrpcStats;

    let mut group = c.benchmark_group("grpc_stats");
    group.throughput(Throughput::Elements(1));

    /// Stats creation: Initializes counters and state tracking
    group.bench_function("stats_creation", |b| {
        b.iter(|| {
            let stats = RiGrpcStats::new();
            black_box(stats);
        });
    });

    /// Stats cloning: Shares underlying counters
    /// Used for passing stats to multiple components
    group.bench_function("stats_clone", |b| {
        let stats = RiGrpcStats::new();
        b.iter(|| {
            let cloned = stats.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

#[cfg(not(feature = "grpc"))]
fn bench_grpc_stats(_c: &mut Criterion) {}

#[cfg(feature = "grpc")]
fn bench_grpc_config(c: &mut Criterion) {
    use ri::grpc::RiGrpcConfig;

    let mut group = c.benchmark_group("grpc_config");
    group.throughput(Throughput::Elements(1));

    /// Config creation: Default configuration values
    /// Typically includes timeouts, buffer sizes, etc.
    group.bench_function("config_creation", |b| {
        b.iter(|| {
            let config = RiGrpcConfig::default();
            black_box(config);
        });
    });

    /// Config cloning: Deep copies configuration
    group.bench_function("config_clone", |b| {
        let config = RiGrpcConfig::default();
        b.iter(|| {
            let cloned = config.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

#[cfg(not(feature = "grpc"))]
fn bench_grpc_config(_c: &mut Criterion) {}

/// Benchmark group registration for gRPC module benchmarks.
///
/// Note: When `grpc` feature is disabled, these benchmarks become no-ops.
/// This allows the benchmark suite to compile without the grpc dependency.
criterion_group!(
    grpc_benches,
    bench_grpc_client_creation,
    bench_grpc_server_creation,
    bench_grpc_stats,
    bench_grpc_config,
);

criterion_main!(grpc_benches);
