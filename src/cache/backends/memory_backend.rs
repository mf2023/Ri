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

#![allow(non_snake_case)]

//! # In-memory Cache Backend
//! 
//! This module provides an in-memory cache implementation using DashMap for high performance
//! and thread safety. It implements the RiCache trait, providing all standard cache operations
//! with automatic expiration handling and comprehensive statistics.
//! 
//! ## Key Features
//! 
//! - **High Performance**: Uses DashMap for concurrent access without blocking
//! - **Automatic Expiration**: Automatically removes expired entries on access
//! - **Comprehensive Statistics**: Tracks hit count, miss count, and eviction count
//! - **Thread Safe**: Safe for concurrent access from multiple threads
//! - **LRU-like Behavior**: Touches entries on access to support LRU eviction (if implemented)
//! - **Expired Entry Cleanup**: Provides a method to explicitly cleanup all expired entries
//! 
//! ## Design Principles
//! 
//! 1. **Non-blocking**: Uses DashMap for lock-free concurrent access
//! 2. **Automatic Expiration**: Expired entries are removed when accessed
//! 3. **Statistics-driven**: Comprehensive cache statistics for monitoring
//! 4. **Simple API**: Implements the standard RiCache trait
//! 5. **Memory Efficient**: Automatically cleans up expired entries
//! 6. **Thread-safe**: Safe for use in multi-threaded applications
//! 7. **Fast Access**: In-memory storage for minimal latency
//! 8. **Easy to Use**: Simple constructor with no configuration required
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a new in-memory cache
//!     let cache = RiMemoryCache::new();
//!     
//!     // Create a cached value with 1-hour expiration
//!     let value = RiCachedValue::new(b"test_value".to_vec(), Duration::from_secs(3600));
//!     
//!     // Set the value in the cache
//!     cache.set("test_key", value).await?;
//!     
//!     // Get the value from the cache
//!     if let Some(retrieved_value) = cache.get("test_key").await {
//!         println!("Retrieved value: {:?}", retrieved_value.payload);
//!     }
//!     
//!     // Check if a key exists
//!     if cache.exists("test_key").await {
//!         println!("Key exists in cache");
//!     }
//!     
//!     // Get cache statistics
//!     let stats = cache.stats().await;
//!     println!("Cache hit rate: {:.2}%", stats.avg_hit_rate * 100.0);
//!     
//!     // Cleanup expired entries
//!     let cleaned = cache.cleanup_expired().await?;
//!     println!("Cleaned up {} expired entries", cleaned);
//!     
//!     Ok(())
//! }
//! ```

use dashmap::DashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::cache::{RiCache, RiCachedValue, RiCacheStats};
use crate::core::RiResult;

/// Atomic cache statistics for lock-free performance tracking.
struct AtomicCacheStats {
    hits: AtomicU64,
    misses: AtomicU64,
    entries: AtomicUsize,
    memory_usage_bytes: AtomicUsize,
    eviction_count: AtomicU64,
}

impl AtomicCacheStats {
    fn new() -> Self {
        Self {
            hits: AtomicU64::new(0),
            misses: AtomicU64::new(0),
            entries: AtomicUsize::new(0),
            memory_usage_bytes: AtomicUsize::new(0),
            eviction_count: AtomicU64::new(0),
        }
    }

