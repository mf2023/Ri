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

#![cfg(feature = "redis")]

//! # Redis Cache Backend
//!
//! This module provides a Redis-based cache implementation that offers persistence,
//! distributed caching capabilities, and automatic expiration handling. It implements
//! the [`RiCache`](crate::cache::RiCache) trait for consistency with other cache backends.
//!
//! ## Key Features
//!
//! - **Persistence**: Redis provides data persistence to disk
//! - **Distributed**: Supports distributed caching across multiple instances
//! - **Automatic Expiration**: Leverages Redis' built-in TTL (Time-To-Live) mechanism
//! - **Connection Pooling**: Uses Redis connection pooling for efficient resource usage
//! - **Statistics Tracking**: Tracks hit/miss counts and error rates
//! - **Safety**: Uses pattern matching to avoid clearing all Redis data
//! - **Async Operations**: Fully asynchronous implementation
//!
//! ## Design Principles
//!
//! 1. **Reliability**: Leverages Redis' proven persistence and clustering capabilities
//! 2. **Efficiency**: Connection pooling for optimal resource utilization
//! 3. **Consistency**: Same interface as other backends via RiCache trait
//! 4. **Observability**: Comprehensive statistics for monitoring cache performance
//! 5. **Safety First**: Pattern-based operations prevent accidental data loss
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use ri::cache::backends::RiRedisCache;
//!
//! async fn example() -> ri::core::RiResult<()> {
//!     // Create a Redis cache instance
//!     let redis_cache = RiRedisCache::new("redis://localhost:6379").await?;
//!
//!     // Set a value with expiration
//!     redis_cache.set("user:123", "{\"name\": \"Alice\"}", Some(3600)).await?;
//!
//!     // Get a value
//!     let value = redis_cache.get("user:123").await?;
//!
//!     // Check if a key exists
//!     let exists = redis_cache.exists("user:123").await;
//!
//!     // Delete a value
//!     redis_cache.delete("user:123").await?;
//!
//!     // Get cache statistics
//!     let stats = redis_cache.stats().await;
//!
//!     Ok(())
//! }
//! ```

#![allow(non_snake_case)]

use redis::{AsyncCommands, Client};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use std::ops::AddAssign;
use crate::cache::{RiCache, RiCacheStats};
use crate::core::RiResult;

/// Redis cache implementation.
///
/// This struct provides a Redis-based cache implementation that leverages Redis'
/// persistence, distributed capabilities, and built-in expiration mechanism.
pub struct RiRedisCache {
    /// Redis connection manager for efficient connection pooling
    connection: Arc<ConnectionManager>,
    /// Thread-safe statistics tracking
    stats: Arc<dashmap::DashMap<&'static str, u64>>,
}

impl RiRedisCache {
    /// Creates a new Redis cache instance.
    ///
    /// # Parameters
    ///
    /// - `redis_url`: Redis connection URL (e.g., "redis://localhost:6379")
    ///
    /// # Returns
    ///
    /// A new instance of `RiRedisCache`
    ///
    /// # Errors
    ///
    /// Returns an error if the Redis client cannot be created or if the connection fails
    pub async fn new(redis_url: &str) -> crate::core::RiResult<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| crate::core::RiError::Other(format!("Redis client error: {e}")))?;

        let connection = ConnectionManager::new(client).await
            .map_err(|e| crate::core::RiError::Other(format!("Redis connection error: {e}")))?;

        let stats = dashmap::DashMap::new();
        stats.insert("hit_count", 0);
        stats.insert("miss_count", 0);
        stats.insert("error_count", 0);

        Ok(Self {
            connection: Arc::new(connection),
            stats: Arc::new(stats),
        })
    }
}

