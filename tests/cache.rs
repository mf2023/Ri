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

extern crate dms;

use dms::cache::{CachedValue, CacheStats, DMSCacheConfig, CacheBackendType, DMSCacheManager, DMSMemoryCache};
use std::time::Duration;

#[test]
fn test_cached_value_new() {
    let data = serde_json::json!("test_value");
    let ttl = Some(Duration::from_secs(60));
    
    let cached_value = CachedValue::_Fnew(data.clone(), ttl);
    
    assert_eq!(cached_value._Fget_data(), &data);
    assert!(!cached_value._Fis_expired());
    assert_eq!(cached_value.access_count, 0);
    assert!(cached_value.expires_at.is_some());
}

#[test]
fn test_cached_value_touch() {
    let data = serde_json::json!("test_value");
    let ttl = Some(Duration::from_secs(60));
    
    let mut cached_value = CachedValue::_Fnew(data.clone(), ttl);
    let initial_access_count = cached_value.access_count;
    
    // Touch the value
    cached_value._Ftouch();
    
    // Verify access count increased
    assert_eq!(cached_value.access_count, initial_access_count + 1);
}

#[test]
fn test_cached_value_expired() {
    let data = serde_json::json!("test_value");
    
    // Create a value with a very short TTL
    let ttl = Some(Duration::from_millis(10));
    let mut cached_value = CachedValue::_Fnew(data.clone(), ttl);
    
    // Should not be expired immediately
    assert!(!cached_value._Fis_expired());
    
    // Simulate expiration by manually setting a past expires_at
    cached_value.expires_at = Some(0); // 1970-01-01 00:00:00 UTC
    
    // Should be expired now
    assert!(cached_value._Fis_expired());
}

#[test]
fn test_cached_value_deserialize() {
    let data = serde_json::json!("test_value");
    let ttl = Some(Duration::from_secs(60));
    
    let cached_value = CachedValue::_Fnew(data.clone(), ttl);
    
    // Test deserialization to string
    let result: String = cached_value._Fdeserialize().unwrap();
    assert_eq!(result, "test_value");
    
    // Test deserialization to a different type should fail
    let result: Result<u32, _> = cached_value._Fdeserialize();
    assert!(result.is_err());
}

#[test]
fn test_cache_stats_default() {
    let stats = CacheStats::default();
    
    assert_eq!(stats.total_keys, 0);
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
    let cache = DMSMemoryCache::_Fnew();
    
    // Test set and get
    let key = "test_key";
    let value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_secs(60)));
    
    cache._Fset(key, value.clone()).await.unwrap();
    let retrieved = cache._Fget(key).await;
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap()._Fget_data(), value._Fget_data());
    
    // Test non-existent key
    let retrieved_none = cache._Fget("non_existent_key").await;
    assert!(retrieved_none.is_none());
}

#[tokio::test]
async fn test_memory_cache_delete() {
    let cache = DMSMemoryCache::_Fnew();
    
    // Test set, delete, and get
    let key = "test_key";
    let value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_secs(60)));
    
    cache._Fset(key, value.clone()).await.unwrap();
    assert!(cache._Fget(key).await.is_some());
    
    cache._Fdelete(key).await.unwrap();
    assert!(cache._Fget(key).await.is_none());
}

#[tokio::test]
async fn test_memory_cache_exists() {
    let cache = DMSMemoryCache::_Fnew();
    
    // Test exists
    let key = "test_key";
    let value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_secs(60)));
    
    assert!(!cache._Fexists(key).await);
    
    cache._Fset(key, value.clone()).await.unwrap();
    assert!(cache._Fexists(key).await);
    
    cache._Fdelete(key).await.unwrap();
    assert!(!cache._Fexists(key).await);
}

#[tokio::test]
async fn test_memory_cache_clear() {
    let cache = DMSMemoryCache::_Fnew();
    
    // Set multiple keys
    let value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_secs(60)));
    
    cache._Fset("key1", value.clone()).await.unwrap();
    cache._Fset("key2", value.clone()).await.unwrap();
    cache._Fset("key3", value.clone()).await.unwrap();
    
    // Verify keys exist
    assert!(cache._Fexists("key1").await);
    assert!(cache._Fexists("key2").await);
    assert!(cache._Fexists("key3").await);
    
    // Clear cache
    cache._Fclear().await.unwrap();
    
    // Verify all keys are gone
    assert!(!cache._Fexists("key1").await);
    assert!(!cache._Fexists("key2").await);
    assert!(!cache._Fexists("key3").await);
}

