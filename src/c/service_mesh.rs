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

//! # Service Mesh Module C API
//!
//! This module provides C language bindings for Ri's service mesh infrastructure. The service mesh
//! module delivers comprehensive service-to-service communication capabilities including service discovery,
//! load balancing, circuit breaking, traffic routing, and observability for distributed systems. This
//! C API enables C/C++ applications to leverage Ri's service mesh functionality for building resilient,
//! observable, and manageable microservices architectures.
//!
//! ## Module Architecture
//!
//! The service mesh module comprises three primary components that together provide complete service
//! management capabilities:
//!
//! - **RiServiceMesh**: Central service mesh controller managing service registry, traffic routing,
//!   and communication policies across all connected services. The mesh controller handles the complete
//!   lifecycle of service communication including discovery, routing, load balancing, and failure handling.
//!
//! - **RiServiceMeshConfig**: Configuration container for service mesh parameters including discovery
//!   settings, traffic management policies, circuit breaker thresholds, and observability options.
//!   The configuration object controls mesh behavior, resource allocation, and operational characteristics.
//!
//! - **RiServiceEndpoint**: Individual service endpoint representation within the mesh, tracking
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
//! RiServiceMeshConfig* config = ri_service_mesh_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create service mesh config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure mesh settings
//! ri_service_mesh_config_set_enable_mtls(config, true);
//! ri_service_mesh_config_set_enable_circuit_breaker(config, true);
//! ri_service_mesh_config_set_enable_tracing(config, true);
//! ri_service_mesh_config_set_enable_metrics(config, true);
//!
//! // Configure circuit breaker
//! ri_service_mesh_config_set_circuit_breaker_failure_rate(config, 0.5);
//! ri_service_mesh_config_set_circuit_breaker_timeout_ms(config, 30000);
//!
//! // Configure load balancing
//! ri_service_mesh_config_set_load_balancing_algorithm(config, LB_LEAST_REQUESTS);
//!
//! // Create service mesh controller
//! RiServiceMesh* mesh = ri_service_mesh_new(config);
//! if (mesh == NULL) {
//!     fprintf(stderr, "Failed to create service mesh\n");
//!     ri_service_mesh_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Start the mesh controller
//! int result = ri_service_mesh_start(mesh);
//! if (result != 0) {
//!     fprintf(stderr, "Failed to start service mesh: %d\n", result);
//!     ri_service_mesh_free(mesh);
//!     ri_service_mesh_config_free(config);
//!     return ERROR_START;
//! }
//!
//! printf("Service mesh started successfully\n");
//!
//! // Register a service endpoint
//! RiServiceEndpoint* endpoint = ri_service_endpoint_new();
//! if (endpoint == NULL) {
//!     fprintf(stderr, "Failed to create service endpoint\n");
//!     ri_service_mesh_stop(mesh);
//!     ri_service_mesh_free(mesh);
//!     ri_service_mesh_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! ri_service_endpoint_set_name(endpoint, "user-service");
//! ri_service_endpoint_set_host(endpoint, "10.0.1.5");
//! ri_service_endpoint_set_port(endpoint, 8080);
//! ri_service_endpoint_set_version(endpoint, "v1.2.3");
//! ri_service_endpoint_set_health_check_url(endpoint, "/health");
//!
//! result = ri_service_mesh_register(mesh, endpoint);
//! if (result != 0) {
//!     fprintf(stderr, "Failed to register endpoint: %d\n", result);
//!     ri_service_endpoint_free(endpoint);
//! } else {
//!     printf("Service endpoint registered successfully\n");
//! }
//!
//! // Get service information
//! const char* service_name = ri_service_endpoint_get_name(endpoint);
//! const char* service_version = ri_service_endpoint_get_version(endpoint);
//! uint32_t healthy_count = ri_service_mesh_get_healthy_count(mesh, service_name);
//!
//! printf("Service: %s (version: %s), healthy instances: %u\n",
//!        service_name, service_version, healthy_count);
//!
//! // Discover a service
//! RiServiceEndpoint* discovered = NULL;
//! result = ri_service_mesh_discover(mesh, "user-service", &discovered);
//!
//! if (result == 0 && discovered != NULL) {
//!     const char* host = ri_service_endpoint_get_host(discovered);
//!     uint16_t port = ri_service_endpoint_get_port(discovered);
//!
//!     printf("Discovered service at %s:%d\n", host, port);
//!
//!     ri_service_endpoint_free(discovered);
//! }
//!
//! // Update service mesh configuration at runtime
//! ri_service_mesh_config_set_circuit_breaker_failure_rate(mesh, 0.3);
//! ri_service_mesh_reload_config(mesh);
//!
//! // Get service mesh statistics
//! uint64_t total_requests = ri_service_mesh_get_total_requests(mesh);
//! uint64_t failed_requests = ri_service_mesh_get_failed_requests(mesh);
//!
//! printf("Mesh stats: %lu total requests, %lu failed\n",
//!        total_requests, failed_requests);
//!
//! // Deregister service when shutting down
//! ri_service_mesh_deregister(mesh, endpoint);
//!
//! // Graceful shutdown
//! ri_service_mesh_stop(mesh);
//! ri_service_endpoint_free(endpoint);
//! ri_service_mesh_free(mesh);
//! ri_service_mesh_config_free(config);
//!
//! printf("Service mesh shutdown complete\n");
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
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

