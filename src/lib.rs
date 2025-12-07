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
///     let app = DMSAppBuilder::new()
///         .with_config("config.yaml")?
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMS service started")?;
///         Ok(())
///     }).await
/// }
/// ```
pub mod prelude {
    // Re-export commonly used public classes here.
    // Only DMSXxxXxx format classes are exposed in prelude
    
    /// Application builder for constructing DMS applications
    pub use crate::core::{DMSAppBuilder, DMSAppRuntime};
    /// Service context providing access to application resources
    pub use crate::core::DMSServiceContext;
    /// Module traits for extending DMS functionality
    pub use crate::core::DMSModule;
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
    /// Module lifecycle phase definition
    pub use crate::hooks::DMSModulePhase;
    
    /// Main cache module for DMS
    pub use crate::cache::DMSCacheModule;
    /// Cache configuration structure
    pub use crate::cache::DMSCacheConfig;
    
    /// Main queue module for DMS
    pub use crate::queue::DMSQueueModule;
    /// Queue configuration structure
    pub use crate::queue::DMSQueueConfig;
    
    /// Main gateway struct implementing the DMSModule trait
    pub use crate::gateway::DMSGateway;
    /// Configuration for the DMS Gateway
    pub use crate::gateway::DMSGatewayConfig;
    /// Route definition for API endpoints
    pub use crate::gateway::DMSRoute;
    /// Router for handling request routing
    pub use crate::gateway::DMSRouter;
    
    /// Main device control module for DMS
    pub use crate::device::DMSDeviceControlModule;
    /// Configuration for the device control module
    pub use crate::device::DMSDeviceControlConfig;
    /// Device representation with type, status, and capabilities
    pub use crate::device::DMSDevice;
    /// Enum defining supported device types
    pub use crate::device::DMSDeviceType;
    
    /// Main authentication module for DMS
    pub use crate::auth::DMSAuthModule;
    /// Configuration for the authentication module
    pub use crate::auth::DMSAuthConfig;
    
    /// Main service mesh struct implementing the DMSModule trait
    pub use crate::service_mesh::DMSServiceMesh;
    /// Configuration for the service mesh
    pub use crate::service_mesh::DMSServiceMeshConfig;
    
    /// Main observability module for DMS
    pub use crate::observability::DMSObservabilityModule;
    /// Configuration for the observability module
    pub use crate::observability::DMSObservabilityConfig;
    /// Distributed tracing implementation
    pub use crate::observability::DMSTracer;
    /// Metrics collection and aggregation
    pub use crate::observability::DMSMetricsRegistry;
}

/// Python bindings for DMS
#[cfg(feature = "pyo3")]
pub mod py {
    use pyo3::prelude::*;
    use pyo3::types::PyModule;
    use crate::prelude::*;
    
    /// Initialize the Python module
    #[pymodule]
    pub fn dms_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Add core types that implement PyClass
        m.add_class::<DMSAppBuilder>()?;
        
        // Create and add submodules
        let log_module = PyModule::new(m.py(), "log")?;
        m.add_submodule(&log_module)?;
        
        let config_module = PyModule::new(m.py(), "config")?;
        m.add_submodule(&config_module)?;
        
        let device_module = PyModule::new(m.py(), "device")?;
        m.add_submodule(&device_module)?;
        
        let cache_module = PyModule::new(m.py(), "cache")?;
        m.add_submodule(&cache_module)?;
        
        let fs_module = PyModule::new(m.py(), "fs")?;
        m.add_submodule(&fs_module)?;
        
        let hooks_module = PyModule::new(m.py(), "hooks")?;
        m.add_submodule(&hooks_module)?;
        
        let observability_module = PyModule::new(m.py(), "observability")?;
        m.add_submodule(&observability_module)?;
        
        Ok(())
    }
}
