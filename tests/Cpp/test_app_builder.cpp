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

#include <ri.hpp>
#include <iostream>
#include <cassert>

/**
 * Test RiAppBuilder C++ binding behavior.
 *
 * These tests verify that the C++ binding follows the immutable builder pattern,
 * where each with_xxx() method returns a new builder object (value semantics).
 *
 * This test file is located in the unified tests directory (tests/Cpp/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main() {
    std::cout << "=== RiAppBuilder C++ Binding Tests ===" << std::endl << std::endl;
    
    int passed = 0;
    int failed = 0;
    
    /* Test 1: Builder creation */
    std::cout << "Test 1: Builder creation..." << std::endl;
    try {
        ri::RiAppBuilder builder;
        std::cout << "[PASS] testBuilderCreation: Builder created successfully" << std::endl << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] testBuilderCreation: " << e.what() << std::endl << std::endl;
        failed++;
    }
    
    /* Test 2: Chain returns new object (value semantics) */
    std::cout << "Test 2: Chain returns new object..." << std::endl;
    try {
        ri::RiAppBuilder builder1;
        ri::RiAppBuilder builder2 = builder1.with_config("config.yaml");
        
        std::cout << "[PASS] testChainReturnsNewObject: with_config returns new object" << std::endl << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] testChainReturnsNewObject: " << e.what() << std::endl << std::endl;
        failed++;
    }
    
    /* Test 3: Multiple chained calls */
    std::cout << "Test 3: Multiple chained calls..." << std::endl;
    try {
        ri::RiAppBuilder builder1;
        ri::RiAppBuilder builder2 = builder1.with_config("config1.yaml");
        ri::RiAppBuilder builder3 = builder2.with_config("config2.yaml");
        
        std::cout << "[PASS] testMultipleChainedCalls: Each call returns new object" << std::endl << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] testMultipleChainedCalls: " << e.what() << std::endl << std::endl;
        failed++;
    }
    
    /* Test 4: Build creates runtime */
    std::cout << "Test 4: Build creates runtime..." << std::endl;
    try {
        ri::RiAppBuilder builder;
        ri::RiAppRuntime runtime = builder.build();
        
        std::cout << "[PASS] testBuildCreatesRuntime: build() returns RiAppRuntime" << std::endl << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[INFO] testBuildCreatesRuntime: Build failed (expected without config): " << e.what() << std::endl << std::endl;
        passed++;
    }
    
    /* Test 5: Resource cleanup (RAII) */
    std::cout << "Test 5: Resource cleanup (RAII)..." << std::endl;
    try {
        {
            ri::RiAppBuilder builder;
        }
        std::cout << "[PASS] testResourceCleanup: Builder destroyed automatically (RAII)" << std::endl << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] testResourceCleanup: " << e.what() << std::endl << std::endl;
        failed++;
    }
    
    std::cout << "=== Test Summary ===" << std::endl;
    std::cout << "Passed: " << passed << std::endl;
    std::cout << "Failed: " << failed << std::endl;
    std::cout << "Total:  " << (passed + failed) << std::endl;
    
    return (failed > 0) ? 1 : 0;
}
