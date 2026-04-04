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
//! This module provides performance benchmarks for the DMSC authentication system,
//! specifically measuring JWT (JSON Web Token) operations provided by DMSCJWTManager.
//!
//! ## Benchmark Categories
//!
//! 1. **Token Generation**: Measures the performance of creating new JWT tokens
//!    with varying complexity (no roles, with roles, many permissions)
//!
//! 2. **Token Validation**: Measures the performance of verifying and parsing
//!    existing JWT tokens to extract claims
//!
//! 3. **Round-trip Operations**: Measures the complete cycle of generating a token
//!    and then validating it
//!
//! 4. **Manager Creation**: Measures the overhead of creating a new JWT manager
//!    instance with different configurations
//!
//! ## Usage
//!
//! Run these benchmarks with:
//! ```bash
//! cargo bench --bench auth
//! ```
//!
//! ## Understanding Results
//!
//! Key metrics to observe:
//! - `jwt_generation`: Higher is better (tokens/second)
//! - `jwt_validation`: Higher is better (validations/second)
//! - `jwt_round_trip`: Combined generation + validation throughput
//!
//! ## Security Notes
//!
//! These benchmarks use a fixed secret key for consistency. In production,
//! secret keys should be:
//! - At least 256 bits (32 bytes) for HS256
//! - Generated using cryptographically secure random number generators
//! - Stored securely (environment variables, secrets management)

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dmsc::auth::DMSCJWTManager;

/// Benchmarks for JWT token generation operations.
///
/// Token generation is typically called during:
/// - User login/authentication
/// - Token refresh operations
/// - Session creation
///
/// The complexity of token generation scales with:
/// - Number of roles attached to the token
/// - Number of permissions/claims included
fn bench_jwt_token_generation(c: &mut Criterion) {
    let manager = DMSCJWTManager::create("benchmark_secret_key_12345".to_string(), 3600);

    let mut group = c.benchmark_group("jwt_generation");
    group.throughput(Throughput::Elements(1));

    /// Benchmark: Generate token without roles or permissions
    /// This represents the simplest case - just user authentication
    group.bench_function("generate_token_no_roles", |b| {
        b.iter(|| {
            let token = manager.generate_token("user123", vec![], vec![]).unwrap();
            black_box(token);
        });
    });

    /// Benchmark: Generate token with multiple roles and permissions
    /// This represents a typical authenticated user with role-based access
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

    /// Benchmark: Generate token with many permissions (50)
    /// This stress-tests token generation for complex access control scenarios
    /// May be relevant for:
    /// - Fine-grained permission systems
    /// - Token caching scenarios
    /// - Systems with many resource-level permissions
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

/// Benchmarks for JWT token validation operations.
///
/// Token validation is called on every authenticated request to:
/// - Verify token signature authenticity
/// - Check token expiration
/// - Extract user claims/permissions
///
/// Validation is typically more frequent than generation in production systems.
fn bench_jwt_token_validation(c: &mut Criterion) {
    let manager = DMSCJWTManager::create("benchmark_secret_key_12345".to_string(), 3600);

    /// Pre-generate tokens outside the benchmark loop to isolate validation cost
    let simple_token = manager.generate_token("user123", vec![], vec![]).unwrap();
    let roles_token = manager.generate_token(
        "user123",
        vec!["admin".to_string(), "user".to_string()],
        vec!["read".to_string(), "write".to_string()],
    ).unwrap();

    let mut group = c.benchmark_group("jwt_validation");
    group.throughput(Throughput::Elements(1));

    /// Benchmark: Validate a simple token with minimal claims
    group.bench_function("validate_simple_token", |b| {
        b.iter(|| {
            let claims = manager.validate_token(&simple_token).unwrap();
            black_box(claims);
        });
    });

    /// Benchmark: Validate a token containing roles and permissions
    group.bench_function("validate_token_with_roles", |b| {
        b.iter(|| {
            let claims = manager.validate_token(&roles_token).unwrap();
            black_box(claims);
        });
    });

    group.finish();
}

/// Benchmarks for complete generate-and-validate round-trip operations.
///
/// This measures the full authentication cycle:
/// 1. Server generates token for authenticated user
/// 2. Client presents token on subsequent request
/// 3. Server validates token and extracts claims
///
/// This represents a real-world authenticated request flow.
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

/// Benchmarks for JWT manager instance creation.
///
/// Manager creation involves:
/// - Secret key parsing and validation
/// - Internal state initialization
/// - Potential cryptographic context setup
///
/// While typically done once at application startup, this measures
/// the overhead for scenarios like:
/// - Per-request token validation contexts
/// - Multi-tenant applications with per-tenant keys
/// - Testing scenarios
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

/// Benchmark group registration for authentication benchmarks.
///
/// Groups are registered with criterion using the `criterion_group!` macro.
/// Each group is named and contains related benchmark functions.
///
/// Run order is determined by criterion, not by declaration order.
criterion_group!(
    auth_benches,
    bench_jwt_token_generation,
    bench_jwt_token_validation,
    bench_jwt_round_trip,
    bench_jwt_manager_creation,
);

/// Main entry point for criterion benchmark runner.
///
/// `criterion_main!` sets up:
/// - Signal handlers for graceful shutdown
/// - Statistical analysis of benchmark results
/// - Report formatting (HTML, JSON, CSV)
criterion_main!(auth_benches);
