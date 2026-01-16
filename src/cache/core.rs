//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! Cache implementation for DMSC Core

use crate::core::{DMSCResult, DMSCError};
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Cache trait for DMSC cache implementations
#[async_trait::async_trait]
pub trait DMSCCache: Send + Sync {
    async fn get(&self, key: &str) -> DMSCResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> DMSCResult<()>;
    async fn delete(&self, key: &str) -> DMSCResult<bool>;
    async fn clear(&self) -> DMSCResult<()>;
    async fn stats(&self) -> DMSCCacheStats;
    async fn cleanup_expired(&self) -> DMSCResult<usize>;
    async fn exists(&self, key: &str) -> bool;
    async fn keys(&self) -> DMSCResult<Vec<String>>;

    async fn get_multi(&self, keys: &[&str]) -> DMSCResult<Vec<Option<String>>> {
        let mut results = Vec::with_capacity(keys.len());
        for &key in keys {
            results.push(self.get(key).await?);
        }
        Ok(results)
    }

    async fn set_multi(&self, items: &[(&str, &str)], ttl_seconds: Option<u64>) -> DMSCResult<()> {
        for &(key, value) in items {
            self.set(key, value, ttl_seconds).await?;
        }
        Ok(())
    }

    async fn delete_multi(&self, keys: &[&str]) -> DMSCResult<usize> {
        let mut count = 0;
        for &key in keys {
            if self.delete(key).await? {
                count += 1;
            }
        }
        Ok(count)
    }

    async fn exists_multi(&self, keys: &[&str]) -> DMSCResult<Vec<bool>> {
        let mut results = Vec::with_capacity(keys.len());
        for &key in keys {
            results.push(self.exists(key).await);
        }
        Ok(results)
    }

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
    #[pyo3(get, set)]
    pub hits: u64,
    #[pyo3(get, set)]
    pub misses: u64,
    #[pyo3(get, set)]
    pub entries: usize,
    #[pyo3(get, set)]
    pub memory_usage_bytes: usize,
    #[pyo3(get, set)]
    pub avg_hit_rate: f64,
    #[pyo3(get, set)]
    pub hit_count: u64,
    #[pyo3(get, set)]
    pub miss_count: u64,
    #[pyo3(get, set)]
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
    #[pyo3(get, set)]
    pub value: String,
    /// Optional expiration timestamp (UNIX epoch seconds)
    /// If None, the value never expires based on TTL
    #[pyo3(get, set)]
    pub expires_at: Option<u64>,
    /// Optional last access timestamp (UNIX epoch seconds)
    /// Used for LRU-based cache eviction policies
    #[pyo3(get, set)]
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
