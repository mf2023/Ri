//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # Ri Authentication Module Example
//!
//! This example demonstrates how to use the authentication module in Ri,
//! including JWT token generation, validation, and session management.
//!
//! ## Running this Example
//!
//! ```bash
//! cargo run --example auth --features auth
//! ```
//!
//! ## Features Demonstrated
//!
//! - JWT token creation and validation
//! - Session management using session IDs
//! - Token revocation
//! - Permission-based access control

use ri::auth::{RiAuthModule, RiAuthConfig};
use ri::core::{RiResult, RiError};

/// Main entry point for the authentication module example.
///
/// This function demonstrates the complete authentication workflow including:
/// - Configuration of authentication settings with JWT and session parameters
/// - JWT token generation with user roles and permissions
/// - Token validation and claims extraction
/// - Session creation with user and connection metadata
/// - Session retrieval and expiration checking
/// - Session extension and permission verification
///
/// The example creates a complete authentication scenario showing how
/// Ri handles user authentication, session tracking, and access control
/// in a Rust async runtime environment.
fn main() -> RiResult<()> {
    println!("=== Ri Authentication Module Example ===\n");

    // Configuration Setup: Create authentication configuration with security settings
    // - enabled: Enable/disable authentication module (true = authentication required)
    // - jwt_secret: Secret key for JWT token signing (use secure secret in production)
    // - jwt_expiry_secs: Token validity duration in seconds (3600 = 1 hour)
    // - session_timeout_secs: Session timeout duration (28800 = 8 hours)
    // - oauth_providers: List of OAuth providers for social login (empty for local auth)
    // - enable_api_keys: Enable API key-based authentication for service-to-service calls
    // - enable_session_auth: Enable session-based authentication for web clients
    let auth_config = RiAuthConfig {
        enabled: true,
        jwt_secret: "your-secret-key-here".to_string(),
        jwt_expiry_secs: 3600,
        session_timeout_secs: 28800,
        oauth_providers: vec![],
        enable_api_keys: true,
        enable_session_auth: true,
    };

    // Create async runtime for handling asynchronous operations
    // tokio::runtime::Runtime provides the async executor for.await operations
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    // Execute async authentication operations within the runtime
    rt.block_on(async {
        // Module Initialization: Create authentication module instance with configuration
        // The module provides JWT management and session management capabilities
        let auth_module = RiAuthModule::new(auth_config).await;
        
        // Get JWT manager for token operations (generation and validation)
        // JWT manager handles token lifecycle: create, validate, refresh, revoke
        let jwt_manager = auth_module.jwt_manager();

        // Token Operations: Generate and validate JWT tokens
        // Step 1: Generate a JWT token for a user with specific roles and permissions
        // Parameters:
        // - user_id: Subject identifier (typically username or user ID)
        // - roles: Vector of role names assigned to the user (e.g., "admin", "user")
        // - permissions: Vector of permission strings (e.g., "read:data", "write:data")
        // Returns: Signed JWT token string
        println!("1. Generating JWT token for user 'user123'...");
        let token = jwt_manager.generate_token(
            "user123",
            vec!["admin".to_string(), "user".to_string()],
            vec!["read:data".to_string(), "write:data".to_string()],
        )?;
        println!("   Token generated: {}...\n", &token[..50]);

        // Step 2: Validate the generated JWT token
        // Validates token signature, expiration, and claims
        // Returns: Claims struct containing subject, expiration, roles, permissions
        // Throws: RiError if token is invalid, expired, or signature mismatch
        println!("2. Validating JWT token...");
        let claims = jwt_manager.validate_token(&token)?;
        println!("   Token valid for user: {}", claims.sub);
        println!("   Token expires at: {:?}", claims.exp);
        println!("   Token roles: {:?}\n", claims.roles);

        // Session Management: Create and manage user sessions
        // Sessions provide stateful authentication tracking separate from JWT
        println!("3. Creating user session...");
        let session_manager = auth_module.session_manager();
        
        // Create a new session for user with connection metadata
        // Parameters:
        // - user_id: Identifier of the authenticated user
        // - ip_address: Option<Client IP address> for security tracking and auditing
        // - user_agent: Option<Browser/client user agent> for device tracking
        // Returns: Unique session ID string for later retrieval
        let session_id = session_manager.write().await.create_session(
            "user123".to_string(),
            Some("192.168.1.100".to_string()),
            Some("Mozilla/5.0".to_string()),
        ).await?;
        println!("   Session created with ID: {}\n", session_id);

        // Retrieve session details using session ID
        // Used to validate existing sessions and get user information
        println!("4. Retrieving session by ID...");
        let session = session_manager.read().await.get_session(&session_id).await?;
        if let Some(session_ref) = session {
            println!("   Session found for user: {}", session_ref.user_id);
            println!("   Session IP: {:?}", session_ref.ip_address);
            println!("   Session expires at: {}\n", session_ref.expires_at);

            // Check if session is still valid (not expired)
            // is_expired() compares current time against session expiration
            if !session_ref.is_expired() {
                println!("5. Session is active and not expired\n");
            }
        }

        // Extend session timeout to prevent premature logout
        // Resets the session expiration timer to maintain active session
        println!("6. Extending session...");
        session_manager.write().await.extend_session(&session_id).await?;
        println!("   Session extended successfully\n");

        // Permission Verification: Check user access rights
        // Demonstrates role-based access control (RBAC) implementation
        println!("7. Checking permissions...");
        let has_admin = claims.roles.contains(&"admin".to_string());
        let has_superuser = claims.roles.contains(&"superuser".to_string());
        println!("   Has 'admin' role: {}", has_admin);
        println!("   Has 'superuser' role: {}\n", has_superuser);

        println!("=== Authentication Example Completed ===");
        Ok::<(), RiError>(())
    })
}
