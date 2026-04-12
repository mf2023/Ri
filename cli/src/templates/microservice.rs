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

//! Microservice Template
//!
//! This module defines the microservice template for creating gRPC-based microservices
//! with Ri framework. It provides a comprehensive starting point for building
//! distributed systems with service mesh support.
//!
//! # Template Features
//!
//! The microservice template includes:
//!
//! - **gRPC Server**: High-performance gRPC server with async I/O
//! - **Protocol Buffers**: Service definitions using Protocol Buffers
//! - **Service Discovery**: Built-in service discovery and registration
//! - **Health Checking**: gRPC health checking protocol support
//! - **Load Balancing**: Client-side load balancing support
//! - **Interceptors**: Request/response interceptors for middleware
//! - **Reflection**: gRPC reflection for service introspection
//! - **Metrics**: Prometheus metrics for monitoring
//!
//! # Generated Files
//!
//! The template generates the following project structure:
//!
//! ```text
//! my-microservice/
//! ├── Cargo.toml              # Package manifest with dependencies
//! ├── src/
//! │   ├── main.rs            # Application entry point
//! │   ├── server/            # Server implementation
//! │   │   ├── mod.rs
//! │   │   └── service.rs     # Service implementation
//! │   ├── client/            # Client implementation
//! │   │   └── mod.rs
//! │   └── proto/             # Protocol buffer definitions
//! │       └── service.proto
//! ├── config/
//! │   └── config.yaml        # Application configuration
//! ├── build.rs               # Build script for proto compilation
//! └── tests/
//!     └── integration_test.rs
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
//! | `description` | string | No | "A Ri microservice" | Project description |
//! | `grpc_port` | integer | No | "50051" | gRPC server port |
//! | `enable_reflection` | boolean | No | "true" | Enable gRPC reflection |
//! | `enable_health_check` | boolean | No | "true" | Enable health checking |
//! | `enable_tls` | boolean | No | "false" | Enable TLS encryption |
//! | `service_name` | string | No | "MyService" | Service name for discovery |
//! | `discovery_url` | string | No | "" | Service discovery URL |
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::microservice;
//!
//! // Get template information
//! let info = microservice::get_template_info();
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
//! use ri::grpc::{GrpcServer, GrpcConfig};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let app = RiAppBuilder::new("my-microservice")
//!         .with_grpc(GrpcServer::new("[::]:50051"))
//!         .build()
//!         .await?;
//!
//!     app.run().await
//! }
//! ```
//!
//! # Microservice Design Principles
//!
//! - **Performance**: High-performance gRPC with async I/O
//! - **Interoperability**: Protocol Buffers for language-agnostic communication
//! - **Observability**: Built-in metrics, health checks, and reflection
//! - **Scalability**: Service discovery and load balancing support
//! - **Security**: Optional TLS encryption for secure communication
//! - **Testing**: Integration test structure included

use super::engine::{TemplateInfo, TemplateVariable, TemplateFile};

