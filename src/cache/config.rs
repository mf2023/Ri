//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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

//! # Cache Configuration Module
//!
//! This module defines the configuration structures and types used by the cache system.
//! It includes main cache configuration, backend type selection, and per-entry cache policies.
//!
//! ## Key Components
//!
//! - **[`RiCacheConfig`](RiCacheConfig)**: Main cache configuration structure
//! - **[`RiCacheBackendType`](RiCacheBackendType)**: Enum for selecting cache backend type
//! - **[`RiCachePolicy`](RiCachePolicy)**: Per-entry cache policy with TTL and size limits
//!
//! ## Design Principles
//!
//! 1. **Flexibility**: Supports multiple cache backends (Memory, Redis, Hybrid)
//! 2. **Configurability**: Extensive configuration options for each backend
//! 3. **Default Values**: Sensible defaults for all configuration parameters
//! 4. **Type Safety**: Strongly typed configuration with enum-based backend selection
//! 5. **Serialization**: Built-in JSON/YAML/TOML serialization support
//! 6. **Policy-Based**: Supports per-entry cache policies
//!
//! ## Usage Example
//!
//! ```rust,ignore
//! use ri::cache::{RiCacheConfig, RiCacheBackendType, RiCachePolicy};
//! use std::time::Duration;
//!
//! // Create default cache configuration
//! let default_config = RiCacheConfig::default();
//!
//! // Create custom cache configuration
//! let custom_config = RiCacheConfig {
//!     enabled: true,
//!     default_ttl_secs: 7200, // 2 hours
//!     max_memory_mb: 1024,
//!     cleanup_interval_secs: 600, // 10 minutes
//!     backend_type: RiCacheBackendType::Hybrid,
//!     redis_url: "redis://localhost:6379/1".to_string(),
//!     redis_pool_size: 20,
//! };
//!
//! // Parse backend type from string
//! let backend_type: RiCacheBackendType = "redis".parse().unwrap();
//!
//! // Create a custom cache policy
//! let cache_policy = RiCachePolicy {
//!     ttl: Some(Duration::from_secs(1800)), // 30 minutes
//!     refresh_on_access: true,
//!     max_size: Some(1024 * 1024), // 1MB
//! };
//! ```

#![allow(non_snake_case)]

use serde::{Serialize, Deserialize};
use std::time::Duration;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Main cache configuration structure.
///
/// This struct contains all configuration options for the cache system,
/// including backend selection, TTL settings, memory limits, and cleanup intervals.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
pub struct RiCacheConfig {
    /// Whether caching is enabled
    pub enabled: bool,
    /// Default time-to-live in seconds
    pub default_ttl_secs: u64,
    /// Maximum memory usage in megabytes
    pub max_memory_mb: u64,
    /// Interval for cleaning up expired entries in seconds
    pub cleanup_interval_secs: u64,
    /// Type of cache backend to use
    pub backend_type: RiCacheBackendType,
    /// Redis connection URL (if using Redis or Hybrid backend)
    pub redis_url: String,
    /// Redis connection pool size
    pub redis_pool_size: usize,
}

impl Default for RiCacheConfig {
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
    /// A new `RiCacheConfig` instance with default values
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl_secs: 3600, // 1 hour
            max_memory_mb: 512,
            cleanup_interval_secs: 300, // 5 minutes
            backend_type: RiCacheBackendType::Memory,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            redis_pool_size: 10,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiCacheConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn default_config() -> Self {
        Self::default()
    }
}

/// Cache backend type enumeration.
///
/// Defines the different cache backend types supported by Ri.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiCacheBackendType {
    /// In-memory cache (fast, non-persistent)
    Memory,
    /// Redis cache (persistent, distributed)
    Redis,
    /// Hybrid cache (Memory + Redis for performance and persistence)
    Hybrid,
}

impl RiCacheBackendType {
    /// Converts a string to a `RiCacheBackendType`.
    ///
    /// This method provides a custom string conversion for cache backend types.
    ///
    /// # Parameters
    ///
    /// - `s`: String to convert to `RiCacheBackendType`
    ///
    /// # Returns
    ///
    /// A `RiCacheBackendType` based on the input string
    ///
    /// # Mapping
    ///
    /// - "redis" -> `RiCacheBackendType::Redis`
    /// - "hybrid" -> `RiCacheBackendType::Hybrid`
    /// - Any other value -> `RiCacheBackendType::Memory`
    pub fn from_str_custom(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "redis" => RiCacheBackendType::Redis,
            "hybrid" => RiCacheBackendType::Hybrid,
            _ => RiCacheBackendType::Memory,
        }
    }
}

/// Implements standard FromStr trait for RiCacheBackendType
impl std::str::FromStr for RiCacheBackendType {
    type Err = ();

    /// Parses a string to a `RiCacheBackendType`.
    ///
    /// This implementation of the standard `FromStr` trait allows for easy
    /// parsing of strings to `RiCacheBackendType` using the `parse()` method.
    ///
    /// # Parameters
    ///
    /// - `s`: String to parse
    ///
    /// # Returns
    ///
    /// `Ok(RiCacheBackendType)` if parsing succeeds, otherwise `Ok(RiCacheBackendType::Memory)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_custom(s))
    }
}

/// Cache policy for individual cache entries.
///
/// This struct defines the caching policy for individual cache entries,
/// including TTL, refresh behavior, and size limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
pub struct RiCachePolicy {
    /// Time-to-live for cache entries
    pub ttl: Option<Duration>,
    /// Whether to refresh TTL on access
    pub refresh_on_access: bool,
    /// Maximum size for cached data in bytes
    pub max_size: Option<usize>,
}

impl Default for RiCachePolicy {
    /// Creates a default cache policy with sensible values.
    ///
    /// Default values:
    /// - ttl: Some(Duration::from_secs(3600)) (1 hour)
    /// - refresh_on_access: false
    /// - max_size: None (no size limit)
    ///
    /// # Returns
    ///
    /// A new `RiCachePolicy` instance with default values
    fn default() -> Self {
        Self {
            ttl: Some(Duration::from_secs(3600)),
            refresh_on_access: false,
            max_size: None,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiCachePolicy {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn default_policy() -> Self {
        Self::default()
    }
}
