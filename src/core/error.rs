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

#![allow(non_snake_case)]

//! # Error Handling
//! 
//! This module provides the core error handling types for Ri, including the `RiError` enum
//! and `RiResult` type alias. It defines a comprehensive set of error variants for different
//! error scenarios encountered in the Ri library.
//! 
//! ## Key Components
//! 
//! - **RiError**: Enum representing all possible errors in Ri
//! - **RiResult**: Type alias for `Result<T, RiError>` used throughout the library
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Coverage**: Covers all major error categories encountered in Ri
//! 2. **Type Safety**: Each error variant provides specific context about the error
//! 3. **Easy Conversion**: Implements `From` traits for common external error types
//! 4. **Human-Readable**: Provides clear, descriptive error messages
//! 5. **Standard Compliance**: Implements `std::error::Error` and `std::fmt::Display`
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! fn example_function() -> RiResult<()> {
//!     // Return a custom error
//!     Err(RiError::Other("An error occurred"))
//! }
//! 
//! #[tokio::main]
//! async fn main() -> RiResult<()> {
//!     match example_function() {
//!         Ok(_) => println!("Success"),
//!         Err(err) => {
//!             println!("Error: {}", err);
//!             Err(err)
//!         }
//!     }
//! }
//! ```

#[cfg(feature = "pyo3")]
use pyo3::types::PyTracebackMethods;

/// Core error type for Ri. Represents all possible errors that can occur in the library.
/// 
/// This enum provides a comprehensive set of error variants, each tailored to a specific
/// error scenario encountered in Ri. It includes variants for I/O errors, serialization errors,
/// configuration errors, module errors, and more.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RiError {
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
    /// Frame error. Contains a descriptive error message for frame parsing/building errors.
    FrameError(String),
    /// Database error. Contains a descriptive error message for database operations.
    Database(String),
}

/// Result type alias for Ri operations. Used throughout the library.
/// 
/// This type alias simplifies error handling by providing a consistent result type
/// for all Ri operations. It wraps the standard `Result` type with `RiError` as the error type.
pub type RiResult<T> = Result<T, RiError>;

/// Implements Display trait for human-readable error messages.
/// 
/// Each error variant is formatted with a clear, descriptive prefix indicating
/// the error category, followed by the specific error details. This enables
/// developers to quickly identify the source and nature of errors during
/// development and debugging.
impl std::fmt::Display for RiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RiError::Io(err) => write!(f, "IO error: {err}"),
            RiError::Serde(err) => write!(f, "Serialization error: {err}"),
            RiError::Config(msg) => write!(f, "Configuration error: {msg}"),
            RiError::Hook(msg) => write!(f, "Hook error: {msg}"),
            RiError::Prometheus(err) => write!(f, "Prometheus error: {err}"),
            RiError::ServiceMesh(err) => write!(f, "Service mesh error: {err}"),
            RiError::InvalidState(msg) => write!(f, "Invalid state: {msg}"),
            RiError::InvalidInput(msg) => write!(f, "Invalid input: {msg}"),
            RiError::SecurityViolation(msg) => write!(f, "Security violation: {msg}"),
            RiError::DeviceNotFound { device_id } => write!(f, "Device not found: {device_id}"),
            RiError::DeviceAllocationFailed { device_id, reason } => {
                write!(f, "Device allocation failed for {device_id}: {reason}")
            }
            RiError::AllocationNotFound { allocation_id } => {
                write!(f, "Allocation not found: {allocation_id}")
            }
            RiError::ModuleNotFound { module_name } => {
                write!(f, "Module not found: {module_name}")
            }
            RiError::ModuleInitFailed { module_name, reason } => {
                write!(f, "Module initialization failed for {module_name}: {reason}")
            }
            RiError::ModuleStartFailed { module_name, reason } => {
                write!(f, "Module start failed for {module_name}: {reason}")
            }
            RiError::ModuleShutdownFailed { module_name, reason } => {
                write!(f, "Module shutdown failed for {module_name}: {reason}")
            }
            RiError::CircularDependency { modules } => {
                write!(f, "Circular dependency detected: {}", modules.join(" -> "))
            }
            RiError::MissingDependency { module_name, dependency } => {
                write!(f, "Module {module_name} depends on missing module: {dependency}")
            }
            RiError::Other(msg) => write!(f, "{msg}"),
            RiError::ExternalError(msg) => write!(f, "External error: {msg}"),
            RiError::PoolError(msg) => write!(f, "Pool error: {msg}"),
            RiError::DeviceError(msg) => write!(f, "Device error: {msg}"),
            RiError::RedisError(msg) => write!(f, "Redis error: {msg}"),
            RiError::HttpClientError(msg) => write!(f, "HTTP client error: {msg}"),
            RiError::TomlError(msg) => write!(f, "TOML error: {msg}"),
            RiError::YamlError(msg) => write!(f, "YAML error: {msg}"),
            RiError::Queue(msg) => write!(f, "Queue error: {msg}"),
            RiError::FrameError(msg) => write!(f, "Frame error: {msg}"),
            RiError::Database(msg) => write!(f, "Database error: {msg}"),
        }
    }
}

