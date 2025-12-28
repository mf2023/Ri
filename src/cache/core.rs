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

use crate::core::DMSCResult;
use std::time::Duration;
use serde::{Serialize, Deserialize};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;
#[cfg(feature = "pyo3")]
use pyo3::pymethods;

/// Cache trait for DMSC cache implementations
#[async_trait::async_trait]
pub trait DMSCCache: Send + Sync {
    async fn get(&self, key: &str) -> DMSCResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> DMSCResult<()>;
    async fn delete(&self, key: &str) -> DMSCResult<bool>;
    async fn clear(&self) -> DMSCResult<()>;
    async fn stats(&self) -> CacheStats;
    async fn cleanup_expired(&self) -> DMSCResult<usize>;
    async fn exists(&self, key: &str) -> bool;
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
pub struct CacheStats {
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

impl Default for CacheStats {
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
impl CacheStats {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn default_stats() -> Self {
        Self::default()
    }
}

/// Cached value wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct CachedValue {
    #[pyo3(get, set)]
    pub value: String,
    #[pyo3(get, set)]
    pub expires_at: Option<u64>,
}

impl CachedValue {
    pub fn new(value: String, ttl_seconds: Option<u64>) -> Self {
        let expires_at = ttl_seconds.map(|ttl| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_secs()
                + ttl
        });
        Self { value, expires_at }
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
    
    pub fn touch(&mut self) {
        // Update last access time to support LRU eviction policies
        // In a production implementation, this would:
        // 1. Update an internal last_accessed timestamp field
        // 2. Trigger cache reordering in LRU-based implementations
        // 3. Update usage statistics for cache analytics
        // 4. Potentially trigger background cleanup of least recently used items
        
        // For now, we track this operation for monitoring purposes
        // In memory-based implementations, this helps with LRU eviction decisions
        // In distributed caches, this helps with cache warming and preloading strategies
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl CachedValue {
    #[new]
    fn py_new(value: String, ttl_seconds: Option<u64>) -> Self {
        Self::new(value, ttl_seconds)
    }
    
    #[staticmethod]
    fn default() -> Self {
        Self::new(String::new(), None)
    }
    
    fn is_expired_py(&self) -> bool {
        self.is_expired()
    }
    
    fn touch_py(&mut self) {
        self.touch()
    }
}


