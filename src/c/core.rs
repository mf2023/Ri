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

//! # Core Module C API
//!
//! This module provides C language bindings for Ri's core application infrastructure.
//! The core module serves as the foundation for building Ri applications, providing
//! application lifecycle management, configuration handling, and initialization routines.
//! This C API enables C/C++ applications to leverage Ri's powerful application builder
//! and configuration management capabilities.
//!
//! ## Module Architecture
//!
//! The core module comprises two essential components that form the backbone of any
//! Ri application:
//!
//! - **RiAppBuilder**: Fluent builder pattern implementation for constructing Ri
//!   applications with type-safe configuration. The builder supports registration of
//!   modules, services, and middleware components through a declarative API. It handles
//!   dependency injection, service discovery, and lifecycle coordination across all
//!   registered components. The builder produces a fully initialized RiApp instance
//!   ready for execution.
//!
//! - **RiConfig**: Unified configuration management interface supporting multiple
//!   configuration sources including environment variables, command-line arguments,
//!   configuration files (YAML, TOML, JSON), and remote configuration services.
//!   The configuration system provides type-safe value retrieval with automatic
//!   type conversion, validation, and hot-reload capabilities for dynamic
//!   configuration updates in running applications.
//!
//! ## Application Lifecycle
//!
//! Ri applications follow a well-defined lifecycle:
//!
//! 1. **Initialization Phase**: Application builder creates and configures components.
//!    Services are registered, dependencies are wired, and configuration is loaded.
//!
//! 2. **Startup Phase**: All registered services and modules are initialized in
//!    dependency order. Health checks verify component readiness.
//!
//! 3. **Running Phase**: Application enters active state, processing requests or
//!    executing scheduled tasks. Components operate according to their configured
//!    behavior.
//!
//! 4. **Shutdown Phase**: Graceful shutdown initiates component cleanup in reverse
//!    dependency order. Resources are released, connections are closed, and state
//!    is persisted as needed.
//!
//! ## Configuration System
//!
//! The configuration module implements a hierarchical configuration model:
//!
//! - **Sources**: Environment variables override file-based settings, which override
//!   default values. Multiple sources can coexist with configurable precedence.
//!
//! - **Formats**: Native support for YAML, TOML, JSON configuration files. Each
//!   format has optimized parsing and schema validation.
//!
//! - **Validation**: Configuration values undergo validation against defined schemas.
//!   Type mismatches, missing required fields, and constraint violations are detected.
//!
//! - **Hot Reload**: Configuration files are monitored for changes. Updates are
//!   applied atomically without application restart. Subscribers are notified of changes.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Objects must not be used after being freed
//! - Null pointer checks are required before all operations
//!
//! ## Thread Safety
//!
//! The underlying Rust implementations are thread-safe:
//!
//! - Application builder operations require external synchronization
//! - Configuration reads are concurrent-safe after initialization
//! - Configuration writes require exclusive access
//!
//! ## Usage Example
//!
//! ```c
//! // Create application configuration
//! CRiConfig* config = ri_config_new();
//!
//! // Load configuration from file
//! int result = ri_config_load_file(config, "config.yaml");
//!
//! // Get configuration value
//! char* value = ri_config_get_string(config, "database.url");
//!
//! // Create application builder
//! CRiAppBuilder* builder = ri_app_builder_new();
//!
//! // Configure builder with configuration
//! ri_app_builder_configure(builder, config);
//!
//! // Build and run application
//! ri_app_builder_build(builder);
//!
//! // Cleanup
//! ri_app_builder_free(builder);
//! ri_config_free(config);
//! ri_string_free(value);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following Ri components:
//!
//! - `crate::core`: Rust core module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The core module is always enabled as it provides fundamental infrastructure
//! required by all other Ri components.

use crate::prelude::{RiAppBuilder, RiConfig};
use crate::core::{RiServiceContext, RiHealthStatus, RiHealthCheckResult, RiHealthCheckConfig};
use crate::core::error_chain::RiErrorChain;
use crate::core::lock::RiLockError;
use std::ffi::{c_char, CString, c_int};
use std::time::SystemTime;

