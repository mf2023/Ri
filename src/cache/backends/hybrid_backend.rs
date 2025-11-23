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

use std::sync::Arc;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// Hybrid cache implementation that combines memory and Redis backends
pub struct DMSHybridCache {
    memory_cache: Arc<crate::cache::backends::DMSMemoryCache>,
    redis_cache: Arc<crate::cache::backends::DMSRedisCache>,
}

impl DMSHybridCache {
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
    
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()> {
        // Set in both caches
        self.memory_cache._Fset(key, value.clone()).await?;
        self.redis_cache._Fset(key, value).await?;
        Ok(())
    }
    
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        // Delete from both caches
        self.memory_cache._Fdelete(key).await?;
        self.redis_cache._Fdelete(key).await?;
        Ok(())
    }
    
    async fn _Fexists(&self, key: &str) -> bool {
        // Check memory first, then Redis
        self.memory_cache._Fexists(key).await || self.redis_cache._Fexists(key).await
    }
    
    async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        // Clear both caches
        self.memory_cache._Fclear().await?;
        self.redis_cache._Fclear().await?;
        Ok(())
    }
    
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
    
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        let memory_cleaned = self.memory_cache._Fcleanup_expired().await?;
        let redis_cleaned = self.redis_cache._Fcleanup_expired().await?;
        Ok(memory_cleaned + redis_cleaned)
    }
}