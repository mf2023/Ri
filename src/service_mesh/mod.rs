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

//! # Service Mesh Module
//! 
//! This module provides a comprehensive service mesh implementation for DMS, offering service discovery,
//! health checking, traffic management, load balancing, and circuit breaking capabilities for distributed systems.
//! 
//! ## Key Components
//! 
//! - **DMSServiceMesh**: Main service mesh struct implementing the DMSModule trait
//! - **DMSServiceMeshConfig**: Configuration for service mesh behavior
//! - **DMSServiceEndpoint**: Service endpoint representation
//! - **DMSServiceHealthStatus**: Enum defining service health statuses
//! - **DMSServiceDiscovery**: Service discovery component
//! - **DMSServiceInstance**: Service instance representation
//! - **DMSServiceStatus**: Service status enum
//! - **DMSHealthChecker**: Health checking component
//! - **DMSHealthCheckResult**: Health check result structure
//! - **DMSHealthSummary**: Health summary structure
//! - **DMSHealthStatus**: Health status enum
//! - **DMSTrafficManager**: Traffic management component
//! - **DMSTrafficRoute**: Traffic route definition
//! - **DMSMatchCriteria**: Match criteria for traffic routing
//! - **DMSRouteAction**: Route action for traffic routing
//! - **DMSCircuitBreaker**: Circuit breaker for preventing cascading failures
//! - **DMSLoadBalancer**: Load balancer for distributing traffic across services
//! - **DMSLoadBalancerStrategy**: Load balancing strategies
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
//! 8. **Service Module Integration**: Implements DMSModule trait for seamless integration into DMS
//! 9. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 10. **Critical Component**: Marked as critical for the system's operation
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use dms::service_mesh::{DMSServiceMesh, DMSServiceMeshConfig};
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create service mesh configuration
//!     let mesh_config = DMSServiceMeshConfig::default();
//!     
//!     // Create service mesh instance
//!     let service_mesh = DMSServiceMesh::new(mesh_config)?;
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

use crate::core::{DMSModule, DMSResult, DMSError};
use crate::gateway::circuit_breaker::{DMSCircuitBreaker, DMSCircuitBreakerConfig};
use crate::gateway::load_balancer::{DMSLoadBalancer, DMSLoadBalancerStrategy, DMSBackendServer};

pub mod service_discovery;
pub mod health_check;
pub mod traffic_management;

use service_discovery::DMSServiceDiscovery;
use health_check::DMSHealthChecker;
use traffic_management::DMSTrafficManager;

pub use service_discovery::{DMSServiceInstance, DMSServiceStatus};
pub use health_check::{DMSHealthCheckResult, DMSHealthSummary, DMSHealthStatus};
pub use traffic_management::{DMSTrafficRoute, DMSMatchCriteria, DMSRouteAction};

/// Configuration for the service mesh.
/// 
/// This struct defines the configuration options for the service mesh, including service discovery,
/// health checking, traffic management, circuit breaking, and load balancing settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSServiceMeshConfig {
    /// Whether to enable service discovery
    pub enable_service_discovery: bool,
    /// Whether to enable health checking
    pub enable_health_check: bool,
    /// Whether to enable traffic management
    pub enable_traffic_management: bool,
    /// Interval between health checks
    pub health_check_interval: Duration,
    /// Configuration for circuit breakers
    pub circuit_breaker_config: DMSCircuitBreakerConfig,
    /// Load balancing strategy to use
    pub load_balancer_strategy: DMSLoadBalancerStrategy,
    /// Maximum number of retry attempts for failed requests
    pub max_retry_attempts: u32,
    /// Timeout for retry attempts
    pub retry_timeout: Duration,
}

impl Default for DMSServiceMeshConfig {
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
            circuit_breaker_config: DMSCircuitBreakerConfig::default(),
            load_balancer_strategy: DMSLoadBalancerStrategy::RoundRobin,
            max_retry_attempts: 3,
            retry_timeout: Duration::from_secs(5),
        }
    }
}

