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

//! Code Generation Engine
//!
//! This module provides the core code generation engine for the Ri CLI.
//! It handles generating modules, middleware, configuration structures,
//! and formatting code using rustfmt.
//!
//! # Features
//!
//! - **Module Generation**: Generate boilerplate code for different module types
//! - **Middleware Generation**: Create middleware components with standard patterns
//! - **Config Struct Generation**: Generate Rust structs from YAML configuration files
//! - **Code Formatting**: Automatic code formatting using rustfmt
//! - **Dependency Addition**: Add dependencies to Cargo.toml files
//!
//! # Architecture
//!
//! The engine is built around the `CodeGenerator` struct which provides:
//!
//! - Template-based code generation
//! - Integration with the template system
//! - Code formatting capabilities
//! - Cargo.toml manipulation
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::generator::engine::{CodeGenerator, ModuleType};
//!
//! // Create a new generator instance
//! let generator = CodeGenerator::new()?;
//!
//! // Generate a cache module
//! let code = generator.generate_module(ModuleType::Cache, "my_cache")?;
//!
//! // Generate middleware
//! let middleware = generator.generate_middleware("auth_middleware")?;
//!
//! // Generate config struct from YAML file
//! let config = generator.generate_config_struct("config/app.yaml")?;
//!
//! // Format the generated code
//! let formatted = generator.format_code(&code)?;
//!
//! // Add dependency to Cargo.toml
//! generator.add_dependency("Cargo.toml", "serde = { version = \"1.0\", features = [\"derive\"] }")?;
//! ```

use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

// =============================================================================
// Module Type Enumeration
// =============================================================================

/// Enumeration of supported module types for code generation
///
/// Each variant represents a different type of module that can be generated
/// by the code generator. Each module type has its own template and
/// default configuration.
///
/// # Variants
///
/// - `Cache`: Distributed caching module with Redis/Memcached support
/// - `Queue`: Message queue module for async processing
/// - `Gateway`: API gateway module with routing and rate limiting
/// - `Auth`: Authentication and authorization module
/// - `Device`: IoT device management module
/// - `Observability`: Logging, metrics, and tracing module
///
/// # Example
///
/// ```rust,ignore
/// use ric::generator::ModuleType;
///
/// let module_type = ModuleType::Cache;
/// let name = "my_cache";
///
/// // Generate the module
/// let generator = CodeGenerator::new()?;
/// let code = generator.generate_module(module_type, name)?;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleType {
    /// Distributed caching module
    ///
    /// Provides caching capabilities with support for:
    /// - Redis integration
    /// - Memcached integration
    /// - In-memory caching
    /// - Cache invalidation strategies
    Cache,

    /// Message queue module
    ///
    /// Provides message queue capabilities with support for:
    /// - RabbitMQ integration
    /// - Kafka integration
    /// - Redis-based queues
    /// - Job scheduling and processing
    Queue,

    /// API gateway module
    ///
    /// Provides gateway capabilities with support for:
    /// - Request routing
    /// - Rate limiting
    /// - Load balancing
    /// - Request/response transformation
    Gateway,

    /// Authentication and authorization module
    ///
    /// Provides security capabilities with support for:
    /// - JWT authentication
    /// - OAuth2 integration
    /// - Role-based access control
    /// - Session management
    Auth,

    /// IoT device management module
    ///
    /// Provides device management capabilities with support for:
    /// - Device registration and discovery
    /// - MQTT protocol support
    /// - Device state management
    /// - Telemetry collection
    Device,

    /// Observability module
    ///
    /// Provides monitoring capabilities with support for:
    /// - Structured logging
    /// - Metrics collection (Prometheus)
    /// - Distributed tracing (Jaeger)
    /// - Health checks
    Observability,
}

