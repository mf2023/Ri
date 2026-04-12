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

//! Configuration Schema Definitions
//!
//! This module defines the configuration schemas for all Ri framework modules.
//! Each schema specifies the structure, constraints, and validation rules for
//! module-specific configuration.
//!
//! # Schema Components
//!
//! - `ConfigSchema` - Base trait for all configuration schemas
//! - `FieldSchema` - Schema definition for individual configuration fields
//! - `FieldType` - Supported field types in configuration
//! - `FieldConstraint` - Constraints applied to field values
//!
//! # Design Principles
//!
//! - **Declarative**: Schemas are declarative and easy to understand
//! - **Composable**: Complex schemas can be built from simpler ones
//! - **Validatable**: Each schema provides its own validation logic
//! - **Documentable**: Schemas include documentation for each field

use std::collections::HashMap;

/// Supported field types in configuration schemas.
///
/// This enum defines the primitive and complex types that can be used
/// in configuration field definitions.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    /// String type for text values
    String,
    /// Integer type for whole numbers
    Integer,
    /// Float type for decimal numbers
    Float,
    /// Boolean type for true/false values
    Boolean,
    /// Array type for lists of values
    Array(Box<FieldType>),
    /// Object type for nested structures
    Object(String),
    /// Enum type with allowed values
    Enum(Vec<String>),
    /// Optional type wrapper
    Optional(Box<FieldType>),
}

impl FieldType {
    /// Returns a human-readable name for the field type.
    ///
    /// # Returns
    ///
    /// A string representation of the field type
    pub fn type_name(&self) -> String {
        match self {
            FieldType::String => "string".to_string(),
            FieldType::Integer => "integer".to_string(),
            FieldType::Float => "float".to_string(),
            FieldType::Boolean => "boolean".to_string(),
            FieldType::Array(inner) => format!("array<{}>", inner.type_name()),
            FieldType::Object(name) => format!("object({})", name),
            FieldType::Enum(values) => format!("enum({})", values.join("|")),
            FieldType::Optional(inner) => format!("optional<{}>", inner.type_name()),
        }
    }
}

/// Constraints applied to configuration field values.
///
/// Field constraints define validation rules that field values must satisfy.
/// These constraints are used during configuration validation to ensure
/// values are within acceptable ranges and formats.
#[derive(Debug, Clone)]
pub enum FieldConstraint {
    /// Minimum value constraint for numeric types
    MinValue(f64),
    /// Maximum value constraint for numeric types
    MaxValue(f64),
    /// Minimum length constraint for strings and arrays
    MinLength(usize),
    /// Maximum length constraint for strings and arrays
    MaxLength(usize),
    /// Regular expression pattern constraint for strings
    Pattern(String),
    /// Allowed values constraint for enums
    AllowedValues(Vec<String>),
    /// Custom validation rule with description
    CustomRule {
        /// Name of the custom rule
        name: String,
        /// Description of what the rule validates
        description: String,
    },
}

impl FieldConstraint {
    /// Returns a human-readable description of the constraint.
    ///
    /// # Returns
    ///
    /// A string describing the constraint
    pub fn description(&self) -> String {
        match self {
            FieldConstraint::MinValue(v) => format!("minimum value: {}", v),
            FieldConstraint::MaxValue(v) => format!("maximum value: {}", v),
            FieldConstraint::MinLength(l) => format!("minimum length: {}", l),
            FieldConstraint::MaxLength(l) => format!("maximum length: {}", l),
            FieldConstraint::Pattern(p) => format!("must match pattern: {}", p),
            FieldConstraint::AllowedValues(v) => format!("allowed values: {}", v.join(", ")),
            FieldConstraint::CustomRule { name, description } => format!("{}: {}", name, description),
        }
    }
}

/// Schema definition for individual configuration fields.
///
/// This struct defines the structure and constraints for a single
/// configuration field, including its type, default value, and
/// validation rules.
#[derive(Debug, Clone)]
pub struct FieldSchema {
    /// Field name in the configuration
    pub name: String,
    /// Human-readable description of the field
    pub description: String,
    /// Field type definition
    pub field_type: FieldType,
    /// Whether the field is required
    pub required: bool,
    /// Default value for optional fields (as string representation)
    pub default: Option<String>,
    /// Constraints applied to the field value
    pub constraints: Vec<FieldConstraint>,
    /// Dependencies on other fields
    pub dependencies: Vec<String>,
    /// Dependent fields that must be present if this field is set
    pub dependent_fields: Vec<String>,
    /// Example value for documentation
    pub example: Option<String>,
}

impl FieldSchema {
    /// Creates a new field schema with the given name and type.
    ///
    /// # Parameters
    ///
    /// - `name`: Field name in the configuration
    /// - `field_type`: Field type definition
    ///
    /// # Returns
    ///
    /// A new `FieldSchema` instance with default values
    pub fn new(name: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            description: String::new(),
            field_type,
            required: true,
            default: None,
            constraints: Vec::new(),
            dependencies: Vec::new(),
            dependent_fields: Vec::new(),
            example: None,
        }
    }

    /// Sets the description for the field.
    ///
    /// # Parameters
    ///
    /// - `description`: Human-readable description
    ///
    /// # Returns
    ///
    /// The updated `FieldSchema` instance for method chaining
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Marks the field as optional with a default value.
    ///
    /// # Parameters
    ///
    /// - `default`: Default value as a string
    ///
    /// # Returns
    ///
    /// The updated `FieldSchema` instance for method chaining
    pub fn with_default(mut self, default: impl Into<String>) -> Self {
        self.required = false;
        self.default = Some(default.into());
        self
    }

    /// Adds a constraint to the field.
    ///
    /// # Parameters
    ///
    /// - `constraint`: Constraint to add
    ///
    /// # Returns
    ///
    /// The updated `FieldSchema` instance for method chaining
    pub fn with_constraint(mut self, constraint: FieldConstraint) -> Self {
        self.constraints.push(constraint);
        self
    }

    /// Adds a dependency on another field.
    ///
    /// # Parameters
    ///
    /// - `field`: Name of the required field
    ///
    /// # Returns
    ///
    /// The updated `FieldSchema` instance for method chaining
    pub fn with_dependency(mut self, field: impl Into<String>) -> Self {
        self.dependencies.push(field.into());
        self
    }

    /// Sets an example value for documentation.
    ///
    /// # Parameters
    ///
    /// - `example`: Example value as a string
    ///
    /// # Returns
    ///
    /// The updated `FieldSchema` instance for method chaining
    pub fn with_example(mut self, example: impl Into<String>) -> Self {
        self.example = Some(example.into());
        self
    }
}

