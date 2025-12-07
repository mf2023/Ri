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

//! Redis cache implementation for DMS.
//! 
//! This module provides a Redis-based cache implementation that offers persistence,
//! distributed caching capabilities, and automatic expiration handling. It implements
//! the `DMSCache` trait for consistency with other cache backends.
//! 
//! # Design Principles
//! - **Persistence**: Redis provides data persistence to disk
//! - **Distributed**: Supports distributed caching across multiple instances
//! - **Automatic Expiration**: Leverages Redis' built-in TTL (Time-To-Live) mechanism
//! - **Connection Management**: Uses Redis connection pooling for efficient resource usage
//! - **Statistics Tracking**: Tracks hit/miss counts and error rates
//! - **Safety**: Uses pattern matching to avoid clearing all Redis data
//! - **Async Operations**: Fully asynchronous implementation
//! 
//! # Usage Examples
//! ```rust
//! // Create a Redis cache instance
//! let redis_cache = DMSRedisCache::new("redis://localhost:6379").await?;
//! 
//! // Set a value with expiration
//! let cached_value = CachedValue {
//!     value: b"test_data".to_vec(),
//!     expires_at: Some(SystemTime::now() + Duration::from_secs(3600)),
//!     metadata: HashMap::new(),
//! };
//! redis_cache.set("test_key", cached_value).await?;
//! 
//! // Get a value
//! let value = redis_cache.get("test_key").await;
//! 
//! // Check if a key exists
//! let exists = redis_cache.exists("test_key").await;
//! 
//! // Delete a value
//! redis_cache.delete("test_key").await?;
//! 
//! // Get cache statistics
//! let stats = redis_cache.stats().await;
//! ```

#![allow(non_snake_case)]

use redis::{AsyncCommands, Client};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use std::ops::AddAssign;
use crate::cache::{DMSCache, CachedValue, CacheStats};
use crate::core::DMSResult;

/// Redis cache implementation.
/// 
/// This struct provides a Redis-based cache implementation that leverages Redis' 
/// persistence, distributed capabilities, and built-in expiration mechanism.
pub struct DMSRedisCache {
    connection: Arc<ConnectionManager>,       // Redis connection manager for efficient connection pooling
    stats: Arc<dashmap::DashMap<&'static str, u64>>, // Thread-safe statistics tracking
}

