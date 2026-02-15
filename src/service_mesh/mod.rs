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

//! # Service Mesh Module
//! 
//! This module provides a comprehensive service mesh implementation for DMSC, offering service discovery,
//! health checking, traffic management, load balancing, and circuit breaking capabilities for distributed systems.
//! 
//! ## Key Components
//! 
//! - **DMSCServiceMesh**: Main service mesh struct implementing the DMSCModule trait
//! - **DMSCServiceMeshConfig**: Configuration for service mesh behavior
//! - **DMSCServiceEndpoint**: Service endpoint representation
//! - **DMSCServiceHealthStatus**: Enum defining service health statuses
//! - **DMSCServiceDiscovery**: Service discovery component
//! - **DMSCServiceInstance**: Service instance representation
//! - **DMSCServiceStatus**: Service status enum
//! - **DMSCHealthChecker**: Health checking component
//! - **DMSCHealthCheckResult**: Health check result structure
//! - **DMSCHealthSummary**: Health summary structure
//! - **DMSCHealthStatus**: Health status enum
//! - **DMSCTrafficManager**: Traffic management component
//! - **DMSCTrafficRoute**: Traffic route definition
//! - **DMSCMatchCriteria**: Match criteria for traffic routing
//! - **DMSCRouteAction**: Route action for traffic routing
//! - **DMSCCircuitBreaker**: Circuit breaker for preventing cascading failures
//! - **DMSCLoadBalancer**: Load balancer for distributing traffic across services
//! - **DMSCLoadBalancerStrategy**: Load balancing strategies
//! 
//! ## Design Principles
//! 
//! 1. **Service Discovery**: Automatic discovery of services and their endpoints
//! 2. **Health Monitoring**: Continuous health checks for service endpoints
//! 3. **Traffic Management**: Intelligent routing and load balancing of traffic
//! 4. **Resilience**: Circuit breaking and retry mechanisms for service resilience
//! 5. **Configurable**: Highly configurable service mesh behavior
//! 6. **Async-First**: All service mesh operations are asynchronous
//! 7. **Modular Design**: Separate components for service discovery, health checking, and traffic management
//! 8. **Service Module Integration**: Implements DMSCModule trait for seamless integration into DMSC
//! 9. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 10. **Critical Component**: Marked as critical for the system's operation
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! use dmsc::service_mesh::{DMSCServiceMesh, DMSCServiceMeshConfig};
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create service mesh configuration
//!     let mesh_config = DMSCServiceMeshConfig::default();
//!     
//!     // Create service mesh instance
//!     let service_mesh = DMSCServiceMesh::new(mesh_config)?;
//!     
//!     // Register services
//!     service_mesh.register_service("user-service", "http://user-service:8080", 100).await?;
//!     service_mesh.register_service("order-service", "http://order-service:8080", 100).await?;
//!     service_mesh.register_service("payment-service", "http://payment-service:8080", 100).await?;
//!     
//!     // Discover services
//!     let user_service_endpoints = service_mesh.discover_service("user-service").await?;
//!     println!("User service endpoints: {:?}", user_service_endpoints);
//!     
//!     // Call a service
//!     let request_data = r#"{ "user_id": "123" }"#.as_bytes().to_vec();
//!     let response = service_mesh.call_service("user-service", request_data).await?;
//!     println!("Service response: {}", String::from_utf8_lossy(&response));
//!     
//!     // Get service mesh components for advanced configuration
//!     let health_checker = service_mesh.get_health_checker();
//!     let traffic_manager = service_mesh.get_traffic_manager();
//!     let circuit_breaker = service_mesh.get_circuit_breaker();
//!     let load_balancer = service_mesh.get_load_balancer();
//!     
//!     // Example: Configure traffic manager
//!     // traffic_manager.add_route(route).await?;
//!     
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

use crate::core::{DMSCModule, DMSCResult, DMSCError};
use crate::gateway::{DMSCCircuitBreaker, DMSCCircuitBreakerConfig, DMSCLoadBalancer, DMSCLoadBalancerStrategy};
use crate::gateway::load_balancer::DMSCBackendServer;
use crate::observability::{DMSCTracer, DMSCSpanKind, DMSCSpanStatus};

pub mod service_discovery;
pub mod health_check;
pub mod traffic_management;