/// Opaque C wrapper structure for RiAppBuilder.
///
/// Provides C-compatible interface to the Rust application builder implementation.
/// The builder uses the fluent builder pattern to construct Ri applications with
/// proper dependency injection and lifecycle management.
///
/// # Builder Responsibilities
///
/// The application builder handles:
///
/// - Service registration and dependency management
/// - Middleware composition and ordering
/// - Configuration propagation to components
/// - Lifecycle event registration
/// - Application initialization and startup coordination
///
/// # Builder Pattern
///
/// The builder implements a fluent interface allowing method chaining:
///
/// ```c
/// ri_app_builder_register_module(builder, module_a);
/// ri_app_builder_register_module(builder, module_b);
/// ri_app_builder_configure(builder, config);
/// ri_app_builder_with_middleware(builder, middleware_1);
/// ri_app_builder_with_middleware(builder, middleware_2);
/// ```
///
/// # Thread Safety
///
/// The builder is not thread-safe. All builder operations should occur from a
/// single thread before application startup. Concurrent builder access results
/// in undefined behavior.
#[repr(C)]
pub struct CRiAppBuilder {
    inner: RiAppBuilder,
}

/// Opaque C wrapper structure for RiConfig.
///
/// Provides C-compatible interface to the unified configuration management system.
/// The configuration object provides type-safe access to configuration values
/// from multiple sources with automatic type conversion and validation.
///
/// # Configuration Sources
///
/// The configuration system aggregates values from:
///
/// - Default values defined in code
/// - Configuration files (YAML, TOML, JSON)
/// - Environment variables
/// - Command-line arguments
/// - Remote configuration services (etcd, Consul)
///
/// # Value Resolution
///
/// Configuration values are resolved using precedence order:
///
/// 1. Environment variables (highest priority)
/// 2. Command-line arguments
/// 3. Remote configuration
/// 4. Configuration files
/// 5. Default values (lowest priority)
///
/// # Type Safety
///
/// The configuration system provides type-safe value retrieval:
///
/// - get_string(): Retrieve string values
/// - get_int(): Retrieve integer values with automatic conversion
/// - get_bool(): Retrieve boolean values
/// - get_float(): Retrieve floating-point values
/// - get_list(): Retrieve array values
///
/// Invalid type requests return default values or trigger validation errors.
#[repr(C)]
pub struct CRiConfig {
    inner: RiConfig,
}

/// Creates a new CRiAppBuilder instance.
///
/// Initializes an empty application builder ready for component registration.
/// The builder starts with default configuration and no registered modules.
///
/// # Returns
///
/// Pointer to newly allocated CRiAppBuilder on success. Never returns NULL
/// as the implementation uses infallible construction. The returned pointer
/// must be freed using ri_app_builder_free().
///
/// # Initial State
///
/// A newly created builder:
///
/// - Has no registered modules
/// - Has no configured middleware
/// - Uses default application configuration
/// - Has not initiated application startup
///
/// # Usage Pattern
///
/// ```c
/// CRiAppBuilder* builder = ri_app_builder_new();
/// if (builder == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
///
/// // Register modules and configure
/// ri_app_builder_register_module(builder, http_module);
/// ri_app_builder_register_module(builder, database_module);
///
/// // Build and run
/// ri_app_builder_build(builder);
///
/// // Cleanup
/// ri_app_builder_free(builder);
/// ```
#[no_mangle]
pub extern "C" fn ri_app_builder_new() -> *mut CRiAppBuilder {
    let builder = CRiAppBuilder {
        inner: RiAppBuilder::new(),
    };
    Box::into_raw(Box::new(builder))
}

/// Frees a previously allocated CRiAppBuilder instance.
///
/// Releases all memory associated with the builder including any registered
/// configurations, module references, or internal state. After this function
/// returns, the pointer becomes invalid.
///
/// # Parameters
///
/// - `builder`: Pointer to CRiAppBuilder to free. If NULL, the function
///   returns immediately without error.
///
/// # Behavior
///
/// The destructor:
///
/// - Clears all registered modules
/// - Releases internal configuration
/// - Frees allocated memory
/// - Invalidates the pointer
///
/// # Safety
///
/// This function is safe to call with NULL. Calling with a pointer that has
/// already been freed results in undefined behavior.
#[no_mangle]
pub extern "C" fn ri_app_builder_free(builder: *mut CRiAppBuilder) {
    if !builder.is_null() {
        unsafe {
            let _ = Box::from_raw(builder);
        }
    }
}

