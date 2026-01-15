//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
//! This module provides the application builder for constructing DMSC applications.
//! The `DMSCAppBuilder` follows the builder pattern for fluent configuration,
//! enabling developers to compose applications from various modules, configuration
//! sources, and runtime settings in a declarative manner.
//!
//! ## Builder Pattern
//!
//! The builder pattern allows step-by-step construction of complex objects.
//! Each method on `DMSCAppBuilder` configures a specific aspect of the application
//! and returns the builder for method chaining. This results in a fluent API
//! that is both readable and type-safe.
//!
//! ## Module Registration
//!
//! Modules are the primary extension point for DMSC applications. The builder
//! supports multiple types of modules:
//!
//! - **Synchronous modules**: Implement `ServiceModule` trait for sync operations
//! - **Asynchronous modules**: Implement `AsyncServiceModule` trait for async operations
//! - **DMSC modules**: Implement public `DMSCModule` trait (converted to async internally)
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
//! use dmsc::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> DMSCResult<()> {
//!     let app = DMSCAppBuilder::new()
//!         .with_config("config.yaml")?
//!         .with_module(Box::new(MySyncModule::new()))
//!         .with_async_module(Box::new(MyAsyncModule::new()))
//!         .build()?;
//!
//!     app.run(|ctx| async move {
//!         ctx.logger().info("service", "DMSC service started")?;
//!         Ok(())
//!     }).await
//! }
//! ```
//!
//! ## Thread Safety
//!
//! The `DMSCAppBuilder` is designed to be used in a single-threaded context
//! during application construction. After calling `build()`, the resulting
//! `DMSCAppRuntime` is safe to use across multiple threads.
//!
//! ## Error Handling
//!
//! All builder methods that can fail return `DMSCResult`, enabling proper
//! error handling through the `?` operator or explicit match statements.

use crate::core::{DMSCResult, DMSCServiceContext, ServiceModule, AsyncServiceModule};
use super::module_sorter::sort_modules;
use super::module_types::{ModuleSlot, ModuleType};
use super::lifecycle::DMSCLifecycleObserver;
use super::analytics::DMSCLogAnalyticsModule;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;
use crate::core::app_runtime::DMSCAppRuntime;

/// Public-facing application builder for DMSC.
/// 
/// The `DMSCAppBuilder` provides a fluent API for configuring and building DMSC applications.
/// It follows the builder pattern, allowing users to configure various aspects of the application
/// before building the final runtime.
/// 
/// ## Usage
/// 
/// ```rust
/// use dmsc::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> DMSCResult<()> {
///     let app = DMSCAppBuilder::new()
///         .with_config("config.yaml")?
///         .with_module(Box::new(MySyncModule::new()))
///         .with_async_module(Box::new(MyAsyncModule::new()))
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMSC service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCAppBuilder {
    /// Vector of modules with their state, including both sync and async modules
    modules: Vec<ModuleSlot>, 
    /// Configuration file paths to load
    config_paths: Vec<String>, 
    /// Custom logging configuration (optional)
    logging_config: Option<crate::log::DMSCLogConfig>, 
    /// Custom observability configuration (optional)
    observability_config: Option<crate::observability::DMSCObservabilityConfig>, 
}
 
impl Default for DMSCAppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
 
impl DMSCAppBuilder {
    /// Create a new empty application builder.
    /// 
    /// # Returns
    /// 
    /// A new `DMSCAppBuilder` instance with default settings.
    pub fn new() -> Self {
        DMSCAppBuilder {
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
    /// The updated `DMSCAppBuilder` instance for method chaining.
    pub fn with_module(mut self, module: Box<dyn ServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Sync(module), failed: false });
        self
    }
    
