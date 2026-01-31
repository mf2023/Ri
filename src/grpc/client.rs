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

//! # gRPC Client Implementation
//!
//! This module provides the gRPC client implementation for DMSC.

use super::*;
use std::time::Duration;
use std::sync::atomic::AtomicU64;

#[pyclass]
pub struct DMSCGrpcClient {
    channel: Option<tonic::transport::Channel>,
    endpoint: String,
    timeout: Duration,
    stats: Arc<RwLock<DMSCGrpcStats>>,
    _request_id: Arc<AtomicU64>,
    connected: Arc<RwLock<bool>>,
}

#[pymethods]
impl DMSCGrpcClient {
    #[new]
    fn new(endpoint: String) -> Self {
        Self {
            channel: None,
            endpoint,
            timeout: Duration::from_secs(30),
            stats: Arc::new(RwLock::new(DMSCGrpcStats::new())),
            _request_id: Arc::new(AtomicU64::new(0)),
            connected: Arc::new(RwLock::new(false)),
        }
    }

    fn get_stats(&self) -> DMSCGrpcStats {
        self.stats.try_read().unwrap().clone()
    }
}

impl DMSCGrpcClient {
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub async fn connect(&mut self) -> DMSCResult<()> {
        let channel = tonic::transport::Channel::from_shared(self.endpoint.clone())
            .map_err(|e| GrpcError::ConnectionFailed {
                message: format!("Invalid endpoint: {}", e)
            })?
            .connect_timeout(self.timeout)
            .connect()
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
        *self.connected.read().await
    }

    pub async fn call(&mut self, _method: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let _channel = match &self.channel {
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

        // Simplified gRPC call - just record stats and return data
        // Full implementation would use tonic client
        let mut stats = self.stats.write().await;
        stats.record_request(data.len());
        stats.record_response(data.len());

        // Placeholder response - in real implementation, this would make actual gRPC call
        Ok(data.to_vec())
    }

    pub async fn disconnect(&mut self) {
        self.channel.take();
        *self.connected.write().await = false;
        tracing::info!("gRPC client disconnected from {}", self.endpoint);
    }
}

impl Drop for DMSCGrpcClient {
    fn drop(&mut self) {
        let _ = tokio::runtime::Handle::current().block_on(async {
            self.disconnect().await;
        });
    }
}
