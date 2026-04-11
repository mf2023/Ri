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

/**
 * Ri Auth Module Example for C.
 *
 * This example demonstrates how to use the Ri authentication module
 * for JWT token generation and validation.
 */
int main(void) {
    printf("=== Ri Auth Module Example - C ===\n\n");

    int passed = 0;
    int failed = 0;

    /* Create authentication configuration */
    printf("1. Creating authentication configuration...\n");
    RiAuthConfig* config = ri_auth_config_new();
    if (config == NULL) {
        printf("   [FAIL] Failed to create auth config\n");
        return 1;
    }

    ri_auth_config_set_jwt_secret(config, "your-secret-key-here");
    ri_auth_config_set_jwt_expiry_secs(config, 3600);
    ri_auth_config_set_session_timeout_secs(config, 86400);
    printf("   Auth config created\n\n");

    /* Initialize auth module */
    printf("2. Creating authentication module...\n");
    RiAuthModule* auth_module = ri_auth_module_new(config);
    if (auth_module == NULL) {
        printf("   [FAIL] Failed to create auth module\n");
        ri_auth_config_free(config);
        return 1;
    }
    printf("   Auth module created\n\n");

    /* Generate JWT token */
    printf("3. Generating JWT token...\n");
    const char* user_id = "user123";
    const char* roles[] = {"admin", "user"};
    const char* permissions[] = {"read:data", "write:data"};

    char* token = ri_auth_module_generate_token(
        auth_module,
        user_id,
        roles,
        2,
        permissions,
        2
    );

    if (token != NULL) {
        printf("   Generated token: %.50s...\n\n", token);
        passed++;
    } else {
        printf("   [FAIL] Failed to generate token\n\n");
        failed++;
    }

    /* Validate JWT token */
    printf("4. Validating JWT token...\n");
    if (token != NULL) {
        bool is_valid = ri_auth_module_validate_token(auth_module, token);
        printf("   Token is valid: %s\n\n", is_valid ? "true" : "false");
        passed++;
        ri_string_free(token);
    } else {
        printf("   [SKIP] No token to validate\n\n");
    }

    /* Check auth module properties */
    printf("5. Auth module properties:\n");
    printf("   Enabled: %s\n", ri_auth_module_is_enabled(auth_module) ? "true" : "false");
    printf("   JWT expiry: %llu seconds\n", ri_auth_module_get_jwt_expiry_secs(auth_module));
    printf("   Session timeout: %llu seconds\n", ri_auth_module_get_session_timeout_secs(auth_module));
    printf("   API keys enabled: %s\n", ri_auth_module_is_api_keys_enabled(auth_module) ? "true" : "false");
    printf("   Session auth enabled: %s\n\n", ri_auth_module_is_session_auth_enabled(auth_module) ? "true" : "false");

    /* Cleanup */
    ri_auth_module_free(auth_module);
    ri_auth_config_free(config);

    printf("=== Auth Example Completed ===\n\n");

    printf("Test Summary:\n");
    printf("  Passed: %d\n", passed);
    printf("  Failed: %d\n", failed);

    return (failed > 0) ? 1 : 0;
}
