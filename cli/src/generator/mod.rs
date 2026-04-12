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

//! Code Generation System Module
//!
//! This module provides a comprehensive code generation system for the Ri CLI.
//! It enables automatic generation of modules, middleware, configuration structures,
//! and dependency management for Ri framework projects.
//!
//! # Architecture
//!
//! The code generation system is organized into three main components:
//!
//! - **Code Generator Engine**: Core engine for generating code from templates
//! - **Template System**: Predefined templates for various module types
//! - **Dependency Manager**: Manages project dependencies in Cargo.toml
//!
//! # Features
//!
//! - **Module Generation**: Generate boilerplate code for different module types
//! - **Middleware Generation**: Create middleware components with standard patterns
//! - **Config Struct Generation**: Generate Rust structs from YAML configuration files
//! - **Code Formatting**: Automatic code formatting using rustfmt
//! - **Dependency Management**: Add, remove, and update dependencies in Cargo.toml
//!
//! # Supported Module Types
//!
//! The generator supports the following module types:
//!
//! 1. **Cache Module**: Distributed caching with Redis/Memcached support
//! 2. **Queue Module**: Message queue integration (RabbitMQ, Kafka, etc.)
//! 3. **Gateway Module**: API gateway with routing and rate limiting
//! 4. **Auth Module**: Authentication and authorization components
//! 5. **Device Module**: IoT device management and communication
//! 6. **Observability Module**: Logging, metrics, and tracing
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::generator::{CodeGenerator, DependencyManager, ModuleType};
//!
//! // Create a code generator
//! let generator = CodeGenerator::new()?;
//!
//! // Generate a cache module
//! let code = generator.generate_module(ModuleType::Cache, "my_cache")?;
//!
//! // Generate middleware
//! let middleware = generator.generate_middleware("auth_middleware")?;
//!
//! // Generate config struct from YAML
//! let config = generator.generate_config_struct("config/app.yaml")?;
//!
//! // Format the generated code
//! let formatted = generator.format_code(&code)?;
//!
//! // Manage dependencies
//! let mut dep_manager = DependencyManager::new("Cargo.toml")?;
//! dep_manager.add_dependency("serde", "1.0", Some(&["derive"]))?;
//! ```

// =============================================================================
// Module Declarations
// =============================================================================

/// Code generation engine
///
/// Provides the core functionality for generating code from templates.
/// Handles module generation, middleware creation, config struct generation,
/// and code formatting.
pub mod engine;

/// Code templates
///
/// Contains predefined templates for various module types and middleware.
/// Templates are used by the code generator to produce consistent,
/// well-structured code.
pub mod templates;

/// Dependency management
///
/// Manages project dependencies in Cargo.toml files.
/// Provides functionality to add, remove, update, and check dependencies.
pub mod dependency;

// =============================================================================
// Re-exports
// =============================================================================

/// Re-export code generator for convenient access
///
/// This allows users to import the generator directly:
/// ```rust,ignore
/// use ric::generator::CodeGenerator;
/// ```
pub use engine::CodeGenerator;

/// Re-export dependency manager for convenient access
///
/// This allows users to import the dependency manager directly:
/// ```rust,ignore
/// use ric::generator::DependencyManager;
/// ```
pub use dependency::DependencyManager;

/// Re-export module type enumeration
///
/// Provides access to the module type enum for specifying
/// what kind of module to generate:
/// ```rust,ignore
/// use ric::generator::ModuleType;
/// ```
pub use engine::ModuleType;
