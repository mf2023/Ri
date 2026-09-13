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
use crate::java::runtime::{block_on_jni, ttl_from_jlong};

// =============================================================================
// RiCacheModule JNI Bindings
// =============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_new0(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    if config_ptr == 0 {
        return 0;
    }
    let config = unsafe { &*(config_ptr as *const RiCacheConfig) };
    let module = Box::new(RiCacheModule::with_config(config.clone()));
    Box::into_raw(module) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_set0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    value: JString,
    ttl_secs: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return;
    }
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let value_str: String = match env.get_string(&value) {
        Ok(s) => s.into(),
        Err(_) => return,
    };

    let module = unsafe { &*(ptr as *const RiCacheModule) };
    let manager = module.cache_manager();
    let ttl = ttl_from_jlong(ttl_secs);
    if let Some(_) = block_on_jni(&mut env, "RiCacheModule::set", async move {
        let mgr = manager.read().await;
        mgr.set::<String>(&key_str, &value_str, ttl).await
    }) {
        // Ok(()) -> success; Err was already converted to a Java exception.
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_get0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jstring {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return std::ptr::null_mut();
    }
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let module = unsafe { &*(ptr as *const RiCacheModule) };
    let manager = module.cache_manager();
    let result = block_on_jni(&mut env, "RiCacheModule::get", async move {
        let mgr = manager.read().await;
        mgr.get::<String>(&key_str).await
    });

    match result {
        Some(Ok(Some(value))) => match env.new_string(value) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        Some(Ok(None)) => std::ptr::null_mut(),
        Some(Err(_)) => {
            // Error already surfaced as a Java exception.
            std::ptr::null_mut()
        }
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_delete0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return;
    }
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return,
    };

    let module = unsafe { &*(ptr as *const RiCacheModule) };
    let manager = module.cache_manager();
    if let Some(result) = block_on_jni(&mut env, "RiCacheModule::delete", async move {
        let mgr = manager.read().await;
        mgr.delete(&key_str).await
    }) {
        let _ = result;
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_exists0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jboolean {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return 0;
    }
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    let module = unsafe { &*(ptr as *const RiCacheModule) };
    let manager = module.cache_manager();
    match block_on_jni(&mut env, "RiCacheModule::exists", async move {
        let mgr = manager.read().await;
        mgr.exists(&key_str).await
    }) {
        Some(exists) => if exists { 1 } else { 0 },
        None => 0,
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheModule_getStats0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if !check_not_null(&mut env, ptr, "RiCacheModule") {
        return 0;
    }

    let module = unsafe { &*(ptr as *const RiCacheModule) };
    let manager = module.cache_manager();
    match block_on_jni(&mut env, "RiCacheModule::getStats", async move {
        let mgr = manager.read().await;
        mgr.stats().await
    }) {
        Some(stats) => Box::into_raw(Box::new(stats)) as jlong,
        None => 0,
    }
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
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setEnabled0(
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
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setDefaultTtlSecs0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    ttl: jlong,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }

    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.default_ttl_secs = if ttl >= 0 { ttl as u64 } else { 0 };
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setBackendType0(
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
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheConfig_setRedisUrl0(
    mut env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    url: JString,
) {
    if !check_not_null(&mut env, ptr, "RiCacheConfig") {
        return;
    }

    let url_str: String = match env.get_string(&url) {
        Ok(s) => s.into(),
        Err(_) => return,
    };

    let config = unsafe { &mut *(ptr as *mut RiCacheConfig) };
    config.redis_url = url_str;
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
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheStats_getHits0(
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
pub extern "system" fn Java_com_dunimd_ri_cache_RiCacheStats_getMisses0(
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
    let value_str: String = match env.get_string(&value) {
        Ok(s) => s.into(),
        Err(_) => {
            crate::java::exception::throw_ri_error(&mut env, "Failed to get value");
            return 0;
        }
    };
    
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
    match env.new_string(&cached_value.value) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
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
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return std::ptr::null_mut(),
    };

    let result = block_on_jni(&mut env, "RiCacheManager::get", async move {
        manager.get::<String>(&key_str).await
    });

    match result {
        Some(Ok(Some(value))) => match env.new_string(value) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
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
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let value_str: String = match env.get_string(&value) {
        Ok(s) => s.into(),
        Err(_) => return,
    };
    let ttl = ttl_from_jlong(ttl_secs);

    let result = block_on_jni(&mut env, "RiCacheManager::set", async move {
        manager.set(&key_str, &value_str, ttl).await
    });
    let _ = result;
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
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    match block_on_jni(&mut env, "RiCacheManager::delete", async move {
        manager.delete(&key_str).await
    }) {
        Some(Ok(deleted)) => if deleted { 1 } else { 0 },
        Some(Err(_)) => 0,
        None => 0,
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
    let key_str: String = match env.get_string(&key) {
        Ok(s) => s.into(),
        Err(_) => return 0,
    };

    match block_on_jni(&mut env, "RiCacheManager::exists", async move {
        manager.exists(&key_str).await
    }) {
        Some(exists) => if exists { 1 } else { 0 },
        None => 0,
    }
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

    let result = block_on_jni(&mut env, "RiCacheManager::clear", async move {
        manager.clear().await
    });
    let _ = result;
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

    match block_on_jni(&mut env, "RiCacheManager::stats", async move {
        manager.stats().await
    }) {
        Some(stats) => Box::into_raw(Box::new(stats)) as jlong,
        None => 0,
    }
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

    match block_on_jni(&mut env, "RiCacheManager::cleanupExpired", async move {
        manager.cleanup_expired().await
    }) {
        Some(Ok(count)) => count as jlong,
        Some(Err(_)) => 0,
        None => 0,
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
    let pattern_str: String = match env.get_string(&pattern) {
        Ok(s) => s.into(),
        Err(_) => return,
    };

    let result = block_on_jni(&mut env, "RiCacheManager::invalidatePattern", async move {
        manager.invalidate_pattern(&pattern_str).await
    });
    let _ = result;
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
