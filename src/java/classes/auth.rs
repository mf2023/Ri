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

//! # Auth Module JNI Bindings
//!
//! JNI bindings for Ri auth classes.

use crate::auth::{
    RiAuthConfig, RiAuthModule, RiJWTClaims, RiJWTManager, RiJWTRevocationList,
    RiJWTValidationOptions, RiOAuthManager, RiOAuthProvider, RiOAuthToken, RiOAuthUserInfo,
    RiPermission, RiPermissionManager, RiRevokedTokenInfo, RiRole, RiSecurityManager, RiSession,
    RiSessionManager,
};
use crate::java::exception::check_not_null;
use jni::objects::{JClass, JObjectArray, JString};
use jni::sys::{jboolean, jint, jlong, jobjectArray, jstring};
use jni::JNIEnv;
use std::collections::HashSet;

// =============================================================================
// RiAuthConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiAuthConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiAuthConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiAuthConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAuthConfig);
        }
    }
}

// =============================================================================
// RiAuthModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiAuthModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiAuthConfig") {
        return 0;
    }

    let config = unsafe { &*(config_ptr as *const RiAuthConfig) };
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { RiAuthModule::new(config.clone()).await }) {
        Ok(module) => Box::into_raw(Box::new(module)) as jlong,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiAuthModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAuthModule);
        }
    }
}

// =============================================================================
// RiJWTClaims JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_new0(
    mut env: JNIEnv,
    _class: JClass,
    sub: JString,
    roles: JObjectArray,
    permissions: JObjectArray,
) -> jlong {
    let sub_str: String = env.get_string(&sub).expect("Failed to get sub").into();

    let roles_len = env
        .get_array_length(&roles)
        .expect("Failed to get roles length");
    let mut roles_vec = Vec::with_capacity(roles_len as usize);
    for i in 0..roles_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&roles, i) {
            let role_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get role")
                .into();
            roles_vec.push(role_str);
        }
    }

    let perms_len = env
        .get_array_length(&permissions)
        .expect("Failed to get permissions length");
    let mut perms_vec = Vec::with_capacity(perms_len as usize);
    for i in 0..perms_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&permissions, i) {
            let perm_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get permission")
                .into();
            perms_vec.push(perm_str);
        }
    }

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("System time error")
        .as_secs();

    let claims = Box::new(RiJWTClaims {
        sub: sub_str,
        exp: now + 3600,
        iat: now,
        roles: roles_vec,
        permissions: perms_vec,
    });
    Box::into_raw(claims) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_getSub0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiJWTClaims") {
        return std::ptr::null_mut();
    }

    let claims = unsafe { &*(ptr as *const RiJWTClaims) };
    env.new_string(&claims.sub)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_getExp0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiJWTClaims") {
        return 0;
    }

    let claims = unsafe { &*(ptr as *const RiJWTClaims) };
    claims.exp as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_getIat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiJWTClaims") {
        return 0;
    }

    let claims = unsafe { &*(ptr as *const RiJWTClaims) };
    claims.iat as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_getRoles0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiJWTClaims") {
        return std::ptr::null_mut();
    }

    let claims = unsafe { &*(ptr as *const RiJWTClaims) };
    let roles: Vec<String> = claims.roles.clone();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(roles.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, role) in roles.iter().enumerate() {
        let jstr = env.new_string(role).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_getPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiJWTClaims") {
        return std::ptr::null_mut();
    }

    let claims = unsafe { &*(ptr as *const RiJWTClaims) };
    let perms: Vec<String> = claims.permissions.clone();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(perms.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, perm) in perms.iter().enumerate() {
        let jstr = env.new_string(perm).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTClaims_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiJWTClaims);
        }
    }
}

