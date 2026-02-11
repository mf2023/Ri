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

#![allow(non_snake_case)]

//! # Gateway Module
//! 
//! This module provides a comprehensive API gateway functionality for DMSC, offering routing, middleware support,
//! load balancing, rate limiting, and circuit breaking capabilities.
//! 
//! ## Key Components
//! 
//! - **DMSCGateway**: Main gateway struct implementing the DMSCModule trait
//! - **DMSCGatewayConfig**: Configuration for gateway behavior
//! - **DMSCGatewayRequest**: Request structure for gateway operations
//! - **DMSCGatewayResponse**: Response structure for gateway operations
//! - **DMSCRoute**: Route definition for API endpoints
//! - **DMSCRouter**: Router for handling request routing
//! - **DMSCMiddleware**: Middleware interface for request processing
//! - **DMSCMiddlewareChain**: Chain of middleware for sequential execution
//! - **DMSCLoadBalancer**: Load balancing for distributing requests across multiple services
//! - **DMSCLoadBalancerStrategy**: Load balancing strategies (RoundRobin, LeastConnections, etc.)
//! - **DMSCRateLimiter**: Rate limiting for controlling request rates
//! - **DMSCRateLimitConfig**: Configuration for rate limiting
//! - **DMSCCircuitBreaker**: Circuit breaker for preventing cascading failures
//! - **DMSCCircuitBreakerConfig**: Configuration for circuit breakers
//! 
//! ## Design Principles
//! 
//! 1. **Modular Design**: Separate components for routing, middleware, load balancing, rate limiting, and circuit breaking
//! 2. **Async-First**: All gateway operations are asynchronous
//! 3. **Configurable**: Highly configurable gateway behavior through DMSCGatewayConfig
//! 4. **Middleware Support**: Extensible middleware system for request processing
//! 5. **Resilience**: Built-in circuit breaker and rate limiting for service resilience
//! 6. **Load Balancing**: Support for distributing requests across multiple service instances
//! 7. **CORS Support**: Built-in CORS configuration for cross-origin requests
//! 8. **Logging**: Comprehensive logging support
//! 9. **Service Integration**: Implements DMSCModule trait for seamless integration into DMSC
//! 10. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! use dmsc::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCRoute};
//! use std::collections::HashMap;
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create gateway configuration
//!     let gateway_config = DMSCGatewayConfig {
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
//!     let gateway = DMSCGateway::new();
//!     
//!     // Get router and add routes
//!     let router = gateway.router();
//!     
//!     // Add a simple GET route
//!     router.add_route(DMSCRoute {
//!         path: "/api/v1/health".to_string(),
//!         method: "GET".to_string(),
//!         handler: Arc::new(|req| Box::pin(async move {
//!             Ok(DMSCGatewayResponse::json(200, &serde_json::json!({ "status": "ok" }), req.id.clone())?)
//!         })),
//!         ..Default::default()
//!     }).await?;
//!     
//!     // Add middleware
//!     let middleware_chain = gateway.middleware_chain();
//!     middleware_chain.add_middleware(Arc::new(|req, next| Box::pin(async move {
//!         // Log request
//!         println!("Request: {} {}", req.method, req.path);
//!         next(req).await
//!     }))).await;
//!     
//!     // Handle a sample request
//!     let sample_request = DMSCGatewayRequest::new(
//!         "GET".to_string(),
//!         "/api/v1/health".to_string(),
//!         HashMap::new(),
//!         HashMap::new(),
//!         None,
//!         "127.0.0.1:12345".to_string(),
//!     );
//!     
//!     let response = gateway.handle_request(sample_request).await;
//!     println!("Response: {} {}", response.status_code, String::from_utf8_lossy(&response.body));
//!     
//!     Ok(())
//! }
//! ```

use crate::core::{DMSCModule, DMSCServiceContext};
use log;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub mod middleware;
pub mod routing;
pub mod circuit_breaker;
pub mod load_balancer;
pub mod rate_limiter;
pub mod server;

pub use routing::{DMSCRoute, DMSCRouter};
pub use middleware::{DMSCMiddleware, DMSCMiddlewareChain};
pub use load_balancer::{DMSCLoadBalancer, DMSCLoadBalancerStrategy};
pub use rate_limiter::{DMSCRateLimiter, DMSCRateLimitConfig};
pub use circuit_breaker::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig};

#[cfg(feature = "gateway")]
pub use server::{DMSCGatewayServer, load_tls_config};

/// Configuration for the DMSC Gateway.
/// 
/// This struct defines the configuration options for the API gateway, including network settings,
/// feature toggles, and CORS configuration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCGatewayConfig {
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

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCGatewayConfig
#[pyo3::prelude::pymethods]
impl DMSCGatewayConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn py_new_with_address(listen_address: String, listen_port: u16) -> Self {
        Self {
            listen_address,
            listen_port,
            ..Self::default()
        }
    }
}

