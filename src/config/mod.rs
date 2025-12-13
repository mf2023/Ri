//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! You may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # Configuration Management
//! 
//! This module provides a comprehensive configuration management system for DMS, supporting
//! multiple configuration sources, hot reload, and flexible configuration access.
//! 
//! ## Key Components
//! 
//! - **DMSConfig**: Basic configuration storage with typed access methods
//! - **DMSConfigManager**: Configuration manager that handles multiple sources and hot reload
//! - **DMSConfigSource**: Internal enum for different configuration source types
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Sources**: Supports configuration from files (JSON, YAML, TOML) and environment variables
//! 2. **Source Priority**: Environment variables override file configuration
//! 3. **Typed Access**: Provides type-safe methods for accessing configuration values
//! 4. **Flattened Structure**: All configuration is flattened into a single key-value store with dot notation
//! 5. **Hot Reload Support**: Simplified hot reload implementation with full support planned for future
//! 6. **Default Sources**: Automatically loads configuration from common locations
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a new config manager
//!     let mut config_manager = DMSConfigManager::new();
//!     
//!     // Add configuration sources
//!     config_manager.add_file_source("config.yaml");
//!     config_manager.add_environment_source();
//!     
//!     // Load configuration
//!     config_manager.load()?;
//!     
//!     // Access configuration values
//!     let config = config_manager.config();
//!     let port = config.get_u64("server.port").unwrap_or(8080);
//!     let debug = config.get_bool("app.debug").unwrap_or(false);
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use yaml_rust::{YamlLoader, Yaml};

/// Basic configuration storage with typed access methods.
/// 
/// This struct provides a simple key-value store for configuration values, with
/// type-safe methods for accessing values as different types.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSConfig {
    /// Internal storage for configuration values
    values: HashMap<String, String>,
}

impl Default for DMSConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSConfig {
    /// Creates a new empty configuration.
    /// 
    /// Returns a new `DMSConfig` instance with an empty key-value store.
    pub fn new() -> Self {
        DMSConfig { values: HashMap::new() }
    }

    /// Sets a configuration value.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key (typically using dot notation, e.g., "server.port")
    /// - `value`: The configuration value as a string
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.values.insert(key.into(), value.into());
    }

    /// Gets a configuration value as a string.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<&String>` containing the value if it exists
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    /// Gets a configuration value as a string slice.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<&str>` containing the value if it exists
    pub fn get_str(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(|s| s.as_str())
    }

    /// Gets a configuration value as a boolean.
    /// 
    /// Supports the following truthy values: "true", "1", "yes", "on"
    /// Supports the following falsy values: "false", "0", "no", "off"
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<bool>` containing the parsed boolean value if the key exists and can be parsed
    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.values.get(key).and_then(|s| {
            let v = s.trim().to_ascii_lowercase();
            match v.as_str() {
                "true" | "1" | "yes" | "on" => Some(true),
                "false" | "0" | "no" | "off" => Some(false),
                _ => None,
            }
        })
    }

    /// Gets a configuration value as a 64-bit signed integer.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<i64>` containing the parsed integer value if the key exists and can be parsed
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.values.get(key).and_then(|s| s.trim().parse::<i64>().ok())
    }

    /// Gets a configuration value as a 64-bit unsigned integer.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<u64>` containing the parsed integer value if the key exists and can be parsed
    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.values.get(key).and_then(|s| s.trim().parse::<u64>().ok())
    }

    /// Gets a configuration value as a 32-bit floating point number.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<f32>` containing the parsed float value if the key exists and can be parsed
    pub fn get_f32(&self, key: &str) -> Option<f32> {
        self.values.get(key).and_then(|s| s.trim().parse::<f32>().ok())
    }

    /// Merges another configuration into this one.
    /// 
    /// Values from the other configuration will override existing values with the same keys.
    /// 
    /// # Parameters
    /// 
    /// - `other`: The other configuration to merge into this one
    pub fn merge(&mut self, other: &DMSConfig) {
        for (k, v) in &other.values {
            self.values.insert(k.clone(), v.clone());
        }
    }

    /// Clears all configuration values.
    /// 
    /// Removes all key-value pairs from the configuration.
    pub fn clear(&mut self) {
        self.values.clear();
    }
}

