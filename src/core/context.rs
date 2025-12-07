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
pub struct ServiceContextInner {
    /// File system accessor for secure file operations
    pub fs: DMSFileSystem,
    /// Logger for structured logging
    pub logger: DMSLogger,
    /// Configuration manager for accessing application settings
    pub config: DMSConfigManager,
    /// Hook bus for emitting and handling lifecycle events
    pub hooks: DMSHookBus,
}

impl ServiceContextInner {
    /// Create a new `ServiceContextInner` instance with the provided components.
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
    /// A new `ServiceContextInner` instance.
    pub fn new(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        ServiceContextInner { fs, logger, config, hooks }
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
///     ctx.logger().info("request", "Handling request");
///     
///     // Access configuration
///     let config_value = ctx.config().config().get_str("app.name");
///     
///     // Access file system
///     let file_path = ctx.fs().app_data_path("logs/app.log");
///     
///     Ok(())
/// }
/// ```
pub struct DMSServiceContext {
    /// Internal implementation details
    inner: ServiceContextInner,
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
    pub fn new_with(fs: DMSFileSystem, logger: DMSLogger, config: DMSConfigManager, hooks: DMSHookBus) -> Self {
        let inner = ServiceContextInner::new(fs, logger, config, hooks);
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
    pub fn new_default() -> DMSResult<Self> {
        // Create default configuration manager
        let config = DMSConfigManager::new_default();
        let cfg = config.config();

        // Determine project root directory
        let project_root = std::env::current_dir()
            .map_err(|e| crate::core::DMSError::Other(format!("detect project root failed: {e}")))?;
        
        // Determine application data root directory
        let app_data_root = if let Some(root_str) = cfg.get_str("fs.app_data_root") {
            project_root.join(root_str)
        } else {
            project_root.join(".dms")
        };

        // Initialize file system
        let fs = DMSFileSystem::new_with_roots(project_root, app_data_root);

        // Initialize logging
        let log_config = DMSLogConfig::from_config(cfg);
        let logger = DMSLogger::new(&log_config, fs.clone());
        
        // Initialize hook bus
        let hooks = DMSHookBus::new();
        
        Ok(DMSServiceContext::new_with(fs, logger, config, hooks))
    }

    /// Get a reference to the file system accessor.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSFileSystem` instance.
    pub fn fs(&self) -> &DMSFileSystem {
        &self.inner.fs
    }
    


    /// Get a reference to the structured logger.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSLogger` instance.
    pub fn logger(&self) -> &DMSLogger {
        &self.inner.logger
    }
    


    /// Get a reference to the configuration manager.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSConfigManager` instance.
    pub fn config(&self) -> &DMSConfigManager {
        &self.inner.config
    }
    


    /// Get a reference to the hook bus for emitting events.
    /// 
    /// # Returns
    /// 
    /// A reference to the `DMSHookBus` instance.
    pub fn hooks(&self) -> &DMSHookBus {
        &self.inner.hooks
    }
    


    /// Get a mutable reference to the hook bus for registering handlers.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSHookBus` instance.
    pub fn hooks_mut(&mut self) -> &mut DMSHookBus {
        &mut self.inner.hooks
    }

    /// Get a mutable reference to the configuration manager.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSConfigManager` instance.
    pub fn config_mut(&mut self) -> &mut DMSConfigManager {
        &mut self.inner.config
    }

    /// Get a mutable reference to the file system accessor.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to to the `DMSFileSystem` instance.
    pub fn fs_mut(&mut self) -> &mut DMSFileSystem {
        &mut self.inner.fs
    }

    /// Get a mutable reference to the structured logger.
    /// 
    /// # Returns
    /// 
    /// A mutable reference to the `DMSLogger` instance.
    pub fn logger_mut(&mut self) -> &mut DMSLogger {
        &mut self.inner.logger
    }
}
