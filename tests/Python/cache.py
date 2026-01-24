#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# You may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

"""
DMSC Cache Module Python Tests.

This module contains comprehensive tests for the DMSC caching system Python bindings.
The cache system provides a unified interface for various cache backends with support
for TTL (time-to-live), memory management, and statistics tracking.

Cache Architecture:
- DMSCCacheModule: Main module coordinating cache functionality
- DMSCCacheManager: Core cache operations and entry management
- DMSCCacheConfig: Configuration for cache behavior and backend selection
- DMSCCachePolicy: Eviction and refresh policies
- DMSCCacheStats: Cache performance metrics
- DMSCCachedValue: Wrapper for cached entries with metadata

Backend Types:
- Memory: In-memory cache for single-instance deployments
- Redis: Distributed cache for multi-instance deployments
- Hybrid: Combination of memory and Redis for optimal performance

Test Classes:
- TestCacheConfig: Configuration options and defaults
- TestCacheBackendType: Backend type enumeration
- TestCachePolicy: Cache eviction and refresh policies
- TestCacheStats: Statistics tracking and metrics
- TestCachedValue: Cached entry with TTL support
- TestCacheManager: Core cache operations
- TestCacheModule: Module-level cache configuration
"""

import unittest
from datetime import timedelta
from dmsc import (
    DMSCCacheModule, DMSCCacheManager, DMSCCacheConfig,
    DMSCCacheBackendType, DMSCCachePolicy, DMSCCacheStats,
    DMSCCachedValue
)


class TestCacheConfig(unittest.TestCase):
    """Test suite for DMSCCacheConfig class.
    
    The DMSCCacheConfig class defines all configuration options for the cache system.
    It controls backend selection, memory limits, TTL defaults, and cleanup behavior.
    
    Configuration Properties:
    - enabled: Enable or disable caching globally (bool)
    - default_ttl_secs: Default time-to-live for cache entries in seconds (3600 = 1 hour)
    - max_memory_mb: Maximum memory usage in megabytes (512 MB default)
    - cleanup_interval_secs: Interval between cache cleanup operations (300 = 5 minutes)
    - backend_type: Which backend to use (Memory, Redis, or Hybrid)
    - redis_url: Connection URL for Redis backend (default: redis://127.0.0.1:6379)
    - redis_pool_size: Connection pool size for Redis (default: 10 connections)
    
    Backend Types:
    - Memory: In-memory cache, fastest but single-instance only, no persistence
    - Redis: Distributed cache, requires Redis server, persistent across restarts
    - Hybrid: Memory cache with Redis backup, best of both worlds
    
    Test Methods:
    - test_cache_config_default: Verify default configuration values
    - test_cache_config_setters: Test updating configuration properties
    - test_cache_config_default_config: Test static factory method
    """

    def test_cache_config_default(self):
        """Test default cache configuration.
        
        This test verifies the default values for DMSCCacheConfig:
        - Caching is enabled by default (True)
        - Default TTL is 3600 seconds (1 hour)
        - Maximum memory is 512 MB
        - Cleanup interval is 300 seconds (5 minutes)
        - Default backend is in-memory (DMSCCacheBackendType.Memory)
        - Default Redis URL points to localhost
        - Default pool size is 10 connections
        
        Expected Behavior:
        - All default values match the expected configuration
        - Cache is ready to use with sensible defaults
        """
        config = DMSCCacheConfig()
        self.assertTrue(config.enabled)
        self.assertEqual(config.default_ttl_secs, 3600)
        self.assertEqual(config.max_memory_mb, 512)
        self.assertEqual(config.cleanup_interval_secs, 300)
        self.assertEqual(str(config.backend_type), "DMSCCacheBackendType.Memory")
        self.assertEqual(config.redis_url, "redis://127.0.0.1:6379")
        self.assertEqual(config.redis_pool_size, 10)

    def test_cache_config_setters(self):
        """Test cache configuration setters.
        
        This test validates that configuration properties can be modified
        after object creation. All setters should properly update values.
        
        Configuration Changes:
        - Disable caching: enabled = False
        - Reduce TTL: default_ttl_secs = 1800 (30 minutes)
        - Reduce memory limit: max_memory_mb = 256 MB
        - Switch to Redis backend: backend_type = Redis
        - Change Redis URL: redis_url = "redis://example.com:6379"
        
        Expected Behavior:
        - All setter methods update the corresponding property
        - Changes are immediately reflected when reading properties
        """
        config = DMSCCacheConfig()
        config.enabled = False
        config.default_ttl_secs = 1800
        config.max_memory_mb = 256
        config.backend_type = DMSCCacheBackendType.Redis
        config.redis_url = "redis://example.com:6379"
        self.assertFalse(config.enabled)
        self.assertEqual(config.default_ttl_secs, 1800)
        self.assertEqual(config.max_memory_mb, 256)
        self.assertEqual(str(config.backend_type), "DMSCCacheBackendType.Redis")
        self.assertEqual(config.redis_url, "redis://example.com:6379")

    def test_cache_config_default_config_static_method(self):
        """Test default_config static method.
        
        The default_config() static method provides a convenient way to
        get a configuration with sensible defaults without manual setup.
        
        Method Behavior:
        - Returns a new DMSCCacheConfig with all defaults set
        - Equivalent to calling DMSCCacheConfig()
        - Useful for quick initialization
        
        Expected Behavior:
        - Returns a valid configuration object
        - Configuration has caching enabled
        - Default TTL is 3600 seconds (1 hour)
        """
        config = DMSCCacheConfig.default_config()
        self.assertTrue(config.enabled)
        self.assertEqual(config.default_ttl_secs, 3600)


