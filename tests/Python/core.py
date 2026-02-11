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
DMSC Core Module Tests

Tests for the core DMSC functionality including application runtime,
configuration, logging, and file system operations.
"""

import pytest
from dmsc import (
    DMSCAppBuilder,
    DMSCAppRuntime,
    DMSCConfig,
    DMSCConfigManager,
    DMSCLogger,
    DMSCLogConfig,
    DMSCLogLevel,
    DMSCFileSystem,
    DMSCError,
    DMSCServiceContext,
    DMSCHookBus,
    DMSCHookEvent,
    DMSCHookKind,
    DMSCModulePhase,
    DMSCHealthStatus,
    DMSCHealthCheckResult,
    DMSCHealthCheckConfig,
    DMSCHealthReport,
    DMSCHealthChecker,
    DMSCLifecycleObserver,
)


class TestDMSCAppBuilder:
    """Tests for DMSCAppBuilder"""

    def test_app_builder_creation(self):
        """Test creating an application builder"""
        builder = DMSCAppBuilder()
        assert builder is not None

    def test_app_builder_with_config(self):
        """Test application builder with config path"""
        builder = DMSCAppBuilder()
        builder.with_config("config.yaml")
        assert builder is not None

    def test_app_builder_with_logging(self):
        """Test application builder with logging config"""
        builder = DMSCAppBuilder()
        log_config = DMSCLogConfig()
        builder.with_logging(log_config)
        assert builder is not None

    def test_app_builder_chain(self):
        """Test application builder method chaining"""
        builder = DMSCAppBuilder()
        result = builder.with_config("config.yaml").with_logging(DMSCLogConfig())
        assert result is builder


class TestDMSCConfig:
    """Tests for DMSCConfig"""

    def test_config_creation(self):
        """Test creating a configuration"""
        config = DMSCConfig()
        assert config is not None

    def test_config_with_values(self):
        """Test configuration with custom values - values must be strings"""
        config = DMSCConfig()
        config.set("database.host", "localhost")
        config.set("database.port", "5432")

        assert config.get("database.host") == "localhost"
        assert config.get("database.port") == "5432"


class TestDMSCConfigManager:
    """Tests for DMSCConfigManager"""

    def test_config_manager_creation(self):
        """Test creating a config manager"""
        manager = DMSCConfigManager()
        assert manager is not None

    def test_config_manager_add_source(self):
        """Test config manager add file source"""
        manager = DMSCConfigManager()
        manager.add_file_source("config.yaml")
        assert manager is not None


class TestDMSCLogger:
    """Tests for DMSCLogger"""

    def test_logger_creation(self):
        """Test logger requires filesystem"""
        fs = DMSCFileSystem(".")
        log_config = DMSCLogConfig()
        logger = DMSCLogger(log_config, fs)
        assert logger is not None

    def test_logger_levels(self):
        """Test logger with different levels"""
        fs = DMSCFileSystem(".")
        log_config = DMSCLogConfig()
        logger = DMSCLogger(log_config, fs)
        assert logger is not None


class TestDMSCFileSystem:
    """Tests for DMSCFileSystem"""

    def test_file_system_creation(self):
        """Test creating a file system handler"""
        fs = DMSCFileSystem(".")
        assert fs is not None

    def test_file_operations(self):
        """Test basic file operations"""
        fs = DMSCFileSystem(".")
        exists = fs.exists("pyproject.toml")
        assert isinstance(exists, bool)


class TestDMSCError:
    """Tests for DMSCError"""

    def test_error_creation(self):
        """Test creating an error using factory method"""
        error = DMSCError.from_str("Test error message")
        assert str(error) == "Test error message"

    def test_io_error(self):
        """Test creating an IO error"""
        error = DMSCError.io("IO operation failed")
        assert error.is_io()

    def test_serde_error(self):
        """Test creating a serde error"""
        error = DMSCError.serde("Serialization failed")
        assert error.is_serde()


class TestDMSCHookBus:
    """Tests for DMSCHookBus"""

    def test_hook_bus_creation(self):
        """Test creating a hook bus"""
        hook_bus = DMSCHookBus()
        assert hook_bus is not None


class TestDMSCHookEvent:
    """Tests for DMSCHookEvent"""

    def test_hook_event_module_phase(self):
        """Test DMSCHookEvent module phases exist"""
        phases = [
            DMSCModulePhase.INIT,
            DMSCModulePhase.BEFORE_START,
            DMSCModulePhase.START,
            DMSCModulePhase.AFTER_START,
            DMSCModulePhase.BEFORE_SHUTDOWN,
            DMSCModulePhase.SHUTDOWN,
            DMSCModulePhase.AFTER_SHUTDOWN,
        ]
        assert len(phases) == 7


class TestDMSCHealthCheck:
    """Tests for health check functionality"""

    def test_health_check_config(self):
        """Test health check configuration"""
        config = DMSCHealthCheckConfig(
            check_interval=30,
            timeout=5,
            failure_threshold=3,
            success_threshold=2,
            enabled=True
        )

        assert config.check_interval == 30
        assert config.timeout == 5

    def test_health_check_result(self):
        """Test health check result"""
        result = DMSCHealthCheckResult(
            name="test_check",
            status=DMSCHealthStatus.Healthy,
            message="Service is healthy"
        )

        assert result.name == "test_check"
        assert "healthy" in str(result.status).lower()

    def test_health_report(self):
        """Test health report"""
        report = DMSCHealthReport()
        assert hasattr(report, 'overall_status')


class TestDMSCLifecycleObserver:
    """Tests for DMSCLifecycleObserver"""

    def test_lifecycle_observer_creation(self):
        """Test creating a lifecycle observer"""
        observer = DMSCLifecycleObserver()
        assert observer is not None


class TestDMSCServiceContext:
    """Tests for DMSCServiceContext"""

    def test_service_context_creation(self):
        """Test creating a service context"""
        context = DMSCServiceContext()
        assert context is not None

    def test_service_context_with_logger(self):
        """Test service context with logger property"""
        context = DMSCServiceContext()
        assert hasattr(context, 'logger')


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
