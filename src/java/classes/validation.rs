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

//! # Validation Module JNI Bindings
//!
//! JNI bindings for DMSC validation classes.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::{jlong, jboolean};
use crate::validation::{DMSCValidatorBuilder, DMSCValidationResult};
use crate::java::exception::check_not_null;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidationModule_nativeValidateEmail(
    mut env: JNIEnv,
    _class: JClass,
    value: jni::objects::JString,
) -> jlong {
    let value_str: String = env.get_string(&value)
        .expect("Failed to get email value")
        .into();
    
    let result = DMSCValidatorBuilder::new("email")
        .is_email()
        .max_length(255)
        .build()
        .validate_value(Some(&value_str));
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidationModule_nativeValidateUsername(
    mut env: JNIEnv,
    _class: JClass,
    value: jni::objects::JString,
) -> jlong {
    let value_str: String = env.get_string(&value)
        .expect("Failed to get username value")
        .into();
    
    let result = DMSCValidatorBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(32)
        .alphanumeric()
        .build()
        .validate_value(Some(&value_str));
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidationResult_isValid(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "DMSCValidationResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const DMSCValidationResult) };
    result.is_valid as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidationResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCValidationResult);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidatorBuilder_new0(
    mut env: JNIEnv,
    _class: JClass,
    field_name: jni::objects::JString,
) -> jlong {
    let name: String = env.get_string(&field_name)
        .expect("Failed to get field name")
        .into();
    
    let builder = DMSCValidatorBuilder::new(&name);
    Box::into_raw(Box::new(builder)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidatorBuilder_build(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCValidatorBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut DMSCValidatorBuilder) };
    let runner = builder.build();
    Box::into_raw(Box::new(runner)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_validation_DMSCValidatorBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCValidatorBuilder);
        }
    }
}
