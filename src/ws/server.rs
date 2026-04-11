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

//! # WebSocket Server Implementation

use super::*;
use uuid::Uuid;
use tokio::sync::mpsc;
use tokio::time::Duration;
use futures::StreamExt;
use tungstenite::Message;

#[cfg(feature = "pyo3")]
#[allow(unused_imports)]
use pyo3::prelude::*;

pub struct RiWSServer {
    config: RiWSServerConfig,
    stats: Arc<RwLock<RiWSServerStats>>,
    session_manager: Arc<RiWSSessionManager>,
    event_tx: Arc<RwLock<Option<broadcast::Sender<RiWSEvent>>>>,
    shutdown_tx: Option<mpsc::Sender<()>>,
    running: Arc<RwLock<bool>>,
    handler: Arc<RwLock<Option<Arc<dyn RiWSSessionHandler>>>>,
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct RiWSServerPy {
    inner: RiWSServer,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSServerPy {
    #[new]
    fn new(config: RiWSServerConfig) -> Self {
        Self {
            inner: RiWSServer::new(config),
        }
    }

    fn get_stats(&self) -> RiWSServerStats {
        self.inner.get_stats()
    }

    fn is_running(&self) -> bool {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.inner.is_running().await }))
            .unwrap_or(false)
    }

    fn start(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.start().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn stop(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.inner.stop().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn broadcast(&self, data: Vec<u8>) -> usize {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.inner.broadcast(&data).await.unwrap_or(0) }))
            .unwrap_or(0)
    }

    fn get_active_session_count(&self) -> usize {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.inner.get_active_session_count().await }))
            .unwrap_or(0)
    }
}

