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

//! # Cache Module JNI Bindings
//!
//! JNI bindings for Ri cache classes.

use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::{jlong, jboolean, jint, jstring};
use crate::cache::{RiCacheModule, RiCacheConfig, RiCacheBackendType, RiCacheStats};
use crate::java::exception::check_not_null;

// =============================================================================
// RiCacheModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_new0(
    mut env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, config_ptr, "RiCacheConfig") {
        return 0;
    }
    
    let config = unsafe { &*(config_ptr as *const RiCacheConfig) };
    let module = Box::new(RiCacheModule::new(config.clone()));
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_set(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
    _ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return;
    }
    
    let _module = unsafe { &*(ptr as *const RiCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    let _value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_get(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return std::ptr::null_mut();
    }
    
    let _module = unsafe { &*(ptr as *const RiCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    std::ptr::null_mut()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_delete(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return;
    }
    
    let _module = unsafe { &*(ptr as *const RiCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_exists(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return 0;
    }
    
    let _module = unsafe { &*(ptr as *const RiCacheModule) };
    let _key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    0
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_getStats(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return 0;
    }
    
    let _module = unsafe { &*(ptr as *const RiCacheModule) };
    
    let stats = Box::new(RiCacheStats::default());
    Box::into_raw(stats) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCacheModule);
        }
    }
}

// =============================================================================
// RiCacheConfig JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiCacheConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setEnabled(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    enabled: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.enabled = enabled != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setDefaultTtlSecs(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    ttl: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.default_ttl_secs = ttl as u64;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setBackendType(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    backend_type: jint,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.backend_type = match backend_type {
        0 => RiCacheBackendType::Memory,
        1 => RiCacheBackendType::Redis,
        2 => RiCacheBackendType::Hybrid,
        _ => RiCacheBackendType::Memory,
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setRedisUrl(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    url: JString,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }
    
    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.redis_url = env.get_string(&url)
        .expect("Failed to get redis url")
        .into();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCacheConfig);
        }
    }
}

// =============================================================================
// RiCacheStats JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheStats_getHits(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiCacheStats) };
    stats.hits as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheStats_getMisses(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheStats") {
        return 0;
    }
    
    let stats = unsafe { &*(ptr as *const RiCacheStats) };
    stats.misses as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCacheStats);
        }
    }
}
