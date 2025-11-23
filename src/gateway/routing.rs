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

use super::{DMSGatewayRequest, DMSGatewayResponse};
use crate::core::DMSResult;
use crate::gateway::middleware::DMSMiddleware;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type DMSRouteHandler = Arc<
    dyn Fn(DMSGatewayRequest) -> Pin<Box<dyn Future<Output = DMSResult<DMSGatewayResponse>> + Send>>
        + Send
        + Sync,
>;

#[derive(Clone)]
pub struct DMSRoute {
    pub method: String,
    pub path: String,
    pub handler: DMSRouteHandler,
    pub middleware: Vec<Arc<dyn DMSMiddleware>>,
}

impl DMSRoute {
    pub fn _Fnew(
        method: String,
        path: String,
        handler: DMSRouteHandler,
    ) -> Self {
        Self {
            method,
            path,
            handler,
            middleware: Vec::new(),
        }
    }

    pub fn _Fwith_middleware(mut self, middleware: Arc<dyn DMSMiddleware>) -> Self {
        self.middleware.push(middleware);
        self
    }
}

pub struct DMSRouter {
    routes: std::sync::RwLock<Vec<DMSRoute>>,
    route_cache: std::sync::RwLock<HashMap<String, DMSRoute>>,
}

impl DMSRouter {
    pub fn _Fnew() -> Self {
        Self {
            routes: std::sync::RwLock::new(Vec::new()),
            route_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    pub fn _Fadd_route(&self, route: DMSRoute) {
        let mut routes = self.routes.write().unwrap();
        routes.push(route);
        
        // Clear cache when routes are modified
        let mut cache = self.route_cache.write().unwrap();
        cache.clear();
    }

    pub fn _Fget(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("GET".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub fn _Fpost(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("POST".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub fn _Fput(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("PUT".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub fn _Fdelete(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("DELETE".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub fn _Fpatch(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("PATCH".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub fn _Foptions(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::_Fnew("OPTIONS".to_string(), path.to_string(), handler);
        self._Fadd_route(route);
    }

    pub async fn _Froute(&self, request: &DMSGatewayRequest) -> DMSResult<DMSRouteHandler> {
        let cache_key = format!("{}:{}", request.method, request.path);
        
        // Check cache first
        {
            let cache = self.route_cache.read().unwrap();
            if let Some(cached_route) = cache.get(&cache_key) {
                return Ok(cached_route.handler.clone());
            }
        }

        // Find matching route
        let routes = self.routes.read().unwrap();
        for route in routes.iter() {
            if self._Fmatches_route(&route.method, &route.path, &request.method, &request.path) {
                // Cache the result
                let mut cache = self.route_cache.write().unwrap();
                cache.insert(cache_key.clone(), route.clone());
                
                return Ok(route.handler.clone());
            }
        }

        Err(crate::core::DMSError::Other(format!(
            "No route found for {} {}",
            request.method, request.path
        )))
    }

    fn _Fmatches_route(&self, route_method: &str, route_path: &str, request_method: &str, request_path: &str) -> bool {
        // Check method
        if route_method != request_method {
            return false;
        }

        // Simple path matching (can be enhanced with proper path parameters)
        if route_path == request_path {
            return true;
        }

        // Handle wildcards
        if route_path == "*" {
            return true;
        }

        // Handle path parameters (basic implementation)
        if route_path.contains(':') {
            let route_parts: Vec<&str> = route_path.split('/').collect();
            let request_parts: Vec<&str> = request_path.split('/').collect();

            if route_parts.len() != request_parts.len() {
                return false;
            }

            for (route_part, request_part) in route_parts.iter().zip(request_parts.iter()) {
                if !route_part.starts_with(':') && route_part != request_part {
                    return false;
                }
            }

            return true;
        }

        false
    }

    pub fn _Fmount(&self, prefix: &str, router: &DMSRouter) {
        let routes = router.routes.read().unwrap();
        for route in routes.iter() {
            let mounted_path = if prefix.ends_with('/') && route.path.starts_with('/') {
                format!("{}{}", prefix, &route.path[1..])
            } else if !prefix.ends_with('/') && !route.path.starts_with('/') {
                format!("{}/{}", prefix, route.path)
            } else {
                format!("{}{}", prefix, route.path)
            };

            let mut mounted_route = route.clone();
            mounted_route.path = mounted_path;
            self._Fadd_route(mounted_route);
        }
    }

    pub fn _Fclear_routes(&self) {
        let mut routes = self.routes.write().unwrap();
        let mut cache = self.route_cache.write().unwrap();
        routes.clear();
        cache.clear();
    }

    pub fn _Froute_count(&self) -> usize {
        self.routes.read().unwrap().len()
    }
}