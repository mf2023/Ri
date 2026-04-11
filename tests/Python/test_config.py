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
Ri Config Module Tests

Tests for the configuration management functionality.
"""

import pytest
from ri import (
    RiConfig,
    RiConfigManager,
)


class TestRiConfig:
    """Tests for RiConfig"""

    def test_config_creation(self):
        """Test creating configuration"""
        config = RiConfig()
        assert config is not None

    def test_config_set_get(self):
        """Test setting and getting config values"""
        config = RiConfig()

        config.set("database.host", "localhost")
        config.set("database.port", "5432")
        config.set("database.name", "test_db")

        assert config.get("database.host") == "localhost"
        assert config.get("database.port") == "5432"
        assert config.get("database.name") == "test_db"

    def test_config_get_nonexistent(self):
        """Test getting nonexistent config value"""
        config = RiConfig()
        
        result = config.get("nonexistent.key")
        assert result is None

    def test_config_get_f64(self):
        """Test getting float config values"""
        config = RiConfig()
        
        config.set("server.port", "8080")
        config.set("server.timeout", "30.5")
        
        assert config.get_f64("server.port") == 8080.0
        assert config.get_f64("server.timeout") == 30.5

    def test_config_get_usize(self):
        """Test getting usize config values"""
        config = RiConfig()
        
        config.set("server.port", "8080")
        config.set("server.timeout", "30")
        
        assert config.get_usize("server.port") == 8080
        assert config.get_usize("server.timeout") == 30

    def test_config_contains(self):
        """Test checking if key exists"""
        config = RiConfig()
        
        config.set("feature.enabled", "true")
        
        assert config.contains("feature.enabled") is True
        assert config.contains("nonexistent.key") is False


class TestRiConfigManager:
    """Tests for RiConfigManager"""

    def test_config_manager_creation(self):
        """Test creating config manager"""
        manager = RiConfigManager()
        assert manager is not None

    def test_config_manager_get(self):
        """Test getting config from manager"""
        manager = RiConfigManager()
        
        result = manager.get("nonexistent.key")
        assert result is None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
