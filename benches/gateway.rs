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
//! This benchmark suite measures the performance of DMSC gateway operations.

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dmsc::gateway::{DMSCGateway, DMSCGatewayRequest, DMSCGatewayResponse, DMSCRoute, DMSCRouter};
use dmsc::gateway::{DMSCRateLimiter, DMSCRateLimitConfig, DMSCCircuitBreaker, DMSCCircuitBreakerConfig};
use dmsc::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};
use dmsc::prelude::DMSCBackendServer;
use std::collections::HashMap;
use std::sync::Arc;

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

fn bench_gateway_response_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("gateway_response_creation");
    group.throughput(Throughput::Elements(1));
    
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
    
    group.bench_function("create_json_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::json(
                200,
                &serde_json::json!({"status": "ok"}),
                "request-123".to_string(),
            );
            black_box(response);
        });
    });
    
    group.bench_function("create_error_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::error(
                404,
                "Not Found".to_string(),
                "request-123".to_string(),
            );
            black_box(response);
        });
    });
    
    group.finish();
}

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

fn bench_router_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let router = Arc::new(DMSCRouter::new());
    
    for i in 0..100 {
        let route = create_route(&format!("/api/v1/route_{}", i), "GET");
        router.add_route(route);
    }
    
    let mut group = c.benchmark_group("gateway_router");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("add_route", |b| {
        b.iter(|| {
            let route = create_route("/api/v1/new_route", "GET");
            router.add_route(route);
            black_box(());
        });
    });
    
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
                black_box(result);
            });
        });
    });
    
    group.finish();
}

fn bench_gateway_handle_request(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let gateway = Arc::new(DMSCGateway::new());
    
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

fn bench_circuit_breaker(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let config = DMSCCircuitBreakerConfig::default();
    let cb = Arc::new(DMSCCircuitBreaker::new(config));
    
    let mut group = c.benchmark_group("gateway_circuit_breaker");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("is_closed", |b| {
        b.iter(|| {
            rt.block_on(async {
                let closed = cb.is_closed();
                black_box(closed);
            });
        });
    });
    
    group.bench_function("record_success", |b| {
        b.iter(|| {
            cb.record_success();
            black_box(());
        });
    });
    
    group.finish();
}

fn bench_load_balancer(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let lb = Arc::new(DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin));
    
    rt.block_on(async {
        lb.add_server(DMSCBackendServer::new("server1".to_string(), "http://server1:8080".to_string())).await;
        lb.add_server(DMSCBackendServer::new("server2".to_string(), "http://server2:8080".to_string())).await;
        lb.add_server(DMSCBackendServer::new("server3".to_string(), "http://server3:8080".to_string())).await;
    });
    
    let mut group = c.benchmark_group("gateway_load_balancer");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("select_server_round_robin", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = lb.select_server(None).await;
                black_box(server);
            });
        });
    });
    
    group.finish();
}

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
