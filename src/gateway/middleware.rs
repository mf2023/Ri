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

//! # Middleware Module
//! 
//! This module provides a flexible middleware system for the DMS gateway, allowing for
//! request processing and modification through a chain of middleware components.
//! 
//! ## Key Components
//! 
//! - **DMSMiddleware**: Trait defining the middleware interface
//! - **DMSMiddlewareChain**: Manages a chain of middleware components
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
//! use dms::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a middleware chain
//!     let mut chain = DMSMiddlewareChain::_Fnew();
//!     
//!     // Add built-in middleware
//!     chain._Fadd(Arc::new(DMSLoggingMiddleware::_Fnew("info".to_string())));
//!     chain._Fadd(Arc::new(DMSAuthMiddleware::_Fnew("Authorization".to_string())));
//!     chain._Fadd(Arc::new(DMSCorsMiddleware::_Fnew(
//!         vec!["*".to_string()],
//!         vec!["GET".to_string(), "POST".to_string()],
//!         vec!["Content-Type".to_string(), "Authorization".to_string()]
//!     )));
//!     
//!     // Create a request and execute the middleware chain
//!     let mut request = DMSGatewayRequest::_Fnew("GET".to_string(), "/api/v1/resource".to_string());
//!     chain._Fexecute(&mut request).await?;
//!     
//!     Ok(())
//! }
//! ```

use super::DMSGatewayRequest;
use crate::core::DMSResult;
use async_trait::async_trait;
use std::sync::Arc;

/// Trait defining the middleware interface for request processing.
/// 
/// All middleware components must implement this trait, which provides methods for
/// executing middleware logic and identifying the middleware.
#[async_trait]
pub trait DMSMiddleware: Send + Sync {
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()>;
    
    /// Gets the name of the middleware.
    /// 
    /// This method returns a static string identifier for the middleware, useful for logging
    /// and debugging purposes.
    /// 
    /// # Returns
    /// 
    /// A static string containing the middleware name
    fn _Fname(&self) -> &'static str;
}

/// Manages a chain of middleware components.
/// 
/// This struct maintains a list of middleware instances and provides methods for
/// adding, removing, and executing middleware in sequence.
pub struct DMSMiddlewareChain {
    /// Vector of middleware instances in the order they should be executed
    middlewares: Vec<Arc<dyn DMSMiddleware>>,
}

impl DMSMiddlewareChain {
    /// Creates a new empty middleware chain.
    /// 
    /// # Returns
    /// 
    /// A new `DMSMiddlewareChain` instance with no middleware
    pub fn _Fnew() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// Adds a middleware to the end of the chain.
    /// 
    /// # Parameters
    /// 
    /// - `middleware`: The middleware to add to the chain
    pub fn _Fadd(&mut self, middleware: Arc<dyn DMSMiddleware>) {
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
    /// A `DMSResult<()>` indicating success or failure
    pub async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        for middleware in &self.middlewares {
            middleware._Fexecute(request).await?;
        }
        Ok(())
    }

    /// Clears all middleware from the chain.
    pub fn _Fclear(&mut self) {
        self.middlewares.clear();
    }

    /// Gets the number of middleware in the chain.
    /// 
    /// # Returns
    /// 
    /// The number of middleware in the chain
    pub fn _Flen(&self) -> usize {
        self.middlewares.len()
    }
}

// Built-in middleware implementations

/// Authentication middleware for validating request credentials.
/// 
/// This middleware checks for and validates authorization headers in requests.
pub struct DMSAuthMiddleware {
    /// Name of the authorization header to check
    auth_header: String,
}

impl DMSAuthMiddleware {
    /// Creates a new authentication middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `auth_header`: Name of the authorization header to check
    /// 
    /// # Returns
    /// 
    /// A new `DMSAuthMiddleware` instance
    pub fn _Fnew(auth_header: String) -> Self {
        Self { auth_header }
    }
}

#[async_trait]
impl DMSMiddleware for DMSAuthMiddleware {
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Check for authorization header
        if let Some(auth_header) = request.headers.get(&self.auth_header) {
            // Basic auth validation - in a real implementation, this would validate JWT tokens
            if let Some(token) = auth_header.strip_prefix("Bearer ") {
                if token.is_empty() {
                    return Err(crate::core::DMSError::Other("Empty bearer token".to_string()));
                }
                // Here you would validate the JWT token using your auth module
            } else {
                return Err(crate::core::DMSError::Other("Invalid authorization header format".to_string()));
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
    fn _Fname(&self) -> &'static str {
        "AuthMiddleware"
    }
}

/// CORS (Cross-Origin Resource Sharing) middleware.
/// 
/// This middleware validates CORS headers and ensures requests come from allowed origins.
pub struct DMSCorsMiddleware {
    /// List of allowed origins for CORS requests
    allowed_origins: Vec<String>,
    /// List of allowed HTTP methods for CORS requests
    #[allow(dead_code)]
    allowed_methods: Vec<String>,
    /// List of allowed headers for CORS requests
    #[allow(dead_code)]
    allowed_headers: Vec<String>,
}

impl DMSCorsMiddleware {
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
    /// A new `DMSCorsMiddleware` instance
    pub fn _Fnew(
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
    /// # Parameters
    /// 
    /// - `origin`: The origin to check
    /// 
    /// # Returns
    /// 
    /// `true` if the origin is allowed, `false` otherwise
    fn _Fis_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) || 
        self.allowed_origins.iter().any(|allowed| allowed == origin)
    }
}

