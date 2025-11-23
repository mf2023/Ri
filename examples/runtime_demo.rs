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

use DMS::prelude::*;
use DMS::core::_CServiceModule;
use DMS::log::DMSLogContext;

struct DemoModule {
    name: String,
    critical: bool,
    fail_phase: Option<&'static str>,
}

impl DemoModule {
    fn new(name: &str, critical: bool, fail_phase: Option<&'static str>) -> Self {
        DemoModule {
            name: name.to_string(),
            critical,
            fail_phase,
        }
    }

    fn should_fail(&self, phase: &str) -> bool {
        self.fail_phase.map(|p| p == phase).unwrap_or(false)
    }

    fn log_phase(&self, ctx: &mut DMSServiceContext, phase: &str) -> DMSResult<()> {
        let logger = ctx._Flogger();
        DMSLogContext::_Fput("module", self.name.clone());
        DMSLogContext::_Fput("phase", phase.to_string());
        logger._Finfo("DMS.RuntimeDemo", format!("{}: phase={} ok", self.name, phase))
    }

    fn fail(&self, phase: &str) -> DMSResult<()> {
        Err(DMSError::Other(format!("{} forced failure at {}", self.name, phase)))
    }
}

impl _CServiceModule for DemoModule {
    fn _Fname(&self) -> &str {
        &self.name
    }

    fn _Fis_critical(&self) -> bool {
        self.critical
    }

    fn _Fbefore_start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("before_start") {
            return self.fail("before_start");
        }
        self.log_phase(ctx, "before_start")
    }

    fn _Fstart(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("start") {
            return self.fail("start");
        }
        self.log_phase(ctx, "start")
    }

    fn _Fafter_start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("after_start") {
            return self.fail("after_start");
        }
        self.log_phase(ctx, "after_start")
    }

    fn _Fbefore_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("before_shutdown") {
            return self.fail("before_shutdown");
        }
        self.log_phase(ctx, "before_shutdown")
    }

    fn _Fshutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("shutdown") {
            return self.fail("shutdown");
        }
        self.log_phase(ctx, "shutdown")
    }

    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        if self.should_fail("after_shutdown") {
            return self.fail("after_shutdown");
        }
        self.log_phase(ctx, "after_shutdown")
    }
}

fn main() -> DMSResult<()> {
    let runtime = DMSAppBuilder::_Fnew()
        ._Fwith_module(Box::new(DemoModule::new("DMS.ModuleAlpha", true, None)))
        ._Fwith_module(Box::new(DemoModule::new("DMS.ModuleBeta", false, Some("start"))))
        ._Fbuild()?;

    match runtime._Frun() {
        Ok(()) => {
            println!("Runtime finished successfully");
        }
        Err(err) => {
            println!("Runtime stopped with error: {}", err);
        }
    }

    Ok(())
}
