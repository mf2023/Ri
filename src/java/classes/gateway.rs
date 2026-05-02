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

//! # Gateway Module JNI Bindings
//!
//! JNI bindings for Ri gateway classes.

use crate::gateway::{
    RiBackendServer, RiCircuitBreaker, RiCircuitBreakerConfig, RiCircuitBreakerMetrics,
    RiCircuitBreakerState, RiGateway, RiGatewayConfig, RiLoadBalancer, RiLoadBalancerServerStats,
    RiLoadBalancerStrategy, RiRateLimitConfig, RiRateLimitStats, RiRateLimiter, RiRoute, RiRouter,
    RiSlidingWindowRateLimiter,
};
use jni::objects::{JClass, JString};
use jni::sys::{jboolean, jdouble, jint, jlong, jstring};
use jni::JNIEnv;

// ============================================================================
// RiGateway
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGateway_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let gateway = Box::new(RiGateway::new());
    Box::into_raw(gateway) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGateway_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiGateway);
        }
    }
}

// ============================================================================
// RiGatewayConfig
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGatewayConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiGatewayConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiGatewayConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiGatewayConfig);
        }
    }
}

// ============================================================================
// RiRouter
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let router = Box::new(RiRouter::new());
    Box::into_raw(router) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRouter);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addRoute0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    route_ptr: jlong,
) {
    if ptr != 0 && route_ptr != 0 {
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let route = Box::from_raw(route_ptr as *mut RiRoute);
            router.add_route(*route);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addGetRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.get(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addPostRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.post(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addPutRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.put(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addDeleteRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.delete(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addPatchRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.patch(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addOptionsRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            router.options(&path_str, handler);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_addCustomRoute0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    method: JString,
    path: JString,
) {
    if ptr != 0 {
        let method_str: String = env
            .get_string(&method)
            .expect("Invalid method string")
            .into();
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let router = &*(ptr as *const RiRouter);
            let handler = std::sync::Arc::new(|_req| {
                Box::pin(async move {
                    Ok(crate::gateway::RiGatewayResponse::new(
                        200,
                        b"OK".to_vec(),
                        String::new(),
                    ))
                })
            });
            let route = RiRoute::new(method_str, path_str, handler);
            router.add_route(route);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_getRouteCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let router = &*(ptr as *const RiRouter);
            router.route_count() as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRouter_clearRoutes0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let router = &*(ptr as *const RiRouter);
            router.clear_routes();
        }
    }
}

// ============================================================================
// RiRoute
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRoute_new0(
    env: JNIEnv,
    _class: JClass,
    method: JString,
    path: JString,
) -> jlong {
    let method_str: String = env
        .get_string(&method)
        .expect("Invalid method string")
        .into();
    let path_str: String = env.get_string(&path).expect("Invalid path string").into();

    let handler = std::sync::Arc::new(|_req| {
        Box::pin(async move {
            Ok(crate::gateway::RiGatewayResponse::new(
                200,
                b"OK".to_vec(),
                String::new(),
            ))
        })
    });

    let route = Box::new(RiRoute::new(method_str, path_str, handler));
    Box::into_raw(route) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRoute_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRoute);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRoute_getMethod0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let route = &*(ptr as *const RiRoute);
            env.new_string(&route.method)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRoute_getPath0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let route = &*(ptr as *const RiRoute);
            env.new_string(&route.path)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

// ============================================================================
// RiRateLimitConfig
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiRateLimitConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_newWithValues0(
    _env: JNIEnv,
    _class: JClass,
    requests_per_second: jint,
    burst_size: jint,
    window_seconds: jlong,
) -> jlong {
    let config = Box::new(RiRateLimitConfig {
        requests_per_second: requests_per_second as u32,
        burst_size: burst_size as u32,
        window_seconds: window_seconds as u64,
    });
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRateLimitConfig);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_getRequestsPerSecond0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiRateLimitConfig);
            config.requests_per_second as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_setRequestsPerSecond0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiRateLimitConfig);
            config.requests_per_second = value as u32;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_getBurstSize0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiRateLimitConfig);
            config.burst_size as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_setBurstSize0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiRateLimitConfig);
            config.burst_size = value as u32;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_getWindowSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiRateLimitConfig);
            config.window_seconds as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitConfig_setWindowSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jlong,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiRateLimitConfig);
            config.window_seconds = value as u64;
        }
    }
}

// ============================================================================
// RiRateLimitStats
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitStats_new0(
    _env: JNIEnv,
    _class: JClass,
    current_tokens: jlong,
    total_requests: jlong,
) -> jlong {
    let stats = Box::new(RiRateLimitStats {
        current_tokens: current_tokens as usize,
        total_requests: total_requests as usize,
    });
    Box::into_raw(stats) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRateLimitStats);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitStats_getCurrentTokens0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiRateLimitStats);
            stats.current_tokens as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimitStats_getTotalRequests0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiRateLimitStats);
            stats.total_requests as jlong
        }
    } else {
        0
    }
}

