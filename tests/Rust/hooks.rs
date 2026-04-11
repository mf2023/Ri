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

//! # Hooks Module Tests
//!
//! This module contains comprehensive tests for the Ri hooks system, a publish-subscribe
//! mechanism for lifecycle event handling that enables modular extension and integration
//! of system components through event-driven coordination.
//!
//! ## Test Coverage
//!
//! - **RiHookBus**: Tests for the central event bus that manages hook registration,
//!   emission, and propagation. The bus supports both simple emission and detailed
//!   emission with module/phase context
//!
//! - **RiHookEvent**: Tests for event data structure including event kind, timestamp,
//!   associated module name, and lifecycle phase information
//!
//! - **RiHookKind**: Tests for event type classification including Startup, Shutdown,
//!   and custom event kinds for different system integration points
//!
//! - **RiModulePhase**: Tests for lifecycle phase enumeration covering both synchronous
//!   and asynchronous phases: Init, BeforeStart, Start, AfterStart, BeforeShutdown,
//!   Shutdown, AfterShutdown, and their async variants
//!
//! ## Design Principles
//!
//! The hooks system follows an event-driven architecture:
//! - **Decoupling**: Components communicate through events rather than direct dependencies
//! - **Extensibility**: New functionality can be added by registering new hooks without
//!   modifying existing code
//! - **Lifecycle Awareness**: System components can participate in startup and shutdown
//!   sequences in a coordinated manner
//! - **Error Isolation**: Hook failures are isolated and don't crash the entire system
//!
//! ## Hook Execution Model
//!
//! The hook bus implements a synchronous execution model:
//! 1. Handlers are registered with a hook kind and identifier
//! 2. When emit() is called, all registered handlers for that kind are executed in order
//! 3. Each handler receives the service context and event details
//! 4. If any handler returns an error, emission stops and returns the error
//! 5. Successful handlers complete before the emit call returns
//!
//! ## Module Lifecycle Phases
//!
//! The system supports comprehensive lifecycle phase coverage:
//! - **Init**: Early initialization before any services start
//! - **BeforeStart**: Pre-startup configuration and validation
//! - **Start**: Main startup phase where services become operational
//! - **AfterStart**: Post-startup verification and registration
//! - **BeforeShutdown**: Pre-shutdown graceful termination preparation
//! - **Shutdown**: Main shutdown phase where services stop
//! - **AfterShutdown**: Final cleanup and resource release
//!
//! Each phase has both synchronous and asynchronous variants to support different
//! component types and initialization requirements.
//!
//! ## Use Cases
//!
//! The hooks system enables several integration patterns:
//! - **Configuration Loading**: Load config from various sources during Init phase
//! - **Database Migration**: Run migrations before Start phase completes
//! - **Health Registration**: Register with service discovery in AfterStart
//! - **Metrics Export**: Initialize metrics exporters in Init phase
//! - **Graceful Shutdown**: Stop accepting new requests in BeforeShutdown
//! - **Connection Cleanup**: Close database connections in Shutdown phase

use ri::hooks::{RiHookBus, RiHookEvent, RiHookKind, RiModulePhase};
use ri::core::RiServiceContext;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
/// Tests RiModulePhase string representation with as_str().
///
/// Verifies that each module phase correctly returns its string
/// representation for logging and display purposes.
///
/// ## Phase String Mappings
///
/// - Init -> "init"
/// - BeforeStart -> "before_start"
/// - Start -> "start"
/// - AfterStart -> "after_start"
/// - BeforeShutdown -> "before_shutdown"
/// - Shutdown -> "shutdown"
/// - AfterShutdown -> "after_shutdown"
/// - Async variants have "async_" prefix
///
/// ## Expected Behavior
///
/// Each phase returns the correct string representation
fn test_module_phase_as_str() {
    assert_eq!(RiModulePhase::Init.as_str(), "init");
    assert_eq!(RiModulePhase::BeforeStart.as_str(), "before_start");
    assert_eq!(RiModulePhase::Start.as_str(), "start");
    assert_eq!(RiModulePhase::AfterStart.as_str(), "after_start");
    assert_eq!(RiModulePhase::BeforeShutdown.as_str(), "before_shutdown");
    assert_eq!(RiModulePhase::Shutdown.as_str(), "shutdown");
    assert_eq!(RiModulePhase::AfterShutdown.as_str(), "after_shutdown");
    assert_eq!(RiModulePhase::AsyncInit.as_str(), "async_init");
    assert_eq!(RiModulePhase::AsyncBeforeStart.as_str(), "async_before_start");
    assert_eq!(RiModulePhase::AsyncStart.as_str(), "async_start");
    assert_eq!(RiModulePhase::AsyncAfterStart.as_str(), "async_after_start");
    assert_eq!(RiModulePhase::AsyncBeforeShutdown.as_str(), "async_before_shutdown");
    assert_eq!(RiModulePhase::AsyncShutdown.as_str(), "async_shutdown");
    assert_eq!(RiModulePhase::AsyncAfterShutdown.as_str(), "async_after_shutdown");
}

#[test]
/// Tests RiHookBus creation with new().
///
/// Verifies that a hook bus can be created successfully and
/// is ready for hook registration and emission.
///
/// ## Expected Behavior
///
/// - Hook bus is created without errors
/// - The bus is ready for use
fn test_hook_bus_new() {
    let hook_bus = RiHookBus::new();
    // Just test that creation works without panicking
}

