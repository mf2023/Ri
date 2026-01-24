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
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tokio::time::{Duration, Interval};
use tungstenite::protocol::WebSocketConfig;
use uuid::Uuid;

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

pub struct DMSCWSClient {
    config: DMSCWSClientConfig,
    stats: Arc<RwLock<DMSCWSClientStats>>,
    connected: Arc<RwLock<bool>>,
    server_url: String,
    ws_stream: Arc<RwLock<Option<tokio_tungstenite::WebSocketStream<TcpStream>>>>,
    message_tx: Arc<RwLock<Option<mpsc::Sender<Result<Message, tungstenite::Error>>>>>,
    message_rx: Arc<RwLock<Option<mpsc::Receiver<Result<Message, tungstenite::Error>>>>>,
    heartbeat_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>>,
    shutdown_tx: Arc<RwLock<Option<mpsc::Sender<()>>>>,
    running: Arc<RwLock<bool>>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCWSClient {
    #[new]
    fn new(server_url: String) -> Self {
        Self::with_config(server_url, DMSCWSClientConfig::default())
    }
}

impl DMSCWSClient {
    pub fn with_config(server_url: String, config: DMSCWSClientConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DMSCWSClientStats::new())),
            connected: Arc::new(RwLock::new(false)),
            server_url,
            ws_stream: Arc::new(RwLock::new(None)),
            message_tx: Arc::new(RwLock::new(None)),
            message_rx: Arc::new(RwLock::new(None)),
            heartbeat_tx: Arc::new(RwLock::new(None)),
            shutdown_tx: Arc::new(RwLock::new(None)),
            running: Arc::new(RwLock::new(false)),
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

        let (ws_stream, _) = tokio_tungstenite::connect_async_tls_with_config(
            &url,
            Some(WebSocketConfig {
                max_message_size: Some(self.config.max_message_size),
                ..Default::default()
            }),
            true,
        )
        .await
        .map_err(|e| WSError::Connection {
            message: format!("Failed to connect to WebSocket server: {}", e)
        })?;