/// Base trait for configuration schemas.
///
/// This trait defines the interface for all configuration schemas.
/// Each module-specific schema implements this trait to provide
/// validation rules and field definitions.
pub trait ConfigSchema {
    /// Returns the name of the schema.
    ///
    /// # Returns
    ///
    /// The schema name as a string
    fn name(&self) -> &str;

    /// Returns the description of the schema.
    ///
    /// # Returns
    ///
    /// A human-readable description of what this schema validates
    fn description(&self) -> &str;

    /// Returns all field schemas defined in this schema.
    ///
    /// # Returns
    ///
    /// A map of field names to their schemas
    fn fields(&self) -> &HashMap<String, FieldSchema>;

    /// Returns the root key for this schema in the configuration.
    ///
    /// # Returns
    ///
    /// The root key name (e.g., "cache", "queue", "gateway")
    fn root_key(&self) -> &str;

    /// Validates a configuration value against this schema.
    ///
    /// # Parameters
    ///
    /// - `config`: The configuration value to validate
    ///
    /// # Returns
    ///
    /// A vector of validation errors, empty if valid
    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError>;
}

/// Validation error for configuration fields.
///
/// This struct represents a single validation error found during
/// configuration validation, including the field path, error message,
/// and suggested fix.
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// Path to the field with the error (dot notation)
    pub field_path: String,
    /// Error message describing the validation failure
    pub message: String,
    /// Suggested fix for the error
    pub suggestion: Option<String>,
    /// Severity of the error
    pub severity: ValidationSeverity,
}

/// Severity level for validation errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationSeverity {
    /// Critical error that prevents configuration from being used
    Error,
    /// Warning that should be addressed but doesn't prevent usage
    Warning,
    /// Informational message about potential issues
    Info,
}

// =============================================================================
// Module-Specific Schemas
// =============================================================================

/// Configuration schema for RiAppBuilder.
///
/// This schema validates the configuration for the RiAppBuilder,
/// which is the main application builder for Ri framework applications.
#[derive(Debug, Clone)]
pub struct AppBuilderSchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for AppBuilderSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl AppBuilderSchema {
    /// Creates a new AppBuilderSchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `AppBuilderSchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Configuration paths
        fields.insert(
            "config_paths".to_string(),
            FieldSchema::new("config_paths", FieldType::Array(Box::new(FieldType::String)))
                .with_description("List of configuration file paths to load")
                .with_default("[]")
                .with_example("['config/dms.yaml', 'config/local.yaml']"),
        );

        // Logging configuration
        fields.insert(
            "logging".to_string(),
            FieldSchema::new("logging", FieldType::Object("RiLogConfig".to_string()))
                .with_description("Logging configuration for the application")
                .with_default("{}"),
        );

        // Observability configuration
        fields.insert(
            "observability".to_string(),
            FieldSchema::new("observability", FieldType::Object("RiObservabilityConfig".to_string()))
                .with_description("Observability configuration including tracing and metrics")
                .with_default("{}"),
        );

        // Modules configuration
        fields.insert(
            "modules".to_string(),
            FieldSchema::new("modules", FieldType::Array(Box::new(FieldType::Object("ModuleConfig".to_string()))))
                .with_description("List of modules to register with the application")
                .with_default("[]"),
        );

        Self { fields }
    }
}

