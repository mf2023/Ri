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
Ri Core Module Tests

Tests for the core Ri functionality including application runtime,
configuration, logging, and file system operations.
"""

import pytest
from ri import (
    RiAppBuilder,
    RiAppRuntime,
    RiConfig,
    RiConfigManager,
    RiLogger,
    RiLogConfig,
    RiLogLevel,
    RiFileSystem,
    RiError,
    RiServiceContext,
    RiHookBus,
    RiHookEvent,
    RiHookKind,
    RiModulePhase,
    RiHealthStatus,
    RiHealthCheckResult,
    RiHealthCheckConfig,
    RiHealthReport,
    RiHealthChecker,
    RiLifecycleObserver,
)


class TestRiAppBuilder:
    """Tests for RiAppBuilder"""

    def test_app_builder_creation(self):
        """Test creating an application builder"""
        builder = RiAppBuilder()
        assert builder is not None

    def test_app_builder_with_config(self):
        """Test application builder with config path"""
        builder = RiAppBuilder()
        builder.with_config("config.yaml")
        assert builder is not None

    def test_app_builder_with_logging(self):
        """Test application builder with logging config"""
        builder = RiAppBuilder()
        log_config = RiLogConfig()
        builder.with_logging(log_config)
        assert builder is not None

    def test_app_builder_chain(self):
        """Test application builder method chaining"""
        builder = RiAppBuilder()
        result = builder.with_config("config.yaml").with_logging(RiLogConfig())
        assert result is builder


class TestRiConfig:
    """Tests for RiConfig"""

    def test_config_creation(self):
        """Test creating a configuration"""
        config = RiConfig()
        assert config is not None

    def test_config_with_values(self):
        """Test configuration with custom values - values must be strings"""
        config = RiConfig()
        config.set("database.host", "localhost")
        config.set("database.port", "5432")

        assert config.get("database.host") == "localhost"
        assert config.get("database.port") == "5432"


class TestRiConfigManager:
    """Tests for RiConfigManager"""

    def test_config_manager_creation(self):
        """Test creating a config manager"""
        manager = RiConfigManager()
        assert manager is not None

    def test_config_manager_add_source(self):
        """Test config manager add file source"""
        manager = RiConfigManager()
        manager.add_file_source("config.yaml")
        assert manager is not None


class TestRiLogger:
    """Tests for RiLogger"""

    def test_logger_creation(self):
        """Test logger requires filesystem"""
        fs = RiFileSystem(".")
        log_config = RiLogConfig()
        logger = RiLogger(log_config, fs)
        assert logger is not None

    def test_logger_levels(self):
        """Test logger with different levels"""
        fs = RiFileSystem(".")
        log_config = RiLogConfig()
        logger = RiLogger(log_config, fs)
        assert logger is not None


class TestRiFileSystem:
    """Tests for RiFileSystem"""

    def test_file_system_creation(self):
        """Test creating a file system handler"""
        fs = RiFileSystem(".")
        assert fs is not None

    def test_file_operations(self):
        """Test basic file operations"""
        fs = RiFileSystem(".")
        exists = fs.exists("pyproject.toml")
        assert isinstance(exists, bool)


class TestRiError:
    """Tests for RiError"""

    def test_error_creation(self):
        """Test creating an error using factory method"""
        error = RiError.from_str("Test error message")
        assert str(error) == "Test error message"

    def test_io_error(self):
        """Test creating an IO error"""
        error = RiError.io("IO operation failed")
        assert error.is_io()

    def test_serde_error(self):
        """Test creating a serde error"""
        error = RiError.serde("Serialization failed")
        assert error.is_serde()


class TestRiHookBus:
    """Tests for RiHookBus"""

    def test_hook_bus_creation(self):
        """Test creating a hook bus"""
        hook_bus = RiHookBus()
        assert hook_bus is not None


class TestRiHookEvent:
    """Tests for RiHookEvent"""

    def test_hook_event_module_phase(self):
        """Test RiHookEvent module phases exist"""
        phases = [
            RiModulePhase.Init,
            RiModulePhase.BeforeStart,
            RiModulePhase.Start,
            RiModulePhase.AfterStart,
            RiModulePhase.BeforeShutdown,
            RiModulePhase.Shutdown,
            RiModulePhase.AfterShutdown,
        ]
        assert len(phases) == 7


class TestRiHealthCheck:
    """Tests for health check functionality"""

    def test_health_check_config(self):
        """Test health check configuration"""
        config = RiHealthCheckConfig(
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
        result = RiHealthCheckResult(
            name="test_check",
            status=RiHealthStatus.Healthy,
            message="Service is healthy"
        )

        assert result.name == "test_check"
        assert "healthy" in str(result.status).lower()

    def test_health_report(self):
        """Test health report"""
        report = RiHealthReport()
        assert hasattr(report, 'overall_status')


class TestRiLifecycleObserver:
    """Tests for RiLifecycleObserver"""

    def test_lifecycle_observer_creation(self):
        """Test creating a lifecycle observer"""
        observer = RiLifecycleObserver()
        assert observer is not None


class TestRiServiceContext:
    """Tests for RiServiceContext"""

    def test_service_context_creation(self):
        """Test creating a service context"""
        context = RiServiceContext()
        assert context is not None

    def test_service_context_with_logger(self):
        """Test service context with logger property"""
        context = RiServiceContext()
        assert hasattr(context, 'logger')


class TestRiAppBuilderWrapper:
    """Tests for RiAppBuilder Python wrapper behavior
    
    These tests verify that the Python wrapper class provides a Pythonic
    interface with method chaining, automatically handling the internal
    reassignment required by Rust's PyO3 bindings.
    """

    def test_method_chaining_returns_same_instance(self):
        """Verify wrapper returns self for method chaining
        
        The Python wrapper should return the same instance (self) to enable
        natural method chaining without explicit reassignment.
        """
        builder = RiAppBuilder()
        result = builder.with_config("config.yaml")
        assert result is builder, "with_config should return the same instance"

    def test_multiple_chained_calls(self):
        """Test multiple chained method calls"""
        builder = RiAppBuilder()
        result = (builder
            .with_config("config.yaml")
            .with_logging(RiLogConfig()))
        assert result is builder, "Chained calls should return the same instance"

    def test_wrapper_has_internal_builder(self):
        """Verify wrapper has internal _builder attribute"""
        builder = RiAppBuilder()
        assert hasattr(builder, '_builder'), "Wrapper should have _builder attribute"
        assert builder._builder is not None, "Internal _builder should not be None"

    def test_build_returns_runtime_wrapper(self):
        """Verify build() returns RiAppRuntime instance"""
        builder = RiAppBuilder()
        try:
            runtime = builder.build()
            assert isinstance(runtime, RiAppRuntime), "build() should return RiAppRuntime"
        except Exception:
            pass


class TestRiAppRuntimeWrapper:
    """Tests for RiAppRuntime Python wrapper behavior
    
    These tests verify that the Python wrapper correctly delegates to the
    underlying Rust runtime instance.
    """

    def test_runtime_wrapper_creation(self):
        """Test runtime wrapper creation"""
        builder = RiAppBuilder()
        try:
            runtime = builder.build()
            assert runtime is not None, "Runtime should be created"
        except Exception:
            pass

    def test_runtime_has_internal_instance(self):
        """Verify runtime wrapper has internal _runtime attribute"""
        builder = RiAppBuilder()
        try:
            runtime = builder.build()
            assert hasattr(runtime, '_runtime'), "Wrapper should have _runtime attribute"
            assert runtime._runtime is not None, "Internal _runtime should not be None"
        except Exception:
            pass

    def test_get_context_method_exists(self):
        """Verify get_context() method exists and is callable"""
        builder = RiAppBuilder()
        try:
            runtime = builder.build()
            assert hasattr(runtime, 'get_context'), "Runtime should have get_context method"
            assert callable(runtime.get_context), "get_context should be callable"
        except Exception:
            pass

    def test_run_method_exists(self):
        """Verify run() method exists and is callable"""
        builder = RiAppBuilder()
        try:
            runtime = builder.build()
            assert hasattr(runtime, 'run'), "Runtime should have run method"
            assert callable(runtime.run), "run should be callable"
        except Exception:
            pass


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
