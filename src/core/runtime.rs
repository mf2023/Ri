//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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

//! # Application Runtime and Builder
//! 
//! This module provides the application runtime and builder for constructing DMS applications.
//! The `DMSAppBuilder` follows the builder pattern for fluent configuration, while the `DMSAppRuntime`
//! manages the application lifecycle and module execution.
//! 
//! ## Key Components
//! 
//! - **DMSAppBuilder**: Fluent API for configuring and building DMS applications
//! - **DMSAppRuntime**: Manages the application lifecycle and module execution
//! - **ModuleType**: Internal enum for distinguishing between sync and async modules
//! - **ModuleSlot**: Internal struct for tracking module state
//! 
//! ## Design Principles
//! 
//! 1. **Builder Pattern**: The `DMSAppBuilder` provides a fluent API for configuring applications
//! 2. **Module Lifecycle**: Modules go through a well-defined lifecycle with init, start, and shutdown phases
//! 3. **Dependency Resolution**: Modules are sorted based on dependencies and priority
//! 4. **Async Support**: Full support for both synchronous and asynchronous modules
//! 5. **Fault Tolerance**: Non-critical modules can fail without crashing the entire application

use crate::core::{DMSResult, DMSServiceContext};
use crate::hooks::DMSModulePhase;
use crate::core::{ServiceModule, AsyncServiceModule};
use super::lifecycle::DMSLifecycleObserver;
use super::analytics::DMSLogAnalyticsModule;
use crate::hooks::DMSHookKind;
use tokio::sync::RwLock as AsyncRwLock;
use std::sync::Arc;
use std::collections::HashMap;

/// Internal enum for distinguishing between synchronous and asynchronous modules.
/// 
/// This enum allows the runtime to handle both sync and async modules in a unified way,
/// while still respecting their different execution requirements.
enum ModuleType {
    /// Synchronous module that implements `ServiceModule`
    Sync(Box<dyn ServiceModule>),
    /// Asynchronous module that implements `AsyncServiceModule`
    Async(Box<dyn AsyncServiceModule>),
}

impl ModuleType {
    /// Get the name of the module.
    /// 
    /// # Returns
    /// 
    /// The name of the module as a string slice.
    fn name(&self) -> &str {
        match self {
            ModuleType::Sync(module) => module.name(),
            ModuleType::Async(module) => module.name(),
        }
    }

    /// Check if the module is critical.
    /// 
    /// Critical modules will cause the application to fail if they fail during initialization or startup.
    /// 
    /// # Returns
    /// 
    /// `true` if the module is critical, `false` otherwise.
    fn is_critical(&self) -> bool {
        match self {
            ModuleType::Sync(module) => module.is_critical(),
            ModuleType::Async(module) => module.is_critical(),
        }
    }

    /// Get the priority of the module.
    /// 
    /// Modules with higher priority are initialized and started first.
    /// 
    /// # Returns
    /// 
    /// The priority of the module as an integer.
    fn priority(&self) -> i32 {
        match self {
            ModuleType::Sync(module) => module.priority(),
            ModuleType::Async(module) => module.priority(),
        }
    }

    /// Get the dependencies of the module.
    /// 
    /// Dependencies are module names that must be initialized before this module.
    /// 
    /// # Returns
    /// 
    /// A vector of dependency module names.
    fn dependencies(&self) -> Vec<&str> {
        match self {
            ModuleType::Sync(module) => module.dependencies(),
            ModuleType::Async(module) => module.dependencies(),
        }
    }
}

/// Internal struct for tracking module state.
/// 
/// This struct wraps a module and tracks whether it has failed during execution.
struct ModuleSlot {
    /// The module itself, either sync or async
    module: ModuleType,
    /// Whether the module has failed during execution
    failed: bool,
}

