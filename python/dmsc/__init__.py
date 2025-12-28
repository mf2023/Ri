#!/usr/bin/env python3

# Copyright © 2025 Wenze Wei. All Rights Reserved.
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
    PyDMSCModule, PythonModuleAdapter, PyServiceModule, PyAsyncServiceModule,
    
    # Queue classes - also available directly
    DMSCQueueModule, DMSCQueueConfig, DMSCQueueManager, DMSCQueueMessage, DMSCQueueStats,
    
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

# Core classes available directly
__all__ = [
    # Core classes
    'DMSCAppBuilder', 'DMSCAppRuntime', 'DMSCConfig', 'DMSCConfigManager', 'DMSCError',
    'DMSCFileSystem', 'DMSCHookBus', 'DMSCHookEvent', 'DMSCHookKind', 'DMSCLogConfig',
    'DMSCLogLevel', 'DMSCLogger', 'DMSCModulePhase', 'DMSCServiceContext',
    
    # Python module support
    'PyDMSCModule', 'PythonModuleAdapter', 'PyServiceModule', 'PyAsyncServiceModule',
    
    # Queue classes
    'DMSCQueueModule', 'DMSCQueueConfig', 'DMSCQueueManager', 'DMSCQueueMessage', 'DMSCQueueStats',
    
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