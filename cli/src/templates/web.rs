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

//! Web Application Template
//!
//! This module defines the web application template for creating full-featured
//! web applications with Ri framework. It provides a comprehensive starting point
//! for building modern web services.
//!
//! # Template Features
//!
//! The web template includes:
//!
//! - **HTTP Server**: Async HTTP server with configurable port and workers
//! - **Routing**: Flexible request routing with path parameters
//! - **Middleware**: Built-in middleware support (logging, CORS, compression)
//! - **Static Files**: Static file serving capabilities
//! - **Templates**: Server-side template rendering
//! - **Session Management**: Session handling for stateful applications
//! - **Error Handling**: Comprehensive error handling and responses
//!
//! # Generated Files
//!
//! The template generates the following project structure:
//!
//! ```text
//! my-web-app/
//! ├── Cargo.toml              # Package manifest with dependencies
//! ├── src/
//! │   ├── main.rs            # Application entry point
//! │   ├── routes/            # Route handlers
//! │   │   ├── mod.rs
//! │   │   └── api.rs
//! │   ├── middleware/        # Custom middleware
//! │   │   └── mod.rs
//! │   └── handlers/          # Request handlers
//! │       └── mod.rs
//! ├── config/
//! │   └── config.yaml        # Application configuration
//! ├── static/                # Static files directory
//! │   └── .gitkeep
//! └── templates/             # Server-side templates
//!     └── .gitkeep
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
//! | `description` | string | No | "A Ri web application" | Project description |
//! | `port` | integer | No | "8080" | HTTP server port |
//! | `workers` | integer | No | "4" | Number of worker threads |
//! | `enable_tls` | boolean | No | "false" | Enable TLS/HTTPS |
//! | `enable_cors` | boolean | No | "true" | Enable CORS middleware |
//! | `enable_compression` | boolean | No | "true" | Enable response compression |
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::web;
//!
//! // Get template information
//! let info = web::get_template_info();
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
//! use ri::web::{WebServer, WebConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = RiAppBuilder::new("my-web-app")
//!         .with_web(WebServer::new("0.0.0.0:8080"))
//!         .build()
//!         .await?;
//!
//!     app.run().await
//! }
//! ```
//!
//! # Design Decisions
//!
//! - **Modular Structure**: Routes and handlers are separated for maintainability
//! - **Configuration-Driven**: Most settings are configurable via config.yaml
//! - **Async-First**: All I/O operations are asynchronous
//! - **Middleware Stack**: Common middleware is pre-configured but customizable
//! - **Development Ready**: Includes development-friendly defaults

use super::engine::{TemplateInfo, TemplateVariable, TemplateFile};

/// Get web template metadata
///
/// Returns the complete metadata for the web application template,
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
/// assert_eq!(info.name, "web");
/// assert!(!info.features.is_empty());
/// ```
pub fn get_template_info() -> TemplateInfo {
    TemplateInfo {
        name: "web".to_string(),
        display_name: "Web Application".to_string(),
        description: "Full-featured web application with HTTP server, routing, middleware, and static file serving".to_string(),
        author: "Dunimd Team".to_string(),
        version: "1.0.0".to_string(),
        features: vec![
            "HTTP Server with async I/O".to_string(),
            "Flexible request routing".to_string(),
            "Middleware support (logging, CORS, compression)".to_string(),
            "Static file serving".to_string(),
            "Server-side template rendering".to_string(),
            "Session management".to_string(),
            "Comprehensive error handling".to_string(),
        ],
        variables: vec![
            TemplateVariable {
                name: "project_name".to_string(),
                description: "Project name used in package manifest and application".to_string(),
                default_value: "my-web-app".to_string(),
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
                default_value: "A Ri web application".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "port".to_string(),
                description: "HTTP server listening port".to_string(),
                default_value: "8080".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "workers".to_string(),
                description: "Number of worker threads for the server".to_string(),
                default_value: "4".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "enable_tls".to_string(),
                description: "Enable TLS/HTTPS support".to_string(),
                default_value: "false".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_cors".to_string(),
                description: "Enable CORS middleware for cross-origin requests".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_compression".to_string(),
                description: "Enable response compression middleware".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
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
                source: "src/routes/mod.rs.tera".to_string(),
                destination: "src/routes/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/routes/api.rs.tera".to_string(),
                destination: "src/routes/api.rs".to_string(),
            },
            TemplateFile {
                source: "src/middleware/mod.rs.tera".to_string(),
                destination: "src/middleware/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/handlers/mod.rs.tera".to_string(),
                destination: "src/handlers/mod.rs".to_string(),
            },
            TemplateFile {
                source: "config/config.yaml.tera".to_string(),
                destination: "config/config.yaml".to_string(),
            },
            TemplateFile {
                source: "static/.gitkeep.tera".to_string(),
                destination: "static/.gitkeep".to_string(),
            },
            TemplateFile {
                source: "templates/.gitkeep.tera".to_string(),
                destination: "templates/.gitkeep".to_string(),
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
/// assert_eq!(defaults.get("port"), Some(&"8080".to_string()));
/// assert_eq!(defaults.get("workers"), Some(&"4".to_string()));
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
/// Performs additional validation for web template variables beyond
/// basic type checking. This includes port range validation and
/// worker count validation.
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
/// vars.insert("port".to_string(), "8080".to_string());
/// vars.insert("workers".to_string(), "4".to_string());
///
/// validate_variables(&vars)?; // Ok
///
/// vars.insert("port".to_string(), "99999".to_string());
/// validate_variables(&vars)?; // Error: port out of range
/// ```
pub fn validate_variables(variables: &std::collections::HashMap<String, String>) -> anyhow::Result<()> {
    // Validate port range (1-65535)
    if let Some(port) = variables.get("port") {
        let port_num: u16 = port.parse().map_err(|_| {
            anyhow::anyhow!("Port must be a valid number between 1 and 65535")
        })?;
        if port_num == 0 {
            return Err(anyhow::anyhow!("Port cannot be 0"));
        }
    }

    // Validate worker count (1-1024)
    if let Some(workers) = variables.get("workers") {
        let workers_num: usize = workers.parse().map_err(|_| {
            anyhow::anyhow!("Workers must be a valid positive number")
        })?;
        if workers_num == 0 || workers_num > 1024 {
            return Err(anyhow::anyhow!("Workers must be between 1 and 1024"));
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
        assert_eq!(info.name, "web");
        assert!(!info.features.is_empty());
        assert!(!info.variables.is_empty());
        assert!(!info.files.is_empty());
    }

    #[test]
    fn test_get_default_variables() {
        let defaults = get_default_variables();
        assert_eq!(defaults.get("port"), Some(&"8080".to_string()));
        assert_eq!(defaults.get("workers"), Some(&"4".to_string()));
        assert_eq!(defaults.get("enable_tls"), Some(&"false".to_string()));
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
        vars.insert("port".to_string(), "8080".to_string());
        vars.insert("workers".to_string(), "4".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_invalid_port() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("port".to_string(), "99999".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_invalid_workers() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("workers".to_string(), "0".to_string());
        assert!(validate_variables(&vars).is_err());
    }
}
