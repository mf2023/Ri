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

//! # WebSocket Client Implementation
//!
//! This module provides the WebSocket client implementation for DMSC.

use super::*;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCWSClientConfig {
    pub heartbeat_interval: u64,
    pub heartbeat_timeout: u64,
    pub max_message_size: usize,
    pub connect_timeout: u64,
    pub auto_reconnect: bool,
    pub reconnect_interval: u64,
}

impl Default for DMSCWSClientConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: 30,
            heartbeat_timeout: 60,
            max_message_size: 65536,
            connect_timeout: 10,
            auto_reconnect: false,
            reconnect_interval: 5,
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCWSClientStats {
    pub total_connections: u64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub connection_errors: u64,
    pub message_errors: u64,
    pub last_connected_at: Option<u64>,
}

impl DMSCWSClientStats {
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            total_messages_sent: 0,
            total_messages_received: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            connection_errors: 0,
            message_errors: 0,
            last_connected_at: None,
        }
    }

    fn record_connection(&mut self) {
        self.total_connections += 1;
        self.last_connected_at = Some(chrono::Utc::now().timestamp() as u64);
    }

    fn record_message_sent(&mut self, size: usize) {
        self.total_messages_sent += 1;
        self.total_bytes_sent += size as u64;
    }

    fn record_message_received(&mut self, size: usize) {
        self.total_messages_received += 1;
        self.total_bytes_received += size as u64;
    }

    fn record_connection_error(&mut self) {
        self.connection_errors += 1;
    }

    fn record_message_error(&mut self) {
        self.message_errors += 1;
    }
}

impl Default for DMSCWSClientStats {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCWSClient {
    config: DMSCWSClientConfig,
    stats: Arc<RwLock<DMSCWSClientStats>>,
    connected: Arc<RwLock<bool>>,
    server_url: String,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCWSClient {
    #[new]
    fn new(server_url: String) -> Self {
        Self::with_config(server_url, DMSCWSClientConfig::default())
    }

    #[staticmethod]
    fn with_config_py(server_url: String, config: DMSCWSClientConfig) -> Self {
        Self::with_config(server_url, config)
    }

    fn get_stats_py(&self) -> DMSCWSClientStats {
        self.get_stats()
    }

    fn is_connected_py(&self) -> bool {
        let connected = self.connected.clone();
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            *connected.read().await
        })
    }
}

impl DMSCWSClient {
    pub fn with_config(server_url: String, config: DMSCWSClientConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DMSCWSClientStats::new())),
            connected: Arc::new(RwLock::new(false)),
            server_url,
        }
    }

    pub fn get_stats(&self) -> DMSCWSClientStats {
        self.stats.try_read().unwrap().clone()
    }

    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    pub async fn connect(&mut self) -> DMSCResult<()> {
        if *self.connected.read().await {
            return Ok(());
        }

        let url = self.server_url.parse::<http::Uri>().map_err(|e| WSError::Connection {
            message: format!("Invalid WebSocket URL: {}", e)
        })?;

        let ws_config = WebSocketConfig {
            max_message_size: Some(self.config.max_message_size),
            max_frame_size: Some(self.config.max_message_size),
            max_write_buffer_size: self.config.max_message_size,
            ..Default::default()
        };

        let (_ws_stream, _response) = tokio_tungstenite::connect_async_with_config(
            &url,
            Some(ws_config),
            true,
        )
        .await
        .map_err(|e| WSError::Connection {
            message: format!("Failed to connect to WebSocket server: {}", e)
        })?;

        *self.connected.write().await = true;
        self.stats.write().await.record_connection();

        tracing::info!("WebSocket client connected to {}", self.server_url);
        Ok(())
    }

    pub async fn send(&self, _data: &[u8]) -> DMSCResult<()> {
        if !*self.connected.read().await {
            return Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into());
        }
        Ok(())
    }

    pub async fn send_text(&self, _text: &str) -> DMSCResult<()> {
        if !*self.connected.read().await {
            return Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into());
        }
        Ok(())
    }

    pub async fn close(&mut self) -> DMSCResult<()> {
        *self.connected.write().await = false;
        tracing::info!("WebSocket client disconnected from {}", self.server_url);
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        let _ = self.close().await;
    }
}

impl Drop for DMSCWSClient {
    fn drop(&mut self) {
        let runtime = tokio::runtime::Handle::current();
        let _ = runtime.block_on(async {
            self.close().await.ok();
        });
    }
}
