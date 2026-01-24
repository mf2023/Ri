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

//! # Auth Module Tests
//!
//! This module contains comprehensive tests for the DMSC authentication system,
//! covering JWT management, session handling, permissions, and configuration.
//!
//! ## Test Coverage
//!
//! - **DMSCAuthConfig**: Tests for authentication configuration creation and defaults
//! - **DMSCJWTClaims**: Tests for JWT claims structure and serialization
//! - **DMSCJWTValidationOptions**: Tests for JWT validation options configuration
//! - **DMSCPermission**: Tests for permission structure and operations
//! - **DMSCRole**: Tests for role structure with permission management
//! - **DMSCSession**: Tests for session creation, timeout, and validation

use dmsc::auth::{DMSCAuthConfig, DMSCJWTClaims, DMSCJWTValidationOptions, DMSCPermission, DMSCRole, DMSCSession};
use std::time::Duration;

#[test]
/// Tests DMSCAuthConfig default creation.
fn test_auth_config_default() {
    let config = DMSCAuthConfig::default();
    assert!(config.enabled);
    assert_eq!(config.jwt_expiry_secs, 3600);
    assert_eq!(config.session_timeout_secs, 86400);
    assert!(config.enable_api_keys);
    assert!(config.enable_session_auth);
}

#[test]
/// Tests DMSCAuthConfig with custom settings.
fn test_auth_config_custom() {
    let config = DMSCAuthConfig::new()
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
/// Tests DMSCJWTClaims creation and fields.
fn test_jwt_claims_creation() {
    let claims = DMSCJWTClaims {
        sub: "user-123".to_string(),
        name: Some("Test User".to_string()),
        email: Some("test@example.com".to_string()),
        roles: vec!["admin".to_string(), "user".to_string()],
        permissions: vec!["read".to_string(), "write".to_string()],
        issued_at: Some(1000000),
        expires_at: Some(2000000),
        not_before: Some(500000),
        issuer: Some("dmsc".to_string()),
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
/// Tests DMSCJWTClaims with minimal fields.
fn test_jwt_claims_minimal() {
    let claims = DMSCJWTClaims {
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
/// Tests DMSCJWTValidationOptions default values.
fn test_jwt_validation_options_default() {
    let options = DMSCJWTValidationOptions::default();
    assert!(options.verify_signature);
    assert!(options.verify_expiration);
    assert!(options.verify_not_before);
    assert!(options.verify_issuer);
    assert!(options.verify_audience);
    assert!(options.allow_expired_session);
}

#[test]
/// Tests DMSCJWTValidationOptions custom configuration.
fn test_jwt_validation_options_custom() {
    let options = DMSCJWTValidationOptions::new()
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
/// Tests DMSCPermission creation and operations.
fn test_permission_creation() {
    let permission = DMSCPermission::new("read", "posts", Some("Read access to posts"));

    assert_eq!(permission.action, "read");
    assert_eq!(permission.resource, "posts");
    assert_eq!(permission.description, Some("Read access to posts".to_string()));
}

#[test]
/// Tests DMSCPermission equality and comparison.
fn test_permission_equality() {
    let perm1 = DMSCPermission::new("write", "users", None);
    let perm2 = DMSCPermission::new("write", "users", None);
    let perm3 = DMSCPermission::new("read", "users", None);

    assert_eq!(perm1, perm2);
    assert_ne!(perm1, perm3);
}

#[test]
/// Tests DMSCRole creation and permission management.
fn test_role_creation() {
    let mut role = DMSCRole::new("admin", "Administrator role");

    assert_eq!(role.name, "admin");
    assert_eq!(role.description, "Administrator role");
    assert!(role.permissions.is_empty());

    let read_perm = DMSCPermission::new("read", "*", None);
    let write_perm = DMSCPermission::new("write", "*", None);

    role.add_permission(read_perm.clone());
    role.add_permission(write_perm.clone());

    assert_eq!(role.permissions.len(), 2);
    assert!(role.has_permission(&read_perm));
    assert!(role.has_permission(&write_perm));
}

#[test]
/// Tests DMSCRole permission removal.
fn test_role_permission_removal() {
    let mut role = DMSCRole::new("editor", "Editor role");
    let perm = DMSCPermission::new("edit", "articles", None);

    role.add_permission(perm.clone());
    assert!(role.has_permission(&perm));

    role.remove_permission(&perm);
    assert!(!role.has_permission(&perm));
}

#[test]
/// Tests DMSCSession creation and basic properties.
fn test_session_creation() {
    let session = DMSCSession::new("session-123", "user-456");

    assert_eq!(session.session_id, "session-123");
    assert_eq!(session.user_id, "user-456");
    assert!(session.is_valid());
    assert!(!session.is_expired());
}

#[test]
/// Tests DMSCSession timeout and expiration.
fn test_session_timeout() {
    let mut session = DMSCSession::new("session-789", "user-123");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    session.expires_at = now - 100;
    assert!(session.is_expired());
    assert!(!session.is_valid());
}

#[test]
/// Tests DMSCSession touch and extend operations.
fn test_session_touch_extend() {
    let mut session = DMSCSession::new("session-abc", "user-xyz");
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
/// Tests DMSCSession invalidation.
fn test_session_invalidation() {
    let mut session = DMSCSession::new("session-def", "user-ghi");
    assert!(session.is_valid());

    session.invalidate();
    assert!(!session.is_valid());
}

#[test]
/// Tests DMSCAuthConfig builder pattern.
fn test_auth_config_builder() {
    let config = DMSCAuthConfig::new()
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
