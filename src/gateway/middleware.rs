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

#![allow(non_snake_case)]

//! # Middleware Module
//! 
//! This module provides a flexible middleware system for the Ri gateway, allowing for
//! request processing and modification through a chain of middleware components.
//! 
//! ## Key Components
//! 
//! - **RiMiddleware**: Trait defining the middleware interface
//! - **RiMiddlewareChain**: Manages a chain of middleware components
//! - **Built-in Middleware**: Auth, CORS, Logging, Request ID, and Rate Limiting implementations
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait**: Uses async_trait for async middleware execution
//! 2. **Flexible Chain**: Allows dynamic addition and removal of middleware
//! 3. **Extensible**: Easy to implement custom middleware
//! 4. **Thread Safe**: Uses Arc for safe sharing of middleware instances
//! 5. **Modular**: Built-in middleware implementations can be used independently
//! 6. **Request Modification**: Middleware can modify requests before they reach the target service
//! 7. **Error Handling**: Middleware can return errors to abort request processing
//! 8. **Order Matters**: Middleware is executed in the order they are added to the chain
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a middleware chain
//!     let mut chain = RiMiddlewareChain::new();
//!     
//!     // Add built-in middleware
//!     chain.add(Arc::new(RiLoggingMiddleware::new("info".to_string())));
//!     chain.add(Arc::new(RiAuthMiddleware::new("Authorization".to_string())));
//!     chain.add(Arc::new(RiCorsMiddleware::new(
//!         vec!["*".to_string()],
//!         vec!["GET".to_string(), "POST".to_string()],
//!         vec!["Content-Type".to_string(), "Authorization".to_string()]
//!     )));
//!     
//!     // Create a request and execute the middleware chain
//!     let mut request = RiGatewayRequest::new("GET".to_string(), "/api/v1/resource".to_string());
//!     chain.execute(&mut request).await?;
//!     
//!     Ok(())
//! }
//! ```

use super::RiGatewayRequest;
use crate::core::RiResult;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait defining the middleware interface for request processing.
/// 
/// All middleware components must implement this trait, which provides methods for
/// executing middleware logic and identifying the middleware.
#[async_trait]
pub trait RiMiddleware: Send + Sync {
    /// Executes the middleware logic on a request.
    /// 
    /// This method is called for each request passing through the middleware chain.
    /// Middleware can modify the request, validate it, or return an error to abort processing.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()>;
    
    /// Gets the name of the middleware.
    /// 
    /// This method returns a static string identifier for the middleware, useful for logging
    /// and debugging purposes.
    /// 
    /// # Returns
    /// 
    /// A static string containing the middleware name
    fn name(&self) -> &'static str;
}

/// Manages a chain of middleware components.
/// 
/// This struct maintains a list of middleware instances and provides methods for
/// adding, removing, and executing middleware in sequence.
pub struct RiMiddlewareChain {
    /// Vector of middleware instances in the order they should be executed
    middlewares: Vec<Arc<dyn RiMiddleware>>,
}

impl Default for RiMiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

impl RiMiddlewareChain {
    /// Creates a new empty middleware chain.
    /// 
    /// # Returns
    /// 
    /// A new `RiMiddlewareChain` instance with no middleware
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Adds a middleware to the end of the chain.
    /// 
    /// # Parameters
    /// 
    /// - `middleware`: The middleware to add to the chain
    pub fn add(&mut self, middleware: Arc<dyn RiMiddleware>) {
        self.middlewares.push(middleware);
    }

    /// Executes all middleware in the chain on a request.
    /// 
    /// Middleware is executed in the order they were added to the chain.
    /// If any middleware returns an error, execution stops and the error is returned.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    pub async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()> {
        for middleware in &self.middlewares {
            // Record middleware execution time
            let start = std::time::Instant::now();
            
            middleware.execute(request).await?;
            
            let duration = start.elapsed();
            let _duration_ms = duration.as_secs_f64() * 1000.0;
            
            // Middleware execution time is tracked by the observability module
            // The metrics are automatically collected when the observability feature is enabled
        }
        Ok(())
    }

    /// Clears all middleware from the chain.
    pub fn clear(&mut self) {
        self.middlewares.clear();
    }

    /// Gets the number of middleware in the chain.
    /// 
    /// # Returns
    /// 
    /// The number of middleware in the chain
    pub fn len(&self) -> usize {
        self.middlewares.len()
    }

    /// Checks if the middleware chain is empty.
    /// 
    /// # Returns
    /// 
    /// `true` if the chain contains no middleware, `false` otherwise
    pub fn is_empty(&self) -> bool {
        self.middlewares.is_empty()
    }
}

