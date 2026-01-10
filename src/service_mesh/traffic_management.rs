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

//! # Traffic Management Module
//! 
//! This module provides traffic management functionality for the DMSC service mesh. It allows
//! configuring and managing traffic routes, traffic splits, circuit breakers, rate limits,
//! and fault injection for services in the mesh.
//! 
//! ## Key Components
//! 
//! - **DMSCTrafficRoute**: Configuration for routing traffic between services
//! - **DMSCMatchCriteria**: Criteria for matching requests to routes
//! - **DMSCRouteAction**: Action to take for matched requests
//! - **DMSCWeightedDestination**: Weighted destination for traffic splitting
//! - **DMSCRetryPolicy**: Configuration for request retries
//! - **DMSCFaultInjection**: Configuration for fault injection
//! - **DMSCTrafficSplit**: Configuration for splitting traffic between service subsets
//! - **DMSCSubset**: Service subset definition for traffic splitting
//! - **DMSCTrafficManager**: Main traffic management service
//! - **DMSCCircuitBreakerConfig**: Configuration for circuit breakers
//! - **DMSCRateLimitConfig**: Configuration for rate limiting
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
//! async fn example() -> DMSCResult<()> {
//!     // Create a traffic manager
//!     let traffic_manager = DMSCTrafficManager::new(true);
//!     
//!     // Create a traffic route
//!     let route = DMSCTrafficRoute {
//!         name: "http-route".to_string(),
//!         source_service: "gateway".to_string(),
//!         destination_service: "backend".to_string(),
//!         match_criteria: DMSCMatchCriteria {
//!             path_prefix: Some("/api".to_string()),
//!             headers: HashMap::new(),
//!             method: Some("GET".to_string()),
//!             query_parameters: HashMap::new(),
//!         },
//!         route_action: DMSCRouteAction::Route(vec![DMSCWeightedDestination {
//!             service: "backend-v1".to_string(),
//!             weight: 80,
//!             subset: None,
//!         }, DMSCWeightedDestination {
//!             service: "backend-v2".to_string(),
//!             weight: 20,
//!             subset: None,
//!         }]),
//!         retry_policy: Some(DMSCRetryPolicy {
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
//!     let cb_config = DMSCCircuitBreakerConfig {
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
#[cfg(feature = "http_client")]
use reqwest;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

