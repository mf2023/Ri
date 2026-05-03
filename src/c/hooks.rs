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

//! # Hooks Module C API
//!
//! This module provides C language bindings for Ri's hook system. The hooks module enables
//! extensible application behavior through a publish-subscribe pattern where components can
//! register callback functions (hooks) that are invoked at specific points in the application
//! lifecycle or in response to specific events. This C API enables C/C++ applications to
//! leverage Ri's extensibility mechanisms for building modular and customizable applications.
//!
//! ## Module Architecture
//!
//! The hooks module centers around a single primary component:
//!
//! - **RiHookBus**: Central event bus for registering hooks and dispatching events. The hook
//!   bus manages the complete lifecycle of hooks including registration, invocation, and
//!   unregistration. It provides a thread-safe mechanism for components to communicate through
//!   loosely-coupled event handlers.
//!
//! ## Hook Concepts
//!
//! The hook system implements several key concepts:
//!
//! - **Hooks**: Callback functions registered at specific points in the application lifecycle.
//!   Hooks can modify behavior, perform side effects, or transform data as it flows through
//!   the system.
//!
//! - **Hook Points**: Well-defined locations in the code where registered hooks are invoked.
//!   Common hook points include application startup, shutdown, request processing, error
//!   handling, and custom business events.
//!
//! - **Hook Priority**: Ordering mechanism that controls the sequence of hook execution when
//!   multiple hooks are registered at the same hook point. Higher priority hooks execute first.
//!
//! - **Hook Context**: Data passed to hooks containing information about the event and allowing
//!   hooks to communicate through shared context.
//!
//! - **Hook Filters**: Capability for hooks to filter whether they should be invoked based on
//!   event properties without full registration overhead.
//!
//! ## Hook Types
//!
//! The system supports various hook types for different use cases:
//!
//! - **Synchronous Hooks**: Execute immediately when the hook point is reached. Most common
//!   type for application extension. Blocking until all hooks complete.
//!
//! - **Asynchronous Hooks**: Execute in the background without blocking the main flow. Useful
//!   for logging, metrics, or long-running operations that shouldn't delay primary processing.
//!
//! - **One-Time Hooks**: Execute only once, then automatically unregister. Useful for
//!   initialization or cleanup tasks that should run a single time.
//!
//! - **Conditional Hooks**: Only execute when specific conditions are met. Conditions checked
//!   before invoking the hook function, reducing overhead for hooks that rarely apply.
//!
//! - **Transform Hooks**: Modify data passing through the hook point. Data is passed to the
//!   hook, transformed, and passed to the next hook or back to the caller.
//!
//! ## Hook Bus Architecture
//!
//! The hook bus implements a centralized event distribution system:
//!
//! - **Event Dispatching**: Efficient routing of events to registered hooks based on hook point.
//!   Supports both synchronous and asynchronous dispatch.
//!
//! - **Hook Registration**: Thread-safe registration of hooks with priority, filters, and
//!   configuration options. Supports registration during any phase of application lifecycle.
//!
//! - **Error Handling**: Configurable behavior when hooks return errors including stop-on-error,
//!   continue-with-error, and error logging strategies.
//!
//! - **Performance Optimization**: Batch dispatch, hook filtering, and lazy evaluation minimize
//!   overhead for high-frequency hook points.
//!
//! ## Common Hook Points
//!
//! The Ri framework defines standard hook points:
//!
//! - **Application Lifecycle Hooks**: pre_startup, post_startup, pre_shutdown, post_shutdown
//! - **Request Processing Hooks**: pre_request, post_request, on_error
//! - **Configuration Hooks**: pre_config_load, post_config_load, on_config_change
//! - **Logging Hooks**: on_log_message, on_log_level_change
//! - **Custom Application Hooks**: Application-defined event types
//!
//! ## Hook Priority System
//!
//! Hooks execute in priority order from highest to lowest:
//!
//! - **System Hooks** (1000-900): Reserved for Ri framework internal use
//! - **High Priority** (800-600): Critical application extensions
//! - **Normal Priority** (500-400): Standard application hooks
//! - **Low Priority** (300-200): Monitoring and observability hooks
//! - **System Low** (100-0): Reserved for cleanup and finalization
//!
//! Equal priority hooks execute in registration order. Consider using explicit priorities
//! rather than relying on registration order for reproducibility.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Hook callbacks must be properly unregistered before freeing context
//! - Hook context data must be freed by the application
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Hook registration is safe from any thread
//! - Hook invocation occurs in the context of the triggering code
//! - Asynchronous hooks run in a thread pool
//! - Concurrent hook dispatch uses internal synchronization
//!
//! ## Performance Characteristics
//!
//! Hook operations have the following performance profiles:
//!
//! - Hook registration: O(1) amortized
//! - Hook lookup: O(log n) where n is registered hook count
//! - Synchronous dispatch: O(n * t) where n is hook count, t is execution time
//! - Asynchronous dispatch: O(1) to queue, O(n * t) in thread pool
//!
//! ## Usage Example
//!
//! ```c
//! // Create hook bus instance
//! RiHookBus* bus = ri_hook_bus_new();
//! if (bus == NULL) {
//!     fprintf(stderr, "Failed to create hook bus\n");
//!     return ERROR_INIT;
//! }
//!
//! // Register startup hook
//! int startup_result = ri_hook_bus_register(
//!     bus,
//!     "pre_startup",
//!     500,  // normal priority
//!     on_startup_hook,
//!     NULL  // user data
//! );
//!
//! if (startup_result != 0) {
//!     fprintf(stderr, "Failed to register startup hook\n");
//! }
//!
//! // Register request processing hook
//! ri_hook_bus_register(bus, "pre_request", 500, on_pre_request, NULL);
//! ri_hook_bus_register(bus, "post_request", 500, on_post_request, NULL);
//!
//! // Register shutdown hook
//! ri_hook_bus_register(bus, "pre_shutdown", 300, on_shutdown, NULL);
//!
//! // Trigger custom event
//! RiHookContext* context = ri_hook_context_create();
//! ri_hook_context_set_string(context, "event_name", "user_action");
//! ri_hook_context_set_int(context, "user_id", 12345);
//!
//! ri_hook_bus_dispatch(bus, "on_user_action", context);
//!
//! ri_hook_context_free(context);
//!
//! // Unregister hooks before shutdown
//! ri_hook_bus_unregister(bus, "pre_startup", on_startup_hook);
//! ri_hook_bus_unregister(bus, "pre_request", on_pre_request);
//! ri_hook_bus_unregister(bus, "post_request", on_post_request);
//! ri_hook_bus_unregister(bus, "pre_shutdown", on_shutdown);
//!
//! // Cleanup
//! ri_hook_bus_free(bus);
//! ```
//!
//! ## Hook Callback Signature
//!
//! Hook callbacks must conform to the following signature:
//!
//! ```c
//! typedef int (*RiHookCallback)(
//!     const char* hook_point,      // Name of the hook point
//!     RiHookContext* context,     // Event context data
//!     void* user_data              // User-provided data
//! );
//! ```
//!
//! Return values:
//!
//! - 0: Success, continue processing other hooks
//! - Positive: Success with value, stop processing if configured
//! - Negative: Error, stop processing if configured
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::hooks`: Rust hooks module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The hooks module is always enabled as it provides fundamental extensibility
//! infrastructure for Ri applications.

use crate::hooks::RiHookBus;


c_wrapper!(CRiHookBus, RiHookBus);

// RiHookBus constructors and destructors
#[no_mangle]
pub extern "C" fn ri_hook_bus_new() -> *mut CRiHookBus {
    let bus = RiHookBus::new();
    let ptr = Box::into_raw(Box::new(CRiHookBus::new(bus)));
    crate::c::register_ptr(ptr as usize);
    ptr
}
c_destructor!(ri_hook_bus_free, CRiHookBus);