pub use service_discovery::{DMSCServiceDiscovery, DMSCServiceInstance, DMSCServiceStatus};
pub use health_check::{DMSCHealthChecker, DMSCHealthCheckResult, DMSCHealthSummary, DMSCHealthStatus};    
pub use traffic_management::{DMSCTrafficRoute, DMSCMatchCriteria, DMSCRouteAction, DMSCWeightedDestination, DMSCTrafficManager};

/// Configuration for the service mesh.
/// 
/// This struct defines the configuration options for the service mesh, including service discovery,
/// health checking, traffic management, circuit breaking, and load balancing settings.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCServiceMeshConfig {
    /// Whether to enable service discovery
    pub enable_service_discovery: bool,
    /// Whether to enable health checking
    pub enable_health_check: bool,
    /// Whether to enable traffic management
    pub enable_traffic_management: bool,
    /// Interval between health checks
    pub health_check_interval: Duration,
    /// Configuration for circuit breakers
    pub circuit_breaker_config: DMSCCircuitBreakerConfig,
    /// Load balancing strategy to use
    pub load_balancer_strategy: DMSCLoadBalancerStrategy,
    /// Maximum number of retry attempts for failed requests
    pub max_retry_attempts: u32,
    /// Timeout for retry attempts
    pub retry_timeout: Duration,
}

impl Default for DMSCServiceMeshConfig {
    /// Returns the default configuration for the service mesh.
    /// 
    /// Default values:
    /// - enable_service_discovery: true
    /// - enable_health_check: true
    /// - enable_traffic_management: true
    /// - health_check_interval: 30 seconds
    /// - circuit_breaker_config: Default circuit breaker config
    /// - load_balancer_strategy: RoundRobin
    /// - max_retry_attempts: 3
    /// - retry_timeout: 5 seconds
    fn default() -> Self {
        Self {
            enable_service_discovery: true,
            enable_health_check: true,
            enable_traffic_management: true,
            health_check_interval: Duration::from_secs(30),
            circuit_breaker_config: DMSCCircuitBreakerConfig::default(),
            load_balancer_strategy: DMSCLoadBalancerStrategy::RoundRobin,
            max_retry_attempts: 3,
            retry_timeout: Duration::from_secs(5),
        }
    }
}

/// Service endpoint representation.
/// 
/// This struct represents a service endpoint with its name, URL, weight, metadata, health status,
/// and last health check time.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSCServiceEndpoint {
    /// Name of the service
    pub service_name: String,
    /// Endpoint URL
    pub endpoint: String,
    /// Weight for load balancing
    pub weight: u32,
    /// Metadata associated with the endpoint
    pub metadata: HashMap<String, String>,
    /// Health status of the endpoint
    pub health_status: DMSCServiceHealthStatus,
    /// Time of the last health check
    pub last_health_check: SystemTime,
}

