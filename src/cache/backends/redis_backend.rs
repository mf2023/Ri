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

use redis::{AsyncCommands, Client};
use redis::aio::ConnectionManager;
use std::sync::Arc;
use std::ops::AddAssign;
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// Redis cache implementation
pub struct DMSRedisCache {
    connection: Arc<ConnectionManager>,
    stats: Arc<dashmap::DashMap<&'static str, u64>>,
}

impl DMSRedisCache {
    pub async fn _Fnew(redis_url: &str) -> crate::core::DMSResult<Self> {
        let client = Client::open(redis_url)
            .map_err(|e| crate::core::DMSError::Other(format!("Redis client error: {}", e)))?;
        
        let connection = ConnectionManager::new(client).await
            .map_err(|e| crate::core::DMSError::Other(format!("Redis connection error: {}", e)))?;
        
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
    async fn _Fget(&self, key: &str) -> Option<CachedValue> {
        let mut conn = (*self.connection).clone();
        
        let result: redis::RedisResult<String> = conn.get(key).await;
        match result {
            Ok(json_str) => {
                match serde_json::from_str::<CachedValue>(&json_str) {
                    Ok(value) => {
                        if value._Fis_expired() {
                            let _: redis::RedisResult<()> = conn.del::<_, ()>(key).await;
                            self.stats.get_mut("eviction_count").unwrap().value_mut().add_assign(1);
                            self.stats.get_mut("miss_count").unwrap().value_mut().add_assign(1);
                            None
                        } else {
                            self.stats.get_mut("hit_count").unwrap().value_mut().add_assign(1);
                            Some(value)
                        }
                    }
                    Err(_) => {
                        self.stats.get_mut("error_count").unwrap().value_mut().add_assign(1);
                        None
                    }
                }
            }
            Err(_) => {
                self.stats.get_mut("miss_count").unwrap().value_mut().add_assign(1);
                None
            }
        }
    }
    
    async fn _Fset(&self, key: &str, value: CachedValue) -> crate::core::DMSResult<()> {
        let mut conn = (*self.connection).clone();
        
        let json_str = serde_json::to_string(&value)
            .map_err(|e| crate::core::DMSError::Other(format!("Serialization error: {}", e)))?;
        
        let ttl = if let Some(expires_at) = value.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if expires_at > now {
                Some(expires_at - now)
            } else {
                Some(1) // Minimum TTL
            }
        } else {
            None
        };
        
        let result: redis::RedisResult<()> = match ttl {
            Some(ttl_secs) => {
                conn.set_ex(key, json_str, ttl_secs).await
            }
            None => {
                conn.set(key, json_str).await
            }
        };
        
        result.map_err(|e| crate::core::DMSError::Other(format!("Redis set error: {}", e)))?;
        Ok(())
    }
    
    async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        let mut conn = (*self.connection).clone();
        let result: redis::RedisResult<()> = conn.del(key).await;
        result.map_err(|e| crate::core::DMSError::Other(format!("Redis delete error: {}", e)))?;
        Ok(())
    }
    
    async fn _Fexists(&self, key: &str) -> bool {
        let mut conn = (*self.connection).clone();
        
        let result: redis::RedisResult<bool> = conn.exists(key).await;
        match result {
            Ok(exists) => exists,
            Err(_) => false,
        }
    }
    
    async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        let mut conn = (*self.connection).clone();
        
        // Use a specific pattern to avoid clearing all Redis data
        let pattern = "dms:cache:*";
        let keys: Vec<String> = conn.keys(pattern).await
            .map_err(|e| crate::core::DMSError::Other(format!("Redis keys error: {}", e)))?;
        
        if !keys.is_empty() {
            conn.del::<_, ()>(keys).await
                .map_err(|e| crate::core::DMSError::Other(format!("Redis clear error: {}", e)))?;
        }
        
        Ok(())
    }
    
    async fn _Fstats(&self) -> CacheStats {
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
            total_keys,
            memory_usage_bytes: 0, // Redis manages memory
            hit_count,
            miss_count,
            eviction_count: error_count,
            avg_hit_rate,
        }
    }
    
    async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        // Redis automatically handles expiration
        Ok(0)
    }
}