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

//! # Authentication Module Benchmarks
//!
//! This benchmark suite measures the performance of DMSC authentication operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dmsc::auth::DMSCJWTManager;

fn bench_jwt_token_generation(c: &mut Criterion) {
    let manager = DMSCJWTManager::create("benchmark_secret_key_12345".to_string(), 3600);
    
    let mut group = c.benchmark_group("jwt_generation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("generate_token_no_roles", |b| {
        b.iter(|| {
            let token = manager.generate_token("user123", vec![], vec![]).unwrap();
            black_box(token);
        });
    });
    
    group.bench_function("generate_token_with_roles", |b| {
        b.iter(|| {
            let token = manager.generate_token(
                "user123",
                vec!["admin".to_string(), "user".to_string()],
                vec!["read".to_string(), "write".to_string()],
            ).unwrap();
            black_box(token);
        });
    });
    
    group.bench_function("generate_token_many_permissions", |b| {
        b.iter(|| {
            let permissions: Vec<String> = (0..50).map(|i| format!("perm_{}", i)).collect();
            let token = manager.generate_token(
                "user123",
                vec!["admin".to_string()],
                permissions,
            ).unwrap();
            black_box(token);
        });
    });
    
    group.finish();
}

fn bench_jwt_token_validation(c: &mut Criterion) {
    let manager = DMSCJWTManager::create("benchmark_secret_key_12345".to_string(), 3600);
    
    let simple_token = manager.generate_token("user123", vec![], vec![]).unwrap();
    let roles_token = manager.generate_token(
        "user123",
        vec!["admin".to_string(), "user".to_string()],
        vec!["read".to_string(), "write".to_string()],
    ).unwrap();
    
    let mut group = c.benchmark_group("jwt_validation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("validate_simple_token", |b| {
        b.iter(|| {
            let claims = manager.validate_token(&simple_token).unwrap();
            black_box(claims);
        });
    });
    
    group.bench_function("validate_token_with_roles", |b| {
        b.iter(|| {
            let claims = manager.validate_token(&roles_token).unwrap();
            black_box(claims);
        });
    });
    
    group.finish();
}

fn bench_jwt_round_trip(c: &mut Criterion) {
    let manager = DMSCJWTManager::create("benchmark_secret_key_12345".to_string(), 3600);
    
    let mut group = c.benchmark_group("jwt_round_trip");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("generate_and_validate", |b| {
        b.iter(|| {
            let token = manager.generate_token("user123", vec!["admin".to_string()], vec!["read".to_string()]).unwrap();
            let claims = manager.validate_token(&token).unwrap();
            black_box(claims);
        });
    });
    
    group.finish();
}

fn bench_jwt_manager_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("jwt_manager_creation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("create_manager", |b| {
        b.iter(|| {
            let manager = DMSCJWTManager::create("secret_key".to_string(), 3600);
            black_box(manager);
        });
    });
    
    group.finish();
}

criterion_group!(
    auth_benches,
    bench_jwt_token_generation,
    bench_jwt_token_validation,
    bench_jwt_round_trip,
    bench_jwt_manager_creation,
);

criterion_main!(auth_benches);