impl RiWSServer {
    pub fn new(config: RiWSServerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(RiWSServerStats::new())),
            session_manager: Arc::new(RiWSSessionManager::new(1000)),
            event_tx: Arc::new(RwLock::new(None)),
            shutdown_tx: None,
            running: Arc::new(RwLock::new(false)),
            handler: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn set_handler<H: RiWSSessionHandler + 'static>(&self, handler: H) {
        *self.handler.write().await = Some(Arc::new(handler));
    }

    pub async fn start(&mut self) -> RiResult<()> {
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
        *self.event_tx.write().await = Some(event_tx);

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
        session_manager: Arc<RiWSSessionManager>,
        stats: Arc<RwLock<RiWSServerStats>>,
        handler: Arc<RwLock<Option<Arc<dyn RiWSSessionHandler>>>>,
        config: RiWSServerConfig,
        mut shutdown_rx: mpsc::Receiver<()>,
        running: Arc<RwLock<bool>>,
    ) {
        let mut shutdown = false;

        while !shutdown {
            let result = listener.accept().await;
            
            if shutdown {
                break;
            }

            match result {
                Ok((stream, remote_addr)) => {
                    let session_id = Uuid::new_v4().to_string();
                    let remote_addr_str = remote_addr.to_string();
                    
                    tracing::info!("New WebSocket connection: {} (session: {})", remote_addr_str, session_id);

                    match tokio_tungstenite::accept_async(stream).await {
                        Ok(ws_stream) => {
                            let (_sender, receiver) = ws_stream.split();
                            let (tx, rx) = mpsc::channel(100);

                            let session = Arc::new(RiWSSession::new(
                                session_id.clone(),
                                tx,
                                receiver,
                                remote_addr_str.clone(),
                            ));

                            if session_manager.add_session(session.clone()).await.is_ok() {
                                stats.write().await.record_connection();

                                let handler_clone = handler.clone();
                                let session_manager_clone = session_manager.clone();
                                let stats_clone = stats.clone();

                                tokio::spawn(async move {
                                    Self::handle_session(
                                        session.clone(),
                                        rx,
                                        handler_clone,
                                        session_manager_clone,
                                        stats_clone,
                                    ).await;
                                });
                            } else {
                                tracing::trace!("Failed to add session: {}", session_id);
                            }
                        }
                        Err(e) => {
                            tracing::error!("WebSocket upgrade failed: {}", e);
                            stats.write().await.record_connection_error();
                        }
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to accept connection: {}", e);
                    stats.write().await.record_connection_error();
                }
            }

            tokio::time::sleep(Duration::from_secs(config.heartbeat_interval)).await;
            
            if !*running.read().await {
                break;
            }
            
            let _timeout = Duration::from_secs(config.heartbeat_timeout);
            let sessions = session_manager.get_all_sessions().await;
            for session_info in sessions {
                let last_heartbeat_time = chrono::DateTime::from_timestamp(session_info.last_heartbeat as i64, 0)
                    .unwrap_or_else(|| chrono::Utc::now());
                let elapsed = last_heartbeat_time.signed_duration_since(chrono::Utc::now());
                let elapsed_secs = elapsed.num_seconds() as u64;
                
                if elapsed_secs > config.heartbeat_timeout {
                    if let Some(session) = session_manager.get_session(&session_info.session_id).await {
                        let _ = session.close().await;
                    }
                }
            }
            
            if shutdown_rx.try_recv().is_ok() {
                shutdown = true;
            }
        }

        tracing::info!("WebSocket server stopped");
    }

    async fn handle_session(
        session: Arc<RiWSSession>,
        mut rx: mpsc::Receiver<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
        handler: Arc<RwLock<Option<Arc<dyn RiWSSessionHandler>>>>,
        session_manager: Arc<RiWSSessionManager>,
        stats: Arc<RwLock<RiWSServerStats>>,
    ) {
        let session_id = session.id.clone();

        while let Some(message_result) = rx.recv().await {
            match message_result {
                Ok(message) => {
                    match message {
                        Message::Binary(data) => {
                            stats.write().await.record_message_received(data.len());

                            let handler_read = handler.read().await;
                            if let Some(handler) = &*handler_read {
                                if let Ok(response) = handler.on_message(&session_id, &data).await {
                                    if session.send(&response).await.is_err() {
                                        break;
                                    }
                                    stats.write().await.record_message_sent(response.len());
                                }
                            } else {
                                if session.send(&data).await.is_err() {
                                    break;
                                }
                                stats.write().await.record_message_sent(data.len());
                            }
                        }
                        Message::Text(text) => {
                            let data = text.into_bytes();
                            stats.write().await.record_message_received(data.len());
                            
                            let handler_read = handler.read().await;
                            if let Some(handler) = &*handler_read {
                                if let Ok(response) = handler.on_message(&session_id, &data).await {
                                    if session.send(&response).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                        Message::Ping(ping_data) => {
                            if session.send(&ping_data).await.is_err() {
                                break;
                            }
                        }
                        Message::Pong(_) => {}
                        Message::Close(_) => {
                            break;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket error for session {}: {}", session_id, e);
                    stats.write().await.record_message_error();
                    break;
                }
            }
        }

        session_manager.remove_session(&session_id).await;
        stats.write().await.record_disconnection();

        let handler_read = handler.read().await;
        if let Some(handler) = &*handler_read {
            let _ = handler.on_disconnect(&session_id).await;
        }
    }

    pub async fn stop(&mut self) -> RiResult<()> {
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

    pub fn get_stats(&self) -> RiWSServerStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiWSServerStats::new())
    }

    pub async fn get_session_info(&self, session_id: &str) -> Option<RiWSSessionInfo> {
        self.session_manager.get_session(session_id).await
            .map(|s| s.get_info())
    }

    pub async fn broadcast(&self, data: &[u8]) -> RiResult<usize> {
        let count = self.session_manager.broadcast(data).await?;
        self.stats.write().await.record_message_sent(data.len() * count);
        Ok(count)
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    pub async fn get_active_session_count(&self) -> usize {
        self.session_manager.get_session_count().await
    }
}

impl Clone for RiWSServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            stats: self.stats.clone(),
            session_manager: self.session_manager.clone(),
            event_tx: self.event_tx.clone(),
            shutdown_tx: None,
            running: self.running.clone(),
            handler: self.handler.clone(),
        }
    }
}

impl Default for RiWSServer {
    fn default() -> Self {
        Self::new(RiWSServerConfig::default())
    }
}
