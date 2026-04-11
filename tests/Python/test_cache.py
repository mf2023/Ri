#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri Cache Module Tests

Tests for the cache functionality including cache operations,
policies, and statistics.
"""

import pytest
from ri import (
    RiCacheModule,
    RiCacheConfig,
    RiCacheBackendType,
    RiCacheStats,
    RiCachedValue,
)


class TestRiCacheModule:
    """Tests for RiCacheModule"""

    def test_cache_module_creation(self):
        """Test creating cache module"""
        config = RiCacheConfig.default_config()
        config.backend_type = RiCacheBackendType.Memory
        config.default_ttl_secs = 300

        cache_module = RiCacheModule(config)
        assert cache_module is not None


class TestRiCacheConfig:
    """Tests for RiCacheConfig"""

    def test_cache_config_creation(self):
        """Test creating cache configuration"""
        config = RiCacheConfig.default_config()
        config.backend_type = RiCacheBackendType.Memory
        config.default_ttl_secs = 300
        config.max_memory_mb = 512
        config.enabled = True

        assert str(config.backend_type) == "RiCacheBackendType.Memory"
        assert config.default_ttl_secs == 300
        assert config.max_memory_mb == 512
        assert config.enabled is True


class TestRiCachedValue:
    """Tests for RiCachedValue"""

    def test_cached_value_creation(self):
        """Test creating cached value"""
        value = RiCachedValue("test data", 3600)

        assert value.value == "test data"
        assert value.expires_at is not None

    def test_cached_value_no_ttl(self):
        """Test creating cached value without TTL"""
        value = RiCachedValue("persistent data", None)

        assert value.value == "persistent data"
        assert value.expires_at is None


class TestRiCacheStats:
    """Tests for RiCacheStats"""

    def test_cache_stats_creation(self):
        """Test creating cache statistics"""
        stats = RiCacheStats()
        stats.hits = 100
        stats.misses = 20
        stats.entries = 50
        stats.memory_usage_bytes = 1024000

        assert stats.hits == 100
        assert stats.misses == 20
        assert stats.entries == 50
        assert stats.memory_usage_bytes == 1024000


class TestRiCacheBackendType:
    """Tests for RiCacheBackendType"""

    def test_backend_types(self):
        """Test cache backend types"""
        assert RiCacheBackendType.Memory is not None
        assert RiCacheBackendType.Redis is not None
        assert RiCacheBackendType.Hybrid is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
