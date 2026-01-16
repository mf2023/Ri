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

#![allow(non_snake_case)]

//! # Error Handling
//! 
//! This module provides the core error handling types for DMSC, including the `DMSCError` enum
//! and `DMSCResult` type alias. It defines a comprehensive set of error variants for different
//! error scenarios encountered in the DMSC library.
//! 
//! ## Key Components
//! 
//! - **DMSCError**: Enum representing all possible errors in DMSC
//! - **DMSCResult**: Type alias for `Result<T, DMSCError>` used throughout the library
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Coverage**: Covers all major error categories encountered in DMSC
//! 2. **Type Safety**: Each error variant provides specific context about the error
//! 3. **Easy Conversion**: Implements `From` traits for common external error types
//! 4. **Human-Readable**: Provides clear, descriptive error messages
//! 5. **Standard Compliance**: Implements `std::error::Error` and `std::fmt::Display`
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! 
//! fn example_function() -> DMSCResult<()> {
//!     // Return a custom error
//!     Err(DMSCError::Other("An error occurred"))
//! }
//! 
//! #[tokio::main]
//! async fn main() -> DMSCResult<()> {
//!     match example_function() {
//!         Ok(_) => println!("Success"),
//!         Err(err) => {
//!             println!("Error: {}", err);
//!             Err(err)
//!         }
//!     }
//! }
//! ```

/// Core error type for DMSC. Represents all possible errors that can occur in the library.
/// 
/// This enum provides a comprehensive set of error variants, each tailored to a specific
/// error scenario encountered in DMSC. It includes variants for I/O errors, serialization errors,
/// configuration errors, module errors, and more.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DMSCError {
    /// I/O operation failed. Contains a descriptive error message.
    Io(String),
    /// Serialization or deserialization failed. Contains a descriptive error message.
    Serde(String),
    /// Configuration error. Contains a descriptive error message.
    Config(String),
    /// Hook execution error. Contains a descriptive error message.
    Hook(String),
    /// Prometheus metrics error. Contains a descriptive error message.
    Prometheus(String),
    /// Service mesh error. Contains a descriptive error message.
    ServiceMesh(String),
    /// Invalid state error. Indicates an operation was attempted in an invalid state.
    InvalidState(String),
    /// Invalid input error. Indicates that provided input data is not valid.
    InvalidInput(String),
    /// Security violation error. Indicates a security policy or rule was violated.
    SecurityViolation(String),
    /// Device not found. Contains the device ID that was not found.
    DeviceNotFound { device_id: String },
    /// Device allocation failed. Contains the device ID and reason for failure.
    DeviceAllocationFailed { device_id: String, reason: String },
    /// Allocation not found. Contains the allocation ID that was not found.
    AllocationNotFound { allocation_id: String },
    /// Module not found. Contains the module name that was not found.
    ModuleNotFound { module_name: String },
    /// Module initialization failed. Contains the module name and reason for failure.
    ModuleInitFailed { module_name: String, reason: String },
    /// Module start failed. Contains the module name and reason for failure.
    ModuleStartFailed { module_name: String, reason: String },
    /// Module shutdown failed. Contains the module name and reason for failure.
    ModuleShutdownFailed { module_name: String, reason: String },
    /// Circular dependency detected. Contains the list of modules involved in the cycle.
    CircularDependency { modules: Vec<String> },
    /// Missing dependency. Contains the module name and the missing dependency.
    MissingDependency { module_name: String, dependency: String },
    /// Other error. Contains a descriptive error message for unclassified errors.
    Other(String),
    /// External error. Contains a descriptive error message for external service errors.
    ExternalError(String),
    /// Pool error. Contains a descriptive error message for connection pool errors.
    PoolError(String),
    /// Device error. Contains a descriptive error message for device-related errors.
    DeviceError(String),
    /// Redis error. Contains a descriptive error message for Redis operations.
    RedisError(String),
    /// HTTP client error. Contains a descriptive error message for HTTP requests.
    HttpClientError(String),
    /// TOML parsing error. Contains a descriptive error message for TOML parsing.
    TomlError(String),
    /// YAML parsing error. Contains a descriptive error message for YAML parsing.
    YamlError(String),
    /// Queue error. Contains a descriptive error message for queue operations.
    Queue(String),
}

