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

//! # Core Module
//!
//! This module provides the core abstractions and data structures for the DMSC caching system.
//! It defines the foundational traits, event types, statistics, and value wrappers that all
//! cache backend implementations rely upon.
//!
//! ## Key Components
//!
//! - **[`DMSCCache`](DMSCCache)**: Core trait defining the cache interface with async operations
//! - **[`DMSCCacheEvent`](DMSCCacheEvent)**: Event types for cache monitoring and consistency
//! - **[`DMSCCacheStats`](DMSCCacheStats)**: Statistics tracking for cache performance monitoring
//! - **[`DMSCCachedValue`](DMSCCachedValue)**: Wrapper for cached values with TTL and LRU support
//!
//! ## Design Principles
//!
//! 1. **Trait-based Architecture**: All backends implement the DMSCCache trait for consistency
//! 2. **Async-first**: Full async/await support for non-blocking cache operations
//! 3. **Thread Safety**: All implementations are Send + Sync for concurrent access
//! 4. **Extensibility**: Easy to add new cache backends by implementing the trait
//! 5. **Monitoring**: Built-in event system for cache activity tracking
//! 6. **Statistics**: Comprehensive metrics for cache performance analysis
//!
//! ## Usage Example
//!
//! ```rust
//! use dmsc::cache::{DMSCCache, DMSCCacheEvent, DMSCCacheStats, DMSCCachedValue};
//! use dmsc::cache::backends::DMSCMemoryCache;
//!
//! async fn example() -> dmsc::core::DMSCResult<()> {
//!     // Create a memory cache backend
//!     let cache = DMSCMemoryCache::new();
//!
//!     // Set a value with 1-hour TTL
//!     cache.set("user:123", "{\"name\": \"Alice\"}", Some(3600)).await?;
//!
//!     // Retrieve the value
//!     let value = cache.get("user:123").await?;
//!     println!("Retrieved: {:?}", value);
//!
//!     // Check if key exists
//!     let exists = cache.exists("user:123").await;
//!
//!     // Get cache statistics
//!     let stats: DMSCCacheStats = cache.stats().await;
//!
//!     // Clean up expired entries
//!     let cleaned = cache.cleanup_expired().await?;
//!
//!     Ok(())
//! }
//! ```

