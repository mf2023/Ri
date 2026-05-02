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

//! # Core Module JNI Bindings
//!
//! JNI bindings for Ri core classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jstring, jint};
use crate::core::{RiAppBuilder, RiAppRuntime, RiError, RiServiceContext, RiHealthStatus, RiHealthCheckResult, RiHealthCheckConfig, RiHealthReport, RiHealthChecker, RiErrorChain, RiLockError, RiLifecycleObserver, RiLogAnalyticsModule};
use crate::java::exception::{throw_ri_error, check_not_null};
use crate::config::RiConfig;
use std::time::Duration;

// =============================================================================
// RiAppBuilder JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let builder = Box::new(RiAppBuilder::new());
    Box::into_raw(builder) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_withConfig(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    config_path: JString,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut RiAppBuilder) };
    let path: String = env.get_string(&config_path)
        .expect("Failed to get config path")
        .into();
    
    match builder.with_config(&path) {
        Ok(new_builder) => {
            let boxed = Box::new(new_builder);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_ri_error(&mut env, &e.to_string());
            // 这里存在内存泄漏或者潜在的 use-after-free 问题
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_build(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiAppBuilder") {
        return 0;
    }
    
    let builder = unsafe { Box::from_raw(ptr as *mut RiAppBuilder) };
    
    match builder.build() {
        Ok(runtime) => {
            let boxed = Box::new(runtime);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_ri_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppBuilder_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAppBuilder);
        }
    }
}

// =============================================================================
// RiAppRuntime JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiAppRuntime);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_isRunning(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiAppRuntime") {
        return 0;
    }
    
    let _runtime = unsafe { &*(ptr as *const RiAppRuntime) };
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiAppRuntime_shutdown(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiAppRuntime") {
        return;
    }
    
    let _runtime = unsafe { &*(ptr as *const RiAppRuntime) };
}

// =============================================================================
// RiConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiConfig);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiConfig_get(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiConfig") {
        return std::ptr::null_mut();
    }
    
    let config = unsafe { &*(ptr as *const RiConfig) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    match config.get(&key_str) {
        Some(value) => {
            env.new_string(value)
                .expect("Failed to create Java string")
                .into_raw()
        }
        None => std::ptr::null_mut(),
    }
}

// =============================================================================
// RiError JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiError_getMessage(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiError) };
    env.new_string(error.to_string())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiError_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiError);
        }
    }
}

// =============================================================================
// RiServiceContext JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_new0(
    mut env: JNIEnv,
    _class: JClass,
) -> jlong {
    match RiServiceContext::new_default() {
        Ok(ctx) => {
            let boxed = Box::new(ctx);
            Box::into_raw(boxed) as jlong
        }
        Err(e) => {
            throw_ri_error(&mut env, &e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_logger0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceContext") {
        return 0;
    }
    
    let ctx = unsafe { &*(ptr as *const RiServiceContext) };
    let logger = ctx.logger().clone();
    let boxed = Box::new(logger);
    Box::into_raw(boxed) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_config0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceContext") {
        return 0;
    }
    
    let ctx = unsafe { &*(ptr as *const RiServiceContext) };
    let config_manager = ctx.config();
    let config = (*config_manager).config().clone();
    let boxed = Box::new(config);
    Box::into_raw(boxed) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_fs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceContext") {
        return 0;
    }
    
    let ctx = unsafe { &*(ptr as *const RiServiceContext) };
    let fs = ctx.fs().clone();
    let boxed = Box::new(fs);
    Box::into_raw(boxed) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_hooks0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiServiceContext") {
        return 0;
    }
    
    let ctx = unsafe { &*(ptr as *const RiServiceContext) };
    let hooks = ctx.hooks();
    let hooks_inner = (*hooks).clone();
    let boxed = Box::new(hooks_inner);
    Box::into_raw(boxed) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiServiceContext_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiServiceContext);
        }
    }
}

