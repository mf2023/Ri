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

// Service context definitions for DMS core.

use crate::fs::DMSFileSystem;
use crate::log::{DMSLogConfig, DMSLogger};
use crate::config::DMSConfigManager;
use crate::hooks::DMSHookBus;
use crate::core::DMSResult;

// Internal context implementation struct. Not exposed directly.
pub struct _CServiceContextInner {
    pub fs: DMSFileSystem,
    pub logger: DMSLogger,
    pub config: DMSConfigManager,
    pub hooks: DMSHookBus,
}

impl _CServiceContextInner {
    pub fn _Fnew(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        _CServiceContextInner { fs, logger, config, hooks }
    }
}

// Public-facing service context class.
pub struct DMSServiceContext {
    inner: _CServiceContextInner,
}

impl DMSServiceContext {
    pub fn _Fnew_with(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        let inner = _CServiceContextInner::_Fnew(fs, logger, config, hooks);
        DMSServiceContext { inner }
    }

    pub fn _Fnew_default() -> DMSResult<Self> {
        let config = DMSConfigManager::_Fnew_default();
        let cfg = config._Fconfig();

        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {}", e)))?;
        let app_data_root = if let Some(root_str) = cfg._Fget_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        let fs = DMSFileSystem::_Fnew_with_roots(project_root, app_data_root);

        let log_config = DMSLogConfig::_Ffrom_config(cfg);
        let logger = DMSLogger::_Fnew(&log_config, fs.clone());
        let hooks = DMSHookBus::_Fnew();
        Ok(DMSServiceContext::_Fnew_with(fs, logger, config, hooks))
    }

    pub fn _Ffs(&self) -> &DMSFileSystem {
        &self.inner.fs
    }

    pub fn _Flogger(&self) -> &DMSLogger {
        &self.inner.logger
    }

    pub fn _Fconfig(&self) -> &DMSConfigManager {
        &self.inner.config
    }

    pub fn _Fhooks(&self) -> &DMSHookBus {
        &self.inner.hooks
    }

    pub fn _Fhooks_mut(&mut self) -> &mut DMSHookBus {
        &mut self.inner.hooks
    }
}