/// Result type alias for DMSC operations. Used throughout the library.
/// 
/// This type alias simplifies error handling by providing a consistent result type
/// for all DMSC operations. It wraps the standard `Result` type with `DMSCError` as the error type.
pub type DMSCResult<T> = Result<T, DMSCError>;

/// Implements Display trait for human-readable error messages.
/// 
/// Each error variant is formatted with a clear, descriptive prefix indicating
/// the error category, followed by the specific error details. This enables
/// developers to quickly identify the source and nature of errors during
/// development and debugging.
impl std::fmt::Display for DMSCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMSCError::Io(err) => write!(f, "IO error: {err}"),
            DMSCError::Serde(err) => write!(f, "Serialization error: {err}"),
            DMSCError::Config(msg) => write!(f, "Configuration error: {msg}"),
            DMSCError::Hook(msg) => write!(f, "Hook error: {msg}"),
            DMSCError::Prometheus(err) => write!(f, "Prometheus error: {err}"),
            DMSCError::ServiceMesh(err) => write!(f, "Service mesh error: {err}"),
            DMSCError::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            DMSCError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            DMSCError::SecurityViolation(msg) => write!(f, "Security violation: {msg}"),
            DMSCError::DeviceNotFound { device_id } => write!(f, "Device not found: {device_id}"),
            DMSCError::DeviceAllocationFailed { device_id, reason } => {
                write!(f, "Device allocation failed for {device_id}: {reason}")
            }
            DMSCError::AllocationNotFound { allocation_id } => {
                write!(f, "Allocation not found: {allocation_id}")
            }
            DMSCError::ModuleNotFound { module_name } => {
                write!(f, "Module not found: {module_name}")
            }
            DMSCError::ModuleInitFailed { module_name, reason } => {
                write!(f, "Module initialization failed for {module_name}: {reason}")
            }
            DMSCError::ModuleStartFailed { module_name, reason } => {
                write!(f, "Module start failed for {module_name}: {reason}")
            }
            DMSCError::ModuleShutdownFailed { module_name, reason } => {
                write!(f, "Module shutdown failed for {module_name}: {reason}")
            }
            DMSCError::CircularDependency { modules } => {
                write!(f, "Circular dependency detected: {}", modules.join(" -> "))
            }
            DMSCError::MissingDependency { module_name, dependency } => {
                write!(f, "Module {module_name} depends on missing module: {dependency}")
            }
            DMSCError::Other(msg) => write!(f, "{msg}"),
            DMSCError::ExternalError(msg) => write!(f, "External error: {msg}"),
            DMSCError::PoolError(msg) => write!(f, "Pool error: {msg}"),
            DMSCError::DeviceError(msg) => write!(f, "Device error: {msg}"),
            DMSCError::RedisError(msg) => write!(f, "Redis error: {msg}"),
            DMSCError::HttpClientError(msg) => write!(f, "HTTP client error: {msg}"),
            DMSCError::TomlError(msg) => write!(f, "TOML error: {msg}"),
            DMSCError::YamlError(msg) => write!(f, "YAML error: {msg}"),
            DMSCError::Queue(msg) => write!(f, "Queue error: {msg}"),
        }
    }
}

impl std::error::Error for DMSCError {}

/// Enables automatic conversion from standard I/O errors to DMSC errors.
/// This implementation wraps std::io::Error instances into DMSCError::Io variants,
/// allowing seamless error propagation when working with file operations, network I/O,
/// and other standard I/O operations.
impl From<std::io::Error> for DMSCError {
    fn from(error: std::io::Error) -> Self {
        DMSCError::Io(error.to_string())
    }
}

/// Enables automatic conversion from JSON serialization/deserialization errors.
/// This implementation wraps serde_json::Error instances into DMSCError::Serde variants,
/// providing consistent error handling for JSON parsing and generation operations.
impl From<serde_json::Error> for DMSCError {
    fn from(error: serde_json::Error) -> Self {
        DMSCError::Serde(error.to_string())
    }
}

/// Enables automatic conversion from Prometheus metrics errors to DMSC errors.
/// This implementation is conditionally compiled with the "observability" feature.
/// Prometheus errors are wrapped into DMSCError::Prometheus variants.
#[cfg(feature = "observability")]
impl From<prometheus::Error> for DMSCError {
    fn from(error: prometheus::Error) -> Self {
        DMSCError::Prometheus(error.to_string())
    }
}

