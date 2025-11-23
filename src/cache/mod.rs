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

/// Cache module for DMS - provides unified caching abstraction with multiple backend support
pub struct DMSCacheModule {
    config: DMSCacheConfig,
    manager: std::sync::Arc<tokio::sync::RwLock<DMSCacheManager>>,
}

impl DMSCacheModule {
    pub fn _Fnew(config: DMSCacheConfig) -> Self {
        // Create a dummy manager that will be replaced during initialization
        let dummy_backend = Arc::new(DMSMemoryCache::_Fnew());
        let dummy_manager = DMSCacheManager::_Fnew(dummy_backend);
        
        Self {
            config,
            manager: Arc::new(RwLock::new(dummy_manager)),
        }
    }
    
    pub fn _Fcache_manager(&self) -> Arc<RwLock<DMSCacheManager>> {
        self.manager.clone()
    }
}

#[async_trait::async_trait]
impl crate::core::_CAsyncServiceModule for DMSCacheModule {
    fn _Fname(&self) -> &str {
        "DMS.Cache"
    }
    
    fn _Fis_critical(&self) -> bool {
        false // Cache failures should not break the application
    }
    
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
    
    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        println!("Cleaning up DMS Cache Module");
        
        let manager = self.manager.read().await;
        let stats = manager._Fstats().await;
        println!("Cache stats: {:?}", stats);
        
        // Cleanup expired entries
        let cleaned = manager._Fcleanup_expired().await?;
        println!("Cleaned up {} expired cache entries", cleaned);
        
        println!("DMS Cache Module cleanup completed");
        Ok(())
    }
}

impl crate::core::_CServiceModule for DMSCacheModule {
    fn _Fname(&self) -> &str {
        "DMS.Cache"
    }
    
    fn _Fis_critical(&self) -> bool {
        false
    }
    
    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        self.config = DMSCacheConfig {
            enabled: cfg._Fget_bool("cache.enabled").unwrap_or(true),
            default_ttl_secs: cfg._Fget_u64("cache.default_ttl_secs").unwrap_or(3600),
            max_memory_mb: cfg._Fget_u64("cache.max_memory_mb").unwrap_or(512),
            cleanup_interval_secs: cfg._Fget_u64("cache.cleanup_interval_secs").unwrap_or(300),
            backend_type: CacheBackendType::from_str(&cfg._Fget_str("cache.backend_type").unwrap_or_else(|| "memory")),
            redis_url: cfg._Fget_str("cache.redis_url").unwrap_or_else(|| "redis://127.0.0.1:6379").to_string(),
            redis_pool_size: cfg._Fget_u64("cache.redis_pool_size").unwrap_or(10) as usize,
        };
        
        // Cache manager is already initialized in the async _Finit method
        // No additional blocking initialization needed
        
        // Cache initialization is handled in the async _Finit method
        
        Ok(())
    }
    
    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Cleanup cache resources
        // No additional blocking cleanup needed
        
        // Cache cleanup is handled in the async _Fafter_shutdown method
        
        Ok(())
    }
}