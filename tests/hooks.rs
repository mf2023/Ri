//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
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

//! # Hooks Module Tests
//!
//! This module contains comprehensive tests for the DMSC hooks system, a publish-subscribe
//! mechanism for lifecycle event handling that enables modular extension and integration
//! of system components through event-driven coordination.
//!
//! ## Test Coverage
//!
//! - **DMSCHookBus**: Tests for the central event bus that manages hook registration,
//!   emission, and propagation. The bus supports both simple emission and detailed
//!   emission with module/phase context
//!
//! - **DMSCHookEvent**: Tests for event data structure including event kind, timestamp,
//!   associated module name, and lifecycle phase information
//!
//! - **DMSCHookKind**: Tests for event type classification including Startup, Shutdown,
//!   and custom event kinds for different system integration points
//!
//! - **DMSCModulePhase**: Tests for lifecycle phase enumeration covering both synchronous
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

use dmsc::hooks::{DMSCHookBus, DMSCHookEvent, DMSCHookKind, DMSCModulePhase};
use dmsc::core::DMSCServiceContext;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn test_module_phase_as_str() {
    assert_eq!(DMSCModulePhase::Init.as_str(), "init");
    assert_eq!(DMSCModulePhase::BeforeStart.as_str(), "before_start");
    assert_eq!(DMSCModulePhase::Start.as_str(), "start");
    assert_eq!(DMSCModulePhase::AfterStart.as_str(), "after_start");
    assert_eq!(DMSCModulePhase::BeforeShutdown.as_str(), "before_shutdown");
    assert_eq!(DMSCModulePhase::Shutdown.as_str(), "shutdown");
    assert_eq!(DMSCModulePhase::AfterShutdown.as_str(), "after_shutdown");
    assert_eq!(DMSCModulePhase::AsyncInit.as_str(), "async_init");
    assert_eq!(DMSCModulePhase::AsyncBeforeStart.as_str(), "async_before_start");
    assert_eq!(DMSCModulePhase::AsyncStart.as_str(), "async_start");
    assert_eq!(DMSCModulePhase::AsyncAfterStart.as_str(), "async_after_start");
    assert_eq!(DMSCModulePhase::AsyncBeforeShutdown.as_str(), "async_before_shutdown");
    assert_eq!(DMSCModulePhase::AsyncShutdown.as_str(), "async_shutdown");
    assert_eq!(DMSCModulePhase::AsyncAfterShutdown.as_str(), "async_after_shutdown");
}

#[test]
fn test_hook_bus_new() {
    let hook_bus = DMSCHookBus::new();
    // Just test that creation works without panicking
}

#[test]
fn test_hook_bus_register_emit() {
    let mut hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new_default().unwrap();
    
    // Test registration and emission
    let called = Arc::new(AtomicBool::new(false));
    let called_handle = Arc::clone(&called);
    
    hook_bus.register(DMSCHookKind::Startup, "test_hook".to_string(), move |_ctx, _event| {
        called_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&DMSCHookKind::Startup, &ctx).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_hook_bus_register_emit_with() {
    let mut hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new_default().unwrap();
    
    // Test registration and emission with module and phase
    let called = Arc::new(AtomicBool::new(false));
    let captured_event: Arc<Mutex<Option<DMSCHookEvent>>> = Arc::new(Mutex::new(None));
    let called_handle = Arc::clone(&called);
    let captured_handle: Arc<Mutex<Option<DMSCHookEvent>>> = Arc::clone(&captured_event);
    
    hook_bus.register(DMSCHookKind::Startup, "test_hook".to_string(), move |_ctx, event| {
        called_handle.store(true, Ordering::SeqCst);
        *captured_handle.lock().unwrap() = Some(event.clone());
        Ok(())
    });
    
    // Emit the hook with module and phase
    hook_bus.emit_with(
        &DMSCHookKind::Startup, 
        &ctx, 
        Some("test_module"), 
        Some(DMSCModulePhase::Init)
    ).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
    
    // Verify the event was captured correctly
    let captured = {
        let guard = captured_event.lock().unwrap();
        guard.clone()
    };
    if let Some(event) = captured {
        assert_eq!(event.kind, DMSCHookKind::Startup);
        assert_eq!(event.module, Some("test_module".to_string()));
        assert_eq!(event.phase, Some(DMSCModulePhase::Init));
    } else {
        // Event was not captured, this should fail the test gracefully
        assert!(false, "Event was not captured");
    }
}

#[test]
fn test_hook_bus_multiple_handlers() {
    let mut hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new_default().unwrap();
    
    // Test multiple handlers for the same hook kind
    let called1 = Arc::new(AtomicBool::new(false));
    let called2 = Arc::new(AtomicBool::new(false));
    let called1_handle = Arc::clone(&called1);
    let called2_handle = Arc::clone(&called2);
    
    hook_bus.register(DMSCHookKind::Startup, "test_hook1".to_string(), move |_ctx, _event| {
        called1_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(DMSCHookKind::Startup, "test_hook2".to_string(), move |_ctx, _event| {
        called2_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&DMSCHookKind::Startup, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(called1.load(Ordering::SeqCst));
    assert!(called2.load(Ordering::SeqCst));
}

#[test]
fn test_hook_bus_different_hook_kinds() {
    let mut hook_bus = DMSCHookBus::new();
    let ctx = DMSCServiceContext::new_default().unwrap();
    
    // Test different hook kinds
    let startup_called = Arc::new(AtomicBool::new(false));
    let shutdown_called = Arc::new(AtomicBool::new(false));
    let startup_handle = Arc::clone(&startup_called);
    let shutdown_handle = Arc::clone(&shutdown_called);
    
    hook_bus.register(DMSCHookKind::Startup, "startup_hook".to_string(), move |_ctx, _event| {
        startup_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(DMSCHookKind::Shutdown, "shutdown_hook".to_string(), move |_ctx, _event| {
        shutdown_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit startup hook
    hook_bus.emit(&DMSCHookKind::Startup, &ctx).unwrap();
    
    // Verify only startup hook was called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(!shutdown_called.load(Ordering::SeqCst));
    
    // Emit shutdown hook
    hook_bus.emit(&DMSCHookKind::Shutdown, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(shutdown_called.load(Ordering::SeqCst));
}