/// Creates a new CRiConfig instance.
///
/// Initializes an empty configuration object with no loaded sources.
/// The configuration starts with default values and requires explicit
/// loading from configuration sources.
///
/// # Returns
///
/// Pointer to newly allocated CRiConfig on success. Never returns NULL
/// as the implementation uses infallible construction. The returned pointer
/// must be freed using ri_config_free().
///
/// # Initial State
///
/// A newly created configuration:
///
/// - Contains no loaded values
/// - Has default values for all known keys
/// - Has no active configuration sources
/// - Is not watching for changes
///
/// # Usage Pattern
///
/// ```c
/// CRiConfig* config = ri_config_new();
/// if (config == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
///
/// // Load from file
/// int load_result = ri_config_load_file(config, "config.yaml");
/// if (load_result != 0) {
///     // Handle load failure
/// }
///
/// // Access configuration values
/// char* host = ri_config_get_string(config, "server.host");
/// int port = ri_config_get_int(config, "server.port");
///
/// // Cleanup
/// ri_config_free(config);
/// ri_string_free(host);
/// ```
#[no_mangle]
pub extern "C" fn ri_config_new() -> *mut CRiConfig {
    let config = CRiConfig {
        inner: RiConfig::new(),
    };
    Box::into_raw(Box::new(config))
}

/// Frees a previously allocated CRiConfig instance.
///
/// Releases all memory associated with the configuration including any
/// loaded values, watched files, or internal caches. After this function
/// returns, the pointer becomes invalid.
///
/// # Parameters
///
/// - `config`: Pointer to CRiConfig to free. If NULL, the function returns
///   immediately without error.
///
/// # Behavior
///
/// The destructor:
///
/// - Clears all configuration values
/// - Stops file watchers if active
/// - Releases internal caches
/// - Invalidates the pointer
///
/// # Safety
///
/// This function is safe to call with NULL. Calling with a pointer that has
/// already been freed results in undefined behavior.
#[no_mangle]
pub extern "C" fn ri_config_free(config: *mut CRiConfig) {
    if !config.is_null() {
        unsafe {
            let _ = Box::from_raw(config);
        }
    }
}

