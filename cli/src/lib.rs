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

//! Ri CLI (ric) - Library Module
//!
//! This is the library entry point for the Ri CLI tool. It provides the core
//! functionality for managing Ri framework projects through a programmatic API.
//!
//! # Module Structure
//!
//! The library is organized into several modules, each responsible for a specific
//! aspect of CLI functionality:
//!
//! - `cli` - Command-line interface definitions and argument parsing
//! - `commands` - Command execution logic and handlers
//! - `config` - Configuration file management and validation
//! - `error` - Custom error types and error handling
//! - `generator` - Code generation system for modules, middleware, and dependencies
//! - `template` - Project template generation and scaffolding
//! - `utils` - Utility functions and helper methods
//!
//! # Usage
//!
//! The library can be used programmatically to integrate Ri CLI functionality
//! into other tools or to extend the CLI with custom commands:
//!
//! ```rust,ignore
//! use ric::commands;
//! use ric::config::RicConfig;
//! use std::path::PathBuf;
//!
//! // Create a new project
//! commands::new_project(
//!     "my-project".to_string(),
//!     "minimal".to_string(),
//!     None
//! )?;
//!
//! // Create a project with custom path
//! commands::new_project(
//!     "my-web-app".to_string(),
//!     "web".to_string(),
//!     Some(PathBuf::from("/path/to/projects"))
//! )?;
//!
//! // Load and validate configuration
//! let config = RicConfig::load()?;
//! config.validate()?;
//! ```
//!
//! # Design Principles
//!
//! - **Modularity**: Each module has a single responsibility
//! - **Error transparency**: Rich error types with detailed context
//! - **User experience**: Colored output and progress indicators
//!
//! # Integration with Ri Framework
//!
//! The CLI tool integrates with the parent Ri framework to provide:
//! - Project scaffolding based on Ri templates
//! - Build management for multiple targets (Python, Java, C)
//! - Configuration management for Ri applications
//! - Development workflow automation

// =============================================================================
// Module Declarations
// =============================================================================

/// Command-line interface definitions and argument parsing
///
/// This module uses clap's derive macros to define the CLI structure:
/// - `Cli` - Top-level CLI structure with subcommands
/// - `Commands` - Enum of all available commands
/// - `ConfigAction` - Subcommands for configuration management
pub mod cli;

/// Command execution logic and handlers
///
/// This module contains the implementation of all CLI commands:
/// - `new_project` - Create a new Ri project
/// - `build_project` - Build the project
/// - `run_project` - Run the project
/// - `handle_config` - Manage configuration
/// - `check_project` - Check for errors
/// - `clean_project` - Clean build artifacts
/// - `show_info` - Display project information
pub mod commands;

/// Configuration file management
///
/// This module provides:
/// - `RicConfig` - Main configuration structure
/// - Configuration loading and saving
/// - Key-value access to configuration settings
pub mod config;

/// Configuration schema validation and parsing
///
/// This module provides:
/// - Schema definitions for all Ri modules
/// - Configuration validation
/// - YAML/JSON configuration parsing
pub mod config_validation;

/// Custom error types and error handling
///
/// This module defines domain-specific error types:
/// - `RicError` - Enum of all possible CLI errors
/// - `Result<T>` - Type alias for `std::result::Result<T, RicError>`
pub mod error;

/// Code generation system
///
/// This module provides code generation capabilities:
/// - `CodeGenerator` - Generate modules, middleware, and config structs
/// - `DependencyManager` - Manage Cargo.toml dependencies
/// - `ModuleType` - Enumeration of supported module types
///
/// Supported module types:
/// - Cache: Distributed caching with Redis/Memcached support
/// - Queue: Message queue integration
/// - Gateway: API gateway with routing and rate limiting
/// - Auth: Authentication and authorization components
/// - Device: IoT device management
/// - Observability: Logging, metrics, and tracing
pub mod generator;

/// Project template generation and scaffolding
///
/// This module provides template generation for:
/// - Cargo.toml files
/// - Main.rs entry points
/// - Configuration files
/// - Different project templates (default, gateway, microservice)
pub mod template;

/// Template engine and project templates
///
/// This module provides:
/// - TemplateEngine for rendering templates
/// - Template definitions for different project types
/// - Template metadata and variables
pub mod templates;

/// Utility functions and helper methods
///
/// This module contains helper functions for:
/// - Directory operations
/// - Project detection
/// - Duration formatting
pub mod utils;

// =============================================================================
// Public Exports
// =============================================================================

/// Re-export error types for convenience
///
/// This allows users to import error types directly from the crate root:
/// ```rust,ignore
/// use ric::{Result, RicError};
/// ```
pub use error::{Result, RicError};
