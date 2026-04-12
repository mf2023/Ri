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

//! Minimal Application Template
//!
//! This module defines the minimal application template for creating simple
//! Ri applications with minimal setup and dependencies. It provides a clean
//! starting point for small projects or learning Ri framework basics.
//!
//! # Template Features
//!
//! The minimal template includes:
//!
//! - **Basic Application Structure**: Simple and clean project layout
//! - **Minimal Dependencies**: Only essential dependencies included
//! - **Simple Configuration**: Basic configuration file
//! - **Quick Start**: Fast project setup for simple use cases
//! - **Learning Friendly**: Easy to understand for beginners
//! - **Lightweight**: Small footprint and fast compilation
//!
//! # Generated Files
//!
//! The template generates the following minimal project structure:
//!
//! ```text
//! my-app/
//! ├── Cargo.toml              # Package manifest with minimal dependencies
//! ├── src/
//! │   └── main.rs            # Application entry point
//! └── config/
//!     └── config.yaml        # Basic configuration
//! ```
//!
//! # Template Variables
//!
//! The template supports the following variables:
//!
//! | Variable | Type | Required | Default | Description |
//! |----------|------|----------|---------|-------------|
//! | `project_name` | string | Yes | - | Project name |
//! | `version` | string | No | "0.1.0" | Project version |
//! | `author` | string | No | "Dunimd Team" | Project author |
//! | `description` | string | No | "A Ri application" | Project description |
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::minimal;
//!
//! // Get template information
//! let info = minimal::get_template_info();
//! println!("Template: {}", info.display_name);
//!
//! // List template features
//! for feature in &info.features {
//!     println!("- {}", feature);
//! }
//!
//! // Get template variables
//! for var in &info.variables {
//!     println!("{}: {} (default: {})", var.name, var.description, var.default_value);
//! }
//! ```
//!
//! # Example Generated Code
//!
//! The generated `main.rs` will look like:
//!
//! ```rust,ignore
//! use ri::core::RiAppBuilder;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = RiAppBuilder::new("my-app")
//!         .build()
//!         .await?;
//!
//!     app.run().await
//! }
//! ```
//!
//! # When to Use This Template
//!
//! The minimal template is ideal for:
//!
//! - **Learning Ri**: Understanding Ri framework basics
//! - **Prototyping**: Quick proof-of-concept applications
//! - **Simple Services**: Small services with minimal requirements
//! - **Custom Templates**: Starting point for custom templates
//! - **Testing**: Simple test applications
//!
//! # When to Use Other Templates
//!
//! Consider using other templates when you need:
//!
//! - **Web Application**: Use `web` template for HTTP servers
//! - **API Service**: Use `api` template for REST APIs
//! - **Background Jobs**: Use `worker` template for task processing
//! - **Microservice**: Use `microservice` template for gRPC services

use super::engine::{TemplateInfo, TemplateVariable, TemplateFile};

