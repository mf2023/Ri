//! Copyright 2025-2026 Wenze Wei. All Rights Reserved.
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

//! # gRPC Client Implementation

use super::*;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub struct RiGrpcClient {
    channel: Option<tonic::transport::Channel>,
    endpoint: String,
    timeout: Duration,
    stats: Arc<RwLock<RiGrpcStats>>,
    request_id: Arc<AtomicU64>,
    connected: Arc<RwLock<bool>>,
    retry_count: u32,
    retry_delay: Duration,
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct RiGrpcClientPy {
    inner: RiGrpcClient,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiGrpcClientPy {
    #[new]
    fn new(endpoint: String) -> Self {
        Self {
            inner: RiGrpcClient::new(endpoint),
        }
    }

    #[pyo3(signature = (timeout_secs=30))]
    fn with_timeout(&mut self, timeout_secs: u64) {
        self.inner.timeout = Duration::from_secs(timeout_secs);
    }

    #[pyo3(signature = (count=3, delay_ms=100))]
    fn with_retry(&mut self, count: u32, delay_ms: u64) {
        self.inner.retry_count = count;
        self.inner.retry_delay = Duration::from_millis(delay_ms);
    }

    fn get_stats(&self) -> RiGrpcStats {
        self.inner.get_stats()
    }

    fn is_connected(&self) -> bool {
        self.inner.channel.is_some()
    }

    fn get_endpoint(&self) -> String {
        self.inner.endpoint.clone()
    }

    fn connect(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.connect().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn disconnect(&mut self) {
        if let Ok(rt) = tokio::runtime::Runtime::new() {
            rt.block_on(async {
                self.inner.disconnect().await
            });
        }
    }

    #[pyo3(signature = (service_name, method, data))]
    fn call(&mut self, service_name: String, method: String, data: Vec<u8>) -> PyResult<Vec<u8>> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.call(&service_name, &method, &data).await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

impl RiGrpcClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            channel: None,
            endpoint,
            timeout: Duration::from_secs(30),
            stats: Arc::new(RwLock::new(RiGrpcStats::new())),
            request_id: Arc::new(AtomicU64::new(0)),
            connected: Arc::new(RwLock::new(false)),
            retry_count: 3,
            retry_delay: Duration::from_millis(100),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_retry(mut self, count: u32, delay: Duration) -> Self {
        self.retry_count = count;
        self.retry_delay = delay;
        self
    }

    pub fn get_stats(&self) -> RiGrpcStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiGrpcStats::new())
    }

    pub async fn connect(&mut self) -> RiResult<()> {
        let endpoint = tonic::transport::Endpoint::from_shared(self.endpoint.clone())
            .map_err(|e| GrpcError::ConnectionFailed {
                message: format!("Invalid endpoint: {}", e)
            })?
            .connect_timeout(self.timeout)
            .timeout(self.timeout);

        let channel = endpoint.connect()
            .await
            .map_err(|e| GrpcError::ConnectionFailed {
                message: format!("Connection failed: {}", e)
            })?;

        self.channel = Some(channel);
        *self.connected.write().await = true;

        tracing::info!("gRPC client connected to {}", self.endpoint);
        Ok(())
    }

    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await && self.channel.is_some()
    }

    fn generate_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn call(&mut self, service_name: &str, method: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        let channel = match &self.channel {
            Some(ch) => ch.clone(),
            None => {
                return Err(GrpcError::Client {
                    message: "Not connected to gRPC server".to_string()
                }.into());
            }
        };

        if !*self.connected.read().await {
            return Err(GrpcError::Client {
                message: "gRPC client not connected".to_string()
            }.into());
        }

        let request_id = self.generate_request_id();
        let path = format!("/{}/{}", service_name, method);

        tracing::debug!("gRPC call: {} (request_id={})", path, request_id);

        let mut last_error: Option<RiError> = None;
        for attempt in 0..=self.retry_count {
            if attempt > 0 {
                tokio::time::sleep(self.retry_delay).await;
                tracing::warn!("Retrying gRPC call (attempt {}/{})", attempt, self.retry_count);
            }

            match Self::execute_unary_call(channel.clone(), &path, data).await {
                Ok(response) => {
                    let mut stats = self.stats.write().await;
                    stats.record_request(data.len());
                    stats.record_response(response.len());
                    return Ok(response);
                }
                Err(e) => {
                    last_error = Some(e.clone());
                    let mut stats = self.stats.write().await;
                    stats.record_error();
                    
                    if !Self::is_retryable_error(&e) {
                        return Err(e);
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| GrpcError::Client {
            message: "Unknown error after retries".to_string()
        }.into()))
    }

    async fn execute_unary_call(
        channel: tonic::transport::Channel,
        path: &str,
        data: &[u8],
    ) -> RiResult<Vec<u8>> {
        use tonic::client::Grpc;
        use tonic::codec::ProstCodec;
        
        let codec = ProstCodec::<Vec<u8>, Vec<u8>>::new();
        let mut client = Grpc::new(channel);
        
        let request = tonic::Request::new(data.to_vec());
        let path_and_query: http::uri::PathAndQuery = path.parse()
            .map_err(|e| GrpcError::Client {
                message: format!("Invalid path: {}", e)
            })?;
        
        let response = client.unary(request, path_and_query, codec)
            .await
            .map_err(|e| GrpcError::Client {
                message: format!("RPC call failed: {}", e)
            })?;

        Ok(response.into_inner())
    }

    fn is_retryable_error(error: &RiError) -> bool {
        let error_str = error.to_string();
        error_str.contains("UNAVAILABLE") ||
        error_str.contains("DEADLINE_EXCEEDED") ||
        error_str.contains("RESOURCE_EXHAUSTED")
    }

    pub async fn disconnect(&mut self) {
        self.channel.take();
        *self.connected.write().await = false;
        tracing::info!("gRPC client disconnected from {}", self.endpoint);
    }
}

impl Drop for RiGrpcClient {
    fn drop(&mut self) {
        if self.channel.is_some() {
            if let Ok(rt) = tokio::runtime::Runtime::new() {
                rt.block_on(async {
                    self.disconnect().await;
                });
            }
        }
    }
}

impl Default for RiGrpcClient {
    fn default() -> Self {
        Self::new("http://127.0.0.1:50051".to_string())
    }
}

impl Clone for RiGrpcClient {
    fn clone(&self) -> Self {
        Self {
            channel: self.channel.clone(),
            endpoint: self.endpoint.clone(),
            timeout: self.timeout,
            stats: self.stats.clone(),
            request_id: self.request_id.clone(),
            connected: self.connected.clone(),
            retry_count: self.retry_count,
            retry_delay: self.retry_delay,
        }
    }
}
