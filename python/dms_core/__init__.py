#!/usr/bin/env python3

# Copyright © 2025 Wenze Wei. All Rights Reserved.
#
# This file is part of DMS.
# The DMS project belongs to the Dunimd Team.
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
DMS (Dunimd Middleware Service) - A high-performance Rust middleware framework with modular architecture.

This Python library provides bindings to the DMS Rust core, allowing you to leverage DMS functionality
in Python applications.
"""

__version__ = "0.1.2"
__author__ = "Dunimd Team"
__license__ = "Apache-2.0"

# Import the Rust extension
from .dms_core import (
    # Core classes
    DMSAppBuilder, DMSAppRuntime, DMSConfig, DMSConfigManager, DMSError,
    DMSFileSystem, DMSHookBus, DMSHookEvent, DMSHookKind, DMSLogConfig,
    DMSLogLevel, DMSLogger, DMSModulePhase, DMSServiceContext,
    
    # Queue classes - also available directly
    DMSQueueModule, DMSQueueConfig, DMSQueueManager, DMSQueueMessage, DMSQueueStats,
    
    # Gateway classes - also available directly
    DMSGateway, DMSGatewayConfig, DMSRouter, DMSRoute,
    
    # Service mesh classes - also available directly
    DMSServiceMesh, DMSServiceDiscovery, DMSHealthChecker, DMSTrafficManager,
    
    # Auth classes - also available directly
    DMSAuthModule, DMSAuthConfig, DMSJWTManager, DMSSessionManager, 
    DMSPermissionManager, DMSOAuthManager
)

# Import submodules
from .dms_core import (
    log, config, device, cache, fs, hooks, observability,
    queue, gateway, service_mesh, auth
)

# Core classes available directly
__all__ = [
    # Core classes
    'DMSAppBuilder', 'DMSAppRuntime', 'DMSConfig', 'DMSConfigManager', 'DMSError',
    'DMSFileSystem', 'DMSHookBus', 'DMSHookEvent', 'DMSHookKind', 'DMSLogConfig',
    'DMSLogLevel', 'DMSLogger', 'DMSModulePhase', 'DMSServiceContext',
    
    # Queue classes
    'DMSQueueModule', 'DMSQueueConfig', 'DMSQueueManager', 'DMSQueueMessage', 'DMSQueueStats',
    
    # Gateway classes
    'DMSGateway', 'DMSGatewayConfig', 'DMSRouter', 'DMSRoute',
    
    # Service mesh classes
    'DMSServiceMesh', 'DMSServiceDiscovery', 'DMSHealthChecker', 'DMSTrafficManager',
    
    # Auth classes
    'DMSAuthModule', 'DMSAuthConfig', 'DMSJWTManager', 'DMSSessionManager', 
    'DMSPermissionManager', 'DMSOAuthManager',
    
    # Submodules - these contain the actual classes
    'log', 'config', 'device', 'cache', 'fs', 'hooks', 'observability',
    'queue', 'gateway', 'service_mesh', 'auth'
]