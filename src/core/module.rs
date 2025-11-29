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

//! # Service Module Traits
//! 
//! This module defines the core service module traits for DMS, including both synchronous and asynchronous service modules.
//! It provides a comprehensive lifecycle management system for modules with multiple phases.
//! 
//! ## Key Components
//! 
//! - **_CServiceModule**: Synchronous service module trait with lifecycle methods
//! - **_CAsyncServiceModule**: Asynchronous service module trait with lifecycle methods
//! - **DMSModule**: Public asynchronous module trait
//! 
//! ## Design Principles
//! 
//! 1. **Lifecycle Management**: Modules follow a well-defined lifecycle with multiple phases
//! 2. **Sync/Async Support**: Clear separation between synchronous and asynchronous modules
//! 3. **Default Implementations**: Most methods have sensible defaults to minimize boilerplate
//! 4. **Dependency Resolution**: Modules can declare dependencies on other modules
//! 5. **Priority System**: Modules can specify execution priority
//! 6. **Criticality Flag**: Modules can be marked as critical or non-critical
//! 
//! ## Module Lifecycle
//! 
//! Modules go through the following lifecycle phases:
//! 
//! 1. **Initialization**: `_Finit` - Set up module resources
//! 2. **Before Start**: `_Fbefore_start` - Prepare for module startup
//! 3. **Start**: `_Fstart` - Start module execution
//! 4. **After Start**: `_Fafter_start` - Post-startup operations
//! 5. **Before Shutdown**: `_Fbefore_shutdown` - Prepare for shutdown
//! 6. **Shutdown**: `_Fshutdown` - Stop module execution
//! 7. **After Shutdown**: `_Fafter_shutdown` - Cleanup resources

use crate::core::{DMSResult, DMSServiceContext};

/// Internal synchronous service module trait.
/// 
/// This trait defines the interface for synchronous service modules in DMS. It provides
/// a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::core::{_CServiceModule, DMSResult, DMSServiceContext};
/// 
/// struct MySyncModule;
/// 
/// impl _CServiceModule for MySyncModule {
///     fn _Fname(&self) -> &str {
///         "my_sync_module"
///     }
///     
///     fn _Fis_critical(&self) -> bool {
///         false
///     }
///     
///     fn _Fpriority(&self) -> i32 {
///         10
///     }
///     
///     fn _Fdependencies(&self) -> Vec<&str> {
///         vec!["dependency_module"]
///     }
///     
///     fn _Fstart(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
///         // Start module logic
///         Ok(())
///     }
/// }
/// ```
pub trait _CServiceModule: Send + Sync {
    /// Returns the name of the service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn _Fname(&self) -> &str;

    /// Indicates if the module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn _Fis_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn _Fpriority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn _Fdependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    fn _Fbefore_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Starts the service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    fn _Fstart(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-startup operations.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    fn _Fafter_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    fn _Fbefore_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Shuts down the service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    fn _Fshutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}

/// Public asynchronous service module trait.
/// 
/// This trait defines the public interface for asynchronous service modules in DMS.
/// It provides a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::core::{DMSModule, DMSResult, DMSServiceContext};
/// use async_trait::async_trait;
/// 
/// struct MyAsyncModule;
/// 
/// #[async_trait]
/// impl DMSModule for MyAsyncModule {
///     fn name(&self) -> &str {
///         "my_async_module"
///     }
///     
///     async fn start(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
///         // Start async module logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait DMSModule: Send + Sync {
    /// Returns the name of the service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn name(&self) -> &str;

    /// Indicates if the module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn is_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn priority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn dependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    async fn init(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    async fn before_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Starts the service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-startup operations.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    async fn after_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    async fn before_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Shuts down the service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}

/// Internal asynchronous service module trait.
/// 
/// This trait defines the interface for internal asynchronous service modules in DMS.
/// It provides a comprehensive lifecycle management system with multiple phases.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::core::{_CAsyncServiceModule, DMSResult, DMSServiceContext};
/// use async_trait::async_trait;
/// 
/// struct MyInternalAsyncModule;
/// 
/// #[async_trait]
/// impl _CAsyncServiceModule for MyInternalAsyncModule {
///     fn _Fname(&self) -> &str {
///         "my_internal_async_module"
///     }
///     
///     async fn _Fstart(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
///         // Start internal async module logic
///         Ok(())
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait _CAsyncServiceModule: Send + Sync {
    /// Returns the name of the internal async service module.
    /// 
    /// This name is used for identification, dependency resolution, and logging purposes.
    fn _Fname(&self) -> &str;

    /// Indicates if the internal module is critical to the operation of the system.
    /// 
    /// Critical modules will cause the entire system to fail if they encounter an error,
    /// while non-critical modules can fail independently.
    /// 
    /// Default: `true`
    fn _Fis_critical(&self) -> bool {
        true
    }

    /// Returns the priority of the internal module.
    /// 
    /// Modules with higher priority are executed first within the same dependency level.
    /// 
    /// Default: `0`
    fn _Fpriority(&self) -> i32 {
        0
    }

    /// Returns the list of module dependencies.
    /// 
    /// Dependencies are module names that must be initialized and started before this module.
    /// The runtime will ensure dependencies are processed in the correct order.
    /// 
    /// Default: `Vec::new()`
    fn _Fdependencies(&self) -> Vec<&str> {
        Vec::new()
    }

    /// Initializes the internal async service module.
    /// 
    /// This method is called during the initialization phase to set up module resources.
    /// 
    /// Default: `Ok(())`
    async fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the internal module for startup.
    /// 
    /// This method is called after initialization but before the main start phase.
    /// 
    /// Default: `Ok(())`
    async fn _Fbefore_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Starts the internal async service module.
    /// 
    /// This method is called to start the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn _Fstart(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-startup operations for the internal module.
    /// 
    /// This method is called after the main start phase but before the module is considered fully started.
    /// 
    /// Default: `Ok(())`
    async fn _Fafter_start(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Prepares the internal module for shutdown.
    /// 
    /// This method is called before the main shutdown phase.
    /// 
    /// Default: `Ok(())`
    async fn _Fbefore_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Shuts down the internal async service module.
    /// 
    /// This method is called to stop the main functionality of the module.
    /// 
    /// Default: `Ok(())`
    async fn _Fshutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }

    /// Performs post-shutdown cleanup for the internal module.
    /// 
    /// This method is called after the main shutdown phase to clean up resources.
    /// 
    /// Default: `Ok(())`
    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        Ok(())
    }
}
