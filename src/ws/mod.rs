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

//! # WebSocket Support

use crate::core::{RiResult, RiError};
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
use futures::stream::SplitStream;
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use std::collections::HashMap;
use tungstenite::Message;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "websocket")]
mod server;

#[cfg(feature = "websocket")]
mod client;

#[cfg(feature = "websocket")]
pub use server::RiWSServer;

#[cfg(feature = "websocket")]
pub use client::RiWSClient;

#[cfg(feature = "websocket")]
pub use client::RiWSClientConfig;

#[cfg(feature = "websocket")]
pub use client::RiWSClientStats;

#[cfg(all(feature = "websocket", feature = "pyo3"))]
pub use server::RiWSServerPy;

#[cfg(all(feature = "websocket", feature = "pyo3"))]
pub use client::RiWSClientPy;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiWSServerConfig {
    pub addr: String,
    pub port: u16,
    pub max_connections: usize,
    pub heartbeat_interval: u64,
    pub heartbeat_timeout: u64,
    pub max_message_size: usize,
    pub ping_interval: u64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSServerConfig {
    #[new]
    fn new() -> Self {
        Self::default()
    }
    
    #[getter]
    fn get_addr(&self) -> String {
        self.addr.clone()
    }
    
    #[setter]
    fn set_addr(&mut self, addr: String) {
        self.addr = addr;
    }
    
    #[getter]
    fn get_port(&self) -> u16 {
        self.port
    }
    
    #[setter]
    fn set_port(&mut self, port: u16) {
        self.port = port;
    }
}

impl Default for RiWSServerConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            heartbeat_interval: 30,
            heartbeat_timeout: 60,
            max_message_size: 65536,
            ping_interval: 25,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub enum RiWSEvent {
    Connected { session_id: String },
    Disconnected { session_id: String },
    Message { session_id: String, data: Vec<u8> },
    Error { session_id: String, message: String },
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiWSSessionInfo {
    pub session_id: String,
    pub remote_addr: String,
    pub connected_at: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub is_active: bool,
    pub last_heartbeat: u64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSSessionInfo {
    #[getter]
    fn get_session_id(&self) -> String {
        self.session_id.clone()
    }
    
    #[getter]
    fn get_remote_addr(&self) -> String {
        self.remote_addr.clone()
    }
    
    #[getter]
    fn get_connected_at(&self) -> u64 {
        self.connected_at
    }
    
    #[getter]
    fn get_messages_sent(&self) -> u64 {
        self.messages_sent
    }
    
    #[getter]
    fn get_messages_received(&self) -> u64 {
        self.messages_received
    }
    
    #[getter]
    fn get_bytes_sent(&self) -> u64 {
        self.bytes_sent
    }
    
    #[getter]
    fn get_bytes_received(&self) -> u64 {
        self.bytes_received
    }
    
    #[getter]
    fn get_is_active(&self) -> bool {
        self.is_active
    }
    
    #[getter]
    fn get_last_heartbeat(&self) -> u64 {
        self.last_heartbeat
    }
}

impl Default for RiWSSessionInfo {
    fn default() -> Self {
        Self {
            session_id: String::new(),
            remote_addr: String::new(),
            connected_at: 0,
            messages_sent: 0,
            messages_received: 0,
            bytes_sent: 0,
            bytes_received: 0,
            is_active: false,
            last_heartbeat: 0,
        }
    }
}

#[async_trait]
pub trait RiWSSessionHandler: Send + Sync {
    async fn on_connect(&self, session_id: &str, remote_addr: &str) -> RiResult<()>;
    async fn on_disconnect(&self, session_id: &str) -> RiResult<()>;
    async fn on_message(&self, session_id: &str, data: &[u8]) -> RiResult<Vec<u8>>;
    async fn on_error(&self, session_id: &str, error: &str) -> RiResult<()>;
}

#[derive(Debug, thiserror::Error)]
pub enum WSError {
    #[error("Server error: {message}")]
    Server { message: String },
    #[error("Session error: {message}")]
    Session { message: String },
    #[error("Connection error: {message}")]
    Connection { message: String },
    #[error("Message too large: {size} bytes (max: {max_size})")]
    MessageTooLarge { size: usize, max_size: usize },
    #[error("Session not found: {session_id}")]
    SessionNotFound { session_id: String },
    #[error("Invalid message format")]
    InvalidFormat,
}

impl From<WSError> for RiError {
    fn from(error: WSError) -> Self {
        RiError::Other(format!("WebSocket error: {}", error))
    }
}

pub struct RiWSSession {
    pub id: String,
    pub sender: tokio::sync::mpsc::Sender<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
    pub receiver: SplitStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>,
    pub info: Arc<RwLock<RiWSSessionInfo>>,
}

impl RiWSSession {
    pub fn new(
        id: String,
        sender: tokio::sync::mpsc::Sender<std::result::Result<Message, tokio_tungstenite::tungstenite::Error>>,
        receiver: SplitStream<tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>>,
        remote_addr: String,
    ) -> Self {
            let now = chrono::Utc::now().timestamp() as u64;
        let session_id = id.clone();
        Self {
            id,
            sender,
            receiver,
            info: Arc::new(RwLock::new(RiWSSessionInfo {
                session_id,
                remote_addr,
                connected_at: now,
                messages_sent: 0,
                messages_received: 0,
                bytes_sent: 0,
                bytes_received: 0,
                is_active: true,
                last_heartbeat: now,
            })),
        }
    }

    pub async fn send(&self, data: &[u8]) -> RiResult<()> {
        let message = Message::Binary(data.to_vec());
        
        self.sender.send(Ok(message))
            .await
            .map_err(|e| WSError::Session {
                message: format!("Failed to send message: {}", e)
            })?;

        let mut info = self.info.write().await;
        info.messages_sent += 1;
        info.bytes_sent += data.len() as u64;

        Ok(())
    }

    pub async fn send_text(&self, text: &str) -> RiResult<()> {
        let message = Message::Text(text.to_string());
        
        self.sender.send(Ok(message))
            .await
            .map_err(|e| WSError::Session {
                message: format!("Failed to send message: {}", e)
            })?;

        let mut info = self.info.write().await;
        info.messages_sent += 1;
        info.bytes_sent += text.len() as u64;

        Ok(())
    }

    pub async fn close(&self) -> RiResult<()> {
        self.sender.send(Ok(Message::Close(None)))
            .await
            .map_err(|e| WSError::Session {
                message: format!("Failed to close session: {}", e)
            })?;

        let mut info = self.info.write().await;
        info.is_active = false;

        Ok(())
    }

    pub fn get_info(&self) -> RiWSSessionInfo {
        self.info.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| RiWSSessionInfo::default())
    }
}

pub struct RiWSSessionManager {
    sessions: Arc<RwLock<HashMap<String, Arc<RiWSSession>>>>,
    max_connections: usize,
}

impl Clone for RiWSSessionManager {
    fn clone(&self) -> Self {
        Self {
            sessions: self.sessions.clone(),
            max_connections: self.max_connections,
        }
    }
}

impl RiWSSessionManager {
    pub fn new(max_connections: usize) -> Self {
        Self {
            sessions: Arc::new(RwLock::new(HashMap::new())),
            max_connections,
        }
    }

    pub async fn add_session(&self, session: Arc<RiWSSession>) -> RiResult<()> {
        let mut sessions = self.sessions.write().await;
        
        if sessions.len() >= self.max_connections {
            return Err(WSError::Session {
                message: format!("Max connections reached: {}", self.max_connections)
            }.into());
        }

        sessions.insert(session.id.clone(), session);
        Ok(())
    }

    pub async fn remove_session(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        sessions.remove(session_id);
    }

    pub async fn get_session(&self, session_id: &str) -> Option<Arc<RiWSSession>> {
        let sessions = self.sessions.read().await;
        sessions.get(session_id).cloned()
    }

    pub async fn broadcast(&self, data: &[u8]) -> RiResult<usize> {
        let sessions = self.sessions.read().await;
        let mut count = 0;

        for session in sessions.values() {
            if session.send(data).await.is_ok() {
                count += 1;
            }
        }

        Ok(count)
    }

    pub async fn get_session_count(&self) -> usize {
        self.sessions.read().await.len()
    }

    pub async fn get_all_sessions(&self) -> Vec<RiWSSessionInfo> {
        let sessions = self.sessions.read().await;
        sessions.values().map(|s| s.get_info()).collect()
    }
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct RiWSPythonHandler {
    on_connect: Arc<Py<PyAny>>,
    on_disconnect: Arc<Py<PyAny>>,
    on_message: Arc<Py<PyAny>>,
    on_error: Arc<Py<PyAny>>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSPythonHandler {
    #[new]
    fn new(
        on_connect: Py<PyAny>,
        on_disconnect: Py<PyAny>,
        on_message: Py<PyAny>,
        on_error: Py<PyAny>,
    ) -> Self {
        Self {
            on_connect: Arc::new(on_connect),
            on_disconnect: Arc::new(on_disconnect),
            on_message: Arc::new(on_message),
            on_error: Arc::new(on_error),
        }
    }
}

#[cfg(feature = "pyo3")]
#[async_trait]
impl RiWSSessionHandler for RiWSPythonHandler {
    async fn on_connect(&self, session_id: &str, remote_addr: &str) -> RiResult<()> {
        let on_connect = Arc::clone(&self.on_connect);
        let session_id = session_id.to_string();
        let remote_addr = remote_addr.to_string();
        
        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let handler = on_connect.clone_ref(py);
                let _ = handler.call(py, (session_id, remote_addr), None);
            });
        }).await.ok();
        
        Ok(())
    }
    
    async fn on_disconnect(&self, session_id: &str) -> RiResult<()> {
        let on_disconnect = Arc::clone(&self.on_disconnect);
        let session_id = session_id.to_string();
        
        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let handler = on_disconnect.clone_ref(py);
                let _ = handler.call(py, (session_id,), None);
            });
        }).await.ok();
        
        Ok(())
    }
    
    async fn on_message(&self, session_id: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        let on_message = Arc::clone(&self.on_message);
        let session_id = session_id.to_string();
        let data_vec = data.to_vec();
        
        let result = tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let handler = on_message.clone_ref(py);
                match handler.call(py, (session_id, data_vec), None) {
                    Ok(obj) => obj.extract::<Vec<u8>>(py).ok(),
                    Err(_) => None,
                }
            })
        }).await.ok().flatten();
        
        Ok(result.unwrap_or_default())
    }
    
    async fn on_error(&self, session_id: &str, error: &str) -> RiResult<()> {
        let on_error = Arc::clone(&self.on_error);
        let session_id = session_id.to_string();
        let error = error.to_string();
        
        tokio::task::spawn_blocking(move || {
            Python::attach(|py| {
                let handler = on_error.clone_ref(py);
                let _ = handler.call(py, (session_id, error), None);
            });
        }).await.ok();
        
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct RiWSSessionManagerPy {
    manager: RiWSSessionManager,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSSessionManagerPy {
    #[new]
    fn new(max_connections: usize) -> Self {
        Self {
            manager: RiWSSessionManager::new(max_connections),
        }
    }
    
    fn get_session_count(&self) -> usize {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.manager.get_session_count().await }))
            .unwrap_or(0)
    }
    
    fn get_all_sessions(&self) -> Vec<RiWSSessionInfo> {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.manager.get_all_sessions().await }))
            .unwrap_or_default()
    }
    
    fn broadcast(&self, data: Vec<u8>) -> usize {
        tokio::runtime::Handle::try_current()
            .map(|handle| handle.block_on(async { self.manager.broadcast(&data).await.unwrap_or(0) }))
            .unwrap_or(0)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiWSServerStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub total_messages_sent: u64,
    pub total_messages_received: u64,
    pub total_bytes_sent: u64,
    pub total_bytes_received: u64,
    pub connection_errors: u64,
    pub message_errors: u64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiWSServerStats {
    #[getter]
    fn get_total_connections(&self) -> u64 {
        self.total_connections
    }
    
    #[getter]
    fn get_active_connections(&self) -> u64 {
        self.active_connections
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
}

impl RiWSServerStats {
    pub fn new() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            total_messages_sent: 0,
            total_messages_received: 0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            connection_errors: 0,
            message_errors: 0,
        }
    }

    pub fn record_connection(&mut self) {
        self.total_connections += 1;
        self.active_connections += 1;
    }

    pub fn record_disconnection(&mut self) {
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }

    pub fn record_message_sent(&mut self, size: usize) {
        self.total_messages_sent += 1;
        self.total_bytes_sent += size as u64;
    }

    pub fn record_message_received(&mut self, size: usize) {
        self.total_messages_received += 1;
        self.total_bytes_received += size as u64;
    }

    pub fn record_connection_error(&mut self) {
        self.connection_errors += 1;
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }

    pub fn record_message_error(&mut self) {
        self.message_errors += 1;
    }
}

impl Default for RiWSServerStats {
    fn default() -> Self {
        Self::new()
    }
}
