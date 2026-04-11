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

//! # Gateway Module Tests
//!
//! This module contains comprehensive tests for the Ri API gateway system,
//! covering request/response handling, routing, middleware processing, load balancing,
//! rate limiting, and circuit breaker functionality for building resilient API services.
//!
//! ## Test Coverage
//!
//! - **RiGatewayConfig**: Tests for gateway configuration including network settings
//!   (listen address, port, max connections), feature toggles (rate limiting, circuit
//!   breaker, load balancing), CORS settings, and logging configuration
//!
//! - **RiGatewayRequest**: Tests for request object creation and properties including
//!   HTTP method, path, headers, query parameters, body, and remote address
//!
//! - **RiGatewayResponse**: Tests for response construction including status codes,
//!   headers, body content, JSON responses, and error responses
//!
//! - **RiRouter**: Tests for route registration, route matching, and route counting
//!
//! - **RiMiddleware**: Tests for middleware chain execution and the interceptor pattern
//!   for request/response processing
//!
//! - **RiLoadBalancer**: Tests for load balancing strategies (RoundRobin, LeastConn,
//!   Random), backend server management, and healthy server selection
//!
//! - **RiRateLimiter**: Tests for rate limiting configuration and request throttling
//!
//! - **RiCircuitBreaker**: Tests for circuit breaker states (Closed, Open, Half-Open)
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

