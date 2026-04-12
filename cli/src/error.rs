// Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//
// This file is part of Ri.
// The Ri project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// You may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Error Handling Module
//!
//! This module defines custom error types for the Ri CLI tool using the `thiserror` crate.
//! It provides a comprehensive error taxonomy that covers all possible failure scenarios
//! in the CLI application.
//!
//! # Error Categories
//!
//! - **Project Errors**: Project creation and management errors
//! - **Build Errors**: Compilation and build process errors
//! - **Configuration Errors**: Configuration file and validation errors
//! - **I/O Errors**: File system and network I/O errors
//! - **Serialization Errors**: YAML and JSON parsing errors
//! - **Generation Errors**: Code generation and template errors
//!
//! # Error Handling Strategy
//!
//! All errors are defined as variants of the `RicError` enum, which implements:
//! - `std::error::Error` for standard error handling
//! - `std::fmt::Display` for user-friendly error messages
//! - `From` trait for automatic error conversion
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::error::{Result, RicError};
//!
//! fn my_function() -> Result<()> {
//!     // Operations that can fail
//!     Ok(())
//! }
//! ```

use thiserror::Error;

/// Result type alias for Ri CLI operations
///
/// This is a convenience type alias that uses `RicError` as the error type.
/// All CLI functions should return this result type for consistency.
pub type Result<T> = std::result::Result<T, RicError>;

/// Comprehensive error enum for Ri CLI
///
/// This enum defines all possible error conditions that can occur
/// during CLI operations. Each variant includes relevant context
/// information to help diagnose and fix the issue.
///
/// # Error Variants
///
/// ## Project Errors
/// - `ProjectExists`: Attempted to create a project that already exists
///
/// ## Build Errors
/// - `BuildFailed`: Build process failed
/// - `RunFailed`: Run process failed
/// - `CheckFailed`: Check process failed
/// - `CleanFailed`: Clean process failed
///
/// ## Configuration Errors
/// - `ConfigInvalid`: Configuration validation failed
/// - `ConfigKeyNotFound`: Requested configuration key doesn't exist
///
/// ## I/O Errors
/// - `Io`: File system or network I/O error
///
/// ## Serialization Errors
/// - `Yaml`: YAML parsing or serialization error
/// - `Json`: JSON parsing or serialization error
/// - `Template`: Template processing error
#[derive(Error, Debug)]
pub enum RicError {
    /// Project already exists error
    ///
    /// Returned when attempting to create a project with a name that
    /// already exists as a directory in the current location.
    #[error("Project '{0}' already exists")]
    ProjectExists(String),

    /// Build process failed error
    ///
    /// Returned when the cargo build command fails.
    /// Includes the error message from the build process.
    #[error("Build failed: {0}")]
    BuildFailed(String),

    /// Run process failed error
    ///
    /// Returned when the cargo run command fails.
    /// Includes the error message from the run process.
    #[error("Run failed: {0}")]
    RunFailed(String),

    /// Check process failed error
    ///
    /// Returned when the cargo check command fails.
    /// Includes the error message from the check process.
    #[error("Check failed: {0}")]
    CheckFailed(String),

    /// Clean process failed error
    ///
    /// Returned when the cargo clean command fails.
    /// Includes the error message from the clean process.
    #[error("Clean failed: {0}")]
    CleanFailed(String),

    /// Configuration validation error
    ///
    /// Returned when configuration validation fails.
    /// Includes a description of what validation failed.
    #[error("Configuration invalid: {0}")]
    ConfigInvalid(String),

    /// Configuration key not found error
    ///
    /// Returned when attempting to access a configuration key
    /// that doesn't exist in the configuration file.
    #[error("Configuration key not found: {0}")]
    ConfigKeyNotFound(String),

    /// Configuration file not found error
    ///
    /// Returned when attempting to load a configuration file
    /// that doesn't exist at the specified path.
    #[error("Configuration file not found: {0}")]
    ConfigFileNotFound(String),