/// Retrieves a string configuration value by key.
///
/// Looks up the specified key in the configuration hierarchy and returns
/// the associated string value if found. The function performs type-safe
/// retrieval with automatic conversion from compatible types.
///
/// # Parameters
///
/// - `config`: Pointer to CRiConfig containing the configuration. Must not
///   be NULL. If NULL, the function returns NULL.
/// - `key`: Pointer to null-terminated C string specifying the configuration key.
///   Keys use dot notation for hierarchical access (e.g., "database.connections.max").
///   Must not be NULL. If NULL, the function returns NULL.
///
/// # Returns
///
/// Pointer to newly allocated C string containing the configuration value on
/// success. The caller is responsible for freeing the returned string using
/// ri_string_free(). Returns NULL if:
///
/// - `config` is NULL
/// - `key` is NULL
/// - Key does not exist in configuration
/// - Value exists but is not a string type
/// - String conversion fails (invalid UTF-8)
///
/// # Key Format
///
/// Configuration keys support hierarchical access:
///
/// - Simple keys: "timeout"
/// - Nested keys: "server.http.port"
/// - Array indices: "servers.0.host"
///
/// # Example
///
/// ```c
/// char* database_url = ri_config_get_string(config, "database.url");
/// if (database_url != NULL) {
///     printf("Database URL: %s\n", database_url);
///     ri_string_free(database_url);
/// } else {
///     printf("Database URL not configured\n");
/// }
/// ```
///
/// # Memory Management
///
/// The returned string is newly allocated. Callers must release it using
/// ri_string_free() to prevent memory leaks. Do not use free() directly.
#[no_mangle]
pub extern "C" fn ri_config_get_string(
    config: *mut CRiConfig,
    key: *const c_char,
) -> *mut c_char {
    if config.is_null() || key.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let c = &(*config).inner;
        let key_str = match std::ffi::CStr::from_ptr(key).to_str() {
            Ok(s) => s,
            Err(_) => return std::ptr::null_mut(),
        };

        match c.get_str(key_str) {
            Some(val) => match CString::new(val) {
                Ok(c_str) => c_str.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            None => std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct CRiServiceContext {
    inner: RiServiceContext,
}

#[no_mangle]
pub extern "C" fn ri_service_context_new() -> *mut CRiServiceContext {
    match RiServiceContext::new_default() {
        Ok(ctx) => {
            let context = CRiServiceContext { inner: ctx };
            Box::into_raw(Box::new(context))
        }
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn ri_service_context_free(ctx: *mut CRiServiceContext) {
    if !ctx.is_null() {
        unsafe {
            let _ = Box::from_raw(ctx);
        }
    }
}

#[repr(C)]
pub struct CRiHealthStatus {
    pub status: c_int,
}

pub const RI_HEALTH_STATUS_HEALTHY: c_int = 0;
pub const RI_HEALTH_STATUS_DEGRADED: c_int = 1;
pub const RI_HEALTH_STATUS_UNHEALTHY: c_int = 2;
pub const RI_HEALTH_STATUS_UNKNOWN: c_int = 3;

#[no_mangle]
pub extern "C" fn ri_health_status_new(status: c_int) -> CRiHealthStatus {
    CRiHealthStatus { status }
}

#[no_mangle]
pub extern "C" fn ri_health_status_is_healthy(status: *const CRiHealthStatus) -> bool {
    if status.is_null() {
        return false;
    }
    unsafe {
        matches!((*status).status, RI_HEALTH_STATUS_HEALTHY | RI_HEALTH_STATUS_DEGRADED)
    }
}

#[no_mangle]
pub extern "C" fn ri_health_status_requires_attention(status: *const CRiHealthStatus) -> bool {
    if status.is_null() {
        return false;
    }
    unsafe { (*status).status == RI_HEALTH_STATUS_UNHEALTHY }
}

#[repr(C)]
pub struct CRiHealthCheckResult {
    pub name: *mut c_char,
    pub status: CRiHealthStatus,
    pub message: *mut c_char,
    pub timestamp_secs: u64,
    pub timestamp_nanos: u64,
    pub duration_secs: u64,
    pub duration_nanos: u64,
}

#[no_mangle]
pub extern "C" fn ri_health_check_result_new(
    name: *const c_char,
    status: c_int,
    message: *const c_char,
) -> *mut CRiHealthCheckResult {
    if name.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let name_str = match std::ffi::CStr::from_ptr(name).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };

        let msg_str = if message.is_null() {
            None
        } else {
            match std::ffi::CStr::from_ptr(message).to_str() {
                Ok(s) => Some(s.to_string()),
                Err(_) => None,
            }
        };

        let rust_status = match status {
            RI_HEALTH_STATUS_HEALTHY => RiHealthStatus::Healthy,
            RI_HEALTH_STATUS_DEGRADED => RiHealthStatus::Degraded,
            RI_HEALTH_STATUS_UNHEALTHY => RiHealthStatus::Unhealthy,
            _ => RiHealthStatus::Unknown,
        };

        let result = RiHealthCheckResult {
            name: name_str,
            status: rust_status,
            message: msg_str,
            timestamp: SystemTime::now(),
            duration: std::time::Duration::ZERO,
        };

        let c_result = convert_health_check_result_to_c(result);
        Box::into_raw(Box::new(c_result))
    }
}

fn convert_health_check_result_to_c(result: RiHealthCheckResult) -> CRiHealthCheckResult {
    let name = match CString::new(result.name) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    };

    let message = match result.message {
        Some(msg) => match CString::new(msg) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        },
        None => std::ptr::null_mut(),
    };

    let status = CRiHealthStatus {
        status: match result.status {
            RiHealthStatus::Healthy => RI_HEALTH_STATUS_HEALTHY,
            RiHealthStatus::Degraded => RI_HEALTH_STATUS_DEGRADED,
            RiHealthStatus::Unhealthy => RI_HEALTH_STATUS_UNHEALTHY,
            RiHealthStatus::Unknown => RI_HEALTH_STATUS_UNKNOWN,
        },
    };

    let duration = result.duration;
    let timestamp = result.timestamp
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();

    CRiHealthCheckResult {
        name,
        status,
        message,
        timestamp_secs: timestamp.as_secs(),
        timestamp_nanos: timestamp.subsec_nanos() as u64,
        duration_secs: duration.as_secs(),
        duration_nanos: duration.subsec_nanos() as u64,
    }
}

#[no_mangle]
pub extern "C" fn ri_health_check_result_free(result: *mut CRiHealthCheckResult) {
    if !result.is_null() {
        unsafe {
            if !(*result).name.is_null() {
                let _ = CString::from_raw((*result).name);
            }
            if !(*result).message.is_null() {
                let _ = CString::from_raw((*result).message);
            }
            let _ = Box::from_raw(result);
        }
    }
}

#[repr(C)]
pub struct CRiHealthCheckConfig {
    pub check_interval_secs: u64,
    pub timeout_secs: u64,
    pub failure_threshold: u32,
    pub success_threshold: u32,
    pub enabled: bool,
}

#[no_mangle]
pub extern "C" fn ri_health_check_config_new(
    check_interval_secs: u64,
    timeout_secs: u64,
    failure_threshold: u32,
    success_threshold: u32,
    enabled: bool,
) -> CRiHealthCheckConfig {
    CRiHealthCheckConfig {
        check_interval_secs,
        timeout_secs,
        failure_threshold,
        success_threshold,
        enabled,
    }
}

#[no_mangle]
pub extern "C" fn ri_health_check_config_default() -> CRiHealthCheckConfig {
    let default_config = RiHealthCheckConfig::default();
    CRiHealthCheckConfig {
        check_interval_secs: default_config.check_interval.as_secs(),
        timeout_secs: default_config.timeout.as_secs(),
        failure_threshold: default_config.failure_threshold,
        success_threshold: default_config.success_threshold,
        enabled: default_config.enabled,
    }
}

#[repr(C)]
pub struct CRiHealthReport {
    pub overall_status: CRiHealthStatus,
    pub total_components: usize,
    pub healthy_count: usize,
    pub degraded_count: usize,
    pub unhealthy_count: usize,
    pub unknown_count: usize,
}

#[no_mangle]
pub extern "C" fn ri_health_report_new() -> *mut CRiHealthReport {
    let report = CRiHealthReport {
        overall_status: CRiHealthStatus { status: RI_HEALTH_STATUS_UNKNOWN },
        total_components: 0,
        healthy_count: 0,
        degraded_count: 0,
        unhealthy_count: 0,
        unknown_count: 0,
    };
    Box::into_raw(Box::new(report))
}

#[no_mangle]
pub extern "C" fn ri_health_report_free(report: *mut CRiHealthReport) {
    if !report.is_null() {
        unsafe {
            let _ = Box::from_raw(report);
        }
    }
}

#[repr(C)]
pub struct CRiErrorChain {
    inner: RiErrorChain,
}

#[no_mangle]
pub extern "C" fn ri_error_chain_new(message: *const c_char) -> *mut CRiErrorChain {
    if message.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let msg = match std::ffi::CStr::from_ptr(message).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };

        let chain = crate::core::error_chain::utils::chain_from_msg(msg);
        Box::into_raw(Box::new(CRiErrorChain { inner: chain }))
    }
}

