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

//! # DMS (Dunimd Middleware Service) Library
//! 
//! This is the main entry point for the DMS library, which provides a comprehensive
//! middleware service framework for building enterprise-grade backend applications.
//! 
//! ## Core Modules
//! 
//! DMS is organized into 12 core modules, each responsible for a specific set of functionalities:
//! 
//! - **core**: Core runtime, application builder, and service context
//! - **fs**: Secure file system operations and management
//! - **log**: Structured logging with tracing integration
//! - **config**: Multi-source configuration management
//! - **hooks**: Lifecycle event hooks for modules
//! - **observability**: Metrics, tracing, and monitoring support
//! - **device**: Device management and scheduling
//! - **cache**: Multi-backend cache abstraction
//! - **queue**: Distributed queue management
//! - **gateway**: API gateway with load balancing and rate limiting
//! - **service_mesh**: Service discovery and traffic management
//! - **auth**: Authentication and authorization mechanisms
//! 
//! ## Prelude
//! 
//! The `prelude` module re-exports commonly used types and traits for convenient access,
//! allowing users to import all essential components with a single `use dms::prelude::*;` statement.

#![allow(non_snake_case)]

/// Core runtime, application builder, and service context
pub mod core;
/// Secure file system operations and management
pub mod fs;
/// Structured logging with tracing integration
pub mod log;
/// Multi-source configuration management
pub mod config;
/// Lifecycle event hooks for modules
pub mod hooks;
/// Metrics, tracing, and monitoring support
pub mod observability;
/// Device management and scheduling
pub mod device;
/// Multi-backend cache abstraction
pub mod cache;
/// Distributed queue management
pub mod queue;
/// API gateway with load balancing and rate limiting
pub mod gateway;
/// Service discovery and traffic management
pub mod service_mesh;
/// Authentication and authorization mechanisms
pub mod auth;

/// Common re-exports for convenient access to core functionality
/// 
/// This module provides a single import point for all commonly used DMS components,
/// simplifying application code and reducing the number of import statements.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> DMSResult<()> {
///     let app = DMSAppBuilder::_Fnew()
///         ._Fwith_config("config.yaml")?
///         ._Fbuild()?;
///     
///     app._Frun(|ctx| async move {
///         ctx._Flogger()._Finfo("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```
pub mod prelude {
    // Re-export commonly used public classes here.
    /// Application builder for constructing DMS applications
    pub use crate::core::DMSAppBuilder;
    /// Service context providing access to application resources
    pub use crate::core::DMSServiceContext;
    /// Error type used throughout DMS
    pub use crate::core::DMSError;
    /// Result type alias using DMSError
    pub use crate::core::DMSResult;
    /// Secure file system operations
    pub use crate::fs::DMSFileSystem;
    /// Structured logger with tracing integration
    pub use crate::log::DMSLogger;
    /// Log configuration structure
    pub use crate::log::DMSLogConfig;
    /// Log level enum
    pub use crate::log::DMSLogLevel;
    /// Configuration management
    pub use crate::config::DMSConfig;
    /// Configuration manager for multi-source configuration
    pub use crate::config::DMSConfigManager;
    /// Hook bus for managing lifecycle events
    pub use crate::hooks::DMSHookBus;
    /// Hook event structure
    pub use crate::hooks::DMSHookEvent;
    /// Hook kind enum
    pub use crate::hooks::DMSHookKind;
}
