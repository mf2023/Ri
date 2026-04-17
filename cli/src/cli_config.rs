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

//! Configuration Management Module
//!
//! This module provides configuration file management for the Ri CLI tool.
//! It handles loading, saving, validating, and manipulating project configuration
//! stored in YAML format (ric.yaml).
//!
//! # Configuration Structure
//!
//! The configuration is organized into three main sections:
//! - `project`: Project metadata (name, version, template)
//! - `build`: Build settings (release mode, target, features)
//! - `runtime`: Runtime settings (log level, workers)
//!
//! # Example Configuration
//!
//! ```yaml
//! project:
//!   name: my-project
//!   version: 0.1.0
//!   template: default
//!
//! build:
//!   release: false
//!   target: all
//!   features:
//!     - default
//!
//! runtime:
//!   log_level: info
//!   workers: 4
//! ```
//!
//! # Key Features
//!
//! - Automatic default values for missing configuration
//! - Configuration validation with detailed error messages
//! - Key-value access using dot notation (e.g., "runtime.workers")
//! - Type-safe configuration with serde serialization

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration file name
const CONFIG_FILE: &str = "ric.yaml";

/// Main configuration structure for Ri CLI
///
/// This structure holds all configuration settings for a Ri project,
/// organized into logical sections for project metadata, build settings,
/// and runtime configuration.
///
/// # Sections
///
/// - `project`: Project identification and template information
/// - `build`: Build configuration and compilation settings
/// - `runtime`: Application runtime behavior settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RicConfig {
    /// Project metadata section
    pub project: ProjectConfig,
    
    /// Build configuration section
    pub build: BuildConfig,
    
    /// Runtime configuration section
    pub runtime: RuntimeConfig,
}

/// Project metadata configuration
///
/// Contains basic project information used for identification
/// and project scaffolding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectConfig {
    /// Project name
    ///
    /// Used as the package name in Cargo.toml and as the default
    /// application name in runtime configuration.
    pub name: String,
    
    /// Project version
    ///
    /// Semantic version string (MAJOR.MINOR.PATCH format).
    pub version: String,
    
    /// Project template
    ///
    /// Specifies the template used for project generation.
    /// Available templates: default, gateway, microservice.
    pub template: String,
}

/// Build configuration settings
///
/// Controls how the project is built and compiled,
/// including optimization level and target platforms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// Release mode flag
    ///
    /// When true, enables compiler optimizations for production deployment.
    /// Results in smaller binary size and better performance but slower compilation.
    pub release: bool,
    
    /// Build target
    ///
    /// Specifies the target platform or binding type:
    /// - "all": Build all targets (default)
    /// - "python": Build Python bindings
    /// - "java": Build Java bindings
    /// - "c": Build C/C++ bindings
    pub target: String,
    
    /// Enabled features
    ///
    /// List of Cargo features to enable during compilation.
    /// Features control conditional compilation and optional dependencies.
    pub features: Vec<String>,
}

/// Runtime configuration settings
///
/// Controls application behavior during execution,
/// including logging and concurrency settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    /// Log level
    ///
    /// Controls the verbosity of logging output.
    /// Common values: "trace", "debug", "info", "warn", "error".
    pub log_level: String,
    
    /// Number of worker threads
    ///
    /// Controls the number of threads used for concurrent task execution.
    /// Should be set based on available CPU cores and workload characteristics.
    pub workers: usize,
}

/// Default configuration implementation
///
/// Provides sensible default values for all configuration fields.
/// These defaults are suitable for most development scenarios and
/// can be customized as needed.
impl Default for RicConfig {
    fn default() -> Self {
        Self {
            project: ProjectConfig {
                name: "my-ri-project".to_string(),
                version: "0.1.0".to_string(),
                template: "default".to_string(),
            },
            build: BuildConfig {
                release: false,
                target: "all".to_string(),
                features: vec!["default".to_string()],
            },
            runtime: RuntimeConfig {
                log_level: "info".to_string(),
                workers: 4,
            },
        }
    }
}