impl ModuleType {
    /// Get the string identifier for the module type
    ///
    /// Returns a lowercase string identifier used in template selection
    /// and file naming conventions.
    ///
    /// # Returns
    ///
    /// A string slice representing the module type identifier.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let module_type = ModuleType::Cache;
    /// assert_eq!(module_type.as_str(), "cache");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            ModuleType::Cache => "cache",
            ModuleType::Queue => "queue",
            ModuleType::Gateway => "gateway",
            ModuleType::Auth => "auth",
            ModuleType::Device => "device",
            ModuleType::Observability => "observability",
        }
    }

    /// Get the display name for the module type
    ///
    /// Returns a human-readable name for display in CLI output
    /// and documentation.
    ///
    /// # Returns
    ///
    /// A string slice representing the module type display name.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let module_type = ModuleType::Cache;
    /// assert_eq!(module_type.display_name(), "Cache Module");
    /// ```
    pub fn display_name(&self) -> &'static str {
        match self {
            ModuleType::Cache => "Cache Module",
            ModuleType::Queue => "Queue Module",
            ModuleType::Gateway => "Gateway Module",
            ModuleType::Auth => "Auth Module",
            ModuleType::Device => "Device Module",
            ModuleType::Observability => "Observability Module",
        }
    }
}

// =============================================================================
// Code Generator
// =============================================================================

/// Code generation engine
///
/// The main engine for generating code from templates. Provides functionality
/// for generating modules, middleware, configuration structures, and formatting
/// code using rustfmt.
///
/// # Features
///
/// - **Template-based Generation**: Uses predefined templates for consistency
/// - **Module Generation**: Generate complete module structures
/// - **Middleware Generation**: Create middleware components
/// - **Config Generation**: Generate Rust structs from YAML configs
/// - **Code Formatting**: Format generated code with rustfmt
/// - **Dependency Management**: Add dependencies to Cargo.toml
///
/// # Example
///
/// ```rust,ignore
/// // Create a new generator
/// let generator = CodeGenerator::new()?;
///
/// // Generate a cache module
/// let code = generator.generate_module(ModuleType::Cache, "my_cache")?;
/// println!("Generated code:\n{}", code);
///
/// // Generate middleware
/// let middleware = generator.generate_middleware("logging_middleware")?;
///
/// // Format code
/// let formatted = generator.format_code(&code)?;
/// ```
pub struct CodeGenerator {
    /// Base directory for generated output
    output_dir: std::path::PathBuf,
}

impl CodeGenerator {
    /// Create a new code generator instance
    ///
    /// Initializes the code generator with the current directory as the
    /// default output directory.
    ///
    /// # Returns
    ///
    /// Returns `Ok(CodeGenerator)` on success.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    /// ```
    pub fn new() -> Result<Self> {
        Ok(Self {
            output_dir: std::env::current_dir()
                .context("Failed to get current directory")?,
        })
    }

    /// Create a code generator with a custom output directory
    ///
    /// Initializes the code generator with a specified output directory
    /// for generated files.
    ///
    /// # Arguments
    ///
    /// * `output_dir` - Path to the output directory
    ///
    /// # Returns
    ///
    /// Returns `Ok(CodeGenerator)` on success.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::with_output_dir("./generated")?;
    /// ```
    pub fn with_output_dir<P: AsRef<Path>>(output_dir: P) -> Result<Self> {
        Ok(Self {
            output_dir: output_dir.as_ref().to_path_buf(),
        })
    }

    /// Generate module code from a template
    ///
    /// Generates complete module code based on the specified module type.
    /// The generated code includes the module structure, trait implementations,
    /// and basic functionality.
    ///
    /// # Arguments
    ///
    /// * `module_type` - Type of module to generate
    /// * `name` - Name of the module (used for struct and file naming)
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the generated module code.
    /// Returns an error if template retrieval or code generation fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    /// let code = generator.generate_module(ModuleType::Cache, "my_cache")?;
    /// println!("Generated cache module:\n{}", code);
    /// ```
    pub fn generate_module(&self, module_type: ModuleType, name: &str) -> Result<String> {
        // Validate module name
        Self::validate_module_name(name)?;

        // Get the appropriate template
        let template = super::templates::get_module_template(module_type);

        // Replace placeholders in the template
        let code = template
            .replace("{{MODULE_NAME}}", name)
            .replace("{{MODULE_NAME_PASCAL}}", &Self::to_pascal_case(name))
            .replace("{{MODULE_TYPE}}", module_type.as_str());

        Ok(code)
    }

