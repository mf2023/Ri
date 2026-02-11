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

//! # Core Module C API
//!
//! This module provides C language bindings for DMSC's core application infrastructure.
//! The core module serves as the foundation for building DMSC applications, providing
//! application lifecycle management, configuration handling, and initialization routines.
//! This C API enables C/C++ applications to leverage DMSC's powerful application builder
//! and configuration management capabilities.
//!
//! ## Module Architecture
//!
//! The core module comprises two essential components that form the backbone of any
//! DMSC application:
//!
//! - **DMSCAppBuilder**: Fluent builder pattern implementation for constructing DMSC
//!   applications with type-safe configuration. The builder supports registration of
//!   modules, services, and middleware components through a declarative API. It handles
//!   dependency injection, service discovery, and lifecycle coordination across all
//!   registered components. The builder produces a fully initialized DMSCApp instance
//!   ready for execution.
//!
//! - **DMSCConfig**: Unified configuration management interface supporting multiple
//!   configuration sources including environment variables, command-line arguments,
//!   configuration files (YAML, TOML, JSON), and remote configuration services.
//!   The configuration system provides type-safe value retrieval with automatic
//!   type conversion, validation, and hot-reload capabilities for dynamic
//!   configuration updates in running applications.
//!
//! ## Application Lifecycle
//!
//! DMSC applications follow a well-defined lifecycle:
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
//! CDMSCConfig* config = dmsc_config_new();
//!
//! // Load configuration from file
//! int result = dmsc_config_load_file(config, "config.yaml");
//!
//! // Get configuration value
//! char* value = dmsc_config_get_string(config, "database.url");
//!
//! // Create application builder
//! CDMSCAppBuilder* builder = dmsc_app_builder_new();
//!
//! // Configure builder with configuration
//! dmsc_app_builder_configure(builder, config);
//!
//! // Build and run application
//! dmsc_app_builder_build(builder);
//!
//! // Cleanup
//! dmsc_app_builder_free(builder);
//! dmsc_config_free(config);
//! dmsc_string_free(value);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::core`: Rust core module implementation
//! - `crate::prelude`: Common types and traits
//!
//! ## Feature Flags
//!
//! The core module is always enabled as it provides fundamental infrastructure
//! required by all other DMSC components.

use crate::prelude::{DMSCAppBuilder, DMSCConfig};
use std::ffi::{c_char, CString};

/// Opaque C wrapper structure for DMSCAppBuilder.
///
/// Provides C-compatible interface to the Rust application builder implementation.
/// The builder uses the fluent builder pattern to construct DMSC applications with
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
/// dmsc_app_builder_register_module(builder, module_a);
/// dmsc_app_builder_register_module(builder, module_b);
/// dmsc_app_builder_configure(builder, config);
/// dmsc_app_builder_with_middleware(builder, middleware_1);
/// dmsc_app_builder_with_middleware(builder, middleware_2);
/// ```
///
/// # Thread Safety
///
/// The builder is not thread-safe. All builder operations should occur from a
/// single thread before application startup. Concurrent builder access results
/// in undefined behavior.
#[repr(C)]
pub struct CDMSCAppBuilder {
    inner: DMSCAppBuilder,
}

/// Opaque C wrapper structure for DMSCConfig.
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
pub struct CDMSCConfig {
    inner: DMSCConfig,
}

/// Creates a new CDMSCAppBuilder instance.
///
/// Initializes an empty application builder ready for component registration.
/// The builder starts with default configuration and no registered modules.
///
/// # Returns
///
/// Pointer to newly allocated CDMSCAppBuilder on success. Never returns NULL
/// as the implementation uses infallible construction. The returned pointer
/// must be freed using dmsc_app_builder_free().
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
/// CDMSCAppBuilder* builder = dmsc_app_builder_new();
/// if (builder == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
///
/// // Register modules and configure
/// dmsc_app_builder_register_module(builder, http_module);
/// dmsc_app_builder_register_module(builder, database_module);
///
/// // Build and run
/// dmsc_app_builder_build(builder);
///
/// // Cleanup
/// dmsc_app_builder_free(builder);
/// ```
#[no_mangle]
pub extern "C" fn dmsc_app_builder_new() -> *mut CDMSCAppBuilder {
    let builder = CDMSCAppBuilder {
        inner: DMSCAppBuilder::new(),
    };
    Box::into_raw(Box::new(builder))
}

/// Frees a previously allocated CDMSCAppBuilder instance.
///
/// Releases all memory associated with the builder including any registered
/// configurations, module references, or internal state. After this function
/// returns, the pointer becomes invalid.
///
/// # Parameters
///
/// - `builder`: Pointer to CDMSCAppBuilder to free. If NULL, the function
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
pub extern "C" fn dmsc_app_builder_free(builder: *mut CDMSCAppBuilder) {
    if !builder.is_null() {
        unsafe {
            let _ = Box::from_raw(builder);
        }
    }
}

/// Creates a new CDMSCConfig instance.
///
/// Initializes an empty configuration object with no loaded sources.
/// The configuration starts with default values and requires explicit
/// loading from configuration sources.
///
/// # Returns
///
/// Pointer to newly allocated CDMSCConfig on success. Never returns NULL
/// as the implementation uses infallible construction. The returned pointer
/// must be freed using dmsc_config_free().
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
/// CDMSCConfig* config = dmsc_config_new();
/// if (config == NULL) {
///     // Handle allocation failure
///     return ERROR_MEMORY_ALLOCATION;
/// }
///
/// // Load from file
/// int load_result = dmsc_config_load_file(config, "config.yaml");
/// if (load_result != 0) {
///     // Handle load failure
/// }
///
/// // Access configuration values
/// char* host = dmsc_config_get_string(config, "server.host");
/// int port = dmsc_config_get_int(config, "server.port");
///
/// // Cleanup
/// dmsc_config_free(config);
/// dmsc_string_free(host);
/// ```
#[no_mangle]
pub extern "C" fn dmsc_config_new() -> *mut CDMSCConfig {
    let config = CDMSCConfig {
        inner: DMSCConfig::new(),
    };
    Box::into_raw(Box::new(config))
}

/// Frees a previously allocated CDMSCConfig instance.
///
/// Releases all memory associated with the configuration including any
/// loaded values, watched files, or internal caches. After this function
/// returns, the pointer becomes invalid.
///
/// # Parameters
///
/// - `config`: Pointer to CDMSCConfig to free. If NULL, the function returns
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
pub extern "C" fn dmsc_config_free(config: *mut CDMSCConfig) {
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
/// - `config`: Pointer to CDMSCConfig containing the configuration. Must not
///   be NULL. If NULL, the function returns NULL.
/// - `key`: Pointer to null-terminated C string specifying the configuration key.
///   Keys use dot notation for hierarchical access (e.g., "database.connections.max").
///   Must not be NULL. If NULL, the function returns NULL.
///
/// # Returns
///
/// Pointer to newly allocated C string containing the configuration value on
/// success. The caller is responsible for freeing the returned string using
/// dmsc_string_free(). Returns NULL if:
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
/// char* database_url = dmsc_config_get_string(config, "database.url");
/// if (database_url != NULL) {
///     printf("Database URL: %s\n", database_url);
///     dmsc_string_free(database_url);
/// } else {
///     printf("Database URL not configured\n");
/// }
/// ```
///
/// # Memory Management
///
/// The returned string is newly allocated. Callers must release it using
/// dmsc_string_free() to prevent memory leaks. Do not use free() directly.
#[no_mangle]
pub extern "C" fn dmsc_config_get_string(
    config: *mut CDMSCConfig,
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