impl DMSCGatewayConfig {
    /// Creates a new DMSCGatewayConfig with default values.
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for DMSCGatewayConfig {
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGatewayRequest {
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

impl DMSCGatewayRequest {
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
    /// A new `DMSCGatewayRequest` instance
    pub fn new(
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
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGatewayResponse {
    /// HTTP status code
    pub status_code: u16,
    /// HTTP headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Request ID associated with this response
    pub request_id: String,
}

impl DMSCGatewayResponse {
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
    /// A new `DMSCGatewayResponse` instance
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
    /// The updated `DMSCGatewayResponse` instance
    pub fn with_header(mut self, key: String, value: String) -> Self {
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
    /// A `DMSCResult<Self>` containing the JSON response
    pub fn json<T: serde::Serialize>(status_code: u16, data: &T, request_id: String) -> crate::core::DMSCResult<Self> {
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
    /// A new `DMSCGatewayResponse` instance with error information
    pub fn error(status_code: u16, message: String, request_id: String) -> Self {
        let error_body = serde_json::json!({
            "error": message,
            "request_id": request_id
        });
        
        let body = serde_json::to_vec(&error_body).unwrap_or_else(|_| b"{}".to_vec());
        Self::new(status_code, body, request_id)
    }
}

/// Main gateway struct implementing the DMSCModule trait.
/// 
/// This struct provides the core gateway functionality, including request handling,
/// routing, middleware execution, rate limiting, and circuit breaking.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGateway {
    /// Gateway configuration, protected by a RwLock for thread-safe access
    config: RwLock<DMSCGatewayConfig>,
    /// Router for handling request routing
    router: Arc<DMSCRouter>,
    /// Middleware chain for request processing
    middleware_chain: Arc<DMSCMiddlewareChain>,
    /// Rate limiter for controlling request rates
    rate_limiter: Option<Arc<DMSCRateLimiter>>,
    /// Circuit breaker for preventing cascading failures
    circuit_breaker: Option<Arc<DMSCCircuitBreaker>>,
}

impl Default for DMSCGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCGateway {
    /// Creates a new gateway instance with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSCGateway` instance
    pub fn new() -> Self {
        let config = DMSCGatewayConfig::default();
        let router = Arc::new(DMSCRouter::new());
        let middleware_chain = Arc::new(DMSCMiddlewareChain::new());
        
        let rate_limiter = if config.enable_rate_limiting {
            Some(Arc::new(DMSCRateLimiter::new(DMSCRateLimitConfig::default())))
        } else {
            None
        };
        
        let circuit_breaker = if config.enable_circuit_breaker {
            Some(Arc::new(DMSCCircuitBreaker::new(DMSCCircuitBreakerConfig::default())))
        } else {
            None
        };

        Self {
            config: RwLock::new(config),
            router,
            middleware_chain,
            rate_limiter,
            circuit_breaker,
        }
    }

    /// Returns a reference to the router.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSCRouter> providing thread-safe access to the router
    pub fn router(&self) -> Arc<DMSCRouter> {
        self.router.clone()
    }

    /// Returns a reference to the middleware chain.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSCMiddlewareChain> providing thread-safe access to the middleware chain
    pub fn middleware_chain(&self) -> Arc<DMSCMiddlewareChain> {
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
    /// A `DMSCGatewayResponse` containing the response to the request
    pub async fn handle_request(&self, request: DMSCGatewayRequest) -> DMSCGatewayResponse {
        let request_id = request.id.clone();
        
        // Apply rate limiting
        if let Some(rate_limiter) = &self.rate_limiter {
            if !rate_limiter.check_request(&request).await {
                return DMSCGatewayResponse::new(429, "Rate limit exceeded".to_string().into_bytes(), request_id);
            }
        }

        // Apply circuit breaker
        if let Some(circuit_breaker) = &self.circuit_breaker {
            if !circuit_breaker.allow_request() {
                return DMSCGatewayResponse::new(503, "Service temporarily unavailable".to_string().into_bytes(), request_id);
            }
        }

        // Apply middleware chain
        let mut request = request;
        match self.middleware_chain.execute(&mut request).await {
            Ok(()) => {
                // Route the request
                match self.router.route(&request).await {
                    Ok(route_handler) => {
                        // Execute the route handler
                        match route_handler(request).await {
                            Ok(response) => response,
                            Err(e) => {
                                DMSCGatewayResponse::new(500, format!("Internal server error: {e}").into_bytes(), request_id)
                            }
                        }
                    },
                    Err(e) => {
                        DMSCGatewayResponse::new(404, format!("Route not found: {e}").into_bytes(), request_id)
                    }
                }
            },
            Err(e) => {
                DMSCGatewayResponse::new(403, format!("Middleware error: {e}").into_bytes(), request_id)
            }
        }
    }
}

#[async_trait::async_trait]
impl DMSCModule for DMSCGateway {
    /// Returns the name of the gateway module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.Gateway"
    }

    /// Initializes the gateway module.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: Service context containing configuration and other services
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> crate::core::DMSCResult<()> {
        let logger = ctx.logger();
        logger.info("DMSC.Gateway", "Initializing API gateway module")?;

        let config = self.config.read().await;
        logger.info(
            "DMSC.Gateway",
            format!("Gateway will listen on {}:{}", config.listen_address, config.listen_port)
        )?;

        logger.info("DMSC.Gateway", "API gateway module initialized successfully")?;
        Ok(())
    }

    /// Performs cleanup after the gateway has shut down.
    /// 
    /// This method ensures proper resource cleanup during gateway shutdown:
    /// - Clears all rate limiter buckets
    /// - Resets the circuit breaker to closed state
    /// - Clears all registered routes
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: Service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    /// 
    /// # Logs
    /// 
    /// Logs cleanup progress at INFO level for debugging purposes
    async fn after_shutdown(&mut self, _ctx: &mut DMSCServiceContext) -> crate::core::DMSCResult<()> {
        log::info!("Cleaning up DMSC Gateway Module");
        
        if let Some(rate_limiter) = &self.rate_limiter {
            rate_limiter.clear_all_buckets();
            log::info!("Rate limiter cleanup completed");
        }
        
        if let Some(circuit_breaker) = &self.circuit_breaker {
            circuit_breaker.reset();
            log::info!("Circuit breaker reset completed");
        }
        
        self.router.clear_routes();
        log::info!("Router cleanup completed");
        
        log::info!("DMSC Gateway Module cleanup completed");
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCGateway
#[pyo3::prelude::pymethods]
impl DMSCGateway {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
}
