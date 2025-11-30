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
//! - **_CModuleType**: Internal enum for distinguishing between sync and async modules
//! - **_CModuleSlot**: Internal struct for tracking module state
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
use crate::core::{_CServiceModule, _CAsyncServiceModule};
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
enum _CModuleType {
    /// Synchronous module that implements `_CServiceModule`
    Sync(Box<dyn _CServiceModule>),
    /// Asynchronous module that implements `_CAsyncServiceModule`
    Async(Box<dyn _CAsyncServiceModule>),
}

impl _CModuleType {
    /// Get the name of the module.
    /// 
    /// # Returns
    /// 
    /// The name of the module as a string slice.
    fn _Fname(&self) -> &str {
        match self {
            _CModuleType::Sync(module) => module._Fname(),
            _CModuleType::Async(module) => module._Fname(),
        }
    }

    /// Check if the module is critical.
    /// 
    /// Critical modules will cause the application to fail if they fail during initialization or startup.
    /// 
    /// # Returns
    /// 
    /// `true` if the module is critical, `false` otherwise.
    fn _Fis_critical(&self) -> bool {
        match self {
            _CModuleType::Sync(module) => module._Fis_critical(),
            _CModuleType::Async(module) => module._Fis_critical(),
        }
    }

    /// Get the priority of the module.
    /// 
    /// Modules with higher priority are initialized and started first.
    /// 
    /// # Returns
    /// 
    /// The priority of the module as an integer.
    fn _Fpriority(&self) -> i32 {
        match self {
            _CModuleType::Sync(module) => module._Fpriority(),
            _CModuleType::Async(module) => module._Fpriority(),
        }
    }

    /// Get the dependencies of the module.
    /// 
    /// Dependencies are module names that must be initialized before this module.
    /// 
    /// # Returns
    /// 
    /// A vector of dependency module names.
    fn _Fdependencies(&self) -> Vec<&str> {
        match self {
            _CModuleType::Sync(module) => module._Fdependencies(),
            _CModuleType::Async(module) => module._Fdependencies(),
        }
    }
}

/// Internal struct for tracking module state.
/// 
/// This struct wraps a module and tracks whether it has failed during execution.
struct _CModuleSlot {
    /// The module itself, either sync or async
    module: _CModuleType,
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
///     let app = DMSAppBuilder::_Fnew()
///         ._Fwith_config("config.yaml")?
///         ._Fbuild()?;
///     
///     app._Frun(|ctx| async move {
///         ctx._Flogger()._Finfo("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSAppRuntime {
    /// Service context providing access to core functionalities
    ctx: DMSServiceContext,
    /// Vector of modules with their state, protected by an async RwLock
    modules: Arc<AsyncRwLock<Vec<_CModuleSlot>>>,
}

