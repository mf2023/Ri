//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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
//! This module provides C language bindings for DMSC's authentication and authorization
//! subsystem. The authentication module is responsible for handling user authentication,
//! session management, permission verification, and OAuth authentication flows. This C API
//! enables C/C++ applications to integrate with DMSC's security features without requiring
//! Rust runtime dependencies.
//!
//! ## Module Architecture
//!
//! The authentication module consists of five primary components:
//!
//! - **DMSCAuthConfig**: Centralized configuration container for authentication parameters.
//!   Manages JWT secret keys, session timeouts, token expiration settings, and OAuth
//!   provider configurations. This configuration object is required for initializing
//!   authentication managers and controls security policy enforcement across the system.
//!
//! - **DMSCJWTManager**: JSON Web Token (JWT) generation and validation handler.
//!   Provides token creation with custom claims, signature verification using HMAC-SHA256,
//!   expiration checking, and audience validation. The JWT manager supports both access
//!   tokens and refresh tokens with configurable expiration periods. It implements RFC 7519
//!   specification for secure stateless authentication in distributed systems.
//!
//! - **DMSCSessionManager**: Server-side session state management for stateful authentication.
//!   Maintains active user sessions in memory with configurable timeout policies. Supports
//!   session creation, validation, renewal, and invalidation. The session manager uses
//!   DashMap for thread-safe concurrent access in multi-threaded server environments.
//!
//! - **DMSCPermissionManager**: Role-based access control (RBAC) permission evaluator.
//!   Manages user roles, permissions, and resource access policies. Supports hierarchical
//!   role definitions with permission inheritance. The permission manager provides efficient
//!   permission checking for high-throughput authorization decisions.
//!
//! - **DMSCOAuthManager**: OAuth 2.0 authentication flow handler for third-party integrations.
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
//! CDMSCAuthConfig* config = dmsc_auth_config_new();
//!
//! // Create JWT manager with secret and expiration
//! CDMSCJWTManager* jwt = dmsc_jwt_manager_new("your-secret-key", 3600);
//!
//! // Generate token for authenticated user
//! const char* token = dmsc_jwt_manager_generate(jwt, "user-id", "admin");
//!
//! // Validate token on subsequent requests
//! bool valid = dmsc_jwt_manager_validate(jwt, token);
//!
//! // Cleanup resources
//! dmsc_jwt_manager_free(jwt);
//! dmsc_auth_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following core DMSC modules:
//!
//! - `crate::auth`: Rust implementation of authentication logic
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The authentication module is enabled by default with the "auth" feature flag.
//! Disable this feature to reduce binary size in deployments that do not require
//! authentication capabilities.

use crate::auth::{DMSCAuthConfig, DMSCJWTManager, DMSCSessionManager, DMSCPermissionManager, DMSCOAuthManager};
use std::ffi::c_char;

/// Opaque C wrapper structure for DMSCAuthConfig.
///
/// This structure provides C-compatible memory layout for the Rust authentication
/// configuration type. The internal implementation details are hidden from C callers,
/// ensuring ABI stability across Rust version updates. Instances must be created using
/// dmsc_auth_config_new() and freed using dmsc_auth_config_free().
///
/// # Memory Layout
///
/// The structure uses #[repr(C)] attribute to ensure consistent memory layout:
/// - Field alignment follows C ABI requirements
/// - Size is predictable for FFI boundaries
/// - No padding or alignment issues when passed between languages
///
/// # Thread Safety
///
/// This structure is not thread-safe. Concurrent access requires external synchronization.
/// Consider using thread-local storage or mutexes for multi-threaded access patterns.
c_wrapper!(CDMSCAuthConfig, DMSCAuthConfig);

