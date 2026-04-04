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

#include <dmsc.hpp>
#include <iostream>
#include <memory>

/**
 * Test all DMSC C++ bindings.
 *
 * This test file is located in the unified tests directory (tests/Cpp/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main() {
    std::cout << "=== DMSC C++ Binding Test ===" << std::endl << std::endl;
    
    int passed = 0;
    int failed = 0;
    
    /* Test DMSCAppBuilder */
    std::cout << "Testing DMSCAppBuilder..." << std::endl;
    try {
        dmsc::DMSCAppBuilder builder;
        std::cout << "[PASS] DMSCAppBuilder created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCAppBuilder: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCAppBuilder chaining (returns new object) */
    std::cout << std::endl << "Testing DMSCAppBuilder chaining..." << std::endl;
    try {
        dmsc::DMSCAppBuilder builder1;
        dmsc::DMSCAppBuilder builder2 = builder1.with_config("config.yaml");
        
        std::cout << "[PASS] DMSCAppBuilder chaining creates new object" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCAppBuilder chaining: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCConfig */
    std::cout << std::endl << "Testing DMSCConfig..." << std::endl;
    try {
        dmsc::DMSCConfig config;
        std::cout << "[PASS] DMSCConfig created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCConfig: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCCacheModule */
    std::cout << std::endl << "Testing DMSCCacheModule..." << std::endl;
    try {
        dmsc::DMSCCacheConfig config;
        dmsc::DMSCCacheModule cache(config);
        std::cout << "[PASS] DMSCCacheModule created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCCacheModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCAuthModule */
    std::cout << std::endl << "Testing DMSCAuthModule..." << std::endl;
    try {
        dmsc::DMSCAuthConfig config;
        dmsc::DMSCAuthModule auth(config);
        std::cout << "[PASS] DMSCAuthModule created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCAuthModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCGateway */
    std::cout << std::endl << "Testing DMSCGateway..." << std::endl;
    try {
        dmsc::DMSCGateway gateway;
        std::cout << "[PASS] DMSCGateway created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCGateway: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCQueueModule */
    std::cout << std::endl << "Testing DMSCQueueModule..." << std::endl;
    try {
        dmsc::DMSCQueueConfig config;
        dmsc::DMSCQueueModule queue(config);
        std::cout << "[PASS] DMSCQueueModule created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCQueueModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCServiceMesh */
    std::cout << std::endl << "Testing DMSCServiceMesh..." << std::endl;
    try {
        dmsc::DMSCServiceMeshConfig config;
        dmsc::DMSCServiceMesh mesh(config);
        std::cout << "[PASS] DMSCServiceMesh created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCServiceMesh: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCLogger */
    std::cout << std::endl << "Testing DMSCLogger..." << std::endl;
    try {
        dmsc::DMSCLogger logger;
        std::cout << "[PASS] DMSCLogger created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCLogger: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test DMSCFileSystem */
    std::cout << std::endl << "Testing DMSCFileSystem..." << std::endl;
    try {
        dmsc::DMSCFileSystem fs;
        std::cout << "[PASS] DMSCFileSystem created" << std::endl;
        passed++;
    } catch (const dmsc::DMSCError& e) {
        std::cout << "[FAIL] DMSCFileSystem: " << e.what() << std::endl;
        failed++;
    }
    
    std::cout << std::endl << "=== Test Summary ===" << std::endl;
    std::cout << "Passed: " << passed << std::endl;
    std::cout << "Failed: " << failed << std::endl;
    std::cout << "Total:  " << (passed + failed) << std::endl;
    
    return (failed > 0) ? 1 : 0;
}
