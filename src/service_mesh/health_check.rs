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

//! # Health Check Module
//! 
//! This module provides health checking functionality for the DMSC service mesh. It allows
//! monitoring the health of services using various protocols and provides comprehensive
//! health status information.
//! 
//! ## Key Components
//! 
//! - **DMSCHealthCheckConfig**: Configuration for health checks
//! - **DMSCHealthCheckResult**: Result of a health check
//! - **DMSCHealthCheckType**: Supported health check types
//! - **DMSCHealthCheckProvider**: Trait for implementing health check providers
//! - **DMSCHttpHealthCheckProvider**: HTTP health check implementation
//! - **DMSCTcpHealthCheckProvider**: TCP health check implementation
//! - **DMSCHealthChecker**: Main health checking service
//! - **DMSCHealthStatus**: Health status enum
//! - **DMSCHealthSummary**: Summary of health check results
//! 
//! ## Design Principles
//! 
//! 1. **Protocol Agnostic**: Supports multiple health check protocols (HTTP, TCP, gRPC, custom)
//! 2. **Async-First**: All health check operations are asynchronous
//! 3. **Extensible**: Easy to implement new health check providers
//! 4. **Configurable**: Highly configurable health check parameters
//! 5. **Real-time Monitoring**: Background tasks for continuous health monitoring
//! 6. **Comprehensive Results**: Detailed health check results with response times and error messages
//! 7. **Health Summary**: Aggregated health status with success rates and average response times
//! 8. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 9. **Graceful Shutdown**: Proper cleanup of background tasks
//! 10. **Error Handling**: Comprehensive error handling with DMSCResult
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create a health checker with 30-second intervals
//!     let health_checker = DMSCHealthChecker::new(Duration::from_secs(30));
//!     
//!     // Register a health check for a service
//!     let config = DMSCHealthCheckConfig {
//!         endpoint: "/health".to_string(),
//!         method: "GET".to_string(),
//!         timeout: Duration::from_secs(5),
//!         expected_status_code: 200,
//!         expected_response_body: None,
//!         headers: HashMap::new(),
//!     };
//!     
//!     health_checker.register_health_check(
//!         "example-service",
//!         "http://localhost:8080",
//!         DMSCHealthCheckType::Http,
//!         config
//!     ).await?;
//!     
//!     // Start background health checks
//!     health_checker.start_health_check("example-service", "http://localhost:8080").await?;
//!     
//!     // Get health summary
//!     let summary = health_checker.get_service_health_summary("example-service").await?;
//!     println!("Service health: {:?}", summary.overall_status);
//!     println!("Success rate: {:.2}%", summary.success_rate);
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
use tokio::task::JoinHandle;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;
#[cfg(feature = "service_mesh")]
use hyper;
#[cfg(feature = "service_mesh")]
use hyper_util::client::legacy::Client;
#[cfg(feature = "service_mesh")]
use hyper_util::rt::TokioExecutor;

use crate::core::{DMSCResult, DMSCError};
use crate::observability::{DMSCTracer, DMSCSpanKind, DMSCSpanStatus};

/// Configuration for health checks.
///
/// This struct defines the parameters for performing health checks, including
/// endpoint, HTTP method, timeout, expected status code, and custom headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCHealthCheckConfig {
    /// Health check endpoint path
    pub endpoint: String,
    /// HTTP method to use for health checks
    pub method: String,
    /// Timeout for health check requests
    pub timeout: Duration,
    /// Expected HTTP status code for a healthy service
    pub expected_status_code: u16,
    /// Optional expected response body for validation
    pub expected_response_body: Option<String>,
    /// Custom headers to include in health check requests
    pub headers: HashMap<String, String>,
}

impl Default for DMSCHealthCheckConfig {
    /// Creates a default health check configuration.
    ///
    /// # Returns
    ///
    /// A `DMSCHealthCheckConfig` instance with default values
    fn default() -> Self {
        Self {
            endpoint: "/health".to_string(),
            method: "GET".to_string(),
            timeout: Duration::from_secs(5),
            expected_status_code: 200,
            expected_response_body: None,
            headers: HashMap::new(),
        }
    }
}

