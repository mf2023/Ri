//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

#![allow(non_snake_case)]

//! # Application Builder
//!
//! This module provides the application builder for constructing Ri applications.
//! The `RiAppBuilder` follows the builder pattern for fluent configuration,
//! enabling developers to compose applications from various modules, configuration
//! sources, and runtime settings in a declarative manner.
//!
//! ## Builder Pattern
//!
//! The builder pattern allows step-by-step construction of complex objects.
//! Each method on `RiAppBuilder` configures a specific aspect of the application
//! and returns the builder for method chaining. This results in a fluent API
//! that is both readable and type-safe.
//!
//! ## Module Registration
//!
//! Modules are the primary extension point for Ri applications. The builder
//! supports multiple types of modules:
//!
//! - **Synchronous modules**: Implement `ServiceModule` trait for sync operations
//! - **Asynchronous modules**: Implement `AsyncServiceModule` trait for async operations
//! - **Ri modules**: Implement public `RiModule` trait (converted to async internally)
//! - **Python modules**: Modules created from Python code (with pyo3 feature)
//!
//! ## Configuration Management
//!
//! The builder supports multiple configuration sources with a defined priority order:
//!
//! 1. Configuration files (lowest priority): `dms.yaml`, `dms.yml`, `dms.toml`, `dms.json`
//! 2. Custom configuration via `with_config()` method
//! 3. Environment variables (highest priority)
//!
//! ## Usage Example
//!
//! ```rust
//! use ri::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     let app = RiAppBuilder::new()
//!         .with_config("config.yaml")?
//!         .with_module(Box::new(MySyncModule::new()))
//!         .with_async_module(Box::new(MyAsyncModule::new()))
//!         .build()?;
//!
//!     app.run(|ctx| async move {
//!         ctx.logger().info("service", "Ri service started")?;
//!         Ok(())
//!     }).await
//! }
//! ```
//!
//! ## Thread Safety
//!
//! The `RiAppBuilder` is designed to be used in a single-threaded context
//! during application construction. After calling `build()`, the resulting
//! `RiAppRuntime` is safe to use across multiple threads.
//!
//! ## Error Handling
//!
//! All builder methods that can fail return `RiResult`, enabling proper
//! error handling through the `?` operator or explicit match statements.

use crate::core::{RiResult, RiServiceContext, ServiceModule, AsyncServiceModule};
use super::module_sorter::sort_modules;
use super::module_types::{ModuleSlot, ModuleType};
use super::lifecycle::RiLifecycleObserver;
use super::analytics::RiLogAnalyticsModule;
use std::sync::Arc;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;
#[cfg(feature = "pyo3")]
use crate::core::app_runtime::RiAppRuntime;

/// Public-facing application builder for Ri.
/// 
/// The `RiAppBuilder` provides a fluent API for configuring and building Ri applications.
/// It follows the builder pattern, allowing users to configure various aspects of the application
/// before building the final runtime.
/// 
/// ## Usage
/// 
/// ```rust
/// use ri::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> RiResult<()> {
///     let app = RiAppBuilder::new()
///         .with_config("config.yaml")?
///         .with_module(Box::new(MySyncModule::new()))
///         .with_async_module(Box::new(MyAsyncModule::new()))
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "Ri service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct RiAppBuilder {
    /// Vector of modules with their state, including both sync and async modules
    modules: Vec<ModuleSlot>, 
    /// Configuration file paths to load
    config_paths: Vec<String>, 
    /// Custom logging configuration (optional)
    logging_config: Option<crate::log::RiLogConfig>, 
    /// Custom observability configuration (optional)
    observability_config: Option<crate::observability::RiObservabilityConfig>, 
}
 
impl Default for RiAppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
 
impl RiAppBuilder {
    /// Create a new empty application builder.
    /// 
    /// # Returns
    /// 
    /// A new `RiAppBuilder` instance with default settings.
    pub fn new() -> Self {
        RiAppBuilder {
            modules: Vec::new(),
            config_paths: Vec::new(),
            logging_config: None,
            observability_config: None,
        }
    }
 
