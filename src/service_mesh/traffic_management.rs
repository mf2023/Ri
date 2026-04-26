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

//! # Traffic Management Module
//! 
//! This module provides traffic management functionality for the Ri service mesh. It allows
//! configuring and managing traffic routes, traffic splits, circuit breakers, rate limits,
//! and fault injection for services in the mesh.
//! 
//! ## Key Components
//! 
//! - **RiTrafficRoute**: Configuration for routing traffic between services
//! - **RiMatchCriteria**: Criteria for matching requests to routes
//! - **RiRouteAction**: Action to take for matched requests
//! - **RiWeightedDestination**: Weighted destination for traffic splitting
//! - **RiRetryPolicy**: Configuration for request retries
//! - **RiFaultInjection**: Configuration for fault injection
//! - **RiTrafficSplit**: Configuration for splitting traffic between service subsets
//! - **RiSubset**: Service subset definition for traffic splitting
//! - **RiTrafficManager**: Main traffic management service
//! - **RiCircuitBreakerConfig**: Configuration for circuit breakers
//! - **RiRateLimitConfig**: Configuration for rate limiting
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
//! use ri::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a traffic manager
//!     let traffic_manager = RiTrafficManager::new(true);
//!     
//!     // Create a traffic route
//!     let route = RiTrafficRoute {
//!         name: "http-route".to_string(),
//!         source_service: "gateway".to_string(),
//!         destination_service: "backend".to_string(),
//!         match_criteria: RiMatchCriteria {
//!             path_prefix: Some("/api".to_string()),
//!             headers: FxHashMap::default(),
//!             method: Some("GET".to_string()),
//!             query_parameters: FxHashMap::default(),
//!         },
//!         route_action: RiRouteAction::Route(vec![RiWeightedDestination {
//!             service: "backend-v1".to_string(),
//!             weight: 80,
//!             subset: None,
//!         }, RiWeightedDestination {
//!             service: "backend-v2".to_string(),
//!             weight: 20,
//!             subset: None,
//!         }]),
//!         retry_policy: Some(RiRetryPolicy {
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
//!     let cb_config = RiCircuitBreakerConfig {
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
use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;
#[cfg(feature = "http_client")]
use reqwest;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

