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

//! # Validation Module JNI Bindings
//!
//! JNI bindings for Ri validation classes.
//!
//! ## Security
//!
//! This module uses a secure pointer registry to prevent:
//! - Use-After-Free: Pointers are validated before access
//! - Double-Free: Free operations are idempotent
//! - Invalid Pointer Dereference: All pointers are tracked

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jstring};
use dashmap::DashSet;

use crate::validation::{
    RiValidationError, RiValidationResult, RiValidationSeverity,
    RiSanitizer, RiSanitizationConfig, RiSchemaValidator,
    RiValidatorBuilder, RiValidationRunner,
};

lazy_static::lazy_static! {
    static ref VALIDATION_MODULE_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATION_ERROR_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATION_RESULT_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SANITIZATION_CONFIG_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SANITIZER_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SCHEMA_VALIDATOR_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATION_RUNNER_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATOR_BUILDER_REGISTRY: DashSet<usize> = DashSet::new();
}

fn register_ptr(registry: &DashSet<usize>, ptr: usize) {
    registry.insert(ptr);
}

fn unregister_ptr(registry: &DashSet<usize>, ptr: usize) -> bool {
    registry.remove(&ptr).is_some()
}

fn is_valid_ptr(registry: &DashSet<usize>, ptr: usize) -> bool {
    ptr != 0 && registry.contains(&ptr)
}

// =============================================================================
// RiValidationModule JNI Bindings (static validation helpers)
// =============================================================================

fn box_result(result: RiValidationResult) -> jlong {
    let result = Box::new(result);
    let result_ptr = Box::into_raw(result) as jlong;
    register_ptr(&VALIDATION_RESULT_REGISTRY, result_ptr as usize);
    result_ptr
}

fn get_string_arg(env: &mut JNIEnv, s: &JString, arg: &str) -> Option<String> {
    match env.get_string(s) {
        Ok(v) => Some(v.into()),
        Err(_) => {
            let _ = env.throw_new(
                "java/lang/IllegalArgumentException",
                &format!("Failed to get {} string", arg),
            );
            None
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateEmail(
    mut env: JNIEnv,
    _class: JClass,
    email: JString,
) -> jlong {
    let Some(email) = get_string_arg(&mut env, &email, "email") else { return 0 };
    let result = RiValidatorBuilder::new("email")
        .is_email()
        .max_length(255)
        .build()
        .validate_value(Some(&email));
    box_result(result)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateUsername(
    mut env: JNIEnv,
    _class: JClass,
    username: JString,
) -> jlong {
    let Some(username) = get_string_arg(&mut env, &username, "username") else { return 0 };
    let result = RiValidatorBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(32)
        .alphanumeric()
        .build()
        .validate_value(Some(&username));
    box_result(result)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidatePassword(
    mut env: JNIEnv,
    _class: JClass,
    password: JString,
) -> jlong {
    let Some(password) = get_string_arg(&mut env, &password, "password") else { return 0 };
    let result = RiValidatorBuilder::new("password")
        .not_empty()
        .min_length(8)
        .build()
        .validate_value(Some(&password));
    box_result(result)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateUrl(
    mut env: JNIEnv,
    _class: JClass,
    url: JString,
) -> jlong {
    let Some(url) = get_string_arg(&mut env, &url, "url") else { return 0 };
    let result = RiValidatorBuilder::new("url")
        .is_url()
        .build()
        .validate_value(Some(&url));
    box_result(result)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateIp(
    mut env: JNIEnv,
    _class: JClass,
    ip: JString,
) -> jlong {
    let Some(ip) = get_string_arg(&mut env, &ip, "ip") else { return 0 };
    let result = RiValidatorBuilder::new("ip")
        .is_ip()
        .build()
        .validate_value(Some(&ip));
    box_result(result)
}

// =============================================================================
// RiValidationError JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getField0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !is_valid_ptr(&VALIDATION_ERROR_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationError pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let error = unsafe { &*(ptr as *const RiValidationError) };
    match env.new_string(&error.field) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getMessage0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !is_valid_ptr(&VALIDATION_ERROR_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationError pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let error = unsafe { &*(ptr as *const RiValidationError) };
    match env.new_string(&error.message) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getCode0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !is_valid_ptr(&VALIDATION_ERROR_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationError pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let error = unsafe { &*(ptr as *const RiValidationError) };
    match env.new_string(&error.code) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getSeverity0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !is_valid_ptr(&VALIDATION_ERROR_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationError pointer")
            .unwrap_or(());
        return 0;
    }

    let error = unsafe { &*(ptr as *const RiValidationError) };
    match error.severity.clone() {
        RiValidationSeverity::Error => 0,
        RiValidationSeverity::Warning => 1,
        RiValidationSeverity::Info => 2,
        RiValidationSeverity::Critical => 3,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&VALIDATION_ERROR_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiValidationError);
        }
    }
}

// =============================================================================
// RiValidationResult JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationResult_isValid0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&VALIDATION_RESULT_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationResult pointer")
            .unwrap_or(());
        return 0;
    }

    let result = unsafe { &*(ptr as *const RiValidationResult) };
    result.is_valid as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&VALIDATION_RESULT_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiValidationResult);
        }
    }
}

// =============================================================================
// RiSanitizationConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiSanitizationConfig::default());
    let ptr = Box::into_raw(config) as jlong;
    register_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isTrimWhitespace0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(ptr as *const RiSanitizationConfig) };
    config.trim_whitespace as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_setTrimWhitespace0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiSanitizationConfig) };
    config.trim_whitespace = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isLowercase0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(ptr as *const RiSanitizationConfig) };
    config.lowercase as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_setLowercase0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiSanitizationConfig) };
    config.lowercase = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isUppercase0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(ptr as *const RiSanitizationConfig) };
    config.uppercase as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_setUppercase0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiSanitizationConfig) };
    config.uppercase = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isRemoveHtmlTags0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(ptr as *const RiSanitizationConfig) };
    config.remove_html_tags as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_setRemoveHtmlTags0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiSanitizationConfig) };
    config.remove_html_tags = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isEscapeSpecialChars0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(ptr as *const RiSanitizationConfig) };
    config.escape_special_chars as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_setEscapeSpecialChars0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiSanitizationConfig) };
    config.escape_special_chars = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&SANITIZATION_CONFIG_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSanitizationConfig);
        }
    }
}

