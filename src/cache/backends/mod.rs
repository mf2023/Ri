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

//! # Cache Backends Module
//! 
//! This module provides various cache backend implementations for the DMS cache system. 
//! It includes in-memory, Redis, and hybrid cache backends, allowing users to choose 
//! the appropriate cache implementation based on their requirements.
//! 
//! ## Available Backends
//! 
//! - **DMSMemoryCache**: In-memory cache implementation using DashMap for high performance
//! - **DMSRedisCache**: Redis-based cache implementation for distributed systems
//! - **DMSHybridCache**: Hybrid cache combining in-memory and Redis cache for optimal performance
//! 
//! ## Design Principles
//! 
//! 1. **Unified Interface**: All backends implement the DMSCache trait for consistent usage
//! 2. **Performance Focus**: Each backend is optimized for its specific use case
//! 3. **Thread Safety**: All backends are thread-safe for concurrent access
//! 4. **Expiration Support**: Built-in support for cache entry expiration
//! 5. **Statistics**: Comprehensive cache statistics for monitoring
//! 6. **Cleanup Mechanism**: Automatic cleanup of expired entries
//! 7. **Easy Integration**: Simple API for integrating with any application
//! 8. **Extensible**: Easy to add new cache backends
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create an in-memory cache
//!     let memory_cache = DMSMemoryCache::new();
//!     
//!     // Create a Redis cache
//!     let redis_cache = DMSRedisCache::new("redis://localhost:6379").await?;
//!     
//!     // Create a hybrid cache
//!     let hybrid_cache = DMSHybridCache::new("redis://localhost:6379").await?;
//!     
//!     Ok(())
//! }
//! ```

/// In-memory cache backend implementation
pub mod memory_backend;
/// Redis cache backend implementation
pub mod redis_backend;
/// Hybrid cache backend implementation (in-memory + Redis)
pub mod hybrid_backend;

/// In-memory cache implementation using DashMap for high performance
pub use memory_backend::DMSMemoryCache;
/// Redis-based cache implementation for distributed systems
pub use redis_backend::DMSRedisCache;
/// Hybrid cache combining in-memory and Redis cache for optimal performance
pub use hybrid_backend::DMSHybridCache;
