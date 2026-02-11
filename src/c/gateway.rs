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

//! # Gateway Module C API
//!
//! This module provides C language bindings for DMSC's API gateway subsystem. The gateway module
//! delivers high-performance HTTP request routing, load balancing, rate limiting, and request/response
//! transformation capabilities. This C API enables C/C++ applications to leverage DMSC's gateway
//! functionality for building scalable API endpoints with enterprise-grade features.
//!
//! ## Module Architecture
//!
//! The gateway module comprises three primary components that together provide complete API gateway
//! functionality:
//!
//! - **DMSCGateway**: Core gateway server implementation handling HTTP request processing,
//!   middleware composition, and response generation. The gateway acts as the entry point for
//!   all incoming API requests, applying configured middleware chains and routing requests to
//!   appropriate backend services.
//!
//! - **DMSCGatewayConfig**: Configuration container for gateway server parameters including listen
//!   address, thread pool sizing, TLS settings, and middleware configuration. The configuration
//!   object controls resource allocation, security settings, and behavioral characteristics.
//!
//! - **DMSCRouter**: Request routing component responsible for matching incoming requests to
//!   registered routes based on method, path, headers, and other request attributes. The router
//!   supports complex routing patterns including path parameters, wildcards, and regex matching.
//!
//! ## Gateway Features
//!
//! The API gateway provides comprehensive features for production API management:
//!
//! - **Request Routing**: Advanced routing capabilities including path matching, method-based
//!   routing, header-based routing, and query parameter routing. Supports route groups and
//!   hierarchical routing patterns.
//!
//! - **Load Balancing**: Distribution of requests across multiple backend instances with
//!   configurable algorithms including round-robin, least-connections, weighted distribution,
//!   and consistent hashing for session affinity.
//!
//! - **Rate Limiting**: Request rate control at multiple granularity levels including global,
//!   per-client, per-route, and per-user rate limits. Supports sliding window, token bucket,
//!   and leaky bucket algorithms.
//!
//! - **Middleware Chain**: Composable middleware for request/response processing including
//!   authentication, authorization, logging, compression, caching, and transformation.
//!   Middleware executes in configured order with early exit capabilities.
//!
//! - **Request/Response Transformation**: Content transformation between client and backend
//!   formats including JSON to XML conversion, header manipulation, body rewriting, and
//!   protocol translation.
//!
//! - **Circuit Breaker**: Automatic detection of backend failures with configurable thresholds
//!   for failure rates, timeout windows, and recovery strategies. Prevents cascade failures
//!   in distributed systems.
//!
//! - **WebSocket Support**: Full-duplex WebSocket connections with session management,
//!   heartbeat handling, and connection lifecycle events.
//!
//! - **TLS Termination**: Secure communication with configurable TLS settings including
//!   certificate management, cipher suite selection, and HTTP/2 support.
//!
//! ## Routing Capabilities
//!
//! The router supports sophisticated routing patterns:
//!
//! - **Static Paths**: Exact path matching for simple routes (e.g., /api/users)
//!
//! - **Path Parameters**: Variable path segments captured as parameters (e.g., /users/:id)
//!
//! - **Wildcard Matching**: Catch-all routes for static file serving (e.g., /static/*)
//!
//! - **Regex Patterns**: Complex matching rules using regular expressions
//!
//! - **Method Matching**: Route requests by HTTP method (GET, POST, PUT, DELETE, etc.)
//!
//! - **Header Matching**: Route based on request headers (e.g., X-API-Version)
//!
//! - **Query Matching**: Route based on query parameters
//!
//! ## Middleware Types
//!
//! The gateway supports various middleware categories:
//!
//! - **Authentication Middleware**: JWT validation, OAuth token verification, API key checking,
//!   and custom authentication schemes.
//!
//! - **Authorization Middleware**: Role-based access control, permission checking, and
//!   policy enforcement at route level.
//!
//! - **Logging Middleware**: Request/response logging, access logging, and structured logging
//!   for observability.
//!
//! - **Compression Middleware**: Gzip, Brotli, and Deflate compression for response bodies.
//!
//! - **Caching Middleware**: Response caching with TTL control, cache invalidation, and
//!   conditional requests (ETag, If-Modified-Since).
//!
//! - **Transformation Middleware**: Header addition/removal, body transformation, and content
//!   type conversion.
//!
//! - **Rate Limiting Middleware**: Request throttling with configurable limits and response
//!   handling.
//!
//! - **Circuit Breaker Middleware**: Failure detection and fallback handling for backend
//!   services.
//!
//! ## Load Balancing Strategies
//!
//! The gateway implements multiple load balancing algorithms:
//!
//! - **Round Robin**: Sequential distribution across available backends. Simple and effective
//!   for homogeneous backends with similar capacity.
//!
//! - **Least Connections**: Route to backend with fewest active connections. Adapts to
//!   varying request processing times.
//!
//! - **Weighted Distribution**: Proportional routing based on backend capacity or priority.
//!   Requires backend health and capacity configuration.
//!
//! - **Consistent Hashing**: Request affinity based on request attributes (e.g., user ID).
//!   Minimizes redistribution when backends change.
//!
//! - **Random**: Random selection among healthy backends. Simple and effective for
//!   homogeneous backends.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Route handlers must be properly registered and unregistered
//! - Connection handles must be properly closed
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Gateway server handles concurrent requests using thread pool
//! - Router operations are thread-safe for route lookups
//! - Configuration changes may require gateway restart
//! - Middleware should be stateless when possible
//!
//! ## Performance Characteristics
//!
//! Gateway operations have the following performance profiles:
//!
//! - Request routing: O(log n) for route lookup with path parameters
//! - Middleware processing: O(m) where m is number of middleware
//! - Load balancing: O(1) for most algorithms
//! - Request throughput: Thousands of requests per second per core
//!
//! ## Usage Example
//!
//! ```c
//! // Create gateway configuration
//! DMSCGatewayConfig* config = dmsc_gateway_config_new();
//! dmsc_gateway_config_set_address(config, "0.0.0.0", 8080);
//! dmsc_gateway_config_set_workers(config, 4);
//! dmsc_gateway_config_set_tls_enabled(config, false);
//!
//! // Create gateway instance
//! DMSCGateway* gateway = dmsc_gateway_new(config);
//!
//! // Create router and configure routes
//! DMSCRouter* router = dmsc_router_new();
//!
//! // Register routes
//! dmsc_router_add_route(router, "GET", "/api/users", handle_users);
//! dmsc_router_add_route(router, "POST", "/api/users", create_user);
//! dmsc_router_add_route(router, "GET", "/api/users/:id", get_user_by_id);
//!
//! // Mount router on gateway
//! dmsc_gateway_mount(gateway, "/api", router);
//!
//! // Start gateway
//! dmsc_gateway_start(gateway);
//!
//! // Graceful shutdown on signal
//! // dmsc_gateway_shutdown(gateway);
//!
//! // Cleanup
//! dmsc_gateway_free(gateway);
//! dmsc_gateway_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::gateway`: Rust gateway module implementation
//! - `crate::prelude`: Common types and traits
//! - Hyper for HTTP server functionality
//! - Redis for rate limiting and session storage
//!
//! ## Feature Flags
//!
//! The gateway module is enabled by default with the "gateway" feature flag.
//! Disable this feature to reduce binary size when gateway functionality is not required.

use crate::gateway::{DMSCGateway, DMSCGatewayConfig, DMSCRouter};


c_wrapper!(CDMSCGateway, DMSCGateway);

c_wrapper!(CDMSCGatewayConfig, DMSCGatewayConfig);

c_wrapper!(CDMSCRouter, DMSCRouter);

c_constructor!(dmsc_gateway_config_new, CDMSCGatewayConfig, DMSCGatewayConfig, DMSCGatewayConfig::default());

c_destructor!(dmsc_gateway_config_free, DMSCGatewayConfig);
