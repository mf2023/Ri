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
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;
use tonic::{Request, Response, Status};
use tokio::time::{timeout, Duration};

pub struct DMSCGrpcServer {
    config: DMSCGrpcConfig,
    stats: Arc<RwLock<DMSCGrpcStats>>,
    registry: DMSCGrpcServiceRegistry,
    shutdown_tx: Option<mpsc::Sender<()>>,
    running: Arc<RwLock<bool>>,
}

impl DMSCGrpcServer {
    pub fn new(config: DMSCGrpcConfig) -> Self {
        Self {
            config,
            stats: Arc::new(RwLock::new(DMSCGrpcStats::new())),
            registry: DMSCGrpcServiceRegistry::new(),
            shutdown_tx: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    pub fn register_service<S: DMSCGrpcService + 'static>(&self, service: S) {
        self.registry.register(service);
    }

    pub async fn start(&mut self) -> DMSCResult<()> {
        let addr: SocketAddr = format!("{}:{}", self.config.addr, self.config.port)
            .parse()
            .map_err(|e| GrpcError::Server {
                message: format!("Invalid address: {}", e)
            })?;

        let (shutdown_tx, shutdown_rx) = mpsc::channel(1);
        self.shutdown_tx = Some(shutdown_tx);

        *self.running.write().await = true;

        let stats = self.stats.clone();
        let registry = self.registry.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            let result = Self::run_server(addr, stats, registry, shutdown_rx, running).await;
            if let Err(e) = result {
                tracing::error!("gRPC server error: {}", e);
            }
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
    ) -> Result<(), GrpcError> {
        let service = GrpcServiceImpl {
            stats,
            registry,
        };

        let mut shutdown_rx = shutdown_rx;

        loop {
            let server_result = Server::builder()
                .max_concurrent_requests(self.config.max_concurrent_requests as usize)
                .add_service(tonic::reflection::server::ServerReflection::new(
                    GrpcReflectionServiceImpl,
                ))
                .add_service(DMSCGrpcServiceServer::new(service.clone()))
                .serve_with_shutdown(addr, async {
                    shutdown_rx.recv().await;
                });

            tokio::select! {
                result = server_result => {
                    if let Err(e) = result {
                        return Err(GrpcError::Server {
                            message: format!("Server error: {}", e)
                        });
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(1)) => {
                    if !*running.read().await {
                        break;
                    }
                }
            }
        }

        Ok(())
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

    pub fn get_stats(&self) -> DMSCGrpcStats {
        self.stats.try_read().unwrap().clone()
    }

    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }
}

#[derive(Clone)]
struct GrpcServiceImpl {
    stats: Arc<RwLock<DMSCGrpcStats>>,
    registry: DMSCGrpcServiceRegistry,
}

#[tonic::async_trait]
impl tonic::transport::NamedService for GrpcServiceImpl {
    const NAME: &'static str = "dmsc.grpc.v1";
}

#[tonic::async_trait]
impl DMSCGrpcService for GrpcServiceImpl {
    async fn handle_request(&self, method: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let parts: Vec<&str> = method.split('/').collect();
        if parts.len() < 3 {
            return Err(GrpcError::ServiceNotFound {
                service_name: method.to_string()
            }.into());
        }

        let service_name = parts[1];
        let _method_name = parts[2];

        if let Some(service) = self.registry.get_service(service_name).await {
            service.handle_request(method, data).await
        } else {
            Err(GrpcError::ServiceNotFound {
                service_name: service_name.to_string()
            }.into())
        }
    }

    fn service_name(&self) -> &'static str {
        "dmsc.grpc.v1"
    }
}

struct GrpcReflectionServiceImpl;

#[tonic::async_trait]
impl tonic::reflection::ServerReflectionService for GrpcReflectionServiceImpl {
    async fn file_by_filename(
        &self,
        request: Request<tonic::reflection::FileByFilenameRequest>,
    ) -> Result<Response<tonic::reflection::FileByFilenameResponse>, Status> {
        let _request = request.into_inner();
        Ok(Response::new(tonic::reflection::FileByFilenameResponse {
            file_descriptor_proto: vec![],
        }))
    }

    async fn file_containing_symbol(
        &self,
        request: Request<tonic::reflection::FileContainingSymbolRequest>,
    ) -> Result<Response<tonic::reflection::FileContainingSymbolResponse>, Status> {
        let _request = request.into_inner();
        Ok(Response::new(tonic::reflection::FileContainingSymbolResponse {
            file_descriptor_proto: vec![],
        }))
    }

    async fn list_services(
        &self,
        request: Request<tonic::reflection::ListServicesRequest>,
    ) -> Result<Response<tonic::reflection::ListServicesResponse>, Status> {
        let _request = request.into_inner();
        Ok(Response::new(tonic::reflection::ListServicesResponse {
            service: vec![],
        }))
    }
}

#[derive(Clone)]
pub struct DMSCGrpcServiceServer {
    service: GrpcServiceImpl,
}

impl DMSCGrpcServiceServer {
    pub fn new(service: GrpcServiceImpl) -> Self {
        Self { service }
    }
}

#[tonic::async_trait]
impl tonic::service::Interceptor for DMSCGrpcServiceServer {
    async fn intercept(
        &self,
        request: Request<()>,
    ) -> Result<Request<()>, Status> {
        let mut stats = self.service.stats.write();
        stats.record_request(0);
        Ok(request)
    }
}

impl std::clone::Clone for DMSCGrpcServiceServer {
    fn clone(&self) -> Self {
        Self {
            service: self.service.clone(),
        }
    }
}
