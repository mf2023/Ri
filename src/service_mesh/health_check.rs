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

//! # Health Check Module
//! 
//! This module provides health checking functionality for the DMS service mesh. It allows
//! monitoring the health of services using various protocols and provides comprehensive
//! health status information.
//! 
//! ## Key Components
//! 
//! - **DMSHealthCheckConfig**: Configuration for health checks
//! - **DMSHealthCheckResult**: Result of a health check
//! - **DMSHealthCheckType**: Supported health check types
//! - **DMSHealthCheckProvider**: Trait for implementing health check providers
//! - **DMSHttpHealthCheckProvider**: HTTP health check implementation
//! - **DMSTcpHealthCheckProvider**: TCP health check implementation
//! - **DMSHealthChecker**: Main health checking service
//! - **DMSHealthStatus**: Health status enum
//! - **DMSHealthSummary**: Summary of health check results
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
//! 10. **Error Handling**: Comprehensive error handling with DMSResult
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use std::time::Duration;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a health checker with 30-second intervals
//!     let health_checker = DMSHealthChecker::new(Duration::from_secs(30));
//!     
//!     // Register a health check for a service
//!     let config = DMSHealthCheckConfig {
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
//!         DMSHealthCheckType::Http,
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

use crate::core::{DMSResult, DMSError};

/// Configuration for health checks.
///
/// This struct defines the parameters for performing health checks, including
/// endpoint, HTTP method, timeout, expected status code, and custom headers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSHealthCheckConfig {
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