use crate::core::{DMSCResult, DMSCError};
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Cache trait for DMSC cache implementations.
///
/// This trait defines the core interface for all cache backends in DMSC.
/// Implementations must provide thread-safe, asynchronous cache operations
/// with support for TTL-based expiration and comprehensive statistics tracking.
///
/// ## Implementations
///
/// DMSC provides several built-in implementations:
/// - **[`DMSCMemoryCache`](super::backends::DMSCMemoryCache)**: In-memory cache using DashMap
/// - **[`DMSCRedisCache`](super::backends::DMSCRedisCache)**: Distributed cache using Redis
/// - **[`DMSCHybridCache`](super::backends::DMSCHybridCache)**: Multi-layer cache combining memory and Redis
///
/// ## Thread Safety
///
/// All implementations must be `Send + Sync` to ensure safe concurrent access
/// from multiple async tasks or threads. The trait uses interior mutability
/// patterns internally.
///
/// ## Async Operations
///
/// All operations are asynchronous and use async/await syntax. This enables
/// non-blocking cache operations suitable for high-throughput applications.
///
/// ## Key Operations
///
/// 1. **Basic Operations**: `get`, `set`, `delete`, `exists`
/// 2. **Batch Operations**: `get_multi`, `set_multi`, `delete_multi`
/// 3. **Maintenance**: `clear`, `cleanup_expired`, `stats`
/// 4. **Pattern Matching**: `keys`, `delete_by_pattern`
///
/// ## Example
///
/// ```rust
/// use dmsc::cache::DMSCCache;
/// use dmsc::cache::backends::DMSCMemoryCache;
///
/// async fn example() -> dmsc::core::DMSCResult<()> {
///     let cache = DMSCMemoryCache::new();
///
///     // Store a value with 1-hour TTL
///     cache.set("user:123", "Alice", Some(3600)).await?;
///
///     // Retrieve the value
///     let value = cache.get("user:123").await?;
///     assert_eq!(value, Some("Alice".to_string()));
///
///     // Check if key exists
///     assert!(cache.exists("user:123").await);
///
///     // Get cache statistics
///     let stats = cache.stats().await;
///     println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
///
///     // Delete the value
///     cache.delete("user:123").await?;
///
///     Ok(())
/// }
/// ```
#[async_trait::async_trait]
pub trait DMSCCache: Send + Sync {
    /// Retrieves a value from the cache by key.
    ///
    /// This method looks up the specified key in the cache. If the key exists
    /// and the value is not expired, it returns the value as a string. Expired
    /// entries are automatically removed during the lookup.
    ///
    /// ## Expiration Handling
    ///
    /// If the cached value has an associated TTL (Time-To-Live) and the current
    /// time has passed the expiration timestamp, the entry is treated as missing
    /// and removed from the cache.
    ///
    /// ## Statistics
    ///
    /// This operation updates cache statistics:
    /// - Increments `hits` counter on successful retrieval
    /// - Increments `misses` counter when key is not found or expired
    ///
    /// # Parameters
    ///
    /// * `key` - The cache key to look up (typically a string identifier)
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Option<String>>` containing:
    /// - `Ok(Some(value))` if the key exists and is not expired
    /// - `Ok(None)` if the key doesn't exist or has expired
    /// - `Err(DMSCError)` if an error occurred during the operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Key doesn't exist
    ///     let result = cache.get("missing").await?;
    ///     assert_eq!(result, None);
    ///
    ///     // Store a value
    ///     cache.set("key", "value", None).await?;
    ///
    ///     // Key exists
    ///     let result = cache.get("key").await?;
    ///     assert_eq!(result, Some("value".to_string()));
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get(&self, key: &str) -> DMSCResult<Option<String>>;

    /// Stores a value in the cache with an optional TTL.
    ///
    /// This method inserts or updates a cache entry with the specified key and value.
    /// The entry will automatically expire after the specified TTL duration if provided.
    ///
    /// ## Overwrite Behavior
    ///
    /// If a value already exists for the given key, it will be overwritten with the
    /// new value. The expiration time will be reset based on the new TTL.
    ///
    /// ## TTL Handling
    ///
    /// - `Some(seconds)`: The entry will expire after the specified number of seconds
    /// - `None`: The entry will never expire automatically
    ///
    /// ## Storage Format
    ///
    /// The value is stored as a string. For complex types, serialize them to a string
    /// format (e.g., JSON) before caching.
    ///
    /// # Parameters
    ///
    /// * `key` - The cache key to store the value under
    /// * `value` - The string value to cache
    /// * `ttl_seconds` - Optional time-to-live in seconds (None for persistent storage)
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Store a value without expiration
    ///     cache.set("persistent", "data", None).await?;
    ///
    ///     // Store a value with 1-hour expiration
    ///     cache.set("temp", "data", Some(3600)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> DMSCResult<()>;

    /// Removes a value from the cache by key.
    ///
    /// This method deletes the specified key from the cache. If the key doesn't
    /// exist, the operation still succeeds but returns false.
    ///
    /// ## Behavior
    ///
    /// - The entry is completely removed from the cache
    /// - If the key doesn't exist, no error is raised
    /// - No statistics are updated for delete operations
    ///
    /// # Parameters
    ///
    /// * `key` - The cache key to delete
    ///
    /// # Returns
    ///
    /// A `DMSCResult<bool>` containing:
    /// - `Ok(true)` if the key was found and deleted
    /// - `Ok(false)` if the key didn't exist
    /// - `Err(DMSCError)` if an error occurred during the operation
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Delete non-existent key
    ///     let deleted = cache.delete("missing").await?;
    ///     assert!(!deleted);
    ///
    ///     // Store and delete
    ///     cache.set("key", "value", None).await?;
    ///     let deleted = cache.delete("key").await?;
    ///     assert!(deleted);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn delete(&self, key: &str) -> DMSCResult<bool>;

    /// Removes all entries from the cache.
    ///
    /// This method clears all cached values regardless of their expiration status.
    /// The operation is typically O(n) where n is the number of cached entries.
    ///
    /// ## Behavior
    ///
    /// - All entries are immediately removed
    /// - Statistics are reset to their default values
    /// - This operation cannot be undone
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Add some entries
    ///     cache.set("a", "1", None).await?;
    ///     cache.set("b", "2", None).await?;
    ///     cache.set("c", "3", None).await?;
    ///
    ///     // Clear all entries
    ///     cache.clear().await?;
    ///
    ///     // Verify cache is empty
    ///     assert!(!cache.exists("a").await);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn clear(&self) -> DMSCResult<()>;

    /// Returns current cache statistics.
    ///
    /// This method retrieves performance metrics and usage statistics from the cache.
    /// The statistics provide insights into cache effectiveness and resource usage.
    ///
    /// ## Statistics Collected
    ///
    /// - `hits`: Number of successful cache lookups
    /// - `misses`: Number of cache lookups that returned None
    /// - `entries`: Current number of entries in the cache
    /// - `memory_usage_bytes`: Estimated memory consumption
    /// - `avg_hit_rate`: Ratio of hits to total lookups
    /// - `eviction_count`: Number of entries evicted due to size limits
    ///
    /// ## Thread Safety
    ///
    /// The returned statistics are a snapshot taken at call time. Other threads
    /// may modify the cache immediately after, making the statistics slightly stale.
    ///
    /// # Returns
    ///
    /// A `DMSCCacheStats` struct containing all cache metrics
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Perform some cache operations
    ///     let _ = cache.get("missing").await;
    ///     cache.set("key", "value", None).await.unwrap();
    ///     let _ = cache.get("key").await;
    ///
    ///     // Get statistics
    ///     let stats = cache.stats().await;
    ///     println!("Hits: {}, Misses: {}", stats.hits, stats.misses);
    ///     println!("Hit rate: {:.1}%", stats.avg_hit_rate * 100.0);
    /// }
    /// ```
    async fn stats(&self) -> DMSCCacheStats;

