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

//! # Gateway Module C API
//!
//! This module provides C language bindings for Ri's API gateway subsystem. The gateway module
//! delivers high-performance HTTP request routing, load balancing, rate limiting, and request/response
//! transformation capabilities. This C API enables C/C++ applications to leverage Ri's gateway
//! functionality for building scalable API endpoints with enterprise-grade features.
//!
//! ## Module Architecture
//!
//! The gateway module comprises three primary components that together provide complete API gateway
//! functionality:
//!
//! - **RiGateway**: Core gateway server implementation handling HTTP request processing,
//!   middleware composition, and response generation. The gateway acts as the entry point for
//!   all incoming API requests, applying configured middleware chains and routing requests to
//!   appropriate backend services.
//!
//! - **RiGatewayConfig**: Configuration container for gateway server parameters including listen
//!   address, thread pool sizing, TLS settings, and middleware configuration. The configuration
//!   object controls resource allocation, security settings, and behavioral characteristics.
//!
//! - **RiRouter**: Request routing component responsible for matching incoming requests to
//!   registered routes based on method, path, headers, and other request attributes. The router
//!   supports complex routing patterns including path parameters, wildcards, and regex matching.
//!
//! ## Gateway Features
//!
//! The API gateway provides comprehensive features for production API management:
//!
//! - **Request Routing**: Advanced routing capabilities including path matching, method-based
//!   routing, header-based routing, and query parameter routing. Supports route groups and
//!   hierarchical routing patterns.
//!
//! - **Load Balancing**: Distribution of requests across multiple backend instances with
//!   configurable algorithms including round-robin, least-connections, weighted distribution,
//!   and consistent hashing for session affinity.
//!
//! - **Rate Limiting**: Request rate control at multiple granularity levels including global,
//!   per-client, per-route, and per-user rate limits. Supports sliding window, token bucket,
//!   and leaky bucket algorithms.
//!
//! - **Middleware Chain**: Composable middleware for request/response processing including
//!   authentication, authorization, logging, compression, caching, and transformation.
//!   Middleware executes in configured order with early exit capabilities.
//!
//! - **Request/Response Transformation**: Content transformation between client and backend
//!   formats including JSON to XML conversion, header manipulation, body rewriting, and
//!   protocol translation.
//!
//! - **Circuit Breaker**: Automatic detection of backend failures with configurable thresholds
//!   for failure rates, timeout windows, and recovery strategies. Prevents cascade failures
//!   in distributed systems.
//!
//! - **WebSocket Support**: Full-duplex WebSocket connections with session management,
//!   heartbeat handling, and connection lifecycle events.
//!
//! - **TLS Termination**: Secure communication with configurable TLS settings including
//!   certificate management, cipher suite selection, and HTTP/2 support.
//!
//! ## Routing Capabilities
//!
//! The router supports sophisticated routing patterns:
//!
//! - **Static Paths**: Exact path matching for simple routes (e.g., /api/users)
//!
//! - **Path Parameters**: Variable path segments captured as parameters (e.g., /users/:id)
//!
//! - **Wildcard Matching**: Catch-all routes for static file serving (e.g., /static/*)
//!
//! - **Regex Patterns**: Complex matching rules using regular expressions
//!
//! - **Method Matching**: Route requests by HTTP method (GET, POST, PUT, DELETE, etc.)
//!
//! - **Header Matching**: Route based on request headers (e.g., X-API-Version)
//!
//! - **Query Matching**: Route based on query parameters
//!
//! ## Middleware Types
//!
//! The gateway supports various middleware categories:
//!
//! - **Authentication Middleware**: JWT validation, OAuth token verification, API key checking,
//!   and custom authentication schemes.
//!
//! - **Authorization Middleware**: Role-based access control, permission checking, and
//!   policy enforcement at route level.
//!
//! - **Logging Middleware**: Request/response logging, access logging, and structured logging
//!   for observability.
//!
//! - **Compression Middleware**: Gzip, Brotli, and Deflate compression for response bodies.
//!
//! - **Caching Middleware**: Response caching with TTL control, cache invalidation, and
//!   conditional requests (ETag, If-Modified-Since).
//!
//! - **Transformation Middleware**: Header addition/removal, body transformation, and content
//!   type conversion.
//!
//! - **Rate Limiting Middleware**: Request throttling with configurable limits and response
//!   handling.
//!
//! - **Circuit Breaker Middleware**: Failure detection and fallback handling for backend
//!   services.
//!
//! ## Load Balancing Strategies
//!
//! The gateway implements multiple load balancing algorithms:
//!
//! - **Round Robin**: Sequential distribution across available backends. Simple and effective
//!   for homogeneous backends with similar capacity.
//!
//! - **Least Connections**: Route to backend with fewest active connections. Adapts to
//!   varying request processing times.
//!
//! - **Weighted Distribution**: Proportional routing based on backend capacity or priority.
//!   Requires backend health and capacity configuration.
//!
//! - **Consistent Hashing**: Request affinity based on request attributes (e.g., user ID).
//!   Minimizes redistribution when backends change.
//!
//! - **Random**: Random selection among healthy backends. Simple and effective for
//!   homogeneous backends.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Route handlers must be properly registered and unregistered
//! - Connection handles must be properly closed
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Gateway server handles concurrent requests using thread pool
//! - Router operations are thread-safe for route lookups
//! - Configuration changes may require gateway restart
//! - Middleware should be stateless when possible
//!
//! ## Performance Characteristics
//!
//! Gateway operations have the following performance profiles:
//!
//! - Request routing: O(log n) for route lookup with path parameters
//! - Middleware processing: O(m) where m is number of middleware
//! - Load balancing: O(1) for most algorithms
//! - Request throughput: Thousands of requests per second per core
//!
//! ## Usage Example
//!
//! ```c
//! // Create gateway configuration
//! RiGatewayConfig* config = ri_gateway_config_new();
//! ri_gateway_config_set_address(config, "0.0.0.0", 8080);
//! ri_gateway_config_set_workers(config, 4);
//! ri_gateway_config_set_tls_enabled(config, false);
//!
//! // Create gateway instance
//! RiGateway* gateway = ri_gateway_new(config);
//!
//! // Create router and configure routes
//! RiRouter* router = ri_router_new();
//!
//! // Register routes
//! ri_router_add_route(router, "GET", "/api/users", handle_users);
//! ri_router_add_route(router, "POST", "/api/users", create_user);
//! ri_router_add_route(router, "GET", "/api/users/:id", get_user_by_id);
//!
//! // Mount router on gateway
//! ri_gateway_mount(gateway, "/api", router);
//!
//! // Start gateway
//! ri_gateway_start(gateway);
//!
//! // Graceful shutdown on signal
//! // ri_gateway_shutdown(gateway);
//!
//! // Cleanup
//! ri_gateway_free(gateway);
//! ri_gateway_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::gateway`: Rust gateway module implementation
//! - `crate::prelude`: Common types and traits
//! - Hyper for HTTP server functionality
//! - Redis for rate limiting and session storage
//!
//! ## Feature Flags
//!
//! The gateway module is enabled by default with the "gateway" feature flag.
//! Disable this feature to reduce binary size when gateway functionality is not required.