#[cfg(feature = "pyo3")]
/// Python constructor for DMSConfig
#[pyo3::prelude::pymethods]
impl DMSConfig {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Sets a configuration value from Python
    fn set_py(&mut self, key: String, value: String) {
        self.set(key, value);
    }
    
    /// Gets a configuration value as string from Python
    fn get_py(&self, key: String) -> Option<String> {
        self.get(&key).cloned()
    }
}

/// Internal enum for different configuration source types.
/// 
/// This enum represents the different types of configuration sources that the
/// `DMSConfigManager` can handle.
#[derive(Clone)]
enum DMSConfigSource {
    /// File-based configuration source
    File(PathBuf),
    /// Environment variable configuration source
    Environment,
}

/// Public-facing configuration manager with hot reload support.
/// 
/// This struct manages multiple configuration sources, loads configuration values,
/// and provides access to the configuration. It supports hot reload and multiple
/// configuration formats.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSConfigManager {
    /// Internal configuration storage
    config: DMSConfig,
    /// List of configuration sources to load from
    sources: Vec<DMSConfigSource>,
}

impl Default for DMSConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSConfigManager {
    /// Creates a new empty configuration manager.
    /// 
    /// Returns a new `DMSConfigManager` instance with no configuration sources.
    pub fn new() -> Self {
        DMSConfigManager {
            config: DMSConfig::new(),
            sources: Vec::new(),
        }
    }

    /// Adds a file-based configuration source.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the configuration file
    /// 
    /// Supported file formats: JSON, YAML, TOML
    pub fn add_file_source(&mut self, path: impl AsRef<Path>) {
        self.sources.push(DMSConfigSource::File(path.as_ref().to_path_buf()));
    }

    /// Adds environment variables as a configuration source.
    /// 
    /// Environment variables with the prefix `DMS_` are loaded as configuration values.
    /// Double underscores (`__`) in environment variable names are converted to dots.
    /// For example, `DMS_SERVER__PORT=8080` becomes `server.port=8080`.
    pub fn add_environment_source(&mut self) {
        self.sources.push(DMSConfigSource::Environment);
    }

    /// Loads configuration from all registered sources.
    /// 
    /// This method loads configuration from all registered sources in the order they were added,
    /// with later sources overriding earlier ones.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), DMSError>` indicating success or failure
    pub fn load(&mut self) -> Result<(), crate::core::DMSError> {
        let mut cfg = DMSConfig::new();

        for source in &self.sources {
            match source {
                DMSConfigSource::File(path) => {
                    self.load_file(path, &mut cfg)?;
                }
                DMSConfigSource::Environment => {
                    self.load_environment(&mut cfg);
                }
            }
        }

        self.config = cfg;
        Ok(())
    }

    /// Creates a new configuration manager with default sources.
    /// 
    /// This method creates a new `DMSConfigManager` with the following default sources:
    /// 1. Configuration files in the `config` directory (dms.yaml, dms.yml, dms.toml, dms.json)
    /// 2. Environment variables with the prefix `DMS_`
    /// 
    /// It also loads the configuration immediately.
    /// 
    /// # Returns
    /// 
    /// A new `DMSConfigManager` instance with default sources and loaded configuration
    pub fn new_default() -> Self {
        let mut manager = Self::new();
        
        // Add default configuration sources
        if let Ok(cwd) = std::env::current_dir() {
            let config_dir = cwd.join("config");
            
            // Add all supported config files in order of priority (lowest to highest)
            manager.add_file_source(config_dir.join("dms.yaml"));
            manager.add_file_source(config_dir.join("dms.yml"));
            manager.add_file_source(config_dir.join("dms.toml"));
            manager.add_file_source(config_dir.join("dms.json"));
        }
        
        // Add environment variables as highest priority
        manager.add_environment_source();
        
        // Load configuration immediately
        let _ = manager.load();
        
        manager
    }

