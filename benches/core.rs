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

//! # Core Module Benchmarks
//!
//! This module provides performance benchmarks for fundamental DMSC operations
//! including error handling, concurrency primitives, UUID generation, and
//! serialization utilities.
//!
//! ## Benchmark Categories
//!
//! 1. **Error Handling**: Measures error type creation and display overhead
//!
//! 2. **Concurrency Primitives**: Measures RwLock read/write performance
//!
//! 3. **UUID Operations**: Measures UUID generation, string conversion, and parsing
//!
//! 4. **Timestamp Operations**: Measures system time retrieval and conversion
//!
//! 5. **JSON Serialization**: Measures JSON encoding/decoding performance
//!
//! 6. **String Operations**: Measures common string manipulation performance
//!
//! ## Testing Methodology
//!
//! - Uses criterion for statistical analysis
//! - Measures throughput (operations/second) for all benchmarks
//! - Black-boxes results to prevent compiler optimization

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dmsc::core::error::DMSCError;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Benchmark: Error type creation across different error variants.
///
/// DMSCError is the central error type for the framework. Different
/// error variants may have different creation costs due to:
/// - String allocation
/// - Error chain construction
/// - Context capture
fn bench_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_creation");
    group.throughput(Throughput::Elements(1));

    /// IO errors: File, network, system resource errors
    group.bench_function("create_io_error", |b| {
        b.iter(|| {
            let error = DMSCError::io("Test IO error");
            black_box(error);
        });
    });

    /// Configuration errors: Invalid config, missing fields
    group.bench_function("create_config_error", |b| {
        b.iter(|| {
            let error = DMSCError::config("Test config error");
            black_box(error);
        });
    });

    /// Generic errors: Catch-all for other error types
    group.bench_function("create_database_error", |b| {
        b.iter(|| {
            let error = DMSCError::Other("Test database error".to_string());
            black_box(error);
        });
    });

    /// Validation errors: Input validation failures
    group.bench_function("create_validation_error", |b| {
        b.iter(|| {
            let error = DMSCError::InvalidInput("Test validation error".to_string());
            black_box(error);
        });
    });

    group.finish();
}

/// Benchmark: Error display (to_string) conversion.
///
/// Error display is called when:
/// - Logging errors
/// - Converting errors to API responses
/// - Error message propagation
fn bench_error_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_display");
    group.throughput(Throughput::Elements(1));

    let error = DMSCError::io("Test IO error with some longer message");

    group.bench_function("error_to_string", |b| {
        b.iter(|| {
            let msg = error.to_string();
            black_box(msg);
        });
    });

    group.finish();
}

/// Benchmark: Async RwLock operations for concurrent data access.
///
/// RwLock allows multiple readers or single writer:
/// - Read operations can proceed concurrently
/// - Write operations require exclusive access
///
/// DMSC uses RwLock for:
/// - Shared configuration state
/// - Connection pool management
/// - Cache access synchronization
fn bench_rwlock_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = Arc::new(RwLock::new(0u64));

    let mut group = c.benchmark_group("rwlock");
    group.throughput(Throughput::Elements(1));

    /// Read lock: Multiple readers can hold simultaneously
    /// Used for: Config reads, stats access
    group.bench_function("read_lock", |b| {
        b.iter(|| {
            rt.block_on(async {
                let guard = data.read().await;
                let value = *guard;
                black_box(value);
            });
        });
    });

    /// Write lock: Exclusive access required
    /// Used for: Config updates, state mutations
    group.bench_function("write_lock", |b| {
        b.iter(|| {
            rt.block_on(async {
                let mut guard = data.write().await;
                *guard += 1;
                black_box(());
            });
        });
    });

    group.finish();
}

/// Benchmark: UUID v4 generation and manipulation.
///
/// UUIDs are used in DMSC for:
/// - Request tracing/correlation IDs
/// - Unique identifier generation
/// - Distributed ID generation
///
/// UUID v4 uses cryptographically random data.
fn bench_uuid_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuid_generation");
    group.throughput(Throughput::Elements(1));

    /// Generate new UUID: Requires random number generation
    group.bench_function("generate_v4", |b| {
        b.iter(|| {
            let uuid = uuid::Uuid::new_v4();
            black_box(uuid);
        });
    });

    /// UUID to string: Converts 16-byte UUID to 36-char string
    group.bench_function("uuid_to_string", |b| {
        let uuid = uuid::Uuid::new_v4();
        b.iter(|| {
            let s = uuid.to_string();
            black_box(s);
        });
    });

    /// Parse UUID from string: Converts string back to UUID
    group.bench_function("uuid_parse", |b| {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        b.iter(|| {
            let uuid = uuid::Uuid::parse_str(uuid_str).unwrap();
            black_box(uuid);
        });
    });

    group.finish();
}

