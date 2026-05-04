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

//! # Service Mesh Module JNI Bindings
//!
//! JNI bindings for Ri service mesh classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString, JObjectArray};
use jni::sys::{jlong, jboolean, jint, jstring, jobjectArray, jdouble, jlongArray};
use std::collections::HashMap as FxHashMap;
use std::time::Duration;

use crate::service_mesh::{
    RiServiceMesh, RiServiceMeshConfig, RiServiceMeshStats,
    RiServiceEndpoint, RiServiceHealthStatus,
    RiServiceDiscovery, RiServiceInstance, RiServiceStatus,
    RiHealthChecker, RiHealthSummary, RiHealthStatus, RiHealthCheckType,
    RiTrafficRoute, RiMatchCriteria, RiRouteAction, RiWeightedDestination, RiTrafficManager,
};
use crate::gateway::RiCircuitBreakerConfig;
use crate::service_mesh::traffic_management::RiRateLimitConfig;
use crate::java::exception::check_not_null;

// =============================================================================
// RiServiceMeshConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiServiceMeshConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceMeshConfig);
        }
    }
}

// =============================================================================
// RiServiceMesh JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMesh_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiServiceMeshConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiServiceMeshConfig) };
    match RiServiceMesh::new(config.clone()) {
        Ok(mesh) => {
            let boxed = Box::new(mesh);
            Box::into_raw(boxed) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMesh_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceMesh);
        }
    }
}

// =============================================================================
// RiServiceDiscovery JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_new0(
    _env: JNIEnv,
    _class: JClass,
    enabled: jboolean,
) -> jlong {
    let discovery = Box::new(RiServiceDiscovery::new(enabled != 0));
    Box::into_raw(discovery) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_register0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
    host: JString,
    port: jint,
    keys: JObjectArray,
    values: JObjectArray,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return std::ptr::null_mut();
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let host_str: String = env.get_string(&host)
        .expect("Failed to get host")
        .into();
    
    let mut metadata = FxHashMap::new();
    let len = env.get_array_length(&keys).unwrap_or(0);
    for i in 0..len {
        let key: JString = env.get_object_array_element(&keys, i)
            .expect("Failed to get key")
            .into();
        let value: JString = env.get_object_array_element(&values, i)
            .expect("Failed to get value")
            .into();
        let key_str: String = env.get_string(&key)
            .expect("Failed to get key string")
            .into();
        let value_str: String = env.get_string(&value)
            .expect("Failed to get value string")
            .into();
        metadata.insert(key_str, value_str);
    }
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        discovery.register_service(&service_name_str, &host_str, port as u16, metadata).await
    });
    
    match result {
        Ok(instance_id) => env.new_string(&instance_id)
            .expect("Failed to create string")
            .into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_deregister0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    instance_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return;
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    let instance_id_str: String = env.get_string(&instance_id)
        .expect("Failed to get instance id")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        discovery.deregister_service(&instance_id_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_discover0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return std::ptr::null_mut();
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        discovery.discover_service(&service_name_str).await
    });
    
    match result {
        Ok(instances) => {
            let ptrs: Vec<jlong> = instances.iter().map(|i| {
                let boxed = Box::new(i.clone());
                Box::into_raw(boxed) as jlong
            }).collect();
            
            let array = env.new_long_array(ptrs.len() as i32)
                .expect("Failed to create long array");
            env.set_long_array_region(array, 0, &ptrs)
                .expect("Failed to set long array");
            array
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_updateHeartbeat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    instance_id: JString,
) {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return;
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    let instance_id_str: String = env.get_string(&instance_id)
        .expect("Failed to get instance id")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        discovery.update_heartbeat(&instance_id_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_setServiceStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    instance_id: JString,
    status_ordinal: jint,
) {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return;
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    let instance_id_str: String = env.get_string(&instance_id)
        .expect("Failed to get instance id")
        .into();
    
    let status = match status_ordinal {
        0 => RiServiceStatus::Starting,
        1 => RiServiceStatus::Running,
        2 => RiServiceStatus::Stopping,
        3 => RiServiceStatus::Stopped,
        4 => RiServiceStatus::Unhealthy,
        _ => RiServiceStatus::Starting,
    };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        discovery.set_service_status(&instance_id_str, status).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_getAllServices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiServiceDiscovery") {
        return std::ptr::null_mut();
    }
    
    let discovery = unsafe { &*(ptr as *const RiServiceDiscovery) };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        discovery.get_all_services().await
    });
    
    match result {
        Ok(services) => {
            let string_class = env.find_class("java/lang/String")
                .expect("Failed to find String class");
            let array = env.new_object_array(services.len() as i32, string_class, std::ptr::null_mut())
                .expect("Failed to create string array");
            
            for (i, service) in services.iter().enumerate() {
                let jstr = env.new_string(service)
                    .expect("Failed to create string")
                    .into_raw();
                env.set_object_array_element(&array, i as i32, jstr)
                    .expect("Failed to set array element");
            }
            
            array
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceDiscovery_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceDiscovery);
        }
    }
}

