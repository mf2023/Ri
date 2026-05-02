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

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jstring};
use crate::validation::{
    RiValidationModule, RiValidationError, RiValidationSeverity, RiValidationResult,
    RiSanitizer, RiSanitizationConfig, RiSchemaValidator,
};
use crate::java::exception::check_not_null;

// =============================================================================
// RiValidationModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let module = Box::new(RiValidationModule::new());
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_validate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiValidationModule") {
        return 0;
    }
    
    let data_str: String = env.get_string(&data)
        .expect("Failed to get data")
        .into();
    
    let module = unsafe { &*(ptr as *const RiValidationModule) };
    let result = Box::new(module.validate(&data_str));
    Box::into_raw(result) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
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
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiValidationError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiValidationError) };
    env.new_string(&error.field).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getMessage0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiValidationError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiValidationError) };
    env.new_string(&error.message).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getCode0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiValidationError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiValidationError) };
    env.new_string(&error.code).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationError_getSeverity0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiValidationError") {
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
    if ptr != 0 {
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
    if !check_not_null(&mut env, ptr, "RiValidationResult") {
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
    if !check_not_null(&mut env, ptr, "RiValidationResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const RiValidationResult) };
    let errors = Box::new(result.errors.clone());
    Box::into_raw(errors) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
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
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizationConfig_isTrimWhitespace0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if !check_not_null(&mut env, ptr, "RiSanitizationConfig") {
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
    if ptr != 0 {
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
    Box::into_raw(sanitizer) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_newWithConfig0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiSanitizationConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiSanitizationConfig) };
    let sanitizer = Box::new(RiSanitizer::with_config(config.clone()));
    Box::into_raw(sanitizer) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitize0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiSanitizer") {
        return std::ptr::null_mut();
    }
    
    let input_str: String = env.get_string(&input)
        .expect("Failed to get input")
        .into();
    
    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize(&input_str);
    env.new_string(&result).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitizeEmail0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiSanitizer") {
        return std::ptr::null_mut();
    }
    
    let input_str: String = env.get_string(&input)
        .expect("Failed to get input")
        .into();
    
    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize_email(&input_str);
    env.new_string(&result).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_sanitizeFilename0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
    input: JString,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiSanitizer") {
        return std::ptr::null_mut();
    }
    
    let input_str: String = env.get_string(&input)
        .expect("Failed to get input")
        .into();
    
    let sanitizer = unsafe { &*(ptr as *const RiSanitizer) };
    let result = sanitizer.sanitize_filename(&input_str);
    env.new_string(&result).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSanitizer_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
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
    let schema_str: String = env.get_string(&schema_json)
        .expect("Failed to get schema")
        .into();
    
    match RiSchemaValidator::new(&schema_str) {
        Ok(validator) => {
            let validator = Box::new(validator);
            Box::into_raw(validator) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSchemaValidator_validate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    data_json: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiSchemaValidator") {
        return 0;
    }
    
    let data_str: String = env.get_string(&data_json)
        .expect("Failed to get data")
        .into();
    
    let validator = unsafe { &*(ptr as *const RiSchemaValidator) };
    let result = Box::new(validator.validate(&data_str));
    Box::into_raw(result) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiSchemaValidator_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSchemaValidator);
        }
    }
}
