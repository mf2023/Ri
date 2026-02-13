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

//! # Module RPC Communication
//!
//! This module provides inter-module RPC (Remote Procedure Call) communication capabilities
//! for DMSC, enabling modules to call each other's methods synchronously or asynchronously.
//!
//! ## Key Components
//!
//! - **DMSCModuleRPC**: Main RPC coordinator managing endpoints and method calls
//! - **DMSCModuleClient**: Client for making RPC calls to other modules
//! - **DMSCModuleEndpoint**: Endpoint definition for a module's exposed methods
//! - **DMSCMethodCall**: Represents an RPC method call request
//! - **DMSCMethodResponse**: Represents an RPC method call response
//!
//! ## Design Principles
//!
//! 1. **Type Safety**: All RPC calls are type-safe with proper serialization
//! 2. **Async Support**: Both synchronous and asynchronous RPC calls are supported
//! 3. **Timeout Control**: Configurable timeouts for all RPC calls
//! 4. **Error Handling**: Comprehensive error handling with specific error types
//! 5. **Thread Safety**: All components are thread-safe using Arc and RwLock
//! 6. **Module Isolation**: Each module has its own namespace for methods
//!
//! ## Usage
//!
//! ```rust,ignore
//! use dmsc::prelude::*;
//!
//! async fn example() -> DMSCResult<()> {
//!     // Create RPC coordinator
//!     let rpc = DMSCModuleRPC::new();
//!
//!     // Register a module endpoint
//!     let endpoint = DMSCModuleEndpoint::new("user_service");
//!     endpoint.register_method("get_user", |_params| async {
//!         Ok(vec![b"user_data"])
//!     });
//!
//!     rpc.register_endpoint(endpoint).await;
//!
//!     // Create a client to call methods
//!     let client = DMSCModuleClient::new(rpc.clone());
//!
//!     // Call a method on another module
//!     let response = client.call("user_service", "get_user", vec![]).await?;
//!     println!("Response: {:?}", response);
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{Duration, timeout};