// =============================================================================
// RiServiceInstance JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    service_name: JString,
    host: JString,
    port: jint,
) -> jlong {
    let id_str: String = env.get_string(&id)
        .expect("Failed to get id")
        .into();
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let host_str: String = env.get_string(&host)
        .expect("Failed to get host")
        .into();
    
    let instance = Box::new(RiServiceInstance {
        id: id_str,
        service_name: service_name_str,
        host: host_str,
        port: port as u16,
        metadata: FxHashMap::default(),
        registered_at: std::time::SystemTime::now(),
        last_heartbeat: std::time::SystemTime::now(),
        status: RiServiceStatus::Starting,
    });
    Box::into_raw(instance) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getId0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return std::ptr::null_mut();
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    env.new_string(&instance.id)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getServiceName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return std::ptr::null_mut();
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    env.new_string(&instance.service_name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getHost0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return std::ptr::null_mut();
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    env.new_string(&instance.host)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getPort0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return 0;
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    instance.port as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return 0;
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    match instance.status {
        RiServiceStatus::Starting => 0,
        RiServiceStatus::Running => 1,
        RiServiceStatus::Stopping => 2,
        RiServiceStatus::Stopped => 3,
        RiServiceStatus::Unhealthy => 4,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getMetadataKeys0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return std::ptr::null_mut();
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    let keys: Vec<&String> = instance.metadata.keys().collect();
    
    let string_class = env.find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env.new_object_array(keys.len() as i32, string_class, std::ptr::null_mut())
        .expect("Failed to create string array");
    
    for (i, key) in keys.iter().enumerate() {
        let jstr = env.new_string(key)
            .expect("Failed to create string")
            .into_raw();
        env.set_object_array_element(&array, i as i32, jstr)
            .expect("Failed to set array element");
    }
    
    array
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_getMetadataValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceInstance") {
        return std::ptr::null_mut();
    }
    
    let instance = unsafe { &*(ptr as *const RiServiceInstance) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match instance.metadata.get(&key_str) {
        Some(value) => env.new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceInstance_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceInstance);
        }
    }
}

// =============================================================================
// RiServiceMeshStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let stats = Box::new(RiServiceMeshStats {
        total_services: 0,
        total_endpoints: 0,
        healthy_endpoints: 0,
        unhealthy_endpoints: 0,
    });
    Box::into_raw(stats) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_getTotalServices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceMeshStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiServiceMeshStats) };
    stats.total_services as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_getTotalEndpoints0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceMeshStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiServiceMeshStats) };
    stats.total_endpoints as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_getHealthyEndpoints0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceMeshStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiServiceMeshStats) };
    stats.healthy_endpoints as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_getUnhealthyEndpoints0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceMeshStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiServiceMeshStats) };
    stats.unhealthy_endpoints as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceMeshStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceMeshStats);
        }
    }
}

// =============================================================================
// RiServiceEndpoint JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_new0(
    mut env: JNIEnv,
    _class: JClass,
    service_name: JString,
    endpoint: JString,
    weight: jint,
) -> jlong {
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let endpoint_str: String = env.get_string(&endpoint)
        .expect("Failed to get endpoint")
        .into();
    
    let endpoint = Box::new(RiServiceEndpoint {
        service_name: service_name_str,
        endpoint: endpoint_str,
        weight: weight as u32,
        metadata: FxHashMap::default(),
        health_status: RiServiceHealthStatus::Unknown,
        last_health_check: std::time::SystemTime::now(),
    });
    Box::into_raw(endpoint) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getServiceName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    env.new_string(&endpoint.service_name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getEndpoint0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    env.new_string(&endpoint.endpoint)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getWeight0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return 0;
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    endpoint.weight as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getHealthStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return 0;
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    match endpoint.health_status {
        RiServiceHealthStatus::Healthy => 0,
        RiServiceHealthStatus::Unhealthy => 1,
        RiServiceHealthStatus::Unknown => 2,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getMetadataKeys0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    let keys: Vec<&String> = endpoint.metadata.keys().collect();
    
    let string_class = env.find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env.new_object_array(keys.len() as i32, string_class, std::ptr::null_mut())
        .expect("Failed to create string array");
    
    for (i, key) in keys.iter().enumerate() {
        let jstr = env.new_string(key)
            .expect("Failed to create string")
            .into_raw();
        env.set_object_array_element(&array, i as i32, jstr)
            .expect("Failed to set array element");
    }
    
    array
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_getMetadataValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiServiceEndpoint") {
        return std::ptr::null_mut();
    }
    
    let endpoint = unsafe { &*(ptr as *const RiServiceEndpoint) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match endpoint.metadata.get(&key_str) {
        Some(value) => env.new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiServiceEndpoint_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceEndpoint);
        }
    }
}

