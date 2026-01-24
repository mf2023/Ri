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
    """Test suite for DMSCLogLevel enum.
    
    The DMSCLogLevel enum defines log severity levels that control which
    messages are logged. Each level represents a different severity of
    information. Log levels are fundamental for filtering log output
    and focusing on relevant information.
    
    Level Hierarchy (from least to most severe):
    - DEBUG (10): Most verbose, detailed debugging information
      Use: Print variable values, function calls, execution paths
      Example: "Processing item 42 of 100"
    - INFO (20): General informational messages
      Use: Track normal operations and milestones
      Example: "Server started on port 8080"
    - WARN (30): Warning conditions
      Use: Indicate potential issues that don't prevent operation
      Example: "Cache miss, fetching from database"
    - ERROR (40): Error conditions
      Use: Report failures that affect functionality
      Example: "Failed to connect to database"
    
    Level Comparison:
    - Lower numbers = less severe = more verbose
    - Setting level to INFO excludes DEBUG messages
    - Setting level to WARN excludes DEBUG and INFO
    - Setting level to ERROR excludes DEBUG, INFO, and WARN
    
    Filtering Behavior:
    When a minimum level is set, only messages at or above that level
    are logged. Messages below the threshold are discarded silently.
    This filtering happens before log record construction for efficiency.
    
    Common Configurations:
    - Development: DEBUG level for maximum visibility
    - Production: INFO or WARN for operational monitoring
    - Error Investigation: DEBUG temporarily for specific issues
    
    Test Methods:
    - test_log_level_values: Verify all log levels exist
    """

    def test_log_level_values(self):
        """Test log level enum values exist.
        
        Each standard log level should have a string representation
        for configuration, logging, and display purposes.
        
        Expected Behavior:
        - Debug level string matches expected format
        - Info level string matches expected format
        - Warn level string matches expected format
        - Error level string matches expected format
        """
        self.assertEqual(str(DMSCLogLevel.Debug), "DMSCLogLevel.Debug")
        self.assertEqual(str(DMSCLogLevel.Info), "DMSCLogLevel.Info")
        self.assertEqual(str(DMSCLogLevel.Warn), "DMSCLogLevel.Warn")
        self.assertEqual(str(DMSCLogLevel.Error), "DMSCLogLevel.Error")


