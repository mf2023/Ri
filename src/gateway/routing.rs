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

//! # Routing Module
//! 
//! This module provides a flexible routing system for the DMS gateway, allowing for
//! defining API endpoints and their handlers with support for middleware.
//! 
//! ## Key Components
//! 
//! - **DMSRouteHandler**: Type alias for route handler functions
//! - **DMSRoute**: Represents a single API route with method, path, handler, and middleware
//! - **DMSRouter**: Manages routes, provides route matching, and supports route mounting
//! 
//! ## Design Principles
//! 
//! 1. **Type Safety**: Uses type aliases for clear handler signatures
//! 2. **Middleware Support**: Allows attaching middleware to individual routes
//! 3. **Route Caching**: Caches route matches for improved performance
//! 4. **Flexible Path Matching**: Supports exact paths, wildcards, and path parameters
//! 5. **Method Support**: Supports all HTTP methods (GET, POST, PUT, DELETE, PATCH, OPTIONS)
//! 6. **Route Mounting**: Allows mounting routers with prefixes for modularity
//! 7. **Thread Safe**: Uses RwLock for safe operation in multi-threaded environments
//! 8. **Async Compatibility**: Built with async/await patterns for modern Rust applications
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a router
//!     let router = Arc::new(DMSRouter::new());
//!     
//!     // Create a simple handler
//!     let hello_handler = Arc::new(|req| {
//!         Box::pin(async move {
//!             Ok(DMSGatewayResponse {
//!                 status_code: 200,
//!                 headers: HashMap::new(),
//!                 body: "Hello, DMS!".as_bytes().to_vec(),
//!             })
//!         })
//!     });
//!     
//!     // Add routes
//!     router.get("/hello", hello_handler.clone());
//!     router.post("/api/v1/users", hello_handler.clone());
//!     
//!     // Add route with middleware
//!     let auth_middleware = Arc::new(DMSAuthMiddleware::new("Authorization".to_string()));
//!     let protected_route = DMSRoute::new("GET".to_string(), "/api/v1/protected".to_string(), hello_handler)
//!         .with_middleware(auth_middleware);
//!     router.add_route(protected_route);
//!     
//!     Ok(())
//! }
//! ```

use super::{DMSGatewayRequest, DMSGatewayResponse};
use crate::core::DMSResult;
use crate::gateway::middleware::DMSMiddleware;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Type alias for route handler functions.
/// 
/// This type represents an asynchronous function that takes a gateway request and returns
/// a gateway response. It is wrapped in an Arc to allow safe sharing across threads.
pub type DMSRouteHandler = Arc<
    dyn Fn(DMSGatewayRequest) -> Pin<Box<dyn Future<Output = DMSResult<DMSGatewayResponse>> + Send>>
        + Send
        + Sync,
>;

/// Represents a single API route with method, path, handler, and middleware.
/// 
/// This struct encapsulates all the information needed for a single API endpoint,
/// including the HTTP method, path pattern, request handler, and attached middleware.
#[derive(Clone)]
pub struct DMSRoute {
    /// HTTP method for this route (GET, POST, PUT, DELETE, PATCH, OPTIONS)
    pub method: String,
    /// Path pattern for this route (e.g., "/api/v1/users", "/users/:id")
    pub path: String,
    /// Request handler for this route
    pub handler: DMSRouteHandler,
    /// List of middleware attached to this route
    pub middleware: Vec<Arc<dyn DMSMiddleware>>,
}