// ============================================================================
// RiRateLimiter
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_new0(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    let config = if config_ptr != 0 {
        unsafe { (*(config_ptr as *const RiRateLimitConfig)).clone() }
    } else {
        RiRateLimitConfig::default()
    };
    let limiter = Box::new(RiRateLimiter::new(config));
    Box::into_raw(limiter) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiRateLimiter);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_checkRateLimit0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
    tokens: jint,
) -> jboolean {
    if ptr != 0 {
        let key_str: String = env.get_string(&key).expect("Invalid key string").into();
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            limiter.check_rate_limit(&key_str, tokens as usize) as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_getStats0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jlong {
    if ptr != 0 {
        let key_str: String = env.get_string(&key).expect("Invalid key string").into();
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            if let Some(stats) = limiter.get_stats(&key_str) {
                Box::into_raw(Box::new(stats)) as jlong
            } else {
                0
            }
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_getRemaining0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) -> jdouble {
    if ptr != 0 {
        let key_str: String = env.get_string(&key).expect("Invalid key string").into();
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            limiter.get_remaining(&key_str).unwrap_or(0.0)
        }
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_resetBucket0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    key: JString,
) {
    if ptr != 0 {
        let key_str: String = env.get_string(&key).expect("Invalid key string").into();
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            limiter.reset_bucket(&key_str);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_clearAllBuckets0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            limiter.clear_all_buckets();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_getConfig0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            let config = limiter.get_config();
            Box::into_raw(Box::new(config)) as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiRateLimiter_bucketCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiRateLimiter);
            limiter.bucket_count() as jint
        }
    } else {
        0
    }
}

// ============================================================================
// RiSlidingWindowRateLimiter
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_new0(
    _env: JNIEnv,
    _class: JClass,
    max_requests: jint,
    window_seconds: jlong,
) -> jlong {
    let limiter = Box::new(RiSlidingWindowRateLimiter::new(
        max_requests as u32,
        window_seconds as u64,
    ));
    Box::into_raw(limiter) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiSlidingWindowRateLimiter);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_allowRequest0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiSlidingWindowRateLimiter);
            limiter.allow_request() as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_getCurrentCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiSlidingWindowRateLimiter);
            limiter.get_current_count() as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_reset0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiSlidingWindowRateLimiter);
            limiter.reset();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_getMaxRequests0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiSlidingWindowRateLimiter);
            limiter.get_max_requests() as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiSlidingWindowRateLimiter_getWindowSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let limiter = &*(ptr as *const RiSlidingWindowRateLimiter);
            limiter.get_window_seconds() as jlong
        }
    } else {
        0
    }
}

// ============================================================================
// RiCircuitBreakerConfig
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_new0(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    let config = Box::new(RiCircuitBreakerConfig::default());
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_newWithValues0(
    _env: JNIEnv,
    _class: JClass,
    failure_threshold: jint,
    success_threshold: jint,
    timeout_seconds: jlong,
    monitoring_period_seconds: jlong,
) -> jlong {
    let config = Box::new(RiCircuitBreakerConfig {
        failure_threshold: failure_threshold as u32,
        success_threshold: success_threshold as u32,
        timeout_seconds: timeout_seconds as u64,
        monitoring_period_seconds: monitoring_period_seconds as u64,
    });
    Box::into_raw(config) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCircuitBreakerConfig);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_getFailureThreshold0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiCircuitBreakerConfig);
            config.failure_threshold as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_setFailureThreshold0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiCircuitBreakerConfig);
            config.failure_threshold = value as u32;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_getSuccessThreshold0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiCircuitBreakerConfig);
            config.success_threshold as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_setSuccessThreshold0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jint,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiCircuitBreakerConfig);
            config.success_threshold = value as u32;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_getTimeoutSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiCircuitBreakerConfig);
            config.timeout_seconds as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_setTimeoutSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jlong,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiCircuitBreakerConfig);
            config.timeout_seconds = value as u64;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_getMonitoringPeriodSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let config = &*(ptr as *const RiCircuitBreakerConfig);
            config.monitoring_period_seconds as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerConfig_setMonitoringPeriodSeconds0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    value: jlong,
) {
    if ptr != 0 {
        unsafe {
            let config = &mut *(ptr as *mut RiCircuitBreakerConfig);
            config.monitoring_period_seconds = value as u64;
        }
    }
}

