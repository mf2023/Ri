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

//! # Routing Module
//! 
//! This module provides a flexible routing system for the Ri gateway, using a Radix Tree
//! for O(k) route lookup performance, where k is the path length.
//! 
//! ## Key Components
//! 
//! - **RiRouteHandler**: Type alias for route handler functions
//! - **RiRoute**: Represents a single API route with method, path, handler, and middleware
//! - **RiRouter**: Manages routes using radix trees for efficient O(k) lookup
//! 
//! ## Design Principles
//! 
//! 1. **O(k) Performance**: Uses radix tree for route matching independent of route count
//! 2. **Type Safety**: Uses type aliases for clear handler signatures
//! 3. **Middleware Support**: Allows attaching middleware to individual routes
//! 4. **Route Caching**: Caches route matches for improved performance
//! 5. **Flexible Path Matching**: Supports exact paths, wildcards (`*path`), and path parameters (`:param`)
//! 6. **Method Support**: Supports all HTTP methods (GET, POST, PUT, DELETE, PATCH, OPTIONS)
//! 7. **Route Mounting**: Allows mounting routers with prefixes for modularity
//! 8. **Thread Safe**: Uses RwLock for safe operation in multi-threaded environments
//! 9. **Async Compatibility**: Built with async/await patterns for modern Rust applications
//! 
//! ## Path Pattern Syntax
//! 
//! - `/users/:id` - Matches `/users/123`, extracts `id = "123"`
//! - `/files/*path` - Matches `/files/docs/readme.txt`, extracts `path = "docs/readme.txt"`
//! - `/api/v1/users` - Exact match for static paths
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a router
//!     let router = Arc::new(RiRouter::new());
//!     
//!     // Create a simple handler
//!     let hello_handler = Arc::new(|req| {
//!         Box::pin(async move {
//!             Ok(RiGatewayResponse {
//!                 status_code: 200,
//!                 headers: FxHashMap::default(),
//!                 body: "Hello, Ri!".as_bytes().to_vec(),
//!             })
//!         })
//!     });
//!     
//!     // Add routes with O(k) lookup performance
//!     router.get("/hello", hello_handler.clone());
//!     router.post("/api/v1/users", hello_handler.clone());
//!     
//!     // Add route with path parameter
//!     router.get("/users/:id", hello_handler.clone());
//!     
//!     // Add route with wildcard
//!     router.get("/files/*path", hello_handler.clone());
//!     
//!     // Add route with middleware
//!     let auth_middleware = Arc::new(RiAuthMiddleware::new("Authorization".to_string()));
//!     let protected_route = RiRoute::new("GET".to_string(), "/api/v1/protected".to_string(), hello_handler)
//!         .with_middleware(auth_middleware);
//!     router.add_route(protected_route);
//!     
//!     Ok(())
//! }
//! ```

use super::{RiGatewayRequest, RiGatewayResponse};
use super::radix_tree::RiRadixTree;
use crate::core::RiResult;
use crate::core::lock::RwLockExtensions;
use crate::gateway::middleware::RiMiddleware;
use std::collections::HashMap as FxHashMap;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Type alias for route handler functions.
/// 
/// This type represents an asynchronous function that takes a gateway request and returns
/// a gateway response. It is wrapped in an Arc to allow safe sharing across threads.
pub type RiRouteHandler = Arc<
    dyn Fn(RiGatewayRequest) -> Pin<Box<dyn Future<Output = RiResult<RiGatewayResponse>> + Send>>
        + Send
        + Sync,
>;

/// Represents a single API route with method, path, handler, and middleware.
/// 
/// This struct encapsulates all the information needed for a single API endpoint,
/// including the HTTP method, path pattern, request handler, and attached middleware.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct RiRoute {
    /// HTTP method for this route (GET, POST, PUT, DELETE, PATCH, OPTIONS)
    pub method: String,
    /// Path pattern for this route (e.g., "/api/v1/users", "/users/:id")
    pub path: String,
    /// Request handler for this route
    pub handler: RiRouteHandler,
    /// List of middleware attached to this route
    pub middleware: Vec<Arc<dyn RiMiddleware>>,
}

impl fmt::Debug for RiRoute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RiRoute")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("handler", &"<handler>")
            .field("middleware_count", &self.middleware.len())
            .finish()
    }
}

