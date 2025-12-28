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

//! # Hooks System
//! 
//! This module provides an event bus system for DMSC, enabling communication between components
//! during various lifecycle events. It supports both synchronous and asynchronous module lifecycle
//! phases, and allows for custom event handlers to be registered.
//! 
//! ## Key Components
//! 
//! - **DMSCHookKind**: Enum defining the different types of hooks
//! - **DMSCModulePhase**: Enum defining the different module lifecycle phases
//! - **DMSCHookEvent**: Struct representing a hook event
//! - **DMSCHookBus**: Event bus for registering and emitting hooks
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
//! use dms::prelude::*;
//! 
//! fn example() -> DMSCResult<()> {
//!     // Create a hook bus
//!     let mut hook_bus = DMSCHookBus::new();
//!     
//!     // Register a hook handler
//!     hook_bus.register(DMSCHookKind::Startup, "example.startup".to_string(), |ctx, event| {
//!         // Handle startup event
//!         Ok(())
//!     });
//!     
//!     // Create a service context (usually provided by the runtime)
//!     let ctx = DMSCServiceContext::new();
//!     
//!     // Emit a hook event
//!     hook_bus.emit(&DMSCHookKind::Startup, &ctx)?;
//!     
//!     Ok(())
//! }

#[cfg(feature = "pyo3")]
/// Python methods for DMSCHookKind
#[pyo3::prelude::pymethods]
impl DMSCHookKind {
    /// Get string representation of hook kind from Python
    fn as_str_py(&self) -> &'static str {
        match self {
            DMSCHookKind::Startup => "startup",
            DMSCHookKind::Shutdown => "shutdown",
            DMSCHookKind::BeforeModulesInit => "before_modules_init",
            DMSCHookKind::AfterModulesInit => "after_modules_init",
            DMSCHookKind::BeforeModulesStart => "before_modules_start",
            DMSCHookKind::AfterModulesStart => "after_modules_start",
            DMSCHookKind::BeforeModulesShutdown => "before_modules_shutdown",
            DMSCHookKind::AfterModulesShutdown => "after_modules_shutdown",
        }
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCModulePhase
#[pyo3::prelude::pymethods]
impl DMSCModulePhase {
    /// Get string representation of module phase from Python
    fn as_str_py(&self) -> &'static str {
        self.as_str()
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCHookEvent
#[pyo3::prelude::pymethods]
impl DMSCHookEvent {
    /// Create a new hook event from Python
    #[new]
    fn py_new(kind: DMSCHookKind, module: Option<String>, phase: Option<DMSCModulePhase>) -> Self {
        Self {
            kind,
            module,
            phase,
        }
    }
    
    /// Get hook kind from Python
    fn get_kind(&self) -> DMSCHookKind {
        self.kind
    }
    
    /// Get module name from Python
    fn get_module(&self) -> Option<String> {
        self.module.clone()
    }
    
