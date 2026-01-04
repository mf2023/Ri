//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

//! # Cache Module
//! 
//! This module provides a comprehensive caching abstraction for DMSC, offering a unified interface
//! with support for multiple backend implementations. It enables efficient data caching with
//! configurable policies and backend selection.
//! 
//! ## Key Components
//! 
//! - **DMSCCacheModule**: Main cache module implementing both sync and async service module traits
//! - **DMSCCacheManager**: Central cache management component
//! - **DMSCCache**: Unified cache interface implemented by all backends
//! - **DMSCCacheConfig**: Configuration for cache behavior
//! - **Backend Implementations**:
//!   - **DMSCMemoryCache**: In-memory cache implementation (internal)
//!   - **DMSCRedisCache**: Redis-based distributed cache (internal)
//!   - **DMSCHybridCache**: Combined memory and Redis cache for optimal performance (internal)
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
//! async fn example() -> DMSCResult<()> {
//!     // Create cache configuration
//!     let cache_config = DMSCCacheConfig {
//!         enabled: true,
//!         default_ttl_secs: 3600,
//!         max_memory_mb: 512,
//!         cleanup_interval_secs: 300,
//!         backend_type: DMSCCacheBackendType::Memory,
//!         redis_url: "redis://127.0.0.1:6379".to_string(),
//!         redis_pool_size: 10,
//!     };
//!     
//!     // Create cache module
//!     let cache_module = DMSCCacheModule::new(cache_config);
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

pub use config::{DMSCCacheConfig, DMSCCacheBackendType, DMSCCachePolicy};
pub use manager::{DMSCCacheManager, DMSCCacheEvent};
pub use core::{DMSCCachedValue, DMSCCacheStats, DMSCCache, DMSCCacheEvent as CoreCacheEvent};
// Re-export backend implementations
pub use backends::DMSCMemoryCache;
#[cfg(feature = "redis")]
pub use backends::{DMSCRedisCache, DMSCHybridCache};

use crate::core::{DMSCResult, DMSCServiceContext};
use std::sync::Arc;
use tokio::sync::RwLock;

#[cfg(feature = "pyo3")]
use pyo3::pymethods;

/// Main cache module for DMSC.
/// 
/// This module provides a unified caching abstraction with support for multiple backend implementations.
/// It implements both the `AsyncServiceModule` and `ServiceModule` traits for seamless integration
/// into the DMSC application lifecycle.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCacheModule {
    /// Cache configuration
    #[pyo3(get, set)]
    config: DMSCCacheConfig,
    /// Cache manager wrapped in an async RwLock for thread-safe access
    manager: std::sync::Arc<tokio::sync::RwLock<DMSCCacheManager>>,
}

impl DMSCCacheModule {
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
    /// A new `DMSCCacheModule` instance
    pub fn new(config: DMSCCacheConfig) -> Self {
        // Create the appropriate backend based on configuration
        let backend = match config.backend_type {
            crate::cache::config::DMSCCacheBackendType::Memory => {
                Arc::new(DMSCMemoryCache::new())
            }
            crate::cache::config::DMSCCacheBackendType::Redis => {
                // For Redis, we'll use memory backend initially and replace it in init()
                // since we need async context for Redis connection
                Arc::new(DMSCMemoryCache::new())
            }
            crate::cache::config::DMSCCacheBackendType::Hybrid => {
                // Same for Hybrid - use memory backend initially
                Arc::new(DMSCMemoryCache::new())
            }
        };
        
        let manager = DMSCCacheManager::new(backend);
        
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
    /// An Arc<RwLock<DMSCCacheManager>> providing thread-safe access to the cache manager
    pub fn cache_manager(&self) -> Arc<RwLock<DMSCCacheManager>> {
        self.manager.clone()
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCCacheModule {
    #[new]
    fn py_new(config: DMSCCacheConfig) -> Self {
        Self::new(config)
    }
    
    /// Get cache manager status for Python (Python wrapper)
    fn get_cache_manager_status(&self) -> String {
        format!("Cache manager initialized with backend: {:?}", self.config.backend_type)
    }
}

#[async_trait::async_trait]
impl crate::core::DMSCModule for DMSCCacheModule {
    /// Returns the name of the cache module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.Cache"
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        log::info!("Initializing DMSC Cache Module");
        
        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        // Update configuration if provided
        if let Some(cache_config) = cfg.get("cache") {
            self.config = serde_yaml::from_str(cache_config)
                .unwrap_or_else(|_| DMSCCacheConfig::default());
        } else {
            self.config = DMSCCacheConfig::default();
        }
        
        // Initialize the cache manager based on configuration
        match self.config.backend_type {
            crate::cache::config::DMSCCacheBackendType::Memory => {
                let backend = Arc::new(DMSCMemoryCache::new());
                let manager = DMSCCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
            #[cfg(feature = "redis")]
            crate::cache::config::DMSCCacheBackendType::Redis => {
                let backend = Arc::new(DMSCRedisCache::new(&self.config.redis_url).await?);
                let manager = DMSCCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
            #[cfg(feature = "redis")]
            crate::cache::config::DMSCCacheBackendType::Hybrid => {
                let backend = Arc::new(DMSCHybridCache::new(&self.config.redis_url).await?);
                let manager = DMSCCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
            #[cfg(not(feature = "redis"))]
            crate::cache::config::DMSCCacheBackendType::Redis | crate::cache::config::DMSCCacheBackendType::Hybrid => {
                // Fallback to memory cache if Redis is not enabled
                let backend = Arc::new(DMSCMemoryCache::new());
                let manager = DMSCCacheManager::new(backend);
                *self.manager.write().await = manager;
            }
        }
        
                // Log successful initialization
        if let Ok(fs) = crate::fs::DMSCFileSystem::new_auto_root() {
            let logger = crate::log::DMSCLogger::new(&crate::log::DMSCLogConfig::default(), fs);
            let _ = logger.info("cache", "DMSC Cache Module initialized successfully");
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
    /// A `DMSCResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        log::info!("Cleaning up DMSC Cache Module");
        
        let manager = self.manager.read().await;
        let stats = manager.stats().await;
        log::info!("Cache stats: {stats:?}");
        
        // Cleanup expired entries
        let cleaned = manager.cleanup_expired().await?;
        log::info!("Cleaned up {cleaned} expired cache entries");
        log::info!("DMSC Cache Module cleanup completed");
        Ok(())
    }
}
