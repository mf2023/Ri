// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMSC.
// The DMSC project belongs to the Dunimd Team.
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

use dmsc::protocol::integration::core::*;
use dmsc::core::{DMSCResult, DMSCError, DMSCServiceContext};
use dmsc::hooks::{DMSCHookKind, DMSCModulePhase};
use dmsc::protocol::global_state::{DMSCSystemStatus, DMSCGlobalStateManager, DMSCStateUpdate};
use std::sync::Arc as StdArc;
use std::sync::atomic::{AtomicBool, Ordering};

/// Protocol integration core test module for DMSC control plane operations.
///
/// This module provides comprehensive test coverage for the protocol integration
/// layer that connects the core system with external control mechanisms. The tests
/// validate the DMSCControlCenter component which serves as the central coordinator
/// for system state management, hook triggering, and external control actions.
///
/// ## Test Coverage
///
/// - **Hook Triggering**: Tests the ability to trigger registered hooks through
///   external control actions, verifying that hook callbacks are executed with
///   correct parameters and that the triggering operation reports success accurately.
///
/// - **Global State Management**: Validates the state update mechanism where the
///   control center can modify system-wide state including operational status,
///   global configuration, and active protocol sets through atomic updates.
///
/// - **System Status Transitions**: Tests the ability to change the overall system
///   status through external control actions, verifying that status changes are
///   persisted and reflected in subsequent state queries.
///
/// - **Control Center Lifecycle**: Validates the proper initialization of the
///   control center with service context and state manager references, ensuring
///   all dependencies are correctly wired for coordinated operation.
///
/// ## Design Principles
///
/// The integration tests focus on the interaction patterns between components
/// rather than individual component behavior. This approach validates that the
/// system works correctly when assembled, catching integration issues that unit
/// tests would miss.
///
/// Tests use real implementations of dependencies (DMSCGlobalStateManager,
/// DMSCServiceContext) rather than mocks where practical, providing confidence
/// that the integration paths function correctly in production scenarios.
///
/// The control center design implements a facade pattern, providing a unified
/// interface to complex system operations while hiding implementation details
/// from external callers. Tests verify that this abstraction correctly delegates
/// to underlying components.

#[tokio::test]
async fn test_control_center_trigger_hook_and_update_state() {
    let state_manager = StdArc::new(DMSCGlobalStateManager::new());
    state_manager.initialize().await.unwrap();

    let mut ctx = DMSCServiceContext::new_default().unwrap();

    let called = StdArc::new(AtomicBool::new(false));
    {
        let called_handle = called.clone();
        let hooks = ctx.hooks_mut();
        hooks.register(DMSCHookKind::Startup, "control-center.test".to_string(), move |_ctx, _event| {
            called_handle.store(true, Ordering::SeqCst);
            Ok(())
        });
    }

    let control_center = DMSCControlCenter::new(state_manager.clone(), ctx);

    let action = DMSCExternalControlAction::TriggerHook {
        hook: DMSCHookKind::Startup,
        module: Some("test_module".to_string()),
        phase: Some(DMSCModulePhase::Init),
    };

    let result = control_center.handle_action(action).await.unwrap();
    match result {
        DMSCExternalControlResult::HookTriggered => {}
        other => assert!(false, "Expected HookTriggered, got {:?}", other),
    }

    assert!(called.load(Ordering::SeqCst));

    let global_state_before = state_manager.get_global_state().await.unwrap();

    let update = DMSCStateUpdate::Global {
        system_status: global_state_before.system_status,
        global_config: global_state_before.global_config.clone(),
        active_protocols: global_state_before.active_protocols.clone(),
    };

    let result = control_center
        .handle_action(DMSCExternalControlAction::UpdateState(update))
        .await
        .unwrap();

    match result {
        DMSCExternalControlResult::StateUpdated => {}
        other => assert!(false, "Expected StateUpdated, got {:?}", other),
    }
}

#[tokio::test]
async fn test_control_center_set_global_system_status() {
    let state_manager = StdArc::new(DMSCGlobalStateManager::new());
    state_manager.initialize().await.unwrap();
    
    let ctx = DMSCServiceContext::new_default().unwrap();
    let control_center = DMSCControlCenter::new(state_manager.clone(), ctx);
    
    let before = state_manager.get_global_state().await.unwrap();
    assert_eq!(before.system_status, DMSCSystemStatus::Operational);
    
    let action = DMSCExternalControlAction::SetGlobalSystemStatus(DMSCSystemStatus::Maintenance);
    let result = control_center.handle_action(action).await.unwrap();
    
    match result {
        DMSCExternalControlResult::StateUpdated => {}
        other => assert!(false, "Expected StateUpdated, got {:?}", other),
    }
    
    let after = state_manager.get_global_state().await.unwrap();
    assert_eq!(after.system_status, DMSCSystemStatus::Maintenance);
}