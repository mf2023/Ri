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

//! # Gateway Module Benchmarks
//!
//! This module provides performance benchmarks for the DMSC gateway system,
//! which provides request routing, load balancing, rate limiting, and circuit
//! breaker functionality for API gateway implementations.
//!
//! ## Benchmark Categories
//!
//! 1. **Request/Response Creation**: Object allocation overhead for HTTP modeling
//!
//! 2. **Router Operations**: Route registration and request routing performance
//!
//! 3. **Rate Limiting**: Token bucket algorithm performance
//!
//! 4. **Circuit Breaker**: Failure detection and state management
//!
//! 5. **Load Balancing**: Server selection strategies
//!
//! ## Gateway Architecture
//!
//! DMSCGateway provides:
//! - Path-based routing to backend services
//! - Middleware chain for cross-cutting concerns
//! - Rate limiting to prevent abuse
//! - Circuit breaker for fault tolerance
//! - Load balancing across backend replicas
//!
//! ## Testing Notes
//!
//! Benchmarks use Arc<...> for shared ownership across async tasks.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dmsc::gateway::{DMSCGateway, DMSCGatewayRequest, DMSCGatewayResponse, DMSCRoute, DMSCRouter};
use dmsc::gateway::{DMSCRateLimiter, DMSCRateLimitConfig, DMSCCircuitBreaker, DMSCCircuitBreakerConfig};
use dmsc::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};
use dmsc::prelude::DMSCBackendServer;
use std::collections::HashMap;
use std::sync::Arc;

/// Benchmark: Gateway request object creation.
///
/// DMSCGatewayRequest models incoming HTTP requests:
/// - Method (GET, POST, etc.)
/// - Path and query parameters
/// - Headers
/// - Body
/// - Client address
///
/// Object creation cost matters for high-throughput gateways.
fn bench_gateway_request_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("gateway_request_creation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("create_request", |b| {
        b.iter(|| {
            let request = DMSCGatewayRequest::new(
                "GET".to_string(),
                "/api/v1/users".to_string(),
                HashMap::new(),
                HashMap::new(),
                None,
                "127.0.0.1:12345".to_string(),
            );
            black_box(request);
        });
    });

    group.finish();
}

/// Benchmark: Gateway response object creation.
///
/// DMSCGatewayResponse models HTTP responses:
/// - Status code
/// - Body bytes
/// - Request correlation ID
///
/// Specialized methods exist for JSON and error responses.
fn bench_gateway_response_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("gateway_response_creation");
    group.throughput(Throughput::Elements(1));

    /// Basic response: Status + body
    group.bench_function("create_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::new(
                200,
                b"Hello, World!".to_vec(),
                "request-123".to_string(),
            );
            black_box(response);
        });
    });

    /// JSON response: Serializes data to JSON automatically
    group.bench_function("create_json_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::json(
                200,
                &serde_json::json!({"status": "ok"}),
                "request-123".to_string(),
            );
            let _ = black_box(response);
        });
    });

    /// Error response: Standardized error formatting
    group.bench_function("create_error_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::error(
                404,
                "Not Found".to_string(),
                "request-123".to_string(),
            );
            let _ = black_box(response);
        });
    });

    group.finish();
}

/// Helper function to create a test route with given path and method.
fn create_route(path: &str, method: &str) -> DMSCRoute {
    DMSCRoute::new(
        method.to_string(),
        path.to_string(),
        Arc::new(|req| {
            Box::pin(async move {
                Ok(DMSCGatewayResponse::new(200, b"OK".to_vec(), req.id.clone()))
            })
        }),
    )
}

/// Benchmark: Router operations including route registration and matching.
///
/// DMSCRouter handles:
/// - Route registration via add_route()
/// - Request matching via route()
/// - Handler execution
///
/// Route matching is typically O(n) for registered routes.
fn bench_router_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let router = Arc::new(DMSCRouter::new());

    /// Pre-register 100 routes to simulate a real gateway
    for i in 0..100 {
        let route = create_route(&format!("/api/v1/route_{}", i), "GET");
        router.add_route(route);
    }

    let mut group = c.benchmark_group("gateway_router");
    group.throughput(Throughput::Elements(1));

    /// Add new route: Registration overhead
    group.bench_function("add_route", |b| {
        b.iter(|| {
            let route = create_route("/api/v1/new_route", "GET");
            router.add_route(route);
            black_box(());
        });
    });

    /// Route request: Find matching route and execute handler
    group.bench_function("route_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let request = DMSCGatewayRequest::new(
                    "GET".to_string(),
                    "/api/v1/route_50".to_string(),
                    HashMap::new(),
                    HashMap::new(),
                    None,
                    "127.0.0.1:12345".to_string(),
                );
                let result = router.route(&request).await;
                let _ = black_box(result);
            });
        });
    });

    group.finish();
}