    /// Add a synchronous module to the application.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed synchronous module implementing `ServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_module(mut self, module: Box<dyn ServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Sync(module), failed: false });
        self
    }
    
    /// Add a Python module to the application.
    /// 
    /// This method adds a module created from Python code to the application.
    /// The module will be treated as an asynchronous Ri module.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A Python module adapter implementing module configuration
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    #[cfg(feature = "pyo3")]
    pub fn with_python_module(mut self, module: crate::core::module::RiPythonModuleAdapter) -> Self {
        self.modules.push(ModuleSlot { 
            module: ModuleType::Async(Box::new(module)), 
            failed: false 
        });
        self
    }
 
    /// Add an asynchronous module to the application.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed asynchronous module implementing `AsyncServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_async_module(mut self, module: Box<dyn AsyncServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        self
    }
 
    /// Add a Ri module to the application.
    /// 
    /// This method adds a module implementing the public `RiModule` trait to the application.
    /// The module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed module implementing `RiModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_dms_module(mut self, module: Box<dyn crate::core::RiModule>) -> Self {
        // Wrap RiModule into AsyncServiceModule adapter
        struct RiModuleAdapter(Box<dyn crate::core::RiModule + Send + Sync + 'static>);
        
        #[async_trait::async_trait]
        impl AsyncServiceModule for RiModuleAdapter {
            fn name(&self) -> &str {
                self.0.name()
            }
            
            fn is_critical(&self) -> bool {
                self.0.is_critical()
            }
            
            fn priority(&self) -> i32 {
                self.0.priority()
            }
            
            fn dependencies(&self) -> Vec<&str> {
                self.0.dependencies()
            }
            
            async fn init(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.init(ctx).await
            }
            
            async fn before_start(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.before_start(ctx).await
            }
            
            async fn start(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.start(ctx).await
            }
            
            async fn after_start(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.after_start(ctx).await
            }
            
            async fn before_shutdown(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.before_shutdown(ctx).await
            }
            
            async fn shutdown(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.shutdown(ctx).await
            }
            
            async fn after_shutdown(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
                self.0.after_shutdown(ctx).await
            }
        }
        
        self.modules.push(ModuleSlot { 
            module: ModuleType::Async(Box::new(RiModuleAdapter(module))), 
            failed: false 
        });
        self
    }
 
    /// Add multiple synchronous modules to the application.
    /// 
    /// # Parameters
    /// 
    /// - `modules`: A vector of boxed synchronous modules implementing `ServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_modules(mut self, modules: Vec<Box<dyn ServiceModule>>) -> Self {
        for module in modules {
            self.modules.push(ModuleSlot { module: ModuleType::Sync(module), failed: false });
        }
        self
    }
 
    /// Add multiple asynchronous modules to the application.
    /// 
    /// # Parameters
    /// 
    /// - `modules`: A vector of boxed asynchronous modules implementing `AsyncServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_async_modules(mut self, modules: Vec<Box<dyn AsyncServiceModule>>) -> Self {
        for module in modules {
            self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        }
        self
    }
    
    /// Add multiple Ri modules to the application.
    /// 
    /// This method adds multiple modules implementing the public `RiModule` trait to the application.
    /// Each module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `modules`: A vector of boxed modules implementing `RiModule`
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_dms_modules(mut self, modules: Vec<Box<dyn crate::core::RiModule>>) -> Self {
        for module in modules {
            self = self.with_dms_module(module);
        }
        self
    }
 
    /// Add a configuration file to the application.
    /// 
    /// # Parameters
    /// 
    /// - `config_path`: Path to the configuration file
    /// 
    /// # Returns
    /// 
    /// A `RiResult` containing the updated `RiAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `RiResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_config(mut self, config_path: impl Into<String>) -> RiResult<Self> {
        self.config_paths.push(config_path.into());
        Ok(self)
    }
 
    /// Set custom logging configuration for the application.
    /// 
    /// # Parameters
    /// 
    /// - `logging_config`: Custom logging configuration
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_logging(mut self, logging_config: crate::log::RiLogConfig) -> Self {
        self.logging_config = Some(logging_config);
        self
    }
 
    /// Set custom observability configuration for the application.
    /// 
    /// # Parameters
    /// 
    /// - `observability_config`: Custom observability configuration
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_observability(mut self, observability_config: crate::observability::RiObservabilityConfig) -> Self {
        self.observability_config = Some(observability_config);
        self
    }

    /// Add cache module with configuration.
    /// 
    /// This method adds a cache module to the application with custom configuration.
    /// The configuration is provided via a closure that receives a cache config builder.
    /// 
    /// # Parameters
    /// 
    /// - `config_fn`: Closure for configuring the cache module
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_cache_module<F>(mut self, config_fn: F) -> Self
    where
        F: FnOnce(&mut crate::cache::RiCacheConfig) -> &mut crate::cache::RiCacheConfig,
    {
        let mut config = crate::cache::RiCacheConfig::default();
        config_fn(&mut config);
        let cache_module = crate::cache::RiCacheModule::with_config(config);
        self.modules.push(ModuleSlot {
            module: ModuleType::Sync(Box::new(cache_module)),
            failed: false,
        });
        self
    }

    /// Add authentication module with configuration.
    /// 
    /// This method adds an authentication module to the application with custom configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config_fn`: Closure for configuring the auth module
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_auth_module<F>(mut self, config_fn: F) -> Self
    where
        F: FnOnce(&mut crate::auth::RiAuthConfig) -> &mut crate::auth::RiAuthConfig,
    {
        let mut config = crate::auth::RiAuthConfig::default();
        config_fn(&mut config);
        let auth_module = crate::auth::RiAuthModule::with_config(config);
        self.modules.push(ModuleSlot {
            module: ModuleType::Sync(Box::new(auth_module)),
            failed: false,
        });
        self
    }

    /// Add queue module with configuration.
    /// 
    /// This method adds a message queue module to the application with custom configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config_fn`: Closure for configuring the queue module
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_queue_module<F>(mut self, config_fn: F) -> Self
    where
        F: FnOnce(&mut crate::queue::RiQueueConfig) -> &mut crate::queue::RiQueueConfig,
    {
        let mut config = crate::queue::RiQueueConfig::default();
        config_fn(&mut config);
        match crate::queue::RiQueueModule::with_config(config) {
            Ok(queue_module) => {
                self.modules.push(ModuleSlot {
                    module: ModuleType::Sync(Box::new(queue_module)),
                    failed: false,
                });
            }
            Err(e) => {
                log::error!("Failed to create queue module: {}", e);
            }
        }
        self
    }

    /// Add device control module with configuration.
    /// 
    /// This method adds a device control module to the application with custom configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config_fn`: Closure for configuring the device module
    /// 
    /// # Returns
    /// 
    /// The updated `RiAppBuilder` instance for method chaining.
    pub fn with_device_module<F>(mut self, config_fn: F) -> Self
    where
        F: FnOnce(&mut crate::device::RiDeviceControlConfig) -> &mut crate::device::RiDeviceControlConfig,
    {
        let mut config = crate::device::RiDeviceControlConfig::default();
        config_fn(&mut config);
        let device_module = crate::device::RiDeviceControlModule::new().with_config(config);
        self.modules.push(ModuleSlot {
            module: ModuleType::Sync(Box::new(device_module)),
            failed: false,
        });
        self
    }
 
    /// Build the application runtime.
    /// 
    /// This method performs the following steps:
    /// 1. Creates and configures the configuration manager
    /// 2. Loads configuration from specified files and environment variables
    /// 3. Creates the service context with core functionalities
    /// 4. Adds core modules (analytics and lifecycle observer)
    /// 5. Sorts modules based on dependencies and priority
    /// 6. Creates and returns the application runtime
    /// 
    /// # Returns
    /// 
    /// A `RiResult` containing the built `RiAppRuntime` instance, or an error if building fails.
    /// 
    /// # Errors
    /// 
    /// - If configuration loading fails
    /// - If service context creation fails
    /// - If module sorting fails due to circular dependencies
    pub fn build(mut self) -> RiResult<super::app_runtime::RiAppRuntime> {
        // Create config manager with specified config paths
        let mut config_manager = crate::config::RiConfigManager::new();
        
        // Add specified config files
        for path in &self.config_paths {
            config_manager.add_file_source(path);
        }
        
        // Add default config sources if no paths specified
        if self.config_paths.is_empty() {
            if let Ok(cwd) = std::env::current_dir() {
                let config_dir = cwd.join("config");
                
                // Add all supported config files in order of priority (lowest to highest)
                config_manager.add_file_source(config_dir.join("dms.yaml"));
                config_manager.add_file_source(config_dir.join("dms.yml"));
                config_manager.add_file_source(config_dir.join("dms.toml"));
                config_manager.add_file_source(config_dir.join("dms.json"));
            }
        }
        
        // Add environment variables as highest priority
        config_manager.add_environment_source();
        
        // Load configuration
        config_manager.load()?;
 
        // Create service context with custom configuration
        let ctx = self.create_service_context(config_manager)?;
        
        // Add core modules
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(RiLogAnalyticsModule::new())), failed: false });
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(RiLifecycleObserver::new())), failed: false });
        
        // Sort modules based on dependencies and priority
        self.modules = sort_modules(self.modules)?;
        
        let runtime = super::app_runtime::RiAppRuntime::new(ctx, self.modules);
        Ok(runtime)
    }
    
    /// Create the service context with the given configuration manager.
    /// 
    /// This method creates the service context with the following components:
    /// 1. File system accessor
    /// 2. Logger (using custom config if provided, otherwise from configuration)
    /// 3. Configuration manager
    /// 4. Hook bus for lifecycle events
    /// 
    /// # Parameters
    /// 
    /// - `config_manager`: Configuration manager with loaded configuration
    /// 
    /// # Returns
    /// 
    /// A `RiResult` containing the created `RiServiceContext` instance, or an error if creation fails.
    /// 
    /// # Errors
    /// 
    /// - If project root directory detection fails
    /// - If file system creation fails
    /// - If logger creation fails
    fn create_service_context(&self, config_manager: crate::config::RiConfigManager) -> RiResult<RiServiceContext> {
        let cfg = config_manager.config();
 
        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::RiError::Other(format!("detect project root failed: {e}")))?;
        let app_data_root = if let Some(root_str) = cfg.get_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };
 
        let fs = crate::fs::RiFileSystem::new_with_roots(project_root, app_data_root);

        // Use custom logging config if provided, otherwise create from config
        let log_config: crate::log::RiLogConfig = if let Some(log_config) = &self.logging_config {
            log_config.clone()
        } else {
            crate::log::RiLogConfig::from_config(&cfg)
        };
        let logger = crate::log::RiLogger::new(&log_config, fs.clone());
        let hooks = crate::hooks::RiHookBus::new();
        let metrics_registry = Some(Arc::new(crate::observability::RiMetricsRegistry::new()));
        
        Ok(RiServiceContext::new_with(fs, logger, config_manager, hooks, metrics_registry))
    }
}
 
