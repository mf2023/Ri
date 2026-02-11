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

use crate::auth::{DMSCAuthConfig, DMSCJWTManager, DMSCSessionManager, DMSCPermissionManager, DMSCOAuthManager};
use std::ffi::{c_char, c_int, CString};

c_wrapper!(CDMSCAuthConfig, DMSCAuthConfig);
c_wrapper!(CDMSCJWTManager, DMSCJWTManager);
c_wrapper!(CDMSCSessionManager, DMSCSessionManager);
c_wrapper!(CDMSCPermissionManager, DMSCPermissionManager);
c_wrapper!(CDMSCOAuthManager, DMSCOAuthManager);

// DMSCAuthConfig constructors and destructors
c_constructor!(dmsc_auth_config_new, CDMSCAuthConfig, DMSCAuthConfig, DMSCAuthConfig::default());
c_destructor!(dmsc_auth_config_free, CDMSCAuthConfig);

// DMSCJWTManager constructors and destructors
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
c_destructor!(dmsc_jwt_manager_free, CDMSCJWTManager);

// DMSCSessionManager constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_session_manager_new(timeout_secs: u64) -> *mut CDMSCSessionManager {
    let manager = DMSCSessionManager::new(timeout_secs);
    Box::into_raw(Box::new(CDMSCSessionManager::new(manager)))
}
c_destructor!(dmsc_session_manager_free, CDMSCSessionManager);

// DMSCPermissionManager constructors and destructors
#[no_mangle]
pub extern "C" fn dmsc_permission_manager_new() -> *mut CDMSCPermissionManager {
    let manager = DMSCPermissionManager::new();
    Box::into_raw(Box::new(CDMSCPermissionManager::new(manager)))
}
c_destructor!(dmsc_permission_manager_free, CDMSCPermissionManager);
