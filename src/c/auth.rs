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

//! # Auth Module C API
//!
//! This module provides C language bindings for Ri's authentication and authorization
//! subsystem. The authentication module is responsible for handling user authentication,
//! session management, permission verification, and OAuth authentication flows. This C API
//! enables C/C++ applications to integrate with Ri's security features without requiring
//! Rust runtime dependencies.
//!
//! ## Module Architecture
//!
//! The authentication module consists of five primary components:
//!
//! - **RiAuthConfig**: Centralized configuration container for authentication parameters.
//!   Manages JWT secret keys, session timeouts, token expiration settings, and OAuth
//!   provider configurations. This configuration object is required for initializing
//!   authentication managers and controls security policy enforcement across the system.
//!
//! - **RiJWTManager**: JSON Web Token (JWT) generation and validation handler.
//!   Provides token creation with custom claims, signature verification using HMAC-SHA256,
//!   expiration checking, and audience validation. The JWT manager supports both access
//!   tokens and refresh tokens with configurable expiration periods. It implements RFC 7519
//!   specification for secure stateless authentication in distributed systems.
//!
//! - **RiSessionManager**: Server-side session state management for stateful authentication.
//!   Maintains active user sessions in memory with configurable timeout policies. Supports
//!   session creation, validation, renewal, and invalidation. The session manager uses
//!   DashMap for thread-safe concurrent access in multi-threaded server environments.
//!
//! - **RiPermissionManager**: Role-based access control (RBAC) permission evaluator.
//!   Manages user roles, permissions, and resource access policies. Supports hierarchical
//!   role definitions with permission inheritance. The permission manager provides efficient
//!   permission checking for high-throughput authorization decisions.
//!
//! - **RiOAuthManager**: OAuth 2.0 authentication flow handler for third-party integrations.
//!   Implements authorization code flow for web applications, implicit flow for single-page
//!   applications, and client credentials flow for machine-to-machine communication. Supports
//!   multiple OAuth providers with configurable redirect URIs and scope requirements.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management. The caller is
//! responsible for freeing allocated objects using the provided destructor functions.
//! Objects must not be used after being freed to prevent use-after-free vulnerabilities.
//! Null pointer checks must be performed before accessing any object methods or fields.
//!
//! ## Thread Safety
//!
//! All underlying Rust implementations use synchronization primitives appropriate for
//! concurrent access. The C API itself is not thread-safe; callers must implement their
//! own synchronization when accessing objects from multiple threads simultaneously.
//!
//! ## Error Handling
//!
//! Functions return null pointers or error codes (-1) to indicate failure conditions.
//! Callers should check return values and handle errors appropriately. Memory allocation
//! failures and invalid arguments are the primary error conditions.
//!
//! ## Usage Example
//!
//! ```c
//! // Create authentication configuration
//! CRiAuthConfig* config = ri_auth_config_new();
//!
//! // Create JWT manager with secret and expiration
//! CRiJWTManager* jwt = ri_jwt_manager_new("your-secret-key", 3600);
//!
//! // Generate token for authenticated user
//! const char* token = ri_jwt_manager_generate(jwt, "user-id", "admin");
//!
//! // Validate token on subsequent requests
//! bool valid = ri_jwt_manager_validate(jwt, token);
//!
//! // Cleanup resources
//! ri_jwt_manager_free(jwt);
//! ri_auth_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following core Ri modules:
//!
//! - `crate::auth`: Rust implementation of authentication logic
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The authentication module is enabled by default with the "auth" feature flag.
//! Disable this feature to reduce binary size in deployments that do not require
//! authentication capabilities.

use crate::auth::{RiAuthConfig, RiJWTManager, RiSessionManager, RiPermissionManager, RiOAuthManager};
use std::ffi::c_char;

c_wrapper!(CRiAuthConfig, RiAuthConfig);

c_wrapper!(CRiJWTManager, RiJWTManager);

c_wrapper!(CRiSessionManager, RiSessionManager);

c_wrapper!(CRiPermissionManager, RiPermissionManager);

c_wrapper!(CRiOAuthManager, RiOAuthManager);

c_constructor!(ri_auth_config_new, CRiAuthConfig, RiAuthConfig, RiAuthConfig::default());

