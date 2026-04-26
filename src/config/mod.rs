//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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
//! This module provides a comprehensive configuration management system for Ri, supporting
//! multiple configuration sources, hot reload, and flexible configuration access.
//! 
//! ## Key Components
//! 
//! - **RiConfig**: Basic configuration storage with typed access methods
//! - **RiConfigManager**: Configuration manager that handles multiple sources and hot reload
//! - **RiConfigSource**: Internal enum for different configuration source types
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
//! use ri::prelude::*;
//! 
//! fn example() -> RiResult<()> {
//!     // Create a new config manager
//!     let mut config_manager = RiConfigManager::new();
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

use std::collections::HashMap as FxHashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use tokio::sync::RwLock as TokioRwLock;
use tokio::task::JoinHandle;
use yaml_rust::{YamlLoader, Yaml};

#[cfg(feature = "config_hot_reload")]
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

#[cfg(feature = "pyo3")]
use crate::hooks::RiHookKind;
#[cfg(feature = "pyo3")]
use crate::core::RiServiceContext;

/// Basic configuration storage with typed access methods.
/// 
/// This struct provides a simple key-value store for configuration values, with
/// type-safe methods for accessing values as different types.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct RiConfig {
    /// Internal storage for configuration values
    values: FxHashMap<String, String>,
}

