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

//! # Cache Module C API
//!
//! This module provides C language bindings for Ri's caching subsystem. The cache module
//! delivers high-performance in-memory data caching capabilities for accelerating application
//! performance, reducing database load, and improving system throughput. This C API enables
//! C/C++ applications to leverage Ri's sophisticated caching infrastructure including memory
//! caching, distributed caching support, and intelligent cache eviction policies.
//!
//! ## Module Architecture
//!
//! The caching module comprises three primary components:
//!
//! - **RiCacheConfig**: Configuration container for cache system parameters. Controls cache
//!   size limits, eviction policies, expiration timeouts, and connection settings for
//!   distributed cache backends. The configuration object is essential for initializing
//!   cache managers with appropriate resource limits and behavior characteristics.
//!
//! - **RiCacheManager**: Central cache management interface providing unified operations
//!   across different cache backends. Handles cache lifecycle, backend selection, and
//!   provides high-level cache operations including get, set, delete, and invalidation.
//!   The cache manager supports automatic serialization of complex types and provides
//!   consistent API regardless of underlying storage implementation.
//!
//! - **RiMemoryCache**: In-memory cache implementation using concurrent data structures.
//!   Provides thread-safe caching with O(1) average-case operations for read and write.
//!   The memory cache implements sophisticated eviction policies to manage memory usage
//!   and prevent unbounded growth. Ideal for single-instance deployments or as a
//!   local cache tier in multi-level caching architectures.
//!
//! ## Cache Strategies
//!
//! The caching system implements multiple strategies optimized for different use cases:
//!
//! - **LRU (Least Recently Used)**: Evicts least recently accessed items when capacity
//!   is reached. Optimal for workloads with temporal locality where recently accessed
//!   items are likely to be accessed again. Memory-efficient implementation using
//!   linked hash map for O(1) access and eviction.
//!
//! - **LFU (Least Frequently Used)**: Evicts items with lowest access frequency.
//!   Suitable for workloads where access frequency correlates with importance.
//!   Maintains frequency counters for eviction decisions. More computationally
//!   expensive than LRU but provides better hit rates for certain access patterns.
//!
//! - **TTL-Based Expiration**: Automatic expiration based on time-to-live values.
//!   Each cache entry has associated expiration timestamp. Entries are lazily
//!   removed during access or via background cleanup tasks. Ensures data freshness
//!   for time-sensitive cached content.
//!
//! - **Write-Through/Write-Behind**: Cache synchronization strategies for persistent
//!   backends. Write-through updates cache and backend simultaneously. Write-behind
//!   queues writes for batch processing improving write throughput.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management responsibilities:
//!
//! - Objects must be allocated using constructor functions
//! - Destructor functions must be called to release memory
//! - Null pointer checks required before all operations
//! - Double-free prevention is caller's responsibility
//!
//! ## Thread Safety
//!
//! All underlying implementations provide thread-safe concurrent access:
//!
//! - Memory cache uses fine-grained locking or lock-free data structures
//! - Operations achieve high throughput under concurrent load
//! - C API itself requires external synchronization for multi-threaded access
//!
//! ## Performance Characteristics
//!
//! Cache operations have the following performance profiles:
//!
//! - Cache hit (memory): O(1) average, O(n) worst case for hash collisions
//! - Cache miss: O(1) plus backend fetch latency
//! - Cache write: O(1) amortized
//! - Eviction: O(1) for LRU, O(log n) for LFU
//!
//! ## Integration with Distributed Systems
//!
//! The cache module supports integration with distributed cache backends:
//!
//! - Redis cluster support for horizontal scaling
//! - Memcached protocol compatibility
//! - Consistent hashing for distribution
//! - Automatic failover and replication
//!
//! ## Usage Example
//!
//! ```c
//! // Create cache configuration
//! RiCacheConfig* config = ri_cache_config_new();
//! ri_cache_config_set_max_size(config, 10000);
//! ri_cache_config_set_ttl(config, 3600);
//!
//! // Create memory cache instance
//! RiMemoryCache* cache = ri_memory_cache_new();
//!
//! // Store cached value
//! const char* key = "user:12345";
//! const char* value = "{\"name\":\"John\",\"age\":30}";
//! ri_memory_cache_set(cache, key, value, strlen(value));
//!
//! // Retrieve cached value
//! size_t value_len;
//! char* cached = ri_memory_cache_get(cache, key, &value_len);
//! if (cached != NULL) {
//!     // Process cached data
//!     free(cached);
//! }
//!
//! // Cleanup
//! ri_memory_cache_free(cache);
//! ri_cache_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::cache`: Rust cache implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The cache module is enabled by default with the "cache" feature flag.
//! Disable this feature to reduce binary size when caching is not required.

use crate::cache::{RiCacheConfig, RiCacheManager, RiMemoryCache, RiCachePolicy, RiCacheStats};
use std::ffi::{c_char, c_int, c_void};
use std::sync::Arc;

c_wrapper!(CRiCacheConfig, RiCacheConfig);

c_wrapper!(CRiCacheManager, RiCacheManager);

c_wrapper!(CRiMemoryCache, RiMemoryCache);

