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

//! # DMSC Cache Module Example
//!
//! This example demonstrates how to use the caching module in DMSC,
//! including in-memory cache operations and Redis backend integration.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example cache --features cache
//! ```
//!
//! ## Features Demonstrated
//!
//! - In-memory cache operations (get, set, delete)
//! - TTL (Time-To-Live) based expiration
//! - Cache statistics and monitoring
//! - Multiple backend support

use dmsc::{DMSCAppBuilder, DMSCCacheModule, DMSCCacheManager};
use std::time::Duration;

/// Async main function for the cache module example.
///
/// This function demonstrates the complete caching workflow including:
/// - Application builder pattern for cache module configuration
/// - Basic CRUD operations (Create, Read, Delete) on cache
/// - TTL-based expiration for temporary data
/// - Cache statistics monitoring (hits, misses, size)
/// - Cache clearing operations
///
/// The example shows how DMSC handles caching with features like
/// automatic expiration, hit/miss tracking, and memory management
/// in a Rust async environment.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== DMSC Cache Module Example ===\n");

    // Application Builder: Create application with cache module
    // DMSCAppBuilder provides a fluent API for configuring modules
    // The builder pattern allows for chained configuration calls
    let app = DMSCAppBuilder::new()
        .with_cache_module(|cache| {
            // Configure cache module with closure-based settings
            cache.with_config(|config| {
                // Set maximum cache size (10000 entries)
                // Prevents unbounded memory growth in production
                config
                    .with_max_size(10000)
                    .with_default_ttl(Duration::from_secs(300))
            })
            // Select memory backend (alternatives: Redis, Memcached)
            // Memory backend is suitable for single-instance applications
            .with_memory_backend()
        })
        .build()?;

    // Get cache module from built application
    let cache_module = app.get_module::<DMSCCacheModule>();
    
    // Get cache manager for cache operations
    // The manager provides thread-safe operations for cache manipulation
    let cache_manager = cache_module.get_manager();

    // Section 1: Basic Cache Operations
    // Demonstrates fundamental cache operations: SET and GET
    println!("1. Basic Cache Operations");
    println!("   ------------------------");

    // SET Operation: Store a value in the cache
    // Parameters:
    // - key: &str unique identifier for the cached value
    // - value: &[u8] bytes object to store (supports any serializable data)
    // - ttl: Option<Duration> for time-to-live (None = no expiration)
    // Returns: Result<()> indicating success or failure
    println!("   Setting 'user:1001' = 'John Doe'");
    cache_manager.set("user:1001", b"John Doe", None).await?;
    println!("   ✓ Value set successfully\n");

    // GET Operation: Retrieve a value from the cache
    // Returns: Result<Option<Vec<u8>>> Some(value) if found, None if not
    // This triggers cache hit/miss statistics tracking internally
    println!("   Getting 'user:1001'...");
    if let Some(value) = cache_manager.get("user:1001").await? {
        // Value found - cache hit scenario
        // Convert bytes back to string for display
        println!("   ✓ Found value: {}\n", String::from_utf8_lossy(&value));
    } else {
        // Value not found - cache miss scenario
        println!("   ✗ Value not found\n");
    }

    // Section 2: TTL-Based Expiration
    // Demonstrates time-to-live (TTL) functionality for automatic data expiration
    println!("2. TTL-Based Expiration");
    println!("   ---------------------");

    // SET with TTL: Store a value with automatic expiration
    // After TTL expires, the key is automatically removed from cache
    // TTL is useful for: session data, temporary calculations, rate limiting
    println!("   Setting 'temporary:key' with 5 second TTL");
    cache_manager.set(
        "temporary:key",
        b"This will expire",
        Some(Duration::from_secs(5))
    ).await?;
    println!("   ✓ Value set with TTL\n");

    // Verify value exists before expiration
    // This demonstrates cache hit before TTL expiry
    println!("   Checking 'temporary:key' before expiration...");
    if let Some(value) = cache_manager.get("temporary:key").await? {
        println!("   ✓ Value still exists: {}\n", String::from_utf8_lossy(&value));
    }

    // Wait for TTL expiration
    // Block for 6 seconds to ensure the 5-second TTL has passed
    // tokio::time::sleep is async and doesn't block the runtime
    println!("   Waiting 6 seconds for TTL expiration...");
    tokio::time::sleep(Duration::from_secs(6)).await;
    println!("   ✓ Wait complete\n");

    // Verify value is gone after expiration
    // This demonstrates automatic cleanup by the cache system
    println!("   Checking 'temporary:key' after expiration...");
    if cache_manager.get("temporary:key").await?.is_none() {
        println!("   ✓ Value expired and removed\n");
    }

    // Section 3: Cache Statistics
    // Demonstrates monitoring and metrics collection for cache operations
    println!("3. Cache Statistics");
    println!("   -----------------");

    // Get cache statistics from the manager
    // Statistics include:
    // - hits: Number of successful cache lookups
    // - misses: Number of failed cache lookups
    // - size: Current number of items in cache
    let stats = cache_manager.get_stats().await?;
    println!("   Current cache stats:");
    println!("   - Hits: {}", stats.hits());
    println!("   - Misses: {}", stats.misses());
    println!("   - Size: {}\n", stats.size());

    // Section 4: Delete Operations
    // Demonstrates manual cache entry removal
    println!("4. Delete Operations");
    println!("   ------------------");

    // DELETE Operation: Remove a specific key from cache
    // Unlike expiration, this is an explicit removal operation
    // Useful for: cache invalidation, removing stale data, cleanup
    println!("   Deleting 'user:1001'...");
    cache_manager.delete("user:1001").await?;
    println!("   ✓ Value deleted\n");

    // Verify deletion was successful
    println!("   Verifying deletion...");
    if cache_manager.get("user:1001").await?.is_none() {
        println!("   ✓ Value successfully deleted\n");
    }

    // Section 5: Clear All Cache
    // Demonstrates bulk cache cleanup operation
    println!("5. Clear All Cache");
    println!("   ----------------");

    // Add multiple values for demonstration
    println!("   Adding more values...");
    cache_manager.set("key1", b"value1", None).await?;
    cache_manager.set("key2", b"value2", None).await?;
    cache_manager.set("key3", b"value3", None).await?;
    println!("   ✓ Added 3 values\n");

    // CLEAR Operation: Remove all entries from cache
    // Use with caution - this removes all cached data
    // Commonly used during: application shutdown, testing, cache warming
    println!("   Clearing all cache...");
    cache_manager.clear().await?;
    println!("   ✓ Cache cleared\n");

    // Section 6: Final Statistics
    // Verify cache state after cleanup operations
    println!("6. Final Statistics");
    println!("   -----------------");

    // Get updated statistics after all operations
    // Expected: size should be 0 after clear operation
    let final_stats = cache_manager.get_stats().await?;
    println!("   Final cache stats:");
    println!("   - Hits: {}", final_stats.hits());
    println!("   - Misses: {}", final_stats.misses());
    println!("   - Size: {}\n", final_stats.size());

    println!("=== Cache Example Completed ===");
    Ok(())
}
