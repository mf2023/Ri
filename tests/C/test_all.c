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
#include <stdbool.h>

/**
 * Test all Ri C bindings.
 *
 * This test file is located in the unified tests directory (tests/C/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main(void) {
    printf("=== Ri C Binding Test ===\n\n");
    
    int passed = 0;
    int failed = 0;
    
    /* Test RiAppBuilder */
    printf("Testing RiAppBuilder...\n");
    RiAppBuilder* builder = ri_app_builder_new();
    if (builder != NULL) {
        printf("[PASS] RiAppBuilder created\n");
        ri_app_builder_free(builder);
        passed++;
    } else {
        printf("[FAIL] RiAppBuilder creation failed\n");
        failed++;
    }
    
    /* Test RiAppBuilder chaining (returns new pointer) */
    printf("\nTesting RiAppBuilder chaining...\n");
    RiAppBuilder* builder1 = ri_app_builder_new();
    if (builder1 != NULL) {
        RiAppBuilder* builder2 = ri_app_builder_with_config(builder1, "config.yaml");
        
        if (builder2 != NULL && builder1 != builder2) {
            printf("[PASS] RiAppBuilder chaining creates new instance\n");
            passed++;
        } else {
            printf("[FAIL] RiAppBuilder chaining should create new instance\n");
            failed++;
        }
        
        ri_app_builder_free(builder1);
        if (builder2 != NULL) {
            ri_app_builder_free(builder2);
        }
    } else {
        printf("[FAIL] Failed to create builder for chaining test\n");
        failed++;
    }
    
    /* Test RiConfig */
    printf("\nTesting RiConfig...\n");
    RiConfig* config = ri_config_new();
    if (config != NULL) {
        printf("[PASS] RiConfig created\n");
        ri_config_free(config);
        passed++;
    } else {
        printf("[FAIL] RiConfig creation failed\n");
        failed++;
    }
    
    /* Test RiCacheModule */
    printf("\nTesting RiCacheModule...\n");
    RiCacheConfig* cache_config = ri_cache_config_new();
    if (cache_config != NULL) {
        RiCacheModule* cache = ri_cache_module_new(cache_config);
        if (cache != NULL) {
            printf("[PASS] RiCacheModule created\n");
            ri_cache_module_free(cache);
            passed++;
        } else {
            printf("[FAIL] RiCacheModule creation failed\n");
            failed++;
        }
        ri_cache_config_free(cache_config);
    } else {
        printf("[FAIL] RiCacheConfig creation failed\n");
        failed++;
    }
    
    /* Test RiAuthModule */
    printf("\nTesting RiAuthModule...\n");
    RiAuthConfig* auth_config = ri_auth_config_new();
    if (auth_config != NULL) {
        RiAuthModule* auth = ri_auth_module_new(auth_config);
        if (auth != NULL) {
            printf("[PASS] RiAuthModule created\n");
            ri_auth_module_free(auth);
            passed++;
        } else {
            printf("[FAIL] RiAuthModule creation failed\n");
            failed++;
        }
        ri_auth_config_free(auth_config);
    } else {
        printf("[FAIL] RiAuthConfig creation failed\n");
        failed++;
    }
    
    /* Test RiGateway */
    printf("\nTesting RiGateway...\n");
    RiGateway* gateway = ri_gateway_new();
    if (gateway != NULL) {
        printf("[PASS] RiGateway created\n");
        ri_gateway_free(gateway);
        passed++;
    } else {
        printf("[FAIL] RiGateway creation failed\n");
        failed++;
    }
    
    /* Test RiQueueModule */
    printf("\nTesting RiQueueModule...\n");
    RiQueueConfig* queue_config = ri_queue_config_new();
    if (queue_config != NULL) {
        RiQueueModule* queue = ri_queue_module_new(queue_config);
        if (queue != NULL) {
            printf("[PASS] RiQueueModule created\n");
            ri_queue_module_free(queue);
            passed++;
        } else {
            printf("[FAIL] RiQueueModule creation failed\n");
            failed++;
        }
        ri_queue_config_free(queue_config);
    } else {
        printf("[FAIL] RiQueueConfig creation failed\n");
        failed++;
    }
    
    /* Test RiServiceMesh */
    printf("\nTesting RiServiceMesh...\n");
    RiServiceMeshConfig* mesh_config = ri_service_mesh_config_new();
    if (mesh_config != NULL) {
        RiServiceMesh* mesh = ri_service_mesh_new(mesh_config);
        if (mesh != NULL) {
            printf("[PASS] RiServiceMesh created\n");
            ri_service_mesh_free(mesh);
            passed++;
        } else {
            printf("[FAIL] RiServiceMesh creation failed\n");
            failed++;
        }
        ri_service_mesh_config_free(mesh_config);
    } else {
        printf("[FAIL] RiServiceMeshConfig creation failed\n");
        failed++;
    }
    
    /* Test RiLogger */
    printf("\nTesting RiLogger...\n");
    RiLogger* logger = ri_logger_new();
    if (logger != NULL) {
        printf("[PASS] RiLogger created\n");
        ri_logger_free(logger);
        passed++;
    } else {
        printf("[FAIL] RiLogger creation failed\n");
        failed++;
    }
    
    /* Test RiFileSystem */
    printf("\nTesting RiFileSystem...\n");
    RiFileSystem* fs = ri_file_system_new();
    if (fs != NULL) {
        printf("[PASS] RiFileSystem created\n");
        ri_file_system_free(fs);
        passed++;
    } else {
        printf("[FAIL] RiFileSystem creation failed\n");
        failed++;
    }
    
    printf("\n=== Test Summary ===\n");
    printf("Passed: %d\n", passed);
    printf("Failed: %d\n", failed);
    printf("Total:  %d\n", passed + failed);
    
    return (failed > 0) ? 1 : 0;
}
