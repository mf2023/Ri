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

//! # Lifecycle Observer
//! 
//! This module provides a lifecycle observer that logs all hook events in the Ri application.
//! It implements the `ServiceModule` trait and registers handlers for all hook kinds to provide
//! comprehensive lifecycle logging.
//! 
//! ## Key Components
//! 
//! - **RiLifecycleObserver**: Service module that logs all hook events
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Logging**: Logs all hook events with detailed information
//! 2. **Non-Intrusive**: Operates by listening to hook events without modifying core functionality
//! 3. **Non-Critical**: Can fail without causing the entire system to fail
//! 4. **Detailed Context**: Provides module, phase, and kind information for each event

use crate::core::{RiResult, RiServiceContext, ServiceModule};
use crate::hooks::{RiHookBus, RiHookEvent, RiHookKind};

/// Lifecycle observer module for Ri.
/// 
/// This module logs all hook events in the Ri application, providing comprehensive
/// visibility into the application lifecycle.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiLifecycleObserver;

impl Default for RiLifecycleObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl RiLifecycleObserver {
    /// Creates a new instance of the lifecycle observer.
    /// 
    /// Returns a new `RiLifecycleObserver` instance.
    pub fn new() -> Self {
        RiLifecycleObserver
    }
}

impl ServiceModule for RiLifecycleObserver {
    /// Returns the name of the lifecycle observer module.
    /// 
    /// This name is used for identification, logging, and dependency resolution.
    fn name(&self) -> &str {
        "Ri.LifecycleObserver"
    }

    /// Indicates if the lifecycle observer is critical to the operation of the system.
    /// 
    /// The lifecycle observer is non-critical, meaning it can fail without causing the entire
    /// system to fail.
    fn is_critical(&self) -> bool {
        false
    }

    /// Initializes the lifecycle observer.
    /// 
    /// This method registers handlers for all hook kinds to log lifecycle events.
    /// Each handler logs detailed information about the event, including:
    /// - Hook kind
    /// - Module name (if applicable)
    /// - Module phase (if applicable)
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing the hook bus
    /// 
    /// # Returns
    /// 
    /// A `RiResult` indicating success or failure
    fn init(&mut self, ctx: &mut RiServiceContext) -> RiResult<()> {
        let hooks: &mut RiHookBus = ctx.hooks_mut();
        let all_kinds = [
            RiHookKind::Startup,
            RiHookKind::Shutdown,
            RiHookKind::BeforeModulesInit,
            RiHookKind::AfterModulesInit,
            RiHookKind::BeforeModulesStart,
            RiHookKind::AfterModulesStart,
            RiHookKind::BeforeModulesShutdown,
            RiHookKind::AfterModulesShutdown,
            RiHookKind::ConfigReload,
        ];

        for &kind in &all_kinds {
            let kind_str = match kind {
                RiHookKind::Startup => "Startup",
                RiHookKind::Shutdown => "Shutdown",
                RiHookKind::BeforeModulesInit => "BeforeModulesInit",
                RiHookKind::AfterModulesInit => "AfterModulesInit",
                RiHookKind::BeforeModulesStart => "BeforeModulesStart",
                RiHookKind::AfterModulesStart => "AfterModulesStart",
                RiHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                RiHookKind::AfterModulesShutdown => "AfterModulesShutdown",
                RiHookKind::ConfigReload => "ConfigReload",
            };
            let handler_name = format!("dms.lifecycle.{}", kind_str.to_lowercase());
            
            hooks.register(kind, handler_name, move |_ctx, event: &RiHookEvent| {
                let logger = _ctx.logger();
                let module = event.module.as_deref().unwrap_or("-");
                let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
                let message = format!("kind={} module={} phase={}", kind_str, module, phase);
                let _ = logger.info("Ri.Lifecycle", message);
                Ok(())
            });
        }

        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiLifecycleObserver {
    #[new]
    fn new_py() -> Self {
        Self::new()
    }

    fn name(&self) -> String {
        "Ri.LifecycleObserver".to_string()
    }

    fn is_critical(&self) -> bool {
        false
    }

    fn __repr__(&self) -> String {
        "RiLifecycleObserver".to_string()
    }
}