use crate::service_mesh::{
    RiServiceEndpoint, RiServiceMesh, RiServiceMeshConfig,
    RiServiceDiscovery, RiServiceInstance,
    RiHealthChecker, RiHealthStatus,
    RiTrafficManager, RiTrafficRoute,
};


c_wrapper!(CRiServiceMesh, RiServiceMesh);
c_wrapper!(CRiServiceMeshConfig, RiServiceMeshConfig);
c_wrapper!(CRiServiceEndpoint, RiServiceEndpoint);

// RiServiceMeshConfig constructors and destructors
c_constructor!(
    ri_service_mesh_config_new,
    CRiServiceMeshConfig,
    RiServiceMeshConfig,
    RiServiceMeshConfig::default()
);
c_destructor!(ri_service_mesh_config_free, CRiServiceMeshConfig);

// RiServiceMesh constructors and destructors
#[no_mangle]
pub extern "C" fn ri_service_mesh_new(config: *mut CRiServiceMeshConfig) -> *mut CRiServiceMesh {
    if config.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let config = (*config).inner.clone();
        match RiServiceMesh::new(config) {
            Ok(mesh) => Box::into_raw(Box::new(CRiServiceMesh::new(mesh))),
            Err(_) => std::ptr::null_mut(),
        }
    }
}
c_destructor!(ri_service_mesh_free, CRiServiceMesh);

// RiServiceEndpoint constructors and destructors
#[no_mangle]
pub extern "C" fn ri_service_endpoint_new(
    service_name: *const std::ffi::c_char,
    endpoint: *const std::ffi::c_char,
    weight: u32,
) -> *mut CRiServiceEndpoint {
    if service_name.is_null() || endpoint.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let endpoint_str = match std::ffi::CStr::from_ptr(endpoint).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let ep = RiServiceEndpoint {
            service_name: service_name_str,
            endpoint: endpoint_str,
            weight,
            metadata: std::collections::HashMap::default(),
            health_status: crate::service_mesh::RiServiceHealthStatus::Unknown,
            last_health_check: std::time::SystemTime::now(),
        };
        Box::into_raw(Box::new(CRiServiceEndpoint::new(ep)))
    }
}
c_destructor!(ri_service_endpoint_free, CRiServiceEndpoint);

// RiServiceEndpoint getters
c_string_getter!(
    ri_service_endpoint_get_service_name,
    CRiServiceEndpoint,
    |inner: &RiServiceEndpoint| inner.service_name.clone()
);
c_string_getter!(
    ri_service_endpoint_get_endpoint,
    CRiServiceEndpoint,
    |inner: &RiServiceEndpoint| inner.endpoint.clone()
);

#[no_mangle]
pub extern "C" fn ri_service_endpoint_get_weight(obj: *mut CRiServiceEndpoint) -> u32 {
    if obj.is_null() {
        return 0;
    }
    unsafe { (*obj).inner.weight }
}

#[no_mangle]
pub extern "C" fn ri_service_endpoint_get_health_status(obj: *mut CRiServiceEndpoint) -> std::ffi::c_int {
    if obj.is_null() {
        return -1;
    }
    unsafe {
        match (*obj).inner.health_status {
            crate::service_mesh::RiServiceHealthStatus::Healthy => 0,
            crate::service_mesh::RiServiceHealthStatus::Unhealthy => 1,
            crate::service_mesh::RiServiceHealthStatus::Unknown => 2,
        }
    }
}

