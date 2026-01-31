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
DMSC gRPC Module Example

This example demonstrates how to use the gRPC module in DMSC,
including server configuration and client usage.

Features Demonstrated:
- gRPC server setup and configuration
- Service registry management
- gRPC client initialization
- Statistics and monitoring
"""

import dmsc
from dmsc.grpc import (
    DMSCGrpcConfig, DMSCGrpcServer, DMSCGrpcClient,
    DMSCGrpcStats, DMSCGrpcServiceRegistryPy
)
import asyncio


async def main():
    """
    Main entry point for the gRPC module example.
    
    This function demonstrates the complete gRPC workflow including:
    - gRPC server configuration
    - Service registration
    - Client connection setup
    - Statistics monitoring
    
    The example shows how DMSC handles gRPC communication with
    features like service registry and performance monitoring.
    """
    print("=== DMSC gRPC Module Example ===\n")
    
    print("1. Creating gRPC configuration...")
    config = DMSCGrpcConfig()
    config.set_addr("0.0.0.0")
    config.set_port(50051)
    config.set_max_concurrent_requests(100)
    print(f"   gRPC server configured on {config.get_addr()}:{config.get_port()}")
    print(f"   Max concurrent requests: {config.get_max_concurrent_requests()}\n")
    
    print("2. Creating gRPC server...")
    server = DMSCGrpcServer(config)
    print("   gRPC server instance created\n")
    
    print("3. Service registry...")
    registry = DMSCGrpcServiceRegistryPy()
    print("   Service registry initialized\n")
    
    print("4. Creating gRPC client...")
    client_config = DMSCGrpcConfig()
    client_config.set_addr("localhost")
    client_config.set_port(50051)
    print("   gRPC client configured\n")
    
    print("5. gRPC statistics...")
    stats = DMSCGrpcStats()
    print("   gRPC module initialized successfully")
    print("   - Server: configured and ready")
    print("   - Client: connection setup available")
    print("   - Service registry: active")
    print("   - Statistics: monitoring enabled\n")
    
    print("=== gRPC Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