impl ConfigSchema for AppBuilderSchema {
    fn name(&self) -> &str {
        "AppBuilder"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiAppBuilder, the main application builder for Ri framework"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "app"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(app_config) = config.get(self.root_key()) {
            if let Some(obj) = app_config.as_mapping() {
                // Validate config_paths
                if let Some(paths) = obj.get(&serde_yaml::Value::String("config_paths".to_string())) {
                    if !paths.is_sequence() {
                        errors.push(ValidationError {
                            field_path: "app.config_paths".to_string(),
                            message: "config_paths must be an array of strings".to_string(),
                            suggestion: Some("Change to: config_paths: [\"path/to/config.yaml\"]".to_string()),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }

                // Validate logging
                if let Some(logging) = obj.get(&serde_yaml::Value::String("logging".to_string())) {
                    if !logging.is_mapping() {
                        errors.push(ValidationError {
                            field_path: "app.logging".to_string(),
                            message: "logging must be an object".to_string(),
                            suggestion: Some("Change to: logging: { level: \"info\" }".to_string()),
                            severity: ValidationSeverity::Error,
                        });
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiCacheModule.
///
/// This schema validates the configuration for the cache module,
/// including backend selection, TTL settings, and memory limits.
#[derive(Debug, Clone)]
pub struct CacheModuleSchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for CacheModuleSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl CacheModuleSchema {
    /// Creates a new CacheModuleSchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `CacheModuleSchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Enabled flag
        fields.insert(
            "enabled".to_string(),
            FieldSchema::new("enabled", FieldType::Boolean)
                .with_description("Whether caching is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Default TTL
        fields.insert(
            "default_ttl_secs".to_string(),
            FieldSchema::new("default_ttl_secs", FieldType::Integer)
                .with_description("Default time-to-live in seconds")
                .with_default("3600")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(86400.0))
                .with_example("3600"),
        );

        // Maximum memory
        fields.insert(
            "max_memory_mb".to_string(),
            FieldSchema::new("max_memory_mb", FieldType::Integer)
                .with_description("Maximum memory usage in megabytes")
                .with_default("512")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(102400.0))
                .with_example("1024"),
        );

        // Cleanup interval
        fields.insert(
            "cleanup_interval_secs".to_string(),
            FieldSchema::new("cleanup_interval_secs", FieldType::Integer)
                .with_description("Interval for cleaning up expired entries in seconds")
                .with_default("300")
                .with_constraint(FieldConstraint::MinValue(10.0))
                .with_constraint(FieldConstraint::MaxValue(3600.0))
                .with_example("300"),
        );

        // Backend type
        fields.insert(
            "backend_type".to_string(),
            FieldSchema::new("backend_type", FieldType::Enum(vec![
                "Memory".to_string(),
                "Redis".to_string(),
                "Hybrid".to_string(),
            ]))
                .with_description("Type of cache backend to use")
                .with_default("Memory")
                .with_example("Redis"),
        );

        // Redis URL
        fields.insert(
            "redis_url".to_string(),
            FieldSchema::new("redis_url", FieldType::String)
                .with_description("Redis connection URL (if using Redis or Hybrid backend)")
                .with_default("redis://127.0.0.1:6379")
                .with_constraint(FieldConstraint::Pattern("redis://.*".to_string()))
                .with_example("redis://localhost:6379/1"),
        );

        // Redis pool size
        fields.insert(
            "redis_pool_size".to_string(),
            FieldSchema::new("redis_pool_size", FieldType::Integer)
                .with_description("Redis connection pool size")
                .with_default("10")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(100.0))
                .with_example("20"),
        );

        Self { fields }
    }
}

impl ConfigSchema for CacheModuleSchema {
    fn name(&self) -> &str {
        "CacheModule"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiCacheModule, providing caching functionality"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "cache"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(cache_config) = config.get(self.root_key()) {
            if let Some(obj) = cache_config.as_mapping() {
                // Validate backend_type
                if let Some(backend) = obj.get(&serde_yaml::Value::String("backend_type".to_string())) {
                    if let Some(backend_str) = backend.as_str() {
                        let valid_backends = ["Memory", "Redis", "Hybrid"];
                        if !valid_backends.contains(&backend_str) {
                            errors.push(ValidationError {
                                field_path: "cache.backend_type".to_string(),
                                message: format!("Invalid backend_type '{}'. Must be one of: {}", 
                                    backend_str, valid_backends.join(", ")),
                                suggestion: Some("Use one of: Memory, Redis, or Hybrid".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }

                        // Check redis_url is present when using Redis or Hybrid
                        if backend_str == "Redis" || backend_str == "Hybrid" {
                            if !obj.contains_key(&serde_yaml::Value::String("redis_url".to_string())) {
                                errors.push(ValidationError {
                                    field_path: "cache.redis_url".to_string(),
                                    message: "redis_url is required when backend_type is Redis or Hybrid".to_string(),
                                    suggestion: Some("Add: redis_url: \"redis://localhost:6379\"".to_string()),
                                    severity: ValidationSeverity::Error,
                                });
                            }
                        }
                    }
                }

                // Validate TTL range
                if let Some(ttl) = obj.get(&serde_yaml::Value::String("default_ttl_secs".to_string())) {
                    if let Some(ttl_val) = ttl.as_i64() {
                        if ttl_val < 1 {
                            errors.push(ValidationError {
                                field_path: "cache.default_ttl_secs".to_string(),
                                message: "default_ttl_secs must be at least 1 second".to_string(),
                                suggestion: Some("Set default_ttl_secs to at least 1".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                        if ttl_val > 86400 {
                            errors.push(ValidationError {
                                field_path: "cache.default_ttl_secs".to_string(),
                                message: "default_ttl_secs should not exceed 86400 seconds (24 hours)".to_string(),
                                suggestion: Some("Consider using a smaller TTL value".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiQueueModule.
///
/// This schema validates the configuration for the message queue module,
/// including backend selection, retry policies, and dead letter queues.
#[derive(Debug, Clone)]
pub struct QueueModuleSchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for QueueModuleSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueModuleSchema {
    /// Creates a new QueueModuleSchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `QueueModuleSchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Enabled flag
        fields.insert(
            "enabled".to_string(),
            FieldSchema::new("enabled", FieldType::Boolean)
                .with_description("Whether the queue system is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Backend type
        fields.insert(
            "backend_type".to_string(),
            FieldSchema::new("backend_type", FieldType::Enum(vec![
                "Memory".to_string(),
                "RabbitMQ".to_string(),
                "Kafka".to_string(),
                "Redis".to_string(),
            ]))
                .with_description("The type of queue backend to use")
                .with_default("Memory")
                .with_example("RabbitMQ"),
        );

        // Connection string
        fields.insert(
            "connection_string".to_string(),
            FieldSchema::new("connection_string", FieldType::String)
                .with_description("Connection string for the queue backend")
                .with_default("memory://localhost")
                .with_example("amqp://guest:guest@localhost:5672/"),
        );

        // Max connections
        fields.insert(
            "max_connections".to_string(),
            FieldSchema::new("max_connections", FieldType::Integer)
                .with_description("Maximum number of connections to the queue backend")
                .with_default("10")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(1000.0))
                .with_example("20"),
        );

        // Message max size
        fields.insert(
            "message_max_size".to_string(),
            FieldSchema::new("message_max_size", FieldType::Integer)
                .with_description("Maximum size of messages in bytes")
                .with_default("1048576")
                .with_constraint(FieldConstraint::MinValue(1024.0))
                .with_constraint(FieldConstraint::MaxValue(104857600.0))
                .with_example("2097152"),
        );

        // Consumer timeout
        fields.insert(
            "consumer_timeout_ms".to_string(),
            FieldSchema::new("consumer_timeout_ms", FieldType::Integer)
                .with_description("Timeout for consumer operations in milliseconds")
                .with_default("30000")
                .with_constraint(FieldConstraint::MinValue(1000.0))
                .with_constraint(FieldConstraint::MaxValue(300000.0))
                .with_example("60000"),
        );

        // Producer timeout
        fields.insert(
            "producer_timeout_ms".to_string(),
            FieldSchema::new("producer_timeout_ms", FieldType::Integer)
                .with_description("Timeout for producer operations in milliseconds")
                .with_default("5000")
                .with_constraint(FieldConstraint::MinValue(100.0))
                .with_constraint(FieldConstraint::MaxValue(60000.0))
                .with_example("10000"),
        );

        // Retry policy
        fields.insert(
            "retry_policy".to_string(),
            FieldSchema::new("retry_policy", FieldType::Object("RiRetryPolicy".to_string()))
                .with_description("Configuration for message retry behavior")
                .with_default("{}"),
        );

        // Dead letter config
        fields.insert(
            "dead_letter_config".to_string(),
            FieldSchema::new("dead_letter_config", FieldType::Optional(Box::new(
                FieldType::Object("RiDeadLetterConfig".to_string())
            )))
                .with_description("Configuration for dead letter queue functionality")
                .with_default("null"),
        );

        Self { fields }
    }
}

impl ConfigSchema for QueueModuleSchema {
    fn name(&self) -> &str {
        "QueueModule"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiQueueModule, providing message queue functionality"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "queue"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(queue_config) = config.get(self.root_key()) {
            if let Some(obj) = queue_config.as_mapping() {
                // Validate backend_type
                if let Some(backend) = obj.get(&serde_yaml::Value::String("backend_type".to_string())) {
                    if let Some(backend_str) = backend.as_str() {
                        let valid_backends = ["Memory", "RabbitMQ", "Kafka", "Redis"];
                        if !valid_backends.contains(&backend_str) {
                            errors.push(ValidationError {
                                field_path: "queue.backend_type".to_string(),
                                message: format!("Invalid backend_type '{}'. Must be one of: {}", 
                                    backend_str, valid_backends.join(", ")),
                                suggestion: Some("Use one of: Memory, RabbitMQ, Kafka, or Redis".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }

                        // Check connection_string is present for non-Memory backends
                        if backend_str != "Memory" {
                            if !obj.contains_key(&serde_yaml::Value::String("connection_string".to_string())) {
                                errors.push(ValidationError {
                                    field_path: "queue.connection_string".to_string(),
                                    message: format!("connection_string is required when backend_type is {}", backend_str),
                                    suggestion: Some(format!("Add: connection_string: \"{}\"", 
                                        if backend_str == "RabbitMQ" { "amqp://guest:guest@localhost:5672/" } 
                                        else if backend_str == "Kafka" { "localhost:9092" } 
                                        else { "redis://localhost:6379" })),
                                    severity: ValidationSeverity::Error,
                                });
                            }
                        }
                    }
                }

                // Validate retry_policy
                if let Some(retry) = obj.get(&serde_yaml::Value::String("retry_policy".to_string())) {
                    if let Some(retry_obj) = retry.as_mapping() {
                        if let Some(max_retries) = retry_obj.get(&serde_yaml::Value::String("max_retries".to_string())) {
                            if let Some(val) = max_retries.as_i64() {
                                if val < 0 {
                                    errors.push(ValidationError {
                                        field_path: "queue.retry_policy.max_retries".to_string(),
                                        message: "max_retries must be non-negative".to_string(),
                                        suggestion: Some("Set max_retries to 0 or higher".to_string()),
                                        severity: ValidationSeverity::Error,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiGateway.
///
/// This schema validates the configuration for the API gateway,
/// including network settings, CORS, and feature toggles.
#[derive(Debug, Clone)]
pub struct GatewaySchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for GatewaySchema {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewaySchema {
    /// Creates a new GatewaySchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `GatewaySchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Listen address
        fields.insert(
            "listen_address".to_string(),
            FieldSchema::new("listen_address", FieldType::String)
                .with_description("Address to listen on")
                .with_default("0.0.0.0")
                .with_constraint(FieldConstraint::Pattern(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$".to_string()))
                .with_example("0.0.0.0"),
        );

        // Listen port
        fields.insert(
            "listen_port".to_string(),
            FieldSchema::new("listen_port", FieldType::Integer)
                .with_description("Port to listen on")
                .with_default("8080")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(65535.0))
                .with_example("8080"),
        );

        // Max connections
        fields.insert(
            "max_connections".to_string(),
            FieldSchema::new("max_connections", FieldType::Integer)
                .with_description("Maximum number of concurrent connections")
                .with_default("10000")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(1000000.0))
                .with_example("10000"),
        );

        // Request timeout
        fields.insert(
            "request_timeout_seconds".to_string(),
            FieldSchema::new("request_timeout_seconds", FieldType::Integer)
                .with_description("Request timeout in seconds")
                .with_default("30")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(3600.0))
                .with_example("30"),
        );

        // Rate limiting
        fields.insert(
            "enable_rate_limiting".to_string(),
            FieldSchema::new("enable_rate_limiting", FieldType::Boolean)
                .with_description("Whether to enable rate limiting")
                .with_default("true")
                .with_example("true"),
        );

        // Circuit breaker
        fields.insert(
            "enable_circuit_breaker".to_string(),
            FieldSchema::new("enable_circuit_breaker", FieldType::Boolean)
                .with_description("Whether to enable circuit breaker")
                .with_default("true")
                .with_example("true"),
        );

        // Load balancing
        fields.insert(
            "enable_load_balancing".to_string(),
            FieldSchema::new("enable_load_balancing", FieldType::Boolean)
                .with_description("Whether to enable load balancing")
                .with_default("true")
                .with_example("true"),
        );

        // CORS enabled
        fields.insert(
            "cors_enabled".to_string(),
            FieldSchema::new("cors_enabled", FieldType::Boolean)
                .with_description("Whether to enable CORS")
                .with_default("true")
                .with_example("true"),
        );

        // CORS origins
        fields.insert(
            "cors_origins".to_string(),
            FieldSchema::new("cors_origins", FieldType::Array(Box::new(FieldType::String)))
                .with_description("Allowed CORS origins")
                .with_default("[\"*\"]")
                .with_example("[\"https://example.com\", \"https://api.example.com\"]"),
        );

        // CORS methods
        fields.insert(
            "cors_methods".to_string(),
            FieldSchema::new("cors_methods", FieldType::Array(Box::new(FieldType::String)))
                .with_description("Allowed CORS methods")
                .with_default("[\"GET\", \"POST\", \"PUT\", \"DELETE\", \"OPTIONS\"]")
                .with_example("[\"GET\", \"POST\"]"),
        );

        // CORS headers
        fields.insert(
            "cors_headers".to_string(),
            FieldSchema::new("cors_headers", FieldType::Array(Box::new(FieldType::String)))
                .with_description("Allowed CORS headers")
                .with_default("[\"Content-Type\", \"Authorization\"]")
                .with_example("[\"Content-Type\", \"Authorization\", \"X-Request-ID\"]"),
        );

        // Logging
        fields.insert(
            "enable_logging".to_string(),
            FieldSchema::new("enable_logging", FieldType::Boolean)
                .with_description("Whether to enable logging")
                .with_default("true")
                .with_example("true"),
        );

        // Log level
        fields.insert(
            "log_level".to_string(),
            FieldSchema::new("log_level", FieldType::Enum(vec![
                "trace".to_string(),
                "debug".to_string(),
                "info".to_string(),
                "warn".to_string(),
                "error".to_string(),
            ]))
                .with_description("Log level for gateway operations")
                .with_default("info")
                .with_example("info"),
        );

        Self { fields }
    }
}

impl ConfigSchema for GatewaySchema {
    fn name(&self) -> &str {
        "Gateway"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiGateway, providing API gateway functionality"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "gateway"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(gateway_config) = config.get(self.root_key()) {
            if let Some(obj) = gateway_config.as_mapping() {
                // Validate listen_port
                if let Some(port) = obj.get(&serde_yaml::Value::String("listen_port".to_string())) {
                    if let Some(port_val) = port.as_i64() {
                        if port_val < 1 || port_val > 65535 {
                            errors.push(ValidationError {
                                field_path: "gateway.listen_port".to_string(),
                                message: "listen_port must be between 1 and 65535".to_string(),
                                suggestion: Some("Use a valid port number between 1 and 65535".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                        if port_val < 1024 {
                            errors.push(ValidationError {
                                field_path: "gateway.listen_port".to_string(),
                                message: "Ports below 1024 require root privileges".to_string(),
                                suggestion: Some("Consider using a port above 1024 (e.g., 8080)".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }

                // Validate log_level
                if let Some(level) = obj.get(&serde_yaml::Value::String("log_level".to_string())) {
                    if let Some(level_str) = level.as_str() {
                        let valid_levels = ["trace", "debug", "info", "warn", "error"];
                        if !valid_levels.contains(&level_str) {
                            errors.push(ValidationError {
                                field_path: "gateway.log_level".to_string(),
                                message: format!("Invalid log_level '{}'. Must be one of: {}", 
                                    level_str, valid_levels.join(", ")),
                                suggestion: Some("Use one of: trace, debug, info, warn, error".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                    }
                }

                // Validate CORS configuration
                if let Some(cors_enabled) = obj.get(&serde_yaml::Value::String("cors_enabled".to_string())) {
                    if cors_enabled.as_bool().unwrap_or(false) {
                        if !obj.contains_key(&serde_yaml::Value::String("cors_origins".to_string())) {
                            errors.push(ValidationError {
                                field_path: "gateway.cors_origins".to_string(),
                                message: "cors_origins is recommended when CORS is enabled".to_string(),
                                suggestion: Some("Add: cors_origins: [\"https://your-domain.com\"]".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiAuthModule.
///
/// This schema validates the configuration for the authentication module,
/// including JWT settings, session management, and OAuth providers.
#[derive(Debug, Clone)]
pub struct AuthModuleSchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for AuthModuleSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthModuleSchema {
    /// Creates a new AuthModuleSchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `AuthModuleSchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Enabled flag
        fields.insert(
            "enabled".to_string(),
            FieldSchema::new("enabled", FieldType::Boolean)
                .with_description("Whether authentication is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // JWT secret
        fields.insert(
            "jwt_secret".to_string(),
            FieldSchema::new("jwt_secret", FieldType::String)
                .with_description("Secret key for JWT token generation and validation")
                .with_default("(loaded from Ri_JWT_SECRET env var)")
                .with_constraint(FieldConstraint::MinLength(16))
                .with_example("your-secure-secret-key-here"),
        );

        // JWT expiry
        fields.insert(
            "jwt_expiry_secs".to_string(),
            FieldSchema::new("jwt_expiry_secs", FieldType::Integer)
                .with_description("JWT token expiry time in seconds")
                .with_default("3600")
                .with_constraint(FieldConstraint::MinValue(60.0))
                .with_constraint(FieldConstraint::MaxValue(604800.0))
                .with_example("3600"),
        );

        // Session timeout
        fields.insert(
            "session_timeout_secs".to_string(),
            FieldSchema::new("session_timeout_secs", FieldType::Integer)
                .with_description("Session timeout in seconds")
                .with_default("86400")
                .with_constraint(FieldConstraint::MinValue(60.0))
                .with_constraint(FieldConstraint::MaxValue(2592000.0))
                .with_example("86400"),
        );

        // OAuth providers
        fields.insert(
            "oauth_providers".to_string(),
            FieldSchema::new("oauth_providers", FieldType::Array(Box::new(FieldType::String)))
                .with_description("List of OAuth providers to enable")
                .with_default("[]")
                .with_example("[\"google\", \"github\"]"),
        );

        // API keys enabled
        fields.insert(
            "enable_api_keys".to_string(),
            FieldSchema::new("enable_api_keys", FieldType::Boolean)
                .with_description("Whether API key authentication is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Session auth enabled
        fields.insert(
            "enable_session_auth".to_string(),
            FieldSchema::new("enable_session_auth", FieldType::Boolean)
                .with_description("Whether session authentication is enabled")
                .with_default("true")
                .with_example("true"),
        );

        Self { fields }
    }
}

impl ConfigSchema for AuthModuleSchema {
    fn name(&self) -> &str {
        "AuthModule"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiAuthModule, providing authentication and authorization"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "auth"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(auth_config) = config.get(self.root_key()) {
            if let Some(obj) = auth_config.as_mapping() {
                // Validate jwt_secret
                if let Some(secret) = obj.get(&serde_yaml::Value::String("jwt_secret".to_string())) {
                    if let Some(secret_str) = secret.as_str() {
                        if secret_str.len() < 16 {
                            errors.push(ValidationError {
                                field_path: "auth.jwt_secret".to_string(),
                                message: "jwt_secret should be at least 16 characters for security".to_string(),
                                suggestion: Some("Use a longer secret key (32+ characters recommended)".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                        if secret_str == "secret" || secret_str == "changeme" || secret_str == "password" {
                            errors.push(ValidationError {
                                field_path: "auth.jwt_secret".to_string(),
                                message: "jwt_secret appears to be a default or weak value".to_string(),
                                suggestion: Some("Use a cryptographically secure random secret".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                    }
                }

                // Validate jwt_expiry_secs
                if let Some(expiry) = obj.get(&serde_yaml::Value::String("jwt_expiry_secs".to_string())) {
                    if let Some(expiry_val) = expiry.as_i64() {
                        if expiry_val < 60 {
                            errors.push(ValidationError {
                                field_path: "auth.jwt_expiry_secs".to_string(),
                                message: "jwt_expiry_secs should be at least 60 seconds".to_string(),
                                suggestion: Some("Increase jwt_expiry_secs to at least 60".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                        if expiry_val > 86400 {
                            errors.push(ValidationError {
                                field_path: "auth.jwt_expiry_secs".to_string(),
                                message: "jwt_expiry_secs over 24 hours may pose security risks".to_string(),
                                suggestion: Some("Consider using a shorter expiry time".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }

                // Validate OAuth providers
                if let Some(providers) = obj.get(&serde_yaml::Value::String("oauth_providers".to_string())) {
                    if let Some(providers_arr) = providers.as_sequence() {
                        if !providers_arr.is_empty() {
                            // Check for required environment variables
                            for provider in providers_arr {
                                if let Some(provider_name) = provider.as_str() {
                                    let client_id_env = format!("Ri_OAUTH_{}_CLIENT_ID", provider_name.to_uppercase());
                                    let client_secret_env = format!("Ri_OAUTH_{}_CLIENT_SECRET", provider_name.to_uppercase());
                                    
                                    errors.push(ValidationError {
                                        field_path: format!("auth.oauth_providers.{}", provider_name),
                                        message: format!("OAuth provider '{}' requires environment variables: {} and {}", 
                                            provider_name, client_id_env, client_secret_env),
                                        suggestion: Some(format!("Set environment variables: export {}=your_client_id && export {}=your_client_secret", 
                                            client_id_env, client_secret_env)),
                                        severity: ValidationSeverity::Info,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiDeviceControlModule.
///
/// This schema validates the configuration for the device control module,
/// including device discovery, scheduling, and resource management.
#[derive(Debug, Clone)]
pub struct DeviceControlSchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for DeviceControlSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceControlSchema {
    /// Creates a new DeviceControlSchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `DeviceControlSchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Discovery enabled
        fields.insert(
            "discovery_enabled".to_string(),
            FieldSchema::new("discovery_enabled", FieldType::Boolean)
                .with_description("Whether device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Discovery interval
        fields.insert(
            "discovery_interval_secs".to_string(),
            FieldSchema::new("discovery_interval_secs", FieldType::Integer)
                .with_description("Interval between device discovery scans in seconds")
                .with_default("30")
                .with_constraint(FieldConstraint::MinValue(5.0))
                .with_constraint(FieldConstraint::MaxValue(3600.0))
                .with_example("30"),
        );

        // Auto scheduling
        fields.insert(
            "auto_scheduling_enabled".to_string(),
            FieldSchema::new("auto_scheduling_enabled", FieldType::Boolean)
                .with_description("Whether automatic resource scheduling is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Max concurrent tasks
        fields.insert(
            "max_concurrent_tasks".to_string(),
            FieldSchema::new("max_concurrent_tasks", FieldType::Integer)
                .with_description("Maximum number of concurrent tasks")
                .with_default("100")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(10000.0))
                .with_example("100"),
        );

        // Resource allocation timeout
        fields.insert(
            "resource_allocation_timeout_secs".to_string(),
            FieldSchema::new("resource_allocation_timeout_secs", FieldType::Integer)
                .with_description("Timeout for resource allocation in seconds")
                .with_default("60")
                .with_constraint(FieldConstraint::MinValue(10.0))
                .with_constraint(FieldConstraint::MaxValue(600.0))
                .with_example("60"),
        );

        // CPU discovery
        fields.insert(
            "enable_cpu_discovery".to_string(),
            FieldSchema::new("enable_cpu_discovery", FieldType::Boolean)
                .with_description("Whether CPU device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // GPU discovery
        fields.insert(
            "enable_gpu_discovery".to_string(),
            FieldSchema::new("enable_gpu_discovery", FieldType::Boolean)
                .with_description("Whether GPU device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Memory discovery
        fields.insert(
            "enable_memory_discovery".to_string(),
            FieldSchema::new("enable_memory_discovery", FieldType::Boolean)
                .with_description("Whether memory device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Storage discovery
        fields.insert(
            "enable_storage_discovery".to_string(),
            FieldSchema::new("enable_storage_discovery", FieldType::Boolean)
                .with_description("Whether storage device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Network discovery
        fields.insert(
            "enable_network_discovery".to_string(),
            FieldSchema::new("enable_network_discovery", FieldType::Boolean)
                .with_description("Whether network device discovery is enabled")
                .with_default("true")
                .with_example("true"),
        );

        Self { fields }
    }
}

impl ConfigSchema for DeviceControlSchema {
    fn name(&self) -> &str {
        "DeviceControl"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiDeviceControlModule, providing device management"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "device"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(device_config) = config.get(self.root_key()) {
            if let Some(obj) = device_config.as_mapping() {
                // Check if any discovery is enabled
                let cpu_enabled = obj.get(&serde_yaml::Value::String("enable_cpu_discovery".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);
                let gpu_enabled = obj.get(&serde_yaml::Value::String("enable_gpu_discovery".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);
                let memory_enabled = obj.get(&serde_yaml::Value::String("enable_memory_discovery".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);
                let storage_enabled = obj.get(&serde_yaml::Value::String("enable_storage_discovery".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);
                let network_enabled = obj.get(&serde_yaml::Value::String("enable_network_discovery".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);

                let discovery_enabled = obj.get(&serde_yaml::Value::String("discovery_enabled".to_string()))
                    .and_then(|v| v.as_bool()).unwrap_or(true);

                if discovery_enabled && !cpu_enabled && !gpu_enabled && !memory_enabled 
                    && !storage_enabled && !network_enabled {
                    errors.push(ValidationError {
                        field_path: "device".to_string(),
                        message: "discovery_enabled is true but no device type discovery is enabled".to_string(),
                        suggestion: Some("Enable at least one device type discovery (cpu, gpu, memory, storage, or network)".to_string()),
                        severity: ValidationSeverity::Warning,
                    });
                }

                // Validate discovery_interval_secs
                if let Some(interval) = obj.get(&serde_yaml::Value::String("discovery_interval_secs".to_string())) {
                    if let Some(interval_val) = interval.as_i64() {
                        if interval_val < 5 {
                            errors.push(ValidationError {
                                field_path: "device.discovery_interval_secs".to_string(),
                                message: "discovery_interval_secs should be at least 5 seconds to avoid excessive CPU usage".to_string(),
                                suggestion: Some("Increase discovery_interval_secs to at least 5".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }

                // Validate max_concurrent_tasks
                if let Some(tasks) = obj.get(&serde_yaml::Value::String("max_concurrent_tasks".to_string())) {
                    if let Some(tasks_val) = tasks.as_i64() {
                        if tasks_val > 1000 {
                            errors.push(ValidationError {
                                field_path: "device.max_concurrent_tasks".to_string(),
                                message: "max_concurrent_tasks over 1000 may cause resource exhaustion".to_string(),
                                suggestion: Some("Consider using a lower value based on your system resources".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }
            }
        }

        errors
    }
}

/// Configuration schema for RiObservabilityModule.
///
/// This schema validates the configuration for the observability module,
/// including tracing and metrics collection settings.
#[derive(Debug, Clone)]
pub struct ObservabilitySchema {
    /// Field schemas for this configuration
    fields: HashMap<String, FieldSchema>,
}

impl Default for ObservabilitySchema {
    fn default() -> Self {
        Self::new()
    }
}

impl ObservabilitySchema {
    /// Creates a new ObservabilitySchema with all field definitions.
    ///
    /// # Returns
    ///
    /// A new `ObservabilitySchema` instance with predefined fields
    pub fn new() -> Self {
        let mut fields = HashMap::new();

        // Tracing enabled
        fields.insert(
            "tracing_enabled".to_string(),
            FieldSchema::new("tracing_enabled", FieldType::Boolean)
                .with_description("Whether distributed tracing is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Metrics enabled
        fields.insert(
            "metrics_enabled".to_string(),
            FieldSchema::new("metrics_enabled", FieldType::Boolean)
                .with_description("Whether metrics collection is enabled")
                .with_default("true")
                .with_example("true"),
        );

        // Tracing sampling rate
        fields.insert(
            "tracing_sampling_rate".to_string(),
            FieldSchema::new("tracing_sampling_rate", FieldType::Float)
                .with_description("Sampling rate for distributed tracing (0.0 to 1.0)")
                .with_default("0.1")
                .with_constraint(FieldConstraint::MinValue(0.0))
                .with_constraint(FieldConstraint::MaxValue(1.0))
                .with_example("0.5"),
        );

        // Tracing sampling strategy
        fields.insert(
            "tracing_sampling_strategy".to_string(),
            FieldSchema::new("tracing_sampling_strategy", FieldType::Enum(vec![
                "rate".to_string(),
                "probability".to_string(),
                "always".to_string(),
                "never".to_string(),
            ]))
                .with_description("Sampling strategy for distributed tracing")
                .with_default("rate")
                .with_example("rate"),
        );

        // Metrics window size
        fields.insert(
            "metrics_window_size_secs".to_string(),
            FieldSchema::new("metrics_window_size_secs", FieldType::Integer)
                .with_description("Window size for metrics aggregation in seconds")
                .with_default("300")
                .with_constraint(FieldConstraint::MinValue(10.0))
                .with_constraint(FieldConstraint::MaxValue(3600.0))
                .with_example("300"),
        );

        // Metrics bucket size
        fields.insert(
            "metrics_bucket_size_secs".to_string(),
            FieldSchema::new("metrics_bucket_size_secs", FieldType::Integer)
                .with_description("Bucket size for metrics aggregation in seconds")
                .with_default("10")
                .with_constraint(FieldConstraint::MinValue(1.0))
                .with_constraint(FieldConstraint::MaxValue(60.0))
                .with_example("10"),
        );

        Self { fields }
    }
}

impl ConfigSchema for ObservabilitySchema {
    fn name(&self) -> &str {
        "Observability"
    }

    fn description(&self) -> &str {
        "Configuration schema for RiObservabilityModule, providing tracing and metrics"
    }

    fn fields(&self) -> &HashMap<String, FieldSchema> {
        &self.fields
    }

    fn root_key(&self) -> &str {
        "observability"
    }

    fn validate(&self, config: &serde_yaml::Value) -> Vec<ValidationError> {
        let mut errors = Vec::new();

        if let Some(obs_config) = config.get(self.root_key()) {
            if let Some(obj) = obs_config.as_mapping() {
                // Validate tracing_sampling_rate
                if let Some(rate) = obj.get(&serde_yaml::Value::String("tracing_sampling_rate".to_string())) {
                    if let Some(rate_val) = rate.as_f64() {
                        if rate_val < 0.0 || rate_val > 1.0 {
                            errors.push(ValidationError {
                                field_path: "observability.tracing_sampling_rate".to_string(),
                                message: "tracing_sampling_rate must be between 0.0 and 1.0".to_string(),
                                suggestion: Some("Use a value between 0.0 (0%) and 1.0 (100%)".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                        if rate_val > 0.5 {
                            errors.push(ValidationError {
                                field_path: "observability.tracing_sampling_rate".to_string(),
                                message: "tracing_sampling_rate over 0.5 may impact performance".to_string(),
                                suggestion: Some("Consider using a lower sampling rate in production".to_string()),
                                severity: ValidationSeverity::Warning,
                            });
                        }
                    }
                }

                // Validate tracing_sampling_strategy
                if let Some(strategy) = obj.get(&serde_yaml::Value::String("tracing_sampling_strategy".to_string())) {
                    if let Some(strategy_str) = strategy.as_str() {
                        let valid_strategies = ["rate", "probability", "always", "never"];
                        if !valid_strategies.contains(&strategy_str) {
                            errors.push(ValidationError {
                                field_path: "observability.tracing_sampling_strategy".to_string(),
                                message: format!("Invalid tracing_sampling_strategy '{}'. Must be one of: {}", 
                                    strategy_str, valid_strategies.join(", ")),
                                suggestion: Some("Use one of: rate, probability, always, never".to_string()),
                                severity: ValidationSeverity::Error,
                            });
                        }
                    }
                }

                // Validate metrics_window_size_secs vs metrics_bucket_size_secs
                let window_size = obj.get(&serde_yaml::Value::String("metrics_window_size_secs".to_string()))
                    .and_then(|v| v.as_i64()).unwrap_or(300);
                let bucket_size = obj.get(&serde_yaml::Value::String("metrics_bucket_size_secs".to_string()))
                    .and_then(|v| v.as_i64()).unwrap_or(10);

                if window_size < bucket_size {
                    errors.push(ValidationError {
                        field_path: "observability".to_string(),
                        message: "metrics_window_size_secs should be greater than or equal to metrics_bucket_size_secs".to_string(),
                        suggestion: Some(format!("Increase metrics_window_size_secs to at least {} or decrease metrics_bucket_size_secs", bucket_size)),
                        severity: ValidationSeverity::Error,
                    });
                }

                if window_size % bucket_size != 0 {
                    errors.push(ValidationError {
                        field_path: "observability".to_string(),
                        message: "metrics_window_size_secs should be a multiple of metrics_bucket_size_secs for optimal performance".to_string(),
                        suggestion: Some("Adjust values so that window_size is evenly divisible by bucket_size".to_string()),
                        severity: ValidationSeverity::Warning,
                    });
                }
            }
        }

        errors
    }
}

/// Returns all available configuration schemas.
///
/// This function creates and returns a vector of all configuration schemas
/// available in the Ri framework, allowing for comprehensive validation
/// of configuration files.
///
/// # Returns
///
/// A vector of boxed configuration schemas
pub fn all_schemas() -> Vec<Box<dyn ConfigSchema>> {
    vec![
        Box::new(AppBuilderSchema::new()),
        Box::new(CacheModuleSchema::new()),
        Box::new(QueueModuleSchema::new()),
        Box::new(GatewaySchema::new()),
        Box::new(AuthModuleSchema::new()),
        Box::new(DeviceControlSchema::new()),
        Box::new(ObservabilitySchema::new()),
    ]
}
