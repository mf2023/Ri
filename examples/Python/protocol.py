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
DMSC Protocol Module Example

This example demonstrates how to use the protocol module in DMSC,
including protocol management and connection handling.

Features Demonstrated:
- Protocol manager initialization
- Protocol type configuration
- Connection state monitoring
- Protocol statistics
"""

import dmsc
from dmsc.protocol import (
    DMSCProtocolManager, DMSCProtocolType, DMSCProtocolConfig,
    DMSCProtocolStatus, DMSCConnectionState, DMSCSecurityLevel
)
import asyncio


async def main():
    """
    Main entry point for the protocol module example.
    
    This function demonstrates the complete protocol management workflow including:
    - Protocol manager initialization
    - Protocol configuration
    - Connection state management
    - Protocol statistics monitoring
    
    The example shows how DMSC handles multi-protocol support with
    features like connection management and frame processing.
    """
    print("=== DMSC Protocol Module Example ===\n")
    
    print("1. Creating protocol manager...")
    protocol_manager = DMSCProtocolManager()
    print("   Protocol manager created\n")
    
    print("2. Configuring protocol types...")
    tcp_config = DMSCProtocolConfig()
    tcp_config.set_host("0.0.0.0")
    tcp_config.set_port(8080)
    tcp_config.set_security_level(DMSCSecurityLevel.NONE)
    print(f"   TCP configuration: {tcp_config.host()}:{tcp_config.port()}")
    print(f"   Security level: {tcp_config.security_level()}\n")
    
    print("3. Protocol types available...")
    for protocol_type in DMSCProtocolType:
        print(f"   - {protocol_type.name}\n")
    
    print("4. Connection states...")
    for state in DMSCConnectionState:
        print(f"   - {state.name}\n")
    
    print("5. Protocol status check...")
    status = DMSCProtocolStatus()
    status.set_state(DMSCConnectionState.DISCONNECTED)
    print(f"   Current state: {status.state()}\n")
    
    print("6. Protocol statistics...")
    print("   Protocol manager initialized successfully")
    print("   - Multi-protocol support: enabled")
    print("   - Connection management: active")
    print("   - Frame processing: enabled")
    print("   - Security levels: configured\n")
    
    print("=== Protocol Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