    /// Loads configuration from a file.
    /// 
    /// This method loads configuration from a file, parses it based on its extension,
    /// and flattens it into the provided configuration object.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the configuration file
    /// - `cfg`: The configuration object to load values into
    /// 
    /// # Returns
    /// 
    /// A `Result<(), DMSError>` indicating success or failure
    fn load_file(&self, path: &Path, cfg: &mut DMSConfig) -> Result<(), crate::core::DMSError> {
        if !path.exists() {
            return Ok(());
        }

        let text = fs::read_to_string(path)?;
        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

        match extension.to_lowercase().as_str() {
            "json" => {
                if let Ok(map) = serde_json::from_str::<serde_json::Value>(&text) {
                    self.flatten_json(&map, "", cfg);
                }
            }
            "yaml" | "yml" => {
                if let Ok(yaml_docs) = YamlLoader::load_from_str(&text) {
                    for doc in yaml_docs {
                        self.flatten_yaml(&doc, "", cfg);
                    }
                }
            }
            "toml" => {
                if let Ok(toml) = toml::from_str(&text) {
                    self.flatten_toml(&toml, "", cfg);
                }
            }
            _ => {
                // Ignore unsupported file types
            }
        }

        Ok(())
    }

    /// Flattens a JSON value into the configuration.
    /// 
    /// This method recursively flattens a JSON value into the configuration using dot notation.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The JSON value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_json(&self, value: &serde_json::Value, prefix: &str, cfg: &mut DMSConfig) {
        Self::flatten_json_static(value, prefix, cfg);
    }

    /// Static version of `flatten_json` for recursion.
    /// 
    /// This static method is used for recursion to avoid the "parameter is only used in recursion" warning.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The JSON value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_json_static(value: &serde_json::Value, prefix: &str, cfg: &mut DMSConfig) {
        match value {
            serde_json::Value::Object(map) => {
                for (k, v) in map {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{prefix}.{k}")
                    };
                    Self::flatten_json_static(v, &new_prefix, cfg);
                }
            }
            serde_json::Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{prefix}.{i}");
                    Self::flatten_json_static(v, &new_prefix, cfg);
                }
            }
            serde_json::Value::String(s) => {
                cfg.set(prefix, s);
            }
            serde_json::Value::Number(n) => {
                cfg.set(prefix, n.to_string());
            }
            serde_json::Value::Bool(b) => {
                cfg.set(prefix, b.to_string());
            }
            serde_json::Value::Null => {
                cfg.set(prefix, "");
            }
        }
    }

    /// Flattens a YAML value into the configuration.
    /// 
    /// This method recursively flattens a YAML value into the configuration using dot notation.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The YAML value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_yaml(&self, value: &Yaml, prefix: &str, cfg: &mut DMSConfig) {
        Self::flatten_yaml_static(value, prefix, cfg);
    }

    /// Static version of `flatten_yaml` for recursion.
    /// 
    /// This static method is used for recursion to avoid the "parameter is only used in recursion" warning.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The YAML value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_yaml_static(value: &Yaml, prefix: &str, cfg: &mut DMSConfig) {
        match value {
            Yaml::Hash(map) => {
                for (k, v) in map {
                    if let Yaml::String(key) = k {
                        let new_prefix = if prefix.is_empty() {
                            key.clone()
                        } else {
                            format!("{prefix}.{key}")
                        };
                        Self::flatten_yaml_static(v, &new_prefix, cfg);
                    }
                }
            }
            Yaml::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{prefix}.{i}");
                    Self::flatten_yaml_static(v, &new_prefix, cfg);
                }
            }
            Yaml::String(s) => {
                cfg.set(prefix, s);
            }
            Yaml::Integer(n) => {
                cfg.set(prefix, n.to_string());
            }
            Yaml::Real(r) => {
                cfg.set(prefix, r);
            }
            Yaml::Boolean(b) => {
                cfg.set(prefix, b.to_string());
            }
            Yaml::Null => {
                cfg.set(prefix, "");
            }
            _ => {
                // Ignore other YAML types
            }
        }
    }

    /// Flattens a TOML value into the configuration.
    /// 
    /// This method recursively flattens a TOML value into the configuration using dot notation.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The TOML value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_toml(&self, value: &toml::Value, prefix: &str, cfg: &mut DMSConfig) {
        Self::flatten_toml_static(value, prefix, cfg);
    }

    /// Static version of `flatten_toml` for recursion.
    /// 
    /// This static method is used for recursion to avoid the "parameter is only used in recursion" warning.
    /// 
    /// # Parameters
    /// 
    /// - `value`: The TOML value to flatten
    /// - `prefix`: The current prefix for keys (used for recursion)
    /// - `cfg`: The configuration object to load values into
    fn flatten_toml_static(value: &toml::Value, prefix: &str, cfg: &mut DMSConfig) {
        match value {
            toml::Value::Table(table) => {
                for (k, v) in table {
                    let new_prefix = if prefix.is_empty() {
                        k.clone()
                    } else {
                        format!("{prefix}.{k}")
                    };
                    Self::flatten_toml_static(v, &new_prefix, cfg);
                }
            }
            toml::Value::Array(arr) => {
                for (i, v) in arr.iter().enumerate() {
                    let new_prefix = format!("{prefix}.{i}");
                    Self::flatten_toml_static(v, &new_prefix, cfg);
                }
            }
            toml::Value::String(s) => {
                cfg.set(prefix, s);
            }
            toml::Value::Integer(n) => {
                cfg.set(prefix, n.to_string());
            }
            toml::Value::Float(f) => {
                cfg.set(prefix, f.to_string());
            }
            toml::Value::Boolean(b) => {
                cfg.set(prefix, b.to_string());
            }
            toml::Value::Datetime(dt) => {
                cfg.set(prefix, dt.to_string());
            }
        }
    }

    /// Loads configuration from environment variables.
    /// 
    /// This method loads environment variables with the prefix `DMS_` into the configuration.
    /// Double underscores (`__`) in environment variable names are converted to dots.
    /// 
    /// # Parameters
    /// 
    /// - `cfg`: The configuration object to load values into
    fn load_environment(&self, cfg: &mut DMSConfig) {
        for (name, value) in std::env::vars() {
            if let Some(rest) = name.strip_prefix("DMS_") {
                let key_parts: Vec<String> = rest
                    .split("__")
                    .map(|part| part.to_ascii_lowercase())
                    .collect();
                let key = key_parts.join(".");
                if !key.is_empty() {
                    cfg.set(key, value);
                }
            }
        }
    }

    /// Starts the configuration watcher for hot reload.
    /// 
    /// **Note**: This is a simplified implementation. Full hot reload support
    /// will be implemented in a future update.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), DMSError>` indicating success or failure
    pub async fn start_watcher(&mut self) -> Result<(), crate::core::DMSError> {
        // Watcher implementation is simplified for now
        // Full hot reload support will be implemented in a future update
        Ok(())
    }

    /// Gets a reference to the loaded configuration.
    /// 
    /// # Returns
    /// 
    /// A `&DMSConfig` reference to the loaded configuration
    pub fn config(&self) -> &DMSConfig {
        &self.config
    }

    /// Gets a mutable reference to the loaded configuration.
    /// 
    /// # Returns
    /// 
    /// A `&mut DMSConfig` reference to the loaded configuration
    pub fn config_mut(&mut self) -> &mut DMSConfig {
        &mut self.config
    }
}

#[cfg(feature = "pyo3")]
/// Python constructor for DMSConfigManager
#[pyo3::prelude::pymethods]
impl DMSConfigManager {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Adds a file-based configuration source from Python
    fn add_file_source_py(&mut self, path: String) {
        self.add_file_source(path);
    }
    
    /// Adds environment variables as a configuration source from Python
    fn add_environment_source_py(&mut self) {
        self.add_environment_source();
    }
    
    /// Gets the configuration value as string from Python
    fn get_config_py(&self, key: String) -> Option<String> {
        self.config().get_py(key)
    }
}
