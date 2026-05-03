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

//! # Device Module JNI Bindings
//!
//! JNI bindings for Ri device classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString, JObject};
use jni::sys::{jlong, jboolean, jint, jdouble, jstring, jlongArray, jobjectArray};
use crate::device::{
    RiDeviceControlModule, RiDeviceControlConfig, RiDevice, RiDeviceType, RiDeviceStatus,
    RiDeviceCapabilities, RiDeviceHealthMetrics, RiDeviceConfig, RiNetworkDeviceInfo,
    RiDiscoveryResult, RiResourceRequest, RiResourceAllocation, RiDeviceSchedulingConfig,
};
use crate::java::exception::check_not_null;
use crate::java::exception::throw_illegal_argument;
use crate::java::{register_jni_ptr, unregister_jni_ptr, is_jni_ptr_valid};

// =============================================================================
// RiDeviceControlModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceControlModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiDeviceControlConfig") {
        return 0;
    }
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceControlModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceControlModule);
        }
    }
}

// =============================================================================
// RiDeviceControlConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceControlConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config_boxed = Box::new(RiDeviceControlConfig::default());
    let config = Box::into_raw(config_boxed);
    register_jni_ptr(config as usize);
    config as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceControlConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceControlConfig);
        }
    }
}

// =============================================================================
// RiDevice JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_new0(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    device_type: jint,
) -> jlong {
    let name_str: String = env.get_string(&name)
        .expect("Failed to get name")
        .into();
    
    let dtype = match device_type {
        0 => RiDeviceType::CPU,
        1 => RiDeviceType::GPU,
        2 => RiDeviceType::Memory,
        3 => RiDeviceType::Storage,
        4 => RiDeviceType::Network,
        5 => RiDeviceType::Sensor,
        6 => RiDeviceType::Actuator,
        _ => RiDeviceType::Custom,
    };
    
    let device_boxed = Box::new(RiDevice::new(name_str, dtype));
    let device = Box::into_raw(device_boxed);
    register_jni_ptr(device as usize);
    device as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return std::ptr::null_mut();
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    env.new_string(device.id()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return std::ptr::null_mut();
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    env.new_string(device.name()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getDeviceType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    device.device_type() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    device.status() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_setStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    status: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return;
    }
    
    let device = unsafe { &mut *(ptr as *mut RiDevice) };
    let status_val = match status {
        0 => RiDeviceStatus::Unknown,
        1 => RiDeviceStatus::Available,
        2 => RiDeviceStatus::Busy,
        3 => RiDeviceStatus::Error,
        4 => RiDeviceStatus::Offline,
        5 => RiDeviceStatus::Maintenance,
        6 => RiDeviceStatus::Degraded,
        _ => RiDeviceStatus::Allocated,
    };
    device.set_status(status_val);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getCapabilities0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    let capabilities_boxed = Box::new(device.capabilities().clone());
    let capabilities = Box::into_raw(capabilities_boxed);
    register_jni_ptr(capabilities as usize);
    capabilities as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_setCapabilities0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    capabilities_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDevice") || !check_not_null(&mut env, capabilities_ptr, "RiDeviceCapabilities") {
        return;
    }
    
    let device = unsafe { &mut *(ptr as *mut RiDevice) };
    let capabilities = unsafe { &*(capabilities_ptr as *const RiDeviceCapabilities) };
    let _ = device.clone().with_capabilities(capabilities.clone());
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getHealthMetrics0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    let metrics_boxed = Box::new(device.health_metrics().clone());
    let metrics = Box::into_raw(metrics_boxed);
    register_jni_ptr(metrics as usize);
    metrics as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_isAvailable0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    device.is_available() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_isAllocated0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    device.is_allocated() as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_getHealthScore0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDevice") {
        return 0;
    }
    
    let device = unsafe { &*(ptr as *const RiDevice) };
    device.health_score() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDevice_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDevice);
        }
    }
}