/// Benchmark: System time retrieval and conversion.
///
/// Time operations are used for:
/// - Request timestamping
/// - Rate limiting calculations
/// - TTL expiration checks
/// - Audit logging
fn bench_timestamp_operations(c: &mut Criterion) {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut group = c.benchmark_group("timestamp");
    group.throughput(Throughput::Elements(1));

    /// Get current system time: Wall clock time retrieval
    group.bench_function("system_time_now", |b| {
        b.iter(|| {
            let now = SystemTime::now();
            black_box(now);
        });
    });

    /// Unix timestamp in seconds: Seconds since epoch (1970-01-01)
    group.bench_function("unix_timestamp", |b| {
        b.iter(|| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            black_box(now);
        });
    });

    /// Unix timestamp in milliseconds: Higher precision for timing
    group.bench_function("unix_timestamp_millis", |b| {
        b.iter(|| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            black_box(now);
        });
    });

    group.finish();
}

/// Benchmark: JSON serialization and deserialization.
///
/// JSON is used for:
/// - API request/response bodies
/// - Configuration files
/// - Inter-service communication
///
/// serde_json is the standard JSON library for Rust.
fn bench_json_serialization(c: &mut Criterion) {
    use serde_json::json;

    let mut group = c.benchmark_group("json_serialization");
    group.throughput(Throughput::Elements(1));

    /// Serialize simple object: Few fields, basic types
    group.bench_function("serialize_simple", |b| {
        let data = json!({
            "id": 1,
            "name": "test",
            "active": true
        });
        b.iter(|| {
            let result = serde_json::to_string(&data).unwrap();
            black_box(result);
        });
    });

    /// Serialize complex object: Nested structures, arrays
    group.bench_function("serialize_complex", |b| {
        let data = json!({
            "users": (0..100).map(|i| {
                json!({
                    "id": i,
                    "name": format!("user_{}", i),
                    "roles": ["admin", "user"],
                    "metadata": {
                        "created": "2024-01-01",
                        "updated": "2024-01-02"
                    }
                })
            }).collect::<Vec<_>>()
        });
        b.iter(|| {
            let result = serde_json::to_string(&data).unwrap();
            black_box(result);
        });
    });

    /// Deserialize simple JSON string
    group.bench_function("deserialize_simple", |b| {
        let json_str = r#"{"id":1,"name":"test","active":true}"#;
        b.iter(|| {
            let result: serde_json::Value = serde_json::from_str(json_str).unwrap();
            black_box(result);
        });
    });

    group.finish();
}

/// Benchmark: Common string operations.
///
/// String operations are fundamental to text processing:
/// - Formatting for logging
/// - Path construction
/// - Text manipulation
fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    group.throughput(Throughput::Elements(1));

    /// Format macro: Combines literals and variables
    group.bench_function("string_format", |b| {
        b.iter(|| {
            let s = format!("key_{}_{}", 123, 456);
            black_box(s);
        });
    });

    /// String concatenation: Combining strings
    group.bench_function("string_concat", |b| {
        b.iter(|| {
            let s = "prefix_".to_string() + "suffix";
            black_box(s);
        });
    });

    /// String cloning: Reference counting vs deep copy
    group.bench_function("string_clone", |b| {
        let original = "This is a test string for cloning".to_string();
        b.iter(|| {
            let cloned = original.clone();
            black_box(cloned);
        });
    });

    group.finish();
}

/// Benchmark group registration for core module benchmarks.
criterion_group!(
    core_benches,
    bench_error_creation,
    bench_error_display,
    bench_rwlock_operations,
    bench_uuid_generation,
    bench_timestamp_operations,
    bench_json_serialization,
    bench_string_operations,
);

criterion_main!(core_benches);
