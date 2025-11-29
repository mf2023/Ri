//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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
//! and thread safety. It implements the DMSCache trait, providing all standard cache operations
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
//! 4. **Simple API**: Implements the standard DMSCache trait
//! 5. **Memory Efficient**: Automatically cleans up expired entries
//! 6. **Thread-safe**: Safe for use in multi-threaded applications
//! 7. **Fast Access**: In-memory storage for minimal latency
//! 8. **Easy to Use**: Simple constructor with no configuration required
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a new in-memory cache
//!     let cache = DMSMemoryCache::_Fnew();
//!     
//!     // Create a cached value with 1-hour expiration
//!     let value = CachedValue::new(b"test_value".to_vec(), Duration::from_secs(3600));
//!     
//!     // Set the value in the cache
//!     cache._Fset("test_key", value).await?;
//!     
//!     // Get the value from the cache
//!     if let Some(retrieved_value) = cache._Fget("test_key").await {
//!         println!("Retrieved value: {:?}", retrieved_value.payload);
//!     }
//!     
//!     // Check if a key exists
//!     if cache._Fexists("test_key").await {
//!         println!("Key exists in cache");
//!     }
//!     
//!     // Get cache statistics
//!     let stats = cache._Fstats().await;
//!     println!("Cache hit rate: {:.2}%", stats.avg_hit_rate * 100.0);
//!     
//!     // Cleanup expired entries
//!     let cleaned = cache._Fcleanup_expired().await?;
//!     println!("Cleaned up {} expired entries", cleaned);
//!     
//!     Ok(())
//! }
//! ```

use dashmap::DashMap;
use std::sync::Arc;
use std::ops::AddAssign;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// In-memory cache implementation using DashMap for high performance and thread safety.
///
/// This struct provides an in-memory cache with automatic expiration handling, comprehensive
/// statistics, and thread-safe concurrent access.
pub struct DMSMemoryCache {
    /// Underlying storage using DashMap for concurrent access
    store: Arc<DashMap<String, CachedValue>>,
    /// Cache statistics tracking hit count, miss count, and eviction count
    stats: Arc<dashmap::DashMap<&'static str, u64>>,
}

impl DMSMemoryCache {
    /// Creates a new in-memory cache instance.
    ///
    /// # Returns
    ///
    /// A new DMSMemoryCache instance
    pub fn _Fnew() -> Self {
        let stats = DashMap::new();
        stats.insert("hit_count", 0);
        stats.insert("miss_count", 0);
        stats.insert("eviction_count", 0);
        
        Self {
            store: Arc::new(DashMap::new()),
            stats: Arc::new(stats),
        }
    }
}

#[async_trait::async_trait]
impl DMSCache for DMSMemoryCache {
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
    /// An `Option<CachedValue>` containing the value if it exists and is not expired, or None otherwise
    async fn _Fget(&self, key: &str) -> Option<CachedValue> {
        match self.store.get(key) {
            Some(entry) => {
                let mut value = entry.clone();
                if value._Fis_expired() {
                    drop(entry);
                    self.store.remove(key);
                    self.stats.get_mut("eviction_count").unwrap().value_mut().add_assign(1);
                    self.stats.get_mut("miss_count").unwrap().value_mut().add_assign(1);
                    None
                } else {
                    value._Ftouch();
                    self.stats.get_mut("hit_count").unwrap().value_mut().add_assign(1);
                    Some(value)
                }
            }
            None => {
                self.stats.get_mut("miss_count").unwrap().value_mut().add_assign(1);
                None
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()> {
        self.store.insert(key.to_string(), value);
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        self.store.remove(key);
        Ok(())
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
    async fn _Fexists(&self, key: &str) -> bool {
        if let Some(entry) = self.store.get(key) {
            if entry._Fis_expired() {
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        self.store.clear();
        Ok(())
    }
    
    /// Gets cache statistics.
    ///
    /// # Returns
    ///
    /// A `CacheStats` struct containing cache statistics
    async fn _Fstats(&self) -> CacheStats {
        let total_keys = self.store.len();
        let hit_count = *self.stats.get("hit_count").unwrap().value();
        let miss_count = *self.stats.get("miss_count").unwrap().value();
        let eviction_count = *self.stats.get("eviction_count").unwrap().value();
        
        let total_requests = hit_count + miss_count;
        let avg_hit_rate = if total_requests > 0 {
            hit_count as f64 / total_requests as f64
        } else {
            0.0
        };
        
        // Estimate memory usage (simplified)
        let memory_usage_bytes = total_keys * 100; // Rough estimate per entry
        
        CacheStats {
            total_keys,
            memory_usage_bytes,
            hit_count,
            miss_count,
            eviction_count,
            avg_hit_rate,
        }
    }
    
    /// Cleans up all expired entries from the cache.
    ///
    /// # Returns
    ///
    /// A `DMSResult<usize>` containing the number of expired entries cleaned up
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        let mut cleaned = 0;
        let keys: Vec<String> = self.store.iter().map(|entry| entry.key().clone()).collect();
        
        for key in keys {
            if let Some(entry) = self.store.get(&key) {
                if entry._Fis_expired() {
                    drop(entry);
                    self.store.remove(&key);
                    cleaned += 1;
                }
            }
        }
        
        Ok(cleaned)
    }
}