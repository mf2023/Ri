//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//! 
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//! 
//!     http://www.apache.org/licenses/LICENSE-2.0
//! 
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # Module Types
//! 
//! This module provides internal types for managing module state and distinguishing between
//! synchronous and asynchronous modules.

use crate::core::{ServiceModule, AsyncServiceModule};

/// Internal enum for distinguishing between synchronous and asynchronous modules.
/// 
/// This enum allows the runtime to handle both sync and async modules in a unified way,
/// while still respecting their different execution requirements.
pub(crate) enum ModuleType {
    /// Synchronous module that implements `ServiceModule`
    Sync(Box<dyn ServiceModule>),
    /// Asynchronous module that implements `AsyncServiceModule`
    Async(Box<dyn AsyncServiceModule>),
}



impl ModuleType {
    /// Get the name of the module.
    /// 
    /// # Returns
    /// 
    /// The name of the module as a string slice.
    pub(crate) fn name(&self) -> &str {
        match self {
            ModuleType::Sync(module) => module.name(),
            ModuleType::Async(module) => module.name(),
        }
    }

    /// Check if the module is critical.
    /// 
    /// Critical modules will cause the application to fail if they fail during initialization or startup.
    /// 
    /// # Returns
    /// 
    /// `true` if the module is critical, `false` otherwise.
    pub(crate) fn is_critical(&self) -> bool {
        match self {
            ModuleType::Sync(module) => module.is_critical(),
            ModuleType::Async(module) => module.is_critical(),
        }
    }

    /// Get the priority of the module.
    /// 
    /// Modules with higher priority are initialized and started first.
    /// 
    /// # Returns
    /// 
    /// The priority of the module as an integer.
    pub(crate) fn priority(&self) -> i32 {
        match self {
            ModuleType::Sync(module) => module.priority(),
            ModuleType::Async(module) => module.priority(),
        }
    }

    /// Get the dependencies of the module.
    /// 
    /// Dependencies are module names that must be initialized before this module.
    /// 
    /// # Returns
    /// 
    /// A vector of dependency module names.
    pub(crate) fn dependencies(&self) -> Vec<&str> {
        match self {
            ModuleType::Sync(module) => module.dependencies(),
            ModuleType::Async(module) => module.dependencies(),
        }
    }
}

/// Internal struct for tracking module state.
/// 
/// This struct wraps a module and tracks whether it has failed during execution.
pub struct ModuleSlot {
    /// The module itself, either sync or async
    pub(crate) module: ModuleType,
    /// Whether the module has failed during execution
    pub(crate) failed: bool,
}
