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

//! # WebSocket Module JNI Bindings
//!
//! JNI bindings for Ri WebSocket classes.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_ws_RiWSServer_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_ws_RiWSServer_free0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_ws_RiWSClient_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_ws_RiWSClient_free0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
}