/// Service health status enum.
/// 
/// This enum defines the possible health statuses for a service endpoint.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, PartialEq)]
pub enum DMSCServiceHealthStatus {
    /// Service is healthy and available
    Healthy,
    /// Service is unhealthy and unavailable
    Unhealthy,
    /// Service health status is unknown
    Unknown,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCServiceEndpoint {
    #[new]
    fn py_new(
        service_name: String,
        endpoint: String,
        weight: u32,
    ) -> Self {
        Self {
            service_name,
            endpoint,
            weight,
            metadata: HashMap::new(),
            health_status: DMSCServiceHealthStatus::Unknown,
            last_health_check: SystemTime::now(),
        }
    }

    #[getter]
    fn service_name(&self) -> &str {
        &self.service_name
    }

    #[getter]
    fn endpoint(&self) -> &str {
        &self.endpoint
    }

    #[getter]
    fn weight(&self) -> u32 {
        self.weight
    }

    #[getter]
    fn health_status(&self) -> DMSCServiceHealthStatus {
        self.health_status.clone()
    }
}

/// Service discovery cache entry
/// 
/// This struct represents a cached entry for service discovery results.
#[derive(Debug, Clone)]
struct ServiceDiscoveryCacheEntry {
    /// Discovered service endpoints
    endpoints: Vec<DMSCServiceEndpoint>,
    /// Cache entry expiration time
    expiration: SystemTime,
}

/// Service mesh statistics.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCServiceMeshStats {
    /// Total number of registered services
    pub total_services: usize,
    /// Total number of registered endpoints
    pub total_endpoints: usize,
    /// Number of healthy endpoints
    pub healthy_endpoints: usize,
    /// Number of unhealthy endpoints
    pub unhealthy_endpoints: usize,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCServiceMeshStats {
    #[new]
    fn py_new() -> Self {
        Self {
            total_services: 0,
            total_endpoints: 0,
            healthy_endpoints: 0,
            unhealthy_endpoints: 0,
        }
    }

    #[getter]
    fn total_services(&self) -> usize {
        self.total_services
    }

    #[getter]
    fn total_endpoints(&self) -> usize {
        self.total_endpoints
    }

    #[getter]
    fn healthy_endpoints(&self) -> usize {
        self.healthy_endpoints
    }

    #[getter]
    fn unhealthy_endpoints(&self) -> usize {
        self.unhealthy_endpoints
    }
}

/// Main service mesh struct implementing the DMSCModule trait.
/// 
/// This struct provides comprehensive service mesh functionality, including service discovery,
/// health checking, traffic management, load balancing, and circuit breaking.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCServiceMesh {
    config: DMSCServiceMeshConfig,
    service_discovery: Arc<DMSCServiceDiscovery>,
    health_checker: Arc<DMSCHealthChecker>,
    traffic_manager: Arc<DMSCTrafficManager>,
    circuit_breaker: Arc<DMSCCircuitBreaker>,
    load_balancer: Arc<DMSCLoadBalancer>,
    services: Arc<RwLock<HashMap<String, Vec<DMSCServiceEndpoint>>>>,
    discovery_cache: Arc<RwLock<HashMap<String, ServiceDiscoveryCacheEntry>>>,
    cache_expiration: Duration,
    tracer: Option<Arc<DMSCTracer>>,
}

impl DMSCServiceMesh {
    pub fn new(config: DMSCServiceMeshConfig) -> DMSCResult<Self> {
        let service_discovery = Arc::new(DMSCServiceDiscovery::new(config.enable_service_discovery));
        let health_checker = Arc::new(DMSCHealthChecker::new(config.health_check_interval));
        let traffic_manager = Arc::new(DMSCTrafficManager::new(config.enable_traffic_management));
        let circuit_breaker = Arc::new(DMSCCircuitBreaker::new(config.circuit_breaker_config.clone()));
        let load_balancer = Arc::new(DMSCLoadBalancer::new(config.load_balancer_strategy.clone()));
        
        Ok(Self {
            config,
            service_discovery,
            health_checker,
            traffic_manager,
            circuit_breaker,
            load_balancer,
            services: Arc::new(RwLock::new(HashMap::new())),
            discovery_cache: Arc::new(RwLock::new(HashMap::new())),
            cache_expiration: Duration::from_secs(30),
            tracer: None,
        })
    }
    
    pub fn with_tracer(mut self, tracer: Arc<DMSCTracer>) -> Self {
        self.tracer = Some(tracer.clone());
        let mut traffic_manager = DMSCTrafficManager::new(self.config.enable_traffic_management);
        traffic_manager.set_tracer(tracer);
        self.traffic_manager = Arc::new(traffic_manager);
        self
    }
    
    pub fn set_tracer(&mut self, tracer: Arc<DMSCTracer>) {
        self.tracer = Some(tracer.clone());
        let mut traffic_manager = DMSCTrafficManager::new(self.config.enable_traffic_management);
        traffic_manager.set_tracer(tracer);
        self.traffic_manager = Arc::new(traffic_manager);
    }

