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

//! # Cache Module Benchmarks
//!
//! This benchmark suite measures the performance of DMSC cache operations.
//! It tests various cache backends and operations including:
//! - Memory cache get/set operations
//! - Memory cache batch operations
//! - Cache hit/miss scenarios
//! - TTL-based operations
//! - LRU eviction patterns
//!
//! ## Running Benchmarks
//!
//! ```bash
//! cargo bench --bench cache_benchmark
//! ```

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dmsc::cache::backends::DMSCMemoryCache;
use dmsc::cache::DMSCCache;

fn bench_cache_set(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    let mut group = c.benchmark_group("cache_set");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("set_small_value", |b| {
        b.to_async(&rt).iter(|| async {
            cache.set("key", "value", None).await.unwrap();
            black_box(());
        });
    });
    
    group.bench_function("set_medium_value", |b| {
        let medium_value = "x".repeat(100);
        b.to_async(&rt).iter(|| async {
            cache.set("key", &medium_value, None).await.unwrap();
            black_box(());
        });
    });
    
    group.bench_function("set_large_value", |b| {
        let large_value = "x".repeat(10000);
        b.to_async(&rt).iter(|| async {
            cache.set("key", &large_value, None).await.unwrap();
            black_box(());
        });
    });
    
    group.bench_function("set_with_ttl", |b| {
        b.to_async(&rt).iter(|| async {
            cache.set("key", "value", Some(3600)).await.unwrap();
            black_box(());
        });
    });
    
    group.finish();
}

fn bench_cache_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("cache_get");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("get_hit", |b| {
        b.to_async(&rt).iter(|| async {
            let result = cache.get("key_500").await.unwrap();
            black_box(result);
        });
    });
    
    group.bench_function("get_miss", |b| {
        b.to_async(&rt).iter(|| async {
            let result = cache.get("nonexistent_key").await.unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

fn bench_cache_batch_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    let mut group = c.benchmark_group("cache_batch");
    
    for size in [10, 100, 1000].iter() {
        let keys: Vec<String> = (0..*size).map(|i| format!("batch_key_{}", i)).collect();
        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        let items: Vec<(&str, &str)> = (0..*size).map(|i| {
            (format!("batch_key_{}", i).as_str(), "value")
        }).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        group.bench_with_input(BenchmarkId::new("get_multi", size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                let result = cache.get_multi(&key_refs).await.unwrap();
                black_box(result);
            });
        });
        
        group.bench_with_input(BenchmarkId::new("set_multi", size), size, |b, _| {
            b.to_async(&rt).iter(|| async {
                cache.set_multi(&items, None).await.unwrap();
                black_box(());
            });
        });
    }
    
    group.finish();
}

fn bench_cache_exists(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("exists_key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("cache_exists");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("exists_true", |b| {
        b.to_async(&rt).iter(|| async {
            let result = cache.exists("exists_key_500").await;
            black_box(result);
        });
    });
    
    group.bench_function("exists_false", |b| {
        b.to_async(&rt).iter(|| async {
            let result = cache.exists("nonexistent_key").await;
            black_box(result);
        });
    });
    
    group.finish();
}

fn bench_cache_delete(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    let mut group = c.benchmark_group("cache_delete");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("delete_existing", |b| {
        b.to_async(&rt).iter(|| async {
            let cache = DMSCMemoryCache::new();
            cache.set("delete_key", "value", None).await.unwrap();
            let result = cache.delete("delete_key").await.unwrap();
            black_box(result);
        });
    });
    
    group.bench_function("delete_nonexistent", |b| {
        let cache = DMSCMemoryCache::new();
        b.to_async(&rt).iter(|| async {
            let result = cache.delete("nonexistent_key").await.unwrap();
            black_box(result);
        });
    });
    
    group.finish();
}

fn bench_cache_concurrent_access(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    rt.block_on(async {
        for i in 0..10000 {
            cache.set(&format!("concurrent_key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("cache_concurrent");
    
    group.throughput(Throughput::Elements(100));
    
    group.bench_function("mixed_read_write", |b| {
        b.to_async(&rt).iter(|| async {
            let mut tasks = Vec::new();
            for i in 0..50 {
                let cache_ref = &cache;
                tasks.push(async move {
                    cache_ref.get(&format!("concurrent_key_{}", i)).await.unwrap();
                    cache_ref.set(&format!("new_key_{}", i), "new_value", None).await.unwrap();
                });
            }
            futures::future::join_all(tasks).await;
            black_box(());
        });
    });
    
    group.finish();
}

fn bench_cache_stats(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = DMSCMemoryCache::new();
    
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("stats_key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("cache_stats");
    
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("get_stats", |b| {
        b.to_async(&rt).iter(|| async {
            let stats = cache.stats().await;
            black_box(stats);
        });
    });
    
    group.finish();
}

criterion_group!(
    cache_benches,
    bench_cache_set,
    bench_cache_get,
    bench_cache_batch_operations,
    bench_cache_exists,
    bench_cache_delete,
    bench_cache_concurrent_access,
    bench_cache_stats,
);

criterion_main!(cache_benches);
