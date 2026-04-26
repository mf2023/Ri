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

//! # Hooks System
//! 
//! This module provides an event bus system for Ri, enabling communication between components
//! during various lifecycle events. It supports both synchronous and asynchronous module lifecycle
//! phases, and allows for custom event handlers to be registered.
//! 
//! ## Key Components
//! 
//! - **RiHookKind**: Enum defining the different types of hooks
//! - **RiModulePhase**: Enum defining the different module lifecycle phases
//! - **RiHookEvent**: Struct representing a hook event
//! - **RiHookBus**: Event bus for registering and emitting hooks
//! 
//! ## Design Principles
//! 
//! 1. **Event-Driven Architecture**: Uses an event bus pattern for loose coupling between components
//! 2. **Lifecycle Support**: Covers all stages of module lifecycle, both synchronous and asynchronous
//! 3. **Type Safety**: Uses enums for hook kinds and phases to ensure type safety
//! 4. **Flexibility**: Allows registering multiple handlers for the same hook
//! 5. **Contextual Information**: Events carry contextual information about the module and phase
//! 6. **Error Propagation**: Hook handlers can return errors that propagate up the call stack
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! fn example() -> RiResult<()> {
//!     // Create a hook bus
//!     let mut hook_bus = RiHookBus::new();
//!     
//!     // Register a hook handler
//!     hook_bus.register(RiHookKind::Startup, "example.startup".to_string(), |ctx, event| {
//!         // Handle startup event
//!         Ok(())
//!     });
//!     
//!     // Create a service context (usually provided by the runtime)
//!     let ctx = RiServiceContext::new();
//!     
//!     // Emit a hook event
//!     hook_bus.emit(&RiHookKind::Startup, &ctx)?;
//!     
//!     Ok(())
//! }

use std::collections::HashMap as FxHashMap;

use crate::core::{RiResult, RiServiceContext};

// Type aliases for complex types
/// Type alias for a hook handler function
pub type RiHookHandler = Box<dyn Fn(&RiServiceContext, &RiHookEvent) -> RiResult<()> + Send + Sync>;

/// Type alias for a hook handler entry (ID + handler)
pub type RiHookHandlerEntry = (RiHookId, RiHookHandler);

/// Type alias for a collection of hook handlers grouped by hook kind
pub type RiHookHandlersMap = FxHashMap<RiHookKind, Vec<RiHookHandlerEntry>>;

/// Hook kind definition.
/// 
/// This enum defines the different types of hooks that can be emitted during the application lifecycle.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum RiHookKind {
    /// Emitted when the application starts up
    Startup,
    /// Emitted when the application shuts down
    Shutdown,
    /// Emitted before modules are initialized
    BeforeModulesInit,
    /// Emitted after modules are initialized
    AfterModulesInit,
    /// Emitted before modules are started
    BeforeModulesStart,
    /// Emitted after modules are started
    AfterModulesStart,
    /// Emitted before modules are shut down
    BeforeModulesShutdown,
    /// Emitted after modules are shut down
    AfterModulesShutdown,
    /// Emitted when configuration is reloaded
    ConfigReload,
}

/// Module lifecycle phase definition.
/// 
/// This enum defines the different phases a module can go through during its lifecycle,
/// including both synchronous and asynchronous phases.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum RiModulePhase {
    /// Synchronous initialization phase
    Init,
    /// Synchronous phase before starting
    BeforeStart,
    /// Synchronous start phase
    Start,
    /// Synchronous phase after starting
    AfterStart,
    /// Synchronous phase before shutting down
    BeforeShutdown,
    /// Synchronous shutdown phase
    Shutdown,
    /// Synchronous phase after shutting down
    AfterShutdown,
    /// Asynchronous initialization phase
    AsyncInit,
    /// Asynchronous phase before starting
    AsyncBeforeStart,
    /// Asynchronous start phase
    AsyncStart,
    /// Asynchronous phase after starting
    AsyncAfterStart,
    /// Asynchronous phase before shutting down
    AsyncBeforeShutdown,
    /// Asynchronous shutdown phase
    AsyncShutdown,
    /// Asynchronous phase after shutting down
    AsyncAfterShutdown,
}

