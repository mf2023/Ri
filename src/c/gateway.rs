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
!
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


/// Opaque C wrapper structure for DMSCGateway.
///
/// Core gateway server implementation handling HTTP request processing, middleware composition,
/// and response generation. The gateway acts as the entry point for all incoming API requests.
///
/// # Gateway Responsibilities
///
/// The gateway server manages:
///
/// - **Request Acceptance**: Accept incoming TCP connections and parse HTTP requests
/// - **Request Processing**: Apply middleware chain and route matching
/// - **Backend Communication**: Forward requests to backend services and receive responses
/// - **Response Generation**: Apply response middleware and send responses to clients
/// - **Connection Management**: Handle keep-alive, timeouts, and connection lifecycle
/// - **Health Monitoring**: Track backend health and remove unhealthy instances
///
/// # Server Architecture
///
/// The gateway uses a multi-threaded architecture:
///
/// - **Acceptor Thread**: Accepts incoming connections and distributes to worker threads
/// - **Worker Threads**: Process requests through middleware and routing
/// - **Backend Pool**: Manages connections to backend services
/// - **Metrics Collector**: Tracks request statistics and performance metrics
///
/// # Request Processing Pipeline
///
/// Requests flow through the gateway:
///
/// 1. Connection accepted and HTTP request parsed
/// 2. Request enters middleware chain (authentication, logging, etc.)
/// 3. Router matches request to registered route
/// 4. Request optionally transformed before backend forwarding
/// 5. Backend selected via load balancing algorithm
/// 6. Request forwarded to backend service
/// 7. Response received and transformed if needed
/// 8. Response middleware applied (compression, caching, etc.)
/// 9. Response sent to client
///
/// # Thread Safety
///
/// The gateway server is fully thread-safe:
///
/// - Concurrent request handling supported
/// - Dynamic route updates may require synchronization
/// - Configuration changes may require restart
/// - Metrics collection is lock-free for performance
c_wrapper!(CDMSCGateway, DMSCGateway);

/// Opaque C wrapper structure for DMSCGatewayConfig.
///
/// Configuration container for gateway server parameters controlling resource allocation,
/// security settings, and behavioral characteristics.
///
/// # Configuration Parameters
///
/// The gateway configuration controls:
///
/// - **Network Settings**: Listen address, port, and binding options
/// - **Thread Configuration**: Number of worker threads, connection queue size
/// - **Timeout Settings**: Request timeout, keep-alive duration, slow request threshold
/// - **TLS Configuration**: Certificate paths, cipher suites, HTTP/2 settings
/// - **Rate Limiting**: Global and per-client rate limits
/// - **Circuit Breaker**: Failure thresholds and recovery settings
/// - **Logging**: Log level, format, and output destinations
///
/// # Memory Layout
///
/// The structure uses #[repr(C)] ensuring binary compatibility:
/// - Consistent field alignment across Rust versions
/// - Predictable size for FFI boundaries
/// - No hidden padding affecting pointer arithmetic
c_wrapper!(CDMSCGatewayConfig, DMSCGatewayConfig);

/// Opaque C wrapper structure for DMSCRouter.
///
/// Request routing component responsible for matching incoming requests to registered routes
/// based on method, path, headers, and other request attributes.
///
/// # Routing Responsibilities
///
/// The router handles:
///
/// - **Route Registration**: Adding routes with handlers and middleware
/// - **Route Matching**: Finding matching route for incoming requests
/// - **Parameter Extraction**: Capturing path parameters and query values
/// - **Middleware Attachment**: Applying middleware to specific routes
/// - **Route Groups**: Organizing routes with shared configuration
/// - **Route Documentation**: Generating OpenAPI/Swagger documentation
///
/// # Route Matching Algorithm
///
/// The router uses an efficient matching algorithm:
///
/// 1. Match HTTP method (reject non-matching methods early)
/// 2. Match path prefix
/// 3. Match path pattern (static, parameterized, wildcard)
/// 4. Match header conditions if specified
/// 5. Match query conditions if specified
/// 6. Return most specific match (parameterized over wildcard)
///
/// # Performance Optimization
///
/// The router implements several optimizations:
///
/// - **Prefix Trie**: Efficient path lookup using prefix tree structure
/// - **Method Routing**: Separate trees per HTTP method for faster matching
/// - **Cached Matches**: Recent route matches cached for performance
/// - **Zero-Copy Parsing**: Path parsing minimizes memory allocation
///
/// # Thread Safety
///
/// The router supports concurrent read access:
///
/// - Route lookups are lock-free for read operations
/// - Route registration may require write lock
/// - Dynamic updates use atomic pointer swaps
/// - Consider read-write lock for high-frequency updates
c_wrapper!(CDMSCRouter, DMSCRouter);

/// Creates a new DMSCGatewayConfig instance with default configuration values.
///
/// Initializes a gateway configuration object with sensible production defaults:
/// - Default listen address: 0.0.0.0
/// - Default port: 8080
/// - Default worker threads: number of CPU cores
/// - Default request timeout: 30 seconds
/// - Default keep-alive: 75 seconds
/// - Default rate limit: 1000 requests per minute
///
/// # Returns
///
/// Pointer to newly allocated DMSCGatewayConfig on success, or NULL if memory
/// allocation fails. The returned pointer must be freed using dmsc_gateway_config_free().
///
/// # Default Configuration
///
/// The default configuration is suitable for most deployment scenarios:
///
/// - Listens on all network interfaces
/// - Uses reasonable thread pool for multi-core systems
/// - Provides adequate timeouts for typical API operations
/// - Enables reasonable rate limiting out of the box
///
/// # Customization
///
/// After creation, configuration can be customized:
///
/// - dmsc_gateway_config_set_address() for custom bind address
/// - dmsc_gateway_config_set_port() for custom port
/// - dmsc_gateway_config_set_workers() for thread pool tuning
/// - dmsc_gateway_config_set_timeout() for request timeout adjustment
/// - dmsc_gateway_config_set_tls_cert() for HTTPS support
c_constructor!(dmsc_gateway_config_new, DMSCGatewayConfig, DMSCGatewayConfig, DMSCGatewayConfig::default());

/// Frees a previously allocated DMSCGatewayConfig instance.
///
/// Releases all memory associated with the configuration object including any
/// internally allocated certificates, TLS configurations, or sub-objects.
///
/// # Parameters
///
/// - `config`: Pointer to DMSCGatewayConfig to free. NULL is safe and returns immediately.
///
/// # Safety
///
/// Safe to call with NULL. Calling with already-freed pointer is undefined behavior.
/// Implement proper ownership tracking to prevent double-free vulnerabilities.
c_destructor!(dmsc_gateway_config_free, DMSCGatewayConfig);
