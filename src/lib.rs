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

//! # Ri (Ri) Library
//! 
//! This is the main entry point for the Ri library, which provides a comprehensive
//! middleware service framework for building enterprise-grade backend applications.
//! 
//! ## Core Modules
//! 
//! Ri is organized into 12 core modules, each responsible for a specific set of functionalities:
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
//! allowing users to import all essential components with a single `use ri::prelude::*;` statement.

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
/// This module provides a single import point for all commonly used Ri components,
/// simplifying application code and reducing the number of import statements.
/// 
/// ## Usage
/// 
/// ```rust,ignore
/// use ri::prelude::*;
/// 
/// #[tokio::main]
/// async fn main() -> RiResult<()> {
///     let app = RiAppBuilder::new()
///         .with_config("config.yaml")?
///         .build()?;
///     
///     app.run(|ctx| async move {
///         ctx.logger().info("service", "Ri service started")?;
///         Ok(())
///     }).await
/// }
/// ```
pub mod prelude {
    // Re-export commonly used public classes here.
    // Only RiXxxXxx format classes are exposed in prelude
    
    /// Application builder for constructing Ri applications
    pub use crate::core::{RiAppBuilder, RiAppRuntime};
    /// Service context providing access to application resources
    pub use crate::core::RiServiceContext;
    /// Module traits for extending Ri functionality
    pub use crate::core::RiModule;
    /// Error type used throughout Ri
    pub use crate::core::RiError;
    /// Result type alias using RiError
    pub use crate::core::RiResult;
    
    /// Lock utilities - Only RiLockError is available in Python
    /// Note: RiLockResult, RwLockExtensions, MutexExtensions, and from_poison_error
    /// are Rust-only types and not exposed in Python bindings
    #[cfg(feature = "pyo3")]
    pub use crate::core::RiLockError;
    
    /// Python module support
    #[cfg(feature = "pyo3")]
    pub use crate::core::{RiPythonModule, RiPythonModuleAdapter, RiPythonServiceModule, RiPythonAsyncServiceModule};
    
    /// Error chain utilities
    #[cfg(feature = "pyo3")]
    pub use crate::core::{RiErrorChain, RiErrorChainIter, RiErrorContext, RiOptionErrorContext};
    
    /// Health check types
    #[cfg(feature = "pyo3")]
    pub use crate::core::{RiHealthStatus, RiHealthCheckResult, RiHealthCheckConfig, RiHealthReport, RiHealthChecker};
    
    /// Service mesh health check types
    #[cfg(feature = "pyo3")]
    pub use crate::service_mesh::health_check::{RiHealthCheckType, RiHealthSummary};
    
    /// Lifecycle management
    #[cfg(feature = "pyo3")]
    pub use crate::core::RiLifecycleObserver;
    
    /// Analytics module
    #[cfg(feature = "pyo3")]
    pub use crate::core::RiLogAnalyticsModule;
    
    /// Secure file system operations
    pub use crate::fs::RiFileSystem;
    
    /// Structured logger with tracing integration
    pub use crate::log::RiLogger;
    /// Log configuration structure
    pub use crate::log::RiLogConfig;
    /// Log level enum
    pub use crate::log::RiLogLevel;
    
    /// Configuration management
    pub use crate::config::RiConfig;
    /// Configuration manager for multi-source configuration
    pub use crate::config::RiConfigManager;
    
    /// Hook bus for managing lifecycle events
    pub use crate::hooks::RiHookBus;
    /// Hook event structure
    pub use crate::hooks::RiHookEvent;
    /// Hook kind enum
    pub use crate::hooks::RiHookKind;
    /// Module lifecycle phase definition
    pub use crate::hooks::RiModulePhase;
    
    /// Main cache module for Ri
    pub use crate::cache::RiCacheModule;
    /// Cache configuration structure
    pub use crate::cache::RiCacheConfig;
    
    /// Main queue module for Ri
    pub use crate::queue::RiQueueModule;
    /// Queue configuration structure
    pub use crate::queue::RiQueueConfig;
    
