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
DMSC Config Module Tests

Tests for the configuration management functionality.
"""

import pytest
from dmsc import (
    DMSCConfig,
    DMSCConfigManager,
)


class TestDMSCConfig:
    """Tests for DMSCConfig"""

    def test_config_creation(self):
        """Test creating configuration"""
        config = DMSCConfig()
        assert config is not None

    def test_config_set_get(self):
        """Test setting and getting config values"""
        config = DMSCConfig()

        config.set("database.host", "localhost")
        config.set("database.port", 5432)
        config.set("database.name", "test_db")

        assert config.get("database.host") == "localhost"
        assert config.get("database.port") == 5432
        assert config.get("database.name") == "test_db"

    def test_config_with_nested_values(self):
        """Test configuration with nested values"""
        config = DMSCConfig()

        nested_config = {
            "server": {
                "host": "0.0.0.0",
                "port": 8080
            },
            "logging": {
                "level": "info",
                "format": "json"
            }
        }

        config.set("app", nested_config)

        app_config = config.get("app")
        assert app_config["server"]["host"] == "0.0.0.0"
        assert app_config["server"]["port"] == 8080


class TestDMSCConfigManager:
    """Tests for DMSCConfigManager"""

    def test_config_manager_creation(self):
        """Test creating config manager"""
        manager = DMSCConfigManager()
        assert manager is not None

    def test_load_from_dict(self):
        """Test loading config from dictionary"""
        manager = DMSCConfigManager()

        config_dict = {
            "app_name": "test-app",
            "version": "1.0.0",
            "environment": "test"
        }

        config = manager.load_from_dict(config_dict)

        assert config is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
