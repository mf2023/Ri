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
use std::sync::Arc;
use dashmap::DashSet;
use std::any::TypeId;

use crate::validation::{
    RiValidationModule, RiValidationError, RiValidationResult,
    RiSanitizer, RiSanitizationConfig, RiSchemaValidator,
};
use crate::java::exception::check_not_null;

lazy_static::lazy_static! {
    static ref VALIDATION_MODULE_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATION_ERROR_REGISTRY: DashSet<usize> = DashSet::new();
    static ref VALIDATION_RESULT_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SANITIZATION_CONFIG_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SANITIZER_REGISTRY: DashSet<usize> = DashSet::new();
    static ref SCHEMA_VALIDATOR_REGISTRY: DashSet<usize> = DashSet::new();
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
// RiValidationModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let module = Box::new(RiValidationModule::new());
    let ptr = Box::into_raw(module) as jlong;
    register_ptr(&VALIDATION_MODULE_REGISTRY, ptr as usize);
    ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_validate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: JString,
) -> jlong {
    if !is_valid_ptr(&VALIDATION_MODULE_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationModule pointer")
            .unwrap_or(());
        return 0;
    }

    let data_str: String = match env.get_string(&data) {
        Ok(s) => s.into(),
        Err(_) => {
            env.throw_new("java/lang/IllegalArgumentException", "Failed to get data string")
                .unwrap_or(());
            return 0;
        }
    };

    let module = unsafe { &*(ptr as *const RiValidationModule) };
    let result = Box::new(module.validate(&data_str));
    let result_ptr = Box::into_raw(result) as jlong;
    register_ptr(&VALIDATION_RESULT_REGISTRY, result_ptr as usize);
    result_ptr
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if unregister_ptr(&VALIDATION_MODULE_REGISTRY, ptr as usize) {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiValidationModule);
        }
    }
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
    error.severity as jint
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
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationResult_getErrors0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !is_valid_ptr(&VALIDATION_RESULT_REGISTRY, ptr as usize) {
        env.throw_new("java/lang/IllegalStateException", "Invalid or freed RiValidationResult pointer")
            .unwrap_or(());
        return 0;
    }

    let result = unsafe { &*(ptr as *const RiValidationResult) };
    let errors = Box::new(result.errors.clone());
    let errors_ptr = Box::into_raw(errors) as jlong;
    register_ptr(&VALIDATION_ERROR_REGISTRY, errors_ptr as usize);
    errors_ptr
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

    match RiSchemaValidator::new(&schema_str) {
        Ok(validator) => {
            let ptr = Box::into_raw(Box::new(validator)) as jlong;
            register_ptr(&SCHEMA_VALIDATOR_REGISTRY, ptr as usize);
            ptr
        }
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", &format!("Failed to create validator: {}", e))
                .unwrap_or(());
            0
        }
    }
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
    let result = Box::new(validator.validate(&data_str));
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