    /// Main gateway struct implementing the RiModule trait
    pub use crate::gateway::RiGateway;
    /// Configuration for the Ri Gateway
    pub use crate::gateway::RiGatewayConfig;
    /// Route definition for API endpoints
    pub use crate::gateway::RiRoute;
    /// Router for handling request routing
    pub use crate::gateway::RiRouter;
    /// Backend server for load balancing
    pub use crate::gateway::load_balancer::RiBackendServer;
    /// Load balancer server statistics
    pub use crate::gateway::load_balancer::RiLoadBalancerServerStats;
    /// Load balancer implementation
    pub use crate::gateway::load_balancer::RiLoadBalancer;
    /// Load balancing strategy enum
    pub use crate::gateway::load_balancer::RiLoadBalancerStrategy;
    
    /// Main device control module for Ri
    pub use crate::device::RiDeviceControlModule;
    /// Configuration for device discovery
    pub use crate::device::RiDeviceControlConfig;
    /// Scheduling configuration for device control
    pub use crate::device::RiDeviceSchedulingConfig;
    /// Device representation with type, status, and capabilities
    pub use crate::device::RiDevice;
    /// Enum defining supported device types
    pub use crate::device::RiDeviceType;
    
    /// Main authentication module for Ri
    pub use crate::auth::RiAuthModule;
    /// Configuration for the authentication module
    pub use crate::auth::RiAuthConfig;
    
    /// Main service mesh struct implementing the RiModule trait
    pub use crate::service_mesh::RiServiceMesh;
    /// Configuration for the service mesh
    pub use crate::service_mesh::RiServiceMeshConfig;
    /// Statistics for the service mesh
    pub use crate::service_mesh::RiServiceMeshStats;
    /// Service endpoint information
    pub use crate::service_mesh::RiServiceEndpoint;
    /// Service health status enum
    pub use crate::service_mesh::RiServiceHealthStatus;
    
    /// Main observability module for Ri
    pub use crate::observability::RiObservabilityModule;
    /// Configuration for the observability module
    pub use crate::observability::RiObservabilityConfig;
    /// Distributed tracing implementation
    pub use crate::observability::RiTracer;
    /// Metrics collection and aggregation
    pub use crate::observability::RiMetricsRegistry;
    
    /// Database configuration structure
    pub use crate::database::RiDatabaseConfig;
    /// Database type enum
    pub use crate::database::DatabaseType;
    /// Database connection pool
    pub use crate::database::RiDatabasePool;
    /// Database row representation
    pub use crate::database::RiDBRow;
    /// Database query result
    pub use crate::database::RiDBResult;
    
    /// Inter-module RPC coordinator
    pub use crate::module_rpc::RiModuleRPC;
    /// RPC client for making method calls
    pub use crate::module_rpc::RiModuleClient;
    /// RPC endpoint for a module
    pub use crate::module_rpc::RiModuleEndpoint;
    /// RPC method call request
    pub use crate::module_rpc::RiMethodCall;
    /// RPC method call response
    pub use crate::module_rpc::RiMethodResponse;
}

/// Python bindings for Ri
#[cfg(feature = "pyo3")]
pub mod py {
    use pyo3::prelude::*;
    use pyo3::types::PyModule;
    use crate::prelude::*;
    
