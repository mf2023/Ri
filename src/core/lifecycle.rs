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

use crate::core::{DMSResult, DMSServiceContext, _CServiceModule};
use crate::hooks::{DMSHookBus, DMSHookEvent, DMSHookKind};

pub struct DMSLifecycleObserver;

impl DMSLifecycleObserver {
    pub fn _Fnew() -> Self {
        DMSLifecycleObserver
    }
}

impl _CServiceModule for DMSLifecycleObserver {
    fn _Fname(&self) -> &str {
        "DMS.LifecycleObserver"
    }

    fn _Fis_critical(&self) -> bool {
        false
    }

    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        let hooks: &mut DMSHookBus = ctx._Fhooks_mut();

        hooks._Fregister(DMSHookKind::Startup, "dms.lifecycle.startup".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::Shutdown, "dms.lifecycle.shutdown".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::BeforeModulesInit, "dms.lifecycle.before_init".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::AfterModulesInit, "dms.lifecycle.after_init".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::BeforeModulesStart, "dms.lifecycle.before_start".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::AfterModulesStart, "dms.lifecycle.after_start".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::BeforeModulesShutdown, "dms.lifecycle.before_shutdown".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        hooks._Fregister(DMSHookKind::AfterModulesShutdown, "dms.lifecycle.after_shutdown".to_string(), |_ctx, event: &DMSHookEvent| {
            let logger = _ctx._Flogger();
            let module = event.module.as_deref().unwrap_or("-");
            let phase = event.phase.as_deref().unwrap_or("-");
            let kind = match event.kind {
                DMSHookKind::Startup => "Startup",
                DMSHookKind::Shutdown => "Shutdown",
                DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
                DMSHookKind::AfterModulesInit => "AfterModulesInit",
                DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
                DMSHookKind::AfterModulesStart => "AfterModulesStart",
                DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
                DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
            };
            let message = format!("kind={} module={} phase={}", kind, module, phase);
            let _ = logger._Finfo("DMS.Lifecycle", message);
            Ok(())
        });

        Ok(())
    }
}
