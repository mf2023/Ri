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

//! Configuration Validator
//!
//! This module provides comprehensive validation for Ri configuration files.
//! It supports multiple validation levels including syntax, structure, and
//! semantic validation with detailed error messages and fix suggestions.
//!
//! # Validation Levels
//!
//! 1. **Syntax Validation**: Validates YAML/JSON syntax correctness
//! 2. **Structure Validation**: Validates configuration structure against schemas
//! 3. **Logic Validation**: Validates configuration logic and dependencies
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::config::validator::ConfigValidator;
//!
//! // Create a validator
//! let validator = ConfigValidator::new();
//!
//! // Validate a configuration file
//! let result = validator.validate_file("config.yaml")?;
//!
//! // Check validation result
//! if result.is_valid() {
//!     println!("Configuration is valid!");
//! } else {
//!     // Print errors
//!     for error in result.errors() {
//!         println!("Error: {}", error);
//!     }
//!     
//!     // Print suggestions
//!     for suggestion in result.suggestions() {
//!         println!("Suggestion: {}", suggestion);
//!     }
//! }
//! ```

use std::path::Path;
use std::collections::HashMap;

use crate::error::Result;
use crate::config_validation::schema::{ConfigSchema, all_schemas, ValidationSeverity, ValidationError};

/// Configuration validator for Ri configuration files.
///
/// This struct provides methods for validating configuration files
/// at multiple levels: syntax, structure, and logic.
pub struct ConfigValidator {
    /// Registered configuration schemas
    schemas: Vec<Box<dyn ConfigSchema>>,
    /// Validation errors collected during validation
    errors: Vec<ValidationError>,
    /// Validation warnings collected during validation
    warnings: Vec<ValidationWarning>,
    /// Fix suggestions for validation issues
    suggestions: Vec<ValidationSuggestion>,
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigValidator {
    /// Creates a new configuration validator with all default schemas.
    ///
    /// # Returns
    ///
    /// A new `ConfigValidator` instance with all Ri module schemas registered
    pub fn new() -> Self {
        Self {
            schemas: all_schemas(),
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Creates a new configuration validator with custom schemas.
    ///
    /// # Parameters
    ///
    /// - `schemas`: Vector of custom configuration schemas to use
    ///
    /// # Returns
    ///
    /// A new `ConfigValidator` instance with custom schemas
    pub fn with_schemas(schemas: Vec<Box<dyn ConfigSchema>>) -> Self {
        Self {
            schemas,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Validates YAML/JSON syntax of configuration content.
    ///
    /// This method checks if the configuration content is valid YAML or JSON.
    /// It does not validate the structure or logic of the configuration.
    ///
    /// # Parameters
    ///
    /// - `content`: The configuration content as a string
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed YAML value or an error
    ///
    /// # Errors
    ///
    /// Returns an error if the content is not valid YAML/JSON
    pub fn validate_syntax(&mut self, content: &str) -> Result<serde_yaml::Value> {
        // Try to parse as YAML (which also handles JSON)
        match serde_yaml::from_str::<serde_yaml::Value>(content) {
            Ok(value) => Ok(value),
            Err(e) => {
                let error = ValidationError {
                    field_path: "<root>".to_string(),
                    message: format!("Syntax error: {}", e),
                    suggestion: self.get_syntax_fix_suggestion(&e.to_string()),
                    severity: ValidationSeverity::Error,
                };
                self.errors.push(error);
                Err(crate::error::RicError::ConfigInvalid(
                    format!("Configuration syntax error: {}", e)
                ))
            }
        }
    }

    /// Validates configuration structure against registered schemas.
    ///
    /// This method checks if the configuration structure matches the expected
    /// schema for each Ri module, including required fields and types.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors found
    pub fn validate_structure(&mut self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut all_errors = Vec::new();

        // Validate against each registered schema
        for schema in &self.schemas {
            let errors = schema.validate(config);
            all_errors.extend(errors.clone());
            
            // Categorize errors and generate suggestions
            for error in errors {
                match error.severity {
                    ValidationSeverity::Error => {
                        self.errors.push(error.clone());
                        if let Some(suggestion) = &error.suggestion {
                            self.suggestions.push(ValidationSuggestion {
                                field_path: error.field_path.clone(),
                                suggestion: suggestion.clone(),
                            });
                        }
                    }
                    ValidationSeverity::Warning => {
                        self.warnings.push(ValidationWarning {
                            field_path: error.field_path.clone(),
                            message: error.message.clone(),
                            suggestion: error.suggestion.clone(),
                        });
                    }
                    ValidationSeverity::Info => {
                        // Info level messages are added as suggestions
                        if let Some(suggestion) = &error.suggestion {
                            self.suggestions.push(ValidationSuggestion {
                                field_path: error.field_path.clone(),
                                suggestion: format!("Info: {}", suggestion),
                            });
                        }
                    }
                }
            }
        }

        all_errors
    }

    /// Validates configuration logic and dependencies.
    ///
    /// This method checks for logical consistency in the configuration,
    /// including field dependencies, cross-field validations, and
    /// semantic correctness.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors found
    pub fn validate_logic(&mut self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check for common configuration issues
        errors.extend(self.check_port_conflicts(config));
        errors.extend(self.check_url_validity(config));
        errors.extend(self.check_timeout_consistency(config));
        errors.extend(self.check_feature_dependencies(config));

        // Add errors to internal state
        for error in &errors {
            if error.severity == ValidationSeverity::Error {
                self.errors.push(error.clone());
            } else if error.severity == ValidationSeverity::Warning {
                self.warnings.push(ValidationWarning {
                    field_path: error.field_path.clone(),
                    message: error.message.clone(),
                    suggestion: error.suggestion.clone(),
                });
            }
        }

        errors
    }

    /// Validates a configuration file at the given path.
    ///
    /// This method performs all validation levels (syntax, structure, logic)
    /// on the configuration file and returns a comprehensive validation result.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the configuration file
    ///
    /// # Returns
    ///
    /// A `Result` containing the validation result
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read
    pub fn validate_file<P: AsRef<Path>>(&mut self, path: P) -> Result<ValidationResult> {
        // Clear previous validation state
        self.errors.clear();
        self.warnings.clear();
        self.suggestions.clear();

        // Read file content
        let content = std::fs::read_to_string(path.as_ref())?;

        // Validate syntax
        let config = self.validate_syntax(&content)?;

        // Validate structure
        self.validate_structure(&config);

        // Validate logic
        self.validate_logic(&config);

        Ok(ValidationResult {
            is_valid: self.errors.is_empty(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
            suggestions: self.suggestions.clone(),
        })
    }

    /// Validates configuration content directly.
    ///
    /// This method performs all validation levels on the configuration
    /// content string and returns a comprehensive validation result.
    ///
    /// # Parameters
    ///
    /// - `content`: Configuration content as a string
    ///
    /// # Returns
    ///
    /// A `Result` containing the validation result
    pub fn validate_content(&mut self, content: &str) -> Result<ValidationResult> {
        // Clear previous validation state
        self.errors.clear();
        self.warnings.clear();
        self.suggestions.clear();

        // Validate syntax
        let config = self.validate_syntax(content)?;

        // Validate structure
        self.validate_structure(&config);

        // Validate logic
        self.validate_logic(&config);

        Ok(ValidationResult {
            is_valid: self.errors.is_empty(),
            errors: self.errors.clone(),
            warnings: self.warnings.clone(),
            suggestions: self.suggestions.clone(),
        })
    }

    /// Returns all validation errors.
    ///
    /// # Returns
    ///
    /// A slice of all validation errors found
    pub fn get_validation_errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Returns all validation warnings.
    ///
    /// # Returns
    ///
    /// A slice of all validation warnings found
    pub fn get_validation_warnings(&self) -> &[ValidationWarning] {
        &self.warnings
    }

    /// Returns all fix suggestions.
    ///
    /// # Returns
    ///
    /// A slice of all fix suggestions for validation issues
    pub fn get_suggestions(&self) -> &[ValidationSuggestion] {
        &self.suggestions
    }

    /// Clears all validation state.
    ///
    /// This method resets the validator to its initial state,
    /// clearing all errors, warnings, and suggestions.
    pub fn clear(&mut self) {
        self.errors.clear();
        self.warnings.clear();
        self.suggestions.clear();
    }

    // =========================================================================
    // Private Helper Methods
    // =========================================================================

    /// Generates a fix suggestion for syntax errors.
    ///
    /// # Parameters
    ///
    /// - `error_message`: The syntax error message
    ///
    /// # Returns
    ///
    /// A suggested fix for the syntax error
    fn get_syntax_fix_suggestion(&self, error_message: &str) -> Option<String> {
        if error_message.contains("did not find expected key") {
            Some("Check for missing colons after keys or incorrect indentation".to_string())
        } else if error_message.contains("could not find expected ':'") {
            Some("Add a colon after the key name".to_string())
        } else if error_message.contains("unexpected character") {
            Some("Check for special characters that need to be quoted".to_string())
        } else if error_message.contains("mapping values are not allowed here") {
            Some("Check indentation and ensure key-value pairs are properly formatted".to_string())
        } else if error_message.contains("invalid type") {
            Some("Check that the value type matches the expected type for this field".to_string())
        } else {
            Some("Review YAML syntax documentation at https://yaml.org/spec/".to_string())
        }
    }

    /// Checks for port conflicts across modules.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors for port conflicts
    fn check_port_conflicts(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        let mut ports: HashMap<u16, String> = HashMap::new();

        // Check gateway port
        if let Some(gateway) = config.get("gateway") {
            if let Some(port) = gateway.get("listen_port").and_then(|p| p.as_u64()) {
                let port = port as u16;
                if let Some(existing) = ports.insert(port, "gateway.listen_port".to_string()) {
                    errors.push(ValidationError {
                        field_path: "gateway.listen_port".to_string(),
                        message: format!("Port {} is already used by {}", port, existing),
                        suggestion: Some(format!("Change gateway.listen_port to an unused port")),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }

        // Add more port checks for other modules as needed

        errors
    }

    /// Checks for URL validity across configuration.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors for invalid URLs
    fn check_url_validity(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check Redis URL in cache config
        if let Some(cache) = config.get("cache") {
            if let Some(url) = cache.get("redis_url").and_then(|u| u.as_str()) {
                if !url.starts_with("redis://") && !url.starts_with("rediss://") {
                    errors.push(ValidationError {
                        field_path: "cache.redis_url".to_string(),
                        message: format!("Invalid Redis URL format: {}", url),
                        suggestion: Some("Use format: redis://host:port or redis://host:port/db".to_string()),
                        severity: ValidationSeverity::Error,
                    });
                }
            }
        }

        // Check connection string in queue config
        if let Some(queue) = config.get("queue") {
            if let Some(conn) = queue.get("connection_string").and_then(|c| c.as_str()) {
                let backend = queue.get("backend_type").and_then(|b| b.as_str()).unwrap_or("Memory");
                
                match backend {
                    "RabbitMQ" if !conn.starts_with("amqp://") && !conn.starts_with("amqps://") => {
                        errors.push(ValidationError {
                            field_path: "queue.connection_string".to_string(),
                            message: format!("Invalid RabbitMQ connection string: {}", conn),
                            suggestion: Some("Use format: amqp://user:pass@host:port/vhost".to_string()),
                            severity: ValidationSeverity::Error,
                        });
                    }
                    "Redis" if !conn.starts_with("redis://") && !conn.starts_with("rediss://") => {
                        errors.push(ValidationError {
                            field_path: "queue.connection_string".to_string(),
                            message: format!("Invalid Redis connection string: {}", conn),
                            suggestion: Some("Use format: redis://host:port".to_string()),
                            severity: ValidationSeverity::Error,
                        });
                    }
                    _ => {}
                }
            }
        }

        errors
    }

    /// Checks for timeout consistency across configuration.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors for timeout inconsistencies
    fn check_timeout_consistency(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check gateway timeout vs auth JWT expiry
        if let (Some(gateway), Some(auth)) = (config.get("gateway"), config.get("auth")) {
            let gateway_timeout = gateway.get("request_timeout_seconds")
                .and_then(|t| t.as_i64()).unwrap_or(30);
            let jwt_expiry = auth.get("jwt_expiry_secs")
                .and_then(|e| e.as_i64()).unwrap_or(3600);

            // If gateway timeout is longer than JWT expiry, requests might fail
            if gateway_timeout > jwt_expiry {
                errors.push(ValidationError {
                    field_path: "gateway.request_timeout_seconds".to_string(),
                    message: format!(
                        "Gateway timeout ({}s) is longer than JWT expiry ({}s). \
                         Long-running requests may fail due to token expiration.",
                        gateway_timeout, jwt_expiry
                    ),
                    suggestion: Some(format!(
                        "Consider increasing jwt_expiry_secs to at least {} or decreasing request_timeout_seconds",
                        gateway_timeout
                    )),
                    severity: ValidationSeverity::Warning,
                });
            }
        }

        // Check queue timeouts
        if let Some(queue) = config.get("queue") {
            let consumer_timeout = queue.get("consumer_timeout_ms")
                .and_then(|t| t.as_i64()).unwrap_or(30000);
            let producer_timeout = queue.get("producer_timeout_ms")
                .and_then(|t| t.as_i64()).unwrap_or(5000);

            if producer_timeout > consumer_timeout {
                errors.push(ValidationError {
                    field_path: "queue.producer_timeout_ms".to_string(),
                    message: format!(
                        "Producer timeout ({}ms) is greater than consumer timeout ({}ms). \
                         This may cause inconsistent behavior.",
                        producer_timeout, consumer_timeout
                    ),
                    suggestion: Some("Consider adjusting timeouts for consistency".to_string()),
                    severity: ValidationSeverity::Warning,
                });
            }
        }

        errors
    }

    /// Checks for feature dependencies across configuration.
    ///
    /// # Parameters
    ///
    /// - `config`: The parsed configuration value
    ///
    /// # Returns
    ///
    /// A vector of validation errors for missing feature dependencies
    fn check_feature_dependencies(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        // Check if observability is enabled but tracing is disabled
        if let Some(obs) = config.get("observability") {
            let obs_enabled = obs.get("tracing_enabled")
                .and_then(|e| e.as_bool()).unwrap_or(true);
            
            if obs_enabled {
                // Check if there are modules that could benefit from tracing
                let has_gateway = config.get("gateway").is_some();
                let has_queue = config.get("queue").is_some();
                
                if has_gateway || has_queue {
                    // This is just an informational suggestion
                    errors.push(ValidationError {
                        field_path: "observability.tracing_enabled".to_string(),
                        message: "Tracing is enabled. Consider enabling tracing for gateway and queue modules for better observability.".to_string(),
                        suggestion: Some("Tracing helps debug distributed systems and monitor performance".to_string()),
                        severity: ValidationSeverity::Info,
                    });
                }
            }
        }

        // Check if auth is enabled but cache is disabled (sessions need cache)
        if let (Some(auth), Some(cache)) = (config.get("auth"), config.get("cache")) {
            let auth_enabled = auth.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true);
            let session_auth = auth.get("enable_session_auth").and_then(|e| e.as_bool()).unwrap_or(true);
            let cache_enabled = cache.get("enabled").and_then(|e| e.as_bool()).unwrap_or(true);

            if auth_enabled && session_auth && !cache_enabled {
                errors.push(ValidationError {
                    field_path: "cache.enabled".to_string(),
                    message: "Session authentication is enabled but cache is disabled. Sessions require a cache backend.".to_string(),
                    suggestion: Some("Enable cache or disable session authentication".to_string()),
                    severity: ValidationSeverity::Warning,
                });
            }
        }

        errors
    }
}

/// Result of configuration validation.
///
/// This struct contains the complete result of validating a configuration
/// file, including errors, warnings, and fix suggestions.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether the configuration is valid (no errors)
    is_valid: bool,
    /// Validation errors found
    errors: Vec<ValidationError>,
    /// Validation warnings found
    warnings: Vec<ValidationWarning>,
    /// Fix suggestions for validation issues
    suggestions: Vec<ValidationSuggestion>,
}

impl ValidationResult {
    /// Returns whether the configuration is valid.
    ///
    /// # Returns
    ///
    /// `true` if there are no validation errors
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Returns all validation errors.
    ///
    /// # Returns
    ///
    /// A slice of all validation errors
    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }

    /// Returns all validation warnings.
    ///
    /// # Returns
    ///
    /// A slice of all validation warnings
    pub fn warnings(&self) -> &[ValidationWarning] {
        &self.warnings
    }

    /// Returns all fix suggestions.
    ///
    /// # Returns
    ///
    /// A slice of all fix suggestions
    pub fn suggestions(&self) -> &[ValidationSuggestion] {
        &self.suggestions
    }

    /// Returns the number of errors.
    ///
    /// # Returns
    ///
    /// The count of validation errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Returns the number of warnings.
    ///
    /// # Returns
    ///
    /// The count of validation warnings
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    /// Formats the validation result as a human-readable string.
    ///
    /// # Returns
    ///
    /// A formatted string of the validation result
    pub fn format(&self) -> String {
        let mut output = String::new();

        if self.is_valid {
            output.push_str("Configuration is valid!\n");
        } else {
            output.push_str(&format!("Configuration has {} error(s) and {} warning(s)\n\n",
                self.errors.len(), self.warnings.len()));
        }

        if !self.errors.is_empty() {
            output.push_str("Errors:\n");
            for (i, error) in self.errors.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, error));
            }
            output.push('\n');
        }

        if !self.warnings.is_empty() {
            output.push_str("Warnings:\n");
            for (i, warning) in self.warnings.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, warning));
            }
            output.push('\n');
        }

        if !self.suggestions.is_empty() {
            output.push_str("Suggestions:\n");
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, suggestion));
            }
        }

        output
    }
}

