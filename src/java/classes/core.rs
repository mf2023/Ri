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

//! # Core Module JNI Bindings
//!
//! JNI bindings for DMSC core classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jstring};
use crate::core::{DMSCAppBuilder, DMSCAppRuntime, DMSCError};
use crate::java::exception::{throw_dmsc_error, check_not_null};
use crate::config::DMSCConfig;

// =============================================================================
// DMSCAppBuilder JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppBuilder_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let builder = Box::new(DMSCAppBuilder::new());
    Box::into_raw(builder) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppBuilder_withConfig(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    config_path: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut DMSCAppBuilder) };
    let path: String = env.get_string(&config_path)
        .expect("Failed to get config path")
        .into();
    
    match builder.with_config(&path) {
        Ok(new_builder) => {
            let boxed = Box::new(new_builder);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_dmsc_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppBuilder_build(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut DMSCAppBuilder) };
    
    match builder.build() {
        Ok(runtime) => {
            let boxed = Box::new(runtime);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_dmsc_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCAppBuilder);
        }
    }
}

// =============================================================================
// DMSCAppRuntime JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppRuntime_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCAppRuntime);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppRuntime_isRunning(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "DMSCAppRuntime") {
        return 0;
    }
    
    let _runtime = unsafe { &*(ptr as *const DMSCAppRuntime) };
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCAppRuntime_shutdown(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "DMSCAppRuntime") {
        return;
    }
    
    let _runtime = unsafe { &*(ptr as *const DMSCAppRuntime) };
}

// =============================================================================
// DMSCConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(DMSCConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCConfig);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCConfig_get(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "DMSCConfig") {
        return std::ptr::null_mut();
    }
    
    let config = unsafe { &*(ptr as *const DMSCConfig) };
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
// DMSCError JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCError_getMessage(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "DMSCError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const DMSCError) };
    env.new_string(error.to_string())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_DMSCError_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCError);
        }
    }
}