/// Service endpoint representation.
/// 
/// This struct represents a service endpoint with its name, URL, weight, metadata, health status,
/// and last health check time.
#[derive(Debug, Clone)]
pub struct DMSServiceEndpoint {
    /// Name of the service
    pub service_name: String,
    /// Endpoint URL
    pub endpoint: String,
    /// Weight for load balancing
    pub weight: u32,
    /// Metadata associated with the endpoint
    pub metadata: HashMap<String, String>,
    /// Health status of the endpoint
    pub health_status: DMSServiceHealthStatus,
    /// Time of the last health check
    pub last_health_check: SystemTime,
}

/// Service health status enum.
/// 
/// This enum defines the possible health statuses for a service endpoint.
#[derive(Debug, Clone, PartialEq)]
pub enum DMSServiceHealthStatus {
    /// Service is healthy and available
    Healthy,
    /// Service is unhealthy and unavailable
    Unhealthy,
    /// Service health status is unknown
    Unknown,
}

/// Main service mesh struct implementing the DMSModule trait.
/// 
/// This struct provides comprehensive service mesh functionality, including service discovery,
/// health checking, traffic management, load balancing, and circuit breaking.
pub struct DMSServiceMesh {
    /// Service mesh configuration
    config: DMSServiceMeshConfig,
    /// Service discovery component
    service_discovery: Arc<DMSServiceDiscovery>,
    /// Health checking component
    health_checker: Arc<DMSHealthChecker>,
    /// Traffic management component
    traffic_manager: Arc<DMSTrafficManager>,
    /// Circuit breaker for preventing cascading failures
    circuit_breaker: Arc<DMSCircuitBreaker>,
    /// Load balancer for distributing traffic
    load_balancer: Arc<DMSLoadBalancer>,
    /// Map of service names to their endpoints, protected by a RwLock for thread-safe access
    services: Arc<RwLock<HashMap<String, Vec<DMSServiceEndpoint>>>>,
}