    /// Add a Python module to the application.
    /// 
    /// This method adds a module created from Python code to the application.
    /// The module will be treated as an asynchronous DMSC module.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A Python module adapter implementing module configuration
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCAppBuilder` instance for method chaining.
    #[cfg(feature = "pyo3")]
    pub fn with_python_module(mut self, module: crate::core::module::DMSCPythonModuleAdapter) -> Self {
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
    /// The updated `DMSCAppBuilder` instance for method chaining.
    pub fn with_async_module(mut self, module: Box<dyn AsyncServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        self
    }
 
    /// Add a DMSC module to the application.
    /// 
    /// This method adds a module implementing the public `DMSCModule` trait to the application.
    /// The module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed module implementing `DMSCModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCAppBuilder` instance for method chaining.
    pub fn with_dms_module(mut self, module: Box<dyn crate::core::DMSCModule>) -> Self {
        // Wrap DMSCModule into AsyncServiceModule adapter
        struct DMSCModuleAdapter(Box<dyn crate::core::DMSCModule + Send + Sync + 'static>);
        
        #[async_trait::async_trait]
        impl AsyncServiceModule for DMSCModuleAdapter {
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
            
            async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.init(ctx).await
            }
            
            async fn before_start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.before_start(ctx).await
            }
            
            async fn start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.start(ctx).await
            }
            
            async fn after_start(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.after_start(ctx).await
            }
            
            async fn before_shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.before_shutdown(ctx).await
            }
            
            async fn shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.shutdown(ctx).await
            }
            
            async fn after_shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
                self.0.after_shutdown(ctx).await
            }
        }
        
        self.modules.push(ModuleSlot { 
            module: ModuleType::Async(Box::new(DMSCModuleAdapter(module))), 
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
    /// The updated `DMSCAppBuilder` instance for method chaining.
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
    /// The updated `DMSCAppBuilder` instance for method chaining.
    pub fn with_async_modules(mut self, modules: Vec<Box<dyn AsyncServiceModule>>) -> Self {
        for module in modules {
            self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        }
        self
    }
    
    /// Add multiple DMSC modules to the application.
    /// 
    /// This method adds multiple modules implementing the public `DMSCModule` trait to the application.
    /// Each module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `modules`: A vector of boxed modules implementing `DMSCModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCAppBuilder` instance for method chaining.
    pub fn with_dms_modules(mut self, modules: Vec<Box<dyn crate::core::DMSCModule>>) -> Self {
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
    /// A `DMSCResult` containing the updated `DMSCAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSCResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_config(mut self, config_path: impl Into<String>) -> DMSCResult<Self> {
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
    /// A `DMSCResult` containing the updated `DMSCAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSCResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_logging(mut self, logging_config: crate::log::DMSCLogConfig) -> DMSCResult<Self> {
        self.logging_config = Some(logging_config);
        Ok(self)
    }
 
    /// Set custom observability configuration for the application.
    /// 
    /// # Parameters
    /// 
    /// - `observability_config`: Custom observability configuration
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` containing the updated `DMSCAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSCResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_observability(mut self, observability_config: crate::observability::DMSCObservabilityConfig) -> DMSCResult<Self> {
        self.observability_config = Some(observability_config);
        Ok(self)
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
    /// A `DMSCResult` containing the built `DMSCAppRuntime` instance, or an error if building fails.
    /// 
    /// # Errors
    /// 
    /// - If configuration loading fails
    /// - If service context creation fails
    /// - If module sorting fails due to circular dependencies
    pub fn build(mut self) -> DMSCResult<super::app_runtime::DMSCAppRuntime> {
        // Create config manager with specified config paths
        let mut config_manager = crate::config::DMSCConfigManager::new();
        
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
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(DMSCLogAnalyticsModule::new())), failed: false });
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(DMSCLifecycleObserver::new())), failed: false });
        
        // Sort modules based on dependencies and priority
        self.modules = sort_modules(self.modules)?;
        
        let runtime = super::app_runtime::DMSCAppRuntime::new(ctx, self.modules);
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
    /// A `DMSCResult` containing the created `DMSCServiceContext` instance, or an error if creation fails.
    /// 
    /// # Errors
    /// 
    /// - If project root directory detection fails
    /// - If file system creation fails
    /// - If logger creation fails
    fn create_service_context(&self, config_manager: crate::config::DMSCConfigManager) -> DMSCResult<DMSCServiceContext> {
        let cfg = config_manager.config();
 
        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSCError::Other(format!("detect project root failed: {e}")))?;
        let app_data_root = if let Some(root_str) = cfg.get_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };
 
        let fs = crate::fs::DMSCFileSystem::new_with_roots(project_root, app_data_root);
 
        // Use custom logging config if provided, otherwise create from config
        let log_config: crate::log::DMSCLogConfig = if let Some(log_config) = &self.logging_config {
            log_config.clone()
        } else {
            crate::log::DMSCLogConfig::from_config(&cfg)
        };
        let logger = crate::log::DMSCLogger::new(&log_config, fs.clone());
        let hooks = crate::hooks::DMSCHookBus::new();
        
        Ok(DMSCServiceContext::new_with(fs, logger, config_manager, hooks, None))
    }
}
 
