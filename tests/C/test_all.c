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
#include <stdbool.h>

/**
 * Test all DMSC C bindings.
 *
 * This test file is located in the unified tests directory (tests/C/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main(void) {
    printf("=== DMSC C Binding Test ===\n\n");
    
    int passed = 0;
    int failed = 0;
    
    /* Test DMSCAppBuilder */
    printf("Testing DMSCAppBuilder...\n");
    DMSCAppBuilder* builder = dmsc_app_builder_new();
    if (builder != NULL) {
        printf("[PASS] DMSCAppBuilder created\n");
        dmsc_app_builder_free(builder);
        passed++;
    } else {
        printf("[FAIL] DMSCAppBuilder creation failed\n");
        failed++;
    }
    
    /* Test DMSCAppBuilder chaining (returns new pointer) */
    printf("\nTesting DMSCAppBuilder chaining...\n");
    DMSCAppBuilder* builder1 = dmsc_app_builder_new();
    if (builder1 != NULL) {
        DMSCAppBuilder* builder2 = dmsc_app_builder_with_config(builder1, "config.yaml");
        
        if (builder2 != NULL && builder1 != builder2) {
            printf("[PASS] DMSCAppBuilder chaining creates new instance\n");
            passed++;
        } else {
            printf("[FAIL] DMSCAppBuilder chaining should create new instance\n");
            failed++;
        }
        
        dmsc_app_builder_free(builder1);
        if (builder2 != NULL) {
            dmsc_app_builder_free(builder2);
        }
    } else {
        printf("[FAIL] Failed to create builder for chaining test\n");
        failed++;
    }
    
    /* Test DMSCConfig */
    printf("\nTesting DMSCConfig...\n");
    DMSCConfig* config = dmsc_config_new();
    if (config != NULL) {
        printf("[PASS] DMSCConfig created\n");
        dmsc_config_free(config);
        passed++;
    } else {
        printf("[FAIL] DMSCConfig creation failed\n");
        failed++;
    }
    
    /* Test DMSCCacheModule */
    printf("\nTesting DMSCCacheModule...\n");
    DMSCCacheConfig* cache_config = dmsc_cache_config_new();
    if (cache_config != NULL) {
        DMSCCacheModule* cache = dmsc_cache_module_new(cache_config);
        if (cache != NULL) {
            printf("[PASS] DMSCCacheModule created\n");
            dmsc_cache_module_free(cache);
            passed++;
        } else {
            printf("[FAIL] DMSCCacheModule creation failed\n");
            failed++;
        }
        dmsc_cache_config_free(cache_config);
    } else {
        printf("[FAIL] DMSCCacheConfig creation failed\n");
        failed++;
    }
    
    /* Test DMSCAuthModule */
    printf("\nTesting DMSCAuthModule...\n");
    DMSCAuthConfig* auth_config = dmsc_auth_config_new();
    if (auth_config != NULL) {
        DMSCAuthModule* auth = dmsc_auth_module_new(auth_config);
        if (auth != NULL) {
            printf("[PASS] DMSCAuthModule created\n");
            dmsc_auth_module_free(auth);
            passed++;
        } else {
            printf("[FAIL] DMSCAuthModule creation failed\n");
            failed++;
        }
        dmsc_auth_config_free(auth_config);
    } else {
        printf("[FAIL] DMSCAuthConfig creation failed\n");
        failed++;
    }
    
    /* Test DMSCGateway */
    printf("\nTesting DMSCGateway...\n");
    DMSCGateway* gateway = dmsc_gateway_new();
    if (gateway != NULL) {
        printf("[PASS] DMSCGateway created\n");
        dmsc_gateway_free(gateway);
        passed++;
    } else {
        printf("[FAIL] DMSCGateway creation failed\n");
        failed++;
    }
    
    /* Test DMSCQueueModule */
    printf("\nTesting DMSCQueueModule...\n");
    DMSCQueueConfig* queue_config = dmsc_queue_config_new();
    if (queue_config != NULL) {
        DMSCQueueModule* queue = dmsc_queue_module_new(queue_config);
        if (queue != NULL) {
            printf("[PASS] DMSCQueueModule created\n");
            dmsc_queue_module_free(queue);
            passed++;
        } else {
            printf("[FAIL] DMSCQueueModule creation failed\n");
            failed++;
        }
        dmsc_queue_config_free(queue_config);
    } else {
        printf("[FAIL] DMSCQueueConfig creation failed\n");
        failed++;
    }
    
    /* Test DMSCServiceMesh */
    printf("\nTesting DMSCServiceMesh...\n");
    DMSCServiceMeshConfig* mesh_config = dmsc_service_mesh_config_new();
    if (mesh_config != NULL) {
        DMSCServiceMesh* mesh = dmsc_service_mesh_new(mesh_config);
        if (mesh != NULL) {
            printf("[PASS] DMSCServiceMesh created\n");
            dmsc_service_mesh_free(mesh);
            passed++;
        } else {
            printf("[FAIL] DMSCServiceMesh creation failed\n");
            failed++;
        }
        dmsc_service_mesh_config_free(mesh_config);
    } else {
        printf("[FAIL] DMSCServiceMeshConfig creation failed\n");
        failed++;
    }
    
    /* Test DMSCLogger */
    printf("\nTesting DMSCLogger...\n");
    DMSCLogger* logger = dmsc_logger_new();
    if (logger != NULL) {
        printf("[PASS] DMSCLogger created\n");
        dmsc_logger_free(logger);
        passed++;
    } else {
        printf("[FAIL] DMSCLogger creation failed\n");
        failed++;
    }
    
    /* Test DMSCFileSystem */
    printf("\nTesting DMSCFileSystem...\n");
    DMSCFileSystem* fs = dmsc_file_system_new();
    if (fs != NULL) {
        printf("[PASS] DMSCFileSystem created\n");
        dmsc_file_system_free(fs);
        passed++;
    } else {
        printf("[FAIL] DMSCFileSystem creation failed\n");
        failed++;
    }
    
    printf("\n=== Test Summary ===\n");
    printf("Passed: %d\n", passed);
    printf("Failed: %d\n", failed);
    printf("Total:  %d\n", passed + failed);
    
    return (failed > 0) ? 1 : 0;
}
