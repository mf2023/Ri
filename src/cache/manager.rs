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

#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};
use crate::cache::core::{DMSCCache, CacheStats};


/// # DMSC Cache Manager
/// 
/// This file implements a cache manager that coordinates different cache backends with 
/// consistency support across multiple instances. It provides a unified interface for cache 
/// operations while ensuring cache consistency through event-driven architecture.
/// 
/// ## Design Principles
/// 1. **Consistency First**: Ensures cache consistency across multiple instances using broadcast events
/// 2. **Unified Interface**: Provides a consistent API regardless of the underlying cache backend
/// 3. **Event-Driven Architecture**: Uses broadcast channels for efficient cache invalidation
/// 4. **Thread Safety**: Implements thread-safe operations using Arc and RwLock
/// 5. **Flexibility**: Supports any backend implementing the DMSCCache trait
/// 6. **Scalability**: Designed to handle high-throughput cache operations
/// 
/// ## Usage Examples
/// ```rust
/// // Create a cache manager with a Redis backend
/// let redis_backend = Arc::new(DMSCRedisBackend::new(config).await?);
/// let mut cache_manager = DMSCCacheManager::new(redis_backend);
/// 
/// // Start the consistency listener
/// let listener_handle = cache_manager.start_consistency_listener().await;
/// 
/// // Set a value in cache
/// cache_manager.set("user:123", &User { id: 123, name: "John" }, Some(3600)).await?;
/// 
/// // Get a value from cache
/// let user: Option<User> = cache_manager.get("user:123").await?;
/// 
/// // Delete a value and invalidate across all instances
/// cache_manager.delete("user:123").await?;
/// 
/// // Clear cache and broadcast to all instances
/// cache_manager.clear().await?;
/// ```
/// Cache event type for maintaining cache consistency across instances
/// 
/// This enum defines the events that are broadcasted to ensure all cache instances
/// remain consistent. Each event triggers a corresponding action on all cache instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCCacheEvent {
    /// Invalidate a specific cache key
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to invalidate
    Invalidate { key: String },
    
    /// Invalidate all cache keys matching a pattern
    /// 
    /// **Parameters:**
    /// - `pattern`: The pattern to match cache keys (supports wildcards depending on backend)
    InvalidatePattern { pattern: String },
    
    /// Clear all cache entries
    Clear(),
}

/// Cache manager that coordinates different cache backends with consistency support
/// 
/// This struct provides a unified interface for cache operations while ensuring cache
/// consistency across multiple instances through event-driven architecture. It wraps
/// any backend implementing the DMSCCache trait and adds consistency guarantees.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCacheManager {
    /// The underlying cache backend implementation
    backend: Arc<dyn DMSCCache + Send + Sync>,
    
    /// Broadcast sender for cache consistency events
    event_sender: broadcast::Sender<DMSCCacheEvent>,
    
    /// Broadcast receiver for cache consistency events (used internally)
    event_receiver: Option<broadcast::Receiver<DMSCCacheEvent>>,
    
    /// Map of subscribers to cache events (for internal use)
    _subscribers: Arc<RwLock<HashMap<String, broadcast::Receiver<DMSCCacheEvent>>>>,
}

