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
use jni::objects::JClass;
use jni::sys::{jlong, jboolean};
use crate::validation::{RiValidatorBuilder, RiValidationResult};
use crate::java::exception::check_not_null;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateEmail(
    mut env: JNIEnv,
    _class: JClass,
    value: jni::objects::JString,
) -> jlong {
    let value_str: String = env.get_string(&value)
        .expect("Failed to get email value")
        .into();
    
    let result = RiValidatorBuilder::new("email")
        .is_email()
        .max_length(255)
        .build()
        .validate_value(Some(&value_str));
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationModule_nativeValidateUsername(
    mut env: JNIEnv,
    _class: JClass,
    value: jni::objects::JString,
) -> jlong {
    let value_str: String = env.get_string(&value)
        .expect("Failed to get username value")
        .into();
    
    let result = RiValidatorBuilder::new("username")
        .not_empty()
        .min_length(3)
        .max_length(32)
        .alphanumeric()
        .build()
        .validate_value(Some(&value_str));
    Box::into_raw(Box::new(result)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidationResult_isValid(
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

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_new0(
    mut env: JNIEnv,
    _class: JClass,
    field_name: jni::objects::JString,
) -> jlong {
    let name: String = env.get_string(&field_name)
        .expect("Failed to get field name")
        .into();
    
    let builder = RiValidatorBuilder::new(&name);
    Box::into_raw(Box::new(builder)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_build(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiValidatorBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut RiValidatorBuilder) };
    let runner = builder.build();
    Box::into_raw(Box::new(runner)) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_validation_RiValidatorBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiValidatorBuilder);
        }
    }
}
