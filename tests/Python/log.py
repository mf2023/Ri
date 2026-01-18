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
DMSC Logging Module Python Tests.

This module contains comprehensive tests for the DMSC logging system Python bindings.
The logging system provides structured logging with configurable outputs, formats,
and rotation policies.

Logging Components:
- DMSCLogger: Main logger instance for logging operations
- DMSCLogConfig: Logging configuration and behavior
- DMSCLogLevel: Log severity levels
- DMSCFileSystem: File system abstraction for log storage

Log Levels (increasing severity):
- Debug: Detailed information for debugging
- Info: General informational messages
- Warn: Warning conditions
- Error: Error conditions

Logging Features:
- Structured logging with key-value pairs
- Multiple output destinations (console, file)
- Log rotation based on size or time
- Configurable sampling rates
- JSON formatting for machine parsing
- Context-aware logging with request IDs

Configuration Options:
- Level: Minimum severity to log
- Console output: Enable/disable stdout/stderr
- File output: Enable/disable file logging
- File name: Target log file path
- Max bytes: Size limit before rotation
- JSON format: Structured vs plain text
- Sampling: Percentage of logs to record

File Rotation:
- Size-based: Rotate when file exceeds max_bytes
- Time-based: Rotate based on time intervals
- Retention: Number of old logs to keep

Test Classes:
- TestDMSCLogLevel: Log level enumeration
- TestDMSCLogConfig: Logging configuration
- TestDMSCLogger: Logger instantiation
"""

import unittest
from dmsc import DMSCLogger, DMSCLogConfig, DMSCLogLevel, DMSCFileSystem


class TestDMSCLogLevel(unittest.TestCase):
    """
    Test suite for DMSCLogLevel enum.

    The DMSCLogLevel enum defines log severity levels that control which
    messages are logged. Each level represents a different severity of
    information.

    Level Hierarchy:
    - Debug: Most verbose, development only
    - Info: General operational messages
    - Warn: Potential issues requiring attention
    - Error: Actual problems affecting operation

    Filtering:
    When a minimum level is set, only messages at or above that level
    are logged. For example, setting level to Warn excludes Debug and Info.

    Test Methods:
    - test_log_level_values: Verify all log levels exist
    """

    def test_log_level_values(self):
        """Test log level enum values exist.

        All standard log levels should be available for configuring
        logging verbosity.
        """
        self.assertEqual(str(DMSCLogLevel.Debug), "DMSCLogLevel.Debug")
        self.assertEqual(str(DMSCLogLevel.Info), "DMSCLogLevel.Info")
        self.assertEqual(str(DMSCLogLevel.Warn), "DMSCLogLevel.Warn")
        self.assertEqual(str(DMSCLogLevel.Error), "DMSCLogLevel.Error")


class TestDMSCLogConfig(unittest.TestCase):
    """
    Test suite for DMSCLogConfig class.

    The DMSCLogConfig class configures logging behavior including output
    destinations, formats, and rotation policies. It provides both getter
    and setter methods for all configuration options.

    Configuration Properties:
    - level: Minimum log severity
    - console_enabled: Enable console output
    - file_enabled: Enable file output
    - file_name: Target log file path
    - json_format: Use JSON vs plain text
    - sampling_default: Percentage of logs to record
    - rotate_when: Trigger for rotation (size, time)
    - max_bytes: Size limit before rotation

    Default Values:
    - Level: Info
    - Console: Enabled
    - File: Enabled
    - File name: "dms.log"
    - Format: Plain text
    - Sampling: 100% (1.0)
    - Rotation: Size-based
    - Max size: 10 MB

    Test Methods:
    - test_log_config_default: Verify default configuration
    - test_log_config_setters: Test configuration updates
    - test_log_config_set_level_invalid: Test invalid level rejection
    - test_log_config_set_sampling_invalid: Test invalid sampling rejection
    - test_log_config_set_sampling_negative: Test negative sampling rejection
    """

    def test_log_config_default(self):
        """Test default log configuration.

        Default logging configuration provides sensible defaults:
        - Info level (logs Info, Warn, Error)
        - Console logging enabled
        - File logging enabled
        - Default file name "dms.log"
        - Plain text format (not JSON)
        - Full sampling (100% of logs)
        - Size-based rotation at 10 MB
        """
        config = DMSCLogConfig()
        self.assertEqual(config.get_level(), "info")
        self.assertTrue(config.get_console_enabled())
        self.assertTrue(config.get_file_enabled())
        self.assertEqual(config.get_sampling_default(), 1.0)
        self.assertEqual(config.get_file_name(), "dms.log")
        self.assertFalse(config.get_json_format())
        self.assertEqual(config.get_rotate_when(), "size")
        self.assertEqual(config.get_max_bytes(), 10 * 1024 * 1024)

    def test_log_config_setters(self):
        """Test log configuration setters.

        Configuration properties can be updated:
        - Set level to debug for verbose output
        - Disable console for production
        - Enable file logging
        - Custom file name for multi-service
        - JSON format for log aggregation
        - 50% sampling for high-volume services
        """
        config = DMSCLogConfig()
        config.set_level("debug")
        config.set_console_enabled(False)
        config.set_file_enabled(True)
        config.set_file_name("app.log")
        config.set_json_format(True)
        config.set_sampling_default(0.5)
        
        self.assertEqual(config.get_level(), "debug")
        self.assertFalse(config.get_console_enabled())
        self.assertTrue(config.get_file_enabled())
        self.assertEqual(config.get_file_name(), "app.log")
        self.assertTrue(config.get_json_format())
        self.assertEqual(config.get_sampling_default(), 0.5)

    def test_log_config_set_level_invalid(self):
        """Test setting invalid log level.

        Invalid log levels should be rejected to prevent
        configuration errors.
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_level("invalid")

    def test_log_config_set_sampling_invalid(self):
        """Test setting invalid sampling rate.

        Sampling rates must be between 0 and 1. Values
        outside this range are invalid.
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_sampling_default(1.5)

    def test_log_config_set_sampling_negative(self):
        """Test setting negative sampling rate.

        Negative sampling rates are invalid and should
        be rejected.
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_sampling_default(-0.1)


class TestDMSCLogger(unittest.TestCase):
    """
    Test suite for DMSCLogger class.

    The DMSCLogger class provides the main logging interface for
    recording log messages with various severity levels. It is
    configured with a DMSCLogConfig and uses DMSCFileSystem for
    file operations.

    Logging Methods:
    - debug(): Log debug severity messages
    - info(): Log informational messages
    - warn(): Log warning messages
    - error(): Log error messages

    Structured Logging:
    Each log method accepts format strings with arguments
    and optional key-value pairs for structured data.

    Lifecycle:
    - Created with configuration and filesystem
    - Used throughout application lifecycle
    - Cleaned up on shutdown

    Test Methods:
    - test_logger_new: Verify logger instantiation
    """

    def test_logger_new(self):
        """Test creating a new logger.

        A logger is created with configuration and filesystem
        access, ready to record log messages.
        """
        config = DMSCLogConfig()
        fs = DMSCFileSystem(".")
        logger = DMSCLogger(config, fs)
        self.assertIsNotNone(logger)


if __name__ == "__main__":
    unittest.main()
