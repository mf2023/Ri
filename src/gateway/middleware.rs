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

use super::DMSGatewayRequest;
use crate::core::DMSResult;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait DMSMiddleware: Send + Sync {
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()>;
    fn _Fname(&self) -> &'static str;
}

pub struct DMSMiddlewareChain {
    middlewares: Vec<Arc<dyn DMSMiddleware>>,
}

impl DMSMiddlewareChain {
    pub fn _Fnew() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    pub fn _Fadd(&mut self, middleware: Arc<dyn DMSMiddleware>) {
        self.middlewares.push(middleware);
    }

    pub async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        for middleware in &self.middlewares {
            middleware._Fexecute(request).await?;
        }
        Ok(())
    }

    pub fn _Fclear(&mut self) {
        self.middlewares.clear();
    }

    pub fn _Flen(&self) -> usize {
        self.middlewares.len()
    }
}

// Built-in middleware implementations

pub struct DMSAuthMiddleware {
    auth_header: String,
}

impl DMSAuthMiddleware {
    pub fn _Fnew(auth_header: String) -> Self {
        Self { auth_header }
    }
}

#[async_trait]
impl DMSMiddleware for DMSAuthMiddleware {
    async fn _Fexecute(&self, request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Check for authorization header
        if let Some(auth_header) = request.headers.get(&self.auth_header) {
            // Basic auth validation - in a real implementation, this would validate JWT tokens
            if auth_header.starts_with("Bearer ") {
                let token = &auth_header[7..];
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

    fn _Fname(&self) -> &'static str {
        "AuthMiddleware"
    }
}

pub struct DMSCorsMiddleware {
    allowed_origins: Vec<String>,
    #[allow(dead_code)]
    allowed_methods: Vec<String>,
    #[allow(dead_code)]
    allowed_headers: Vec<String>,
}

impl DMSCorsMiddleware {
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
}

#[async_trait]
impl DMSMiddleware for DMSCorsMiddleware {
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

    fn _Fname(&self) -> &'static str {
        "CorsMiddleware"
    }
}

impl DMSCorsMiddleware {
    fn _Fis_origin_allowed(&self, origin: &str) -> bool {
        self.allowed_origins.contains(&"*".to_string()) || 
        self.allowed_origins.iter().any(|allowed| allowed == origin)
    }
}

pub struct DMSLoggingMiddleware {
    #[allow(dead_code)]
    log_level: String,
}

impl DMSLoggingMiddleware {
    pub fn _Fnew(log_level: String) -> Self {
        Self { log_level }
    }
}

#[async_trait]
impl DMSMiddleware for DMSLoggingMiddleware {
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

    fn _Fname(&self) -> &'static str {
        "LoggingMiddleware"
    }
}

pub struct DMSRequestIdMiddleware;

impl DMSRequestIdMiddleware {
    pub fn _Fnew() -> Self {
        Self
    }
}

#[async_trait]
impl DMSMiddleware for DMSRequestIdMiddleware {
    async fn _Fexecute(&self, _request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Request ID is already generated in DMSGatewayRequest::new
        // This middleware can be used for additional request ID processing
        Ok(())
    }

    fn _Fname(&self) -> &'static str {
        "RequestIdMiddleware"
    }
}

pub struct DMSRateLimitMiddleware {
    #[allow(dead_code)]
    max_requests: u32,
    #[allow(dead_code)]
    window_seconds: u64,
}

impl DMSRateLimitMiddleware {
    pub fn _Fnew(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_seconds,
        }
    }
}

#[async_trait]
impl DMSMiddleware for DMSRateLimitMiddleware {
    async fn _Fexecute(&self, _request: &mut DMSGatewayRequest) -> DMSResult<()> {
        // Basic rate limiting logic - in a real implementation, this would use the DMSRateLimiter
        // For now, we'll just allow it through
        Ok(())
    }

    fn _Fname(&self) -> &'static str {
        "RateLimitMiddleware"
    }
}