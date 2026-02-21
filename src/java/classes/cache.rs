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

//! # Cache Module JNI Bindings
//!
//! JNI bindings for DMSC cache classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jstring};
use crate::cache::{DMSCCacheModule, DMSCCacheConfig, DMSCCacheBackendType, DMSCCacheStats};
use crate::java::exception::check_not_null;

// =============================================================================
// DMSCCacheModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "DMSCCacheConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const DMSCCacheConfig) };
    let module = Box::new(DMSCCacheModule::new(config.clone()));
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_set(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
    _ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheModule") {
        return;
    }
    
    let _module = unsafe { &*(ptr as *const DMSCCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    let _value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_get(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "DMSCCacheModule") {
        return std::ptr::null_mut();
    }
    
    let _module = unsafe { &*(ptr as *const DMSCCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_delete(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheModule") {
        return;
    }
    
    let _module = unsafe { &*(ptr as *const DMSCCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_exists(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "DMSCCacheModule") {
        return 0;
    }
    
    let _module = unsafe { &*(ptr as *const DMSCCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_getStats(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCCacheModule") {
        return 0;
    }
    
    let _module = unsafe { &*(ptr as *const DMSCCacheModule) };
    
    let stats = Box::new(DMSCCacheStats::default());
    Box::into_raw(stats) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCCacheModule);
        }
    }
}

// =============================================================================
// DMSCCacheConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(DMSCCacheConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_setEnabled(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut DMSCCacheConfig) };
    config.enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_setDefaultTtlSecs(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    ttl: jlong,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut DMSCCacheConfig) };
    config.default_ttl_secs = ttl as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_setBackendType(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    backend_type: jint,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut DMSCCacheConfig) };
    config.backend_type = match backend_type {
        0 => DMSCCacheBackendType::Memory,
        1 => DMSCCacheBackendType::Redis,
        2 => DMSCCacheBackendType::Hybrid,
        _ => DMSCCacheBackendType::Memory,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_setRedisUrl(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    url: JString,
) {
    if !check_not_null(&mut env, ptr, "DMSCCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut DMSCCacheConfig) };
    config.redis_url = env.get_string(&url)
        .expect("Failed to get redis url")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCCacheConfig);
        }
    }
}

// =============================================================================
// DMSCCacheStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheStats_getHits(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCCacheStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const DMSCCacheStats) };
    stats.hits as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheStats_getMisses(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "DMSCCacheStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const DMSCCacheStats) };
    stats.misses as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_dmsc_cache_DMSCCacheStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut DMSCCacheStats);
        }
    }
}
