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

use crate::auth::{
    RiAuthConfig, RiJWTManager, RiSessionManager, RiPermissionManager, RiOAuthManager,
    RiRole,
};
use std::ffi::{c_char, c_int};
use std::collections::HashSet;

c_wrapper!(CRiAuthConfig, RiAuthConfig);
c_wrapper!(CRiJWTManager, RiJWTManager);
c_wrapper!(CRiSessionManager, RiSessionManager);
c_wrapper!(CRiPermissionManager, RiPermissionManager);
c_wrapper!(CRiOAuthManager, RiOAuthManager);

c_constructor!(ri_auth_config_new, CRiAuthConfig, RiAuthConfig, RiAuthConfig::default());
c_destructor!(ri_auth_config_free, CRiAuthConfig);

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
        let ptr = Box::into_raw(Box::new(CRiJWTManager::new(manager)));
        $crate::c::register_ptr(ptr as usize);
        ptr
    }
}

c_destructor!(ri_jwt_manager_free, CRiJWTManager);

#[no_mangle]
pub extern "C" fn ri_jwt_manager_generate(
    manager: *mut CRiJWTManager,
    user_id: *const c_char,
    roles: *const *const c_char,
    roles_count: usize,
    permissions: *const *const c_char,
    permissions_count: usize,
    out_token: *mut *mut c_char,
) -> c_int {
    if manager.is_null() || user_id.is_null() || out_token.is_null() {
        return -1;
    }

    unsafe {
        let user_id_str = match std::ffi::CStr::from_ptr(user_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let roles_vec: Vec<String> = if roles.is_null() || roles_count == 0 {
            Vec::new()
        } else {
            let roles_slice = std::slice::from_raw_parts(roles, roles_count);
            roles_slice
                .iter()
                .filter_map(|&r| {
                    if r.is_null() {
                        None
                    } else {
                        std::ffi::CStr::from_ptr(r).to_str().ok().map(|s| s.to_string())
                    }
                })
                .collect()
        };

        let permissions_vec: Vec<String> = if permissions.is_null() || permissions_count == 0 {
            Vec::new()
        } else {
            let perms_slice = std::slice::from_raw_parts(permissions, permissions_count);
            perms_slice
                .iter()
                .filter_map(|&p| {
                    if p.is_null() {
                        None
                    } else {
                        std::ffi::CStr::from_ptr(p).to_str().ok().map(|s| s.to_string())
                    }
                })
                .collect()
        };

        match (*manager).inner.generate_token(user_id_str, roles_vec, permissions_vec) {
            Ok(token) => {
                match std::ffi::CString::new(token) {
                    Ok(c_token) => {
                        *out_token = c_token.into_raw();
                        0
                    }
                    Err(_) => -4,
                }
            }
            Err(_) => -3,
        }
    }
}

#[repr(C)]
pub struct CRiJWTClaims {
    pub sub: *mut c_char,
    pub exp: u64,
    pub iat: u64,
}

#[no_mangle]
pub extern "C" fn ri_jwt_manager_validate(
    manager: *mut CRiJWTManager,
    token: *const c_char,
    out_claims: *mut CRiJWTClaims,
) -> c_int {
    if manager.is_null() || token.is_null() || out_claims.is_null() {
        return -1;
    }

    unsafe {
        let token_str = match std::ffi::CStr::from_ptr(token).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        match (*manager).inner.validate_token(token_str) {
            Ok(claims) => {
                let sub = match std::ffi::CString::new(claims.sub) {
                    Ok(s) => s.into_raw(),
                    Err(_) => return -4,
                };

                *out_claims = CRiJWTClaims {
                    sub,
                    exp: claims.exp,
                    iat: claims.iat,
                };
                0
            }
            Err(_) => -3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_jwt_claims_free(claims: *mut CRiJWTClaims) {
    if claims.is_null() {
        return;
    }

    unsafe {
        let claims = Box::from_raw(claims);
        if !claims.sub.is_null() {
            let _ = std::ffi::CString::from_raw(claims.sub);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_session_manager_new(timeout_secs: u64) -> *mut CRiSessionManager {
    let manager = RiSessionManager::new(timeout_secs);
    let ptr = Box::into_raw(Box::new(CRiSessionManager::new(manager)));
    $crate::c::register_ptr(ptr as usize);
    ptr
}

c_destructor!(ri_session_manager_free, CRiSessionManager);

#[no_mangle]
pub extern "C" fn ri_session_manager_create(
    manager: *mut CRiSessionManager,
    user_id: *const c_char,
    ip_address: *const c_char,
    user_agent: *const c_char,
    out_session_id: *mut *mut c_char,
) -> c_int {
    if manager.is_null() || user_id.is_null() || out_session_id.is_null() {
        return -1;
    }

    unsafe {
        let user_id_str = match std::ffi::CStr::from_ptr(user_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let ip_str = if ip_address.is_null() {
            None
        } else {
            std::ffi::CStr::from_ptr(ip_address).to_str().ok().map(|s| s.to_string())
        };

        let ua_str = if user_agent.is_null() {
            None
        } else {
            std::ffi::CStr::from_ptr(user_agent).to_str().ok().map(|s| s.to_string())
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result = rt.block_on(async {
            (*manager).inner.create_session(user_id_str.to_string(), ip_str, ua_str).await
        });

        match result {
            Ok(session_id) => {
                match std::ffi::CString::new(session_id) {
                    Ok(c_id) => {
                        *out_session_id = c_id.into_raw();
                        0
                    }
                    Err(_) => -5,
                }
            }
            Err(_) => -4,
        }
    }
}

#[repr(C)]
pub struct CRiSession {
    pub id: *mut c_char,
    pub user_id: *mut c_char,
    pub created_at: u64,
    pub expires_at: u64,
}

#[no_mangle]
pub extern "C" fn ri_session_manager_get(
    manager: *mut CRiSessionManager,
    session_id: *const c_char,
    out_session: *mut CRiSession,
) -> c_int {
    if manager.is_null() || session_id.is_null() || out_session.is_null() {
        return -1;
    }

    unsafe {
        let session_id_str = match std::ffi::CStr::from_ptr(session_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result = rt.block_on(async {
            (*manager).inner.get_session(session_id_str).await
        });

        match result {
            Ok(Some(session)) => {
                let id = match std::ffi::CString::new(session.id) {
                    Ok(s) => s.into_raw(),
                    Err(_) => return -5,
                };

                let user_id = match std::ffi::CString::new(session.user_id) {
                    Ok(s) => s.into_raw(),
                    Err(_) => {
                        let _ = std::ffi::CString::from_raw(id);
                        return -6;
                    }
                };

                *out_session = CRiSession {
                    id,
                    user_id,
                    created_at: session.created_at,
                    expires_at: session.expires_at,
                };
                0
            }
            Ok(None) => 1,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_session_free(session: *mut CRiSession) {
    if session.is_null() {
        return;
    }

    unsafe {
        let session = Box::from_raw(session);
        if !session.id.is_null() {
            let _ = std::ffi::CString::from_raw(session.id);
        }
        if !session.user_id.is_null() {
            let _ = std::ffi::CString::from_raw(session.user_id);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_session_manager_destroy(
    manager: *mut CRiSessionManager,
    session_id: *const c_char,
) -> c_int {
    if manager.is_null() || session_id.is_null() {
        return -1;
    }

    unsafe {
        let session_id_str = match std::ffi::CStr::from_ptr(session_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result = rt.block_on(async {
            (*manager).inner.destroy_session(session_id_str).await
        });

        match result {
            Ok(destroyed) => if destroyed { 0 } else { 1 },
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_permission_manager_new() -> *mut CRiPermissionManager {
    let manager = RiPermissionManager::new();
    let ptr = Box::into_raw(Box::new(CRiPermissionManager::new(manager)));
    $crate::c::register_ptr(ptr as usize);
    ptr
}

c_destructor!(ri_permission_manager_free, CRiPermissionManager);

#[no_mangle]
pub extern "C" fn ri_permission_manager_create_role(
    manager: *mut CRiPermissionManager,
    role_id: *const c_char,
    role_name: *const c_char,
    description: *const c_char,
    permissions: *const *const c_char,
    permissions_count: usize,
) -> c_int {
    if manager.is_null() || role_id.is_null() || role_name.is_null() {
        return -1;
    }

    unsafe {
        let role_id_str = match std::ffi::CStr::from_ptr(role_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let role_name_str = match std::ffi::CStr::from_ptr(role_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let desc_str = if description.is_null() {
            ""
        } else {
            match std::ffi::CStr::from_ptr(description).to_str() {
                Ok(s) => s,
                Err(_) => "",
            }
        };

        let perms_set: HashSet<String> = if permissions.is_null() || permissions_count == 0 {
            HashSet::new()
        } else {
            let perms_slice = std::slice::from_raw_parts(permissions, permissions_count);
            perms_slice
                .iter()
                .filter_map(|&p| {
                    if p.is_null() {
                        None
                    } else {
                        std::ffi::CStr::from_ptr(p).to_str().ok().map(|s| s.to_string())
                    }
                })
                .collect()
        };

        let role = RiRole::new(
            role_id_str.to_string(),
            role_name_str.to_string(),
            desc_str.to_string(),
            perms_set,
        );

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        let result = rt.block_on(async {
            (*manager).inner.create_role(role).await
        });

        match result {
            Ok(()) => 0,
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_permission_manager_assign_role(
    manager: *mut CRiPermissionManager,
    user_id: *const c_char,
    role_id: *const c_char,
) -> c_int {
    if manager.is_null() || user_id.is_null() || role_id.is_null() {
        return -1;
    }

    unsafe {
        let user_id_str = match std::ffi::CStr::from_ptr(user_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let role_id_str = match std::ffi::CStr::from_ptr(role_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        let result = rt.block_on(async {
            (*manager).inner.assign_role_to_user(user_id_str.to_string(), role_id_str.to_string()).await
        });

        match result {
            Ok(assigned) => if assigned { 0 } else { 1 },
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_permission_manager_has_permission(
    manager: *mut CRiPermissionManager,
    user_id: *const c_char,
    permission_id: *const c_char,
) -> c_int {
    if manager.is_null() || user_id.is_null() || permission_id.is_null() {
        return -1;
    }

    unsafe {
        let user_id_str = match std::ffi::CStr::from_ptr(user_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let perm_id_str = match std::ffi::CStr::from_ptr(permission_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        let result = rt.block_on(async {
            (*manager).inner.has_permission(user_id_str, perm_id_str).await
        });

        match result {
            Ok(has_perm) => if has_perm { 1 } else { 0 },
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_permission_manager_remove_role(
    manager: *mut CRiPermissionManager,
    user_id: *const c_char,
    role_id: *const c_char,
) -> c_int {
    if manager.is_null() || user_id.is_null() || role_id.is_null() {
        return -1;
    }

    unsafe {
        let user_id_str = match std::ffi::CStr::from_ptr(user_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let role_id_str = match std::ffi::CStr::from_ptr(role_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        let result = rt.block_on(async {
            (*manager).inner.remove_role_from_user(user_id_str, role_id_str).await
        });

        match result {
            Ok(removed) => if removed { 0 } else { 1 },
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_permission_manager_delete_role(
    manager: *mut CRiPermissionManager,
    role_id: *const c_char,
) -> c_int {
    if manager.is_null() || role_id.is_null() {
        return -1;
    }

    unsafe {
        let role_id_str = match std::ffi::CStr::from_ptr(role_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result = rt.block_on(async {
            (*manager).inner.delete_role(role_id_str).await
        });

        match result {
            Ok(deleted) => if deleted { 0 } else { 1 },
            Err(_) => -4,
        }
    }
}