impl DMSServiceMesh {
    /// Creates a new service mesh instance with the given configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The service mesh configuration to use
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Self>` containing the new service mesh instance
    pub fn new(config: DMSServiceMeshConfig) -> DMSResult<Self> {
        let service_discovery = Arc::new(DMSServiceDiscovery::_Fnew(config.enable_service_discovery));
        let health_checker = Arc::new(DMSHealthChecker::_Fnew(config.health_check_interval));
        let traffic_manager = Arc::new(DMSTrafficManager::_Fnew(config.enable_traffic_management));
        let circuit_breaker = Arc::new(DMSCircuitBreaker::_Fnew(config.circuit_breaker_config.clone()));
        let load_balancer = Arc::new(DMSLoadBalancer::_Fnew(config.load_balancer_strategy.clone()));
        
        Ok(Self {
            config,
            service_discovery,
            health_checker,
            traffic_manager,
            circuit_breaker,
            load_balancer,
            services: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Registers a service endpoint with the service mesh.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service
    /// - `endpoint`: The endpoint URL of the service
    /// - `weight`: The weight of the endpoint for load balancing
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub async fn register_service(&self, service_name: &str, endpoint: &str, weight: u32) -> DMSResult<()> {
        let service_endpoint = DMSServiceEndpoint {
            service_name: service_name.to_string(),
            endpoint: endpoint.to_string(),
            weight,
            metadata: HashMap::new(),
            health_status: DMSServiceHealthStatus::Unknown,
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

    /// Discovers healthy endpoints for a service.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: The name of the service to discover
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Vec<DMSServiceEndpoint>>` containing the healthy endpoints for the service
    pub async fn discover_service(&self, service_name: &str) -> DMSResult<Vec<DMSServiceEndpoint>> {
        if !self.config.enable_service_discovery {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        let services = self.services.read().await;
        let endpoints = services.get(service_name)
            .ok_or_else(|| DMSError::ServiceMesh(format!("Service '{service_name}' not found")))?
            .clone();

        let healthy_endpoints: Vec<DMSServiceEndpoint> = endpoints
            .into_iter()
            .filter(|ep| ep.health_status == DMSServiceHealthStatus::Healthy)
            .collect();

        if healthy_endpoints.is_empty() {
            return Err(DMSError::ServiceMesh(format!("No healthy endpoints for service '{service_name}'")));
        }

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
    /// A `DMSResult<Vec<u8>>` containing the response from the service
    pub async fn call_service(&self, service_name: &str, request_data: Vec<u8>) -> DMSResult<Vec<u8>> {
        let endpoints = self.discover_service(service_name).await?;
        
        let _backend_servers: Vec<DMSBackendServer> = endpoints
            .iter()
            .map(|ep| DMSBackendServer {
                id: format!("{}-{}", service_name, ep.endpoint),
                url: ep.endpoint.clone(),
                weight: ep.weight,
                max_connections: 100,
                health_check_path: "/health".to_string(),
                is_healthy: ep.health_status == DMSServiceHealthStatus::Healthy,
            })
            .collect();

        let selected_server = match self.load_balancer._Fselect_server(None).await {
            Ok(server) => server,
            Err(_) => return Err(DMSError::ServiceMesh("No available backend server".to_string())),
        };

        if !self.circuit_breaker._Fallow_request().await {
            return Err(DMSError::ServiceMesh("Circuit breaker is open".to_string()));
        }

        let result = self.execute_service_call(&selected_server.url, request_data.clone()).await;

        match &result {
            Ok(_) => {
                self.circuit_breaker._Frecord_success().await;
            }
            Err(_) => {
                self.circuit_breaker._Frecord_failure().await;
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
    /// A `DMSResult<Vec<u8>>` containing the response from the service
    async fn execute_service_call(&self, endpoint: &str, request_data: Vec<u8>) -> DMSResult<Vec<u8>> {
        let mut last_error = None;
        
        for attempt in 0..self.config.max_retry_attempts {
            match self.traffic_manager.route_request(endpoint, request_data.clone()).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    last_error = Some(e);
                    if attempt < self.config.max_retry_attempts - 1 {
                        tokio::time::sleep(Duration::from_millis(100 * (attempt + 1) as u64)).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| DMSError::ServiceMesh("All retry attempts failed".to_string())))
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
    /// A `DMSResult<()>` indicating success or failure
    pub async fn update_service_health(&self, service_name: &str, endpoint: &str, is_healthy: bool) -> DMSResult<()> {
        let mut services = self.services.write().await;
        if let Some(endpoints) = services.get_mut(service_name) {
            if let Some(service_ep) = endpoints.iter_mut().find(|ep| ep.endpoint == endpoint) {
                service_ep.health_status = if is_healthy {
                    DMSServiceHealthStatus::Healthy
                } else {
                    DMSServiceHealthStatus::Unhealthy
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
    /// A reference to the `DMSCircuitBreaker` instance
    pub fn get_circuit_breaker(&self) -> &DMSCircuitBreaker {
        &self.circuit_breaker
    }

    /// Returns a reference to the load balancer.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSLoadBalancer` instance
    pub fn get_load_balancer(&self) -> &DMSLoadBalancer {
        &self.load_balancer
    }

    /// Returns a reference to the health checker.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSHealthChecker` instance
    pub fn get_health_checker(&self) -> &DMSHealthChecker {
        &self.health_checker
    }

    /// Returns a reference to the traffic manager.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSTrafficManager` instance
    pub fn get_traffic_manager(&self) -> &DMSTrafficManager {
        &self.traffic_manager
    }

    /// Returns a reference to the service discovery component.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSServiceDiscovery` instance
    pub fn get_service_discovery(&self) -> &DMSServiceDiscovery {
        &self.service_discovery
    }
}

#[async_trait]
impl DMSModule for DMSServiceMesh {
    /// Returns the name of the service mesh module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "service-mesh"
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
    /// A `DMSResult<()>` indicating success or failure
    async fn start(&mut self, _ctx: &mut crate::core::DMSServiceContext) -> DMSResult<()> {
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
    /// A `DMSResult<()>` indicating success or failure
    async fn shutdown(&mut self, _ctx: &mut crate::core::DMSServiceContext) -> DMSResult<()> {
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