use crate::core::{DMSCResult, DMSCError};

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCTrafficRoute {
    pub name: String,
    pub source_service: String,
    pub destination_service: String,
    pub match_criteria: DMSCMatchCriteria,
    pub route_action: DMSCRouteAction,
    pub retry_policy: Option<DMSCRetryPolicy>,
    pub timeout: Option<Duration>,
    pub fault_injection: Option<DMSCFaultInjection>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCMatchCriteria {
    pub path_prefix: Option<String>,
    pub headers: HashMap<String, String>,
    pub method: Option<String>,
    pub query_parameters: HashMap<String, String>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSCRouteAction {
    Route(Vec<DMSCWeightedDestination>),
    Redirect(String),
    DirectResponse(u16, String),
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCWeightedDestination {
    pub service: String,
    pub weight: u32,
    pub subset: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCRetryPolicy {
    pub attempts: u32,
    pub per_try_timeout: Duration,
    pub retry_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCFaultInjection {
    pub delay: Option<DMSCDelayFault>,
    pub abort: Option<DMSCAbortFault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDelayFault {
    pub percentage: f64,
    pub fixed_delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCAbortFault {
    pub percentage: f64,
    pub http_status: u16,
}

#[derive(Debug, Clone)]
pub struct DMSCTrafficSplit {
    pub service: String,
    pub subsets: HashMap<String, DMSCSubset>,
    pub default_subset: String,
}

#[derive(Debug, Clone)]
pub struct DMSCSubset {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub weight: u32,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCTrafficManager {
    enabled: bool,
    routes: Arc<RwLock<HashMap<String, Vec<DMSCTrafficRoute>>>>,
    traffic_splits: Arc<RwLock<HashMap<String, DMSCTrafficSplit>>>,
    circuit_breakers: Arc<RwLock<HashMap<String, DMSCCircuitBreakerConfig>>>,
    rate_limits: Arc<RwLock<HashMap<String, DMSCRateLimitConfig>>>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    #[cfg(feature = "http_client")]
    http_client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCCircuitBreakerConfig {
    pub consecutive_errors: u32,
    pub interval: Duration,
    pub base_ejection_time: Duration,
    pub max_ejection_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCRateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window: Duration,
}

impl DMSCTrafficManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            routes: Arc::new(RwLock::new(HashMap::new())),
            traffic_splits: Arc::new(RwLock::new(HashMap::new())),
            circuit_breakers: Arc::new(RwLock::new(HashMap::new())),
            rate_limits: Arc::new(RwLock::new(HashMap::new())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            #[cfg(feature = "http_client")]
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .connect_timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }

    pub async fn add_traffic_route(&self, route: DMSCTrafficRoute) -> DMSCResult<()> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut routes = self.routes.write().await;
        routes.entry(route.source_service.clone())
            .or_insert_with(Vec::new)
            .push(route);

        Ok(())
    }

    pub async fn remove_traffic_route(&self, source_service: &str, route_name: &str) -> DMSCResult<()> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
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

    pub async fn get_traffic_routes(&self, source_service: &str) -> DMSCResult<Vec<DMSCTrafficRoute>> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let routes = self.routes.read().await;
        let service_routes = routes.get(source_service)
            .cloned()
            .unwrap_or_default();

        Ok(service_routes)
    }

    pub async fn create_traffic_split(&self, split: DMSCTrafficSplit) -> DMSCResult<()> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut traffic_splits = self.traffic_splits.write().await;
        traffic_splits.insert(split.service.clone(), split);

        Ok(())
    }

    pub async fn get_traffic_split(&self, service: &str) -> DMSCResult<Option<DMSCTrafficSplit>> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let traffic_splits = self.traffic_splits.read().await;
        Ok(traffic_splits.get(service).cloned())
    }

    pub async fn route_request(&self, endpoint: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        if !self.enabled {
            return Ok(request_data);
        }

        if let Some(fault_injection) = self.should_inject_fault() {
            self.inject_fault(&fault_injection).await?;
        }

        if self.should_rate_limit(endpoint).await? {
            return Err(DMSCError::ServiceMesh("Rate limit exceeded".to_string()));
        }

        // Apply traffic policies to transform request data
        let transformed_request = self.apply_traffic_policies(request_data).await?;
        
        // Try to find matching routes
        if let Some(matching_route) = self.find_matching_route(endpoint).await {
            // Apply the matched route
            return self.apply_route(&matching_route, endpoint, transformed_request).await;
        }
        
        // If no matching route found, perform default HTTP call
        self.make_http_request(endpoint, transformed_request).await
    }
    
    /// Finds a matching traffic route for the given endpoint
    async fn find_matching_route(&self, endpoint: &str) -> Option<DMSCTrafficRoute> {
        let routes = self.routes.read().await;
        
        // Iterate through all routes to find a match
        for (_source_service, service_routes) in &*routes {
            for route in service_routes {
                if self.is_route_match(route, endpoint) {
                    return Some(route.clone());
                }
            }
        }
        
        None
    }
    
    /// Checks if a route matches the given endpoint
    fn is_route_match(&self, _route: &DMSCTrafficRoute, _endpoint: &str) -> bool {
        #[cfg(feature = "http_client")]
        if let Ok(url) = _endpoint.parse::<reqwest::Url>() {
            let host = url.host_str().unwrap_or("");
            if _route.destination_service.contains(host) {
                return true;
            }
        }
        false
    }
    
    /// Applies a matched route to the request
    async fn apply_route(&self, route: &DMSCTrafficRoute, original_endpoint: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        // Handle different route actions
        match &route.route_action {
            DMSCRouteAction::Route(destinations) => {
                // Select destination index based on weights
                let selected_index = self.select_destination_index(destinations).await;
                let mut selected_destination = destinations[selected_index].clone();
                
                // Apply traffic splitting if configured
                if let Some(split_destination) = self.apply_traffic_split(&selected_destination.service).await {
                    // Override service name with split destination
                    selected_destination.service = split_destination;
                }
                
                // Replace endpoint with selected destination
                let new_endpoint = self.replace_endpoint(original_endpoint, &selected_destination).await;
                
                // Apply retry policy if configured
                if let Some(retry_policy) = &route.retry_policy {
                    self.retry_request(new_endpoint.as_str(), request_data, retry_policy).await
                } else {
                    // Perform HTTP call with selected destination
                    self.make_http_request(new_endpoint.as_str(), request_data).await
                }
            },
            DMSCRouteAction::Redirect(redirect_uri) => {
                // Handle redirect action
                Err(DMSCError::ServiceMesh(format!("Redirect to: {}", redirect_uri)))
            },
            DMSCRouteAction::DirectResponse(_status, body) => {
                // Return direct response without making a network call
                Ok(body.clone().into())
            }
        }
    }
    
    /// Selects a destination index based on weights
    async fn select_destination_index(&self, destinations: &[DMSCWeightedDestination]) -> usize {
        if destinations.len() == 1 {
            return 0;
        }
        
        // Calculate total weight
        let total_weight: u32 = destinations.iter().map(|d| d.weight).sum();
        
        // Select random destination based on weights
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut current_weight = 0;
        let random_weight = rng.gen_range(0..total_weight);
        
        for (index, destination) in destinations.iter().enumerate() {
            current_weight += destination.weight;
            if random_weight < current_weight {
                return index;
            }
        }
        
        // Fallback to first destination
        0
    }
    
    /// Replaces the original endpoint with the selected destination
    async fn replace_endpoint(&self, original_endpoint: &str, _destination: &DMSCWeightedDestination) -> String {
        // Simple replacement logic for demonstration
        // In a full implementation, this would use a more sophisticated approach
        original_endpoint.to_string()
    }
    
    /// Retries a request according to the retry policy
    async fn retry_request(&self, endpoint: &str, request_data: Vec<u8>, retry_policy: &DMSCRetryPolicy) -> DMSCResult<Vec<u8>> {
        let max_attempts = retry_policy.attempts;
        
        for attempt in 1..=max_attempts {
            let result = self.make_http_request(endpoint, request_data.clone()).await;
            
            match result {
                Ok(response) => return Ok(response),
                Err(e) => {
                    // Check if retry should be attempted
                    if attempt < max_attempts && self.should_retry(&e, retry_policy) {
                        // Wait before retry (exponential backoff)
                        let delay = Duration::from_millis(100 * 2u64.pow(attempt - 1));
                        tokio::time::sleep(delay).await;
                        continue;
                    }
                    return Err(e);
                }
            }
        }
        
        Err(DMSCError::ServiceMesh("All retry attempts failed".to_string()))
    }
    
    /// Checks if a request should be retried based on the error and retry policy
    fn should_retry(&self, _error: &DMSCError, retry_policy: &DMSCRetryPolicy) -> bool {
        // Check if error should be retried based on retry_on conditions
        // Simple implementation for demonstration
        retry_policy.retry_on.iter().any(|s| s == "5xx" || s == "all")
    }

    #[cfg(feature = "http_client")]
    async fn make_http_request(&self, endpoint: &str, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        // Parse endpoint URL
        let url = endpoint.parse::<reqwest::Url>()
            .map_err(|e| DMSCError::ServiceMesh(format!("Invalid endpoint URL: {e}")))?;
        
        // Create HTTP request
        let response = self.http_client
            .post(url)
            .header("Content-Type", "application/octet-stream")
            .body(request_data)
            .send()
            .await
            .map_err(|e| DMSCError::ServiceMesh(format!("HTTP request failed: {e}")))?;
        
        // Check response status
        if !response.status().is_success() {
            return Err(DMSCError::ServiceMesh(format!(
                "HTTP request failed with status: {}", 
                response.status()
            )));
        }
        
        // Get response body
        let response_data = response
            .bytes()
            .await
            .map_err(|e| DMSCError::ServiceMesh(format!("Failed to read response body: {e}")))?
            .to_vec();
        
        Ok(response_data)
    }
    
    #[cfg(not(feature = "http_client"))]
    async fn make_http_request(&self, _endpoint: &str, _request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        Err(DMSCError::ServiceMesh(format!("HTTP client is not enabled. Enable the 'http_client' feature to use HTTP requests.")))
    }

    async fn apply_traffic_policies(&self, request_data: Vec<u8>) -> DMSCResult<Vec<u8>> {
        // For now, we'll pass through the request data
        // In a full implementation, this would apply transformations based on traffic policies
        Ok(request_data)
    }
    
    /// Applies traffic splitting to determine the destination service
    /// based on configured traffic splits and weights
    async fn apply_traffic_split(&self, service: &str) -> Option<String> {
        let traffic_splits = self.traffic_splits.read().await;
        
        if let Some(traffic_split) = traffic_splits.get(service) {
            // Calculate total weight for all subsets
            let total_weight: u32 = traffic_split.subsets.values()
                .map(|subset| subset.weight)
                .sum();
            
            if total_weight == 0 {
                // If total weight is 0, use default subset
                Some(traffic_split.default_subset.clone())
            } else {
                // Select random destination based on weights
                use rand::Rng;
                let mut rng = rand::thread_rng();
                let random_weight = rng.gen_range(0..total_weight);
                
                let mut current_weight = 0;
                for subset in traffic_split.subsets.values() {
                    current_weight += subset.weight;
                    if random_weight < current_weight {
                        return Some(subset.name.clone());
                    }
                }
                
                // Fallback to default subset
                Some(traffic_split.default_subset.clone())
            }
        } else {
            None
        }
    }

    fn should_inject_fault(&self) -> Option<DMSCFaultInjection> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen_bool(0.01) {
            Some(DMSCFaultInjection {
                delay: Some(DMSCDelayFault {
                    percentage: 0.5,
                    fixed_delay: Duration::from_millis(100),
                }),
                abort: None,
            })
        } else {
            None
        }
    }

    async fn inject_fault(&self, fault: &DMSCFaultInjection) -> DMSCResult<()> {
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
                return Err(DMSCError::ServiceMesh(format!("Fault injection: HTTP {}", abort.http_status)));
            }
        }

        Ok(())
    }

    /// Implements a sliding window rate limiter using the leaky bucket algorithm
    async fn should_rate_limit(&self, endpoint: &str) -> DMSCResult<bool> {
        let rate_limits = self.rate_limits.read().await;
        
        // Check if there's a rate limit configured for this endpoint
        if let Some(config) = rate_limits.get(endpoint) {
            // Use a thread-safe per-endpoint rate limiter with sliding window
            use std::sync::atomic::{AtomicU64, Ordering};
            use std::collections::HashMap;
            use std::sync::Arc;
            
            // Store rate limiters in a thread-safe map
            static RATE_LIMITERS: std::sync::Mutex<Option<HashMap<String, Arc<RateLimiter>>>> = 
                std::sync::Mutex::new(None);
            
            // Rate limiter implementation using leaky bucket algorithm
            struct RateLimiter {
                capacity: u32,
                rate: f64, // requests per second
                tokens: AtomicU64, // current tokens available
                last_update: AtomicU64, // last update time in milliseconds
            }
            
            impl RateLimiter {
                fn new(config: &DMSCRateLimitConfig) -> Self {
                    let rate = config.requests_per_second as f64;
                    Self {
                        capacity: config.burst_size,
                        rate,
                        tokens: AtomicU64::new(config.burst_size as u64),
                        last_update: AtomicU64::new(
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or(std::time::Duration::from_secs(0))
                                .as_millis() as u64
                        ),
                    }
                }
                
                fn try_acquire(&self) -> bool {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or(std::time::Duration::from_secs(0))
                        .as_millis() as u64;
                    let last = self.last_update.load(Ordering::Acquire);
                    let elapsed = now - last;
                    
                    // Calculate tokens to add based on elapsed time (in milliseconds) and rate
                    let tokens_to_add = (elapsed as f64 / 1000.0) * self.rate;
                    let tokens_to_add = tokens_to_add as u64;
                    
                    let current = self.tokens.load(Ordering::Acquire);
                    let new_tokens = std::cmp::min(current.saturating_add(tokens_to_add), self.capacity as u64);
                    
                    // Try to acquire one token
                    if new_tokens > 0 {
                        if self.tokens.compare_exchange(current, new_tokens - 1, 
                                                       Ordering::AcqRel, Ordering::Acquire).is_ok() {
                            // Update last update time if we successfully acquired a token
                            self.last_update.store(now, Ordering::Release);
                            return true;
                        }
                    }
                    
                    false
                }
            }
            
            let mut limiters = RATE_LIMITERS.lock()
                .map_err(|e| DMSCError::ServiceMesh(format!("Failed to acquire rate limiter lock: {}", e)))?;
            if limiters.is_none() {
                *limiters = Some(HashMap::new());
            }
            
            let limiters = limiters.as_mut()
                .ok_or_else(|| DMSCError::InvalidState("Rate limiters not initialized".to_string()))?;
            
            // Get or create rate limiter for this endpoint
            let limiter = limiters.entry(endpoint.to_string())
                .or_insert_with(|| Arc::new(RateLimiter::new(config)));
            
            // Try to acquire a token
            Ok(!limiter.try_acquire())
        } else {
            Ok(false) // No rate limit configured
        }
    }

    pub async fn set_circuit_breaker_config(&self, service: &str, config: DMSCCircuitBreakerConfig) -> DMSCResult<()> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut circuit_breakers = self.circuit_breakers.write().await;
        circuit_breakers.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn set_rate_limit_config(&self, service: &str, config: DMSCRateLimitConfig) -> DMSCResult<()> {
        if !self.enabled {
            return Err(DMSCError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut rate_limits = self.rate_limits.write().await;
        rate_limits.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn get_circuit_breaker_config(&self, service: &str) -> DMSCResult<Option<DMSCCircuitBreakerConfig>> {
        let circuit_breakers = self.circuit_breakers.read().await;
        Ok(circuit_breakers.get(service).cloned())
    }

    pub async fn get_rate_limit_config(&self, service: &str) -> DMSCResult<Option<DMSCRateLimitConfig>> {
        let rate_limits = self.rate_limits.read().await;
        Ok(rate_limits.get(service).cloned())
    }

    pub async fn start_background_tasks(&self) -> DMSCResult<()> {
        if !self.enabled {
            return Ok(());
        }

        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> DMSCResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn health_check(&self) -> DMSCResult<bool> {
        Ok(self.enabled)
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCTrafficManager
#[pyo3::prelude::pymethods]
impl DMSCTrafficManager {
    #[new]
    fn py_new(enabled: bool) -> PyResult<Self> {
        Ok(Self::new(enabled))
    }
    
    /// Add traffic route from Python
    #[pyo3(name = "add_traffic_route")]
    fn add_traffic_route_impl(&self, _route: DMSCTrafficRoute) -> PyResult<()> {
        // For now, we'll return an error since we can't easily run async code from Python
        // In a real implementation, you'd want to integrate with Python's async runtime
        Err(pyo3::exceptions::PyRuntimeError::new_err("Async traffic management not supported from Python yet"))
    }
    
    /// Get traffic routes from Python
    #[pyo3(name = "get_traffic_routes")]
    fn get_traffic_routes_impl(&self, _service_name: String) -> PyResult<Vec<DMSCTrafficRoute>> {
        // For now, we'll return an error since we can't easily run async code from Python
        // In a real implementation, you'd want to integrate with Python's async runtime
        Err(pyo3::exceptions::PyRuntimeError::new_err("Async traffic management not supported from Python yet"))
    }
}
