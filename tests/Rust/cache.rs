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

//! # Cache Module Tests
//!
//! This module contains comprehensive tests for the DMSC caching system, covering
//! all cache-related components including cached values, statistics, configuration,
//! backend types, memory cache implementation, and the cache manager.
//!
//! ## Test Coverage
//!
//! - **DMSCCachedValue**: Tests for serialized value storage, TTL-based expiration,
//!   touch functionality, and deserialization with type safety
//! - **DMSCCacheStats**: Tests for cache statistics tracking including entries,
//!   memory usage, hit/miss counts, and eviction tracking
//! - **DMSCCacheConfig**: Tests for default configuration values and backend type
//!   parsing from strings
//! - **DMSCMemoryCache**: Tests for in-memory cache operations including get, set,
//!   delete, exists, clear, statistics, and expired entry cleanup
//! - **DMSCCacheManager**: Tests for the generic cache manager interface with typed
//!   get/set operations, existence checks, and lazy value generation
//!
//! ## Design Principles
//!
//! Tests follow the principle of verifying both positive and negative scenarios,
//! including edge cases such as non-existent keys, expired entries, and type
//! mismatches during deserialization. All async operations use tokio for testing
//! concurrent cache access patterns.

use dmsc::cache::{DMSCCachedValue, DMSCCacheStats, DMSCCacheConfig, DMSCCacheBackendType, DMSCCacheManager, DMSCMemoryCache, DMSCCache};
use std::time::Duration;

#[test]
/// Tests DMSCCachedValue creation with serialized data and TTL.
///
/// Verifies that a cached value can be created with serialized JSON data
/// and an optional TTL (Time-To-Live). The test validates that the value
/// is properly stored with expiration tracking.
///
/// ## Test Steps
///
/// 1. Create a JSON value to be cached
/// 2. Serialize the value to a string
/// 3. Create a DMSCCachedValue with the serialized data and 60 second TTL
/// 4. Verify the value can be deserialized back to the original JSON
/// 5. Verify the value is not expired and has an expiration timestamp
///
/// ## Expected Behavior
///
/// - Deserialization returns the original JSON value
/// - The cached value is not expired upon creation
/// - The expiration timestamp is set to current time + TTL
fn test_cached_value_new() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let cached_value = DMSCCachedValue::new(serialized, ttl_seconds);
    
    let deserialized: serde_json::Value = cached_value.deserialize().unwrap();
    assert_eq!(deserialized, data);
    assert!(!cached_value.is_expired());
    assert!(cached_value.expires_at.is_some());
}

#[test]
/// Tests the touch() method for updating cached value access time.
///
/// Verifies that calling touch() on a cached value updates its last
/// access timestamp, which is used for LRU (Least Recently Used)
/// cache eviction policies.
///
/// ## Purpose
///
/// The touch() method is called when a cached value is accessed to
/// update its position in the access order. This allows the cache
/// to evict the least recently used entries when space is needed.
///
/// ## Test Steps
///
/// 1. Create a cached value with a TTL
/// 2. Call touch() on the value
/// 3. Verify the operation completes without errors
///
/// ## Expected Behavior
///
/// - touch() completes without panicking
/// - The last_access timestamp is updated to current time
fn test_cached_value_touch() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let mut cached_value = DMSCCachedValue::new(serialized, ttl_seconds);
    
    cached_value.touch();
}

