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
use tokio::task::JoinHandle;

use crate::core::{DMSResult, DMSError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSHealthCheckConfig {
    pub endpoint: String,
    pub method: String,
    pub timeout: Duration,
    pub expected_status_code: u16,
    pub expected_response_body: Option<String>,
    pub headers: HashMap<String, String>,
}

impl Default for DMSHealthCheckConfig {
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

#[derive(Debug, Clone)]
pub struct DMSHealthCheckResult {
    pub service_name: String,
    pub endpoint: String,
    pub is_healthy: bool,
    pub status_code: Option<u16>,
    pub response_time: Duration,
    pub error_message: Option<String>,
    pub timestamp: SystemTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DMSHealthCheckType {
    Http,
    Tcp,
    Grpc,
    Custom,
}

#[async_trait]
pub trait DMSHealthCheckProvider: Send + Sync {
    async fn check_health(&self, endpoint: &str, config: &DMSHealthCheckConfig) -> DMSResult<DMSHealthCheckResult>;
}

pub struct DMSHttpHealthCheckProvider;

#[async_trait]
impl DMSHealthCheckProvider for DMSHttpHealthCheckProvider {
    async fn check_health(&self, endpoint: &str, _config: &DMSHealthCheckConfig) -> DMSResult<DMSHealthCheckResult> {
        let start_time = SystemTime::now();
        
        let client = hyper::Client::new();

        let uri: hyper::Uri = endpoint.parse()
            .map_err(|e| DMSError::ServiceMesh(format!("Invalid URI: {}", e)))?;

        let req = hyper::Request::builder()
            .method(_config.method.as_str())
            .uri(uri)
            .body(hyper::body::Body::empty())
            .map_err(|e| DMSError::ServiceMesh(format!("Failed to build request: {}", e)))?;

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

pub struct DMSTcpHealthCheckProvider;

#[async_trait]
impl DMSHealthCheckProvider for DMSTcpHealthCheckProvider {
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

pub struct DMSHealthChecker {
    check_interval: Duration,
    providers: Arc<RwLock<HashMap<DMSHealthCheckType, Box<dyn DMSHealthCheckProvider>>>>,
    check_results: Arc<RwLock<HashMap<String, Vec<DMSHealthCheckResult>>>>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
}

impl DMSHealthChecker {
    pub fn _Fnew(check_interval: Duration) -> Self {
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

    pub async fn register_health_check(
        &self,
        service_name: &str,
        endpoint: &str,
        check_type: DMSHealthCheckType,
        config: DMSHealthCheckConfig,
    ) -> DMSResult<()> {
        let providers = self.providers.read().await;
        let provider = providers.get(&check_type)
            .ok_or_else(|| DMSError::ServiceMesh(format!("Health check provider for {:?} not found", check_type)))?;

        let result = provider.check_health(endpoint, &config).await?;
        
        let mut check_results = self.check_results.write().await;
        let service_results = check_results.entry(service_name.to_string())
            .or_insert_with(Vec::new);
        service_results.push(result);

        Ok(())
    }

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
                            eprintln!("Health check failed for {}: {}", endpoint_clone, e);
                        }
                    }
                }
            }
        });

        tasks.push(task);
        Ok(())
    }

    pub async fn get_health_status(&self, service_name: &str) -> DMSResult<Vec<DMSHealthCheckResult>> {
        let check_results = self.check_results.read().await;
        let results = check_results.get(service_name)
            .cloned()
            .unwrap_or_default();

        Ok(results)
    }

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

    pub async fn start_background_tasks(&self) -> DMSResult<()> {
        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> DMSResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn _Fhealth_check(&self) -> DMSResult<bool> {
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub enum DMSHealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DMSHealthSummary {
    pub service_name: String,
    pub total_checks: usize,
    pub healthy_checks: usize,
    pub unhealthy_checks: usize,
    pub success_rate: f64,
    pub average_response_time: Duration,
    pub last_check_time: Option<SystemTime>,
    pub overall_status: DMSHealthStatus,
}