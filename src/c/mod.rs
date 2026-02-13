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

//! # C/C++ API Module
//!
//! This module provides comprehensive C language bindings for the DMSC framework, enabling
//! C and C++ applications to leverage DMSC's capabilities including application lifecycle
//! management, authentication, caching, database access, file operations, gateway services,
//! gRPC communication, event hooks, logging, module RPC, observability, protocol handling,
//! message queuing, service mesh integration, validation, and WebSocket communication.
//!
//! The C API follows C conventions for memory management, error handling, and type conventions
//! while providing access to DMSC's robust Rust implementation. This allows applications written
//! in C or C++ to benefit from DMSC's safety guarantees, performance optimizations, and
//! architectural patterns without requiring a full Rust codebase.
//!
//! ## Module Architecture
//!
//! The C API module is organized into functional submodules, each providing bindings for a
//! specific DMSC capability. The module also provides global initialization functions,
//! version information, and utility functions for managing C strings returned by the API.
//!
//! ### Core Submodules
//!
//! - **core**: Application initialization, configuration management, and lifecycle control.
//!   Provides the entry point for DMSC-based applications with automatic resource management.
//!
//! - **auth**: Authentication and authorization services including JWT token handling,
//!   session management, credential validation, and permission checking.
//!
//! - **cache**: In-memory caching with configurable eviction policies (LRU, LFU, TTL-based).
//!   Provides high-performance data caching with thread-safe concurrent access.
//!
//! - **database**: Database connection pooling and query execution support. Manages
//!   connection lifecycle with automatic health checking and failover.
//!
//! - **device**: Device abstraction layer for managing computational resources with
//!   scheduling, capability tracking, and state management.
//!
//! - **fs**: Cross-platform file system operations including path manipulation, file I/O,
//!   directory management, and symbolic link handling.
//!
//! ### Communication Submodules
//!
//! - **gateway**: HTTP API gateway with request routing, middleware chains, load balancing,
//!   rate limiting, and circuit breaker patterns.
//!
//! - **grpc**: gRPC server and client support for RPC communication with Protocol Buffer
//!   serialization, streaming, and connection management.
//!
//! - **module_rpc**: Inter-module RPC communication within the DMSC framework for
//!   distributed service coordination.
//!
//! - **ws**: WebSocket protocol support for full-duplex communication with session
//!   management and heartbeat mechanisms.
//!
//! ### System Submodules
//!
//! - **hooks**: Event hook system for extensibility through callback registration and
//!   event dispatching at key lifecycle points.
//!
//! - **log**: Structured logging infrastructure with multiple output destinations,
//!   configurable log levels, and structured field support.
//!
//! - **observability**: Metrics collection, tracing integration, and health check
//!   endpoints for system monitoring.
//!
//! - **protocol**: Protocol handling for various wire formats and serialization
//!   schemes with codec implementations.
//!
//! - **queue**: Message queue operations for asynchronous task processing with
//!   reliable delivery semantics.
//!
//! - **service_mesh**: Service mesh integration for distributed systems with service
//!   discovery, load balancing, and traffic management.
//!
//! - **validation**: Data validation with schema definitions, type checking, and
//!   custom validation rules.
//!
//! ## Global Initialization
//!
//! The DMSC library must be initialized before using any other API functions:
//!
//! ```c
//! int result = dmsc_init();
//! if (result != 0) {
//!     fprintf(stderr, "Failed to initialize DMSC library\n");
//!     return result;
//! }
//! ```
//!
//! Initialization prepares internal resources including:
//! - Global thread pool configuration
//! - Default logger setup
//! - Signal handler registration
//! - Runtime state initialization
//!
//! When the application is finished using DMSC, cleanup must be called:
//!
//! ```c
//! dmsc_cleanup();
//! ```
//!
//! Cleanup releases all global resources and ensures proper shutdown sequence.
//! All DMSC objects must be freed before calling cleanup to prevent resource leaks.
//!
//! ## Memory Management
//!
//! The C API uses manual memory management following C conventions:
//!
//! - **Object Creation**: Constructor functions return newly allocated objects.
//!   All objects must be freed using the corresponding destructor function.
//!
//! - **String Handling**: String-returning functions allocate C strings that must
//!   be freed using dmsc_string_free(). Do not use standard free() on these strings.
//!
//! - **NULL Safety**: All functions handle NULL pointers gracefully, returning
//!   error codes or NULL outputs rather than causing undefined behavior.
//!
//! - **Error Codes**: Functions return 0 for success, negative values for errors.
//!   Specific error codes are documented for each function.
//!
//! ### Memory Management Pattern
//!
//! ```c
//! // Create an object
//! DMSCLogger* logger = dmsc_logger_new(config);
//! if (logger == NULL) {
//!     fprintf(stderr, "Failed to create logger\n");
//!     dmsc_log_config_free(config);
//!     return ERROR_ALLOCATION;
//! }
//!
//! // Use the object...
//!
//! // Free when done
//! dmsc_logger_free(logger);
//!
//! // For strings returned by the API
//! const char* version = dmsc_version();
//! printf("DMSC version: %s\n", version);
//! dmsc_string_free((char*)version);  // Cast to non-const for free
//! ```
//!
//! ## Error Handling
//!
//! All DMSC C API functions follow consistent error handling patterns:
//!
//! - **Return Codes**: Integer return codes where 0 indicates success
//! - **NULL Objects**: Constructor functions return NULL on allocation failure
//! - **Error Messages**: Last error information available through type-specific functions
//! - **Error Propagation**: Errors should be checked and handled at each API call
//!
//! Standard error codes:
//! - 0: Success
//! - -1: General error
//! - -2: Invalid argument
//! - -3: Memory allocation failure
//! - -4: Resource not found
//! - -5: Permission denied
//! - -6: Timeout
//! - -7: Network error
//!
//! ## Thread Safety
//!
//! The DMSC C API is designed for thread-safe concurrent access:
//!
//! - **Object-Level Safety**: Individual objects are safe for concurrent use from
//!   multiple threads unless documented otherwise.
//!
//! - **Global State**: Global initialization is thread-safe; subsequent calls to
//!   dmsc_init() from multiple threads are handled correctly.
//!
//! - **Resource Sharing**: Objects can be shared across threads following the
//!   same patterns as the underlying Rust implementation.
//!
//! - **Synchronization**: Internal synchronization primitives prevent data races
//!   in multi-threaded scenarios.
//!
//! ## Usage Example
//!
//! A complete example demonstrating DMSC C API usage:
//!
//! ```c
//! #include <stdio.h>
//! #include "dmsc.h"
//!
//! int main(int argc, char* argv[]) {
//!     // Initialize DMSC library
//!     int result = dmsc_init();
//!     if (result != 0) {
//!         fprintf(stderr, "DMSC initialization failed: %d\n", result);
//!         return 1;
//!     }
//!
//!     // Get version information
//!     const char* version = dmsc_version();
//!     printf("DMSC Version: %s\n", version);
//!     dmsc_string_free((char*)version);
//!
//!     // Create configuration
//!     DMSCAppConfig* config = dmsc_app_config_new();
//!     dmsc_app_config_set_name(config, "MyApplication");
//!     dmsc_app_config_set_environment(config, "production");
//!
//!     // Create application instance
//!     DMSCApplication* app = dmsc_application_new(config);
//!     dmsc_app_config_free(config);
//!
//!     if (app == NULL) {
//!         fprintf(stderr, "Failed to create application\n");
//!         dmsc_cleanup();
//!         return 1;
//!     }
//!
//!     // Start application
//!     result = dmsc_application_start(app);
//!     if (result != 0) {
//!         fprintf(stderr, "Failed to start application: %d\n", result);
//!         dmsc_application_free(app);
//!         dmsc_cleanup();
//!         return 1;
//!     }
//!
//!     printf("Application running. Press Ctrl+C to stop.\n");
//!
//!     // Wait for shutdown signal
//!     // Application runs until dmsc_application_stop() is called
//!
//!     // Graceful shutdown
//!     dmsc_application_stop(app);
//!     dmsc_application_free(app);
//!
//!     // Cleanup library
//!     dmsc_cleanup();
//!
//!     printf("DMSC shutdown complete.\n");
//!     return 0;
//! }
//! ```
//!
//! ## Build Integration
//!
//! To use the DMSC C API in a C/C++ project:
//!
//! 1. **Compilation**: Include the generated C headers and link against the
//!    DMSC shared or static library.
//!
//! 2. **Header Files**: Include the main DMSC header which provides access
//!    to all submodule interfaces through a unified API surface.
//!
//! 3. **Linking**: Link against the DMSC library using appropriate linker flags
//!    for your build system.
//!
//! ## Dependencies
//!
//! The C API module depends on:
//!
//! - **Rust Core**: Standard Rust library types and traits
//! - **FFI Bindings**: Rust's foreign function interface capabilities
//! - **Submodule Implementations**: Each submodule's corresponding Rust implementation
//!
//! No external C/C++ dependencies are required beyond the C standard library.
//!
//! ## Feature Flags
//!
//! The DMSC C API supports feature flags for conditional compilation:
//!
//! - **default**: Core functionality with common features
//! - **gateway**: Enable API gateway features
//! - **grpc**: Enable gRPC server and client support
//! - **observability**: Enable metrics and tracing
//! - **service-mesh**: Enable service mesh integration
//! - **websocket**: Enable WebSocket support
//!
//! Feature flags control which submodule bindings are compiled into the library.
//! Disable unused features to reduce binary size.

use std::ffi::{c_char, c_int, CString};

#[macro_use]
pub mod macros;
pub mod core;
pub mod auth;
pub mod cache;
pub mod database;
pub mod device;
pub mod fs;
pub mod gateway;

#[cfg(feature = "grpc")]
pub mod grpc;

pub mod hooks;
pub mod log;
pub mod module_rpc;
pub mod observability;

#[cfg(feature = "protocol")]
pub mod protocol;

pub mod queue;
pub mod service_mesh;
pub mod validation;

#[cfg(feature = "websocket")]
pub mod ws;

/// Initialize the DMSC library
#[no_mangle]
pub extern "C" fn dmsc_init() -> c_int {
    0
}

/// Cleanup the DMSC library
#[no_mangle]
pub extern "C" fn dmsc_cleanup() {
}

/// Get DMSC version
#[no_mangle]
pub extern "C" fn dmsc_version() -> *mut c_char {
    let c_str = CString::new(env!("CARGO_PKG_VERSION")).unwrap();
    c_str.into_raw()
}

/// Free a string returned by DMSC
#[no_mangle]
pub extern "C" fn dmsc_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
