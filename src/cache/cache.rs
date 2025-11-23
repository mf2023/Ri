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

use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedValue {
    pub data: serde_json::Value,
    pub created_at: u64,
    pub expires_at: Option<u64>,
    pub access_count: u64,
    pub last_accessed: u64,
}

impl CachedValue {
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
    
    pub fn _Ftouch(&mut self) {
        self.access_count += 1;
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
    
    pub fn _Fget_data(&self) -> &serde_json::Value {
        &self.data
    }

    pub fn _Fdeserialize<T: serde::de::DeserializeOwned>(&self) -> crate::core::DMSResult<T> {
        serde_json::from_value(self.data.clone())
            .map_err(|e| crate::core::DMSError::Other(format!("Deserialization error: {}", e)))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub total_keys: usize,
    pub memory_usage_bytes: usize,
    pub hit_count: u64,
    pub miss_count: u64,
    pub eviction_count: u64,
    pub avg_hit_rate: f64,
}

impl Default for CacheStats {
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

/// Core cache interface
#[async_trait::async_trait]
pub trait DMSCache: Send + Sync {
    /// Get value from cache
    async fn _Fget(&self, key: &str) -> Option<CachedValue>;
    
    /// Set value in cache
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()>;
    
    /// Delete value from cache
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()>;
    
    /// Check if key exists
    async fn _Fexists(&self, key: &str) -> bool;
    
    /// Clear all cache
    async fn _Fclear(&self) -> crate::core::DMSResult<()>;
    
    /// Get cache statistics
    async fn _Fstats(&self) -> CacheStats;
    
    /// Cleanup expired entries
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize>;
}