impl Default for RiConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl RiConfig {
    /// Creates a new empty configuration.
    /// 
    /// Returns a new `RiConfig` instance with an empty key-value store.
    pub fn new() -> Self {
        RiConfig { values: FxHashMap::default() }
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

    /// Gets a configuration value as a 64-bit signed integer with bounds checking.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// - `min`: Minimum allowed value
    /// - `max`: Maximum allowed value
    /// 
    /// # Returns
    /// 
    /// An `Option<i64>` containing the parsed and validated integer value
    pub fn get_i64_with_bounds(&self, key: &str, min: i64, max: i64) -> Option<i64> {
        self.get_i64(key).filter(|&v| v >= min && v <= max)
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

    /// Gets a configuration value as a 64-bit unsigned integer with bounds checking.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// - `min`: Minimum allowed value (default: 0)
    /// - `max`: Maximum allowed value
    /// 
    /// # Returns
    /// 
    /// An `Option<u64>` containing the parsed and validated integer value
    pub fn get_u64_with_bounds(&self, key: &str, min: u64, max: u64) -> Option<u64> {
        self.get_u64(key).filter(|&v| v >= min && v <= max)
    }

    /// Gets a configuration value as a positive 64-bit unsigned integer (must be > 0).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<u64>` containing the positive integer value
    pub fn get_positive_u64(&self, key: &str) -> Option<u64> {
        self.get_u64(key).filter(|&v| v > 0)
    }

    /// Gets a configuration value as a port number (1-65535).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<u16>` containing the valid port number
    pub fn get_port(&self, key: &str) -> Option<u16> {
        self.get_u64_with_bounds(key, 1, 65535).map(|v| v as u16)
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

    /// Gets a configuration value as a 32-bit floating point number with bounds checking.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// - `min`: Minimum allowed value
    /// - `max`: Maximum allowed value
    /// 
    /// # Returns
    /// 
    /// An `Option<f32>` containing the parsed and validated float value
    pub fn get_f32_with_bounds(&self, key: &str, min: f32, max: f32) -> Option<f32> {
        self.get_f32(key).filter(|&v| v >= min && v <= max)
    }

    /// Gets a configuration value as a percentage (0.0-100.0).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<f32>` containing the percentage value
    pub fn get_percentage(&self, key: &str) -> Option<f32> {
        self.get_f32_with_bounds(key, 0.0, 100.0)
    }

    /// Gets a configuration value as a rate (0.0-1.0).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<f32>` containing the rate value
    pub fn get_rate(&self, key: &str) -> Option<f32> {
        self.get_f32_with_bounds(key, 0.0, 1.0)
    }

    /// Merges another configuration into this one.
    /// 
    /// Values from the other configuration will override existing values with the same keys.
    /// 
    /// # Parameters
    /// 
    /// - `other`: The other configuration to merge into this one
    pub fn merge(&mut self, other: &RiConfig) {
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

    pub fn get_or_default<T>(&self, key: &str, default: T) -> T 
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Debug,
    {
        self.values.get(key).and_then(|s| s.trim().parse::<T>().ok()).unwrap_or(default)
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|s| s.trim().parse::<f64>().ok())
    }

    pub fn get_usize(&self, key: &str) -> Option<usize> {
        self.values.get(key).and_then(|s| s.trim().parse::<usize>().ok())
    }

    pub fn get_i32(&self, key: &str) -> Option<i32> {
        self.values.get(key).and_then(|s| s.trim().parse::<i32>().ok())
    }

    pub fn get_u32(&self, key: &str) -> Option<u32> {
        self.values.get(key).and_then(|s| s.trim().parse::<u32>().ok())
    }

    /// Gets a configuration value as a 32-bit unsigned integer with bounds checking.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// - `min`: Minimum allowed value
    /// - `max`: Maximum allowed value
    /// 
    /// # Returns
    /// 
    /// An `Option<u32>` containing the parsed and validated integer value
    pub fn get_u32_with_bounds(&self, key: &str, min: u32, max: u32) -> Option<u32> {
        self.get_u32(key).filter(|&v| v >= min && v <= max)
    }

    /// Gets a configuration value as a timeout value in seconds (1-86400).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<u32>` containing the timeout in seconds
    pub fn get_timeout_secs(&self, key: &str) -> Option<u32> {
        self.get_u32_with_bounds(key, 1, 86400)
    }

    /// Gets a configuration value as a retry count (0-100).
    /// 
    /// # Parameters
    /// 
    /// - `key`: The configuration key to look up
    /// 
    /// # Returns
    /// 
    /// An `Option<u32>` containing the retry count
    pub fn get_retry_count(&self, key: &str) -> Option<u32> {
        self.get_u32_with_bounds(key, 0, 100)
    }

    pub fn keys(&self) -> Vec<&str> {
        self.values.keys().map(|s| s.as_str()).collect()
    }

    pub fn all_values(&self) -> Vec<&str> {
        self.values.values().map(|s| s.as_str()).collect()
    }

    pub fn has_key(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn count(&self) -> usize {
        self.values.len()
    }

    #[cfg(feature = "pyo3")]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

#[cfg(feature = "pyo3")]
/// Python constructor for RiConfig
#[pyo3::prelude::pymethods]
impl RiConfig {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    #[pyo3(name = "set")]
    fn set_impl(&mut self, key: String, value: String) {
        self.set(key, value);
    }
    
    #[pyo3(name = "get")]
    fn get_impl(&self, key: String) -> Option<String> {
        self.get(&key).cloned()
    }
    
    #[pyo3(name = "get_f64")]
    fn get_f64_impl(&self, key: String) -> Option<f64> {
        self.get_f64(&key)
    }
    
    #[pyo3(name = "get_usize")]
    fn get_usize_impl(&self, key: String) -> Option<usize> {
        self.get_usize(&key)
    }
    
    #[pyo3(name = "keys")]
    fn py_keys(&self) -> Vec<String> {
        self.keys().iter().map(|s| s.to_string()).collect()
    }
    
    #[pyo3(name = "values")]
    fn py_values(&self) -> Vec<String> {
        self.all_values().iter().map(|s| s.to_string()).collect()
    }
    
    #[pyo3(name = "contains")]
    fn py_contains(&self, key: String) -> bool {
        self.has_key(&key)
    }
    
    #[pyo3(name = "len")]
    fn py_len(&self) -> usize {
        self.count()
    }
}

/// Internal enum for different configuration source types.
/// 
/// This enum represents the different types of configuration sources that the
/// `RiConfigManager` can handle.
#[derive(Clone)]
enum RiConfigSource {
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
pub struct RiConfigManager {
    /// Internal configuration storage
    config: Arc<RwLock<RiConfig>>,
    /// List of configuration sources to load from
    sources: Vec<RiConfigSource>,
    /// Optional hook bus for emitting config reload events
    #[cfg(feature = "pyo3")]
    hooks: Option<Arc<crate::hooks::RiHookBus>>,
    /// Hot reload watcher handle
    #[cfg(feature = "config_hot_reload")]
    watcher: Option<Arc<RecommendedWatcher>>,
    /// Background task handle for the watcher
    watcher_task: Arc<TokioRwLock<Option<JoinHandle<()>>>>,
    /// Monitored file paths for hot reload
    monitored_paths: Arc<TokioRwLock<Vec<PathBuf>>>,
    /// Callback for config changes
    #[cfg(feature = "config_hot_reload")]
    change_callback: Option<Arc<dyn Fn() + Send + Sync>>,
}

impl Default for RiConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for RiConfigManager {
    fn clone(&self) -> Self {
        RiConfigManager {
            config: self.config.clone(),
            sources: self.sources.clone(),
            #[cfg(feature = "pyo3")]
            hooks: self.hooks.clone(),
            #[cfg(feature = "config_hot_reload")]
            watcher: self.watcher.clone(),
            watcher_task: self.watcher_task.clone(),
            monitored_paths: self.monitored_paths.clone(),
            #[cfg(feature = "config_hot_reload")]
            change_callback: self.change_callback.clone(),
        }
    }
}

impl RiConfigManager {
    /// Creates a new empty configuration manager.
    /// 
    /// Returns a new `RiConfigManager` instance with no configuration sources.
    pub fn new() -> Self {
        RiConfigManager {
            config: Arc::new(RwLock::new(RiConfig::new())),
            sources: Vec::new(),
            #[cfg(feature = "pyo3")]
            hooks: None,
            #[cfg(feature = "config_hot_reload")]
            watcher: None,
            watcher_task: Arc::new(TokioRwLock::new(None)),
            monitored_paths: Arc::new(TokioRwLock::new(Vec::new())),
            #[cfg(feature = "config_hot_reload")]
            change_callback: None,
        }
    }

    /// Creates a new configuration manager with the provided hook bus.
    /// 
    /// This method allows the config manager to emit hooks when configuration is reloaded.
    /// 
    /// # Parameters
    /// 
    /// - `hooks`: The hook bus to use for emitting events
    /// 
    /// # Returns
    /// 
    /// A new `RiConfigManager` instance with the provided hook bus
    #[cfg(feature = "pyo3")]
    pub fn with_hooks(hooks: Arc<crate::hooks::RiHookBus>) -> Self {
        RiConfigManager {
            config: Arc::new(RwLock::new(RiConfig::new())),
            sources: Vec::new(),
            hooks: Some(hooks),
            #[cfg(feature = "config_hot_reload")]
            watcher: None,
            watcher_task: Arc::new(TokioRwLock::new(None)),
            monitored_paths: Arc::new(TokioRwLock::new(Vec::new())),
            #[cfg(feature = "config_hot_reload")]
            change_callback: None,
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
        self.sources.push(RiConfigSource::File(path.as_ref().to_path_buf()));
    }

    /// Adds environment variables as a configuration source.
    /// 
    /// Environment variables with the prefix `Ri_` are loaded as configuration values.
    /// Double underscores (`__`) in environment variable names are converted to dots.
    /// For example, `Ri_SERVER__PORT=8080` becomes `server.port=8080`.
    pub fn add_environment_source(&mut self) {
        self.sources.push(RiConfigSource::Environment);
    }

    /// Notifies registered hooks when configuration is reloaded.
    #[cfg(feature = "pyo3")]
    fn notify_config_reload(&self, _path: &str) {
        if let Some(hooks) = &self.hooks {
            let _ = hooks.emit_with(
                &RiHookKind::ConfigReload,
                &RiServiceContext::new_default().unwrap_or_else(|_| {
                    RiServiceContext::new_with(
                        crate::fs::RiFileSystem::new_auto_root().unwrap_or_else(|_| crate::fs::RiFileSystem::new_with_root(std::env::current_dir().unwrap_or_default())),
                        crate::log::RiLogger::new(&crate::log::RiLogConfig::default(), crate::fs::RiFileSystem::new_with_root(std::env::current_dir().unwrap_or_default())),
                        crate::config::RiConfigManager::new(),
                        crate::hooks::RiHookBus::new(),
                        None,
                    )
                }),
                Some("config_manager"),
                None,
            );
        }
    }

    /// Loads configuration from all registered sources.
    /// 
    /// This method loads configuration from all registered sources in the order they were added,
    /// with later sources overriding earlier ones.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    pub fn load(&mut self) -> Result<(), crate::core::RiError> {
        let mut cfg = RiConfig::new();

        for source in &self.sources {
            match source {
                RiConfigSource::File(path) => {
                    self.load_file(path, &mut cfg)?;
                    #[cfg(feature = "pyo3")]
                    self.notify_config_reload(path.to_str().unwrap_or(""));
                }
                RiConfigSource::Environment => {
                    self.load_environment(&mut cfg);
                }
            }
        }

        *self.config.write().expect("Failed to lock config for writing") = cfg;
        
        Ok(())
    }

    /// Creates a new configuration manager with default sources.
    /// 
    /// This method creates a new `RiConfigManager` with the following default sources:
    /// 1. Configuration files in the `config` directory (dms.yaml, dms.yml, dms.toml, dms.json)
    /// 2. Environment variables with the prefix `Ri_`
    /// 
    /// It also loads the configuration immediately.
    /// 
    /// # Returns
    /// 
    /// A new `RiConfigManager` instance with default sources and loaded configuration
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
    /// A `Result<(), RiError>` indicating success or failure
    fn load_file(&self, path: &Path, cfg: &mut RiConfig) -> Result<(), crate::core::RiError> {
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
    fn flatten_json(&self, value: &serde_json::Value, prefix: &str, cfg: &mut RiConfig) {
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
    fn flatten_json_static(value: &serde_json::Value, prefix: &str, cfg: &mut RiConfig) {
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
    fn flatten_yaml(&self, value: &Yaml, prefix: &str, cfg: &mut RiConfig) {
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
    fn flatten_yaml_static(value: &Yaml, prefix: &str, cfg: &mut RiConfig) {
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
    fn flatten_toml(&self, value: &toml::Value, prefix: &str, cfg: &mut RiConfig) {
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
    fn flatten_toml_static(value: &toml::Value, prefix: &str, cfg: &mut RiConfig) {
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
    /// This method loads environment variables with the prefix `Ri_` into the configuration.
    /// Double underscores (`__`) in environment variable names are converted to dots.
    /// 
    /// # Parameters
    /// 
    /// - `cfg`: The configuration object to load values into
    fn load_environment(&self, cfg: &mut RiConfig) {
        for (name, value) in std::env::vars() {
            if let Some(rest) = name.strip_prefix("Ri_") {
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
    /// This method starts watching all registered file-based configuration sources
    /// for changes. When a configuration file is modified, it will be automatically
    /// reloaded and the change callback (if registered) will be invoked.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    #[cfg(feature = "config_hot_reload")]
    pub async fn start_watcher(&mut self) -> Result<(), crate::core::RiError> {
        self.start_watcher_with_callback::<fn()>(None).await
    }

    /// Starts the configuration watcher with a custom change callback.
    /// 
    /// This method starts watching all registered file-based configuration sources
    /// for changes. When a configuration file is modified, it will be automatically
    /// reloaded and the provided callback will be invoked.
    /// 
    /// # Parameters
    /// 
    /// - `callback`: Optional callback function to invoke when configuration changes
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    #[cfg(feature = "config_hot_reload")]
    pub async fn start_watcher_with_callback<F>(&mut self, callback: Option<Arc<dyn Fn() + Send + Sync>>) -> Result<(), crate::core::RiError> {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<notify::Result<notify::Event>>(100);
        
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.blocking_send(res);
            },
            notify::Config::default(),
        ).map_err(|e| crate::core::RiError::Config(format!("Failed to create config watcher: {}", e)))?;
        
        let mut monitored = Vec::with_capacity(self.sources.len());
        
        for source in &self.sources {
            if let RiConfigSource::File(path) = source {
                if path.exists() {
                    watcher.watch(path, RecursiveMode::NonRecursive)
                        .map_err(|e| crate::core::RiError::Config(format!("Failed to watch config file {}: {}", path.display(), e)))?;
                    monitored.push(path.clone());
                }
            }
        }
        
        let monitored_paths = self.monitored_paths.clone();
        let manager = self.clone();
        let change_callback = callback.clone();
        
        let task = tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                match event {
                    Ok(event) => {
                        if let Some(paths) = event.paths.first() {
                            let changed_path = paths.clone();
                            
                            {
                                let mut paths_guard = monitored_paths.write().await;
                                if !paths_guard.contains(&changed_path) {
                                    paths_guard.push(changed_path.clone());
                                }
                            }
                            
                            log::info!("Config file changed: {}", changed_path.display());
                            
                            if let Err(e) = manager.reload_file(&changed_path).await {
                                log::error!("Failed to reload config file {}: {}", changed_path.display(), e);
                            }
                            
                            if let Some(ref cb) = change_callback {
                                cb();
                            }
                            
                            #[cfg(feature = "pyo3")]
                            manager.notify_config_reload(changed_path.to_str().unwrap_or(""));
                        }
                    }
                    Err(e) => {
                        log::warn!("Config watcher error: {:?}", e);
                    }
                }
            }
        });
        
        self.watcher = Some(Arc::new(watcher));
        *self.watcher_task.write().await = Some(task);
        self.change_callback = callback;
        
        let mut paths_guard = self.monitored_paths.write().await;
        *paths_guard = monitored;
        
        Ok(())
    }

    /// Reloads configuration from a specific file.
    /// 
    /// # Parameters
    /// 
    /// - `path`: The path to the configuration file to reload
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    #[cfg(feature = "config_hot_reload")]
    async fn reload_file(&self, path: &PathBuf) -> Result<(), crate::core::RiError> {
        let mut new_config = self.config.read().expect("Failed to lock config for reading").clone();
        self.load_file(path, &mut new_config)?;
        
        *self.config.write().expect("Failed to lock config for writing") = new_config;
        
        Ok(())
    }

    /// Stops the configuration watcher.
    /// 
    /// This method stops the configuration watcher and cleans up associated resources.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    #[cfg(feature = "config_hot_reload")]
    pub async fn stop_watcher(&mut self) -> Result<(), crate::core::RiError> {
        let task = self.watcher_task.write().await.take();
        if let Some(task) = task {
            task.abort();
        }
        
        self.watcher = None;
        
        let mut paths_guard = self.monitored_paths.write().await;
        paths_guard.clear();
        
        Ok(())
    }

    /// Gets the list of monitored configuration file paths.
    /// 
    /// # Returns
    /// 
    /// A vector of paths being monitored for changes
    pub async fn get_monitored_paths(&self) -> Vec<PathBuf> {
        self.monitored_paths.read().await.clone()
    }

    /// Starts the configuration watcher for hot reload.
    /// 
    /// This is a no-op implementation when the `config_hot_reload` feature is not enabled.
    /// 
    /// # Returns
    /// 
    /// A `Result<(), RiError>` indicating success or failure
    #[cfg(not(feature = "config_hot_reload"))]
    pub async fn start_watcher(&mut self) -> Result<(), crate::core::RiError> {
        Ok(())
    }

    /// Gets a reference to the loaded configuration.
    /// 
    /// # Returns
    /// 
    /// A `RiConfig` clone of the loaded configuration
    pub fn config(&self) -> RiConfig {
        self.config.read().expect("Failed to lock config for reading").clone()
    }

    /// Gets a mutable reference to the loaded configuration.
    /// 
    /// # Returns
    /// 
    /// A `std::sync::RwLockWriteGuard<RiConfig>` for the loaded configuration
    pub fn config_mut(&mut self) -> std::sync::RwLockWriteGuard<'_, RiConfig> {
        self.config.write().expect("Failed to lock config for writing")
    }
}

#[cfg(feature = "pyo3")]
/// Python constructor for RiConfigManager
#[pyo3::prelude::pymethods]
impl RiConfigManager {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Adds a file-based configuration source from Python
    ///
    /// ## Supported Formats
    ///
    /// - `.json`: JSON configuration format
    /// - `.yaml`, `.yml`: YAML configuration format
    /// - `.toml`: TOML configuration format
    #[pyo3(name = "add_file_source")]
    fn add_file_source_impl(&mut self, path: String) {
        self.add_file_source(path);
    }

    /// Adds environment variables as a configuration source from Python
    ///
    /// Environment variables are prefixed with `Ri_` and double underscores
    /// are converted to dots (`.`) in the configuration key hierarchy.
    /// Example: `Ri_DATABASE__HOST` becomes `database.host`
    #[pyo3(name = "add_environment_source")]
    fn add_environment_source_impl(&mut self) {
        self.add_environment_source();
    }

    /// Gets a configuration value as string from Python
    ///
    /// This method retrieves a configuration value by key, returning it as a string.
    /// If the key does not exist, returns `None`.
    ///
    /// ## Parameters
    ///
    /// - `key`: Configuration key string (dot-notation supported, e.g., "server.port")
    ///
    /// ## Returns
    ///
    /// Optional string containing the configuration value if found, `None` otherwise
    ///
    /// ## Example
    ///
    /// ```python
    /// manager = RiConfigManager.new_default()
    /// value = manager.get("server.port")
    /// if value:
    ///     print(f"Server port: {value}")
    /// ```
    #[pyo3(name = "get")]
    fn get_config_impl(&self, key: String) -> Option<String> {
        self.config().get(&key).cloned()
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone, Debug)]
pub struct RiConfigValidator {
    required_keys: Vec<String>,
    port_keys: Vec<String>,
    timeout_keys: Vec<String>,
    secret_keys: Vec<String>,
    url_keys: Vec<String>,
    positive_int_keys: Vec<String>,
}

impl Default for RiConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl RiConfigValidator {
    pub fn new() -> Self {
        RiConfigValidator {
            required_keys: Vec::new(),
            port_keys: vec!["server.port".to_string(), "cache.redis.port".to_string(), "database.port".to_string()],
            timeout_keys: vec!["server.timeout".to_string(), "cache.ttl".to_string(), "session.timeout".to_string()],
            secret_keys: vec!["auth.jwt.secret".to_string(), "auth.password.salt".to_string(), "encryption.key".to_string()],
            url_keys: vec!["database.url".to_string(), "cache.redis.url".to_string(), "mq.url".to_string()],
            positive_int_keys: vec!["pool.size".to_string(), "worker.count".to_string(), "retry.max".to_string()],
        }
    }

    pub fn add_required(&mut self, key: String) -> &mut Self {
        self.required_keys.push(key);
        self
    }

    pub fn add_port_check(&mut self, key: String) -> &mut Self {
        self.port_keys.push(key);
        self
    }

    pub fn add_timeout_check(&mut self, key: String) -> &mut Self {
        self.timeout_keys.push(key);
        self
    }

    pub fn add_secret_check(&mut self, key: String) -> &mut Self {
        self.secret_keys.push(key);
        self
    }

    pub fn add_url_check(&mut self, key: String) -> &mut Self {
        self.url_keys.push(key);
        self
    }

    pub fn add_positive_int_check(&mut self, key: String) -> &mut Self {
        self.positive_int_keys.push(key);
        self
    }

    pub fn validate_config(&self, config: &RiConfig) -> Result<(), crate::core::RiError> {
        for key in &self.required_keys {
            if !config.has_key(key) {
                return Err(crate::core::RiError::Config(format!(
                    "Missing required configuration key: {}", key
                )));
            }
        }

        for key in &self.port_keys {
            if let Some(port) = config.get_port(key) {
                if port == 0 {
                    return Err(crate::core::RiError::Config(format!(
                        "Invalid port number for {}: must be between 1 and 65535", key
                    )));
                }
            }
        }

        for key in &self.timeout_keys {
            if let Some(timeout) = config.get_timeout_secs(key) {
                if timeout == 0 {
                    return Err(crate::core::RiError::Config(format!(
                        "Invalid timeout for {}: must be between 1 and 86400 seconds", key
                    )));
                }
            }
        }

        for key in &self.secret_keys {
            if let Some(secret) = config.get_str(key) {
                if secret.len() < 8 {
                    return Err(crate::core::RiError::Config(format!(
                        "Secret key {} is too short: minimum length is 8 characters", key
                    )));
                }
                if secret == "secret" || secret == "password" || secret == "123456" {
                    return Err(crate::core::RiError::Config(format!(
                        "Insecure secret key detected for {}: using default or weak value", key
                    )));
                }
            }
        }

        for key in &self.url_keys {
            if let Some(url) = config.get_str(key) {
                if !url.starts_with("http://") && !url.starts_with("https://")
                    && !url.starts_with("redis://") && !url.starts_with("postgresql://")
                    && !url.starts_with("mysql://") && !url.starts_with("amqp://")
                    && !url.starts_with("kafka://") && !url.starts_with("sqlite://")
                {
                    return Err(crate::core::RiError::Config(format!(
                        "Invalid URL format for {}: {}", key, url
                    )));
                }
            }
        }

        for key in &self.positive_int_keys {
            if let Some(value) = config.get_u32(key) {
                if value == 0 {
                    return Err(crate::core::RiError::Config(format!(
                        "Invalid value for {}: must be a positive integer", key
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn validate_with_requirements(
        &self,
        config: &RiConfig,
        requirements: &[String],
    ) -> Result<(), crate::core::RiError> {
        for key in requirements {
            if !config.has_key(key) {
                return Err(crate::core::RiError::Config(format!(
                    "Missing required configuration key: {}", key
                )));
            }
        }
        self.validate_config(config)
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiConfigValidator {
    #[new]
    fn py_new() -> Self {
        Self::new()
    }

    #[pyo3(name = "require")]
    fn py_add_required(&mut self, key: String) {
        self.required_keys.push(key);
    }

    #[pyo3(name = "require_port")]
    fn py_add_port_check(&mut self, key: String) {
        self.port_keys.push(key);
    }

    #[pyo3(name = "require_timeout")]
    fn py_add_timeout_check(&mut self, key: String) {
        self.timeout_keys.push(key);
    }

    #[pyo3(name = "require_secret")]
    fn py_add_secret_check(&mut self, key: String) {
        self.secret_keys.push(key);
    }

    #[pyo3(name = "require_url")]
    fn py_add_url_check(&mut self, key: String) {
        self.url_keys.push(key);
    }

    #[pyo3(name = "require_positive_int")]
    fn py_add_positive_int_check(&mut self, key: String) {
        self.positive_int_keys.push(key);
    }

    #[pyo3(name = "validate")]
    fn py_validate(&self, config: &RiConfig) -> bool {
        self.validate_config(config).is_ok()
    }
}
