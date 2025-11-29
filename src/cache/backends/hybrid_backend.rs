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

//! Hybrid cache implementation combining memory and Redis backends.
//! 
//! This module provides a hybrid cache implementation that combines the speed of
//! in-memory caching with the persistence and distributed capabilities of Redis.
//! It follows a two-level caching strategy:
//! 1. First checks the in-memory cache for fast access
//! 2. If not found, checks Redis and caches the result in memory for future requests
//! 3. Writes to both caches simultaneously for consistency
//! 
//! # Design Principles
//! - **Performance**: Fast in-memory access for frequently accessed data
//! - **Persistence**: Redis provides data persistence and crash recovery
//! - **Consistency**: Writes are propagated to both caches
//! - **Scalability**: Redis enables distributed caching across multiple instances
//! - **Efficiency**: Automatic caching of Redis results in memory
//! - **Transparency**: Implements the same `DMSCache` trait as other cache backends
//! 
//! # Usage Examples
//! ```rust
//! // Create a hybrid cache with Redis connection
//! let hybrid_cache = DMSHybridCache::_Fnew("redis://localhost:6379").await?;
//! 
//! // Set a value (stored in both memory and Redis)
//! let cached_value = CachedValue {
//!     value: b"test_data".to_vec(),
//!     expires_at: Some(SystemTime::now() + Duration::from_secs(3600)),
//!     metadata: HashMap::new(),
//! };
//! hybrid_cache._Fset("test_key", cached_value).await?;
//! 
//! // Get a value (checked in memory first, then Redis)
//! let value = hybrid_cache._Fget("test_key").await;
//! 
//! // Delete a value (removed from both caches)
//! hybrid_cache._Fdelete("test_key").await?;
//! ```

#![allow(non_snake_case)]

use std::sync::Arc;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// Hybrid cache implementation combining memory and Redis backends.
/// 
/// This struct implements a two-level caching strategy that leverages both
/// in-memory caching for speed and Redis for persistence and distributed caching.
pub struct DMSHybridCache {
    memory_cache: Arc<crate::cache::backends::DMSMemoryCache>, // Fast in-memory cache
    redis_cache: Arc<crate::cache::backends::DMSRedisCache>,  // Persistent Redis cache
}

impl DMSHybridCache {
    /// Creates a new hybrid cache instance.
    /// 
    /// # Parameters
    /// - `redis_url`: Redis connection URL (e.g., "redis://localhost:6379")
    /// 
    /// # Returns
    /// A new instance of `DMSHybridCache`
    pub async fn _Fnew(redis_url: &str) -> crate::core::DMSResult<Self> {
        let memory_cache = Arc::new(crate::cache::backends::DMSMemoryCache::_Fnew());
        let redis_cache = Arc::new(crate::cache::backends::DMSRedisCache::_Fnew(redis_url).await?);
        
        Ok(Self {
            memory_cache,
            redis_cache,
        })
    }
}

#[async_trait::async_trait]
impl DMSCache for DMSHybridCache {
    /// Gets a value from the hybrid cache.
    /// 
    /// Follows a two-level lookup strategy:
    /// 1. First checks the in-memory cache for fast access
    /// 2. If not found, checks Redis
    /// 3. If found in Redis, caches the result in memory for future requests
    /// 
    /// # Parameters
    /// - `key`: Cache key to retrieve
    /// 
    /// # Returns
    /// `Some(CachedValue)` if the key exists in either cache, otherwise `None`
    async fn _Fget(&self, key: &str) -> Option<CachedValue> {
        // First check memory cache
        if let Some(value) = self.memory_cache._Fget(key).await {
            return Some(value);
        }
        
        // If not in memory, check Redis
        if let Some(value) = self.redis_cache._Fget(key).await {
            // Store in memory cache for future requests
            let _ = self.memory_cache._Fset(key, value.clone()).await;
            return Some(value);
        }
        
        None
    }
    
    /// Sets a value in both caches.
    /// 
    /// Writes the value to both the in-memory cache and Redis simultaneously
    /// to ensure consistency across both cache levels.
    /// 
    /// # Parameters
    /// - `key`: Cache key to set
    /// - `value`: Value to store in the cache
    /// 
    /// # Returns
    /// `Ok(())` if the value was successfully set in both caches
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()> {
        // Set in both caches
        self.memory_cache._Fset(key, value.clone()).await?;
        self.redis_cache._Fset(key, value).await?;
        Ok(())
    }
    
    /// Deletes a value from both caches.
    /// 
    /// Removes the value from both the in-memory cache and Redis to ensure
    /// consistency across both cache levels.
    /// 
    /// # Parameters
    /// - `key`: Cache key to delete
    /// 
    /// # Returns
    /// `Ok(())` if the value was successfully deleted from both caches
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        // Delete from both caches
        self.memory_cache._Fdelete(key).await?;
        self.redis_cache._Fdelete(key).await?;
        Ok(())
    }
    
    /// Checks if a key exists in either cache.
    /// 
    /// First checks the in-memory cache, then Redis if not found.
    /// 
    /// # Parameters
    /// - `key`: Cache key to check
    /// 
    /// # Returns
    /// `true` if the key exists in either cache, otherwise `false`
    async fn _Fexists(&self, key: &str) -> bool {
        // Check memory first, then Redis
        self.memory_cache._Fexists(key).await || self.redis_cache._Fexists(key).await
    }
    
    /// Clears both caches.
    /// 
    /// Removes all entries from both the in-memory cache and Redis.
    /// 
    /// # Returns
    /// `Ok(())` if both caches were successfully cleared
    async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        // Clear both caches
        self.memory_cache._Fclear().await?;
        self.redis_cache._Fclear().await?;
        Ok(())
    }
    
    /// Gets combined statistics from both caches.
    /// 
    /// Aggregates statistics from both the in-memory cache and Redis,
    /// including total keys, memory usage, hit/miss counts, and eviction counts.
    /// 
    /// # Returns
    /// A `CacheStats` struct containing combined statistics from both caches
    async fn _Fstats(&self) -> CacheStats {
        let memory_stats = self.memory_cache._Fstats().await;
        let redis_stats = self.redis_cache._Fstats().await;
        
        CacheStats {
            total_keys: memory_stats.total_keys + redis_stats.total_keys,
            memory_usage_bytes: memory_stats.memory_usage_bytes + redis_stats.memory_usage_bytes,
            hit_count: memory_stats.hit_count + redis_stats.hit_count,
            miss_count: memory_stats.miss_count + redis_stats.miss_count,
            eviction_count: memory_stats.eviction_count + redis_stats.eviction_count,
            avg_hit_rate: (memory_stats.avg_hit_rate + redis_stats.avg_hit_rate) / 2.0,
        }
    }
    
    /// Cleans up expired entries from both caches.
    /// 
    /// Removes expired entries from both the in-memory cache and Redis.
    /// 
    /// # Returns
    /// The total number of expired entries cleaned up from both caches
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        let memory_cleaned = self.memory_cache._Fcleanup_expired().await?;
        let redis_cleaned = self.redis_cache._Fcleanup_expired().await?;
        Ok(memory_cleaned + redis_cleaned)
    }
}