/*
 * Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
 *
 * This file is part of DMSC.
 * The DMSC project belongs to the Dunimd Team.
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

#include <dmsc.h>
#include <stdio.h>
#include <stdlib.h>

/**
 * DMSC Cache Module Example for C.
 *
 * This example demonstrates how to use the DMSC cache module for multi-backend
 * caching with support for memory backend.
 */
int main(void) {
    printf("=== DMSC Cache Module Example - C ===\n\n");

    int passed = 0;
    int failed = 0;

    /* Create cache configuration */
    printf("1. Creating cache configuration...\n");
    DMSCCacheConfig* config = dmsc_cache_config_new();
    if (config == NULL) {
        printf("   [FAIL] Failed to create cache config\n");
        return 1;
    }

    dmsc_cache_config_set_enabled(config, true);
    dmsc_cache_config_set_backend_type(config, DMSC_CACHE_BACKEND_MEMORY);
    dmsc_cache_config_set_default_ttl_secs(config, 300);
    dmsc_cache_config_set_max_memory_mb(config, 512);
    printf("   Cache config created (memory backend)\n\n");

    /* Initialize cache module */
    printf("2. Creating cache module...\n");
    DMSCCacheModule* cache_module = dmsc_cache_module_new(config);
    if (cache_module == NULL) {
        printf("   [FAIL] Failed to create cache module\n");
        dmsc_cache_config_free(config);
        return 1;
    }
    printf("   Cache module created\n\n");

    /* Display cache configuration */
    printf("3. Cache configuration:\n");
    printf("   Enabled: %s\n", dmsc_cache_config_is_enabled(config) ? "true" : "false");
    printf("   Backend type: %d\n", dmsc_cache_config_get_backend_type(config));
    printf("   Default TTL: %llu seconds\n", dmsc_cache_config_get_default_ttl_secs(config));
    printf("   Max memory: %llu MB\n\n", dmsc_cache_config_get_max_memory_mb(config));

    /* Cleanup */
    dmsc_cache_module_free(cache_module);
    dmsc_cache_config_free(config);

    printf("=== Cache Example Completed ===\n\n");

    printf("Test Summary:\n");
    printf("  Passed: %d\n", passed);
    printf("  Failed: %d\n", failed);

    return (failed > 0) ? 1 : 0;
}
