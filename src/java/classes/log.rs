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
use jni::objects::JClass;
use jni::sys::jlong;
use crate::log::{RiLogger, RiLogConfig};
use crate::fs::RiFileSystem;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = RiLogConfig::default();
    let fs = RiFileSystem::new_auto_root().unwrap_or_else(|_| RiFileSystem::new_with_root(std::env::current_dir().unwrap_or_default()));
    let logger = Box::new(RiLogger::new(&config, fs));
    Box::into_raw(logger) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_log_RiLogger_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLogger);
        }
    }
}