impl DMSAppRuntime {
    /// Run the application lifecycle.
    /// 
    /// This method executes the complete application lifecycle, including:
    /// 1. Emitting startup hooks
    /// 2. Initializing synchronous modules
    /// 3. Starting synchronous modules
    /// 4. Initializing and starting asynchronous modules
    /// 5. Running the application business logic
    /// 6. Shutting down asynchronous modules
    /// 7. Shutting down synchronous modules
    /// 8. Emitting shutdown hooks
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` indicating success or failure.
    /// 
    /// # Errors
    /// 
    /// Returns an error if a critical module fails during execution.
    pub async fn _Frun(mut self) -> DMSResult<()> {
        // Emit startup hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::Startup, &self.ctx, None, None)?;

        // Emit before modules init hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, None, None)?;
        
        // Get module count
        let modules_guard = self.modules.read().await;
        let module_len = modules_guard.len();
        drop(modules_guard); // Release lock early
        
        // Initialize synchronous modules
        for idx in 0..module_len {
            let mut error: Option<crate::core::DMSError> = None;
            let critical;
            let module_name;
            let skip;
            
            // Check module state
            let modules_guard = self.modules.read().await;
            if idx < modules_guard.len() {
                let slot = &modules_guard[idx];
                skip = slot.failed;
                if !skip {
                    module_name = slot.module._Fname().to_string();
                    critical = slot.module._Fis_critical();
                } else {
                    module_name = String::new();
                    critical = false;
                }
            } else {
                skip = true;
                module_name = String::new();
                critical = false;
            }
            drop(modules_guard);
            
            if !skip {
                // Emit before module init hook
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, Some(&module_name), Some(DMSModulePhase::Init))?;
                
                // Initialize module
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    match &mut modules_guard[idx].module {
                        _CModuleType::Sync(_module) => {
                            if let Err(err) = _module._Finit(&mut self.ctx) {
                                error = Some(err);
                            }
                        }
                        _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                    }
                }
                drop(modules_guard);
            }
            
            // Handle module initialization error
            if let Some(err) = error {
                self._Flog_module_error("init", &module_name, &err);
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
        
        // Emit after modules init hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesInit, &self.ctx, None, None)?;

        // Emit before modules start hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, None, None)?;
        
        // Start synchronous modules
        for idx in 0..module_len {
            let mut error: Option<crate::core::DMSError> = None;
            let mut err_phase = "start";
            let critical;
            let module_name;
            let skip;
            
            // Check module state
            let modules_guard = self.modules.read().await;
            if idx < modules_guard.len() {
                let slot = &modules_guard[idx];
                skip = slot.failed;
                if !skip {
                    module_name = slot.module._Fname().to_string();
                    critical = slot.module._Fis_critical();
                } else {
                    module_name = String::new();
                    critical = false;
                }
            } else {
                skip = true;
                module_name = String::new();
                critical = false;
            }
            drop(modules_guard);
            
            if !skip {
                // Emit before module before_start hook
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::BeforeStart))?;
                
                // Execute before_start
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    match &mut modules_guard[idx].module {
                        _CModuleType::Sync(_module) => {
                            if let Err(err) = _module._Fbefore_start(&mut self.ctx) {
                                err_phase = "before_start";
                                error = Some(err);
                            }
                        }
                        _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    // Emit before module start hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::Start))?;
                    
                    // Execute start
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            _CModuleType::Sync(_module) => {
                                if let Err(err) = _module._Fstart(&mut self.ctx) {
                                    err_phase = "start";
                                    error = Some(err);
                                }
                            }
                            _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    // Emit before module after_start hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AfterStart))?;
                    
                    // Execute after_start
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            _CModuleType::Sync(_module) => {
                                if let Err(err) = _module._Fafter_start(&mut self.ctx) {
                                    err_phase = "after_start";
                                    error = Some(err);
                                }
                            }
                            _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
            }
            
            // Handle module start error
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
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
        
        // Emit after modules start hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, None)?;

        // Initialize and start asynchronous modules
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, None, None)?;
        
        for idx in 0..module_len {
            let mut error: Option<crate::core::DMSError> = None;
            let mut err_phase = "async_start";
            let critical;
            let module_name;
            let mut skip;
            
            // Check module state for async modules
            let modules_guard = self.modules.read().await;
            if idx < modules_guard.len() {
                let slot = &modules_guard[idx];
                skip = slot.failed;
                if !skip {
                    match &slot.module {
                        _CModuleType::Async(module) => {
                            module_name = module._Fname().to_string();
                            critical = module._Fis_critical();
                        }
                        _CModuleType::Sync(_) => {
                            skip = true;
                            module_name = String::new();
                            critical = false;
                        }
                    }
                } else {
                    module_name = String::new();
                    critical = false;
                }
            } else {
                skip = true;
                module_name = String::new();
                critical = false;
            }
            drop(modules_guard);
            
            if !skip {
                // Emit before async module init hook
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncInit))?;
                
                // Execute async init
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                        if let Err(err) = module._Finit(&mut self.ctx).await {
                            err_phase = "async_init";
                            error = Some(err);
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    // Emit before async module before_start hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncBeforeStart))?;
                    
                    // Execute async before_start
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                            if let Err(err) = module._Fbefore_start(&mut self.ctx).await {
                                err_phase = "async_before_start";
                                error = Some(err);
                            }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    // Emit before async module start hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncStart))?;
                    
                    // Execute async start
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                            if let Err(err) = module._Fstart(&mut self.ctx).await {
                                err_phase = "async_start";
                                error = Some(err);
                            }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    // Emit before async module after_start hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncAfterStart))?;
                    
                    // Execute async after_start
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                            if let Err(err) = module._Fafter_start(&mut self.ctx).await {
                                err_phase = "async_after_start";
                                error = Some(err);
                            }
                        }
                    }
                    drop(modules_guard);
                }
            }
            
            // Handle async module error
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
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
        
        // Emit after async modules start hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, None)?;

        // Emit before modules shutdown hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, None)?;
        
        // Shutdown synchronous modules in reverse order
        for idx in (0..module_len).rev() {
            let mut error: Option<crate::core::DMSError> = None;
            let mut err_phase = "shutdown";
            let critical;
            let module_name;
            let skip;
            
            // Check module state
            let modules_guard = self.modules.read().await;
            if idx < modules_guard.len() {
                let slot = &modules_guard[idx];
                skip = slot.failed;
                if !skip {
                    module_name = slot.module._Fname().to_string();
                    critical = slot.module._Fis_critical();
                } else {
                    module_name = String::new();
                    critical = false;
                }
            } else {
                skip = true;
                module_name = String::new();
                critical = false;
            }
            drop(modules_guard);
            
            if !skip {
                // Emit before module before_shutdown hook
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::BeforeShutdown))?;
                
                // Execute before_shutdown
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    match &mut modules_guard[idx].module {
                        _CModuleType::Sync(module) => {
                            if let Err(err) = module._Fbefore_shutdown(&mut self.ctx) {
                                err_phase = "before_shutdown";
                                error = Some(err);
                            }
                        }
                        _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    // Emit before module shutdown hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::Shutdown))?;
                    
                    // Execute shutdown
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            _CModuleType::Sync(_module) => {
                                if let Err(err) = _module._Fshutdown(&mut self.ctx) {
                                    err_phase = "shutdown";
                                    error = Some(err);
                                }
                            }
                            _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    // Emit before module after_shutdown hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AfterShutdown))?;
                    
                    // Execute after_shutdown
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        match &mut modules_guard[idx].module {
                            _CModuleType::Sync(_module) => {
                                if let Err(err) = _module._Fafter_shutdown(&mut self.ctx) {
                                    err_phase = "after_shutdown";
                                    error = Some(err);
                                }
                            }
                            _CModuleType::Async(_module) => {
                            // Async modules are handled separately in the async phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
            }
            
            // Handle module shutdown error
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
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
        
        // Emit after modules shutdown hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Shutdown asynchronous modules in reverse order
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, None)?;
        
        for idx in (0..module_len).rev() {
            let mut error: Option<crate::core::DMSError> = None;
            let mut err_phase = "async_shutdown";
            let critical;
            let module_name;
            let mut skip;
            
            // Check module state for async modules
            let modules_guard = self.modules.read().await;
            if idx < modules_guard.len() {
                let slot = &modules_guard[idx];
                skip = slot.failed;
                if !skip {
                    match &slot.module {
                        _CModuleType::Async(module) => {
                            module_name = module._Fname().to_string();
                            critical = module._Fis_critical();
                        }
                        _CModuleType::Sync(_) => {
                            skip = true;
                            module_name = String::new();
                            critical = false;
                        }
                    }
                } else {
                    module_name = String::new();
                    critical = false;
                }
            } else {
                skip = true;
                module_name = String::new();
                critical = false;
            }
            drop(modules_guard);
            
            if !skip {
                // Emit before async module before_shutdown hook
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncBeforeShutdown))?;
                
                // Execute async before_shutdown
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                        if let Err(err) = module._Fbefore_shutdown(&mut self.ctx).await {
                            err_phase = "async_before_shutdown";
                            error = Some(err);
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    // Emit before async module shutdown hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncShutdown))?;
                    
                    // Execute async shutdown
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                            if let Err(err) = module._Fshutdown(&mut self.ctx).await {
                                err_phase = "async_shutdown";
                                error = Some(err);
                            }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    // Emit before async module after_shutdown hook
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some(DMSModulePhase::AsyncAfterShutdown))?;
                    
                    // Execute async after_shutdown
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        if let _CModuleType::Async(module) = &mut modules_guard[idx].module {
                            if let Err(err) = module._Fafter_shutdown(&mut self.ctx).await {
                                err_phase = "async_after_shutdown";
                                error = Some(err);
                            }
                        }
                    }
                    drop(modules_guard);
                }
            }
            
            // Handle async module shutdown error
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
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
        
        // Emit after async modules shutdown hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, None)?;

        // Emit shutdown hook
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::Shutdown, &self.ctx, None, None)?;

        Ok(())
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
    fn _Flog_module_error(&self, phase: &str, module_name: &str, err: &crate::core::DMSError) {
        let logger = self.ctx._Flogger();
        let message = format!("module={module_name} phase={phase} error={err}");
        let _ = logger._Ferror("DMS.Runtime", message);
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
///     let app = DMSAppBuilder::_Fnew()
///         ._Fwith_config("config.yaml")?
///         ._Fwith_module(Box::new(MySyncModule::new()))
///         ._Fwith_async_module(Box::new(MyAsyncModule::new()))
///         ._Fbuild()?;
///     
///     app._Frun(|ctx| async move {
///         ctx._Flogger()._Finfo("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSAppBuilder {
    /// Vector of modules with their state, including both sync and async modules
    modules: Vec<_CModuleSlot>, 
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
    pub fn _Fnew() -> Self {
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
    /// - `module`: A boxed synchronous module implementing `_CServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn _Fwith_module(mut self, module: Box<dyn _CServiceModule>) -> Self {
        self.modules.push(_CModuleSlot { module: _CModuleType::Sync(module), failed: false });
        self
    }

