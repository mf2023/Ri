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
#include <memory>

/**
 * Test all Ri C++ bindings.
 *
 * This test file is located in the unified tests directory (tests/Cpp/)
 * rather than in the source code directory, following the project's testing convention.
 */
int main() {
    std::cout << "=== Ri C++ Binding Test ===" << std::endl << std::endl;
    
    int passed = 0;
    int failed = 0;
    
    /* Test RiAppBuilder */
    std::cout << "Testing RiAppBuilder..." << std::endl;
    try {
        ri::RiAppBuilder builder;
        std::cout << "[PASS] RiAppBuilder created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiAppBuilder: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiAppBuilder chaining (returns new object) */
    std::cout << std::endl << "Testing RiAppBuilder chaining..." << std::endl;
    try {
        ri::RiAppBuilder builder1;
        ri::RiAppBuilder builder2 = builder1.with_config("config.yaml");
        
        std::cout << "[PASS] RiAppBuilder chaining creates new object" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiAppBuilder chaining: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiConfig */
    std::cout << std::endl << "Testing RiConfig..." << std::endl;
    try {
        ri::RiConfig config;
        std::cout << "[PASS] RiConfig created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiConfig: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiCacheModule */
    std::cout << std::endl << "Testing RiCacheModule..." << std::endl;
    try {
        ri::RiCacheConfig config;
        ri::RiCacheModule cache(config);
        std::cout << "[PASS] RiCacheModule created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiCacheModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiAuthModule */
    std::cout << std::endl << "Testing RiAuthModule..." << std::endl;
    try {
        ri::RiAuthConfig config;
        ri::RiAuthModule auth(config);
        std::cout << "[PASS] RiAuthModule created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiAuthModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiGateway */
    std::cout << std::endl << "Testing RiGateway..." << std::endl;
    try {
        ri::RiGateway gateway;
        std::cout << "[PASS] RiGateway created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiGateway: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiQueueModule */
    std::cout << std::endl << "Testing RiQueueModule..." << std::endl;
    try {
        ri::RiQueueConfig config;
        ri::RiQueueModule queue(config);
        std::cout << "[PASS] RiQueueModule created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiQueueModule: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiServiceMesh */
    std::cout << std::endl << "Testing RiServiceMesh..." << std::endl;
    try {
        ri::RiServiceMeshConfig config;
        ri::RiServiceMesh mesh(config);
        std::cout << "[PASS] RiServiceMesh created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiServiceMesh: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiLogger */
    std::cout << std::endl << "Testing RiLogger..." << std::endl;
    try {
        ri::RiLogger logger;
        std::cout << "[PASS] RiLogger created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiLogger: " << e.what() << std::endl;
        failed++;
    }
    
    /* Test RiFileSystem */
    std::cout << std::endl << "Testing RiFileSystem..." << std::endl;
    try {
        ri::RiFileSystem fs;
        std::cout << "[PASS] RiFileSystem created" << std::endl;
        passed++;
    } catch (const ri::RiError& e) {
        std::cout << "[FAIL] RiFileSystem: " << e.what() << std::endl;
        failed++;
    }
    
    std::cout << std::endl << "=== Test Summary ===" << std::endl;
    std::cout << "Passed: " << passed << std::endl;
    std::cout << "Failed: " << failed << std::endl;
    std::cout << "Total:  " << (passed + failed) << std::endl;
    
    return (failed > 0) ? 1 : 0;
}