impl RiModulePhase {
    /// Returns the string representation of the module phase.
    /// 
    /// # Returns
    /// 
    /// A static string representing the module phase (e.g., "init", "start", "async_shutdown")
    pub fn as_str(&self) -> &'static str {
        match self {
            RiModulePhase::Init => "init",
            RiModulePhase::BeforeStart => "before_start",
            RiModulePhase::Start => "start",
            RiModulePhase::AfterStart => "after_start",
            RiModulePhase::BeforeShutdown => "before_shutdown",
            RiModulePhase::Shutdown => "shutdown",
            RiModulePhase::AfterShutdown => "after_shutdown",
            RiModulePhase::AsyncInit => "async_init",
            RiModulePhase::AsyncBeforeStart => "async_before_start",
            RiModulePhase::AsyncStart => "async_start",
            RiModulePhase::AsyncAfterStart => "async_after_start",
            RiModulePhase::AsyncBeforeShutdown => "async_before_shutdown",
            RiModulePhase::AsyncShutdown => "async_shutdown",
            RiModulePhase::AsyncAfterShutdown => "async_after_shutdown",
        }
    }
}

/// Hook event structure.
/// 
/// This struct represents an event that is emitted when a hook is triggered. It contains
/// information about the hook kind, the module (if applicable), and the module phase (if applicable).
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone, Debug)]
pub struct RiHookEvent {
    /// The kind of hook that was triggered
    pub kind: RiHookKind,
    /// The name of the module associated with the event (if any)
    pub module: Option<String>,
    /// The module phase associated with the event (if any)
    pub phase: Option<RiModulePhase>,
}

impl RiHookEvent {
    /// Creates a new hook event.
    pub fn new(kind: RiHookKind, module: Option<String>, phase: Option<RiModulePhase>) -> Self {
        Self { kind, module, phase }
    }

    /// Creates a config reload event.
    pub fn config_reload(_path: String, _timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        Self { 
            kind: RiHookKind::ConfigReload, 
            module: Some("config_manager".to_string()), 
            phase: None,
        }
    }
}

/// Type alias for hook IDs.
/// 
/// Hook IDs are used to identify hook handlers and can be used for debugging and logging purposes.
pub type RiHookId = String;

/// Hook bus for registering and emitting hooks.
/// 
/// This struct manages the registration of hook handlers and the emission of hook events.
/// It allows multiple handlers to be registered for the same hook kind.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiHookBus {
    /// Internal storage for hook handlers, organized by hook kind
    handlers: RiHookHandlersMap,
}

impl Default for RiHookBus {
    fn default() -> Self {
        Self::new()
    }
}

impl RiHookBus {
    /// Creates a new hook bus instance.
    /// 
    /// Returns a new `RiHookBus` instance with no registered handlers.
    pub fn new() -> Self {
        RiHookBus { handlers: FxFxHashMap::default() }
    }

    /// Registers a hook handler for a specific hook kind.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to register the handler for
    /// - `id`: A unique ID for the hook handler
    /// - `handler`: The handler function to execute when the hook is emitted
    /// 
    /// The handler function takes a `RiServiceContext` and a `RiHookEvent` and returns a `RiResult<()>`. 
    pub fn register<F>(&mut self, kind: RiHookKind, id: RiHookId, handler: F)
    where
        F: Fn(&RiServiceContext, &RiHookEvent) -> RiResult<()> + Send + Sync + 'static,
    {
        self.handlers.entry(kind).or_default().push((id, Box::new(handler)));
    }

    /// Emits a hook event of the specified kind.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to emit
    /// - `ctx`: The service context to pass to the hook handlers
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    pub fn emit(&self, kind: &RiHookKind, ctx: &RiServiceContext) -> RiResult<()> {
        self.emit_with(kind, ctx, None, None)
    }

    /// Emits a hook event with additional contextual information.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to emit
    /// - `ctx`: The service context to pass to the hook handlers
    /// - `module`: The name of the module associated with the event (if any)
    /// - `phase`: The module phase associated with the event (if any)
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    pub fn emit_with(
        &self,
        kind: &RiHookKind,
        ctx: &RiServiceContext,
        module: Option<&str>,
        phase: Option<RiModulePhase>,
    ) -> RiResult<()> {
        let event = RiHookEvent {
            kind: *kind,
            module: module.map(|s| s.to_string()),
            phase,
        };
        if let Some(list) = self.handlers.get(kind) {
            for (_id, handler) in list {
                handler(ctx, &event)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiHookBus {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
}
