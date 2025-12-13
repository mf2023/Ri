// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dms_core::cache::{CachedValue, CacheStats, DMSCacheConfig, CacheBackendType, DMSCacheManager, DMSMemoryCache, DMSCache};
use std::time::Duration;

#[test]
fn test_cached_value_new() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let cached_value = CachedValue::new(serialized, ttl_seconds);
    
    let deserialized: serde_json::Value = cached_value.deserialize().unwrap();
    assert_eq!(deserialized, data);
    assert!(!cached_value.is_expired());
    assert!(cached_value.expires_at.is_some());
}

#[test]
fn test_cached_value_touch() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let mut cached_value = CachedValue::new(serialized, ttl_seconds);
    
    cached_value.touch();
}

#[test]
fn test_cached_value_expired() {
    let data = serde_json::json!("test_value");
    
    // Create a value with a very short TTL
    let ttl_seconds = Some(1_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    let mut cached_value = CachedValue::new(serialized, ttl_seconds);
    
    // Should not be expired immediately
    assert!(!cached_value.is_expired());
    
    // Simulate expiration by manually setting a past expires_at
    cached_value.expires_at = Some(0); // 1970-01-01 00:00:00 UTC
    
    // Should be expired now
    assert!(cached_value.is_expired());
}

#[test]
fn test_cached_value_deserialize() {
    let data = serde_json::json!("test_value");
    let ttl_seconds = Some(60_u64);
    let serialized = serde_json::to_string(&data).unwrap();
    
    let cached_value = CachedValue::new(serialized, ttl_seconds);
    
    // Test deserialization to string
    let result: String = cached_value.deserialize().unwrap();
    assert_eq!(result, "test_value");
    
    // Test deserialization to a different type should fail
    let result: Result<u32, _> = cached_value.deserialize();
    assert!(result.is_err());
}

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats::default();
    
    assert_eq!(stats.entries, 0);
    assert_eq!(stats.memory_usage_bytes, 0);
    assert_eq!(stats.hit_count, 0);
    assert_eq!(stats.miss_count, 0);
    assert_eq!(stats.eviction_count, 0);
    assert_eq!(stats.avg_hit_rate, 0.0);
}

#[test]
fn test_cache_config_default() {
    let config = DMSCacheConfig::default();
    
    assert!(config.enabled);
    assert_eq!(config.default_ttl_secs, 3600);
    assert_eq!(config.max_memory_mb, 512);
    assert_eq!(config.cleanup_interval_secs, 300);
    assert_eq!(config.backend_type, CacheBackendType::Memory);
    assert_eq!(config.redis_url, "redis://127.0.0.1:6379");
    assert_eq!(config.redis_pool_size, 10);
}

#[test]
fn test_cache_backend_type_from_str() {
    // Test from_str_custom method
    assert_eq!(CacheBackendType::from_str_custom("memory"), CacheBackendType::Memory);
    assert_eq!(CacheBackendType::from_str_custom("redis"), CacheBackendType::Redis);
    assert_eq!(CacheBackendType::from_str_custom("hybrid"), CacheBackendType::Hybrid);
    assert_eq!(CacheBackendType::from_str_custom("invalid"), CacheBackendType::Memory);
    
    // Test standard FromStr trait
    assert_eq!("memory".parse::<CacheBackendType>().unwrap(), CacheBackendType::Memory);
    assert_eq!("redis".parse::<CacheBackendType>().unwrap(), CacheBackendType::Redis);
    assert_eq!("hybrid".parse::<CacheBackendType>().unwrap(), CacheBackendType::Hybrid);
    assert_eq!("invalid".parse::<CacheBackendType>().unwrap(), CacheBackendType::Memory);
}

#[tokio::test]
async fn test_memory_cache_get_set() {
    let cache = DMSMemoryCache::new();
    
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
async fn test_memory_cache_delete() {
    let cache = DMSMemoryCache::new();
    
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
async fn test_memory_cache_exists() {
    let cache = DMSMemoryCache::new();
    
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
async fn test_memory_cache_clear() {
    let cache = DMSMemoryCache::new();
    
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
async fn test_memory_cache_stats() {
    let cache = DMSMemoryCache::new();
    
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

#[tokio::test]
async fn test_memory_cache_cleanup_expired() {
    let cache = DMSMemoryCache::new();
    
    // Set a key with a very short TTL
    let key = "expiring_key";
    cache.set(key, "test_value", Some(1)).await.unwrap();
    
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    let cleaned = cache.cleanup_expired().await.unwrap();
    
    assert!(cleaned >= 1);
    assert!(!cache.exists(key).await);
}

#[tokio::test]
async fn test_cache_manager_get_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCacheManager::new(backend);
    
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
    let backend = std::sync::Arc::new(DMSMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCacheManager::new(backend);
    
    // Test set, delete, and get
    let key = "test_key";
    let value = "test_value";
    
    manager.set(key, &value, Some(60)).await.unwrap();
    assert!(manager.get::<String>(key).await.unwrap().is_some());
    
    manager.delete(key).await.unwrap();
    assert!(manager.get::<String>(key).await.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_manager_exists() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCacheManager::new(backend);
    
    // Test exists
    let key = "test_key";
    let value = "test_value";
    
    assert!(!manager.exists(key).await);
    
    manager.set(key, &value, Some(60)).await.unwrap();
    assert!(manager.exists(key).await);
    
    manager.delete(key).await.unwrap();
    assert!(!manager.exists(key).await);
}

#[tokio::test]
async fn test_cache_manager_get_or_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::new());
    
    // Create a cache manager
    let manager = DMSCacheManager::new(backend);
    
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