// =============================================================================
// RiHealthCheckResult JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckResult_new0(
    mut env: JNIEnv,
    _class: JClass,
    name: JString,
    status: jint,
    message: JString,
) -> jlong {
    let name_str: String = env.get_string(&name)
        .expect("Failed to get name")
        .into();
    
    let health_status = match status {
        0 => RiHealthStatus::Healthy,
        1 => RiHealthStatus::Degraded,
        2 => RiHealthStatus::Unhealthy,
        _ => RiHealthStatus::Unknown,
    };
    
    let message_str: Option<String> = if message.is_null() {
        None
    } else {
        Some(env.get_string(&message).expect("Failed to get message").into())
    };
    
    let result = RiHealthCheckResult {
        name: name_str,
        status: health_status,
        message: message_str,
        timestamp: std::time::SystemTime::now(),
        duration: Duration::ZERO,
    };
    
    let boxed = Box::new(result);
    Box::into_raw(boxed) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckResult_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiHealthCheckResult") {
        return std::ptr::null_mut();
    }
    
    let result = unsafe { &*(ptr as *const RiHealthCheckResult) };
    env.new_string(&result.name)
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckResult_getStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthCheckResult") {
        return 3;
    }
    
    let result = unsafe { &*(ptr as *const RiHealthCheckResult) };
    match result.status {
        RiHealthStatus::Healthy => 0,
        RiHealthStatus::Degraded => 1,
        RiHealthStatus::Unhealthy => 2,
        RiHealthStatus::Unknown => 3,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckResult_getMessage0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiHealthCheckResult") {
        return std::ptr::null_mut();
    }
    
    let result = unsafe { &*(ptr as *const RiHealthCheckResult) };
    match &result.message {
        Some(msg) => env.new_string(msg)
            .expect("Failed to create Java string")
            .into_raw(),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckResult_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHealthCheckResult);
        }
    }
}

// =============================================================================
// RiHealthCheckConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiHealthCheckConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_newWithValues0(
    _env: JNIEnv,
    _class: JClass,
    check_interval: jlong,
    timeout: jlong,
    failure_threshold: jint,
    success_threshold: jint,
    enabled: jboolean,
) -> jlong {
    let config = Box::new(RiHealthCheckConfig {
        check_interval: Duration::from_secs(check_interval as u64),
        timeout: Duration::from_secs(timeout as u64),
        failure_threshold: failure_threshold as u32,
        success_threshold: success_threshold as u32,
        enabled: enabled != 0,
    });
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_getCheckInterval0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiHealthCheckConfig) };
    config.check_interval.as_secs() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_setCheckInterval0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiHealthCheckConfig) };
    config.check_interval = Duration::from_secs(value as u64);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_getTimeout0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiHealthCheckConfig) };
    config.timeout.as_secs() as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_setTimeout0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiHealthCheckConfig) };
    config.timeout = Duration::from_secs(value as u64);
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_getFailureThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiHealthCheckConfig) };
    config.failure_threshold as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_getSuccessThreshold0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiHealthCheckConfig) };
    config.success_threshold as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_isEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(ptr as *const RiHealthCheckConfig) };
    if config.enabled { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthCheckConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHealthCheckConfig);
        }
    }
}

// =============================================================================
// RiHealthReport JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let report = Box::new(RiHealthReport::new());
    Box::into_raw(report) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getOverallStatus0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 3;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    match report.overall_status {
        RiHealthStatus::Healthy => 0,
        RiHealthStatus::Degraded => 1,
        RiHealthStatus::Unhealthy => 2,
        RiHealthStatus::Unknown => 3,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getTotalComponents0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 0;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    report.total_components as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getHealthyCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 0;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    report.healthy_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getDegradedCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 0;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    report.degraded_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getUnhealthyCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 0;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    report.unhealthy_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_getUnknownCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthReport") {
        return 0;
    }
    
    let report = unsafe { &*(ptr as *const RiHealthReport) };
    report.unknown_count as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthReport_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiHealthReport);
        }
    }
}

// =============================================================================
// RiHealthChecker JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthChecker_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let checker = Box::new(RiHealthChecker::new());
    Box::into_raw(checker) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthChecker_withConfig0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiHealthCheckConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiHealthCheckConfig) };
    let checker = Box::new(RiHealthChecker::with_config(config.clone()));
    Box::into_raw(checker) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthChecker_getCheckCount0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if !check_not_null(&mut env, ptr, "RiHealthChecker") {
        return 0;
    }
    
    let checker = unsafe { &*(ptr as *const RiHealthChecker) };
    checker.check_count() as jint
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiHealthChecker_free0(
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
// RiErrorChain JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_new0(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) -> jlong {
    let msg: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let error = std::io::Error::other(msg);
    let chain = Box::new(RiErrorChain::new(error));
    Box::into_raw(chain) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_withContext0(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
    context: JString,
) -> jlong {
    let msg: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    let ctx: String = env.get_string(&context)
        .expect("Failed to get context")
        .into();
    
    let error = std::io::Error::other(msg);
    let chain = Box::new(RiErrorChain::with_context(error, ctx));
    Box::into_raw(chain) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_getContext0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiErrorChain") {
        return std::ptr::null_mut();
    }
    
    let chain = unsafe { &*(ptr as *const RiErrorChain) };
    env.new_string(chain.get_context())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_getSourceError0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiErrorChain") {
        return std::ptr::null_mut();
    }
    
    let chain = unsafe { &*(ptr as *const RiErrorChain) };
    env.new_string(chain.source_error().to_string())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_prettyFormat0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiErrorChain") {
        return std::ptr::null_mut();
    }
    
    let chain = unsafe { &*(ptr as *const RiErrorChain) };
    env.new_string(chain.pretty_format())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorChain_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiErrorChain);
        }
    }
}