// =============================================================================
// RiHealthChecker JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_new0(
    _env: JNIEnv,
    _class: JClass,
    check_interval_seconds: jlong,
) -> jlong {
    let checker = Box::new(RiHealthChecker::new(Duration::from_secs(check_interval_seconds as u64)));
    Box::into_raw(checker) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_startHealthCheck0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
    endpoint: JString,
) {
    if !check_not_null(&mut env, ptr, "RiHealthChecker") {
        return;
    }
    
    let checker = unsafe { &*(ptr as *const RiHealthChecker) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let endpoint_str: String = env.get_string(&endpoint)
        .expect("Failed to get endpoint")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        checker.start_health_check(&service_name_str, &endpoint_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_startHealthCheckWithType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
    endpoint: JString,
    check_type_ordinal: jint,
) {
    if !check_not_null(&mut env, ptr, "RiHealthChecker") {
        return;
    }
    
    let checker = unsafe { &*(ptr as *const RiHealthChecker) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let endpoint_str: String = env.get_string(&endpoint)
        .expect("Failed to get endpoint")
        .into();
    
    let check_type = match check_type_ordinal {
        0 => RiHealthCheckType::Http,
        1 => RiHealthCheckType::Tcp,
        2 => RiHealthCheckType::Grpc,
        3 => RiHealthCheckType::Custom,
        _ => RiHealthCheckType::Http,
    };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        checker.start_health_check_with_type(&service_name_str, &endpoint_str, check_type).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_stopHealthCheck0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
    endpoint: JString,
) {
    if !check_not_null(&mut env, ptr, "RiHealthChecker") {
        return;
    }
    
    let checker = unsafe { &*(ptr as *const RiHealthChecker) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    let endpoint_str: String = env.get_string(&endpoint)
        .expect("Failed to get endpoint")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        checker.stop_health_check(&service_name_str, &endpoint_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_getServiceHealthSummary0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service_name: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthChecker") {
        return 0;
    }
    
    let checker = unsafe { &*(ptr as *const RiHealthChecker) };
    let service_name_str: String = env.get_string(&service_name)
        .expect("Failed to get service name")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        checker.get_service_health_summary(&service_name_str).await
    });
    
    match result {
        Ok(summary) => {
            let boxed = Box::new(summary);
            Box::into_raw(boxed) as jlong
        }
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthChecker_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHealthChecker);
        }
    }
}

// =============================================================================
// RiHealthSummary JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getServiceName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return std::ptr::null_mut();
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    env.new_string(&summary.service_name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getTotalChecks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    summary.total_checks as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getHealthyChecks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    summary.healthy_checks as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getUnhealthyChecks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    summary.unhealthy_checks as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getSuccessRate0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0.0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    summary.success_rate
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getAverageResponseTimeMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    summary.average_response_time.as_millis() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_getOverallStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthSummary") {
        return 0;
    }
    
    let summary = unsafe { &*(ptr as *const RiHealthSummary) };
    match summary.overall_status {
        RiHealthStatus::Healthy => 0,
        RiHealthStatus::Degraded => 1,
        RiHealthStatus::Unhealthy => 2,
        RiHealthStatus::Unknown => 3,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiHealthSummary_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHealthSummary);
        }
    }
}

