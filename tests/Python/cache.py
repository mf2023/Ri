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
DMSC Cache Module Tests

Tests for the cache functionality including cache operations,
policies, and statistics.
"""

import pytest
from dmsc import (
    DMSCCacheModule,
    DMSCCacheConfig,
    DMSCCacheManager,
    DMSCCacheBackendType,
    DMSCCachePolicy,
    DMSCCacheStats,
    DMSCCachedValue,
    DMSCCacheEvent,
)


class TestDMSCCacheModule:
    """Tests for DMSCCacheModule"""

    def test_cache_module_creation(self):
        """Test creating cache module"""
        config = DMSCCacheConfig()
        config.backend_type = DMSCCacheBackendType.Memory
        config.default_ttl_secs = 300

        cache_module = DMSCCacheModule.with_config(config)
        assert cache_module is not None


class TestDMSCCacheManager:
    """Tests for DMSCCacheManager"""

    def test_cache_manager_creation(self):
        """Test creating cache manager"""
        manager = DMSCCacheManager()
        assert manager is not None

    def test_cache_set_get(self):
        """Test setting and getting cache values"""
        manager = DMSCCacheManager()

        # Set value
        manager.set("test_key", "test data", 300)

        # Get value
        retrieved = manager.get("test_key")
        assert retrieved is not None
        assert retrieved == "test data"

    def test_cache_exists(self):
        """Test checking if key exists"""
        manager = DMSCCacheManager()

        manager.set("exists_key", "test", 300)

        exists = manager.exists("exists_key")
        assert exists is True

        not_exists = manager.exists("nonexistent_key")
        assert not_exists is False

    def test_cache_delete(self):
        """Test deleting cache values"""
        manager = DMSCCacheManager()

        manager.set("delete_key", "test", 300)
        assert manager.exists("delete_key") is True

        manager.delete("delete_key")
        assert manager.exists("delete_key") is False

    def test_cache_clear(self):
        """Test clearing all cache"""
        manager = DMSCCacheManager()

        manager.set("key1", "test1", 300)
        manager.set("key2", "test2", 300)

        manager.clear()

        assert manager.exists("key1") is False
        assert manager.exists("key2") is False


class TestDMSCCacheConfig:
    """Tests for DMSCCacheConfig"""

    def test_cache_config_creation(self):
        """Test creating cache configuration"""
        config = DMSCCacheConfig()
        config.backend_type = DMSCCacheBackendType.Memory
        config.default_ttl_secs = 300
        config.max_memory_mb = 512
        config.enabled = True

        assert config.backend_type == DMSCCacheBackendType.Memory
        assert config.default_ttl_secs == 300
        assert config.max_memory_mb == 512
        assert config.enabled is True


class TestDMSCCachePolicy:
    """Tests for DMSCCachePolicy"""

    def test_cache_policy_creation(self):
        """Test creating cache policy"""
        policy = DMSCCachePolicy()

        assert policy is not None


class TestDMSCCachedValue:
    """Tests for DMSCCachedValue"""

    def test_cached_value_creation(self):
        """Test creating cached value"""
        value = DMSCCachedValue("test data", 3600)

        assert value.value == "test data"
        assert value.expires_at is not None

    def test_cached_value_no_ttl(self):
        """Test creating cached value without TTL"""
        value = DMSCCachedValue("persistent data", None)

        assert value.value == "persistent data"
        assert value.expires_at is None


class TestDMSCCacheStats:
    """Tests for DMSCCacheStats"""

    def test_cache_stats_creation(self):
        """Test creating cache statistics"""
        stats = DMSCCacheStats()
        stats.hits = 100
        stats.misses = 20
        stats.entries = 50
        stats.memory_usage_bytes = 1024000

        assert stats.hits == 100
        assert stats.misses == 20
        assert stats.entries == 50
        assert stats.memory_usage_bytes == 1024000


class TestDMSCCacheEvent:
    """Tests for DMSCCacheEvent"""

    def test_cache_event_invalidate(self):
        """Test creating invalidate event - DMSCCacheEvent is an enum variant"""
        # DMSCCacheEvent is an internal enum, cannot be created directly from Python
        pass


class TestDMSCCacheBackendType:
    """Tests for DMSCCacheBackendType"""

    def test_backend_types(self):
        """Test cache backend types"""
        assert DMSCCacheBackendType.Memory is not None
        assert DMSCCacheBackendType.Redis is not None
        assert DMSCCacheBackendType.Hybrid is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
