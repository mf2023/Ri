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

//! # Service Context
//! 
//! The service context provides access to all core functionalities of DMS,
//! acting as a central hub for accessing various components such as logging,
//! configuration, file system, and hooks.

#![allow(non_snake_case)]

use crate::fs::DMSFileSystem;
use crate::log::{DMSLogConfig, DMSLogger};
use crate::config::DMSConfigManager;
use crate::hooks::DMSHookBus;
use crate::core::DMSResult;

/// Internal service context implementation. Not exposed directly to users.
/// 
/// This struct contains all the core components of the service context,
/// but is wrapped by `DMSServiceContext` for controlled access.
pub struct _CServiceContextInner {
    /// File system accessor for secure file operations
    pub fs: DMSFileSystem,
    /// Logger for structured logging
    pub logger: DMSLogger,
    /// Configuration manager for accessing application settings
    pub config: DMSConfigManager,
    /// Hook bus for emitting and handling lifecycle events
    pub hooks: DMSHookBus,
}

impl _CServiceContextInner {
    /// Create a new `_CServiceContextInner` instance with the provided components.
    /// 
    /// # Parameters
    /// 
    /// - `fs`: File system accessor
    /// - `logger`: Structured logger
    /// - `config`: Configuration manager
    /// - `hooks`: Hook bus for lifecycle events
    /// 
    /// # Returns
    /// 
    /// A new `_CServiceContextInner` instance.
    pub fn _Fnew(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        _CServiceContextInner { fs, logger, config, hooks }
    }
}

/// Public-facing service context for DMS applications.
/// 
/// The `DMSServiceContext` is the primary way for modules and business logic to
/// access core DMS functionalities. It follows the dependency injection pattern,
/// providing a centralized access point to all core components.
/// 
/// ## Design Principle
/// 
/// The service context is designed to be immutable from the outside, with controlled
/// access to mutable components through dedicated methods. This ensures thread safety
/// while allowing for necessary mutations in specific contexts.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::prelude::*;
/// 
/// async fn handle_request(ctx: &DMSServiceContext) -> DMSResult<()> {
///     // Access logger
///     ctx._Flogger()._Finfo("request", "Handling request");
///     
///     // Access configuration
///     let config_value = ctx._Fconfig()._Fconfig()._Fget_str("app.name");
///     
///     // Access file system
///     let file_path = ctx._Ffs()._Fapp_data_path("logs/app.log");
///     
///     Ok(())
/// }
/// ```
pub struct DMSServiceContext {
    /// Internal implementation details
    inner: _CServiceContextInner,
}

impl DMSServiceContext {
    /// Create a new `DMSServiceContext` with the provided components.
    /// 
    /// This method is typically used by the framework itself during application startup,
    /// but can be used for testing or custom initialization.
    /// 
    /// # Parameters
    /// 
    /// - `fs`: File system accessor
    /// - `logger`: Structured logger
    /// - `config`: Configuration manager
    /// - `hooks`: Hook bus for lifecycle events
    /// 
    /// # Returns
    /// 
    /// A new `DMSServiceContext` instance.
    pub fn _Fnew_with(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        let inner = _CServiceContextInner::_Fnew(fs, logger, config, hooks);
        DMSServiceContext { inner }
    }

    /// Create a new `DMSServiceContext` with default configuration.
    /// 
    /// This is the most common way to create a service context, as it handles
    /// the initialization of all core components automatically.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult` containing the new service context, or an error if initialization failed.
    /// 
    /// # Errors
    /// 
    /// - If the project root directory cannot be determined
    /// - If there are issues initializing any of the core components
    pub fn _Fnew_default() -> DMSResult<Self> {
        // Create default configuration manager
        let config = DMSConfigManager::_Fnew_default();
        let cfg = config._Fconfig();

        // Determine project root directory
        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {e}")))?;
        
        // Determine application data root directory
        let app_data_root = if let Some(root_str) = cfg._Fget_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        // Initialize file system
        let fs = DMSFileSystem::_Fnew_with_roots(project_root, app_data_root);

        // Initialize logging
        let log_config = DMSLogConfig::_Ffrom_config(cfg);
        let logger = DMSLogger::_Fnew(&log_config, fs.clone());
        
        // Initialize hook bus
        let hooks = DMSHookBus::_Fnew();
        
        Ok(DMSServiceContext::_Fnew_with(fs, logger, config, hooks))
    }

    /// Get a reference to the file system accessor.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSFileSystem` instance.
    pub fn _Ffs(&self) -> &DMSFileSystem {
        &self.inner.fs
    }

    /// Get a reference to the structured logger.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSLogger` instance.
    pub fn _Flogger(&self) -> &DMSLogger {
        &self.inner.logger
    }

    /// Get a reference to the configuration manager.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSConfigManager` instance.
    pub fn _Fconfig(&self) -> &DMSConfigManager {
        &self.inner.config
    }

    /// Get a reference to the hook bus for emitting events.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSHookBus` instance.
    pub fn _Fhooks(&self) -> &DMSHookBus {
        &self.inner.hooks
    }

    /// Get a mutable reference to the hook bus for registering handlers.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSHookBus` instance.
    pub fn _Fhooks_mut(&mut self) -> &mut DMSHookBus {
        &mut self.inner.hooks
    }
}