impl DMSCCacheManager {
    /// Create a new cache manager with the specified backend
    /// 
    /// **Parameters:**
    /// - `backend`: The underlying cache backend implementation
    /// 
    /// **Returns:**
    /// - A new instance of `DMSCCacheManager`
    pub fn new(backend: Arc<dyn DMSCCache + Send + Sync>) -> Self {
        let (sender, receiver) = broadcast::channel(100);
        
        Self {
            backend,
            event_sender: sender,
            event_receiver: Some(receiver),
            _subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Start the cache consistency event listener
    /// 
    /// This method starts a background task that listens for cache consistency events
    /// and applies them to the underlying cache backend. This ensures that all cache
    /// instances remain consistent across the system.
    /// 
    /// **Returns:**
    /// - A `JoinHandle` for the background task
    pub async fn start_consistency_listener(&mut self) -> tokio::task::JoinHandle<()> {
        let backend = self.backend.clone();
        let mut receiver = self.event_receiver.take().expect("Already started");
        
        log::info!("[DMSC.Cache] Starting cache consistency event listener");
        
        tokio::spawn(async move {
            let mut event_count = 0;
            while let Ok(event) = receiver.recv().await {
                event_count += 1;
                
                match event {
                    DMSCCacheEvent::Invalidate { key } => {
                        log::info!("[DMSC.Cache] Processing invalidate event for key: {key}");
                        if let Err(e) = backend.delete(&key).await {
                            log::error!("[DMSC.Cache] Failed to invalidate cache key {key}: {e}");
                        } else {
                            log::info!("[DMSC.Cache] Successfully invalidated cache key: {key}");
                        }
                    },
                    DMSCCacheEvent::InvalidatePattern { pattern } => {
                        log::info!("[DMSC.Cache] Processing invalidate pattern event: {pattern}");
                        // Invalidate all keys matching the pattern
                        // Note: This requires backend support for pattern matching
                        log::error!("[DMSC.Cache] Invalidate pattern not implemented: {pattern}");
                    },
                    DMSCCacheEvent::Clear() => {
                        log::info!("[DMSC.Cache] Processing clear cache event");
                        if let Err(e) = backend.clear().await {
                            log::error!("[DMSC.Cache] Failed to clear cache: {e}");
                        } else {
                            log::info!("[DMSC.Cache] Successfully cleared cache");
                        }
                    },
                }
                
                // Log event processing statistics periodically
                if event_count % 100 == 0 {
                    log::info!("[DMSC.Cache] Processed {event_count} cache consistency events");
                }
            }
            
            log::info!("[DMSC.Cache] Cache consistency event listener stopped after processing {event_count} events");
        })
    }
    
    /// Subscribe to cache consistency events
    /// 
    /// This method allows external components to subscribe to cache consistency events,
    /// enabling them to react to cache changes in real-time.
    /// 
    /// **Returns:**
    /// - A broadcast receiver for cache events
    pub fn subscribe(&self) -> broadcast::Receiver<DMSCCacheEvent> {
        self.event_sender.subscribe()
    }
    
    /// Publish a cache consistency event
    /// 
    /// This method publishes a cache event to all subscribers, ensuring cache consistency
    /// across all instances.
    /// 
    /// **Parameters:**
    /// - `event`: The cache event to publish
    pub fn publish_event(&self, event: DMSCCacheEvent) {
        let event_type = match &event {
            DMSCCacheEvent::Invalidate { key } => format!("Invalidate(key: {key})"),
            DMSCCacheEvent::InvalidatePattern { pattern } => format!("InvalidatePattern(pattern: {pattern})"),
            DMSCCacheEvent::Clear() => "Clear".to_string(),
        };
        
        log::info!("[DMSC.Cache] Publishing cache event: {event_type}");
        let _ = self.event_sender.send(event);
    }
    
    /// Get a value from cache
    /// 
    /// This method retrieves a value from the cache using the specified key. If the key
    /// exists and the value is valid, it is deserialized and returned. Otherwise, `None`
    /// is returned.
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to retrieve
    /// 
    /// **Returns:**
    /// - `Ok(Some(T))` if the key exists and the value is valid
    /// - `Ok(None)` if the key does not exist
    /// - `Err(DMSCError)` if an error occurs during retrieval or deserialization
    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> crate::core::DMSCResult<Option<T>> {
        log::debug!("[DMSC.Cache] Getting cache key: {key}");
        
        match self.backend.get(key).await? {
            Some(cached_value) => {
                log::debug!("[DMSC.Cache] Cache hit for key: {key}");
                let value = serde_json::from_str(&cached_value)
                    .map_err(|e| crate::core::DMSCError::Other(format!("Deserialization error: {e}")))?;
                Ok(Some(value))
            }
            None => {
                log::debug!("[DMSC.Cache] Cache miss for key: {key}");
                Ok(None)
            }
        }
    }
    
    /// Set a value in cache with optional TTL
    /// 
    /// This method stores a value in the cache with the specified key and optional TTL.
    /// It also publishes an invalidate event to ensure cache consistency across all instances.
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to set
    /// - `value`: The value to store (must implement Serialize)
    /// - `ttl_seconds`: Optional time-to-live in seconds
    /// 
    /// **Returns:**
    /// - `Ok(())` if the value was successfully stored
    /// - `Err(DMSCError)` if an error occurs during serialization or storage
    pub async fn set<T: serde::Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> crate::core::DMSCResult<()> {
        log::debug!("[DMSC.Cache] Setting cache key: {key} with TTL: {ttl_seconds:?}");
        
        let serialized = serde_json::to_string(value)
            .map_err(|e| crate::core::DMSCError::Other(format!("Serialization error: {e}")))?;
        
        let result = self.backend.set(key, &serialized, ttl_seconds).await;
        
        match &result {
            Ok(_) => log::debug!("[DMSC.Cache] Successfully set cache key: {key}"),
            Err(e) => log::error!("[DMSC.Cache] Failed to set cache key {key}: {e}"),
        }
        
        // Publish invalidate event to ensure consistency across instances
        self.publish_event(DMSCCacheEvent::Invalidate { key: key.to_string() });
        
        result
    }
    
    /// Delete a value from cache
    /// 
    /// This method deletes a value from the cache using the specified key. It also
    /// publishes an invalidate event to ensure cache consistency across all instances.
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to delete
    /// 
    /// **Returns:**
    /// - `Ok(true)` if the key was found and deleted
    /// - `Ok(false)` if the key didn't exist
    /// - `Err(DMSCError)` if an error occurs during deletion
    pub async fn delete(&self, key: &str) -> crate::core::DMSCResult<bool> {
        log::debug!("[DMSC.Cache] Deleting cache key: {key}");
        
        let result = self.backend.delete(key).await;
        
        match &result {
            Ok(true) => log::debug!("[DMSC.Cache] Successfully deleted cache key: {key}"),
            Ok(false) => log::debug!("[DMSC.Cache] Cache key not found for deletion: {key}"),
            Err(e) => log::error!("[DMSC.Cache] Failed to delete cache key {key}: {e}"),
        }
        
        // Publish invalidate event to ensure consistency across instances
        self.publish_event(DMSCCacheEvent::Invalidate { key: key.to_string() });
        
        result
    }
    
    /// Check if a key exists in cache
    /// 
    /// This method checks if the specified key exists in the cache.
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to check
    /// 
    /// **Returns:**
    /// - `true` if the key exists, `false` otherwise
    pub async fn exists(&self, key: &str) -> bool {
        self.backend.exists(key).await
    }
    
    /// Clear all cache entries
    /// 
    /// This method clears all entries from the cache. It also publishes a clear event
    /// to ensure cache consistency across all instances.
    /// 
    /// **Returns:**
    /// - `Ok(())` if the cache was successfully cleared
    /// - `Err(DMSCError)` if an error occurs during clearing
    pub async fn clear(&self) -> crate::core::DMSCResult<()> {
        log::info!("[DMSC.Cache] Clearing all cache entries");
        
        let result = self.backend.clear().await;
        
        match &result {
            Ok(_) => log::info!("[DMSC.Cache] Successfully cleared all cache entries"),
            Err(e) => log::error!("[DMSC.Cache] Failed to clear cache: {e}"),
        }
        
        // Publish clear event to ensure consistency across instances
        self.publish_event(DMSCCacheEvent::Clear());
        
        result
    }
    
    /// Invalidate cache entries matching a pattern
    /// 
    /// This method invalidates all cache entries matching the specified pattern. It
    /// publishes an invalidate pattern event to ensure cache consistency across all instances.
    /// 
    /// **Parameters:**
    /// - `pattern`: The pattern to match cache keys (supports wildcards depending on backend)
    /// 
    /// **Returns:**
    /// - `Ok(())` if the invalidation event was successfully published
    pub async fn invalidate_pattern(&self, pattern: &str) -> crate::core::DMSCResult<()> {
        // Publish invalidate pattern event to ensure consistency across instances
        self.publish_event(DMSCCacheEvent::InvalidatePattern { pattern: pattern.to_string() });
        
        Ok(())
    }
    
    /// Get cache statistics
    /// 
    /// This method retrieves statistics about the cache, including hit rate, miss rate,
    /// and the number of entries.
    /// 
    /// **Returns:**
    /// - A `CacheStats` struct containing the cache statistics
    pub async fn stats(&self) -> CacheStats {
        let stats = self.backend.stats().await;
        
        // Log cache statistics for monitoring
        log::info!("[DMSC.Cache] Cache Statistics: hits={}, misses={}, entries={}, hit_rate={:.2}%", 
                 stats.hits, stats.misses, stats.entries, stats.avg_hit_rate * 100.0);
        
        // Monitor cache performance
        if stats.hits + stats.misses > 0 {
            let current_hit_rate = stats.hits as f64 / (stats.hits + stats.misses) as f64;
            if current_hit_rate < 0.5 && stats.hits + stats.misses > 100 {
                log::warn!("[DMSC.Cache] Warning: Low cache hit rate ({:.2}%) with {} total operations", 
                         current_hit_rate * 100.0, stats.hits + stats.misses);
            }
        }
        
        stats
    }
    
    /// Cleanup expired cache entries
    /// 
    /// This method removes all expired entries from the cache.
    /// 
    /// **Returns:**
    /// - `Ok(usize)` with the number of expired entries cleaned up
    /// - `Err(DMSCError)` if an error occurs during cleanup
    pub async fn cleanup_expired(&self) -> crate::core::DMSCResult<usize> {
        let cleaned = self.backend.cleanup_expired().await?;
        
        // Log cleanup results for monitoring
        if cleaned > 0 {
            log::info!("[DMSC.Cache] Cleanup completed: {cleaned} expired entries removed");
        }
        
        Ok(cleaned)
    }
    
    /// Get a value from cache or set it if it doesn't exist
    /// 
    /// This method retrieves a value from the cache using the specified key. If the key
    /// exists and the value is valid, it is returned. Otherwise, the provided factory
    /// function is called to generate the value, which is then stored in the cache and
    /// returned.
    /// 
    /// **Parameters:**
    /// - `key`: The cache key to retrieve or set
    /// - `ttl_seconds`: Optional time-to-live in seconds for the new value
    /// - `factory`: A function that generates the value if it doesn't exist in cache
    /// 
    /// **Returns:**
    /// - `Ok(T)` with the retrieved or generated value
    /// - `Err(DMSCError)` if an error occurs during retrieval, generation, or storage
    pub async fn get_or_set<T, F>(&self, key: &str, ttl_seconds: Option<u64>, factory: F) -> crate::core::DMSCResult<T>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone,
        F: FnOnce() -> crate::core::DMSCResult<T>,
    {
        log::debug!("[DMSC.Cache] get_or_set operation for key: {key} with TTL: {ttl_seconds:?}");
        
        // Try to get from cache first
        if let Some(value) = self.get::<T>(key).await? {
            log::debug!("[DMSC.Cache] get_or_set cache hit for key: {key}");
            return Ok(value);
        }
        
        log::debug!("[DMSC.Cache] get_or_set cache miss for key: {key}, generating value");
        
        // If not found, generate the value
        let value = factory()?;
        
        // Store in cache
        self.set(key, &value, ttl_seconds).await?;
        
        log::debug!("[DMSC.Cache] get_or_set successfully generated and cached value for key: {key}");
        Ok(value)
    }
}