// =============================================================================
// RiTrafficRoute JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_new0(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    source_service: JString,
    destination_service: JString,
) -> jlong {
    let name_str: String = env.get_string(&name)
        .expect("Failed to get name")
        .into();
    let source_service_str: String = env.get_string(&source_service)
        .expect("Failed to get source service")
        .into();
    let destination_service_str: String = env.get_string(&destination_service)
        .expect("Failed to get destination service")
        .into();
    
    let route = Box::new(RiTrafficRoute {
        name: name_str,
        source_service: source_service_str,
        destination_service: destination_service_str,
        match_criteria: RiMatchCriteria {
            path_prefix: None,
            headers: FxHashMap::default(),
            method: None,
            query_parameters: FxHashMap::default(),
        },
        route_action: RiRouteAction::Route(vec![]),
        retry_policy: None,
        timeout: None,
        fault_injection: None,
    });
    Box::into_raw(route) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return std::ptr::null_mut();
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    env.new_string(&route.name)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getSourceService0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return std::ptr::null_mut();
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    env.new_string(&route.source_service)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getDestinationService0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return std::ptr::null_mut();
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    env.new_string(&route.destination_service)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getMatchCriteria0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return 0;
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    let criteria = Box::new(route.match_criteria.clone());
    Box::into_raw(criteria) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_setMatchCriteria0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    criteria_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return;
    }
    if !check_not_null(&mut env, criteria_ptr, "RiMatchCriteria") {
        return;
    }
    
    let route = unsafe { &mut *(ptr as *mut RiTrafficRoute) };
    let criteria = unsafe { &*(criteria_ptr as *const RiMatchCriteria) };
    route.match_criteria = criteria.clone();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getRouteAction0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return 0;
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    let action = Box::new(route.route_action.clone());
    Box::into_raw(action) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_setRouteAction0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    action_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return;
    }
    if !check_not_null(&mut env, action_ptr, "RiRouteAction") {
        return;
    }
    
    let route = unsafe { &mut *(ptr as *mut RiTrafficRoute) };
    let action = unsafe { &*(action_ptr as *const RiRouteAction) };
    route.route_action = action.clone();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getTimeoutMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return 0;
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    route.timeout.map(|t| t.as_millis() as jlong).unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_setTimeoutMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    timeout_ms: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return;
    }
    
    let route = unsafe { &mut *(ptr as *mut RiTrafficRoute) };
    route.timeout = Some(Duration::from_millis(timeout_ms as u64));
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_getRetryAttempts0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return 0;
    }
    
    let route = unsafe { &*(ptr as *const RiTrafficRoute) };
    route.retry_policy.as_ref().map(|p| p.attempts as jint).unwrap_or(0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_setRetryAttempts0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    attempts: jint,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficRoute") {
        return;
    }
    
    let route = unsafe { &mut *(ptr as *mut RiTrafficRoute) };
    if let Some(ref mut policy) = route.retry_policy {
        policy.attempts = attempts as u32;
    } else {
        use crate::service_mesh::traffic_management::RiRetryPolicy;
        route.retry_policy = Some(RiRetryPolicy {
            attempts: attempts as u32,
            per_try_timeout: Duration::from_secs(1),
            retry_on: vec!["5xx".to_string()],
        });
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficRoute_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiTrafficRoute);
        }
    }
}

