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

#![allow(non_snake_case)]

//! # Application Runtime and Builder
//! 
//! This module provides the application runtime and builder for constructing DMSC applications.
//! The `DMSCAppBuilder` follows the builder pattern for fluent configuration, while the `DMSCAppRuntime`
//! manages the application lifecycle and module execution.
//! 
//! ## Key Components
//! 
//! - **DMSCAppBuilder**: Fluent API for configuring and building DMSC applications
//! - **DMSCAppRuntime**: Manages the application lifecycle and module execution
//! 
//! ## Design Principles
//! 
//! 1. **Builder Pattern**: The `DMSCAppBuilder` provides a fluent API for configuring applications
//! 2. **Module Lifecycle**: Modules go through a well-defined lifecycle with init, start, and shutdown phases
//! 3. **Dependency Resolution**: Modules are sorted based on dependencies and priority
//! 4. **Async Support**: Full support for both synchronous and asynchronous modules
//! 5. **Fault Tolerance**: Non-critical modules can fail without crashing the entire application

// Re-export from app_builder.rs
pub use super::app_builder::DMSCAppBuilder;

// Re-export from app_runtime.rs
pub use super::app_runtime::DMSCAppRuntime;
