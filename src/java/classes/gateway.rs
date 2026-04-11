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

//! # Gateway Module JNI Bindings
//!
//! JNI bindings for Ri gateway classes.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use crate::gateway::{RiGateway, RiGatewayConfig};

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGateway_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let gateway = Box::new(RiGateway::new());
    Box::into_raw(gateway) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGateway_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiGateway);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGatewayConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiGatewayConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGatewayConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiGatewayConfig);
        }
    }
}
