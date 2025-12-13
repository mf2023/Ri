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

#![allow(non_snake_case)]

//! # Error Handling
//! 
//! This module provides the core error handling types for DMS, including the `DMSError` enum
//! and `DMSResult` type alias. It defines a comprehensive set of error variants for different
//! error scenarios encountered in the DMS library.
//! 
//! ## Key Components
//! 
//! - **DMSError**: Enum representing all possible errors in DMS
//! - **DMSResult**: Type alias for `Result<T, DMSError>` used throughout the library
//! 
//! ## Design Principles
//! 
//! 1. **Comprehensive Coverage**: Covers all major error categories encountered in DMS
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
//! fn example_function() -> DMSResult<()> {
//!     // Return a custom error
//!     Err(DMSError::Other("An error occurred"))
//! }
//! 
//! #[tokio::main]
//! async fn main() -> DMSResult<()> {
//!     match example_function() {
//!         Ok(_) => println!("Success"),
//!         Err(err) => {
//!             println!("Error: {}", err);
//!             Err(err)
//!         }
//!     }
//! }
//! ```

/// Core error type for DMS. Represents all possible errors that can occur in the library.
/// 
/// This enum provides a comprehensive set of error variants, each tailored to a specific
/// error scenario encountered in DMS. It includes variants for I/O errors, serialization errors,
/// configuration errors, module errors, and more.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DMSError {
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
}

/// Result type alias for DMS operations. Used throughout the library.
/// 
/// This type alias simplifies error handling by providing a consistent result type
/// for all DMS operations. It wraps the standard `Result` type with `DMSError` as the error type.
pub type DMSResult<T> = Result<T, DMSError>;

impl std::fmt::Display for DMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMSError::Io(err) => write!(f, "IO error: {err}"),
            DMSError::Serde(err) => write!(f, "Serialization error: {err}"),
            DMSError::Config(msg) => write!(f, "Configuration error: {msg}"),
            DMSError::Hook(msg) => write!(f, "Hook error: {msg}"),
            DMSError::Prometheus(err) => write!(f, "Prometheus error: {err}"),
            DMSError::ServiceMesh(err) => write!(f, "Service mesh error: {err}"),
            DMSError::DeviceNotFound { device_id } => write!(f, "Device not found: {device_id}"),
            DMSError::DeviceAllocationFailed { device_id, reason } => {
                write!(f, "Device allocation failed for {device_id}: {reason}")
            }
            DMSError::AllocationNotFound { allocation_id } => {
                write!(f, "Allocation not found: {allocation_id}")
            }
            DMSError::ModuleNotFound { module_name } => {
                write!(f, "Module not found: {module_name}")
            }
            DMSError::ModuleInitFailed { module_name, reason } => {
                write!(f, "Module initialization failed for {module_name}: {reason}")
            }
            DMSError::ModuleStartFailed { module_name, reason } => {
                write!(f, "Module start failed for {module_name}: {reason}")
            }
            DMSError::ModuleShutdownFailed { module_name, reason } => {
                write!(f, "Module shutdown failed for {module_name}: {reason}")
            }
            DMSError::CircularDependency { modules } => {
                write!(f, "Circular dependency detected: {}", modules.join(" -> "))
            }
            DMSError::MissingDependency { module_name, dependency } => {
                write!(f, "Module {module_name} depends on missing module: {dependency}")
            }
            DMSError::Other(msg) => write!(f, "{msg}"),
            DMSError::ExternalError(msg) => write!(f, "External error: {msg}"),
            DMSError::PoolError(msg) => write!(f, "Pool error: {msg}"),
            DMSError::DeviceError(msg) => write!(f, "Device error: {msg}"),
        }
    }
}

impl std::error::Error for DMSError {}

impl From<std::io::Error> for DMSError {
    fn from(error: std::io::Error) -> Self {
        DMSError::Io(error.to_string())
    }
}

impl From<serde_json::Error> for DMSError {
    fn from(error: serde_json::Error) -> Self {
        DMSError::Serde(error.to_string())
    }
}

impl From<prometheus::Error> for DMSError {
    fn from(error: prometheus::Error) -> Self {
        DMSError::Prometheus(error.to_string())
    }
}

impl From<redis::RedisError> for DMSError {
    fn from(error: redis::RedisError) -> Self {
        DMSError::Other(format!("Redis error: {error}"))
    }
}

#[cfg(feature = "rabbitmq")]
impl From<lapin::Error> for DMSError {
    fn from(error: lapin::Error) -> Self {
        DMSError::Other(format!("RabbitMQ error: {error}"))
    }
}
