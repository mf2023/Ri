/*
 * Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
 *
 * This file is part of Ri.
 * The Ri project belongs to the Dunimd Team.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * You may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <ri.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * Ri Cache Module Example for C.
 *
 * This example demonstrates how to use the Ri cache module for multi-backend
 * caching with support for memory backend.
 */
int main(void) {
    printf("=== Ri Cache Module Example - C ===\n\n");

    int passed = 0;
    int failed = 0;

    /* Create cache configuration */
    printf("1. Creating cache configuration...\n");
    RiCacheConfig* config = ri_cache_config_new();
    if (config == NULL) {
        printf("   [FAIL] Failed to create cache config\n");
        return 1;
    }

    ri_cache_config_set_enabled(config, true);
    ri_cache_config_set_backend_type(config, Ri_CACHE_BACKEND_MEMORY);
    ri_cache_config_set_default_ttl_secs(config, 300);
    ri_cache_config_set_max_memory_mb(config, 512);
    printf("   Cache config created (memory backend)\n\n");

    /* Initialize cache module */
    printf("2. Creating cache module...\n");
    RiCacheModule* cache_module = ri_cache_module_new(config);
    if (cache_module == NULL) {
        printf("   [FAIL] Failed to create cache module\n");
        ri_cache_config_free(config);
        return 1;
    }
    printf("   Cache module created\n\n");

    /* Display cache configuration */
    printf("3. Cache configuration:\n");
    printf("   Enabled: %s\n", ri_cache_config_is_enabled(config) ? "true" : "false");
    printf("   Backend type: %d\n", ri_cache_config_get_backend_type(config));
    printf("   Default TTL: %llu seconds\n", ri_cache_config_get_default_ttl_secs(config));
    printf("   Max memory: %llu MB\n\n", ri_cache_config_get_max_memory_mb(config));

    /* Cleanup */
    ri_cache_module_free(cache_module);
    ri_cache_config_free(config);

    printf("=== Cache Example Completed ===\n\n");

    printf("Test Summary:\n");
    printf("  Passed: %d\n", passed);
    printf("  Failed: %d\n", failed);

    return (failed > 0) ? 1 : 0;
}