#[async_trait::async_trait]
impl RiCache for RiRedisCache {
    /// Gets a value from Redis cache.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to retrieve
    ///
    /// # Returns
    ///
    /// `Option<String>` containing the value if the key exists, otherwise `None`
    ///
    /// # Implementation Details
    ///
    /// 1. Retrieves the value from Redis
    /// 2. Attempts to parse as JSON string
    /// 3. Updates hit/miss statistics accordingly
    /// 4. Returns the parsed string value
    async fn get(&self, key: &str) -> RiResult<Option<String>> {
        let mut conn = (*self.connection).clone();

        let result: redis::RedisResult<String> = conn.get(key).await;
        match result {
            Ok(json_str) => {
                let json_str_owned = json_str.to_owned();
                // Try to parse as simple string first, then as JSON if that fails
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str_owned) {
                    if let Some(str_value) = value.as_str() {
                        if let Some(mut hit_count) = self.stats.get_mut("hit_count") {
                            hit_count.value_mut().add_assign(1);
                        }
                        Ok(Some(str_value.to_string()))
                    } else {
                        if let Some(mut error_count) = self.stats.get_mut("error_count") {
                            error_count.value_mut().add_assign(1);
                        }
                        Ok(None)
                    }
                } else {
                    // If not valid JSON, treat as plain string
                    if let Some(mut hit_count) = self.stats.get_mut("hit_count") {
                        hit_count.value_mut().add_assign(1);
                    }
                    Ok(Some(json_str_owned))
                }
            }
            Err(_) => {
                if let Some(mut miss_count) = self.stats.get_mut("miss_count") {
                    miss_count.value_mut().add_assign(1);
                }
                Ok(None)
            }
        }
    }

    /// Sets a value in Redis cache.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to set
    /// - `value`: Value to store in the cache
    /// - `ttl_seconds`: Optional TTL in seconds
    ///
    /// # Returns
    ///
    /// `Ok(())` if the value was successfully set, otherwise an error
    ///
    /// # Implementation Details
    ///
    /// 1. Serializes the string value
    /// 2. Uses SET or SETEX command depending on TTL specification
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::core::RiResult<()> {
        let mut conn = (*self.connection).clone();

        let result: redis::RedisResult<()> = match ttl_seconds {
            Some(ttl_secs) => {
                conn.set_ex(key, value, ttl_secs).await
            }
            None => {
                conn.set(key, value).await
            }
        };

        result.map_err(|e| crate::core::RiError::Other(format!("Redis set error: {e}")))?;
        Ok(())
    }

    /// Deletes a value from Redis cache.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to delete
    ///
    /// # Returns
    ///
    /// `Ok(true)` if the key was found and deleted, `Ok(false)` if the key didn't exist
    async fn delete(&self, key: &str) -> crate::core::RiResult<bool> {
        let mut conn = (*self.connection).clone();
        let result: redis::RedisResult<bool> = conn.del(key).await;
        result.map_err(|e| crate::core::RiError::Other(format!("Redis delete error: {e}")))
    }

    /// Checks if a key exists in Redis cache.
    ///
    /// # Parameters
    ///
    /// - `key`: Cache key to check
    ///
    /// # Returns
    ///
    /// `true` if the key exists, otherwise `false`
    async fn exists(&self, key: &str) -> bool {
        let mut conn = (*self.connection).clone();

        let result: redis::RedisResult<bool> = conn.exists(key).await;
        result.unwrap_or_default()
    }

    /// Gets all cache keys from Redis.
    ///
    /// # Returns
    ///
    /// A `RiResult<Vec<String>>` containing all cache keys matching the Ri pattern
    ///
    /// # Performance Note
    ///
    /// Uses SCAN instead of KEYS to avoid blocking Redis in production environments.
    /// SCAN is an iterator-based command that doesn't block the Redis server.
    async fn keys(&self) -> crate::core::RiResult<Vec<String>> {
        let mut conn = (*self.connection).clone();

        let pattern = "ri:cache:*";
        let mut all_keys = Vec::new();
        let mut cursor: u64 = 0;

        // Use SCAN instead of KEYS to avoid blocking Redis
        loop {
            let result: redis::RedisResult<(u64, Vec<String>)> = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await;

            match result {
                Ok((new_cursor, keys)) => {
                    all_keys.extend(keys);
                    cursor = new_cursor;
                    if cursor == 0 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(crate::core::RiError::Other(format!("Redis scan error: {e}")));
                }
            }
        }

        Ok(all_keys)
    }

    /// Clears all Ri-related cache entries from Redis.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the cache was successfully cleared, otherwise an error
    ///
    /// # Notes
    ///
    /// - Uses the pattern "ri:cache:*" to avoid clearing all Redis data
    /// - Only clears keys matching the Ri cache pattern
    /// - Uses SCAN instead of KEYS to avoid blocking Redis in production
    async fn clear(&self) -> crate::core::RiResult<()> {
        let mut conn = (*self.connection).clone();

        // Use a specific pattern to avoid clearing all Redis data
        let pattern = "ri:cache:*";
        let mut cursor: u64 = 0;
        let mut total_deleted = 0;

        // Use SCAN instead of KEYS to avoid blocking Redis
        loop {
            let result: redis::RedisResult<(u64, Vec<String>)> = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await;

            match result {
                Ok((new_cursor, keys)) => {
                    if !keys.is_empty() {
                        conn.del::<_, ()>(&keys).await
                            .map_err(|e| crate::core::RiError::Other(format!("Redis clear error: {e}")))?;
                        total_deleted += keys.len();
                    }
                    cursor = new_cursor;
                    if cursor == 0 {
                        break;
                    }
                }
                Err(e) => {
                    return Err(crate::core::RiError::Other(format!("Redis scan error: {e}")));
                }
            }
        }

        log::debug!("[Ri.Redis] Cleared {} cache entries", total_deleted);
        Ok(())
    }

    /// Gets cache statistics.
    ///
    /// # Returns
    ///
    /// A `RiCacheStats` struct containing cache statistics
    ///
    /// # Statistics Included
    ///
    /// - Total keys (approximate using DBSIZE command)
    /// - Hit count
    /// - Miss count
    /// - Error count (used as eviction count)
    /// - Average hit rate
    /// - Memory usage (always 0 as Redis manages memory)
    async fn stats(&self) -> RiCacheStats {
        let hit_count = self.stats.get("hit_count")
            .map(|entry| *entry.value())
            .unwrap_or(0);
        let miss_count = self.stats.get("miss_count")
            .map(|entry| *entry.value())
            .unwrap_or(0);
        let error_count = self.stats.get("error_count")
            .map(|entry| *entry.value())
            .unwrap_or(0);

        let total_requests = hit_count + miss_count;
        let avg_hit_rate = if total_requests > 0 {
            hit_count as f64 / total_requests as f64
        } else {
            0.0
        };

        // Get total keys (approximate)
        let total_keys = match redis::cmd("DBSIZE").query_async::<_, u64>(&mut (*self.connection).clone()).await {
            Ok(size) => size as usize,
            Err(_) => 0,
        };

        RiCacheStats {
            hits: hit_count,
            misses: miss_count,
            entries: total_keys,
            memory_usage_bytes: 0, // Redis manages memory
            avg_hit_rate,
            hit_count,
            miss_count,
            eviction_count: error_count,
        }
    }

    /// Cleans up expired entries from the cache.
    ///
    /// # Returns
    ///
    /// Always returns `Ok(0)` as Redis automatically handles expiration
    ///
    /// # Notes
    ///
    /// Redis uses an active expiration policy with lazy deletion, so no manual cleanup is needed
    async fn cleanup_expired(&self) -> crate::core::RiResult<usize> {
        // Redis automatically handles expiration
        Ok(0)
    }
}
