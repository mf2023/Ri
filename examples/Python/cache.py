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

import dmsc
from dmsc.cache import DMSCCacheModule, DMSCCacheConfig
import asyncio
import time


def main():
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
    
    # Module Initialization: Create cache module instance
    # The module provides caching capabilities with support for multiple backends
    # (in-memory, Redis, etc.) and automatic TTL-based expiration
    cache_module = DMSCCacheModule()
    
    # Get cache manager for cache operations
    # The manager provides thread-safe operations for cache manipulation
    cache_manager = cache_module.get_manager()
    
    # Async wrapper to demonstrate all cache operations
    async def run_cache_operations():
        """
        Async function containing all cache demonstration operations.
        
        This function executes a comprehensive set of cache operations:
        1. Basic cache CRUD (Create, Read, Delete)
        2. TTL-based expiration with time tracking
        3. Statistics monitoring (hits, misses, size)
        4. Bulk operations (clear all)
        
        The operations demonstrate proper async/await patterns for
        cache interactions and error handling scenarios.
        """
        # Section 1: Basic Cache Operations
        # Demonstrates fundamental cache operations: SET and GET
        print("1. Basic Cache Operations")
        print("   ------------------------")
        
        # SET Operation: Store a value in the cache
        # Parameters:
        # - key: Unique identifier for the cached value (supports namespacing with ':')
        # - value: Bytes object to store (supports any serializable data)
        # Key format 'user:1001' follows convention: entity_type:entity_id
        print("   Setting 'user:1001' = 'John Doe'")
        await cache_manager.set("user:1001", b"John Doe")
        print("   ✓ Value set successfully\n")
        
        # GET Operation: Retrieve a value from the cache
        # Returns the cached value if key exists, None otherwise
        # This triggers cache hit/miss statistics tracking
        print("   Getting 'user:1001'...")
        value = await cache_manager.get("user:1001")
        if value:
            # Value found - cache hit scenario
            print(f"   ✓ Found value: {value.decode()}\n")
        else:
            # Value not found - cache miss scenario
            print("   ✗ Value not found\n")
        
        # Section 2: TTL-Based Expiration
        # Demonstrates time-to-live (TTL) functionality for automatic data expiration
        print("2. TTL-Based Expiration")
        print("   ---------------------")
        
        # SET with TTL: Store a value with automatic expiration
        # Parameters:
        # - key: Cache key identifier
        # - value: Bytes value to store
        # - ttl: Time-to-live in seconds (None for no expiration)
        # After TTL expires, the key is automatically removed from cache
        print("   Setting 'temporary:key' with 5 second TTL")
        await cache_manager.set(
            "temporary:key",
            b"This will expire",
            5  # 5 seconds TTL
        )
        print("   ✓ Value set with TTL\n")
        
        # Verify value exists before expiration
        # This demonstrates cache hit before TTL expiry
        print("   Checking 'temporary:key' before expiration...")
        value = await cache_manager.get("temporary:key")
        if value:
            print(f"   ✓ Value still exists: {value.decode()}\n")
        
        # Wait for TTL expiration
        # Block for 6 seconds to ensure the 5-second TTL has passed
        print("   Waiting 6 seconds for TTL expiration...")
        time.sleep(6)
        print("   ✓ Wait complete\n")
        
        # Verify value is gone after expiration
        # This demonstrates automatic cleanup by the cache system
        print("   Checking 'temporary:key' after expiration...")
        value = await cache_manager.get("temporary:key")
        if value is None:
            print("   ✓ Value expired and removed\n")
        
        # Section 3: Cache Statistics
        # Demonstrates monitoring and metrics collection for cache operations
        print("3. Cache Statistics")
        print("   -----------------")
        
        # Get cache statistics from the manager
        # Statistics include:
        # - hits: Number of successful cache lookups
        # - misses: Number of failed cache lookups
        # - size: Current number of items in cache
        stats = await cache_manager.get_stats()
        print(f"   Current cache stats:")
        print(f"   - Hits: {stats.hits()}")
        print(f"   - Misses: {stats.misses()}")
        print(f"   - Size: {stats.size()}\n")
        
        # Section 4: Delete Operations
        # Demonstrates manual cache entry removal
        print("4. Delete Operations")
        print("   ------------------")
        
        # DELETE Operation: Remove a specific key from cache
        # Unlike expiration, this is an explicit removal
        print("   Deleting 'user:1001'...")
        await cache_manager.delete("user:1001")
        print("   ✓ Value deleted\n")
        
        # Verify deletion was successful
        print("   Verifying deletion...")
        value = await cache_manager.get("user:1001")
        if value is None:
            print("   ✓ Value successfully deleted\n")
        
        # Section 5: Clear All Cache
        # Demonstrates bulk cache cleanup operation
        print("5. Clear All Cache")
        print("   ----------------")
        
        # Add multiple values for demonstration
        print("   Adding more values...")
        await cache_manager.set("key1", b"value1")
        await cache_manager.set("key2", b"value2")
        await cache_manager.set("key3", b"value3")
        print("   ✓ Added 3 values\n")
        
        # CLEAR Operation: Remove all entries from cache
        # Use with caution - this removes all cached data
        print("   Clearing all cache...")
        await cache_manager.clear()
        print("   ✓ Cache cleared\n")
        
        # Section 6: Final Statistics
        # Verify cache state after cleanup operations
        print("6. Final Statistics")
        print("   -----------------")
        
        # Get updated statistics after all operations
        # Expected: size should be 0 after clear operation
        final_stats = await cache_manager.get_stats()
        print(f"   Final cache stats:")
        print(f"   - Hits: {final_stats.hits()}")
        print(f"   - Misses: {final_stats.misses()}")
        print(f"   - Size: {final_stats.size()}\n")
    
    # Execute the async cache operations demonstration
    asyncio.run(run_cache_operations())
    print("=== Cache Example Completed ===")


if __name__ == "__main__":
    main()
