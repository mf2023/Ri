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

//! # WebSocket Server Implementation
//!
//! This module provides the WebSocket server implementation for DMSC.

use super::*;
use uuid::Uuid;
use tokio::sync::mpsc;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use tokio::time::{interval, Duration};
use futures::stream::FuturesUnordered;

pub struct DMSCWSServer {
    config: DMSCWSServerConfig,
    stats: Arc<RwLock<DMSCWSServerStats>>,
    session_manager: Arc<DMSCWSSessionManager>,
    event_tx: Arc<RwLock<Option<broadcast::Sender<DMSCWSEvent>>>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
    running: Arc<RwLock<bool>>,
    handler: Arc<RwLock<Option<Arc<dyn DMSCWSSessionHandler>>>>,
}

impl DMSCWSServer {
    pub fn new(config: DMSCWSServerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DMSCWSServerStats::new())),
            session_manager: Arc::new(DMSCWSSessionManager::new(1000)),
            event_tx: Arc::new(RwLock::new(None)),
            shutdown_tx: None,
            running: Arc::new(RwLock::new(false)),
            handler: Arc::new(RwLock::new(None)),
        }
    }

    pub fn set_handler<H: DMSCWSSessionHandler + 'static>(&self, handler: H) {
        *self.handler.write() = Some(Arc::new(handler));
    }

    pub async fn start(&mut self) -> DMSCResult<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.addr, self.config.port)
            .parse()
            .map_err(|e| WSError::Server {
                message: format!("Invalid address: {}", e)
            })?;

        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| WSError::Server {
                message: format!("Failed to bind: {}", e)
            })?;

        let (event_tx, _) = broadcast::channel(100);
        *self.event_tx.write() = Some(Arc::new(event_tx));

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        let running = self.running.clone();
        let stats = self.stats.clone();
        let session_manager = self.session_manager.clone();
        let handler = self.handler.clone();
        let config = self.config.clone();

        *running.write().await = true;

        tokio::spawn(async move {
            Self::accept_connections(
                listener,
                session_manager,
                stats,
                handler,
                config,
                shutdown_rx,
                running,
            ).await;
        });

        tracing::info!("WebSocket server started on {}", addr);
        Ok(())
    }

    async fn accept_connections(
        listener: TcpListener,
        session_manager: Arc<DMSCWSSessionManager>,
        stats: Arc<RwLock<DMSCWSServerStats>>,
        handler: Arc<RwLock<Option<Arc<dyn DMSCWSSessionHandler>>>>,
        config: DMSCWSServerConfig,
        mut shutdown_rx: mpsc::Receiver<()>,
        running: Arc<RwLock<bool>>,
    ) {
        let mut interval = interval(Duration::from_secs(config.heartbeat_interval));
        let mut active_sessions = FuturesUnordered::new();

        loop {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, remote_addr)) => {
                            let session_id = Uuid::new_v4().to_string();
                            let remote_addr_str = remote_addr.to_string();
                            
                            tracing::info!("New WebSocket connection: {} (session: {})", remote_addr_str, session_id);

                            match tokio_tungstenite::accept_async(stream).await {
                                Ok(ws_stream) => {
                                    let (sender, receiver) = ws_stream.split();
                                    let (tx, mut rx) = mpsc::channel(100);

                                    let session = Arc::new(DMSCWSSession::new(
                                        session_id.clone(),
                                        tx,
                                        receiver,
                                        remote_addr_str.clone(),
                                    ));

                                    if session_manager.add_session(session.clone()).await.is_ok() {
                                        stats.write().record_connection();

                                        let handler_clone = handler.clone();
                                        let session_manager_clone = session_manager.clone();
                                        let stats_clone = stats.clone();

                                        active_sessions.push(async move {
                                            Self::handle_session(
                                                session.clone(),
                                                rx,
                                                handler_clone,
                                                session_manager_clone,
                                                stats_clone,
                                            ).await;
                                        });
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("WebSocket upgrade failed: {}", e);
                                    stats.write().record_connection_error();
                                }
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to accept connection: {}", e);
                            stats.write().record_connection_error();
                        }
                    }
                }
                _ = interval.tick() => {
                    if !*running.read().await {
                        break;
                    }
                }
                _ = shutdown_rx.recv() => {
                    break;
                }
                _ = active_sessions.next() => {}
            }
        }

        tracing::info!("WebSocket server stopped");
    }

    async fn handle_session(
        session: Arc<DMSCWSSession>,
        mut rx: mpsc::Receiver<std::result::Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>>,
        handler: Arc<RwLock<Option<Arc<dyn DMSCWSSessionHandler>>>>,
        session_manager: Arc<DMSCWSSessionManager>,
        stats: Arc<RwLock<DMSCWSServerStats>>,
    ) {
        let session_id = session.id.clone();

        while let Some(message_result) = rx.recv().await {
            match message_result {
                Ok(message) => {
                    match message {
                        tokio_tungstenite::tungstenite::Message::Binary(data) => {
                            stats.write().record_message_received(data.len());

                            let handler_read = handler.read();
                            if let Some(handler) = &*handler_read {
                                if let Ok(response) = handler.on_message(&session_id, &data).await {
                                    if session.send(&response).await.is_err() {
                                        break;
                                    }
                                    stats.write().record_message_sent(response.len());
                                }
                            } else {
                                if session.send(&data).await.is_err() {
                                    break;
                                }
                                stats.write().record_message_sent(data.len());
                            }
                        }
                        tokio_tungstenite::tungstenite::Message::Text(text) => {
                            if let Ok(data) = text.into_bytes() {
                                stats.write().record_message_received(data.len());
                                
                                let handler_read = handler.read();
                                if let Some(handler) = &*handler_read {
                                    if let Ok(response) = handler.on_message(&session_id, &data).await {
                                        if session.send(&response).await.is_err() {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        tokio_tungstenite::tungstenite::Message::Ping(ping_data) => {
                            if session.send_pong(ping_data).await.is_err() {
                                break;
                            }
                        }
                        tokio_tungstenite::tungstenite::Message::Pong(_) => {}
                        tokio_tungstenite::tungstenite::Message::Close(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket error for session {}: {}", session_id, e);
                    stats.write().record_message_error();
                    break;
                }
            }
        }

        session_manager.remove_session(&session_id).await;
        stats.write().record_disconnection();

        let handler_read = handler.read();
        if let Some(handler) = &*handler_read {
            let _ = handler.on_disconnect(&session_id).await;
        }
    }

    pub async fn stop(&mut self) -> DMSCResult<()> {
        *self.running.write().await = false;

        let sessions = self.session_manager.get_all_sessions().await;
        for session_info in sessions {
            if let Some(session) = self.session_manager.get_session(&session_info.session_id).await {
                let _ = session.close().await;
            }
        }

        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(()).await.map_err(|e| WSError::Server {
                message: format!("Shutdown error: {}", e)
            })?;
        }

        tracing::info!("WebSocket server stopped");
        Ok(())
    }

    pub fn get_stats(&self) -> DMSCWSServerStats {
        self.stats.try_read().unwrap().clone()
    }

    pub async fn get_session_info(&self, session_id: &str) -> Option<DMSCWSSessionInfo> {
        self.session_manager.get_session(session_id).await
            .map(|s| s.get_info())
    }

    pub async fn broadcast(&self, data: &[u8]) -> DMSCResult<usize> {
        let count = self.session_manager.broadcast(data).await?;
        self.stats.write().record_message_sent(data.len() * count);
        Ok(count)
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    pub async fn get_active_session_count(&self) -> usize {
        self.session_manager.get_session_count().await
    }
}