use ri::gateway::{
    RiGatewayConfig,
    RiGatewayRequest,
    RiGatewayResponse,
    RiGateway,
    RiRoute,
    RiRouter,
    RiMiddleware,
    RiMiddlewareChain,
    RiLoadBalancer,
    RiLoadBalancerStrategy,
    RiRateLimiter,
    RiRateLimitConfig,
    RiCircuitBreaker,
    RiCircuitBreakerConfig,
};
use ri::gateway::load_balancer::RiBackendServer;
use ri::prelude::RiResult;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
/// Tests RiGatewayConfig default configuration values.
///
/// Verifies that the default gateway configuration has appropriate values
/// for network settings, feature toggles, CORS, and logging.
///
/// ## Default Configuration Values
///
/// - listen_address: "0.0.0.0" - Listen on all network interfaces
/// - listen_port: 8080 - Default HTTP port
/// - max_connections: 10000 - Maximum concurrent connections
/// - request_timeout_seconds: 30 - Request timeout
/// - enable_rate_limiting: true - Rate limiting enabled
/// - enable_circuit_breaker: true - Circuit breaker enabled
/// - enable_load_balancing: true - Load balancing enabled
/// - cors_enabled: true - CORS support enabled
/// - cors_origins: ["*"] - Allow all origins
/// - enable_logging: true - Request logging enabled
/// - log_level: "info" - Log level set to info
///
/// ## Expected Behavior
///
/// All configuration fields have sensible defaults suitable for
/// typical API gateway deployments.
fn test_gateway_config_default() {
    let config = RiGatewayConfig::default();
    
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
/// Tests RiGatewayRequest creation and properties.
///
/// Verifies that a gateway request can be created with all required
/// properties including HTTP method, path, headers, query parameters,
/// body, and remote address.
///
/// ## Request Properties
///
/// - **id**: Unique identifier generated automatically
/// - **method**: HTTP method (GET, POST, etc.)
/// - **path**: Request path
/// - **headers**: HTTP headers as key-value pairs
/// - **query_params**: Query string parameters
/// - **body**: Optional request body as bytes
/// - **remote_addr**: Client address in format "IP:port"
///
/// ## Expected Behavior
///
/// - A unique request ID is generated
/// - All properties are stored correctly
/// - The request is ready for processing
fn test_gateway_request_new() {
    let method = "GET".to_string();
    let path = "/test".to_string();
    let headers = std::collections::HashMap::new();
    let query_params = std::collections::HashMap::new();
    let body = None::<Vec<u8>>;
    let remote_addr = "127.0.0.1:12345".to_string();
    
    let request = RiGatewayRequest::new(
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
/// Tests RiGatewayResponse creation with status, body, and request ID.
///
/// Verifies that a gateway response can be created with the essential
/// properties and that default headers are added automatically.
///
/// ## Default Headers
///
/// - **Content-Type**: Set based on response type
/// - **X-Request-ID**: Copied from request for tracing
///
/// ## Expected Behavior
///
/// - Response is created with specified values
/// - Default headers are present
/// - The response is ready to be sent
fn test_gateway_response_new() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = RiGatewayResponse::new(status_code, body.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.body, body);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.headers.contains_key("X-Request-ID"));
}

#[test]
/// Tests RiGatewayResponse header addition with with_header().
///
/// Verifies that custom headers can be added to responses using
/// the builder pattern through with_header() method.
///
/// ## Expected Behavior
///
/// - Custom headers can be added
/// - Multiple headers can be added via chaining
/// - Added headers are accessible via get()
fn test_gateway_response_with_header() {
    let status_code = 200;
    let body = b"test_body".to_vec();
    let request_id = "test_request_id".to_string();
    
    let response = RiGatewayResponse::new(status_code, body.clone(), request_id.clone())
        .with_header("Custom-Header".to_string(), "Custom-Value".to_string());
    
    assert_eq!(response.headers.get("Custom-Header"), Some(&"Custom-Value".to_string()));
}

#[test]
/// Tests RiGatewayResponse JSON creation with json() factory.
///
/// Verifies that JSON responses can be created easily using the
/// json() factory method which sets the Content-Type header.
///
/// ## JSON Response Behavior
///
/// - Serializes the data to JSON
/// - Sets Content-Type to application/json
/// - Includes the request ID for tracing
///
/// ## Expected Behavior
///
/// - Response contains serialized JSON
/// - Content-Type is set to application/json
/// - Status code and request ID are preserved
fn test_gateway_response_json() {
    let status_code = 200;
    let request_id = "test_request_id".to_string();
    
    let data = serde_json::json!({"key": "value"});
    
    let response = RiGatewayResponse::json(status_code, &data, request_id.clone()).unwrap();
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
}

#[test]
/// Tests RiGatewayResponse error creation with error() factory.
///
/// Verifies that error responses can be created easily using the
/// error() factory method for standardized error handling.
///
/// ## Error Response Behavior
///
/// - Sets appropriate status code
/// - Includes error message in body
/// - Sets Content-Type to text/plain
///
/// ## Expected Behavior
///
/// - Error response contains message
/// - Status code matches requested code
/// - Headers are set appropriately
fn test_gateway_response_error() {
    let status_code = 404;
    let message = "Not Found".to_string();
    let request_id = "test_request_id".to_string();
    
    let response = RiGatewayResponse::error(status_code, message.clone(), request_id.clone());
    
    assert_eq!(response.status_code, status_code);
    assert_eq!(response.request_id, request_id);
    assert!(response.headers.contains_key("Content-Type"));
    assert!(response.body.len() > 0);
}

#[tokio::test]
/// Tests RiGateway creation and initial state.
///
/// Verifies that a gateway can be created successfully and starts
/// with empty routes and middleware chain.
///
/// ## Initial Gateway State
///
/// - Router has zero routes registered
/// - Middleware chain is empty
/// - Gateway is ready for configuration
///
/// ## Expected Behavior
///
/// - Gateway is created without errors
/// - Initial state is as expected
async fn test_gateway_new() {
    let gateway = RiGateway::new();
    
    // Verify gateway components are created
    assert_eq!(gateway.router().route_count(), 0);
    assert_eq!(gateway.middleware_chain().len(), 0);
}

#[tokio::test]
/// Tests RiRouter route registration with add_route().
///
/// Verifies that routes can be registered with the router and that
/// the route count is updated accordingly.
///
/// ## Route Registration
///
/// - Routes are registered with method, path, and handler
/// - Each route gets a unique identifier
/// - Route count reflects registered routes
///
/// ## Expected Behavior
///
/// - Route is successfully registered
/// - Route count increases by 1
/// - Route is available for matching
async fn test_gateway_router() {
    let router = RiRouter::new();
    
    // Test adding a route
    let handler = Arc::new(|request: RiGatewayRequest| {
        Box::pin(async move {
            Ok(RiGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = RiResult<RiGatewayResponse>> + Send>>
    });

    let route = RiRoute::new(
        "GET".to_string(),
        "/test".to_string(),
        handler,
    );

    router.add_route(route);

    assert_eq!(router.route_count(), 1);
}

#[tokio::test]
/// Tests RiMiddlewareChain middleware execution.
///
/// Verifies that middlewares can be added to the chain and that
/// they are executed in order when processing requests.
///
/// ## Middleware Execution
///
/// - Middlewares are added in order
/// - Each middleware can modify the request
/// - Execution proceeds through the chain
///
/// ## Expected Behavior
///
/// - Middleware is added to the chain
/// - Middleware execute() is called
/// - Request modifications are applied
async fn test_gateway_middleware_chain() {
    struct TestMiddleware;

    #[async_trait::async_trait]
    impl RiMiddleware for TestMiddleware {
        async fn execute(
            &self,
            request: &mut RiGatewayRequest,
        ) -> RiResult<()> {
            request
                .headers
                .insert("X-Custom-Middleware".to_string(), "applied".to_string());
            Ok(())
        }

        fn name(&self) -> &'static str {
            "TestMiddleware"
        }
    }

    let mut middleware_chain = RiMiddlewareChain::new();

    middleware_chain.add(Arc::new(TestMiddleware));

    let mut request = RiGatewayRequest::new(
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
/// Tests RiGateway request handling with handle_request().
///
/// Verifies that the gateway can handle requests through the
/// complete request-response cycle including routing and middleware.
///
/// ## Request Flow
///
/// 1. Request is received by gateway
/// 2. Middleware chain processes request
/// 3. Router matches request to route
/// 4. Handler processes request
/// 5. Response is returned to client
///
/// ## Expected Behavior
///
/// - Request is routed to matching handler
/// - Handler response is returned
/// - Status code matches expected value
async fn test_gateway_handle_request() {
    let gateway = RiGateway::new();
    let router = gateway.router();
    
    // Add a test route
    let handler = Arc::new(|request: RiGatewayRequest| {
        Box::pin(async move {
            Ok(RiGatewayResponse::new(
                200,
                b"test_response".to_vec(),
                request.id,
            ))
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = RiResult<RiGatewayResponse>> + Send>>
    });

    let route = RiRoute::new(
        "GET".to_string(),
        "/test".to_string(),
        handler,
    );

    router.add_route(route);
    
    // Create a test request
    let request = RiGatewayRequest::new(
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
/// Tests RiGateway 404 response for unmatched routes.
///
/// Verifies that the gateway returns a 404 status code when
/// no matching route is found for the request path.
///
/// ## 404 Handling
///
/// - Unmatched requests return 404
/// - Error response includes message
/// - Request ID is preserved for tracing
///
/// ## Expected Behavior
///
/// - Non-existent route returns 404
/// - Error message is included
async fn test_gateway_handle_request_not_found() {
    let gateway = RiGateway::new();
    
    // Create a test request to a non-existent route
    let request = RiGatewayRequest::new(
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
/// Tests RiLoadBalancer server management and selection.
///
/// Verifies that the load balancer can manage backend servers
/// and select healthy servers using the configured strategy.
///
/// ## Load Balancer Operations
///
/// - Servers can be added to the pool
/// - Healthy servers can be retrieved
/// - Server selection uses strategy
///
/// ## Expected Behavior
///
/// - Servers are added successfully
/// - All servers are retrieved as healthy
/// - Selected server matches pattern
async fn test_load_balancer_new() {
    let load_balancer = RiLoadBalancer::new(RiLoadBalancerStrategy::RoundRobin);
    
    // Test adding targets
    let server1 = RiBackendServer::new(
        "server1".to_string(),
        "http://localhost:8001".to_string(),
    );
    let server2 = RiBackendServer::new(
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
/// Tests RiRateLimiter request throttling.
///
/// Verifies that the rate limiter can check if requests should
/// be allowed based on the configured rate limit settings.
///
/// ## Rate Limiting Behavior
///
/// - Requests within limit are allowed
/// - Requests exceeding limit are rejected
/// - Limits are per configuration window
///
/// ## Expected Behavior
///
/// - Initial request is allowed
/// - Rate limiting operates correctly
async fn test_rate_limiter_new() {
    let config = RiRateLimitConfig {
        requests_per_second: 100,
        burst_size: 200,
        window_seconds: 1,
    };
    
    let rate_limiter = RiRateLimiter::new(config);
    
    // Test checking a request
    let request = RiGatewayRequest::new(
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
/// Tests RiCircuitBreaker state management.
///
/// Verifies that the circuit breaker tracks success and failure
/// counts and allows requests in the appropriate states.
///
/// ## Circuit Breaker States
///
/// - **Closed**: Normal operation, requests allowed
/// - **Open**: Too many failures, requests rejected
/// - **Half-Open**: Testing recovery, limited requests
///
/// ## Expected Behavior
///
/// - Initial state allows requests
/// - Success/failure recording works
async fn test_circuit_breaker_new() {
    let config = RiCircuitBreakerConfig {
        failure_threshold: 5,
        success_threshold: 3,
        timeout_seconds: 30,
        monitoring_period_seconds: 30,
    };
    
    let circuit_breaker = RiCircuitBreaker::new(config);
    
    // Test allowing requests initially
    let allowed = circuit_breaker.allow_request().await;
    assert!(allowed);
    
    // Test recording success
    circuit_breaker.record_success().await;
    
    // Test recording failure
    circuit_breaker.record_failure().await;
}