// =============================================================================
// RiJWTValidationOptions JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let options = Box::new(RiJWTValidationOptions::default());
    Box::into_raw(options) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_isValidateExp0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return 0;
    }

    let options = unsafe { &*(ptr as *const RiJWTValidationOptions) };
    if options.validate_exp {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_setValidateExp0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    validate: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return;
    }

    let options = unsafe { &mut *(ptr as *mut RiJWTValidationOptions) };
    options.validate_exp = validate != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_isValidateIat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return 0;
    }

    let options = unsafe { &*(ptr as *const RiJWTValidationOptions) };
    if options.validate_iat {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_setValidateIat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    validate: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return;
    }

    let options = unsafe { &mut *(ptr as *mut RiJWTValidationOptions) };
    options.validate_iat = validate != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_getRequiredRoles0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return std::ptr::null_mut();
    }

    let options = unsafe { &*(ptr as *const RiJWTValidationOptions) };
    let roles: Vec<String> = options.required_roles.clone();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(roles.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, role) in roles.iter().enumerate() {
        let jstr = env.new_string(role).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_setRequiredRoles0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    roles: JObjectArray,
) {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return;
    }

    let options = unsafe { &mut *(ptr as *mut RiJWTValidationOptions) };
    let roles_len = env
        .get_array_length(&roles)
        .expect("Failed to get roles length");
    let mut roles_vec = Vec::with_capacity(roles_len as usize);
    for i in 0..roles_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&roles, i) {
            let role_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get role")
                .into();
            roles_vec.push(role_str);
        }
    }
    options.required_roles = roles_vec;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_getRequiredPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return std::ptr::null_mut();
    }

    let options = unsafe { &*(ptr as *const RiJWTValidationOptions) };
    let perms: Vec<String> = options.required_permissions.clone();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(perms.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, perm) in perms.iter().enumerate() {
        let jstr = env.new_string(perm).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_setRequiredPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permissions: JObjectArray,
) {
    if !check_not_null(&mut env, ptr, "RiJWTValidationOptions") {
        return;
    }

    let options = unsafe { &mut *(ptr as *mut RiJWTValidationOptions) };
    let perms_len = env
        .get_array_length(&permissions)
        .expect("Failed to get permissions length");
    let mut perms_vec = Vec::with_capacity(perms_len as usize);
    for i in 0..perms_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&permissions, i) {
            let perm_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get permission")
                .into();
            perms_vec.push(perm_str);
        }
    }
    options.required_permissions = perms_vec;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTValidationOptions_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiJWTValidationOptions);
        }
    }
}

// =============================================================================
// RiJWTManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTManager_generateToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    claims_ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiJWTManager") {
        return std::ptr::null_mut();
    }
    if !check_not_null(&mut env, claims_ptr, "RiJWTClaims") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiJWTManager) };
    let claims = unsafe { &*(claims_ptr as *const RiJWTClaims) };

    match manager.generate_token(
        &claims.sub,
        claims.roles.clone(),
        claims.permissions.clone(),
    ) {
        Ok(token) => env
            .new_string(&token)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTManager_validateToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    token: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiJWTManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiJWTManager) };
    let token_str: String = env.get_string(&token).expect("Failed to get token").into();

    match manager.validate_token(&token_str) {
        Ok(claims) => {
            let claims_box = Box::new(claims);
            Box::into_raw(claims_box) as jlong
        }
        Err(_) => 0,
    }
}

