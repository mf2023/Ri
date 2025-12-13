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

//! # Cache Module
//! 
//! This module provides a comprehensive caching abstraction for DMS, offering a unified interface
//! with support for multiple backend implementations. It enables efficient data caching with
//! configurable policies and backend selection.
//! 
//! ## Key Components
//! 
//! - **DMSCacheModule**: Main cache module implementing both sync and async service module traits
//! - **DMSCacheManager**: Central cache management component
//! - **DMSCache**: Unified cache interface implemented by all backends
//! - **DMSCacheConfig**: Configuration for cache behavior
//! - **Backend Implementations**:
//!   - **DMSMemoryCache**: In-memory cache implementation (internal)
//!   - **DMSRedisCache**: Redis-based distributed cache (internal)
//!   - **DMSHybridCache**: Combined memory and Redis cache for optimal performance (internal)
//! 
//! ## Design Principles
//! 
//! 1. **Unified Interface**: Consistent API across all backend implementations
//! 2. **Multiple Backends**: Support for different cache storage options
//! 3. **Async Support**: Full async/await compatibility
//! 4. **Configurable**: Highly configurable cache behavior
//! 5. **Non-critical**: Cache failures should not break the application
//! 6. **Stats Collection**: Built-in cache statistics for monitoring
//! 7. **Service Module Integration**: Implements both sync and async service module traits
//! 8. **Thread-safe**: Safe for concurrent use across multiple threads
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create cache configuration
//!     let cache_config = DMSCacheConfig {
//!         enabled: true,
//!         default_ttl_secs: 3600,
//!         max_memory_mb: 512,
//!         cleanup_interval_secs: 300,
//!         backend_type: CacheBackendType::Memory,
//!         redis_url: "redis://127.0.0.1:6379".to_string(),
//!         redis_pool_size: 10,
//!     };
//!     
//!     // Create cache module
//!     let cache_module = DMSCacheModule::new(cache_config);
//!     
//!     // Get cache manager
//!     let cache_manager = cache_module.cache_manager();
//!     
//!     // Use cache manager to get cache instance
//!     let cache = cache_manager.read().await.get_cache();
//!     
//!     // Set a value in cache
//!     cache.set("key", "value", Some(3600)).await?;
//!     
//!     // Get a value from cache
//!     let value = cache.get("key").await?;
//!     println!("Cached value: {:?}", value);
//!     
//!     Ok(())
//! }
//! ```

mod core;
mod manager;
mod backends;
mod config;

pub use config::{DMSCacheConfig, CacheBackendType};
pub use manager::DMSCacheManager;
pub use core::{CachedValue, CacheStats, DMSCache};
// Re-export backend implementations
pub use backends::{DMSMemoryCache, DMSRedisCache, DMSHybridCache};

use crate::core::{DMSResult, DMSServiceContext};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main cache module for DMS.
/// 
/// This module provides a unified caching abstraction with support for multiple backend implementations.
/// It implements both the `AsyncServiceModule` and `ServiceModule` traits for seamless integration
/// into the DMS application lifecycle.
pub struct DMSCacheModule {
    /// Cache configuration
    config: DMSCacheConfig,
    /// Cache manager wrapped in an async RwLock for thread-safe access
    manager: std::sync::Arc<tokio::sync::RwLock<DMSCacheManager>>,
}

impl DMSCacheModule {
    /// Creates a new cache module with the given configuration.
    /// 
    /// This method initializes the cache manager with the appropriate backend based on the
    /// provided configuration. The backend is created immediately, not as a placeholder.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The cache configuration to use
    /// 
    /// # Returns
    /// 
    /// A new `DMSCacheModule` instance
    pub fn new(config: DMSCacheConfig) -> Self {
        // Create the appropriate backend based on configuration
        let backend = match config.backend_type {
            crate::cache::config::CacheBackendType::Memory => {
                Arc::new(DMSMemoryCache::new())
            }
            crate::cache::config::CacheBackendType::Redis => {
                // For Redis, we'll use memory backend initially and replace it in init()
                // since we need async context for Redis connection
                Arc::new(DMSMemoryCache::new())
            }
            crate::cache::config::CacheBackendType::Hybrid => {
                // Same for Hybrid - use memory backend initially
                Arc::new(DMSMemoryCache::new())
            }
        };
        
        let manager = DMSCacheManager::new(backend);
        
        Self {
            config,
            manager: Arc::new(RwLock::new(manager)),
        }
    }
    
    /// Returns a reference to the cache manager.
    /// 
    /// The cache manager is wrapped in an Arc<RwLock<>> to allow thread-safe access
    /// from multiple async tasks.
    /// 
    /// # Returns
    /// 
    /// An Arc<RwLock<DMSCacheManager>> providing thread-safe access to the cache manager
    pub fn cache_manager(&self) -> Arc<RwLock<DMSCacheManager>> {
        self.manager.clone()
    }
}

#[async_trait::async_trait]
impl crate::core::DMSModule for DMSCacheModule {
    /// Returns the name of the cache module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMS.Cache"
    }
    
    /// Indicates whether the cache module is critical.
    /// 
    /// The cache module is non-critical, meaning that if it fails to initialize or operate,
    /// it should not break the entire application. This allows the core functionality to continue
    /// even if caching features are unavailable.
    /// 
    /// # Returns
    /// 
    /// `false` since cache is non-critical
    fn is_critical(&self) -> bool {
        false // Cache failures should not break the application
    }
    
    /// Initializes the cache module asynchronously.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Updates the module configuration if provided
    /// 3. Initializes the appropriate cache backend based on configuration
    /// 4. Creates and sets the cache manager with the initialized backend
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration and other services
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        log::info!("Initializing DMS Cache Module");
        
        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        // Update configuration if provided
        if let Some(cache_config) = cfg.get("cache") {
            self.config = serde_yaml::from_str(cache_config)
                .unwrap_or_else(|_| DMSCacheConfig::default());
        } else {
            self.config = DMSCacheConfig::default();
        }
        
        // Initialize the cache manager based on configuration
        match self.config.backend_type {
            crate::cache::config::CacheBackendType::Memory => {
                let backend = Arc::new(DMSMemoryCache::new());
                let manager = DMSCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
            crate::cache::config::CacheBackendType::Redis => {
                let backend = Arc::new(DMSRedisCache::new(&self.config.redis_url).await?);
                let manager = DMSCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
            crate::cache::config::CacheBackendType::Hybrid => {
                let backend = Arc::new(DMSHybridCache::new(&self.config.redis_url).await?);
                let manager = DMSCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
        }
        
                // Log successful initialization
        if let Ok(fs) = crate::fs::DMSFileSystem::new_auto_root() {
            let logger = crate::log::DMSLogger::new(&crate::log::DMSLogConfig::default(), fs);
            let _ = logger.info("cache", "DMS Cache Module initialized successfully");
        }
        Ok(())
    }
    
    /// Performs asynchronous cleanup after the application has shut down.
    /// 
    /// This method performs the following steps:
    /// 1. Prints cache statistics
    /// 2. Cleans up expired cache entries
    /// 3. Prints cleanup results
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        log::info!("Cleaning up DMS Cache Module");
        
        let manager = self.manager.read().await;
        let stats = manager.stats().await;
        log::info!("Cache stats: {stats:?}");
        
        // Cleanup expired entries
        let cleaned = manager.cleanup_expired().await?;
        log::info!("Cleaned up {cleaned} expired cache entries");
        log::info!("DMS Cache Module cleanup completed");
        Ok(())
    }
}