impl RicConfig {
    /// Load configuration from file
    ///
    /// Loads the configuration from ric.yaml in the current directory.
    /// If the file doesn't exist, returns default configuration.
    ///
    /// # Returns
    ///
    /// Returns the loaded configuration or default values if file doesn't exist.
    ///
    /// # Errors
    ///
    /// Returns `RicError::Io` if file reading fails.
    /// Returns `RicError::Yaml` if YAML parsing fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = RicConfig::load()?;
    /// println!("Project name: {}", config.project.name);
    /// ```
    pub fn load() -> Result<Self> {
        if Path::new(CONFIG_FILE).exists() {
            let content = std::fs::read_to_string(CONFIG_FILE)?;
            let config: RicConfig = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// Save configuration to file
    ///
    /// Saves the current configuration to ric.yaml in the current directory.
    /// Creates a new file if it doesn't exist, overwrites if it does.
    ///
    /// # Errors
    ///
    /// Returns `RicError::Io` if file writing fails.
    /// Returns `RicError::Yaml` if YAML serialization fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut config = RicConfig::default();
    /// config.project.name = "my-project".to_string();
    /// config.save()?;
    /// ```
    pub fn save(&self) -> Result<()> {
        let content = serde_yaml::to_string(self)?;
        std::fs::write(CONFIG_FILE, content)?;
        Ok(())
    }

    /// Validate configuration
    ///
    /// Validates the configuration for correctness and consistency.
    /// Checks for:
    /// - Non-empty project name
    /// - Valid worker count (greater than 0)
    ///
    /// # Errors
    ///
    /// Returns `RicError::ConfigInvalid` if validation fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = RicConfig::load()?;
    /// config.validate()?; // Returns error if invalid
    /// ```
    pub fn validate(&self) -> Result<()> {
        if self.project.name.is_empty() {
            return Err(crate::error::RicError::ConfigInvalid(
                "Project name cannot be empty".to_string(),
            ));
        }
        if self.runtime.workers == 0 {
            return Err(crate::error::RicError::ConfigInvalid(
                "Workers must be greater than 0".to_string(),
            ));
        }
        Ok(())
    }

    /// Get a configuration value by key
    ///
    /// Retrieves a configuration value using dot notation.
    /// Supported keys:
    /// - "project.name", "project.version", "project.template"
    /// - "build.release", "build.target"
    /// - "runtime.log_level", "runtime.workers"
    ///
    /// # Arguments
    ///
    /// * `key` - Configuration key in dot notation
    ///
    /// # Returns
    ///
    /// Returns the value as a string.
    ///
    /// # Errors
    ///
    /// Returns `RicError::ConfigKeyNotFound` if the key doesn't exist.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let config = RicConfig::load()?;
    /// let name = config.get("project.name")?;
    /// println!("Project: {}", name);
    /// ```
    pub fn get(&self, key: &str) -> Result<String> {
        match key {
            "project.name" => Ok(self.project.name.clone()),
            "project.version" => Ok(self.project.version.clone()),
            "project.template" => Ok(self.project.template.clone()),
            "build.release" => Ok(self.build.release.to_string()),
            "build.target" => Ok(self.build.target.clone()),
            "runtime.log_level" => Ok(self.runtime.log_level.clone()),
            "runtime.workers" => Ok(self.runtime.workers.to_string()),
            _ => Err(crate::error::RicError::ConfigKeyNotFound(key.to_string())),
        }
    }

    /// Set a configuration value by key
    ///
    /// Updates a configuration value using dot notation.
    /// The value is automatically converted to the appropriate type.
    /// Supported keys:
    /// - "project.name", "project.version", "project.template"
    /// - "build.release", "build.target"
    /// - "runtime.log_level", "runtime.workers"
    ///
    /// # Arguments
    ///
    /// * `key` - Configuration key in dot notation
    /// * `value` - New value as a string (will be converted to appropriate type)
    ///
    /// # Errors
    ///
    /// Returns `RicError::ConfigKeyNotFound` if the key doesn't exist.
    /// Returns `RicError::ConfigInvalid` if type conversion fails.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// let mut config = RicConfig::load()?;
    /// config.set("runtime.workers", "8")?;
    /// config.save()?;
    /// ```
    pub fn set(&mut self, key: &str, value: &str) -> Result<()> {
        match key {
            "project.name" => self.project.name = value.to_string(),
            "project.version" => self.project.version = value.to_string(),
            "project.template" => self.project.template = value.to_string(),
            "build.release" => {
                self.build.release = value.parse().map_err(|_| {
                    crate::error::RicError::ConfigInvalid(
                        "Invalid boolean value".to_string(),
                    )
                })?
            }
            "build.target" => self.build.target = value.to_string(),
            "runtime.log_level" => self.runtime.log_level = value.to_string(),
            "runtime.workers" => {
                self.runtime.workers = value.parse().map_err(|_| {
                    crate::error::RicError::ConfigInvalid("Invalid number".to_string())
                })?
            }
            _ => return Err(crate::error::RicError::ConfigKeyNotFound(key.to_string())),
        }
        Ok(())
    }
}
