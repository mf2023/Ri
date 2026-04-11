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

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jstring};
use crate::auth::{RiAuthModule, RiAuthConfig};
use crate::java::exception::check_not_null;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiAuthModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiAuthConfig") {
        return 0;
    }
    0
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

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTManager_generateToken(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    _claims_ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiJWTManager") {
        return std::ptr::null_mut();
    }
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_auth_RiJWTManager_validateToken(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    _token: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiJWTManager") {
        return 0;
    }
    0
}