// =============================================================================
// RiSession JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return std::ptr::null_mut();
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    env.new_string(&session.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getUserId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return std::ptr::null_mut();
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    env.new_string(&session.user_id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getCreatedAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return 0;
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    session.created_at as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getLastAccessed0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return 0;
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    session.last_accessed as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getExpiresAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return 0;
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    session.expires_at as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getIpAddress0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return std::ptr::null_mut();
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    match &session.ip_address {
        Some(ip) => env
            .new_string(ip)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getUserAgent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return std::ptr::null_mut();
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    match &session.user_agent {
        Some(ua) => env
            .new_string(ua)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_isExpired0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return 0;
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    if session.is_expired() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_getData0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return std::ptr::null_mut();
    }

    let session = unsafe { &*(ptr as *const RiSession) };
    let key_str: String = env.get_string(&key).expect("Failed to get key").into();

    match session.get_data(&key_str) {
        Some(value) => env
            .new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_setData0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
) {
    if !check_not_null(&mut env, ptr, "RiSession") {
        return;
    }

    let session = unsafe { &mut *(ptr as *mut RiSession) };
    let key_str: String = env.get_string(&key).expect("Failed to get key").into();
    let value_str: String = env.get_string(&value).expect("Failed to get value").into();

    session.set_data(key_str, value_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSession_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSession);
        }
    }
}

// =============================================================================
// RiSessionManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_new0(
    _env: JNIEnv,
    _class: JClass,
    timeout_secs: jlong,
) -> jlong {
    let manager = Box::new(RiSessionManager::new(timeout_secs as u64));
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_createSession0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
    ip_address: JString,
    user_agent: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();

    let ip_str = if ip_address.is_null() {
        None
    } else {
        Some(
            env.get_string(&ip_address)
                .expect("Failed to get ip")
                .into(),
        )
    };

    let ua_str = if user_agent.is_null() {
        None
    } else {
        Some(
            env.get_string(&user_agent)
                .expect("Failed to get user_agent")
                .into(),
        )
    };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.create_session(user_id_str, ip_str, ua_str).await }) {
        Ok(session_id) => env
            .new_string(&session_id)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_getSession0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    session_id: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };
    let session_id_str: String = env
        .get_string(&session_id)
        .expect("Failed to get session_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.get_session(&session_id_str).await }) {
        Ok(Some(session)) => {
            let session_box = Box::new(session);
            Box::into_raw(session_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_destroySession0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    session_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };
    let session_id_str: String = env
        .get_string(&session_id)
        .expect("Failed to get session_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.destroy_session(&session_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_extendSession0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    session_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };
    let session_id_str: String = env
        .get_string(&session_id)
        .expect("Failed to get session_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.extend_session(&session_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_cleanupExpired0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.cleanup_expired().await }) {
        Ok(count) => count as jint,
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_getTimeout0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSessionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiSessionManager) };
    manager.get_timeout() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSessionManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSessionManager);
        }
    }
}

// =============================================================================
// RiPermission JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    name: JString,
    description: JString,
    resource: JString,
    action: JString,
) -> jlong {
    let id_str: String = env.get_string(&id).expect("Failed to get id").into();
    let name_str: String = env.get_string(&name).expect("Failed to get name").into();
    let desc_str: String = env
        .get_string(&description)
        .expect("Failed to get description")
        .into();
    let resource_str: String = env
        .get_string(&resource)
        .expect("Failed to get resource")
        .into();
    let action_str: String = env
        .get_string(&action)
        .expect("Failed to get action")
        .into();

    let permission = Box::new(RiPermission {
        id: id_str,
        name: name_str,
        description: desc_str,
        resource: resource_str,
        action: action_str,
    });
    Box::into_raw(permission) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiPermission") {
        return std::ptr::null_mut();
    }

    let permission = unsafe { &*(ptr as *const RiPermission) };
    env.new_string(&permission.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiPermission") {
        return std::ptr::null_mut();
    }

    let permission = unsafe { &*(ptr as *const RiPermission) };
    env.new_string(&permission.name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_getDescription0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiPermission") {
        return std::ptr::null_mut();
    }

    let permission = unsafe { &*(ptr as *const RiPermission) };
    env.new_string(&permission.description)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_getResource0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiPermission") {
        return std::ptr::null_mut();
    }

    let permission = unsafe { &*(ptr as *const RiPermission) };
    env.new_string(&permission.resource)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_getAction0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiPermission") {
        return std::ptr::null_mut();
    }

    let permission = unsafe { &*(ptr as *const RiPermission) };
    env.new_string(&permission.action)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermission_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiPermission);
        }
    }
}

// =============================================================================
// RiRole JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    name: JString,
    description: JString,
    permissions: JObjectArray,
    is_system: jboolean,
) -> jlong {
    let id_str: String = env.get_string(&id).expect("Failed to get id").into();
    let name_str: String = env.get_string(&name).expect("Failed to get name").into();
    let desc_str: String = env
        .get_string(&description)
        .expect("Failed to get description")
        .into();

    let perms_len = env
        .get_array_length(&permissions)
        .expect("Failed to get permissions length");
    let mut perms_set = HashSet::with_capacity(perms_len as usize);
    for i in 0..perms_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&permissions, i) {
            let perm_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get permission")
                .into();
            perms_set.insert(perm_str);
        }
    }

    let role = Box::new(RiRole {
        id: id_str,
        name: name_str,
        description: desc_str,
        permissions: perms_set,
        is_system: is_system != 0,
    });
    Box::into_raw(role) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return std::ptr::null_mut();
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    env.new_string(&role.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return std::ptr::null_mut();
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    env.new_string(&role.name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_getDescription0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return std::ptr::null_mut();
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    env.new_string(&role.description)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_getPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return std::ptr::null_mut();
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    let perms: Vec<String> = role.permissions.iter().cloned().collect();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(perms.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, perm) in perms.iter().enumerate() {
        let jstr = env.new_string(perm).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_isSystem0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return 0;
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    if role.is_system {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_hasPermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permission_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return 0;
    }

    let role = unsafe { &*(ptr as *const RiRole) };
    let perm_str: String = env
        .get_string(&permission_id)
        .expect("Failed to get permission_id")
        .into();

    if role.has_permission(&perm_str) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_addPermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permission_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiRole") {
        return;
    }

    let role = unsafe { &mut *(ptr as *mut RiRole) };
    let perm_str: String = env
        .get_string(&permission_id)
        .expect("Failed to get permission_id")
        .into();

    role.add_permission(perm_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRole_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRole);
        }
    }
}

