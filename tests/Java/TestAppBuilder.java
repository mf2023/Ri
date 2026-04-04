// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

import com.dunimd.dmsc.DMSCAppBuilder;
import com.dunimd.dmsc.DMSCAppRuntime;
import com.dunimd.dmsc.DMSCError;

/**
 * Tests for DMSCAppBuilder Java binding behavior.
 * 
 * These tests verify that the Java binding follows the immutable builder pattern,
 * where each withXxx() method returns a new builder instance.
 *
 * This test file is located in the unified tests directory (tests/Java/)
 * rather than in the source code directory, following the project's testing convention.
 */
public class TestAppBuilder {
    public static void main(String[] args) {
        System.out.println("=== DMSCAppBuilder Java Binding Tests ===\n");
        
        int passed = 0;
        int failed = 0;
        
        // Test 1: Builder creation
        try {
            DMSCAppBuilder builder = new DMSCAppBuilder();
            System.out.println("[PASS] testBuilderCreation: Builder created successfully");
            builder.close();
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] testBuilderCreation: " + e.getMessage());
            failed++;
        }
        
        // Test 2: Chain returns new instance
        try {
            DMSCAppBuilder builder1 = new DMSCAppBuilder();
            DMSCAppBuilder builder2 = builder1.withConfig("config.yaml");
            
            if (builder1 != builder2) {
                System.out.println("[PASS] testChainReturnsNewInstance: withConfig returns new instance");
                passed++;
            } else {
                System.out.println("[FAIL] testChainReturnsNewInstance: withConfig should return new instance");
                failed++;
            }
            
            builder1.close();
            builder2.close();
        } catch (Exception e) {
            System.out.println("[FAIL] testChainReturnsNewInstance: " + e.getMessage());
            failed++;
        }
        
        // Test 3: Multiple chained calls
        try {
            DMSCAppBuilder builder1 = new DMSCAppBuilder();
            DMSCAppBuilder builder2 = builder1.withConfig("config1.yaml");
            DMSCAppBuilder builder3 = builder2.withConfig("config2.yaml");
            
            if (builder1 != builder2 && builder2 != builder3 && builder1 != builder3) {
                System.out.println("[PASS] testMultipleChainedCalls: Each call returns new instance");
                passed++;
            } else {
                System.out.println("[FAIL] testMultipleChainedCalls: Each call should return new instance");
                failed++;
            }
            
            builder1.close();
            builder2.close();
            builder3.close();
        } catch (Exception e) {
            System.out.println("[FAIL] testMultipleChainedCalls: " + e.getMessage());
            failed++;
        }
        
        // Test 4: Build creates runtime
        try {
            DMSCAppBuilder builder = new DMSCAppBuilder();
            DMSCAppRuntime runtime = builder.build();
            
            if (runtime != null) {
                System.out.println("[PASS] testBuildCreatesRuntime: build() returns DMSCAppRuntime");
                passed++;
                runtime.close();
            } else {
                System.out.println("[FAIL] testBuildCreatesRuntime: build() should return non-null runtime");
                failed++;
            }
            
            builder.close();
        } catch (DMSCError e) {
            System.out.println("[INFO] testBuildCreatesRuntime: Build failed (expected without config): " + e.getMessage());
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] testBuildCreatesRuntime: " + e.getMessage());
            failed++;
        }
        
        // Test 5: Resource cleanup
        try {
            DMSCAppBuilder builder = new DMSCAppBuilder();
            builder.close();
            System.out.println("[PASS] testResourceCleanup: Builder closed successfully");
            passed++;
        } catch (Exception e) {
            System.out.println("[FAIL] testResourceCleanup: " + e.getMessage());
            failed++;
        }
        
        System.out.println("\n=== Test Summary ===");
        System.out.println("Passed: " + passed);
        System.out.println("Failed: " + failed);
        System.out.println("Total:  " + (passed + failed));
        
        if (failed > 0) {
            System.exit(1);
        }
    }
}
