# Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
#
# This file is part of Ri.
# The Ri project belongs to the Dunimd Team.
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
Ri Cache Module Example

This example demonstrates how to use the Ri cache module for multi-backend
caching with support for memory, Redis, and other backends.
"""

import asyncio
from ri import (
    RiCacheModule,
    RiCacheConfig,
    RiCacheManager,
    RiCacheBackendType,
    RiCachePolicy,
    RiCacheStats,
    RiCachedValue,
    RiCacheEvent,
)


async def main():
    # Create cache configuration
    config = RiCacheConfig()
    config.backend_type = RiCacheBackendType.Memory
    config.default_ttl_secs = 300
    config.max_memory_mb = 512

    # Initialize cache module
    cache_module = RiCacheModule(config)

    # Create cache manager
    manager = RiCacheManager()

    # Create cache policy
    policy = RiCachePolicy()
    policy.ttl_seconds = 600
    policy.max_size = 1000
    policy.eviction_policy = "lru"

    # Set values with different types
    print("Setting cache values...")

    # String value
    string_value = RiCachedValue()
    string_value.data = b"Hello, Ri Cache!"
    string_value.content_type = "text/plain"
    manager.set("greeting", string_value, policy)

    # JSON value
    json_value = RiCachedValue()
    json_value.data = b'{"name": "John", "age": 30, "city": "New York"}'
    json_value.content_type = "application/json"
    manager.set("user:123", json_value, policy)

    # Binary value
    binary_value = RiCachedValue()
    binary_value.data = b"\x00\x01\x02\x03\x04\x05"
    binary_value.content_type = "application/octet-stream"
    manager.set("binary:data", binary_value, policy)

    # Get values from cache
    print("\nGetting cache values...")

    greeting = manager.get("greeting")
    if greeting:
        print(f"Greeting: {greeting.data.decode('utf-8')}")

    user = manager.get("user:123")
    if user:
        print(f"User data: {user.data.decode('utf-8')}")

    # Check if key exists
    exists = manager.exists("greeting")
    print(f"\nKey 'greeting' exists: {exists}")

    # Get cache statistics
    print("\nCache Statistics:")
    stats = RiCacheStats()
    print(f"Cache hits: {stats.hits}")
    print(f"Cache misses: {stats.misses}")
    print(f"Hit rate: {stats.hit_rate:.2%}")
    print(f"Total keys: {stats.total_keys}")
    print(f"Memory usage: {stats.memory_usage_bytes} bytes")

    # Delete a key
    print("\nDeleting key 'binary:data'...")
    manager.delete("binary:data")

    # Verify deletion
    exists_after_delete = manager.exists("binary:data")
    print(f"Key 'binary:data' exists after delete: {exists_after_delete}")

    # Set with custom TTL
    print("\nSetting value with custom TTL...")
    temp_value = RiCachedValue()
    temp_value.data = b"This will expire soon"
    temp_value.content_type = "text/plain"

    custom_policy = RiCachePolicy()
    custom_policy.ttl_seconds = 10
    manager.set("temp:key", temp_value, custom_policy)
    print("Value set with 10 second TTL")

    # List all keys
    print("\nAll cache keys:")
    keys = manager.keys()
    for key in keys:
        print(f"  - {key}")

    # Clear cache
    print("\nClearing cache...")
    manager.clear()
    print("Cache cleared!")

    print("\nCache operations completed successfully!")


if __name__ == "__main__":
    asyncio.run(main())