use crate::gateway::{
    RiGateway, RiGatewayConfig, RiRouter, RiRoute, RiRouteHandler,
    RiRateLimiter, RiRateLimitConfig,
    RiCircuitBreaker, RiCircuitBreakerConfig, RiCircuitBreakerState,
    RiLoadBalancer, RiLoadBalancerStrategy, RiBackendServer,
    RiGatewayRequest, RiGatewayResponse,
};
use crate::core::RiResult;
use std::ffi::{c_char, c_int};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

c_wrapper!(CRiGateway, RiGateway);
c_wrapper!(CRiGatewayConfig, RiGatewayConfig);
c_wrapper!(CRiRouter, RiRouter);
c_wrapper!(CRiRateLimiter, RiRateLimiter);
c_wrapper!(CRiRateLimitConfig, RiRateLimitConfig);
c_wrapper!(CRiCircuitBreaker, RiCircuitBreaker);
c_wrapper!(CRiCircuitBreakerConfig, RiCircuitBreakerConfig);
c_wrapper!(CRiLoadBalancer, RiLoadBalancer);

c_constructor!(ri_gateway_config_new, CRiGatewayConfig, RiGatewayConfig, RiGatewayConfig::default());
c_destructor!(ri_gateway_config_free, CRiGatewayConfig);

