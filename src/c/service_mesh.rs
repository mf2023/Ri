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

//! # Service Mesh Module C API
//!
//! This module provides C language bindings for DMSC's service mesh infrastructure. The service mesh
//! module delivers comprehensive service-to-service communication capabilities including service discovery,
//! load balancing, circuit breaking, traffic routing, and observability for distributed systems. This
//! C API enables C/C++ applications to leverage DMSC's service mesh functionality for building resilient,
//! observable, and manageable microservices architectures.
//!
//! ## Module Architecture
//!
//! The service mesh module comprises three primary components that together provide complete service
//! management capabilities:
//!
//! - **DMSCServiceMesh**: Central service mesh controller managing service registry, traffic routing,
//!   and communication policies across all connected services. The mesh controller handles the complete
//!   lifecycle of service communication including discovery, routing, load balancing, and failure handling.
//!
//! - **DMSCServiceMeshConfig**: Configuration container for service mesh parameters including discovery
//!   settings, traffic management policies, circuit breaker thresholds, and observability options.
//!   The configuration object controls mesh behavior, resource allocation, and operational characteristics.
//!
//! - **DMSCServiceEndpoint**: Individual service endpoint representation within the mesh, tracking
//!   service instance metadata, health status, load metrics, and communication properties. Endpoints
//!   represent actual running instances of services that can receive traffic.
//!
//! ## Service Discovery
//!
//! The service mesh provides automatic service discovery capabilities:
//!
//! - **Service Registration**: Services automatically register with the mesh when they start,
//!   advertising their network location, port, and metadata. Registration includes health check
//!   endpoints and load reporting.
//!
//! - **Instance Tracking**: The mesh maintains a live registry of all service instances with
//!   real-time health status, load metrics, and topology information.
//!
//! - **DNS-Based Discovery**: Services discover each other using DNS-like naming conventions.
//!   Service names resolve to healthy instances based on routing policies.
//!
//! - **Health Checking**: Automatic health checks verify service instance availability. Unhealthy
//!   instances are removed from the load balancer rotation until they recover.
//!
//! - **Service Metadata**: Rich metadata attached to service registrations including version, zone,
//!   cluster, and custom labels for sophisticated routing decisions.
//!
//! ## Traffic Management
//!
//! The mesh implements sophisticated traffic routing:
//!
//! - **Load Balancing**: Multiple load balancing algorithms including round-robin, least-connections,
//!   weighted distribution, and consistent hashing for session affinity.
//!
//! - **Canary Deployments**: Gradual traffic shifting between service versions for safe rollouts.
//!   Configure percentage-based or rules-based traffic splitting.
//!
//! - **A/B Testing**: Route specific percentages of traffic to different service versions
//!   for testing new features with production traffic.
//!
//! - **Blue-Green Deployments**: Zero-downtime deployments by switching traffic between complete
//!   service environments. Enables instant rollback capability.
//!
//! - **Traffic Mirroring**: Copy production traffic to shadow services for testing without
//!   affecting production responses. Useful for validating new service versions.
//!
//! - **Retries and Timeouts**: Configurable retry policies and timeout values for all
//!   service-to-service calls. Automatic retry for idempotent operations.
//!
//! ## Circuit Breaking
//!
//! The service mesh implements circuit breaker patterns for failure isolation:
//!
//! - **Failure Detection**: Automatic detection of downstream service failures through
//!   error rates, latency percentiles, and custom health indicators.
//!
//! - **Open Circuit**: When failure threshold is exceeded, the circuit opens and requests
//!   immediately fail without making downstream calls. Prevents cascade failures.
//!
//! - **Half-Open State**: After cooldown period, limited requests are allowed through to
//!   test if the downstream service has recovered.
//!
//! - **Close Circuit**: Successful responses during half-open state close the circuit
//!   and restore normal operation.
//!
//! - **Configuration**: Tunable failure thresholds, timeout values, and volume thresholds
//!   for each service dependency.
//!
//! - **Fallback Handling**: Configurable fallback responses or alternative service calls
//!   when circuit is open. Enables graceful degradation.
//!
//! ## Resilience Patterns
//!
//! The mesh provides comprehensive resilience mechanisms:
//!
//! - **Bulkhead Isolation**: Isolate critical resources by limiting concurrent requests
//!   to downstream services. Prevents one slow service from affecting others.
//!
//! - **Rate Limiting**: Per-service and per-instance rate limits prevent overload scenarios.
//!   Supports token bucket and sliding window algorithms.
//!
//! - **Retry Policies**: Configurable retry attempts, backoff strategies (fixed, exponential,
//!   jitter), and retry conditions (5xx, timeouts, connection failures).
//!
//! - **Timeout Management**: End-to-end timeout propagation ensures requests don't wait
//!   indefinitely. Supports deadline propagation across service boundaries.
//!
//! ## Security
//!
//! The service mesh implements zero-trust security principles:
//!
//! - **mTLS Encryption**: Automatic mutual TLS encryption for all service-to-service
//!   communication. Certificates automatically rotated and validated.
//!
//! - **Service Identity**: Strong identity verification using SPIFFE-style identifiers.
//!   Each service has verifiable credentials independent of network location.
//!
//! - **Authorization Policies**: Fine-grained access control policies defining which
//!   services can communicate. Default deny-all policy with explicit allow rules.
//!
//! - **Authentication**: JWT validation, OAuth token exchange, and custom authentication
//!   mechanisms integrated at the mesh layer.
//!
//! - **Network Policies**: L3/L4 network filtering restricting communication between
//!   services based on identity and metadata.
//!
//! ## Observability
//!
//! The mesh provides comprehensive observability features:
//!
//! - **Traffic Metrics**: Request rates, latencies (p50, p95, p99), and error rates
//!   for all service-to-service communication.
//!
//! - **Distributed Tracing**: Automatic trace context propagation across service
//!   boundaries with W3C Trace Context standard.
//!
//! - **Service Graph**: Real-time visualization of service topology and communication
//!   patterns. Identifies dependencies and communication bottlenecks.
//!
//! - **Access Logging**: Detailed logs of all service-to-service communication including
//!   headers, status codes, and timing information.
//!
//! - **Custom Metrics**: User-defined metrics collected at the mesh layer independent
//!   of application code.
//!
//! ## Performance Characteristics
//!
//! Service mesh operations are optimized for minimal overhead:
//!
//! - **Sidecar Pattern**: Lightweight proxy (Envoy) deployed alongside each service instance.
//!   Intercepts all traffic with minimal latency overhead.
//!
//! - **xDS Configuration**: Efficient configuration distribution using xDS (discovery services)
//!   protocol. Changes propagate in seconds across the mesh.
//!
//! - **Connection Pooling**: Efficient connection management to upstream services with
//!   configurable pool sizes and connection limits.
//!
//! - **Batching and Buffering**: Request batching and response buffering optimize throughput
//!   while maintaining latency SLAs.
//!
//! - **Zero-Copy**: Modern data plane implementations use zero-copy techniques where
//!   possible to minimize CPU overhead.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Service endpoints are managed by the mesh controller
//! - Configuration objects control mesh-wide settings
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Concurrent service registration from multiple instances supported
//! - Route configuration updates atomic across the mesh
//! - Load balancing decisions thread-safe
//! - Metrics collection lock-free for performance
//!
//! ## Usage Example
//!
//! ```c
//! // Create service mesh configuration
//! DMSCServiceMeshConfig* config = dmsc_service_mesh_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create service mesh config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure mesh settings
//! dmsc_service_mesh_config_set_enable_mtls(config, true);
//! dmsc_service_mesh_config_set_enable_circuit_breaker(config, true);
//! dmsc_service_mesh_config_set_enable_tracing(config, true);
//! dmsc_service_mesh_config_set_enable_metrics(config, true);
//!
//! // Configure circuit breaker
//! dmsc_service_mesh_config_set_circuit_breaker_failure_rate(config, 0.5);
//! dmsc_service_mesh_config_set_circuit_breaker_timeout_ms(config, 30000);
//!
//! // Configure load balancing
//! dmsc_service_mesh_config_set_load_balancing_algorithm(config, LB_LEAST_REQUESTS);
//!
//! // Create service mesh controller
//! DMSCServiceMesh* mesh = dmsc_service_mesh_new(config);
//! if (mesh == NULL) {
//!     fprintf(stderr, "Failed to create service mesh\n");
//!     dmsc_service_mesh_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Start the mesh controller
//! int result = dmsc_service_mesh_start(mesh);
//! if (result != 0) {
//!     fprintf(stderr, "Failed to start service mesh: %d\n", result);
//!     dmsc_service_mesh_free(mesh);
//!     dmsc_service_mesh_config_free(config);
//!     return ERROR_START;
//! }
//!
//! printf("Service mesh started successfully\n");
//!
//! // Register a service endpoint
//! DMSCServiceEndpoint* endpoint = dmsc_service_endpoint_new();
//! if (endpoint == NULL) {
//!     fprintf(stderr, "Failed to create service endpoint\n");
//!     dmsc_service_mesh_stop(mesh);
//!     dmsc_service_mesh_free(mesh);
//!     dmsc_service_mesh_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! dmsc_service_endpoint_set_name(endpoint, "user-service");
//! dmsc_service_endpoint_set_host(endpoint, "10.0.1.5");
//! dmsc_service_endpoint_set_port(endpoint, 8080);
//! dmsc_service_endpoint_set_version(endpoint, "v1.2.3");
//! dmsc_service_endpoint_set_health_check_url(endpoint, "/health");
//!
//! result = dmsc_service_mesh_register(mesh, endpoint);
//! if (result != 0) {
//!     fprintf(stderr, "Failed to register endpoint: %d\n", result);
//!     dmsc_service_endpoint_free(endpoint);
//! } else {
//!     printf("Service endpoint registered successfully\n");
//! }
//!
//! // Get service information
//! const char* service_name = dmsc_service_endpoint_get_name(endpoint);
//! const char* service_version = dmsc_service_endpoint_get_version(endpoint);
//! uint32_t healthy_count = dmsc_service_mesh_get_healthy_count(mesh, service_name);
//!
//! printf("Service: %s (version: %s), healthy instances: %u\n",
//!        service_name, service_version, healthy_count);
//!
//! // Discover a service
//! DMSCServiceEndpoint* discovered = NULL;
//! result = dmsc_service_mesh_discover(mesh, "user-service", &discovered);
//!
//! if (result == 0 && discovered != NULL) {
//!     const char* host = dmsc_service_endpoint_get_host(discovered);
//!     uint16_t port = dmsc_service_endpoint_get_port(discovered);
//!
//!     printf("Discovered service at %s:%d\n", host, port);
//!
//!     dmsc_service_endpoint_free(discovered);
//! }
//!
//! // Update service mesh configuration at runtime
//! dmsc_service_mesh_config_set_circuit_breaker_failure_rate(mesh, 0.3);
//! dmsc_service_mesh_reload_config(mesh);
//!
//! // Get service mesh statistics
//! uint64_t total_requests = dmsc_service_mesh_get_total_requests(mesh);
//! uint64_t failed_requests = dmsc_service_mesh_get_failed_requests(mesh);
//!
//! printf("Mesh stats: %lu total requests, %lu failed\n",
//!        total_requests, failed_requests);
//!
//! // Deregister service when shutting down
//! dmsc_service_mesh_deregister(mesh, endpoint);
//!
//! // Graceful shutdown
//! dmsc_service_mesh_stop(mesh);
//! dmsc_service_endpoint_free(endpoint);
//! dmsc_service_mesh_free(mesh);
//! dmsc_service_mesh_config_free(config);
//!
//! printf("Service mesh shutdown complete\n");
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::service_mesh`: Rust service mesh module implementation
//! - `crate::prelude`: Common types and traits
//! - Envoy proxy for data plane (embedded or external)
//! - xDS protocol stack for configuration management
//!
//! ## Feature Flags
//!
//! The service mesh module is enabled by the "service-mesh" feature flag.
//! Disable this feature to reduce binary size when service mesh is not required.
//!
//! Additional features:
//!
//! - `service-mesh-mtls`: Enable mutual TLS encryption
//! - `service-mesh-circuit-breaker`: Enable circuit breaker functionality
//! - `service-mesh-tracing`: Enable distributed tracing
//! - `service-mesh-metrics`: Enable metrics collection
//! - `service-mesh-dns`: Enable DNS-based service discovery

use crate::service_mesh::{DMSCServiceEndpoint, DMSCServiceMesh, DMSCServiceMeshConfig};


c_wrapper!(CDMSCServiceMesh, DMSCServiceMesh);
c_wrapper!(CDMSCServiceMeshConfig, DMSCServiceMeshConfig);
c_wrapper!(CDMSCServiceEndpoint, DMSCServiceEndpoint);

// DMSCServiceMeshConfig constructors and destructors
c_constructor!(
    dmsc_service_mesh_config_new,
    CDMSCServiceMeshConfig,
    DMSCServiceMeshConfig,
    DMSCServiceMeshConfig::default()
);
c_destructor!(dmsc_service_mesh_config_free, CDMSCServiceMeshConfig);
