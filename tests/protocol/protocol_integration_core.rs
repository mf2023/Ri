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

use dms::protocol::integration::core::*;
use dms::core::{DMSCResult, DMSCError, DMSCServiceContext};
use dms::hooks::{DMSCHookKind, DMSCModulePhase};
use dms::protocol::global_state::{DMSCSystemStatus, DMSCGlobalStateManager, DMSCStateUpdate};
use std::sync::Arc as StdArc;
use std::sync::atomic::{AtomicBool, Ordering};

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