class TestCacheBackendType(unittest.TestCase):
    """Test suite for DMSCCacheBackendType enum.
    
    The DMSCCacheBackendType enum defines the available cache backend options.
    Each backend has different characteristics suited for different deployment
    scenarios.
    
    Backend Characteristics:
    - Memory: Fastest access, single-instance only, no persistence, ideal for dev/test
    - Redis: Distributed, persistent, requires Redis server, good for multi-instance prod
    - Hybrid: Memory cache with Redis backup, combines speed with persistence
    
    String Representation:
    Each enum variant has a string representation in format "DMSCCacheBackendType.X"
    where X is the variant name (Memory, Redis, or Hybrid).
    
    Test Methods:
    - test_backend_type_values: Verify enum value string representations
    """

    def test_backend_type_values(self):
        """Test backend type enum values.
        
        Each backend type should have a string representation that can be
        used for configuration and logging purposes.
        
        Expected Values:
        - DMSCCacheBackendType.Memory -> "DMSCCacheBackendType.Memory"
        - DMSCCacheBackendType.Redis -> "DMSCCacheBackendType.Redis"
        - DMSCCacheBackendType.Hybrid -> "DMSCCacheBackendType.Hybrid"
        """
        self.assertEqual(str(DMSCCacheBackendType.Memory), "DMSCCacheBackendType.Memory")
        self.assertEqual(str(DMSCCacheBackendType.Redis), "DMSCCacheBackendType.Redis")
        self.assertEqual(str(DMSCCacheBackendType.Hybrid), "DMSCCacheBackendType.Hybrid")