    /// Removes all expired entries from the cache.
    ///
    /// This method scans the cache and removes entries that have exceeded their
    /// TTL (Time-To-Live). This is useful for reclaiming memory used by expired entries.
    ///
    /// ## Performance
    ///
    /// The performance characteristics depend on the implementation:
    /// - In-memory caches: Typically O(n) where n is total entries
    /// - Distributed caches: May involve network round-trips for each entry
    ///
    /// ## Automatic Cleanup
    ///
    /// Many implementations automatically remove expired entries during normal
    /// operations (e.g., during `get()` calls). This explicit cleanup is useful
    /// for periodic maintenance or when entries have no recent access.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<usize>` containing the number of expired entries removed
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     // Add entries with short TTL
    ///     cache.set("short-lived", "data", Some(1)).await?;
    ///
    ///     // Wait for expiration
    ///     tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    ///
    ///     // Cleanup expired entries
    ///     let cleaned = cache.cleanup_expired().await?;
    ///     println!("Cleaned {} expired entries", cleaned);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn cleanup_expired(&self) -> DMSCResult<usize>;

    /// Checks if a key exists in the cache and is not expired.
    ///
    /// This method provides a lightweight way to check key existence without
    /// retrieving the value. Expired entries are automatically removed.
    ///
    /// ## Expiration Check
    ///
    /// If the key exists but the value is expired, the entry is removed and
    /// the method returns false.
    ///
    /// ## Performance
    ///
    /// This operation is typically faster than `get()` because it doesn't
    /// need to deserialize or return the cached value.
    ///
    /// # Parameters
    ///
    /// * `key` - The cache key to check
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the key exists and is not expired
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     assert!(!cache.exists("missing").await);
    ///
    ///     cache.set("key", "value", None).await?;
    ///     assert!(cache.exists("key").await);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn exists(&self, key: &str) -> bool;