// ============================================================================
// RiCircuitBreakerMetrics
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_new0(
    env: JNIEnv,
    _class: JClass,
    state: JString,
    failure_count: jlong,
    success_count: jlong,
    consecutive_failures: jlong,
    consecutive_successes: jlong,
) -> jlong {
    let state_str: String = env.get_string(&state).expect("Invalid state string").into();
    let metrics = Box::new(RiCircuitBreakerMetrics {
        state: state_str,
        failure_count: failure_count as usize,
        success_count: success_count as usize,
        consecutive_failures: consecutive_failures as usize,
        consecutive_successes: consecutive_successes as usize,
    });
    Box::into_raw(metrics) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCircuitBreakerMetrics);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_getState0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let metrics = &*(ptr as *const RiCircuitBreakerMetrics);
            env.new_string(&metrics.state)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_getFailureCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let metrics = &*(ptr as *const RiCircuitBreakerMetrics);
            metrics.failure_count as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_getSuccessCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let metrics = &*(ptr as *const RiCircuitBreakerMetrics);
            metrics.success_count as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_getConsecutiveFailures0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let metrics = &*(ptr as *const RiCircuitBreakerMetrics);
            metrics.consecutive_failures as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreakerMetrics_getConsecutiveSuccesses0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let metrics = &*(ptr as *const RiCircuitBreakerMetrics);
            metrics.consecutive_successes as jlong
        }
    } else {
        0
    }
}

// ============================================================================
// RiCircuitBreaker
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_new0(
    _env: JNIEnv,
    _class: JClass,
    config_ptr: jlong,
) -> jlong {
    let config = if config_ptr != 0 {
        unsafe { (*(config_ptr as *const RiCircuitBreakerConfig)).clone() }
    } else {
        RiCircuitBreakerConfig::default()
    };
    let cb = Box::new(RiCircuitBreaker::new(config));
    Box::into_raw(cb) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiCircuitBreaker);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_allowRequest0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.allow_request() as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_recordSuccess0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.record_success();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_recordFailure0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.record_failure();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getState0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            match cb.get_state() {
                RiCircuitBreakerState::Closed => 0,
                RiCircuitBreakerState::Open => 1,
                RiCircuitBreakerState::HalfOpen => 2,
            }
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getStats0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            let stats = cb.get_stats();
            Box::into_raw(Box::new(stats)) as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getConfig0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            let config = cb.get_config();
            Box::into_raw(Box::new(config)) as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_reset0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.reset();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_forceOpen0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.force_open();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_forceClose0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.force_close();
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getFailureRate0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.failure_rate()
        }
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getSuccessRate0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jdouble {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.success_rate()
        }
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_getTotalRequests0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.total_requests() as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_isOpen0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.is_open() as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_isClosed0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.is_closed() as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiCircuitBreaker_isHalfOpen0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let cb = &*(ptr as *const RiCircuitBreaker);
            cb.is_half_open() as jboolean
        }
    } else {
        0
    }
}