// =============================================================================
// RiPermissionManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let manager = Box::new(RiPermissionManager::new());
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_createPermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permission_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return;
    }
    if !check_not_null(&mut env, permission_ptr, "RiPermission") {
        return;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let permission = unsafe { &*(permission_ptr as *const RiPermission) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async { manager.create_permission(permission.clone()).await });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_getPermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permission_id: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let perm_id_str: String = env
        .get_string(&permission_id)
        .expect("Failed to get permission_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.get_permission(&perm_id_str).await }) {
        Ok(Some(perm)) => {
            let perm_box = Box::new(perm);
            Box::into_raw(perm_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_createRole0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    role_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return;
    }
    if !check_not_null(&mut env, role_ptr, "RiRole") {
        return;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let role = unsafe { &*(role_ptr as *const RiRole) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async { manager.create_role(role.clone()).await });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_getRole0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    role_id: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let role_id_str: String = env
        .get_string(&role_id)
        .expect("Failed to get role_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.get_role(&role_id_str).await }) {
        Ok(Some(role)) => {
            let role_box = Box::new(role);
            Box::into_raw(role_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_assignRoleToUser0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
    role_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();
    let role_id_str: String = env
        .get_string(&role_id)
        .expect("Failed to get role_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.assign_role_to_user(user_id_str, role_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_removeRoleFromUser0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
    role_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();
    let role_id_str: String = env
        .get_string(&role_id)
        .expect("Failed to get role_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async {
        manager
            .remove_role_from_user(&user_id_str, &role_id_str)
            .await
    }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_getUserRoles0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let roles = rt
        .block_on(async { manager.get_user_roles(&user_id_str).await })
        .unwrap_or_default();

    let ptrs: Vec<jlong> = roles
        .into_iter()
        .map(|r| Box::into_raw(Box::new(r)) as jlong)
        .collect();

    let long_class = env
        .find_class("java/lang/Long")
        .expect("Failed to find Long class");
    let array = env
        .new_long_array(ptrs.len() as jint)
        .expect("Failed to create array");
    env.set_long_array_region(&array, 0, &ptrs)
        .expect("Failed to set array");

    array.into_raw()
}

use jni::sys::jlongArray;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_hasPermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
    permission_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();
    let perm_id_str: String = env
        .get_string(&permission_id)
        .expect("Failed to get permission_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.has_permission(&user_id_str, &perm_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_getUserPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let perms = rt
        .block_on(async { manager.get_user_permissions(&user_id_str).await })
        .unwrap_or_default();

    let perms_vec: Vec<String> = perms.into_iter().collect();

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(perms_vec.len() as jint, string_class, std::ptr::null_mut())
        .expect("Failed to create array");

    for (i, perm) in perms_vec.iter().enumerate() {
        let jstr = env.new_string(perm).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_deletePermission0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    permission_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let perm_id_str: String = env
        .get_string(&permission_id)
        .expect("Failed to get permission_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.delete_permission(&perm_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_deleteRole0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    role_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };
    let role_id_str: String = env
        .get_string(&role_id)
        .expect("Failed to get role_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.delete_role(&role_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_listPermissions0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let perms = rt
        .block_on(async { manager.list_permissions().await })
        .unwrap_or_default();

    let ptrs: Vec<jlong> = perms
        .into_iter()
        .map(|p| Box::into_raw(Box::new(p)) as jlong)
        .collect();

    let array = env
        .new_long_array(ptrs.len() as jint)
        .expect("Failed to create array");
    env.set_long_array_region(&array, 0, &ptrs)
        .expect("Failed to set array");

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_listRoles0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiPermissionManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiPermissionManager) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let roles = rt
        .block_on(async { manager.list_roles().await })
        .unwrap_or_default();

    let ptrs: Vec<jlong> = roles
        .into_iter()
        .map(|r| Box::into_raw(Box::new(r)) as jlong)
        .collect();

    let array = env
        .new_long_array(ptrs.len() as jint)
        .expect("Failed to create array");
    env.set_long_array_region(&array, 0, &ptrs)
        .expect("Failed to set array");

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiPermissionManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiPermissionManager);
        }
    }
}

// =============================================================================
// RiOAuthProvider JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    name: JString,
    client_id: JString,
    client_secret: JString,
    auth_url: JString,
    token_url: JString,
    user_info_url: JString,
    scopes: JObjectArray,
    enabled: jboolean,
    redirect_uri: JString,
) -> jlong {
    let id_str: String = env.get_string(&id).expect("Failed to get id").into();
    let name_str: String = env.get_string(&name).expect("Failed to get name").into();
    let client_id_str: String = env
        .get_string(&client_id)
        .expect("Failed to get client_id")
        .into();
    let client_secret_str: String = env
        .get_string(&client_secret)
        .expect("Failed to get client_secret")
        .into();
    let auth_url_str: String = env
        .get_string(&auth_url)
        .expect("Failed to get auth_url")
        .into();
    let token_url_str: String = env
        .get_string(&token_url)
        .expect("Failed to get token_url")
        .into();
    let user_info_url_str: String = env
        .get_string(&user_info_url)
        .expect("Failed to get user_info_url")
        .into();

    let scopes_len = env
        .get_array_length(&scopes)
        .expect("Failed to get scopes length");
    let mut scopes_vec = Vec::with_capacity(scopes_len as usize);
    for i in 0..scopes_len {
        if let Ok(Some(elem)) = env.get_object_array_element(&scopes, i) {
            let scope_str: String = env
                .get_string(&JString::from(elem))
                .expect("Failed to get scope")
                .into();
            scopes_vec.push(scope_str);
        }
    }

    let redirect_uri_opt = if redirect_uri.is_null() {
        None
    } else {
        Some(
            env.get_string(&redirect_uri)
                .expect("Failed to get redirect_uri")
                .into(),
        )
    };

    let provider = Box::new(RiOAuthProvider {
        id: id_str,
        name: name_str,
        client_id: client_id_str,
        client_secret: client_secret_str,
        auth_url: auth_url_str,
        token_url: token_url_str,
        user_info_url: user_info_url_str,
        scopes: scopes_vec,
        enabled: enabled != 0,
        redirect_uri: redirect_uri_opt,
    });
    Box::into_raw(provider) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getClientId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.client_id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getAuthUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.auth_url)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getTokenUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.token_url)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getUserInfoUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    env.new_string(&provider.user_info_url)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getScopes0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };

    let string_class = env
        .find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env
        .new_object_array(
            provider.scopes.len() as jint,
            string_class,
            std::ptr::null_mut(),
        )
        .expect("Failed to create array");

    for (i, scope) in provider.scopes.iter().enumerate() {
        let jstr = env.new_string(scope).expect("Failed to create string");
        env.set_object_array_element(&array, i as jint, jstr)
            .expect("Failed to set array element");
    }

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_isEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return 0;
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    if provider.enabled {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_getRedirectUri0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthProvider") {
        return std::ptr::null_mut();
    }

    let provider = unsafe { &*(ptr as *const RiOAuthProvider) };
    match &provider.redirect_uri {
        Some(uri) => env
            .new_string(uri)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthProvider_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiOAuthProvider);
        }
    }
}

