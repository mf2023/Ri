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

//! # gRPC Support

use crate::core::RiResult;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
#[cfg(feature = "grpc")]
use std::collections::HashMap as FxHashMap;

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg(feature = "grpc")]
mod server;
#[cfg(feature = "grpc")]
mod client;

#[cfg(feature = "grpc")]
pub use server::RiGrpcServer;
#[cfg(feature = "grpc")]
pub use client::RiGrpcClient;

#[cfg(all(feature = "grpc", feature = "pyo3"))]
pub use server::RiGrpcServerPy;
#[cfg(all(feature = "grpc", feature = "pyo3"))]
pub use client::RiGrpcClientPy;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiGrpcConfig {
    pub addr: String,
    pub port: u16,
    pub max_concurrent_requests: u32,
    pub enable_tls: bool,
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiGrpcConfig {
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
    
    #[getter]
    fn get_max_concurrent_requests(&self) -> u32 {
        self.max_concurrent_requests
    }
    
    #[setter]
    fn set_max_concurrent_requests(&mut self, max_concurrent_requests: u32) {
        self.max_concurrent_requests = max_concurrent_requests;
    }
    
    #[getter]
    fn get_enable_tls(&self) -> bool {
        self.enable_tls
    }
    
    #[setter]
    fn set_enable_tls(&mut self, enable_tls: bool) {
        self.enable_tls = enable_tls;
    }
    
    #[getter]
    fn get_cert_path(&self) -> Option<String> {
        self.cert_path.clone()
    }
    
    #[setter]
    fn set_cert_path(&mut self, cert_path: Option<String>) {
        self.cert_path = cert_path;
    }
    
    #[getter]
    fn get_key_path(&self) -> Option<String> {
        self.key_path.clone()
    }
    
    #[setter]
    fn set_key_path(&mut self, key_path: Option<String>) {
        self.key_path = key_path;
    }
}

impl Default for RiGrpcConfig {
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
pub trait RiGrpcService: Send + Sync {
    async fn handle_request(&self, method: &str, data: &[u8]) -> RiResult<Vec<u8>>;
    fn service_name(&self) -> &'static str;
}

#[cfg(feature = "grpc")]
#[derive(Clone)]
pub struct RiGrpcServiceRegistry {
    pub services: Arc<RwLock<FxHashMap<String, Arc<dyn RiGrpcService>>>>,
}

#[cfg(feature = "grpc")]
impl RiGrpcServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(FxHashMap::default())),
        }
    }

    pub fn register_service(&mut self, service: Arc<dyn RiGrpcService>) {
        let name = service.service_name().to_string();
        let mut services = self.services.blocking_write();
        services.insert(name, service);
    }

    pub fn list_services(&self) -> Vec<String> {
        let services = self.services.blocking_read();
        services.keys().cloned().collect()
    }
}

#[cfg(feature = "grpc")]
impl Default for RiGrpcServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
#[pyclass]
pub struct RiGrpcServiceRegistryPy {
    registry: RiGrpcServiceRegistry,
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
#[pymethods]
impl RiGrpcServiceRegistryPy {
    #[new]
    fn new() -> Self {
        Self {
            registry: RiGrpcServiceRegistry::new(),
        }
    }
    
    fn register(&mut self, service_name: &str, handler: Py<PyAny>) {
        let service = RiGrpcPythonService::new(service_name, handler);
        self.registry.register_service(Arc::new(service));
    }
    
    fn list_services(&self) -> Vec<String> {
        self.registry.list_services()
    }
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
impl Default for RiGrpcServiceRegistryPy {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyclass)]
pub struct RiGrpcStats {
    pub requests_received: u64,
    pub requests_completed: u64,
    pub requests_failed: u64,
    pub bytes_received: u64,
    pub bytes_sent: u64,
    pub active_connections: u64,
}

#[cfg(feature = "pyo3")]
#[pymethods]
impl RiGrpcStats {
    #[getter]
    fn get_requests_received(&self) -> u64 {
        self.requests_received
    }

    #[getter]
    fn get_requests_completed(&self) -> u64 {
        self.requests_completed
    }

    #[getter]
    fn get_requests_failed(&self) -> u64 {
        self.requests_failed
    }

    #[getter]
    fn get_bytes_received(&self) -> u64 {
        self.bytes_received
    }

    #[getter]
    fn get_bytes_sent(&self) -> u64 {
        self.bytes_sent
    }

    #[getter]
    fn get_active_connections(&self) -> u64 {
        self.active_connections
    }
}

impl RiGrpcStats {
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

impl Default for RiGrpcStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
#[pyclass]
pub struct RiGrpcPythonService {
    service_name: String,
    handler: Py<PyAny>,
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
impl RiGrpcPythonService {
    pub fn new(service_name: &str, handler: Py<PyAny>) -> Self {
        Self {
            service_name: service_name.to_string(),
            handler,
        }
    }
}

#[cfg(all(feature = "grpc", feature = "pyo3"))]
#[async_trait]
impl RiGrpcService for RiGrpcPythonService {
    async fn handle_request(&self, method: &str, data: &[u8]) -> RiResult<Vec<u8>> {
        let method_str = method.to_string();
        let data_vec = data.to_vec();
        
        let result = pyo3::Python::attach(|py| {
            self.handler.call1(py, (method_str, data_vec))
        });
        
        match result {
            Ok(obj) => {
                let result_vec = pyo3::Python::attach(|py| {
                    obj.extract::<Vec<u8>>(py)
                });
                match result_vec {
                    Ok(bytes) => Ok(bytes),
                    Err(e) => Err(RiError::Other(format!("Failed to extract response bytes: {:?}", e))),
                }
            }
            Err(e) => Err(RiError::Other(format!("Python handler error: {:?}", e))),
        }
    }
    
    fn service_name(&self) -> &'static str {
        Box::leak(self.service_name.clone().into_boxed_str())
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

impl From<GrpcError> for RiError {
    fn from(error: GrpcError) -> Self {
        RiError::Other(format!("gRPC error: {}", error))
    }
}

use crate::core::RiError;
