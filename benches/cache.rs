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

//! # Cache Module Benchmarks
//!
//! This module provides performance benchmarks for the Ri cache system,
//! specifically measuring in-memory cache operations via RiMemoryCache.
//!
//! ## Benchmark Categories
//!
//! 1. **Set Operations**: Measures cache write performance with varying value sizes
//!    and TTL (time-to-live) configurations
//!
//! 2. **Get Operations**: Measures cache read performance including cache hits
//!    and misses
//!
//! 3. **Batch Operations**: Measures efficiency of bulk get/set operations
//!    for batch processing scenarios
//!
//! 4. **Exists Operations**: Measures key existence check performance
//!
//! 5. **Delete Operations**: Measures cache entry removal performance
//!
//! 6. **Stats Operations**: Measures cache statistics retrieval overhead
//!
//! ## Cache Architecture Notes
//!
//! The RiMemoryCache provides an in-memory caching layer typically used for:
//! - Session storage
//! - API response caching
//! - Computed result memoization
//! - Distributed cache local fallback
//!
//! ## Test Methodology
//!
//! - Each benchmark creates its own cache instance to avoid cross-contamination
//! - Pre-population is done outside benchmark loops where applicable
//! - Async operations are executed using tokio runtime

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ri::cache::{RiMemoryCache, RiCache};

/// Benchmark: Cache SET operations with varying data sizes.
///
/// SET operations are write-heavy and typically happen:
/// - On cache misses (lazy loading)
/// - After cache invalidation
/// - For caching new data
///
/// This measures how value size affects write performance.
fn bench_cache_set(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_set");
    group.throughput(Throughput::Elements(1));

    /// Small value: Typical for flags, counters, small tokens
    /// Size: ~5 bytes
    group.bench_function("set_small_value", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let cache = RiMemoryCache::new();
        b.iter(|| {
            rt.block_on(async {
                cache.set("key", "value", None).await.unwrap();
                black_box(());
            });
        });
    });

    /// Medium value: Typical for JSON responses, serialized objects
    /// Size: ~100 bytes
    group.bench_function("set_medium_value", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let cache = RiMemoryCache::new();
        let medium_value = "x".repeat(100);
        b.iter(|| {
            rt.block_on(async {
                cache.set("key", &medium_value, None).await.unwrap();
                black_box(());
            });
        });
    });

    /// Large value: Typical for file contents, large documents
    /// Size: ~10KB
    group.bench_function("set_large_value", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let cache = RiMemoryCache::new();
        let large_value = "x".repeat(10000);
        b.iter(|| {
            rt.block_on(async {
                cache.set("key", &large_value, None).await.unwrap();
                black_box(());
            });
        });
    });

    /// With TTL: Cache entries that expire after a time period
    /// Important for session management and temporary caching
    group.bench_function("set_with_ttl", |b| {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let cache = RiMemoryCache::new();
        b.iter(|| {
            rt.block_on(async {
                cache.set("key", "value", Some(3600)).await.unwrap();
                black_box(());
            });
        });
    });

    group.finish();
}