impl Default for DMSHealthCheckConfig {
    /// Creates a default health check configuration.
    ///
    /// # Returns
    ///
    /// A `DMSHealthCheckConfig` instance with default values
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
#[derive(Debug, Clone)]
pub struct DMSHealthCheckResult {
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

/// Types of health checks supported.
///
/// This enum defines the different protocols that can be used for health checking.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DMSHealthCheckType {
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
pub trait DMSHealthCheckProvider: Send + Sync {
    /// Performs a health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The endpoint to check
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSResult<DMSHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, config: &DMSHealthCheckConfig) -> DMSResult<DMSHealthCheckResult>;
}

/// HTTP health check provider.
///
/// This struct implements the `DMSHealthCheckProvider` trait for HTTP health checks.
pub struct DMSHttpHealthCheckProvider;

#[async_trait]
impl DMSHealthCheckProvider for DMSHttpHealthCheckProvider {
    /// Performs an HTTP health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The HTTP endpoint to check
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSResult<DMSHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, _config: &DMSHealthCheckConfig) -> DMSResult<DMSHealthCheckResult> {
        let start_time = SystemTime::now();
        
        let client = hyper::Client::new();

        let uri: hyper::Uri = endpoint.parse()
            .map_err(|e| DMSError::ServiceMesh(format!("Invalid URI: {e}")))?;

        let req = hyper::Request::builder()
            .method(_config.method.as_str())
            .uri(uri)
            .body(hyper::body::Body::empty())
            .map_err(|e| DMSError::ServiceMesh(format!("Failed to build request: {e}")))?;

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

                Ok(DMSHealthCheckResult {
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

                Ok(DMSHealthCheckResult {
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

/// TCP health check provider.
///
/// This struct implements the `DMSHealthCheckProvider` trait for TCP health checks.
pub struct DMSTcpHealthCheckProvider;

#[async_trait]
impl DMSHealthCheckProvider for DMSTcpHealthCheckProvider {
    /// Performs a TCP health check on the specified endpoint.
    ///
    /// # Parameters
    ///
    /// - `endpoint`: The TCP endpoint to check (format: "host:port")
    /// - `config`: Health check configuration
    ///
    /// # Returns
    ///
    /// A `DMSResult<DMSHealthCheckResult>` containing the health check result
    async fn check_health(&self, endpoint: &str, _config: &DMSHealthCheckConfig) -> DMSResult<DMSHealthCheckResult> {
        let start_time = SystemTime::now();
        
        match tokio::net::TcpStream::connect(endpoint).await {
            Ok(_) => {
                let response_time = SystemTime::now().duration_since(start_time)
                    .unwrap_or(Duration::from_secs(0));

                Ok(DMSHealthCheckResult {
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

                Ok(DMSHealthCheckResult {
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
pub struct DMSHealthChecker {
    /// Interval between health checks
    check_interval: Duration,
    /// Map of health check providers by type
    providers: Arc<RwLock<HashMap<DMSHealthCheckType, Box<dyn DMSHealthCheckProvider>>>>,
    /// Map of service names to health check results
    check_results: Arc<RwLock<HashMap<String, Vec<DMSHealthCheckResult>>>>,
    /// Background task handles for continuous health monitoring
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

impl DMSHealthChecker {
    /// Creates a new health checker with the specified check interval.
    ///
    /// # Parameters
    ///
    /// - `check_interval`: Interval between health checks
    ///
    /// # Returns
    ///
    /// A new `DMSHealthChecker` instance
    pub fn new(check_interval: Duration) -> Self {
        let mut providers: HashMap<DMSHealthCheckType, Box<dyn DMSHealthCheckProvider>> = HashMap::new();
        providers.insert(DMSHealthCheckType::Http, Box::new(DMSHttpHealthCheckProvider));
        providers.insert(DMSHealthCheckType::Tcp, Box::new(DMSTcpHealthCheckProvider));

        Self {
            check_interval,
            providers: Arc::new(RwLock::new(providers)),
            check_results: Arc::new(RwLock::new(HashMap::new())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
        }
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
    /// A `DMSResult<()>` indicating success or failure
    pub async fn register_health_check(
        &self,
        service_name: &str,
        endpoint: &str,
        check_type: DMSHealthCheckType,
        config: DMSHealthCheckConfig,
    ) -> DMSResult<()> {
        let providers = self.providers.read().await;
        let provider = providers.get(&check_type)
            .ok_or_else(|| DMSError::ServiceMesh(format!("Health check provider for {check_type:?} not found")))?;

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
    /// A `DMSResult<()>` indicating success or failure
    pub async fn start_health_check(&self, service_name: &str, endpoint: &str) -> DMSResult<()> {
        let mut tasks = self.background_tasks.write().await;
        
        let service_name_clone = service_name.to_string();
        let endpoint_clone = endpoint.to_string();
        let check_interval = self.check_interval;
        let providers = Arc::clone(&self.providers);
        let check_results = Arc::clone(&self.check_results);

        let task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(check_interval);
            let config = DMSHealthCheckConfig::default();
            
            loop {
                interval.tick().await;
                
                let providers_guard = providers.read().await;
                if let Some(provider) = providers_guard.get(&DMSHealthCheckType::Http) {
                    match provider.check_health(&endpoint_clone, &config).await {
                        Ok(result) => {
                            let mut results = check_results.write().await;
                            let service_results = results.entry(service_name_clone.clone())
                                .or_insert_with(Vec::new);
                            service_results.push(result);
                        }
                        Err(e) => {
                            eprintln!("Health check failed for {endpoint_clone}: {e}");
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
    /// A `DMSResult<Vec<DMSHealthCheckResult>>` containing the health check results
    pub async fn get_health_status(&self, service_name: &str) -> DMSResult<Vec<DMSHealthCheckResult>> {
        let check_results = self.check_results.read().await;
        let results = check_results.get(service_name)
            .cloned()
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
    /// A `DMSResult<DMSHealthSummary>` containing the health summary
    pub async fn get_service_health_summary(&self, service_name: &str) -> DMSResult<DMSHealthSummary> {
        let results = self.get_health_status(service_name).await?;
        
        if results.is_empty() {
            return Ok(DMSHealthSummary {
                service_name: service_name.to_string(),
                total_checks: 0,
                healthy_checks: 0,
                unhealthy_checks: 0,
                success_rate: 0.0,
                average_response_time: Duration::from_secs(0),
                last_check_time: None,
                overall_status: DMSHealthStatus::Unknown,
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
            DMSHealthStatus::Healthy
        } else if success_rate >= 50.0 {
            DMSHealthStatus::Degraded
        } else {
            DMSHealthStatus::Unhealthy
        };

        Ok(DMSHealthSummary {
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
    /// This method is a placeholder for future implementation.
    ///
    /// # Returns
    ///
    /// A `DMSResult<()>` indicating success or failure
    pub async fn start_background_tasks(&self) -> DMSResult<()> {
        Ok(())
    }

    /// Stops all background health check tasks.
    ///
    /// This method aborts all running background health check tasks and cleans up resources.
    ///
    /// # Returns
    ///
    /// A `DMSResult<()>` indicating success or failure
    pub async fn stop_background_tasks(&self) -> DMSResult<()> {
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
    /// A `DMSResult<bool>` indicating whether the health checker is healthy
    pub async fn health_check(&self) -> DMSResult<bool> {
        Ok(true)
    }
}

/// Health status enum.
///
/// This enum represents the overall health status of a service.
#[derive(Debug, Clone)]
pub enum DMSHealthStatus {
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
#[derive(Debug, Clone)]
pub struct DMSHealthSummary {
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
    pub overall_status: DMSHealthStatus,
}