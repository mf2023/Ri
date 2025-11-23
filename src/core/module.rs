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

// Internal module abstraction with basic lifecycle.

use crate::core::{DMSResult, DMSServiceContext};

pub trait _CServiceModule {
    fn _Fname(&self) -> &str;

    fn _Fis_critical(&self) -> bool {
        true
    }

    fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fbefore_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fstart(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fafter_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fbefore_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fshutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}

// Async version of the service module trait
#[async_trait::async_trait]
pub trait DMSModule: Send + Sync {
    fn name(&self) -> &str;

    fn is_critical(&self) -> bool {
        true
    }

    async fn init(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn before_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn after_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn before_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}

// Async version of the service module trait
#[async_trait::async_trait]
pub trait _CAsyncServiceModule: Send + Sync {
    fn _Fname(&self) -> &str;

    fn _Fis_critical(&self) -> bool {
        true
    }

    async fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fbefore_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fstart(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fafter_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fbefore_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fshutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}