// ============================================================================
// RiBackendServer
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_new0(
    env: JNIEnv,
    _class: JClass,
    id: JString,
    url: JString,
) -> jlong {
    let id_str: String = env.get_string(&id).expect("Invalid id string").into();
    let url_str: String = env.get_string(&url).expect("Invalid url string").into();
    let server = Box::new(RiBackendServer::new(id_str, url_str));
    Box::into_raw(server) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiBackendServer);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_getId0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            env.new_string(&server.id)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_getUrl0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            env.new_string(&server.url)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_getWeight0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            server.weight as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_setWeight0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    weight: jint,
) {
    if ptr != 0 {
        unsafe {
            let server = &mut *(ptr as *mut RiBackendServer);
            server.weight = weight as u32;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_getMaxConnections0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            server.max_connections as jint
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_setMaxConnections0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    max_connections: jint,
) {
    if ptr != 0 {
        unsafe {
            let server = &mut *(ptr as *mut RiBackendServer);
            server.max_connections = max_connections as usize;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_getHealthCheckPath0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jstring {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            env.new_string(&server.health_check_path)
                .expect("Failed to create string")
                .into_raw()
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_setHealthCheckPath0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    path: JString,
) {
    if ptr != 0 {
        let path_str: String = env.get_string(&path).expect("Invalid path string").into();
        unsafe {
            let server = &mut *(ptr as *mut RiBackendServer);
            server.health_check_path = path_str;
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiBackendServer_isHealthy0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jboolean {
    if ptr != 0 {
        unsafe {
            let server = &*(ptr as *const RiBackendServer);
            server.is_healthy as jboolean
        }
    } else {
        0
    }
}

// ============================================================================
// RiLoadBalancerServerStats
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_new0(
    _env: JNIEnv,
    _class: JClass,
    active_connections: jlong,
    total_requests: jlong,
    failed_requests: jlong,
    response_time_ms: jlong,
) -> jlong {
    let stats = Box::new(RiLoadBalancerServerStats {
        active_connections: active_connections as usize,
        total_requests: total_requests as usize,
        failed_requests: failed_requests as usize,
        response_time_ms: response_time_ms as usize,
    });
    Box::into_raw(stats) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLoadBalancerServerStats);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_getActiveConnections0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiLoadBalancerServerStats);
            stats.active_connections as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_getTotalRequests0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiLoadBalancerServerStats);
            stats.total_requests as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_getFailedRequests0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiLoadBalancerServerStats);
            stats.failed_requests as jlong
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancerServerStats_getResponseTimeMs0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jlong {
    if ptr != 0 {
        unsafe {
            let stats = &*(ptr as *const RiLoadBalancerServerStats);
            stats.response_time_ms as jlong
        }
    } else {
        0
    }
}

// ============================================================================
// RiLoadBalancer
// ============================================================================

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_new0(
    _env: JNIEnv,
    _class: JClass,
    strategy: jint,
) -> jlong {
    let strategy_enum = match strategy {
        0 => RiLoadBalancerStrategy::RoundRobin,
        1 => RiLoadBalancerStrategy::WeightedRoundRobin,
        2 => RiLoadBalancerStrategy::LeastConnections,
        3 => RiLoadBalancerStrategy::Random,
        4 => RiLoadBalancerStrategy::IpHash,
        5 => RiLoadBalancerStrategy::LeastResponseTime,
        6 => RiLoadBalancerStrategy::ConsistentHash,
        _ => RiLoadBalancerStrategy::RoundRobin,
    };
    let lb = Box::new(RiLoadBalancer::new(strategy_enum));
    Box::into_raw(lb) as jlong
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_free0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    if ptr != 0 {
        unsafe {
            let _ = Box::from_raw(ptr as *mut RiLoadBalancer);
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_addServer0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_ptr: jlong,
) {
    if ptr != 0 && server_ptr != 0 {
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            let server = Box::from_raw(server_ptr as *mut RiBackendServer);
            futures::executor::block_on(async {
                lb.add_server(*server).await;
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_removeServer0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
) -> jboolean {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async { lb.remove_server(&server_id_str).await })
                as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_selectBackend0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    client_ip: JString,
) -> jlong {
    if ptr != 0 {
        let client_ip_str: Option<String> = if client_ip.is_null() {
            None
        } else {
            Some(
                env.get_string(&client_ip)
                    .expect("Invalid client_ip string")
                    .into(),
            )
        };
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                match lb.select_server(client_ip_str.as_deref()).await {
                    Ok(server) => Box::into_raw(Box::new(server)) as jlong,
                    Err(_) => 0,
                }
            })
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_releaseServer0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
) {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                lb.release_server(&server_id_str).await;
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_recordServerFailure0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
) {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                lb.record_server_failure(&server_id_str).await;
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_recordResponseTime0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
    response_time_ms: jlong,
) {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                lb.record_response_time(&server_id_str, response_time_ms as u64)
                    .await;
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_getServerStats0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
) -> jlong {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                match lb.get_server_stats(&server_id_str).await {
                    Some(stats) => Box::into_raw(Box::new(stats)) as jlong,
                    None => 0,
                }
            })
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_markServerHealthy0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
    healthy: jboolean,
) {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async {
                lb.mark_server_healthy(&server_id_str, healthy != 0).await;
            });
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_performHealthCheck0(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
    server_id: JString,
) -> jboolean {
    if ptr != 0 {
        let server_id_str: String = env
            .get_string(&server_id)
            .expect("Invalid server_id string")
            .into();
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async { lb.perform_health_check(&server_id_str).await })
                as jboolean
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_getServerCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async { lb.get_server_count().await as jint })
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_getHealthyServerCount0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            futures::executor::block_on(async { lb.get_healthy_server_count().await as jint })
        }
    } else {
        0
    }
}

#[no_mangle]
pub extern "system" fn Java_com_dunimd_ri_gateway_RiLoadBalancer_getStrategy0(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) -> jint {
    if ptr != 0 {
        unsafe {
            let lb = &*(ptr as *const RiLoadBalancer);
            match lb.get_strategy() {
                RiLoadBalancerStrategy::RoundRobin => 0,
                RiLoadBalancerStrategy::WeightedRoundRobin => 1,
                RiLoadBalancerStrategy::LeastConnections => 2,
                RiLoadBalancerStrategy::Random => 3,
                RiLoadBalancerStrategy::IpHash => 4,
                RiLoadBalancerStrategy::LeastResponseTime => 5,
                RiLoadBalancerStrategy::ConsistentHash => 6,
            }
        }
    } else {
        0
    }
}
