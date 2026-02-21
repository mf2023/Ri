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

//! # Service Mesh Module JNI Bindings
//!
//! JNI bindings for DMSC service mesh classes.

use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jlong;
use crate::service_mesh::DMSCServiceMesh;
use crate::java::exception::check_not_null;

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_servicemesh_DMSCServiceMesh_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "DMSCServiceMeshConfig") {
        return 0;
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_servicemesh_DMSCServiceMesh_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCServiceMesh);
        }
    }
}
