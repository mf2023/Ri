//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! # Service Context
//! 
//! The service context provides access to all core functionalities of DMSC,
//! acting as a central hub for accessing various components such as logging,
//! configuration, file system, and hooks.

#![allow(non_snake_case)]

use crate::fs::DMSCFileSystem;
use crate::log::{DMSCLogConfig, DMSCLogger};
use crate::config::DMSCConfigManager;
use crate::hooks::DMSCHookBus;
use crate::core::DMSCResult;
use crate::observability::DMSCMetricsRegistry;
use std::sync::Arc;

/// Internal service context implementation. Not exposed directly to users.
/// 
/// This struct contains all the core components of the service context,
/// but is wrapped by `DMSCServiceContext` for controlled access.
#[derive(Clone)]
pub struct ServiceContextInner {
    /// File system accessor for secure file operations
    pub fs: DMSCFileSystem,
    /// Logger for structured logging
    pub logger: Arc<DMSCLogger>,
    /// Configuration manager for accessing application settings
    pub config: Arc<DMSCConfigManager>,
    /// Hook bus for emitting and handling lifecycle events
    pub hooks: Arc<DMSCHookBus>,
    /// Metrics registry for observability (optional)
    pub metrics_registry: Option<Arc<DMSCMetricsRegistry>>,
}

impl ServiceContextInner {
    /// Create a new `ServiceContextInner` instance with the provided components.
    /// 
    /// # Parameters
    /// 
    /// - `fs`: File system accessor
    /// - `logger`: Structured logger
    /// - `config`: Configuration manager
    /// - `hooks`: Hook bus for lifecycle events
    /// - `metrics_registry`: Optional metrics registry for observability
    /// 
    /// # Returns
    /// 
    /// A new `ServiceContextInner` instance.
    pub fn new(fs: DMSCFileSystem, logger: DMSCLogger, config: DMSCConfigManager, hooks: DMSCHookBus, metrics_registry: Option<Arc<DMSCMetricsRegistry>>) -> Self {
        ServiceContextInner { 
            fs, 
            logger: Arc::new(logger), 
            config: Arc::new(config), 
            hooks: Arc::new(hooks), 
            metrics_registry 
        }
    }
    

}

/// Public-facing service context for DMSC applications.
/// 
/// The `DMSCServiceContext` is the primary way for modules and business logic to
/// access core DMSC functionalities. It follows the dependency injection pattern,
/// providing a centralized access point to all core components.
/// 
/// ## Design Principle
/// 
/// The service context is designed to be immutable from the outside, with controlled
/// access to mutable components through dedicated methods. This ensures thread safety
/// while allowing for necessary mutations in specific contexts.
/// 
/// ## Usage
/// 
/// ```rust
/// use dmsc::prelude::*;
/// 
/// async fn handle_request(ctx: &DMSCServiceContext) -> DMSCResult<()> {
///     // Access logger
///     ctx.logger().info("request", "Handling request");
///     
///     // Access configuration
///     let config_value = ctx.config().config().get_str("app.name");
///     
///     // Access file system
///     let file_path = ctx.fs().app_data_path("logs/app.log");
///     
///     Ok(())
/// }
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSCServiceContext {
    /// Internal implementation details
    inner: ServiceContextInner,
}

impl DMSCServiceContext {
    /// Create a new `DMSCServiceContext` with the provided components.
    /// 
    /// This method is typically used by the framework itself during application startup,
    /// but can be used for testing or custom initialization.
    /// 
    /// # Parameters
    /// 
    /// - `fs`: File system accessor
    /// - `logger`: Structured logger
    /// - `config`: Configuration manager
    /// - `hooks`: Hook bus for lifecycle events
    /// - `metrics_registry`: Optional metrics registry for observability
    /// 
    /// # Returns
    /// 
    /// A new `DMSCServiceContext` instance.
    pub fn new_with(fs: DMSCFileSystem, logger: DMSCLogger, config: DMSCConfigManager, hooks: DMSCHookBus, metrics_registry: Option<Arc<DMSCMetricsRegistry>>) -> Self {
        let inner = ServiceContextInner::new(fs, logger, config, hooks, metrics_registry);
        DMSCServiceContext { inner }
    }
    


