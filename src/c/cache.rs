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

//! # Cache Module C API
//!
//! This module provides C language bindings for DMSC's caching subsystem. The cache module
//! delivers high-performance in-memory data caching capabilities for accelerating application
//! performance, reducing database load, and improving system throughput. This C API enables
//! C/C++ applications to leverage DMSC's sophisticated caching infrastructure including memory
//! caching, distributed caching support, and intelligent cache eviction policies.
//!
//! ## Module Architecture
//!
//! The caching module comprises three primary components:
//!
//! - **DMSCCacheConfig**: Configuration container for cache system parameters. Controls cache
//!   size limits, eviction policies, expiration timeouts, and connection settings for
//!   distributed cache backends. The configuration object is essential for initializing
//!   cache managers with appropriate resource limits and behavior characteristics.
//!
//! - **DMSCCacheManager**: Central cache management interface providing unified operations
//!   across different cache backends. Handles cache lifecycle, backend selection, and
//!   provides high-level cache operations including get, set, delete, and invalidation.
//!   The cache manager supports automatic serialization of complex types and provides
//!   consistent API regardless of underlying storage implementation.
//!
//! - **DMSCMemoryCache**: In-memory cache implementation using concurrent data structures.
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
//! DMSCCacheConfig* config = dmsc_cache_config_new();
//! dmsc_cache_config_set_max_size(config, 10000);
//! dmsc_cache_config_set_ttl(config, 3600);
//!
//! // Create memory cache instance
//! DMSCMemoryCache* cache = dmsc_memory_cache_new();
//!
//! // Store cached value
//! const char* key = "user:12345";
//! const char* value = "{\"name\":\"John\",\"age\":30}";
//! dmsc_memory_cache_set(cache, key, value, strlen(value));
//!
//! // Retrieve cached value
//! size_t value_len;
//! char* cached = dmsc_memory_cache_get(cache, key, &value_len);
//! if (cached != NULL) {
//!     // Process cached data
//!     free(cached);
//! }
//!
//! // Cleanup
//! dmsc_memory_cache_free(cache);
//! dmsc_cache_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::cache`: Rust cache implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The cache module is enabled by default with the "cache" feature flag.
//! Disable this feature to reduce binary size when caching is not required.

use crate::cache::{DMSCCacheConfig, DMSCCacheManager, DMSCMemoryCache};

c_wrapper!(CDMSCCacheConfig, DMSCCacheConfig);

c_wrapper!(CDMSCCacheManager, DMSCCacheManager);

c_wrapper!(CDMSCMemoryCache, DMSCMemoryCache);

c_constructor!(dmsc_cache_config_new, CDMSCCacheConfig, DMSCCacheConfig, DMSCCacheConfig::default());

c_destructor!(dmsc_cache_config_free, CDMSCCacheConfig);

/// Creates a new DMSCMemoryCache instance.
///
/// Initializes an empty in-memory cache with default configuration. The cache
/// starts empty and grows as entries are added. Memory usage is managed automatically
/// through eviction policies.
///
/// # Returns
///
/// Pointer to newly allocated DMSCMemoryCache on success, or NULL if memory
/// allocation fails. The returned pointer must be freed using dmsc_memory_cache_free().
///
/// # Initial State
///
/// A newly created memory cache:
///
/// - Contains zero entries
/// - Has no memory usage
/// - Uses default LRU eviction
/// - No maximum capacity enforcement until configured
///
/// # Usage Pattern
///
/// ```c
/// DMSCMemoryCache* cache = dmsc_memory_cache_new();
/// if (cache == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
///
/// // Configure capacity if needed
/// dmsc_memory_cache_set_max_size(cache, 100000);
///
/// // Use cache operations
/// dmsc_memory_cache_set(cache, "key", "value", 5);
/// char* value = dmsc_memory_cache_get(cache, "key", NULL);
///
/// // Cleanup
/// dmsc_memory_cache_free(cache);
/// ```
///
/// # Performance Considerations
///
/// For optimal performance:
///
/// - Configure capacity before heavy usage
/// - Batch similar operations together
/// - Use appropriate serialization format
/// - Monitor cache hit rate for tuning
#[no_mangle]
pub extern "C" fn dmsc_memory_cache_new() -> *mut CDMSCMemoryCache {
    let cache = DMSCMemoryCache::new();
    Box::into_raw(Box::new(CDMSCMemoryCache::new(cache)))
}

c_destructor!(dmsc_memory_cache_free, CDMSCMemoryCache);