class TestCachePolicy(unittest.TestCase):
    """Test suite for DMSCCachePolicy class.
    
    The DMSCCachePolicy class defines how cache entries are managed,
    including eviction strategies and refresh behavior.
    
    Policy Options:
    - ttl: Time-to-live for cache entries (timedelta or None for no expiration)
    - refresh_on_access: Whether to update TTL on each access (LRU-like behavior)
    - max_size: Maximum number of entries before eviction (None for unlimited)
    
    Eviction Strategies:
    - TTL-based: Entries expire after their TTL, automatically removed
    - LRU (Least Recently Used): Evict least recently accessed entries first
    - LFU (Least Frequently Used): Evict least frequently accessed entries
    - Size-based: Evict when entry count exceeds max_size
    
    Test Methods:
    - test_cache_policy_default: Verify default policy values
    - test_cache_policy_setters: Test policy property updates
    - test_cache_policy_default_policy: Test static factory method
    """

    def test_cache_policy_default(self):
        """Test default cache policy.
        
        This test verifies that a newly created policy has sensible defaults:
        - TTL is set to some value (not None, indicating expiration is enabled)
        - Refresh on access is disabled by default (False)
        - No maximum size limit (None)
        
        Expected Behavior:
        - policy.ttl is not None (has a value)
        - policy.refresh_on_access is False
        - policy.max_size is None (unlimited)
        """
        policy = DMSCCachePolicy()
        self.assertIsNotNone(policy.ttl)
        self.assertFalse(policy.refresh_on_access)
        self.assertIsNone(policy.max_size)

    def test_cache_policy_setters(self):
        """Test cache policy setters.
        
        This test validates that policy properties can be modified:
        - Set TTL to 1 hour using timedelta
        - Enable refresh on access (LRU behavior)
        - Set maximum size to 2048 entries
        
        Policy Configuration:
        - ttl = timedelta(hours=1): Entries expire after 1 hour of inactivity
        - refresh_on_access = True: Accessing an entry resets its TTL (LRU)
        - max_size = 2048: Cache evicts oldest entries when exceeding this count
        
        Expected Behavior:
        - All setter methods update the corresponding property
        - Policy correctly reflects configured values
        """
        policy = DMSCCachePolicy()
        policy.ttl = timedelta(seconds=3600)
        policy.refresh_on_access = True
        policy.max_size = 2048
        self.assertEqual(policy.ttl, timedelta(seconds=3600))
        self.assertTrue(policy.refresh_on_access)
        self.assertEqual(policy.max_size, 2048)

    def test_cache_policy_default_policy_static_method(self):
        """Test default_policy static method.
        
        The default_policy() method provides a standard policy configuration
        suitable for most use cases without requiring explicit setup.
        
        Method Behavior:
        - Returns a new DMSCCachePolicy with sensible defaults
        - TTL is typically set (e.g., 1 hour)
        - Refresh on access is disabled
        - No maximum size limit
        
        Expected Behavior:
        - Returns a valid policy object
        - Policy has TTL configured
        - Refresh on access is disabled
        """
        policy = DMSCCachePolicy.default_policy()
        self.assertIsNotNone(policy.ttl)
        self.assertFalse(policy.refresh_on_access)


