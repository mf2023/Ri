// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

import com.dunimd.ri.cache.RiCacheModule;
import com.dunimd.ri.cache.RiCacheConfig;
import com.dunimd.ri.cache.RiCacheBackendType;
import com.dunimd.ri.RiError;

/**
 * Ri Cache Module Example for Java.
 *
 * This example demonstrates how to use the Ri cache module for multi-backend
 * caching with support for memory backend.
 */
public class CacheExample {
    public static void main(String[] args) {
        System.out.println("=== Ri Cache Module Example - Java ===\n");

        try {
            // Create cache configuration
            System.out.println("1. Creating cache configuration...");
            RiCacheConfig config = new RiCacheConfig();
            config.setEnabled(true);
            config.setBackendType(RiCacheBackendType.Memory);
            config.setDefaultTtlSecs(300);
            config.setMaxMemoryMb(512);
            System.out.println("   Cache config created (memory backend)\n");

            // Initialize cache module
            System.out.println("2. Creating cache module...");
            RiCacheModule cacheModule = new RiCacheModule(config);
            System.out.println("   Cache module created\n");

            // Display cache configuration
            System.out.println("3. Cache configuration:");
            System.out.println("   Enabled: " + cacheModule.getConfig().isEnabled());
            System.out.println("   Backend type: " + cacheModule.getConfig().getBackendType());
            System.out.println("   Default TTL: " + cacheModule.getConfig().getDefaultTtlSecs() + " seconds");
            System.out.println("   Max memory: " + cacheModule.getConfig().getMaxMemoryMb() + " MB\n");

            // Cleanup
            cacheModule.close();

            System.out.println("=== Cache Example Completed ===");

        } catch (RiError e) {
            System.err.println("Ri Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
}
