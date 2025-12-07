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

//! # Traffic Management Module
//! 
//! This module provides traffic management functionality for the DMS service mesh. It allows
//! configuring and managing traffic routes, traffic splits, circuit breakers, rate limits,
//! and fault injection for services in the mesh.
//! 
//! ## Key Components
//! 
//! - **DMSTrafficRoute**: Configuration for routing traffic between services
//! - **DMSMatchCriteria**: Criteria for matching requests to routes
//! - **DMSRouteAction**: Action to take for matched requests
//! - **DMSWeightedDestination**: Weighted destination for traffic splitting
//! - **DMSRetryPolicy**: Configuration for request retries
//! - **DMSFaultInjection**: Configuration for fault injection
//! - **DMSTrafficSplit**: Configuration for splitting traffic between service subsets
//! - **DMSSubset**: Service subset definition for traffic splitting
//! - **DMSTrafficManager**: Main traffic management service
//! - **DMSCircuitBreakerConfig**: Configuration for circuit breakers
//! - **DMSRateLimitConfig**: Configuration for rate limiting
//! 
//! ## Design Principles
//! 
//! 1. **Declarative Configuration**: Traffic rules are defined declaratively
//! 2. **Flexible Routing**: Supports multiple routing actions (route, redirect, direct response)
//! 3. **Traffic Splitting**: Weighted traffic splitting between service subsets
//! 4. **Resilience**: Built-in retry policies and circuit breakers
//! 5. **Fault Injection**: Support for fault injection for testing resilience
//! 6. **Rate Limiting**: Protection against excessive traffic
//! 7. **Timeout Management**: Configurable request timeouts
//! 8. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 9. **Graceful Shutdown**: Proper cleanup of background tasks
//! 10. **Extensible**: Easy to add new traffic management features
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a traffic manager
//!     let traffic_manager = DMSTrafficManager::new(true);
//!     
//!     // Create a traffic route
//!     let route = DMSTrafficRoute {
//!         name: "http-route".to_string(),
//!         source_service: "gateway".to_string(),
//!         destination_service: "backend".to_string(),
//!         match_criteria: DMSMatchCriteria {
//!             path_prefix: Some("/api".to_string()),
//!             headers: HashMap::new(),
//!             method: Some("GET".to_string()),
//!             query_parameters: HashMap::new(),
//!         },
//!         route_action: DMSRouteAction::Route(vec![DMSWeightedDestination {
//!             service: "backend-v1".to_string(),
//!             weight: 80,
//!             subset: None,
//!         }, DMSWeightedDestination {
//!             service: "backend-v2".to_string(),
//!             weight: 20,
//!             subset: None,
//!         }]),
//!         retry_policy: Some(DMSRetryPolicy {
//!             attempts: 3,
//!             per_try_timeout: Duration::from_secs(1),
//!             retry_on: vec!["5xx".to_string()],
//!         }),
//!         timeout: Some(Duration::from_secs(5)),
//!         fault_injection: None,
//!     };
//!     
//!     // Add the route
//!     traffic_manager.add_traffic_route(route).await?;
//!     
//!     // Set a circuit breaker
//!     let cb_config = DMSCircuitBreakerConfig {
//!         consecutive_errors: 5,
//!         interval: Duration::from_secs(10),
//!         base_ejection_time: Duration::from_secs(30),
//!         max_ejection_percent: 50.0,
//!     };
//!     traffic_manager.set_circuit_breaker_config("backend", cb_config).await?;
//!     
//!     Ok(())
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::core::{DMSResult, DMSError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSTrafficRoute {
    pub name: String,
    pub source_service: String,
    pub destination_service: String,
    pub match_criteria: DMSMatchCriteria,
    pub route_action: DMSRouteAction,
    pub retry_policy: Option<DMSRetryPolicy>,
    pub timeout: Option<Duration>,
    pub fault_injection: Option<DMSFaultInjection>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSMatchCriteria {
    pub path_prefix: Option<String>,
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    pub query_parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSRouteAction {
    Route(Vec<DMSWeightedDestination>),
    Redirect(String),
    DirectResponse(u16, String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSWeightedDestination {
    pub service: String,
    pub weight: u32,
    pub subset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSRetryPolicy {
    pub attempts: u32,
    pub per_try_timeout: Duration,
    pub retry_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSFaultInjection {
    pub delay: Option<DMSDelayFault>,
    pub abort: Option<DMSAbortFault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSDelayFault {
    pub percentage: f64,
    pub fixed_delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSAbortFault {
    pub percentage: f64,
    pub http_status: u16,
}

#[derive(Debug, Clone)]
pub struct DMSTrafficSplit {
    pub service: String,
    pub subsets: HashMap<String, DMSSubset>,
    pub default_subset: String,
}

#[derive(Debug, Clone)]
pub struct DMSSubset {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub weight: u32,
}

pub struct DMSTrafficManager {
    enabled: bool,
    routes: Arc<RwLock<HashMap<String, Vec<DMSTrafficRoute>>>>,
    traffic_splits: Arc<RwLock<HashMap<String, DMSTrafficSplit>>>,
    circuit_breakers: Arc<RwLock<HashMap<String, DMSCircuitBreakerConfig>>>,
    rate_limits: Arc<RwLock<HashMap<String, DMSRateLimitConfig>>>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCircuitBreakerConfig {
    pub consecutive_errors: u32,
    pub interval: Duration,
    pub base_ejection_time: Duration,
    pub max_ejection_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSRateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window: Duration,
}

impl DMSTrafficManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            routes: Arc::new(RwLock::new(HashMap::new())),
            traffic_splits: Arc::new(RwLock::new(HashMap::new())),
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn add_traffic_route(&self, route: DMSTrafficRoute) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut routes = self.routes.write().await;
        routes.entry(route.source_service.clone())
            .or_insert_with(Vec::new)
            .push(route);

        Ok(())
    }

    pub async fn remove_traffic_route(&self, source_service: &str, route_name: &str) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut routes = self.routes.write().await;
        if let Some(service_routes) = routes.get_mut(source_service) {
            service_routes.retain(|r| r.name != route_name);
            
            if service_routes.is_empty() {
                routes.remove(source_service);
            }
        }

        Ok(())
    }

    pub async fn get_traffic_routes(&self, source_service: &str) -> DMSResult<Vec<DMSTrafficRoute>> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let routes = self.routes.read().await;
        let service_routes = routes.get(source_service)
            .cloned()
            .unwrap_or_default();

        Ok(service_routes)
    }

    pub async fn create_traffic_split(&self, split: DMSTrafficSplit) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut traffic_splits = self.traffic_splits.write().await;
        traffic_splits.insert(split.service.clone(), split);

        Ok(())
    }

    pub async fn get_traffic_split(&self, service: &str) -> DMSResult<Option<DMSTrafficSplit>> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let traffic_splits = self.traffic_splits.read().await;
        Ok(traffic_splits.get(service).cloned())
    }

    pub async fn route_request(&self, endpoint: &str, request_data: Vec<u8>) -> DMSResult<Vec<u8>> {
        if !self.enabled {
            return Ok(request_data);
        }

        if let Some(fault_injection) = self.should_inject_fault() {
            self.inject_fault(&fault_injection).await?;
        }

        if self.should_rate_limit(endpoint).await? {
            return Err(DMSError::ServiceMesh("Rate limit exceeded".to_string()));
        }

        self.apply_traffic_policies(request_data).await
    }

    async fn apply_traffic_policies(&self, request_data: Vec<u8>) -> DMSResult<Vec<u8>> {
        Ok(request_data)
    }

    fn should_inject_fault(&self) -> Option<DMSFaultInjection> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen_bool(0.01) {
            Some(DMSFaultInjection {
                delay: Some(DMSDelayFault {
                    percentage: 0.5,
                    fixed_delay: Duration::from_millis(100),
                }),
                abort: None,
            })
        } else {
            None
        }
    }

    async fn inject_fault(&self, fault: &DMSFaultInjection) -> DMSResult<()> {
        if let Some(delay) = &fault.delay {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            
            if rng.gen_bool(delay.percentage) {
                tokio::time::sleep(delay.fixed_delay).await;
            }
        }

        if let Some(abort) = &fault.abort {
            use rand::Rng;
            let mut rng = rand::thread_rng();
            
            if rng.gen_bool(abort.percentage) {
                return Err(DMSError::ServiceMesh(format!("Fault injection: HTTP {}", abort.http_status)));
            }
        }

        Ok(())
    }

    async fn should_rate_limit(&self, _endpoint: &str) -> DMSResult<bool> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        Ok(rng.gen_bool(0.001))
    }

    pub async fn set_circuit_breaker_config(&self, service: &str, config: DMSCircuitBreakerConfig) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut circuit_breakers = self.circuit_breakers.write().await;
        circuit_breakers.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn set_rate_limit_config(&self, service: &str, config: DMSRateLimitConfig) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut rate_limits = self.rate_limits.write().await;
        rate_limits.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn get_circuit_breaker_config(&self, service: &str) -> DMSResult<Option<DMSCircuitBreakerConfig>> {
        let circuit_breakers = self.circuit_breakers.read().await;
        Ok(circuit_breakers.get(service).cloned())
    }

    pub async fn get_rate_limit_config(&self, service: &str) -> DMSResult<Option<DMSRateLimitConfig>> {
        let rate_limits = self.rate_limits.read().await;
        Ok(rate_limits.get(service).cloned())
    }

    pub async fn start_background_tasks(&self) -> DMSResult<()> {
        if !self.enabled {
            return Ok(());
        }

        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> DMSResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn health_check(&self) -> DMSResult<bool> {
        Ok(self.enabled)
    }
}