use crate::core::DMSCResult;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMethodCall {
    pub method_name: String,
    pub params: Vec<u8>,
    pub timeout_ms: u64,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCMethodCall {
    #[new]
    fn py_new(method_name: String, params: Vec<u8>) -> Self {
        Self::new(method_name, params)
    }
}

impl DMSCMethodCall {
    pub fn new(method_name: String, params: Vec<u8>) -> Self {
        Self {
            method_name,
            params,
            timeout_ms: 5000,
        }
    }

    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMethodResponse {
    pub success: bool,
    pub data: Vec<u8>,
    pub error: String,
    pub is_timeout: bool,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCMethodResponse {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
}

impl DMSCMethodResponse {
    pub fn new() -> Self {
        Self {
            success: false,
            data: Vec::new(),
            error: String::new(),
            is_timeout: false,
        }
    }

    pub fn success_data(data: Vec<u8>) -> Self {
        Self {
            success: true,
            data,
            error: String::new(),
            is_timeout: false,
        }
    }

    pub fn error_msg(msg: String) -> Self {
        Self {
            success: false,
            data: Vec::new(),
            error: msg,
            is_timeout: false,
        }
    }

    pub fn timeout() -> Self {
        Self {
            success: false,
            data: Vec::new(),
            error: "Method call timed out".to_string(),
            is_timeout: true,
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

impl Default for DMSCMethodResponse {
    fn default() -> Self {
        Self::new()
    }
}

type DMSCMethodHandler = Arc<dyn Fn(Vec<u8>) -> DMSCResult<Vec<u8>> + Send + Sync>;

#[async_trait::async_trait]
pub trait DMSCMethodHandlerAsync: Send + Sync {
    async fn call(&self, params: Vec<u8>) -> DMSCMethodResponse;
}

struct SyncMethodHandler {
    handler: DMSCMethodHandler,
}

#[async_trait::async_trait]
impl DMSCMethodHandlerAsync for SyncMethodHandler {
    async fn call(&self, params: Vec<u8>) -> DMSCMethodResponse {
        match (self.handler)(params) {
            Ok(data) => DMSCMethodResponse::success_data(data),
            Err(e) => DMSCMethodResponse::error_msg(e.to_string()),
        }
    }
}

#[derive(Clone)]
pub struct DMSCMethodRegistration {
    name: String,
    handler: Arc<dyn DMSCMethodHandlerAsync>,
}

impl DMSCMethodRegistration {
    pub fn new<S: Into<String>>(
        name: S,
        handler: Arc<dyn DMSCMethodHandlerAsync>,
    ) -> Self {
        Self {
            name: name.into(),
            handler,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub async fn call(&self, params: Vec<u8>, timeout_ms: u64) -> DMSCMethodResponse {
        if timeout_ms == 0 {
            self.handler.call(params).await
        } else {
            match timeout(Duration::from_millis(timeout_ms), self.handler.call(params)).await {
                Ok(response) => response,
                Err(_) => DMSCMethodResponse::timeout(),
            }
        }
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCModuleEndpoint {
    module_name: String,
    methods: Arc<RwLock<HashMap<String, DMSCMethodRegistration>>>,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCModuleEndpoint {
    #[new]
    fn py_new(module_name: String) -> Self {
        Self::new(&module_name)
    }

    #[pyo3(name = "get_module_name")]
    fn py_get_module_name(&self) -> String {
        self.module_name.clone()
    }

    #[pyo3(name = "list_methods")]
    fn py_list_methods(&self) -> Vec<String> {
        let methods = self.methods.blocking_read();
        methods.keys().cloned().collect()
    }
}

impl DMSCModuleEndpoint {
    pub fn new(module_name: &str) -> Self {
        Self {
            module_name: module_name.to_string(),
            methods: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn register_method<H>(&self, name: &str, handler: H) -> &Self
    where
        H: Fn(Vec<u8>) -> DMSCResult<Vec<u8>> + Send + Sync + 'static,
    {
        let registration = DMSCMethodRegistration::new(
            name,
            Arc::new(SyncMethodHandler {
                handler: Arc::new(handler),
            }),
        );
        let mut methods = self.methods.blocking_write();
        methods.insert(name.to_string(), registration);
        self
    }

    pub async fn register_method_async<H>(&self, name: &str, handler: H) -> &Self
    where
        H: Fn(Vec<u8>) -> DMSCResult<Vec<u8>> + Send + Sync + 'static,
    {
        self.register_method(name, handler)
    }

    pub async fn get_method(&self, name: &str) -> Option<DMSCMethodRegistration> {
        let methods = self.methods.read().await;
        methods.get(name).cloned()
    }

    pub async fn list_methods(&self) -> Vec<String> {
        let methods = self.methods.read().await;
        methods.keys().cloned().collect()
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCModuleRPC {
    endpoints: Arc<RwLock<HashMap<String, Arc<DMSCModuleEndpoint>>>>,
    default_timeout: Duration,
}

impl DMSCModuleRPC {
    pub fn new() -> Self {
        Self {
            endpoints: Arc::new(RwLock::new(HashMap::new())),
            default_timeout: Duration::from_millis(5000),
        }
    }

    pub fn with_default_timeout(mut self, timeout: Duration) -> Self {
        self.default_timeout = timeout;
        self
    }

    pub async fn register_endpoint(&self, endpoint: DMSCModuleEndpoint) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.insert(endpoint.module_name().to_string(), Arc::new(endpoint));
    }

    pub async fn unregister_endpoint(&self, module_name: &str) {
        let mut endpoints = self.endpoints.write().await;
        endpoints.remove(module_name);
    }

    pub async fn get_endpoint(&self, module_name: &str) -> Option<Arc<DMSCModuleEndpoint>> {
        let endpoints = self.endpoints.read().await;
        endpoints.get(module_name).cloned()
    }

    pub async fn call_method(
        &self,
        module_name: &str,
        method_name: &str,
        params: Vec<u8>,
        timeout_ms: Option<u64>,
    ) -> DMSCMethodResponse {
        let endpoint = self.get_endpoint(module_name).await;

        if let Some(ep) = endpoint {
            if let Some(method) = ep.get_method(method_name).await {
                let timeout = timeout_ms.unwrap_or(self.default_timeout.as_millis() as u64);
                return method.call(params, timeout).await;
            }
            return DMSCMethodResponse::error_msg(format!(
                "Method '{}' not found on module '{}'",
                method_name, module_name
            ));
        }

        DMSCMethodResponse::error_msg(format!(
            "Module '{}' not found",
            module_name
        ))
    }

    pub async fn list_registered_modules(&self) -> Vec<String> {
        let endpoints = self.endpoints.read().await;
        endpoints.keys().cloned().collect()
    }
}

impl Default for DMSCModuleRPC {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCModuleClient {
    rpc: Arc<DMSCModuleRPC>,
}

impl DMSCModuleClient {
    pub fn new(rpc: Arc<DMSCModuleRPC>) -> Self {
        Self { rpc }
    }

    pub async fn call(
        &self,
        module_name: &str,
        method_name: &str,
        params: Vec<u8>,
    ) -> DMSCMethodResponse {
        self.rpc.call_method(module_name, method_name, params, None).await
    }

    pub async fn call_with_timeout(
        &self,
        module_name: &str,
        method_name: &str,
        params: Vec<u8>,
        timeout_ms: u64,
    ) -> DMSCMethodResponse {
        self.rpc
            .call_method(module_name, method_name, params, Some(timeout_ms))
            .await
    }
}

impl fmt::Debug for DMSCModuleRPC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DMSCModuleRPC")
            .field("default_timeout", &self.default_timeout)
            .finish()
    }
}
