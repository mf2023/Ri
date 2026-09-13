//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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

use std::sync::Arc;

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jbyteArray, jstring, jobject};
use jni::objects::JValue;
use crate::module_rpc::{RiModuleRPC, RiModuleClient, RiModuleEndpoint, RiMethodCall, RiMethodResponse};
use crate::java::exception::{check_not_null, throw_ri_error};

/// Converts a Java string argument into a `String`, throwing a RiError and
/// returning `None` when the argument cannot be read.
fn read_string(env: &mut JNIEnv, value: &JString, arg: &str) -> Option<String> {
    match env.get_string(value) {
        Ok(s) => Some(s.into()),
        Err(e) => {
            throw_ri_error(env, &format!("Failed to read {arg}: {e}"));
            None
        }
    }
}

/// Converts a Java byte array argument into a `Vec<u8>`, returning an empty
/// vector for a null array.
fn read_byte_array(env: &mut JNIEnv, array: jbyteArray, arg: &str) -> Option<Vec<u8>> {
    if array.is_null() {
        return Some(Vec::new());
    }
    match env.convert_byte_array(unsafe { jni::objects::JByteArray::from_raw(array) }) {
        Ok(v) => Some(v),
        Err(e) => {
            throw_ri_error(env, &format!("Failed to read {arg}: {e}"));
            None
        }
    }
}

/// Copies a `Vec<u8>` into a fresh Java byte array, returning a null pointer
/// when allocation fails.
fn write_byte_array(env: &mut JNIEnv, data: &[u8]) -> jbyteArray {
    match env.byte_array_from_slice(data) {
        Ok(array) => array.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Clones the `Arc<RiModuleRPC>` behind a raw `Arc::into_raw` pointer without
/// consuming the original reference.
///
/// # Safety
///
/// `ptr` must have been produced by `Arc::into_raw` over an `Arc<RiModuleRPC>`
/// that is still alive.
unsafe fn clone_rpc_arc(ptr: jlong) -> Option<Arc<RiModuleRPC>> {
    if ptr == 0 {
        return None;
    }
    let arc = Arc::from_raw(ptr as *const RiModuleRPC);
    let cloned = arc.clone();
    let _ = Arc::into_raw(arc);
    Some(cloned)
}

// =============================================================================
// RiModuleRPC JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Arc::into_raw(Arc::new(RiModuleRPC::new())) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleRPC_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Arc::from_raw(ptr as *const RiModuleRPC);
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
    match unsafe { clone_rpc_arc(rpc_ptr) } {
        Some(arc) => Box::into_raw(Box::new(RiModuleClient::new(arc))) as jlong,
        None => {
            throw_ri_error(&mut env, "RiModuleRPC pointer is null");
            0
        }
    }
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
    let (Some(module), Some(method), Some(params_vec)) = (
        read_string(&mut env, &module_name, "module name"),
        read_string(&mut env, &method_name, "method name"),
        read_byte_array(&mut env, params, "params"),
    ) else {
        return 0;
    };

    let client = unsafe { &*(ptr as *const RiModuleClient) };
    crate::java::runtime::block_on_jni(&mut env, "RiModuleClient::call", async move {
        client.call(&module, &method, params_vec).await
    })
    .map(|response| Box::into_raw(Box::new(response)) as jlong)
    .unwrap_or(0)
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
    let (Some(module), Some(method), Some(params_vec)) = (
        read_string(&mut env, &module_name, "module name"),
        read_string(&mut env, &method_name, "method name"),
        read_byte_array(&mut env, params, "params"),
    ) else {
        return 0;
    };
    let timeout = crate::java::runtime::ttl_from_jlong(timeout_ms).unwrap_or(5000);

    let client = unsafe { &*(ptr as *const RiModuleClient) };
    crate::java::runtime::block_on_jni(&mut env, "RiModuleClient::call_with_timeout", async move {
        client
            .call_with_timeout(&module, &method, params_vec, timeout)
            .await
    })
    .map(|response| Box::into_raw(Box::new(response)) as jlong)
    .unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleClient_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiModuleClient);
        }
    }
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
    let Some(module) = read_string(&mut env, &module_name, "module name") else {
        return 0;
    };

    let endpoint = Box::new(RiModuleEndpoint::new(&module));
    Box::into_raw(endpoint) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_getModuleName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiModuleEndpoint") {
        return std::ptr::null_mut();
    }

    let endpoint = unsafe { &*(ptr as *const RiModuleEndpoint) };
    match env.new_string(endpoint.module_name()) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiModuleEndpoint_listMethods0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jobject {
    if !check_not_null(&mut env, ptr, "RiModuleEndpoint") {
        return std::ptr::null_mut();
    }

    let endpoint = unsafe { &*(ptr as *const RiModuleEndpoint) };
    let methods = futures::executor::block_on(endpoint.list_methods());

    // Java declares `List<String>` — must return an ArrayList, not a String[].
    let Ok(list) = env.new_object("java/util/ArrayList", "()V", &[]) else {
        return std::ptr::null_mut();
    };
    for method in methods.iter() {
        let Ok(s) = env.new_string(method) else {
            continue;
        };
        let _ = env.call_method(
            &list,
            "add",
            "(Ljava/lang/Object;)Z",
            &[JValue::Object(&s)],
        );
    }
    list.into_raw()
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
    let (Some(method), Some(params_vec)) = (
        read_string(&mut env, &method_name, "method name"),
        read_byte_array(&mut env, params, "params"),
    ) else {
        return 0;
    };

    let call = Box::new(RiMethodCall::new(method, params_vec));
    Box::into_raw(call) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodCall_getMethodName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMethodCall") {
        return std::ptr::null_mut();
    }

    let call = unsafe { &*(ptr as *const RiMethodCall) };
    match env.new_string(&call.method_name) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
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
    write_byte_array(&mut env, &call.params)
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
    call.timeout_ms as jlong
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
    call.timeout_ms = timeout_ms.max(0) as u64;
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
    response.success as jboolean
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
    write_byte_array(&mut env, &response.data)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_modulerpc_RiMethodResponse_getError0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMethodResponse") {
        return std::ptr::null_mut();
    }

    let response = unsafe { &*(ptr as *const RiMethodResponse) };
    match env.new_string(&response.error) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
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
    response.is_timeout as jboolean
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