// RiServiceMesh functions
#[no_mangle]
pub extern "C" fn ri_service_mesh_register(
    mesh: *mut CRiServiceMesh,
    endpoint: *mut CRiServiceEndpoint,
) -> std::ffi::c_int {
    if mesh.is_null() || endpoint.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let ep = (*endpoint).inner.clone();
        rt.block_on(async {
            match (*mesh).inner.register_service(&ep.service_name, &ep.endpoint, ep.weight, None).await {
                Ok(_) => 0,
                Err(_) => -3,
            }
        })
    }
}

#[no_mangle]
pub extern "C" fn ri_service_mesh_deregister(
    mesh: *mut CRiServiceMesh,
    service_name: *const std::ffi::c_char,
    endpoint: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if mesh.is_null() || service_name.is_null() || endpoint.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let endpoint_str = match std::ffi::CStr::from_ptr(endpoint).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        rt.block_on(async {
            match (*mesh).inner.unregister_service(service_name_str, endpoint_str).await {
                Ok(_) => 0,
                Err(_) => -4,
            }
        })
    }
}

#[no_mangle]
pub extern "C" fn ri_service_mesh_discover(
    mesh: *mut CRiServiceMesh,
    service_name: *const std::ffi::c_char,
    out_endpoints: *mut *mut CRiServiceEndpoint,
    out_count: *mut usize,
) -> std::ffi::c_int {
    if mesh.is_null() || service_name.is_null() || out_endpoints.is_null() || out_count.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*mesh).inner.discover_service(service_name_str).await }) {
            Ok(endpoints) => {
                let count = endpoints.len();
                *out_count = count;
                if count == 0 {
                    *out_endpoints = std::ptr::null_mut();
                    return 0;
                }
                let ptr = Box::into_raw(Box::new(Vec::with_capacity(count)));
                (*ptr).extend(endpoints.into_iter().map(CRiServiceEndpoint::new));
                *out_endpoints = ptr as *mut CRiServiceEndpoint;
                0
            }
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_service_mesh_get_healthy_count(
    mesh: *mut CRiServiceMesh,
    service_name: *const std::ffi::c_char,
) -> u32 {
    if mesh.is_null() || service_name.is_null() {
        return 0;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return 0,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };
        match rt.block_on(async { (*mesh).inner.discover_service(service_name_str).await }) {
            Ok(endpoints) => endpoints.len() as u32,
            Err(_) => 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_service_mesh_get_stats(
    mesh: *mut CRiServiceMesh,
    out_total_services: *mut usize,
    out_total_endpoints: *mut usize,
    out_healthy_endpoints: *mut usize,
    out_unhealthy_endpoints: *mut usize,
) -> std::ffi::c_int {
    if mesh.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let stats = rt.block_on(async { (*mesh).inner.get_stats().await });
        if !out_total_services.is_null() {
            *out_total_services = stats.total_services;
        }
        if !out_total_endpoints.is_null() {
            *out_total_endpoints = stats.total_endpoints;
        }
        if !out_healthy_endpoints.is_null() {
            *out_healthy_endpoints = stats.healthy_endpoints;
        }
        if !out_unhealthy_endpoints.is_null() {
            *out_unhealthy_endpoints = stats.unhealthy_endpoints;
        }
        0
    }
}

// RiServiceDiscovery C bindings
c_wrapper!(CRiServiceDiscovery, RiServiceDiscovery);

#[no_mangle]
pub extern "C" fn ri_service_discovery_new(enabled: bool) -> *mut CRiServiceDiscovery {
    Box::into_raw(Box::new(CRiServiceDiscovery::new(RiServiceDiscovery::new(enabled))))
}
c_destructor!(ri_service_discovery_free, CRiServiceDiscovery);

#[no_mangle]
pub extern "C" fn ri_service_discovery_register(
    discovery: *mut CRiServiceDiscovery,
    service_name: *const std::ffi::c_char,
    host: *const std::ffi::c_char,
    port: u16,
) -> *mut std::ffi::c_char {
    if discovery.is_null() || service_name.is_null() || host.is_null() {
        return std::ptr::null_mut();
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        let host_str = match std::ffi::CStr::from_ptr(host).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };
        match rt.block_on(async {
            (*discovery).inner.register_service(service_name_str, host_str, port, std::collections::HashMap::default()).await
        }) {
            Ok(instance_id) => match std::ffi::CString::new(instance_id) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_service_discovery_deregister(
    discovery: *mut CRiServiceDiscovery,
    instance_id: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if discovery.is_null() || instance_id.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let instance_id_str = match std::ffi::CStr::from_ptr(instance_id).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*discovery).inner.deregister_service(instance_id_str).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_service_discovery_discover(
    discovery: *mut CRiServiceDiscovery,
    service_name: *const std::ffi::c_char,
    out_instances: *mut *mut CRiServiceInstance,
    out_count: *mut usize,
) -> std::ffi::c_int {
    if discovery.is_null() || service_name.is_null() || out_instances.is_null() || out_count.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*discovery).inner.discover_service(service_name_str).await }) {
            Ok(instances) => {
                let count = instances.len();
                *out_count = count;
                if count == 0 {
                    *out_instances = std::ptr::null_mut();
                    return 0;
                }
                let ptr = Box::into_raw(Box::new(Vec::with_capacity(count)));
                (*ptr).extend(instances.into_iter().map(CRiServiceInstance::new));
                *out_instances = ptr as *mut CRiServiceInstance;
                0
            }
            Err(_) => -4,
        }
    }
}