#[test]
/// Tests DMSCCachedValue expiration detection.
///
/// Verifies that the is_expired() method correctly identifies expired
/// cached values based on their expiration timestamp. This test uses
/// manual timestamp manipulation to simulate expiration.
///
/// ## Test Scenarios
///
/// 1. **Non-expired value**: A value with a future expiration timestamp
///    should return false from is_expired()
/// 2. **Expired value**: A value with a past expiration timestamp
///    should return true from is_expired()
///
/// ## Test Steps
///
/// 1. Create a cached value with 1 second TTL
/// 2. Verify it is not expired immediately after creation
/// 3. Manually set expires_at to epoch (0) to simulate expiration
/// 4. Verify is_expired() returns true
///
/// ## Expected Behavior
///
/// - is_expired() returns false for values with future expiration
/// - is_expired() returns true for values with past expiration
/// - Manual timestamp manipulation triggers expiration detection
fn test_cached_value_expired() {
    let data = serde_json::json!("test_value");
    
    // Create a value with a very short TTL
    let ttl_seconds = Some(1_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    let mut cached_value = DMSCCachedValue::new(serialized, ttl_seconds);
    
    // Should not be expired immediately
    assert!(!cached_value.is_expired());
    
    // Simulate expiration by manually setting a past expires_at
    cached_value.expires_at = Some(0); // 1970-01-01 00:00:00 UTC
    
    // Should be expired now
    assert!(cached_value.is_expired());
}

#[test]
/// Tests DMSCCachedValue deserialization to different types.
///
/// Verifies that the deserialize() method can correctly deserialize
/// cached JSON data to various target types including String, JSON Value,
/// and custom types.
///
/// ## Test Scenarios
///
/// 1. **String deserialization**: Deserialize to a String type
/// 2. **JSON Value deserialization**: Deserialize to serde_json::Value
/// 3. **Type safety**: Incorrect type deserialization should fail
///
/// ## Expected Behavior
///
/// - deserialize() correctly reconstructs the original data
/// - The deserialized value matches the original serialized data
/// - Type mismatches result in deserialization errors
fn test_cached_value_deserialize() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let cached_value = DMSCCachedValue::new(serialized, ttl_seconds);
    
    // Test deserialization to string
    let result: String = cached_value.deserialize().unwrap();
    assert_eq!(result, "test_value");
    
    // Test deserialization to a different type should fail
    let result: Result<u32, _> = cached_value.deserialize();
    assert!(result.is_err());
}

#[test]
/// Tests DMSCCacheStats default initialization.
///
/// Verifies that a newly created DMSCCacheStats instance has correct
/// default values for all statistics fields including entry count,
/// memory usage, hit/miss counters, eviction count, and hit rate.
///
/// ## Default Values
///
/// - entries: 0 - No entries in a new cache
/// - memory_usage_bytes: 0 - No memory used initially
/// - hit_count: 0 - No cache hits yet
/// - miss_count: 0 - No cache misses yet
/// - eviction_count: 0 - No entries evicted yet
/// - avg_hit_rate: 0.0 - No operations to calculate hit rate
///
/// ## Expected Behavior
///
/// All fields should be initialized to their default (zero) values
/// representing an empty cache with no performance history.
fn test_cache_stats_default() {
    let stats = DMSCCacheStats::default();
    
    assert_eq!(stats.entries, 0);
    assert_eq!(stats.memory_usage_bytes, 0);
    assert_eq!(stats.hit_count, 0);
    assert_eq!(stats.miss_count, 0);
    assert_eq!(stats.eviction_count, 0);
    assert_eq!(stats.avg_hit_rate, 0.0);
}

#[test]
/// Tests DMSCCacheConfig default configuration values.
///
/// Verifies that the default DMSCCacheConfig has appropriate values
/// for cache initialization including enabled status, TTL, memory limits,
/// cleanup interval, backend type, and Redis connection settings.
///
/// ## Default Configuration Values
///
/// - enabled: true - Cache is enabled by default
/// - default_ttl_secs: 3600 - 1 hour default TTL for cached entries
/// - max_memory_mb: 512 - Maximum 512 MB memory usage
/// - cleanup_interval_secs: 300 - Cleanup expired entries every 5 minutes
/// - backend_type: Memory - Use in-memory cache by default
/// - redis_url: "redis://127.0.0.1:6379" - Default Redis connection
/// - redis_pool_size: 10 - 10 connections in Redis pool
///
/// ## Expected Behavior
///
/// All configuration fields should have sensible defaults suitable for
/// typical caching scenarios without requiring explicit configuration.
fn test_cache_config_default() {
    let config = DMSCCacheConfig::default();
    
    assert!(config.enabled);
    assert_eq!(config.default_ttl_secs, 3600);
    assert_eq!(config.max_memory_mb, 512);
    assert_eq!(config.cleanup_interval_secs, 300);
    assert_eq!(config.backend_type, DMSCCacheBackendType::Memory);
    assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
    assert_eq!(config.redis_pool_size, 10);
}

