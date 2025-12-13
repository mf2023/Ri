//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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
//! This module provides the application builder for constructing DMS applications.
//! The `DMSAppBuilder` follows the builder pattern for fluent configuration.

use crate::core::{DMSResult, DMSServiceContext, ServiceModule, AsyncServiceModule};
use super::module_sorter::sort_modules;
use super::module_types::{ModuleSlot, ModuleType};
use super::lifecycle::DMSLifecycleObserver;
use super::analytics::DMSLogAnalyticsModule;

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Public-facing application builder for DMS.
/// 
/// The `DMSAppBuilder` provides a fluent API for configuring and building DMS applications.
/// It follows the builder pattern, allowing users to configure various aspects of the application
/// before building the final runtime.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> DMSResult<()> {
///     let app = DMSAppBuilder::new()
///         .with_config("config.yaml")?
///         .with_module(Box::new(MySyncModule::new()))
///         .with_async_module(Box::new(MyAsyncModule::new()))
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSAppBuilder {
    /// Vector of modules with their state, including both sync and async modules
    modules: Vec<ModuleSlot>, 
    /// Configuration file paths to load
    config_paths: Vec<String>, 
    /// Custom logging configuration (optional)
    logging_config: Option<crate::log::DMSLogConfig>, 
    /// Custom observability configuration (optional)
    observability_config: Option<crate::observability::DMSObservabilityConfig>, 
}

impl DMSAppBuilder {
    /// Create a new empty application builder.
    /// 
    /// # Returns
    /// 
    /// A new `DMSAppBuilder` instance with default settings.
    pub fn new() -> Self {
        DMSAppBuilder {
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
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn with_module(mut self, module: Box<dyn ServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Sync(module), failed: false });
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
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn with_async_module(mut self, module: Box<dyn AsyncServiceModule>) -> Self {
        self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        self
    }

    /// Add a DMS module to the application.
    /// 
    /// This method adds a module implementing the public `DMSModule` trait to the application.
    /// The module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed module implementing `DMSModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn with_dms_module(mut self, module: Box<dyn crate::core::DMSModule>) -> Self {
        // Wrap DMSModule into AsyncServiceModule adapter
        struct DMSModuleAdapter(Box<dyn crate::core::DMSModule + Send + Sync + 'static>);
        
        #[async_trait::async_trait]
        impl AsyncServiceModule for DMSModuleAdapter {
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
            
            async fn init(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.init(ctx).await
            }
            
            async fn before_start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.before_start(ctx).await
            }
            
            async fn start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.start(ctx).await
            }
            
            async fn after_start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.after_start(ctx).await
            }
            
            async fn before_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.before_shutdown(ctx).await
            }
            
            async fn shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.shutdown(ctx).await
            }
            
            async fn after_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
                self.0.after_shutdown(ctx).await
            }
        }
        
        self.modules.push(ModuleSlot { 
            module: ModuleType::Async(Box::new(DMSModuleAdapter(module))), 
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
    /// The updated `DMSAppBuilder` instance for method chaining.
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
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn with_async_modules(mut self, modules: Vec<Box<dyn AsyncServiceModule>>) -> Self {
        for module in modules {
            self.modules.push(ModuleSlot { module: ModuleType::Async(module), failed: false });
        }
        self
    }
    
    /// Add multiple DMS modules to the application.
    /// 
    /// This method adds multiple modules implementing the public `DMSModule` trait to the application.
    /// Each module will be treated as an asynchronous module.
    /// 
    /// # Parameters
    /// 
    /// - `modules`: A vector of boxed modules implementing `DMSModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn with_dms_modules(mut self, modules: Vec<Box<dyn crate::core::DMSModule>>) -> Self {
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
    /// A `DMSResult` containing the updated `DMSAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_config(mut self, config_path: impl Into<String>) -> DMSResult<Self> {
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
    /// A `DMSResult` containing the updated `DMSAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_logging(mut self, logging_config: crate::log::DMSLogConfig) -> DMSResult<Self> {
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
    /// A `DMSResult` containing the updated `DMSAppBuilder` instance for method chaining.
    /// 
    /// # Errors
    /// 
    /// This method currently never returns an error, but returns `DMSResult` for consistency
    /// with other builder methods and to allow for future error handling.
    pub fn with_observability(mut self, observability_config: crate::observability::DMSObservabilityConfig) -> DMSResult<Self> {
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
    /// A `DMSResult` containing the built `DMSAppRuntime` instance, or an error if building fails.
    /// 
    /// # Errors
    /// 
    /// - If configuration loading fails
    /// - If service context creation fails
    /// - If module sorting fails due to circular dependencies
    pub fn build(mut self) -> DMSResult<super::app_runtime::DMSAppRuntime> {
        // Create config manager with specified config paths
        let mut config_manager = crate::config::DMSConfigManager::new();
        
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
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(DMSLogAnalyticsModule::new())), failed: false });
        self.modules.push(ModuleSlot { module: ModuleType::Sync(Box::new(DMSLifecycleObserver::new())), failed: false });
        
        // Sort modules based on dependencies and priority
        self.modules = sort_modules(self.modules)?;
        
        let runtime = super::app_runtime::DMSAppRuntime::new(ctx, self.modules);
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
    /// A `DMSResult` containing the created `DMSServiceContext` instance, or an error if creation fails.
    /// 
    /// # Errors
    /// 
    /// - If project root directory detection fails
    /// - If file system creation fails
    /// - If logger creation fails
    fn create_service_context(&self, config_manager: crate::config::DMSConfigManager) -> DMSResult<DMSServiceContext> {
        let cfg = config_manager.config();

        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {e}")))?;
        let app_data_root = if let Some(root_str) = cfg.get_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        let fs = crate::fs::DMSFileSystem::new_with_roots(project_root, app_data_root);

        // Use custom logging config if provided, otherwise create from config
        let log_config: crate::log::DMSLogConfig = if let Some(log_config) = &self.logging_config {
            log_config.clone()
        } else {
            crate::log::DMSLogConfig::from_config(cfg)
        };
        let logger = crate::log::DMSLogger::new(&log_config, fs.clone());
        let hooks = crate::hooks::DMSHookBus::new();
        
        Ok(DMSServiceContext::new_with(fs, logger, config_manager, hooks, None))
    }
}

#[cfg(feature = "pyo3")]
    /// Python bindings for DMSAppBuilder
    #[pyo3::prelude::pymethods]
    impl DMSAppBuilder {
        #[new]
        fn py_new() -> Self {
            Self::new()
        }
        
        /// Add a configuration file from Python (creates a new builder)
        fn with_config_py(&self, config_path: String) -> PyResult<Self> {
            // For Python, we'll create a new builder and manually copy the state
            // Since we can't clone ModuleSlot, we'll create a fresh builder with just the config
            let mut new_builder = Self::new();
            new_builder.config_paths.push(config_path);
            // Copy other configurations if needed
            new_builder.logging_config = self.logging_config.clone();
            new_builder.observability_config = self.observability_config.clone();
            Ok(new_builder)
        }
        
        /// Build the application runtime from Python
        fn build_py(&self) -> PyResult<super::app_runtime::DMSAppRuntime> {
            // Create a new builder and copy the configuration
            let mut builder = Self::new();
            builder.config_paths = self.config_paths.clone();
            builder.logging_config = self.logging_config.clone();
            builder.observability_config = self.observability_config.clone();
            
            match builder.build() {
                Ok(runtime) => Ok(runtime),
                Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to build runtime: {}", e))),
            }
        }
    }