#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiAppBuilder {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    fn py_with_config(&mut self, config_path: &str) -> PyResult<Self> {
        self.config_paths.push(config_path.to_string());
        Ok(std::mem::take(self))
    }

    fn py_with_logging(&mut self, logging_config: crate::log::RiLogConfig) -> PyResult<Self> {
        self.logging_config = Some(logging_config);
        Ok(std::mem::take(self))
    }

    fn py_with_observability(&mut self, observability_config: crate::observability::RiObservabilityConfig) -> PyResult<Self> {
        self.observability_config = Some(observability_config);
        Ok(std::mem::take(self))
    }

    fn py_build(&mut self) -> PyResult<RiAppRuntime> {
        let builder = std::mem::take(self);
        RiAppBuilder::build(builder).map_err(|e| pyo3::prelude::PyErr::from(e))
    }

    fn py_with_module(&mut self, module: super::module::RiPythonServiceModule) -> PyResult<Self> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Sync(Box::new(module)),
            failed: false,
        });
        Ok(std::mem::take(self))
    }

    fn py_with_python_module(&mut self, module: super::module::RiPythonModuleAdapter) -> PyResult<Self> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(std::mem::take(self))
    }

    fn py_with_async_module(&mut self, module: super::module::RiPythonAsyncServiceModule) -> PyResult<Self> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(std::mem::take(self))
    }

    fn py_with_dms_module(&mut self, module: super::module::RiPythonModuleAdapter) -> PyResult<Self> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(std::mem::take(self))
    }

    fn py_with_modules(&mut self, modules: Vec<super::module::RiPythonServiceModule>) -> PyResult<Self> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Sync(Box::new(module)),
                failed: false,
            });
        }
        Ok(std::mem::take(self))
    }

    fn py_with_async_modules(&mut self, modules: Vec<super::module::RiPythonAsyncServiceModule>) -> PyResult<Self> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Async(Box::new(module)),
                failed: false,
            });
        }
        Ok(std::mem::take(self))
    }

    fn py_with_dms_modules(&mut self, modules: Vec<super::module::RiPythonModuleAdapter>) -> PyResult<Self> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Async(Box::new(module)),
                failed: false,
            });
        }
        Ok(std::mem::take(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_builder_creation() {
        let builder = RiAppBuilder::new();
        assert!(builder.modules.is_empty());
        assert!(builder.config_paths.is_empty());
        assert!(builder.logging_config.is_none());
        assert!(builder.observability_config.is_none());
    }

    #[test]
    fn test_app_builder_with_config() {
        let builder = RiAppBuilder::new()
            .with_config("config.yaml")
            .unwrap();
        assert_eq!(builder.config_paths.len(), 1);
        assert_eq!(builder.config_paths[0], "config.yaml");
    }

    #[test]
    fn test_app_builder_method_chaining() {
        let builder = RiAppBuilder::new()
            .with_config("config.yaml")
            .unwrap()
            .with_logging(crate::log::RiLogConfig::default());
        assert_eq!(builder.config_paths.len(), 1);
        assert!(builder.logging_config.is_some());
    }

    #[test]
    fn test_app_builder_with_observability() {
        let builder = RiAppBuilder::new()
            .with_observability(crate::observability::RiObservabilityConfig::default());
        assert!(builder.observability_config.is_some());
    }

    #[test]
    fn test_app_builder_with_multiple_configs() {
        let builder = RiAppBuilder::new()
            .with_config("config1.yaml")
            .unwrap()
            .with_config("config2.yaml")
            .unwrap();
        assert_eq!(builder.config_paths.len(), 2);
    }
}