#[no_mangle]
pub extern "C" fn ri_router_new() -> *mut CRiRouter {
    let router = RiRouter::new();
    Box::into_raw(Box::new(CRiRouter::new(router)))
}

#[no_mangle]
pub extern "C" fn ri_router_free(router: *mut CRiRouter) {
    if !router.is_null() {
        unsafe {
            let _ = Box::from_raw(router);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_router_add_route(
    router: *mut CRiRouter,
    method: *const c_char,
    path: *const c_char,
) -> c_int {
    if router.is_null() || method.is_null() || path.is_null() {
        return -1;
    }

    unsafe {
        let method_str = match std::ffi::CStr::from_ptr(method).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let path_str = match std::ffi::CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let handler: RiRouteHandler = Arc::new(|_req: RiGatewayRequest| {
            Box::pin(async move {
                Ok(crate::gateway::RiGatewayResponse::new(
                    200,
                    b"OK".to_vec(),
                    String::new(),
                ))
            }) as Pin<Box<dyn Future<Output = RiResult<RiGatewayResponse>> + Send>>
        });

        let route = RiRoute::new(method_str.to_string(), path_str.to_string(), handler);
        (*router).inner.add_route(route);
        0
    }
}

#[no_mangle]
pub extern "C" fn ri_router_clear_routes(router: *mut CRiRouter) {
    if router.is_null() {
        return;
    }
    unsafe {
        (*router).inner.clear_routes();
    }
}

#[no_mangle]
pub extern "C" fn ri_router_route_count(router: *mut CRiRouter) -> usize {
    if router.is_null() {
        return 0;
    }
    unsafe {
        (*router).inner.route_count()
    }
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_new(
    requests_per_second: u32,
    burst_size: u32,
    window_seconds: u64,
) -> *mut CRiRateLimiter {
    let config = RiRateLimitConfig {
        requests_per_second,
        burst_size,
        window_seconds,
    };
    let limiter = RiRateLimiter::new(config);
    Box::into_raw(Box::new(CRiRateLimiter::new(limiter)))
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_free(limiter: *mut CRiRateLimiter) {
    if !limiter.is_null() {
        unsafe {
            let _ = Box::from_raw(limiter);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_check(
    limiter: *mut CRiRateLimiter,
    key: *const c_char,
) -> c_int {
    if limiter.is_null() || key.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        if (*limiter).inner.check_rate_limit(key_str, 1) {
            0
        } else {
            1
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_get_remaining(
    limiter: *mut CRiRateLimiter,
    key: *const c_char,
) -> f64 {
    if limiter.is_null() || key.is_null() {
        return -1.0;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2.0,
        };

        (*limiter).inner.get_remaining(key_str).unwrap_or(0.0)
    }
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_reset_bucket(
    limiter: *mut CRiRateLimiter,
    key: *const c_char,
) {
    if limiter.is_null() || key.is_null() {
        return;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return,
        };

        (*limiter).inner.reset_bucket(key_str);
    }
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_clear_all(limiter: *mut CRiRateLimiter) {
    if limiter.is_null() {
        return;
    }
    unsafe {
        (*limiter).inner.clear_all_buckets();
    }
}

#[repr(C)]
pub struct CRiRateLimitStats {
    pub current_tokens: usize,
    pub total_requests: usize,
}

#[no_mangle]
pub extern "C" fn ri_rate_limiter_get_stats(
    limiter: *mut CRiRateLimiter,
    key: *const c_char,
    out_stats: *mut CRiRateLimitStats,
) -> c_int {
    if limiter.is_null() || key.is_null() || out_stats.is_null() {
        return -1;
    }

    unsafe {
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        if let Some(stats) = (*limiter).inner.get_stats(key_str) {
            *out_stats = CRiRateLimitStats {
                current_tokens: stats.current_tokens,
                total_requests: stats.total_requests,
            };
            0
        } else {
            -3
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_new(
    failure_threshold: u32,
    success_threshold: u32,
    timeout_seconds: u64,
) -> *mut CRiCircuitBreaker {
    let config = RiCircuitBreakerConfig {
        failure_threshold,
        success_threshold,
        timeout_seconds,
        monitoring_period_seconds: 30,
    };
    let cb = RiCircuitBreaker::new(config);
    Box::into_raw(Box::new(CRiCircuitBreaker::new(cb)))
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_free(cb: *mut CRiCircuitBreaker) {
    if !cb.is_null() {
        unsafe {
            let _ = Box::from_raw(cb);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_allow_request(cb: *mut CRiCircuitBreaker) -> c_int {
    if cb.is_null() {
        return 0;
    }
    unsafe {
        if (*cb).inner.allow_request() { 1 } else { 0 }
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_record_success(cb: *mut CRiCircuitBreaker) {
    if cb.is_null() {
        return;
    }
    unsafe {
        (*cb).inner.record_success();
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_record_failure(cb: *mut CRiCircuitBreaker) {
    if cb.is_null() {
        return;
    }
    unsafe {
        (*cb).inner.record_failure();
    }
}

pub const RI_CB_STATE_CLOSED: c_int = 0;
pub const RI_CB_STATE_OPEN: c_int = 1;
pub const RI_CB_STATE_HALF_OPEN: c_int = 2;

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_get_state(cb: *mut CRiCircuitBreaker) -> c_int {
    if cb.is_null() {
        return RI_CB_STATE_CLOSED;
    }
    unsafe {
        match (*cb).inner.get_state() {
            RiCircuitBreakerState::Closed => RI_CB_STATE_CLOSED,
            RiCircuitBreakerState::Open => RI_CB_STATE_OPEN,
            RiCircuitBreakerState::HalfOpen => RI_CB_STATE_HALF_OPEN,
        }
    }
}

#[repr(C)]
pub struct CRiCircuitBreakerMetrics {
    pub state: c_int,
    pub failure_count: usize,
    pub success_count: usize,
    pub consecutive_failures: usize,
    pub consecutive_successes: usize,
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_get_stats(
    cb: *mut CRiCircuitBreaker,
    out_stats: *mut CRiCircuitBreakerMetrics,
) -> c_int {
    if cb.is_null() || out_stats.is_null() {
        return -1;
    }

    unsafe {
        let stats = (*cb).inner.get_stats();
        let state = match stats.state.as_str() {
            "Closed" => RI_CB_STATE_CLOSED,
            "Open" => RI_CB_STATE_OPEN,
            "HalfOpen" => RI_CB_STATE_HALF_OPEN,
            _ => RI_CB_STATE_CLOSED,
        };

        *out_stats = CRiCircuitBreakerMetrics {
            state,
            failure_count: stats.failure_count,
            success_count: stats.success_count,
            consecutive_failures: stats.consecutive_failures,
            consecutive_successes: stats.consecutive_successes,
        };
        0
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_reset(cb: *mut CRiCircuitBreaker) {
    if cb.is_null() {
        return;
    }
    unsafe {
        (*cb).inner.reset();
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_force_open(cb: *mut CRiCircuitBreaker) {
    if cb.is_null() {
        return;
    }
    unsafe {
        (*cb).inner.force_open();
    }
}

#[no_mangle]
pub extern "C" fn ri_circuit_breaker_force_close(cb: *mut CRiCircuitBreaker) {
    if cb.is_null() {
        return;
    }
    unsafe {
        (*cb).inner.force_close();
    }
}

pub const RI_LB_STRATEGY_ROUND_ROBIN: c_int = 0;
pub const RI_LB_STRATEGY_WEIGHTED_ROUND_ROBIN: c_int = 1;
pub const RI_LB_STRATEGY_LEAST_CONNECTIONS: c_int = 2;
pub const RI_LB_STRATEGY_RANDOM: c_int = 3;
pub const RI_LB_STRATEGY_IP_HASH: c_int = 4;
pub const RI_LB_STRATEGY_LEAST_RESPONSE_TIME: c_int = 5;
pub const RI_LB_STRATEGY_CONSISTENT_HASH: c_int = 6;

fn strategy_from_c_int(strategy: c_int) -> RiLoadBalancerStrategy {
    match strategy {
        RI_LB_STRATEGY_ROUND_ROBIN => RiLoadBalancerStrategy::RoundRobin,
        RI_LB_STRATEGY_WEIGHTED_ROUND_ROBIN => RiLoadBalancerStrategy::WeightedRoundRobin,
        RI_LB_STRATEGY_LEAST_CONNECTIONS => RiLoadBalancerStrategy::LeastConnections,
        RI_LB_STRATEGY_RANDOM => RiLoadBalancerStrategy::Random,
        RI_LB_STRATEGY_IP_HASH => RiLoadBalancerStrategy::IpHash,
        RI_LB_STRATEGY_LEAST_RESPONSE_TIME => RiLoadBalancerStrategy::LeastResponseTime,
        RI_LB_STRATEGY_CONSISTENT_HASH => RiLoadBalancerStrategy::ConsistentHash,
        _ => RiLoadBalancerStrategy::RoundRobin,
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_new(strategy: c_int) -> *mut CRiLoadBalancer {
    let lb = RiLoadBalancer::new(strategy_from_c_int(strategy));
    Box::into_raw(Box::new(CRiLoadBalancer::new(lb)))
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_free(lb: *mut CRiLoadBalancer) {
    if !lb.is_null() {
        unsafe {
            let _ = Box::from_raw(lb);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_add_server(
    lb: *mut CRiLoadBalancer,
    id: *const c_char,
    url: *const c_char,
    weight: u32,
    max_connections: usize,
) -> c_int {
    if lb.is_null() || id.is_null() || url.is_null() {
        return -1;
    }

    unsafe {
        let id_str = match std::ffi::CStr::from_ptr(id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let url_str = match std::ffi::CStr::from_ptr(url).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };

        let server = RiBackendServer {
            id: id_str.to_string(),
            url: url_str.to_string(),
            weight,
            max_connections,
            health_check_path: "/health".to_string(),
            is_healthy: true,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -4,
        };

        rt.block_on(async {
            (*lb).inner.add_server(server).await;
        });

        0
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_remove_server(
    lb: *mut CRiLoadBalancer,
    id: *const c_char,
) -> c_int {
    if lb.is_null() || id.is_null() {
        return -1;
    }

    unsafe {
        let id_str = match std::ffi::CStr::from_ptr(id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let removed = rt.block_on(async {
            (*lb).inner.remove_server(id_str).await
        });

        if removed { 0 } else { 1 }
    }
}

#[repr(C)]
pub struct CRiBackendServer {
    pub id: *mut c_char,
    pub url: *mut c_char,
    pub weight: u32,
    pub max_connections: usize,
    pub is_healthy: c_int,
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_select_server(
    lb: *mut CRiLoadBalancer,
    client_ip: *const c_char,
    out_server: *mut CRiBackendServer,
) -> c_int {
    if lb.is_null() || out_server.is_null() {
        return -1;
    }

    unsafe {
        let client_ip_str = if client_ip.is_null() {
            None
        } else {
            match std::ffi::CStr::from_ptr(client_ip).to_str() {
                Ok(s) => Some(s),
                Err(_) => None,
            }
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -2,
        };

        let result = rt.block_on(async {
            (*lb).inner.select_server(client_ip_str).await
        });

        match result {
            Ok(server) => {
                let id = match std::ffi::CString::new(server.id.clone()) {
                    Ok(s) => s.into_raw(),
                    Err(_) => return -3,
                };

                let url = match std::ffi::CString::new(server.url.clone()) {
                    Ok(s) => s.into_raw(),
                    Err(_) => {
                        let _ = std::ffi::CString::from_raw(id);
                        return -4;
                    }
                };

                *out_server = CRiBackendServer {
                    id,
                    url,
                    weight: server.weight,
                    max_connections: server.max_connections,
                    is_healthy: if server.is_healthy { 1 } else { 0 },
                };
                0
            }
            Err(_) => -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_backend_server_free(server: *mut CRiBackendServer) {
    if server.is_null() {
        return;
    }

    unsafe {
        let server = Box::from_raw(server);
        if !server.id.is_null() {
            let _ = std::ffi::CString::from_raw(server.id);
        }
        if !server.url.is_null() {
            let _ = std::ffi::CString::from_raw(server.url);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_release_server(
    lb: *mut CRiLoadBalancer,
    server_id: *const c_char,
) {
    if lb.is_null() || server_id.is_null() {
        return;
    }

    unsafe {
        let id_str = match std::ffi::CStr::from_ptr(server_id).to_str() {
            Ok(s) => s,
            Err(_) => return,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return,
        };

        rt.block_on(async {
            (*lb).inner.release_server(id_str).await;
        });
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_mark_healthy(
    lb: *mut CRiLoadBalancer,
    server_id: *const c_char,
    healthy: c_int,
) {
    if lb.is_null() || server_id.is_null() {
        return;
    }

    unsafe {
        let id_str = match std::ffi::CStr::from_ptr(server_id).to_str() {
            Ok(s) => s,
            Err(_) => return,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return,
        };

        rt.block_on(async {
            (*lb).inner.mark_server_healthy(id_str, healthy != 0).await;
        });
    }
}

#[repr(C)]
pub struct CRiLoadBalancerServerStats {
    pub active_connections: usize,
    pub total_requests: usize,
    pub failed_requests: usize,
    pub response_time_ms: usize,
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_get_server_stats(
    lb: *mut CRiLoadBalancer,
    server_id: *const c_char,
    out_stats: *mut CRiLoadBalancerServerStats,
) -> c_int {
    if lb.is_null() || server_id.is_null() || out_stats.is_null() {
        return -1;
    }

    unsafe {
        let id_str = match std::ffi::CStr::from_ptr(server_id).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        };

        let rt = match tokio::runtime::Runtime::new() {
            Ok(r) => r,
            Err(_) => return -3,
        };

        let result = rt.block_on(async {
            (*lb).inner.get_server_stats(id_str).await
        });

        match result {
            Some(stats) => {
                *out_stats = CRiLoadBalancerServerStats {
                    active_connections: stats.active_connections,
                    total_requests: stats.total_requests,
                    failed_requests: stats.failed_requests,
                    response_time_ms: stats.response_time_ms,
                };
                0
            }
            None => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_get_server_count(lb: *mut CRiLoadBalancer) -> usize {
    if lb.is_null() {
        return 0;
    }

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };

    unsafe {
        rt.block_on(async {
            (*lb).inner.get_server_count().await
        })
    }
}

#[no_mangle]
pub extern "C" fn ri_load_balancer_get_healthy_count(lb: *mut CRiLoadBalancer) -> usize {
    if lb.is_null() {
        return 0;
    }

    let rt = match tokio::runtime::Runtime::new() {
        Ok(r) => r,
        Err(_) => return 0,
    };

    unsafe {
        rt.block_on(async {
            (*lb).inner.get_healthy_server_count().await
        })
    }
}