// =============================================================================
// RiOAuthToken JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_new0(
    mut env: JNIEnv,
    _class: JClass,
    access_token: JString,
    token_type: JString,
    refresh_token: JString,
    scope: JString,
    expires_in: jlong,
) -> jlong {
    let access_token_str: String = env
        .get_string(&access_token)
        .expect("Failed to get access_token")
        .into();
    let token_type_str: String = env
        .get_string(&token_type)
        .expect("Failed to get token_type")
        .into();
    let refresh_token_opt = if refresh_token.is_null() {
        None
    } else {
        Some(
            env.get_string(&refresh_token)
                .expect("Failed to get refresh_token")
                .into(),
        )
    };
    let scope_opt = if scope.is_null() {
        None
    } else {
        Some(env.get_string(&scope).expect("Failed to get scope").into())
    };

    let token = Box::new(RiOAuthToken {
        access_token: access_token_str,
        token_type: token_type_str,
        refresh_token: refresh_token_opt,
        scope: scope_opt,
        expires_in: Some(expires_in),
    });
    Box::into_raw(token) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_getAccessToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthToken") {
        return std::ptr::null_mut();
    }

    let token = unsafe { &*(ptr as *const RiOAuthToken) };
    env.new_string(&token.access_token)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_getTokenType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthToken") {
        return std::ptr::null_mut();
    }

    let token = unsafe { &*(ptr as *const RiOAuthToken) };
    env.new_string(&token.token_type)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_getRefreshToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthToken") {
        return std::ptr::null_mut();
    }

    let token = unsafe { &*(ptr as *const RiOAuthToken) };
    match &token.refresh_token {
        Some(rt) => env
            .new_string(rt)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_getScope0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthToken") {
        return std::ptr::null_mut();
    }

    let token = unsafe { &*(ptr as *const RiOAuthToken) };
    match &token.scope {
        Some(s) => env
            .new_string(s)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_getExpiresIn0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiOAuthToken") {
        return 0;
    }

    let token = unsafe { &*(ptr as *const RiOAuthToken) };
    token.expires_in.unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthToken_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiOAuthToken);
        }
    }
}

