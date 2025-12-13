// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use dms_core::hooks::{DMSHookBus, DMSHookEvent, DMSHookKind, DMSModulePhase};
use dms_core::core::DMSServiceContext;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn test_module_phase_as_str() {
    assert_eq!(DMSModulePhase::Init.as_str(), "init");
    assert_eq!(DMSModulePhase::BeforeStart.as_str(), "before_start");
    assert_eq!(DMSModulePhase::Start.as_str(), "start");
    assert_eq!(DMSModulePhase::AfterStart.as_str(), "after_start");
    assert_eq!(DMSModulePhase::BeforeShutdown.as_str(), "before_shutdown");
    assert_eq!(DMSModulePhase::Shutdown.as_str(), "shutdown");
    assert_eq!(DMSModulePhase::AfterShutdown.as_str(), "after_shutdown");
    assert_eq!(DMSModulePhase::AsyncInit.as_str(), "async_init");
    assert_eq!(DMSModulePhase::AsyncBeforeStart.as_str(), "async_before_start");
    assert_eq!(DMSModulePhase::AsyncStart.as_str(), "async_start");
    assert_eq!(DMSModulePhase::AsyncAfterStart.as_str(), "async_after_start");
    assert_eq!(DMSModulePhase::AsyncBeforeShutdown.as_str(), "async_before_shutdown");
    assert_eq!(DMSModulePhase::AsyncShutdown.as_str(), "async_shutdown");
    assert_eq!(DMSModulePhase::AsyncAfterShutdown.as_str(), "async_after_shutdown");
}

#[test]
fn test_hook_bus_new() {
    let hook_bus = DMSHookBus::new();
    // Just test that creation works without panicking
}

#[test]
fn test_hook_bus_register_emit() {
    let mut hook_bus = DMSHookBus::new();
    let ctx = DMSServiceContext::new_default().unwrap();
    
    // Test registration and emission
    let called = Arc::new(AtomicBool::new(false));
    let called_handle = Arc::clone(&called);
    
    hook_bus.register(DMSHookKind::Startup, "test_hook".to_string(), move |_ctx, _event| {
        called_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&DMSHookKind::Startup, &ctx).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
}

#[test]
fn test_hook_bus_register_emit_with() {
    let mut hook_bus = DMSHookBus::new();
    let ctx = DMSServiceContext::new_default().unwrap();
    
    // Test registration and emission with module and phase
    let called = Arc::new(AtomicBool::new(false));
    let captured_event: Arc<Mutex<Option<DMSHookEvent>>> = Arc::new(Mutex::new(None));
    let called_handle = Arc::clone(&called);
    let captured_handle: Arc<Mutex<Option<DMSHookEvent>>> = Arc::clone(&captured_event);
    
    hook_bus.register(DMSHookKind::Startup, "test_hook".to_string(), move |_ctx, event| {
        called_handle.store(true, Ordering::SeqCst);
        *captured_handle.lock().unwrap() = Some(event.clone());
        Ok(())
    });
    
    // Emit the hook with module and phase
    hook_bus.emit_with(
        &DMSHookKind::Startup, 
        &ctx, 
        Some("test_module"), 
        Some(DMSModulePhase::Init)
    ).unwrap();
    
    // Verify the hook was called
    assert!(called.load(Ordering::SeqCst));
    
    // Verify the event was captured correctly
    let captured = {
        let guard = captured_event.lock().unwrap();
        guard.clone()
    };
    if let Some(event) = captured {
        assert_eq!(event.kind, DMSHookKind::Startup);
        assert_eq!(event.module, Some("test_module".to_string()));
        assert_eq!(event.phase, Some(DMSModulePhase::Init));
    } else {
        // Event was not captured, this should fail the test gracefully
        assert!(false, "Event was not captured");
    }
}

#[test]
fn test_hook_bus_multiple_handlers() {
    let mut hook_bus = DMSHookBus::new();
    let ctx = DMSServiceContext::new_default().unwrap();
    
    // Test multiple handlers for the same hook kind
    let called1 = Arc::new(AtomicBool::new(false));
    let called2 = Arc::new(AtomicBool::new(false));
    let called1_handle = Arc::clone(&called1);
    let called2_handle = Arc::clone(&called2);
    
    hook_bus.register(DMSHookKind::Startup, "test_hook1".to_string(), move |_ctx, _event| {
        called1_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(DMSHookKind::Startup, "test_hook2".to_string(), move |_ctx, _event| {
        called2_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit the hook
    hook_bus.emit(&DMSHookKind::Startup, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(called1.load(Ordering::SeqCst));
    assert!(called2.load(Ordering::SeqCst));
}

#[test]
fn test_hook_bus_different_hook_kinds() {
    let mut hook_bus = DMSHookBus::new();
    let ctx = DMSServiceContext::new_default().unwrap();
    
    // Test different hook kinds
    let startup_called = Arc::new(AtomicBool::new(false));
    let shutdown_called = Arc::new(AtomicBool::new(false));
    let startup_handle = Arc::clone(&startup_called);
    let shutdown_handle = Arc::clone(&shutdown_called);
    
    hook_bus.register(DMSHookKind::Startup, "startup_hook".to_string(), move |_ctx, _event| {
        startup_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    hook_bus.register(DMSHookKind::Shutdown, "shutdown_hook".to_string(), move |_ctx, _event| {
        shutdown_handle.store(true, Ordering::SeqCst);
        Ok(())
    });
    
    // Emit startup hook
    hook_bus.emit(&DMSHookKind::Startup, &ctx).unwrap();
    
    // Verify only startup hook was called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(!shutdown_called.load(Ordering::SeqCst));
    
    // Emit shutdown hook
    hook_bus.emit(&DMSHookKind::Shutdown, &ctx).unwrap();
    
    // Verify both hooks were called
    assert!(startup_called.load(Ordering::SeqCst));
    assert!(shutdown_called.load(Ordering::SeqCst));
}
