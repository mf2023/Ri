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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSServiceMeshConfig {
    pub enable_service_discovery: bool,
    pub enable_health_check: bool,
    pub enable_traffic_management: bool,
    pub health_check_interval: Duration,
    pub circuit_breaker_config: DMSCircuitBreakerConfig,
    pub load_balancer_strategy: DMSLoadBalancerStrategy,
    pub max_retry_attempts: u32,
    pub retry_timeout: Duration,
}

impl Default for DMSServiceMeshConfig {
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

#[derive(Debug, Clone)]
pub struct DMSServiceEndpoint {
    pub service_name: String,
    pub endpoint: String,
    pub weight: u32,
    pub metadata: HashMap<String, String>,
    pub health_status: DMSServiceHealthStatus,
    pub last_health_check: SystemTime,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DMSServiceHealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

pub struct DMSServiceMesh {
    config: DMSServiceMeshConfig,
    service_discovery: Arc<DMSServiceDiscovery>,
    health_checker: Arc<DMSHealthChecker>,
    traffic_manager: Arc<DMSTrafficManager>,
    circuit_breaker: Arc<DMSCircuitBreaker>,
    load_balancer: Arc<DMSLoadBalancer>,
    services: Arc<RwLock<HashMap<String, Vec<DMSServiceEndpoint>>>>,
}

impl DMSServiceMesh {
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

    pub async fn discover_service(&self, service_name: &str) -> DMSResult<Vec<DMSServiceEndpoint>> {
        if !self.config.enable_service_discovery {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        let services = self.services.read().await;
        let endpoints = services.get(service_name)
            .ok_or_else(|| DMSError::ServiceMesh(format!("Service '{}' not found", service_name)))?
            .clone();

        let healthy_endpoints: Vec<DMSServiceEndpoint> = endpoints
            .into_iter()
            .filter(|ep| ep.health_status == DMSServiceHealthStatus::Healthy)
            .collect();

        if healthy_endpoints.is_empty() {
            return Err(DMSError::ServiceMesh(format!("No healthy endpoints for service '{}'", service_name)));
        }

        Ok(healthy_endpoints)
    }

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

    pub fn get_circuit_breaker(&self) -> &DMSCircuitBreaker {
        &self.circuit_breaker
    }

    pub fn get_load_balancer(&self) -> &DMSLoadBalancer {
        &self.load_balancer
    }

    pub fn get_health_checker(&self) -> &DMSHealthChecker {
        &self.health_checker
    }

    pub fn get_traffic_manager(&self) -> &DMSTrafficManager {
        &self.traffic_manager
    }

    pub fn get_service_discovery(&self) -> &DMSServiceDiscovery {
        &self.service_discovery
    }
}

#[async_trait]
impl DMSModule for DMSServiceMesh {
    fn name(&self) -> &str {
        "service-mesh"
    }

    fn is_critical(&self) -> bool {
        true
    }

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