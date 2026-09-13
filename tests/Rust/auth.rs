//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
//! Tests for the Ri authentication system aligned with the current API:
//! JWT claims/validation options, permission and role structures,
//! and session lifecycle (creation, timeout, touch, extend).

use ri::auth::{RiAuthConfig, RiJWTClaims, RiJWTValidationOptions, RiPermission, RiRole, RiSession};
use std::collections::HashSet;
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
    assert!(!config.jwt_secret.is_empty());
}

#[test]
/// Tests RiAuthConfig field mutation on a default config.
fn test_auth_config_custom() {
    let mut config = RiAuthConfig::default();
    config.enabled = false;
    config.jwt_secret = "test-secret".to_string();
    config.jwt_expiry_secs = 7200;
    config.session_timeout_secs = 172800;

    assert!(!config.enabled);
    assert_eq!(config.jwt_secret, "test-secret");
    assert_eq!(config.jwt_expiry_secs, 7200);
    assert_eq!(config.session_timeout_secs, 172800);
}

#[test]
/// Tests RiJWTClaims creation and fields (sub/exp/iat/roles/permissions).
fn test_jwt_claims_creation() {
    let claims = RiJWTClaims {
        sub: "user-123".to_string(),
        exp: 2000000,
        iat: 1000000,
        roles: vec!["admin".to_string(), "user".to_string()],
        permissions: vec!["read".to_string(), "write".to_string()],
    };

    assert_eq!(claims.sub, "user-123");
    assert_eq!(claims.exp, 2000000);
    assert_eq!(claims.iat, 1000000);
    assert_eq!(claims.roles.len(), 2);
    assert_eq!(claims.permissions.len(), 2);
}

#[test]
/// Tests RiJWTClaims with minimal fields.
fn test_jwt_claims_minimal() {
    let claims = RiJWTClaims {
        sub: "user-456".to_string(),
        exp: 0,
        iat: 0,
        roles: vec![],
        permissions: vec![],
    };

    assert_eq!(claims.sub, "user-456");
    assert!(claims.roles.is_empty());
    assert!(claims.permissions.is_empty());
}

#[test]
/// Tests RiJWTValidationOptions default values.
fn test_jwt_validation_options_default() {
    let options = RiJWTValidationOptions::default();
    assert!(options.validate_exp);
    assert!(options.validate_iat);
    assert!(options.required_roles.is_empty());
    assert!(options.required_permissions.is_empty());
}

#[test]
/// Tests RiJWTValidationOptions custom configuration.
fn test_jwt_validation_options_custom() {
    let options = RiJWTValidationOptions {
        validate_exp: false,
        validate_iat: false,
        required_roles: vec!["admin".to_string()],
        required_permissions: vec!["read:device".to_string()],
    };

    assert!(!options.validate_exp);
    assert!(!options.validate_iat);
    assert_eq!(options.required_roles.len(), 1);
    assert_eq!(options.required_permissions.len(), 1);
}

#[test]
/// Tests RiPermission field structure (resource:action convention).
fn test_permission_structure() {
    let permission = RiPermission {
        id: "read:posts".to_string(),
        name: "Read Posts".to_string(),
        description: "Read access to posts".to_string(),
        resource: "posts".to_string(),
        action: "read".to_string(),
    };

    assert_eq!(permission.id, "read:posts");
    assert_eq!(permission.resource, "posts");
    assert_eq!(permission.action, "read");
    assert_eq!(permission.description, "Read access to posts");
}

#[test]
/// Tests RiRole creation and permission management.
fn test_role_creation() {
    let mut perms: HashSet<String> = HashSet::new();
    perms.insert("read:*".to_string());
    perms.insert("write:*".to_string());

    let mut role = RiRole::new(
        "admin".to_string(),
        "Administrator".to_string(),
        "Administrator role".to_string(),
        perms,
    );

    assert_eq!(role.name, "Administrator");
    assert!(!role.is_system);
    assert_eq!(role.permissions.len(), 2);
    assert!(role.has_permission("read:*"));
    assert!(role.has_permission("write:*"));

    role.add_permission("delete:*".to_string());
    assert!(role.has_permission("delete:*"));
    assert_eq!(role.permissions.len(), 3);

    role.remove_permission("delete:*");
    assert!(!role.has_permission("delete:*"));
    assert_eq!(role.permissions.len(), 2);
}

#[test]
/// Tests RiSession creation and basic properties.
fn test_session_creation() {
    let session = RiSession::new("user-456".to_string(), 3600, None, None);

    assert!(!session.id.is_empty());
    assert_eq!(session.user_id, "user-456");
    assert!(!session.is_expired());
}

#[test]
/// Tests RiSession timeout and expiration.
fn test_session_timeout() {
    let mut session = RiSession::new("user-123".to_string(), 3600, None, None);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Force expiration by moving expires_at into the past.
    session.expires_at = now - 100;
    assert!(session.is_expired());
}

#[test]
/// Tests RiSession touch and extend operations.
fn test_session_touch_extend() {
    let mut session = RiSession::new("user-xyz".to_string(), 60, None, None);
    let initial_accessed = session.last_accessed;

    std::thread::sleep(Duration::from_secs(1));
    session.touch();
    assert!(session.last_accessed >= initial_accessed);

    session.extend(3600);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    assert!(session.expires_at >= now + 3500);
}

#[test]
/// Tests RiSession data storage operations.
fn test_session_data() {
    let mut session = RiSession::new("user-xyz".to_string(), 60, None, None);

    assert!(session.get_data("theme").is_none());
    session.set_data("theme".to_string(), "dark".to_string());
    assert_eq!(session.get_data("theme"), Some(&"dark".to_string()));
}

#[test]
/// Tests RiSessionManager basic operations (async via tokio runtime).
fn test_session_manager() {
    use ri::auth::RiSessionManager;

    let rt = tokio::runtime::Runtime::new().expect("test runtime");
    rt.block_on(async {
        let manager = RiSessionManager::new(3600);
        let session_id = manager
            .create_session("user-123".to_string(), None, None)
            .await
            .expect("create session");

        assert!(!session_id.is_empty());
        assert!(manager.get_session(&session_id).await.expect("get").is_some());
        manager.destroy_session(&session_id).await.expect("destroy");
        assert!(manager.get_session(&session_id).await.expect("get again").is_none());
    });
}