    /// Registers a service endpoint with the service mesh.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service
    /// - `endpoint`: The endpoint URL of the service
    /// - `weight`: The weight of the endpoint for load balancing
    /// - `metadata`: Optional metadata associated with the service
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn register_service(&self, service_name: &str, endpoint: &str, weight: u32, metadata: Option<HashMap<String, String>>) -> DMSCResult<()> {
        if service_name.is_empty() {
            return Err(DMSCError::ServiceMesh("Service name cannot be empty".to_string()));
        }
        if endpoint.is_empty() {
            return Err(DMSCError::ServiceMesh("Endpoint cannot be empty".to_string()));
        }
        if weight == 0 {
            return Err(DMSCError::ServiceMesh("Weight must be greater than zero".to_string()));
        }

        let service_endpoint = DMSCServiceEndpoint {
            service_name: service_name.to_string(),
            endpoint: endpoint.to_string(),
            weight,
            metadata: metadata.unwrap_or_default(),
            health_status: DMSCServiceHealthStatus::Unknown,
            last_health_check: SystemTime::now(),
        };

        let mut services = self.services.write().await;
        services.entry(service_name.to_string())
            .or_insert_with(Vec::new)
            .push(service_endpoint);

        if self.config.enable_health_check {
            self.health_checker.start_health_check(service_name, endpoint).await?;
        }

        Ok(())
    }
    
    /// Registers a service with full metadata including version information.
    pub async fn register_versioned_service(&self, service_name: &str, version: &str, endpoint: &str, weight: u32, metadata: Option<HashMap<String, String>>) -> DMSCResult<()> {
        let mut enriched_metadata = metadata.unwrap_or_default();
        enriched_metadata.insert("version".to_string(), version.to_string());
        
        self.register_service(service_name, endpoint, weight, Some(enriched_metadata)).await
    }
    
    /// Unregisters a service endpoint from the service mesh.
    pub async fn unregister_service(&self, service_name: &str, endpoint: &str) -> DMSCResult<()> {
        let mut services = self.services.write().await;
        
        if let Some(endpoints) = services.get_mut(service_name) {
            endpoints.retain(|ep| ep.endpoint != endpoint);
            
            if endpoints.is_empty() {
                services.remove(service_name);
            }
            
            if self.config.enable_health_check {
                self.health_checker.stop_health_check(service_name, endpoint).await?;
            }
        }
        
        Ok(())
    }
    
    /// Gets all registered endpoints for a service regardless of health status.
    pub async fn get_all_endpoints(&self, service_name: &str) -> DMSCResult<Vec<DMSCServiceEndpoint>> {
        let services = self.services.read().await;
        
        services.get(service_name)
            .cloned()
            .ok_or_else(|| DMSCError::ServiceMesh(format!("Service '{service_name}' not found")))
    }
    
    /// Gets service mesh statistics.
    pub async fn get_stats(&self) -> DMSCServiceMeshStats {
        let services = self.services.read().await;
        let healthy_count = services.values()
            .flat_map(|endpoints| endpoints.iter())
            .filter(|ep| ep.health_status == DMSCServiceHealthStatus::Healthy)
            .count();
        
        DMSCServiceMeshStats {
            total_services: services.len(),
            total_endpoints: services.values().map(|v| v.len()).sum(),
            healthy_endpoints: healthy_count,
            unhealthy_endpoints: services.values()
                .flat_map(|endpoints| endpoints.iter())
                .filter(|ep| ep.health_status == DMSCServiceHealthStatus::Unhealthy)
                .count(),
        }
    }

