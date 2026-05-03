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

//! # Log Module JNI Bindings
//!
//! JNI bindings for Ri log classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jfloat};
use crate::log::{RiLogger, RiLogConfig, RiLogLevel};
use crate::fs::RiFileSystem;
use crate::java::exception::{check_not_null, throw_illegal_argument};
use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};

// =============================================================================
// RiLogger JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = RiLogConfig::default();
    let fs = RiFileSystem::new_auto_root().unwrap_or_else(|_| RiFileSystem::new_with_root(std::env::current_dir().unwrap_or_default()));
    let logger = Box::new(RiLogger::new(&config, fs));
    let ptr = Box::into_raw(logger);
    register_jni_ptr(ptr as usize);
    ptr as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLogger);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_debug0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target: JString,
    message: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogger") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogger pointer");
        return;
    }
    
    let logger = unsafe { &*(ptr as *const RiLogger) };
    let target_str: String = env.get_string(&target)
        .expect("Failed to get target")
        .into();
    let message_str: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let _ = logger.debug(&target_str, &message_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_info0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target: JString,
    message: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogger") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogger pointer");
        return;
    }
    
    let logger = unsafe { &*(ptr as *const RiLogger) };
    let target_str: String = env.get_string(&target)
        .expect("Failed to get target")
        .into();
    let message_str: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let _ = logger.info(&target_str, &message_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_warn0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target: JString,
    message: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogger") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogger pointer");
        return;
    }
    
    let logger = unsafe { &*(ptr as *const RiLogger) };
    let target_str: String = env.get_string(&target)
        .expect("Failed to get target")
        .into();
    let message_str: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let _ = logger.warn(&target_str, &message_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_error0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    target: JString,
    message: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogger") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogger pointer");
        return;
    }
    
    let logger = unsafe { &*(ptr as *const RiLogger) };
    let target_str: String = env.get_string(&target)
        .expect("Failed to get target")
        .into();
    let message_str: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let _ = logger.error(&target_str, &message_str);
}

// =============================================================================
// RiLogConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiLogConfig::default());
    let ptr = Box::into_raw(config);
    register_jni_ptr(ptr as usize);
    ptr as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setLevel0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    level: jint,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.level = match level {
        0 => RiLogLevel::Debug,
        1 => RiLogLevel::Info,
        2 => RiLogLevel::Warn,
        3 => RiLogLevel::Error,
        _ => RiLogLevel::Info,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setConsoleEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.console_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setFileEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.file_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setSamplingDefault0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    rate: jfloat,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.sampling_default = rate.clamp(0.0, 1.0);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setFileName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    file_name: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.file_name = env.get_string(&file_name)
        .expect("Failed to get file name")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setJsonFormat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    json_format: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.json_format = json_format != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setRotateWhen0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    rotate_when: JString,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.rotate_when = env.get_string(&rotate_when)
        .expect("Failed to get rotate_when")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setMaxBytes0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_bytes: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.max_bytes = max_bytes as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_setColorBlocks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    color_blocks: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiLogConfig") {
        return;
    }
    
    if !is_jni_ptr_valid(ptr as usize) {
        throw_illegal_argument(&mut env, "Invalid RiLogConfig pointer");
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiLogConfig) };
    config.color_blocks = color_blocks != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLogConfig);
        }
    }
}