// =============================================================================
// RiSanitizer JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let sanitizer = Box::new(RiSanitizer::new());
    let ptr = Box::into_raw(sanitizer) as jlong;
    register_ptr(&SANITIZER_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_newWithConfig0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !is_valid_ptr(&SANITIZATION_CONFIG_REGISTRY, config_ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizationConfig pointer")
            .unwrap_or(());
        return 0;
    }

    let config = unsafe { &*(config_ptr as *const RiSanitizationConfig) };
    let sanitizer = Box::new(RiSanitizer::with_config(config.clone()));
    let ptr = Box::into_raw(sanitizer) as jlong;
    register_ptr(&SANITIZER_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitize0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring {
    if !is_valid_ptr(&SANITIZER_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizer pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get input string")
                .unwrap_or(());
            return std::ptr::null_mut();
        }
    };

    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize(&input_str);
    match env.new_string(&result) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitizeEmail0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring {
    if !is_valid_ptr(&SANITIZER_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizer pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get input string")
                .unwrap_or(());
            return std::ptr::null_mut();
        }
    };

    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize_email(&input_str);
    match env.new_string(&result) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitizeFilename0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring {
    if !is_valid_ptr(&SANITIZER_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSanitizer pointer")
            .unwrap_or(());
        return std::ptr::null_mut();
    }

    let input_str: String = match env.get_string(&input) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get input string")
                .unwrap_or(());
            return std::ptr::null_mut();
        }
    };

    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize_filename(&input_str);
    match env.new_string(&result) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&SANITIZER_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSanitizer);
        }
    }
}