    /// Discovers healthy endpoints for a service.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service to discover
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Vec<DMSCServiceEndpoint>>` containing the healthy endpoints for the service
    pub async fn discover_service(&self, service_name: &str) -> DMSCResult<Vec<DMSCServiceEndpoint>> {
        if !self.config.enable_service_discovery {
            return Err(DMSCError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        // Check cache first
        {
            let cache = self.discovery_cache.read().await;
            if let Some(entry) = cache.get(service_name) {
                if entry.expiration > SystemTime::now() {
                    // Cache is still valid, return cached endpoints
                    return Ok(entry.endpoints.clone());
                }
            }
        }

        // Cache miss or expired, perform regular service discovery
        let services = self.services.read().await;
        let endpoints = services.get(service_name)
            .ok_or_else(|| DMSCError::ServiceMesh(format!("Service '{service_name}' not found")))?
            .clone();

        let healthy_endpoints: Vec<DMSCServiceEndpoint> = endpoints
            .into_iter()
            .filter(|ep| ep.health_status == DMSCServiceHealthStatus::Healthy)
            .collect();

        if healthy_endpoints.is_empty() {
            return Err(DMSCError::ServiceMesh(format!("No healthy endpoints for service '{service_name}'")));
        }

        // Cache the discovered endpoints
        let expiration = SystemTime::now() + self.cache_expiration;
        let cache_entry = ServiceDiscoveryCacheEntry {
            endpoints: healthy_endpoints.clone(),
            expiration,
        };
        
        let mut cache = self.discovery_cache.write().await;
        cache.insert(service_name.to_string(), cache_entry);

        Ok(healthy_endpoints)
    }

    /// Calls a service with the given request data.
    /// 
    /// This method performs the following steps:
    /// 1. Discovers healthy endpoints for the service
    /// 2. Selects a server using the load balancer
    /// 3. Checks the circuit breaker status
    /// 4. Executes the service call with retry logic
    /// 5. Records success/failure with the circuit breaker
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service to call
    /// - `request_data`: The request data to send to the service
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Vec<u8>>` containing the response from the service
    pub async fn call_service(&self, service_name: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        let span_id = if let Some(tracer) = &self.tracer {
            let span_id = tracer.start_span_from_context(
                format!("call_service:{}", service_name),
                DMSCSpanKind::Client,
            );
            if let Some(ref sid) = span_id {
                let _ = tracer.span_mut(sid, |span| {
                    span.set_attribute("service_name".to_string(), service_name.to_string());
                    span.set_attribute("request_size".to_string(), request_data.len().to_string());
                });
            }
            span_id
        } else {
            None
        };

        let result = self.call_service_internal(service_name, request_data).await;

        if let (Some(tracer), Some(sid)) = (&self.tracer, span_id) {
            let status = match &result {
                Ok(_) => DMSCSpanStatus::Ok,
                Err(e) => DMSCSpanStatus::Error(e.to_string()),
            };
            let _ = tracer.end_span(&sid, status);
        }

        result
    }
    
    async fn call_service_internal(&self, service_name: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        let endpoints = self.discover_service(service_name).await?;
        
        let mut existing_servers = self.load_balancer.get_healthy_servers().await;
        existing_servers.retain(|s| !s.id.starts_with(&format!("{service_name}-")));
        
        for ep in &endpoints {
            if ep.health_status == DMSCServiceHealthStatus::Healthy {
                let server = DMSCBackendServer {
                    id: format!("{}-{}", service_name, ep.endpoint),
                    url: ep.endpoint.clone(),
                    weight: ep.weight,
                    max_connections: 100,
                    health_check_path: "/health".to_string(),
                    is_healthy: true,
                };
                self.load_balancer.add_server(server).await;
            }
        }

        let selected_server = match self.load_balancer.select_server(None).await {
            Ok(server) => server,
            Err(_) => return Err(DMSCError::ServiceMesh("No available backend server".to_string())),
        };

        if !self.circuit_breaker.allow_request() {
            return Err(DMSCError::ServiceMesh("Circuit breaker is open".to_string()));
        }

        let result = self.execute_service_call(&selected_server.url, request_data.clone()).await;

        match &result {
            Ok(_) => {
                self.circuit_breaker.record_success();
            }
            Err(_) => {
                self.circuit_breaker.record_failure();
            }
        }

        result
    }

    /// Executes a service call with retry logic.
    /// 
    /// This method attempts to call a service endpoint with exponential backoff retry logic.
    /// 
    /// # Parameters
    /// 
    /// - `endpoint`: The endpoint URL to call
    /// - `request_data`: The request data to send
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Vec<u8>>` containing the response from the service
    async fn execute_service_call(&self, endpoint: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        let mut last_error = None;
        
        for attempt in 0..self.config.max_retry_attempts {
            match self.traffic_manager.route_request(endpoint, request_data.clone()).await {
                Ok(response) => return Ok(response),
                Err(_e) => {
                    let sanitized_error = DMSCError::ServiceMesh(format!("Retry attempt {} failed", attempt + 1));
                    last_error = Some(sanitized_error);
                    if attempt < self.config.max_retry_attempts - 1 {
                        tokio::time::sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| DMSCError::ServiceMesh("All retry attempts failed".to_string())))
    }

    /// Updates the health status of a service endpoint.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service
    /// - `endpoint`: The endpoint URL
    /// - `is_healthy`: Whether the endpoint is healthy
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn update_service_health(&self, service_name: &str, endpoint: &str, is_healthy: bool) -> DMSCResult<()> {
        let mut services = self.services.write().await;
        if let Some(endpoints) = services.get_mut(service_name) {
            if let Some(service_ep) = endpoints.iter_mut().find(|ep| ep.endpoint == endpoint) {
                service_ep.health_status = if is_healthy {
                    DMSCServiceHealthStatus::Healthy
                } else {
                    DMSCServiceHealthStatus::Unhealthy
                };
                service_ep.last_health_check = SystemTime::now();
            }
        }
        Ok(())
    }

    /// Returns a reference to the circuit breaker.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCCircuitBreaker` instance
    pub fn get_circuit_breaker(&self) -> &DMSCCircuitBreaker {
        &self.circuit_breaker
    }

