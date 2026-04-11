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

//! # Core Module JNI Bindings
//!
//! JNI bindings for Ri core classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jstring};
use crate::core::{RiAppBuilder, RiAppRuntime, RiError};
use crate::java::exception::{throw_ri_error, check_not_null};
use crate::config::RiConfig;

// =============================================================================
// RiAppBuilder JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let builder = Box::new(RiAppBuilder::new());
    Box::into_raw(builder) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_withConfig(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    config_path: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut RiAppBuilder) };
    let path: String = env.get_string(&config_path)
        .expect("Failed to get config path")
        .into();
    
    match builder.with_config(&path) {
        Ok(new_builder) => {
            let boxed = Box::new(new_builder);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_ri_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_build(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut RiAppBuilder) };
    
    match builder.build() {
        Ok(runtime) => {
            let boxed = Box::new(runtime);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_ri_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAppBuilder);
        }
    }
}

// =============================================================================
// RiAppRuntime JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAppRuntime);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_isRunning(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiAppRuntime") {
        return 0;
    }
    
    let _runtime = unsafe { &*(ptr as *const RiAppRuntime) };
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_shutdown(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiAppRuntime") {
        return;
    }
    
    let _runtime = unsafe { &*(ptr as *const RiAppRuntime) };
}

// =============================================================================
// RiConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiConfig);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_get(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiConfig") {
        return std::ptr::null_mut();
    }
    
    let config = unsafe { &*(ptr as *const RiConfig) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match config.get(&key_str) {
        Some(value) => {
            env.new_string(value)
                .expect("Failed to create Java string")
                .into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

// =============================================================================
// RiError JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiError_getMessage(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiError) };
    env.new_string(error.to_string())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiError_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiError);
        }
    }
}