// =============================================================================
// RiSchemaValidator JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSchemaValidator_new0(
    mut env: JNIEnv,
    _class: JClass,
    schema_json: JString,
) -> jlong {
    let schema_str: String = match env.get_string(&schema_json) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get schema string")
                .unwrap_or(());
            return 0;
        }
    };

    let schema_value: serde_json::Value = match serde_json::from_str(&schema_str) {
        Ok(v) => v,
        Err(e) => {
            let _ = env.throw_new(
                "java/lang/IllegalArgumentException",
                &format!("Failed to parse schema JSON: {}", e),
            );
            return 0;
        }
    };

    let validator = RiSchemaValidator::new(schema_value);
    let ptr = Box::into_raw(Box::new(validator)) as jlong;
    register_ptr(&SCHEMA_VALIDATOR_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSchemaValidator_validate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data_json: JString,
) -> jlong {
    if !is_valid_ptr(&SCHEMA_VALIDATOR_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiSchemaValidator pointer")
            .unwrap_or(());
        return 0;
    }

    let data_str: String = match env.get_string(&data_json) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get data string")
                .unwrap_or(());
            return 0;
        }
    };

    let validator = unsafe { &*(ptr as *const RiSchemaValidator) };
    let data_value: serde_json::Value = match serde_json::from_str(&data_str) {
        Ok(v) => v,
        Err(_) => serde_json::Value::Null,
    };
    let result = Box::new(validator.validate(&data_value));
    let result_ptr = Box::into_raw(result) as jlong;
    register_ptr(&VALIDATION_RESULT_REGISTRY, result_ptr as usize);
    result_ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSchemaValidator_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&SCHEMA_VALIDATOR_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSchemaValidator);
        }
    }
}

// =============================================================================
// RiValidationRunner JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationRunner_validate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: JString,
) -> jlong {
    if !is_valid_ptr(&VALIDATION_RUNNER_REGISTRY, ptr as usize) {
        let _ = env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationRunner pointer");
        return 0;
    }

    let value_str = match get_string_arg(&mut env, &value, "value") {
        Some(v) => v,
        None => return 0,
    };

    let runner = unsafe { &*(ptr as *const RiValidationRunner) };
    box_result(runner.validate_value(Some(&value_str)))
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationRunner_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&VALIDATION_RUNNER_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiValidationRunner);
        }
    }
}

// =============================================================================
// RiValidatorBuilder JNI Bindings
// =============================================================================

/// Stable slot holding an owned builder. The Rust builder API is consuming
/// (methods take `self`), so each chained JNI call swaps the builder inside
/// this fixed-address slot, keeping the Java-side pointer valid.
pub struct BuilderSlot {
    inner: std::sync::Mutex<Option<RiValidatorBuilder>>,
}

fn builder_slot_valid(env: &mut JNIEnv, ptr: jlong) -> bool {
    if !is_valid_ptr(&VALIDATOR_BUILDER_REGISTRY, ptr as usize) {
        let _ = env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidatorBuilder pointer");
        return false;
    }
    true
}

fn modify_builder(
    env: &mut JNIEnv,
    ptr: jlong,
    f: impl FnOnce(RiValidatorBuilder) -> RiValidatorBuilder,
) {
    if !builder_slot_valid(env, ptr) {
        return;
    }
    let slot = unsafe { &*(ptr as *const BuilderSlot) };
    if let Ok(mut guard) = slot.inner.lock() {
        if let Some(builder) = guard.take() {
            *guard = Some(f(builder));
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_new0(
    mut env: JNIEnv,
    _class: JClass,
    field_name: JString,
) -> jlong {
    let Some(field_name) = get_string_arg(&mut env, &field_name, "fieldName") else { return 0 };
    let slot = Box::new(BuilderSlot {
        inner: std::sync::Mutex::new(Some(RiValidatorBuilder::new(&field_name))),
    });
    let ptr = Box::into_raw(slot) as jlong;
    register_ptr(&VALIDATOR_BUILDER_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_notEmpty0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    modify_builder(&mut env, ptr, |b| b.not_empty());
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_minLength0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    min: jint,
) {
    modify_builder(&mut env, ptr, |b| b.min_length(min.max(0) as usize));
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_maxLength0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max: jint,
) {
    modify_builder(&mut env, ptr, |b| b.max_length(max.max(0) as usize));
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_isEmail0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    modify_builder(&mut env, ptr, |b| b.is_email());
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_build0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !builder_slot_valid(&mut env, ptr) {
        return 0;
    }
    let slot = unsafe { &*(ptr as *const BuilderSlot) };
    let builder = match slot.inner.lock() {
        Ok(mut guard) => guard.take(),
        Err(_) => None,
    };
    match builder {
        Some(builder) => {
            let runner = builder.build();
            let runner_ptr = Box::into_raw(Box::new(runner)) as jlong;
            register_ptr(&VALIDATION_RUNNER_REGISTRY, runner_ptr as usize);
            runner_ptr
        }
        None => {
            let _ = env.throw_new("java/lang/IllegalStateException", "RiValidatorBuilder already consumed");
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&VALIDATOR_BUILDER_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut BuilderSlot);
        }
    }
}