// =============================================================================
// RiDeviceCapabilities JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let capabilities_boxed = Box::new(RiDeviceCapabilities::new());
    let capabilities = Box::into_raw(capabilities_boxed);
    register_jni_ptr(capabilities as usize);
    capabilities as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_getComputeUnits0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return -1;
    }
    
    let capabilities = unsafe { &*(ptr as *const RiDeviceCapabilities) };
    capabilities.compute_units.map(|v| v as jint).unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_setComputeUnits0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    units: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return;
    }
    
    let capabilities = unsafe { &mut *(ptr as *mut RiDeviceCapabilities) };
    capabilities.compute_units = Some(units as usize);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_getMemoryGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return -1.0;
    }
    
    let capabilities = unsafe { &*(ptr as *const RiDeviceCapabilities) };
    capabilities.memory_gb.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_setMemoryGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    memory_gb: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return;
    }
    
    let capabilities = unsafe { &mut *(ptr as *mut RiDeviceCapabilities) };
    capabilities.memory_gb = Some(memory_gb);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_getStorageGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return -1.0;
    }
    
    let capabilities = unsafe { &*(ptr as *const RiDeviceCapabilities) };
    capabilities.storage_gb.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_setStorageGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    storage_gb: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return;
    }
    
    let capabilities = unsafe { &mut *(ptr as *mut RiDeviceCapabilities) };
    capabilities.storage_gb = Some(storage_gb);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_getBandwidthGbps0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return -1.0;
    }
    
    let capabilities = unsafe { &*(ptr as *const RiDeviceCapabilities) };
    capabilities.bandwidth_gbps.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_setBandwidthGbps0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    bandwidth_gbps: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") {
        return;
    }
    
    let capabilities = unsafe { &mut *(ptr as *mut RiDeviceCapabilities) };
    capabilities.bandwidth_gbps = Some(bandwidth_gbps);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_meetsRequirements0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    requirements_ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceCapabilities") || !check_not_null(&mut env, requirements_ptr, "RiDeviceCapabilities") {
        return 0;
    }
    
    let capabilities = unsafe { &*(ptr as *const RiDeviceCapabilities) };
    let requirements = unsafe { &*(requirements_ptr as *const RiDeviceCapabilities) };
    capabilities.meets_requirements(requirements) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceCapabilities_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceCapabilities);
        }
    }
}

// =============================================================================
// RiDeviceHealthMetrics JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let metrics_boxed = Box::new(RiDeviceHealthMetrics::default());
    let metrics = Box::into_raw(metrics_boxed);
    register_jni_ptr(metrics as usize);
    metrics as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getCpuUsagePercent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.cpu_usage_percent
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setCpuUsagePercent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    percent: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.cpu_usage_percent = percent;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getMemoryUsagePercent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.memory_usage_percent
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setMemoryUsagePercent0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    percent: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.memory_usage_percent = percent;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getTemperatureCelsius0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.temperature_celsius
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setTemperatureCelsius0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    temperature: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.temperature_celsius = temperature;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getErrorCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.error_count as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setErrorCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    count: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.error_count = count as u32;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getThroughput0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.throughput as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setThroughput0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    throughput: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.throughput = throughput as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getNetworkLatencyMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.network_latency_ms
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setNetworkLatencyMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    latency_ms: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.network_latency_ms = latency_ms;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getDiskIops0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.disk_iops as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setDiskIops0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    iops: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.disk_iops = iops as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_getResponseTimeMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return 0.0;
    }
    
    let metrics = unsafe { &*(ptr as *const RiDeviceHealthMetrics) };
    metrics.response_time_ms
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_setResponseTimeMs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    response_time_ms: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceHealthMetrics") {
        return;
    }
    
    let metrics = unsafe { &mut *(ptr as *mut RiDeviceHealthMetrics) };
    metrics.response_time_ms = response_time_ms;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceHealthMetrics_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceHealthMetrics);
        }
    }
}

// =============================================================================
// RiDeviceConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config_boxed = Box::new(RiDeviceConfig::default());
    let config = Box::into_raw(config_boxed);
    register_jni_ptr(config as usize);
    config as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_isCpuDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceConfig) };
    config.enable_cpu_discovery as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_setCpuDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceConfig) };
    config.enable_cpu_discovery = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_isGpuDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceConfig) };
    config.enable_gpu_discovery as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_setGpuDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceConfig) };
    config.enable_gpu_discovery = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_getDiscoveryTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceConfig) };
    config.discovery_timeout_secs as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_setDiscoveryTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    timeout_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceConfig) };
    config.discovery_timeout_secs = timeout_secs as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_getMaxDevicesPerType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceConfig) };
    config.max_devices_per_type as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_setMaxDevicesPerType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceConfig) };
    config.max_devices_per_type = max as usize;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceConfig);
        }
    }
}

// =============================================================================
// RiDeviceSchedulingConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config_boxed = Box::new(RiDeviceSchedulingConfig::default());
    let config = Box::into_raw(config_boxed);
    register_jni_ptr(config as usize);
    config as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_isDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceSchedulingConfig) };
    config.discovery_enabled as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_setDiscoveryEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceSchedulingConfig) };
    config.discovery_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_getDiscoveryIntervalSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceSchedulingConfig) };
    config.discovery_interval_secs as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_setDiscoveryIntervalSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    interval_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceSchedulingConfig) };
    config.discovery_interval_secs = interval_secs as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_isAutoSchedulingEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceSchedulingConfig) };
    config.auto_scheduling_enabled as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_setAutoSchedulingEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceSchedulingConfig) };
    config.auto_scheduling_enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_getMaxConcurrentTasks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceSchedulingConfig) };
    config.max_concurrent_tasks as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_setMaxConcurrentTasks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max: jint,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceSchedulingConfig) };
    config.max_concurrent_tasks = max as usize;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_getResourceAllocationTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiDeviceSchedulingConfig) };
    config.resource_allocation_timeout_secs as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_setResourceAllocationTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    timeout_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceSchedulingConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiDeviceSchedulingConfig) };
    config.resource_allocation_timeout_secs = timeout_secs as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceSchedulingConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDeviceSchedulingConfig);
        }
    }
}