    /// Generate middleware code from a template
    ///
    /// Generates middleware component code with standard patterns for
    /// request/response handling, error management, and async processing.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the middleware (used for struct and file naming)
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the generated middleware code.
    /// Returns an error if template retrieval or code generation fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    /// let code = generator.generate_middleware("auth_middleware")?;
    /// println!("Generated middleware:\n{}", code);
    /// ```
    pub fn generate_middleware(&self, name: &str) -> Result<String> {
        // Validate middleware name
        Self::validate_module_name(name)?;

        // Get the middleware template
        let template = super::templates::get_middleware_template();

        // Replace placeholders in the template
        let code = template
            .replace("{{MIDDLEWARE_NAME}}", name)
            .replace("{{MIDDLEWARE_NAME_PASCAL}}", &Self::to_pascal_case(name));

        Ok(code)
    }

    /// Generate config struct from a YAML configuration file
    ///
    /// Parses a YAML configuration file and generates a corresponding
    /// Rust struct with serde derive macros for serialization support.
    ///
    /// # Arguments
    ///
    /// * `config_file` - Path to the YAML configuration file
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the generated struct code.
    /// Returns an error if the file cannot be read or parsed.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    /// let code = generator.generate_config_struct("config/app.yaml")?;
    /// println!("Generated config struct:\n{}", code);
    /// ```
    pub fn generate_config_struct<P: AsRef<Path>>(&self, config_file: P) -> Result<String> {
        let config_path = config_file.as_ref();

        // Read the YAML file
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;

        // Parse YAML
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .with_context(|| format!("Failed to parse YAML file: {}", config_path.display()))?;

        // Get the template
        let template = super::templates::get_config_template();

        // Generate struct fields from YAML
        let fields = Self::yaml_to_struct_fields(&yaml_value);

        // Replace placeholders
        let config_name = config_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Config");

        let code = template
            .replace("{{CONFIG_NAME}}", &Self::to_pascal_case(config_name))
            .replace("{{CONFIG_FIELDS}}", &fields);

        Ok(code)
    }