/// Enables automatic conversion from Redis client errors to DMSC errors.
/// This implementation is conditionally compiled with the "redis" feature.
/// Redis errors are wrapped into DMSCError::RedisError variants, providing
/// consistent error handling for Redis connection and operation failures.
#[cfg(feature = "redis")]
impl From<redis::RedisError> for DMSCError {
    fn from(error: redis::RedisError) -> Self {
        DMSCError::RedisError(error.to_string())
    }
}

/// Enables automatic conversion from HTTP client errors to DMSC errors.
/// This implementation is conditionally compiled with the "http_client" feature.
/// HTTP request failures, timeouts, and network errors are wrapped into
/// DMSCError::HttpClientError variants for consistent error handling.
#[cfg(feature = "http_client")]
impl From<reqwest::Error> for DMSCError {
    fn from(error: reqwest::Error) -> Self {
        DMSCError::HttpClientError(error.to_string())
    }
}

/// Enables automatic conversion from TOML parsing errors to DMSC errors.
/// This implementation wraps toml::de::Error instances into DMSCError::TomlError variants,
/// providing consistent error handling for TOML configuration file parsing.
impl From<toml::de::Error> for DMSCError {
    fn from(error: toml::de::Error) -> Self {
        DMSCError::TomlError(error.to_string())
    }
}

/// Enables automatic conversion from TOML serialization errors to DMSC errors.
/// This implementation wraps toml::ser::Error instances into DMSCError::TomlError variants,
/// providing consistent error handling for TOML configuration generation.
impl From<toml::ser::Error> for DMSCError {
    fn from(error: toml::ser::Error) -> Self {
        DMSCError::TomlError(error.to_string())
    }
}

/// Enables automatic conversion from YAML parsing errors to DMSC errors.
/// This implementation wraps serde_yaml::Error instances into DMSCError::YamlError variants,
/// providing consistent error handling for YAML configuration file parsing.
impl From<serde_yaml::Error> for DMSCError {
    fn from(error: serde_yaml::Error) -> Self {
        DMSCError::YamlError(error.to_string())
    }
}

/// Enables automatic conversion from RabbitMQ client errors to DMSC errors.
/// This implementation is conditionally compiled with the "rabbitmq" feature.
/// RabbitMQ connection and channel errors are wrapped into DMSCError::Other variants
/// with a "RabbitMQ error:" prefix for consistent error categorization.
#[cfg(feature = "rabbitmq")]
impl From<lapin::Error> for DMSCError {
    fn from(error: lapin::Error) -> Self {
        DMSCError::Other(format!("RabbitMQ error: {error}"))
    }
}

/// Enables automatic conversion from Kafka client errors to DMSC errors.
/// This implementation is conditionally compiled with the "kafka" feature on non-Windows platforms.
/// Kafka producer, consumer, and administration errors are wrapped into DMSCError::Queue variants
/// for consistent error handling in message queue operations.
#[cfg(all(feature = "kafka", not(windows)))]
impl From<rdkafka::error::KafkaError> for DMSCError {
    fn from(error: rdkafka::error::KafkaError) -> Self {
        DMSCError::Queue(format!("Kafka error: {}", error))
    }
}

impl From<tokio::time::error::Elapsed> for DMSCError {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        DMSCError::Io(format!("Operation timed out: {}", error))
    }
}

impl From<std::str::Utf8Error> for DMSCError {
    fn from(error: std::str::Utf8Error) -> Self {
        DMSCError::Serde(format!("UTF-8 conversion error: {}", error))
    }
}

impl From<tokio::sync::TryLockError> for DMSCError {
    fn from(error: tokio::sync::TryLockError) -> Self {
        DMSCError::InvalidState(format!("Lock acquisition failed: {}", error))
    }
}

impl From<super::lock::DMSCLockError> for DMSCError {
    fn from(error: super::lock::DMSCLockError) -> Self {
        DMSCError::InvalidState(format!("Lock error: {}", error))
    }
}

