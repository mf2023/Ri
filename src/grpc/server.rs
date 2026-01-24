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

use super::*;
use tokio::sync::mpsc;
use std::net::SocketAddr;

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

    fn register_service(&self, service_name: &str, handler: Py<PyAny>) {
        let service = DMSCGrpcPythonService::new(service_name, handler);
        self.registry.register(service);
    }

    fn get_stats(&self) -> DMSCGrpcStats {
        self.stats.try_read().unwrap().clone()
    }
}

impl DMSCGrpcServer {
    pub async fn start(&mut self) -> DMSCResult<()> {
        let addr: SocketAddr = self.config.addr.parse()
            .map_err(|e| GrpcError::Server { message: format!("Invalid address: {}", e) })?;

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        *self.running.write().await = true;

        let stats = self.stats.clone();
        let registry = self.registry.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            let _ = Self::run_server(addr, stats, registry, shutdown_rx, running).await;
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
    ) {
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("Failed to bind gRPC server to {}: {}", addr, e);
                return;
            }
        };
        
        tracing::info!("gRPC server listening on {}", addr);

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    tracing::info!("gRPC server shutting down");
                    break;
                }
                result = listener.accept() => {
                    match result {
                        Ok((_stream, _)) => {
                            stats.write().await.record_request(0);
                            tracing::debug!("gRPC client connected");
                        }
                        Err(e) => {
                            tracing::error!("Failed to accept connection: {}", e);
                        }
                    }
                }
            }
        }
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