class TestDMSCLogConfig(unittest.TestCase):
    """Test suite for DMSCLogConfig class.
    
    The DMSCLogConfig class configures logging behavior including output
    destinations, formats, and rotation policies. It provides both getter
    and setter methods for all configuration options, allowing runtime
    configuration updates.
    
    Configuration Properties:
    - level: Minimum log severity (debug, info, warn, error)
    - console_enabled: Enable/disable stdout/stderr output
    - file_enabled: Enable/disable file logging
    - file_name: Target log file path (e.g., "/var/log/dms.log")
    - json_format: Use structured JSON vs plain text format
    - sampling_default: Percentage of logs to record (0.0 to 1.0)
    - rotate_when: Trigger for rotation (size, time, none)
    - max_bytes: Size limit before rotation (bytes)
    - max_files: Number of rotated files to keep
    
    Default Configuration Values:
    - Level: "info" (logs Info, Warn, Error)
    - Console: Enabled (true)
    - File: Enabled (true)
    - File name: "dms.log"
    - JSON format: Disabled (plain text)
    - Sampling: 1.0 (100% of logs recorded)
    - Rotate when: "size" (rotate when file exceeds max_bytes)
    - Max bytes: 10 MB (10 * 1024 * 1024)
    
    JSON Format Benefits:
    - Machine readable for log aggregation systems
    - Easy parsing with tools like jq, grep, etc.
    - Compatible with ELK stack, Splunk, etc.
    - Structured fields for filtering and analysis
    
    Sampling Use Cases:
    - High-volume services: Sample 10% of logs (0.1)
    - Debug sessions: Sample 100% temporarily (1.0)
    - Error tracing: Sample errors at 100%, others at 10%
    
    File Rotation Strategies:
    - Size-based: Rotate when file exceeds max_bytes
    - Time-based: Rotate at interval (daily, hourly)
    - Both: Rotate on either condition
    - None: Single file, unlimited growth
    
    Test Methods:
    - test_log_config_default: Verify default configuration
    - test_log_config_setters: Test configuration updates
    - test_log_config_set_level_invalid: Test invalid level rejection
    - test_log_config_set_sampling_invalid: Test invalid sampling rejection
    - test_log_config_set_sampling_negative: Test negative sampling rejection
    """

    def test_log_config_default(self):
        """Test default log configuration.
        
        Default logging configuration provides sensible defaults suitable
        for most development and production environments:
        - Info level (logs Info, Warn, Error, excludes Debug)
        - Console logging enabled (output to stdout/stderr)
        - File logging enabled (output to file)
        - Default file name "dms.log" in current directory
        - Plain text format (human-readable, not JSON)
        - Full sampling (100% of logs recorded)
        - Size-based rotation at 10 MB
        - Unlimited rotated file retention
        
        Expected Behavior:
        - get_level() returns "info"
        - get_console_enabled() returns True
        - get_file_enabled() returns True
        - get_sampling_default() returns 1.0
        - get_file_name() returns "dms.log"
        - get_json_format() returns False
        - get_rotate_when() returns "size"
        - get_max_bytes() returns 10,485,760 (10 MB)
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
        
        Configuration properties can be updated to customize logging
        behavior for specific environments and requirements:
        - Debug level for detailed troubleshooting
        - Disable console for containerized deployments
        - Enable file logging for persistence
        - Custom file name for multi-service environments
        - JSON format for log aggregation systems
        - 50% sampling to reduce log volume
        
        Expected Behavior:
        - set_level() updates the log level
        - set_console_enabled() toggles console output
        - set_file_enabled() toggles file output
        - set_file_name() changes the log file path
        - set_json_format() switches format
        - set_sampling_default() changes sampling rate
        - Getters return updated values
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
        
        Invalid log levels should be rejected with an exception
        to prevent configuration errors and ensure logging works correctly.
        
        Expected Behavior:
        - set_level() with invalid value raises exception
        - Invalid level is not accepted
        - Config remains unchanged or throws before modification
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_level("invalid")

    def test_log_config_set_sampling_invalid(self):
        """Test setting invalid sampling rate.
        
        Sampling rates must be between 0 and 1 (inclusive).
        Values outside this range are invalid and should be rejected.
        
        Expected Behavior:
        - set_sampling_default() with value > 1.0 raises exception
        - Invalid sampling rate is not accepted
        - Config remains unchanged
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_sampling_default(1.5)

    def test_log_config_set_sampling_negative(self):
        """Test setting negative sampling rate.
        
        Negative sampling rates are invalid and should be rejected.
        Sampling rate must be non-negative.
        
        Expected Behavior:
        - set_sampling_default() with negative value raises exception
        - Negative sampling rate is not accepted
        - Config remains unchanged
        """
        config = DMSCLogConfig()
        with self.assertRaises(Exception):
            config.set_sampling_default(-0.1)


class TestDMSCLogger(unittest.TestCase):
    """Test suite for DMSCLogger class.
    
    The DMSCLogger class provides the main logging interface for
    recording log messages with various severity levels. It is
    configured with a DMSCLogConfig and uses DMSCFileSystem for
    file operations. The logger is thread-safe and designed for
    use in concurrent applications.
    
    Logging Methods:
    - debug(format, **kwargs): Log debug severity messages
    - info(format, **kwargs): Log informational messages
    - warn(format, **kwargs): Log warning messages
    - error(format, **kwargs): Log error messages
    
    Method Signatures:
    Each logging method accepts:
    - format: Format string using {} or {:,} placeholders
    - **kwargs: Key-value pairs for structured logging
    
    Structured Logging:
    Each log method accepts optional key-value pairs that are
    included in the log output as structured data. This is
    especially useful when using JSON format.
    
    Example Usage:
    ```python
    logger.info("User logged in", user_id=123, ip="192.168.1.1")
    logger.error("Connection failed", host="db.example.com", error=str(e))
    ```
    
    Lifecycle Management:
    - Created with configuration and filesystem
    - Used throughout application lifecycle
    - Cleaned up on shutdown to flush buffers and close files
    
    Thread Safety:
    The logger is designed to be used from multiple threads
    concurrently. Internal locking ensures log records are
    not interleaved.
    
    Performance Considerations:
    - Log messages below configured level are discarded efficiently
    - Structured fields are only processed if log will be written
    - File I/O may block; consider async logging for high throughput
    
    Test Methods:
    - test_logger_new: Verify logger instantiation
    """

    def test_logger_new(self):
        """Test creating a new logger.
        
        This test verifies that DMSCLogger can be instantiated with
        configuration and filesystem access. The logger is ready
        to record log messages.
        
        Expected Behavior:
        - Constructor accepts config and filesystem parameters
        - Returns a valid logger instance
        - Logger is ready to record log messages
        - Logger is configured according to config
        """
        config = DMSCLogConfig()
        fs = DMSCFileSystem(".")
        logger = DMSCLogger(config, fs)
        self.assertIsNotNone(logger)


if __name__ == "__main__":
    unittest.main()
