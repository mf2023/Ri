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
//! This benchmark suite measures the performance of DMSC core operations.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dmsc::core::error::DMSCError;
use std::sync::Arc;
use tokio::sync::RwLock;

fn bench_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_creation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("create_io_error", |b| {
        b.iter(|| {
            let error = DMSCError::IoError("Test IO error".to_string());
            black_box(error);
        });
    });
    
    group.bench_function("create_config_error", |b| {
        b.iter(|| {
            let error = DMSCError::ConfigError("Test config error".to_string());
            black_box(error);
        });
    });
    
    group.bench_function("create_database_error", |b| {
        b.iter(|| {
            let error = DMSCError::DatabaseError("Test database error".to_string());
            black_box(error);
        });
    });
    
    group.bench_function("create_validation_error", |b| {
        b.iter(|| {
            let error = DMSCError::ValidationError("Test validation error".to_string());
            black_box(error);
        });
    });
    
    group.finish();
}

fn bench_error_display(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_display");
    group.throughput(Throughput::Elements(1));
    
    let error = DMSCError::IoError("Test IO error with some longer message".to_string());
    
    group.bench_function("error_to_string", |b| {
        b.iter(|| {
            let msg = error.to_string();
            black_box(msg);
        });
    });
    
    group.finish();
}

fn bench_rwlock_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = Arc::new(RwLock::new(0u64));
    
    let mut group = c.benchmark_group("rwlock");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("read_lock", |b| {
        b.iter(|| {
            rt.block_on(async {
                let guard = data.read().await;
                let value = *guard;
                black_box(value);
            });
        });
    });
    
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

fn bench_uuid_generation(c: &mut Criterion) {
    let mut group = c.benchmark_group("uuid_generation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("generate_v4", |b| {
        b.iter(|| {
            let uuid = uuid::Uuid::new_v4();
            black_box(uuid);
        });
    });
    
    group.bench_function("uuid_to_string", |b| {
        let uuid = uuid::Uuid::new_v4();
        b.iter(|| {
            let s = uuid.to_string();
            black_box(s);
        });
    });
    
    group.bench_function("uuid_parse", |b| {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        b.iter(|| {
            let uuid = uuid::Uuid::parse_str(uuid_str).unwrap();
            black_box(uuid);
        });
    });
    
    group.finish();
}

fn bench_timestamp_operations(c: &mut Criterion) {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let mut group = c.benchmark_group("timestamp");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("system_time_now", |b| {
        b.iter(|| {
            let now = SystemTime::now();
            black_box(now);
        });
    });
    
    group.bench_function("unix_timestamp", |b| {
        b.iter(|| {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            black_box(now);
        });
    });
    
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

fn bench_json_serialization(c: &mut Criterion) {
    use serde_json::json;
    
    let mut group = c.benchmark_group("json_serialization");
    group.throughput(Throughput::Elements(1));
    
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
    
    let json_str = r#"{"id":1,"name":"test","active":true}"#;
    
    group.bench_function("deserialize_simple", |b| {
        b.iter(|| {
            let result: serde_json::Value = serde_json::from_str(json_str).unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

fn bench_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("string_format", |b| {
        b.iter(|| {
            let s = format!("key_{}_{}", 123, 456);
            black_box(s);
        });
    });
    
    group.bench_function("string_concat", |b| {
        b.iter(|| {
            let s = "prefix_".to_string() + "suffix";
            black_box(s);
        });
    });
    
    group.bench_function("string_clone", |b| {
        let original = "This is a test string for cloning".to_string();
        b.iter(|| {
            let cloned = original.clone();
            black_box(cloned);
        });
    });
    
    group.finish();
}

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