/// Public-facing application runtime.
/// 
/// The `DMSAppRuntime` manages the application lifecycle, including module initialization,
/// startup, and shutdown. It also handles the execution of both synchronous and asynchronous modules.
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
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSAppRuntime {
    /// Service context providing access to core functionalities
    ctx: DMSServiceContext,
    /// Vector of modules with their state, protected by an async RwLock
    modules: Arc<AsyncRwLock<Vec<ModuleSlot>>>,
}

impl DMSAppRuntime {
    /// Run the application lifecycle.
    /// 
    /// This method executes the complete application lifecycle, including:
    /// 1. Emitting startup hooks
    /// 2. Initializing synchronous modules
    /// 3. Starting synchronous modules
    /// 4. Initializing and starting asynchronous modules
    /// 5. Running the application business logic via the provided closure
    /// 6. Shutting down asynchronous modules
    /// 7. Shutting down synchronous modules
    /// 8. Emitting shutdown hooks
    /// 
    /// # Parameters
    /// 
    /// - `f`: A closure that takes a `DMSServiceContext` and returns a `DMSResult<()>`. 
    ///   This closure contains the application's business logic and is executed after all
    ///   modules have been initialized and started, but before any modules are shut down.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` indicating success or failure.
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - A critical module fails during execution
    /// - The provided closure returns an error
    pub async fn run<F, Fut>(mut self, f: F) -> DMSResult<()> 
    where
        F: FnOnce(&DMSServiceContext) -> Fut,
        Fut: std::future::Future<Output = DMSResult<()>>,
    {
        // Emit startup hook
        self.ctx.hooks().emit_with(&DMSHookKind::Startup, &self.ctx, None, None)?;

        // Emit before modules init hook
        self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, None, None)?;
        
        // Get module count
        let modules_guard = self.modules.read().await;
        let module_len = modules_guard.len();
        drop(modules_guard); // Release lock early
        
        // Collect module states first to avoid repeated lock acquisitions
        let mut module_states = Vec::new();
        {
            let modules_guard = self.modules.read().await;
            for idx in 0..module_len {
                if idx < modules_guard.len() {
                    let slot = &modules_guard[idx];
                    module_states.push((
                        idx,
                        !slot.failed,
                        if !slot.failed {
                            slot.module.name().to_string()
                        } else {
                            String::new()
                        },
                        if !slot.failed {
                            slot.module.is_critical()
                        } else {
                            false
                        },
                    ));
                } else {
                    module_states.push((idx, false, String::new(), false));
                }
            }
        }