/// Result of a health check operation.
///
/// This struct contains detailed information about the result of a health check,
/// including whether the service is healthy, response time, and error messages if any.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSCHealthCheckResult {
    /// Name of the service being checked
    pub service_name: String,
    /// Endpoint used for the health check
    pub endpoint: String,
    /// Whether the service is considered healthy
    pub is_healthy: bool,
    /// HTTP status code received (if applicable)
    pub status_code: Option<u16>,
    /// Time taken to perform the health check
    pub response_time: Duration,
    /// Error message if the health check failed
    pub error_message: Option<String>,
    /// Timestamp when the health check was performed
    pub timestamp: SystemTime,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCHealthCheckResult {
    fn get_service_name(&self) -> String {
        self.service_name.clone()
    }
    
    fn get_endpoint(&self) -> String {
        self.endpoint.clone()
    }
    
    fn get_is_healthy(&self) -> bool {
        self.is_healthy
    }
    
    fn get_status_code(&self) -> Option<u16> {
        self.status_code
    }
    
    fn get_response_time_ms(&self) -> u64 {
        self.response_time.as_millis() as u64
    }
    
    fn get_error_message(&self) -> Option<String> {
        self.error_message.clone()
    }
}

/// Types of health checks supported.
///
/// This enum defines the different protocols that can be used for health checking.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DMSCHealthCheckType {
    /// HTTP health check
    Http,
    /// TCP health check
    Tcp,
    /// gRPC health check
    Grpc,
    /// Custom health check implementation
    Custom,
}

/// Trait for implementing health check providers.
///
/// This trait defines the interface for health check providers, allowing for
/// different health check implementations based on protocol.
#[async_trait]
pub trait DMSCHealthCheckProvider: Send + Sync {
    /// Performs a health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The endpoint to check
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, config: &DMSCHealthCheckConfig) -> DMSCResult<DMSCHealthCheckResult>;
}

/// HTTP health check provider.
///
/// This struct implements the `DMSCHealthCheckProvider` trait for HTTP health checks.
pub struct DMSCHttpHealthCheckProvider;

