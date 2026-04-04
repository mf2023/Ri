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

import com.dunimd.dmsc.auth.DMSCAuthModule;
import com.dunimd.dmsc.auth.DMSCAuthConfig;
import com.dunimd.dmsc.auth.DMSCJWTManager;
import com.dunimd.dmsc.auth.DMSCJWTClaims;
import com.dunimd.dmsc.DMSCError;

/**
 * DMSC Auth Module Example for Java.
 *
 * This example demonstrates how to use the DMSC authentication module
 * for JWT token generation and validation.
 */
public class AuthExample {
    public static void main(String[] args) {
        System.out.println("=== DMSC Auth Module Example - Java ===\n");

        try {
            // Create authentication configuration
            System.out.println("1. Creating authentication configuration...");
            DMSCAuthConfig authConfig = new DMSCAuthConfig();
            authConfig.setEnabled(true);
            authConfig.setJwtSecret("your-secret-key-here");
            authConfig.setJwtExpirySecs(3600);
            authConfig.setSessionTimeoutSecs(86400);
            System.out.println("   Auth config created\n");

            // Initialize auth module
            System.out.println("2. Creating authentication module...");
            DMSCAuthModule authModule = new DMSCAuthModule(authConfig);
            System.out.println("   Auth module created\n");

            // Generate JWT token
            System.out.println("3. Generating JWT token...");
            String token = authModule.generateTestToken(
                "user123",
                java.util.Arrays.asList("admin", "user"),
                java.util.Arrays.asList("read:data", "write:data")
            );
            System.out.println("   Generated token: " + token.substring(0, Math.min(50, token.length())) + "...\n");

            // Validate JWT token
            System.out.println("4. Validating JWT token...");
            boolean isValid = authModule.validateJwtToken(token);
            System.out.println("   Token is valid: " + isValid + "\n");

            // Check auth module properties
            System.out.println("5. Auth module properties:");
            System.out.println("   Enabled: " + authModule.isEnabled());
            System.out.println("   JWT expiry: " + authModule.getJwtExpirySecs() + " seconds");
            System.out.println("   Session timeout: " + authModule.getSessionTimeoutSecs() + " seconds");
            System.out.println("   API keys enabled: " + authModule.isApiKeysEnabled());
            System.out.println("   Session auth enabled: " + authModule.isSessionAuthEnabled() + "\n");

            // Cleanup
            authModule.close();

            System.out.println("=== Auth Example Completed ===");

        } catch (DMSCError e) {
            System.err.println("DMSC Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
            System.exit(1);
        }
    }
}