/// Opaque C wrapper structure for DMSCJWTManager.
///
/// Wraps the Rust JWT management implementation providing token generation and validation
/// capabilities. The wrapper maintains ownership of the underlying JWT manager and
/// delegates all operations to the Rust implementation. Thread-safe concurrent access
/// is provided through internal synchronization primitives.
///
/// # Performance Characteristics
///
/// Token generation and validation operations have the following complexity:
/// - Token generation: O(n) where n is the number of claims
/// - Token validation: O(1) average case, O(n) worst case for expired tokens
/// - Signature verification: O(1) using constant-time comparison
///
/// # Security Considerations
///
/// The JWT manager implements constant-time comparison for signature verification to
/// prevent timing attacks. Token expiration is checked automatically during validation.
/// Use sufficiently long secret keys (minimum 256 bits) for HMAC signatures.
c_wrapper!(CDMSCJWTManager, DMSCJWTManager);

/// Opaque C wrapper structure for DMSCSessionManager.
///
/// Provides C-compatible interface to the Rust session management system. Sessions are
/// stored in memory with configurable expiration policies. The session manager handles
/// automatic cleanup of expired sessions to prevent memory leaks in long-running processes.
///
/// # Session Lifecycle
///
/// Sessions transition through the following states:
/// 1. CREATED: Session object allocated but not activated
/// 2. ACTIVE: Session has been assigned a user and is tracking activity
/// 3. IDLE: Session exists but user has not performed recent activity
/// 4. EXPIRED: Session lifetime has elapsed
/// 5. DESTROYED: Session explicitly invalidated or cleaned up
///
/// # Storage Implementation
///
/// Sessions are stored in a DashMap concurrent HashMap providing:
/// - Lock-free reads for high-performance concurrent access
/// - Fine-grained locking for writes minimizing contention
/// - Automatic rehash and load factor management
c_wrapper!(CDMSCSessionManager, DMSCSessionManager);

/// Opaque C wrapper structure for DMSCPermissionManager.
///
/// Wraps the Rust role-based access control implementation. The permission manager
/// evaluates authorization requests against defined roles and permissions. Supports
/// hierarchical roles with inherited permissions and resource-level access control.
///
/// # Permission Model
///
/// The permission system implements a flexible RBAC model:
/// - Users are assigned one or more roles
/// - Roles contain collections of permissions
/// - Permissions specify actions on resources
/// - Role hierarchies allow permission inheritance
///
/// # Evaluation Performance
///
/// Permission checks are optimized for high-throughput scenarios:
/// - Permission cache with automatic invalidation
/// - Hierarchical role resolution at role assignment time
/// - Constant-time permission lookup for common cases
c_wrapper!(CDMSCPermissionManager, DMSCPermissionManager);

/// Opaque C wrapper structure for DMSCOAuthManager.
///
/// Provides C interface to OAuth 2.0 authentication flow handling. The OAuth manager
/// implements authorization code flow, implicit flow, and client credentials flow
/// as defined in RFC 6749. Supports multiple OAuth providers with different endpoint
/// configurations.
///
/// # OAuth 2.0 Flows Supported
///
/// 1. Authorization Code Flow (for web server applications)
///    - User redirects to authorization endpoint
///    - Authorization code returned via redirect
///    - Code exchanged for access token server-side
///
/// 2. Implicit Flow (for single-page applications)
///    - User redirects to authorization endpoint
///    - Access token returned directly in URL fragment
///
/// 3. Client Credentials Flow (for machine-to-machine)
///    - Client authenticates directly
///    - Access token issued based on client credentials
///
/// # Provider Integration
///
/// The OAuth manager supports major providers including:
/// - Google OAuth 2.0
/// - Microsoft Azure AD
/// - GitHub OAuth
/// - Facebook Login
/// - Custom OAuth providers via configuration
c_wrapper!(CDMSCOAuthManager, DMSCOAuthManager);

