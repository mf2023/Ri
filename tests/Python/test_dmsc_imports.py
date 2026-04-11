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
Ri Module Import Tests

Tests that verify all documented Ri types can be imported from Python.
This ensures Python bindings are correctly exposing Rust types.
"""

import pytest


class TestCoreImports:
    """Test core module imports"""

    def test_app_builder_import(self):
        """Test RiAppBuilder can be imported"""
        from ri import RiAppBuilder
        assert RiAppBuilder is not None

    def test_config_imports(self):
        """Test config-related types can be imported"""
        from ri import RiConfig, RiConfigManager
        assert RiConfig is not None
        assert RiConfigManager is not None

    def test_error_import(self):
        """Test RiError can be imported"""
        from ri import RiError
        assert RiError is not None

    def test_logger_imports(self):
        """Test logger types can be imported"""
        from ri import RiLogger, RiLogConfig, RiLogLevel
        assert RiLogger is not None
        assert RiLogConfig is not None
        assert RiLogLevel is not None

    def test_filesystem_import(self):
        """Test RiFileSystem can be imported"""
        from ri import RiFileSystem
        assert RiFileSystem is not None

    def test_service_context_import(self):
        """Test RiServiceContext can be imported"""
        from ri import RiServiceContext
        assert RiServiceContext is not None

    def test_hook_imports(self):
        """Test hook-related types can be imported"""
        from ri import RiHookBus, RiHookEvent, RiHookKind, RiModulePhase
        assert RiHookBus is not None
        assert RiHookEvent is not None
        assert RiHookKind is not None
        assert RiModulePhase is not None


class TestHealthImports:
    """Test health check module imports"""

    def test_health_types_import(self):
        """Test health-related types can be imported"""
        from ri import (
            RiHealthStatus,
            RiHealthCheckResult,
            RiHealthCheckConfig,
            RiHealthReport,
            RiHealthChecker,
            RiHealthCheckType,
            RiHealthSummary,
        )
        assert RiHealthStatus is not None
        assert RiHealthCheckResult is not None
        assert RiHealthCheckConfig is not None
        assert RiHealthReport is not None
        assert RiHealthChecker is not None
        assert RiHealthCheckType is not None
        assert RiHealthSummary is not None


class TestCacheImports:
    """Test cache module imports"""

    def test_cache_types_import(self):
        """Test cache-related types can be imported"""
        from ri import (
            RiCacheModule,
            RiCacheConfig,
            RiCacheManager,
            RiCacheBackendType,
            RiCachePolicy,
            RiCacheStats,
            RiCachedValue,
            RiCacheEvent,
        )
        assert RiCacheModule is not None
        assert RiCacheConfig is not None
        assert RiCacheManager is not None
        assert RiCacheBackendType is not None
        assert RiCachePolicy is not None
        assert RiCacheStats is not None
        assert RiCachedValue is not None
        assert RiCacheEvent is not None


class TestAuthImports:
    """Test auth module imports"""

    def test_auth_types_import(self):
        """Test auth-related types can be imported"""
        from ri import (
            RiAuthModule,
            RiAuthConfig,
            RiJWTManager,
            RiJWTClaims,
            RiJWTValidationOptions,
            RiSessionManager,
            RiSession,
            RiPermissionManager,
            RiPermission,
            RiRole,
            RiOAuthManager,
            RiOAuthToken,
            RiOAuthUserInfo,
            RiOAuthProvider,
            RiJWTRevocationList,
            RiRevokedTokenInfo,
        )
        assert RiAuthModule is not None
        assert RiAuthConfig is not None
        assert RiJWTManager is not None
        assert RiJWTClaims is not None
        assert RiJWTValidationOptions is not None
        assert RiSessionManager is not None
        assert RiSession is not None
        assert RiPermissionManager is not None
        assert RiPermission is not None
        assert RiRole is not None
        assert RiOAuthManager is not None
        assert RiOAuthToken is not None
        assert RiOAuthUserInfo is not None
        assert RiOAuthProvider is not None
        assert RiJWTRevocationList is not None
        assert RiRevokedTokenInfo is not None


class TestDatabaseImports:
    """Test database module imports"""

    def test_database_types_import(self):
        """Test database-related types can be imported"""
        from ri import (
            RiDatabaseConfig,
            RiDatabasePool,
            RiDBRow,
            RiDBResult,
        )
        assert RiDatabaseConfig is not None
        assert RiDatabasePool is not None
        assert RiDBRow is not None
        assert RiDBResult is not None


class TestGatewayImports:
    """Test gateway module imports"""

    def test_gateway_types_import(self):
        """Test gateway-related types can be imported"""
        from ri import (
            RiGateway,
            RiGatewayConfig,
            RiRoute,
            RiRouter,
            RiRateLimiter,
            RiRateLimitConfig,
            RiRateLimitStats,
            RiSlidingWindowRateLimiter,
            RiCircuitBreaker,
            RiCircuitBreakerConfig,
            RiCircuitBreakerState,
            RiCircuitBreakerMetrics,
            RiBackendServer,
            RiLoadBalancerServerStats,
            RiLoadBalancer,
            RiLoadBalancerStrategy,
        )
        assert RiGateway is not None
        assert RiGatewayConfig is not None
        assert RiRoute is not None
        assert RiRouter is not None
        assert RiRateLimiter is not None
        assert RiRateLimitConfig is not None
        assert RiRateLimitStats is not None
        assert RiSlidingWindowRateLimiter is not None
        assert RiCircuitBreaker is not None
        assert RiCircuitBreakerConfig is not None
        assert RiCircuitBreakerState is not None
        assert RiCircuitBreakerMetrics is not None
        assert RiBackendServer is not None
        assert RiLoadBalancerServerStats is not None
        assert RiLoadBalancer is not None
        assert RiLoadBalancerStrategy is not None


class TestQueueImports:
    """Test queue module imports"""

    def test_queue_types_import(self):
        """Test queue-related types can be imported"""
        from ri import (
            RiQueueModule,
            RiQueueConfig,
            RiQueueManager,
            RiQueueMessage,
            RiQueueStats,
            RiQueueBackendType,
            RiRetryPolicy,
            RiDeadLetterConfig,
        )
        assert RiQueueModule is not None
        assert RiQueueConfig is not None
        assert RiQueueManager is not None
        assert RiQueueMessage is not None
        assert RiQueueStats is not None
        assert RiQueueBackendType is not None
        assert RiRetryPolicy is not None
        assert RiDeadLetterConfig is not None


class TestProtocolImports:
    """Test protocol module imports"""

    def test_protocol_types_import(self):
        """Test protocol-related types can be imported"""
        from ri import (
            RiProtocolManager,
            RiProtocolType,
            RiProtocolConfig,
            RiProtocolStatus,
            RiProtocolStats,
            RiConnectionState,
            RiConnectionStats,
            RiProtocolHealth,
            RiFrame,
            RiFrameHeader,
            RiFrameType,
            RiConnectionInfo,
            RiMessageFlags,
            RiSecurityLevel,
        )
        assert RiProtocolManager is not None
        assert RiProtocolType is not None
        assert RiProtocolConfig is not None
        assert RiProtocolStatus is not None
        assert RiProtocolStats is not None
        assert RiConnectionState is not None
        assert RiConnectionStats is not None
        assert RiProtocolHealth is not None
        assert RiFrame is not None
        assert RiFrameHeader is not None
        assert RiFrameType is not None
        assert RiConnectionInfo is not None
        assert RiMessageFlags is not None
        assert RiSecurityLevel is not None


class TestGrpcImports:
    """Test gRPC module imports"""

    def test_grpc_types_import(self):
        """Test gRPC-related types can be imported"""
        from ri import (
            RiGrpcConfig,
            RiGrpcStats,
            RiGrpcServiceRegistryPy,
            RiGrpcServerPy,
            RiGrpcClientPy,
        )
        assert RiGrpcConfig is not None
        assert RiGrpcStats is not None
        assert RiGrpcServiceRegistryPy is not None
        assert RiGrpcServerPy is not None
        assert RiGrpcClientPy is not None


class TestDeviceImports:
    """Test device module imports"""

    def test_device_types_import(self):
        """Test device-related types can be imported"""
        from ri import (
            RiDeviceControlModule,
            RiDeviceControlConfig,
            RiDeviceSchedulingConfig,
            RiDevice,
            RiDeviceType,
            RiDeviceStatus,
            RiDeviceCapabilities,
            RiDeviceHealthMetrics,
            RiDeviceController,
            RiResourceRequest,
            RiResourceAllocation,
            RiRequestSlaClass,
            RiResourceWeights,
            RiAffinityRules,
            RiResourcePool,
            RiResourcePoolConfig,
            RiResourcePoolStatistics,
            RiResourcePoolManager,
            RiResourceScheduler,
            RiDeviceScheduler,
            RiSchedulingPolicy,
            RiAllocationRecord,
            RiAllocationRequest,
            RiAllocationStatistics,
            RiSchedulingRecommendation,
            RiSchedulingRecommendationType,
            RiDeviceDiscoveryEngine,
        )
        assert RiDeviceControlModule is not None
        assert RiDeviceControlConfig is not None
        assert RiDeviceSchedulingConfig is not None
        assert RiDevice is not None
        assert RiDeviceType is not None
        assert RiDeviceStatus is not None
        assert RiDeviceCapabilities is not None
        assert RiDeviceHealthMetrics is not None
        assert RiDeviceController is not None
        assert RiResourceRequest is not None
        assert RiResourceAllocation is not None
        assert RiRequestSlaClass is not None
        assert RiResourceWeights is not None
        assert RiAffinityRules is not None
        assert RiResourcePool is not None
        assert RiResourcePoolConfig is not None
        assert RiResourcePoolStatistics is not None
        assert RiResourcePoolManager is not None
        assert RiResourceScheduler is not None
        assert RiDeviceScheduler is not None
        assert RiSchedulingPolicy is not None
        assert RiAllocationRecord is not None
        assert RiAllocationRequest is not None
        assert RiAllocationStatistics is not None
        assert RiSchedulingRecommendation is not None
        assert RiSchedulingRecommendationType is not None
        assert RiDeviceDiscoveryEngine is not None


class TestValidationImports:
    """Test validation module imports"""

    def test_validation_types_import(self):
        """Test validation-related types can be imported"""
        from ri import (
            RiValidationError,
            RiValidationResult,
            RiValidationSeverity,
            RiValidatorBuilder,
            RiValidationRunner,
            RiSanitizer,
            RiSanitizationConfig,
            RiSchemaValidator,
            RiValidationModule,
        )
        assert RiValidationError is not None
        assert RiValidationResult is not None
        assert RiValidationSeverity is not None
        assert RiValidatorBuilder is not None
        assert RiValidationRunner is not None
        assert RiSanitizer is not None
        assert RiSanitizationConfig is not None
        assert RiSchemaValidator is not None
        assert RiValidationModule is not None


class TestObservabilityImports:
    """Test observability module imports"""

    def test_observability_types_import(self):
        """Test observability-related types can be imported"""
        from ri import RiLifecycleObserver
        assert RiLifecycleObserver is not None


class TestTrafficManagerImport:
    """Test traffic manager import"""

    def test_traffic_manager_import(self):
        """Test RiTrafficManager can be imported"""
        from ri import RiTrafficManager
        assert RiTrafficManager is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