    fn increment_hits(&self) {
        self.hits.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_misses(&self) {
        self.misses.fetch_add(1, Ordering::Relaxed);
    }

    #[allow(dead_code)]
    fn increment_evictions(&self) {
        self.eviction_count.fetch_add(1, Ordering::Relaxed);
    }

    fn to_cache_stats(&self) -> RiCacheStats {
        let hits = self.hits.load(Ordering::Relaxed);
        let misses = self.misses.load(Ordering::Relaxed);
        let total = hits + misses;
        let avg_hit_rate = if total > 0 {
            hits as f64 / total as f64
        } else {
            0.0
        };

        RiCacheStats {
            hits,
            misses,
            entries: self.entries.load(Ordering::Relaxed),
            memory_usage_bytes: self.memory_usage_bytes.load(Ordering::Relaxed),
            avg_hit_rate,
            hit_count: hits,
            miss_count: misses,
            eviction_count: self.eviction_count.load(Ordering::Relaxed),
        }
    }
}

/// In-memory cache implementation using DashMap for high performance and thread safety.
///
/// This struct provides an in-memory cache with automatic expiration handling, comprehensive
/// statistics, and thread-safe concurrent access.
pub struct RiMemoryCache {
    /// Underlying storage using DashMap for concurrent access
    store: Arc<DashMap<String, RiCachedValue>>,
    /// Cache statistics using atomic operations for lock-free performance
    stats: Arc<AtomicCacheStats>,
}

impl Default for RiMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl RiMemoryCache {
    /// Creates a new in-memory cache instance.
    ///
    /// # Returns
    ///
    /// A new RiMemoryCache instance
    pub fn new() -> Self {
        RiMemoryCache {
            store: Arc::new(DashMap::new()),
            stats: Arc::new(AtomicCacheStats::new()),
        }
    }
}

#[async_trait::async_trait]
impl RiCache for RiMemoryCache {
    /// Gets a value from the cache by key.
    ///
    /// This method checks if the value exists and is not expired. If the value is expired,
    /// it is removed from the cache and None is returned. Otherwise, the value is returned
    /// and its last access time is updated.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to retrieve
    ///
    /// # Returns
    ///
    /// An `Option<RiCachedValue>` containing the value if it exists and is not expired, or None otherwise
    async fn get(&self, key: &str) -> RiResult<Option<String>> {
        match self.store.get(key) {
            Some(entry) => {
                let value = entry.clone();
                if value.is_expired() {
                    drop(entry);
                    self.store.remove(key);
                    self.stats.increment_misses();
                    Ok(None)
                } else {
                    self.stats.increment_hits();
                    Ok(Some(value.value))
                }
            }
            None => {
                self.stats.increment_misses();
                Ok(None)
            }
        }
    }
    
    /// Sets a value in the cache with the given key.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to set
    /// - `value`: The cached value to store
    ///
    /// # Returns
    ///
    /// A `RiResult<()>` indicating success or failure
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::core::RiResult<()> {
        let cached_value = RiCachedValue::new(value.to_string(), ttl_seconds);
        self.store.insert(key.to_string(), cached_value);
        Ok(())
    }
    
    /// Deletes a value from the cache by key.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to delete
    ///
    /// # Returns
    ///
    /// A `RiResult<bool>` indicating whether the key was found and deleted
    async fn delete(&self, key: &str) -> crate::core::RiResult<bool> {
        Ok(self.store.remove(key).is_some())
    }
    
    /// Checks if a key exists in the cache and is not expired.
    ///
    /// If the key exists but the value is expired, it is removed from the cache and false is returned.
    ///
    /// # Parameters
    ///
    /// - `key`: The key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists and is not expired, `false` otherwise
    async fn exists(&self, key: &str) -> bool {
        if let Some(entry) = self.store.get(key) {
            if entry.is_expired() {
                drop(entry);
                self.store.remove(key);
                false
            } else {
                true
            }
        } else {
            false
        }
    }
    
    /// Clears all entries from the cache.
    ///
    /// # Returns
    ///
    /// A `RiResult<()>` indicating success or failure
    async fn clear(&self) -> crate::core::RiResult<()> {
        self.store.clear();
        Ok(())
    }
    
    /// Gets cache statistics.
    ///
    /// # Returns
    ///
    /// A `RiCacheStats` struct containing cache statistics
    async fn stats(&self) -> RiCacheStats {
        let mut stats = self.stats.to_cache_stats();
        stats.entries = self.store.len();
        stats
    }
    
    /// Cleans up all expired entries from the cache.
    ///
    /// # Returns
    ///
    /// A `RiResult<usize>` containing the number of expired entries cleaned up
    async fn cleanup_expired(&self) -> crate::core::RiResult<usize> {
        let mut cleaned = 0;
        let keys: Vec<String> = self.store.iter().map(|entry| entry.key().clone()).collect();
        
        for key in keys {
            if let Some(entry) = self.store.get(&key) {
                if entry.is_expired() {
                    drop(entry);
                    self.store.remove(&key);
                    cleaned += 1;
                }
            }
        }
        
        Ok(cleaned)
    }

    /// Gets all keys from the cache.
    ///
    /// # Returns
    ///
    /// A `RiResult<Vec<String>>` containing all cache keys
    async fn keys(&self) -> crate::core::RiResult<Vec<String>> {
        let keys: Vec<String> = self.store.iter().map(|entry| entry.key().clone()).collect();
        Ok(keys)
    }
}