/// Creates a new CDMSCAuthConfig instance with default configuration values.
///
/// Initializes an authentication configuration object with sensible defaults:
/// - Default JWT algorithm: HS256
/// - Default token expiration: 3600 seconds (1 hour)
/// - Default refresh token expiration: 86400 seconds (24 hours)
/// - Default session timeout: 1800 seconds (30 minutes)
///
/// # Returns
///
/// Pointer to newly allocated CDMSCAuthConfig on success, or NULL if memory allocation
/// fails. The returned pointer must be freed using dmsc_auth_config_free() to avoid
/// memory leaks.
///
/// # Example
///
/// ```c
/// CDMSCAuthConfig* config = dmsc_auth_config_new();
/// if (config == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
/// // Configure settings...
/// dmsc_auth_config_free(config);
/// ```
c_constructor!(dmsc_auth_config_new, CDMSCAuthConfig, DMSCAuthConfig, DMSCAuthConfig::default());

/// Frees a previously allocated CDMSCAuthConfig instance.
///
/// Releases all memory associated with the configuration object including any
/// internally allocated strings, buffers, or sub-objects. After this function returns,
/// the pointer becomes invalid and must not be used.
///
/// # Parameters
///
/// - `config`: Pointer to the CDMSCAuthConfig to free. If NULL, the function returns
///   immediately without error.
///
/// # Safety
///
/// This function is safe to call with NULL pointer. Calling with a pointer that has
/// already been freed results in undefined behavior. Double-free vulnerabilities must
/// be prevented by the caller through proper ownership tracking.
c_destructor!(dmsc_auth_config_free, CDMSCAuthConfig);

/// Creates a new CDMSCJWTManager instance with specified secret and expiration.
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
/// Pointer to newly allocated CDMSCJWTManager on success, or NULL if:
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
/// CDMSCJWTManager* jwt = dmsc_jwt_manager_new(
///     "your-256-bit-secret-key-here",
///     3600  // 1 hour expiration
/// );
/// if (jwt == NULL) {
///     // Handle initialization failure
/// }
/// ```
#[no_mangle]
pub extern "C" fn dmsc_jwt_manager_new(secret: *const c_char, expiry_secs: u64) -> *mut CDMSCJWTManager {
    if secret.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let secret_str = match std::ffi::CStr::from_ptr(secret).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        let manager = DMSCJWTManager::create(secret_str.to_string(), expiry_secs);
        Box::into_raw(Box::new(CDMSCJWTManager::new(manager)))
    }
}

/// Frees a previously allocated CDMSCJWTManager instance.
///
/// Releases all resources held by the JWT manager including any cached tokens or
/// internal state. After this function returns, the pointer becomes invalid.
///
/// # Parameters
///
/// - `manager`: Pointer to the CDMSCJWTManager to free. If NULL, the function returns
///   immediately without error.
c_destructor!(dmsc_jwt_manager_free, CDMSCJWTManager);

/// Creates a new CDMSCSessionManager instance with specified session timeout.
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
/// Pointer to newly allocated CDMSCSessionManager. Never returns NULL as the
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
pub extern "C" fn dmsc_session_manager_new(timeout_secs: u64) -> *mut CDMSCSessionManager {
    let manager = DMSCSessionManager::new(timeout_secs);
    Box::into_raw(Box::new(CDMSCSessionManager::new(manager)))
}

/// Frees a previously allocated CDMSCSessionManager instance.
///
/// # Parameters
///
/// - `manager`: Pointer to the CDMSCSessionManager to free. If NULL, returns immediately.
c_destructor!(dmsc_session_manager_free, CDMSCSessionManager);

/// Creates a new CDMSCPermissionManager instance.
///
/// Initializes an empty permission manager with default configuration. Roles and
/// permissions must be added through configuration or management APIs before use.
///
/// # Returns
///
/// Pointer to newly allocated CDMSCPermissionManager. Never returns NULL.
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
pub extern "C" fn dmsc_permission_manager_new() -> *mut CDMSCPermissionManager {
    let manager = DMSCPermissionManager::new();
    Box::into_raw(Box::new(CDMSCPermissionManager::new(manager)))
}

/// Frees a previously allocated CDMSCPermissionManager instance.
///
/// # Parameters
///
/// - `manager`: Pointer to the CDMSCPermissionManager to free. If NULL, returns immediately.
c_destructor!(dmsc_permission_manager_free, CDMSCPermissionManager);