impl std::fmt::Display for crate::config_validation::schema::ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}: {}", 
            match self.severity {
                crate::config_validation::schema::ValidationSeverity::Error => "ERROR",
                crate::config_validation::schema::ValidationSeverity::Warning => "WARN",
                crate::config_validation::schema::ValidationSeverity::Info => "INFO",
            },
            self.field_path,
            self.message
        )
    }
}

/// Validation warning for configuration fields.
///
/// This struct represents a non-critical validation issue that
/// should be addressed but doesn't prevent configuration usage.
#[derive(Debug, Clone)]
pub struct ValidationWarning {
    /// Path to the field with the warning (dot notation)
    pub field_path: String,
    /// Warning message
    pub message: String,
    /// Suggested fix for the warning
    pub suggestion: Option<String>,
}

impl std::fmt::Display for ValidationWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field_path, self.message)?;
        if let Some(suggestion) = &self.suggestion {
            write!(f, " Suggestion: {}", suggestion)?;
        }
        Ok(())
    }
}

/// Fix suggestion for a validation issue.
///
/// This struct represents a suggested fix for a configuration
/// validation issue.
#[derive(Debug, Clone)]
pub struct ValidationSuggestion {
    /// Path to the field the suggestion applies to
    pub field_path: String,
    /// The suggested fix
    pub suggestion: String,
}

