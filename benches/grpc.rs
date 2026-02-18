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

//! # gRPC Module Benchmarks
//!
//! This benchmark suite measures the performance of DMSC gRPC operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};

#[cfg(feature = "grpc")]
fn bench_grpc_client_creation(c: &mut Criterion) {
    use dmsc::grpc::DMSCGrpcClient;
    
    let mut group = c.benchmark_group("grpc_client");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("client_creation", |b| {
        b.iter(|| {
            let client = DMSCGrpcClient::default();
            black_box(client);
        });
    });
    
    group.bench_function("client_clone", |b| {
        let client = DMSCGrpcClient::default();
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
    use dmsc::grpc::{DMSCGrpcServer, DMSCGrpcConfig};
    
    let mut group = c.benchmark_group("grpc_server");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("server_creation", |b| {
        b.iter(|| {
            let config = DMSCGrpcConfig::default();
            let server = DMSCGrpcServer::default();
            black_box((server, config));
        });
    });
    
    group.bench_function("server_clone", |b| {
        let server = DMSCGrpcServer::default();
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
    use dmsc::grpc::DMSCGrpcStats;
    
    let mut group = c.benchmark_group("grpc_stats");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("stats_creation", |b| {
        b.iter(|| {
            let stats = DMSCGrpcStats::new();
            black_box(stats);
        });
    });
    
    group.bench_function("stats_clone", |b| {
        let stats = DMSCGrpcStats::new();
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
    use dmsc::grpc::DMSCGrpcConfig;
    
    let mut group = c.benchmark_group("grpc_config");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("config_creation", |b| {
        b.iter(|| {
            let config = DMSCGrpcConfig::default();
            black_box(config);
        });
    });
    
    group.bench_function("config_clone", |b| {
        let config = DMSCGrpcConfig::default();
        b.iter(|| {
            let cloned = config.clone();
            black_box(cloned);
        });
    });
    
    group.finish();
}

#[cfg(not(feature = "grpc"))]
fn bench_grpc_config(_c: &mut Criterion) {}

criterion_group!(
    grpc_benches,
    bench_grpc_client_creation,
    bench_grpc_server_creation,
    bench_grpc_stats,
    bench_grpc_config,
);

criterion_main!(grpc_benches);
