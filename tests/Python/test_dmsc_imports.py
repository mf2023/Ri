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
DMSC Module Import Tests

Tests that verify all documented DMSC types can be imported from Python.
This ensures Python bindings are correctly exposing Rust types.
"""

import pytest


class TestCoreImports:
    """Test core module imports"""

    def test_app_builder_import(self):
        """Test DMSCAppBuilder can be imported"""
        from dmsc import DMSCAppBuilder
        assert DMSCAppBuilder is not None

    def test_config_imports(self):
        """Test config-related types can be imported"""
        from dmsc import DMSCConfig, DMSCConfigManager
        assert DMSCConfig is not None
        assert DMSCConfigManager is not None

    def test_error_import(self):
        """Test DMSCError can be imported"""
        from dmsc import DMSCError
        assert DMSCError is not None

    def test_logger_imports(self):
        """Test logger types can be imported"""
        from dmsc import DMSCLogger, DMSCLogConfig, DMSCLogLevel
        assert DMSCLogger is not None
        assert DMSCLogConfig is not None
        assert DMSCLogLevel is not None

    def test_filesystem_import(self):
        """Test DMSCFileSystem can be imported"""
        from dmsc import DMSCFileSystem
        assert DMSCFileSystem is not None

    def test_service_context_import(self):
        """Test DMSCServiceContext can be imported"""
        from dmsc import DMSCServiceContext
        assert DMSCServiceContext is not None

    def test_hook_imports(self):
        """Test hook-related types can be imported"""
        from dmsc import DMSCHookBus, DMSCHookEvent, DMSCHookKind, DMSCModulePhase
        assert DMSCHookBus is not None
        assert DMSCHookEvent is not None
        assert DMSCHookKind is not None
        assert DMSCModulePhase is not None


class TestHealthImports:
    """Test health check module imports"""

    def test_health_types_import(self):
        """Test health-related types can be imported"""
        from dmsc import (
            DMSCHealthStatus,
            DMSCHealthCheckResult,
            DMSCHealthCheckConfig,
            DMSCHealthReport,
            DMSCHealthChecker,
            DMSCHealthCheckType,
            DMSCHealthSummary,
        )
        assert DMSCHealthStatus is not None
        assert DMSCHealthCheckResult is not None
        assert DMSCHealthCheckConfig is not None
        assert DMSCHealthReport is not None
        assert DMSCHealthChecker is not None
        assert DMSCHealthCheckType is not None
        assert DMSCHealthSummary is not None


class TestCacheImports:
    """Test cache module imports"""

    def test_cache_types_import(self):
        """Test cache-related types can be imported"""
        from dmsc import (
            DMSCCacheModule,
            DMSCCacheConfig,
            DMSCCacheManager,
            DMSCCacheBackendType,
            DMSCCachePolicy,
            DMSCCacheStats,
            DMSCCachedValue,
            DMSCCacheEvent,
        )
        assert DMSCCacheModule is not None
        assert DMSCCacheConfig is not None
        assert DMSCCacheManager is not None
        assert DMSCCacheBackendType is not None
        assert DMSCCachePolicy is not None
        assert DMSCCacheStats is not None
        assert DMSCCachedValue is not None
        assert DMSCCacheEvent is not None


class TestAuthImports:
    """Test auth module imports"""

    def test_auth_types_import(self):
        """Test auth-related types can be imported"""
        from dmsc import (
            DMSCAuthModule,
            DMSCAuthConfig,
            DMSCJWTManager,
            DMSCJWTClaims,
            DMSCJWTValidationOptions,
            DMSCSessionManager,
            DMSCSession,
            DMSCPermissionManager,
            DMSCPermission,
            DMSCRole,
            DMSCOAuthManager,
            DMSCOAuthToken,
            DMSCOAuthUserInfo,
            DMSCOAuthProvider,
            DMSCJWTRevocationList,
            DMSCRevokedTokenInfo,
        )
        assert DMSCAuthModule is not None
        assert DMSCAuthConfig is not None
        assert DMSCJWTManager is not None
        assert DMSCJWTClaims is not None
        assert DMSCJWTValidationOptions is not None
        assert DMSCSessionManager is not None
        assert DMSCSession is not None
        assert DMSCPermissionManager is not None
        assert DMSCPermission is not None
        assert DMSCRole is not None
        assert DMSCOAuthManager is not None
        assert DMSCOAuthToken is not None
        assert DMSCOAuthUserInfo is not None
        assert DMSCOAuthProvider is not None
        assert DMSCJWTRevocationList is not None
        assert DMSCRevokedTokenInfo is not None


class TestDatabaseImports:
    """Test database module imports"""

    def test_database_types_import(self):
        """Test database-related types can be imported"""
        from dmsc import (
            DMSCDatabaseConfig,
            DMSCDatabasePool,
            DMSCDBRow,
            DMSCDBResult,
        )
        assert DMSCDatabaseConfig is not None
        assert DMSCDatabasePool is not None
        assert DMSCDBRow is not None
        assert DMSCDBResult is not None


class TestGatewayImports:
    """Test gateway module imports"""

    def test_gateway_types_import(self):
        """Test gateway-related types can be imported"""
        from dmsc import (
            DMSCGateway,
            DMSCGatewayConfig,
            DMSCRoute,
            DMSCRouter,
            DMSCRateLimiter,
            DMSCRateLimitConfig,
            DMSCRateLimitStats,
            DMSCSlidingWindowRateLimiter,
            DMSCCircuitBreaker,
            DMSCCircuitBreakerConfig,
            DMSCCircuitBreakerState,
            DMSCCircuitBreakerMetrics,
            DMSCBackendServer,
            DMSCLoadBalancerServerStats,
            DMSCLoadBalancer,
            DMSCLoadBalancerStrategy,
        )
        assert DMSCGateway is not None
        assert DMSCGatewayConfig is not None
        assert DMSCRoute is not None
        assert DMSCRouter is not None
        assert DMSCRateLimiter is not None
        assert DMSCRateLimitConfig is not None
        assert DMSCRateLimitStats is not None
        assert DMSCSlidingWindowRateLimiter is not None
        assert DMSCCircuitBreaker is not None
        assert DMSCCircuitBreakerConfig is not None
        assert DMSCCircuitBreakerState is not None
        assert DMSCCircuitBreakerMetrics is not None
        assert DMSCBackendServer is not None
        assert DMSCLoadBalancerServerStats is not None
        assert DMSCLoadBalancer is not None
        assert DMSCLoadBalancerStrategy is not None


class TestQueueImports:
    """Test queue module imports"""

    def test_queue_types_import(self):
        """Test queue-related types can be imported"""
        from dmsc import (
            DMSCQueueModule,
            DMSCQueueConfig,
            DMSCQueueManager,
            DMSCQueueMessage,
            DMSCQueueStats,
            DMSCQueueBackendType,
            DMSCRetryPolicy,
            DMSCDeadLetterConfig,
        )
        assert DMSCQueueModule is not None
        assert DMSCQueueConfig is not None
        assert DMSCQueueManager is not None
        assert DMSCQueueMessage is not None
        assert DMSCQueueStats is not None
        assert DMSCQueueBackendType is not None
        assert DMSCRetryPolicy is not None
        assert DMSCDeadLetterConfig is not None


class TestProtocolImports:
    """Test protocol module imports"""

    def test_protocol_types_import(self):
        """Test protocol-related types can be imported"""
        from dmsc import (
            DMSCProtocolManager,
            DMSCProtocolType,
            DMSCProtocolConfig,
            DMSCProtocolStatus,
            DMSCProtocolStats,
            DMSCConnectionState,
            DMSCConnectionStats,
            DMSCProtocolHealth,
            DMSCFrame,
            DMSCFrameHeader,
            DMSCFrameType,
            DMSCConnectionInfo,
            DMSCMessageFlags,
            DMSCSecurityLevel,
        )
        assert DMSCProtocolManager is not None
        assert DMSCProtocolType is not None
        assert DMSCProtocolConfig is not None
        assert DMSCProtocolStatus is not None
        assert DMSCProtocolStats is not None
        assert DMSCConnectionState is not None
        assert DMSCConnectionStats is not None
        assert DMSCProtocolHealth is not None
        assert DMSCFrame is not None
        assert DMSCFrameHeader is not None
        assert DMSCFrameType is not None
        assert DMSCConnectionInfo is not None
        assert DMSCMessageFlags is not None
        assert DMSCSecurityLevel is not None


class TestGrpcImports:
    """Test gRPC module imports"""

    def test_grpc_types_import(self):
        """Test gRPC-related types can be imported"""
        from dmsc import (
            DMSCGrpcConfig,
            DMSCGrpcStats,
            DMSCGrpcPythonService,
            DMSCGrpcServiceRegistryPy,
            DMSCGrpcServerPy,
            DMSCGrpcClientPy,
        )
        assert DMSCGrpcConfig is not None
        assert DMSCGrpcStats is not None
        assert DMSCGrpcPythonService is not None
        assert DMSCGrpcServiceRegistryPy is not None
        assert DMSCGrpcServerPy is not None
        assert DMSCGrpcClientPy is not None


class TestDeviceImports:
    """Test device module imports"""

    def test_device_types_import(self):
        """Test device-related types can be imported"""
        from dmsc import (
            DMSCDeviceControlModule,
            DMSCDeviceControlConfig,
            DMSCDeviceSchedulingConfig,
            DMSCDevice,
            DMSCDeviceType,
            DMSCDeviceStatus,
            DMSCDeviceCapabilities,
            DMSCDeviceHealthMetrics,
            DMSCDeviceController,
            DMSCResourceRequest,
            DMSCResourceAllocation,
            DMSCRequestSlaClass,
            DMSCResourceWeights,
            DMSCAffinityRules,
            DMSCResourcePool,
            DMSCResourcePoolConfig,
            DMSCResourcePoolStatistics,
            DMSCResourcePoolManager,
            DMSCResourceScheduler,
            DMSCDeviceScheduler,
            DMSCSchedulingPolicy,
            DMSCAllocationRecord,
            DMSCAllocationRequest,
            DMSCAllocationStatistics,
            DMSCSchedulingRecommendation,
            DMSCSchedulingRecommendationType,
            DMSCDeviceDiscoveryEngine,
        )
        assert DMSCDeviceControlModule is not None
        assert DMSCDeviceControlConfig is not None
        assert DMSCDeviceSchedulingConfig is not None
        assert DMSCDevice is not None
        assert DMSCDeviceType is not None
        assert DMSCDeviceStatus is not None
        assert DMSCDeviceCapabilities is not None
        assert DMSCDeviceHealthMetrics is not None
        assert DMSCDeviceController is not None
        assert DMSCResourceRequest is not None
        assert DMSCResourceAllocation is not None
        assert DMSCRequestSlaClass is not None
        assert DMSCResourceWeights is not None
        assert DMSCAffinityRules is not None
        assert DMSCResourcePool is not None
        assert DMSCResourcePoolConfig is not None
        assert DMSCResourcePoolStatistics is not None
        assert DMSCResourcePoolManager is not None
        assert DMSCResourceScheduler is not None
        assert DMSCDeviceScheduler is not None
        assert DMSCSchedulingPolicy is not None
        assert DMSCAllocationRecord is not None
        assert DMSCAllocationRequest is not None
        assert DMSCAllocationStatistics is not None
        assert DMSCSchedulingRecommendation is not None
        assert DMSCSchedulingRecommendationType is not None
        assert DMSCDeviceDiscoveryEngine is not None


class TestValidationImports:
    """Test validation module imports"""

    def test_validation_types_import(self):
        """Test validation-related types can be imported"""
        from dmsc import (
            DMSCValidationError,
            DMSCValidationResult,
            DMSCValidationSeverity,
            DMSCValidatorBuilder,
            DMSCValidationRunner,
            DMSCSanitizer,
            DMSCSanitizationConfig,
            DMSCSchemaValidator,
            DMSCValidationModule,
        )
        assert DMSCValidationError is not None
        assert DMSCValidationResult is not None
        assert DMSCValidationSeverity is not None
        assert DMSCValidatorBuilder is not None
        assert DMSCValidationRunner is not None
        assert DMSCSanitizer is not None
        assert DMSCSanitizationConfig is not None
        assert DMSCSchemaValidator is not None
        assert DMSCValidationModule is not None


class TestObservabilityImports:
    """Test observability module imports"""

    def test_observability_types_import(self):
        """Test observability-related types can be imported"""
        from dmsc import DMSCLifecycleObserver
        assert DMSCLifecycleObserver is not None


class TestTrafficManagerImport:
    """Test traffic manager import"""

    def test_traffic_manager_import(self):
        """Test DMSCTrafficManager can be imported"""
        from dmsc import DMSCTrafficManager
        assert DMSCTrafficManager is not None


if __name__ == "__main__":
    pytest.main([__file__, "-v"])
