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

//! Project Template Generation Module
//!
//! This module provides template generation functionality for creating new Ri projects.
//! It generates the necessary files and directory structure for different project types.
//!
//! # Templates
//!
//! Three project templates are available:
//! - **default**: Basic Ri application with minimal setup
//! - **gateway**: API Gateway with routing and middleware
//! - **microservice**: gRPC microservice with service definitions
//!
//! # Generated Files
//!
//! Each template generates:
//! - `Cargo.toml`: Package manifest with Ri dependencies
//! - `src/main.rs`: Application entry point
//! - `config/config.yaml`: Configuration file
//!
//! # Template Structure
//!
//! Templates are generated using string templates with placeholder substitution.
//! This approach provides:
//! - Simple and maintainable template definitions
//! - Fast generation without external template engine overhead
//! - Easy customization and extension

/// Generate Cargo.toml content for a new project
///
/// Creates a Cargo.toml file with the necessary dependencies for a Ri project.
/// The generated manifest includes:
/// - Project metadata (name, version, edition)
/// - Ri framework dependency
/// - Tokio async runtime
/// - Serde for serialization
///
/// # Arguments
///
/// * `name` - Project name to use in the manifest
///
/// # Returns
///
/// Returns the complete Cargo.toml content as a string.
///
/// # Example
///
/// ```rust,ignore
/// let cargo_toml = generate_cargo_toml("my-project");
/// // Returns Cargo.toml content with name = "my-project"
/// ```
pub fn generate_cargo_toml(name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
ri = "0.1.9"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_yaml = "0.9"
"#,
        name
    )
}

/// Generate main.rs content based on template
///
/// Creates the main entry point file for the project based on the selected template.
/// Each template provides a different starting point:
///
/// # Templates
///
/// - `gateway`: API Gateway with routing configuration
/// - `microservice`: gRPC service with server setup
/// - `default` (or any other): Basic Ri application
///
/// # Arguments
///
/// * `template` - Template name (default, gateway, microservice)
///
/// # Returns
///
/// Returns the main.rs content appropriate for the template.
pub fn generate_main_rs(template: &str) -> String {
    match template {
        "gateway" => generate_gateway_main(),
        "microservice" => generate_microservice_main(),
        _ => generate_default_main(),
    }
}

/// Generate default template main.rs
///
/// Creates a basic Ri application entry point with:
/// - RiAppBuilder for application construction
/// - Async main function
/// - Basic error handling
fn generate_default_main() -> String {
    r#"use ri::core::RiAppBuilder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = RiAppBuilder::new("my-app")
        .build()
        .await?;
    
    app.run().await
}
"#.to_string()
}

/// Generate gateway template main.rs
///
/// Creates an API Gateway entry point with:
/// - Gateway configuration
/// - Routing setup
/// - Middleware integration
fn generate_gateway_main() -> String {
    r#"use ri::core::RiAppBuilder;
use ri::gateway::GatewayConfig;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = RiAppBuilder::new("gateway")
        .with_gateway(GatewayConfig::default())
        .build()
        .await?;
    
    app.run().await
}
"#.to_string()
}

/// Generate microservice template main.rs
///
/// Creates a gRPC microservice entry point with:
/// - gRPC server configuration
/// - Service binding
/// - Reflection support
fn generate_microservice_main() -> String {
    r#"use ri::core::RiAppBuilder;
use ri::grpc::GrpcServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = RiAppBuilder::new("microservice")
        .with_grpc(GrpcServer::new("[::]:50051"))
        .build()
        .await?;
    
    app.run().await
}
"#.to_string()
}

/// Generate config.yaml content based on template
///
/// Creates the configuration file for the project based on the selected template.
/// Each template has different configuration requirements:
///
/// # Templates
///
/// - `gateway`: Gateway-specific configuration with routes
/// - `microservice`: gRPC service configuration
/// - `default`: Basic application configuration
///
/// # Arguments
///
/// * `template` - Template name (default, gateway, microservice)
///
/// # Returns
///
/// Returns the config.yaml content appropriate for the template.
pub fn generate_config_yaml(template: &str) -> String {
    match template {
        "gateway" => generate_gateway_config(),
        "microservice" => generate_microservice_config(),
        _ => generate_default_config(),
    }
}

/// Generate default template configuration
///
/// Creates a basic configuration file with:
/// - Application metadata
/// - Logging configuration
/// - Runtime settings
fn generate_default_config() -> String {
    r#"# Ri Application Configuration
app:
  name: my-app
  version: 0.1.0

logging:
  level: info
  format: json

runtime:
  workers: 4
"#.to_string()
}

/// Generate gateway template configuration
///
/// Creates a gateway configuration file with:
/// - Gateway routing rules
/// - Service definitions
/// - Enhanced worker count for gateway workload
fn generate_gateway_config() -> String {
    r#"# Ri Gateway Configuration
app:
  name: gateway
  version: 0.1.0

gateway:
  listen: "0.0.0.0:8080"
  routes:
    - path: /api/*
      service: backend-service

logging:
  level: info
  format: json

runtime:
  workers: 8
"#.to_string()
}

/// Generate microservice template configuration
///
/// Creates a microservice configuration file with:
/// - gRPC server settings
/// - Service reflection
/// - Standard runtime configuration
fn generate_microservice_config() -> String {
    r#"# Ri Microservice Configuration
app:
  name: microservice
  version: 0.1.0

grpc:
  listen: "[::]:50051"
  reflection: true

logging:
  level: info
  format: json

runtime:
  workers: 4
"#.to_string()
}
