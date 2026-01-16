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
use tokio::sync::Mutex;
use std::time::Duration;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct DMSCGrpcClient {
    channel: Option<tonic::transport::Channel>,
    endpoint: String,
    timeout: Duration,
    stats: Arc<RwLock<DMSCGrpcStats>>,
    request_id: Arc<AtomicU64>,
    connected: Arc<RwLock<bool>>,
}

impl DMSCGrpcClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            channel: None,
            endpoint,
            timeout: Duration::from_secs(30),
            stats: Arc::new(RwLock::new(DMSCGrpcStats::new())),
            request_id: Arc::new(AtomicU64::new(0)),
            connected: Arc::new(RwLock::new(false)),
        }
    }

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

    pub async fn call(&self, method: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        if !*self.connected.read().await {
            return Err(GrpcError::ConnectionFailed {
                message: "Not connected".to_string()
            }.into());
        }

        let channel = match &self.channel {
            Some(ch) => ch,
            None => return Err(GrpcError::ConnectionFailed {
                message: "Channel not initialized".to_string()
            }.into()),
        };

        let request_id = self.request_id.fetch_add(1, Ordering::SeqCst);

        {
            let mut stats = self.stats.write();
            stats.record_request(data.len());
        }

        let mut client = tonic::codec::ProstClient::new(channel.clone());

        let request = tonic::Request::new(GrpcRequest {
            method: method.to_string(),
            data: data.to_vec(),
            request_id,
        });

        let response = timeout(self.timeout, client.call(request))
            .await
            .map_err(|_| GrpcError::Timeout)?
            .map_err(|e| GrpcError::Client {
                message: format!("Request failed: {}", e)
            })?;

        let response_data = response.into_inner().response_data;

        {
            let mut stats = self.stats.write();
            stats.record_response(response_data.len());
        }

        Ok(response_data)
    }

    pub async fn call_stream(
        &self,
        method: &str,
        data: &[u8],
    ) -> DMSCResult<impl Stream<Item = DMSCResult<Vec<u8>>>> {
        if !*self.connected.read().await {
            return Err(GrpcError::ConnectionFailed {
                message: "Not connected".to_string()
            }.into());
        }

        let channel = match &self.channel {
            Some(ch) => ch.clone(),
            None => return Err(GrpcError::ConnectionFailed {
                message: "Channel not initialized".to_string()
            }.into()),
        };

        let mut client = tonic::codec::ProstClient::new(channel);

        let request = tonic::Request::new(GrpcRequest {
            method: method.to_string(),
            data: data.to_vec(),
            request_id: self.request_id.fetch_add(1, Ordering::SeqCst),
        });

        let response = client.call_stream(request).await
            .map_err(|e| GrpcError::Client {
                message: format!("Stream request failed: {}", e)
            })?;

        let inner = Arc::new(Mutex::new(response));

        let stream = async_stream::stream! {
            let mut inner = inner.lock().await;
            while let Some(result) = inner.message().await {
                match result {
                    Ok(response) => {
                        yield Ok(response.response_data);
                    }
                    Err(e) => {
                        yield Err(GrpcError::Client {
                            message: format!("Stream error: {}", e)
                        }.into());
                        break;
                    }
                }
            }
        };

        Ok(stream)
    }

    pub fn get_stats(&self) -> DMSCGrpcStats {
        self.stats.try_read().unwrap().clone()
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

#[derive(tonic::prost::Message)]
pub struct GrpcRequest {
    #[prost(string, tag = "1")]
    pub method: String,
    #[prost(bytes, tag = "2")]
    pub data: Vec<u8>,
    #[prost(uint64, tag = "3")]
    pub request_id: u64,
}

#[derive(tonic::prost::Message)]
pub struct GrpcResponse {
    #[prost(bytes, tag = "1")]
    pub response_data: Vec<u8>,
    #[prost(uint64, tag = "2")]
    pub request_id: u64,
    #[prost(bool, tag = "3")]
    pub success: bool,
    #[prost(string, tag = "4")]
    pub error_message: String,
}

tonic::include_proto!("dmsc.grpc.v1");