#[cfg(feature = "pyo3")]
impl std::convert::From<DMSCError> for pyo3::PyErr {
    fn from(error: DMSCError) -> Self {
        pyo3::exceptions::PyRuntimeError::new_err(error.to_string())
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCError.
///
/// This implementation provides a Python interface for the DMSCError type, enabling
/// Python applications to create, inspect, and handle DMSC errors. The bindings expose
/// factory methods for creating specific error types and predicate methods for checking
/// error variants at runtime.
///
/// ## Python Usage Example
///
/// ```python
/// import dms
///
/// try:
///     # Some operation that might fail
///     pass
/// except Exception as e:
///     if isinstance(e, dms.DMSCError):
///         print(f"Error type: {type(e)}")
///         if e.is_io():
///             print("I/O error occurred")
/// ```
///
/// ## Available Methods
///
/// - **Factory methods**: Create specific error types from Python
/// - **Inspection methods**: Check the error variant at runtime
/// - **String representation**: __str__ and __repr__ for display
#[pyo3::prelude::pymethods]
impl DMSCError {
    /// Returns the string representation of the error.
    ///
    /// This method implements the Python __str__ protocol, returning a human-readable
    /// error message that describes the error. The format matches the Display trait
    /// implementation in Rust, providing consistent output across language boundaries.
    ///
    /// Returns:
    ///     A String containing the formatted error message
    fn __str__(&self) -> String {
        self.to_string()
    }

    /// Returns the debug representation of the error.
    ///
    /// This method implements the Python __repr__ protocol, returning a detailed
    /// representation suitable for debugging. Unlike __str__, this format includes
    /// the specific error variant and all associated data.
    ///
    /// Returns:
    ///     A String containing the debug-formatted error representation
    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    /// Creates a new DMSCError from a string message.
    ///
    /// This factory method creates an Other variant error containing the provided
    /// message. It serves as a generic error constructor for custom error scenarios
    /// that don't fit other specific error types.
    ///
    /// Arguments:
    ///     message: The error message describing the failure
    ///
    /// Returns:
    ///     A new DMSCError instance with Other variant
    #[staticmethod]
    fn from_str(message: &str) -> Self {
        DMSCError::Other(message.to_string())
    }

    /// Creates a new IO error.
    ///
    /// This factory method creates an Io variant error for I/O operation failures.
    /// Use this when file operations, network I/O, or other standard I/O operations fail.
    ///
    /// Arguments:
    ///     message: A description of the I/O failure
    ///
    /// Returns:
    ///     A new DMSCError instance with Io variant
    #[staticmethod]
    fn io(message: &str) -> Self {
        DMSCError::Io(message.to_string())
    }

    /// Creates a new serialization error.
    ///
    /// This factory method creates a Serde variant error for serialization or
    /// deserialization failures. Use this for JSON, binary, or other data format
    /// conversion errors.
    ///
    /// Arguments:
    ///     message: A description of the serialization failure
    ///
    /// Returns:
    ///     A new DMSCError instance with Serde variant
    #[staticmethod]
    fn serde(message: &str) -> Self {
        DMSCError::Serde(message.to_string())
    }

    /// Creates a new configuration error.
    ///
    /// This factory method creates a Config variant error for configuration-related
    /// failures. Use this when configuration files are invalid, missing, or contain
    /// unsupported values.
    ///
    /// Arguments:
    ///     message: A description of the configuration error
    ///
    /// Returns:
    ///     A new DMSCError instance with Config variant
    #[staticmethod]
    fn config(message: &str) -> Self {
        DMSCError::Config(message.to_string())
    }

    /// Creates a new hook execution error.
    ///
    /// This factory method creates a Hook variant error for hook callback failures.
    /// Use this when a registered hook function fails to execute properly.
    ///
    /// Arguments:
    ///     message: A description of the hook execution failure
    ///
    /// Returns:
    ///     A new DMSCError instance with Hook variant
    #[staticmethod]
    fn hook(message: &str) -> Self {
        DMSCError::Hook(message.to_string())
    }

    /// Checks if this error is an IO error.
    ///
    /// This predicate method returns true if the error is an Io variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is an Io variant, false otherwise
    fn is_io(&self) -> bool {
        matches!(self, DMSCError::Io(_))
    }

    /// Checks if this error is a serialization error.
    ///
    /// This predicate method returns true if the error is a Serde variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Serde variant, false otherwise
    fn is_serde(&self) -> bool {
        matches!(self, DMSCError::Serde(_))
    }

    /// Checks if this error is a configuration error.
    ///
    /// This predicate method returns true if the error is a Config variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Config variant, false otherwise
    fn is_config(&self) -> bool {
        matches!(self, DMSCError::Config(_))
    }

    /// Checks if this error is a hook error.
    ///
    /// This predicate method returns true if the error is a Hook variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Hook variant, false otherwise
    fn is_hook(&self) -> bool {
        matches!(self, DMSCError::Hook(_))
    }

    /// Checks if this error is a Prometheus metrics error.
    ///
    /// This predicate method returns true if the error is a Prometheus variant.
    /// This method is only available when the "observability" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is a Prometheus variant, false otherwise
    fn is_prometheus(&self) -> bool {
        matches!(self, DMSCError::Prometheus(_))
    }

    /// Checks if this error is a service mesh error.
    ///
    /// This predicate method returns true if the error is a ServiceMesh variant.
    /// Use this for conditional error handling related to service mesh operations.
    ///
    /// Returns:
    ///     true if the error is a ServiceMesh variant, false otherwise
    fn is_service_mesh(&self) -> bool {
        matches!(self, DMSCError::ServiceMesh(_))
    }

    /// Checks if this error is an invalid state error.
    ///
    /// This predicate method returns true if the error is an InvalidState variant.
    /// Use this when an operation was attempted in an invalid program state.
    ///
    /// Returns:
    ///     true if the error is an InvalidState variant, false otherwise
    fn is_invalid_state(&self) -> bool {
        matches!(self, DMSCError::InvalidState(_))
    }

    /// Checks if this error is an invalid input error.
    ///
    /// This predicate method returns true if the error is an InvalidInput variant.
    /// Use this when provided input data fails validation checks.
    ///
    /// Returns:
    ///     true if the error is an InvalidInput variant, false otherwise
    fn is_invalid_input(&self) -> bool {
        matches!(self, DMSCError::InvalidInput(_))
    }

    /// Checks if this error is a security violation error.
    ///
    /// This predicate method returns true if the error is a SecurityViolation variant.
    /// Use this when a security policy or rule has been violated.
    ///
    /// Returns:
    ///     true if the error is a SecurityViolation variant, false otherwise
    fn is_security_violation(&self) -> bool {
        matches!(self, DMSCError::SecurityViolation(_))
    }

    /// Checks if this error is a device not found error.
    ///
    /// This predicate method returns true if the error is a DeviceNotFound variant.
    /// Use this when a requested device identifier does not exist.
    ///
    /// Returns:
    ///     true if the error is a DeviceNotFound variant, false otherwise
    fn is_device_not_found(&self) -> bool {
        matches!(self, DMSCError::DeviceNotFound { .. })
    }

    /// Checks if this error is a device allocation failed error.
    ///
    /// This predicate method returns true if the error is a DeviceAllocationFailed variant.
    /// Use this when a device cannot be allocated for use.
    ///
    /// Returns:
    ///     true if the error is a DeviceAllocationFailed variant, false otherwise
    fn is_device_allocation_failed(&self) -> bool {
        matches!(self, DMSCError::DeviceAllocationFailed { .. })
    }

    /// Checks if this error is an allocation not found error.
    ///
    /// This predicate method returns true if the error is an AllocationNotFound variant.
    /// Use this when a requested allocation identifier does not exist.
    ///
    /// Returns:
    ///     true if the error is an AllocationNotFound variant, false otherwise
    fn is_allocation_not_found(&self) -> bool {
        matches!(self, DMSCError::AllocationNotFound { .. })
    }

    /// Checks if this error is a module not found error.
    ///
    /// This predicate method returns true if the error is a ModuleNotFound variant.
    /// Use this when a requested module does not exist in the system.
    ///
    /// Returns:
    ///     true if the error is a ModuleNotFound variant, false otherwise
    fn is_module_not_found(&self) -> bool {
        matches!(self, DMSCError::ModuleNotFound { .. })
    }

    /// Checks if this error is a module initialization failed error.
    ///
    /// This predicate method returns true if the error is a ModuleInitFailed variant.
    /// Use this when a module fails to initialize properly.
    ///
    /// Returns:
    ///     true if the error is a ModuleInitFailed variant, false otherwise
    fn is_module_init_failed(&self) -> bool {
        matches!(self, DMSCError::ModuleInitFailed { .. })
    }

    /// Checks if this error is a module start failed error.
    ///
    /// This predicate method returns true if the error is a ModuleStartFailed variant.
    /// Use this when a module fails to start after successful initialization.
    ///
    /// Returns:
    ///     true if the error is a ModuleStartFailed variant, false otherwise
    fn is_module_start_failed(&self) -> bool {
        matches!(self, DMSCError::ModuleStartFailed { .. })
    }

    /// Checks if this error is a module shutdown failed error.
    ///
    /// This predicate method returns true if the error is a ModuleShutdownFailed variant.
    /// Use this when a module fails to shut down gracefully.
    ///
    /// Returns:
    ///     true if the error is a ModuleShutdownFailed variant, false otherwise
    fn is_module_shutdown_failed(&self) -> bool {
        matches!(self, DMSCError::ModuleShutdownFailed { .. })
    }

    /// Checks if this error is a circular dependency error.
    ///
    /// This predicate method returns true if the error is a CircularDependency variant.
    /// Use this when modules have circular import or initialization dependencies.
    ///
    /// Returns:
    ///     true if the error is a CircularDependency variant, false otherwise
    fn is_circular_dependency(&self) -> bool {
        matches!(self, DMSCError::CircularDependency { .. })
    }

    /// Checks if this error is a missing dependency error.
    ///
    /// This predicate method returns true if the error is a MissingDependency variant.
    /// Use this when a module depends on another module that is not available.
    ///
    /// Returns:
    ///     true if the error is a MissingDependency variant, false otherwise
    fn is_missing_dependency(&self) -> bool {
        matches!(self, DMSCError::MissingDependency { .. })
    }

    /// Checks if this error is a generic other error.
    ///
    /// This predicate method returns true if the error is an Other variant.
    /// Use this as a catch-all check for unclassified errors.
    ///
    /// Returns:
    ///     true if the error is an Other variant, false otherwise
    fn is_other(&self) -> bool {
        matches!(self, DMSCError::Other(_))
    }

    /// Checks if this error is an external error.
    ///
    /// This predicate method returns true if the error is an ExternalError variant.
    /// Use this for errors originating from external services or dependencies.
    ///
    /// Returns:
    ///     true if the error is an ExternalError variant, false otherwise
    fn is_external_error(&self) -> bool {
        matches!(self, DMSCError::ExternalError(_))
    }

    /// Checks if this error is a connection pool error.
    ///
    /// This predicate method returns true if the error is a PoolError variant.
    /// Use this for errors related to connection pool management.
    ///
    /// Returns:
    ///     true if the error is a PoolError variant, false otherwise
    fn is_pool_error(&self) -> bool {
        matches!(self, DMSCError::PoolError(_))
    }

    /// Checks if this error is a device error.
    ///
    /// This predicate method returns true if the error is a DeviceError variant.
    /// Use this for general device-related errors not covered by specific variants.
    ///
    /// Returns:
    ///     true if the error is a DeviceError variant, false otherwise
    fn is_device_error(&self) -> bool {
        matches!(self, DMSCError::DeviceError(_))
    }

    /// Checks if this error is a Redis error.
    ///
    /// This predicate method returns true if the error is a RedisError variant.
    /// This method is only available when the "redis" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is a RedisError variant, false otherwise
    fn is_redis_error(&self) -> bool {
        matches!(self, DMSCError::RedisError(_))
    }

    /// Checks if this error is an HTTP client error.
    ///
    /// This predicate method returns true if the error is an HttpClientError variant.
    /// This method is only available when the "http_client" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is an HttpClientError variant, false otherwise
    fn is_http_client_error(&self) -> bool {
        matches!(self, DMSCError::HttpClientError(_))
    }

    /// Checks if this error is a TOML parsing error.
    ///
    /// This predicate method returns true if the error is a TomlError variant.
    /// Use this for errors related to TOML configuration file parsing.
    ///
    /// Returns:
    ///     true if the error is a TomlError variant, false otherwise
    fn is_toml_error(&self) -> bool {
        matches!(self, DMSCError::TomlError(_))
    }

    /// Checks if this error is a YAML parsing error.
    ///
    /// This predicate method returns true if the error is a YamlError variant.
    /// Use this for errors related to YAML configuration file parsing.
    ///
    /// Returns:
    ///     true if the error is a YamlError variant, false otherwise
    fn is_yaml_error(&self) -> bool {
        matches!(self, DMSCError::YamlError(_))
    }

    /// Checks if this error is a queue error.
    ///
    /// This predicate method returns true if the error is a Queue variant.
    /// Use this for errors related to message queue operations (RabbitMQ, Kafka).
    ///
    /// Returns:
    ///     true if the error is a Queue variant, false otherwise
    fn is_queue(&self) -> bool {
        matches!(self, DMSCError::Queue(_))
    }
}
