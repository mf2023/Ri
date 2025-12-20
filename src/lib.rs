//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! # DMSC (Dunimd Middleware Service) Library
//! 
//! This is the main entry point for the DMSC library, which provides a comprehensive
//! middleware service framework for building enterprise-grade backend applications.
//! 
//! ## Core Modules
//! 
//! DMSC is organized into 12 core modules, each responsible for a specific set of functionalities:
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
/// This module provides a single import point for all commonly used DMSC components,
/// simplifying application code and reducing the number of import statements.
/// 
/// ## Usage
/// 
/// ```rust
/// use dms::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> DMSCResult<()> {
///     let app = DMSCAppBuilder::new()
///         .with_config("config.yaml")?
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "DMSC service started")?;
///         Ok(())
///     }).await
/// }
/// ```
pub mod prelude {
    // Re-export commonly used public classes here.
    // Only DMSCXxxXxx format classes are exposed in prelude
    
    /// Application builder for constructing DMSC applications
    pub use crate::core::{DMSCAppBuilder, DMSCAppRuntime};
    /// Service context providing access to application resources
    pub use crate::core::DMSCServiceContext;
    /// Module traits for extending DMSC functionality
    pub use crate::core::DMSCModule;
    /// Error type used throughout DMSC
    pub use crate::core::DMSCError;
    /// Result type alias using DMSCError
    pub use crate::core::DMSCResult;
    
    /// Secure file system operations
    pub use crate::fs::DMSCFileSystem;
    
    /// Structured logger with tracing integration
    pub use crate::log::DMSCLogger;
    /// Log configuration structure
    pub use crate::log::DMSCLogConfig;
    /// Log level enum
    pub use crate::log::DMSCLogLevel;
    
    /// Configuration management
    pub use crate::config::DMSCConfig;
    /// Configuration manager for multi-source configuration
    pub use crate::config::DMSCConfigManager;
    
    /// Hook bus for managing lifecycle events
    pub use crate::hooks::DMSCHookBus;
    /// Hook event structure
    pub use crate::hooks::DMSCHookEvent;
    /// Hook kind enum
    pub use crate::hooks::DMSCHookKind;
    /// Module lifecycle phase definition
    pub use crate::hooks::DMSCModulePhase;
    
    /// Main cache module for DMSC
    pub use crate::cache::DMSCCacheModule;
    /// Cache configuration structure
    pub use crate::cache::DMSCCacheConfig;
    
    /// Main queue module for DMSC
    pub use crate::queue::DMSCQueueModule;
    /// Queue configuration structure
    pub use crate::queue::DMSCQueueConfig;
    
    /// Main gateway struct implementing the DMSCModule trait
    pub use crate::gateway::DMSCGateway;
    /// Configuration for the DMSC Gateway
    pub use crate::gateway::DMSCGatewayConfig;
    /// Route definition for API endpoints
    pub use crate::gateway::DMSCRoute;
    /// Router for handling request routing
    pub use crate::gateway::DMSCRouter;
    
    /// Main device control module for DMSC
    pub use crate::device::DMSCDeviceControlModule;
    /// Configuration for the device control module
    pub use crate::device::DMSCDeviceControlConfig;
    /// Device representation with type, status, and capabilities
    pub use crate::device::DMSCDevice;
    /// Enum defining supported device types
    pub use crate::device::DMSCDeviceType;
    
    /// Main authentication module for DMSC
    pub use crate::auth::DMSCAuthModule;
    /// Configuration for the authentication module
    pub use crate::auth::DMSCAuthConfig;
    
    /// Main service mesh struct implementing the DMSCModule trait
    pub use crate::service_mesh::DMSCServiceMesh;
    /// Configuration for the service mesh
    pub use crate::service_mesh::DMSCServiceMeshConfig;
    
    /// Main observability module for DMSC
    pub use crate::observability::DMSCObservabilityModule;
    /// Configuration for the observability module
    pub use crate::observability::DMSCObservabilityConfig;
    /// Distributed tracing implementation
    pub use crate::observability::DMSCTracer;
    /// Metrics collection and aggregation
    pub use crate::observability::DMSCMetricsRegistry;
}

/// Python bindings for DMSC
#[cfg(feature = "pyo3")]
pub mod py {
    use pyo3::prelude::*;
    use pyo3::types::PyModule;
    use crate::prelude::*;
    