    /// Create a new `DMSCServiceContext` with default configuration.
    /// 
    /// This is the most common way to create a service context, as it handles
    /// the initialization of all core components automatically.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` containing the new service context, or an error if initialization failed.
    /// 
    /// # Errors
    /// 
    /// - If the project root directory cannot be determined
    /// - If there are issues initializing any of the core components
    pub fn new_default() -> DMSCResult<Self> {
        // Create default configuration manager
        let config = DMSCConfigManager::new_default();
        let cfg = config.config();

        // Determine project root directory
        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSCError::Other(format!("detect project root failed: {e}")))?;
        
        // Determine application data root directory
        let app_data_root = if let Some(root_str) = cfg.get_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        // Initialize file system
        let fs = DMSCFileSystem::new_with_roots(project_root, app_data_root);

        // Initialize logging
        let log_config = DMSCLogConfig::from_config(&cfg);
        let logger = DMSCLogger::new(&log_config, fs.clone());
        
        // Initialize hook bus
        let hooks = DMSCHookBus::new();
        
        Ok(DMSCServiceContext::new_with(fs, logger, config, hooks, None))
    }

    /// Get a reference to the file system accessor.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCFileSystem` instance.
    pub fn fs(&self) -> &DMSCFileSystem {
        &self.inner.fs
    }
    


    /// Get a reference to the structured logger.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCLogger` instance.
    pub fn logger(&self) -> &DMSCLogger {
        self.inner.logger.as_ref()
    }
    


    /// Get a reference to the configuration manager.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCConfigManager` instance.
    pub fn config(&self) -> Arc<DMSCConfigManager> {
        self.inner.config.clone()
    }
    


    /// Get a reference to the hook bus for emitting events.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSCHookBus` instance.
    pub fn hooks(&self) -> Arc<DMSCHookBus> {
        self.inner.hooks.clone()
    }
    


    /// Get a mutable reference to the hook bus for registering handlers.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSCHookBus` instance.
    pub fn hooks_mut(&mut self) -> &mut DMSCHookBus {
        Arc::get_mut(&mut self.inner.hooks).expect("Cannot get mutable reference to hooks - shared ownership")
    }

    /// Get a mutable reference to the configuration manager.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSCConfigManager` instance.
    pub fn config_mut(&mut self) -> &mut DMSCConfigManager {
        Arc::get_mut(&mut self.inner.config).expect("Cannot get mutable reference to config - shared ownership")
    }

    /// Get a mutable reference to the file system accessor.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to to the `DMSCFileSystem` instance.
    pub fn fs_mut(&mut self) -> &mut DMSCFileSystem {
        &mut self.inner.fs
    }

    /// Get a mutable reference to the structured logger.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSCLogger` instance.
    pub fn logger_mut(&mut self) -> &mut DMSCLogger {
        Arc::get_mut(&mut self.inner.logger).expect("Cannot get mutable reference to logger - shared ownership")
    }

    /// Get a reference to the metrics registry if available.
    /// 
    /// # Returns
    /// 
    /// An optional reference to the `DMSCMetricsRegistry` instance.
    pub fn metrics_registry(&self) -> Option<Arc<DMSCMetricsRegistry>> {
        self.inner.metrics_registry.clone()
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCServiceContext
#[pyo3::prelude::pymethods]
impl DMSCServiceContext {
    /// Create a new DMSCServiceContext with default configuration
    #[new]
    fn py_new() -> pyo3::PyResult<Self> {
        match Self::new_default() {
            Ok(ctx) => Ok(ctx),
            Err(err) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create service context: {err}"))),
        }
    }

    #[pyo3(name = "fs")]
    fn fs_py(&self) -> crate::fs::DMSCFileSystem {
        self.inner.fs.clone()
    }

    #[pyo3(name = "logger")]
    fn logger_py(&self) -> crate::log::DMSCLogger {
        (*self.inner.logger).clone()
    }

    #[pyo3(name = "config")]
    fn config_py(&self) -> crate::config::DMSCConfigManager {
        (*self.inner.config).clone()
    }

    #[pyo3(name = "metrics_registry")]
    fn metrics_registry_py(&self) -> Option<crate::observability::DMSCMetricsRegistry> {
        self.inner.metrics_registry.as_ref().map(|r| (**r).clone())
    }
}
