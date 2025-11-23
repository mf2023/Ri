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

// Hooks module for DMS: event bus for lifecycle and future extension points.

use std::collections::HashMap;

use crate::core::{DMSResult, DMSServiceContext};

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSHookKind {
    Startup,
    Shutdown,
    BeforeModulesInit,
    AfterModulesInit,
    BeforeModulesStart,
    AfterModulesStart,
    BeforeModulesShutdown,
    AfterModulesShutdown,
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub enum DMSModulePhase {
    Init,
    BeforeStart,
    Start,
    AfterStart,
    BeforeShutdown,
    Shutdown,
    AfterShutdown,
    AsyncInit,
    AsyncBeforeStart,
    AsyncStart,
    AsyncAfterStart,
    AsyncBeforeShutdown,
    AsyncShutdown,
    AsyncAfterShutdown,
}

impl DMSModulePhase {
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

pub struct DMSHookEvent {
    pub kind: DMSHookKind,
    pub module: Option<String>,
    pub phase: Option<DMSModulePhase>,
}

pub type DMSHookId = String;

pub struct DMSHookBus {
    handlers: HashMap<DMSHookKind, Vec<(DMSHookId, Box<dyn Fn(&DMSServiceContext, &DMSHookEvent) -> DMSResult<()> + Send + Sync>)>>,
}

impl DMSHookBus {
    pub fn _Fnew() -> Self {
        DMSHookBus { handlers: HashMap::new() }
    }

    pub fn _Fregister<F>(&mut self, kind: DMSHookKind, id: DMSHookId, handler: F)
    where
        F: Fn(&DMSServiceContext, &DMSHookEvent) -> DMSResult<()> + Send + Sync + 'static,
    {
        self.handlers.entry(kind).or_default().push((id, Box::new(handler)));
    }

    pub fn _Femit(&self, kind: &DMSHookKind, ctx: &DMSServiceContext) -> DMSResult<()> {
        self._Femit_with(kind, ctx, None, None)
    }

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