/// Benchmark: Cache GET operations for hit and miss scenarios.
///
/// GET operations are read-heavy and called on every cache lookup:
/// - Cache hits return data immediately (fast path)
/// - Cache misses trigger fallback (slow path)
///
/// Understanding hit/miss ratio is critical for cache efficiency.
fn bench_cache_get(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = RiMemoryCache::new();

    /// Pre-populate cache with 1000 entries
    /// This simulates a warmed-up cache state
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });

    let mut group = c.benchmark_group("cache_get");
    group.throughput(Throughput::Elements(1));

    /// Cache hit: Key exists in cache
    /// Expected to be very fast (direct hash map lookup)
    group.bench_function("get_hit", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = cache.get("key_500").await.unwrap();
                black_box(result);
            });
        });
    });

    /// Cache miss: Key does not exist
    /// Returns None/empty, but still performs hash lookup
    group.bench_function("get_miss", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = cache.get("nonexistent_key").await.unwrap();
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Benchmark: Batch GET and SET operations.
///
/// Batch operations are optimized for bulk data access:
/// - reduce round trips
/// - improve throughput for bulk processing
/// - Better network efficiency in distributed scenarios
fn bench_cache_batch_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = RiMemoryCache::new();

    let mut group = c.benchmark_group("cache_batch");

    /// Test with different batch sizes: 10, 100, 1000
    /// This reveals scaling characteristics of batch operations
    for size in [10, 100, 1000].iter() {
        let keys: Vec<String> = (0..*size).map(|i| format!("batch_key_{}", i)).collect();
        let key_refs: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
        let items: Vec<(&str, &str)> = (0..*size).map(|i| {
            let s = format!("batch_key_{}", i);
            (Box::leak(s.into_boxed_str()) as &str, "value")
        }).collect();

        group.throughput(Throughput::Elements(*size as u64));

        /// Batch GET: Retrieve multiple keys in single operation
        group.bench_with_input(BenchmarkId::new("get_multi", size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async {
                    let result = cache.get_multi(&key_refs).await.unwrap();
                    black_box(result);
                });
            });
        });

        /// Batch SET: Store multiple key-value pairs in single operation
        group.bench_with_input(BenchmarkId::new("set_multi", size), size, |b, _| {
            b.iter(|| {
                rt.block_on(async {
                    cache.set_multi(&items, None).await.unwrap();
                    black_box(());
                });
            });
        });
    }

    group.finish();
}

/// Benchmark: Cache key existence checks.
///
/// EXISTS operations are used for:
/// - Cache warming verification
/// - Conditional writes
/// - Cache coherence checks
fn bench_cache_exists(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = RiMemoryCache::new();

    /// Pre-populate cache
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("exists_key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });

    let mut group = c.benchmark_group("cache_exists");
    group.throughput(Throughput::Elements(1));

    /// Key exists: Fast hash lookup returning true
    group.bench_function("exists_true", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = cache.exists("exists_key_500").await;
                black_box(result);
            });
        });
    });

    /// Key does not exist: Fast hash lookup returning false
    group.bench_function("exists_false", |b| {
        b.iter(|| {
            rt.block_on(async {
                let result = cache.exists("nonexistent_key").await;
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Benchmark: Cache DELETE operations.
///
/// DELETE operations are used for:
/// - Cache invalidation
/// - Memory pressure handling
/// - Explicit cache cleanup
///
/// Note: Each iteration creates a new cache to ensure delete operations
/// have a target to delete.
fn bench_cache_delete(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    let mut group = c.benchmark_group("cache_delete");
    group.throughput(Throughput::Elements(1));

    /// Delete existing key: Should succeed and return true
    group.bench_function("delete_existing", |b| {
        b.iter(|| {
            rt.block_on(async {
                let cache = RiMemoryCache::new();
                cache.set("delete_key", "value", None).await.unwrap();
                let result = cache.delete("delete_key").await.unwrap();
                black_box(result);
            });
        });
    });

    /// Delete non-existent key: Should return false/None
    group.bench_function("delete_nonexistent", |b| {
        let cache = RiMemoryCache::new();
        b.iter(|| {
            rt.block_on(async {
                let result = cache.delete("nonexistent_key").await.unwrap();
                black_box(result);
            });
        });
    });

    group.finish();
}

/// Benchmark: Cache statistics retrieval.
///
/// Stats provide visibility into cache behavior:
/// - Hit/miss ratios
/// - Memory usage
/// - Entry counts
///
/// Typically called for monitoring/observability purposes.
fn bench_cache_stats(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let cache = RiMemoryCache::new();

    /// Pre-populate cache
    rt.block_on(async {
        for i in 0..1000 {
            cache.set(&format!("stats_key_{}", i), &format!("value_{}", i), None).await.unwrap();
        }
    });

    let mut group = c.benchmark_group("cache_stats");
    group.throughput(Throughput::Elements(1));

    group.bench_function("get_stats", |b| {
        b.iter(|| {
            rt.block_on(async {
                let stats = cache.stats().await;
                black_box(stats);
            });
        });
    });

    group.finish();
}

/// Benchmark group registration for cache benchmarks.
criterion_group!(
    cache_benches,
    bench_cache_set,
    bench_cache_get,
    bench_cache_batch_operations,
    bench_cache_exists,
    bench_cache_delete,
    bench_cache_stats,
);

criterion_main!(cache_benches);