class TestCacheStats(unittest.TestCase):
    """Test suite for DMSCCacheStats class.
    
    The DMSCCacheStats class provides detailed metrics about cache performance,
    essential for monitoring, optimization, and capacity planning.
    
    Statistics Tracked:
    - hits: Number of successful cache lookups (requests that found data)
    - misses: Number of cache lookups that failed (requests that didn't find data)
    - entries: Current number of cached entries
    - memory_usage_bytes: Current memory consumption in bytes
    - avg_hit_rate: Ratio of hits to total lookups (hits / (hits + misses))
    - hit_count: Total hits (alternative naming)
    - miss_count: Total misses (alternative naming)
    - eviction_count: Number of entries evicted due to size limits
    
    Performance Metrics:
    - Hit Rate: hits / (hits + misses) - higher is better (target: >90%)
    - Eviction Rate: evictions / total insertions - indicates if cache is sized correctly
    - Memory Usage: Current cache memory footprint for capacity planning
    
    Test Methods:
    - test_cache_stats_default: Verify initial statistics are zero
    - test_cache_stats_setters: Test statistic property updates
    - test_cache_stats_default_stats: Test static factory method
    """

    def test_cache_stats_default(self):
        """Test default cache statistics.
        
        A newly created stats object should have all metrics initialized to zero:
        - No hits or misses recorded yet (0)
        - No entries in cache (0)
        - No memory usage (0 bytes)
        - No evictions (0)
        - Average hit rate is 0.0 (no data to calculate)
        
        Expected Behavior:
        - All numeric fields are initialized to 0
        - Object represents an empty cache with no performance history
        """
        stats = DMSCCacheStats()
        self.assertEqual(stats.hits, 0)
        self.assertEqual(stats.misses, 0)
        self.assertEqual(stats.entries, 0)
        self.assertEqual(stats.memory_usage_bytes, 0)
        self.assertEqual(stats.avg_hit_rate, 0.0)
        self.assertEqual(stats.hit_count, 0)
        self.assertEqual(stats.miss_count, 0)
        self.assertEqual(stats.eviction_count, 0)

    def test_cache_stats_setters(self):
        """Test cache statistics setters.
        
        Statistics can be updated to reflect actual cache performance:
        - 200 hits, 50 misses = 200/250 = 80% hit rate
        - 100 current entries in cache
        - 2048 bytes (2 KB) of memory used
        
        Example Metrics:
        - hits = 200: 200 successful cache lookups
        - misses = 50: 50 cache misses
        - entries = 100: 100 items currently cached
        - memory_usage_bytes = 2048: 2 KB memory footprint
        - avg_hit_rate = 0.75: 75% hit ratio
        
        Expected Behavior:
        - All properties accept integer/float values
        - Values are stored and retrievable
        """
        stats = DMSCCacheStats()
        stats.hits = 200
        stats.misses = 50
        stats.entries = 100
        stats.memory_usage_bytes = 2048
        stats.avg_hit_rate = 0.75
        self.assertEqual(stats.hits, 200)
        self.assertEqual(stats.misses, 50)
        self.assertEqual(stats.entries, 100)
        self.assertEqual(stats.memory_usage_bytes, 2048)
        self.assertEqual(stats.avg_hit_rate, 0.75)

    def test_cache_stats_default_stats_static_method(self):
        """Test default_stats static method.
        
        The default_stats() method provides a fresh stats object initialized
        to zero, ready for tracking cache performance from a clean slate.
        
        Method Behavior:
        - Returns a new DMSCCacheStats with all metrics at zero
        - Useful for initializing a new cache's statistics
        - Equivalent to DMSCCacheStats()
        
        Expected Behavior:
        - Returns a valid statistics object
        - All counters start at 0
        - Ready to record cache operations
        """
        stats = DMSCCacheStats.default_stats()
        self.assertEqual(stats.hits, 0)
        self.assertEqual(stats.misses, 0)


class TestCachedValue(unittest.TestCase):
    """
    Test suite for DMSCCachedValue class.

    The DMSCCachedValue class wraps cached data with metadata including
    expiration time and last access time. This enables TTL-based eviction
    and LRU-style refresh policies.

    Value Metadata:
    - value: The actual cached data
    - expires_at: Timestamp when the entry expires (None for persistent)
    - last_accessed: Timestamp of last access (used for LRU policies)

    Use Cases:
    - TTL-based expiration: Check is_expired() before returning value
    - LRU refresh: Call touch() on each access to update last_accessed
    - Persistent caching: Use None TTL to never expire

    Test Methods:
    - test_cached_value_with_ttl: Verify TTL-based expiration
    - test_cached_value_without_ttl: Test persistent caching
    - test_cached_value_is_expired: Test expiration check method
    - test_cached_value_touch: Test access timestamp update
    - test_cached_value_default: Test static factory method
    """

    def test_cached_value_with_ttl(self):
        """Test cached value with TTL.

        When creating a cached value with a TTL (time-to-live):
        - The value is stored correctly
        - An expiration timestamp is set based on TTL
        - A last access timestamp is recorded
        """
        cached = DMSCCachedValue("test_value", 3600)
        self.assertEqual(cached.value, "test_value")
        self.assertIsNotNone(cached.expires_at)
        self.assertIsNotNone(cached.last_accessed)

    def test_cached_value_without_ttl(self):
        """Test cached value without TTL (never expires).

        When TTL is None, the value persists indefinitely:
        - The value is stored correctly
        - No expiration timestamp is set
        - Last access timestamp is still recorded
        """
        cached = DMSCCachedValue("persistent_value", None)
        self.assertEqual(cached.value, "persistent_value")
        self.assertIsNone(cached.expires_at)
        self.assertIsNotNone(cached.last_accessed)

    def test_cached_value_is_expired_method(self):
        """Test is_expired method.

        The is_expired() method checks if the current time is past
        the expiration timestamp. For persistent entries (no TTL),
        this always returns False.
        """
        cached = DMSCCachedValue("test_value", None)
        self.assertFalse(cached.is_expired())

    def test_cached_value_touch(self):
        """Test cached value touch method.

        The touch() method updates the last_accessed timestamp to
        the current time. This is used for LRU-style refresh policies
        where access resets the eviction order.
        """
        cached = DMSCCachedValue("test_value", None)
        cached.touch()
        self.assertIsNotNone(cached.last_accessed)

    def test_cached_value_default_static_method(self):
        """Test default static method.

        The default() method creates a cached value with an empty
        string as the value and no expiration. Useful for initialization.
        """
        cached = DMSCCachedValue.default()
        self.assertEqual(cached.value, "")


