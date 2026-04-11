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

import com.dunimd.ri.gateway.RiGateway;
import com.dunimd.ri.gateway.RiGatewayConfig;
import com.dunimd.ri.RiError;

/**
 * Ri Gateway Module Example for Java.
 *
 * This example demonstrates how to use the Ri gateway module for API gateway
 * functionality including routing and rate limiting.
 */
public class GatewayExample {
    public static void main(String[] args) {
        System.out.println("=== Ri Gateway Module Example - Java ===\n");

        try {
            // Create gateway configuration
            System.out.println("1. Creating gateway configuration...");
            RiGatewayConfig config = new RiGatewayConfig();
            config.setHost("0.0.0.0");
            config.setPort(8080);
            config.setEnableRateLimiting(true);
            config.setEnableCircuitBreaker(true);
            config.setMaxRequestSizeMb(10);
            config.setTimeoutSeconds(30);
            System.out.println("   Gateway config created\n");

            // Initialize gateway
            System.out.println("2. Creating gateway...");
            RiGateway gateway = new RiGateway(config);
            System.out.println("   Gateway created\n");

            // Display gateway configuration
            System.out.println("3. Gateway configuration:");
            System.out.println("   Host: " + gateway.getConfig().getHost());
            System.out.println("   Port: " + gateway.getConfig().getPort());
            System.out.println("   Rate limiting: " + gateway.getConfig().isEnableRateLimiting());
            System.out.println("   Circuit breaker: " + gateway.getConfig().isEnableCircuitBreaker());
            System.out.println("   Max request size: " + gateway.getConfig().getMaxRequestSizeMb() + " MB");
            System.out.println("   Timeout: " + gateway.getConfig().getTimeoutSeconds() + " seconds\n");

            // Cleanup
            gateway.close();

            System.out.println("=== Gateway Example Completed ===");

        } catch (RiError e) {
            System.err.println("Ri Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
}