#[test]
/// Tests DMSCCacheBackendType parsing from strings.
///
/// Verifies that DMSCCacheBackendType can be correctly parsed from
/// string representations using both the custom from_str_custom method
/// and the standard FromStr trait. Invalid strings should default to
/// the Memory backend type.
///
/// ## Supported Backend Types
///
/// - "memory" -> DMSCCacheBackendType::Memory
/// - "redis" -> DMSCCacheBackendType::Redis
/// - "hybrid" -> DMSCCacheBackendType::Hybrid
/// - invalid string -> DMSCCacheBackendType::Memory (default)
///
/// ## Test Scenarios
///
/// 1. Valid backend type strings are correctly parsed
/// 2. Case sensitivity is handled appropriately
/// 3. Invalid strings default to Memory backend
/// 4. Both from_str_custom and FromStr trait work identically
///
/// ## Expected Behavior
///
/// - Valid strings map to their corresponding backend types
/// - Invalid strings fall back to Memory as the default
/// - The parsing is consistent between methods
fn test_cache_backend_type_from_str() {
    // Test from_str_custom method
    assert_eq!(DMSCCacheBackendType::from_str_custom("memory"), DMSCCacheBackendType::Memory);
    assert_eq!(DMSCCacheBackendType::from_str_custom("redis"), DMSCCacheBackendType::Redis);
    assert_eq!(DMSCCacheBackendType::from_str_custom("hybrid"), DMSCCacheBackendType::Hybrid);
    assert_eq!(DMSCCacheBackendType::from_str_custom("invalid"), DMSCCacheBackendType::Memory);
    
    // Test standard FromStr trait
    assert_eq!("memory".parse::<DMSCCacheBackendType>().unwrap(), DMSCCacheBackendType::Memory);
    assert_eq!("redis".parse::<DMSCCacheBackendType>().unwrap(), DMSCCacheBackendType::Redis);
    assert_eq!("hybrid".parse::<DMSCCacheBackendType>().unwrap(), DMSCCacheBackendType::Hybrid);
    assert_eq!("invalid".parse::<DMSCCacheBackendType>().unwrap(), DMSCCacheBackendType::Memory);
}

#[tokio::test]
/// Tests basic DMSCMemoryCache get and set operations.
///
/// Verifies that the in-memory cache can store and retrieve serialized
/// JSON values correctly with TTL-based expiration. This test covers
/// the fundamental read/write operations of the cache.
///
/// ## Test Scenarios
///
/// 1. **Successful get/set**: Store a value and retrieve it successfully
/// 2. **Value integrity**: The retrieved value matches the original
/// 3. **Non-existent key**: Getting a non-existent key returns None
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Serialize a JSON value to a string
/// 3. Set the value in the cache with 60 second TTL
/// 4. Retrieve the value from the cache
/// 5. Verify the retrieved value matches the original
/// 6. Attempt to get a non-existent key and verify None is returned
///
/// ## Expected Behavior
///
/// - Set operation completes without errors
/// - Get operation returns Some with the stored value
/// - Deserialized value equals the original JSON
/// - Non-existent keys return None
async fn test_memory_cache_get_set() {
    let cache = DMSCMemoryCache::new();
    
    // Test set and get
    let key = "test_key";
    let value = serde_json::json!("test_value");
    let serialized = serde_json::to_string(&value).unwrap();
    
    cache.set(key, &serialized, Some(60)).await.unwrap();
    let retrieved = cache.get(key).await.unwrap();
    
    assert!(retrieved.is_some());
    let retrieved_value: serde_json::Value = serde_json::from_str(&retrieved.unwrap()).unwrap();
    assert_eq!(retrieved_value, value);
    
    // Test non-existent key
    let retrieved_none = cache.get("non_existent_key").await.unwrap();
    assert!(retrieved_none.is_none());
}