// RiServiceInstance C bindings
c_wrapper!(CRiServiceInstance, RiServiceInstance);

c_string_getter!(
    ri_service_instance_get_id,
    CRiServiceInstance,
    |inner: &RiServiceInstance| inner.id.clone()
);
c_string_getter!(
    ri_service_instance_get_service_name,
    CRiServiceInstance,
    |inner: &RiServiceInstance| inner.service_name.clone()
);
c_string_getter!(
    ri_service_instance_get_host,
    CRiServiceInstance,
    |inner: &RiServiceInstance| inner.host.clone()
);

#[no_mangle]
pub extern "C" fn ri_service_instance_get_port(obj: *mut CRiServiceInstance) -> u16 {
    if obj.is_null() {
        return 0;
    }
    unsafe { (*obj).inner.port }
}

#[no_mangle]
pub extern "C" fn ri_service_instance_get_status(obj: *mut CRiServiceInstance) -> std::ffi::c_int {
    if obj.is_null() {
        return -1;
    }
    unsafe {
        match (*obj).inner.status {
            crate::service_mesh::RiServiceStatus::Starting => 0,
            crate::service_mesh::RiServiceStatus::Running => 1,
            crate::service_mesh::RiServiceStatus::Stopping => 2,
            crate::service_mesh::RiServiceStatus::Stopped => 3,
            crate::service_mesh::RiServiceStatus::Unhealthy => 4,
        }
    }
}

c_destructor!(ri_service_instance_free, CRiServiceInstance);

// RiHealthChecker C bindings
c_wrapper!(CRiHealthChecker, RiHealthChecker);

#[no_mangle]
pub extern "C" fn ri_health_checker_new(check_interval_secs: u64) -> *mut CRiHealthChecker {
    Box::into_raw(Box::new(CRiHealthChecker::new(RiHealthChecker::new(std::time::Duration::from_secs(check_interval_secs)))))
}
c_destructor!(ri_health_checker_free, CRiHealthChecker);

#[no_mangle]
pub extern "C" fn ri_health_checker_start(
    checker: *mut CRiHealthChecker,
    service_name: *const std::ffi::c_char,
    endpoint: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if checker.is_null() || service_name.is_null() || endpoint.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let endpoint_str = match std::ffi::CStr::from_ptr(endpoint).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*checker).inner.start_health_check(service_name_str, endpoint_str).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_health_checker_stop(
    checker: *mut CRiHealthChecker,
    service_name: *const std::ffi::c_char,
    endpoint: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if checker.is_null() || service_name.is_null() || endpoint.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let endpoint_str = match std::ffi::CStr::from_ptr(endpoint).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*checker).inner.stop_health_check(service_name_str, endpoint_str).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_health_checker_get_summary(
    checker: *mut CRiHealthChecker,
    service_name: *const std::ffi::c_char,
    out_healthy: *mut bool,
    out_success_rate: *mut f64,
    out_avg_response_ms: *mut u64,
) -> std::ffi::c_int {
    if checker.is_null() || service_name.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_name_str = match std::ffi::CStr::from_ptr(service_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*checker).inner.get_service_health_summary(service_name_str).await }) {
            Ok(summary) => {
                if !out_healthy.is_null() {
                    *out_healthy = matches!(summary.overall_status, RiHealthStatus::Healthy);
                }
                if !out_success_rate.is_null() {
                    *out_success_rate = summary.success_rate;
                }
                if !out_avg_response_ms.is_null() {
                    *out_avg_response_ms = summary.average_response_time.as_millis() as u64;
                }
                0
            }
            Err(_) => -4,
        }
    }
}

// RiTrafficManager C bindings
c_wrapper!(CRiTrafficManager, RiTrafficManager);