        // Initialize synchronous modules
        for (idx, skip, module_name, critical) in module_states.iter().cloned() {
            if !skip {
                // Emit before module init hook
                self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, Some(&module_name), Some(DMSModulePhase::Init))?;
                
                // Initialize module with single write lock acquisition
                let mut error = None;
                {
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            ModuleType::Sync(_module) => {
                                if let Err(err) = _module.init(&mut self.ctx) {
                                    error = Some(err);
                                }
                            }
                            ModuleType::Async(_module) => {
                                // Async modules are handled separately in the async phase
                            }
                        }
                    }
                }
                
                // Handle module initialization error
                if let Some(err) = error {
                    self.log_module_error("init", &module_name, &err);
                    if critical {
                        return Err(err);
                    } else {
                        // Mark module as failed with single write lock acquisition
                        let mut modules_guard = self.modules.write().await;
                        if idx < modules_guard.len() {
                            modules_guard[idx].failed = true;
                        }
                    }
                }
            }
        }
        
        // Emit after modules init hook
        self.ctx.hooks().emit_with(&DMSHookKind::AfterModulesInit, &self.ctx, None, None)?;

        // Emit before modules start hook
        self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, None, None)?;
        
        // Start synchronous modules with optimized locking
        for (idx, skip, module_name, critical) in module_states.iter().cloned() {
            if !skip {
                let mut err_phase = "start";
                
                // Execute all sync module phases with single write lock acquisition
                let mut error = None;
                {
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            ModuleType::Sync(_module) => {
                                // Execute before_start phase
                                if let Err(err) = _module.before_start(&mut self.ctx) {
                                    err_phase = "before_start";
                                    error = Some(err);
                                }
                                
                                // Execute start phase if no error
                                if error.is_none() {
                                    if let Err(err) = _module.start(&mut self.ctx) {
                                        err_phase = "start";
                                        error = Some(err);
                                    }
                                }
                                
                                // Execute after_start phase if no error
                                if error.is_none() {
                                    if let Err(err) = _module.after_start(&mut self.ctx) {
                                        err_phase = "after_start";
                                        error = Some(err);
                                    }
                                }
                            }
                            ModuleType::Async(_module) => {
                                // Async modules are handled separately in the async phase
                            }
                        }
                    }
                }
                
                // Emit hooks outside of lock to avoid potential deadlocks
                if error.is_none() {
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::BeforeStart))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::Start))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AfterStart))?;
                }
                
                // Handle module start error
                if let Some(err) = error {
                    self.log_module_error(err_phase, &module_name, &err);
                    if critical {
                        return Err(err);
                    } else {
                        // Mark module as failed with single write lock acquisition
                        let mut modules_guard = self.modules.write().await;
                        if idx < modules_guard.len() {
                            modules_guard[idx].failed = true;
                        }
                    }
                }
            }
        }
        
        // Emit after modules start hook
        self.ctx.hooks().emit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, None)?;

        // Initialize and start asynchronous modules with optimized locking
        for idx in 0..module_len {
            let mut err_phase = "async_start";
            
            // Check if this is an async module and get its state
            let (skip, module_name, critical) = {
                let modules_guard = self.modules.read().await;
                if idx < modules_guard.len() {
                    let slot = &modules_guard[idx];
                    if !slot.failed {
                        match &slot.module {
                            ModuleType::Async(module) => (
                                false,
                                module.name().to_string(),
                                module.is_critical(),
                            ),
                            ModuleType::Sync(_) => (true, String::new(), false),
                        }
                    } else {
                        (true, String::new(), false)
                    }
                } else {
                    (true, String::new(), false)
                }
            };
            
            if !skip {
                // Execute all async module phases with single write lock acquisition
                let mut error = None;
                {
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let ModuleType::Async(_module) = &mut modules_guard[idx].module {
                            // Execute async init phase
                            if let Err(err) = _module.init(&mut self.ctx).await {
                                err_phase = "async_init";
                                error = Some(err);
                            }
                            
                            // Execute async before_start phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.before_start(&mut self.ctx).await {
                                    err_phase = "async_before_start";
                                    error = Some(err);
                                }
                            }
                            
                            // Execute async start phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.start(&mut self.ctx).await {
                                    err_phase = "async_start";
                                    error = Some(err);
                                }
                            }
                            
                            // Execute async after_start phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.after_start(&mut self.ctx).await {
                                    err_phase = "async_after_start";
                                    error = Some(err);
                                }
                            }
                        }
                    }
                }
                
                // Emit hooks outside of lock to avoid potential deadlocks
                if error.is_none() {
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncInit))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncBeforeStart))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncStart))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncAfterStart))?;
                }
                
                // Handle async module error
                if let Some(err) = error {
                    self.log_module_error(err_phase, &module_name, &err);
                    if critical {
                        return Err(err);
                    } else {
                        // Mark module as failed with single write lock acquisition
                        let mut modules_guard = self.modules.write().await;
                        if idx < modules_guard.len() {
                            modules_guard[idx].failed = true;
                        }
                    }
                }
            }
        }
        
        // Emit after async modules start hook
        self.ctx.hooks().emit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, None)?;
        
        // Run the application business logic (provided closure)
        let result = f(&self.ctx).await;
        
        // Emit before modules shutdown hook
        // Note: We're using a new context here since we've moved the original to the closure
        let _ = self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, None);
        
        // Shutdown synchronous modules in reverse order with optimized locking
        for idx in (0..module_len).rev() {
            // Check if this is a sync module and get its state
            let (skip, module_name, critical) = {
                let modules_guard = self.modules.read().await;
                if idx < modules_guard.len() {
                    let slot = &modules_guard[idx];
                    if !slot.failed {
                        match &slot.module {
                            ModuleType::Sync(module) => (
                                false,
                                module.name().to_string(),
                                module.is_critical(),
                            ),
                            ModuleType::Async(_) => (true, String::new(), false),
                        }
                    } else {
                        (true, String::new(), false)
                    }
                } else {
                    (true, String::new(), false)
                }
            };
            
            if !skip {
                // Execute all sync module shutdown phases with single write lock acquisition
                let mut err_phase = "shutdown";
                let mut error = None;
                {
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let ModuleType::Sync(_module) = &mut modules_guard[idx].module {
                            // Execute before_shutdown phase
                            if let Err(err) = _module.before_shutdown(&mut self.ctx) {
                                err_phase = "before_shutdown";
                                error = Some(err);
                            }
                            
                            // Execute shutdown phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.shutdown(&mut self.ctx) {
                                    err_phase = "shutdown";
                                    error = Some(err);
                                }
                            }
                            
                            // Execute after_shutdown phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.after_shutdown(&mut self.ctx) {
                                    err_phase = "after_shutdown";
                                    error = Some(err);
                                }
                            }
                        }
                    }
                }
                
                // Emit hooks outside of lock to avoid potential deadlocks
                if error.is_none() {
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::BeforeShutdown))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::Shutdown))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AfterShutdown))?;
                }
                
                // Handle module shutdown error
                if let Some(err) = error {
                    self.log_module_error(err_phase, &module_name, &err);
                    if critical {
                        return Err(err);
                    } else {
                        // Mark module as failed
                        let mut modules_guard = self.modules.write().await;
                        if idx < modules_guard.len() {
                            modules_guard[idx].failed = true;
                        }
                    }
                }
            }
        }
        
        // Emit after modules shutdown hook
        self.ctx.hooks().emit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Shutdown asynchronous modules in reverse order with optimized locking
        self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, None)?;
        
        for idx in (0..module_len).rev() {
            // Check if this is an async module and get its state
            let (skip, module_name, critical) = {
                let modules_guard = self.modules.read().await;
                if idx < modules_guard.len() {
                    let slot = &modules_guard[idx];
                    if !slot.failed {
                        match &slot.module {
                            ModuleType::Async(module) => (
                                false,
                                module.name().to_string(),
                                module.is_critical(),
                            ),
                            ModuleType::Sync(_) => (true, String::new(), false),
                        }
                    } else {
                        (true, String::new(), false)
                    }
                } else {
                    (true, String::new(), false)
                }
            };
            
            if !skip {
                // Execute all async module shutdown phases with single write lock acquisition
                let mut err_phase = "async_shutdown";
                let mut error = None;
                {
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let ModuleType::Async(_module) = &mut modules_guard[idx].module {
                            // Execute async before_shutdown phase
                            if let Err(err) = _module.before_shutdown(&mut self.ctx).await {
                                err_phase = "async_before_shutdown";
                                error = Some(err);
                            }
                            
                            // Execute async shutdown phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.shutdown(&mut self.ctx).await {
                                    err_phase = "async_shutdown";
                                    error = Some(err);
                                }
                            }
                            
                            // Execute async after_shutdown phase if no error
                            if error.is_none() {
                                if let Err(err) = _module.after_shutdown(&mut self.ctx).await {
                                    err_phase = "async_after_shutdown";
                                    error = Some(err);
                                }
                            }
                        }
                    }
                }
                
                // Emit hooks outside of lock to avoid potential deadlocks
                if error.is_none() {
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncBeforeShutdown))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncShutdown))?;
                    self.ctx.hooks().emit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncAfterShutdown))?;
                }
                
                // Handle async module shutdown error
                if let Some(err) = error {
                    self.log_module_error(err_phase, &module_name, &err);
                    if critical {
                        return Err(err);
                    } else {
                        // Mark module as failed with single write lock acquisition
                        let mut modules_guard = self.modules.write().await;
                        if idx < modules_guard.len() {
                            modules_guard[idx].failed = true;
                        }
                    }
                }
            }
        }
        
        // Emit after async modules shutdown hook
        self.ctx.hooks().emit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Emit shutdown hook
        self.ctx.hooks().emit_with(&DMSHookKind::Shutdown, &self.ctx, None, None)?;

        // Return the result of the closure execution
        result
    }

    /// Log a module error.
    /// 
    /// This method logs an error that occurred during module execution, including
    /// the module name, phase, and error message.
    /// 
    /// # Parameters
    /// 
    /// - `phase`: The lifecycle phase during which the error occurred
    /// - `module_name`: The name of the module that failed
    /// - `err`: The error that occurred
    fn log_module_error(&self, phase: &str, module_name: &str, err: &crate::core::DMSError) {
        let logger = self.ctx.logger();
        let message = format!("module={module_name} phase={phase} error={err}");
        let _ = logger.error("DMS.Runtime", message);
    }
}

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
    pub fn build(mut self) -> DMSResult<DMSAppRuntime> {
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
        
        let runtime = DMSAppRuntime {
            ctx,
            modules: Arc::new(AsyncRwLock::new(self.modules)),
        };
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
        
        Ok(DMSServiceContext::new_with(fs, logger, config_manager, hooks))
    }
}

