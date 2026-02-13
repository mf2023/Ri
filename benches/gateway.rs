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

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dmsc::gateway::{DMSCGateway, DMSCGatewayRequest, DMSCGatewayResponse, DMSCRoute, DMSCRouter};
use dmsc::gateway::{DMSCRateLimiter, DMSCRateLimitConfig, DMSCCircuitBreaker, DMSCCircuitBreakerConfig};
use dmsc::gateway::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};
use std::collections::HashMap;
use std::sync::Arc;

fn bench_gateway_request_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("gateway_request_creation");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("create_simple_request", |b| {
        b.iter(|| {
            let request = DMSCGatewayRequest::new(
                "GET".to_string(),
                "/api/v1/test".to_string(),
                HashMap::new(),
                HashMap::new(),
                None,
                "127.0.0.1:12345".to_string(),
            );
            black_box(request);
        });
    });
    
    group.bench_function("create_request_with_headers", |b| {
        b.iter(|| {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            headers.insert("Authorization".to_string(), "Bearer token".to_string());
            
            let request = DMSCGatewayRequest::new(
                "POST".to_string(),
                "/api/v1/data".to_string(),
                headers,
                HashMap::new(),
                Some(b"{}".to_vec()),
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
    
    group.bench_function("create_simple_response", |b| {
        b.iter(|| {
            let response = DMSCGatewayResponse::new(
                200,
                b"OK".to_vec(),
                "request-123".to_string(),
            );
            black_box(response);
        });
    });
    
    group.bench_function("create_json_response", |b| {
        b.iter(|| {
            let data = serde_json::json!({
                "status": "success",
                "data": {
                    "id": 1,
                    "name": "test"
                }
            });
            let response = DMSCGatewayResponse::json(200, &data, "request-123".to_string()).unwrap();
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

fn bench_router_operations(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let router = Arc::new(DMSCRouter::new());
    
    rt.block_on(async {
        for i in 0..100 {
            let route = DMSCRoute {
                path: format!("/api/v1/route_{}", i),
                method: "GET".to_string(),
                handler: Arc::new(|req| {
                    Box::pin(async move {
                        Ok(DMSCGatewayResponse::new(200, b"OK".to_vec(), req.id.clone()))
                    })
                }),
                ..Default::default()
            };
            router.add_route(route).await.unwrap();
        }
    });
    
    let mut group = c.benchmark_group("gateway_router");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("add_route", |b| {
        b.iter(|| {
            rt.block_on(async {
                let route = DMSCRoute {
                    path: "/api/v1/new_route".to_string(),
                    method: "GET".to_string(),
                    handler: Arc::new(|req| {
                        Box::pin(async move {
                            Ok(DMSCGatewayResponse::new(200, b"OK".to_vec(), req.id.clone()))
                        })
                    }),
                    ..Default::default()
                };
                router.add_route(route).await.unwrap();
                black_box(());
            });
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
    
    rt.block_on(async {
        let router = gateway.router();
        let route = DMSCRoute {
            path: "/api/v1/health".to_string(),
            method: "GET".to_string(),
            handler: Arc::new(|req| {
                Box::pin(async move {
                    Ok(DMSCGatewayResponse::json(
                        200,
                        &serde_json::json!({"status": "ok"}),
                        req.id.clone(),
                    )?)
                })
            }),
            ..Default::default()
        };
        router.add_route(route).await.unwrap();
    });
    
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
    let rate_limiter = Arc::new(DMSCRateLimiter::new(DMSCRateLimitConfig::default()));
    
    let mut group = c.benchmark_group("gateway_rate_limiter");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("check_request_allowed", |b| {
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
                let allowed = rate_limiter.check_request(&request).await;
                black_box(allowed);
            });
        });
    });
    
    group.finish();
}

fn bench_circuit_breaker(c: &mut Criterion) {
    let circuit_breaker = Arc::new(DMSCCircuitBreaker::new(DMSCCircuitBreakerConfig::default()));
    
    let mut group = c.benchmark_group("gateway_circuit_breaker");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("allow_request", |b| {
        b.iter(|| {
            let allowed = circuit_breaker.allow_request();
            black_box(allowed);
        });
    });
    
    group.bench_function("record_success", |b| {
        b.iter(|| {
            circuit_breaker.record_success();
            black_box(());
        });
    });
    
    group.bench_function("record_failure", |b| {
        b.iter(|| {
            circuit_breaker.record_failure();
            black_box(());
        });
    });
    
    group.finish();
}

fn bench_load_balancer(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let lb = Arc::new(DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin));
    
    rt.block_on(async {
        lb.add_server("server1:8080".to_string()).await;
        lb.add_server("server2:8080".to_string()).await;
        lb.add_server("server3:8080".to_string()).await;
    });
    
    let mut group = c.benchmark_group("gateway_load_balancer");
    group.throughput(Throughput::Elements(1));
    
    group.bench_function("select_server_round_robin", |b| {
        b.iter(|| {
            rt.block_on(async {
                let server = lb.select_server().await;
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
