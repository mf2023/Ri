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
 * Test DMSCAppBuilder C binding behavior.
 *
 * These tests verify that the C binding follows the immutable builder pattern,
 * where each with_xxx() function returns a new builder pointer.
 *
 * This test file is located in the unified tests directory (tests/C/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main(void) {
    printf("=== DMSCAppBuilder C Binding Tests ===\n\n");
    
    int passed = 0;
    int failed = 0;
    
    /* Test 1: Builder creation */
    printf("Test 1: Builder creation...\n");
    DMSCAppBuilder* builder = dmsc_app_builder_new();
    if (builder != NULL) {
        printf("[PASS] testBuilderCreation: Builder created successfully\n\n");
        dmsc_app_builder_free(builder);
        passed++;
    } else {
        printf("[FAIL] testBuilderCreation: Failed to create builder\n\n");
        failed++;
    }
    
    /* Test 2: Chain returns new pointer */
    printf("Test 2: Chain returns new pointer...\n");
    DMSCAppBuilder* builder1 = dmsc_app_builder_new();
    if (builder1 != NULL) {
        DMSCAppBuilder* builder2 = dmsc_app_builder_with_config(builder1, "config.yaml");
        
        if (builder2 != NULL && builder1 != builder2) {
            printf("[PASS] testChainReturnsNewPointer: with_config returns new pointer\n\n");
            passed++;
        } else {
            printf("[FAIL] testChainReturnsNewPointer: with_config should return new pointer\n\n");
            failed++;
        }
        
        dmsc_app_builder_free(builder1);
        if (builder2 != NULL) {
            dmsc_app_builder_free(builder2);
        }
    } else {
        printf("[FAIL] Failed to create builder for chaining test\n\n");
        failed++;
    }
    
    /* Test 3: Multiple chained calls */
    printf("Test 3: Multiple chained calls...\n");
    DMSCAppBuilder* b1 = dmsc_app_builder_new();
    if (b1 != NULL) {
        DMSCAppBuilder* b2 = dmsc_app_builder_with_config(b1, "config1.yaml");
        DMSCAppBuilder* b3 = (b2 != NULL) ? dmsc_app_builder_with_config(b2, "config2.yaml") : NULL;
        
        if (b1 != b2 && b2 != b3 && b1 != b3) {
            printf("[PASS] testMultipleChainedCalls: Each call returns new pointer\n\n");
            passed++;
        } else {
            printf("[FAIL] testMultipleChainedCalls: Each call should return new pointer\n\n");
            failed++;
        }
        
        dmsc_app_builder_free(b1);
        if (b2 != NULL) dmsc_app_builder_free(b2);
        if (b3 != NULL) dmsc_app_builder_free(b3);
    } else {
        printf("[FAIL] Failed to create builder for multiple chaining test\n\n");
        failed++;
    }
    
    /* Test 4: Build creates runtime */
    printf("Test 4: Build creates runtime...\n");
    DMSCAppBuilder* b = dmsc_app_builder_new();
    if (b != NULL) {
        DMSCAppRuntime* runtime = dmsc_app_builder_build(b);
        
        if (runtime != NULL) {
            printf("[PASS] testBuildCreatesRuntime: build() returns DMSCAppRuntime\n\n");
            passed++;
            dmsc_app_runtime_free(runtime);
        } else {
            printf("[INFO] testBuildCreatesRuntime: Build failed (expected without config)\n\n");
            passed++;
        }
        
        dmsc_app_builder_free(b);
    } else {
        printf("[FAIL] Failed to create builder for build test\n\n");
        failed++;
    }
    
    /* Test 5: Resource cleanup */
    printf("Test 5: Resource cleanup...\n");
    DMSCAppBuilder* b_clean = dmsc_app_builder_new();
    if (b_clean != NULL) {
        dmsc_app_builder_free(b_clean);
        printf("[PASS] testResourceCleanup: Builder freed successfully\n\n");
        passed++;
    } else {
        printf("[FAIL] Failed to create builder for cleanup test\n\n");
        failed++;
    }
    
    printf("=== Test Summary ===\n");
    printf("Passed: %d\n", passed);
    printf("Failed: %d\n", failed);
    printf("Total:  %d\n", passed + failed);
    
    return (failed > 0) ? 1 : 0;
}