#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCAppBuilder {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[pyo3(name = "with_config")]
    fn with_config_py(&mut self, config_path: &str) -> PyResult<()> {
        self.config_paths.push(config_path.to_string());
        Ok(())
    }

    #[pyo3(name = "with_logging")]
    fn with_logging_py(&mut self, logging_config: crate::log::DMSCLogConfig) -> PyResult<()> {
        self.logging_config = Some(logging_config);
        Ok(())
    }

    #[pyo3(name = "with_observability")]
    fn with_observability_py(&mut self, observability_config: crate::observability::DMSCObservabilityConfig) -> PyResult<()> {
        self.observability_config = Some(observability_config);
        Ok(())
    }

    #[pyo3(name = "build")]
    fn build_py(&mut self) -> PyResult<DMSCAppRuntime> {
        let builder = std::mem::take(self);
        DMSCAppBuilder::build(builder).map_err(|e| pyo3::prelude::PyErr::from(e))
    }

    #[pyo3(name = "with_module")]
    fn with_module_py(&mut self, module: super::module::DMSCPythonServiceModule) -> PyResult<()> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Sync(Box::new(module)),
            failed: false,
        });
        Ok(())
    }

    #[pyo3(name = "with_python_module")]
    fn with_python_module_py(&mut self, module: super::module::DMSCPythonModuleAdapter) -> PyResult<()> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(())
    }

    #[pyo3(name = "with_async_module")]
    fn with_async_module_py(&mut self, module: super::module::DMSCPythonAsyncServiceModule) -> PyResult<()> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(())
    }

    #[pyo3(name = "with_dms_module")]
    fn with_dms_module_py(&mut self, module: super::module::DMSCPythonModuleAdapter) -> PyResult<()> {
        self.modules.push(crate::core::module_types::ModuleSlot {
            module: crate::core::module_types::ModuleType::Async(Box::new(module)),
            failed: false,
        });
        Ok(())
    }

    #[pyo3(name = "with_modules")]
    fn with_modules_py(&mut self, modules: Vec<super::module::DMSCPythonServiceModule>) -> PyResult<()> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Sync(Box::new(module)),
                failed: false,
            });
        }
        Ok(())
    }

    #[pyo3(name = "with_async_modules")]
    fn with_async_modules_py(&mut self, modules: Vec<super::module::DMSCPythonAsyncServiceModule>) -> PyResult<()> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Async(Box::new(module)),
                failed: false,
            });
        }
        Ok(())
    }

    #[pyo3(name = "with_dms_modules")]
    fn with_dms_modules_py(&mut self, modules: Vec<super::module::DMSCPythonModuleAdapter>) -> PyResult<()> {
        for module in modules {
            self.modules.push(crate::core::module_types::ModuleSlot {
                module: crate::core::module_types::ModuleType::Async(Box::new(module)),
                failed: false,
            });
        }
        Ok(())
    }
}