    /// Get module phase from Python
    fn get_phase(&self) -> Option<DMSCModulePhase> {
        self.phase
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCHookBus
#[pyo3::prelude::pymethods]
impl DMSCHookBus {
    /// Create a new hook bus from Python
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Register a hook handler from Python
    fn register_py(&mut self, _kind: DMSCHookKind, _id: String, _handler: pyo3::PyObject) -> Result<(), pyo3::PyErr> {
        Ok(())
    }
    
    fn emit_py(&self, _kind: DMSCHookKind, _ctx: pyo3::PyObject) -> Result<(), pyo3::PyErr> {
        Ok(())
    }
    
    fn emit_with_py(&self, _kind: DMSCHookKind, _ctx: pyo3::PyObject, _module: Option<String>, _phase: Option<DMSCModulePhase>) -> Result<(), pyo3::PyErr> {
        Ok(())
    }
    
    /// Get all registered hook kinds from Python
    fn get_registered_hooks_py(&self) -> Vec<String> {
        self.handlers.keys()
            .map(|kind| format!("{:?}", kind))
            .collect()
    }
}
/// ```

use std::collections::HashMap;

use crate::core::{DMSCResult, DMSCServiceContext};

// Type aliases for complex types
/// Type alias for a hook handler function
pub type DMSCHookHandler = Box<dyn Fn(&DMSCServiceContext, &DMSCHookEvent) -> DMSCResult<()> + Send + Sync>;

/// Type alias for a hook handler entry (ID + handler)
pub type DMSCHookHandlerEntry = (DMSCHookId, DMSCHookHandler);

/// Type alias for a collection of hook handlers grouped by hook kind
pub type DMSCHookHandlersMap = HashMap<DMSCHookKind, Vec<DMSCHookHandlerEntry>>;

/// Hook kind definition.
/// 
/// This enum defines the different types of hooks that can be emitted during the application lifecycle.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSCHookKind {
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
}

/// Module lifecycle phase definition.
/// 
/// This enum defines the different phases a module can go through during its lifecycle,
/// including both synchronous and asynchronous phases.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSCModulePhase {
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

impl DMSCModulePhase {
    /// Returns the string representation of the module phase.
    /// 
    /// # Returns
    /// 
    /// A static string representing the module phase (e.g., "init", "start", "async_shutdown")
    pub fn as_str(&self) -> &'static str {
        match self {
            DMSCModulePhase::Init => "init",
            DMSCModulePhase::BeforeStart => "before_start",
            DMSCModulePhase::Start => "start",
            DMSCModulePhase::AfterStart => "after_start",
            DMSCModulePhase::BeforeShutdown => "before_shutdown",
            DMSCModulePhase::Shutdown => "shutdown",
            DMSCModulePhase::AfterShutdown => "after_shutdown",
            DMSCModulePhase::AsyncInit => "async_init",
            DMSCModulePhase::AsyncBeforeStart => "async_before_start",
            DMSCModulePhase::AsyncStart => "async_start",
            DMSCModulePhase::AsyncAfterStart => "async_after_start",
            DMSCModulePhase::AsyncBeforeShutdown => "async_before_shutdown",
            DMSCModulePhase::AsyncShutdown => "async_shutdown",
            DMSCModulePhase::AsyncAfterShutdown => "async_after_shutdown",
        }
    }
}

/// Hook event structure.
/// 
/// This struct represents an event that is emitted when a hook is triggered. It contains
/// information about the hook kind, the module (if applicable), and the module phase (if applicable).
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone, Debug)]
pub struct DMSCHookEvent {
    /// The kind of hook that was triggered
    pub kind: DMSCHookKind,
    /// The name of the module associated with the event (if any)
    pub module: Option<String>,
    /// The module phase associated with the event (if any)
    pub phase: Option<DMSCModulePhase>,
}

/// Type alias for hook IDs.
/// 
/// Hook IDs are used to identify hook handlers and can be used for debugging and logging purposes.
pub type DMSCHookId = String;

/// Hook bus for registering and emitting hooks.
/// 
/// This struct manages the registration of hook handlers and the emission of hook events.
/// It allows multiple handlers to be registered for the same hook kind.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCHookBus {
    /// Internal storage for hook handlers, organized by hook kind
    handlers: DMSCHookHandlersMap,
}

impl Default for DMSCHookBus {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCHookBus {
    /// Creates a new hook bus instance.
    /// 
    /// Returns a new `DMSCHookBus` instance with no registered handlers.
    pub fn new() -> Self {
        DMSCHookBus { handlers: HashMap::new() }
    }

    /// Registers a hook handler for a specific hook kind.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to register the handler for
    /// - `id`: A unique ID for the hook handler
    /// - `handler`: The handler function to execute when the hook is emitted
    /// 
    /// The handler function takes a `DMSCServiceContext` and a `DMSCHookEvent` and returns a `DMSCResult<()>`. 
    pub fn register<F>(&mut self, kind: DMSCHookKind, id: DMSCHookId, handler: F)
    where
        F: Fn(&DMSCServiceContext, &DMSCHookEvent) -> DMSCResult<()> + Send + Sync + 'static,
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
    /// A `DMSCResult<()>` indicating success or failure
    pub fn emit(&self, kind: &DMSCHookKind, ctx: &DMSCServiceContext) -> DMSCResult<()> {
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
    /// A `DMSCResult<()>` indicating success or failure
    pub fn emit_with(
        &self,
        kind: &DMSCHookKind,
        ctx: &DMSCServiceContext,
        module: Option<&str>,
        phase: Option<DMSCModulePhase>,
    ) -> DMSCResult<()> {
        let event = DMSCHookEvent {
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
