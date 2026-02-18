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

//! # gRPC Server Implementation
//!
//! This module provides the gRPC server implementation for DMSC.
//! Supports service registration, request routing, and streaming RPC.

use super::*;
use tokio::sync::mpsc;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[pyclass]
pub struct DMSCGrpcServer {
    config: DMSCGrpcConfig,
    stats: Arc<RwLock<DMSCGrpcStats>>,
    registry: DMSCGrpcServiceRegistry,
    shutdown_tx: Option<mpsc::Sender<()>>,
    running: Arc<RwLock<bool>>,
}

#[pymethods]
impl DMSCGrpcServer {
    #[new]
    fn new(config: DMSCGrpcConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DMSCGrpcStats::new())),
            registry: DMSCGrpcServiceRegistry::new(),
            shutdown_tx: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    fn register_service(&mut self, service_name: &str, handler: Py<PyAny>) {
        Python::attach(|py| {
            let handler_clone = handler.clone_ref(py);
            let _service = DMSCGrpcPythonService::new(service_name, handler_clone);
            self.registry.register(service_name, handler);
        });
    }

    fn get_stats(&self) -> DMSCGrpcStats {
        self.stats.try_read()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| DMSCGrpcStats::new())
    }

    fn is_running_py(&self) -> bool {
        self.stats.try_read()
            .map(|guard| guard.active_connections > 0)
            .unwrap_or(false)
    }

    fn list_services(&self) -> Vec<String> {
        self.registry.list_services()
    }

    fn start_py(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.start().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }

    fn stop_py(&mut self) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        
        rt.block_on(async {
            self.stop().await
        }).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
    }
}