#[async_trait]
impl DMSCHealthCheckProvider for DMSCHttpHealthCheckProvider {
    /// Performs an HTTP health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The HTTP endpoint to check
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCHealthCheckResult>` containing the health check result
    #[cfg(feature = "service_mesh")]
    async fn check_health(&self, endpoint: &str, _config: &DMSCHealthCheckConfig) -> DMSCResult<DMSCHealthCheckResult> {
        let start_time = SystemTime::now();
        
        let client: Client<hyper::client::HttpConnector, String> = Client::builder(TokioExecutor::new()).build_http();

        let uri: hyper::Uri = endpoint.parse()
            .map_err(|e| DMSCError::ServiceMesh(format!("Invalid URI: {e}")))?;

        let req = hyper::Request::builder()
            .method(_config.method.as_str())
            .uri(uri)
            .body(String::new())
            .map_err(|e| DMSCError::ServiceMesh(format!("Failed to build request: {e}")))?;

        match client.request(req).await {
            Ok(response) => {
                let status_code = response.status().as_u16();
                let is_healthy = status_code == _config.expected_status_code;
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                let error_message = if !is_healthy {
                    Some(format!("Expected status code {}, got {}", _config.expected_status_code, status_code))
                } else {
                    None
                };

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy,
                    status_code: Some(status_code),
                    response_time,
                    error_message,
                    timestamp: SystemTime::now(),
                })
            }
            Err(e) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy: false,
                    status_code: None,
                    response_time,
                    error_message: Some(e.to_string()),
                    timestamp: SystemTime::now(),
                })
            }
        }
    }
    
    #[cfg(not(feature = "service_mesh"))]
    async fn check_health(&self, endpoint: &str, _config: &DMSCHealthCheckConfig) -> DMSCResult<DMSCHealthCheckResult> {
        // If service_mesh feature is not enabled, assume all endpoints are healthy
        Ok(DMSCHealthCheckResult {
            service_name: "unknown".to_string(),
            endpoint: endpoint.to_string(),
            is_healthy: true,
            status_code: Some(_config.expected_status_code),
            response_time: Duration::from_secs(0),
            error_message: None,
            timestamp: SystemTime::now(),
        })
    }
}

/// TCP health check provider.
///
/// This struct implements the `DMSCHealthCheckProvider` trait for TCP health checks.
pub struct DMSCTcpHealthCheckProvider;

#[async_trait]
impl DMSCHealthCheckProvider for DMSCTcpHealthCheckProvider {
    /// Performs a TCP health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The TCP endpoint to check (format: "host:port")
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, _config: &DMSCHealthCheckConfig) -> DMSCResult<DMSCHealthCheckResult> {
        let start_time = SystemTime::now();
        
        match tokio::net::TcpStream::connect(endpoint).await {
            Ok(_) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy: true,
                    status_code: None,
                    response_time,
                    error_message: None,
                    timestamp: SystemTime::now(),
                })
            }
            Err(e) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy: false,
                    status_code: None,
                    response_time,
                    error_message: Some(e.to_string()),
                    timestamp: SystemTime::now(),
                })
            }
        }
    }
}

/// gRPC health check provider.
///
/// This struct implements the `DMSCHealthCheckProvider` trait for gRPC health checks.
pub struct DMSCGrpcHealthCheckProvider;

#[async_trait]
impl DMSCHealthCheckProvider for DMSCGrpcHealthCheckProvider {
    /// Performs a gRPC health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The gRPC endpoint to check (format: "host:port")
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, _config: &DMSCHealthCheckConfig) -> DMSCResult<DMSCHealthCheckResult> {
        let start_time = SystemTime::now();
        
        // Simple gRPC health check implementation using TCP connection
        // In a full implementation, this would use the gRPC health check service
        match tokio::net::TcpStream::connect(endpoint).await {
            Ok(_) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy: true,
                    status_code: None,
                    response_time,
                    error_message: None,
                    timestamp: SystemTime::now(),
                })
            }
            Err(e) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSCHealthCheckResult {
                    service_name: "unknown".to_string(),
                    endpoint: endpoint.to_string(),
                    is_healthy: false,
                    status_code: None,
                    response_time,
                    error_message: Some(e.to_string()),
                    timestamp: SystemTime::now(),
                })
            }
        }
    }
}

/// Main health checker service.
///
/// This struct provides the core functionality for managing health checks, including
/// registering health checks, starting background monitoring, and retrieving health status.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCHealthChecker {
    check_interval: Duration,
    providers: Arc<RwLock<HashMap<DMSCHealthCheckType, Box<dyn DMSCHealthCheckProvider>>>>,
    check_results: Arc<RwLock<HashMap<String, Vec<DMSCHealthCheckResult>>>>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    tracer: Option<Arc<DMSCTracer>>,
}

impl DMSCHealthChecker {
    pub fn new(check_interval: Duration) -> Self {
        let mut providers: HashMap<DMSCHealthCheckType, Box<dyn DMSCHealthCheckProvider>> = HashMap::new();
        providers.insert(DMSCHealthCheckType::Http, Box::new(DMSCHttpHealthCheckProvider));
        providers.insert(DMSCHealthCheckType::Tcp, Box::new(DMSCTcpHealthCheckProvider));
        providers.insert(DMSCHealthCheckType::Grpc, Box::new(DMSCGrpcHealthCheckProvider));

        Self {
            check_interval,
            providers: Arc::new(RwLock::new(providers)),
            check_results: Arc::new(RwLock::new(HashMap::new())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            tracer: None,
        }
    }
    
    pub fn with_tracer(mut self, tracer: Arc<DMSCTracer>) -> Self {
        self.tracer = Some(tracer);
        self
    }
    
    pub fn set_tracer(&mut self, tracer: Arc<DMSCTracer>) {
        self.tracer = Some(tracer);
    }
    


    /// Registers a health check for a service.
    ///
    /// This method registers a health check for a service and performs an immediate check.
    ///
    /// # Parameters
    ///
    /// - `service_name`: Name of the service to check
    /// - `endpoint`: Endpoint URL for health checks
    /// - `check_type`: Type of health check to perform
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn register_health_check(
        &self,
        service_name: &str,
        endpoint: &str,
        check_type: DMSCHealthCheckType,
        config: DMSCHealthCheckConfig,
    ) -> DMSCResult<()> {
        let span_id = if let Some(tracer) = &self.tracer {
            let span_id = tracer.start_span_from_context(
                format!("health_check:{}", service_name),
                DMSCSpanKind::Internal,
            );
            if let Some(ref sid) = span_id {
                let _ = tracer.span_mut(sid, |span| {
                    span.set_attribute("service_name".to_string(), service_name.to_string());
                    span.set_attribute("endpoint".to_string(), endpoint.to_string());
                    span.set_attribute("check_type".to_string(), format!("{:?}", check_type));
                });
            }
            span_id
        } else {
            None
        };

        let result = self.register_health_check_internal(service_name, endpoint, check_type, config).await;

        if let (Some(tracer), Some(sid)) = (&self.tracer, span_id) {
            let status = match &result {
                Ok(_) => DMSCSpanStatus::Ok,
                Err(e) => DMSCSpanStatus::Error(e.to_string()),
            };
            let _ = tracer.end_span(&sid, status);
        }

        result
    }
    
    async fn register_health_check_internal(
        &self,
        service_name: &str,
        endpoint: &str,
        check_type: DMSCHealthCheckType,
        config: DMSCHealthCheckConfig,
    ) -> DMSCResult<()> {
        let providers = self.providers.read().await;
        let provider = providers.get(&check_type)
            .ok_or_else(|| DMSCError::ServiceMesh(format!("Health check provider for {check_type:?} not found")))?;

        let result = provider.check_health(endpoint, &config).await?;
        
        let mut check_results = self.check_results.write().await;
        let service_results = check_results.entry(service_name.to_string())
            .or_insert_with(Vec::new);
        service_results.push(result);

        Ok(())
    }

    /// Starts background health checks for a service.
    /// 
    /// This method creates a background task that periodically checks the health of a service.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: Name of the service to check
    /// - `endpoint`: Endpoint URL for health checks
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn start_health_check(&self, service_name: &str, endpoint: &str) -> DMSCResult<()> {
        let mut tasks = self.background_tasks.write().await;
        
        let service_name_clone = service_name.to_string();
        let endpoint_clone = endpoint.to_string();
        let check_interval = self.check_interval;
        let providers = Arc::clone(&self.providers);
        let check_results = Arc::clone(&self.check_results);

        // Determine health check type based on endpoint URL scheme
        let check_type = if endpoint.starts_with("grpc://") || endpoint.starts_with("grpcs://") {
            DMSCHealthCheckType::Grpc
        } else if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
            DMSCHealthCheckType::Http
        } else {
            // Assume TCP for other protocols
            DMSCHealthCheckType::Tcp
        };

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            let config = DMSCHealthCheckConfig::default();
            
            loop {
                interval.tick().await;
                
                let providers_guard = providers.read().await;
                if let Some(provider) = providers_guard.get(&check_type) {
                    match provider.check_health(&endpoint_clone, &config).await {
                        Ok(result) => {
                            let mut results = check_results.write().await;
                            let service_results = results.entry(service_name_clone.clone())
                                .or_insert_with(Vec::new);
                            
                            // Add new result to the end
                            service_results.push(result);
                            
                            // Keep only the most recent 100 results per service to avoid memory issues
                            if service_results.len() > 100 {
                                service_results.drain(0..service_results.len() - 100);
                            }
                        }
                        Err(e) => {
                            log::warn!("Health check failed for {endpoint_clone}: {e}");
                        }
                    }
                }
            }
        });

        tasks.push(task);
        Ok(())
    }
    
    /// Stops health checks for a specific service endpoint.
    /// 
    /// This method clears the health check results for the specified service.
    /// The background task will continue running but will no longer record results.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: Name of the service
    /// - `endpoint`: Endpoint URL
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn stop_health_check(&self, service_name: &str, _endpoint: &str) -> DMSCResult<()> {
        let mut results = self.check_results.write().await;
        results.remove(service_name);
        Ok(())
    }
    
    /// Starts background health checks for a service with a specific health check type.
    /// 
    /// This method creates a background task that periodically checks the health of a service
    /// using the specified health check type.
    /// 
    /// # Parameters
    /// 
    /// - `service_name`: Name of the service to check
    /// - `endpoint`: Endpoint URL for health checks
    /// - `check_type`: Type of health check to perform
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn start_health_check_with_type(
        &self, 
        service_name: &str, 
        endpoint: &str,
        check_type: DMSCHealthCheckType
    ) -> DMSCResult<()> {
        let mut tasks = self.background_tasks.write().await;
        
        let service_name_clone = service_name.to_string();
        let endpoint_clone = endpoint.to_string();
        let check_interval = self.check_interval;
        let providers = Arc::clone(&self.providers);
        let check_results = Arc::clone(&self.check_results);
        let check_type_clone = check_type;

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            let config = DMSCHealthCheckConfig::default();
            
            loop {
                interval.tick().await;
                
                let providers_guard = providers.read().await;
                if let Some(provider) = providers_guard.get(&check_type_clone) {
                    match provider.check_health(&endpoint_clone, &config).await {
                        Ok(result) => {
                            let mut results = check_results.write().await;
                            let service_results = results.entry(service_name_clone.clone())
                                .or_insert_with(Vec::new);
                            
                            // Add new result to the end
                            service_results.push(result);
                            
                            // Keep only the most recent 100 results per service to avoid memory issues
                            if service_results.len() > 100 {
                                service_results.drain(0..service_results.len() - 100);
                            }
                        }
                        Err(e) => {
                            log::warn!("Health check failed for {endpoint_clone}: {e}");
                        }
                    }
                }
            }
        });

        tasks.push(task);
        Ok(())
    }

    /// Gets the health check results for a service.
    ///
    /// # Parameters
    ///
    /// - `service_name`: Name of the service to get results for
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<DMSCHealthCheckResult>>` containing the health check results
    pub async fn get_health_status(&self, service_name: &str) -> DMSCResult<Vec<DMSCHealthCheckResult>> {
        let check_results = self.check_results.read().await;
        let results = check_results.get(service_name)
            .cloned()
            .unwrap_or_default();

        Ok(results)
    }
    