// Python bindings for DMSAppBuilder are handled in src/lib.rs

/// Sort modules based on dependencies and priority
/// Uses topological sort to handle dependencies, and sorts by priority within the same dependency level
fn sort_modules(mut modules: Vec<ModuleSlot>) -> DMSResult<Vec<ModuleSlot>> {
    let mut result: Vec<ModuleSlot> = Vec::with_capacity(modules.len());
    
    // Create a priority queue that holds (priority, module_index)
    // Higher priority comes first
    use std::collections::BinaryHeap;
    let mut queue: BinaryHeap<(i32, usize)> = BinaryHeap::new();
    
    // Loop until all modules are processed
    while !modules.is_empty() {
        // Create a map from module name to current index
        let name_to_index: HashMap<&str, usize> = modules
            .iter()
            .enumerate()
            .map(|(i, slot)| (slot.module.name(), i))
            .collect();
        
        // Calculate in-degree for each module
        let mut in_degree: Vec<usize> = vec![0; modules.len()];
        
        for (i, slot) in modules.iter().enumerate() {
            let dependencies = slot.module.dependencies();
            for dep_name in dependencies {
                // Check if dependency exists in remaining modules
                if let Some(_dep_index) = name_to_index.get(dep_name) {
                    // Dependency exists, so this module has in-degree
                    in_degree[i] += 1;
                } else {
                    // Dependency not found, check if it's already in result
                    let dep_in_result = result.iter().any(|slot| slot.module.name() == dep_name);
                    if !dep_in_result {
                        return Err(crate::core::DMSError::MissingDependency { 
                            module_name: slot.module.name().to_string(), 
                            dependency: dep_name.to_string() 
                        });
                    }
                }
            }
        }
        
        // Add all modules with in-degree 0 to the queue
        for (i, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                let priority = modules[i].module.priority();
                queue.push((priority, i));
            }
        }
        
        // If no modules with in-degree 0, we have a circular dependency
        if queue.is_empty() {
            return Err(crate::core::DMSError::CircularDependency { 
                modules: modules.iter().map(|slot| slot.module.name().to_string()).collect() 
            });
        }
        
        // Process modules from the queue
        while let Some((_, i)) = queue.pop() {
            // Check if index is still valid (modules may have been removed)
            if i < modules.len() {
                // Remove module from modules and add to result
                let module = modules.swap_remove(i);
                result.push(module);
            }
        }
    }
    
    Ok(result)
}



