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

//! # FS Module JNI Bindings
//!
//! JNI bindings for Ri filesystem classes.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use crate::fs::RiFileSystem;
use crate::java::exception::throw_illegal_argument;
use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_fs_RiFileSystem_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let fs_boxed = Box::new(RiFileSystem::new_auto_root().unwrap_or_else(|_| RiFileSystem::new_with_root(std::env::current_dir().unwrap_or_default())));
    let ptr = Box::into_raw(fs_boxed);
    register_jni_ptr(ptr as usize);
    ptr as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_fs_RiFileSystem_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiFileSystem);
        }
    }
}