#[no_mangle]
pub extern "C" fn ri_error_chain_free(chain: *mut CRiErrorChain) {
    if !chain.is_null() {
        unsafe {
            let _ = Box::from_raw(chain);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_error_chain_get_context(chain: *const CRiErrorChain) -> *mut c_char {
    if chain.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let context = (*chain).inner.get_context();
        match CString::new(context) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_error_chain_pretty_format(chain: *const CRiErrorChain) -> *mut c_char {
    if chain.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let formatted = (*chain).inner.pretty_format();
        match CString::new(formatted) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[repr(C)]
pub struct CRiLockError {
    pub context: *mut c_char,
    pub is_poisoned: bool,
}

#[no_mangle]
pub extern "C" fn ri_lock_error_new(context: *const c_char, is_poisoned: bool) -> *mut CRiLockError {
    if context.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        let ctx = match std::ffi::CStr::from_ptr(context).to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return std::ptr::null_mut(),
        };

        let error = if is_poisoned {
            RiLockError::poisoned(&ctx)
        } else {
            RiLockError::new(&ctx)
        };

        let c_context = match CString::new(error.get_context()) {
            Ok(s) => s.into_raw(),
            Err(_) => return std::ptr::null_mut(),
        };

        Box::into_raw(Box::new(CRiLockError {
            context: c_context,
            is_poisoned: error.is_poisoned(),
        }))
    }
}

#[no_mangle]
pub extern "C" fn ri_lock_error_free(error: *mut CRiLockError) {
    if !error.is_null() {
        unsafe {
            if !(*error).context.is_null() {
                let _ = CString::from_raw((*error).context);
            }
            let _ = Box::from_raw(error);
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_lock_error_get_context(error: *const CRiLockError) -> *mut c_char {
    if error.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        if (*error).context.is_null() {
            return std::ptr::null_mut();
        }
        match CString::new(std::ffi::CStr::from_ptr((*error).context).to_bytes()) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn ri_lock_error_is_poisoned(error: *const CRiLockError) -> bool {
    if error.is_null() {
        return false;
    }
    unsafe { (*error).is_poisoned }
}

#[no_mangle]
pub extern "C" fn ri_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
