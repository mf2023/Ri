//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Core Runtime Module
//! 
//! The core module provides the fundamental building blocks for DMSC applications,
//! including the application builder, service context, error handling, and module lifecycle management.
//! 
//! ## Key Components
//! 
//! - **error**: Error handling with custom error types and result aliases
//! - **context**: Service context for accessing core functionalities
//! - **module**: Module system for extending DMSC with custom functionality
//! - **runtime**: Application runtime and builder for constructing DMSC applications
//! - **lifecycle**: Lifecycle management for modules
//! - **analytics**: Basic analytics and telemetry support
//! 
//! ## Design Principles
//! 
//! The core module follows the following design principles:
//! 
//! 1. **Dependency Injection**: Components are accessed through the service context, 
//!    allowing for easy mocking and testing
//! 2. **Builder Pattern**: The `DMSCAppBuilder` provides a fluent API for configuring applications
//! 3. **Module System**: A flexible module system allows for easy extension
//! 4. **Error Handling**: A unified error type simplifies error management across modules
//! 5. **Async First**: Full support for asynchronous operations
//!
//! ## Usage Example
//!
//! ```rust
//! use dmsc::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> DMSCResult<()> {
//!     let app = DMSCAppBuilder::new()
//!         .with_config("config.yaml")?
//!         .with_module(Box::new(MyModule::new()))
//!         .build()?;
//!
//!     app.run(|ctx| async move {
//!         ctx.logger().info("service", "DMSC service started")?;
//!         Ok(())
//!     }).await
//! }
//! ```

/// Error handling with custom error types and result aliases
pub mod error;
/// Safe lock utilities for concurrent programming
pub mod lock;
/// Service context for accessing core functionalities
pub mod context;
/// Module system for extending DMSC with custom functionality
pub mod module;
/// Application runtime and builder for constructing DMSC applications
pub mod runtime;
/// Application builder for constructing DMSC applications
pub mod app_builder;
/// Application runtime for managing DMSC application lifecycle
pub mod app_runtime;
/// Module types for distinguishing between sync and async modules
pub mod module_types;
/// Module sorter for sorting modules based on dependencies and priority
pub mod module_sorter;
/// Lifecycle management for modules
pub mod lifecycle;
/// Basic analytics and telemetry support
pub mod analytics;
/// Health checks for modules and services
pub mod health;
/// Error chain utilities
pub mod error_chain;

/// Main error type for DMSC operations
pub use error::{DMSCError, DMSCResult};
/// Service context providing access to core functionalities
pub use context::DMSCServiceContext;
/// Module traits for extending DMSC functionality
pub use module::DMSCModule;
/// Application builder and runtime for constructing DMSC applications
pub use runtime::{DMSCAppBuilder, DMSCAppRuntime};
/// Internal module traits
pub(crate) use module::{ServiceModule, AsyncServiceModule};
