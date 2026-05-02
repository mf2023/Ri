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
use crate::cache::{RiCacheModule, RiCacheConfig, RiCacheBackendType, RiCacheStats, RiCachePolicy, RiCachedValue, RiCacheManager};
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

// =============================================================================
// RiCachePolicy JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let policy = Box::new(RiCachePolicy::default());
    Box::into_raw(policy) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_setTtlSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiCachePolicy) };
    policy.ttl = if ttl_secs >= 0 {
        Some(std::time::Duration::from_secs(ttl_secs as u64))
    } else {
        None
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_getTtlSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return -1;
    }
    
    let policy = unsafe { &*(ptr as *const RiCachePolicy) };
    policy.ttl.map(|d| d.as_secs() as jlong).unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_setRefreshOnAccess0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    refresh_on_access: jboolean,
) {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiCachePolicy) };
    policy.refresh_on_access = refresh_on_access != 0;
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_getRefreshOnAccess0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return 0;
    }
    
    let policy = unsafe { &*(ptr as *const RiCachePolicy) };
    if policy.refresh_on_access { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_setMaxSize0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_size: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return;
    }
    
    let policy = unsafe { &mut *(ptr as *mut RiCachePolicy) };
    policy.max_size = if max_size >= 0 {
        Some(max_size as usize)
    } else {
        None
    };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_getMaxSize0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCachePolicy") {
        return -1;
    }
    
    let policy = unsafe { &*(ptr as *const RiCachePolicy) };
    policy.max_size.map(|s| s as jlong).unwrap_or(-1)
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachePolicy_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCachePolicy);
        }
    }
}

// =============================================================================
// RiCachedValue JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_new0(
    mut env: JNIEnv,
    _class: JClass,
    value: JString,
    ttl_secs: jlong,
) -> jlong {
    let value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
    
    let ttl = if ttl_secs >= 0 { Some(ttl_secs as u64) } else { None };
    let cached_value = Box::new(RiCachedValue::new(value_str, ttl));
    Box::into_raw(cached_value) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_getValue0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return std::ptr::null_mut();
    }
    
    let cached_value = unsafe { &*(ptr as *const RiCachedValue) };
    env.new_string(&cached_value.value)
        .expect("Failed to create string")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_getExpiresAt0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return -1;
    }
    
    let cached_value = unsafe { &*(ptr as *const RiCachedValue) };
    cached_value.expires_at.unwrap_or(0) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_getLastAccessed0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return -1;
    }
    
    let cached_value = unsafe { &*(ptr as *const RiCachedValue) };
    cached_value.last_accessed.unwrap_or(0) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_isExpired0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return 0;
    }
    
    let cached_value = unsafe { &*(ptr as *const RiCachedValue) };
    if cached_value.is_expired() { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_touch0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return;
    }
    
    let cached_value = unsafe { &mut *(ptr as *mut RiCachedValue) };
    cached_value.touch();
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_isStale0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_idle_secs: jlong,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCachedValue") {
        return 0;
    }
    
    let cached_value = unsafe { &*(ptr as *const RiCachedValue) };
    if cached_value.is_stale(max_idle_secs as u64) { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCachedValue_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCachedValue);
        }
    }
}

// =============================================================================
// RiCacheManager JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    use crate::cache::RiMemoryCache;
    use std::sync::Arc;
    
    let backend = Arc::new(RiMemoryCache::new());
    let manager = Box::new(RiCacheManager::new(backend));
    Box::into_raw(manager) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_get0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return std::ptr::null_mut();
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        manager.get::<String>(&key_str).await
    });
    
    match result {
        Ok(Some(value)) => env.new_string(value)
            .expect("Failed to create string")
            .into_raw(),
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_set0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
    ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    let value_str: String = env.get_string(&value)
        .expect("Failed to get value")
        .into();
    let ttl = if ttl_secs >= 0 { Some(ttl_secs as u64) } else { None };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.set(&key_str, &value_str, ttl).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_delete0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        manager.delete(&key_str).await
    });
    
    match result {
        Ok(deleted) => if deleted { 1 } else { 0 },
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_exists0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    let key_str: String = env.get_string(&key)
        .expect("Failed to get key")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let exists = rt.block_on(async {
        manager.exists(&key_str).await
    });
    
    if exists { 1 } else { 0 }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_clear0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.clear().await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_stats0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let stats = rt.block_on(async {
        manager.stats().await
    });
    
    let stats_box = Box::new(stats);
    Box::into_raw(stats_box) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_cleanupExpired0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return 0;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let result = rt.block_on(async {
        manager.cleanup_expired().await
    });
    
    match result {
        Ok(count) => count as jlong,
        _ => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_invalidatePattern0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    pattern: JString,
) {
    if !check_not_null(&mut env, ptr, "RiCacheManager") {
        return;
    }
    
    let manager = unsafe { &*(ptr as *const RiCacheManager) };
    let pattern_str: String = env.get_string(&pattern)
        .expect("Failed to get pattern")
        .into();
    
    let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    let _ = rt.block_on(async {
        manager.invalidate_pattern(&pattern_str).await
    });
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheManager_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCacheManager);
        }
    }
}