#[tokio::test]
async fn test_memory_cache_stats() {
    let cache = DMSMemoryCache::_Fnew();
    
    // Get initial stats
    let initial_stats = cache._Fstats().await;
    
    // Set a key
    let key = "test_key";
    let value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_secs(60)));
    cache._Fset(key, value.clone()).await.unwrap();
    
    // Get the key (should be a hit)
    cache._Fget(key).await;
    
    // Get a non-existent key (should be a miss)
    cache._Fget("non_existent_key").await;
    
    // Get updated stats
    let updated_stats = cache._Fstats().await;
    
    // Verify stats changed
    assert_eq!(updated_stats.total_keys, initial_stats.total_keys + 1);
    assert_eq!(updated_stats.hit_count, initial_stats.hit_count + 1);
    assert_eq!(updated_stats.miss_count, initial_stats.miss_count + 1);
}

#[tokio::test]
async fn test_memory_cache_cleanup_expired() {
    let cache = DMSMemoryCache::_Fnew();
    
    // Set a key with a very short TTL
    let key = "expiring_key";
    let mut value = CachedValue::_Fnew(serde_json::json!("test_value"), Some(Duration::from_millis(10)));
    
    // Manually set the expires_at to a past time
    value.expires_at = Some(0); // 1970-01-01 00:00:00 UTC
    
    cache._Fset(key, value.clone()).await.unwrap();
    assert!(cache._Fexists(key).await);
    
    // Cleanup expired entries
    let cleaned = cache._Fcleanup_expired().await.unwrap();
    
    // Verify the entry was cleaned up
    assert_eq!(cleaned, 1);
    assert!(!cache._Fexists(key).await);
}

#[tokio::test]
async fn test_cache_manager_get_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::_Fnew());
    
    // Create a cache manager
    let manager = DMSCacheManager::_Fnew(backend);
    
    // Test set and get with string value
    let key = "test_key";
    let value = "test_value";
    
    manager._Fset(key, &value, Some(60)).await.unwrap();
    let retrieved = manager._Fget::<String>(key).await.unwrap();
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
    
    // Test set and get with integer value
    let key = "test_key_int";
    let value = 42;
    
    manager._Fset(key, &value, Some(60)).await.unwrap();
    let retrieved = manager._Fget::<i32>(key).await.unwrap();
    
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap(), value);
}

#[tokio::test]
async fn test_cache_manager_delete() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::_Fnew());
    
    // Create a cache manager
    let manager = DMSCacheManager::_Fnew(backend);
    
    // Test set, delete, and get
    let key = "test_key";
    let value = "test_value";
    
    manager._Fset(key, &value, Some(60)).await.unwrap();
    assert!(manager._Fget::<String>(key).await.unwrap().is_some());
    
    manager._Fdelete(key).await.unwrap();
    assert!(manager._Fget::<String>(key).await.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_manager_exists() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::_Fnew());
    
    // Create a cache manager
    let manager = DMSCacheManager::_Fnew(backend);
    
    // Test exists
    let key = "test_key";
    let value = "test_value";
    
    assert!(!manager._Fexists(key).await);
    
    manager._Fset(key, &value, Some(60)).await.unwrap();
    assert!(manager._Fexists(key).await);
    
    manager._Fdelete(key).await.unwrap();
    assert!(!manager._Fexists(key).await);
}

#[tokio::test]
async fn test_cache_manager_get_or_set() {
    // Create a memory cache backend
    let backend = std::sync::Arc::new(DMSMemoryCache::_Fnew());
    
    // Create a cache manager
    let manager = DMSCacheManager::_Fnew(backend);
    
    // Test get_or_set
    let key = "test_key";
    let value = "test_value";
    
    // First call should generate the value
    let result1 = manager._Fget_or_set(key, Some(60), || Ok(value)).await.unwrap();
    assert_eq!(result1, value);
    
    // Second call should get from cache
    let result2 = manager._Fget_or_set(key, Some(60), || Ok("different_value")).await.unwrap();
    assert_eq!(result2, value); // Should still be the original value
}