    /// Retrieves all cache keys.
    ///
    /// This method returns a list of all keys currently stored in the cache,
    /// including expired ones. Use `cleanup_expired()` to remove expired entries first.
    ///
    /// ## Order
    ///
    /// The order of returned keys is implementation-defined. Do not rely on
    /// any particular ordering.
    ///
    /// ## Performance
    ///
    /// This operation may be expensive for large caches as it typically requires
    /// iterating through all entries.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<String>>` containing all cache keys
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     cache.set("a", "1", None).await?;
    ///     cache.set("b", "2", None).await?;
    ///     cache.set("c", "3", None).await?;
    ///
    ///     let keys = cache.keys().await?;
    ///     assert_eq!(keys.len(), 3);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn keys(&self) -> DMSCResult<Vec<String>>;

    /// Retrieves multiple values from the cache in a single operation.
    ///
    /// This method is a convenience wrapper that fetches multiple keys efficiently.
    /// The results are returned in the same order as the input keys.
    ///
    /// ## Partial Results
    ///
    /// If some keys exist and others don't, the result vector will contain
    /// `Some(value)` for existing keys and `None` for missing keys.
    ///
    /// # Parameters
    ///
    /// * `keys` - A slice of cache keys to retrieve
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<Option<String>>>` containing values in key order
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     cache.set("a", "1", None).await?;
    ///     cache.set("b", "2", None).await?;
    ///
    ///     let results = cache.get_multi(&["a", "b", "c"]).await?;
    ///     assert_eq!(results, vec![
    ///         Some("1".to_string()),
    ///         Some("2".to_string()),
    ///         None
    ///     ]);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn get_multi(&self, keys: &[&str]) -> DMSCResult<Vec<Option<String>>> {
        let mut results = Vec::with_capacity(keys.len());
        for &key in keys {
            results.push(self.get(key).await?);
        }
        Ok(results)
    }

    /// Stores multiple key-value pairs in the cache.
    ///
    /// This method is a convenience wrapper for setting multiple entries efficiently.
    /// All entries use the same TTL if provided.
    ///
    /// # Parameters
    ///
    /// * `items` - A slice of (key, value) tuples to store
    /// * `ttl_seconds` - Optional TTL for all entries
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     let items = vec![
    ///         ("a", "1"),
    ///         ("b", "2"),
    ///         ("c", "3"),
    ///     ];
    ///
    ///     cache.set_multi(&items, Some(3600)).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn set_multi(&self, items: &[(&str, &str)], ttl_seconds: Option<u64>) -> DMSCResult<()> {
        for &(key, value) in items {
            self.set(key, value, ttl_seconds).await?;
        }
        Ok(())
    }

    /// Removes multiple keys from the cache.
    ///
    /// This method is a convenience wrapper for deleting multiple entries efficiently.
    ///
    /// ## Atomicity
    ///
    /// This operation is not atomic - each delete is performed independently.
    /// Partial failures may result in some keys being deleted while others remain.
    ///
    /// # Parameters
    ///
    /// * `keys` - A slice of cache keys to delete
    ///
    /// # Returns
    ///
    /// A `DMSCResult<usize>` containing the number of keys deleted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     cache.set("a", "1", None).await?;
    ///     cache.set("b", "2", None).await?;
    ///     cache.set("c", "3", None).await?;
    ///
    ///     let count = cache.delete_multi(&["a", "b"]).await?;
    ///     assert_eq!(count, 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn delete_multi(&self, keys: &[&str]) -> DMSCResult<usize> {
        let mut count = 0;
        for &key in keys {
            if self.delete(key).await? {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Checks if multiple keys exist in the cache.
    ///
    /// This method is a convenience wrapper for checking multiple keys efficiently.
    ///
    /// # Parameters
    ///
    /// * `keys` - A slice of cache keys to check
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<bool>>` indicating existence of each key
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     cache.set("a", "1", None).await?;
    ///
    ///     let results = cache.exists_multi(&["a", "b"]).await?;
    ///     assert_eq!(results, vec![true, false]);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn exists_multi(&self, keys: &[&str]) -> DMSCResult<Vec<bool>> {
        let mut results = Vec::with_capacity(keys.len());
        for &key in keys {
            results.push(self.exists(key).await);
        }
        Ok(results)
    }

    /// Removes all keys matching a regex pattern.
    ///
    /// This method is useful for bulk invalidation of related cache entries.
    /// For example, invalidating all user-related cache entries when a user updates their profile.
    ///
    /// ## Pattern Format
    ///
    /// The pattern is a regular expression. Common patterns include:
    /// - `user:*` - Matches all keys starting with "user:"
    /// - `*:session` - Matches all keys ending with ":session"
    /// - `.*` - Matches all keys
    ///
    /// ## Performance
    ///
    /// This operation requires fetching all keys and filtering by regex.
    /// For large caches, consider using key prefixes for better performance.
    ///
    /// # Parameters
    ///
    /// * `pattern` - A regular expression pattern to match keys against
    ///
    /// # Returns
    ///
    /// A `DMSCResult<usize>` containing the number of keys deleted
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dmsc::cache::backends::DMSCMemoryCache;
    ///
    /// async fn example() -> dmsc::core::DMSCResult<()> {
    ///     let cache = DMSCMemoryCache::new();
    ///
    ///     cache.set("user:123:profile", "data", None).await?;
    ///     cache.set("user:123:settings", "data", None).await?;
    ///     cache.set("product:456", "data", None).await?;
    ///
    ///     let count = cache.delete_by_pattern("user:.*").await?;
    ///     assert_eq!(count, 2);
    ///
    ///     Ok(())
    /// }
    /// ```
    async fn delete_by_pattern(&self, pattern: &str) -> DMSCResult<usize> {
        let keys = self.keys().await?;
        let regex = regex::Regex::new(pattern)
            .map_err(|e| DMSCError::Other(format!("Invalid pattern: {}", e)))?;
        let mut count = 0;
        for key in keys {
            if regex.is_match(&key) {
                if self.delete(&key).await? {
                    count += 1;
                }
            }
        }
        Ok(count)
    }
}

