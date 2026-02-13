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

//! # Cache Configuration Module
//!
//! This module defines the configuration structures and types used by the cache system.
//! It includes main cache configuration, backend type selection, and per-entry cache policies.
//!
//! ## Key Components
//!
//! - **[`DMSCCacheConfig`](DMSCCacheConfig)**: Main cache configuration structure
//! - **[`DMSCCacheBackendType`](DMSCCacheBackendType)**: Enum for selecting cache backend type
//! - **[`DMSCCachePolicy`](DMSCCachePolicy)**: Per-entry cache policy with TTL and size limits
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
//! use dmsc::cache::{DMSCCacheConfig, DMSCCacheBackendType, DMSCCachePolicy};
//! use std::time::Duration;
//!
//! // Create default cache configuration
//! let default_config = DMSCCacheConfig::default();
//!
//! // Create custom cache configuration
//! let custom_config = DMSCCacheConfig {
//!     enabled: true,
//!     default_ttl_secs: 7200, // 2 hours
//!     max_memory_mb: 1024,
//!     cleanup_interval_secs: 600, // 10 minutes
//!     backend_type: DMSCCacheBackendType::Hybrid,
//!     redis_url: "redis://localhost:6379/1".to_string(),
//!     redis_pool_size: 20,
//! };
//!
//! // Parse backend type from string
//! let backend_type: DMSCCacheBackendType = "redis".parse().unwrap();
//!
//! // Create a custom cache policy
//! let cache_policy = DMSCCachePolicy {
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCacheConfig {
    /// Whether caching is enabled
    #[pyo3(get, set)]
    pub enabled: bool,
    /// Default time-to-live in seconds
    #[pyo3(get, set)]
    pub default_ttl_secs: u64,
    /// Maximum memory usage in megabytes
    #[pyo3(get, set)]
    pub max_memory_mb: u64,
    /// Interval for cleaning up expired entries in seconds
    #[pyo3(get, set)]
    pub cleanup_interval_secs: u64,
    /// Type of cache backend to use
    #[pyo3(get, set)]
    pub backend_type: DMSCCacheBackendType,
    /// Redis connection URL (if using Redis or Hybrid backend)
    #[pyo3(get, set)]
    pub redis_url: String,
    /// Redis connection pool size
    #[pyo3(get, set)]
    pub redis_pool_size: usize,
}

impl Default for DMSCCacheConfig {
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
    /// A new `DMSCCacheConfig` instance with default values
    fn default() -> Self {
        Self {
            enabled: true,
            default_ttl_secs: 3600, // 1 hour
            max_memory_mb: 512,
            cleanup_interval_secs: 300, // 5 minutes
            backend_type: DMSCCacheBackendType::Memory,
            redis_url: "redis://127.0.0.1:6379".to_string(),
            redis_pool_size: 10,
        }
    }
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCCacheConfig {
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
/// Defines the different cache backend types supported by DMSC.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCCacheBackendType {
    /// In-memory cache (fast, non-persistent)
    Memory,
    /// Redis cache (persistent, distributed)
    Redis,
    /// Hybrid cache (Memory + Redis for performance and persistence)
    Hybrid,
}

impl DMSCCacheBackendType {
    /// Converts a string to a `DMSCCacheBackendType`.
    ///
    /// This method provides a custom string conversion for cache backend types.
    ///
    /// # Parameters
    ///
    /// - `s`: String to convert to `DMSCCacheBackendType`
    ///
    /// # Returns
    ///
    /// A `DMSCCacheBackendType` based on the input string
    ///
    /// # Mapping
    ///
    /// - "redis" -> `DMSCCacheBackendType::Redis`
    /// - "hybrid" -> `DMSCCacheBackendType::Hybrid`
    /// - Any other value -> `DMSCCacheBackendType::Memory`
    pub fn from_str_custom(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "redis" => DMSCCacheBackendType::Redis,
            "hybrid" => DMSCCacheBackendType::Hybrid,
            _ => DMSCCacheBackendType::Memory,
        }
    }
}

/// Implements standard FromStr trait for DMSCCacheBackendType
impl std::str::FromStr for DMSCCacheBackendType {
    type Err = ();

    /// Parses a string to a `DMSCCacheBackendType`.
    ///
    /// This implementation of the standard `FromStr` trait allows for easy
    /// parsing of strings to `DMSCCacheBackendType` using the `parse()` method.
    ///
    /// # Parameters
    ///
    /// - `s`: String to parse
    ///
    /// # Returns
    ///
    /// `Ok(DMSCCacheBackendType)` if parsing succeeds, otherwise `Ok(DMSCCacheBackendType::Memory)`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_str_custom(s))
    }
}

/// Cache policy for individual cache entries.
///
/// This struct defines the caching policy for individual cache entries,
/// including TTL, refresh behavior, and size limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCCachePolicy {
    /// Time-to-live for cache entries
    #[pyo3(get, set)]
    pub ttl: Option<Duration>,
    /// Whether to refresh TTL on access
    #[pyo3(get, set)]
    pub refresh_on_access: bool,
    /// Maximum size for cached data in bytes
    #[pyo3(get, set)]
    pub max_size: Option<usize>,
}

impl Default for DMSCCachePolicy {
    /// Creates a default cache policy with sensible values.
    ///
    /// Default values:
    /// - ttl: Some(Duration::from_secs(3600)) (1 hour)
    /// - refresh_on_access: false
    /// - max_size: None (no size limit)
    ///
    /// # Returns
    ///
    /// A new `DMSCCachePolicy` instance with default values
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
impl DMSCCachePolicy {
    #[new]
    fn new() -> Self {
        Self::default()
    }

    #[staticmethod]
    fn default_policy() -> Self {
        Self::default()
    }
}
