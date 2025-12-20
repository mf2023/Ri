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

#![allow(non_snake_case)]

//! # Lifecycle Observer
//! 
//! This module provides a lifecycle observer that logs all hook events in the DMSC application.
//! It implements the `ServiceModule` trait and registers handlers for all hook kinds to provide
//! comprehensive lifecycle logging.
//! 
//! ## Key Components
//! 
//! - **DMSCLifecycleObserver**: Service module that logs all hook events
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Logging**: Logs all hook events with detailed information
//! 2. **Non-Intrusive**: Operates by listening to hook events without modifying core functionality
//! 3. **Non-Critical**: Can fail without causing the entire system to fail
//! 4. **Detailed Context**: Provides module, phase, and kind information for each event

use crate::core::{DMSCResult, DMSCServiceContext, ServiceModule};
use crate::hooks::{DMSCHookBus, DMSCHookEvent, DMSCHookKind};

/// Lifecycle observer module for DMSC.
/// 
/// This module logs all hook events in the DMSC application, providing comprehensive
/// visibility into the application lifecycle.
pub struct DMSCLifecycleObserver;

impl Default for DMSCLifecycleObserver {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCLifecycleObserver {
    /// Creates a new instance of the lifecycle observer.
    /// 
    /// Returns a new `DMSCLifecycleObserver` instance.
    pub fn new() -> Self {
        DMSCLifecycleObserver
    }
}

impl ServiceModule for DMSCLifecycleObserver {
    /// Returns the name of the lifecycle observer module.
    /// 
    /// This name is used for identification, logging, and dependency resolution.
    fn name(&self) -> &str {
        "DMSC.LifecycleObserver"
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
    /// A `DMSCResult` indicating success or failure
    fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        let hooks: &mut DMSCHookBus = ctx.hooks_mut();

        // Register handler for Startup events
        hooks.register(DMSCHookKind::Startup, "dms.lifecycle.startup".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for Shutdown events
        hooks.register(DMSCHookKind::Shutdown, "dms.lifecycle.shutdown".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for BeforeModulesInit events
        hooks.register(DMSCHookKind::BeforeModulesInit, "dms.lifecycle.before_init".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for AfterModulesInit events
        hooks.register(DMSCHookKind::AfterModulesInit, "dms.lifecycle.after_init".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for BeforeModulesStart events
        hooks.register(DMSCHookKind::BeforeModulesStart, "dms.lifecycle.before_start".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for AfterModulesStart events
        hooks.register(DMSCHookKind::AfterModulesStart, "dms.lifecycle.after_start".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for BeforeModulesShutdown events
        hooks.register(DMSCHookKind::BeforeModulesShutdown, "dms.lifecycle.before_shutdown".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        // Register handler for AfterModulesShutdown events
        hooks.register(DMSCHookKind::AfterModulesShutdown, "dms.lifecycle.after_shutdown".to_string(), |_ctx, event: &DMSCHookEvent| {
            let logger = _ctx.logger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.map(|p| p.as_str()).unwrap_or("-");
            let kind = match event.kind {
                DMSCHookKind::Startup => "Startup",
                DMSCHookKind::Shutdown => "Shutdown",
                DMSCHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSCHookKind::AfterModulesInit => "AfterModulesInit",
                DMSCHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSCHookKind::AfterModulesStart => "AfterModulesStart",
                DMSCHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSCHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={kind} module={module} phase={phase}");
            let _ = logger.info("DMSC.Lifecycle", message);
            Ok(())
        });

        Ok(())
    }
}
