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

//! API Service Template
//!
//! This module defines the API service template for creating RESTful API services
//! with Ri framework. It provides a comprehensive starting point for building
//! modern API backends with documentation and authentication support.
//!
//! # Template Features
//!
//! The API template includes:
//!
//! - **RESTful Endpoints**: Well-structured REST API endpoints
//! - **OpenAPI Documentation**: Automatic OpenAPI/Swagger documentation generation
//! - **Request Validation**: Built-in request validation and sanitization
//! - **Authentication**: JWT-based authentication middleware
//! - **Rate Limiting**: Configurable rate limiting for API protection
//! - **Response Serialization**: JSON response serialization with proper content types
//! - **Error Handling**: Standardized error responses and HTTP status codes
//! - **API Versioning**: Support for API versioning strategies
//!
//! # Generated Files
//!
//! The template generates the following project structure:
//!
//! ```text
//! my-api/
//! ├── Cargo.toml              # Package manifest with dependencies
//! ├── src/
//! │   ├── main.rs            # Application entry point
//! │   ├── api/               # API endpoint definitions
//! │   │   ├── mod.rs
//! │   │   ├── v1/            # API version 1
//! │   │   │   ├── mod.rs
//! │   │   │   ├── users.rs
//! │   │   │   └── health.rs
//! │   ├── middleware/        # Custom middleware
//! │   │   ├── mod.rs
//! │   │   ├── auth.rs        # Authentication middleware
//! │   │   └── rate_limit.rs  # Rate limiting middleware
//! │   ├── models/            # Data models
//! │   │   ├── mod.rs
//! │   │   ├── user.rs
//! │   │   └── error.rs
//! │   └── db/                # Database layer
//! │       └── mod.rs
//! ├── config/
//! │   └── config.yaml        # Application configuration
//! ├── docs/
//! │   └── openapi.yaml       # OpenAPI specification
//! └── tests/
//!     └── api_test.rs        # API integration tests
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
//! | `description` | string | No | "A Ri API service" | Project description |
//! | `api_version` | string | No | "v1" | API version prefix |
//! | `port` | integer | No | "8080" | API server port |
//! | `enable_docs` | boolean | No | "true" | Enable OpenAPI documentation |
//! | `enable_auth` | boolean | No | "true" | Enable authentication middleware |
//! | `enable_rate_limit` | boolean | No | "true" | Enable rate limiting |
//! | `database_url` | string | No | "" | Database connection URL |
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::api;
//!
//! // Get template information
//! let info = api::get_template_info();
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
//! use ri::api::{ApiServer, ApiConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = RiAppBuilder::new("my-api")
//!         .with_api(ApiServer::new("0.0.0.0:8080"))
//!         .build()
//!         .await?;
//!
//!     app.run().await
//! }
//! ```
//!
//! # API Design Principles
//!
//! - **RESTful Conventions**: Follow REST API best practices
//! - **Versioning**: Built-in support for API versioning
//! - **Documentation**: Auto-generated OpenAPI documentation
//! - **Security**: Authentication and rate limiting by default
//! - **Validation**: Request validation and sanitization
//! - **Error Handling**: Consistent error response format
//! - **Testing**: Integration test structure included

use super::engine::{TemplateInfo, TemplateVariable, TemplateFile};