/// Benchmark: Full gateway request handling through the middleware chain.
///
/// DMSCGateway.handle_request() goes through:
/// - Request parsing
/// - Rate limiting check
/// - Route matching
/// - Handler execution
/// - Response formatting
fn bench_gateway_handle_request(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let gateway = Arc::new(DMSCGateway::new());

    /// Register a health check endpoint
    let router = gateway.router();
    let route = DMSCRoute::new(
        "/api/v1/health".to_string(),
        "GET".to_string(),
        Arc::new(|req| {
            Box::pin(async move {
                Ok(DMSCGatewayResponse::json(
                    200,
                    &serde_json::json!({"status": "ok"}),
                    req.id.clone(),
                )?)
            })
        }),
    );
    router.add_route(route);

    let mut group = c.benchmark_group("gateway_handle");
    group.throughput(Throughput::Elements(1));

    group.bench_function("handle_simple_request", |b| {
        b.iter(|| {
            rt.block_on(async {
                let request = DMSCGatewayRequest::new(
                    "GET".to_string(),
                    "/api/v1/health".to_string(),
                    HashMap::new(),
                    HashMap::new(),
                    None,
                    "127.0.0.1:12345".to_string(),
                );
                let response = gateway.handle_request(request).await;
                black_box(response);
            });
        });
    });

    group.finish();
}

/// Benchmark: Rate limiter request checking.
///
/// DMSCRateLimiter implements token bucket algorithm:
/// - Checks if request is allowed under rate limit
/// - Tracks tokens/bucket state
/// - Returns whether request should be allowed or rejected
///
/// Rate limiting is critical for:
/// - API abuse prevention
/// - DoS protection
/// - Fair usage enforcement
fn bench_rate_limiter(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let config = DMSCRateLimitConfig::default();
    let limiter = Arc::new(DMSCRateLimiter::new(config));

    let mut group = c.benchmark_group("gateway_rate_limiter");
    group.throughput(Throughput::Elements(1));

    group.bench_function("check_rate_limit", |b| {
        b.iter(|| {
            rt.block_on(async {
                let request = DMSCGatewayRequest::new(
                    "GET".to_string(),
                    "/api/v1/test".to_string(),
                    HashMap::new(),
                    HashMap::new(),
                    None,
                    "127.0.0.1:12345".to_string(),
                );
                let allowed = limiter.check_request(&request).await;
                black_box(allowed);
            });
        });
    });

    group.finish();
}

/// Benchmark: Circuit breaker state checks and transitions.
///
/// DMSCCircuitBreaker implements circuit breaker pattern:
/// - CLOSED: Normal operation, requests pass through
/// - OPEN: Failures exceeded threshold, requests fail fast
/// - HALF_OPEN: Testing if service recovered
///
/// Used for:
/// - Fault tolerance
/// - Preventing cascade failures
/// - Service health monitoring
fn bench_circuit_breaker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let config = DMSCCircuitBreakerConfig::default();
    let cb = Arc::new(DMSCCircuitBreaker::new(config));

    let mut group = c.benchmark_group("gateway_circuit_breaker");
    group.throughput(Throughput::Elements(1));

    /// Check if circuit is closed (allowing requests)
    group.bench_function("is_closed", |b| {
        b.iter(|| {
            rt.block_on(async {
                let closed = cb.is_closed();
                black_box(closed);
            });
        });
    });

    /// Record successful request (may transition state)
    group.bench_function("record_success", |b| {
        b.iter(|| {
            cb.record_success();
            black_box(());
        });
    });

    group.finish();
}

/// Benchmark: Load balancer server selection.
///
/// DMSCLoadBalancer distributes requests across backend servers:
/// - Round Robin: Sequentially cycles through servers
/// - Random: Randomly selects server
/// - Weighted: Based on server capacity weights
///
/// Load balancing ensures:
/// - Even distribution of load
/// - High availability (failover)
/// - Optimal resource utilization
fn bench_load_balancer(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let lb = Arc::new(DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin));

    /// Add backend servers to the load balancer pool
    rt.block_on(async {
        lb.add_server(DMSCBackendServer::new("server1".to_string(), "http://server1:8080".to_string())).await;
        lb.add_server(DMSCBackendServer::new("server2".to_string(), "http://server2:8080".to_string())).await;
        lb.add_server(DMSCBackendServer::new("server3".to_string(), "http://server3:8080".to_string())).await;
    });

    let mut group = c.benchmark_group("gateway_load_balancer");
    group.throughput(Throughput::Elements(1));

    /// Select server using round-robin strategy
    group.bench_function("select_server_round_robin", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = lb.select_server(None).await;
                let _ = black_box(server);
            });
        });
    });

    group.finish();
}

/// Benchmark group registration for gateway module benchmarks.
criterion_group!(
    gateway_benches,
    bench_gateway_request_creation,
    bench_gateway_response_creation,
    bench_router_operations,
    bench_gateway_handle_request,
    bench_rate_limiter,
    bench_circuit_breaker,
    bench_load_balancer,
);

criterion_main!(gateway_benches);