class TestCacheManager(unittest.TestCase):
    """
    Test suite for DMSCCacheManager class.

    The DMSCCacheManager class provides the core cache operations:
    - get(key): Retrieve a value from cache
    - set(key, value, ttl): Store a value in cache
    - delete(key): Remove a value from cache
    - clear(): Remove all entries
    - stats(): Get cache statistics

    Cache Operations:
    - Lookups: Check cache before expensive operations
    - Storage: Store results for future requests
    - Invalidation: Remove stale or sensitive data
    - Bulk Operations: Clear entire cache when needed

    Test Methods:
    - test_cache_manager_new: Verify manager instantiation
    - test_cache_manager_stats: Test statistics retrieval
    """

    def test_cache_manager_new(self):
        """Test creating a new cache manager.

        This test verifies that DMSCCacheManager can be instantiated.
        The manager is ready to perform cache operations.
        """
        manager = DMSCCacheManager()
        self.assertIsNotNone(manager)

    def test_cache_manager_stats(self):
        """Test cache manager stats.

        The stats() method returns a DMSCCacheStats object containing
        current cache performance metrics.
        """
        manager = DMSCCacheManager()
        stats = manager.stats()
        self.assertIsNotNone(stats)


class TestCacheModule(unittest.TestCase):
    """
    Test suite for DMSCCacheModule class.

    The DMSCCacheModule class provides module-level cache configuration
    and initialization. It serves as the entry point for the cache subsystem.

    Module Responsibilities:
    - Configuration: Set up cache based on provided config
    - Initialization: Prepare cache backends and connection pools
    - Lifecycle: Start and stop the cache system
    - Health: Monitor cache backend health

    Test Methods:
    - test_cache_module_creation: Test module initialization with config
    """

    def test_cache_module_creation(self):
        """Test creating a cache module.

        This test verifies that DMSCCacheModule can be instantiated
        with a valid configuration. The module initializes the cache
        system according to the specified settings.

        Configuration Steps:
        1. Enable the cache module
        2. Set default TTL to 1 hour
        3. Limit memory to 512 MB
        4. Select in-memory backend
        5. Configure Redis URL (though not used for memory backend)
        """
        config = DMSCCacheConfig()
        config.enabled = True
        config.default_ttl_secs = 3600
        config.max_memory_mb = 512
        config.backend_type = DMSCCacheBackendType.Memory
        config.redis_url = "redis://127.0.0.1:6379"
        config.redis_pool_size = 10
        
        module = DMSCCacheModule(config)
        self.assertIsNotNone(module)


if __name__ == "__main__":
    unittest.main()
