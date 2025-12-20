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
//! use dms::prelude::*;
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
}

/// Result type alias for DMSC operations. Used throughout the library.
/// 
/// This type alias simplifies error handling by providing a consistent result type
/// for all DMSC operations. It wraps the standard `Result` type with `DMSCError` as the error type.
pub type DMSCResult<T> = Result<T, DMSCError>;

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
        }
    }
}

impl std::error::Error for DMSCError {}

impl From<std::io::Error> for DMSCError {
    fn from(error: std::io::Error) -> Self {
        DMSCError::Io(error.to_string())
    }
}

impl From<serde_json::Error> for DMSCError {
    fn from(error: serde_json::Error) -> Self {
        DMSCError::Serde(error.to_string())
    }
}

#[cfg(feature = "observability")]
impl From<prometheus::Error> for DMSCError {
    fn from(error: prometheus::Error) -> Self {
        DMSCError::Prometheus(error.to_string())
    }
}

#[cfg(feature = "redis")]
impl From<redis::RedisError> for DMSCError {
    fn from(error: redis::RedisError) -> Self {
        DMSCError::RedisError(error.to_string())
    }
}

#[cfg(feature = "http_client")]
impl From<reqwest::Error> for DMSCError {
    fn from(error: reqwest::Error) -> Self {
        DMSCError::HttpClientError(error.to_string())
    }
}

impl From<toml::de::Error> for DMSCError {
    fn from(error: toml::de::Error) -> Self {
        DMSCError::TomlError(error.to_string())
    }
}

impl From<toml::ser::Error> for DMSCError {
    fn from(error: toml::ser::Error) -> Self {
        DMSCError::TomlError(error.to_string())
    }
}

impl From<serde_yaml::Error> for DMSCError {
    fn from(error: serde_yaml::Error) -> Self {
        DMSCError::YamlError(error.to_string())
    }
}

#[cfg(feature = "rabbitmq")]
impl From<lapin::Error> for DMSCError {
    fn from(error: lapin::Error) -> Self {
        DMSCError::Other(format!("RabbitMQ error: {error}"))
    }
}
