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

//! # WebSocket Client Implementation

use super::*;
use tokio_tungstenite::tungstenite::protocol::WebSocketConfig;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiWSClientConfig {
    pub heartbeat_interval: u64,
    pub heartbeat_timeout: u64,
    pub max_message_size: usize,
    pub connect_timeout: u64,
    pub auto_reconnect: bool,
    pub reconnect_interval: u64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSClientConfig {
    #[new]
    fn new() -> Self {
        Self::default()
    }
    
    #[getter]
    fn get_heartbeat_interval(&self) -> u64 {
        self.heartbeat_interval
    }
    
    #[setter]
    fn set_heartbeat_interval(&mut self, val: u64) {
        self.heartbeat_interval = val;
    }
    
    #[getter]
    fn get_heartbeat_timeout(&self) -> u64 {
        self.heartbeat_timeout
    }
    
    #[setter]
    fn set_heartbeat_timeout(&mut self, val: u64) {
        self.heartbeat_timeout = val;
    }
    
    #[getter]
    fn get_max_message_size(&self) -> usize {
        self.max_message_size
    }
    
    #[setter]
    fn set_max_message_size(&mut self, val: usize) {
        self.max_message_size = val;
    }
    
    #[getter]
    fn get_connect_timeout(&self) -> u64 {
        self.connect_timeout
    }
    
    #[setter]
    fn set_connect_timeout(&mut self, val: u64) {
        self.connect_timeout = val;
    }
    
    #[getter]
    fn get_auto_reconnect(&self) -> bool {
        self.auto_reconnect
    }
    
    #[setter]
    fn set_auto_reconnect(&mut self, val: bool) {
        self.auto_reconnect = val;
    }
    
    #[getter]
    fn get_reconnect_interval(&self) -> u64 {
        self.reconnect_interval
    }
    
    #[setter]
    fn set_reconnect_interval(&mut self, val: u64) {
        self.reconnect_interval = val;
    }
}

impl Default for RiWSClientConfig {
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
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiWSClientStats {
    pub total_connections: u64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub connection_errors: u64,
    pub message_errors: u64,
    pub last_connected_at: Option<u64>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSClientStats {
    #[getter]
    fn get_total_connections(&self) -> u64 {
        self.total_connections
    }
    
    #[getter]
    fn get_total_messages_sent(&self) -> u64 {
        self.total_messages_sent
    }
    
    #[getter]
    fn get_total_messages_received(&self) -> u64 {
        self.total_messages_received
    }
    
    #[getter]
    fn get_total_bytes_sent(&self) -> u64 {
        self.total_bytes_sent
    }
    
    #[getter]
    fn get_total_bytes_received(&self) -> u64 {
        self.total_bytes_received
    }
    
    #[getter]
    fn get_connection_errors(&self) -> u64 {
        self.connection_errors
    }
    
    #[getter]
    fn get_message_errors(&self) -> u64 {
        self.message_errors
    }
    
    #[getter]
    fn get_last_connected_at(&self) -> Option<u64> {
        self.last_connected_at
    }
}

impl RiWSClientStats {
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

    #[allow(dead_code)]
    fn record_message_sent(&mut self, size: usize) {
        self.total_messages_sent += 1;
        self.total_bytes_sent += size as u64;
    }

    #[allow(dead_code)]
    fn record_message_received(&mut self, size: usize) {
        self.total_messages_received += 1;
        self.total_bytes_received += size as u64;
    }

    #[allow(dead_code)]
    fn record_connection_error(&mut self) {
        self.connection_errors += 1;
    }

    #[allow(dead_code)]
    fn record_message_error(&mut self) {
        self.message_errors += 1;
    }
}

impl Default for RiWSClientStats {
    fn default() -> Self {
        Self::new()
    }
}

pub struct RiWSClient {
    config: RiWSClientConfig,
    stats: Arc<RwLock<RiWSClientStats>>,
    connected: Arc<RwLock<bool>>,
    server_url: String,
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct RiWSClientPy {
    inner: RiWSClient,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSClientPy {
    #[new]
    fn new(server_url: String) -> Self {
        Self {
            inner: RiWSClient::new(server_url),
        }
    }

    #[staticmethod]
    fn with_config(server_url: String, config: RiWSClientConfig) -> Self {
        Self {
            inner: RiWSClient::with_config(server_url, config),
        }
    }

    fn get_stats(&self) -> RiWSClientStats {
        self.inner.get_stats()
    }

    fn is_connected(&self) -> bool {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.inner.is_connected().await }))
            .unwrap_or(false)
    }

    fn connect(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.connect().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn send(&self, data: Vec<u8>) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.send(&data).await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn send_text(&self, text: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.send_text(&text).await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn close(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.close().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

impl RiWSClient {
    pub fn new(server_url: String) -> Self {
        Self::with_config(server_url, RiWSClientConfig::default())
    }

    pub fn with_config(server_url: String, config: RiWSClientConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(RiWSClientStats::new())),
            connected: Arc::new(RwLock::new(false)),
            server_url,
        }
    }

    pub fn get_stats(&self) -> RiWSClientStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiWSClientStats::new())
    }

    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    pub async fn connect(&mut self) -> RiResult<()> {
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

    pub async fn send(&self, _data: &[u8]) -> RiResult<()> {
        if !*self.connected.read().await {
            return Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into());
        }
        Ok(())
    }

    pub async fn send_text(&self, _text: &str) -> RiResult<()> {
        if !*self.connected.read().await {
            return Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into());
        }
        Ok(())
    }

    pub async fn close(&mut self) -> RiResult<()> {
        *self.connected.write().await = false;
        tracing::info!("WebSocket client disconnected from {}", self.server_url);
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        let _ = self.close().await;
    }
}

impl Clone for RiWSClient {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            stats: self.stats.clone(),
            connected: self.connected.clone(),
            server_url: self.server_url.clone(),
        }
    }
}

impl Default for RiWSClient {
    fn default() -> Self {
        Self::new("ws://127.0.0.1:8080".to_string())
    }
}