// =============================================================================
// RiMatchCriteria JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let criteria = Box::new(RiMatchCriteria {
        path_prefix: None,
        headers: FxHashMap::default(),
        method: None,
        query_parameters: FxHashMap::default(),
    });
    Box::into_raw(criteria) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getPathPrefix0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    match &criteria.path_prefix {
        Some(prefix) => env.new_string(prefix)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_setPathPrefix0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path_prefix: JString,
) {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return;
    }
    
    let criteria = unsafe { &mut *(ptr as *mut RiMatchCriteria) };
    let prefix_str: String = env.get_string(&path_prefix)
        .expect("Failed to get path prefix")
        .into();
    criteria.path_prefix = Some(prefix_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getMethod0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    match &criteria.method {
        Some(method) => env.new_string(method)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_setMethod0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    method: JString,
) {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return;
    }
    
    let criteria = unsafe { &mut *(ptr as *mut RiMatchCriteria) };
    let method_str: String = env.get_string(&method)
        .expect("Failed to get method")
        .into();
    criteria.method = Some(method_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getHeaderKeys0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    let keys: Vec<&String> = criteria.headers.keys().collect();
    
    let string_class = env.find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env.new_object_array(keys.len() as i32, string_class, std::ptr::null_mut())
        .expect("Failed to create string array");
    
    for (i, key) in keys.iter().enumerate() {
        let jstr = env.new_string(key)
            .expect("Failed to create string")
            .into_raw();
        env.set_object_array_element(&array, i as i32, jstr)
            .expect("Failed to set array element");
    }
    
    array
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getHeaderValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match criteria.headers.get(&key_str) {
        Some(value) => env.new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_addHeader0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
) {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return;
    }
    
    let criteria = unsafe { &mut *(ptr as *mut RiMatchCriteria) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    let value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
    criteria.headers.insert(key_str, value_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getQueryParamKeys0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    let keys: Vec<&String> = criteria.query_parameters.keys().collect();
    
    let string_class = env.find_class("java/lang/String")
        .expect("Failed to find String class");
    let array = env.new_object_array(keys.len() as i32, string_class, std::ptr::null_mut())
        .expect("Failed to create string array");
    
    for (i, key) in keys.iter().enumerate() {
        let jstr = env.new_string(key)
            .expect("Failed to create string")
            .into_raw();
        env.set_object_array_element(&array, i as i32, jstr)
            .expect("Failed to set array element");
    }
    
    array
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_getQueryParamValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return std::ptr::null_mut();
    }
    
    let criteria = unsafe { &*(ptr as *const RiMatchCriteria) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match criteria.query_parameters.get(&key_str) {
        Some(value) => env.new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_addQueryParameter0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
) {
    if !check_not_null(&mut env, ptr, "RiMatchCriteria") {
        return;
    }
    
    let criteria = unsafe { &mut *(ptr as *mut RiMatchCriteria) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    let value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
    criteria.query_parameters.insert(key_str, value_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiMatchCriteria_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiMatchCriteria);
        }
    }
}

// =============================================================================
// RiRouteAction JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_route0(
    mut env: JNIEnv,
    _class: JClass,
    destination_ptrs: jlongArray,
) -> jlong {
    let len = env.get_array_length(&destination_ptrs).unwrap_or(0);
    let mut ptrs = vec![0i64; len as usize];
    env.get_long_array_region(destination_ptrs, 0, &mut ptrs)
        .expect("Failed to get long array");
    
    let destinations: Vec<RiWeightedDestination> = ptrs.iter()
        .filter(|&&p| p != 0)
        .map(|&p| {
            let dest = unsafe { &*(p as *const RiWeightedDestination) };
            dest.clone()
        })
        .collect();
    
    let action = Box::new(RiRouteAction::Route(destinations));
    Box::into_raw(action) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_redirect0(
    mut env: JNIEnv,
    _class: JClass,
    uri: JString,
) -> jlong {
    let uri_str: String = env.get_string(&uri)
        .expect("Failed to get uri")
        .into();
    
    let action = Box::new(RiRouteAction::Redirect(uri_str));
    Box::into_raw(action) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_directResponse0(
    mut env: JNIEnv,
    _class: JClass,
    status_code: jint,
    body: JString,
) -> jlong {
    let body_str: String = env.get_string(&body)
        .expect("Failed to get body")
        .into();
    
    let action = Box::new(RiRouteAction::DirectResponse(status_code as u16, body_str));
    Box::into_raw(action) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_getType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiRouteAction") {
        return 0;
    }
    
    let action = unsafe { &*(ptr as *const RiRouteAction) };
    match action {
        RiRouteAction::Route(_) => 0,
        RiRouteAction::Redirect(_) => 1,
        RiRouteAction::DirectResponse(_, _) => 2,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_getDestinations0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiRouteAction") {
        return std::ptr::null_mut();
    }
    
    let action = unsafe { &*(ptr as *const RiRouteAction) };
    
    match action {
        RiRouteAction::Route(destinations) => {
            let ptrs: Vec<jlong> = destinations.iter().map(|d| {
                let boxed = Box::new(d.clone());
                Box::into_raw(boxed) as jlong
            }).collect();
            
            let array = env.new_long_array(ptrs.len() as i32)
                .expect("Failed to create long array");
            env.set_long_array_region(array, 0, &ptrs)
                .expect("Failed to set long array");
            array
        }
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_getRedirectUri0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRouteAction") {
        return std::ptr::null_mut();
    }
    
    let action = unsafe { &*(ptr as *const RiRouteAction) };
    
    match action {
        RiRouteAction::Redirect(uri) => env.new_string(uri)
            .expect("Failed to create string")
            .into_raw(),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_getDirectResponseStatusCode0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiRouteAction") {
        return 0;
    }
    
    let action = unsafe { &*(ptr as *const RiRouteAction) };
    
    match action {
        RiRouteAction::DirectResponse(status_code, _) => *status_code as jint,
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_getDirectResponseBody0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiRouteAction") {
        return std::ptr::null_mut();
    }
    
    let action = unsafe { &*(ptr as *const RiRouteAction) };
    
    match action {
        RiRouteAction::DirectResponse(_, body) => env.new_string(body)
            .expect("Failed to create string")
            .into_raw(),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiRouteAction_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRouteAction);
        }
    }
}

// =============================================================================
// RiWeightedDestination JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_new0(
    mut env: JNIEnv,
    _class: JClass,
    service: JString,
    weight: jint,
) -> jlong {
    let service_str: String = env.get_string(&service)
        .expect("Failed to get service")
        .into();
    
    let dest = Box::new(RiWeightedDestination {
        service: service_str,
        weight: weight as u32,
        subset: None,
    });
    Box::into_raw(dest) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_getService0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiWeightedDestination") {
        return std::ptr::null_mut();
    }
    
    let dest = unsafe { &*(ptr as *const RiWeightedDestination) };
    env.new_string(&dest.service)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_getWeight0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiWeightedDestination") {
        return 0;
    }
    
    let dest = unsafe { &*(ptr as *const RiWeightedDestination) };
    dest.weight as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_getSubset0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiWeightedDestination") {
        return std::ptr::null_mut();
    }
    
    let dest = unsafe { &*(ptr as *const RiWeightedDestination) };
    match &dest.subset {
        Some(subset) => env.new_string(subset)
            .expect("Failed to create string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_setSubset0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    subset: JString,
) {
    if !check_not_null(&mut env, ptr, "RiWeightedDestination") {
        return;
    }
    
    let dest = unsafe { &mut *(ptr as *mut RiWeightedDestination) };
    let subset_str: String = env.get_string(&subset)
        .expect("Failed to get subset")
        .into();
    dest.subset = Some(subset_str);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiWeightedDestination_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiWeightedDestination);
        }
    }
}