impl std::error::Error for RiError {}

/// Enables automatic conversion from standard I/O errors to Ri errors.
/// This implementation wraps std::io::Error instances into RiError::Io variants,
/// allowing seamless error propagation when working with file operations, network I/O,
/// and other standard I/O operations.
impl From<std::io::Error> for RiError {
    fn from(error: std::io::Error) -> Self {
        RiError::Io(format!("I/O operation failed: {}", error))
    }
}

/// Enhanced error formatting with suggestions.
/// Provides consistent error messages with actionable suggestions for resolution.
pub struct RiErrorFormatter<'a> {
    error: &'a RiError,
}

impl<'a> RiErrorFormatter<'a> {
    /// Creates a new error formatter for the given error.
    pub fn new(error: &'a RiError) -> Self {
        Self { error }
    }

    /// Returns the formatted error message with suggestion.
    pub fn format(&self) -> String {
        let base_message = self.error.to_string();
        let suggestion = self.get_suggestion();

        match suggestion {
            Some(s) => format!("{}\n💡 Suggestion: {}", base_message, s),
            None => base_message,
        }
    }

    /// Returns an actionable suggestion for the error.
    fn get_suggestion(&self) -> Option<&'static str> {
        match self.error {
            RiError::Io(_) => Some("Check file permissions and disk space"),
            RiError::Serde(_) => Some("Verify data format matches expected schema"),
            RiError::Config(_) => Some("Review configuration file syntax and required fields"),
            RiError::Hook(_) => Some("Check hook implementation for errors and ensure proper registration"),
            RiError::Prometheus(_) => Some("Verify Prometheus server is running and metrics endpoint is accessible"),
            RiError::ServiceMesh(_) => Some("Check service mesh configuration and network connectivity"),
            RiError::InvalidState(_) => Some("Ensure module is in correct state before performing operation"),
            RiError::InvalidInput(_) => Some("Validate input data against expected format and constraints"),
            RiError::SecurityViolation(_) => Some("Review security policies and access permissions"),
            RiError::DeviceNotFound { .. } => Some("Verify device ID exists and is properly registered"),
            RiError::DeviceAllocationFailed { .. } => Some("Check device availability and allocation constraints"),
            RiError::AllocationNotFound { .. } => Some("Verify allocation ID is correct and hasn't expired"),
            RiError::ModuleNotFound { .. } => Some("Ensure module is registered and feature flag is enabled"),
            RiError::ModuleInitFailed { .. } => Some("Check module dependencies and initialization parameters"),
            RiError::ModuleStartFailed { .. } => Some("Review module start sequence and resource availability"),
            RiError::ModuleShutdownFailed { .. } => Some("Ensure no active connections before shutdown"),
            RiError::CircularDependency { .. } => Some("Restructure module dependencies to eliminate cycles"),
            RiError::MissingDependency { .. } => Some("Add required module to application configuration"),
            RiError::Other(_) => None,
            RiError::ExternalError(_) => Some("Check external service status and credentials"),
            RiError::PoolError(_) => Some("Verify connection pool configuration and database availability"),
            RiError::DeviceError(_) => Some("Check device connection and configuration"),
            RiError::RedisError(_) => Some("Verify Redis server is running and connection parameters are correct"),
            RiError::HttpClientError(_) => Some("Check network connectivity and target server availability"),
            RiError::TomlError(_) => Some("Validate TOML syntax and required sections"),
            RiError::YamlError(_) => Some("Validate YAML syntax and indentation"),
            RiError::Queue(_) => Some("Check message queue service status and queue configuration"),
            RiError::FrameError(_) => Some("Check frame format and protocol compatibility"),
            RiError::Database(_) => Some("Verify database connection and query syntax"),
        }
    }
}

