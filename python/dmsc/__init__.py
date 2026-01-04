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

__version__ = "0.1.3"
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
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager, DMSCQueueMessage, DMSCQueueStats, DMSCQueueBackendType,
    
    # Gateway classes - also available directly
    DMSCGateway, DMSCGatewayConfig, DMSCRouter, DMSCRoute,
    
    # Service mesh classes - also available directly
    DMSCServiceMesh, DMSCServiceDiscovery, DMSCHealthChecker, DMSCTrafficManager,
    
    # Auth classes - also available directly
    DMSCAuthModule, DMSCAuthConfig, DMSCJWTManager, DMSCSessionManager, 
    DMSCPermissionManager, DMSCOAuthManager
)

# Import submodules
from .dmsc import (
    log, config, device, cache, fs, hooks, observability,
    queue, gateway, service_mesh, auth
)

# Create aliases for methods with _py suffix (pyo3 auto-renaming)
# This ensures Python API matches Rust API naming
def _create_method_alias(cls, old_name, new_name):
    """Create an alias for a method, mapping new_name to old_name."""
    def alias_method(self, *args, **kwargs):
        return getattr(self, old_name)(*args, **kwargs)
    alias_method.__name__ = new_name
    setattr(cls, new_name, alias_method)

# Apply aliases to DMSCJWTManager
if hasattr(DMSCJWTManager, 'generate_token_py'):
    _create_method_alias(DMSCJWTManager, 'generate_token_py', 'generate_token')
if hasattr(DMSCJWTManager, 'validate_token_py'):
    _create_method_alias(DMSCJWTManager, 'validate_token_py', 'validate_token')
if hasattr(DMSCJWTManager, 'get_token_expiry_py'):
    _create_method_alias(DMSCJWTManager, 'get_token_expiry_py', 'get_token_expiry')

# Apply aliases to DMSCQueueManager
if hasattr(DMSCQueueManager, 'push_py'):
    _create_method_alias(DMSCQueueManager, 'push_py', 'push')
if hasattr(DMSCQueueManager, 'pop_py'):
    _create_method_alias(DMSCQueueManager, 'pop_py', 'pop')

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
    'DMSCQueueModule', 'DMSCQueueConfig', 'DMSCQueueManager', 'DMSCQueueMessage', 'DMSCQueueStats', 'DMSCQueueBackendType',
    
    # Gateway classes
    'DMSCGateway', 'DMSCGatewayConfig', 'DMSCRouter', 'DMSCRoute',
    
    # Service mesh classes
    'DMSCServiceMesh', 'DMSCServiceDiscovery', 'DMSCHealthChecker', 'DMSCTrafficManager',
    
    # Auth classes
    'DMSCAuthModule', 'DMSCAuthConfig', 'DMSCJWTManager', 'DMSCSessionManager', 
    'DMSCPermissionManager', 'DMSCOAuthManager',
    
    # Submodules - these contain the actual classes
    'log', 'config', 'device', 'cache', 'fs', 'hooks', 'observability',
    'queue', 'gateway', 'service_mesh', 'auth'
]
