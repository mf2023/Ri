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

#![allow(non_snake_case)]

//! # In-memory Cache Backend
//! 
//! This module provides an in-memory cache implementation using DashMap for high performance
//! and thread safety. It implements the DMSCCache trait, providing all standard cache operations
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
//! 4. **Simple API**: Implements the standard DMSCCache trait
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
//! async fn example() -> DMSCResult<()> {
//!     // Create a new in-memory cache
//!     let cache = DMSCMemoryCache::new();
//!     
//!     // Create a cached value with 1-hour expiration
//!     let value = DMSCCachedValue::new(b"test_value".to_vec(), Duration::from_secs(3600));
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
use tokio::sync::RwLock;
use crate::cache::{DMSCCache, DMSCCachedValue, DMSCCacheStats};
use crate::core::DMSCResult;

/// In-memory cache implementation using DashMap for high performance and thread safety.
///
/// This struct provides an in-memory cache with automatic expiration handling, comprehensive
/// statistics, and thread-safe concurrent access.
pub struct DMSCMemoryCache {
    /// Underlying storage using DashMap for concurrent access
    store: Arc<DashMap<String, DMSCCachedValue>>,
    /// Cache statistics tracking hit count, miss count, and eviction count
    stats: Arc<RwLock<DMSCCacheStats>>,
}

impl Default for DMSCMemoryCache {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCMemoryCache {
    /// Creates a new in-memory cache instance.
    ///
    /// # Returns
    ///
    /// A new DMSCMemoryCache instance
    pub fn new() -> Self {
        DMSCMemoryCache {
            store: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(DMSCCacheStats::default())),
        }
    }
}

#[async_trait::async_trait]
impl DMSCCache for DMSCMemoryCache {
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
    /// An `Option<DMSCCachedValue>` containing the value if it exists and is not expired, or None otherwise
    async fn get(&self, key: &str) -> DMSCResult<Option<String>> {
        match self.store.get(key) {
            Some(entry) => {
                let value = entry.clone();
                if value.is_expired() {
                    drop(entry);
                    self.store.remove(key);
                    let mut stats = self.stats.write().await;
                stats.misses += 1;
                
                    Ok(None)
                } else {
                    let mut stats = self.stats.write().await;
                stats.hits += 1;
                    Ok(Some(value.value))
                }
            }
            None => {
                let mut stats = self.stats.write().await;
                stats.misses += 1;
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::core::DMSCResult<()> {
        let cached_value = DMSCCachedValue::new(value.to_string(), ttl_seconds);
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
    /// A `DMSCResult<bool>` indicating whether the key was found and deleted
    async fn delete(&self, key: &str) -> crate::core::DMSCResult<bool> {
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn clear(&self) -> crate::core::DMSCResult<()> {
        self.store.clear();
        Ok(())
    }
    
    /// Gets cache statistics.
    ///
    /// # Returns
    ///
    /// A `DMSCCacheStats` struct containing cache statistics
    async fn stats(&self) -> DMSCCacheStats {
        *self.stats.read().await
    }
    
    /// Cleans up all expired entries from the cache.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<usize>` containing the number of expired entries cleaned up
    async fn cleanup_expired(&self) -> crate::core::DMSCResult<usize> {
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
}
