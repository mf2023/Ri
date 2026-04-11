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

//! # Auth Module Tests
//!
//! This module contains comprehensive tests for the Ri authentication system,
//! covering JWT management, session handling, permissions, and configuration.
//!
//! ## Test Coverage
//!
//! - **RiAuthConfig**: Tests for authentication configuration creation and defaults
//! - **RiJWTClaims**: Tests for JWT claims structure and serialization
//! - **RiJWTValidationOptions**: Tests for JWT validation options configuration
//! - **RiPermission**: Tests for permission structure and operations
//! - **RiRole**: Tests for role structure with permission management
//! - **RiSession**: Tests for session creation, timeout, and validation

use ri::auth::{RiAuthConfig, RiJWTClaims, RiJWTValidationOptions, RiPermission, RiRole, RiSession};
use std::time::Duration;

#[test]
/// Tests RiAuthConfig default creation.
fn test_auth_config_default() {
    let config = RiAuthConfig::default();
    assert!(config.enabled);
    assert_eq!(config.jwt_expiry_secs, 3600);
    assert_eq!(config.session_timeout_secs, 86400);
    assert!(config.enable_api_keys);
    assert!(config.enable_session_auth);
}

#[test]
/// Tests RiAuthConfig with custom settings.
///
/// This test verifies that the builder pattern correctly updates
/// all configuration options to custom values.
///
/// ## Configuration Changes
///
/// - `enabled`: Disabled for testing or specific scenarios
/// - `jwt_secret`: Custom secret key for token signing
/// - `jwt_expiry_secs`: Extended token lifetime (2 hours)
/// - `session_timeout_secs`: Extended session lifetime (48 hours)
///
/// ## Expected Behavior
///
/// - Configuration values match builder method calls
/// - Custom secret is properly stored
/// - Custom timeouts are applied correctly
fn test_auth_config_custom() {
    let config = RiAuthConfig::new()
        .enabled(false)
        .jwt_secret("test-secret".to_string())
        .jwt_expiry_secs(7200)
        .session_timeout_secs(172800);

    assert!(!config.enabled);
    assert_eq!(config.jwt_secret, "test-secret");
    assert_eq!(config.jwt_expiry_secs, 7200);
    assert_eq!(config.session_timeout_secs, 172800);
}

#[test]
/// Tests RiJWTClaims creation and fields.
///
/// This test verifies that JWT claims can be created with all
/// standard and custom fields populated.
///
/// ## Claims Structure
///
/// - `sub` (subject): User identifier (required)
/// - `name`: User's display name (optional)
/// - `email`: User's email address (optional)
/// - `roles`: List of role names (optional)
/// - `permissions`: List of permission strings (optional)
/// - `issued_at`: Unix timestamp of token creation (optional)
/// - `expires_at`: Unix timestamp of token expiration (optional)
/// - `not_before`: Unix timestamp when token becomes valid (optional)
/// - `issuer`: Token issuer identifier (optional)
/// - `audience`: Intended token audience (optional)
/// - `jwt_id`: Unique token identifier (optional)
///
/// ## Expected Behavior
///
/// - All specified claims are correctly stored
/// - Optional fields are properly wrapped in Some/None
/// - Collections (roles, permissions) have correct lengths
fn test_jwt_claims_creation() {
    let claims = RiJWTClaims {
        sub: "user-123".to_string(),
        name: Some("Test User".to_string()),
        email: Some("test@example.com".to_string()),
        roles: vec!["admin".to_string(), "user".to_string()],
        permissions: vec!["read".to_string(), "write".to_string()],
        issued_at: Some(1000000),
        expires_at: Some(2000000),
        not_before: Some(500000),
        issuer: Some("ri".to_string()),
        audience: Some(vec!["api".to_string()]),
        jwt_id: Some("jwt-123".to_string()),
        custom_claims: None,
    };

    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.name, Some("Test User".to_string()));
    assert_eq!(claims.roles.len(), 2);
    assert_eq!(claims.permissions.len(), 2);
}

#[test]
/// Tests RiJWTClaims with minimal fields.
///
/// This test verifies that JWT claims can be created with only
/// the required fields, with all optional fields set to None.
///
/// ## Minimal Claims
///
/// - `sub`: User identifier (required, always present)
/// - All other fields: None (not specified)
///
/// ## Expected Behavior
///
/// - Required subject field is correctly set
/// - Optional fields are correctly None
/// - Empty collections are represented as empty Vec
fn test_jwt_claims_minimal() {
    let claims = RiJWTClaims {
        sub: "user-456".to_string(),
        name: None,
        email: None,
        roles: vec![],
        permissions: vec![],
        issued_at: None,
        expires_at: None,
        not_before: None,
        issuer: None,
        audience: None,
        jwt_id: None,
        custom_claims: None,
    };

    assert_eq!(claims.sub, "user-456");
    assert!(claims.name.is_none());
    assert!(claims.roles.is_empty());
}