    /// I/O error
    ///
    /// Wrapper for standard I/O errors from file system operations.
    /// Automatically converted from `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// YAML parsing/serialization error
    ///
    /// Wrapper for YAML errors from the serde_yaml crate.
    /// Automatically converted from `serde_yaml::Error`.
    #[error("YAML error: {0}")]
    Yaml(#[from] serde_yaml::Error),

    /// JSON parsing/serialization error
    ///
    /// Wrapper for JSON errors from the serde_json crate.
    /// Automatically converted from `serde_json::Error`.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Template processing error
    ///
    /// Returned when template generation or processing fails.
    /// Includes a description of the template error.
    #[error("Template error: {0}")]
    Template(String),

    // =========================================================================
    // Connection Test Errors
    // =========================================================================

    /// Connection test failed error
    ///
    /// Returned when a connection test to an external service fails.
    /// Includes the service type and error details.
    #[error("Connection test failed for {service}: {message}")]
    ConnectionTestFailed {
        /// The service being tested (redis, postgres, mysql, kafka)
        service: String,
        /// Detailed error message
        message: String,
    },

    /// Invalid connection URL error
    ///
    /// Returned when the provided connection URL is malformed or invalid.
    /// Includes the URL and expected format.
    #[error("Invalid connection URL '{url}': {reason}")]
    InvalidConnectionUrl {
        /// The invalid URL
        url: String,
        /// Reason for invalidity
        reason: String,
    },

    /// Connection timeout error
    ///
    /// Returned when a connection attempt times out.
    /// Includes the service type and timeout duration.
    #[error("Connection timeout for {service} after {timeout_ms}ms")]
    ConnectionTimeout {
        /// The service being tested
        service: String,
        /// Timeout duration in milliseconds
        timeout_ms: u64,
    },

    /// Authentication failed error
    ///
    /// Returned when authentication to a service fails.
    /// Includes the service type and error details.
    #[error("Authentication failed for {service}: {message}")]
    AuthenticationFailed {
        /// The service being tested
        service: String,
        /// Error message from the service
        message: String,
    },

    /// Service not available error
    ///
    /// Returned when the target service is not available or not running.
    /// Includes the service type and connection details.
    #[error("Service not available: {service} at {address}")]
    ServiceNotAvailable {
        /// The service being tested
        service: String,
        /// The address attempted
        address: String,
    },

    // =========================================================================
    // Doctor Diagnostic Errors
    // =========================================================================

    /// Doctor diagnostic error
    ///
    /// Returned when a diagnostic check fails.
    /// Includes a description of what check failed.
    #[error("Doctor diagnostic error: {0}")]
    DoctorFailed(String),

    /// Doctor auto-fix error
    ///
    /// Returned when an auto-fix operation fails.
    /// Includes a description of what fix failed.
    #[error("Doctor auto-fix error: {0}")]
    DoctorFixFailed(String),

    // =========================================================================
    // Code Generation Errors
    // =========================================================================

    /// Invalid module type error
    ///
    /// Returned when an unsupported module type is specified.
    /// Includes the invalid type and list of valid types.
    #[error("Invalid module type '{module_type}'. Valid types: {valid_types}")]
    InvalidModuleType {
        /// The invalid module type
        module_type: String,
        /// Comma-separated list of valid module types
        valid_types: String,
    },

    /// Module already exists error
    ///
    /// Returned when attempting to generate a module that already exists.
    /// Includes the module name and path.
    #[error("Module '{name}' already exists at {path}")]
    ModuleExists {
        /// The module name
        name: String,
        /// Path where the module exists
        path: String,
    },

    /// Middleware already exists error
    ///
    /// Returned when attempting to generate middleware that already exists.
    /// Includes the middleware name and file path.
    #[error("Middleware '{name}' already exists at {path}")]
    MiddlewareExists {
        /// The middleware name
        name: String,
        /// Path to the middleware file
        path: String,
    },

    /// Config file not found for generation error
    ///
    /// Returned when the config file for struct generation doesn't exist.
    /// Includes the file path.
    #[error("Config file not found: {0}")]
    GenerateConfigFileNotFound(String),

    /// Unsupported config format error
    ///
    /// Returned when the config file format is not supported.
    /// Includes the file extension and supported formats.
    #[error("Unsupported config format '{format}'. Supported formats: {supported}")]
    UnsupportedConfigFormat {
        /// The unsupported format (file extension)
        format: String,
        /// Comma-separated list of supported formats
        supported: String,
    },

    /// Code generation error
    ///
    /// Returned when code generation fails.
    /// Includes a description of what generation failed.
    #[error("Code generation error: {0}")]
    GenerationFailed(String),

    /// Code formatting error
    ///
    /// Returned when rustfmt fails to format generated code.
    /// Includes the error message from rustfmt.
    #[error("Code formatting error: {0}")]
    FormattingFailed(String),

    /// Template rendering error
    ///
    /// Returned when template rendering fails during code generation.
    /// Includes the template name and error details.
    #[error("Template rendering error for '{template}': {message}")]
    TemplateRenderError {
        /// The template that failed
        template: String,
        /// Error message
        message: String,
    },
}