// =============================================================================
// RiOAuthUserInfo JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    email: JString,
    name: JString,
    avatar_url: JString,
    provider: JString,
) -> jlong {
    let id_str: String = env.get_string(&id).expect("Failed to get id").into();
    let email_str: String = env.get_string(&email).expect("Failed to get email").into();
    let name_opt = if name.is_null() {
        None
    } else {
        Some(env.get_string(&name).expect("Failed to get name").into())
    };
    let avatar_url_opt = if avatar_url.is_null() {
        None
    } else {
        Some(
            env.get_string(&avatar_url)
                .expect("Failed to get avatar_url")
                .into(),
        )
    };
    let provider_str: String = env
        .get_string(&provider)
        .expect("Failed to get provider")
        .into();

    let user_info = Box::new(RiOAuthUserInfo {
        id: id_str,
        email: email_str,
        name: name_opt,
        avatar_url: avatar_url_opt,
        provider: provider_str,
    });
    Box::into_raw(user_info) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthUserInfo") {
        return std::ptr::null_mut();
    }

    let user_info = unsafe { &*(ptr as *const RiOAuthUserInfo) };
    env.new_string(&user_info.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_getEmail0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthUserInfo") {
        return std::ptr::null_mut();
    }

    let user_info = unsafe { &*(ptr as *const RiOAuthUserInfo) };
    env.new_string(&user_info.email)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthUserInfo") {
        return std::ptr::null_mut();
    }

    let user_info = unsafe { &*(ptr as *const RiOAuthUserInfo) };
    match &user_info.name {
        Some(n) => env
            .new_string(n)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_getAvatarUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthUserInfo") {
        return std::ptr::null_mut();
    }

    let user_info = unsafe { &*(ptr as *const RiOAuthUserInfo) };
    match &user_info.avatar_url {
        Some(url) => env
            .new_string(url)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_getProvider0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthUserInfo") {
        return std::ptr::null_mut();
    }

    let user_info = unsafe { &*(ptr as *const RiOAuthUserInfo) };
    env.new_string(&user_info.provider)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthUserInfo_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiOAuthUserInfo);
        }
    }
}