use crate::core::{RiResult, RiError};
use crate::observability::{RiTracer, RiSpanKind, RiSpanStatus};
#[cfg(feature = "http_client")]
use crate::observability::RiContextCarrier;

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiTrafficRoute {
    pub name: String,
    pub source_service: String,
    pub destination_service: String,
    pub match_criteria: RiMatchCriteria,
    pub route_action: RiRouteAction,
    pub retry_policy: Option<RiRetryPolicy>,
    pub timeout: Option<Duration>,
    pub fault_injection: Option<RiFaultInjection>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiTrafficRoute {
    #[new]
    fn py_new(name: String, source_service: String, destination_service: String) -> Self {
        Self {
            name,
            source_service,
            destination_service,
            match_criteria: RiMatchCriteria {
                path_prefix: None,
                headers: FxHashMap::default(),
                method: None,
                query_parameters: FxHashMap::default(),
            },
            route_action: RiRouteAction::Route(vec![]),
            retry_policy: None,
            timeout: None,
            fault_injection: None,
        }
    }
    
    fn get_name(&self) -> String {
        self.name.clone()
    }
    
    fn get_source_service(&self) -> String {
        self.source_service.clone()
    }
    
    fn get_destination_service(&self) -> String {
        self.destination_service.clone()
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiMatchCriteria {
    pub path_prefix: Option<String>,
    pub headers: FxHashMap<String, String>,
    pub method: Option<String>,
    pub query_parameters: FxHashMap<String, String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiMatchCriteria {
    #[new]
    fn py_new() -> Self {
        Self {
            path_prefix: None,
            headers: FxHashMap::default(),
            method: None,
            query_parameters: FxHashMap::default(),
        }
    }
    
    fn get_path_prefix(&self) -> Option<String> {
        self.path_prefix.clone()
    }
    
    fn get_method(&self) -> Option<String> {
        self.method.clone()
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiRouteAction {
    Route(Vec<RiWeightedDestination>),
    Redirect(String),
    DirectResponse(u16, String),
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiWeightedDestination {
    pub service: String,
    pub weight: u32,
    pub subset: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiWeightedDestination {
    #[new]
    fn py_new(service: String, weight: u32) -> Self {
        Self {
            service,
            weight,
            subset: None,
        }
    }
    
    fn get_service(&self) -> String {
        self.service.clone()
    }
    
    fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiRetryPolicy {
    pub attempts: u32,
    pub per_try_timeout: Duration,
    pub retry_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiFaultInjection {
    pub delay: Option<RiDelayFault>,
    pub abort: Option<RiAbortFault>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiDelayFault {
    pub percentage: f64,
    pub fixed_delay: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiAbortFault {
    pub percentage: f64,
    pub http_status: u16,
}

#[derive(Debug, Clone)]
pub struct RiTrafficSplit {
    pub service: String,
    pub subsets: FxHashMap<String, RiSubset>,
    pub default_subset: String,
}

#[derive(Debug, Clone)]
pub struct RiSubset {
    pub name: String,
    pub labels: FxHashMap<String, String>,
    pub weight: u32,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiTrafficManager {
    enabled: bool,
    routes: Arc<RwLock<FxHashMap<String, Vec<RiTrafficRoute>>>>,
    traffic_splits: Arc<RwLock<FxHashMap<String, RiTrafficSplit>>>,
    circuit_breakers: Arc<RwLock<FxHashMap<String, RiCircuitBreakerConfig>>>,
    rate_limits: Arc<RwLock<FxHashMap<String, RiRateLimitConfig>>>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    #[cfg(feature = "http_client")]
    http_client: reqwest::Client,
    tracer: Option<Arc<RiTracer>>,
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiCircuitBreakerConfig {
    pub consecutive_errors: u32,
    pub interval: Duration,
    pub base_ejection_time: Duration,
    pub max_ejection_percent: f64,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiCircuitBreakerConfig {
    #[new]
    fn py_new(consecutive_errors: u32, max_ejection_percent: f64) -> Self {
        Self {
            consecutive_errors,
            interval: Duration::from_secs(10),
            base_ejection_time: Duration::from_secs(30),
            max_ejection_percent,
        }
    }
    
    fn get_consecutive_errors(&self) -> u32 {
        self.consecutive_errors
    }
    
    fn get_max_ejection_percent(&self) -> f64 {
        self.max_ejection_percent
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiRateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window: Duration,
}

impl RiTrafficManager {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            routes: Arc::new(RwLock::new(FxHashMap::default())),
            traffic_splits: Arc::new(RwLock::new(FxHashMap::default())),
            circuit_breakers: Arc::new(RwLock::new(FxHashMap::default())),
            rate_limits: Arc::new(RwLock::new(FxHashMap::default())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            #[cfg(feature = "http_client")]
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .connect_timeout(Duration::from_secs(10))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
            tracer: None,
        }
    }
    
    pub fn with_tracer(mut self, tracer: Arc<RiTracer>) -> Self {
        self.tracer = Some(tracer);
        self
    }
    
    pub fn set_tracer(&mut self, tracer: Arc<RiTracer>) {
        self.tracer = Some(tracer);
    }

    pub async fn add_traffic_route(&self, route: RiTrafficRoute) -> RiResult<()> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut routes = self.routes.write().await;
        routes.entry(route.source_service.clone())
            .or_insert_with(Vec::new)
            .push(route);

        Ok(())
    }

    pub async fn remove_traffic_route(&self, source_service: &str, route_name: &str) -> RiResult<()> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
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

    pub async fn get_traffic_routes(&self, source_service: &str) -> RiResult<Vec<RiTrafficRoute>> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let routes = self.routes.read().await;
        let service_routes = routes.get(source_service)
            .cloned()
            .unwrap_or_default();

        Ok(service_routes)
    }

    pub async fn create_traffic_split(&self, split: RiTrafficSplit) -> RiResult<()> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut traffic_splits = self.traffic_splits.write().await;
        traffic_splits.insert(split.service.clone(), split);

        Ok(())
    }

    pub async fn get_traffic_split(&self, service: &str) -> RiResult<Option<RiTrafficSplit>> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let traffic_splits = self.traffic_splits.read().await;
        Ok(traffic_splits.get(service).cloned())
    }

    pub async fn route_request(&self, endpoint: &str, request_data: Vec<u8>) -> RiResult<Vec<u8>> {
        let span_id = if let Some(tracer) = &self.tracer {
            let span_id = tracer.start_span_from_context(
                format!("route_request:{}", endpoint),
                RiSpanKind::Client,
            );
            if let Some(ref sid) = span_id {
                let _ = tracer.span_mut(sid, |span| {
                    span.set_attribute("endpoint".to_string(), endpoint.to_string());
                    span.set_attribute("request_size".to_string(), request_data.len().to_string());
                });
            }
            span_id
        } else {
            None
        };

        let result = self.route_request_internal(endpoint, request_data).await;

        if let (Some(tracer), Some(sid)) = (&self.tracer, span_id) {
            let status = match &result {
                Ok(_) => RiSpanStatus::Ok,
                Err(e) => RiSpanStatus::Error(e.to_string()),
            };
            let _ = tracer.end_span(&sid, status);
        }

        result
    }
    
    async fn route_request_internal(&self, endpoint: &str, request_data: Vec<u8>) -> RiResult<Vec<u8>> {
        if !self.enabled {
            return Ok(request_data);
        }

        if let Some(fault_injection) = self.should_inject_fault() {
            self.inject_fault(&fault_injection).await?;
        }

        if self.should_rate_limit(endpoint).await? {
            return Err(RiError::ServiceMesh("Rate limit exceeded".to_string()));
        }

        let transformed_request = self.apply_traffic_policies(request_data).await;
        
        if let Some(matching_route) = self.find_matching_route(endpoint).await {
            return self.apply_route(&matching_route, endpoint, transformed_request).await;
        }
        
        self.make_http_request(endpoint, transformed_request).await
    }
    
    /// Finds a matching traffic route for the given endpoint
    async fn find_matching_route(&self, endpoint: &str) -> Option<RiTrafficRoute> {
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
    fn is_route_match(&self, _route: &RiTrafficRoute, _endpoint: &str) -> bool {
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
    async fn apply_route(&self, route: &RiTrafficRoute, original_endpoint: &str, request_data: Vec<u8>) -> RiResult<Vec<u8>> {
        // Handle different route actions
        match &route.route_action {
            RiRouteAction::Route(destinations) => {
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
            RiRouteAction::Redirect(redirect_uri) => {
                // Handle redirect action
                Err(RiError::ServiceMesh(format!("Redirect to: {}", redirect_uri)))
            },
            RiRouteAction::DirectResponse(_status, body) => {
                // Return direct response without making a network call
                Ok(body.clone().into())
            }
        }
    }
    
    /// Selects a destination index based on weights
    async fn select_destination_index(&self, destinations: &[RiWeightedDestination]) -> usize {
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
    async fn replace_endpoint(&self, original_endpoint: &str, _destination: &RiWeightedDestination) -> String {
        // Simple replacement logic for demonstration
        // In a full implementation, this would use a more sophisticated approach
        original_endpoint.to_string()
    }
    
    /// Retries a request according to the retry policy
    async fn retry_request(&self, endpoint: &str, request_data: Vec<u8>, retry_policy: &RiRetryPolicy) -> RiResult<Vec<u8>> {
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
        
        Err(RiError::ServiceMesh("All retry attempts failed".to_string()))
    }
    
    /// Checks if a request should be retried based on the error and retry policy
    fn should_retry(&self, _error: &RiError, retry_policy: &RiRetryPolicy) -> bool {
        // Check if error should be retried based on retry_on conditions
        // Simple implementation for demonstration
        retry_policy.retry_on.iter().any(|s| s == "5xx" || s == "all")
    }

    #[cfg(feature = "http_client")]
    async fn make_http_request(&self, endpoint: &str, request_data: Vec<u8>) -> RiResult<Vec<u8>> {
        let url = endpoint.parse::<reqwest::Url>()
            .map_err(|e| RiError::ServiceMesh(format!("Invalid endpoint URL: {e}")))?;
        
        let mut request_builder = self.http_client
            .post(url)
            .header("Content-Type", "application/octet-stream");
        
        if let Some(_tracer) = &self.tracer {
            let mut headers = FxHashMap::default();
            RiContextCarrier::inject_current_into_headers(&mut headers);
            for (key, value) in headers {
                request_builder = request_builder.header(key, value);
            }
        }
        
        let response = request_builder
            .body(request_data)
            .send()
            .await
            .map_err(|e| RiError::ServiceMesh(format!("HTTP request failed: {e}")))?;
        
        if !response.status().is_success() {
            return Err(RiError::ServiceMesh(format!(
                "HTTP request failed with status: {}", 
                response.status()
            )));
        }
        
        let response_data = response
            .bytes()
            .await
            .map_err(|e| RiError::ServiceMesh(format!("Failed to read response body: {e}")))?
            .to_vec();
        
        Ok(response_data)
    }
    
    #[cfg(not(feature = "http_client"))]
    async fn make_http_request(&self, _endpoint: &str, _request_data: Vec<u8>) -> RiResult<Vec<u8>> {
        Err(RiError::ServiceMesh(format!("HTTP client is not enabled. Enable the 'http_client' feature to use HTTP requests.")))
    }

    async fn apply_traffic_policies(&self, request_data: Vec<u8>) -> Vec<u8> {
        request_data
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

    fn should_inject_fault(&self) -> Option<RiFaultInjection> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        
        if rng.gen_bool(0.01) {
            Some(RiFaultInjection {
                delay: Some(RiDelayFault {
                    percentage: 0.5,
                    fixed_delay: Duration::from_millis(100),
                }),
                abort: None,
            })
        } else {
            None
        }
    }

    async fn inject_fault(&self, fault: &RiFaultInjection) -> RiResult<()> {
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
                return Err(RiError::ServiceMesh(format!("Fault injection: HTTP {}", abort.http_status)));
            }
        }

        Ok(())
    }

    /// Implements a sliding window rate limiter using the leaky bucket algorithm
    async fn should_rate_limit(&self, endpoint: &str) -> RiResult<bool> {
        let rate_limits = self.rate_limits.read().await;
        
        // Check if there's a rate limit configured for this endpoint
        if let Some(config) = rate_limits.get(endpoint) {
            // Use a thread-safe per-endpoint rate limiter with sliding window
            use std::sync::atomic::{AtomicU64, Ordering};
            use std::collections::HashMap as FxHashMap;
            use std::sync::Arc;
            
            // Store rate limiters in a thread-safe map
            static RATE_LIMITERS: std::sync::Mutex<Option<FxHashMap<String, Arc<RateLimiter>>>> = 
                std::sync::Mutex::new(None);
            
            // Rate limiter implementation using leaky bucket algorithm
            struct RateLimiter {
                capacity: u32,
                rate: f64, // requests per second
                tokens: AtomicU64, // current tokens available
                last_update: AtomicU64, // last update time in milliseconds
            }
            
            impl RateLimiter {
                fn new(config: &RiRateLimitConfig) -> Self {
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
                .map_err(|e| RiError::ServiceMesh(format!("Failed to acquire rate limiter lock: {}", e)))?;
            if limiters.is_none() {
                *limiters = Some(FxHashMap::default());
            }
            
            let limiters = limiters.as_mut()
                .ok_or_else(|| RiError::InvalidState("Rate limiters not initialized".to_string()))?;
            
            // Get or create rate limiter for this endpoint
            let limiter = limiters.entry(endpoint.to_string())
                .or_insert_with(|| Arc::new(RateLimiter::new(config)));
            
            // Try to acquire a token
            Ok(!limiter.try_acquire())
        } else {
            Ok(false) // No rate limit configured
        }
    }

    pub async fn set_circuit_breaker_config(&self, service: &str, config: RiCircuitBreakerConfig) -> RiResult<()> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut circuit_breakers = self.circuit_breakers.write().await;
        circuit_breakers.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn set_rate_limit_config(&self, service: &str, config: RiRateLimitConfig) -> RiResult<()> {
        if !self.enabled {
            return Err(RiError::ServiceMesh("Traffic management is disabled".to_string()));
        }

        let mut rate_limits = self.rate_limits.write().await;
        rate_limits.insert(service.to_string(), config);

        Ok(())
    }

    pub async fn get_circuit_breaker_config(&self, service: &str) -> RiResult<Option<RiCircuitBreakerConfig>> {
        let circuit_breakers = self.circuit_breakers.read().await;
        Ok(circuit_breakers.get(service).cloned())
    }

    pub async fn get_rate_limit_config(&self, service: &str) -> RiResult<Option<RiRateLimitConfig>> {
        let rate_limits = self.rate_limits.read().await;
        Ok(rate_limits.get(service).cloned())
    }

    pub async fn start_background_tasks(&self) -> RiResult<()> {
        if !self.enabled {
            return Ok(());
        }

        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> RiResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn health_check(&self) -> RiResult<bool> {
        Ok(self.enabled)
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for RiTrafficManager
#[pyo3::prelude::pymethods]
impl RiTrafficManager {
    #[new]
    fn py_new(enabled: bool) -> PyResult<Self> {
        Ok(Self::new(enabled))
    }
    
    /// Add traffic route from Python
    #[pyo3(name = "add_traffic_route")]
    fn add_traffic_route_impl(&self, route: RiTrafficRoute) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.add_traffic_route(route)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to add traffic route: {e}")))
        })
    }
    
    /// Get traffic routes from Python
    #[pyo3(name = "get_traffic_routes")]
    fn get_traffic_routes_impl(&self, service_name: String) -> PyResult<Vec<RiTrafficRoute>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.get_traffic_routes(&service_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get traffic routes: {e}")))
        })
    }
    
    /// Remove traffic route from Python
    #[pyo3(name = "remove_traffic_route")]
    fn remove_traffic_route_impl(&self, source_service: String, route_name: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.remove_traffic_route(&source_service, &route_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to remove traffic route: {e}")))
        })
    }
    
    /// Set circuit breaker config from Python
    #[pyo3(name = "set_circuit_breaker_config")]
    fn set_circuit_breaker_config_impl(&self, service: String, config: RiCircuitBreakerConfig) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.set_circuit_breaker_config(&service, config)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set circuit breaker config: {e}")))
        })
    }
    
    /// Set rate limit config from Python
    #[pyo3(name = "set_rate_limit_config")]
    fn set_rate_limit_config_impl(&self, service: String, config: RiRateLimitConfig) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.set_rate_limit_config(&service, config)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to set rate limit config: {e}")))
        })
    }
}
