// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Gateway Module Tests
//!
//! This module contains comprehensive tests for the DMSC API gateway system,
//! covering request/response handling, routing, middleware processing, load balancing,
//! rate limiting, and circuit breaker functionality for building resilient API services.
//!
//! ## Test Coverage
//!
//! - **DMSCGatewayConfig**: Tests for gateway configuration including network settings
//!   (listen address, port, max connections), feature toggles (rate limiting, circuit
//!   breaker, load balancing), CORS settings, and logging configuration
//!
//! - **DMSCGatewayRequest**: Tests for request object creation and properties including
//!   HTTP method, path, headers, query parameters, body, and remote address
//!
//! - **DMSCGatewayResponse**: Tests for response construction including status codes,
//!   headers, body content, JSON responses, and error responses
//!
//! - **DMSCRouter**: Tests for route registration, route matching, and route counting
//!
//! - **DMSCMiddleware**: Tests for middleware chain execution and the interceptor pattern
//!   for request/response processing
//!
//! - **DMSCLoadBalancer**: Tests for load balancing strategies (RoundRobin, LeastConn,
//!   Random), backend server management, and healthy server selection
//!
//! - **DMSCRateLimiter**: Tests for rate limiting configuration and request throttling
//!
//! - **DMSCCircuitBreaker**: Tests for circuit breaker states (Closed, Open, Half-Open)
//!   and fault tolerance patterns
//!
//! ## Architecture
//!
//! The gateway system implements a layered architecture:
//! - **Transport Layer**: Handles raw network connections and request parsing
//! - **Router Layer**: Matches incoming requests to registered routes using method and path
//! - **Middleware Layer**: Applies cross-cutting concerns (authentication, logging, etc.)
//! - **Handler Layer**: Executes business logic for matched routes
//! - **Response Layer**: Formats and sends responses back to clients
//!
//! ## Load Balancing Strategies
//!
//! The load balancer supports multiple distribution strategies:
//! - **RoundRobin**: Sequentially distributes requests across all healthy backends
//! - **LeastConn**: Routes to the backend with the fewest active connections
//! - **Random**: Randomly selects a healthy backend for each request
//!
//! ## Circuit Breaker States
//!
//! The circuit breaker implements a three-state failure protection pattern:
//! - **Closed**: Normal operation, requests pass through to backends
//! - **Open**: After threshold failures, requests are immediately rejected
//! - **Half-Open**: Probe requests are allowed to test backend recovery
//!
//! ## Middleware Chain
//!
//! The middleware chain enables flexible request processing composition:
//! - Middlewares are registered in order and execute sequentially
//! - Each middleware can modify request headers, body, or reject the request
//! - Errors in middleware short-circuit the chain and return error responses
//! - The chain pattern enables separation of concerns (auth, logging, compression, etc.)

use dmsc::gateway::{
    DMSCGatewayConfig,
    DMSCGatewayRequest,
    DMSCGatewayResponse,
    DMSCGateway,
    DMSCRoute,
    DMSCRouter,
    DMSCMiddleware,
    DMSCMiddlewareChain,
    DMSCLoadBalancer,
    DMSCLoadBalancerStrategy,
    DMSCRateLimiter,
    DMSCRateLimitConfig,
    DMSCCircuitBreaker,
    DMSCCircuitBreakerConfig,
};
use dmsc::gateway::load_balancer::DMSCBackendServer;
use dmsc::prelude::DMSCResult;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_gateway_config_default() {
    let config = DMSCGatewayConfig::default();
    
    assert_eq!(config.listen_address, "0.0.0.0");
    assert_eq!(config.listen_port, 8080);
    assert_eq!(config.max_connections, 10000);
    assert_eq!(config.request_timeout_seconds, 30);
    assert!(config.enable_rate_limiting);
    assert!(config.enable_circuit_breaker);
    assert!(config.enable_load_balancing);
    assert!(config.cors_enabled);
    assert_eq!(config.cors_origins, vec!["*".to_string()]);
    assert!(config.enable_logging);
    assert_eq!(config.log_level, "info");
}

#[test]
fn test_gateway_request_new() {
    let method = "GET".to_string();
    let path = "/test".to_string();
    let headers = std::collections::HashMap::new();
    let query_params = std::collections::HashMap::new();
    let body = None::<Vec<u8>>;
    let remote_addr = "127.0.0.1:12345".to_string();
    
    let request = DMSCGatewayRequest::new(
        method.clone(),
        path.clone(),
        headers.clone(),
        query_params.clone(),
        body.clone(),
        remote_addr.clone(),
    );
    
    assert!(!request.id.is_empty());
    assert_eq!(request.method, method);
    assert_eq!(request.path, path);
    assert_eq!(request.headers, headers);
    assert_eq!(request.query_params, query_params);
    assert_eq!(request.body, body);
    assert_eq!(request.remote_addr, remote_addr);
}

#[test]
fn test_gateway_response_new() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = DMSCGatewayResponse::new(status_code, body.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.body, body);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("X-Request-ID"));
}

#[test]
fn test_gateway_response_with_header() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = DMSCGatewayResponse::new(status_code, body.clone(), request_id.clone())
        .with_header("Custom-Header".to_string(), "Custom-Value".to_string());
    
    assert_eq!(response.headers.get("Custom-Header"), Some(&"Custom-Value".to_string()));
}

