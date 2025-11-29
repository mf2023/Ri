//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

#![allow(non_snake_case)]

//! # Gateway Module
//! 
//! This module provides a comprehensive API gateway functionality for DMS, offering routing, middleware support,
//! load balancing, rate limiting, and circuit breaking capabilities.
//! 
//! ## Key Components
//! 
//! - **DMSGateway**: Main gateway struct implementing the DMSModule trait
//! - **DMSGatewayConfig**: Configuration for gateway behavior
//! - **DMSGatewayRequest**: Request structure for gateway operations
//! - **DMSGatewayResponse**: Response structure for gateway operations
//! - **DMSRoute**: Route definition for API endpoints
//! - **DMSRouter**: Router for handling request routing
//! - **DMSMiddleware**: Middleware interface for request processing
//! - **DMSMiddlewareChain**: Chain of middleware for sequential execution
//! - **DMSLoadBalancer**: Load balancing for distributing requests across multiple services
//! - **DMSLoadBalancerStrategy**: Load balancing strategies (RoundRobin, LeastConnections, etc.)
//! - **DMSRateLimiter**: Rate limiting for controlling request rates
//! - **DMSRateLimitConfig**: Configuration for rate limiting
//! - **DMSCircuitBreaker**: Circuit breaker for preventing cascading failures
//! - **DMSCircuitBreakerConfig**: Configuration for circuit breakers
//! 
//! ## Design Principles
//! 
//! 1. **Modular Design**: Separate components for routing, middleware, load balancing, rate limiting, and circuit breaking
//! 2. **Async-First**: All gateway operations are asynchronous
//! 3. **Configurable**: Highly configurable gateway behavior through DMSGatewayConfig
//! 4. **Middleware Support**: Extensible middleware system for request processing
//! 5. **Resilience**: Built-in circuit breaker and rate limiting for service resilience
//! 6. **Load Balancing**: Support for distributing requests across multiple service instances
//! 7. **CORS Support**: Built-in CORS configuration for cross-origin requests
//! 8. **Logging**: Comprehensive logging support
//! 9. **Service Integration**: Implements DMSModule trait for seamless integration into DMS
//! 10. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use dms::gateway::{DMSGateway, DMSGatewayConfig, DMSRoute};
//! use std::collections::HashMap;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create gateway configuration
//!     let gateway_config = DMSGatewayConfig {
//!         listen_address: "0.0.0.0".to_string(),
//!         listen_port: 8080,
//!         max_connections: 10000,
//!         request_timeout_seconds: 30,
//!         enable_rate_limiting: true,
//!         enable_circuit_breaker: true,
//!         enable_load_balancing: true,
//!         cors_enabled: true,
//!         cors_origins: vec!["*".to_string()],
//!         cors_methods: vec!["GET".to_string(), "POST".to_string()],
//!         cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string()],
//!         enable_logging: true,
//!         log_level: "info".to_string(),
//!     };
//!     
//!     // Create gateway instance
//!     let gateway = DMSGateway::_Fnew();
//!     
//!     // Get router and add routes
//!     let router = gateway._Frouter();
//!     
//!     // Add a simple GET route
//!     router._Fadd_route(DMSRoute {
//!         path: "/api/v1/health".to_string(),
//!         method: "GET".to_string(),
//!         handler: Arc::new(|req| Box::pin(async move {
//!             Ok(DMSGatewayResponse::_Fjson(200, &serde_json::json!({ "status": "ok" }), req.id.clone())?)
//!         })),
//!         ..Default::default()
//!     }).await?;
//!     
//!     // Add middleware
//!     let middleware_chain = gateway._Fmiddleware_chain();
//!     middleware_chain._Fadd_middleware(Arc::new(|req, next| Box::pin(async move {
//!         // Log request
//!         println!("Request: {} {}", req.method, req.path);
//!         next(req).await
//!     }))).await;
//!     
//!     // Handle a sample request
//!     let sample_request = DMSGatewayRequest::_Fnew(
//!         "GET".to_string(),
//!         "/api/v1/health".to_string(),
//!         HashMap::new(),
//!         HashMap::new(),
//!         None,
//!         "127.0.0.1:12345".to_string(),
//!     );
//!     
//!     let response = gateway._Fhandle_request(sample_request).await;
//!     println!("Response: {} {}", response.status_code, String::from_utf8_lossy(&response.body));
//!     
//!     Ok(())
//! }
//! ```