/// Formats an error with actionable suggestion.
/// This helper function provides enhanced error messages that include
/// suggestions for resolving the issue.
#[inline]
pub fn format_error(error: &RiError) -> String {
    RiErrorFormatter::new(error).format()
}

/// Logs an error with enhanced formatting.
/// This helper function logs the error with its suggestion for debugging.
#[inline]
pub fn log_error(error: &RiError) {
    log::error!("{}", format_error(error));
}

/// Enables automatic conversion from JSON serialization/deserialization errors.
/// This implementation wraps serde_json::Error instances into RiError::Serde variants,
/// providing consistent error handling for JSON parsing and generation operations.
impl From<serde_json::Error> for RiError {
    fn from(error: serde_json::Error) -> Self {
        RiError::Serde(error.to_string())
    }
}

/// Enables automatic conversion from Prometheus metrics errors to Ri errors.
/// This implementation is conditionally compiled with the "observability" feature.
/// Prometheus errors are wrapped into RiError::Prometheus variants.
#[cfg(feature = "observability")]
impl From<prometheus::Error> for RiError {
    fn from(error: prometheus::Error) -> Self {
        RiError::Prometheus(error.to_string())
    }
}

/// Enables automatic conversion from Redis client errors to Ri errors.
/// This implementation is conditionally compiled with the "redis" feature.
/// Redis errors are wrapped into RiError::RedisError variants, providing
/// consistent error handling for Redis connection and operation failures.
#[cfg(feature = "redis")]
impl From<redis::RedisError> for RiError {
    fn from(error: redis::RedisError) -> Self {
        RiError::RedisError(error.to_string())
    }
}

/// Enables automatic conversion from HTTP client errors to Ri errors.
/// This implementation is conditionally compiled with the "http_client" feature.
/// HTTP request failures, timeouts, and network errors are wrapped into
/// RiError::HttpClientError variants for consistent error handling.
#[cfg(feature = "http_client")]
impl From<reqwest::Error> for RiError {
    fn from(error: reqwest::Error) -> Self {
        RiError::HttpClientError(error.to_string())
    }
}

/// Enables automatic conversion from TOML parsing errors to Ri errors.
/// This implementation wraps toml::de::Error instances into RiError::TomlError variants,
/// providing consistent error handling for TOML configuration file parsing.
impl From<toml::de::Error> for RiError {
    fn from(error: toml::de::Error) -> Self {
        RiError::TomlError(error.to_string())
    }
}

/// Enables automatic conversion from TOML serialization errors to Ri errors.
/// This implementation wraps toml::ser::Error instances into RiError::TomlError variants,
/// providing consistent error handling for TOML configuration generation.
impl From<toml::ser::Error> for RiError {
    fn from(error: toml::ser::Error) -> Self {
        RiError::TomlError(error.to_string())
    }
}

/// Enables automatic conversion from YAML parsing errors to Ri errors.
/// This implementation wraps serde_yaml::Error instances into RiError::YamlError variants,
/// providing consistent error handling for YAML configuration file parsing.
impl From<serde_yaml::Error> for RiError {
    fn from(error: serde_yaml::Error) -> Self {
        RiError::YamlError(error.to_string())
    }
}

/// Enables automatic conversion from RabbitMQ client errors to Ri errors.
/// This implementation is conditionally compiled with the "rabbitmq" feature.
/// RabbitMQ connection and channel errors are wrapped into RiError::Other variants
/// with a "RabbitMQ error:" prefix for consistent error categorization.
#[cfg(feature = "rabbitmq")]
impl From<lapin::Error> for RiError {
    fn from(error: lapin::Error) -> Self {
        RiError::Other(format!("RabbitMQ error: {error}"))
    }
}

/// Enables automatic conversion from Kafka client errors to Ri errors.
/// This implementation is conditionally compiled with the "kafka" feature on non-Windows platforms.
/// Kafka producer, consumer, and administration errors are wrapped into RiError::Queue variants
/// for consistent error handling in message queue operations.
#[cfg(all(feature = "kafka", not(windows)))]
impl From<rdkafka::error::KafkaError> for RiError {
    fn from(error: rdkafka::error::KafkaError) -> Self {
        RiError::Queue(format!("Kafka error: {}", error))
    }
}

/// Stub implementation for Kafka errors on Windows.
/// This prevents compilation errors on Windows where rdkafka is not available.
/// The actual Kafka functionality is disabled on Windows via the kafka_stub module.
#[cfg(all(feature = "kafka", windows))]
impl From<rdkafka::error::KafkaError> for RiError {
    fn from(error: rdkafka::error::KafkaError) -> Self {
        RiError::Queue(format!("Kafka error: {}", error))
    }
}

