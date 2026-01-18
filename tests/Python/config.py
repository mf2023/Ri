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
DMSC Configuration Module Python Tests.

This module contains comprehensive tests for the DMSC configuration system
Python bindings. The configuration system provides a hierarchical key-value
store with support for multiple configuration sources and type-safe retrieval.

Configuration Architecture:
- DMSCConfig: Individual configuration container with key-value storage
- DMSCConfigManager: Manages multiple configuration sources and merging

Configuration Sources:
- File Sources: YAML, JSON, TOML configuration files
- Environment Sources: Environment variables with prefix matching
- Command Line: Command line arguments
- In-memory: Runtime configuration from code

Key Features:
- Hierarchical Keys: Dot-notation keys (e.g., "server.port", "database.host")
- Type Safety: get_f64(), get_usize() for type-specific retrieval
- Source Priority: Later sources override earlier ones
- Change Detection: Track configuration changes

Test Classes:
- TestDMSCConfig: Individual configuration container tests
- TestDMSCConfigManager: Configuration manager and source handling
"""

import unittest
from dmsc import DMSCConfig, DMSCConfigManager


class TestDMSCConfig(unittest.TestCase):
    """
    Test suite for DMSCConfig class.

    The DMSCConfig class provides a hierarchical key-value configuration store
    with support for type-specific value retrieval. It is the fundamental building
    block of the DMSC configuration system.

    Key-Value Model:
    - Keys are strings using dot-notation for hierarchy (e.g., "server.port")
    - Values are stored as strings internally
    - Type-specific getters convert strings to desired types

    Type Conversions:
    - get(): Returns string value (default None if not found)
    - get_f64(): Converts to 64-bit floating point
    - get_usize(): Converts to unsigned size type (typically for counts/sizes)

    Common Use Cases:
    - Server configuration (host, port, timeout)
    - Database connection strings
    - Feature flags and toggle settings
    - Performance tuning parameters

    Test Methods:
    - test_config_new: Verify config instantiation
    - test_config_set_get: Test basic key-value operations
    - test_config_get_f64: Test floating-point value retrieval
    - test_config_get_usize: Test unsigned integer retrieval
    - test_config_keys: Test key enumeration
    - test_config_values: Test value enumeration
    - test_config_contains: Test key existence checking
    - test_config_len: Test configuration size
    """

    def test_config_new(self):
        """Test creating a new config.

        This test verifies that DMSCConfig can be instantiated.
        An empty config is ready to receive key-value pairs.
        """
        config = DMSCConfig()
        self.assertIsNotNone(config)

    def test_config_set_get(self):
        """Test setting and getting config values.

        This test validates the basic key-value operations:
        1. Set a value using dot-notation key ("server.port")
        2. Retrieve the same value using get()
        3. Verify the value matches what was set

        The dot-notation supports hierarchical configuration,
        allowing logical grouping of related settings.
        """
        config = DMSCConfig()
        config.set("server.port", "8080")
        result = config.get("server.port")
        self.assertEqual(result, "8080")

    def test_config_get_f64(self):
        """Test getting config value as f64.

        The get_f64() method converts string values to floating-point numbers.
        This is useful for numeric configuration like timeouts, weights, or rates.
        """
        config = DMSCConfig()
        config.set("server.port", "8080")
        result = config.get_f64("server.port")
        self.assertEqual(result, 8080.0)

    def test_config_get_usize(self):
        """Test getting config value as usize.

        The get_usize() method converts string values to unsigned integers.
        This is useful for configuration like pool sizes, limits, or counts.
        """
        config = DMSCConfig()
        config.set("pool.size", "10")
        result = config.get_usize("pool.size")
        self.assertEqual(result, 10)

    def test_config_keys(self):
        """Test getting config keys.

        The keys() method returns a list of all configuration keys.
        This is useful for iterating over configuration or for
        configuration introspection and validation.
        """
        config = DMSCConfig()
        config.set("key1", "value1")
        config.set("key2", "value2")
        keys = config.keys()
        self.assertEqual(len(keys), 2)
        self.assertIn("key1", keys)
        self.assertIn("key2", keys)

    def test_config_values(self):
        """Test getting config values.

        The values() method returns a list of all configuration values.
        This is useful for processing configuration data or for
        creating configuration summaries.
        """
        config = DMSCConfig()
        config.set("key1", "value1")
        config.set("key2", "value2")
        values = config.values()
        self.assertEqual(len(values), 2)
        self.assertIn("value1", values)
        self.assertIn("value2", values)

    def test_config_contains(self):
        """Test checking if key exists.

        The contains() method checks whether a key exists in the
        configuration without throwing an exception. This is safer
        than catching exceptions for key lookup failures.
        """
        config = DMSCConfig()
        config.set("key1", "value1")
        self.assertTrue(config.contains("key1"))
        self.assertFalse(config.contains("key2"))

    def test_config_len(self):
        """Test getting config length.

        The len() method returns the number of configuration entries.
        This is useful for checking if configuration is empty or
        for displaying configuration summaries.
        """
        config = DMSCConfig()
        self.assertEqual(config.len(), 0)
        config.set("key1", "value1")
        self.assertEqual(config.len(), 1)
        config.set("key2", "value2")
        self.assertEqual(config.len(), 2)


class TestDMSCConfigManager(unittest.TestCase):
    """
    Test suite for DMSCConfigManager class.

    The DMSCConfigManager class coordinates multiple configuration sources
    and provides a unified interface for configuration access. It handles
    source registration, loading, and value resolution with proper precedence.

    Source Management:
    - add_file_source(): Load configuration from file
    - add_environment_source(): Load from environment variables
    - add_command_line_source(): Load from command line arguments

    Value Resolution:
    1. Check sources in reverse order of addition (later overrides earlier)
    2. Return first found value
    3. Return None if not found in any source

    Common Patterns:
    - Base configuration from file
    - Override from environment (for deployment flexibility)
    - Final override from command line (for one-time changes)

    Test Methods:
    - test_config_manager_new: Verify manager instantiation
    - test_config_manager_add_file_source: Test file source registration
    - test_config_manager_add_environment_source: Test env source registration
    - test_config_manager_get: Test value retrieval with resolution
    """

    def test_config_manager_new(self):
        """Test creating a new config manager.

        This test verifies that DMSCConfigManager can be instantiated.
        The manager is ready to have sources added and values retrieved.
        """
        manager = DMSCConfigManager()
        self.assertIsNotNone(manager)

    def test_config_manager_add_file_source(self):
        """Test adding file source.

        The add_file_source() method registers a configuration file to be
        loaded. Common file formats include YAML, JSON, and TOML. The
        manager will parse the file and merge its values.
        """
        manager = DMSCConfigManager()
        manager.add_file_source("config.yaml")
        self.assertIsNotNone(manager)

    def test_config_manager_add_environment_source(self):
        """Test adding environment source.

        The add_environment_source() method registers environment variables
        as a configuration source. This is useful for container deployments
        and cloud environments where configuration comes from the environment.
        """
        manager = DMSCConfigManager()
        manager.add_environment_source()
        self.assertIsNotNone(manager)

    def test_config_manager_get(self):
        """Test getting config value.

        The get() method resolves a configuration key across all registered
        sources. It returns the value from the highest-priority source that
        contains the key, or None if the key is not found anywhere.
        """
        manager = DMSCConfigManager()
        config = DMSCConfig()
        config.set("test.key", "test_value")
        result = manager.get("test.key")
        self.assertIsNone(result)


if __name__ == "__main__":
    unittest.main()
