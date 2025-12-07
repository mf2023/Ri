// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
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

extern crate dms;

use dms::gateway::{DMSGatewayConfig, DMSGatewayRequest, DMSGatewayResponse, DMSGateway};
use dms::gateway::{DMSRoute, DMSRouter, DMSMiddleware, DMSMiddlewareChain};
use dms::gateway::{DMSLoadBalancer, DMSLoadBalancerStrategy, DMSRateLimiter, DMSRateLimitConfig};
use dms::gateway::{DMSCircuitBreaker, DMSCircuitBreakerConfig};

#[test]
async fn test_gateway_config_default() {
    let config = DMSGatewayConfig::default();
    
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
async fn test_gateway_request_new() {
    let method = "GET".to_string();
    let path = "/test".to_string();
    let headers = std::collections::HashMap::new();
    let query_params = std::collections::HashMap::new();
    let body = None;
    let remote_addr = "127.0.0.1:12345".to_string();
    
    let request = DMSGatewayRequest::new(method.clone(), path.clone(), headers.clone(), query_params.clone(), body, remote_addr.clone());
    
    assert!(!request.id.is_empty());
    assert_eq!(request.method, method);
    assert_eq!(request.path, path);
    assert_eq!(request.headers, headers);
    assert_eq!(request.query_params, query_params);
    assert_eq!(request.body, body);
    assert_eq!(request.remote_addr, remote_addr);
}

#[test]
async fn test_gateway_response_new() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = DMSGatewayResponse::new(status_code, body.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.body, body);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("X-Request-ID"));
}

#[test]
async fn test_gateway_response_with_header() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = DMSGatewayResponse::new(status_code, body.clone(), request_id.clone())
        .with_header("Custom-Header".to_string(), "Custom-Value".to_string());
    
    assert_eq!(response.headers.get("Custom-Header"), Some(&"Custom-Value".to_string()));
}

#[test]
async fn test_gateway_response_json() {
    let status_code = 200;
    let request_id = "test_request_id".to_string();
    
    let data = serde_json::json!({"key": "value"});
    
    let response = DMSGatewayResponse::json(status_code, &data, request_id.clone()).unwrap();
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
}

#[test]
async fn test_gateway_response_error() {
    let status_code = 404;
    let message = "Not Found".to_string();
    let request_id = "test_request_id".to_string();
    
    let response = DMSGatewayResponse::error(status_code, message.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.body.len() > 0);
}

#[tokio::test]
async fn test_gateway_new() {
    let gateway = DMSGateway::new();
    
    // Verify gateway components are created
    assert!(gateway.router().is_some());
    assert!(gateway.middleware_chain().is_some());
}

#[tokio::test]
async fn test_gateway_router() {
    let gateway = DMSGateway::new();
    let router = gateway.router();
    
    // Test adding a route
    let route = DMSRoute {
        path: "/test".to_string(),
        method: "GET".to_string(),
        handler: Box::new(|request| {
            Box::pin(async move {
                Ok(DMSGatewayResponse::new(200, b"test_response".to_vec(), request.id))
            })
        }),
    };
    
    router.add_route(route).await.unwrap();
    
    // Test getting routes
    let routes = router.get_routes().await;
    assert_eq!(routes.len(), 1);
}

#[tokio::test]
async fn test_gateway_middleware_chain() {
    let gateway = DMSGateway::new();
    let middleware_chain = gateway.middleware_chain();
    
    // Test adding middleware
    let middleware = Box::new(|| {
        Box::pin(async move |request, next| {
            // Add a custom header
            let mut request = request;
            request.headers.insert("X-Custom-Middleware".to_string(), "applied".to_string());
            next(request).await
        })
    });
    
    middleware_chain.add(middleware).await;
    
    // Test middleware count
    assert_eq!(middleware_chain.count().await, 1);
}

#[tokio::test]
async fn test_gateway_handle_request() {
    let gateway = DMSGateway::new();
    let router = gateway.router();
    
    // Add a test route
    let route = DMSRoute {
        path: "/test".to_string(),
        method: "GET".to_string(),
        handler: Box::new(|request| {
            Box::pin(async move {
                Ok(DMSGatewayResponse::new(200, b"test_response".to_vec(), request.id))
            })
        }),
    };
    
    router.add_route(route).await.unwrap();
    
    // Create a test request
    let request = DMSGatewayRequest::new(
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
    let gateway = DMSGateway::new();
    
    // Create a test request to a non-existent route
    let request = DMSGatewayRequest::new(
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
    let load_balancer = DMSLoadBalancer::new(DMSLoadBalancerStrategy::RoundRobin);
    
    // Test adding targets
    load_balancer.add_target("http://localhost:8001".to_string()).await;
    load_balancer.add_target("http://localhost:8002".to_string()).await;
    
    // Test getting targets
    let targets = load_balancer.get_targets().await;
    assert_eq!(targets.len(), 2);
    
    // Test selecting a target
    let target = load_balancer.select_target().await;
    assert!(target.is_some());
}

#[tokio::test]
async fn test_rate_limiter_new() {
    let config = DMSRateLimitConfig {
        requests_per_second: 100,
        burst_size: 200,
        window_seconds: 1,
    };
    
    let rate_limiter = DMSRateLimiter::new(config);
    
    // Test checking a request
    let request = DMSGatewayRequest::new(
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
    let config = DMSCircuitBreakerConfig {
        failure_threshold: 5,
        success_threshold: 3,
        timeout_seconds: 30,
    };
    
    let circuit_breaker = DMSCircuitBreaker::new(config);
    
    // Test allowing requests initially
    let allowed = circuit_breaker.allow_request().await;
    assert!(allowed);
    
    // Test reporting success
    circuit_breaker.report_success().await;
    
    // Test reporting failure
    circuit_breaker.report_failure().await;
}
