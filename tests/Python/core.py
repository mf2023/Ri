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
DMSC Core Module Python Tests.

This module contains comprehensive tests for the DMSC core module Python bindings.
The core module provides fundamental application services including application
lifecycle management, configuration management, logging, and event hooks.

Core Components Tested:
- DMSCAppBuilder: Application builder for initializing the DMSC runtime
- DMSCLogLevel: Enumeration of log severity levels (Debug, Info, Warn, Error)
- DMSCHookBus: Event hook system for module extensibility and lifecycle events
- DMSCFileSystem: Secure file system abstraction for cross-platform operations
- DMSCConfigManager: Configuration manager for multi-source configuration loading

Test Classes:
- CoreAppBuilderTests: Tests for application builder instantiation and configuration
- CoreLogLevelTests: Tests for log level enumeration values and accessibility
- CoreHookBusTests: Tests for hook bus creation and event system functionality
- CoreFileSystemTests: Tests for file system abstraction initialization
- CoreConfigManagerTests: Tests for configuration manager setup

Usage:
    python core.py -v    # Run with verbose output
    python core.py       # Run with default output

Example:
    >>> from dmsc import DMSCAppBuilder
    >>> builder = DMSCAppBuilder()
    >>> print("Builder created successfully")
"""

import unittest
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))

from dmsc import (
    DMSCAppBuilder,
    DMSCLogLevel,
    DMSCHookBus,
    DMSCFileSystem,
    DMSCConfigManager
)


class CoreAppBuilderTests(unittest.TestCase):
    """
    Test suite for DMSCAppBuilder class.

    The DMSCAppBuilder class provides a fluent API for constructing DMSC
    application instances. It allows step-by-step configuration of core
    services including logging, configuration management, and module registration.

    This test class verifies the basic instantiation and creation of the builder
    without requiring a complete application setup.

    Attributes:
        None (all tests are standalone)

    Example:
        >>> builder = DMSCAppBuilder()
        >>> self.assertIsNotNone(builder)
    """

    def test_builder_new(self):
        """Test creating a new application builder instance.

        Verifies that the DMSCAppBuilder can be instantiated successfully.
        The builder is the entry point for creating DMSC applications and
        should always return a valid builder object.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        builder = DMSCAppBuilder()
        self.assertIsNotNone(builder)


class CoreLogLevelTests(unittest.TestCase):
    """
    Test suite for DMSCLogLevel enumeration.

    DMSCLogLevel defines the severity levels for logging throughout the DMSC
    framework. Each log level represents a different priority:

    - Debug (0): Detailed information for debugging
    - Info (1): General operational information
    - Warn (2): Warning conditions that may need attention
    - Error (3): Error conditions and failures

    This test class verifies that all log level values are accessible
    and properly defined in the Python bindings.

    Attributes:
        None (all tests verify enumeration accessibility)

    Example:
        >>> level = DMSCLogLevel.Info
        >>> print(f"Current log level: {level}")
    """

    def test_log_level_debug(self):
        """Test accessibility of Debug log level.

        Debug level (0) is used for detailed debugging information
        that is typically only needed during development.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        self.assertIsNotNone(DMSCLogLevel.Debug)

    def test_log_level_info(self):
        """Test accessibility of Info log level.

        Info level (1) is used for general operational information
        and events that are expected during normal operation.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        self.assertIsNotNone(DMSCLogLevel.Info)

    def test_log_level_warn(self):
        """Test accessibility of Warn log level.

        Warn level (2) is used for warning conditions that may indicate
        potential issues but do not prevent operation.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        self.assertIsNotNone(DMSCLogLevel.Warn)

    def test_log_level_error(self):
        """Test accessibility of Error log level.

        Error level (3) is used for error conditions and failures
        that may require attention but do not necessarily stop the application.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        self.assertIsNotNone(DMSCLogLevel.Error)


class CoreHookBusTests(unittest.TestCase):
    """
    Test suite for DMSCHookBus class.

    The DMSCHookBus class provides an event hook system for DMSC modules.
    Hooks allow modules to register callbacks for specific lifecycle events
    such as startup, shutdown, and custom events.

    This test class verifies the basic creation and instantiation of the
    hook bus system which is fundamental to DMSC's extensibility model.

    Attributes:
        None (all tests verify hook bus functionality)

    Example:
        >>> bus = DMSCHookBus()
        >>> bus.register_hook("startup", callback)
    """

    def test_hook_bus_new(self):
        """Test creating a new hook bus instance.

        Verifies that the DMSCHookBus can be instantiated successfully.
        The hook bus is the central event distribution system for DMSC
        modules to communicate and respond to lifecycle events.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        bus = DMSCHookBus()
        self.assertIsNotNone(bus)


class CoreFileSystemTests(unittest.TestCase):
    """
    Test suite for DMSCFileSystem class.

    The DMSCFileSystem class provides a secure file system abstraction
    for cross-platform file operations. It encapsulates common file
    operations with proper error handling and security considerations.

    This test class verifies that the file system abstraction can be
    initialized with a project root path for secure file operations.

    Attributes:
        project_root (str): Root directory for file operations

    Example:
        >>> fs = DMSCFileSystem(".")
        >>> files = fs.list_dir(".")
    """

    def test_filesystem_new(self):
        """Test creating a new file system instance.

        Verifies that DMSCFileSystem can be instantiated with a project
        root path. The file system abstraction provides secure file
        operations with proper path validation and error handling.

        Args:
            project_root: The root directory for file operations (use "." for current)

        Returns:
            None (uses assertIsNotNone for verification)
        """
        fs = DMSCFileSystem(".")
        self.assertIsNotNone(fs)


class CoreConfigManagerTests(unittest.TestCase):
    """
    Test suite for DMSCConfigManager class.

    The DMSCConfigManager class handles configuration loading and management
    from multiple sources including files, environment variables, and
    command line arguments. It provides a unified interface for accessing
    configuration values throughout the DMSC application.

    This test class verifies the basic instantiation of the configuration
    manager which is essential for application configuration.

    Attributes:
        None (all tests verify configuration manager functionality)

    Example:
        >>> manager = DMSCConfigManager()
        >>> config = manager.load("config.yaml")
    """

    def test_config_manager_new(self):
        """Test creating a new configuration manager instance.

        Verifies that DMSCConfigManager can be instantiated successfully.
        The configuration manager is responsible for loading, parsing,
        and providing access to application configuration from various sources.

        Returns:
            None (uses assertIsNotNone for verification)
        """
        manager = DMSCConfigManager()
        self.assertIsNotNone(manager)


if __name__ == '__main__':
    unittest.main()