    /// Add an asynchronous module to the application.
    /// 
    /// # Parameters
    /// 
    /// - `module`: A boxed asynchronous module implementing `_CAsyncServiceModule`
    /// 
    /// # Returns
    /// 
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn _Fwith_async_module(mut self, module: Box<dyn _CAsyncServiceModule>) -> Self {
        self.modules.push(_CModuleSlot { module: _CModuleType::Async(module), failed: false });
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
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn _Fwith_config(mut self, config_path: impl Into<String>) -> Self {
        self.config_paths.push(config_path.into());
        self
    }

    /// Set custom logging configuration for the application.
    /// 
    /// # Parameters
    /// 
    /// - `logging_config`: Custom logging configuration
    /// 
    /// # Returns
    /// 
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn _Fwith_logging(mut self, logging_config: crate::log::DMSLogConfig) -> Self {
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
    /// The updated `DMSAppBuilder` instance for method chaining.
    pub fn _Fwith_observability(mut self, observability_config: crate::observability::DMSObservabilityConfig) -> Self {
        self.observability_config = Some(observability_config);
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
    /// A `DMSResult` containing the built `DMSAppRuntime` instance, or an error if building fails.
    /// 
    /// # Errors
    /// 
    /// - If configuration loading fails
    /// - If service context creation fails
    /// - If module sorting fails due to circular dependencies
    pub fn _Fbuild(mut self) -> DMSResult<DMSAppRuntime> {
        // Create config manager with specified config paths
        let mut config_manager = crate::config::DMSConfigManager::_Fnew();
        
        // Add specified config files
        for path in &self.config_paths {
            config_manager._Fadd_file_source(path);
        }
        
        // Add default config sources if no paths specified
        if self.config_paths.is_empty() {
            if let Ok(cwd) = std::env::current_dir() {
                let config_dir = cwd.join("config");
                
                // Add all supported config files in order of priority (lowest to highest)
                config_manager._Fadd_file_source(config_dir.join("dms.yaml"));
                config_manager._Fadd_file_source(config_dir.join("dms.yml"));
                config_manager._Fadd_file_source(config_dir.join("dms.toml"));
                config_manager._Fadd_file_source(config_dir.join("dms.json"));
            }
        }
        
        // Add environment variables as highest priority
        config_manager._Fadd_environment_source();
        
        // Load configuration
        config_manager._Fload()?;

        // Create service context with custom configuration
        let ctx = self._Fcreate_service_context(config_manager)?;
        
        // Add core modules
        self.modules.push(_CModuleSlot { module: _CModuleType::Sync(Box::new(DMSLogAnalyticsModule::_Fnew())), failed: false });
        self.modules.push(_CModuleSlot { module: _CModuleType::Sync(Box::new(DMSLifecycleObserver::_Fnew())), failed: false });
        
        // Sort modules based on dependencies and priority
        self.modules = _Fsort_modules(self.modules)?;
        
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
    fn _Fcreate_service_context(&self, config_manager: crate::config::DMSConfigManager) -> DMSResult<DMSServiceContext> {
        let cfg = config_manager._Fconfig();

        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {e}")))?;
        let app_data_root = if let Some(root_str) = cfg._Fget_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        let fs = crate::fs::DMSFileSystem::_Fnew_with_roots(project_root, app_data_root);

        // Use custom logging config if provided, otherwise create from config
        let log_config: crate::log::DMSLogConfig = if let Some(log_config) = &self.logging_config {
            log_config.clone()
        } else {
            crate::log::DMSLogConfig::_Ffrom_config(cfg)
        };
        let logger = crate::log::DMSLogger::_Fnew(&log_config, fs.clone());
        let hooks = crate::hooks::DMSHookBus::_Fnew();
        
        Ok(DMSServiceContext::_Fnew_with(fs, logger, config_manager, hooks))
    }
}

// Python bindings for DMSAppBuilder are handled in src/lib.rs

/// Sort modules based on dependencies and priority
/// Uses topological sort to handle dependencies, and sorts by priority within the same dependency level
fn _Fsort_modules(mut modules: Vec<_CModuleSlot>) -> DMSResult<Vec<_CModuleSlot>> {
    let mut result: Vec<_CModuleSlot> = Vec::with_capacity(modules.len());
    
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
            .map(|(i, slot)| (slot.module._Fname(), i))
            .collect();
        
        // Calculate in-degree for each module
        let mut in_degree: Vec<usize> = vec![0; modules.len()];
        
        for (i, slot) in modules.iter().enumerate() {
            let dependencies = slot.module._Fdependencies();
            for dep_name in dependencies {
                // Check if dependency exists in remaining modules
                if let Some(_dep_index) = name_to_index.get(dep_name) {
                    // Dependency exists, so this module has in-degree
                    in_degree[i] += 1;
                } else {
                    // Dependency not found, check if it's already in result
                    let dep_in_result = result.iter().any(|slot| slot.module._Fname() == dep_name);
                    if !dep_in_result {
                        return Err(crate::core::DMSError::MissingDependency { 
                            module_name: slot.module._Fname().to_string(), 
                            dependency: dep_name.to_string() 
                        });
                    }
                }
            }
        }
        
        // Add all modules with in-degree 0 to the queue
        for (i, &degree) in in_degree.iter().enumerate() {
            if degree == 0 {
                let priority = modules[i].module._Fpriority();
                queue.push((priority, i));
            }
        }
        
        // If no modules with in-degree 0, we have a circular dependency
        if queue.is_empty() {
            return Err(crate::core::DMSError::CircularDependency { 
                modules: modules.iter().map(|slot| slot.module._Fname().to_string()).collect() 
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