use crate::core::{DMSModule, DMSServiceContext};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod middleware;
pub mod routing;
pub mod circuit_breaker;
pub mod load_balancer;
pub mod rate_limiter;

pub use routing::{DMSRoute, DMSRouter};
pub use middleware::{DMSMiddleware, DMSMiddlewareChain};
pub use load_balancer::{DMSLoadBalancer, DMSLoadBalancerStrategy};
pub use rate_limiter::{DMSRateLimiter, DMSRateLimitConfig};
pub use circuit_breaker::{DMSCircuitBreaker, DMSCircuitBreakerConfig};

/// Configuration for the DMS Gateway.
/// 
/// This struct defines the configuration options for the API gateway, including network settings,
/// feature toggles, and CORS configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSGatewayConfig {
    /// Address to listen on
    pub listen_address: String,
    /// Port to listen on
    pub listen_port: u16,
    /// Maximum number of concurrent connections
    pub max_connections: usize,
    /// Request timeout in seconds
    pub request_timeout_seconds: u64,
    /// Whether to enable rate limiting
    pub enable_rate_limiting: bool,
    /// Whether to enable circuit breaker
    pub enable_circuit_breaker: bool,
    /// Whether to enable load balancing
    pub enable_load_balancing: bool,
    /// Whether to enable CORS
    pub cors_enabled: bool,
    /// Allowed CORS origins
    pub cors_origins: Vec<String>,
    /// Allowed CORS methods
    pub cors_methods: Vec<String>,
    /// Allowed CORS headers
    pub cors_headers: Vec<String>,
    /// Whether to enable logging
    pub enable_logging: bool,
    /// Log level for gateway operations
    pub log_level: String,
}

impl Default for DMSGatewayConfig {
    /// Returns the default configuration for the gateway.
    /// 
    /// Default values:
    /// - listen_address: "0.0.0.0"
    /// - listen_port: 8080
    /// - max_connections: 10000
    /// - request_timeout_seconds: 30
    /// - enable_rate_limiting: true
    /// - enable_circuit_breaker: true
    /// - enable_load_balancing: true
    /// - cors_enabled: true
    /// - cors_origins: ["*"]
    /// - cors_methods: ["GET", "POST", "PUT", "DELETE", "OPTIONS"]
    /// - cors_headers: ["Content-Type", "Authorization", "X-Requested-With"]
    /// - enable_logging: true
    /// - log_level: "info"
    fn default() -> Self {
        Self {
            listen_address: "0.0.0.0".to_string(),
            listen_port: 8080,
            max_connections: 10000,
            request_timeout_seconds: 30,
            enable_rate_limiting: true,
            enable_circuit_breaker: true,
            enable_load_balancing: true,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
            cors_methods: vec!["GET".to_string(), "POST".to_string(), "PUT".to_string(), "DELETE".to_string(), "OPTIONS".to_string()],
            cors_headers: vec!["Content-Type".to_string(), "Authorization".to_string(), "X-Requested-With".to_string()],
            enable_logging: true,
            log_level: "info".to_string(),
        }
    }
}

/// Request structure for gateway operations.
/// 
/// This struct represents an HTTP request received by the gateway, including method, path, headers,
/// query parameters, body, and remote address.
#[derive(Debug, Clone)]
pub struct DMSGatewayRequest {
    /// Unique request ID
    pub id: String,
    /// HTTP method (GET, POST, etc.)
    pub method: String,
    /// Request path
    pub path: String,
    /// HTTP headers
    pub headers: HashMap<String, String>,
    /// Query parameters
    pub query_params: HashMap<String, String>,
    /// Request body (if any)
    pub body: Option<Vec<u8>>,
    /// Remote address of the client
    pub remote_addr: String,
    /// Timestamp when the request was created
    pub timestamp: std::time::Instant,
}