// =============================================================================
// RiTrafficManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_new0(
    _env: JNIEnv,
    _class: JClass,
    enabled: jboolean,
) -> jlong {
    let manager = Box::new(RiTrafficManager::new(enabled != 0));
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_addRoute0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    route_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficManager") {
        return;
    }
    if !check_not_null(&mut env, route_ptr, "RiTrafficRoute") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiTrafficManager) };
    let route = unsafe { &*(route_ptr as *const RiTrafficRoute) };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.add_traffic_route(route.clone()).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_removeRoute0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    source_service: JString,
    route_name: JString,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiTrafficManager) };
    let source_service_str: String = env.get_string(&source_service)
        .expect("Failed to get source service")
        .into();
    let route_name_str: String = env.get_string(&route_name)
        .expect("Failed to get route name")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.remove_traffic_route(&source_service_str, &route_name_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_getRoutes0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    source_service: JString,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiTrafficManager") {
        return std::ptr::null_mut();
    }
    
    let manager = unsafe { &*(ptr as *const RiTrafficManager) };
    let source_service_str: String = env.get_string(&source_service)
        .expect("Failed to get source service")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        manager.get_traffic_routes(&source_service_str).await
    });
    
    match result {
        Ok(routes) => {
            let ptrs: Vec<jlong> = routes.iter().map(|r| {
                let boxed = Box::new(r.clone());
                Box::into_raw(boxed) as jlong
            }).collect();
            
            let array = env.new_long_array(ptrs.len() as i32)
                .expect("Failed to create long array");
            env.set_long_array_region(array, 0, &ptrs)
                .expect("Failed to set long array");
            array
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_setCircuitBreakerConfig0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service: JString,
    consecutive_errors: jint,
    max_ejection_percent: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiTrafficManager) };
    let service_str: String = env.get_string(&service)
        .expect("Failed to get service")
        .into();
    
    let config = RiCircuitBreakerConfig {
        consecutive_errors: consecutive_errors as u32,
        interval: Duration::from_secs(10),
        base_ejection_time: Duration::from_secs(30),
        max_ejection_percent,
    };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.set_circuit_breaker_config(&service_str, config).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_setRateLimitConfig0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    service: JString,
    requests_per_second: jint,
    burst_size: jint,
) {
    if !check_not_null(&mut env, ptr, "RiTrafficManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiTrafficManager) };
    let service_str: String = env.get_string(&service)
        .expect("Failed to get service")
        .into();
    
    let config = RiRateLimitConfig {
        requests_per_second: requests_per_second as u32,
        burst_size: burst_size as u32,
        window: Duration::from_secs(1),
    };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.set_rate_limit_config(&service_str, config).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_servicemesh_RiTrafficManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiTrafficManager);
        }
    }
}