c_destructor!(ri_auth_config_free, CRiAuthConfig);

/// Creates a new CRiJWTManager instance with specified secret and expiration.
///
/// Initializes a JWT manager for token generation and validation. The manager uses
/// HMAC-SHA256 (HS256) algorithm for signing tokens. The secret key must be kept
/// confidential and should be at least 256 bits (32 bytes) for adequate security.
///
/// # Parameters
///
/// - `secret`: Pointer to null-terminated C string containing the JWT signing secret.
///   Must not be NULL. Empty strings are accepted but provide minimal security.
/// - `expiry_secs`: Token expiration time in seconds from issuance. Tokens will be
///   rejected as expired after this duration. Typical values range from 300 (5 minutes)
///   for sensitive operations to 86400 (24 hours) for long-lived sessions.
///
/// # Returns
///
/// Pointer to newly allocated CRiJWTManager on success, or NULL if:
/// - `secret` parameter is NULL
/// - Memory allocation fails
/// - Secret contains invalid UTF-8 sequences
///
/// # Security Considerations
///
/// The secret key should be:
/// - Generated using cryptographically secure random number generator
/// - Stored securely (environment variables, secrets manager)
/// - Rotated periodically in production environments
/// - Unique per environment (development, staging, production)
///
/// # Example
///
/// ```c
/// CRiJWTManager* jwt = ri_jwt_manager_new(
///     "your-256-bit-secret-key-here",
///     3600  // 1 hour expiration
/// );
/// if (jwt == NULL) {
///     // Handle initialization failure
/// }
/// ```
#[no_mangle]
pub extern "C" fn ri_jwt_manager_new(secret: *const c_char, expiry_secs: u64) -> *mut CRiJWTManager {
    if secret.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let secret_str = match std::ffi::CStr::from_ptr(secret).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        let manager = RiJWTManager::create(secret_str.to_string(), expiry_secs);
        Box::into_raw(Box::new(CRiJWTManager::new(manager)))
    }
}

c_destructor!(ri_jwt_manager_free, CRiJWTManager);

/// Creates a new CRiSessionManager instance with specified session timeout.
///
/// Initializes a session manager for stateful authentication sessions. Sessions track
/// authenticated user state and provide automatic timeout management for security.
///
/// # Parameters
///
/// - `timeout_secs`: Session idle timeout in seconds. Sessions are considered expired
///   if no activity occurs within this duration. The timeout is reset on each
///   authenticated request. Typical values range from 300 to 1800 seconds.
///
/// # Returns
///
/// Pointer to newly allocated CRiSessionManager. Never returns NULL as the
/// implementation uses unwrap for default configuration.
///
/// # Session Behavior
///
/// Active sessions will be invalidated after:
/// - `timeout_secs` seconds of inactivity
/// - Explicit call to session invalidation function
/// - Server shutdown or process termination
///
/// Expired sessions remain in memory until:
/// - Automatic cleanup interval runs
/// - Session count exceeds maximum capacity
/// - Manual cleanup function is called
#[no_mangle]
pub extern "C" fn ri_session_manager_new(timeout_secs: u64) -> *mut CRiSessionManager {
    let manager = RiSessionManager::new(timeout_secs);
    Box::into_raw(Box::new(CRiSessionManager::new(manager)))
}

c_destructor!(ri_session_manager_free, CRiSessionManager);

/// Creates a new CRiPermissionManager instance.
///
/// Initializes an empty permission manager with default configuration. Roles and
/// permissions must be added through configuration or management APIs before use.
///
/// # Returns
///
/// Pointer to newly allocated CRiPermissionManager. Never returns NULL.
///
/// # Initial State
///
/// A newly created permission manager:
/// - Contains no roles
/// - Has no role assignments
/// - Has no resource permissions defined
///
/// # Configuration
///
/// Before the permission manager can evaluate access, it must be configured with:
/// - Role definitions (hierarchy, permissions per role)
/// - User role assignments
/// - Resource permission mappings
#[no_mangle]
pub extern "C" fn ri_permission_manager_new() -> *mut CRiPermissionManager {
    let manager = RiPermissionManager::new();
    Box::into_raw(Box::new(CRiPermissionManager::new(manager)))
}

c_destructor!(ri_permission_manager_free, CRiPermissionManager);