impl DMSRedisCache {
    /// Creates a new Redis cache instance.
    /// 
    /// # Parameters
    /// - `redis_url`: Redis connection URL (e.g., "redis://localhost:6379")
    /// 
    /// # Returns
    /// A new instance of `DMSRedisCache`
    /// 
    /// # Errors
    /// Returns an error if the Redis client cannot be created or if the connection fails
    pub async fn new(redis_url: &str) -> crate::core::DMSResult<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| crate::core::DMSError::Other(format!("Redis client error: {e}")))?;
        
        let connection = ConnectionManager::new(client).await
            .map_err(|e| crate::core::DMSError::Other(format!("Redis connection error: {e}")))?;
        
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
impl DMSCache for DMSRedisCache {
    /// Gets a value from Redis cache.
    /// 
    /// # Parameters
    /// - `key`: Cache key to retrieve
    /// 
    /// # Returns
    /// `Some(CachedValue)` if the key exists and the value is not expired, otherwise `None`
    /// 
    /// # Implementation Details
    /// 1. Retrieves the JSON-encoded value from Redis
    /// 2. Deserializes the value to `CachedValue`
    /// 3. Checks if the value is expired
    /// 4. If expired, deletes the key from Redis and returns `None`
    /// 5. Otherwise, updates hit count and returns the value
    async fn get(&self, key: &str) -> DMSResult<Option<String>> {
        let mut conn = (*self.connection).clone();
        
        let result: redis::RedisResult<String> = conn.get(key).await;
        match result {
            Ok(json_str) => {
                let json_str_owned = json_str.to_owned();
                // Try to parse as simple string first, then as JSON if that fails
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&json_str_owned) {
                    if let Some(str_value) = value.as_str() {
                        self.stats.get_mut("hit_count").unwrap().value_mut().add_assign(1);
                        Ok(Some(str_value.to_string()))
                    } else {
                        self.stats.get_mut("error_count").unwrap().value_mut().add_assign(1);
                        Ok(None)
                    }
                } else {
                    // If not valid JSON, treat as plain string
                    self.stats.get_mut("hit_count").unwrap().value_mut().add_assign(1);
                    Ok(Some(json_str_owned))
                }
            }
            Err(_) => {
                self.stats.get_mut("miss_count").unwrap().value_mut().add_assign(1);
                Ok(None)
            }
        }
    }
    
    /// Sets a value in Redis cache.
    /// 
    /// # Parameters
    /// - `key`: Cache key to set
    /// - `value`: Value to store in the cache
    /// - `ttl_seconds`: Optional TTL in seconds
    /// 
    /// # Returns
    /// `Ok(())` if the value was successfully set, otherwise an error
    /// 
    /// # Implementation Details
    /// 1. Creates a CachedValue from the string value
    /// 2. Serializes the CachedValue to JSON
    /// 3. Calculates the TTL (Time-To-Live) based on the ttl_seconds parameter
    /// 4. Uses `SET` or `SETEX` command depending on whether TTL is specified
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> crate::core::DMSResult<()> {
        let mut conn = (*self.connection).clone();
        
        let cached_value = CachedValue {
            value: value.to_string(),
            expires_at: ttl_seconds,
        };
        
        let result: redis::RedisResult<()> = match ttl_seconds {
            Some(ttl_secs) => {
                conn.set_ex(key, cached_value.value, ttl_secs).await
            }
            None => {
                conn.set(key, cached_value.value).await
            }
        };
        
        result.map_err(|e| crate::core::DMSError::Other(format!("Redis set error: {e}")))?;
        Ok(())
    }
    
    /// Deletes a value from Redis cache.
    /// 
    /// # Parameters
    /// - `key`: Cache key to delete
    /// 
    /// # Returns
    /// `Ok(true)` if the key was found and deleted, `Ok(false)` if the key didn't exist
    async fn delete(&self, key: &str) -> crate::core::DMSResult<bool> {
        let mut conn = (*self.connection).clone();
        let result: redis::RedisResult<bool> = conn.del(key).await;
        result.map_err(|e| crate::core::DMSError::Other(format!("Redis delete error: {e}")))
    }
    
    /// Checks if a key exists in Redis cache.
    /// 
    /// # Parameters
    /// - `key`: Cache key to check
    /// 
    /// # Returns
    /// `true` if the key exists, otherwise `false`
    async fn exists(&self, key: &str) -> bool {
        let mut conn = (*self.connection).clone();
        
        let result: redis::RedisResult<bool> = conn.exists(key).await;
        result.unwrap_or_default()
    }
    
    /// Clears all DMS-related cache entries from Redis.
    /// 
    /// # Returns
    /// `Ok(())` if the cache was successfully cleared, otherwise an error
    /// 
    /// # Notes
    /// - Uses the pattern "dms:cache:*" to avoid clearing all Redis data
    /// - Only clears keys matching the DMS cache pattern
    async fn clear(&self) -> crate::core::DMSResult<()> {
        let mut conn = (*self.connection).clone();
        
        // Use a specific pattern to avoid clearing all Redis data
        let pattern = "dms:cache:*";
        let keys: Vec<String> = conn.keys(pattern).await
            .map_err(|e| crate::core::DMSError::Other(format!("Redis keys error: {e}")))?;
        
        if !keys.is_empty() {
            conn.del::<_, ()>(keys).await
                .map_err(|e| crate::core::DMSError::Other(format!("Redis clear error: {e}")))?;
        }
        
        Ok(())
    }
    
    /// Gets cache statistics.
    /// 
    /// # Returns
    /// A `CacheStats` struct containing cache statistics
    /// 
    /// # Statistics Included
    /// - Total keys (approximate using DBSIZE command)
    /// - Hit count
    /// - Miss count
    /// - Error count (used as eviction count)
    /// - Average hit rate
    /// - Memory usage (always 0 as Redis manages memory)
    async fn stats(&self) -> CacheStats {
        let hit_count = *self.stats.get("hit_count").unwrap().value();
        let miss_count = *self.stats.get("miss_count").unwrap().value();
        let error_count = *self.stats.get("error_count").unwrap().value();
        
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
        
        CacheStats {
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
    /// Always returns `Ok(0)` as Redis automatically handles expiration
    /// 
    /// # Notes
    /// Redis uses an active expiration policy with lazy deletion, so no manual cleanup is needed
    async fn cleanup_expired(&self) -> crate::core::DMSResult<usize> {
        // Redis automatically handles expiration
        Ok(0)
    }
}