impl From<tokio::time::error::Elapsed> for RiError {
    fn from(error: tokio::time::error::Elapsed) -> Self {
        RiError::Io(format!("Operation timed out: {}", error))
    }
}

impl From<std::str::Utf8Error> for RiError {
    fn from(error: std::str::Utf8Error) -> Self {
        RiError::Serde(format!("UTF-8 conversion error: {}", error))
    }
}

impl From<tokio::sync::TryLockError> for RiError {
    fn from(error: tokio::sync::TryLockError) -> Self {
        RiError::InvalidState(format!("Lock acquisition failed: {}", error))
    }
}

impl From<super::lock::RiLockError> for RiError {
    fn from(error: super::lock::RiLockError) -> Self {
        RiError::InvalidState(format!("Lock error: {}", error))
    }
}

#[cfg(feature = "pyo3")]
impl std::convert::From<RiError> for pyo3::PyErr {
    fn from(error: RiError) -> Self {
        pyo3::exceptions::PyRuntimeError::new_err(error.to_string())
    }
}

#[cfg(feature = "pyo3")]
impl std::convert::From<pyo3::PyErr> for RiError {
    fn from(error: pyo3::PyErr) -> Self {
        let error_info = pyo3::Python::attach(|py| {
            let traceback = error.traceback(py)
                .and_then(|tb| tb.format().ok())
                .unwrap_or_default();
            let error_type = error.get_type(py).to_string();
            let error_value = error.value(py).to_string();
            format!("{}: {}\n{}", error_type, error_value, traceback)
        });
        RiError::Other(format!("Python error: {}", error_info))
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for RiError.
///
/// This implementation provides a Python interface for the RiError type, enabling
/// Python applications to create, inspect, and handle Ri errors. The bindings expose
/// factory methods for creating specific error types and predicate methods for checking
/// error variants at runtime.
///
/// ## Python Usage Example
///
/// ```python
/// 
///
/// try:
///     # Some operation that might fail
///     pass
/// except Exception as e:
///     if isinstance(e, dms.RiError):
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
impl RiError {
    /// Returns the string representation of the error.
    ///
    /// This method implements the Python __str__ protocol, returning a human-readable
    /// error message that describes the error. The format matches the Display trait
    /// implementation in Rust, providing consistent output across language boundaries.
    ///
    /// Returns:
    ///     A String containing the formatted error message
    pub fn __str__(&self) -> String {
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
    pub fn __repr__(&self) -> String {
        format!("{:?}", self)
    }

    /// Creates a new RiError from a string message.
    ///
    /// This factory method creates an Other variant error containing the provided
    /// message. It serves as a generic error constructor for custom error scenarios
    /// that don't fit other specific error types.
    ///
    /// Arguments:
    ///     message: The error message describing the failure
    ///
    /// Returns:
    ///     A new RiError instance with Other variant
    #[staticmethod]
    pub fn from_str(message: &str) -> Self {
        RiError::Other(message.to_string())
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
    ///     A new RiError instance with Io variant
    #[staticmethod]
    pub fn io(message: &str) -> Self {
        RiError::Io(message.to_string())
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
    ///     A new RiError instance with Serde variant
    #[staticmethod]
    pub fn serde(message: &str) -> Self {
        RiError::Serde(message.to_string())
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
    ///     A new RiError instance with Config variant
    #[staticmethod]
    pub fn config(message: &str) -> Self {
        RiError::Config(message.to_string())
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
    ///     A new RiError instance with Hook variant
    #[staticmethod]
    pub fn hook(message: &str) -> Self {
        RiError::Hook(message.to_string())
    }

    /// Checks if this error is an IO error.
    ///
    /// This predicate method returns true if the error is an Io variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is an Io variant, false otherwise
    pub fn is_io(&self) -> bool {
        matches!(self, RiError::Io(_))
    }

    /// Checks if this error is a serialization error.
    ///
    /// This predicate method returns true if the error is a Serde variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Serde variant, false otherwise
    pub fn is_serde(&self) -> bool {
        matches!(self, RiError::Serde(_))
    }

    /// Checks if this error is a configuration error.
    ///
    /// This predicate method returns true if the error is a Config variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Config variant, false otherwise
    pub fn is_config(&self) -> bool {
        matches!(self, RiError::Config(_))
    }

    /// Checks if this error is a hook error.
    ///
    /// This predicate method returns true if the error is a Hook variant.
    /// Use this for conditional error handling based on error type.
    ///
    /// Returns:
    ///     true if the error is a Hook variant, false otherwise
    pub fn is_hook(&self) -> bool {
        matches!(self, RiError::Hook(_))
    }

    /// Checks if this error is a Prometheus metrics error.
    ///
    /// This predicate method returns true if the error is a Prometheus variant.
    /// This method is only available when the "observability" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is a Prometheus variant, false otherwise
    pub fn is_prometheus(&self) -> bool {
        matches!(self, RiError::Prometheus(_))
    }

    /// Checks if this error is a service mesh error.
    ///
    /// This predicate method returns true if the error is a ServiceMesh variant.
    /// Use this for conditional error handling related to service mesh operations.
    ///
    /// Returns:
    ///     true if the error is a ServiceMesh variant, false otherwise
    pub fn is_service_mesh(&self) -> bool {
        matches!(self, RiError::ServiceMesh(_))
    }

    /// Checks if this error is an invalid state error.
    ///
    /// This predicate method returns true if the error is an InvalidState variant.
    /// Use this when an operation was attempted in an invalid program state.
    ///
    /// Returns:
    ///     true if the error is an InvalidState variant, false otherwise
    pub fn is_invalid_state(&self) -> bool {
        matches!(self, RiError::InvalidState(_))
    }

    /// Checks if this error is an invalid input error.
    ///
    /// This predicate method returns true if the error is an InvalidInput variant.
    /// Use this when provided input data fails validation checks.
    ///
    /// Returns:
    ///     true if the error is an InvalidInput variant, false otherwise
    pub fn is_invalid_input(&self) -> bool {
        matches!(self, RiError::InvalidInput(_))
    }

    /// Checks if this error is a security violation error.
    ///
    /// This predicate method returns true if the error is a SecurityViolation variant.
    /// Use this when a security policy or rule has been violated.
    ///
    /// Returns:
    ///     true if the error is a SecurityViolation variant, false otherwise
    pub fn is_security_violation(&self) -> bool {
        matches!(self, RiError::SecurityViolation(_))
    }

    /// Checks if this error is a device not found error.
    ///
    /// This predicate method returns true if the error is a DeviceNotFound variant.
    /// Use this when a requested device identifier does not exist.
    ///
    /// Returns:
    ///     true if the error is a DeviceNotFound variant, false otherwise
    pub fn is_device_not_found(&self) -> bool {
        matches!(self, RiError::DeviceNotFound { .. })
    }

    /// Checks if this error is a device allocation failed error.
    ///
    /// This predicate method returns true if the error is a DeviceAllocationFailed variant.
    /// Use this when a device cannot be allocated for use.
    ///
    /// Returns:
    ///     true if the error is a DeviceAllocationFailed variant, false otherwise
    pub fn is_device_allocation_failed(&self) -> bool {
        matches!(self, RiError::DeviceAllocationFailed { .. })
    }

    /// Checks if this error is an allocation not found error.
    ///
    /// This predicate method returns true if the error is an AllocationNotFound variant.
    /// Use this when a requested allocation identifier does not exist.
    ///
    /// Returns:
    ///     true if the error is an AllocationNotFound variant, false otherwise
    pub fn is_allocation_not_found(&self) -> bool {
        matches!(self, RiError::AllocationNotFound { .. })
    }

    /// Checks if this error is a module not found error.
    ///
    /// This predicate method returns true if the error is a ModuleNotFound variant.
    /// Use this when a requested module does not exist in the system.
    ///
    /// Returns:
    ///     true if the error is a ModuleNotFound variant, false otherwise
    pub fn is_module_not_found(&self) -> bool {
        matches!(self, RiError::ModuleNotFound { .. })
    }

    /// Checks if this error is a module initialization failed error.
    ///
    /// This predicate method returns true if the error is a ModuleInitFailed variant.
    /// Use this when a module fails to initialize properly.
    ///
    /// Returns:
    ///     true if the error is a ModuleInitFailed variant, false otherwise
    pub fn is_module_init_failed(&self) -> bool {
        matches!(self, RiError::ModuleInitFailed { .. })
    }

    /// Checks if this error is a module start failed error.
    ///
    /// This predicate method returns true if the error is a ModuleStartFailed variant.
    /// Use this when a module fails to start after successful initialization.
    ///
    /// Returns:
    ///     true if the error is a ModuleStartFailed variant, false otherwise
    pub fn is_module_start_failed(&self) -> bool {
        matches!(self, RiError::ModuleStartFailed { .. })
    }

    /// Checks if this error is a module shutdown failed error.
    ///
    /// This predicate method returns true if the error is a ModuleShutdownFailed variant.
    /// Use this when a module fails to shut down gracefully.
    ///
    /// Returns:
    ///     true if the error is a ModuleShutdownFailed variant, false otherwise
    pub fn is_module_shutdown_failed(&self) -> bool {
        matches!(self, RiError::ModuleShutdownFailed { .. })
    }

    /// Checks if this error is a circular dependency error.
    ///
    /// This predicate method returns true if the error is a CircularDependency variant.
    /// Use this when modules have circular import or initialization dependencies.
    ///
    /// Returns:
    ///     true if the error is a CircularDependency variant, false otherwise
    pub fn is_circular_dependency(&self) -> bool {
        matches!(self, RiError::CircularDependency { .. })
    }

    /// Checks if this error is a missing dependency error.
    ///
    /// This predicate method returns true if the error is a MissingDependency variant.
    /// Use this when a module depends on another module that is not available.
    ///
    /// Returns:
    ///     true if the error is a MissingDependency variant, false otherwise
    pub fn is_missing_dependency(&self) -> bool {
        matches!(self, RiError::MissingDependency { .. })
    }

    /// Checks if this error is a generic other error.
    ///
    /// This predicate method returns true if the error is an Other variant.
    /// Use this as a catch-all check for unclassified errors.
    ///
    /// Returns:
    ///     true if the error is an Other variant, false otherwise
    pub fn is_other(&self) -> bool {
        matches!(self, RiError::Other(_))
    }

    /// Checks if this error is an external error.
    ///
    /// This predicate method returns true if the error is an ExternalError variant.
    /// Use this for errors originating from external services or dependencies.
    ///
    /// Returns:
    ///     true if the error is an ExternalError variant, false otherwise
    pub fn is_external_error(&self) -> bool {
        matches!(self, RiError::ExternalError(_))
    }

    /// Checks if this error is a connection pool error.
    ///
    /// This predicate method returns true if the error is a PoolError variant.
    /// Use this for errors related to connection pool management.
    ///
    /// Returns:
    ///     true if the error is a PoolError variant, false otherwise
    pub fn is_pool_error(&self) -> bool {
        matches!(self, RiError::PoolError(_))
    }

    /// Checks if this error is a device error.
    ///
    /// This predicate method returns true if the error is a DeviceError variant.
    /// Use this for general device-related errors not covered by specific variants.
    ///
    /// Returns:
    ///     true if the error is a DeviceError variant, false otherwise
    pub fn is_device_error(&self) -> bool {
        matches!(self, RiError::DeviceError(_))
    }

    /// Checks if this error is a Redis error.
    ///
    /// This predicate method returns true if the error is a RedisError variant.
    /// This method is only available when the "redis" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is a RedisError variant, false otherwise
    pub fn is_redis_error(&self) -> bool {
        matches!(self, RiError::RedisError(_))
    }

    /// Checks if this error is an HTTP client error.
    ///
    /// This predicate method returns true if the error is an HttpClientError variant.
    /// This method is only available when the "http_client" feature is enabled.
    ///
    /// Returns:
    ///     true if the error is an HttpClientError variant, false otherwise
    pub fn is_http_client_error(&self) -> bool {
        matches!(self, RiError::HttpClientError(_))
    }

    /// Checks if this error is a TOML parsing error.
    ///
    /// This predicate method returns true if the error is a TomlError variant.
    /// Use this for errors related to TOML configuration file parsing.
    ///
    /// Returns:
    ///     true if the error is a TomlError variant, false otherwise
    pub fn is_toml_error(&self) -> bool {
        matches!(self, RiError::TomlError(_))
    }

    /// Checks if this error is a YAML parsing error.
    ///
    /// This predicate method returns true if the error is a YamlError variant.
    /// Use this for errors related to YAML configuration file parsing.
    ///
    /// Returns:
    ///     true if the error is a YamlError variant, false otherwise
    pub fn is_yaml_error(&self) -> bool {
        matches!(self, RiError::YamlError(_))
    }

    /// Checks if this error is a queue error.
    ///
    /// This predicate method returns true if the error is a Queue variant.
    /// Use this for errors related to message queue operations (RabbitMQ, Kafka).
    ///
    /// Returns:
    ///     true if the error is a Queue variant, false otherwise
    pub fn is_queue(&self) -> bool {
        matches!(self, RiError::Queue(_))
    }
}