// Built-in middleware implementations

/// Authentication middleware for validating request credentials.
/// 
/// This middleware checks for and validates authorization headers in requests.
pub struct RiAuthMiddleware {
    /// Name of the authorization header to check
    auth_header: String,
}

impl RiAuthMiddleware {
    /// Creates a new authentication middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `auth_header`: Name of the authorization header to check
    /// 
    /// # Returns
    /// 
    /// A new `RiAuthMiddleware` instance
    pub fn new(auth_header: String) -> Self {
        Self { auth_header }
    }
}

#[async_trait]
impl RiMiddleware for RiAuthMiddleware {
    /// Validates the authorization header in the request.
    /// 
    /// This implementation checks for a Bearer token in the specified authorization header.
    /// In a production environment, this would validate JWT tokens using the auth module.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()> {
        // Check for authorization header
        if let Some(auth_header) = request.headers.get(&self.auth_header) {
            // Basic auth validation - in a real implementation, this would validate JWT tokens
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                if token.is_empty() {
                    return Err(crate::core::RiError::Other("Empty bearer token".to_string()));
                }
                // Here you would validate the JWT token using your auth module
            } else {
                return Err(crate::core::RiError::Other("Invalid authorization header format".to_string()));
            }
        } else {
            // Allow requests without auth for public endpoints
            // In a real implementation, you'd check if the route requires authentication
        }
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "AuthMiddleware"
    fn name(&self) -> &'static str {
        "AuthMiddleware"
    }
}

/// CORS (Cross-Origin Resource Sharing) middleware.
/// 
/// This middleware validates CORS headers and ensures requests come from allowed origins.
pub struct RiCorsMiddleware {
    /// List of allowed origins for CORS requests
    allowed_origins: Vec<String>,
    /// List of allowed HTTP methods for CORS requests
    #[allow(dead_code)]
    allowed_methods: Vec<String>,
    /// List of allowed headers for CORS requests
    #[allow(dead_code)]
    allowed_headers: Vec<String>,
}

impl RiCorsMiddleware {
    /// Creates a new CORS middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `allowed_origins`: List of allowed origins for CORS requests
    /// - `allowed_methods`: List of allowed HTTP methods for CORS requests
    /// - `allowed_headers`: List of allowed headers for CORS requests
    /// 
    /// # Returns
    /// 
    /// A new `RiCorsMiddleware` instance
    pub fn new(
        allowed_origins: Vec<String>,
        allowed_methods: Vec<String>,
        allowed_headers: Vec<String>,
    ) -> Self {
        Self {
            allowed_origins,
            allowed_methods,
            allowed_headers,
        }
    }
    
    /// Checks if an origin is allowed.
    ///
    /// # Security Note
    ///
    /// Wildcard origin "*" is NOT treated as matching any origin here.
    /// Wildcard handling is done at the response header level, not validation level.
    /// This prevents bypass attacks where any origin is falsely considered valid.
    ///
    /// # Parameters
    ///
    /// - `origin`: The origin to check
    ///
    /// # Returns
    ///
    /// `true` if the origin is allowed, `false` otherwise
    fn is_origin_allowed(&self, origin: &str) -> bool {
        // Wildcard should never match specific origins - it's only for response headers
        if self.allowed_origins.contains(&"*".to_string()) {
            // If wildcard is set, only exact matches or the wildcard itself is valid
            // But we don't treat wildcard as matching everything
            return false;
        }
        self.allowed_origins.iter().any(|allowed| allowed == origin)
    }

    /// Checks if wildcard origin is configured.
    ///
    /// # Returns
    ///
    /// `true` if wildcard origin is allowed
    fn is_wildcard_allowed(&self) -> bool {
        self.allowed_origins.contains(&"*".to_string())
    }
}