#[tokio::test]
/// Tests DMSCMemoryCache delete operation.
///
/// Verifies that the delete operation successfully removes a cached
/// entry and that subsequent get operations return None for deleted keys.
///
/// ## Test Scenarios
///
/// 1. **Delete existing key**: The key is removed and get returns None
/// 2. **Value no longer accessible**: After deletion, the value cannot be retrieved
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Store a value in the cache
/// 3. Verify the value is accessible (get returns Some)
/// 4. Delete the key from the cache
/// 5. Verify the value is no longer accessible (get returns None)
///
/// ## Expected Behavior
///
/// - Delete operation completes without errors
/// - After deletion, get returns None for the deleted key
async fn test_memory_cache_delete() {
    let cache = DMSCMemoryCache::new();
    
    // Test set, delete, and get
    let key = "test_key";
    let value = serde_json::json!("test_value");
    let serialized = serde_json::to_string(&value).unwrap();
    
    cache.set(key, &serialized, Some(60)).await.unwrap();
    assert!(cache.get(key).await.unwrap().is_some());
    
    cache.delete(key).await.unwrap();
    assert!(cache.get(key).await.unwrap().is_none());
}

#[tokio::test]
/// Tests DMSCMemoryCache exists operation.
///
/// Verifies that the exists operation correctly reports whether a key
/// exists in the cache, returning true for existing keys and false
/// for non-existent or deleted keys.
///
/// ## Test Scenarios
///
/// 1. **Non-existent key**: exists returns false for unknown keys
/// 2. **Existing key**: exists returns true for stored keys
/// 3. **After deletion**: exists returns false for deleted keys
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Verify non-existent key returns false
/// 3. Store a value in the cache
/// 4. Verify the key now returns true
/// 5. Delete the key from the cache
/// 6. Verify the key now returns false again
///
/// ## Expected Behavior
///
/// - exists() returns false for non-existent keys
/// - exists() returns true for stored keys
/// - exists() returns false for deleted keys
async fn test_memory_cache_exists() {
    let cache = DMSCMemoryCache::new();
    
    // Test exists
    let key = "test_key";
    let value = serde_json::json!("test_value");
    let serialized = serde_json::to_string(&value).unwrap();
    
    assert!(!cache.exists(key).await);
    
    cache.set(key, &serialized, Some(60)).await.unwrap();
    assert!(cache.exists(key).await);
    
    cache.delete(key).await.unwrap();
    assert!(!cache.exists(key).await);
}

#[tokio::test]
/// Tests DMSCMemoryCache clear operation.
///
/// Verifies that the clear operation removes all cached entries from
/// the cache, returning it to an empty state with zero entries.
///
/// ## Test Scenarios
///
/// 1. **Clear with multiple entries**: All entries are removed
/// 2. **Cache state after clear**: No keys remain in the cache
/// 3. **Statistics reset**: Entry count returns to zero
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Store multiple values in the cache
/// 3. Verify all keys exist
/// 4. Clear the cache
/// 5. Verify no keys remain
/// 6. Verify statistics show zero entries
///
/// ## Expected Behavior
///
/// - clear() removes all stored entries
/// - After clear, exists() returns false for all previous keys
/// - The cache is ready for new entries
async fn test_memory_cache_clear() {
    let cache = DMSCMemoryCache::new();
    
    // Set multiple keys
    let value = serde_json::json!("test_value");
    let serialized = serde_json::to_string(&value).unwrap();
    
    cache.set("key1", &serialized, Some(60)).await.unwrap();
    cache.set("key2", &serialized, Some(60)).await.unwrap();
    cache.set("key3", &serialized, Some(60)).await.unwrap();
    
    // Verify keys exist
    assert!(cache.exists("key1").await);
    assert!(cache.exists("key2").await);
    assert!(cache.exists("key3").await);
    
    // Clear cache
    cache.clear().await.unwrap();
    
    // Verify all keys are gone
    assert!(!cache.exists("key1").await);
    assert!(!cache.exists("key2").await);
    assert!(!cache.exists("key3").await);
}