/// Get minimal template metadata
///
/// Returns the complete metadata for the minimal application template,
/// including features, variables, and file definitions.
///
/// # Returns
///
/// Returns a `TemplateInfo` struct containing all template metadata.
///
/// # Example
///
/// ```rust,ignore
/// let info = get_template_info();
/// assert_eq!(info.name, "minimal");
/// assert!(!info.features.is_empty());
/// ```
pub fn get_template_info() -> TemplateInfo {
    TemplateInfo {
        name: "minimal".to_string(),
        display_name: "Minimal Application".to_string(),
        description: "Minimal Ri application with basic structure and minimal dependencies".to_string(),
        author: "Dunimd Team".to_string(),
        version: "1.0.0".to_string(),
        features: vec![
            "Basic application structure".to_string(),
            "Minimal dependencies".to_string(),
            "Simple configuration".to_string(),
            "Quick start setup".to_string(),
            "Learning friendly".to_string(),
            "Lightweight footprint".to_string(),
        ],
        variables: vec![
            TemplateVariable {
                name: "project_name".to_string(),
                description: "Project name used in package manifest and application".to_string(),
                default_value: "my-app".to_string(),
                required: true,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "version".to_string(),
                description: "Project version following semantic versioning".to_string(),
                default_value: "0.1.0".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "author".to_string(),
                description: "Project author or maintainer".to_string(),
                default_value: "Dunimd Team".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "description".to_string(),
                description: "Brief description of the project".to_string(),
                default_value: "A Ri application".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
        ],
        files: vec![
            TemplateFile {
                source: "Cargo.toml.tera".to_string(),
                destination: "Cargo.toml".to_string(),
            },
            TemplateFile {
                source: "src/main.rs.tera".to_string(),
                destination: "src/main.rs".to_string(),
            },
            TemplateFile {
                source: "config/config.yaml.tera".to_string(),
                destination: "config/config.yaml".to_string(),
            },
        ],
    }
}

/// Get default template variables
///
/// Returns a map of variable names to their default values.
/// This is useful for pre-populating forms or providing suggestions.
///
/// # Returns
///
/// Returns a HashMap with all variable names mapped to their default values.
///
/// # Example
///
/// ```rust,ignore
/// let defaults = get_default_variables();
/// assert_eq!(defaults.get("version"), Some(&"0.1.0".to_string()));
/// assert_eq!(defaults.get("author"), Some(&"Dunimd Team".to_string()));
/// ```
pub fn get_default_variables() -> std::collections::HashMap<String, String> {
    let info = get_template_info();
    info.variables
        .into_iter()
        .map(|v| (v.name, v.default_value))
        .collect()
}

/// Get required template variables
///
/// Returns a list of variable names that are required for this template.
///
/// # Returns
///
/// Returns a vector of required variable names.
///
/// # Example
///
/// ```rust,ignore
/// let required = get_required_variables();
/// assert!(required.contains(&"project_name".to_string()));
/// ```
pub fn get_required_variables() -> Vec<String> {
    get_template_info()
        .variables
        .into_iter()
        .filter(|v| v.required)
        .map(|v| v.name)
        .collect()
}

/// Validate template-specific variables
///
/// Performs additional validation for minimal template variables beyond
/// basic type checking. This template has minimal validation requirements.
///
/// # Arguments
///
/// * `variables` - Map of variable names to values
///
/// # Returns
///
/// Returns `Ok(())` if all variables are valid.
/// Returns an error if any variable fails validation.
///
/// # Example
///
/// ```rust,ignore
/// let mut vars = HashMap::new();
/// vars.insert("project_name".to_string(), "my-app".to_string());
///
/// validate_variables(&vars)?; // Ok
/// ```
pub fn validate_variables(variables: &std::collections::HashMap<String, String>) -> anyhow::Result<()> {
    // Validate project_name format (should be valid Rust identifier)
    if let Some(project_name) = variables.get("project_name") {
        if project_name.is_empty() {
            return Err(anyhow::anyhow!("Project name cannot be empty"));
        }

        // Check for valid Rust identifier characters
        let valid = project_name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_');

        if !valid {
            return Err(anyhow::anyhow!(
                "Project name can only contain alphanumeric characters, hyphens, and underscores"
            ));
        }

        // Check that it doesn't start with a number
        if project_name.chars().next().map(|c| c.is_numeric()).unwrap_or(false) {
            return Err(anyhow::anyhow!("Project name cannot start with a number"));
        }
    }

    // Validate version format (should be semver-like)
    if let Some(version) = variables.get("version") {
        if !version.is_empty() {
            // Basic semver check: should have at least major.minor
            let parts: Vec<&str> = version.split('.').collect();
            if parts.len() < 2 {
                return Err(anyhow::anyhow!(
                    "Version should be in semantic versioning format (e.g., 0.1.0)"
                ));
            }

            // Check that each part is numeric
            for part in &parts {
                if part.parse::<u32>().is_err() {
                    return Err(anyhow::anyhow!(
                        "Version parts should be numeric (e.g., 0.1.0)"
                    ));
                }
            }
        }
    }

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_template_info() {
        let info = get_template_info();
        assert_eq!(info.name, "minimal");
        assert!(!info.features.is_empty());
        assert!(!info.variables.is_empty());
        assert!(!info.files.is_empty());
    }

    #[test]
    fn test_get_default_variables() {
        let defaults = get_default_variables();
        assert_eq!(defaults.get("version"), Some(&"0.1.0".to_string()));
        assert_eq!(defaults.get("author"), Some(&"Dunimd Team".to_string()));
        assert_eq!(defaults.get("description"), Some(&"A Ri application".to_string()));
    }

    #[test]
    fn test_get_required_variables() {
        let required = get_required_variables();
        assert!(required.contains(&"project_name".to_string()));
        assert_eq!(required.len(), 1); // Only project_name is required
    }

    #[test]
    fn test_validate_variables_valid() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "my-app".to_string());
        vars.insert("version".to_string(), "0.1.0".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_empty_project_name() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_invalid_project_name() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "my app".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_project_name_starts_with_number() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "123app".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_invalid_version() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "my-app".to_string());
        vars.insert("version".to_string(), "invalid".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_version_missing_patch() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("project_name".to_string(), "my-app".to_string());
        vars.insert("version".to_string(), "0.1".to_string());
        assert!(validate_variables(&vars).is_ok()); // Major.minor is acceptable
    }
}
