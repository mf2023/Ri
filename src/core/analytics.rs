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

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::{DMSResult, DMSServiceContext, DMSError, _CServiceModule};
use crate::hooks::{DMSHookBus, DMSHookEvent, DMSHookKind};
use serde_json::json;

#[derive(Default)]
struct _CAnalyticsState {
    total_events: u64,
    per_kind: HashMap<String, u64>,
    per_phase: HashMap<String, u64>,
    per_module: HashMap<String, u64>,
}

pub struct DMSLogAnalyticsModule {
    state: Arc<Mutex<_CAnalyticsState>>,
    enabled: bool,
}

impl DMSLogAnalyticsModule {
    pub fn _Fnew() -> Self {
        DMSLogAnalyticsModule {
            state: Arc::new(Mutex::new(_CAnalyticsState::default())),
            enabled: true,
        }
    }

    fn _Fall_kinds() -> &'static [DMSHookKind] {
        use DMSHookKind::*;
        const KINDS: [DMSHookKind; 8] = [
            Startup,
            Shutdown,
            BeforeModulesInit,
            AfterModulesInit,
            BeforeModulesStart,
            AfterModulesStart,
            BeforeModulesShutdown,
            AfterModulesShutdown,
        ];
        &KINDS
    }

    fn _Fkind_label(kind: DMSHookKind) -> &'static str {
        match kind {
            DMSHookKind::Startup => "Startup",
            DMSHookKind::Shutdown => "Shutdown",
            DMSHookKind::BeforeModulesInit => "BeforeModulesInit",
            DMSHookKind::AfterModulesInit => "AfterModulesInit",
            DMSHookKind::BeforeModulesStart => "BeforeModulesStart",
            DMSHookKind::AfterModulesStart => "AfterModulesStart",
            DMSHookKind::BeforeModulesShutdown => "BeforeModulesShutdown",
            DMSHookKind::AfterModulesShutdown => "AfterModulesShutdown",
        }
    }

    fn _Fregister_handlers(&self, hooks: &mut DMSHookBus) {
        for kind in Self::_Fall_kinds() {
            let state = self.state.clone();
            let id = format!("dms.analytics.{}", Self::_Fkind_label(*kind));
            hooks._Fregister(*kind, id, move |_ctx, event: &DMSHookEvent| {
                let mut guard = state
                    .lock()
                    .map_err(|_| DMSError::Other("analytics state poisoned".to_string()))?;
                guard.total_events = guard.total_events.saturating_add(1);
                let kind_label = Self::_Fkind_label(event.kind).to_string();
                *guard.per_kind.entry(kind_label).or_insert(0) += 1;
                if let Some(phase) = &event.phase {
                    *guard.per_phase.entry(phase.clone()).or_insert(0) += 1;
                }
                if let Some(module) = &event.module {
                    *guard.per_module.entry(module.clone()).or_insert(0) += 1;
                }
                Ok(())
            });
        }
    }

    fn _Fflush_summary(&self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        let snapshot = {
            let guard = self
                .state
                .lock()
                .map_err(|_| DMSError::Other("analytics state poisoned".to_string()))?;
            json!({
                "timestamp": SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map(|d| d.as_secs())
                    .unwrap_or(0),
                "total_events": guard.total_events,
                "per_kind": guard.per_kind,
                "per_phase": guard.per_phase,
                "per_module": guard.per_module,
            })
        };

        let fs = ctx._Ffs();
        let output = fs._Fobservability_dir().join("lifecycle_analytics.json");
        fs._Fwrite_json(&output, &snapshot)?;
        let logger = ctx._Flogger();
        let _ = logger._Finfo("DMS.LogAnalytics", format!("summary_path={}", output.display()));
        Ok(())
    }
}

impl _CServiceModule for DMSLogAnalyticsModule {
    fn _Fname(&self) -> &str {
        "DMS.LogAnalytics"
    }

    fn _Fis_critical(&self) -> bool {
        false
    }

    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        let cfg = ctx._Fconfig()._Fconfig();
        self.enabled = cfg._Fget_bool("analytics.enabled").unwrap_or(true);
        if !self.enabled {
            return Ok(());
        }
        let hooks: &mut DMSHookBus = ctx._Fhooks_mut();
        self._Fregister_handlers(hooks);
        Ok(())
    }

    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if !self.enabled {
            return Ok(());
        }
        self._Fflush_summary(ctx)
    }
}
