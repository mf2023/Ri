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

//! # Java Exception Handling
//!
//! Provides utilities for throwing and handling Java exceptions from Rust.

use jni::JNIEnv;
use jni::sys::jlong;

/// DMSC Java exception types
pub enum DMSCJavaException {
    /// General DMSC error
    DMSCErrors,
    /// Configuration error
    ConfigError,
    /// Validation error
    ValidationError,
    /// Authentication error
    AuthError,
    /// Database error
    DatabaseError,
    /// Cache error
    CacheError,
    /// Network error
    NetworkError,
    /// Null pointer error
    NullPointerError,
    /// Illegal argument error
    IllegalArgumentError,
}

impl DMSCJavaException {
    /// Get the Java class name for this exception type
    pub fn class_name(&self) -> &'static str {
        match self {
            DMSCJavaException::DMSCErrors => "com/dunimd/dmsc/DMSCError",
            DMSCJavaException::ConfigError => "com/dunimd/dmsc/DMSCConfigError",
            DMSCJavaException::ValidationError => "com/dunimd/dmsc/validation/DMSCValidationError",
            DMSCJavaException::AuthError => "com/dunimd/dmsc/auth/DMSCAuthError",
            DMSCJavaException::DatabaseError => "com/dunimd/dmsc/database/DMSCDatabaseError",
            DMSCJavaException::CacheError => "com/dunimd/dmsc/cache/DMSCCacheError",
            DMSCJavaException::NetworkError => "com/dunimd/dmsc/DMSCNetworkError",
            DMSCJavaException::NullPointerError => "java/lang/NullPointerException",
            DMSCJavaException::IllegalArgumentError => "java/lang/IllegalArgumentException",
        }
    }
}

/// Throw a Java exception from Rust
pub fn throw_exception(env: &mut JNIEnv, exception_type: DMSCJavaException, message: &str) {
    let class_name = exception_type.class_name();
    
    if let Err(e) = env.throw_new(class_name, message) {
        eprintln!("Failed to throw Java exception: {:?}", e);
    }
}

/// Throw a DMSC error exception
pub fn throw_dmsc_error(env: &mut JNIEnv, message: &str) {
    throw_exception(env, DMSCJavaException::DMSCErrors, message);
}

/// Throw a null pointer exception
pub fn throw_null_pointer(env: &mut JNIEnv, message: &str) {
    throw_exception(env, DMSCJavaException::NullPointerError, message);
}

/// Throw an illegal argument exception
pub fn throw_illegal_argument(env: &mut JNIEnv, message: &str) {
    throw_exception(env, DMSCJavaException::IllegalArgumentError, message);
}

/// Check if a Java object is null and throw exception if so
pub fn check_not_null(env: &mut JNIEnv, ptr: jlong, context: &str) -> bool {
    if ptr == 0 {
        throw_null_pointer(env, &format!("{} pointer is null", context));
        false
    } else {
        true
    }
}

/// Macro to check for null pointer and return early if null
#[macro_export]
macro_rules! check_java_ptr {
    ($env:expr, $ptr:expr, $context:expr) => {
        if !$crate::java::exception::check_not_null($env, $ptr, $context) {
            return;
        }
    };
}

/// Macro to check for null pointer and return default value if null
#[macro_export]
macro_rules! check_java_ptr_or_default {
    ($env:expr, $ptr:expr, $context:expr, $default:expr) => {
        if !$crate::java::exception::check_not_null($env, $ptr, $context) {
            return $default;
        }
    };
}