impl DMSRoute {
    /// Creates a new route with the specified method, path, and handler.
    /// 
    /// # Parameters
    /// 
    /// - `method`: HTTP method for this route
    /// - `path`: Path pattern for this route
    /// - `handler`: Request handler for this route
    /// 
    /// # Returns
    /// 
    /// A new `DMSRoute` instance with no middleware attached
    pub fn new(
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

    /// Attaches middleware to this route.
    /// 
    /// This method returns a new route instance with the middleware added to the list.
    /// 
    /// # Parameters
    /// 
    /// - `middleware`: Middleware to attach to this route
    /// 
    /// # Returns
    /// 
    /// A new `DMSRoute` instance with the middleware attached
    pub fn with_middleware(mut self, middleware: Arc<dyn DMSMiddleware>) -> Self {
        self.middleware.push(middleware);
        self
    }
}

/// Router for managing API routes and matching requests to handlers.
/// 
/// This struct maintains a collection of routes and provides methods for adding routes,
/// matching requests to handlers, and mounting routers with prefixes.
pub struct DMSRouter {
    /// Vector of registered routes
    routes: std::sync::RwLock<Vec<DMSRoute>>,
    /// Cache of route matches for improved performance
    route_cache: std::sync::RwLock<HashMap<String, DMSRoute>>,
}

impl DMSRouter {
    /// Creates a new router with no routes.
    /// 
    /// # Returns
    /// 
    /// A new `DMSRouter` instance with empty routes and cache
    pub fn new() -> Self {
        Self {
            routes: std::sync::RwLock::new(Vec::new()),
            route_cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Adds a route to the router.
    /// 
    /// This method adds a route to the router's collection and clears the route cache.
    /// 
    /// # Parameters
    /// 
    /// - `route`: The route to add to the router
    pub fn add_route(&self, route: DMSRoute) {
        let mut routes = self.routes.write().unwrap();
        routes.push(route);
        
        // Clear cache when routes are modified
        let mut cache = self.route_cache.write().unwrap();
        cache.clear();
    }

    /// Adds a GET route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn get(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("GET".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a POST route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn post(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("POST".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a PUT route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn put(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("PUT".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a DELETE route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn delete(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("DELETE".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a PATCH route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn patch(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("PATCH".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds an OPTIONS route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn options(&self, path: &str, handler: DMSRouteHandler) {
        let route = DMSRoute::new("OPTIONS".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Finds a matching route for the given request.
    /// 
    /// This method checks the route cache first, then searches through registered routes
    /// to find a match. It returns the handler for the matching route, or an error if no route is found.
    /// 
    /// # Parameters
    /// 
    /// - `request`: The gateway request to find a route for
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<DMSRouteHandler>` with the matching handler, or an error if no route is found
    pub async fn route(&self, request: &DMSGatewayRequest) -> DMSResult<DMSRouteHandler> {
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
            if self.matches_route(&route.method, &route.path, &request.method, &request.path) {
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

    /// Checks if a route matches a request.
    /// 
    /// This method implements route matching logic, including exact path matching,
    /// wildcard matching, and basic path parameter matching.
    /// 
    /// # Parameters
    /// 
    /// - `route_method`: HTTP method of the route
    /// - `route_path`: Path pattern of the route
    /// - `request_method`: HTTP method of the request
    /// - `request_path`: Path of the request
    /// 
    /// # Returns
    /// 
    /// `true` if the route matches the request, `false` otherwise
    fn matches_route(&self, route_method: &str, route_path: &str, request_method: &str, request_path: &str) -> bool {
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

    /// Mounts another router's routes with a prefix.
    /// 
    /// This method adds all routes from another router to this router, prepending
    /// the specified prefix to each route's path.
    /// 
    /// # Parameters
    /// 
    /// - `prefix`: The prefix to prepend to all mounted routes
    /// - `router`: The router to mount
    pub fn mount(&self, prefix: &str, router: &DMSRouter) {
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
            self.add_route(mounted_route);
        }
    }

    /// Clears all routes from the router.
    /// 
    /// This method removes all routes from the router and clears the route cache.
    pub fn clear_routes(&self) {
        let mut routes = self.routes.write().unwrap();
        let mut cache = self.route_cache.write().unwrap();
        routes.clear();
        cache.clear();
    }

    /// Gets the number of routes registered in the router.
    /// 
    /// # Returns
    /// 
    /// The number of routes registered in the router
    pub fn route_count(&self) -> usize {
        self.routes.read().unwrap().len()
    }
}