// =============================================================================
// RiErrorContext JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorContext_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorContext_chainFromMsg0(
    mut env: JNIEnv,
    _class: JClass,
    message: JString,
) -> jlong {
    let msg: String = env.get_string(&message)
        .expect("Failed to get message")
        .into();
    
    let error = std::io::Error::other(msg);
    let chain = Box::new(RiErrorChain::new(error));
    Box::into_raw(chain) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiErrorContext_free0(
    _env: JNIEnv,
    _class: JClass,
    _ptr: jlong,
) {
}

// =============================================================================
// RiLockError JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_new0(
    mut env: JNIEnv,
    _class: JClass,
    context: JString,
) -> jlong {
    let ctx: String = env.get_string(&context)
        .expect("Failed to get context")
        .into();
    
    let error = Box::new(RiLockError::new(&ctx));
    Box::into_raw(error) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_newWithPoisoned0(
    mut env: JNIEnv,
    _class: JClass,
    context: JString,
    is_poisoned: jboolean,
) -> jlong {
    let ctx: String = env.get_string(&context)
        .expect("Failed to get context")
        .into();
    
    let error = if is_poisoned != 0 {
        Box::new(RiLockError::poisoned(&ctx))
    } else {
        Box::new(RiLockError::new(&ctx))
    };
    Box::into_raw(error) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_poisoned0(
    mut env: JNIEnv,
    _class: JClass,
    context: JString,
) -> jlong {
    let ctx: String = env.get_string(&context)
        .expect("Failed to get context")
        .into();
    
    let error = Box::new(RiLockError::poisoned(&ctx));
    Box::into_raw(error) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_getContext0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiLockError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiLockError) };
    env.new_string(error.get_context())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_isPoisoned0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiLockError") {
        return 0;
    }
    
    let error = unsafe { &*(ptr as *const RiLockError) };
    if error.is_poisoned() { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_getMessage0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiLockError") {
        return std::ptr::null_mut();
    }
    
    let error = unsafe { &*(ptr as *const RiLockError) };
    env.new_string(error.to_string())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLockError_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLockError);
        }
    }
}

// =============================================================================
// RiLifecycleObserver JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLifecycleObserver_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let observer = Box::new(RiLifecycleObserver::new());
    Box::into_raw(observer) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLifecycleObserver_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiLifecycleObserver") {
        return std::ptr::null_mut();
    }
    
    let observer = unsafe { &*(ptr as *const RiLifecycleObserver) };
    env.new_string(observer.name())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLifecycleObserver_isCritical0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiLifecycleObserver") {
        return 0;
    }
    
    let observer = unsafe { &*(ptr as *const RiLifecycleObserver) };
    if observer.is_critical() { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLifecycleObserver_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLifecycleObserver);
        }
    }
}

// =============================================================================
// RiLogAnalyticsModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLogAnalyticsModule_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let module = Box::new(RiLogAnalyticsModule::new());
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLogAnalyticsModule_getName0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiLogAnalyticsModule") {
        return std::ptr::null_mut();
    }
    
    let module = unsafe { &*(ptr as *const RiLogAnalyticsModule) };
    env.new_string(module.name())
        .expect("Failed to create Java string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLogAnalyticsModule_isCritical0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiLogAnalyticsModule") {
        return 0;
    }
    
    let module = unsafe { &*(ptr as *const RiLogAnalyticsModule) };
    if module.is_critical() { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLogAnalyticsModule_isEnabled0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiLogAnalyticsModule") {
        return 0;
    }
    
    let module = unsafe { &*(ptr as *const RiLogAnalyticsModule) };
    if module.enabled { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_RiLogAnalyticsModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLogAnalyticsModule);
        }
    }
}