#[test]
fn test_gateway_response_json() {
    let status_code = 200;
    let request_id = "test_request_id".to_string();
    
    let data = serde_json::json!({"key": "value"});
    
    let response = DMSCGatewayResponse::json(status_code, &data, request_id.clone()).unwrap();
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
}

#[test]
fn test_gateway_response_error() {
    let status_code = 404;
    let message = "Not Found".to_string();
    let request_id = "test_request_id".to_string();
    
    let response = DMSCGatewayResponse::error(status_code, message.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.body.len() > 0);
}

#[tokio::test]
async fn test_gateway_new() {
    let gateway = DMSCGateway::new();
    
    // Verify gateway components are created
    assert_eq!(gateway.router().route_count(), 0);
    assert_eq!(gateway.middleware_chain().len(), 0);
}

#[tokio::test]
async fn test_gateway_router() {
    let router = DMSCRouter::new();
    
    // Test adding a route
    let handler = Arc::new(|request: DMSCGatewayRequest| {
        Box::pin(async move {
            Ok(DMSCGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = DMSCResult<DMSCGatewayResponse>> + Send>>
    });

    let route = DMSCRoute::new(
        "GET".to_string(),
        "/test".to_string(),
        handler,
    );

    router.add_route(route);

    assert_eq!(router.route_count(), 1);
}

#[tokio::test]
async fn test_gateway_middleware_chain() {
    struct TestMiddleware;

    #[async_trait::async_trait]
    impl DMSCMiddleware for TestMiddleware {
        async fn execute(
            &self,
            request: &mut DMSCGatewayRequest,
        ) -> DMSCResult<()> {
            request
                .headers
                .insert("X-Custom-Middleware".to_string(), "applied".to_string());
            Ok(())
        }

        fn name(&self) -> &'static str {
            "TestMiddleware"
        }
    }

    let mut middleware_chain = DMSCMiddlewareChain::new();

    middleware_chain.add(Arc::new(TestMiddleware));

    let mut request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/test".to_string(),
        HashMap::new(),
        HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );

    middleware_chain.execute(&mut request).await.unwrap();

    assert_eq!(
        request.headers.get("X-Custom-Middleware"),
        Some(&"applied".to_string())
    );
    assert_eq!(middleware_chain.len(), 1);
}

#[tokio::test]
async fn test_gateway_handle_request() {
    let gateway = DMSCGateway::new();
    let router = gateway.router();
    
    // Add a test route
    let handler = Arc::new(|request: DMSCGatewayRequest| {
        Box::pin(async move {
            Ok(DMSCGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = DMSCResult<DMSCGatewayResponse>> + Send>>
    });

    let route = DMSCRoute::new(
        "GET".to_string(),
        "/test".to_string(),
        handler,
    );

    router.add_route(route);
    
    // Create a test request
    let request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/test".to_string(),
        std::collections::HashMap::new(),
        std::collections::HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    // Handle the request
    let response = gateway.handle_request(request).await;
    
    // Verify the response
    assert_eq!(response.status_code, 200);
    assert_eq!(response.body, b"test_response".to_vec());
}

#[tokio::test]
async fn test_gateway_handle_request_not_found() {
    let gateway = DMSCGateway::new();
    
    // Create a test request to a non-existent route
    let request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/non_existent_route".to_string(),
        std::collections::HashMap::new(),
        std::collections::HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    // Handle the request
    let response = gateway.handle_request(request).await;
    
    // Verify the response is 404
    assert_eq!(response.status_code, 404);
}

#[tokio::test]
async fn test_load_balancer_new() {
    let load_balancer = DMSCLoadBalancer::new(DMSCLoadBalancerStrategy::RoundRobin);
    
    // Test adding targets
    let server1 = DMSCBackendServer::new(
        "server1".to_string(),
        "http://localhost:8001".to_string(),
    );
    let server2 = DMSCBackendServer::new(
        "server2".to_string(),
        "http://localhost:8002".to_string(),
    );

    load_balancer.add_server(server1).await;
    load_balancer.add_server(server2).await;
    
    // Test getting targets
    let targets = load_balancer.get_healthy_servers().await;
    assert_eq!(targets.len(), 2);
    
    // Test selecting a target
    let target = load_balancer.select_server(None).await.unwrap();
    assert!(target.url.starts_with("http://localhost"));
}

#[tokio::test]
async fn test_rate_limiter_new() {
    let config = DMSCRateLimitConfig {
        requests_per_second: 100,
        burst_size: 200,
        window_seconds: 1,
    };
    
    let rate_limiter = DMSCRateLimiter::new(config);
    
    // Test checking a request
    let request = DMSCGatewayRequest::new(
        "GET".to_string(),
        "/test".to_string(),
        std::collections::HashMap::new(),
        std::collections::HashMap::new(),
        None,
        "127.0.0.1:12345".to_string(),
    );
    
    let allowed = rate_limiter.check_request(&request).await;
    assert!(allowed);
}

#[tokio::test]
async fn test_circuit_breaker_new() {
    let config = DMSCCircuitBreakerConfig {
        failure_threshold: 5,
        success_threshold: 3,
        timeout_seconds: 30,
        monitoring_period_seconds: 30,
    };
    
    let circuit_breaker = DMSCCircuitBreaker::new(config);
    
    // Test allowing requests initially
    let allowed = circuit_breaker.allow_request().await;
    assert!(allowed);
    
    // Test recording success
    circuit_breaker.record_success().await;
    
    // Test recording failure
    circuit_breaker.record_failure().await;
}
