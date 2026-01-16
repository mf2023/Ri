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

//! # gRPC Support
//!
//! This module provides gRPC server and client capabilities for DMSC using the tonic framework.
//!
//! ## Key Components
//!
//! - **DMSCGrpcServer**: gRPC server implementation with service registration
//! - **DMSCGrpcClient**: gRPC client for making remote procedure calls
//! - **DMSCGrpcConfig**: Configuration for gRPC server settings
//!
//! ## Usage
//!
//! ```rust
//! use dmsc::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> DMSCResult<()> {
//!     let config = DMSCGrpcConfig::default();
//!     let mut server = DMSCGrpcServer::new(config);
//!
//!     server.add_service(MyServiceImpl::new());
//!
//!     server.start().await?;
//!     Ok(())
//! }
//! ```

use crate::core::DMSCResult;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::net::SocketAddr;
#[cfg(feature = "grpc")]
use tonic::{transport::Server, codegen::StdError};
#[cfg(feature = "grpc")]
use tonic::metadata::MetadataMap;
#[cfg(feature = "grpc")]
use std::collections::HashMap;

#[cfg(feature = "grpc")]
mod server;
#[cfg(feature = "grpc")]
mod client;

#[cfg(feature = "grpc")]
pub use server::DMSCGrpcServer;
#[cfg(feature = "grpc")]
pub use client::DMSCGrpcClient;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGrpcConfig {
    pub addr: String,
    pub port: u16,
    pub max_concurrent_requests: u32,
    pub enable_tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

impl Default for DMSCGrpcConfig {
    fn default() -> Self {
        Self {
            addr: "127.0.0.1".to_string(),
            port: 50051,
            max_concurrent_requests: 100,
            enable_tls: false,
            cert_path: None,
            key_path: None,
        }
    }
}

#[cfg(feature = "grpc")]
#[async_trait]
pub trait DMSCGrpcService: Send + Sync {
    async fn handle_request(&self, method: &str, data: &[u8]) -> DMSCResult<Vec<u8>>;
    fn service_name(&self) -> &'static str;
}

#[cfg(feature = "grpc")]
pub struct DMSCGrpcServiceRegistry {
    pub services: Arc<RwLock<HashMap<String, Arc<dyn DMSCGrpcService>>>>,
}

#[cfg(feature = "grpc")]
impl DMSCGrpcServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn register<S: DMSCGrpcService + 'static>(&self, service: S) {
        let name = service.service_name();
        let mut services = self.services.write();
        services.insert(name.to_string(), Arc::new(service));
    }

    pub async fn get_service(&self, name: &str) -> Option<Arc<dyn DMSCGrpcService>> {
        let services = self.services.read();
        services.get(name).cloned()
    }
}

#[cfg(feature = "grpc")]
impl Default for DMSCGrpcServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCGrpcStats {
    pub requests_received: u64,
    pub requests_completed: u64,
    pub requests_failed: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub active_connections: u64,
}

impl DMSCGrpcStats {
    pub fn new() -> Self {
        Self {
            requests_received: 0,
            requests_completed: 0,
            requests_failed: 0,
            bytes_received: 0,
            bytes_sent: 0,
            active_connections: 0,
        }
    }

    pub fn record_request(&mut self, size: usize) {
        self.requests_received += 1;
        self.bytes_received += size as u64;
        self.active_connections += 1;
    }

    pub fn record_response(&mut self, size: usize) {
        self.requests_completed += 1;
        self.bytes_sent += size as u64;
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }

    pub fn record_error(&mut self) {
        self.requests_failed += 1;
        if self.active_connections > 0 {
            self.active_connections -= 1;
        }
    }
}

#[cfg(feature = "grpc")]
impl Default for DMSCGrpcStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "pyo3")]
use pyo3::{prelude::*, types::PyBytes};

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct DMSCGrpcPythonService {
    service_name: String,
    handler: Py<PyAny>,
}

#[cfg(feature = "pyo3")]
impl DMSCGrpcPythonService {
    pub fn new(service_name: &str, handler: Py<PyAny>) -> Self {
        Self {
            service_name: service_name.to_string(),
            handler,
        }
    }
}

#[cfg(feature = "grpc")]
#[async_trait]
#[cfg(feature = "pyo3")]
impl DMSCGrpcService for DMSCGrpcPythonService {
    async fn handle_request(&self, method: &str, data: &[u8]) -> DMSCResult<Vec<u8>> {
        let gil = Python::acquire_gil();
        let py = gil.python();
        
        let method_str = method.to_string();
        let data_vec = data.to_vec();
        
        let result = py.eval_bound(
            "lambda handler, method, data: handler(method, data)",
            None,
            None,
        )?;
        
        let call_result = result.call((&self.handler, method_str, data_vec), None);
        
        match call_result {
            Ok(obj) => {
                let bytes: Vec<u8> = obj.extract()?;
                Ok(bytes)
            }
            Err(e) => Err(DMSCError::Other(format!("Python handler error: {:?}", e))),
        }
    }
    
    fn service_name(&self) -> &'static str {
        Box::leak(self.service_name.clone().into_boxed_str())
    }
}

#[cfg(feature = "pyo3")]
#[pyclass]
pub struct DMSCGrpcServiceRegistryPy {
    registry: DMSCGrpcServiceRegistry,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl DMSCGrpcServiceRegistryPy {
    #[new]
    fn new() -> Self {
        Self {
            registry: DMSCGrpcServiceRegistry::new(),
        }
    }
    
    fn register(&mut self, service_name: &str, handler: Py<PyAny>) {
        let service = DMSCGrpcPythonService::new(service_name, handler);
        self.registry.register(service);
    }
    
    fn list_services(&self) -> Vec<String> {
        let services = self.registry.services.read();
        services.keys().cloned().collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum GrpcError {
    #[error("Server error: {message}")]
    Server { message: String },
    #[error("Client error: {message}")]
    Client { message: String },
    #[error("Service not found: {service_name}")]
    ServiceNotFound { service_name: String },
    #[error("Connection failed: {message}")]
    ConnectionFailed { message: String },
    #[error("Request timeout")]
    Timeout,
}

impl From<GrpcError> for DMSCError {
    fn from(error: GrpcError) -> Self {
        DMSCError::Other(format!("gRPC error: {}", error))
    }
}

use crate::core::DMSCError;