impl RiRoute {
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
    /// A new `RiRoute` instance with no middleware attached
    pub fn new(
        method: String,
        path: String,
        handler: RiRouteHandler,
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
    /// A new `RiRoute` instance with the middleware attached
    pub fn with_middleware(mut self, middleware: Arc<dyn RiMiddleware>) -> Self {
        self.middleware.push(middleware);
        self
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for RiRoute
#[pyo3::prelude::pymethods]
impl RiRoute {
    #[new]
    fn py_new(method: String, path: String) -> PyResult<Self> {
        use crate::gateway::{RiGatewayRequest, RiGatewayResponse};
        
        // Create a simple default handler for Python usage
        let handler = Arc::new(|_req: RiGatewayRequest| -> Pin<Box<dyn Future<Output = Result<RiGatewayResponse, crate::core::RiError>> + Send>> {
            Box::pin(async move {
                Ok(RiGatewayResponse {
                    status_code: 200,
                    headers: std::collections::FxHashMap::default(),
                    body: b"Hello from Ri Python!".to_vec(),
                    request_id: String::new(),
                })
            })
        });
        
        Ok(Self {
            method,
            path,
            handler,
            middleware: Vec::new(),
        })
    }
}

/// Router for managing API routes and matching requests to handlers.
/// 
/// This struct maintains a collection of routes organized by HTTP method using
/// radix trees for O(k) lookup performance, where k is the path length.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiRouter {
    /// Radix trees for each HTTP method
    trees: std::sync::RwLock<FxHashMap<String, RiRadixTree>>,
    /// Vector of registered routes for backward compatibility and introspection
    routes: std::sync::RwLock<Vec<RiRoute>>,
    /// Cache of route matches for improved performance
    route_cache: std::sync::RwLock<FxHashMap<String, RiRoute>>,
}

impl Default for RiRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl RiRouter {
    /// Creates a new router with no routes.
    /// 
    /// # Returns
    /// 
    /// A new `RiRouter` instance with empty routes and cache
    pub fn new() -> Self {
        Self {
            trees: std::sync::RwLock::new(FxHashMap::default()),
            routes: std::sync::RwLock::new(Vec::new()),
            route_cache: std::sync::RwLock::new(FxHashMap::default()),
        }
    }

    /// Gets or creates a radix tree for the given HTTP method.
    /// 
    /// # Parameters
    /// 
    /// - `method`: The HTTP method
    /// 
    /// # Returns
    /// 
    /// A reference to the radix tree for the method
    #[allow(dead_code)]
    fn get_or_create_tree(&self, method: &str) -> RiRadixTree {
        let trees = match self.trees.read_safe("trees for get_or_create") {
            Ok(t) => t,
            Err(_) => return RiRadixTree::new(),
        };
        
        if let Some(_tree) = trees.get(method) {
            return RiRadixTree::new();
        }
        
        drop(trees);
        
        let mut trees_mut = match self.trees.write_safe("trees for create") {
            Ok(t) => t,
            Err(_) => return RiRadixTree::new(),
        };
        
        let tree = RiRadixTree::new();
        trees_mut.insert(method.to_string(), RiRadixTree::new());
        tree
    }

    /// Adds a route to the router.
    /// 
    /// This method adds a route to the router's radix tree and clears the route cache.
    /// 
    /// # Parameters
    /// 
    /// - `route`: The route to add to the router
    pub fn add_route(&self, route: RiRoute) {
        let method = route.method.clone();
        
        let mut trees = match self.trees.write_safe("trees for add_route") {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to acquire trees write lock: {}", e);
                return;
            }
        };
        
        let tree = trees.entry(method).or_insert_with(RiRadixTree::new);
        tree.insert(route.clone());
        
        drop(trees);
        
        let mut routes = match self.routes.write_safe("routes for add_route") {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to acquire routes write lock: {}", e);
                return;
            }
        };
        routes.push(route);
        
        let mut cache = match self.route_cache.write_safe("cache for add_route") {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to acquire cache write lock: {}", e);
                return;
            }
        };
        cache.clear();
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for RiRouter
#[pyo3::prelude::pymethods]
impl RiRouter {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Adds a GET route to the router from Python
    fn add_get_route(&self, path: String) {
        let route = RiRoute::py_new("GET".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds a POST route to the router from Python
    fn add_post_route(&self, path: String) {
        let route = RiRoute::py_new("POST".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds a PUT route to the router from Python
    fn add_put_route(&self, path: String) {
        let route = RiRoute::py_new("PUT".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds a DELETE route to the router from Python
    fn add_delete_route(&self, path: String) {
        let route = RiRoute::py_new("DELETE".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds a PATCH route to the router from Python
    fn add_patch_route(&self, path: String) {
        let route = RiRoute::py_new("PATCH".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds an OPTIONS route to the router from Python
    fn add_options_route(&self, path: String) {
        let route = RiRoute::py_new("OPTIONS".to_string(), path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Adds a custom route to the router from Python
    fn add_custom_route(&self, method: String, path: String) {
        let route = RiRoute::py_new(method, path).expect("Failed to create route");
        self.add_route(route);
    }
    
    /// Gets the number of routes registered in the router
    fn get_route_count(&self) -> usize {
        self.route_count()
    }
    
    /// Clears all routes from the router
    fn clear_all_routes(&self) {
        self.clear_routes();
    }
}

impl RiRouter {

    /// Adds a GET route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn get(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("GET".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a POST route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn post(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("POST".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a PUT route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn put(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("PUT".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a DELETE route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn delete(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("DELETE".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds a PATCH route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn patch(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("PATCH".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Adds an OPTIONS route to the router.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path pattern for the route
    /// - `handler`: Request handler for the route
    pub fn options(&self, path: &str, handler: RiRouteHandler) {
        let route = RiRoute::new("OPTIONS".to_string(), path.to_string(), handler);
        self.add_route(route);
    }

    /// Finds a matching route for the given request using radix tree.
    /// 
    /// This method uses radix tree for O(k) route lookup where k is the path length.
    /// It checks the route cache first, then searches the radix tree for the matching route.
    /// 
    /// # Parameters
    /// 
    /// - `request`: The gateway request to find a route for
    /// 
    /// # Returns
    /// 
    /// A `RiResult<RiRouteHandler>` with the matching handler, or an error if no route is found
    pub async fn route(&self, request: &RiGatewayRequest) -> RiResult<RiRouteHandler> {
        let cache_key = format!("{}:{}", request.method, request.path);
        
        {
            let cache = match self.route_cache.read_safe("cache for route lookup") {
                Ok(c) => c,
                Err(_) => return Err(crate::core::RiError::InvalidState("Failed to acquire cache read lock".to_string())),
            };
            if let Some(cached_route) = cache.get(&cache_key) {
                return Ok(cached_route.handler.clone());
            }
        }

        let trees = match self.trees.read_safe("trees for route lookup") {
            Ok(t) => t,
            Err(_) => return Err(crate::core::RiError::InvalidState("Failed to acquire trees read lock".to_string())),
        };
        
        if let Some(tree) = trees.get(&request.method) {
            if let Some(route_match) = tree.find(&request.path) {
                let mut cache = match self.route_cache.write_safe("cache for route insert") {
                    Ok(c) => c,
                    Err(_) => return Ok(route_match.route.handler.clone()),
                };
                cache.insert(cache_key.clone(), route_match.route.clone());
                
                return Ok(route_match.route.handler.clone());
            }
        }

        Err(crate::core::RiError::Other(format!(
            "No route found for {} {}",
            request.method, request.path
        )))
    }

    /// Checks if a route matches a request.
    /// 
    /// This method implements route matching logic using the radix tree.
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
    #[allow(dead_code)]
    fn matches_route(&self, route_method: &str, route_path: &str, request_method: &str, request_path: &str) -> bool {
        if route_method != request_method {
            return false;
        }

        let trees = match self.trees.read_safe("trees for matches_route") {
            Ok(t) => t,
            Err(_) => return false,
        };
        
        if let Some(tree) = trees.get(route_method) {
            if let Some(route_match) = tree.find(request_path) {
                return route_match.route.path == route_path;
            }
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
    pub fn mount(&self, prefix: &str, router: &RiRouter) {
        let routes = match router.routes.read_safe("routes for mount") {
            Ok(r) => r,
            Err(_) => return,
        };
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
    /// This method removes all routes from the router, clears all radix trees, and clears the route cache.
    pub fn clear_routes(&self) {
        let trees = match self.trees.write_safe("trees for clear") {
            Ok(t) => t,
            Err(e) => {
                log::error!("Failed to acquire trees write lock: {}", e);
                return;
            }
        };
        for tree in trees.values() {
            tree.clear();
        }
        drop(trees);
        
        let mut routes = match self.routes.write_safe("routes for clear") {
            Ok(r) => r,
            Err(e) => {
                log::error!("Failed to acquire routes write lock: {}", e);
                return;
            }
        };
        let mut cache = match self.route_cache.write_safe("cache for clear") {
            Ok(c) => c,
            Err(e) => {
                log::error!("Failed to acquire cache write lock: {}", e);
                return;
            }
        };
        routes.clear();
        cache.clear();
    }

    /// Gets the number of routes registered in the router.
    /// 
    /// # Returns
    /// 
    /// The number of routes registered in the router
    pub fn route_count(&self) -> usize {
        match self.routes.read_safe("routes for count") {
            Ok(routes) => routes.len(),
            Err(_) => 0,
        }
    }
}