c_constructor!(ri_cache_config_new, CRiCacheConfig, RiCacheConfig, RiCacheConfig::default());

c_destructor!(ri_cache_config_free, CRiCacheConfig);

#[repr(C)]
pub struct CRiCachePolicy {
    pub ttl_secs: u64,
    pub ttl_set: bool,
    pub refresh_on_access: bool,
    pub max_size: usize,
    pub max_size_set: bool,
}

pub const RI_CACHE_POLICY_LRU: c_int = 0;
pub const RI_CACHE_POLICY_LFU: c_int = 1;
pub const RI_CACHE_POLICY_TTL: c_int = 2;

#[no_mangle]
pub extern "C" fn ri_cache_policy_new() -> CRiCachePolicy {
    let default = RiCachePolicy::default();
    CRiCachePolicy {
        ttl_secs: default.ttl.map(|d| d.as_secs()).unwrap_or(0),
        ttl_set: default.ttl.is_some(),
        refresh_on_access: default.refresh_on_access,
        max_size: default.max_size.unwrap_or(0),
        max_size_set: default.max_size.is_some(),
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_policy_with_ttl(ttl_secs: u64) -> CRiCachePolicy {
    let mut policy = CRiCachePolicy::new();
    policy.ttl_secs = ttl_secs;
    policy.ttl_set = true;
    policy
}

#[no_mangle]
pub extern "C" fn ri_memory_cache_new() -> *mut CRiMemoryCache {
    let cache = RiMemoryCache::new();
    Box::into_raw(Box::new(CRiMemoryCache::new(cache)))
}

c_destructor!(ri_memory_cache_free, CRiMemoryCache);

#[no_mangle]
pub extern "C" fn ri_cache_manager_new() -> *mut CRiCacheManager {
    let backend: Arc<dyn crate::cache::RiCache + Send + Sync> = Arc::new(RiMemoryCache::new());
    let manager = RiCacheManager::new(backend);
    Box::into_raw(Box::new(CRiCacheManager::new(manager)))
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_free(manager: *mut CRiCacheManager) {
    if !manager.is_null() {
        unsafe {
            let _ = Box::from_raw(manager);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_get(
    manager: *mut CRiCacheManager,
    key: *const c_char,
    out_value: *mut *mut c_char,
) -> c_int {
    if manager.is_null() || key.is_null() || out_value.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result: crate::core::RiResult<Option<String>> = rt.block_on(async {
            (*manager).inner.get(key_str).await
        });

        match result {
            Ok(Some(value)) => {
                match std::ffi::CString::new(value) {
                    Ok(c_str) => {
                        *out_value = c_str.into_raw();
                        0
                    }
                    Err(_) => -4,
                }
            }
            Ok(None) => 1,
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_set(
    manager: *mut CRiCacheManager,
    key: *const c_char,
    value: *const c_char,
    ttl_secs: u64,
) -> c_int {
    if manager.is_null() || key.is_null() || value.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let value_str = match std::ffi::CStr::from_ptr(value).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let ttl = if ttl_secs > 0 { Some(ttl_secs) } else { None };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        let result: crate::core::RiResult<()> = rt.block_on(async {
            (*manager).inner.set(key_str, &value_str, ttl).await
        });

        match result {
            Ok(()) => 0,
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_delete(
    manager: *mut CRiCacheManager,
    key: *const c_char,
) -> c_int {
    if manager.is_null() || key.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result: crate::core::RiResult<bool> = rt.block_on(async {
            (*manager).inner.delete(key_str).await
        });

        match result {
            Ok(deleted) => if deleted { 0 } else { 1 },
            Err(_) => -4,
        }
    }
}

#[repr(C)]
pub struct CRiCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub entries: usize,
    pub memory_usage_bytes: usize,
    pub avg_hit_rate: f64,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_stats(
    manager: *mut CRiCacheManager,
    out_stats: *mut CRiCacheStats,
) -> c_int {
    if manager.is_null() || out_stats.is_null() {
        return -1;
    }

    unsafe {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -2,
        };

        let stats: RiCacheStats = rt.block_on(async {
            (*manager).inner.stats().await
        });

        *out_stats = CRiCacheStats {
            hits: stats.hits,
            misses: stats.misses,
            entries: stats.entries,
            memory_usage_bytes: stats.memory_usage_bytes,
            avg_hit_rate: stats.avg_hit_rate,
            hit_count: stats.hit_count,
            miss_count: stats.miss_count,
            eviction_count: stats.eviction_count,
        };

        0
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_exists(
    manager: *mut CRiCacheManager,
    key: *const c_char,
) -> c_int {
    if manager.is_null() || key.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let exists: bool = rt.block_on(async {
            (*manager).inner.exists(key_str).await
        });

        if exists { 0 } else { 1 }
    }
}

#[no_mangle]
pub extern "C" fn ri_cache_manager_clear(manager: *mut CRiCacheManager) -> c_int {
    if manager.is_null() {
        return -1;
    }

    unsafe {
        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -2,
        };

        let result: crate::core::RiResult<()> = rt.block_on(async {
            (*manager).inner.clear().await
        });

        match result {
            Ok(()) => 0,
            Err(_) => -3,
        }
    }
}