impl DMSGatewayRequest {
    /// Creates a new gateway request.
    /// 
    /// # Parameters
    /// 
    /// - `method`: HTTP method
    /// - `path`: Request path
    /// - `headers`: HTTP headers
    /// - `query_params`: Query parameters
    /// - `body`: Request body (optional)
    /// - `remote_addr`: Remote address of the client
    /// 
    /// # Returns
    /// 
    /// A new `DMSGatewayRequest` instance
    pub fn _Fnew(
        method: String,
        path: String,
        headers: HashMap<String, String>,
        query_params: HashMap<String, String>,
        body: Option<Vec<u8>>,
        remote_addr: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            method,
            path,
            headers,
            query_params,
            body,
            remote_addr,
            timestamp: std::time::Instant::now(),
        }
    }
}

/// Response structure for gateway operations.
/// 
/// This struct represents an HTTP response returned by the gateway, including status code,
/// headers, body, and request ID.
#[derive(Debug, Clone)]
pub struct DMSGatewayResponse {
    /// HTTP status code
    pub status_code: u16,
    /// HTTP headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Request ID associated with this response
    pub request_id: String,
}

impl DMSGatewayResponse {
    /// Creates a new gateway response.
    /// 
    /// # Parameters
    /// 
    /// - `status_code`: HTTP status code
    /// - `body`: Response body
    /// - `request_id`: Request ID associated with this response
    /// 
    /// # Returns
    /// 
    /// A new `DMSGatewayResponse` instance
    pub fn new(status_code: u16, body: Vec<u8>, request_id: String) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("X-Request-ID".to_string(), request_id.clone());
        