    /// Initialize the Python module
    #[pymodule]
    pub fn dmsc(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Add core types that implement PyClass
        m.add_class::<DMSCAppBuilder>()?;
        m.add_class::<DMSCAppRuntime>()?;
        m.add_class::<DMSCConfig>()?;
        m.add_class::<DMSCConfigManager>()?;
        m.add_class::<DMSCError>()?;
        m.add_class::<DMSCServiceContext>()?;
        
        // Add other core types
        m.add_class::<DMSCLogger>()?;
        m.add_class::<DMSCLogConfig>()?;
        m.add_class::<DMSCLogLevel>()?;
        m.add_class::<DMSCFileSystem>()?;
        m.add_class::<DMSCHookBus>()?;
        m.add_class::<DMSCHookEvent>()?;
        m.add_class::<DMSCHookKind>()?;
        m.add_class::<DMSCModulePhase>()?;
        
        // Add queue types to main module
        m.add_class::<crate::queue::DMSCQueueModule>()?;
        m.add_class::<crate::queue::DMSCQueueConfig>()?;
        m.add_class::<crate::queue::DMSCQueueManager>()?;
        m.add_class::<crate::queue::DMSCQueueMessage>()?;
        m.add_class::<crate::queue::DMSCQueueStats>()?;
        
        // Add gateway types to main module
        m.add_class::<crate::gateway::DMSCGateway>()?;
        m.add_class::<crate::gateway::DMSCGatewayConfig>()?;
        m.add_class::<crate::gateway::DMSCRouter>()?;
        m.add_class::<crate::gateway::DMSCRoute>()?;
        
        // Add service mesh types to main module
        m.add_class::<crate::service_mesh::DMSCServiceMesh>()?;
        m.add_class::<crate::service_mesh::DMSCServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthChecker>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficManager>()?;
        
        // Add auth types to main module
        m.add_class::<crate::auth::DMSCAuthModule>()?;
        m.add_class::<crate::auth::DMSCAuthConfig>()?;
        m.add_class::<crate::auth::DMSCJWTManager>()?;
        m.add_class::<crate::auth::DMSCSessionManager>()?;
        m.add_class::<crate::auth::DMSCPermissionManager>()?;
        m.add_class::<crate::auth::DMSCOAuthManager>()?;
        
        // Create and add submodules
        create_log_module(m)?;
        create_config_module(m)?;
        create_device_module(m)?;
        create_cache_module(m)?;
        create_fs_module(m)?;
        create_hooks_module(m)?;
        create_observability_module(m)?;
        create_queue_module(m)?;
        create_gateway_module(m)?;
        create_service_mesh_module(m)?;
        create_auth_module(m)?;
        
        Ok(())
    }
    
    fn create_log_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "log")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_config_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "config")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_device_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "device")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_cache_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "cache")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_fs_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "fs")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_hooks_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "hooks")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_observability_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "observability")?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_queue_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "queue")?;
        m.add_class::<crate::queue::DMSCQueueModule>()?;
        m.add_class::<crate::queue::DMSCQueueConfig>()?;
        m.add_class::<crate::queue::DMSCQueueManager>()?;
        m.add_class::<crate::queue::DMSCQueueMessage>()?;
        m.add_class::<crate::queue::DMSCQueueStats>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_gateway_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "gateway")?;
        m.add_class::<crate::gateway::DMSCGateway>()?;
        m.add_class::<crate::gateway::DMSCGatewayConfig>()?;
        m.add_class::<crate::gateway::DMSCRoute>()?;
        m.add_class::<crate::gateway::DMSCRouter>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_service_mesh_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "service_mesh")?;
        m.add_class::<crate::service_mesh::DMSCServiceMesh>()?;
        m.add_class::<crate::service_mesh::DMSCServiceMeshConfig>()?;
        m.add_class::<crate::service_mesh::DMSCServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::DMSCServiceInstance>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthChecker>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficManager>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_auth_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "auth")?;
        m.add_class::<crate::auth::DMSCAuthModule>()?;
        m.add_class::<crate::auth::DMSCAuthConfig>()?;
        m.add_class::<crate::auth::DMSCJWTManager>()?;
        m.add_class::<crate::auth::DMSCSessionManager>()?;
        m.add_class::<crate::auth::DMSCPermissionManager>()?;
        m.add_class::<crate::auth::DMSCOAuthManager>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
}
