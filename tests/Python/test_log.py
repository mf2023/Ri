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
DMSC Log Module Tests

Tests for the logging functionality.
"""

import pytest
from dmsc import (
    DMSCLogger,
    DMSCLogConfig,
    DMSCLogLevel,
)


class TestDMSCLogger:
    """Tests for DMSCLogger"""

    def test_logger_creation(self):
        """Test creating logger"""
        log_config = DMSCLogConfig()
        log_config.level = DMSCLogLevel.Info

        logger = DMSCLogger(log_config)
        assert logger is not None

    def test_logger_levels(self):
        """Test logger with different levels"""
        log_config = DMSCLogConfig()
        logger = DMSCLogger(log_config)

        # Test that logger methods exist and can be called
        logger.info("test", "Info message")
        logger.warn("test", "Warning message")
        logger.error("test", "Error message")


class TestDMSCLogConfig:
    """Tests for DMSCLogConfig"""

    def test_log_config_creation(self):
        """Test creating log configuration"""
        config = DMSCLogConfig()
        config.level = DMSCLogLevel.Debug
        config.json_format = True
        config.console_enabled = True
        config.file_enabled = False

        assert config.level == DMSCLogLevel.Debug
        assert config.json_format is True
        assert config.console_enabled is True
        assert config.file_enabled is False


class TestDMSCLogLevel:
    """Tests for DMSCLogLevel"""

    def test_log_levels(self):
        """Test log level enum values"""
        assert DMSCLogLevel.Debug is not None
        assert DMSCLogLevel.Info is not None
        assert DMSCLogLevel.Warn is not None
        assert DMSCLogLevel.Error is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
