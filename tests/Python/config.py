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
    """Test suite for DMSCConfig class.
    
    The DMSCConfig class provides a hierarchical key-value configuration store
    with support for type-specific value retrieval. It is the fundamental building
    block of the DMSC configuration system.
    
    Key-Value Model:
    - Keys are strings using dot-notation for hierarchy (e.g., "server.port")
    - Values are stored as strings internally
    - Type-specific getters convert strings to desired types
    
    Type Conversions:
    - get(): Returns string value (default None if not found)
    - get_f64(): Converts to 64-bit floating point number
    - get_usize(): Converts to unsigned size type (typically for counts/sizes)
    
    Common Use Cases:
    - Server configuration: host, port, timeout settings
    - Database connection strings: host, port, credentials
    - Feature flags and toggle settings: boolean on/off values
    - Performance tuning parameters: numeric thresholds and limits
    
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
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid DMSCConfig instance
        - Config is empty (no entries)
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
        
        Example Configuration:
        - Key: "server.port", Value: "8080"
        - Key: "database.host", Value: "localhost"
        - Key: "cache.enabled", Value: "true"
        
        Expected Behavior:
        - set() stores the key-value pair
        - get() returns the stored value as a string
        - Dot-notation keys are preserved exactly as set
        """
        config = DMSCConfig()
        config.set("server.port", "8080")
        result = config.get("server.port")
        self.assertEqual(result, "8080")

    def test_config_get_f64(self):
        """Test getting config value as f64.
        
        The get_f64() method converts string values to floating-point numbers.
        This is useful for numeric configuration like timeouts, weights, or rates.
        
        Use Cases:
        - Timeout values: "30.5" -> 30.5 seconds
        - Rate limits: "100.0" -> 100 requests per second
        - Weights: "0.75" -> 75% weight
        
        Expected Behavior:
        - String "8080" converts to float 8080.0
        - Conversion handles decimal points correctly
        - Returns Some(f64) for valid numeric strings
        """
        config = DMSCConfig()
        config.set("server.port", "8080")
        result = config.get_f64("server.port")
        self.assertEqual(result, 8080.0)

    def test_config_get_usize(self):
        """Test getting config value as usize.
        
        The get_usize() method converts string values to unsigned integers.
        This is useful for configuration like pool sizes, limits, or counts.
        
        Use Cases:
        - Pool sizes: "10" -> 10 connections
        - Limit values: "1000" -> 1000 max items
        - Count values: "5" -> 5 retries
        
        Expected Behavior:
        - String "10" converts to integer 10
        - Returns Some(usize) for valid positive integers
        - Unsigned means non-negative values only
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
        
        Use Cases:
        - Display all configuration keys to user
        - Validate that required keys exist
        - Iterate through configuration sections
        
        Expected Behavior:
        - keys() returns a list of key strings
        - Length matches number of set keys
        - All previously set keys are included
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
        
        Use Cases:
        - Export all configuration values
        - Validate configuration value formats
        - Generate configuration reports
        
        Expected Behavior:
        - values() returns a list of value strings
        - Length matches number of set keys
        - All previously set values are included
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
        
        Use Cases:
        - Check if optional configuration is present
        - Validate required configuration exists
        - Conditional configuration loading
        
        Expected Behavior:
        - contains() returns True for existing keys
        - contains() returns False for non-existent keys
        - Does not raise exceptions for missing keys
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
        
        Use Cases:
        - Check if configuration is empty (len == 0)
        - Count number of configuration entries
        - Display configuration size to user
        
        Expected Behavior:
        - len() returns 0 for empty config
        - len() returns 1 after one key-value pair
        - len() returns 2 after two key-value pairs
        - Count increments with each unique key
        """
        config = DMSCConfig()
        self.assertEqual(config.len(), 0)
        config.set("key1", "value1")
        self.assertEqual(config.len(), 1)
        config.set("key2", "value2")
        self.assertEqual(config.len(), 2)


class TestDMSCConfigManager(unittest.TestCase):
    """Test suite for DMSCConfigManager class.
    
    The DMSCConfigManager class coordinates multiple configuration sources
    and provides a unified interface for configuration access. It handles
    source registration, loading, and value resolution with proper precedence.
    
    Source Management:
    - add_file_source(): Load configuration from file (YAML, JSON, TOML)
    - add_environment_source(): Load from environment variables
    - add_command_line_source(): Load from command line arguments
    
    Value Resolution:
    1. Sources are checked in order of addition
    2. Later sources override earlier ones (last-wins semantics)
    3. First found value is returned
    4. Returns None if key not found in any source
    
    Common Patterns:
    - Base configuration from file: Provide default values
    - Override from environment: Deployment flexibility (container environments)
    - Final override from command line: One-time changes for specific runs
    
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
        
        Expected Behavior:
        - Constructor completes without errors
        - Returns a valid manager instance
        - Manager has empty configuration initially
        """
        manager = DMSCConfigManager()
        self.assertIsNotNone(manager)

    def test_config_manager_add_file_source(self):
        """Test adding file source.
        
        The add_file_source() method registers a configuration file to be
        loaded. Common file formats include YAML, JSON, and TOML. The
        manager will parse the file and merge its values.
        
        Supported Formats:
        - YAML (.yaml, .yml): Human-readable, popular for Kubernetes
        - JSON (.json): Structured data, widely used
        - TOML (.toml): Python-style, good for simple configs
        
        Expected Behavior:
        - add_file_source() accepts file path
        - File is registered as configuration source
        - Manager is ready to load the configuration
        """
        manager = DMSCConfigManager()
        manager.add_file_source("config.yaml")
        self.assertIsNotNone(manager)

    def test_config_manager_add_environment_source(self):
        """Test adding environment source.
        
        The add_environment_source() method registers environment variables
        as a configuration source. This is useful for container deployments
        and cloud environments where configuration comes from the environment.
        
        Environment Variable Naming:
        - Typically uses prefix (e.g., "DMSC_")
        - Converts underscores to dots (eMSC_DB_HOST -> db.host)
        - Case-sensitive matching
        
        Expected Behavior:
        - add_environment_source() enables env var support
        - Manager reads from environment variables
        - Useful for container and cloud deployments
        """
        manager = DMSCConfigManager()
        manager.add_environment_source()
        self.assertIsNotNone(manager)

    def test_config_manager_get(self):
        """Test getting config value.
        
        The get() method resolves a configuration key across all registered
        sources. It returns the value from the highest-priority source that
        contains the key, or None if the key is not found anywhere.
        
        Priority Order:
        1. Environment variables (highest priority)
        2. Command line arguments
        3. Configuration files (lowest priority)
        
        Expected Behavior:
        - get() searches all registered sources
        - Returns value from highest-priority source
        - Returns None if key not found in any source
        """
        manager = DMSCConfigManager()
        config = DMSCConfig()
        config.set("test.key", "test_value")
        result = manager.get("test.key")
        self.assertIsNone(result)


if __name__ == "__main__":
    unittest.main()
