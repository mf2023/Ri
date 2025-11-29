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

//! # Log Analytics Module
//! 
//! This module provides logging analytics functionality for DMS, tracking hook events and generating
//! comprehensive analytics reports. It implements a service module that monitors the application
//! lifecycle and generates JSON reports with event statistics.
//! 
//! ## Key Components
//! 
//! - **DMSLogAnalyticsModule**: Main analytics module that implements `_CServiceModule`
//! - **_CAnalyticsState**: Internal struct for tracking analytics data
//! 
//! ## Design Principles
//! 
//! 1. **Non-Intrusive**: Operates by listening to hook events without modifying core functionality
//! 2. **Performance-Focused**: Uses efficient data structures for event tracking
//! 3. **Configurable**: Can be enabled/disabled through configuration
//! 4. **Comprehensive**: Tracks events by kind, phase, and module
//! 5. **Persistent**: Generates JSON reports that can be analyzed later
//! 6. **Non-Critical**: Fails gracefully if analytics operations encounter errors

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::core::{DMSResult, DMSServiceContext, DMSError, _CServiceModule};
use crate::hooks::{DMSHookBus, DMSHookEvent, DMSHookKind};
use serde_json::json;

/// Internal analytics state struct.
/// 
/// This struct tracks various metrics about hook events, including:
/// - Total number of events
/// - Events per hook kind
/// - Events per module phase
/// - Events per module name
#[derive(Default)]
struct _CAnalyticsState {
    /// Total number of hook events processed
    total_events: u64,
    /// Number of events per hook kind
    per_kind: HashMap<String, u64>,
    /// Number of events per module phase
    per_phase: HashMap<String, u64>,
    /// Number of events per module name
    per_module: HashMap<String, u64>,
}

/// Log analytics module for DMS.
/// 
/// This module implements the `_CServiceModule` trait and provides analytics functionality
/// by listening to hook events and generating comprehensive reports.
/// 
/// ## Usage
/// 
/// The module is automatically added by the `DMSAppBuilder` and doesn't need to be explicitly
/// configured in most cases. It can be enabled/disabled through the configuration file.
/// 
/// ## Configuration
/// 
/// ```yaml
/// analytics:
///   enabled: true  # Enable or disable analytics
/// ```
pub struct DMSLogAnalyticsModule {
    /// Shared analytics state protected by a mutex
    state: Arc<Mutex<_CAnalyticsState>>,
    /// Whether analytics is enabled
    enabled: bool,
}

impl DMSLogAnalyticsModule {
    /// Creates a new instance of the log analytics module.
    /// 
    /// Returns a new `DMSLogAnalyticsModule` with default settings.
    pub fn _Fnew() -> Self {
        DMSLogAnalyticsModule {
            state: Arc::new(Mutex::new(_CAnalyticsState::default())),
            enabled: true,
        }
    }

    /// Returns all hook kinds that the analytics module tracks.
    /// 
    /// This method returns a static slice of all hook kinds that the analytics module
    /// registers handlers for.
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

    /// Returns a string label for the given hook kind.
    /// 
    /// This method converts a `DMSHookKind` enum variant to a human-readable string.
    /// 
    /// # Parameters
    /// 
    /// - `kind`: The hook kind to get a label for
    /// 
    /// # Returns
    /// 
    /// A static string label for the hook kind
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

    /// Registers hook handlers for all tracked hook kinds.
    /// 
    /// This method registers a handler for each hook kind that updates the analytics state
    /// whenever a hook event is triggered.
    /// 
    /// # Parameters
    /// 
    /// - `hooks`: The hook bus to register handlers with
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
                    *guard.per_phase.entry(phase.as_str().to_string()).or_insert(0) += 1;
                }
                if let Some(module) = &event.module {
                    *guard.per_module.entry(module.clone()).or_insert(0) += 1;
                }
                Ok(())
            });
        }
    }

    /// Flushes the analytics summary to a JSON file.
    /// 
    /// This method generates a JSON summary of the analytics state and writes it to a file
    /// in the observability directory.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context to use for file operations and logging
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` indicating success or failure
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
    /// Returns the name of the analytics module.
    /// 
    /// This name is used for identification, logging, and dependency resolution.
    fn _Fname(&self) -> &str {
        "DMS.LogAnalytics"
    }

    /// Indicates if the analytics module is critical to the operation of the system.
    /// 
    /// The analytics module is non-critical, meaning it can fail without causing the entire
    /// system to fail.
    fn _Fis_critical(&self) -> bool {
        false
    }

    /// Initializes the analytics module.
    /// 
    /// This method:
    /// 1. Reads the analytics configuration from the service context
    /// 2. Enables or disables the module based on configuration
    /// 3. Registers hook handlers if the module is enabled
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration and hooks
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` indicating success or failure
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

    /// Flushes analytics data after the application has shutdown.
    /// 
    /// This method generates a final analytics report and writes it to a file after
    /// all modules have been shutdown.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing file system and logging capabilities
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` indicating success or failure
    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if !self.enabled {
            return Ok(());
        }
        self._Fflush_summary(ctx)
    }
}
