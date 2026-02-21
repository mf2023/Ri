//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

//! # Application Runtime
//! 
//! This module provides the application runtime for DMSC applications.
//! The `DMSCAppRuntime` manages the application lifecycle, including module initialization,
//! startup, and shutdown. It also handles the execution of both synchronous and asynchronous modules.

use crate::core::{DMSCResult, DMSCServiceContext};
use crate::hooks::{DMSCHookKind, DMSCModulePhase};
use super::module_types::{ModuleSlot, ModuleType};
use tokio::sync::RwLock as AsyncRwLock;
use std::sync::Arc;
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Public-facing application runtime.
/// 
/// The `DMSCAppRuntime` manages the application lifecycle, including module initialization,
/// startup, and shutdown. It also handles the execution of both synchronous and asynchronous modules.
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
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMSC service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSCAppRuntime {
    /// Service context providing access to core functionalities
    ctx: DMSCServiceContext,
    /// Vector of modules with their state, protected by an async RwLock
    modules: Arc<AsyncRwLock<Vec<ModuleSlot>>>,
}

impl DMSCAppRuntime {
    /// Create a new application runtime with the given context and modules.
    /// 
    /// This method is typically called by the `DMSCAppBuilder` during the build process.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: Service context with core functionalities
    /// - `modules`: Vector of modules with their initial state
    /// 
    /// # Returns
    /// 
    /// A new `DMSCAppRuntime` instance.
    pub fn new(ctx: DMSCServiceContext, modules: Vec<ModuleSlot>) -> Self {
        Self {
            ctx,
            modules: Arc::new(AsyncRwLock::new(modules)),
        }
    }
    
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
    /// - `f`: A closure that takes a `DMSCServiceContext` and returns a `DMSCResult<()>`. 
    ///   This closure contains the application's business logic and is executed after all
    ///   modules have been initialized and started, but before any modules are shut down.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure.
    /// 
    /// # Errors
    /// 
    /// Returns an error if:
    /// - A critical module fails during execution
    /// - The provided closure returns an error
    pub async fn run<F, Fut>(mut self, f: F) -> DMSCResult<()>
    where
        F: FnOnce(&DMSCServiceContext) -> Fut,
        Fut: std::future::Future<Output = DMSCResult<()>>,
    {
        // Emit startup hook
        self.ctx.hooks().emit_with(&DMSCHookKind::Startup, &self.ctx, None, None)?;

        // Emit before modules init hook
        self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesInit, &self.ctx, None, None)?;
        
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
                self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesInit, &self.ctx, Some(&module_name), Some(DMSCModulePhase::Init))?;
                
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
        self.ctx.hooks().emit_with(&DMSCHookKind::AfterModulesInit, &self.ctx, None, None)?;

        // Emit before modules start hook
        self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, None, None)?;
        
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
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::BeforeStart))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::Start))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AfterStart))?;
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
        self.ctx.hooks().emit_with(&DMSCHookKind::AfterModulesStart, &self.ctx, None, None)?;

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
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncInit))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncBeforeStart))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncStart))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncAfterStart))?;
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
        self.ctx.hooks().emit_with(&DMSCHookKind::AfterModulesStart, &self.ctx, None, None)?;
        
        // Run the application business logic (provided closure)
        let result = f(&self.ctx).await;
        
        // Emit before modules shutdown hook
        // Note: We're using a new context here since we've moved the original to the closure
        let _ = self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, None, None);
        
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
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::BeforeShutdown))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::Shutdown))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AfterShutdown))?;
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
        self.ctx.hooks().emit_with(&DMSCHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Shutdown asynchronous modules in reverse order with optimized locking
        self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, None, None)?;
        
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
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncBeforeShutdown))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncShutdown))?;
                    self.ctx.hooks().emit_with(&DMSCHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSCModulePhase::AsyncAfterShutdown))?;
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
        self.ctx.hooks().emit_with(&DMSCHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Emit shutdown hook
        self.ctx.hooks().emit_with(&DMSCHookKind::Shutdown, &self.ctx, None, None)?;

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
    fn log_module_error(&self, phase: &str, module_name: &str, err: &crate::core::DMSCError) {
        let logger = self.ctx.logger();
        let message = format!("module={module_name} phase={phase} error={err}");
        let _ = logger.error("DMSC.Runtime", message);
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCAppRuntime {
    fn py_run(&self, callback: Py<pyo3::PyAny>) -> PyResult<()> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err::<pyo3::PyErr, _>(|e| e.into())?;
        
        let runtime = self.clone();
        pyo3::Python::attach(|py| {
            rt.block_on(async move {
                let _ = callback.call0(py);
                runtime.run(|_ctx| async move { Ok(()) }).await
            }).map_err(|e| e.into())
        })
    }

    fn get_context(&self) -> PyResult<DMSCServiceContext> {
        Ok(self.ctx.clone())
    }

    #[pyo3(name = "logger")]
    fn logger_py(&self) -> crate::log::DMSCLogger {
        self.ctx.logger().clone()
    }
}