        Self {
            status_code,
            headers,
            body,
            request_id,
        }
    }

    /// Adds a header to the response.
    /// 
    /// # Parameters
    /// 
    /// - `key`: Header name
    /// - `value`: Header value
    /// 
    /// # Returns
    /// 
    /// The updated `DMSGatewayResponse` instance
    pub fn _Fwith_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Creates a JSON response.
    /// 
    /// # Parameters
    /// 
    /// - `status_code`: HTTP status code
    /// - `data`: Data to serialize as JSON
    /// - `request_id`: Request ID associated with this response
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Self>` containing the JSON response
    pub fn _Fjson<T: serde::Serialize>(status_code: u16, data: &T, request_id: String) -> crate::core::DMSResult<Self> {
        let body = serde_json::to_vec(data)?;
        Ok(Self::new(status_code, body, request_id))
    }

    /// Creates an error response.
    /// 
    /// # Parameters
    /// 
    /// - `status_code`: HTTP status code
    /// - `message`: Error message
    /// - `request_id`: Request ID associated with this response
    /// 
    /// # Returns
    /// 
    /// A new `DMSGatewayResponse` instance with error information
    pub fn _Ferror(status_code: u16, message: String, request_id: String) -> Self {
        let error_body = serde_json::json!({
            "error": message,
            "request_id": request_id
        });
        
        let body = serde_json::to_vec(&error_body).unwrap_or_else(|_| b"{}".to_vec());
        Self::new(status_code, body, request_id)
    }
}

/// Main gateway struct implementing the DMSModule trait.
/// 
/// This struct provides the core gateway functionality, including request handling,
/// routing, middleware execution, rate limiting, and circuit breaking.
pub struct DMSGateway {
    /// Gateway configuration, protected by a RwLock for thread-safe access
    config: RwLock<DMSGatewayConfig>,
    /// Router for handling request routing
    router: Arc<DMSRouter>,
    /// Middleware chain for request processing
    middleware_chain: Arc<DMSMiddlewareChain>,
    /// Rate limiter for controlling request rates
    rate_limiter: Option<Arc<DMSRateLimiter>>,
    /// Circuit breaker for preventing cascading failures
    circuit_breaker: Option<Arc<DMSCircuitBreaker>>,
    /// Load balancer for distributing requests across services
    #[allow(dead_code)]
    load_balancer: Option<Arc<DMSLoadBalancer>>,
}

impl DMSGateway {
    /// Creates a new gateway instance with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSGateway` instance
    pub fn _Fnew() -> Self {
        let config = DMSGatewayConfig::default();
        let router = Arc::new(DMSRouter::_Fnew());
        let middleware_chain = Arc::new(DMSMiddlewareChain::_Fnew());
        
        let rate_limiter = if config.enable_rate_limiting {
            Some(Arc::new(DMSRateLimiter::_Fnew(DMSRateLimitConfig::default())))
        } else {
            None
        };
        
        let circuit_breaker = if config.enable_circuit_breaker {
            Some(Arc::new(DMSCircuitBreaker::_Fnew(DMSCircuitBreakerConfig::default())))
        } else {
            None
        };
        
        let load_balancer = if config.enable_load_balancing {
            Some(Arc::new(DMSLoadBalancer::_Fnew(DMSLoadBalancerStrategy::RoundRobin)))
        } else {
            None
        };

        Self {
            config: RwLock::new(config),
            router,
            middleware_chain,
            rate_limiter,
            circuit_breaker,
            load_balancer,
        }
    }

    /// Returns a reference to the router.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSRouter> providing thread-safe access to the router
    pub fn _Frouter(&self) -> Arc<DMSRouter> {
        self.router.clone()
    }

    /// Returns a reference to the middleware chain.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSMiddlewareChain> providing thread-safe access to the middleware chain
    pub fn _Fmiddleware_chain(&self) -> Arc<DMSMiddlewareChain> {
        self.middleware_chain.clone()
    }

    /// Handles a gateway request.
    /// 
    /// This method processes a request through the gateway pipeline, including:
    /// 1. Rate limiting
    /// 2. Circuit breaker check
    /// 3. Middleware chain execution
    /// 4. Request routing
    /// 5. Route handler execution
    /// 
    /// # Parameters
    /// 
    /// - `request`: The request to handle
    /// 
    /// # Returns
    /// 
    /// A `DMSGatewayResponse` containing the response to the request
    pub async fn _Fhandle_request(&self, request: DMSGatewayRequest) -> DMSGatewayResponse {
        let request_id = request.id.clone();
        
        // Apply rate limiting
        if let Some(rate_limiter) = &self.rate_limiter {
            if !rate_limiter._Fcheck_request(&request).await {
                return DMSGatewayResponse::new(429, "Rate limit exceeded".to_string().into_bytes(), request_id);
            }
        }

        // Apply circuit breaker
        if let Some(circuit_breaker) = &self.circuit_breaker {
            if !circuit_breaker._Fallow_request().await {
                return DMSGatewayResponse::new(503, "Service temporarily unavailable".to_string().into_bytes(), request_id);
            }
        }

        // Apply middleware chain
        let mut request = request;
        match self.middleware_chain._Fexecute(&mut request).await {
            Ok(()) => {
                // Route the request
                match self.router._Froute(&request).await {
                    Ok(route_handler) => {
                        // Execute the route handler
                        match route_handler(request).await {
                            Ok(response) => response,
                            Err(e) => {
                                DMSGatewayResponse::new(500, format!("Internal server error: {e}").into_bytes(), request_id)
                            }
                        }
                    },
                    Err(e) => {
                        DMSGatewayResponse::new(404, format!("Route not found: {e}").into_bytes(), request_id)
                    }
                }
            },
            Err(e) => {
                DMSGatewayResponse::new(403, format!("Middleware error: {e}").into_bytes(), request_id)
            }
        }
    }
}

#[async_trait::async_trait]
impl DMSModule for DMSGateway {
    /// Returns the name of the gateway module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMS.Gateway"
    }

    /// Initializes the gateway module.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: Service context containing configuration and other services
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSServiceContext) -> crate::core::DMSResult<()> {
        let logger = ctx._Flogger();
        logger._Finfo("DMS.Gateway", "Initializing API gateway module")?;

        let config = self.config.read().await;
        logger._Finfo(
            "DMS.Gateway",
            format!("Gateway will listen on {}:{}", config.listen_address, config.listen_port)
        )?;

        logger._Finfo("DMS.Gateway", "API gateway module initialized successfully")?;
        Ok(())
    }

    /// Performs cleanup after the gateway has shut down.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: Service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> crate::core::DMSResult<()> {
        // Cleanup gateway resources
        Ok(())
    }
}