        let (write, read) = ws_stream.split();
        let (tx, rx) = mpsc::channel(100);
        let (heartbeat_tx, heartbeat_rx) = mpsc::channel(1);
        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);

        let mut write = write;
        let mut read = read;
        let heartbeat_interval = self.config.heartbeat_interval;

        {
            let mut ws_stream_guard = self.ws_stream.write().await;
            *ws_stream_guard = Some(tokio_tungstenite::WebSocketStream::from_raw_socket(
                write.reunite(read).unwrap(),
                tungstenite::protocol::Role::Client,
                Some(WebSocketConfig {
                    max_message_size: Some(self.config.max_message_size),
                    ..Default::default()
                }),
            ).await);
        }

        {
            let mut message_tx_guard = self.message_tx.write().await;
            *message_tx_guard = Some(tx);
        }

        {
            let mut message_rx_guard = self.message_rx.write().await;
            *message_rx_guard = Some(rx);
        }

        {
            let mut heartbeat_tx_guard = self.heartbeat_tx.write().await;
            *heartbeat_tx_guard = Some(heartbeat_tx);
        }

        {
            let mut shutdown_tx_guard = self.shutdown_tx.write().await;
            *shutdown_tx_guard = Some(shutdown_tx);
        }

        *self.connected.write().await = true;
        *self.running.write().await = true;

        self.stats.write().await.record_connection();

        let stats_clone = self.stats.clone();
        let running_clone = self.running.clone();
        let ws_stream_clone = self.ws_stream.clone();
        let message_rx_clone = self.message_rx.clone();
        let heartbeat_rx_clone = heartbeat_rx;
        let shutdown_rx_clone = shutdown_rx;

        tokio::spawn(async move {
            Self::handle_messages(
                stats_clone,
                running_clone,
                ws_stream_clone,
                message_rx_clone,
                heartbeat_rx_clone,
                shutdown_rx_clone,
            ).await;
        });

        tracing::info!("WebSocket client connected to {}", self.server_url);
        Ok(())
    }

    async fn handle_messages(
        stats: Arc<RwLock<DMSCWSClientStats>>,
        running: Arc<RwLock<bool>>,
        ws_stream: Arc<RwLock<Option<tokio_tungstenite::WebSocketStream<TcpStream>>>>,
        message_rx: Arc<RwLock<Option<mpsc::Receiver<Result<Message, tungstenite::Error>>>>>,
        mut heartbeat_rx: mpsc::Receiver<()>,
        mut shutdown_rx: mpsc::Receiver<()>,
    ) {
        let mut heartbeat_timer = tokio::time::interval(Duration::from_secs(30));

        loop {
            tokio::select! {
                _ = heartbeat_timer.tick() => {
                    if let Some(tx) = Self::get_heartbeat_tx().await {
                        let _ = tx.send(()).await;
                    }
                }
                _ = heartbeat_rx.recv() => {
                    if let Some(stream) = ws_stream.write().await.take() {
                        let (mut write, read) = stream.split();
                        let ping_result = write.send(Message::Ping(vec![])).await;
                        let _ = write.reunite(read);
                        if ping_result.is_err() {
                            *running.write().await = false;
                            break;
                        }
                    }
                }
                result = async {
                    let rx_opt = message_rx.read().await;
                    if let Some(rx) = &*rx_opt {
                        rx.recv().await
                    } else {
                        None
                    }
                } => {
                    match result {
                        Some(Ok(message)) => {
                            let stream_opt = ws_stream.write().await;
                            if let Some(stream) = &*stream_opt {
                                let mut write = stream.clone();
                                if let Err(e) = write.send(message).await {
                                    stats.write().await.record_message_error();
                                    tracing::error!("Failed to send message: {}", e);
                                } else {
                                    stats.write().await.record_message_sent(0);
                                }
                            }
                        }
                        Some(Err(e)) => {
                            stats.write().await.record_message_error();
                            tracing::error!("Message error: {}", e);
                        }
                        None => {
                            break;
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("WebSocket client shutting down");
                    break;
                }
            }

            if !*running.read().await {
                break;
            }
        }

        *running.write().await = false;
    }

    async fn get_heartbeat_tx() -> Option<mpsc::Sender<()>> {
        None
    }

    pub async fn send(&self, data: &[u8]) -> DMSCResult<()> {
        let tx_opt = self.message_tx.read().await;
        if let Some(tx) = &*tx_opt {
            let message = Message::Binary(data.to_vec());
            tx.send(Ok(message)).await
                .map_err(|e| WSError::Session {
                    message: format!("Failed to send message: {}", e)
                })?;
            self.stats.write().await.record_message_sent(data.len());
            Ok(())
        } else {
            Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into())
        }
    }

    pub async fn send_text(&self, text: &str) -> DMSCResult<()> {
        let tx_opt = self.message_tx.read().await;
        if let Some(tx) = &*tx_opt {
            let message = Message::Text(text.to_string());
            tx.send(Ok(message)).await
                .map_err(|e| WSError::Session {
                    message: format!("Failed to send message: {}", e)
                })?;
            self.stats.write().await.record_message_sent(text.len());
            Ok(())
        } else {
            Err(WSError::Connection {
                message: "Not connected to WebSocket server".to_string()
            }.into())
        }
    }

    pub async fn close(&mut self) -> DMSCResult<()> {
        if let Some(tx) = self.shutdown_tx.write().await.take() {
            tx.send(()).await.map_err(|e| WSError::Server {
                message: format!("Shutdown error: {}", e)
            })?;
        }

        {
            let ws_stream_guard = self.ws_stream.write().await;
            if let Some(stream) = ws_stream_guard.take() {
                let (mut write, _) = stream.split();
                let _ = write.send(Message::Close(None)).await;
            }
        }

        *self.connected.write().await = false;
        *self.running.write().await = false;

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
