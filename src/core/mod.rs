//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//! 
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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
//! The core module provides the fundamental building blocks for Ri applications,
//! including the application builder, service context, error handling, and module lifecycle management.
//! 
//! ## Key Components
//! 
//! - **error**: Error handling with custom error types and result aliases
//! - **context**: Service context for accessing core functionalities
//! - **module**: Module system for extending Ri with custom functionality
//! - **runtime**: Application runtime and builder for constructing Ri applications
//! - **lifecycle**: Lifecycle management for modules
//! - **analytics**: Basic analytics and telemetry support
//! 
//! ## Design Principles
//! 
//! The core module follows the following design principles:
//! 
//! 1. **Dependency Injection**: Components are accessed through the service context, 
//!    allowing for easy mocking and testing
//! 2. **Builder Pattern**: The `RiAppBuilder` provides a fluent API for configuring applications
//! 3. **Module System**: A flexible module system allows for easy extension
//! 4. **Error Handling**: A unified error type simplifies error management across modules
//! 5. **Async First**: Full support for asynchronous operations
//!
//! ## Usage Example
//!
//! ```rust
//! use ri::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     let app = RiAppBuilder::new()
//!         .with_config("config.yaml")?
//!         .with_module(Box::new(MyModule::new()))
//!         .build()?;
//!
//!     app.run(|ctx| async move {
//!         ctx.logger().info("service", "Ri service started")?;
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
/// Module system for extending Ri with custom functionality
pub mod module;
/// Application runtime and builder for constructing Ri applications
pub mod runtime;
/// Application builder for constructing Ri applications
pub mod app_builder;
/// Application runtime for managing Ri application lifecycle
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
/// Sharded lock implementation for improved concurrent performance
pub mod concurrent;

/// Main error type for Ri operations
pub use error::{RiError, RiResult};
/// Service context providing access to core functionalities
pub use context::RiServiceContext;
/// Module traits for extending Ri functionality
pub use module::RiModule;
/// Application builder and runtime for constructing Ri applications
pub use runtime::{RiAppBuilder, RiAppRuntime};
/// Internal module traits
pub use module::{ServiceModule, AsyncServiceModule};

/// Lock utilities
#[cfg(feature = "pyo3")]
pub use lock::{RiLockError, RiLockResult, RwLockExtensions, MutexExtensions, from_poison_error};

/// Python module bindings
#[cfg(feature = "pyo3")]
pub use module::{RiPythonModule, RiPythonModuleAdapter, RiPythonServiceModule, RiPythonAsyncServiceModule};

/// Error chain utilities
#[cfg(feature = "pyo3")]
pub use error_chain::{RiErrorChain, RiErrorChainIter, RiErrorContext, RiOptionErrorContext};

/// Health check types
#[cfg(feature = "pyo3")]
pub use health::{RiHealthStatus, RiHealthCheckResult, RiHealthCheckConfig, RiHealthReport, RiHealthChecker};

/// Lifecycle management
#[cfg(feature = "pyo3")]
pub use lifecycle::RiLifecycleObserver;

/// Analytics module
#[cfg(feature = "pyo3")]
pub use analytics::RiLogAnalyticsModule;

/// Sharded lock types
#[cfg(feature = "pyo3")]
pub use concurrent::RiShardedLockStats;