impl DMSCGrpcServer {
    pub async fn start(&mut self) -> DMSCResult<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.addr, self.config.port).parse()
            .map_err(|e| GrpcError::Server { message: format!("Invalid address: {}", e) })?;

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        *self.running.write().await = true;

        let stats = self.stats.clone();
        let registry = self.registry.clone();
        let running = self.running.clone();
        let max_concurrent = self.config.max_concurrent_requests as usize;

        tokio::spawn(async move {
            let _ = Self::run_server(addr, stats, registry, shutdown_rx, running, max_concurrent).await;
        });

        tracing::info!("gRPC server started on {}", addr);
        Ok(())
    }

    async fn run_server(
        addr: SocketAddr,
        stats: Arc<RwLock<DMSCGrpcStats>>,
        registry: DMSCGrpcServiceRegistry,
        mut shutdown_rx: mpsc::Receiver<()>,
        running: Arc<RwLock<bool>>,
        max_concurrent: usize,
    ) {
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to bind gRPC server to {}: {}", addr, e);
                return;
            }
        };
        
        tracing::info!("gRPC server listening on {}", addr);

        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrent));

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    tracing::info!("gRPC server shutting down");
                    break;
                }
                result = listener.accept() => {
                    match result {
                        Ok((stream, peer_addr)) => {
                            let permit = semaphore.clone().acquire_owned().await;
                            if let Ok(permit) = permit {
                                stats.write().await.active_connections += 1;
                                
                                let stats_clone = stats.clone();
                                let registry_clone = registry.clone();
                                
                                tokio::spawn(async move {
                                    Self::handle_connection(stream, peer_addr, stats_clone, registry_clone).await;
                                    drop(permit);
                                });
                            }
                        }
                        Err(e) => {
                            tracing::error!("Failed to accept connection: {}", e);
                        }
                    }
                }
            }
        }

        *running.write().await = false;
    }

    async fn handle_connection(
        mut stream: tokio::net::TcpStream,
        peer_addr: SocketAddr,
        stats: Arc<RwLock<DMSCGrpcStats>>,
        registry: DMSCGrpcServiceRegistry,
    ) {
        tracing::debug!("gRPC client connected from {}", peer_addr);

        let mut buffer = vec![0u8; 65536];
        
        loop {
            let n = match stream.read(&mut buffer).await {
                Ok(0) => break,
                Ok(n) => n,
                Err(e) => {
                    tracing::debug!("Read error from {}: {}", peer_addr, e);
                    break;
                }
            };

            let request_data = &buffer[..n];
            
            if let Err(e) = Self::process_request(&mut stream, request_data, &stats, &registry).await {
                tracing::error!("Error processing request from {}: {}", peer_addr, e);
                break;
            }
        }

        let mut stats_guard = stats.write().await;
        if stats_guard.active_connections > 0 {
            stats_guard.active_connections -= 1;
        }
    }

    async fn process_request(
        stream: &mut tokio::net::TcpStream,
        request_data: &[u8],
        stats: &Arc<RwLock<DMSCGrpcStats>>,
        registry: &DMSCGrpcServiceRegistry,
    ) -> DMSCResult<()> {
        let request_str = String::from_utf8_lossy(request_data);
        
        let (service_name, method_name) = Self::parse_request_path(&request_str)?;
        
        tracing::debug!("gRPC request: {}/{}", service_name, method_name);

        let services = registry.services.read().await;
        let service = services.get(&service_name).cloned();
        drop(services);

        match service {
            Some(svc) => {
                let body_start = Self::find_body_start(request_data);
                let body_data = if body_start < request_data.len() {
                    &request_data[body_start..]
                } else {
                    &request_data[0..0]
                };

                stats.write().await.record_request(body_data.len());

                match svc.handle_request(&method_name, body_data).await {
                    Ok(response_data) => {
                        stats.write().await.record_response(response_data.len());
                        
                        let grpc_response = Self::build_grpc_response(&response_data);
                        stream.write_all(&grpc_response).await
                            .map_err(|e| GrpcError::Server { message: format!("Write error: {}", e) })?;
                    }
                    Err(e) => {
                        stats.write().await.record_error();
                        
                        let error_response = Self::build_grpc_error_response(&e.to_string());
                        stream.write_all(&error_response).await
                            .map_err(|e| GrpcError::Server { message: format!("Write error: {}", e) })?;
                    }
                }
            }
            None => {
                stats.write().await.record_error();
                
                let error_response = Self::build_grpc_error_response(&format!("Service not found: {}", service_name));
                stream.write_all(&error_response).await
                    .map_err(|e| GrpcError::Server { message: format!("Write error: {}", e) })?;
            }
        }

        Ok(())
    }

    fn parse_request_path(request_str: &str) -> DMSCResult<(String, String)> {
        for line in request_str.lines() {
            if line.contains(":path") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let full_path = parts[1].trim_start_matches('/');
                    let path_parts: Vec<&str> = full_path.splitn(2, '/').collect();
                    if path_parts.len() == 2 {
                        return Ok((path_parts[0].to_string(), path_parts[1].to_string()));
                    }
                }
            }
        }
        
        Err(GrpcError::Server { message: "Invalid request path".to_string() }.into())
    }

    fn find_body_start(buffer: &[u8]) -> usize {
        let mut pos = 0;
        while pos + 3 < buffer.len() {
            if buffer[pos] == b'\r' && buffer[pos + 1] == b'\n' && buffer[pos + 2] == b'\r' && buffer[pos + 3] == b'\n' {
                return pos + 4;
            }
            pos += 1;
        }
        buffer.len()
    }

    fn build_grpc_response(data: &[u8]) -> Vec<u8> {
        let mut response = Vec::new();
        
        let header = "HTTP/2.0 200 OK\r\ncontent-type: application/grpc\r\n\r\n";
        response.extend_from_slice(header.as_bytes());
        
        let len = data.len() as u32;
        response.push(0u8);
        response.extend_from_slice(&len.to_be_bytes()[1..4]);
        response.extend_from_slice(data);
        
        let trailers = "\r\ngrpc-status: 0\r\n\r\n";
        response.extend_from_slice(trailers.as_bytes());
        
        response
    }

    fn build_grpc_error_response(message: &str) -> Vec<u8> {
        let mut response = Vec::new();
        
        let header = "HTTP/2.0 200 OK\r\ncontent-type: application/grpc\r\n\r\n";
        response.extend_from_slice(header.as_bytes());
        
        let trailers = format!("\r\ngrpc-status: 2\r\ngrpc-message: {}\r\n\r\n", message);
        response.extend_from_slice(trailers.as_bytes());
        
        response
    }

    pub async fn stop(&mut self) -> DMSCResult<()> {
        *self.running.write().await = false;

        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(()).await.map_err(|e| GrpcError::Server {
                message: format!("Shutdown error: {}", e)
            })?;
        }

        tracing::info!("gRPC server stopped");
        Ok(())
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

impl Clone for DMSCGrpcServer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            stats: self.stats.clone(),
            registry: self.registry.clone(),
            shutdown_tx: None,
            running: self.running.clone(),
        }
    }
}

impl Default for DMSCGrpcServer {
    fn default() -> Self {
        Self::new(DMSCGrpcConfig::default())
    }
}