// =============================================================================
// RiOAuthManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    use crate::cache::RiMemoryCache;
    use std::sync::Arc;

    let cache = Arc::new(RiMemoryCache::new());
    let manager = Box::new(RiOAuthManager::new(cache));
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_registerProvider0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return;
    }
    if !check_not_null(&mut env, provider_ptr, "RiOAuthProvider") {
        return;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider = unsafe { &*(provider_ptr as *const RiOAuthProvider) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async { manager.register_provider(provider.clone()).await });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_getProvider0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.get_provider(&provider_id_str).await }) {
        Ok(Some(provider)) => {
            let provider_box = Box::new(provider);
            Box::into_raw(provider_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_getAuthUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
    state: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();
    let state_str: String = env.get_string(&state).expect("Failed to get state").into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.get_auth_url(&provider_id_str, &state_str).await }) {
        Ok(Some(url)) => env
            .new_string(&url)
            .expect("Failed to create string")
            .into_raw(),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_exchangeCodeForToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
    code: JString,
    redirect_uri: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();
    let code_str: String = env.get_string(&code).expect("Failed to get code").into();
    let redirect_uri_str: String = env
        .get_string(&redirect_uri)
        .expect("Failed to get redirect_uri")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async {
        manager
            .exchange_code_for_token(&provider_id_str, &code_str, &redirect_uri_str)
            .await
    }) {
        Ok(Some(token)) => {
            let token_box = Box::new(token);
            Box::into_raw(token_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_getUserInfo0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
    access_token: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();
    let access_token_str: String = env
        .get_string(&access_token)
        .expect("Failed to get access_token")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async {
        manager
            .get_user_info(&provider_id_str, &access_token_str)
            .await
    }) {
        Ok(Some(user_info)) => {
            let user_info_box = Box::new(user_info);
            Box::into_raw(user_info_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_refreshToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
    refresh_token: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();
    let refresh_token_str: String = env
        .get_string(&refresh_token)
        .expect("Failed to get refresh_token")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async {
        manager
            .refresh_token(&provider_id_str, &refresh_token_str)
            .await
    }) {
        Ok(Some(token)) => {
            let token_box = Box::new(token);
            Box::into_raw(token_box) as jlong
        }
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_revokeToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
    access_token: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();
    let access_token_str: String = env
        .get_string(&access_token)
        .expect("Failed to get access_token")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async {
        manager
            .revoke_token(&provider_id_str, &access_token_str)
            .await
    }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_listProviders0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return std::ptr::null_mut();
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let providers = rt
        .block_on(async { manager.list_providers().await })
        .unwrap_or_default();

    let ptrs: Vec<jlong> = providers
        .into_iter()
        .map(|p| Box::into_raw(Box::new(p)) as jlong)
        .collect();

    let array = env
        .new_long_array(ptrs.len() as jint)
        .expect("Failed to create array");
    env.set_long_array_region(&array, 0, &ptrs)
        .expect("Failed to set array");

    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_disableProvider0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.disable_provider(&provider_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_enableProvider0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    provider_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiOAuthManager") {
        return 0;
    }

    let manager = unsafe { &*(ptr as *const RiOAuthManager) };
    let provider_id_str: String = env
        .get_string(&provider_id)
        .expect("Failed to get provider_id")
        .into();

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    match rt.block_on(async { manager.enable_provider(&provider_id_str).await }) {
        Ok(result) => {
            if result {
                1
            } else {
                0
            }
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiOAuthManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiOAuthManager);
        }
    }
}

// =============================================================================
// RiRevokedTokenInfo JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_new0(
    mut env: JNIEnv,
    _class: JClass,
    token_id: JString,
    user_id: JString,
    revoked_at: jlong,
    expires_at: jlong,
    reason: JString,
) -> jlong {
    let token_id_str: String = env
        .get_string(&token_id)
        .expect("Failed to get token_id")
        .into();
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();
    let reason_opt = if reason.is_null() {
        None
    } else {
        Some(
            env.get_string(&reason)
                .expect("Failed to get reason")
                .into(),
        )
    };

    let info = Box::new(RiRevokedTokenInfo {
        token_id: token_id_str,
        user_id: user_id_str,
        revoked_at: revoked_at as u64,
        expires_at: expires_at as u64,
        reason: reason_opt,
    });
    Box::into_raw(info) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_getTokenId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRevokedTokenInfo") {
        return std::ptr::null_mut();
    }

    let info = unsafe { &*(ptr as *const RiRevokedTokenInfo) };
    env.new_string(&info.token_id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_getUserId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRevokedTokenInfo") {
        return std::ptr::null_mut();
    }

    let info = unsafe { &*(ptr as *const RiRevokedTokenInfo) };
    env.new_string(&info.user_id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_getRevokedAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiRevokedTokenInfo") {
        return 0;
    }

    let info = unsafe { &*(ptr as *const RiRevokedTokenInfo) };
    info.revoked_at as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_getExpiresAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiRevokedTokenInfo") {
        return 0;
    }

    let info = unsafe { &*(ptr as *const RiRevokedTokenInfo) };
    info.expires_at as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_getReason0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRevokedTokenInfo") {
        return std::ptr::null_mut();
    }

    let info = unsafe { &*(ptr as *const RiRevokedTokenInfo) };
    match &info.reason {
        Some(r) => env
            .new_string(r)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiRevokedTokenInfo_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRevokedTokenInfo);
        }
    }
}

