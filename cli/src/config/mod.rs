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

//! Configuration Validation Module
//!
//! This module provides comprehensive configuration validation for the Ri CLI tool.
//! It supports validation of YAML and JSON configuration files against predefined
//! schemas for all Ri framework modules.
//!
//! # Module Structure
//!
//! - `schema` - Configuration schema definitions for all Ri modules
//! - `validator` - Configuration validation logic and error reporting
//! - `parser` - Configuration file parsing for YAML and JSON formats
//!
//! # Supported Configuration Schemas
//!
//! - `AppBuilderSchema` - RiAppBuilder configuration validation
//! - `CacheModuleSchema` - RiCacheModule configuration validation
//! - `QueueModuleSchema` - RiQueueModule configuration validation
//! - `GatewaySchema` - RiGateway configuration validation
//! - `AuthModuleSchema` - RiAuthModule configuration validation
//! - `DeviceControlSchema` - RiDeviceControlModule configuration validation
//! - `ObservabilitySchema` - RiObservabilityModule configuration validation
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::config::{ConfigValidator, ConfigParser, ConfigFormat};
//!
//! // Parse a configuration file
//! let parser = ConfigParser::new();
//! let config = parser.parse_file("config.yaml")?;
//!
//! // Validate the configuration
//! let validator = ConfigValidator::new();
//! let result = validator.validate_file("config.yaml")?;
//!
//! // Check for errors
//! if !result.is_valid() {
//!     for error in result.errors() {
//!         println!("Error: {}", error);
//!     }
//!     
//!     // Get fix suggestions
//!     for suggestion in result.suggestions() {
//!         println!("Suggestion: {}", suggestion);
//!     }
//! }
//! ```
//!
//! # Design Principles
//!
//! - **Schema-Driven**: All validation rules are defined in schemas
//! - **Extensible**: Easy to add new module schemas
//! - **Detailed Errors**: Comprehensive error messages with fix suggestions
//! - **Format Agnostic**: Supports both YAML and JSON configuration files

pub mod schema;
pub mod validator;
pub mod parser;

// Re-export main types for convenience
pub use schema::{
    ConfigSchema, FieldSchema, FieldType, FieldConstraint,
    AppBuilderSchema, CacheModuleSchema, QueueModuleSchema,
    GatewaySchema, AuthModuleSchema, DeviceControlSchema,
    ObservabilitySchema,
};

pub use validator::{
    ConfigValidator, ValidationResult, ValidationError,
    ValidationWarning, ValidationSuggestion,
};

pub use parser::{
    ConfigParser, ConfigFormat, ParsedConfig,
};
