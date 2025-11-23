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

use dashmap::DashMap;
use std::sync::Arc;
use std::ops::AddAssign;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// In-memory cache implementation using DashMap
pub struct DMSMemoryCache {
    store: Arc<DashMap<String, CachedValue>>,
    stats: Arc<dashmap::DashMap<&'static str, u64>>,
}

impl DMSMemoryCache {
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
    
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()> {
        self.store.insert(key.to_string(), value);
        Ok(())
    }
    
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        self.store.remove(key);
        Ok(())
    }
    
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
    
    async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        self.store.clear();
        Ok(())
    }
    
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