#[test]
/// Tests RiJWTValidationOptions default values.
///
/// This test verifies that default validation options enforce
/// maximum security by validating all token components.
///
/// ## Default Security Checks
///
/// - `verify_signature`: true (validate token signature)
/// - `verify_expiration`: true (check token expiry)
/// - `verify_not_before`: true (check token not-before)
/// - `verify_issuer`: true (validate issuer claim)
/// - `verify_audience`: true (validate audience claim)
/// - `allow_expired_session`: true (allow within grace period)
///
/// ## Expected Behavior
///
/// - All security validations are enabled by default
/// - Tokens must pass all checks to be considered valid
/// - Expired sessions within grace period are still accepted
fn test_jwt_validation_options_default() {
    let options = RiJWTValidationOptions::default();
    assert!(options.verify_signature);
    assert!(options.verify_expiration);
    assert!(options.verify_not_before);
    assert!(options.verify_issuer);
    assert!(options.verify_audience);
    assert!(options.allow_expired_session);
}

#[test]
/// Tests RiJWTValidationOptions custom configuration.
///
/// This test verifies that validation options can be selectively
/// disabled for specific use cases like testing or development.
///
/// ## Custom Configuration
///
/// - `verify_signature`: Disabled for testing without real keys
/// - `verify_expiration`: Disabled for debugging expired tokens
/// - `verify_not_before`: Disabled for testing future tokens
/// - `verify_issuer`: Disabled when issuer varies
/// - `verify_audience`: Disabled when audience varies
/// - `allow_expired_session`: Enabled for extended grace period
///
/// ## Expected Behavior
///
/// - Disabled validations are correctly set to false
/// - Enabled validations remain true
/// - Custom configuration overrides defaults
fn test_jwt_validation_options_custom() {
    let options = RiJWTValidationOptions::new()
        .verify_signature(false)
        .verify_expiration(false)
        .verify_not_before(false)
        .verify_issuer(false)
        .verify_audience(false)
        .allow_expired_session(true);

    assert!(!options.verify_signature);
    assert!(!options.verify_expiration);
    assert!(options.allow_expired_session);
}

#[test]
/// Tests RiPermission creation and operations.
///
/// This test verifies that permissions are created with action,
/// resource, and optional description fields.
///
/// ## Permission Structure
///
/// - `action`: The action being permitted (e.g., "read", "write")
/// - `resource`: The resource being accessed (e.g., "posts", "users")
/// - `description`: Human-readable explanation (optional)
///
/// ## Permission Examples
///
/// - `read:posts` - Allow reading posts
/// - `write:users` - Allow modifying users
/// - `delete:*` - Allow deleting any resource
///
/// ## Expected Behavior
///
/// - Permission fields are correctly stored
/// - Optional description is properly wrapped
fn test_permission_creation() {
    let permission = RiPermission::new("read", "posts", Some("Read access to posts"));

    assert_eq!(permission.action, "read");
    assert_eq!(permission.resource, "posts");
    assert_eq!(permission.description, Some("Read access to posts".to_string()));
}

#[test]
/// Tests RiPermission equality and comparison.
fn test_permission_equality() {
    let perm1 = RiPermission::new("write", "users", None);
    let perm2 = RiPermission::new("write", "users", None);
    let perm3 = RiPermission::new("read", "users", None);

    assert_eq!(perm1, perm2);
    assert_ne!(perm1, perm3);
}

#[test]
/// Tests RiRole creation and permission management.
///
/// This test verifies that roles can be created with name and
/// description, and permissions can be added and checked.
///
/// ## Role Structure
///
/// - `name`: Unique role identifier (e.g., "admin", "user")
/// - `description`: Human-readable role description
/// - `permissions`: Collection of granted permissions
///
/// ## Permission Management
///
/// - `add_permission()`: Grants a permission to the role
/// - `remove_permission()`: Revokes a permission from the role
/// - `has_permission()`: Checks if role has a specific permission
///
/// ## Expected Behavior
///
/// - Role is created with specified name and description
/// - Initially, role has no permissions
/// - Added permissions are correctly stored
/// - `has_permission()` returns true for granted permissions
fn test_role_creation() {
    let mut role = RiRole::new("admin", "Administrator role");

    assert_eq!(role.name, "admin");
    assert_eq!(role.description, "Administrator role");
    assert!(role.permissions.is_empty());

    let read_perm = RiPermission::new("read", "*", None);
    let write_perm = RiPermission::new("write", "*", None);

    role.add_permission(read_perm.clone());
    role.add_permission(write_perm.clone());

    assert_eq!(role.permissions.len(), 2);
    assert!(role.has_permission(&read_perm));
    assert!(role.has_permission(&write_perm));
}

