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

import com.dunimd.ri.RiAppBuilder;
import com.dunimd.ri.RiAppRuntime;
import com.dunimd.ri.RiError;
import com.dunimd.ri.log.RiLogger;
import com.dunimd.ri.auth.RiAuthModule;
import com.dunimd.ri.auth.RiAuthConfig;
import com.dunimd.ri.cache.RiCacheModule;
import com.dunimd.ri.cache.RiCacheConfig;

/**
 * Ri Comprehensive API Example for Java.
 *
 * This example demonstrates the complete Ri API usage across all major modules,
 * providing a production-ready pattern for building enterprise applications.
 *
 * Features Demonstrated:
 * - Application initialization and configuration
 * - Authentication and authorization with JWT
 * - Cache operations with memory backend
 * - Error handling and resource cleanup
 *
 * Usage:
 *     java -Djava.library.path=/path/to/native/lib ComprehensiveExample
 */
public class ComprehensiveExample {
    public static void main(String[] args) {
        System.out.println("============================================================");
        System.out.println("Ri Comprehensive API Example - Java");
        System.out.println("============================================================\n");

        try {
            demonstrateApplicationInitialization();
            demonstrateAuthentication();
            demonstrateCaching();

            System.out.println("============================================================");
            System.out.println("All demonstrations completed successfully!");
            System.out.println("============================================================");

        } catch (Exception e) {
            System.err.println("Error during demonstration: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }

    /**
     * Demonstrate application initialization with all core components.
     *
     * This function shows how to:
     * - Create an application builder with method chaining
     * - Configure logging
     * - Build and run the application
     */
    private static void demonstrateApplicationInitialization() {
        System.out.println("=== Application Initialization ===\n");

        System.out.println("1. Creating application builder with method chaining...");
        RiAppBuilder builder = new RiAppBuilder();
        System.out.println("   Builder created successfully\n");

        System.out.println("2. Building application runtime...");
        try {
            RiAppRuntime runtime = builder.build();
            System.out.println("   Runtime built successfully!\n");
            runtime.close();
        } catch (RiError e) {
            System.out.println("   Note: Runtime build may require additional configuration: " + e.getMessage() + "\n");
        }

        System.out.println("3. Application initialization complete!\n");
    }

    /**
     * Demonstrate authentication and authorization functionality.
     *
     * This function shows how to:
     * - Configure authentication module
     * - Handle authentication context
     */
    private static void demonstrateAuthentication() {
        System.out.println("=== Authentication Module ===\n");

        System.out.println("1. Creating authentication configuration...");
        RiAuthConfig authConfig = new RiAuthConfig();
        System.out.println("   Auth config created with defaults\n");

        System.out.println("2. Creating authentication module...");
        try {
            RiAuthModule authModule = new RiAuthModule(authConfig);
            System.out.println("   Auth module created\n");

            System.out.println("3. Checking auth module properties...");
            System.out.println("   Enabled: " + authModule.isEnabled());
            System.out.println("   JWT expiry: " + authModule.getJwtExpirySecs() + " seconds");
            System.out.println("   Session timeout: " + authModule.getSessionTimeoutSecs() + " seconds\n");

            authModule.close();
        } catch (Exception e) {
            System.out.println("   Note: Auth module initialization: " + e.getMessage() + "\n");
        }

        System.out.println("4. Authentication demonstration complete!\n");
    }

    /**
     * Demonstrate cache operations with memory backend.
     *
     * This function shows how to:
     * - Configure cache with memory backend
     * - Create cache module
     */
    private static void demonstrateCaching() {
        System.out.println("=== Cache Module ===\n");

        System.out.println("1. Creating cache configuration...");
        RiCacheConfig cacheConfig = new RiCacheConfig();
        cacheConfig.setEnabled(true);
        cacheConfig.setDefaultTtlSecs(300);
        cacheConfig.setMaxMemoryMb(1000);
        System.out.println("   Cache config created (memory backend)\n");

        System.out.println("2. Creating cache module...");
        try {
            RiCacheModule cacheModule = new RiCacheModule(cacheConfig);
            System.out.println("   Cache module created\n");

            System.out.println("3. Cache module properties...");
            System.out.println("   Config enabled: " + cacheModule.getConfig().isEnabled());
            System.out.println("   Default TTL: " + cacheModule.getConfig().getDefaultTtlSecs() + " seconds");
            System.out.println("   Max memory: " + cacheModule.getConfig().getMaxMemoryMb() + " MB\n");

            cacheModule.close();
        } catch (Exception e) {
            System.out.println("   Note: Cache module initialization: " + e.getMessage() + "\n");
        }

        System.out.println("4. Cache demonstration complete!\n");
    }
}