/// Cache event types for monitoring and consistency
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCCacheEvent {
    /// Cache hit event
    Hit { key: String },
    /// Cache miss event
    Miss { key: String },
    /// Cache eviction event
    Eviction { key: String },
    /// Cache set event
    Set { key: String, ttl_seconds: Option<u64> },
    /// Cache delete event
    Delete { key: String },
    /// Cache clear event
    Clear(),
    /// Cache cleanup event
    Cleanup { cleaned_count: usize },
    /// Cache invalidate pattern event
    InvalidatePattern { pattern: String },
    /// Cache invalidate event
    Invalidate { key: String },
}

/// Cache statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCacheStats {
    pub hits: u64,
    pub misses: u64,
    pub entries: usize,
    pub memory_usage_bytes: usize,
    pub avg_hit_rate: f64,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
}

impl Default for DMSCCacheStats {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            entries: 0,
            memory_usage_bytes: 0,
            avg_hit_rate: 0.0,
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCCacheStats {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_stats() -> Self {
        Self::default()
    }
}

/// Cached value wrapper with TTL and LRU support.
///
/// This struct encapsulates a cached value along with metadata for cache management:
/// - **value**: The actual cached data as a string
/// - **expires_at**: Optional TTL-based expiration timestamp (UNIX epoch seconds)
/// - **last_accessed**: Optional last access timestamp for LRU eviction policies
///
/// # Examples
///
/// ```
/// use dmsc::cache::DMSCCachedValue;
///
/// let cached = DMSCCachedValue::new("test_data".to_string(), Some(3600));
/// assert!(!cached.is_expired());
/// cached.touch();
/// assert!(!cached.is_stale(300));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCachedValue {
    /// The cached value as a string
    pub value: String,
    /// Optional expiration timestamp (UNIX epoch seconds)
    /// If None, the value never expires based on TTL
    pub expires_at: Option<u64>,
    /// Optional last access timestamp (UNIX epoch seconds)
    /// Used for LRU-based cache eviction policies
    pub last_accessed: Option<u64>,
}