    /// Returns a reference to the load balancer.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCLoadBalancer` instance
    pub fn get_load_balancer(&self) -> &DMSCLoadBalancer {
        &self.load_balancer
    }

    /// Returns a reference to the health checker.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCHealthChecker` instance
    pub fn get_health_checker(&self) -> &DMSCHealthChecker {
        &self.health_checker
    }

    /// Returns a reference to the traffic manager.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCTrafficManager` instance
    pub fn get_traffic_manager(&self) -> &DMSCTrafficManager {
        &self.traffic_manager
    }

    /// Returns a reference to the service discovery component.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCServiceDiscovery` instance
    pub fn get_service_discovery(&self) -> &DMSCServiceDiscovery {
        &self.service_discovery
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCServiceMesh
#[pyo3::prelude::pymethods]
impl DMSCServiceMesh {
    #[new]
    fn py_new(config: DMSCServiceMeshConfig) -> PyResult<Self> {
        match Self::new(config) {
            Ok(mesh) => Ok(mesh),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create service mesh: {e}"))),
        }
    }
    
    /// Register a service from Python
    #[pyo3(name = "register_service")]
    fn register_service_impl(&self, service_name: String, endpoint: String, weight: u32) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.register_service(&service_name, &endpoint, weight, None)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to register service: {e}")))
        })
    }
    
    /// Discover services from Python
    #[pyo3(name = "discover_service")]
    fn discover_service_impl(&self, service_name: String) -> PyResult<Vec<DMSCServiceEndpoint>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.discover_service(&service_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to discover service: {e}")))
        })
    }
    
    /// Update service health from Python
    #[pyo3(name = "update_service_health")]
    fn update_service_health_impl(&self, service_name: String, endpoint: String, is_healthy: bool) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.update_service_health(&service_name, &endpoint, is_healthy)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to update health: {e}")))
        })
    }
    
    /// Get the service mesh configuration
    fn get_config(&self) -> DMSCServiceMeshConfig {
        self.config.clone()
    }
}

#[async_trait]
impl DMSCModule for DMSCServiceMesh {
    /// Returns the name of the service mesh module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.ServiceMesh"
    }

    /// Indicates whether the service mesh module is critical.
    /// 
    /// The service mesh is marked as critical because it's essential for the operation
    /// of distributed services in the system.
    /// 
    /// # Returns
    /// 
    /// `true` since service mesh is critical
    fn is_critical(&self) -> bool {
        true
    }

    /// Starts the service mesh module.
    /// 
    /// This method starts background tasks for service discovery, health checking,
    /// and traffic management if they are enabled.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: Service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn start(&mut self, _ctx: &mut crate::core::DMSCServiceContext) -> DMSCResult<()> {
        if self.config.enable_health_check {
            self.health_checker.start_background_tasks().await?;
        }
        
        if self.config.enable_service_discovery {
            self.service_discovery.start_background_tasks().await?;
        }
        
        if self.config.enable_traffic_management {
            self.traffic_manager.start_background_tasks().await?;
        }

        Ok(())
    }

    /// Shuts down the service mesh module.
    /// 
    /// This method stops background tasks for service discovery, health checking,
    /// and traffic management if they are enabled.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: Service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn shutdown(&mut self, _ctx: &mut crate::core::DMSCServiceContext) -> DMSCResult<()> {
        if self.config.enable_health_check {
            self.health_checker.stop_background_tasks().await?;
        }
        
        if self.config.enable_service_discovery {
            self.service_discovery.stop_background_tasks().await?;
        }
        
        if self.config.enable_traffic_management {
            self.traffic_manager.stop_background_tasks().await?;
        }

        Ok(())
    }
}
