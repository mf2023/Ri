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
//!   - **DMSMemoryCache**: In-memory cache implementation
//!   - **DMSRedisCache**: Redis-based distributed cache
//!   - **DMSHybridCache**: Combined memory and Redis cache for optimal performance
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
//!     let cache_module = DMSCacheModule::_Fnew(cache_config);
//!     
//!     // Get cache manager
//!     let cache_manager = cache_module._Fcache_manager();
//!     
//!     // Use cache manager to get cache instance
//!     let cache = cache_manager.read().await._Fcache();
//!     
//!     // Set a value in cache
//!     cache._Fset("key", "value", Some(3600)).await?;
//!     
//!     // Get a value from cache
//!     let value = cache._Fget("key").await?;
//!     println!("Cached value: {:?}", value);
//!     
//!     Ok(())
//! }
//! ```

pub mod cache;
pub mod manager;
pub mod backends;
pub mod config;

pub use cache::{DMSCache, CachedValue, CacheStats};
pub use manager::DMSCacheManager;
pub use config::{DMSCacheConfig, CacheBackendType, CachePolicy};

// Re-export backend implementations
pub use backends::{DMSMemoryCache, DMSRedisCache, DMSHybridCache};

use crate::core::{DMSResult, DMSServiceContext};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Main cache module for DMS.
/// 
/// This module provides a unified caching abstraction with support for multiple backend implementations.
/// It implements both the `_CAsyncServiceModule` and `_CServiceModule` traits for seamless integration
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
    /// This method creates a dummy cache manager that will be replaced during initialization
    /// with the actual backend implementation based on the provided configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The cache configuration to use
    /// 
    /// # Returns
    /// 
    /// A new `DMSCacheModule` instance
    pub fn _Fnew(config: DMSCacheConfig) -> Self {
        // Create a dummy manager that will be replaced during initialization
        let dummy_backend = Arc::new(DMSMemoryCache::_Fnew());
        let dummy_manager = DMSCacheManager::_Fnew(dummy_backend);
        
        Self {
            config,
            manager: Arc::new(RwLock::new(dummy_manager)),
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
    pub fn _Fcache_manager(&self) -> Arc<RwLock<DMSCacheManager>> {
        self.manager.clone()
    }
}

#[async_trait::async_trait]
impl crate::core::_CAsyncServiceModule for DMSCacheModule {
    /// Returns the name of the cache module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
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
    fn _Fis_critical(&self) -> bool {
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
    async fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Initializing DMS Cache Module");
        
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        // Update configuration if provided
        if let Some(cache_config) = cfg._Fget("cache") {
            self.config = serde_json::from_str(cache_config)
                .unwrap_or_else(|_| DMSCacheConfig::default());
        } else {
            self.config = DMSCacheConfig::default();
        }
        
        // Initialize the cache manager based on configuration
        match self.config.backend_type {
            CacheBackendType::Memory => {
                let backend = Arc::new(DMSMemoryCache::_Fnew());
                let manager = DMSCacheManager::_Fnew(backend);
                *self.manager.write().await = manager;
            }
            CacheBackendType::Redis => {
                let backend = Arc::new(DMSRedisCache::_Fnew(&self.config.redis_url).await?);
                let manager = DMSCacheManager::_Fnew(backend);
                *self.manager.write().await = manager;
            }
            CacheBackendType::Hybrid => {
                let backend = Arc::new(DMSHybridCache::_Fnew(&self.config.redis_url).await?);
                let manager = DMSCacheManager::_Fnew(backend);
                *self.manager.write().await = manager;
            }
        }
        
        //println!("DMS Cache Module initialized successfully");
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
    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Cleaning up DMS Cache Module");
        
        let manager = self.manager.read().await;
        let stats = manager._Fstats().await;
        println!("Cache stats: {stats:?}");
        
        // Cleanup expired entries
        let cleaned = manager._Fcleanup_expired().await?;
        println!("Cleaned up {cleaned} expired cache entries");
        
        println!("DMS Cache Module cleanup completed");
        Ok(())
    }
}

impl crate::core::_CServiceModule for DMSCacheModule {
    /// Returns the name of the cache module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn _Fname(&self) -> &str {
        "DMS.Cache"
    }
    
    /// Indicates whether the cache module is critical.
    /// 
    /// # Returns
    /// 
    /// `false` since cache is non-critical
    fn _Fis_critical(&self) -> bool {
        false
    }
    
    /// Initializes the cache module synchronously.
    /// 
    /// This method loads configuration from the service context but defers actual cache
    /// initialization to the async `_Finit` method, which handles backend-specific setup.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        self.config = DMSCacheConfig {
            enabled: cfg._Fget_bool("cache.enabled").unwrap_or(true),
            default_ttl_secs: cfg._Fget_u64("cache.default_ttl_secs").unwrap_or(3600),
            max_memory_mb: cfg._Fget_u64("cache.max_memory_mb").unwrap_or(512),
            cleanup_interval_secs: cfg._Fget_u64("cache.cleanup_interval_secs").unwrap_or(300),
            backend_type: cfg._Fget_str("cache.backend_type").unwrap_or("memory").parse().unwrap_or(CacheBackendType::Memory),
            redis_url: cfg._Fget_str("cache.redis_url").unwrap_or("redis://127.0.0.1:6379").to_string(),
            redis_pool_size: cfg._Fget_u64("cache.redis_pool_size").unwrap_or(10) as usize,
        };
        
        // Cache manager is already initialized in the async _Finit method
        // No additional blocking initialization needed
        
        // Cache initialization is handled in the async _Finit method
        
        Ok(())
    }
    
    /// Performs synchronous cleanup after the application has shut down.
    /// 
    /// This method is a no-op for the cache module, as all actual cleanup is handled
    /// in the async `_Fafter_shutdown` method.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success
    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Cleanup cache resources
        // No additional blocking cleanup needed
        
        // Cache cleanup is handled in the async _Fafter_shutdown method
        
        Ok(())
    }
}