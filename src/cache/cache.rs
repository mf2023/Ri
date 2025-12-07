//! Cache implementation for DMS Core

use crate::core::DMSResult;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};

/// Cache trait for DMS cache implementations
#[async_trait::async_trait]
pub trait DMSCache: Send + Sync {
    async fn get(&self, key: &str) -> DMSResult<Option<String>>;
    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> DMSResult<()>;
    async fn delete(&self, key: &str) -> DMSResult<bool>;
    async fn clear(&self) -> DMSResult<()>;
    async fn stats(&self) -> CacheStats;
    async fn cleanup_expired(&self) -> DMSResult<usize>;
    async fn exists(&self, key: &str) -> bool;
}

/// Cache statistics
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub entries: usize,
    pub memory_usage_bytes: usize,
    pub avg_hit_rate: f64,
    pub hit_count: u64,
    pub miss_count: u64,
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

/// Cached value wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedValue {
    pub value: String,
    pub expires_at: Option<u64>,
}

impl CachedValue {
    pub fn new(value: String, expires_at: Option<u64>) -> Self {
        Self { value, expires_at }
    }
    
    pub fn deserialize<T: serde::de::DeserializeOwned>(&self) -> crate::core::DMSResult<T> {
        serde_json::from_str(&self.value)
            .map_err(|e| crate::core::DMSError::Other(format!("Deserialization error: {e}")))
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now >= expires_at
        } else {
            false
        }
    }
    
    pub fn touch(&mut self) {
        // Update last access time if needed, for now just a placeholder
    }
}

/// In-memory cache implementation
pub struct DMSCacheImpl {
    data: Arc<RwLock<HashMap<String, (String, u64)>>>, // (value, expires_at)
    stats: Arc<RwLock<CacheStats>>,
}

impl DMSCacheImpl {
    /// Create a new cache
    pub fn new() -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(CacheStats {
                hits: 0,
                misses: 0,
                entries: 0,
                memory_usage_bytes: 0,
                avg_hit_rate: 0.0,
                hit_count: 0,
                miss_count: 0,
                eviction_count: 0,
            })),
        }
    }
}

impl Default for DMSCacheImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl DMSCache for DMSCacheImpl {
    async fn get(&self, key: &str) -> DMSResult<Option<String>> {
        let mut stats = self.stats.write().await;
        let data = self.data.read().await;
        
        if let Some((value, expires_at)) = data.get(key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if now < *expires_at {
                stats.hits += 1;
                return Ok(Some(value.clone()));
            }
        }
        
        stats.misses += 1;
        Ok(None)
    }

    async fn set(&self, key: &str, value: &str, ttl_seconds: Option<u64>) -> DMSResult<()> {
        let expires_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() + ttl_seconds.unwrap_or(3600);
        
        let mut data = self.data.write().await;
        data.insert(key.to_string(), (value.to_string(), expires_at));
        
        let mut stats = self.stats.write().await;
        stats.entries = data.len();
        
        Ok(())
    }

    async fn delete(&self, key: &str) -> DMSResult<bool> {
        let mut data = self.data.write().await;
        let removed = data.remove(key).is_some();
        
        if removed {
            let mut stats = self.stats.write().await;
            stats.entries = data.len();
        }
        
        Ok(removed)
    }

    async fn clear(&self) -> DMSResult<()> {
        let mut data = self.data.write().await;
        data.clear();
        
        let mut stats = self.stats.write().await;
        stats.entries = 0;
        
        Ok(())
    }

    async fn stats(&self) -> CacheStats {
        *self.stats.read().await
    }

    async fn cleanup_expired(&self) -> DMSResult<usize> {
        let mut data = self.data.write().await;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let initial_count = data.len();
        data.retain(|_, (_, expires_at)| now < *expires_at);
        let cleaned = initial_count - data.len();
        
        let mut stats = self.stats.write().await;
        stats.entries = data.len();
        
        Ok(cleaned)
    }

    async fn exists(&self, key: &str) -> bool {
        let data = self.data.read().await;
        if let Some((_, expires_at)) = data.get(key) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            now < *expires_at
        } else {
            false
        }
    }
}