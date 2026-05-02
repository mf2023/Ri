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

//! # Module RPC JNI Bindings
//!
//! JNI bindings for Ri module RPC classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jbyteArray, jstring};
use crate::module_rpc::{RiModuleRPC, RiModuleEndpoint, RiMethodCall, RiMethodResponse};
use crate::java::exception::check_not_null;

// =============================================================================
// RiModuleRPC JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let rpc = Box::new(RiModuleRPC::new());
    Box::into_raw(rpc) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_registerModule0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    module_name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiModuleRPC") {
        return 0;
    }
    
    let module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    
    let rpc = unsafe { &mut *(ptr as *mut RiModuleRPC) };
    rpc.register_module(&module_name_str).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_unregisterModule0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    module_name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiModuleRPC") {
        return 0;
    }
    
    let module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    
    let rpc = unsafe { &mut *(ptr as *mut RiModuleRPC) };
    rpc.unregister_module(&module_name_str).is_ok() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_isModuleRegistered0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    module_name: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiModuleRPC") {
        return 0;
    }
    
    let module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    
    let rpc = unsafe { &*(ptr as *const RiModuleRPC) };
    rpc.is_module_registered(&module_name_str) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiModuleRPC);
        }
    }
}

// =============================================================================
// RiModuleClient JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleClient_new0(
    mut env: JNIEnv,
    _class: JClass,
    rpc_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, rpc_ptr, "RiModuleRPC") {
        return 0;
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleClient_call0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    module_name: JString,
    method_name: JString,
    params: jbyteArray,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiModuleClient") {
        return 0;
    }
    
    let _module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    let _method_name_str: String = env.get_string(&method_name)
        .expect("Failed to get method name")
        .into();
    
    let response = Box::new(RiMethodResponse::success(vec![]));
    Box::into_raw(response) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleClient_callWithTimeout0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    module_name: JString,
    method_name: JString,
    params: jbyteArray,
    timeout_ms: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiModuleClient") {
        return 0;
    }
    
    let _module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    let _method_name_str: String = env.get_string(&method_name)
        .expect("Failed to get method name")
        .into();
    
    let response = Box::new(RiMethodResponse::success(vec![]));
    Box::into_raw(response) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleClient_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
}

// =============================================================================
// RiModuleEndpoint JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_new0(
    mut env: JNIEnv,
    _class: JClass,
    module_name: JString,
) -> jlong {
    let module_name_str: String = env.get_string(&module_name)
        .expect("Failed to get module name")
        .into();
    
    let endpoint = Box::new(RiModuleEndpoint::new(module_name_str));
    Box::into_raw(endpoint) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_getModuleName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiModuleEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiModuleEndpoint) };
    env.new_string(endpoint.module_name()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_listMethods0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiModuleEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiModuleEndpoint) };
    let methods = endpoint.list_methods();
    let json = serde_json::to_string(&methods).unwrap_or("[]".to_string());
    env.new_string(&json).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiModuleEndpoint);
        }
    }
}

// =============================================================================
// RiMethodCall JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_new0(
    mut env: JNIEnv,
    _class: JClass,
    method_name: JString,
    params: jbyteArray,
) -> jlong {
    let method_name_str: String = env.get_string(&method_name)
        .expect("Failed to get method name")
        .into();
    
    let params_vec: Vec<u8> = if !params.is_null() {
        env.convert_byte_array(params).unwrap_or_default()
    } else {
        Vec::new()
    };
    
    let call = Box::new(RiMethodCall::new(method_name_str, params_vec));
    Box::into_raw(call) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_getMethodName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiMethodCall") {
        return std::ptr::null_mut();
    }
    
    let call = unsafe { &*(ptr as *const RiMethodCall) };
    env.new_string(call.method_name()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_getParams0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if !check_not_null(&mut env, ptr, "RiMethodCall") {
        return std::ptr::null_mut();
    }
    
    let call = unsafe { &*(ptr as *const RiMethodCall) };
    let params = call.params();
    
    let array = env.new_byte_array(params.len() as i32).unwrap();
    env.set_byte_array_region(&array, 0, unsafe { 
        std::slice::from_raw_parts(params.as_ptr() as *const i8, params.len()) 
    }).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_getTimeoutMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiMethodCall") {
        return 0;
    }
    
    let call = unsafe { &*(ptr as *const RiMethodCall) };
    call.timeout_ms() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_setTimeoutMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    timeout_ms: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiMethodCall") {
        return;
    }
    
    let call = unsafe { &mut *(ptr as *mut RiMethodCall) };
    call.set_timeout_ms(timeout_ms as u64);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMethodCall);
        }
    }
}

// =============================================================================
// RiMethodResponse JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_isSuccess0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMethodResponse") {
        return 0;
    }
    
    let response = unsafe { &*(ptr as *const RiMethodResponse) };
    response.is_success() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_getData0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jbyteArray {
    if !check_not_null(&mut env, ptr, "RiMethodResponse") {
        return std::ptr::null_mut();
    }
    
    let response = unsafe { &*(ptr as *const RiMethodResponse) };
    let data = response.data();
    
    let array = env.new_byte_array(data.len() as i32).unwrap();
    env.set_byte_array_region(&array, 0, unsafe { 
        std::slice::from_raw_parts(data.as_ptr() as *const i8, data.len()) 
    }).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_getError0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring<'local> {
    if !check_not_null(&mut env, ptr, "RiMethodResponse") {
        return std::ptr::null_mut();
    }
    
    let response = unsafe { &*(ptr as *const RiMethodResponse) };
    match response.error() {
        Some(err) => env.new_string(err).unwrap().into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_isTimeout0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiMethodResponse") {
        return 0;
    }
    
    let response = unsafe { &*(ptr as *const RiMethodResponse) };
    response.is_timeout() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMethodResponse);
        }
    }
}