/// Get API template metadata
///
/// Returns the complete metadata for the API service template,
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
/// assert_eq!(info.name, "api");
/// assert!(!info.features.is_empty());
/// ```
pub fn get_template_info() -> TemplateInfo {
    TemplateInfo {
        name: "api".to_string(),
        display_name: "API Service".to_string(),
        description: "RESTful API service with OpenAPI documentation, authentication, and rate limiting".to_string(),
        author: "Dunimd Team".to_string(),
        version: "1.0.0".to_string(),
        features: vec![
            "RESTful API endpoints".to_string(),
            "OpenAPI/Swagger documentation".to_string(),
            "Request validation and sanitization".to_string(),
            "JWT authentication middleware".to_string(),
            "Rate limiting protection".to_string(),
            "JSON response serialization".to_string(),
            "Standardized error handling".to_string(),
            "API versioning support".to_string(),
        ],
        variables: vec![
            TemplateVariable {
                name: "project_name".to_string(),
                description: "Project name used in package manifest and API".to_string(),
                default_value: "my-api".to_string(),
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
                description: "Brief description of the API service".to_string(),
                default_value: "A Ri API service".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "api_version".to_string(),
                description: "API version prefix for URL routing".to_string(),
                default_value: "v1".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "port".to_string(),
                description: "API server listening port".to_string(),
                default_value: "8080".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "enable_docs".to_string(),
                description: "Enable OpenAPI/Swagger documentation endpoint".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_auth".to_string(),
                description: "Enable JWT authentication middleware".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_rate_limit".to_string(),
                description: "Enable rate limiting middleware".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "database_url".to_string(),
                description: "Database connection URL (leave empty for no database)".to_string(),
                default_value: "".to_string(),
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
                source: "src/api/mod.rs.tera".to_string(),
                destination: "src/api/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/api/v1/mod.rs.tera".to_string(),
                destination: "src/api/v1/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/api/v1/users.rs.tera".to_string(),
                destination: "src/api/v1/users.rs".to_string(),
            },
            TemplateFile {
                source: "src/api/v1/health.rs.tera".to_string(),
                destination: "src/api/v1/health.rs".to_string(),
            },
            TemplateFile {
                source: "src/middleware/mod.rs.tera".to_string(),
                destination: "src/middleware/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/middleware/auth.rs.tera".to_string(),
                destination: "src/middleware/auth.rs".to_string(),
            },
            TemplateFile {
                source: "src/middleware/rate_limit.rs.tera".to_string(),
                destination: "src/middleware/rate_limit.rs".to_string(),
            },
            TemplateFile {
                source: "src/models/mod.rs.tera".to_string(),
                destination: "src/models/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/models/user.rs.tera".to_string(),
                destination: "src/models/user.rs".to_string(),
            },
            TemplateFile {
                source: "src/models/error.rs.tera".to_string(),
                destination: "src/models/error.rs".to_string(),
            },
            TemplateFile {
                source: "src/db/mod.rs.tera".to_string(),
                destination: "src/db/mod.rs".to_string(),
            },
            TemplateFile {
                source: "config/config.yaml.tera".to_string(),
                destination: "config/config.yaml".to_string(),
            },
            TemplateFile {
                source: "docs/openapi.yaml.tera".to_string(),
                destination: "docs/openapi.yaml".to_string(),
            },
            TemplateFile {
                source: "tests/api_test.rs.tera".to_string(),
                destination: "tests/api_test.rs".to_string(),
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
/// assert_eq!(defaults.get("api_version"), Some(&"v1".to_string()));
/// assert_eq!(defaults.get("enable_docs"), Some(&"true".to_string()));
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
/// Performs additional validation for API template variables beyond
/// basic type checking. This includes API version format validation
/// and database URL format validation.
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
/// vars.insert("api_version".to_string(), "v1".to_string());
/// vars.insert("port".to_string(), "8080".to_string());
///
/// validate_variables(&vars)?; // Ok
///
/// vars.insert("api_version".to_string(), "invalid".to_string());
/// validate_variables(&vars)?; // Error: invalid API version format
/// ```
pub fn validate_variables(variables: &std::collections::HashMap<String, String>) -> anyhow::Result<()> {
    // Validate API version format (should be like v1, v2, etc.)
    if let Some(api_version) = variables.get("api_version") {
        if !api_version.starts_with('v') {
            return Err(anyhow::anyhow!(
                "API version must start with 'v' (e.g., v1, v2)"
            ));
        }
    }

    // Validate port range (1-65535)
    if let Some(port) = variables.get("port") {
        let port_num: u16 = port.parse().map_err(|_| {
            anyhow::anyhow!("Port must be a valid number between 1 and 65535")
        })?;
        if port_num == 0 {
            return Err(anyhow::anyhow!("Port cannot be 0"));
        }
    }

    // Validate database URL format if provided
    if let Some(db_url) = variables.get("database_url") {
        if !db_url.is_empty() && !db_url.contains("://") {
            return Err(anyhow::anyhow!(
                "Database URL must be a valid connection string (e.g., postgresql://...)"
            ));
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
        assert_eq!(info.name, "api");
        assert!(!info.features.is_empty());
        assert!(!info.variables.is_empty());
        assert!(!info.files.is_empty());
    }

    #[test]
    fn test_get_default_variables() {
        let defaults = get_default_variables();
        assert_eq!(defaults.get("api_version"), Some(&"v1".to_string()));
        assert_eq!(defaults.get("enable_docs"), Some(&"true".to_string()));
        assert_eq!(defaults.get("enable_auth"), Some(&"true".to_string()));
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
        vars.insert("api_version".to_string(), "v1".to_string());
        vars.insert("port".to_string(), "8080".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_invalid_api_version() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("api_version".to_string(), "invalid".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_invalid_database_url() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("database_url".to_string(), "invalid-url".to_string());
        assert!(validate_variables(&vars).is_err());
    }
}
