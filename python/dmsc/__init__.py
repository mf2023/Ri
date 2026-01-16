#!/usr/bin/env python3

# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of DMSC.
# The DMSC project belongs to the Dunimd Team.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
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
DMSC (Dunimd Middleware Service) - A high-performance Rust middleware framework with modular architecture.

This Python library provides bindings to the DMSC Rust core, allowing you to leverage DMSC functionality
in Python applications.
"""

__version__ = "0.1.4"
__author__ = "Dunimd Team"
__license__ = "Apache-2.0"

# Import the Rust extension
from .dmsc import (
    # Core classes
    DMSCAppBuilder, DMSCAppRuntime, DMSCConfig, DMSCConfigManager, DMSCError,
    DMSCFileSystem, DMSCHookBus, DMSCHookEvent, DMSCHookKind, DMSCLogConfig,
    DMSCLogLevel, DMSCLogger, DMSCModulePhase, DMSCServiceContext,
    
    # Python module support
    DMSCPyModule, DMSCPyModuleAdapter, DMSCPyServiceModule, DMSCPyAsyncServiceModule,
    
    # Cache classes - also available directly
    DMSCCacheModule, DMSCCacheManager, DMSCCacheConfig, DMSCCacheBackendType,
    DMSCCachePolicy, DMSCCacheStats, DMSCCachedValue, DMSCCacheEvent,
    
    # Queue classes - also available directly
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager, DMSCQueueMessage, 
    DMSCQueueStats, DMSCQueueBackendType, DMSCRetryPolicy, DMSCDeadLetterConfig,
    
    # Gateway classes - also available directly
    DMSCGateway, DMSCGatewayConfig, DMSCRouter, DMSCRoute,
    DMSCRateLimiter, DMSCRateLimitConfig, RateLimitStats,
    DMSCSlidingWindowRateLimiter, DMSCCircuitBreaker, DMSCCircuitBreakerConfig,
    DMSCCircuitBreakerState, CircuitBreakerMetrics,
    
    # Service mesh classes - also available directly
    DMSCServiceMesh, DMSCServiceMeshConfig, DMSCServiceDiscovery,
    DMSCServiceInstance, DMSCServiceStatus,
    DMSCTrafficRoute, DMSCMatchCriteria, DMSCRouteAction, DMSCWeightedDestination,
    
    # Auth classes - also available directly
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTManager, DMSCSessionManager,
    DMSCSecurityManager, DMSCOAuthManager, DMSCOAuthToken, DMSCOAuthUserInfo,
    DMSCPermissionManager, DMSCPermission, DMSCRole, JWTRevocationList,
    
    # Device classes
    DMSCDeviceControlModule, DMSCDevice, DMSCDeviceType, DMSCDeviceStatus,
    DMSCDeviceCapabilities, DMSCDeviceHealthMetrics, DMSCDeviceController,
    DMSCDeviceConfig, DMSCDeviceControlConfig, NetworkDeviceInfo,
    DMSCDiscoveryResult, DMSCResourceRequest,
    DMSCResourceAllocation, DMSCRequestSlaClass, DMSCResourceWeights,
    DMSCAffinityRules,
    DMSCResourcePool, DMSCResourcePoolConfig, DMSCResourcePoolStatistics, DMSCResourcePoolManager,
    DMSCConnectionPoolStatistics,
    
    # Observability classes
    DMSCObservabilityModule, DMSCObservabilityConfig,
    DMSCMetricsRegistry, DMSCTracer,
    DMSCMetricType, DMSCMetricConfig, DMSCMetricSample, DMSCMetric,
    DMSCObservabilityData,
    
    # Validation classes
    DMSCValidationError, DMSCValidationResult, DMSCValidationSeverity,
    DMSCValidatorBuilder, DMSCValidationRunner, DMSCSanitizer,
    DMSCSanitizationConfig, DMSCSchemaValidator, DMSCValidationModule,
    
    # Protocol classes
    DMSCProtocolManager, DMSCProtocolType, DMSCProtocolConfig,
    DMSCProtocolStatus, DMSCProtocolStats, DMSCConnectionState,
    DMSCConnectionStats, DMSCProtocolHealth,
    DMSCFrame, DMSCFrameHeader, DMSCFrameType,
    DMSCConnectionInfo, DMSCMessageFlags, DMSCSecurityLevel,
    
    # Database classes
    DMSCDatabaseConfig, DMSCDatabasePool, DMSCDBRow, DMSCDBResult, DatabaseType,
)

# Import submodules
from .dmsc import (
    log, config, device, cache, fs, hooks, observability,
    queue, gateway, service_mesh, auth, protocol, database
)

# Core classes available directly
__all__ = [
    # Core classes
    'DMSCAppBuilder', 'DMSCAppRuntime', 'DMSCConfig', 'DMSCConfigManager', 'DMSCError',
    'DMSCFileSystem', 'DMSCHookBus', 'DMSCHookEvent', 'DMSCHookKind', 'DMSCLogConfig',
    'DMSCLogLevel', 'DMSCLogger', 'DMSCModulePhase', 'DMSCServiceContext',
    
    # Python module support
    'DMSCPyModule', 'DMSCPyModuleAdapter', 'DMSCPyServiceModule', 'DMSCPyAsyncServiceModule',
    
    # Cache classes
    'DMSCCacheModule', 'DMSCCacheManager', 'DMSCCacheConfig', 'DMSCCacheBackendType',
    'DMSCCachePolicy', 'DMSCCacheStats', 'DMSCCachedValue', 'DMSCCacheEvent',
    
    # Queue classes
    'DMSCQueueModule', 'DMSCQueueConfig', 'DMSCQueueManager', 'DMSCQueueMessage', 
    'DMSCQueueStats', 'DMSCQueueBackendType', 'DMSCRetryPolicy', 'DMSCDeadLetterConfig',
    
    # Gateway classes
    'DMSCGateway', 'DMSCGatewayConfig', 'DMSCRouter', 'DMSCRoute',
    'DMSCRateLimiter', 'DMSCRateLimitConfig', 'RateLimitStats',
    'DMSCSlidingWindowRateLimiter', 'DMSCCircuitBreaker', 'DMSCCircuitBreakerConfig',
    'DMSCCircuitBreakerState', 'CircuitBreakerMetrics',
    
    # Service mesh classes
    'DMSCServiceMesh', 'DMSCServiceMeshConfig', 'DMSCServiceDiscovery',
    'DMSCServiceInstance', 'DMSCServiceStatus',
    'DMSCTrafficRoute', 'DMSCMatchCriteria', 'DMSCRouteAction', 'DMSCWeightedDestination',
    
    # Auth classes
    'DMSCAuthModule', 'DMSCAuthConfig', 'DMSCJWTManager', 'DMSCSessionManager', 
    'DMSCSecurityManager', 'DMSCOAuthManager', 'DMSCOAuthToken', 'DMSCOAuthUserInfo',
    'DMSCPermissionManager', 'DMSCPermission', 'DMSCRole', 'JWTRevocationList',
    
    # Device classes
    'DMSCDeviceControlModule', 'DMSCDevice', 'DMSCDeviceType', 'DMSCDeviceStatus',
    'DMSCDeviceCapabilities', 'DMSCDeviceHealthMetrics', 'DMSCDeviceController',
    'DMSCDeviceConfig', 'DMSCDeviceControlConfig', 'NetworkDeviceInfo',
    'DMSCDiscoveryResult', 'DMSCResourceRequest',
    'DMSCResourceAllocation', 'DMSCRequestSlaClass', 'DMSCResourceWeights',
    'DMSCAffinityRules',
    'DMSCResourcePool', 'DMSCResourcePoolConfig', 'DMSCResourcePoolStatistics', 'DMSCResourcePoolManager',
    'DMSCConnectionPoolStatistics',
    
    # Observability classes
    'DMSCObservabilityModule', 'DMSCObservabilityConfig',
    'DMSCMetricsRegistry', 'DMSCTracer',
    'DMSCMetricType', 'DMSCMetricConfig', 'DMSCMetricSample', 'DMSCMetric',
    'DMSCObservabilityData',
    
    # Validation classes
    'DMSCValidationError', 'DMSCValidationResult', 'DMSCValidationSeverity',
    'DMSCValidatorBuilder', 'DMSCValidationRunner', 'DMSCSanitizer',
    'DMSCSanitizationConfig', 'DMSCSchemaValidator', 'DMSCValidationModule',
    
    # Protocol classes
    'DMSCProtocolManager', 'DMSCProtocolType', 'DMSCProtocolConfig',
    'DMSCProtocolStatus', 'DMSCProtocolStats', 'DMSCConnectionState',
    'DMSCConnectionStats', 'DMSCProtocolHealth',
    'DMSCFrame', 'DMSCFrameHeader', 'DMSCFrameType',
    'DMSCConnectionInfo', 'DMSCMessageFlags', 'DMSCSecurityLevel',
    
    # Database classes
    'DMSCDatabaseConfig', 'DMSCDatabasePool', 'DMSCDBRow', 'DMSCDBResult', 'DatabaseType',
    
    # Validation classes
    'log', 'config', 'device', 'cache', 'fs', 'hooks', 'observability',
    'queue', 'gateway', 'service_mesh', 'auth', 'protocol', 'database'
]