// =============================================================================
// RiNetworkDeviceInfo JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_new0(
    mut env: JNIEnv,
    _class: JClass,
    id: JString,
    device_type: JString,
    source: JString,
) -> jlong {
    let id_str: String = env.get_string(&id)
        .expect("Failed to get id")
        .into();
    let device_type_str: String = env.get_string(&device_type)
        .expect("Failed to get device type")
        .into();
    let source_str: String = env.get_string(&source)
        .expect("Failed to get source")
        .into();
    
    let info = Box::new(RiNetworkDeviceInfo {
        id: id_str,
        device_type: device_type_str,
        source: source_str,
        compute_units: None,
        memory_gb: None,
        storage_gb: None,
        bandwidth_gbps: None,
    });
    Box::into_raw(info) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    env.new_string(&info.id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getDeviceType0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    env.new_string(&info.device_type).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getSource0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return std::ptr::null_mut();
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    env.new_string(&info.source).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getComputeUnits0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return -1;
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    info.compute_units.map(|v| v as jint).unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_setComputeUnits0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    units: jint,
) {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return;
    }
    
    let info = unsafe { &mut *(ptr as *mut RiNetworkDeviceInfo) };
    info.compute_units = Some(units as usize);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getMemoryGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return -1.0;
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    info.memory_gb.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_setMemoryGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    memory_gb: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return;
    }
    
    let info = unsafe { &mut *(ptr as *mut RiNetworkDeviceInfo) };
    info.memory_gb = Some(memory_gb);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getStorageGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return -1.0;
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    info.storage_gb.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_setStorageGb0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    storage_gb: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return;
    }
    
    let info = unsafe { &mut *(ptr as *mut RiNetworkDeviceInfo) };
    info.storage_gb = Some(storage_gb);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_getBandwidthGbps0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return -1.0;
    }
    
    let info = unsafe { &*(ptr as *const RiNetworkDeviceInfo) };
    info.bandwidth_gbps.unwrap_or(-1.0)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_setBandwidthGbps0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    bandwidth_gbps: jdouble,
) {
    if !check_not_null(&mut env, ptr, "RiNetworkDeviceInfo") {
        return;
    }
    
    let info = unsafe { &mut *(ptr as *mut RiNetworkDeviceInfo) };
    info.bandwidth_gbps = Some(bandwidth_gbps);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiNetworkDeviceInfo_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiNetworkDeviceInfo);
        }
    }
}

// =============================================================================
// RiDiscoveryResult JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDiscoveryResult_getDiscoveredDevices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiDiscoveryResult") {
        return std::ptr::null_mut();
    }
    
    let result = unsafe { &*(ptr as *const RiDiscoveryResult) };
    let devices: Vec<jlong> = result.discovered_devices.iter().map(|_| 0 as jlong).collect();
    
    let array = env.new_long_array(devices.len() as i32).unwrap();
    env.set_long_array_region(&array, 0, &devices).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDiscoveryResult_getUpdatedDevices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiDiscoveryResult") {
        return std::ptr::null_mut();
    }
    
    let result = unsafe { &*(ptr as *const RiDiscoveryResult) };
    let devices: Vec<jlong> = result.updated_devices.iter().map(|_| 0 as jlong).collect();
    
    let array = env.new_long_array(devices.len() as i32).unwrap();
    env.set_long_array_region(&array, 0, &devices).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDiscoveryResult_getRemovedDevices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jobjectArray {
    if !check_not_null(&mut env, ptr, "RiDiscoveryResult") {
        return std::ptr::null_mut();
    }
    
    let result = unsafe { &*(ptr as *const RiDiscoveryResult) };
    let string_class = env.find_class("java/lang/String").unwrap();
    let array = env.new_object_array(result.removed_devices.len() as i32, string_class, JObject::null()).unwrap();
    
    for (i, id) in result.removed_devices.iter().enumerate() {
        let jstr = env.new_string(id).unwrap();
        env.set_object_array_element(&array, i as i32, jstr).unwrap();
    }
    
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDiscoveryResult_getTotalDevices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDiscoveryResult") {
        return 0;
    }
    
    let result = unsafe { &*(ptr as *const RiDiscoveryResult) };
    result.total_devices as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDiscoveryResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiDiscoveryResult);
        }
    }
}