#[test]
/// Tests RiHookBus hook registration and emission.
///
/// Verifies that hooks can be registered with the bus and that
/// registered hooks are executed when the corresponding event
/// is emitted.
///
/// ## Hook Registration and Execution
///
/// 1. A handler is registered for a specific hook kind
/// 2. When emit() is called for that kind, the handler executes
/// 3. The handler receives the service context and event details
///
/// ## Expected Behavior
///
/// - Handler is successfully registered
/// - Emitting the event triggers the handler
/// - The handler execution flag is set to true
fn test_hook_bus_register_emit() {
    let mut hook_bus = RiHookBus::new();
    let ctx = RiServiceContext::new_default().unwrap();
    
    // Test registration and emission
    let called = Arc::new(AtomicBool::new(false));
    let called_handle = Arc::clone(&called);
    
    hook_bus.register(RiHookKind::Startup, "test_hook".to_string(), move |_ctx, _event| {
        called_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&RiHookKind::Startup, &ctx).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
}

#[test]
/// Tests RiHookBus hook emission with module and phase context.
///
/// Verifies that emit_with() correctly passes module name and
/// lifecycle phase information to registered handlers.
///
/// ## Extended Emission
///
/// The emit_with() method provides additional context:
/// - **module**: The name of the module emitting the hook
/// - **phase**: The lifecycle phase during which the hook is emitted
///
/// ## Expected Behavior
///
/// - Handler receives the event with correct kind
/// - Event contains the specified module name
/// - Event contains the specified lifecycle phase
fn test_hook_bus_register_emit_with() {
    let mut hook_bus = RiHookBus::new();
    let ctx = RiServiceContext::new_default().unwrap();
    
    // Test registration and emission with module and phase
    let called = Arc::new(AtomicBool::new(false));
    let captured_event: Arc<Mutex<Option<RiHookEvent>>> = Arc::new(Mutex::new(None));
    let called_handle = Arc::clone(&called);
    let captured_handle: Arc<Mutex<Option<RiHookEvent>>> = Arc::clone(&captured_event);
    
    hook_bus.register(RiHookKind::Startup, "test_hook".to_string(), move |_ctx, event| {
        called_handle.store(true, Ordering::SeqCst);
        *captured_handle.lock().unwrap() = Some(event.clone());
        Ok(())
    });
    
    // Emit the hook with module and phase
    hook_bus.emit_with(
        &RiHookKind::Startup, 
        &ctx, 
        Some("test_module"), 
        Some(RiModulePhase::Init)
    ).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
    
    // Verify the event was captured correctly
    let captured = {
        let guard = captured_event.lock().unwrap();
        guard.clone()
    };
    if let Some(event) = captured {
        assert_eq!(event.kind, RiHookKind::Startup);
        assert_eq!(event.module, Some("test_module".to_string()));
        assert_eq!(event.phase, Some(RiModulePhase::Init));
    } else {
        // Event was not captured, this should fail the test gracefully
        assert!(false, "Event was not captured");
    }
}

#[test]
/// Tests RiHookBus multiple handlers for the same hook kind.
///
/// Verifies that multiple handlers can be registered for the same
/// hook kind and all are executed when the event is emitted.
///
/// ## Multiple Handler Behavior
///
/// - Handlers are registered with unique identifiers
/// - All handlers for a hook kind are executed
/// - Execution order matches registration order
///
/// ## Expected Behavior
///
/// - Both handlers are registered successfully
/// - Both handlers are executed when the event is emitted
fn test_hook_bus_multiple_handlers() {
    let mut hook_bus = RiHookBus::new();
    let ctx = RiServiceContext::new_default().unwrap();
    
    // Test multiple handlers for the same hook kind
    let called1 = Arc::new(AtomicBool::new(false));
    let called2 = Arc::new(AtomicBool::new(false));
    let called1_handle = Arc::clone(&called1);
    let called2_handle = Arc::clone(&called2);
    
    hook_bus.register(RiHookKind::Startup, "test_hook1".to_string(), move |_ctx, _event| {
        called1_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(RiHookKind::Startup, "test_hook2".to_string(), move |_ctx, _event| {
        called2_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&RiHookKind::Startup, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(called1.load(Ordering::SeqCst));
    assert!(called2.load(Ordering::SeqCst));
}

#[test]
/// Tests RiHookBus filtering by hook kind.
///
/// Verifies that hooks are correctly filtered by their kind,
/// ensuring only handlers for the emitted kind are executed.
///
/// ## Kind-Based Filtering
///
/// - Handlers registered for Startup only execute on Startup emit
/// - Handlers registered for Shutdown only execute on Shutdown emit
/// - Different hook kinds are completely independent
///
/// ## Expected Behavior
///
/// - Startup handlers only execute on Startup emit
/// - Shutdown handlers only execute on Shutdown emit
/// - No cross-contamination between hook kinds
fn test_hook_bus_different_hook_kinds() {
    let mut hook_bus = RiHookBus::new();
    let ctx = RiServiceContext::new_default().unwrap();
    
    // Test different hook kinds
    let startup_called = Arc::new(AtomicBool::new(false));
    let shutdown_called = Arc::new(AtomicBool::new(false));
    let startup_handle = Arc::clone(&startup_called);
    let shutdown_handle = Arc::clone(&shutdown_called);
    
    hook_bus.register(RiHookKind::Startup, "startup_hook".to_string(), move |_ctx, _event| {
        startup_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(RiHookKind::Shutdown, "shutdown_hook".to_string(), move |_ctx, _event| {
        shutdown_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit startup hook
    hook_bus.emit(&RiHookKind::Startup, &ctx).unwrap();
    
    // Verify only startup hook was called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(!shutdown_called.load(Ordering::SeqCst));
    
    // Emit shutdown hook
    hook_bus.emit(&RiHookKind::Shutdown, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(shutdown_called.load(Ordering::SeqCst));
}
