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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSGatewayConfig {
    pub listen_address: String,
    pub listen_port: u16,
    pub max_connections: usize,
    pub request_timeout_seconds: u64,
    pub enable_rate_limiting: bool,
    pub enable_circuit_breaker: bool,
    pub enable_load_balancing: bool,
    pub cors_enabled: bool,
    pub cors_origins: Vec<String>,
    pub cors_methods: Vec<String>,
    pub cors_headers: Vec<String>,
    pub enable_logging: bool,
    pub log_level: String,
}

impl Default for DMSGatewayConfig {
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

#[derive(Debug, Clone)]
pub struct DMSGatewayRequest {
    pub id: String,
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub remote_addr: String,
    pub timestamp: std::time::Instant,
}

impl DMSGatewayRequest {
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

#[derive(Debug, Clone)]
pub struct DMSGatewayResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub request_id: String,
}

impl DMSGatewayResponse {
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

    pub fn _Fwith_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    pub fn _Fjson<T: serde::Serialize>(status_code: u16, data: &T, request_id: String) -> crate::core::DMSResult<Self> {
        let body = serde_json::to_vec(data)?;
        Ok(Self::new(status_code, body, request_id))
    }

    pub fn _Ferror(status_code: u16, message: String, request_id: String) -> Self {
        let error_body = serde_json::json!({
            "error": message,
            "request_id": request_id
        });
        
        let body = serde_json::to_vec(&error_body).unwrap_or_else(|_| b"{}".to_vec());
        Self::new(status_code, body, request_id)
    }
}

pub struct DMSGateway {
    config: RwLock<DMSGatewayConfig>,
    router: Arc<DMSRouter>,
    middleware_chain: Arc<DMSMiddlewareChain>,
    rate_limiter: Option<Arc<DMSRateLimiter>>,
    circuit_breaker: Option<Arc<DMSCircuitBreaker>>,
    #[allow(dead_code)]
    load_balancer: Option<Arc<DMSLoadBalancer>>,
}

impl DMSGateway {
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

    pub fn _Frouter(&self) -> Arc<DMSRouter> {
        self.router.clone()
    }

    pub fn _Fmiddleware_chain(&self) -> Arc<DMSMiddlewareChain> {
        self.middleware_chain.clone()
    }

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
                                DMSGatewayResponse::new(500, format!("Internal server error: {}", e).into_bytes(), request_id)
                            }
                        }
                    },
                    Err(e) => {
                        DMSGatewayResponse::new(404, format!("Route not found: {}", e).into_bytes(), request_id)
                    }
                }
            },
            Err(e) => {
                DMSGatewayResponse::new(403, format!("Middleware error: {}", e).into_bytes(), request_id)
            }
        }
    }
}

#[async_trait::async_trait]
impl DMSModule for DMSGateway {
    fn name(&self) -> &str {
        "DMS.Gateway"
    }

    async fn init(&mut self, ctx: &mut DMSServiceContext) -> crate::core::DMSResult<()> {
        let logger = ctx._Flogger();
        logger._Finfo("DMS.Gateway", "Initializing API gateway module")?;

        let config = self.config.read().await;
        logger._Finfo(
            "DMS.Gateway",
            &format!("Gateway will listen on {}:{}", config.listen_address, config.listen_port)
        )?;

        logger._Finfo("DMS.Gateway", "API gateway module initialized successfully")?;
        Ok(())
    }

    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> crate::core::DMSResult<()> {
        // Cleanup gateway resources
        Ok(())
    }


}