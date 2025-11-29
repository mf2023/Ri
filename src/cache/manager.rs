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

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::{RwLock, broadcast};
use crate::cache::{DMSCache, CachedValue, CacheStats};

/// # DMS Cache Manager
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
/// 5. **Flexibility**: Supports any backend implementing the DMSCache trait
/// 6. **Scalability**: Designed to handle high-throughput cache operations
/// 
/// ## Usage Examples
/// ```rust
/// // Create a cache manager with a Redis backend
/// let redis_backend = Arc::new(DMSRedisBackend::new(config).await?);
/// let mut cache_manager = DMSCacheManager::new(redis_backend);
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
pub enum DMSCacheEvent {
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
    Clear,
}

/// Cache manager that coordinates different cache backends with consistency support
/// 
/// This struct provides a unified interface for cache operations while ensuring cache
/// consistency across multiple instances through event-driven architecture. It wraps
/// any backend implementing the DMSCache trait and adds consistency guarantees.
pub struct DMSCacheManager {
    /// The underlying cache backend implementation
    backend: Arc<dyn DMSCache + Send + Sync>,
    
    /// Broadcast sender for cache consistency events
    event_sender: broadcast::Sender<DMSCacheEvent>,
    
    /// Broadcast receiver for cache consistency events (used internally)
    event_receiver: Option<broadcast::Receiver<DMSCacheEvent>>,
    
    /// Map of subscribers to cache events (for internal use)
    _subscribers: Arc<RwLock<HashMap<String, broadcast::Receiver<DMSCacheEvent>>>>,
}

impl DMSCacheManager {
    /// Create a new cache manager with the specified backend
    /// 
    /// **Parameters:**
    /// - `backend`: The underlying cache backend implementation
    /// 
    /// **Returns:**
    /// - A new instance of `DMSCacheManager`
    pub fn _Fnew(backend: Arc<dyn DMSCache + Send + Sync>) -> Self {
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
    pub async fn _Fstart_consistency_listener(&mut self) -> tokio::task::JoinHandle<()> {
        let backend = self.backend.clone();
        let mut receiver = self.event_receiver.take().expect("Already started");
        
        tokio::spawn(async move {
            while let Ok(event) = receiver.recv().await {
                match event {
                    DMSCacheEvent::Invalidate { key } => {
                        if let Err(e) = backend._Fdelete(&key).await {
                            eprintln!("Failed to invalidate cache key {}: {}", key, e);
                        }
                    },
                    DMSCacheEvent::InvalidatePattern { pattern } => {
                        // Invalidate all keys matching the pattern
                        // Note: This requires backend support for pattern matching
                        eprintln!("Invalidate pattern not implemented: {}", pattern);
                    },
                    DMSCacheEvent::Clear => {
                        if let Err(e) = backend._Fclear().await {
                            eprintln!("Failed to clear cache: {}", e);
                        }
                    },
                }
            }
        })
    }
    
    /// Subscribe to cache consistency events
    /// 
    /// This method allows external components to subscribe to cache consistency events,
    /// enabling them to react to cache changes in real-time.
    /// 
    /// **Returns:**
    /// - A broadcast receiver for cache events
    pub fn _Fsubscribe(&self) -> broadcast::Receiver<DMSCacheEvent> {
        self.event_sender.subscribe()
    }
    
    /// Publish a cache consistency event
    /// 
    /// This method publishes a cache event to all subscribers, ensuring cache consistency
    /// across all instances.
    /// 
    /// **Parameters:**
    /// - `event`: The cache event to publish
    pub fn _Fpublish_event(&self, event: DMSCacheEvent) {
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
    /// - `Err(DMSError)` if an error occurs during retrieval or deserialization
    pub async fn _Fget<T: serde::de::DeserializeOwned>(&self, key: &str) -> crate::core::DMSResult<Option<T>> {
        match self.backend._Fget(key).await {
            Some(cached_value) => {
                match cached_value._Fdeserialize::<T>() {
                    Ok(value) => Ok(Some(value)),
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
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
    /// - `Err(DMSError)` if an error occurs during serialization or storage
    pub async fn _Fset<T: serde::Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) -> crate::core::DMSResult<()> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| crate::core::DMSError::Other(format!("Serialization error: {e}")))?;
        
        let cached_value = CachedValue::_Fnew(serde_json::Value::String(serialized), ttl_seconds.map(std::time::Duration::from_secs));
        let result = self.backend._Fset(key, cached_value).await;
        
        // Publish invalidate event to ensure consistency across instances
        self._Fpublish_event(DMSCacheEvent::Invalidate { key: key.to_string() });
        
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
    /// - `Ok(())` if the value was successfully deleted
    /// - `Err(DMSError)` if an error occurs during deletion
    pub async fn _Fdelete(&self, key: &str) -> crate::core::DMSResult<()> {
        let result = self.backend._Fdelete(key).await;
        
        // Publish invalidate event to ensure consistency across instances
        self._Fpublish_event(DMSCacheEvent::Invalidate { key: key.to_string() });
        
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
    pub async fn _Fexists(&self, key: &str) -> bool {
        self.backend._Fexists(key).await
    }
    
    /// Clear all cache entries
    /// 
    /// This method clears all entries from the cache. It also publishes a clear event
    /// to ensure cache consistency across all instances.
    /// 
    /// **Returns:**
    /// - `Ok(())` if the cache was successfully cleared
    /// - `Err(DMSError)` if an error occurs during clearing
    pub async fn _Fclear(&self) -> crate::core::DMSResult<()> {
        let result = self.backend._Fclear().await;
        
        // Publish clear event to ensure consistency across instances
        self._Fpublish_event(DMSCacheEvent::Clear);
        
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
    pub async fn _Finvalidate_pattern(&self, pattern: &str) -> crate::core::DMSResult<()> {
        // Publish invalidate pattern event to ensure consistency across instances
        self._Fpublish_event(DMSCacheEvent::InvalidatePattern { pattern: pattern.to_string() });
        
        Ok(())
    }
    
    /// Get cache statistics
    /// 
    /// This method retrieves statistics about the cache, including hit rate, miss rate,
    /// and the number of entries.
    /// 
    /// **Returns:**
    /// - A `CacheStats` struct containing the cache statistics
    pub async fn _Fstats(&self) -> CacheStats {
        self.backend._Fstats().await
    }
    
    /// Cleanup expired cache entries
    /// 
    /// This method removes all expired entries from the cache.
    /// 
    /// **Returns:**
    /// - `Ok(usize)` with the number of expired entries cleaned up
    /// - `Err(DMSError)` if an error occurs during cleanup
    pub async fn _Fcleanup_expired(&self) -> crate::core::DMSResult<usize> {
        self.backend._Fcleanup_expired().await
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
    /// - `Err(DMSError)` if an error occurs during retrieval, generation, or storage
    pub async fn _Fget_or_set<T, F>(&self, key: &str, ttl_seconds: Option<u64>, factory: F) -> crate::core::DMSResult<T>
    where
        T: serde::Serialize + serde::de::DeserializeOwned + Clone,
        F: FnOnce() -> crate::core::DMSResult<T>,
    {
        // Try to get from cache first
        if let Some(value) = self._Fget::<T>(key).await? {
            return Ok(value);
        }
        
        // If not found, generate the value
        let value = factory()?;
        
        // Store in cache
        self._Fset(key, &value, ttl_seconds).await?;
        
        Ok(value)
    }
}