impl std::fmt::Display for ValidationSuggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field_path, self.suggestion)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_creation() {
        let validator = ConfigValidator::new();
        assert!(!validator.schemas.is_empty());
    }

    #[test]
    fn test_syntax_validation_valid_yaml() {
        let mut validator = ConfigValidator::new();
        let yaml = "key: value\nnumber: 42";
        let result = validator.validate_syntax(yaml);
        assert!(result.is_ok());
    }

    #[test]
    fn test_syntax_validation_invalid_yaml() {
        let mut validator = ConfigValidator::new();
        let yaml = "key: value\n  invalid indent";
        let result = validator.validate_syntax(yaml);
        assert!(result.is_err());
        assert!(!validator.errors.is_empty());
    }

    #[test]
    fn test_validation_result_format() {
        let result = ValidationResult {
            is_valid: true,
            errors: vec![],
            warnings: vec![],
            suggestions: vec![],
        };
        assert!(result.is_valid());
        assert!(result.format().contains("valid"));
    }

    #[test]
    fn test_error_display() {
        let error = ValidationError {
            field_path: "cache.enabled".to_string(),
            message: "must be a boolean".to_string(),
            suggestion: Some("Use true or false".to_string()),
            severity: ValidationSeverity::Error,
        };
        let display = format!("{}", error);
        assert!(display.contains("ERROR"));
        assert!(display.contains("cache.enabled"));
    }
}