// =============================================================================
// RiResourceRequest JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_new0(
    mut env: JNIEnv,
    _class: JClass,
    request_id: JString,
    device_type: jint,
    capabilities_ptr: jlong,
) -> jlong {
    let request_id_str: String = env.get_string(&request_id)
        .expect("Failed to get request id")
        .into();
    
    let dtype = match device_type {
        0 => RiDeviceType::CPU,
        1 => RiDeviceType::GPU,
        2 => RiDeviceType::Memory,
        3 => RiDeviceType::Storage,
        4 => RiDeviceType::Network,
        5 => RiDeviceType::Sensor,
        6 => RiDeviceType::Actuator,
        _ => RiDeviceType::Custom,
    };
    
    let capabilities = if capabilities_ptr != 0 {
        unsafe { &*(capabilities_ptr as *const RiDeviceCapabilities) }.clone()
    } else {
        RiDeviceCapabilities::new()
    };
    
    let request = Box::new(RiResourceRequest {
        request_id: request_id_str,
        device_type: dtype,
        required_capabilities: capabilities,
        priority: 5,
        timeout_secs: 60,
        sla_class: None,
        resource_weights: None,
        affinity: None,
        anti_affinity: None,
    });
    Box::into_raw(request) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_getRequestId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return std::ptr::null_mut();
    }
    
    let request = unsafe { &*(ptr as *const RiResourceRequest) };
    env.new_string(&request.request_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_getDeviceType0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return 0;
    }
    
    let request = unsafe { &*(ptr as *const RiResourceRequest) };
    request.device_type as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_getRequiredCapabilities0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return 0;
    }
    
    let request = unsafe { &*(ptr as *const RiResourceRequest) };
    let capabilities_boxed = Box::new(request.required_capabilities.clone());
    let capabilities = Box::into_raw(capabilities_boxed);
    register_jni_ptr(capabilities as usize);
    capabilities as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_getPriority0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return 0;
    }
    
    let request = unsafe { &*(ptr as *const RiResourceRequest) };
    request.priority as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_setPriority0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    priority: jint,
) {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return;
    }
    
    let request = unsafe { &mut *(ptr as *mut RiResourceRequest) };
    request.priority = priority as u8;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_getTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return 0;
    }
    
    let request = unsafe { &*(ptr as *const RiResourceRequest) };
    request.timeout_secs as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_setTimeoutSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    timeout_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiResourceRequest") {
        return;
    }
    
    let request = unsafe { &mut *(ptr as *mut RiResourceRequest) };
    request.timeout_secs = timeout_secs as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceRequest_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiResourceRequest);
        }
    }
}

// =============================================================================
// RiResourceAllocation JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getAllocationId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return std::ptr::null_mut();
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    env.new_string(&allocation.allocation_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getDeviceId0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return std::ptr::null_mut();
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    env.new_string(&allocation.device_id).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getDeviceName0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return std::ptr::null_mut();
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    env.new_string(&allocation.device_name).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getAllocatedAt0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return std::ptr::null_mut();
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    env.new_string(&allocation.allocated_at.to_rfc3339()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getExpiresAt0<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass<'local>,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return std::ptr::null_mut();
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    env.new_string(&allocation.expires_at.to_rfc3339()).unwrap().into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_isExpired0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return 1;
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    (chrono::Utc::now() > allocation.expires_at) as jboolean
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getRemainingTimeSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return 0;
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    (allocation.expires_at - chrono::Utc::now()).num_seconds().max(0) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_getRequest0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiResourceAllocation") {
        return 0;
    }
    
    let allocation = unsafe { &*(ptr as *const RiResourceAllocation) };
    let request_boxed = Box::new(allocation.request.clone());
    let request = Box::into_raw(request_boxed);
    register_jni_ptr(request as usize);
    request as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiResourceAllocation_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 && is_jni_ptr_valid(ptr as usize) {
        unregister_jni_ptr(ptr as usize);
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiResourceAllocation);
        }
    }
}

// =============================================================================
// RiDeviceController JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_getAllDevices0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlongArray {
    if !check_not_null(&mut env, ptr, "RiDeviceController") {
        return std::ptr::null_mut();
    }
    
    let devices: Vec<jlong> = Vec::new();
    let array = env.new_long_array(0).unwrap();
    array.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_getDevice0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    device_id: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiDeviceController") {
        return 0;
    }
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_addDevice0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    device_ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiDeviceController") || !check_not_null(&mut env, device_ptr, "RiDevice") {
        return;
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_removeDevice0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    device_id: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiDeviceController") {
        return 0;
    }
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_getDeviceCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiDeviceController") {
        return 0;
    }
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_device_RiDeviceController_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
}