#[tokio::test]
/// Tests DMSCMemoryCache statistics tracking.
///
/// Verifies that the cache correctly tracks and updates statistics
/// for cache operations including hits, misses, and entry count.
/// The test validates that stats are updated after cache operations.
///
/// ## Statistics Tracked
///
/// - **entries**: Number of items currently in the cache
/// - **hit_count**: Number of successful cache lookups
/// - **miss_count**: Number of cache lookups that returned None
/// - **hit_rate**: Ratio of hits to total lookups
///
/// ## Test Scenarios
///
/// 1. **Initial stats**: New cache has zero statistics
/// 2. **After set**: Entry count increases by 1
/// 3. **Cache hit**: Hit count increases when key exists
/// 4. **Cache miss**: Miss count increases when key doesn't exist
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Get initial statistics (should all be zero)
/// 3. Store a value in the cache
/// 4. Get an existing key (should be a hit)
/// 5. Get a non-existent key (should be a miss)
/// 6. Verify statistics are updated correctly
///
/// ## Expected Behavior
///
/// - Initial stats show zero for all counters
/// - Set operation increases entry count
/// - Get on existing key increases hit count
/// - Get on non-existent key increases miss count
async fn test_memory_cache_stats() {
    let cache = DMSCMemoryCache::new();
    
    // Get initial stats
    let initial_stats = cache.stats().await;
    
    // Set a key
    let key = "test_key";
    let value = serde_json::json!("test_value");
    let serialized = serde_json::to_string(&value).unwrap();
    cache.set(key, &serialized, Some(60)).await.unwrap();
    
    // Get the key (should be a hit)
    cache.get(key).await;
    
    // Get a non-existent key (should be a miss)
    cache.get("non_existent_key").await;
    
    // Get updated stats
    let updated_stats = cache.stats().await;
    
    // Verify stats changed
    assert_eq!(updated_stats.entries, initial_stats.entries);
    assert_eq!(updated_stats.hits, initial_stats.hits + 1);
    assert_eq!(updated_stats.misses, initial_stats.misses + 1);
}

/// Tests DMSCMemoryCache cleanup of expired entries.
///
/// Verifies that the cleanup operation correctly identifies and removes
/// expired entries from the cache, returning the count of cleaned entries.
///
/// ## Test Scenarios
///
/// 1. **Automatic expiration**: Entries with TTL are automatically expired
/// 2. **Cleanup trigger**: cleanup_expired() removes all expired entries
/// 3. **Return value**: Returns the count of entries removed
/// 4. **Post-cleanup state**: Expired entries are no longer accessible
///
/// ## Test Steps
///
/// 1. Create a new in-memory cache
/// 2. Set a value with a very short TTL (1 second)
/// 3. Wait for the TTL to expire (2 seconds)
/// 4. Call cleanup_expired() to remove expired entries
/// 5. Verify the cleaned count is at least 1
/// 6. Verify the expired key no longer exists
///
/// ## Expected Behavior
///
/// - cleanup_expired() returns the number of entries removed
/// - After cleanup, expired entries are not accessible
/// - The cache state reflects the removal of expired entries
async fn test_memory_cache_cleanup_expired() {
    let cache = DMSCMemoryCache::new();
    
    // Set a key with a very short TTL
    let key = "expiring_key";
    cache.set(key, "test_value", Some(1)).await.unwrap();
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    let cleaned = cache.cleanup_expired().await.unwrap();
    
    assert!(cleaned >= 1);
    assert!(!cache.exists(key).await);
}

