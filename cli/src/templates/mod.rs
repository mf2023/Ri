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

//! Template System Module
//!
//! This module provides a comprehensive template system for generating Ri projects.
//! It includes multiple project templates and a powerful rendering engine based on Tera.
//!
//! # Architecture
//!
//! The template system is organized into several components:
//!
//! - **Template Engine**: Core rendering engine using Tera for template processing
//! - **Template Definitions**: Individual template modules for different project types
//! - **Template Metadata**: Structured information about each template
//!
//! # Available Templates
//!
//! The system provides five built-in templates:
//!
//! 1. **Web Template** (`web`): Full-featured web application with HTTP server, routing, and middleware
//! 2. **API Template** (`api`): RESTful API service with OpenAPI documentation support
//! 3. **Worker Template** (`worker`): Background job processing service with task queues
//! 4. **Microservice Template** (`microservice`): gRPC microservice with service definitions
//! 5. **Minimal Template** (`minimal`): Minimal Ri application for simple use cases
//!
//! # Template Structure
//!
//! Each template defines:
//!
//! - **Metadata**: Name, description, author, version, and features
//! - **Files**: List of files to generate with their templates
//! - **Variables**: Template variables with default values and descriptions
//! - **Rendering Logic**: Custom rendering behavior for specific templates
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::templates::{TemplateEngine, TemplateInfo};
//!
//! // Create a template engine
//! let engine = TemplateEngine::new()?;
//!
//! // List available templates
//! let templates = engine.list_templates();
//! for template in templates {
//!     println!("{}: {}", template.name, template.description);
//! }
//!
//! // Get specific template info
//! let info = engine.get_template_info("web")?;
//! println!("Template: {}", info.name);
//! println!("Features: {:?}", info.features);
//!
//! // Create a project from template
//! use std::collections::HashMap;
//! let mut variables = HashMap::new();
//! variables.insert("project_name".to_string(), "my-web-app".to_string());
//!
//! engine.create_project("my-web-app", "web", &variables, "./output")?;
//! ```
//!
//! # Template Variables
//!
//! Each template supports a set of variables that can be customized:
//!
//! - `project_name`: The name of the project (required)
//! - `version`: Project version (default: "0.1.0")
//! - `author`: Project author (default: "Dunimd Team")
//! - `description`: Project description (default: "A Ri application")
//!
//! Template-specific variables:
//!
//! - **Web**: `port`, `workers`, `enable_tls`
//! - **API**: `api_version`, `enable_docs`, `enable_auth`
//! - **Worker**: `queue_type`, `max_workers`, `enable_persistence`
//! - **Microservice**: `grpc_port`, `enable_reflection`, `enable_health_check`
//! - **Minimal**: No additional variables
//!
//! # Extending the Template System
//!
//! To add a new template:
//!
//! 1. Create a new module file (e.g., `my_template.rs`)
//! 2. Implement the `TemplateDefinition` trait
//! 3. Register the template in `engine.rs`
//! 4. Add the module export in this file
//!
//! # Design Principles
//!
//! - **Separation of Concerns**: Each template is isolated in its own module
//! - **Extensibility**: Easy to add new templates without modifying core logic
//! - **Type Safety**: Strong typing for template metadata and variables
//! - **Error Handling**: Comprehensive error messages for template failures
//! - **Performance**: Lazy loading and caching of template resources

// =============================================================================
// Module Declarations
// =============================================================================

/// Template rendering engine
///
/// Provides the core functionality for template processing and project generation.
/// Uses Tera as the underlying template engine for powerful and flexible rendering.
pub mod engine;

/// Web application template
///
/// Full-featured web application template with:
/// - HTTP server with async I/O
/// - Request routing and middleware
/// - Static file serving
/// - Template rendering
/// - Session management
pub mod web;

/// API service template
///
/// RESTful API service template with:
/// - OpenAPI/Swagger documentation
/// - Request validation
/// - Authentication middleware
/// - Rate limiting
/// - Response serialization
pub mod api;

/// Background worker template
///
/// Background job processing template with:
/// - Task queue integration
/// - Worker pool management
/// - Job scheduling
/// - Error handling and retries
/// - Progress tracking
pub mod worker;

/// Microservice template
///
/// gRPC microservice template with:
/// - Service definitions
/// - Protocol buffer support
/// - Service discovery
/// - Health checking
/// - Load balancing
pub mod microservice;

/// Minimal application template
///
/// Minimal Ri application template with:
/// - Basic application structure
/// - Simple configuration
/// - Minimal dependencies
/// - Quick start setup
pub mod minimal;

// =============================================================================
// Re-exports
// =============================================================================

/// Re-export template engine for convenient access
///
/// This allows users to import the engine directly:
/// ```rust,ignore
/// use ric::templates::TemplateEngine;
/// ```
pub use engine::TemplateEngine;

/// Re-export template metadata structures
///
/// Provides access to template information structures:
/// ```rust,ignore
/// use ric::templates::{TemplateInfo, TemplateVariable, TemplateFile};
/// ```
pub use engine::{TemplateInfo, TemplateVariable, TemplateFile};