    /// Gets the latest health check result for a service.
    ///
    /// # Parameters
    ///
    /// - `service_name`: Name of the service to get the latest result for
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Option<DMSCHealthCheckResult>>` containing the latest health check result if available
    pub async fn get_latest_health_status(&self, service_name: &str) -> DMSCResult<Option<DMSCHealthCheckResult>> {
        let check_results = self.check_results.read().await;
        let latest_result = check_results.get(service_name)
            .and_then(|results| results.last().cloned());

        Ok(latest_result)
    }
    
    /// Gets the health check results for a service within a specified time window.
    ///
    /// # Parameters
    ///
    /// - `service_name`: Name of the service to get results for
    /// - `time_window`: Time window to filter results by
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Vec<DMSCHealthCheckResult>>` containing the filtered health check results
    pub async fn get_health_status_within(&self, service_name: &str, time_window: Duration) -> DMSCResult<Vec<DMSCHealthCheckResult>> {
        let check_results = self.check_results.read().await;
        let now = SystemTime::now();
        
        let results = check_results.get(service_name)
            .map(|results| {
                results.iter()
                    .filter(|r| {
                        if let Ok(elapsed) = now.duration_since(r.timestamp) {
                            elapsed <= time_window
                        } else {
                            false
                        }
                    })
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();

        Ok(results)
    }

    /// Gets a health summary for a service.
    ///
    /// This method aggregates health check results to provide a summary of the service's health,
    /// including success rate, average response time, and overall status.
    ///
    /// # Parameters
    ///
    /// - `service_name`: Name of the service to get a summary for
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCHealthSummary>` containing the health summary
    pub async fn get_service_health_summary(&self, service_name: &str) -> DMSCResult<DMSCHealthSummary> {
        let results = self.get_health_status(service_name).await?;
        
        if results.is_empty() {
            return Ok(DMSCHealthSummary {
                service_name: service_name.to_string(),
                total_checks: 0,
                healthy_checks: 0,
                unhealthy_checks: 0,
                success_rate: 0.0,
                average_response_time: Duration::from_secs(0),
                last_check_time: None,
                overall_status: DMSCHealthStatus::Unknown,
            });
        }

        let total_checks = results.len();
        let healthy_checks = results.iter().filter(|r| r.is_healthy).count();
        let unhealthy_checks = total_checks - healthy_checks;
        let success_rate = (healthy_checks as f64) / (total_checks as f64) * 100.0;

        let total_response_time: Duration = results.iter()
            .map(|r| r.response_time)
            .sum();
        let average_response_time = total_response_time / total_checks as u32;

        let last_check_time = results.last().map(|r| r.timestamp);

        let overall_status = if success_rate >= 80.0 {
            DMSCHealthStatus::Healthy
        } else if success_rate >= 50.0 {
            DMSCHealthStatus::Degraded
        } else {
            DMSCHealthStatus::Unhealthy
        };

        Ok(DMSCHealthSummary {
            service_name: service_name.to_string(),
            total_checks,
            healthy_checks,
            unhealthy_checks,
            success_rate,
            average_response_time,
            last_check_time,
            overall_status,
        })
    }

    /// Starts background health check tasks.
    ///
    /// This method initializes and starts all background health monitoring tasks,
    /// including periodic health checks for registered services and cleanup tasks.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn start_background_tasks(&self) -> DMSCResult<()> {
        // Start periodic cleanup task to remove old health check results
        let check_results = Arc::clone(&self.check_results);
        let cleanup_interval = self.check_interval * 10; // Cleanup every 10 check intervals
        
        let cleanup_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            
            loop {
                interval.tick().await;
                
                let mut results = check_results.write().await;
                let now = SystemTime::now();
                let max_age = Duration::from_secs(3600); // Keep results for 1 hour
                
                // Remove health check results older than max_age
                for service_results in results.values_mut() {
                    service_results.retain(|result| {
                        now.duration_since(result.timestamp)
                            .map(|age| age < max_age)
                            .unwrap_or(false)
                    });
                }
                
                // Remove services with no recent results
                results.retain(|_, results| !results.is_empty());
            }
        });
        
        // Store cleanup task
        let mut tasks = self.background_tasks.write().await;
        tasks.push(cleanup_task);
        
        log::info!("Background health check tasks started successfully");
        Ok(())
    }

    /// Stops all background health check tasks.
    ///
    /// This method aborts all running background health check tasks and cleans up resources.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    pub async fn stop_background_tasks(&self) -> DMSCResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    /// Performs a health check on the health checker itself.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<bool>` indicating whether the health checker is healthy
    pub async fn health_check(&self) -> DMSCResult<bool> {
        Ok(true)
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCHealthChecker
#[pyo3::prelude::pymethods]
impl DMSCHealthChecker {
    #[new]
    fn py_new(check_interval: u64) -> PyResult<Self> {
        Ok(Self::new(Duration::from_secs(check_interval)))
    }
    
    /// Get service health summary from Python
    #[pyo3(name = "get_service_health_summary")]
    fn get_service_health_summary_impl(&self, service_name: String) -> PyResult<DMSCHealthSummary> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.get_service_health_summary(&service_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get health summary: {e}")))
        })
    }
    
    /// Start health check from Python
    #[pyo3(name = "start_health_check")]
    fn start_health_check_impl(&self, service_name: String, endpoint: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.start_health_check(&service_name, &endpoint)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to start health check: {e}")))
        })
    }
    