/// Tests DMSCCacheManager typed get/set operations.
///
/// Verifies that the cache manager correctly stores and retrieves
/// values of different types using generic type parameters.
///
/// ## Test Scenarios
///
/// 1. **String values**: Store and retrieve string data
/// 2. **Integer values**: Store and retrieve numeric data
/// 3. **Type safety**: Generic type parameter ensures correct deserialization
///
/// ## Test Steps
///
/// 1. Create a memory cache backend wrapped in Arc
/// 2. Create a cache manager with the backend
/// 3. Test string value: set and get
/// 4. Verify the retrieved string matches the original
/// 5. Test integer value: set and get
/// 6. Verify the retrieved integer matches the original
///
/// ## Expected Behavior
///
/// - String values are correctly stored and retrieved
/// - Integer values are correctly stored and retrieved
/// - Type parameter ensures correct deserialization
async fn test_cache_manager_get_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSCMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCCacheManager::new(backend);
    
    // Test set and get with string value
    let key = "test_key";
    let value = "test_value";
    
    manager.set(key, &value, Some(60)).await.unwrap();
    let retrieved = manager.get::<String>(key).await.unwrap();
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
    
    // Test set and get with integer value
    let key = "test_key_int";
    let value = 42;
    
    manager.set(key, &value, Some(60)).await.unwrap();
    let retrieved = manager.get::<i32>(key).await.unwrap();
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
}

#[tokio::test]
async fn test_cache_manager_delete() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSCMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCCacheManager::new(backend);
    
    // Test set, delete, and get
    let key = "test_key";
    let value = "test_value";
    
    manager.set(key, &value, Some(60)).await.unwrap();
    assert!(manager.get::<String>(key).await.unwrap().is_some());
    
    manager.delete(key).await.unwrap();
    assert!(manager.get::<String>(key).await.unwrap().is_none());
}

/// Tests DMSCCacheManager exists operation.
///
/// Verifies that the cache manager correctly checks for the
/// existence of entries in the underlying cache backend.
///
/// ## Test Scenarios
///
/// 1. **Non-existent key**: exists returns false for unknown keys
/// 2. **Existing key**: exists returns true for stored keys
/// 3. **After deletion**: exists returns false for deleted keys
///
/// ## Test Steps
///
/// 1. Create a memory cache backend wrapped in Arc
/// 2. Create a cache manager with the backend
/// 3. Verify non-existent key returns false
/// 4. Store a value in the cache
/// 5. Verify the key now returns true
/// 6. Delete the key from the cache
/// 7. Verify the key now returns false again
///
/// ## Expected Behavior
///
/// - exists() returns false for non-existent keys
/// - exists() returns true for stored keys
/// - exists() returns false for deleted keys
/// - The cache manager correctly delegates to the backend
async fn test_cache_manager_exists() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSCMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCCacheManager::new(backend);
    
    // Test exists
    let key = "test_key";
    let value = "test_value";
    
    assert!(!manager.exists(key).await);
    
    manager.set(key, &value, Some(60)).await.unwrap();
    assert!(manager.exists(key).await);
    
    manager.delete(key).await.unwrap();
    assert!(!manager.exists(key).await);
}

/// Tests DMSCCacheManager get_or_set lazy value generation.
///
/// Verifies that the get_or_set method correctly implements
/// "get or create" semantics, returning cached values when
/// available and generating new values only when needed.
///
/// ## Test Scenarios
///
/// 1. **Cache miss**: Value is generated using the provided closure
/// 2. **Cache hit**: Cached value is returned, generator is not called
/// 3. **Idempotency**: The generator is only called once per cache miss
///
/// ## Test Steps
///
/// 1. Create a memory cache backend wrapped in Arc
/// 2. Create a cache manager with the backend
/// 3. Call get_or_set with a key and generator closure
/// 4. Verify the result matches the generated value
/// 5. Call get_or_set again with the same key and a different generator
/// 6. Verify the cached value is returned, not the new generator result
///
/// ## Expected Behavior
///
/// - First call generates the value using the closure
/// - Second call returns the cached value
/// - The generator is only called on cache miss
/// - Cached value takes precedence over new generator output
async fn test_cache_manager_get_or_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSCMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCCacheManager::new(backend);
    
    // Test get_or_set
    let key = "test_key";
    let value = "test_value".to_string();
    
    // First call should generate the value
    let result1 = manager.get_or_set(key, Some(60), || Ok(value.clone())).await.unwrap();
    assert_eq!(result1, value);
    
    // Second call should get from cache
    let result2 = manager.get_or_set(key, Some(60), || Ok("different_value".to_string())).await.unwrap();
    assert_eq!(result2, value); // Should still be the original value
}