// =============================================================================
// RiJWTRevocationList JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let list = Box::new(RiJWTRevocationList::new());
    Box::into_raw(list) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_revokeToken0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    token: JString,
    user_id: JString,
    reason: JString,
    ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    let token_str: String = env.get_string(&token).expect("Failed to get token").into();
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();
    let reason_opt = if reason.is_null() {
        None
    } else {
        Some(
            env.get_string(&reason)
                .expect("Failed to get reason")
                .into(),
        )
    };

    list.revoke_token(&token_str, &user_id_str, reason_opt, ttl_secs as u64);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_isRevoked0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    token: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return 0;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    let token_str: String = env.get_string(&token).expect("Failed to get token").into();

    if list.is_revoked(&token_str) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_revokeByUser0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    user_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return 0;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    let user_id_str: String = env
        .get_string(&user_id)
        .expect("Failed to get user_id")
        .into();

    let count = list.revoke_all_user_tokens(&user_id_str, None);
    if count > 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_getRevocationInfo0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    token: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return 0;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    let token_str: String = env.get_string(&token).expect("Failed to get token").into();

    match list.get_revocation_info(&token_str) {
        Some(info) => {
            let info_box = Box::new(info);
            Box::into_raw(info_box) as jlong
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_getRevokedCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return 0;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    list.get_revoked_count() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_cleanup0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return 0;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };

    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let count = rt.block_on(async { list.cleanup_expired() });

    count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_clear0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiJWTRevocationList") {
        return;
    }

    let list = unsafe { &*(ptr as *const RiJWTRevocationList) };
    list.clear();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTRevocationList_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiJWTRevocationList);
        }
    }
}

// =============================================================================
// RiSecurityManager JNI Bindings (Static Methods)
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_encrypt0(
    mut env: JNIEnv,
    _class: JClass,
    plaintext: JString,
) -> jstring {
    let plaintext_str: String = env
        .get_string(&plaintext)
        .expect("Failed to get plaintext")
        .into();

    match RiSecurityManager::encrypt(&plaintext_str) {
        Ok(encrypted) => env
            .new_string(&encrypted)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_decrypt0(
    mut env: JNIEnv,
    _class: JClass,
    encrypted: JString,
) -> jstring {
    let encrypted_str: String = env
        .get_string(&encrypted)
        .expect("Failed to get encrypted")
        .into();

    match RiSecurityManager::decrypt(&encrypted_str) {
        Ok(decrypted) => env
            .new_string(&decrypted)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_hmacSign0(
    mut env: JNIEnv,
    _class: JClass,
    data: JString,
) -> jstring {
    let data_str: String = env.get_string(&data).expect("Failed to get data").into();

    let signature = RiSecurityManager::hmac_sign(&data_str);
    env.new_string(&signature)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_hmacVerify0(
    mut env: JNIEnv,
    _class: JClass,
    data: JString,
    signature: JString,
) -> jboolean {
    let data_str: String = env.get_string(&data).expect("Failed to get data").into();
    let signature_str: String = env
        .get_string(&signature)
        .expect("Failed to get signature")
        .into();

    if RiSecurityManager::hmac_verify(&data_str, &signature_str) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_generateEncryptionKey0(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let key = RiSecurityManager::generate_encryption_key();
    env.new_string(&key)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiSecurityManager_generateHmacKey0(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let key = RiSecurityManager::generate_hmac_key();
    env.new_string(&key)
        .expect("Failed to create string")
        .into_raw()
}