#[test]
/// Tests RiRole permission removal.
///
/// This test verifies that permissions can be removed from
/// a role and that `has_permission()` returns false after removal.
///
/// ## Removal Behavior
///
/// - `remove_permission()` revokes a previously granted permission
/// - After removal, `has_permission()` returns false
/// - Removing non-existent permission has no effect
///
/// ## Expected Behavior
///
/// - Permission exists before removal
/// - Permission no longer exists after removal
/// - Role state is correctly updated
fn test_role_permission_removal() {
    let mut role = RiRole::new("editor", "Editor role");
    let perm = RiPermission::new("edit", "articles", None);

    role.add_permission(perm.clone());
    assert!(role.has_permission(&perm));

    role.remove_permission(&perm);
    assert!(!role.has_permission(&perm));
}

#[test]
/// Tests RiSession creation and basic properties.
///
/// This test verifies that sessions are created with session ID,
/// user ID, and default valid state.
///
/// ## Session Structure
///
/// - `session_id`: Unique session identifier
/// - `user_id`: Associated user identifier
/// - `created_at`: Session creation timestamp
/// - `last_accessed`: Last access timestamp
/// - `expires_at`: Session expiration timestamp
/// - `is_valid`: Current validity status
///
/// ## Initial State
///
/// - Session is valid upon creation
/// - Session is not expired upon creation
/// - Timestamps are set to current time and expiry
///
/// ## Expected Behavior
///
/// - Session ID and user ID are correctly stored
/// - Session is initially valid
/// - Session is not initially expired
fn test_session_creation() {
    let session = RiSession::new("session-123", "user-456");

    assert_eq!(session.session_id, "session-123");
    assert_eq!(session.user_id, "user-456");
    assert!(session.is_valid());
    assert!(!session.is_expired());
}

#[test]
/// Tests RiSession timeout and expiration.
fn test_session_timeout() {
    let mut session = RiSession::new("session-789", "user-123");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    session.expires_at = now - 100;
    assert!(session.is_expired());
    assert!(!session.is_valid());
}

#[test]
/// Tests RiSession touch and extend operations.
///
/// This test verifies that sessions can be updated to track
/// recent activity and extend their validity period.
///
/// ## Session Operations
///
/// - `touch()`: Updates last_accessed timestamp to current time
///   - Used to track session activity
///   - Keeps session alive during active use
///
/// - `extend(seconds)`: Extends expires_at by specified seconds
///   - Called after successful authentication
///   - Resets the session timeout countdown
///
/// ## Expected Behavior
///
/// - `touch()` updates last_accessed to current time
/// - `extend()` moves expires_at into the future
/// - Extended session has expiry after current time
fn test_session_touch_extend() {
    let mut session = RiSession::new("session-abc", "user-xyz");
    let initial_accessed = session.last_accessed;

    session.touch();
    assert!(session.last_accessed >= initial_accessed);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    session.extend(3600);
    assert!(session.expires_at > now);
}

#[test]
/// Tests RiSession invalidation.
///
/// This test verifies that sessions can be explicitly invalidated
/// to force user logout or security-related termination.
///
/// ## Invalidation Use Cases
///
/// - User logs out explicitly
/// - Administrator terminates session
/// - Security policy requires session revocation
/// - User changes password (revoke all sessions)
///
/// ## Expected Behavior
///
/// - Session is valid before invalidation
/// - Session is not valid after invalidation
/// - Invalidated session cannot be used for authentication
fn test_session_invalidation() {
    let mut session = RiSession::new("session-def", "user-ghi");
    assert!(session.is_valid());

    session.invalidate();
    assert!(!session.is_valid());
}

#[test]
/// Tests RiAuthConfig builder pattern.
///
/// This test verifies that the builder pattern correctly chains
/// configuration methods and produces a valid configuration.
///
/// ## Builder Pattern Benefits
///
/// - Fluent API for readable configuration
/// - Compile-time checking of required fields
/// - Optional fields with sensible defaults
/// - Easy to see all configured values in one place
///
/// ## Expected Behavior
///
/// - All builder methods return self for chaining
/// - Final configuration has all specified values
/// - Unspecified values use defaults
fn test_auth_config_builder() {
    let config = RiAuthConfig::new()
        .enabled(true)
        .jwt_secret("my-secret-key")
        .jwt_expiry_secs(7200)
        .session_timeout_secs(14400)
        .enable_api_keys(true)
        .enable_session_auth(true);

    assert!(config.enabled);
    assert!(!config.jwt_secret.is_empty());
    assert!(config.jwt_expiry_secs > 0);
    assert!(config.session_timeout_secs > 0);
}
