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

//! # Hooks System
//! 
//! This module provides an event bus system for DMS, enabling communication between components
//! during various lifecycle events. It supports both synchronous and asynchronous module lifecycle
//! phases, and allows for custom event handlers to be registered.
//! 
//! ## Key Components
//! 
//! - **DMSHookKind**: Enum defining the different types of hooks
//! - **DMSModulePhase**: Enum defining the different module lifecycle phases
//! - **DMSHookEvent**: Struct representing a hook event
//! - **DMSHookBus**: Event bus for registering and emitting hooks
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
//! fn example() -> DMSResult<()> {
//!     // Create a hook bus
//!     let mut hook_bus = DMSHookBus::_Fnew();
//!     
//!     // Register a hook handler
//!     hook_bus._Fregister(DMSHookKind::Startup, "example.startup".to_string(), |ctx, event| {
//!         // Handle startup event
//!         Ok(())
//!     });
//!     
//!     // Create a service context (usually provided by the runtime)
//!     let ctx = DMSServiceContext::_Fnew();
//!     
//!     // Emit a hook event
//!     hook_bus._Femit(&DMSHookKind::Startup, &ctx)?;
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;

use crate::core::{DMSResult, DMSServiceContext};

/// Hook kind definition.
/// 
/// This enum defines the different types of hooks that can be emitted during the application lifecycle.
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSHookKind {
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
#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSModulePhase {
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

impl DMSModulePhase {
    /// Returns the string representation of the module phase.
    /// 
    /// # Returns
    /// 
    /// A static string representing the module phase (e.g., "init", "start", "async_shutdown")
    pub fn as_str(&self) -> &'static str {
        match self {
            DMSModulePhase::Init => "init",
            DMSModulePhase::BeforeStart => "before_start",
            DMSModulePhase::Start => "start",
            DMSModulePhase::AfterStart => "after_start",
            DMSModulePhase::BeforeShutdown => "before_shutdown",
            DMSModulePhase::Shutdown => "shutdown",
            DMSModulePhase::AfterShutdown => "after_shutdown",
            DMSModulePhase::AsyncInit => "async_init",
            DMSModulePhase::AsyncBeforeStart => "async_before_start",
            DMSModulePhase::AsyncStart => "async_start",
            DMSModulePhase::AsyncAfterStart => "async_after_start",
            DMSModulePhase::AsyncBeforeShutdown => "async_before_shutdown",
            DMSModulePhase::AsyncShutdown => "async_shutdown",
            DMSModulePhase::AsyncAfterShutdown => "async_after_shutdown",
        }
    }
}

/// Hook event structure.
/// 
/// This struct represents an event that is emitted when a hook is triggered. It contains
/// information about the hook kind, the module (if applicable), and the module phase (if applicable).
pub struct DMSHookEvent {
    /// The kind of hook that was triggered
    pub kind: DMSHookKind,
    /// The name of the module associated with the event (if any)
    pub module: Option<String>,
    /// The module phase associated with the event (if any)
    pub phase: Option<DMSModulePhase>,
}

/// Type alias for hook IDs.
/// 
/// Hook IDs are used to identify hook handlers and can be used for debugging and logging purposes.
pub type DMSHookId = String;

/// Hook bus for registering and emitting hooks.
/// 
/// This struct manages the registration of hook handlers and the emission of hook events.
/// It allows multiple handlers to be registered for the same hook kind.
pub struct DMSHookBus {
    /// Internal storage for hook handlers, organized by hook kind
    handlers: HashMap<DMSHookKind, Vec<(DMSHookId, Box<dyn Fn(&DMSServiceContext, &DMSHookEvent) -> DMSResult<()> + Send + Sync>)>>,
}

impl DMSHookBus {
    /// Creates a new hook bus instance.
    /// 
    /// Returns a new `DMSHookBus` instance with no registered handlers.
    pub fn _Fnew() -> Self {
        DMSHookBus { handlers: HashMap::new() }
    }

    /// Registers a hook handler for a specific hook kind.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to register the handler for
    /// - `id`: A unique ID for the hook handler
    /// - `handler`: The handler function to execute when the hook is emitted
    /// 
    /// The handler function takes a `DMSServiceContext` and a `DMSHookEvent` and returns a `DMSResult<()>`. 
    pub fn _Fregister<F>(&mut self, kind: DMSHookKind, id: DMSHookId, handler: F)
    where
        F: Fn(&DMSServiceContext, &DMSHookEvent) -> DMSResult<()> + Send + Sync + 'static,
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
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Femit(&self, kind: &DMSHookKind, ctx: &DMSServiceContext) -> DMSResult<()> {
        self._Femit_with(kind, ctx, None, None)
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
    /// A `DMSResult<()>` indicating success or failure
    pub fn _Femit_with(
        &self,
        kind: &DMSHookKind,
        ctx: &DMSServiceContext,
        module: Option<&str>,
        phase: Option<DMSModulePhase>,
    ) -> DMSResult<()> {
        let event = DMSHookEvent {
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
