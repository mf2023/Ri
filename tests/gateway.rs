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

use dms_core::gateway::{
    DMSGatewayConfig,
    DMSGatewayRequest,
    DMSGatewayResponse,
    DMSGateway,
    DMSRoute,
    DMSRouter,
    DMSMiddleware,
    DMSMiddlewareChain,
    DMSLoadBalancer,
    DMSLoadBalancerStrategy,
    DMSRateLimiter,
    DMSRateLimitConfig,
    DMSCircuitBreaker,
    DMSCircuitBreakerConfig,
};
use dms_core::gateway::load_balancer::DMSBackendServer;
use dms_core::prelude::DMSResult;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_gateway_config_default() {
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
fn test_gateway_request_new() {
    let method = "GET".to_string();
    let path = "/test".to_string();
    let headers = std::collections::HashMap::new();
    let query_params = std::collections::HashMap::new();
    let body = None::<Vec<u8>>;
    let remote_addr = "127.0.0.1:12345".to_string();
    
    let request = DMSGatewayRequest::new(
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
    
    let response = DMSGatewayResponse::new(status_code, body.clone(), request_id.clone());
    
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
    
    let response = DMSGatewayResponse::new(status_code, body.clone(), request_id.clone())
        .with_header("Custom-Header".to_string(), "Custom-Value".to_string());
    
    assert_eq!(response.headers.get("Custom-Header"), Some(&"Custom-Value".to_string()));
}

#[test]
fn test_gateway_response_json() {
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
fn test_gateway_response_error() {
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
    assert_eq!(gateway.router().route_count(), 0);
    assert_eq!(gateway.middleware_chain().len(), 0);
}

#[tokio::test]
async fn test_gateway_router() {
    let router = DMSRouter::new();
    
    // Test adding a route
    let handler = Arc::new(|request: DMSGatewayRequest| {
        Box::pin(async move {
            Ok(DMSGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = DMSResult<DMSGatewayResponse>> + Send>>
    });

    let route = DMSRoute::new(
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
    impl DMSMiddleware for TestMiddleware {
        async fn execute(
            &self,
            request: &mut DMSGatewayRequest,
        ) -> DMSResult<()> {
            request
                .headers
                .insert("X-Custom-Middleware".to_string(), "applied".to_string());
            Ok(())
        }

        fn name(&self) -> &'static str {
            "TestMiddleware"
        }
    }

    let mut middleware_chain = DMSMiddlewareChain::new();

    middleware_chain.add(Arc::new(TestMiddleware));

    let mut request = DMSGatewayRequest::new(
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
    let gateway = DMSGateway::new();
    let router = gateway.router();
    
    // Add a test route
    let handler = Arc::new(|request: DMSGatewayRequest| {
        Box::pin(async move {
            Ok(DMSGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = DMSResult<DMSGatewayResponse>> + Send>>
    });

    let route = DMSRoute::new(
        "GET".to_string(),
        "/test".to_string(),
        handler,
    );

    router.add_route(route);
    
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
    let server1 = DMSBackendServer::new(
        "server1".to_string(),
        "http://localhost:8001".to_string(),
    );
    let server2 = DMSBackendServer::new(
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
        monitoring_period_seconds: 30,
    };
    
    let circuit_breaker = DMSCircuitBreaker::new(config);
    
    // Test allowing requests initially
    let allowed = circuit_breaker.allow_request().await;
    assert!(allowed);
    
    // Test recording success
    circuit_breaker.record_success().await;
    
    // Test recording failure
    circuit_breaker.record_failure().await;
}
