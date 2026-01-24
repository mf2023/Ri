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
DMSC Cache Module Example

This example demonstrates how to use the caching module in DMSC,
including in-memory cache operations and Redis backend integration.

Features Demonstrated:
- In-memory cache operations (get, set, delete)
- TTL (Time-To-Live) based expiration
- Cache statistics and monitoring
- Multiple backend support
"""

from dmsc import (
    DMSCCacheModule, DMSCCacheConfig, DMSCCacheBackendType,
    DMSCCacheManager
)
import asyncio
import time


async def main():
    """
    Main entry point for the cache module example.
    
    This function demonstrates the complete caching workflow including:
    - Cache module initialization and configuration
    - Basic CRUD operations (Create, Read, Delete)
    - TTL-based expiration for temporary data
    - Cache statistics monitoring
    - Cache clearing operations
    
    The example shows how DMSC handles caching with features like
    automatic expiration, hit/miss tracking, and memory management.
    """
    print("=== DMSC Cache Module Example ===\n")
    
    print("1. Creating cache module...")
    cache_module = DMSCCacheModule()
    print("   Cache module created\n")
    
    print("2. Getting cache manager...")
    cache_manager = cache_module.cache_manager()
    print("   Cache manager retrieved\n")
    
    async def run_cache_operations():
        print("3. Basic Cache Operations")
        print("   ------------------------")
        
        print("   Setting 'user:1001' = 'John Doe'")
        await cache_manager.set("user:1001", "John Doe", None)
        print("   ✓ Value set successfully\n")
        
        print("   Getting 'user:1001'...")
        value = await cache_manager.get("user:1001")
        if value:
            print(f"   ✓ Found value: {value}\n")
        else:
            print("   ✗ Value not found\n")
        
        print("4. TTL-Based Expiration")
        print("   ---------------------")
        
        print("   Setting 'temporary:key' with 5 second TTL")
        await cache_manager.set("temporary:key", "This will expire", 5)
        print("   ✓ Value set with TTL\n")
        
        print("   Checking 'temporary:key' before expiration...")
        value = await cache_manager.get("temporary:key")
        if value:
            print(f"   ✓ Value still exists: {value}\n")
        
        print("   Waiting 6 seconds for TTL expiration...")
        time.sleep(6)
        print("   ✓ Wait complete\n")
        
        print("   Checking 'temporary:key' after expiration...")
        value = await cache_manager.get("temporary:key")
        if value is None:
            print("   ✓ Value expired and removed\n")
        
        print("5. Cache Statistics")
        print("   -----------------")
        
        stats = cache_manager.stats()
        print(f"   Current cache stats:")
        print(f"   - Hits: {stats.hits}")
        print(f"   - Misses: {stats.misses}")
        print(f"   - Size: {stats.entries}\n")
        
        print("6. Delete Operations")
        print("   ------------------")
        
        print("   Deleting 'user:1001'...")
        await cache_manager.delete("user:1001")
        print("   ✓ Value deleted\n")
        
        print("   Verifying deletion...")
        value = await cache_manager.get("user:1001")
        if value is None:
            print("   ✓ Value successfully deleted\n")
        
        print("7. Clear All Cache")
        print("   ----------------")
        
        print("   Adding more values...")
        await cache_manager.set("key1", "value1", None)
        await cache_manager.set("key2", "value2", None)
        await cache_manager.set("key3", "value3", None)
        print("   ✓ Added 3 values\n")
        
        print("   Clearing all cache...")
        await cache_manager.clear()
        print("   ✓ Cache cleared\n")
        
        print("8. Final Statistics")
        print("   -----------------")
        
        final_stats = cache_manager.stats()
        print(f"   Final cache stats:")
        print(f"   - Hits: {final_stats.hits}")
        print(f"   - Misses: {final_stats.misses}")
        print(f"   - Size: {final_stats.entries}\n")
    
    await run_cache_operations()
    print("=== Cache Example Completed ===")


if __name__ == "__main__":
    asyncio.run(main())
