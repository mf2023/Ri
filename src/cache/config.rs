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

//! Cache configuration structures and types for DMS.
//! 
//! This module defines the configuration structures and types used by the cache system.
//! It includes:
//! - Main cache configuration with backend selection and settings
//! - Cache backend type enumeration
//! - Cache policy for individual cache entries
//! 
//! # Design Principles
//! - **Flexibility**: Supports multiple cache backends (Memory, Redis, Hybrid)
//! - **Configurability**: Extensive configuration options for each backend
//! - **Default Values**: Sensible defaults for all configuration parameters
//! - **Type Safety**: Strongly typed configuration with enum-based backend selection
//! - **Serialization Support**: Built-in JSON/YAML/TOML serialization
//! - **Standard Traits**: Implements standard traits like FromStr for easy parsing
//! - **Policy-Based**: Supports per-entry cache policies
//! 
//! # Usage Examples
//! ```rust
//! // Create default cache configuration
//! let default_config = DMSCacheConfig::default();
//! 
//! // Create custom cache configuration
//! let custom_config = DMSCacheConfig {
//!     enabled: true,
//!     default_ttl_secs: 7200, // 2 hours
//!     max_memory_mb: 1024,
//!     cleanup_interval_secs: 600, // 10 minutes
//!     backend_type: CacheBackendType::Hybrid,
//!     redis_url: "redis://localhost:6379/1".to_string(),
//!     redis_pool_size: 20,
//! };
//! 
//! // Parse backend type from string
//! let backend_type: CacheBackendType = "redis".parse()?;
//! 
//! // Create a custom cache policy
//! let cache_policy = CachePolicy {
//!     ttl: Some(Duration::from_secs(1800)), // 30 minutes
//!     refresh_on_access: true,
//!     max_size: Some(1024 * 1024), // 1MB
//! };
//! ```

#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use std::time::Duration;

/// Main cache configuration structure.
/// 
/// This struct contains all configuration options for the cache system,
/// including backend selection, TTL settings, memory limits, and cleanup intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCacheConfig {
    pub enabled: bool,                // Whether caching is enabled
    pub default_ttl_secs: u64,        // Default time-to-live in seconds
    pub max_memory_mb: u64,           // Maximum memory usage in megabytes
    pub cleanup_interval_secs: u64,    // Interval for cleaning up expired entries in seconds
    pub backend_type: CacheBackendType, // Type of cache backend to use
    pub redis_url: String,             // Redis connection URL (if using Redis or Hybrid backend)
    pub redis_pool_size: usize,        // Redis connection pool size
}

impl Default for DMSCacheConfig {
    /// Creates a default cache configuration with sensible values.
    /// 
    /// Default values:
    /// - enabled: true
    /// - default_ttl_secs: 3600 (1 hour)
    /// - max_memory_mb: 512
    /// - cleanup_interval_secs: 300 (5 minutes)
    /// - backend_type: Memory
    /// - redis_url: "redis://127.0.0.1:6379"
    /// - redis_pool_size: 10
    /// 
    /// # Returns
    /// A new `DMSCacheConfig` instance with default values
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl_secs: 3600, // 1 hour
            max_memory_mb: 512,
            cleanup_interval_secs: 300, // 5 minutes
            backend_type: CacheBackendType::Memory,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            redis_pool_size: 10,
        }
    }
}

/// Cache backend type enumeration.
/// 
/// Defines the different cache backend types supported by DMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CacheBackendType {
    Memory,  // In-memory cache (fast, non-persistent)
    Redis,   // Redis cache (persistent, distributed)
    Hybrid,  // Hybrid cache (Memory + Redis for performance and persistence)
}

impl CacheBackendType {
    /// Converts a string to a `CacheBackendType`.
    /// 
    /// This method provides a custom string conversion for cache backend types.
    /// 
    /// # Parameters
    /// - `s`: String to convert to `CacheBackendType`
    /// 
    /// # Returns
    /// A `CacheBackendType` based on the input string
    /// 
    /// # Mapping
    /// - "redis" -> `CacheBackendType::Redis`
    /// - "hybrid" -> `CacheBackendType::Hybrid`
    /// - Any other value -> `CacheBackendType::Memory`
    pub fn from_str_custom(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "redis" => CacheBackendType::Redis,
            "hybrid" => CacheBackendType::Hybrid,
            _ => CacheBackendType::Memory,
        }
    }
}

// Implement standard FromStr trait for CacheBackendType
impl std::str::FromStr for CacheBackendType {
    type Err = ();
    
    /// Parses a string to a `CacheBackendType`.
    /// 
    /// This implementation of the standard `FromStr` trait allows for easy
    /// parsing of strings to `CacheBackendType` using the `parse()` method.
    /// 
    /// # Parameters
    /// - `s`: String to parse
    /// 
    /// # Returns
    /// `Ok(CacheBackendType)` if parsing succeeds, otherwise `Ok(CacheBackendType::Memory)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_custom(s))
    }
}

/// Cache policy for individual cache entries.
/// 
/// This struct defines the caching policy for individual cache entries,
/// including TTL, refresh behavior, and size limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePolicy {
    pub ttl: Option<Duration>,        // Time-to-live for cache entries
    pub refresh_on_access: bool,      // Whether to refresh TTL on access
    pub max_size: Option<usize>,      // Maximum size for cached data in bytes
}

impl Default for CachePolicy {
    /// Creates a default cache policy with sensible values.
    /// 
    /// Default values:
    /// - ttl: Some(Duration::from_secs(3600)) (1 hour)
    /// - refresh_on_access: false
    /// - max_size: None (no size limit)
    /// 
    /// # Returns
    /// A new `CachePolicy` instance with default values
    fn default() -> Self {
        Self {
            ttl: Some(Duration::from_secs(3600)),
            refresh_on_access: false,
            max_size: None,
        }
    }
}