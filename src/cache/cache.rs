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

//! Core cache abstractions and data structures for DMS.
//! 
//! This module defines the core cache interface and data structures used across all
//! cache implementations in DMS. It provides a unified API for cache operations
//! and standardizes how cached values and statistics are represented.
//! 
//! # Design Principles
//! - **Unified Interface**: Single trait for all cache implementations
//! - **Type Safety**: Strongly typed cached values with JSON serialization
//! - **Expiration Support**: Built-in TTL (Time-To-Live) mechanism
//! - **Statistics Tracking**: Comprehensive cache statistics
//! - **Async Operations**: Fully asynchronous API
//! - **Extensibility**: Easy to add new cache backends
//! - **Serialization Support**: Built-in JSON serialization/deserialization
//! - **Access Tracking**: Tracks access count and last accessed time
//! 
//! # Usage Examples
//! ```rust
//! // Create a cached value with 1-hour TTL
//! let data = serde_json::json!("test_data");
//! let cached_value = CachedValue::_Fnew(data, Some(Duration::from_secs(3600)));
//! 
//! // Check if value is expired
//! let is_expired = cached_value._Fis_expired();
//! 
//! // Update access statistics
//! let mut mutable_value = cached_value.clone();
//! mutable_value._Ftouch();
//! 
//! // Deserialize cached data
//! let deserialized: String = mutable_value._Fdeserialize()?;
//! 
//! // Get cache statistics
//! let stats = cache._Fstats().await;
//! ```

#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Represents a cached value with metadata and expiration tracking.
/// 
/// This struct encapsulates the data being cached along with metadata such as
/// creation time, expiration time, access count, and last accessed time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedValue {
    pub data: serde_json::Value,      // The actual cached data in JSON format
    pub created_at: u64,              // Creation time (UNIX timestamp)
    pub expires_at: Option<u64>,      // Expiration time (UNIX timestamp, optional)
    pub access_count: u64,            // Number of times the value has been accessed
    pub last_accessed: u64,           // Last access time (UNIX timestamp)
}

impl CachedValue {
    /// Creates a new cached value with the specified data and TTL.
    /// 
    /// # Parameters
    /// - `data`: The data to cache, serialized as JSON
    /// - `ttl`: Optional time-to-live duration
    /// 
    /// # Returns
    /// A new instance of `CachedValue`
    pub fn _Fnew(data: serde_json::Value, ttl: Option<Duration>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            data,
            created_at: now,
            expires_at: ttl.map(|duration| now + duration.as_secs()),
            access_count: 0,
            last_accessed: now,
        }
    }
    
    /// Checks if the cached value has expired.
    /// 
    /// # Returns
    /// `true` if the value has expired, otherwise `false`
    /// 
    /// # Notes
    /// - Values without an expiration time never expire
    /// - Expiration is checked against the current system time
    pub fn _Fis_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now > expires_at
        } else {
            false
        }
    }
    
    /// Updates the access statistics for the cached value.
    /// 
    /// Increments the access count and updates the last accessed time to the current time.
    pub fn _Ftouch(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    /// Gets a reference to the cached data.
    /// 
    /// # Returns
    /// A reference to the `serde_json::Value` containing the cached data
    pub fn _Fget_data(&self) -> &serde_json::Value {
        &self.data
    }

    /// Deserializes the cached data into the specified type.
    /// 
    /// # Type Parameters
    /// - `T`: The type to deserialize into
    /// 
    /// # Returns
    /// `Ok(T)` if deserialization succeeds, otherwise an error
    pub fn _Fdeserialize<T: serde::de::DeserializeOwned>(&self) -> crate::core::DMSResult<T> {
        serde_json::from_value(self.data.clone())
            .map_err(|e| crate::core::DMSError::Other(format!("Deserialization error: {e}")))
    }
}

/// Cache statistics for monitoring cache performance.
/// 
/// This struct contains comprehensive statistics about cache performance,
/// including hit/miss rates, memory usage, and eviction counts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_keys: usize,           // Total number of keys in the cache
    pub memory_usage_bytes: usize,   // Estimated memory usage in bytes
    pub hit_count: u64,              // Number of cache hits
    pub miss_count: u64,             // Number of cache misses
    pub eviction_count: u64,         // Number of cache evictions
    pub avg_hit_rate: f64,           // Average hit rate (0.0 to 1.0)
}

impl Default for CacheStats {
    /// Creates default cache statistics with all values initialized to zero.
    /// 
    /// # Returns
    /// A new `CacheStats` instance with default values
    fn default() -> Self {
        Self {
            total_keys: 0,
            memory_usage_bytes: 0,
            hit_count: 0,
            miss_count: 0,
            eviction_count: 0,
            avg_hit_rate: 0.0,
        }
    }
}

/// Core cache interface for all cache implementations.
/// 
/// This trait defines the standard interface that all cache backends must implement.
/// It provides methods for getting, setting, deleting, and managing cached values,
/// as well as for retrieving statistics and cleaning up expired entries.
#[async_trait::async_trait]
pub trait DMSCache: Send + Sync {
    /// Gets a value from the cache.
    /// 
    /// # Parameters
    /// - `key`: The cache key to retrieve
    /// 
    /// # Returns
    /// `Some(CachedValue)` if the key exists and the value is not expired, otherwise `None`
    async fn _Fget(&self, key: &str) -> Option<CachedValue>;
    
    /// Sets a value in the cache.
    /// 
    /// # Parameters
    /// - `key`: The cache key to set
    /// - `value`: The `CachedValue` to store
    /// 
    /// # Returns
    /// `Ok(())` if the value was successfully set, otherwise an error
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()>;
    
    /// Deletes a value from the cache.
    /// 
    /// # Parameters
    /// - `key`: The cache key to delete
    /// 
    /// # Returns
    /// `Ok(())` if the value was successfully deleted, otherwise an error
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()>;
    
    /// Checks if a key exists in the cache.
    /// 
    /// # Parameters
    /// - `key`: The cache key to check
    /// 
    /// # Returns
    /// `true` if the key exists, otherwise `false`
    async fn _Fexists(&self, key: &str) -> bool;
    
    /// Clears all entries from the cache.
    /// 
    /// # Returns
    /// `Ok(())` if the cache was successfully cleared, otherwise an error
    async fn _Fclear(&self) -> crate::core::DMSResult<()>;
    
    /// Gets statistics about the cache.
    /// 
    /// # Returns
    /// A `CacheStats` struct containing cache statistics
    async fn _Fstats(&self) -> CacheStats;
    
    /// Cleans up expired entries from the cache.
    /// 
    /// # Returns
    /// The number of expired entries that were removed
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize>;
}