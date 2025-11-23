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

// Application runtime and builder for DMS.

use crate::core::{DMSResult, DMSServiceContext};
use crate::core::{_CServiceModule, _CAsyncServiceModule};
use super::lifecycle::DMSLifecycleObserver;
use super::analytics::DMSLogAnalyticsModule;
use crate::hooks::DMSHookKind;
use tokio::sync::RwLock as AsyncRwLock;
use std::sync::Arc;

enum _CModuleType {
    Sync(Box<dyn _CServiceModule>),
    Async(Box<dyn _CAsyncServiceModule>),
}

impl _CModuleType {
    fn _Fname(&self) -> &str {
        match self {
            _CModuleType::Sync(module) => module._Fname(),
            _CModuleType::Async(module) => module._Fname(),
        }
    }

    fn _Fis_critical(&self) -> bool {
        match self {
            _CModuleType::Sync(module) => module._Fis_critical(),
            _CModuleType::Async(module) => module._Fis_critical(),
        }
    }
}

struct _CModuleSlot {
    module: _CModuleType,
    failed: bool,
}

// Public-facing app runtime class.
pub struct DMSAppRuntime {
    ctx: DMSServiceContext,
    modules: Arc<AsyncRwLock<Vec<_CModuleSlot>>>, // internal trait objects with state
}

impl DMSAppRuntime {
    pub async fn _Frun(mut self) -> DMSResult<()> {
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::Startup, &self.ctx, None, Some("startup"))?;

        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, None, Some("before_init"))?;
        
        // Acquire modules lock for initialization
        let modules_guard = self.modules.read().await;
        let module_len = modules_guard.len();
        drop(modules_guard); // Release lock early
        
        for idx in 0..module_len {
            let mut error: Option<crate::core::DMSError> = None;
            let critical;
            let module_name;
            let skip;
            
            // Acquire read lock to check module state
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
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesInit, &self.ctx, Some(&module_name), Some("init"))?;
                
                // Acquire write lock for module initialization
                let mut modules_guard = self.modules.write().await;
                if idx < modules_guard.len() {
                    match &mut modules_guard[idx].module {
                        _CModuleType::Sync(_module) => {
                            if let Err(err) = _module._Finit(&mut self.ctx) {
                                error = Some(err);
                            }
                        }
                        _CModuleType::Async(_module) => {
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                    }
                }
                drop(modules_guard);
            }
            if let Some(err) = error {
                self._Flog_module_error("init", &module_name, &err);
                if critical {
                    return Err(err);
                } else {
                    // Update module failed state
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        modules_guard[idx].failed = true;
                    }
                }
            }
        }
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesInit, &self.ctx, None, Some("after_init"))?;

        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, None, Some("before_start_all"))?;
        
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
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("before_start"))?;
                
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("start"))?;
                    
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
                

                
                if error.is_none() {
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("after_start"))?;
                    
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
            }
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
                if critical {
                    return Err(err);
                } else {
                    // Update module failed state
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        modules_guard[idx].failed = true;
                    }
                }
            }
        }
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, Some("after_start_all"))?;

        // Handle async modules separately
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, None, Some("before_async_start_all"))?;
        
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
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("before_async_init"))?;
                
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
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("before_async_start"))?;
                    
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
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("async_start"))?;
                    
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
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesStart, &self.ctx, Some(&module_name), Some("async_after_start"))?;
                    
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
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
                if critical {
                    return Err(err);
                } else {
                    // Update module failed state
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        modules_guard[idx].failed = true;
                    }
                }
            }
        }
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesStart, &self.ctx, None, Some("after_async_start_all"))?;

        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, Some("before_shutdown_all"))?;
        
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
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("before_shutdown"))?;
                
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                    }
                }
                drop(modules_guard);
                
                if error.is_none() {
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("shutdown"))?;
                    
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
                
                if error.is_none() {
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("after_shutdown"))?;
                    
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
                            // For async modules, we need to handle this differently
                            // We'll need to make the entire runtime async-aware
                            // For now, we'll skip async modules in the sync phase
                        }
                        }
                    }
                    drop(modules_guard);
                }
            }
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
                if critical {
                    return Err(err);
                } else {
                    // Update module failed state
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        modules_guard[idx].failed = true;
                    }
                }
            }
        }
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, Some("after_shutdown_all"))?;

        // Handle async modules shutdown separately
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, None, Some("before_async_shutdown_all"))?;
        
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
                self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("before_async_shutdown"))?;
                
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
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("async_shutdown"))?;
                    
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
                    self.ctx._Fhooks()._Femit_with(&DMSHookKind::BeforeModulesShutdown, &self.ctx, Some(&module_name), Some("async_after_shutdown"))?;
                    
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
            if let Some(err) = error {
                self._Flog_module_error(err_phase, &module_name, &err);
                if critical {
                    return Err(err);
                } else {
                    // Update module failed state
                    let mut modules_guard = self.modules.write().await;
                    if idx < modules_guard.len() {
                        modules_guard[idx].failed = true;
                    }
                }
            }
        }
        self.ctx._Fhooks()._Femit_with(&DMSHookKind::AfterModulesShutdown, &self.ctx, None, Some("after_async_shutdown_all"))?;

        self.ctx._Fhooks()._Femit_with(&DMSHookKind::Shutdown, &self.ctx, None, Some("shutdown"))?;

        Ok(())
    }

    fn _Flog_module_error(&self, phase: &str, module_name: &str, err: &crate::core::DMSError) {
        let logger = self.ctx._Flogger();
        let message = format!("module={} phase={} error={}", module_name, phase, err);
        let _ = logger._Ferror("DMS.Runtime", message);
    }
}

// Public-facing app builder class. All external construction goes through this.
pub struct DMSAppBuilder {
    modules: Vec<_CModuleSlot>, // internal trait objects with state
}

impl DMSAppBuilder {
    pub fn _Fnew() -> Self {
        DMSAppBuilder { modules: Vec::new() }
    }

    pub fn _Fwith_module(mut self, module: Box<dyn _CServiceModule>) -> Self {
        self.modules.push(_CModuleSlot { module: _CModuleType::Sync(module), failed: false });
        self
    }

    pub fn _Fwith_async_module(mut self, module: Box<dyn _CAsyncServiceModule>) -> Self {
        self.modules.push(_CModuleSlot { module: _CModuleType::Async(module), failed: false });
        self
    }

    pub fn _Fbuild(mut self) -> DMSResult<DMSAppRuntime> {
        self
            .modules
            .push(_CModuleSlot { module: _CModuleType::Sync(Box::new(DMSLogAnalyticsModule::_Fnew())), failed: false });
        self.modules.push(_CModuleSlot { module: _CModuleType::Sync(Box::new(DMSLifecycleObserver::_Fnew())), failed: false });
        let ctx = DMSServiceContext::_Fnew_default()?;
        let runtime = DMSAppRuntime {
            ctx,
            modules: Arc::new(AsyncRwLock::new(self.modules)),
        };
        Ok(runtime)
    }
}
