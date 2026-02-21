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
//! allowing users to import all essential components with a single `use dmsc::prelude::*;` statement.

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
/// Database abstraction layer with multiple backend support
pub mod database;
/// Protocol abstraction layer for multiple protocol support
#[cfg(feature = "protocol")]
pub mod protocol;
/// Validation utilities for input validation and data sanitization
pub mod validation;
/// Inter-module RPC communication for distributed method calls
pub mod module_rpc;
/// gRPC server and client support
#[cfg(feature = "grpc")]
pub mod grpc;
/// WebSocket server and client support
#[cfg(feature = "websocket")]
pub mod ws;
/// C/C++ API support
#[cfg(feature = "c")]
pub mod c;
/// Java JNI bindings support
#[cfg(feature = "java")]
pub mod java;

/// Common re-exports for convenient access to core functionality
/// 
/// This module provides a single import point for all commonly used DMSC components,
/// simplifying application code and reducing the number of import statements.
/// 
/// ## Usage
/// 
/// ```rust,ignore
/// use dmsc::prelude::*;
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
    
    /// Lock utilities - Only DMSCLockError is available in Python
    /// Note: DMSCLockResult, RwLockExtensions, MutexExtensions, and from_poison_error
    /// are Rust-only types and not exposed in Python bindings
    #[cfg(feature = "pyo3")]
    pub use crate::core::DMSCLockError;
    
    /// Python module support
    #[cfg(feature = "pyo3")]
    pub use crate::core::{DMSCPythonModule, DMSCPythonModuleAdapter, DMSCPythonServiceModule, DMSCPythonAsyncServiceModule};
    
    /// Error chain utilities
    #[cfg(feature = "pyo3")]
    pub use crate::core::{DMSCErrorChain, DMSCErrorChainIter, DMSCErrorContext, DMSCOptionErrorContext};
    
    /// Health check types
    #[cfg(feature = "pyo3")]
    pub use crate::core::{DMSCHealthStatus, DMSCHealthCheckResult, DMSCHealthCheckConfig, DMSCHealthReport, DMSCHealthChecker};
    
    /// Service mesh health check types
    #[cfg(feature = "pyo3")]
    pub use crate::service_mesh::health_check::{DMSCHealthCheckType, DMSCHealthSummary};
    
    /// Lifecycle management
    #[cfg(feature = "pyo3")]
    pub use crate::core::DMSCLifecycleObserver;
    
    /// Analytics module
    #[cfg(feature = "pyo3")]
    pub use crate::core::DMSCLogAnalyticsModule;
    
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
    /// Backend server for load balancing
    pub use crate::gateway::load_balancer::DMSCBackendServer;
    /// Load balancer server statistics
    pub use crate::gateway::load_balancer::DMSCLoadBalancerServerStats;
    /// Load balancer implementation
    pub use crate::gateway::load_balancer::DMSCLoadBalancer;
    /// Load balancing strategy enum
    pub use crate::gateway::load_balancer::DMSCLoadBalancerStrategy;
    
    /// Main device control module for DMSC
    pub use crate::device::DMSCDeviceControlModule;
    /// Configuration for device discovery
    pub use crate::device::DMSCDeviceControlConfig;
    /// Scheduling configuration for device control
    pub use crate::device::DMSCDeviceSchedulingConfig;
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
    /// Statistics for the service mesh
    pub use crate::service_mesh::DMSCServiceMeshStats;
    /// Service endpoint information
    pub use crate::service_mesh::DMSCServiceEndpoint;
    /// Service health status enum
    pub use crate::service_mesh::DMSCServiceHealthStatus;
    
    /// Main observability module for DMSC
    pub use crate::observability::DMSCObservabilityModule;
    /// Configuration for the observability module
    pub use crate::observability::DMSCObservabilityConfig;
    /// Distributed tracing implementation
    pub use crate::observability::DMSCTracer;
    /// Metrics collection and aggregation
    pub use crate::observability::DMSCMetricsRegistry;
    
    /// Database configuration structure
    pub use crate::database::DMSCDatabaseConfig;
    /// Database type enum
    pub use crate::database::DatabaseType;
    /// Database connection pool
    pub use crate::database::DMSCDatabasePool;
    /// Database row representation
    pub use crate::database::DMSCDBRow;
    /// Database query result
    pub use crate::database::DMSCDBResult;
    
    /// Inter-module RPC coordinator
    pub use crate::module_rpc::DMSCModuleRPC;
    /// RPC client for making method calls
    pub use crate::module_rpc::DMSCModuleClient;
    /// RPC endpoint for a module
    pub use crate::module_rpc::DMSCModuleEndpoint;
    /// RPC method call request
    pub use crate::module_rpc::DMSCMethodCall;
    /// RPC method call response
    pub use crate::module_rpc::DMSCMethodResponse;
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
        
        // Add Python module support
        m.add_class::<crate::core::module::DMSCPythonModule>()?;
        m.add_class::<crate::core::module::DMSCPythonModuleAdapter>()?;
        m.add_class::<crate::core::module::DMSCPythonServiceModule>()?;
        m.add_class::<crate::core::module::DMSCPythonAsyncServiceModule>()?;
        
        // Add other core types
        m.add_class::<DMSCLogger>()?;
        m.add_class::<DMSCLogConfig>()?;
        m.add_class::<DMSCLogLevel>()?;
        m.add_class::<DMSCFileSystem>()?;
        m.add_class::<DMSCHookBus>()?;
        m.add_class::<DMSCHookEvent>()?;
        m.add_class::<DMSCHookKind>()?;
        m.add_class::<DMSCModulePhase>()?;
        
        // Add lock types
        m.add_class::<crate::core::DMSCLockError>()?;
        
        // Add health check types
        m.add_class::<crate::core::DMSCHealthStatus>()?;
        m.add_class::<crate::core::DMSCHealthCheckResult>()?;
        m.add_class::<crate::core::DMSCHealthCheckConfig>()?;
        m.add_class::<crate::core::DMSCHealthReport>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthCheckType>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthSummary>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficManager>()?;
        
        // Add lifecycle types
        m.add_class::<crate::core::DMSCLifecycleObserver>()?;
        
        // Add analytics types
        m.add_class::<crate::core::DMSCLogAnalyticsModule>()?;
        
        // Add cache types to main module
        m.add_class::<crate::cache::DMSCCacheModule>()?;
        m.add_class::<crate::cache::DMSCCacheManager>()?;
        m.add_class::<crate::cache::DMSCCacheConfig>()?;
        m.add_class::<crate::cache::DMSCCacheBackendType>()?;
        m.add_class::<crate::cache::DMSCCachePolicy>()?;
        m.add_class::<crate::cache::DMSCCacheStats>()?;
        m.add_class::<crate::cache::DMSCCachedValue>()?;
        m.add_class::<crate::cache::DMSCCacheEvent>()?;
        
        // Add queue types to main module
        m.add_class::<crate::queue::DMSCQueueModule>()?;
        m.add_class::<crate::queue::DMSCQueueConfig>()?;
        m.add_class::<crate::queue::DMSCQueueManager>()?;
        m.add_class::<crate::queue::DMSCQueueMessage>()?;
        m.add_class::<crate::queue::DMSCQueueStats>()?;
        m.add_class::<crate::queue::DMSCQueueBackendType>()?;
        m.add_class::<crate::queue::DMSCRetryPolicy>()?;
        m.add_class::<crate::queue::DMSCDeadLetterConfig>()?;
        
        // Add gateway types to main module
        m.add_class::<crate::gateway::DMSCGateway>()?;
        m.add_class::<crate::gateway::DMSCGatewayConfig>()?;
        m.add_class::<crate::gateway::DMSCRouter>()?;
        m.add_class::<crate::gateway::DMSCRoute>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimiter>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimitConfig>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimitStats>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCSlidingWindowRateLimiter>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreaker>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerConfig>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerState>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerMetrics>()?;
        
        // Add load balancer types to main module
        m.add_class::<crate::gateway::load_balancer::DMSCBackendServer>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancerServerStats>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancer>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancerStrategy>()?;
        
        // Add service mesh types to main module
        m.add_class::<crate::service_mesh::DMSCServiceMesh>()?;
        m.add_class::<crate::service_mesh::DMSCServiceMeshConfig>()?;
        m.add_class::<crate::service_mesh::DMSCServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::DMSCServiceInstance>()?;
        m.add_class::<crate::service_mesh::DMSCServiceStatus>()?;
        m.add_class::<crate::service_mesh::DMSCServiceMeshStats>()?;
        m.add_class::<crate::service_mesh::DMSCServiceEndpoint>()?;
        m.add_class::<crate::service_mesh::DMSCServiceHealthStatus>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthChecker>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthSummary>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficManager>()?;
        m.add_class::<crate::service_mesh::DMSCTrafficRoute>()?;
        m.add_class::<crate::service_mesh::DMSCMatchCriteria>()?;
        m.add_class::<crate::service_mesh::DMSCRouteAction>()?;
        m.add_class::<crate::service_mesh::DMSCWeightedDestination>()?;
        
        // Add auth types to main module
        m.add_class::<crate::auth::DMSCAuthModule>()?;
        m.add_class::<crate::auth::DMSCAuthConfig>()?;
        m.add_class::<crate::auth::DMSCJWTManager>()?;
        m.add_class::<crate::auth::DMSCJWTClaims>()?;
        m.add_class::<crate::auth::DMSCJWTValidationOptions>()?;
        m.add_class::<crate::auth::DMSCSessionManager>()?;
        m.add_class::<crate::auth::DMSCSession>()?;
        m.add_class::<crate::auth::DMSCPermissionManager>()?;
        m.add_class::<crate::auth::DMSCOAuthManager>()?;
        m.add_class::<crate::auth::DMSCOAuthToken>()?;
        m.add_class::<crate::auth::DMSCOAuthUserInfo>()?;
        m.add_class::<crate::auth::DMSCOAuthProvider>()?;
        m.add_class::<crate::auth::DMSCPermission>()?;
        m.add_class::<crate::auth::DMSCRole>()?;
        m.add_class::<crate::auth::DMSCJWTRevocationList>()?;
        m.add_class::<crate::auth::DMSCRevokedTokenInfo>()?;
        
        // Add observability types to main module
        m.add_class::<crate::observability::DMSCObservabilityModule>()?;
        m.add_class::<crate::observability::DMSCObservabilityConfig>()?;
        m.add_class::<crate::observability::DMSCMetricsRegistry>()?;
        m.add_class::<crate::observability::DMSCTracer>()?;
        m.add_class::<crate::observability::DMSCMetricType>()?;
        m.add_class::<crate::observability::DMSCMetricConfig>()?;
        m.add_class::<crate::observability::DMSCMetricSample>()?;
        m.add_class::<crate::observability::DMSCMetric>()?;
        m.add_class::<crate::observability::DMSCObservabilityData>()?;
        
        // Add system metrics collector types (requires system_info feature)
        #[cfg(feature = "system_info")]
        {
            m.add_class::<crate::observability::DMSCSystemMetricsCollector>()?;
            m.add_class::<crate::observability::DMSCSystemMetrics>()?;
            m.add_class::<crate::observability::DMSCCPUMetrics>()?;
            m.add_class::<crate::observability::DMSCMemoryMetrics>()?;
            m.add_class::<crate::observability::DMSCDiskMetrics>()?;
            m.add_class::<crate::observability::DMSCNetworkMetrics>()?;
        }
        
        // Add validation types to main module
        m.add_class::<crate::validation::DMSCValidationError>()?;
        m.add_class::<crate::validation::DMSCValidationResult>()?;
        m.add_class::<crate::validation::DMSCValidationSeverity>()?;
        m.add_class::<crate::validation::DMSCValidatorBuilder>()?;
        m.add_class::<crate::validation::DMSCValidationRunner>()?;
        m.add_class::<crate::validation::DMSCSanitizer>()?;
        m.add_class::<crate::validation::DMSCSanitizationConfig>()?;
        m.add_class::<crate::validation::DMSCSchemaValidator>()?;
        m.add_class::<crate::validation::DMSCValidationModule>()?;
        
        // Add protocol types to main module
        #[cfg(feature = "protocol")]
        {
            m.add_class::<crate::protocol::DMSCProtocolManager>()?;
            m.add_class::<crate::protocol::DMSCProtocolType>()?;
            m.add_class::<crate::protocol::DMSCProtocolConfig>()?;
            m.add_class::<crate::protocol::DMSCProtocolStatus>()?;
            m.add_class::<crate::protocol::DMSCProtocolStats>()?;
            m.add_class::<crate::protocol::DMSCConnectionState>()?;
            m.add_class::<crate::protocol::DMSCConnectionStats>()?;
            m.add_class::<crate::protocol::DMSCProtocolHealth>()?;
            m.add_class::<crate::protocol::DMSCFrame>()?;
            m.add_class::<crate::protocol::DMSCFrameHeader>()?;
            m.add_class::<crate::protocol::DMSCFrameType>()?;
            m.add_class::<crate::protocol::DMSCConnectionInfo>()?;
            m.add_class::<crate::protocol::DMSCMessageFlags>()?;
            m.add_class::<crate::protocol::DMSCSecurityLevel>()?;
            m.add_class::<crate::protocol::frames::DMSCFrameParser>()?;
            m.add_class::<crate::protocol::frames::DMSCFrameBuilder>()?;
        }

        // Add database types to main module
        m.add_class::<crate::database::DMSCDatabaseConfig>()?;
        m.add_class::<crate::database::DMSCDatabasePool>()?;
        m.add_class::<crate::database::DMSCDBRow>()?;
        m.add_class::<crate::database::DMSCDBResult>()?;
        m.add_class::<crate::database::orm::DMSCPyORMRepository>()?;

        // Add grpc types to main module
        #[cfg(all(feature = "grpc", feature = "pyo3"))]
        {
            m.add_class::<crate::grpc::DMSCGrpcConfig>()?;
            m.add_class::<crate::grpc::DMSCGrpcStats>()?;
            m.add_class::<crate::grpc::DMSCGrpcServiceRegistryPy>()?;
            m.add_class::<crate::grpc::DMSCGrpcServerPy>()?;
            m.add_class::<crate::grpc::DMSCGrpcClientPy>()?;
        }

        // Add websocket types to main module
        #[cfg(all(feature = "websocket", feature = "pyo3"))]
        {
            m.add_class::<crate::ws::DMSCWSServerConfig>()?;
            m.add_class::<crate::ws::DMSCWSEvent>()?;
            m.add_class::<crate::ws::DMSCWSSessionInfo>()?;
            m.add_class::<crate::ws::DMSCWSServerStats>()?;
            m.add_class::<crate::ws::DMSCWSPythonHandler>()?;
            m.add_class::<crate::ws::DMSCWSSessionManagerPy>()?;
            m.add_class::<crate::ws::DMSCWSServerPy>()?;
            m.add_class::<crate::ws::DMSCWSClientConfig>()?;
            m.add_class::<crate::ws::DMSCWSClientStats>()?;
            m.add_class::<crate::ws::DMSCWSClientPy>()?;
        }

        // Add module_rpc types to main module
        m.add_class::<crate::module_rpc::DMSCModuleRPC>()?;
        m.add_class::<crate::module_rpc::DMSCModuleClient>()?;
        m.add_class::<crate::module_rpc::DMSCModuleEndpoint>()?;
        m.add_class::<crate::module_rpc::DMSCMethodCall>()?;
        m.add_class::<crate::module_rpc::DMSCMethodResponse>()?;
        
        // Add device types to main module
        m.add_class::<crate::device::DMSCDeviceControlModule>()?;
        m.add_class::<crate::device::DMSCDevice>()?;
        m.add_class::<crate::device::DMSCDeviceType>()?;
        m.add_class::<crate::device::DMSCDeviceStatus>()?;
        m.add_class::<crate::device::DMSCDeviceCapabilities>()?;
        m.add_class::<crate::device::DMSCDeviceHealthMetrics>()?;
        m.add_class::<crate::device::DMSCDeviceController>()?;
        m.add_class::<crate::device::DMSCDeviceConfig>()?;
        m.add_class::<crate::device::DMSCDeviceControlConfig>()?;
        m.add_class::<crate::device::DMSCDeviceSchedulingConfig>()?;
        m.add_class::<crate::device::DMSCNetworkDeviceInfo>()?;
        m.add_class::<crate::device::DMSCDiscoveryResult>()?;
        m.add_class::<crate::device::DMSCResourceRequest>()?;
        m.add_class::<crate::device::DMSCResourceAllocation>()?;
        m.add_class::<crate::device::DMSCRequestSlaClass>()?;
        m.add_class::<crate::device::DMSCResourceWeights>()?;
        m.add_class::<crate::device::DMSCAffinityRules>()?;
        m.add_class::<crate::device::DMSCResourcePoolStatus>()?;
        m.add_class::<crate::device::pool::DMSCResourcePool>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolConfig>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolStatistics>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolManager>()?;
        m.add_class::<crate::device::pool::DMSCConnectionPoolStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCResourceScheduler>()?;
        m.add_class::<crate::device::scheduler::DMSCDeviceScheduler>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingPolicy>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationRecord>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationRequest>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCDeviceTypeStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingRecommendation>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingRecommendationType>()?;
        m.add_class::<crate::device::DMSCDeviceDiscoveryEngine>()?;
        
        // Add database migration type
        m.add_class::<crate::database::DMSCDatabaseMigration>()?;
        
        // Create and add submodules
        create_device_module(m)?;
        create_cache_module(m)?;
        create_fs_module(m)?;
        create_hooks_module(m)?;
        create_observability_module(m)?;
        create_queue_module(m)?;
        create_gateway_module(m)?;
        create_service_mesh_module(m)?;
        create_auth_module(m)?;
        create_database_module(m)?;
        create_validation_module(m)?;
        #[cfg(feature = "protocol")]
        create_protocol_module(m)?;
        create_grpc_module(m)?;
        create_ws_module(m)?;

        Ok(())
    }

    fn create_device_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "device")?;
        
        m.add_class::<crate::device::DMSCDeviceControlModule>()?;
        m.add_class::<crate::device::DMSCDevice>()?;
        m.add_class::<crate::device::DMSCDeviceType>()?;
        m.add_class::<crate::device::DMSCDeviceStatus>()?;
        m.add_class::<crate::device::DMSCDeviceCapabilities>()?;
        m.add_class::<crate::device::DMSCDeviceHealthMetrics>()?;
        m.add_class::<crate::device::DMSCDeviceController>()?;
        m.add_class::<crate::device::DMSCDeviceConfig>()?;
        m.add_class::<crate::device::DMSCDeviceControlConfig>()?;
        m.add_class::<crate::device::DMSCDeviceSchedulingConfig>()?;
        m.add_class::<crate::device::DMSCNetworkDeviceInfo>()?;
        m.add_class::<crate::device::DMSCDiscoveryResult>()?;
        m.add_class::<crate::device::DMSCResourceRequest>()?;
        m.add_class::<crate::device::DMSCResourceAllocation>()?;
        m.add_class::<crate::device::DMSCRequestSlaClass>()?;
        m.add_class::<crate::device::DMSCResourceWeights>()?;
        m.add_class::<crate::device::DMSCAffinityRules>()?;
        m.add_class::<crate::device::pool::DMSCResourcePool>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolConfig>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolStatistics>()?;
        m.add_class::<crate::device::pool::DMSCResourcePoolManager>()?;
        m.add_class::<crate::device::pool::DMSCConnectionPoolStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCResourceScheduler>()?;
        m.add_class::<crate::device::scheduler::DMSCDeviceScheduler>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingPolicy>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationRecord>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationRequest>()?;
        m.add_class::<crate::device::scheduler::DMSCAllocationStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCDeviceTypeStatistics>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingRecommendation>()?;
        m.add_class::<crate::device::scheduler::DMSCSchedulingRecommendationType>()?;
        m.add_class::<crate::device::DMSCDeviceDiscoveryEngine>()?;
        
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_cache_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "cache")?;
        
        // Add cache types to the cache module
        m.add_class::<crate::cache::DMSCCacheModule>()?;
        m.add_class::<crate::cache::DMSCCacheManager>()?;
        m.add_class::<crate::cache::DMSCCacheConfig>()?;
        m.add_class::<crate::cache::DMSCCacheBackendType>()?;
        m.add_class::<crate::cache::DMSCCachePolicy>()?;
        m.add_class::<crate::cache::DMSCCacheStats>()?;
        m.add_class::<crate::cache::DMSCCachedValue>()?;
        m.add_class::<crate::cache::DMSCCacheEvent>()?;
        
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_fs_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "fs")?;
        m.add_class::<crate::fs::DMSCFileSystem>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_hooks_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "hooks")?;
        m.add_class::<crate::hooks::DMSCHookKind>()?;
        m.add_class::<crate::hooks::DMSCModulePhase>()?;
        m.add_class::<crate::hooks::DMSCHookEvent>()?;
        m.add_class::<crate::hooks::DMSCHookBus>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_observability_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "observability")?;
        m.add_class::<crate::observability::DMSCObservabilityModule>()?;
        m.add_class::<crate::observability::DMSCObservabilityConfig>()?;
        m.add_class::<crate::observability::DMSCObservabilityData>()?;
        m.add_class::<crate::observability::DMSCMetricsRegistry>()?;
        m.add_class::<crate::observability::DMSCTracer>()?;
        m.add_class::<crate::observability::DMSCMetricType>()?;
        m.add_class::<crate::observability::DMSCMetricConfig>()?;
        m.add_class::<crate::observability::DMSCMetricSample>()?;
        m.add_class::<crate::observability::DMSCMetric>()?;
        #[cfg(feature = "system_info")]
        {
            m.add_class::<crate::observability::DMSCSystemMetricsCollector>()?;
            m.add_class::<crate::observability::DMSCSystemMetrics>()?;
            m.add_class::<crate::observability::DMSCCPUMetrics>()?;
            m.add_class::<crate::observability::DMSCMemoryMetrics>()?;
            m.add_class::<crate::observability::DMSCDiskMetrics>()?;
            m.add_class::<crate::observability::DMSCNetworkMetrics>()?;
        }
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
        m.add_class::<crate::queue::DMSCQueueBackendType>()?;
        m.add_class::<crate::queue::DMSCRetryPolicy>()?;
        m.add_class::<crate::queue::DMSCDeadLetterConfig>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_gateway_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "gateway")?;
        m.add_class::<crate::gateway::DMSCGateway>()?;
        m.add_class::<crate::gateway::DMSCGatewayConfig>()?;
        m.add_class::<crate::gateway::DMSCRoute>()?;
        m.add_class::<crate::gateway::DMSCRouter>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimiter>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimitConfig>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCRateLimitStats>()?;
        m.add_class::<crate::gateway::rate_limiter::DMSCSlidingWindowRateLimiter>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreaker>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerConfig>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerState>()?;
        m.add_class::<crate::gateway::circuit_breaker::DMSCCircuitBreakerMetrics>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCBackendServer>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancerServerStats>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancer>()?;
        m.add_class::<crate::gateway::load_balancer::DMSCLoadBalancerStrategy>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_service_mesh_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "service_mesh")?;
        m.add_class::<crate::service_mesh::DMSCServiceMesh>()?;
        m.add_class::<crate::service_mesh::DMSCServiceMeshConfig>()?;
        m.add_class::<crate::service_mesh::DMSCServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::DMSCServiceInstance>()?;
        m.add_class::<crate::service_mesh::DMSCServiceStatus>()?;
        m.add_class::<crate::service_mesh::DMSCServiceMeshStats>()?;
        m.add_class::<crate::service_mesh::DMSCServiceEndpoint>()?;
        m.add_class::<crate::service_mesh::DMSCServiceHealthStatus>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthChecker>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthSummary>()?;
        m.add_class::<crate::service_mesh::health_check::DMSCHealthCheckType>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficManager>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCTrafficRoute>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCMatchCriteria>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCRouteAction>()?;
        m.add_class::<crate::service_mesh::traffic_management::DMSCWeightedDestination>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_auth_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "auth")?;
        m.add_class::<crate::auth::DMSCAuthModule>()?;
        m.add_class::<crate::auth::DMSCAuthConfig>()?;
        m.add_class::<crate::auth::DMSCJWTManager>()?;
        m.add_class::<crate::auth::DMSCJWTClaims>()?;
        m.add_class::<crate::auth::DMSCJWTValidationOptions>()?;
        m.add_class::<crate::auth::DMSCSessionManager>()?;
        m.add_class::<crate::auth::DMSCSession>()?;
        m.add_class::<crate::auth::DMSCPermissionManager>()?;
        m.add_class::<crate::auth::DMSCPermission>()?;
        m.add_class::<crate::auth::DMSCRole>()?;
        m.add_class::<crate::auth::DMSCOAuthManager>()?;
        m.add_class::<crate::auth::DMSCOAuthToken>()?;
        m.add_class::<crate::auth::DMSCOAuthUserInfo>()?;
        m.add_class::<crate::auth::DMSCOAuthProvider>()?;
        m.add_class::<crate::auth::DMSCJWTRevocationList>()?;
        m.add_class::<crate::auth::DMSCRevokedTokenInfo>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_database_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "database")?;
        m.add_class::<crate::database::DMSCDatabaseConfig>()?;
        m.add_class::<crate::database::DMSCDBRow>()?;
        m.add_class::<crate::database::DMSCDBResult>()?;
        m.add_class::<crate::database::orm::ColumnDefinition>()?;
        m.add_class::<crate::database::orm::IndexDefinition>()?;
        m.add_class::<crate::database::orm::ForeignKeyDefinition>()?;
        m.add_class::<crate::database::orm::TableDefinition>()?;
        m.add_class::<crate::database::orm::ComparisonOperator>()?;
        m.add_class::<crate::database::orm::LogicalOperator>()?;
        m.add_class::<crate::database::orm::Criteria>()?;
        m.add_class::<crate::database::orm::JoinClause>()?;
        m.add_class::<crate::database::orm::SortOrder>()?;
        m.add_class::<crate::database::orm::Pagination>()?;
        m.add_class::<crate::database::orm::QueryBuilder>()?;
        m.add_class::<crate::database::orm::JoinType>()?;
        m.add_class::<crate::database::orm::DMSCPyORMRepository>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_validation_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "validation")?;
        m.add_class::<crate::validation::DMSCValidationError>()?;
        m.add_class::<crate::validation::DMSCValidationResult>()?;
        m.add_class::<crate::validation::DMSCValidationSeverity>()?;
        m.add_class::<crate::validation::DMSCValidatorBuilder>()?;
        m.add_class::<crate::validation::DMSCValidationRunner>()?;
        m.add_class::<crate::validation::DMSCSanitizer>()?;
        m.add_class::<crate::validation::DMSCSanitizationConfig>()?;
        m.add_class::<crate::validation::DMSCSchemaValidator>()?;
        m.add_class::<crate::validation::DMSCValidationModule>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    #[cfg(feature = "protocol")]
    fn create_protocol_module(_parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(_parent.py(), "protocol")?;
        m.add_class::<crate::protocol::DMSCProtocolManager>()?;
        m.add_class::<crate::protocol::DMSCProtocolType>()?;
        m.add_class::<crate::protocol::DMSCProtocolConfig>()?;
        m.add_class::<crate::protocol::DMSCProtocolStatus>()?;
        m.add_class::<crate::protocol::DMSCProtocolStats>()?;
        m.add_class::<crate::protocol::DMSCConnectionState>()?;
        m.add_class::<crate::protocol::DMSCConnectionStats>()?;
        m.add_class::<crate::protocol::DMSCProtocolHealth>()?;
        m.add_class::<crate::protocol::DMSCFrame>()?;
        m.add_class::<crate::protocol::DMSCFrameHeader>()?;
        m.add_class::<crate::protocol::DMSCFrameType>()?;
        m.add_class::<crate::protocol::DMSCConnectionInfo>()?;
        m.add_class::<crate::protocol::DMSCMessageFlags>()?;
        m.add_class::<crate::protocol::DMSCSecurityLevel>()?;
        m.add_class::<crate::protocol::frames::DMSCFrameParser>()?;
        m.add_class::<crate::protocol::frames::DMSCFrameBuilder>()?;
        _parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_grpc_module(_parent: &Bound<'_, PyModule>) -> PyResult<()> {
        #[cfg(all(feature = "grpc", feature = "pyo3"))]
        {
            let m = PyModule::new(_parent.py(), "grpc")?;
            m.add_class::<crate::grpc::DMSCGrpcConfig>()?;
            m.add_class::<crate::grpc::DMSCGrpcStats>()?;
            m.add_class::<crate::grpc::DMSCGrpcServiceRegistryPy>()?;
            m.add_class::<crate::grpc::DMSCGrpcServerPy>()?;
            m.add_class::<crate::grpc::DMSCGrpcClientPy>()?;
            _parent.add_submodule(&m)?;
        }
        #[cfg(not(all(feature = "grpc", feature = "pyo3")))]
        {
            let _ = _parent;
        }
        Ok(())
    }
    
    fn create_ws_module(_parent: &Bound<'_, PyModule>) -> PyResult<()> {
        #[cfg(all(feature = "websocket", feature = "pyo3"))]
        {
            let m = PyModule::new(_parent.py(), "ws")?;
            m.add_class::<crate::ws::DMSCWSServerConfig>()?;
            m.add_class::<crate::ws::DMSCWSEvent>()?;
            m.add_class::<crate::ws::DMSCWSSessionInfo>()?;
            m.add_class::<crate::ws::DMSCWSServerStats>()?;
            m.add_class::<crate::ws::DMSCWSPythonHandler>()?;
            m.add_class::<crate::ws::DMSCWSSessionManagerPy>()?;
            m.add_class::<crate::ws::DMSCWSServerPy>()?;
            m.add_class::<crate::ws::DMSCWSClientConfig>()?;
            m.add_class::<crate::ws::DMSCWSClientStats>()?;
            m.add_class::<crate::ws::DMSCWSClientPy>()?;
            _parent.add_submodule(&m)?;
        }
        #[cfg(not(all(feature = "websocket", feature = "pyo3")))]
        {
            let _ = _parent;
        }
        Ok(())
    }
}