#[async_trait]
impl DMSMiddleware for DMSCorsMiddleware {
    /// Validates CORS headers in the request.
    /// 
    /// This implementation checks if the request origin is in the list of allowed origins.
    /// 
    /// # Parameters
    /// 
    /// - `request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // CORS preflight handling would be done at the response level
        // This middleware just validates the request
        
        if let Some(origin) = request.headers.get("origin") {
            if !self._Fis_origin_allowed(origin) {
                return Err(crate::core::DMSError::Other("Origin not allowed".to_string()));
            }
        }
        
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "CorsMiddleware"
    fn _Fname(&self) -> &'static str {
        "CorsMiddleware"
    }
}

/// Logging middleware for recording request details.
/// 
/// This middleware logs request information such as method, path, and remote address.
pub struct DMSLoggingMiddleware {
    /// Log level for the middleware
    #[allow(dead_code)]
    log_level: String,
}

impl DMSLoggingMiddleware {
    /// Creates a new logging middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `log_level`: Log level for the middleware
    /// 
    /// # Returns
    /// 
    /// A new `DMSLoggingMiddleware` instance
    pub fn _Fnew(log_level: String) -> Self {
        Self { log_level }
    }
}

#[async_trait]
impl DMSMiddleware for DMSLoggingMiddleware {
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
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // In a real implementation, this would log the request details
        // For now, we'll just allow it through
        println!("[{}] {} {} from {}", 
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
    fn _Fname(&self) -> &'static str {
        "LoggingMiddleware"
    }
}

/// Request ID middleware for processing request IDs.
/// 
/// This middleware handles request ID generation and processing.
/// Note: Request IDs are already generated in `DMSGatewayRequest::new`.
pub struct DMSRequestIdMiddleware;

impl DMSRequestIdMiddleware {
    /// Creates a new request ID middleware instance.
    /// 
    /// # Returns
    /// 
    /// A new `DMSRequestIdMiddleware` instance
    pub fn _Fnew() -> Self {
        Self
    }
}

#[async_trait]
impl DMSMiddleware for DMSRequestIdMiddleware {
    /// Processes the request ID in the request.
    /// 
    /// This implementation is a no-op since request IDs are generated in `DMSGatewayRequest::new`.
    /// It can be extended for additional request ID processing.
    /// 
    /// # Parameters
    /// 
    /// - `_request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, _request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Request ID is already generated in DMSGatewayRequest::new
        // This middleware can be used for additional request ID processing
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "RequestIdMiddleware"
    fn _Fname(&self) -> &'static str {
        "RequestIdMiddleware"
    }
}

/// Rate limiting middleware for controlling request rates.
/// 
/// This middleware limits the number of requests from a client within a specified time window.
pub struct DMSRateLimitMiddleware {
    /// Maximum number of requests allowed within the time window
    #[allow(dead_code)]
    max_requests: u32,
    /// Time window in seconds for rate limiting
    #[allow(dead_code)]
    window_seconds: u64,
}

impl DMSRateLimitMiddleware {
    /// Creates a new rate limiting middleware instance.
    /// 
    /// # Parameters
    /// 
    /// - `max_requests`: Maximum number of requests allowed within the time window
    /// - `window_seconds`: Time window in seconds for rate limiting
    /// 
    /// # Returns
    /// 
    /// A new `DMSRateLimitMiddleware` instance
    pub fn _Fnew(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

#[async_trait]
impl DMSMiddleware for DMSRateLimitMiddleware {
    /// Applies rate limiting to the request.
    /// 
    /// This implementation is a no-op. In a production environment, this would use
    /// the `DMSRateLimiter` to enforce rate limits.
    /// 
    /// # Parameters
    /// 
    /// - `_request`: Mutable reference to the request being processed
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn _Fexecute(&self, _request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Basic rate limiting logic - in a real implementation, this would use the DMSRateLimiter
        // For now, we'll just allow it through
        Ok(())
    }

    /// Gets the name of the middleware.
    /// 
    /// # Returns
    /// 
    /// The string "RateLimitMiddleware"
    fn _Fname(&self) -> &'static str {
        "RateLimitMiddleware"
    }
}