    /// Format code using rustfmt
    ///
    /// Takes generated code and formats it using the rustfmt tool.
    /// This ensures consistent code style across all generated files.
    ///
    /// # Arguments
    ///
    /// * `code` - The code string to format
    ///
    /// # Returns
    ///
    /// Returns `Ok(String)` containing the formatted code.
    /// Returns an error if rustfmt is not available or formatting fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    /// let unformatted = "fn main(){println!(\"Hello\");}";
    /// let formatted = generator.format_code(unformatted)?;
    /// println!("Formatted code:\n{}", formatted);
    /// ```
    pub fn format_code(&self, code: &str) -> Result<String> {
        // Create a temporary file for rustfmt
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("ric_generated_code.rs");

        // Write code to temp file
        fs::write(&temp_file, code)
            .context("Failed to write temporary file for formatting")?;

        // Run rustfmt
        let output = Command::new("rustfmt")
            .arg(&temp_file)
            .output()
            .context("Failed to execute rustfmt. Make sure rustfmt is installed.")?;

        // Check if formatting succeeded
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("rustfmt failed: {}", stderr));
        }

        // Read formatted code
        let formatted = fs::read_to_string(&temp_file)
            .context("Failed to read formatted code")?;

        // Clean up temp file
        let _ = fs::remove_file(&temp_file);

        Ok(formatted)
    }

    /// Add a dependency to a Cargo.toml file
    ///
    /// Parses the Cargo.toml file and adds a new dependency entry.
    /// Preserves existing content and formatting where possible.
    ///
    /// # Arguments
    ///
    /// * `cargo_toml` - Path to the Cargo.toml file
    /// * `dependency` - Dependency string in Cargo.toml format
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    /// Returns an error if the file cannot be read or modified.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let generator = CodeGenerator::new()?;
    ///
    /// // Add a simple dependency
    /// generator.add_dependency("Cargo.toml", "serde = \"1.0\"")?;
    ///
    /// // Add a dependency with features
    /// generator.add_dependency("Cargo.toml", "serde = { version = \"1.0\", features = [\"derive\"] }")?;
    /// ```
    pub fn add_dependency<P: AsRef<Path>>(&self, cargo_toml: P, dependency: &str) -> Result<()> {
        let cargo_path = cargo_toml.as_ref();

        // Read existing Cargo.toml
        let content = fs::read_to_string(cargo_path)
            .with_context(|| format!("Failed to read Cargo.toml: {}", cargo_path.display()))?;

        // Find the [dependencies] section
        let mut lines: Vec<String> = content.lines().map(String::from).collect();
        let mut deps_index = None;
        let mut has_deps_section = false;

        for (i, line) in lines.iter().enumerate() {
            if line.trim() == "[dependencies]" {
                deps_index = Some(i);
                has_deps_section = true;
                break;
            }
        }

        // Add dependency
        if has_deps_section {
            // Insert after [dependencies] header
            if let Some(index) = deps_index {
                lines.insert(index + 1, dependency.to_string());
            }
        } else {
            // Add [dependencies] section
            lines.push(String::new());
            lines.push("[dependencies]".to_string());
            lines.push(dependency.to_string());
        }

        // Write back to file
        let new_content = lines.join("\n");
        fs::write(cargo_path, new_content)
            .with_context(|| format!("Failed to write Cargo.toml: {}", cargo_path.display()))?;

        Ok(())
    }

    // =========================================================================
    // Helper Methods
    // =========================================================================

    /// Validate a module name
    ///
    /// Ensures the module name follows Rust naming conventions:
    /// - Contains only alphanumeric characters and underscores
    /// - Does not start with a number
    /// - Is not a Rust keyword
    ///
    /// # Arguments
    ///
    /// * `name` - The module name to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the name is valid.
    /// Returns an error if the name is invalid.
    fn validate_module_name(name: &str) -> Result<()> {
        if name.is_empty() {
            return Err(anyhow::anyhow!("Module name cannot be empty"));
        }

        // Check first character
        let first_char = name.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            return Err(anyhow::anyhow!(
                "Module name cannot start with a digit: {}",
                name
            ));
        }

        // Check all characters
        for c in name.chars() {
            if !c.is_ascii_alphanumeric() && c != '_' {
                return Err(anyhow::anyhow!(
                    "Module name contains invalid character '{}'. Only alphanumeric and underscore are allowed.",
                    c
                ));
            }
        }

        // Check for Rust keywords
        let keywords = [
            "as", "break", "const", "continue", "crate", "else", "enum", "extern",
            "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
            "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
            "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
            "while", "async", "await", "dyn",
        ];

        if keywords.contains(&name) {
            return Err(anyhow::anyhow!(
                "Module name '{}' is a Rust keyword. Please choose a different name.",
                name
            ));
        }

        Ok(())
    }

    /// Convert a string to PascalCase
    ///
    /// Transforms a snake_case or kebab-case string to PascalCase.
    ///
    /// # Arguments
    ///
    /// * `s` - The input string
    ///
    /// # Returns
    ///
    /// Returns the PascalCase version of the string.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// assert_eq!(CodeGenerator::to_pascal_case("my_module"), "MyModule");
    /// assert_eq!(CodeGenerator::to_pascal_case("hello-world"), "HelloWorld");
    /// ```
    fn to_pascal_case(s: &str) -> String {
        s.split(|c| c == '_' || c == '-')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect()
    }

    /// Convert YAML value to struct field definitions
    ///
    /// Recursively converts a YAML value to Rust struct field definitions.
    /// Handles nested structures, arrays, and basic types.
    ///
    /// # Arguments
    ///
    /// * `yaml` - The YAML value to convert
    ///
    /// # Returns
    ///
    /// Returns a string containing the struct field definitions.
    fn yaml_to_struct_fields(yaml: &serde_yaml::Value) -> String {
        let mut fields = String::new();

        if let serde_yaml::Value::Mapping(map) = yaml {
            for (key, value) in map {
                if let serde_yaml::Value::String(key_str) = key {
                    let field_name = key_str;
                    let field_type = Self::yaml_type_to_rust_type(value);

                    fields.push_str(&format!("    pub {}: {},\n", field_name, field_type));
                }
            }
        }

        fields
    }

    /// Convert YAML type to Rust type
    ///
    /// Determines the appropriate Rust type for a YAML value.
    ///
    /// # Arguments
    ///
    /// * `value` - The YAML value
    ///
    /// # Returns
    ///
    /// Returns a string representing the Rust type.
    fn yaml_type_to_rust_type(value: &serde_yaml::Value) -> String {
        match value {
            serde_yaml::Value::Null => "Option<()>".to_string(),
            serde_yaml::Value::Bool(_) => "bool".to_string(),
            serde_yaml::Value::Number(n) => {
                if n.is_i64() {
                    "i64".to_string()
                } else if n.is_u64() {
                    "u64".to_string()
                } else {
                    "f64".to_string()
                }
            }
            serde_yaml::Value::String(_) => "String".to_string(),
            serde_yaml::Value::Sequence(seq) => {
                if seq.is_empty() {
                    "Vec<String>".to_string()
                } else {
                    let inner_type = Self::yaml_type_to_rust_type(&seq[0]);
                    format!("Vec<{}>", inner_type)
                }
            }
            serde_yaml::Value::Mapping(_) => "serde_json::Value".to_string(),
        }
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create default CodeGenerator")
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_type_as_str() {
        assert_eq!(ModuleType::Cache.as_str(), "cache");
        assert_eq!(ModuleType::Queue.as_str(), "queue");
        assert_eq!(ModuleType::Gateway.as_str(), "gateway");
        assert_eq!(ModuleType::Auth.as_str(), "auth");
        assert_eq!(ModuleType::Device.as_str(), "device");
        assert_eq!(ModuleType::Observability.as_str(), "observability");
    }

    #[test]
    fn test_module_type_display_name() {
        assert_eq!(ModuleType::Cache.display_name(), "Cache Module");
        assert_eq!(ModuleType::Queue.display_name(), "Queue Module");
        assert_eq!(ModuleType::Gateway.display_name(), "Gateway Module");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(CodeGenerator::to_pascal_case("my_module"), "MyModule");
        assert_eq!(CodeGenerator::to_pascal_case("hello-world"), "HelloWorld");
        assert_eq!(CodeGenerator::to_pascal_case("test"), "Test");
        assert_eq!(CodeGenerator::to_pascal_case("my_test_module"), "MyTestModule");
    }

    #[test]
    fn test_validate_module_name_valid() {
        assert!(CodeGenerator::validate_module_name("my_module").is_ok());
        assert!(CodeGenerator::validate_module_name("cache").is_ok());
        assert!(CodeGenerator::validate_module_name("my_module_2").is_ok());
    }

    #[test]
    fn test_validate_module_name_invalid() {
        assert!(CodeGenerator::validate_module_name("").is_err());
        assert!(CodeGenerator::validate_module_name("123module").is_err());
        assert!(CodeGenerator::validate_module_name("my-module").is_err());
        assert!(CodeGenerator::validate_module_name("fn").is_err());
    }

    #[test]
    fn test_code_generator_new() {
        let generator = CodeGenerator::new();
        assert!(generator.is_ok());
    }

    #[test]
    fn test_generate_module() {
        let generator = CodeGenerator::new().unwrap();
        let result = generator.generate_module(ModuleType::Cache, "my_cache");
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("my_cache"));
    }

    #[test]
    fn test_generate_middleware() {
        let generator = CodeGenerator::new().unwrap();
        let result = generator.generate_middleware("auth_middleware");
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("auth_middleware"));
    }
}