    /// Initialize the Python module
    #[pymodule]
    pub fn ri(m: &Bound<'_, PyModule>) -> PyResult<()> {
        // Add core types that implement PyClass
        m.add_class::<RiAppBuilder>()?;
        m.add_class::<RiAppRuntime>()?;
        m.add_class::<RiConfig>()?;
        m.add_class::<RiConfigManager>()?;
        m.add_class::<RiError>()?;
        m.add_class::<RiServiceContext>()?;
        
        // Add Python module support
        m.add_class::<crate::core::module::RiPythonModule>()?;
        m.add_class::<crate::core::module::RiPythonModuleAdapter>()?;
        m.add_class::<crate::core::module::RiPythonServiceModule>()?;
        m.add_class::<crate::core::module::RiPythonAsyncServiceModule>()?;
        
        // Add other core types
        m.add_class::<RiLogger>()?;
        m.add_class::<RiLogConfig>()?;
        m.add_class::<RiLogLevel>()?;
        m.add_class::<RiFileSystem>()?;
        m.add_class::<RiHookBus>()?;
        m.add_class::<RiHookEvent>()?;
        m.add_class::<RiHookKind>()?;
        m.add_class::<RiModulePhase>()?;
        
        // Add lock types
        m.add_class::<crate::core::RiLockError>()?;
        
        // Add health check types
        m.add_class::<crate::core::RiHealthStatus>()?;
        m.add_class::<crate::core::RiHealthCheckResult>()?;
        m.add_class::<crate::core::RiHealthCheckConfig>()?;
        m.add_class::<crate::core::RiHealthReport>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthCheckType>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthSummary>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiTrafficManager>()?;
        
        // Add lifecycle types
        m.add_class::<crate::core::RiLifecycleObserver>()?;
        
        // Add analytics types
        m.add_class::<crate::core::RiLogAnalyticsModule>()?;
        
        // Add cache types to main module
        m.add_class::<crate::cache::RiCacheModule>()?;
        m.add_class::<crate::cache::RiCacheManager>()?;
        m.add_class::<crate::cache::RiCacheConfig>()?;
        m.add_class::<crate::cache::RiCacheBackendType>()?;
        m.add_class::<crate::cache::RiCachePolicy>()?;
        m.add_class::<crate::cache::RiCacheStats>()?;
        m.add_class::<crate::cache::RiCachedValue>()?;
        m.add_class::<crate::cache::RiCacheEvent>()?;
        
        // Add queue types to main module
        m.add_class::<crate::queue::RiQueueModule>()?;
        m.add_class::<crate::queue::RiQueueConfig>()?;
        m.add_class::<crate::queue::RiQueueManager>()?;
        m.add_class::<crate::queue::RiQueueMessage>()?;
        m.add_class::<crate::queue::RiQueueStats>()?;
        m.add_class::<crate::queue::RiQueueBackendType>()?;
        m.add_class::<crate::queue::RiRetryPolicy>()?;
        m.add_class::<crate::queue::RiDeadLetterConfig>()?;
        
        // Add gateway types to main module
        m.add_class::<crate::gateway::RiGateway>()?;
        m.add_class::<crate::gateway::RiGatewayConfig>()?;
        m.add_class::<crate::gateway::RiRouter>()?;
        m.add_class::<crate::gateway::RiRoute>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimiter>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimitConfig>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimitStats>()?;
        m.add_class::<crate::gateway::rate_limiter::RiSlidingWindowRateLimiter>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreaker>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerConfig>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerState>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerMetrics>()?;
        
        // Add load balancer types to main module
        m.add_class::<crate::gateway::load_balancer::RiBackendServer>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancerServerStats>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancer>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancerStrategy>()?;
        
        // Add service mesh types to main module
        m.add_class::<crate::service_mesh::RiServiceMesh>()?;
        m.add_class::<crate::service_mesh::RiServiceMeshConfig>()?;
        m.add_class::<crate::service_mesh::RiServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::RiServiceInstance>()?;
        m.add_class::<crate::service_mesh::RiServiceStatus>()?;
        m.add_class::<crate::service_mesh::RiServiceMeshStats>()?;
        m.add_class::<crate::service_mesh::RiServiceEndpoint>()?;
        m.add_class::<crate::service_mesh::RiServiceHealthStatus>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthChecker>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthSummary>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiTrafficManager>()?;
        m.add_class::<crate::service_mesh::RiTrafficRoute>()?;
        m.add_class::<crate::service_mesh::RiMatchCriteria>()?;
        m.add_class::<crate::service_mesh::RiRouteAction>()?;
        m.add_class::<crate::service_mesh::RiWeightedDestination>()?;
        
        // Add auth types to main module
        m.add_class::<crate::auth::RiAuthModule>()?;
        m.add_class::<crate::auth::RiAuthConfig>()?;
        m.add_class::<crate::auth::RiJWTManager>()?;
        m.add_class::<crate::auth::RiJWTClaims>()?;
        m.add_class::<crate::auth::RiJWTValidationOptions>()?;
        m.add_class::<crate::auth::RiSessionManager>()?;
        m.add_class::<crate::auth::RiSession>()?;
        m.add_class::<crate::auth::RiPermissionManager>()?;
        m.add_class::<crate::auth::RiOAuthManager>()?;
        m.add_class::<crate::auth::RiOAuthToken>()?;
        m.add_class::<crate::auth::RiOAuthUserInfo>()?;
        m.add_class::<crate::auth::RiOAuthProvider>()?;
        m.add_class::<crate::auth::RiPermission>()?;
        m.add_class::<crate::auth::RiRole>()?;
        m.add_class::<crate::auth::RiJWTRevocationList>()?;
        m.add_class::<crate::auth::RiRevokedTokenInfo>()?;
        
        // Add observability types to main module
        m.add_class::<crate::observability::RiObservabilityModule>()?;
        m.add_class::<crate::observability::RiObservabilityConfig>()?;
        m.add_class::<crate::observability::RiMetricsRegistry>()?;
        m.add_class::<crate::observability::RiTracer>()?;
        m.add_class::<crate::observability::RiMetricType>()?;
        m.add_class::<crate::observability::RiMetricConfig>()?;
        m.add_class::<crate::observability::RiMetricSample>()?;
        m.add_class::<crate::observability::RiMetric>()?;
        m.add_class::<crate::observability::RiObservabilityData>()?;
        
        // Add system metrics collector types (requires system_info feature)
        #[cfg(feature = "system_info")]
        {
            m.add_class::<crate::observability::RiSystemMetricsCollector>()?;
            m.add_class::<crate::observability::RiSystemMetrics>()?;
            m.add_class::<crate::observability::RiCPUMetrics>()?;
            m.add_class::<crate::observability::RiMemoryMetrics>()?;
            m.add_class::<crate::observability::RiDiskMetrics>()?;
            m.add_class::<crate::observability::RiNetworkMetrics>()?;
        }
        
        // Add validation types to main module
        m.add_class::<crate::validation::RiValidationError>()?;
        m.add_class::<crate::validation::RiValidationResult>()?;
        m.add_class::<crate::validation::RiValidationSeverity>()?;
        m.add_class::<crate::validation::RiValidatorBuilder>()?;
        m.add_class::<crate::validation::RiValidationRunner>()?;
        m.add_class::<crate::validation::RiSanitizer>()?;
        m.add_class::<crate::validation::RiSanitizationConfig>()?;
        m.add_class::<crate::validation::RiSchemaValidator>()?;
        m.add_class::<crate::validation::RiValidationModule>()?;
        
        // Add protocol types to main module
        #[cfg(feature = "protocol")]
        {
            m.add_class::<crate::protocol::RiProtocolManager>()?;
            m.add_class::<crate::protocol::RiProtocolType>()?;
            m.add_class::<crate::protocol::RiProtocolConfig>()?;
            m.add_class::<crate::protocol::RiProtocolStatus>()?;
            m.add_class::<crate::protocol::RiProtocolStats>()?;
            m.add_class::<crate::protocol::RiConnectionState>()?;
            m.add_class::<crate::protocol::RiConnectionStats>()?;
            m.add_class::<crate::protocol::RiProtocolHealth>()?;
            m.add_class::<crate::protocol::RiFrame>()?;
            m.add_class::<crate::protocol::RiFrameHeader>()?;
            m.add_class::<crate::protocol::RiFrameType>()?;
            m.add_class::<crate::protocol::RiConnectionInfo>()?;
            m.add_class::<crate::protocol::RiMessageFlags>()?;
            m.add_class::<crate::protocol::RiSecurityLevel>()?;
            m.add_class::<crate::protocol::frames::RiFrameParser>()?;
            m.add_class::<crate::protocol::frames::RiFrameBuilder>()?;
        }

        // Add database types to main module
        m.add_class::<crate::database::RiDatabaseConfig>()?;
        m.add_class::<crate::database::RiDatabasePool>()?;
        m.add_class::<crate::database::RiDBRow>()?;
        m.add_class::<crate::database::RiDBResult>()?;
        m.add_class::<crate::database::orm::RiPyORMRepository>()?;
        m.add_class::<crate::database::DynamicPoolConfig>()?;
        m.add_class::<crate::database::DatabaseMetrics>()?;

        // Add grpc types to main module
        #[cfg(all(feature = "grpc", feature = "pyo3"))]
        {
            m.add_class::<crate::grpc::RiGrpcConfig>()?;
            m.add_class::<crate::grpc::RiGrpcStats>()?;
            m.add_class::<crate::grpc::RiGrpcServiceRegistryPy>()?;
            m.add_class::<crate::grpc::RiGrpcServerPy>()?;
            m.add_class::<crate::grpc::RiGrpcClientPy>()?;
        }

        // Add websocket types to main module
        #[cfg(all(feature = "websocket", feature = "pyo3"))]
        {
            m.add_class::<crate::ws::RiWSServerConfig>()?;
            m.add_class::<crate::ws::RiWSEvent>()?;
            m.add_class::<crate::ws::RiWSSessionInfo>()?;
            m.add_class::<crate::ws::RiWSServerStats>()?;
            m.add_class::<crate::ws::RiWSPythonHandler>()?;
            m.add_class::<crate::ws::RiWSSessionManagerPy>()?;
            m.add_class::<crate::ws::RiWSServerPy>()?;
            m.add_class::<crate::ws::RiWSClientConfig>()?;
            m.add_class::<crate::ws::RiWSClientStats>()?;
            m.add_class::<crate::ws::RiWSClientPy>()?;
        }

        // Add module_rpc types to main module
        m.add_class::<crate::module_rpc::RiModuleRPC>()?;
        m.add_class::<crate::module_rpc::RiModuleClient>()?;
        m.add_class::<crate::module_rpc::RiModuleEndpoint>()?;
        m.add_class::<crate::module_rpc::RiMethodCall>()?;
        m.add_class::<crate::module_rpc::RiMethodResponse>()?;
        
        // Add device types to main module
        m.add_class::<crate::device::RiDeviceControlModule>()?;
        m.add_class::<crate::device::RiDevice>()?;
        m.add_class::<crate::device::RiDeviceType>()?;
        m.add_class::<crate::device::RiDeviceStatus>()?;
        m.add_class::<crate::device::RiDeviceCapabilities>()?;
        m.add_class::<crate::device::RiDeviceHealthMetrics>()?;
        m.add_class::<crate::device::RiDeviceController>()?;
        m.add_class::<crate::device::RiDeviceConfig>()?;
        m.add_class::<crate::device::RiDeviceControlConfig>()?;
        m.add_class::<crate::device::RiDeviceSchedulingConfig>()?;
        m.add_class::<crate::device::RiNetworkDeviceInfo>()?;
        m.add_class::<crate::device::RiDiscoveryResult>()?;
        m.add_class::<crate::device::RiResourceRequest>()?;
        m.add_class::<crate::device::RiResourceAllocation>()?;
        m.add_class::<crate::device::RiRequestSlaClass>()?;
        m.add_class::<crate::device::RiResourceWeights>()?;
        m.add_class::<crate::device::RiAffinityRules>()?;
        m.add_class::<crate::device::RiResourcePoolStatus>()?;
        m.add_class::<crate::device::pool::RiResourcePool>()?;
        m.add_class::<crate::device::pool::RiResourcePoolConfig>()?;
        m.add_class::<crate::device::pool::RiResourcePoolStatistics>()?;
        m.add_class::<crate::device::pool::RiResourcePoolManager>()?;
        m.add_class::<crate::device::pool::RiConnectionPoolStatistics>()?;
        m.add_class::<crate::device::scheduler::RiResourceScheduler>()?;
        m.add_class::<crate::device::scheduler::RiDeviceScheduler>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingPolicy>()?;
        m.add_class::<crate::device::scheduler::RiAllocationRecord>()?;
        m.add_class::<crate::device::scheduler::RiAllocationRequest>()?;
        m.add_class::<crate::device::scheduler::RiAllocationStatistics>()?;
        m.add_class::<crate::device::scheduler::RiDeviceTypeStatistics>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingRecommendation>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingRecommendationType>()?;
        m.add_class::<crate::device::RiDeviceDiscoveryEngine>()?;
        
        // Add database migration type
        m.add_class::<crate::database::RiDatabaseMigration>()?;
        
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
        
        m.add_class::<crate::device::RiDeviceControlModule>()?;
        m.add_class::<crate::device::RiDevice>()?;
        m.add_class::<crate::device::RiDeviceType>()?;
        m.add_class::<crate::device::RiDeviceStatus>()?;
        m.add_class::<crate::device::RiDeviceCapabilities>()?;
        m.add_class::<crate::device::RiDeviceHealthMetrics>()?;
        m.add_class::<crate::device::RiDeviceController>()?;
        m.add_class::<crate::device::RiDeviceConfig>()?;
        m.add_class::<crate::device::RiDeviceControlConfig>()?;
        m.add_class::<crate::device::RiDeviceSchedulingConfig>()?;
        m.add_class::<crate::device::RiNetworkDeviceInfo>()?;
        m.add_class::<crate::device::RiDiscoveryResult>()?;
        m.add_class::<crate::device::RiResourceRequest>()?;
        m.add_class::<crate::device::RiResourceAllocation>()?;
        m.add_class::<crate::device::RiRequestSlaClass>()?;
        m.add_class::<crate::device::RiResourceWeights>()?;
        m.add_class::<crate::device::RiAffinityRules>()?;
        m.add_class::<crate::device::pool::RiResourcePool>()?;
        m.add_class::<crate::device::pool::RiResourcePoolConfig>()?;
        m.add_class::<crate::device::pool::RiResourcePoolStatistics>()?;
        m.add_class::<crate::device::pool::RiResourcePoolManager>()?;
        m.add_class::<crate::device::pool::RiConnectionPoolStatistics>()?;
        m.add_class::<crate::device::scheduler::RiResourceScheduler>()?;
        m.add_class::<crate::device::scheduler::RiDeviceScheduler>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingPolicy>()?;
        m.add_class::<crate::device::scheduler::RiAllocationRecord>()?;
        m.add_class::<crate::device::scheduler::RiAllocationRequest>()?;
        m.add_class::<crate::device::scheduler::RiAllocationStatistics>()?;
        m.add_class::<crate::device::scheduler::RiDeviceTypeStatistics>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingRecommendation>()?;
        m.add_class::<crate::device::scheduler::RiSchedulingRecommendationType>()?;
        m.add_class::<crate::device::RiDeviceDiscoveryEngine>()?;
        
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_cache_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "cache")?;
        