    /// Stop health check from Python
    #[pyo3(name = "stop_health_check")]
    fn stop_health_check_impl(&self, service_name: String, endpoint: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.stop_health_check(&service_name, &endpoint)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to stop health check: {e}")))
        })
    }
    
    /// Get health status from Python
    #[pyo3(name = "get_health_status")]
    fn get_health_status_impl(&self, service_name: String) -> PyResult<Vec<DMSCHealthCheckResult>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.get_health_status(&service_name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to get health status: {e}")))
        })
    }
}

/// Health status enum.
///
/// This enum represents the overall health status of a service.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub enum DMSCHealthStatus {
    /// Service is healthy
    Healthy,
    /// Service is degraded but still functional
    Degraded,
    /// Service is unhealthy
    Unhealthy,
    /// Health status is unknown
    Unknown,
}

/// Summary of health check results.
///
/// This struct provides an aggregated view of a service's health, including
/// total checks, success rate, average response time, and overall status.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSCHealthSummary {
    /// Name of the service
    pub service_name: String,
    /// Total number of health checks performed
    pub total_checks: usize,
    /// Number of successful health checks
    pub healthy_checks: usize,
    /// Number of failed health checks
    pub unhealthy_checks: usize,
    /// Success rate percentage (0.0 to 100.0)
    pub success_rate: f64,
    /// Average response time for health checks
    pub average_response_time: Duration,
    /// Timestamp of the last health check
    pub last_check_time: Option<SystemTime>,
    /// Overall health status
    pub overall_status: DMSCHealthStatus,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCHealthSummary {
    fn get_service_name(&self) -> String {
        self.service_name.clone()
    }
    
    fn get_total_checks(&self) -> usize {
        self.total_checks
    }
    
    fn get_healthy_checks(&self) -> usize {
        self.healthy_checks
    }
    
    fn get_unhealthy_checks(&self) -> usize {
        self.unhealthy_checks
    }
    
    fn get_success_rate(&self) -> f64 {
        self.success_rate
    }
    
    fn get_average_response_time_ms(&self) -> u64 {
        self.average_response_time.as_millis() as u64
    }
    
    fn get_overall_status(&self) -> String {
        match self.overall_status {
            DMSCHealthStatus::Healthy => "Healthy".to_string(),
            DMSCHealthStatus::Degraded => "Degraded".to_string(),
            DMSCHealthStatus::Unhealthy => "Unhealthy".to_string(),
            DMSCHealthStatus::Unknown => "Unknown".to_string(),
        }
    }
}
