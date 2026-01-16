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

//! # Hybrid Cache Backend
//!
//! This module provides a hybrid cache implementation that combines the speed of
//! in-memory caching with the persistence and distributed capabilities of Redis.
//! It follows a two-level caching strategy for optimal performance and reliability.
//!
//! ## Key Features
//!
//! - **Two-Level Caching**: Fast in-memory access with Redis persistence
//! - **Automatic Sync**: Writes propagated to both cache levels
//! - **Lazy Population**: Redis results cached in memory for future requests
//! - **Consistency**: Deletes affect both caches simultaneously
//!
//! ## Caching Strategy
//!
//! 1. **Read Operations**: Check memory first, then Redis, populate memory on miss
//! 2. **Write Operations**: Write to both memory and Redis simultaneously
//! 3. **Delete Operations**: Remove from both caches for consistency
//! 4. **Clear Operations**: Clear both caches together
//!
//! ## Design Principles
//!
//! 1. **Performance**: Fast in-memory access for frequently accessed data
//! 2. **Persistence**: Redis provides data persistence and crash recovery
//! 3. **Consistency**: Writes are propagated to both caches
//! 4. **Scalability**: Redis enables distributed caching across multiple instances
//! 5. **Efficiency**: Automatic caching of Redis results in memory
//! 6. **Transparency**: Implements the same DMSCCache trait as other backends
//!
//! ## Usage Example
//!
//! ```rust
//! use dmsc::cache::backends::DMSCHybridCache;
//!
//! async fn example() -> dmsc::core::DMSCResult<()> {
//!     // Create a hybrid cache with Redis connection
//!     let hybrid_cache = DMSCHybridCache::new("redis://localhost:6379").await?;
//!
//!     // Set a value (stored in both memory and Redis)
//!     hybrid_cache.set("user:123", "{\"name\": \"Alice\"}", Some(3600)).await?;
//!
//!     // Get a value (checked in memory first, then Redis)
//!     let value = hybrid_cache.get("user:123").await?;
//!
//!     // Delete a value (removed from both caches)
//!     hybrid_cache.delete("user:123").await?;
//!
//!     // Get combined statistics
//!     let stats = hybrid_cache.stats().await;
//!
//!     Ok(())
//! }
//! ```

#![allow(non_snake_case)]

use std::sync::Arc;
use crate::cache::{DMSCCache, DMSCCacheStats};
use crate::core::DMSCResult;

/// Hybrid cache implementation combining memory and Redis backends.
///
/// This struct implements a two-level caching strategy that leverages both
/// in-memory caching for speed and Redis for persistence and distributed caching.
pub struct DMSCHybridCache {
    /// Fast in-memory cache
    memory_cache: Arc<crate::cache::backends::DMSCMemoryCache>,
    /// Persistent Redis cache
    redis_cache: Arc<crate::cache::backends::DMSCRedisCache>,
}

impl DMSCHybridCache {
    /// Creates a new hybrid cache instance.
    ///
    /// # Parameters
    ///
    /// - `redis_url`: Redis connection URL (e.g., "redis://localhost:6379")
    ///
    /// # Returns
    ///
    /// A new instance of `DMSCHybridCache`
    pub async fn new(redis_url: &str) -> crate::core::DMSCResult<Self> {
        let memory_cache = Arc::new(crate::cache::backends::DMSCMemoryCache::new());
        let redis_cache = Arc::new(crate::cache::backends::DMSCRedisCache::new(redis_url).await?);

        Ok(Self {
            memory_cache,
            redis_cache,
        })
    }
}

#[async_trait::async_trait]
impl DMSCCache for DMSCHybridCache {
    /// Gets a value from the hybrid cache.
    ///
    /// Follows a two-level lookup strategy:
    /// 1. First checks the in-memory cache for fast access
    /// 2. If not found, checks Redis
    /// 3. If found in Redis, caches the result in memory for future requests
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to retrieve
    ///
    /// # Returns
    ///
    /// `Option<String>` containing the value if the key exists in either cache, otherwise `None`
    async fn get(&self, key: &str) -> DMSCResult<Option<String>> {
        // First check memory cache
        if let Ok(Some(value)) = self.memory_cache.get(key).await {
            return Ok(Some(value));
        }

        // If not in memory, check Redis
        if let Ok(Some(value)) = self.redis_cache.get(key).await {
            // Store in memory cache for future requests
            let _ = self.memory_cache.set(key, &value, Some(3600)).await;
            return Ok(Some(value));
        }

        Ok(None)
    }