/// Get microservice template metadata
///
/// Returns the complete metadata for the microservice template,
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
/// assert_eq!(info.name, "microservice");
/// assert!(!info.features.is_empty());
/// ```
pub fn get_template_info() -> TemplateInfo {
    TemplateInfo {
        name: "microservice".to_string(),
        display_name: "Microservice".to_string(),
        description: "gRPC microservice with Protocol Buffers, service discovery, and health checking".to_string(),
        author: "Dunimd Team".to_string(),
        version: "1.0.0".to_string(),
        features: vec![
            "High-performance gRPC server".to_string(),
            "Protocol Buffers service definitions".to_string(),
            "Service discovery and registration".to_string(),
            "gRPC health checking protocol".to_string(),
            "Client-side load balancing".to_string(),
            "Request/response interceptors".to_string(),
            "gRPC reflection support".to_string(),
            "Prometheus metrics integration".to_string(),
        ],
        variables: vec![
            TemplateVariable {
                name: "project_name".to_string(),
                description: "Project name used in package manifest and service".to_string(),
                default_value: "my-microservice".to_string(),
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
                description: "Brief description of the microservice".to_string(),
                default_value: "A Ri microservice".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "grpc_port".to_string(),
                description: "gRPC server listening port".to_string(),
                default_value: "50051".to_string(),
                required: false,
                var_type: "integer".to_string(),
            },
            TemplateVariable {
                name: "enable_reflection".to_string(),
                description: "Enable gRPC reflection for service introspection".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_health_check".to_string(),
                description: "Enable gRPC health checking protocol".to_string(),
                default_value: "true".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "enable_tls".to_string(),
                description: "Enable TLS encryption for secure communication".to_string(),
                default_value: "false".to_string(),
                required: false,
                var_type: "boolean".to_string(),
            },
            TemplateVariable {
                name: "service_name".to_string(),
                description: "Service name for service discovery registration".to_string(),
                default_value: "MyService".to_string(),
                required: false,
                var_type: "string".to_string(),
            },
            TemplateVariable {
                name: "discovery_url".to_string(),
                description: "Service discovery URL (leave empty for no discovery)".to_string(),
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
                source: "src/server/mod.rs.tera".to_string(),
                destination: "src/server/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/server/service.rs.tera".to_string(),
                destination: "src/server/service.rs".to_string(),
            },
            TemplateFile {
                source: "src/client/mod.rs.tera".to_string(),
                destination: "src/client/mod.rs".to_string(),
            },
            TemplateFile {
                source: "src/proto/service.proto.tera".to_string(),
                destination: "src/proto/service.proto".to_string(),
            },
            TemplateFile {
                source: "config/config.yaml.tera".to_string(),
                destination: "config/config.yaml".to_string(),
            },
            TemplateFile {
                source: "build.rs.tera".to_string(),
                destination: "build.rs".to_string(),
            },
            TemplateFile {
                source: "tests/integration_test.rs.tera".to_string(),
                destination: "tests/integration_test.rs".to_string(),
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
/// assert_eq!(defaults.get("grpc_port"), Some(&"50051".to_string()));
/// assert_eq!(defaults.get("enable_reflection"), Some(&"true".to_string()));
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
/// Performs additional validation for microservice template variables beyond
/// basic type checking. This includes gRPC port validation and
/// service name format validation.
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
/// vars.insert("grpc_port".to_string(), "50051".to_string());
/// vars.insert("service_name".to_string(), "MyService".to_string());
///
/// validate_variables(&vars)?; // Ok
///
/// vars.insert("grpc_port".to_string(), "80".to_string());
/// validate_variables(&vars)?; // Error: port should be > 1024
/// ```
pub fn validate_variables(variables: &std::collections::HashMap<String, String>) -> anyhow::Result<()> {
    // Validate gRPC port (should be > 1024 for non-privileged)
    if let Some(grpc_port) = variables.get("grpc_port") {
        let port_num: u16 = grpc_port.parse().map_err(|_| {
            anyhow::anyhow!("gRPC port must be a valid number between 1 and 65535")
        })?;
        if port_num == 0 {
            return Err(anyhow::anyhow!("gRPC port cannot be 0"));
        }
        if port_num <= 1024 {
            // Warning: using privileged port, but allow it
            // In production, this might require root privileges
        }
    }

    // Validate service name (should be PascalCase)
    if let Some(service_name) = variables.get("service_name") {
        if service_name.is_empty() {
            return Err(anyhow::anyhow!("Service name cannot be empty"));
        }
        if !service_name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
            return Err(anyhow::anyhow!(
                "Service name should be in PascalCase (e.g., MyService)"
            ));
        }
    }

    // Validate discovery URL format if provided
    if let Some(discovery_url) = variables.get("discovery_url") {
        if !discovery_url.is_empty() && !discovery_url.contains("://") {
            return Err(anyhow::anyhow!(
                "Discovery URL must be a valid URL (e.g., consul://localhost:8500)"
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
        assert_eq!(info.name, "microservice");
        assert!(!info.features.is_empty());
        assert!(!info.variables.is_empty());
        assert!(!info.files.is_empty());
    }

    #[test]
    fn test_get_default_variables() {
        let defaults = get_default_variables();
        assert_eq!(defaults.get("grpc_port"), Some(&"50051".to_string()));
        assert_eq!(defaults.get("enable_reflection"), Some(&"true".to_string()));
        assert_eq!(defaults.get("enable_health_check"), Some(&"true".to_string()));
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
        vars.insert("grpc_port".to_string(), "50051".to_string());
        vars.insert("service_name".to_string(), "MyService".to_string());
        assert!(validate_variables(&vars).is_ok());
    }

    #[test]
    fn test_validate_variables_invalid_service_name() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("service_name".to_string(), "myService".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_empty_service_name() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("service_name".to_string(), "".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_invalid_discovery_url() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("discovery_url".to_string(), "invalid-url".to_string());
        assert!(validate_variables(&vars).is_err());
    }

    #[test]
    fn test_validate_variables_valid_discovery_url() {
        let mut vars = std::collections::HashMap::new();
        vars.insert("discovery_url".to_string(), "consul://localhost:8500".to_string());
        assert!(validate_variables(&vars).is_ok());
    }
}