impl DMSCCachedValue {
    /// Creates a new cached value with optional TTL.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The string value to cache
    /// - `ttl_seconds`: Optional time-to-live in seconds
    ///   - If Some(seconds), the value will expire after the specified duration
    ///   - If None, the value never expires based on TTL
    /// 
    /// # Behavior
    /// 
    /// - Initializes `last_accessed` to current timestamp for LRU tracking
    /// - Calculates `expires_at` as current_time + ttl_seconds if TTL is provided
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dmsc::cache::DMSCCachedValue;
    /// 
    /// // Create a value that expires in 1 hour
    /// let cached = DMSCCachedValue::new("data".to_string(), Some(3600));
    /// 
    /// // Create a value that never expires
    /// let persistent = DMSCCachedValue::new("persistent".to_string(), None);
    /// ```
    pub fn new(value: String, ttl_seconds: Option<u64>) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let expires_at = ttl_seconds.map(|ttl| {
            now + ttl
        });
        
        Self { 
            value, 
            expires_at,
            last_accessed: Some(now),
        }
    }
    
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> crate::core::DMSCResult<T> {
        serde_json::from_str(&self.value)
            .map_err(|e| crate::core::DMSCError::Other(format!("Deserialization error: {e}")))
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
            now >= expires_at
        } else {
            false
        }
    }
    
    /// Updates the last access timestamp to current time.
    /// 
    /// This method should be called each time the cached value is accessed
    /// to support LRU (Least Recently Used) cache eviction policies.
    /// 
    /// # Behavior
    /// 
    /// - Sets `last_accessed` to the current UNIX timestamp
    /// - Does not modify `expires_at` or `value`
    /// 
    /// # Use Cases
    /// 
    /// - LRU cache implementations tracking access order
    /// - Cache warming strategies based on access patterns
    /// - Usage analytics and cache performance monitoring
    pub fn touch(&mut self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        self.last_accessed = Some(now);
    }
    
    /// Checks if the cached value is stale based on idle time.
    /// 
    /// A value is considered stale if it has not been accessed for longer
    /// than the specified maximum idle time. This is useful for LRU eviction.
    /// 
    /// # Parameters
    /// 
    /// - `max_idle_secs`: Maximum idle time in seconds before considering stale
    /// 
    /// # Returns
    /// 
    /// - `true` if the value is stale (not accessed within max_idle_secs)
    /// - `false` if the value is still fresh or has no access timestamp
    /// 
    /// # Examples
    /// 
    /// ```
    /// use dmsc::cache::DMSCCachedValue;
    /// 
    /// let mut cached = DMSCCachedValue::new("data".to_string(), None);
    /// 
    /// // Immediately after creation, not stale
    /// assert!(!cached.is_stale(300));
    /// 
    /// cached.touch();
    /// assert!(!cached.is_stale(300));
    /// ```
    pub fn is_stale(&self, max_idle_secs: u64) -> bool {
        if let Some(last_accessed) = self.last_accessed {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs();
            now - last_accessed > max_idle_secs
        } else {
            false
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCCachedValue {
    #[new]
    fn py_new(value: String, ttl_seconds: Option<u64>) -> Self {
        Self::new(value, ttl_seconds)
    }
    
    #[staticmethod]
    fn default() -> Self {
        Self::new(String::new(), None)
    }
    
    #[pyo3(name = "is_expired")]
    fn is_expired_impl(&self) -> bool {
        self.is_expired()
    }
    
    #[pyo3(name = "touch")]
    fn touch_impl(&mut self) {
        self.touch()
    }
}