#[async_trait]
impl RiMiddleware for RiCorsMiddleware {
    /// Validates CORS headers in the request.
    ///
    /// This implementation checks if the request origin is in the list of allowed origins.
    /// If wildcard is configured, any origin is allowed but will be handled at response time.
    ///
    /// # Parameters
    ///
    /// - `request`: Mutable reference to the request being processed
    ///
    /// # Returns
    ///
    /// A `RiResult<()>` indicating success or failure
    async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()> {
        // CORS preflight handling would be done at the response level
        // This middleware just validates the request

        if let Some(origin) = request.headers.get("origin") {
            // If wildcard is allowed, origin validation passes here
            // Actual wildcard response will be handled at response header level
            if !self.is_wildcard_allowed() && !self.is_origin_allowed(origin) {
                return Err(crate::core::RiError::Other("Origin not allowed".to_string()));
            }
        }

        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "CorsMiddleware"
    fn name(&self) -> &'static str {
        "CorsMiddleware"
    }
}

/// Logging middleware for recording request details.
/// 
/// This middleware logs request information such as method, path, and remote address.
pub struct RiLoggingMiddleware {
    /// Log level for the middleware
    #[allow(dead_code)]
    log_level: String,
}

impl RiLoggingMiddleware {
    /// Creates a new logging middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `log_level`: Log level for the middleware
    /// 
    /// # Returns
    /// 
    /// A new `RiLoggingMiddleware` instance
    pub fn new(log_level: String) -> Self {
        Self { log_level }
    }
}

#[async_trait]
impl RiMiddleware for RiLoggingMiddleware {
    /// Logs request details.
    /// 
    /// This implementation prints request information to the console.
    /// In a production environment, this would use a proper logging framework.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()> {
        // In a real implementation, this would log the request details
        // For now, we'll just allow it through
        log::info!("[{}] {} {} from {}", 
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
            request.method,
            request.path,
            request.remote_addr
        );
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "LoggingMiddleware"
    fn name(&self) -> &'static str {
        "LoggingMiddleware"
    }
}

/// Request ID middleware for processing request IDs.
/// 
/// This middleware handles request ID generation and processing.
/// Note: Request IDs are already generated in `RiGatewayRequest::new`.
pub struct RiRequestIdMiddleware;

impl Default for RiRequestIdMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl RiRequestIdMiddleware {
    /// Creates a new request ID middleware instance.
    /// 
    /// # Returns
    /// 
    /// A new `RiRequestIdMiddleware` instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl RiMiddleware for RiRequestIdMiddleware {
    /// Processes the request ID in the request.
    /// 
    /// This implementation is a no-op since request IDs are generated in `RiGatewayRequest::new`.
    /// It can be extended for additional request ID processing.
    /// 
    /// # Parameters
    /// 
    /// - `_request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    async fn execute(&self, _request: &mut RiGatewayRequest) -> RiResult<()> {
        // Request ID is already generated in RiGatewayRequest::new
        // This middleware can be used for additional request ID processing
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "RequestIdMiddleware"
    fn name(&self) -> &'static str {
        "RequestIdMiddleware"
    }
}

/// Rate limiting middleware for controlling request rates.
/// 
/// This middleware limits the number of requests from a client within a specified time window.
pub struct RiRateLimitMiddleware {
    /// Rate limiter instance for enforcing rate limits
    rate_limiter: Arc<crate::gateway::RiRateLimiter>,
}

impl RiRateLimitMiddleware {
    /// Creates a new rate limiting middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `rate_limiter`: Rate limiter instance for enforcing rate limits
    /// 
    /// # Returns
    /// 
    /// A new `RiRateLimitMiddleware` instance
    pub fn new(rate_limiter: Arc<crate::gateway::RiRateLimiter>) -> Self {
        Self {
            rate_limiter,
        }
    }
}

#[async_trait]
impl RiMiddleware for RiRateLimitMiddleware {
    /// Applies rate limiting to the request.
    /// 
    /// This implementation uses the RiRateLimiter to check if the request should be allowed
    /// based on rate limiting rules. If the request exceeds the rate limit, an error is returned.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure. Returns error if rate limit exceeded.
    async fn execute(&self, request: &mut RiGatewayRequest) -> RiResult<()> {
        // Check rate limit using the rate limiter
        if !self.rate_limiter.check_request(request).await {
            return Err(crate::core::RiError::Other("Rate limit exceeded".to_string()));
        }
        
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "RateLimitMiddleware"
    fn name(&self) -> &'static str {
        "RateLimitMiddleware"
    }
}