    /// Sets a value in both caches.
    ///
    /// Writes the value to both the in-memory cache and Redis simultaneously
    /// to ensure consistency across both cache levels.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to set
    /// - `value`: Value to store in the cache
    /// - `ttl_seconds`: Optional TTL in seconds
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was successfully set in both caches
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::core::DMSCResult<()> {
        // Set in both caches
        self.memory_cache.set(key, value, ttl_seconds).await?;
        self.redis_cache.set(key, value, ttl_seconds).await?;
        Ok(())
    }

    /// Deletes a value from both caches.
    ///
    /// Removes the value from both the in-memory cache and Redis to ensure
    /// consistency across both cache levels.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to delete
    ///
    /// # Returns
    ///
    /// `Ok(bool)` indicating whether the key was found in either cache
    async fn delete(&self, key: &str) -> crate::core::DMSCResult<bool> {
        // Delete from both caches
        let memory_deleted = self.memory_cache.delete(key).await?;
        let redis_deleted = self.redis_cache.delete(key).await?;
        Ok(memory_deleted || redis_deleted)
    }

    /// Checks if a key exists in either cache.
    ///
    /// First checks the in-memory cache, then Redis if not found.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists in either cache, otherwise `false`
    async fn exists(&self, key: &str) -> bool {
        // Check memory first, then Redis
        self.memory_cache.exists(key).await || self.redis_cache.exists(key).await
    }

    /// Gets all cache keys from both caches.
    ///
    /// Returns keys from both the in-memory cache and Redis.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<String>>` containing all cache keys
    async fn keys(&self) -> crate::core::DMSCResult<Vec<String>> {
        let memory_keys = self.memory_cache.keys().await?;
        let redis_keys = self.redis_cache.keys().await?;

        // Combine and deduplicate keys
        let mut all_keys = memory_keys;
        for key in redis_keys {
            if !all_keys.contains(&key) {
                all_keys.push(key);
            }
        }

        Ok(all_keys)
    }

    /// Clears both caches.
    ///
    /// Removes all entries from both the in-memory cache and Redis.
    ///
    /// # Returns
    ///
    /// `Ok(())` if both caches were successfully cleared
    async fn clear(&self) -> crate::core::DMSCResult<()> {
        // Clear both caches
        self.memory_cache.clear().await?;
        self.redis_cache.clear().await?;
        Ok(())
    }

    /// Gets combined statistics from both caches.
    ///
    /// Aggregates statistics from both the in-memory cache and Redis,
    /// including total keys, memory usage, hit/miss counts, and eviction counts.
    ///
    /// # Returns
    ///
    /// A `DMSCCacheStats` struct containing combined statistics from both caches
    async fn stats(&self) -> DMSCCacheStats {
        let memory_stats = self.memory_cache.stats().await;
        let redis_stats = self.redis_cache.stats().await;

        DMSCCacheStats {
            hits: memory_stats.hits + redis_stats.hits,
            misses: memory_stats.misses + redis_stats.misses,
            entries: memory_stats.entries + redis_stats.entries,
            memory_usage_bytes: memory_stats.memory_usage_bytes + redis_stats.memory_usage_bytes,
            avg_hit_rate: (memory_stats.avg_hit_rate + redis_stats.avg_hit_rate) / 2.0,
            hit_count: memory_stats.hit_count + redis_stats.hit_count,
            miss_count: memory_stats.miss_count + redis_stats.miss_count,
            eviction_count: memory_stats.eviction_count + redis_stats.eviction_count,
        }
    }

    /// Cleans up expired entries from both caches.
    ///
    /// Removes expired entries from both the in-memory cache and Redis.
    ///
    /// # Returns
    ///
    /// The total number of expired entries cleaned up from both caches
    async fn cleanup_expired(&self) -> crate::core::DMSCResult<usize> {
        let memory_cleaned = self.memory_cache.cleanup_expired().await?;
        let redis_cleaned = self.redis_cache.cleanup_expired().await?;
        Ok(memory_cleaned + redis_cleaned)
    }
}