        // Add cache types to the cache module
        m.add_class::<crate::cache::RiCacheModule>()?;
        m.add_class::<crate::cache::RiCacheManager>()?;
        m.add_class::<crate::cache::RiCacheConfig>()?;
        m.add_class::<crate::cache::RiCacheBackendType>()?;
        m.add_class::<crate::cache::RiCachePolicy>()?;
        m.add_class::<crate::cache::RiCacheStats>()?;
        m.add_class::<crate::cache::RiCachedValue>()?;
        m.add_class::<crate::cache::RiCacheEvent>()?;
        
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_fs_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "fs")?;
        m.add_class::<crate::fs::RiFileSystem>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_hooks_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "hooks")?;
        m.add_class::<crate::hooks::RiHookKind>()?;
        m.add_class::<crate::hooks::RiModulePhase>()?;
        m.add_class::<crate::hooks::RiHookEvent>()?;
        m.add_class::<crate::hooks::RiHookBus>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_observability_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "observability")?;
        m.add_class::<crate::observability::RiObservabilityModule>()?;
        m.add_class::<crate::observability::RiObservabilityConfig>()?;
        m.add_class::<crate::observability::RiObservabilityData>()?;
        m.add_class::<crate::observability::RiMetricsRegistry>()?;
        m.add_class::<crate::observability::RiTracer>()?;
        m.add_class::<crate::observability::RiMetricType>()?;
        m.add_class::<crate::observability::RiMetricConfig>()?;
        m.add_class::<crate::observability::RiMetricSample>()?;
        m.add_class::<crate::observability::RiMetric>()?;
        #[cfg(feature = "system_info")]
        {
            m.add_class::<crate::observability::RiSystemMetricsCollector>()?;
            m.add_class::<crate::observability::RiSystemMetrics>()?;
            m.add_class::<crate::observability::RiCPUMetrics>()?;
            m.add_class::<crate::observability::RiMemoryMetrics>()?;
            m.add_class::<crate::observability::RiDiskMetrics>()?;
            m.add_class::<crate::observability::RiNetworkMetrics>()?;
        }
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_queue_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "queue")?;
        m.add_class::<crate::queue::RiQueueModule>()?;
        m.add_class::<crate::queue::RiQueueConfig>()?;
        m.add_class::<crate::queue::RiQueueManager>()?;
        m.add_class::<crate::queue::RiQueueMessage>()?;
        m.add_class::<crate::queue::RiQueueStats>()?;
        m.add_class::<crate::queue::RiQueueBackendType>()?;
        m.add_class::<crate::queue::RiRetryPolicy>()?;
        m.add_class::<crate::queue::RiDeadLetterConfig>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_gateway_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "gateway")?;
        m.add_class::<crate::gateway::RiGateway>()?;
        m.add_class::<crate::gateway::RiGatewayConfig>()?;
        m.add_class::<crate::gateway::RiRoute>()?;
        m.add_class::<crate::gateway::RiRouter>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimiter>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimitConfig>()?;
        m.add_class::<crate::gateway::rate_limiter::RiRateLimitStats>()?;
        m.add_class::<crate::gateway::rate_limiter::RiSlidingWindowRateLimiter>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreaker>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerConfig>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerState>()?;
        m.add_class::<crate::gateway::circuit_breaker::RiCircuitBreakerMetrics>()?;
        m.add_class::<crate::gateway::load_balancer::RiBackendServer>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancerServerStats>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancer>()?;
        m.add_class::<crate::gateway::load_balancer::RiLoadBalancerStrategy>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_service_mesh_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "service_mesh")?;
        m.add_class::<crate::service_mesh::RiServiceMesh>()?;
        m.add_class::<crate::service_mesh::RiServiceMeshConfig>()?;
        m.add_class::<crate::service_mesh::RiServiceDiscovery>()?;
        m.add_class::<crate::service_mesh::RiServiceInstance>()?;
        m.add_class::<crate::service_mesh::RiServiceStatus>()?;
        m.add_class::<crate::service_mesh::RiServiceMeshStats>()?;
        m.add_class::<crate::service_mesh::RiServiceEndpoint>()?;
        m.add_class::<crate::service_mesh::RiServiceHealthStatus>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthChecker>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthSummary>()?;
        m.add_class::<crate::service_mesh::health_check::RiHealthCheckType>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiTrafficManager>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiTrafficRoute>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiMatchCriteria>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiRouteAction>()?;
        m.add_class::<crate::service_mesh::traffic_management::RiWeightedDestination>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_auth_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "auth")?;
        m.add_class::<crate::auth::RiAuthModule>()?;
        m.add_class::<crate::auth::RiAuthConfig>()?;
        m.add_class::<crate::auth::RiJWTManager>()?;
        m.add_class::<crate::auth::RiJWTClaims>()?;
        m.add_class::<crate::auth::RiJWTValidationOptions>()?;
        m.add_class::<crate::auth::RiSessionManager>()?;
        m.add_class::<crate::auth::RiSession>()?;
        m.add_class::<crate::auth::RiPermissionManager>()?;
        m.add_class::<crate::auth::RiPermission>()?;
        m.add_class::<crate::auth::RiRole>()?;
        m.add_class::<crate::auth::RiOAuthManager>()?;
        m.add_class::<crate::auth::RiOAuthToken>()?;
        m.add_class::<crate::auth::RiOAuthUserInfo>()?;
        m.add_class::<crate::auth::RiOAuthProvider>()?;
        m.add_class::<crate::auth::RiJWTRevocationList>()?;
        m.add_class::<crate::auth::RiRevokedTokenInfo>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_database_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "database")?;
        m.add_class::<crate::database::RiDatabaseConfig>()?;
        m.add_class::<crate::database::RiDatabasePool>()?;
        m.add_class::<crate::database::RiDBRow>()?;
        m.add_class::<crate::database::RiDBResult>()?;
        m.add_class::<crate::database::DynamicPoolConfig>()?;
        m.add_class::<crate::database::DatabaseMetrics>()?;
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
        m.add_class::<crate::database::orm::RiPyORMRepository>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_validation_module(parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(parent.py(), "validation")?;
        m.add_class::<crate::validation::RiValidationError>()?;
        m.add_class::<crate::validation::RiValidationResult>()?;
        m.add_class::<crate::validation::RiValidationSeverity>()?;
        m.add_class::<crate::validation::RiValidatorBuilder>()?;
        m.add_class::<crate::validation::RiValidationRunner>()?;
        m.add_class::<crate::validation::RiSanitizer>()?;
        m.add_class::<crate::validation::RiSanitizationConfig>()?;
        m.add_class::<crate::validation::RiSchemaValidator>()?;
        m.add_class::<crate::validation::RiValidationModule>()?;
        parent.add_submodule(&m)?;
        Ok(())
    }
    
    #[cfg(feature = "protocol")]
    fn create_protocol_module(_parent: &Bound<'_, PyModule>) -> PyResult<()> {
        let m = PyModule::new(_parent.py(), "protocol")?;
        m.add_class::<crate::protocol::RiProtocolManager>()?;
        m.add_class::<crate::protocol::RiProtocolType>()?;
        m.add_class::<crate::protocol::RiProtocolConfig>()?;
        m.add_class::<crate::protocol::RiProtocolStatus>()?;
        m.add_class::<crate::protocol::RiProtocolStats>()?;
        m.add_class::<crate::protocol::RiConnectionState>()?;
        m.add_class::<crate::protocol::RiConnectionStats>()?;
        m.add_class::<crate::protocol::RiProtocolHealth>()?;
        m.add_class::<crate::protocol::RiFrame>()?;
        m.add_class::<crate::protocol::RiFrameHeader>()?;
        m.add_class::<crate::protocol::RiFrameType>()?;
        m.add_class::<crate::protocol::RiConnectionInfo>()?;
        m.add_class::<crate::protocol::RiMessageFlags>()?;
        m.add_class::<crate::protocol::RiSecurityLevel>()?;
        m.add_class::<crate::protocol::frames::RiFrameParser>()?;
        m.add_class::<crate::protocol::frames::RiFrameBuilder>()?;
        _parent.add_submodule(&m)?;
        Ok(())
    }
    
    fn create_grpc_module(_parent: &Bound<'_, PyModule>) -> PyResult<()> {
        #[cfg(all(feature = "grpc", feature = "pyo3"))]
        {
            let m = PyModule::new(_parent.py(), "grpc")?;
            m.add_class::<crate::grpc::RiGrpcConfig>()?;
            m.add_class::<crate::grpc::RiGrpcStats>()?;
            m.add_class::<crate::grpc::RiGrpcServiceRegistryPy>()?;
            m.add_class::<crate::grpc::RiGrpcServerPy>()?;
            m.add_class::<crate::grpc::RiGrpcClientPy>()?;
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
            m.add_class::<crate::ws::RiWSServerConfig>()?;
            m.add_class::<crate::ws::RiWSEvent>()?;
            m.add_class::<crate::ws::RiWSSessionInfo>()?;
            m.add_class::<crate::ws::RiWSServerStats>()?;
            m.add_class::<crate::ws::RiWSPythonHandler>()?;
            m.add_class::<crate::ws::RiWSSessionManagerPy>()?;
            m.add_class::<crate::ws::RiWSServerPy>()?;
            m.add_class::<crate::ws::RiWSClientConfig>()?;
            m.add_class::<crate::ws::RiWSClientStats>()?;
            m.add_class::<crate::ws::RiWSClientPy>()?;
            _parent.add_submodule(&m)?;
        }
        #[cfg(not(all(feature = "websocket", feature = "pyo3")))]
        {
            let _ = _parent;
        }
        Ok(())
    }
}