#[no_mangle]
pub extern "C" fn ri_traffic_manager_new(enabled: bool) -> *mut CRiTrafficManager {
    Box::into_raw(Box::new(CRiTrafficManager::new(RiTrafficManager::new(enabled))))
}
c_destructor!(ri_traffic_manager_free, CRiTrafficManager);

#[no_mangle]
pub extern "C" fn ri_traffic_manager_add_route(
    manager: *mut CRiTrafficManager,
    route: *mut CRiTrafficRoute,
) -> std::ffi::c_int {
    if manager.is_null() || route.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        match rt.block_on(async { (*manager).inner.add_traffic_route((*route).inner.clone()).await }) {
            Ok(_) => 0,
            Err(_) => -3,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_traffic_manager_remove_route(
    manager: *mut CRiTrafficManager,
    source_service: *const std::ffi::c_char,
    route_name: *const std::ffi::c_char,
) -> std::ffi::c_int {
    if manager.is_null() || source_service.is_null() || route_name.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let source_service_str = match std::ffi::CStr::from_ptr(source_service).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let route_name_str = match std::ffi::CStr::from_ptr(route_name).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        match rt.block_on(async { (*manager).inner.remove_traffic_route(source_service_str, route_name_str).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_traffic_manager_set_circuit_breaker(
    manager: *mut CRiTrafficManager,
    service: *const std::ffi::c_char,
    consecutive_errors: u32,
    max_ejection_percent: f64,
) -> std::ffi::c_int {
    if manager.is_null() || service.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_str = match std::ffi::CStr::from_ptr(service).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let config = crate::service_mesh::traffic_management::RiCircuitBreakerConfig {
            consecutive_errors,
            interval: std::time::Duration::from_secs(10),
            base_ejection_time: std::time::Duration::from_secs(30),
            max_ejection_percent,
        };
        match rt.block_on(async { (*manager).inner.set_circuit_breaker_config(service_str, config).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_traffic_manager_set_rate_limit(
    manager: *mut CRiTrafficManager,
    service: *const std::ffi::c_char,
    requests_per_second: u32,
    burst_size: u32,
) -> std::ffi::c_int {
    if manager.is_null() || service.is_null() {
        return -1;
    }
    let rt = match tokio::runtime::Runtime::new() {
        Ok(rt) => rt,
        Err(_) => return -2,
    };
    unsafe {
        let service_str = match std::ffi::CStr::from_ptr(service).to_str() {
            Ok(s) => s,
            Err(_) => return -3,
        };
        let config = crate::service_mesh::traffic_management::RiRateLimitConfig {
            requests_per_second,
            burst_size,
            window: std::time::Duration::from_secs(1),
        };
        match rt.block_on(async { (*manager).inner.set_rate_limit_config(service_str, config).await }) {
            Ok(_) => 0,
            Err(_) => -4,
        }
    }
}

// RiTrafficRoute C bindings
c_wrapper!(CRiTrafficRoute, RiTrafficRoute);

#[no_mangle]
pub extern "C" fn ri_traffic_route_new(
    name: *const std::ffi::c_char,
    source_service: *const std::ffi::c_char,
    destination_service: *const std::ffi::c_char,
) -> *mut CRiTrafficRoute {
    if name.is_null() || source_service.is_null() || destination_service.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let source_str = match std::ffi::CStr::from_ptr(source_service).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let dest_str = match std::ffi::CStr::from_ptr(destination_service).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };
        let route = RiTrafficRoute {
            name: name_str,
            source_service: source_str,
            destination_service: dest_str,
            match_criteria: crate::service_mesh::RiMatchCriteria {
                path_prefix: None,
                headers: std::collections::HashMap::default(),
                method: None,
                query_parameters: std::collections::HashMap::default(),
            },
            route_action: crate::service_mesh::RiRouteAction::Route(vec![]),
            retry_policy: None,
            timeout: None,
            fault_injection: None,
        };
        Box::into_raw(Box::new(CRiTrafficRoute::new(route)))
    }
}
c_destructor!(ri_traffic_route_free, CRiTrafficRoute);

c_string_getter!(
    ri_traffic_route_get_name,
    CRiTrafficRoute,
    |inner: &RiTrafficRoute| inner.name.clone()
);
c_string_getter!(
    ri_traffic_route_get_source_service,
    CRiTrafficRoute,
    |inner: &RiTrafficRoute| inner.source_service.clone()
);
c_string_getter!(
    ri_traffic_route_get_destination_service,
    CRiTrafficRoute,
    |inner: &RiTrafficRoute| inner.destination_service.clone()
);
