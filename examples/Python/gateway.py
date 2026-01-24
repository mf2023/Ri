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
DMSC Gateway Module Example

This example demonstrates how to use the API gateway module in DMSC,
including routing and basic gateway functionality.

Features Demonstrated:
- Route configuration and management
- Route lookup and matching
- Gateway initialization
"""

from dmsc import (
    DMSCGateway, DMSCRoute, DMSCRouter,
    DMSCGatewayConfig
)
import asyncio


async def main():
    """
    Main entry point for the gateway module example.
    
    This function demonstrates the complete API gateway workflow including:
    - Gateway module initialization
    - Route configuration and management
    - Route lookup and matching
    
    The example shows how DMSC handles API gateway functionality with
    features like request routing.
    """
    print("=== DMSC Gateway Module Example ===\n")
    
    print("1. Creating gateway...")
    gateway = DMSCGateway()
    print("   Gateway created\n")
    
    print("2. Getting router...")
    router = gateway.router()
    print("   Router retrieved\n")
    
    print("3. Adding routes...")
    route1 = DMSCRoute("/api/users")
    route1.set_method("GET")
    router.add_route(route1)
    
    route2 = DMSCRoute("/api/products")
    route2.set_method("GET")
    router.add_route(route2)
    
    route3 = DMSCRoute("/api/admin")
    route3.set_method("GET")
    route3.set_method("POST")
    route3.set_method("PUT")
    route3.set_method("DELETE")
    router.add_route(route3)
    
    route4 = DMSCRoute("/health")
    route4.set_method("GET")
    router.add_route(route4)
    
    print(f"   Added 4 routes\n")
    
    print("4. Route lookup...")
    user_route = router.route("/api/users")
    if user_route:
        print(f"   ✓ Found route for /api/users")
        print(f"   Methods: {user_route.methods()}\n")
    else:
        print("   ✗ No route found for /api/users\n")
    
    admin_route = router.route("/api/admin/users")
    if admin_route:
        print(f"   ✓ Found route for /api/admin/users")
        print(f"   Methods: {admin_route.methods()}\n")
    else:
        print("   ✗ No route found for /api/admin/users\n")
    
    unknown_route = router.route("/api/unknown")
    if unknown_route:
        print(f"   Found route for /api/unknown")
    else:
        print("   ✗ No route found for /api/unknown\n")
    
    print("5. Gateway statistics...")
    print("   Gateway initialized successfully")
    print("   - Router: configured with routes")
    print("   - Rate limiting: enabled by default")
    print("   - Circuit breaker: enabled by default\n")
    
    print("=== Gateway Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
