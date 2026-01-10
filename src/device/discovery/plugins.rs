//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

//! # Custom Discovery Plugins
//!
//! This module provides a plugin system for custom hardware discovery implementations.
//! Users can create their own discovery plugins to support specialized hardware or
//! proprietary device interfaces.
//!
//! ## Architecture
//!
//! - **DMSCHardwareDiscoveryPlugin**: Trait for plugin implementations
//! - **PluginRegistry**: Manages registered plugins
//! - **PluginMetadata**: Plugin information and configuration
//!
//! ## Usage
//!
//! ```rust
//! use dms::device::discovery::plugins::{DMSCHardwareDiscoveryPlugin, PluginRegistry};
//!
//! // Create a custom plugin
//! struct MyCustomPlugin;
//!
//! #[async_trait::async_trait]
//! impl DMSCHardwareDiscoveryPlugin for MyCustomPlugin {
//!     fn name(&self) -> &str { "MyCustomPlugin" }
//!     fn version(&self) -> &str { "1.0.0" }
//!     async fn discover(&self) -> Result<Vec<DMSCDevice>, String> {
//!         // Custom discovery logic
//!         Ok(vec![])
//!     }
//! }
//!
//! // Register the plugin
//! let mut registry = PluginRegistry::new();
//! registry.register(Box::new(MyCustomPlugin));
//! ```

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use libloading::{Library, Symbol};
use thiserror::Error as ThisError;

use super::super::core::DMSCDevice;
use super::platform::{PlatformInfo, HardwareCategory};

/// Result type for plugin discovery operations
pub type PluginResult<T> = Result<T, PluginError>;

/// Errors that can occur during plugin operations
#[derive(Debug, Clone, Serialize, Deserialize, ThisError)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum PluginError {
    #[error("Plugin load failed: {0}")]
    LoadFailed(String),

    #[error("Plugin initialization failed: {0}")]
    InitFailed(String),

    #[error("Plugin discovery failed: {0}")]
    DiscoveryFailed(String),

    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already registered: {0}")]
    AlreadyRegistered(String),

    #[error("Unsupported platform for plugin")]
    UnsupportedPlatform(),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Library load failed: {0}")]
    LibraryLoadFailed(String),

    #[error("Symbol resolution failed: {0}")]
    SymbolResolutionFailed(String),

    #[error("Library unload failed: {0}")]
    LibraryUnloadFailed(String),

    #[error("Plugin version mismatch: {0}")]
    VersionMismatch(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for PluginError {
    fn from(e: std::io::Error) -> Self {
        PluginError::LoadFailed(format!("IO error: {}", e))
    }
}

/// Plugin metadata for identification and configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct PluginMetadata {
    /// Plugin name
    pub name: String,
    /// Plugin version
    pub version: String,
    /// Plugin author
    pub author: String,
    /// Plugin description
    pub description: String,
    /// Supported hardware categories
    pub supported_categories: Vec<HardwareCategory>,
    /// Minimum platform version requirement
    pub min_platform_version: Option<String>,
    /// Plugin configuration schema (JSON)
    pub config_schema: Option<String>,
    /// Dependencies
    pub dependencies: Vec<String>,
    /// Whether the plugin is enabled by default
    pub enabled_by_default: bool,
}

impl PluginMetadata {
    /// Creates new plugin metadata
    pub fn new(
        name: String,
        version: String,
        author: String,
        description: String,
    ) -> Self {
        Self {
            name,
            version,
            author,
            description,
            supported_categories: Vec::new(),
            min_platform_version: None,
            config_schema: None,
            dependencies: Vec::new(),
            enabled_by_default: true,
        }
    }

    /// Adds a supported hardware category
    pub fn with_category(mut self, category: HardwareCategory) -> Self {
        self.supported_categories.push(category);
        self
    }

    /// Sets the minimum platform version
    pub fn with_min_platform_version(mut self, version: String) -> Self {
        self.min_platform_version = Some(version);
        self
    }

    /// Sets the configuration schema
    pub fn with_config_schema(mut self, schema: &str) -> Self {
        self.config_schema = Some(schema.to_string());
        self
    }

    /// Adds a dependency
    pub fn with_dependency(mut self, dep: String) -> Self {
        self.dependencies.push(dep);
        self
    }

    /// Sets whether the plugin is enabled by default
    pub fn with_enabled_by_default(mut self, enabled: bool) -> Self {
        self.enabled_by_default = enabled;
        self
    }
}

/// Trait for custom hardware discovery plugins
#[async_trait]
pub trait DMSCHardwareDiscoveryPlugin: Send + Sync {
    /// Returns the plugin metadata
    fn metadata(&self) -> PluginMetadata;

    /// Initializes the plugin with configuration
    async fn initialize(&mut self, config: &str) -> PluginResult<()> {
        let _ = config;
        Ok(())
    }

    /// Discovers hardware devices
    async fn discover(&self, platform: &PlatformInfo) -> PluginResult<Vec<DMSCDevice>>;

    /// Called when the plugin is being unloaded
    async fn shutdown(&mut self) -> PluginResult<()> {
        Ok(())
    }

    /// Returns the current plugin status
    fn status(&self) -> PluginStatus;
}

/// Current status of a plugin
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum PluginStatus {
    Loaded(),
    Ready(),
    Discovering(),
    Disabled(),
    Error(String),
    ShuttingDown(),
}

impl std::fmt::Display for PluginStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginStatus::Loaded() => write!(f, "Plugin is loaded but not initialized"),
            PluginStatus::Ready() => write!(f, "Plugin is initialized and ready"),
            PluginStatus::Discovering() => write!(f, "Plugin is currently discovering"),
            PluginStatus::Disabled() => write!(f, "Plugin has been disabled"),
            PluginStatus::Error(msg) => write!(f, "Plugin encountered an error: {}", msg),
            PluginStatus::ShuttingDown() => write!(f, "Plugin is being unloaded"),
        }
    }
}

/// Plugin wrapper for runtime management
#[allow(dead_code)]
struct PluginWrapper {
    metadata: PluginMetadata,
    plugin: Arc<RwLock<Box<dyn DMSCHardwareDiscoveryPlugin>>>,
    status: Arc<RwLock<PluginStatus>>,
    config: Arc<RwLock<Option<String>>>,
    load_time: std::time::SystemTime,
    library: Option<Arc<Library>>,
}

impl PluginWrapper {
    pub fn new(plugin: Box<dyn DMSCHardwareDiscoveryPlugin>) -> Self {
        let metadata = plugin.metadata();
        let status = plugin.status();

        Self {
            metadata,
            plugin: Arc::new(RwLock::new(plugin)),
            status: Arc::new(RwLock::new(status)),
            config: Arc::new(RwLock::new(None)),
            load_time: std::time::SystemTime::now(),
            library: None,
        }
    }

    pub fn with_library(plugin: Box<dyn DMSCHardwareDiscoveryPlugin>, library: Arc<Library>) -> Self {
        let metadata = plugin.metadata();
        let status = plugin.status();

        Self {
            metadata,
            plugin: Arc::new(RwLock::new(plugin)),
            status: Arc::new(RwLock::new(status)),
            config: Arc::new(RwLock::new(None)),
            load_time: std::time::SystemTime::now(),
            library: Some(library),
        }
    }

    pub async fn initialize(&self, config: &str) -> PluginResult<()> {
        let mut plugin = self.plugin.write().await;
        plugin.initialize(config).await?;
        *self.config.write().await = Some(config.to_string());
        *self.status.write().await = PluginStatus::Ready();
        Ok(())
    }

    pub async fn discover(&self, platform: &PlatformInfo) -> PluginResult<Vec<DMSCDevice>> {
        *self.status.write().await = PluginStatus::Discovering();
        let result = self.plugin.read().await.discover(platform).await;
        *self.status.write().await = PluginStatus::Ready();
        result
    }

    pub async fn shutdown(&self) -> PluginResult<()> {
        *self.status.write().await = PluginStatus::ShuttingDown();
        let mut plugin = self.plugin.write().await;
        plugin.shutdown().await?;
        *self.status.write().await = PluginStatus::Loaded();
        Ok(())
    }

    pub fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    #[allow(dead_code)]
    pub async fn status(&self) -> PluginStatus {
        self.status.read().await.clone()
    }

    #[allow(dead_code)]
    pub fn is_dynamic(&self) -> bool {
        self.library.is_some()
    }
}

/// Plugin registry for managing custom discovery plugins
#[derive(Default)]
pub struct PluginRegistry {
    plugins: Arc<RwLock<HashMap<String, PluginWrapper>>>,
    enabled: Arc<RwLock<HashSet<String>>>,
}

impl PluginRegistry {
    /// Creates a new plugin registry
    pub fn new() -> Self {
        Self {
            plugins: Arc::new(RwLock::new(HashMap::new())),
            enabled: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    /// Registers a new plugin
    pub async fn register(&mut self, plugin: Box<dyn DMSCHardwareDiscoveryPlugin>) -> PluginResult<String> {
        let metadata = plugin.metadata();
        let name = metadata.name.clone();

        let mut plugins = self.plugins.write().await;
        let mut enabled = self.enabled.write().await;

        if plugins.contains_key(&name) {
            return Err(PluginError::AlreadyRegistered(name));
        }

        let wrapper = PluginWrapper::new(plugin);
        plugins.insert(name.clone(), wrapper);
        if metadata.enabled_by_default {
            enabled.insert(name.clone());
        }

        Ok(name)
    }

    /// Unregisters a plugin
    pub async fn unregister(&mut self, name: &str) -> PluginResult<()> {
        let mut plugins = self.plugins.write().await;
        let mut enabled = self.enabled.write().await;

        if let Some(wrapper) = plugins.remove(name) {
            wrapper.shutdown().await?;
            enabled.remove(name);
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Initializes a plugin with configuration
    pub async fn initialize(&self, name: &str, config: &str) -> PluginResult<()> {
        let plugins = self.plugins.read().await;
        if let Some(wrapper) = plugins.get(name) {
            wrapper.initialize(config).await
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Discovers devices using a specific plugin
    pub async fn discover_with_plugin(
        &self,
        name: &str,
        platform: &PlatformInfo,
    ) -> PluginResult<Vec<DMSCDevice>> {
        let enabled = self.enabled.read().await;
        if !enabled.contains(name) {
            return Err(PluginError::NotFound(name.to_string()));
        }

        let plugins = self.plugins.read().await;
        if let Some(wrapper) = plugins.get(name) {
            wrapper.discover(platform).await
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Discovers devices using all enabled plugins
    pub async fn discover_all(&self, platform: &PlatformInfo) -> PluginResult<Vec<DMSCDevice>> {
        let enabled = self.enabled.read().await;
        let plugins = self.plugins.read().await;

        let mut all_devices = Vec::new();
        for name in enabled.iter() {
            if let Some(wrapper) = plugins.get(name) {
                match wrapper.discover(platform).await {
                    Ok(devices) => all_devices.extend(devices),
                    Err(e) => tracing::warn!("Plugin {} discovery failed: {}", name, e),
                }
            }
        }

        Ok(all_devices)
    }

    /// Enables a plugin
    pub async fn enable(&self, name: &str) -> PluginResult<()> {
        let plugins = self.plugins.read().await;
        if plugins.contains_key(name) {
            self.enabled.write().await.insert(name.to_string());
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Disables a plugin
    pub async fn disable(&self, name: &str) -> PluginResult<()> {
        let plugins = self.plugins.read().await;
        if plugins.contains_key(name) {
            self.enabled.write().await.remove(name);
            Ok(())
        } else {
            Err(PluginError::NotFound(name.to_string()))
        }
    }

    /// Checks if a plugin is enabled
    pub async fn is_enabled(&self, name: &str) -> bool {
        self.enabled.read().await.contains(name)
    }

    /// Returns all registered plugin names
    pub async fn registered_plugins(&self) -> Vec<String> {
        self.plugins.read().await.keys().cloned().collect()
    }

    /// Returns all enabled plugin names
    pub async fn enabled_plugins(&self) -> Vec<String> {
        self.enabled.read().await.iter().cloned().collect()
    }

    /// Returns plugin metadata
    pub async fn plugin_metadata(&self, name: &str) -> Option<PluginMetadata> {
        self.plugins.read().await.get(name).map(|w| w.metadata().clone())
    }

    /// Returns the count of registered plugins
    pub async fn count(&self) -> usize {
        self.plugins.read().await.len()
    }

    /// Returns the count of enabled plugins
    pub async fn enabled_count(&self) -> usize {
        self.enabled.read().await.len()
    }
}

/// Plugin loader for dynamic plugin loading
#[derive(Default)]
pub struct PluginLoader {
    search_paths: Arc<RwLock<Vec<PathBuf>>>,
}

impl PluginLoader {
    /// Creates a new plugin loader
    pub fn new() -> Self {
        let mut search_paths = Vec::new();
        search_paths.push(PathBuf::from("./plugins"));
        search_paths.push(PathBuf::from("/usr/local/lib/dmsc/plugins"));
        #[cfg(target_os = "macos")]
        search_paths.push(PathBuf::from("/opt/homebrew/lib/dmsc/plugins"));

        Self {
            search_paths: Arc::new(RwLock::new(search_paths)),
        }
    }

    /// Creates a new plugin loader with custom search paths
    pub fn with_paths(paths: Vec<PathBuf>) -> Self {
        Self {
            search_paths: Arc::new(RwLock::new(paths)),
        }
    }

    /// Adds a search path for plugins
    pub async fn add_search_path(&self, path: PathBuf) {
        self.search_paths.write().await.push(path);
    }

    /// Gets all search paths
    pub async fn search_paths(&self) -> Vec<PathBuf> {
        self.search_paths.read().await.clone()
    }

    /// Clears all search paths
    pub async fn clear_search_paths(&self) {
        self.search_paths.write().await.clear();
    }

    /// Loads plugins from all search paths
    pub async fn load_all(&self, registry: &mut PluginRegistry) -> PluginResult<Vec<String>> {
        let mut loaded = Vec::new();
        let paths = self.search_paths.read().await;

        for path in paths.iter() {
            match self.load_plugins_from_path(path, registry).await {
                Ok(loaded_plugins) => loaded.extend(loaded_plugins),
                Err(e) => tracing::warn!("Failed to load plugins from {}: {}", path.display(), e),
            }
        }

        Ok(loaded)
    }

    /// Loads plugins from a specific directory path
    async fn load_plugins_from_path(&self, path: &PathBuf, registry: &mut PluginRegistry) -> PluginResult<Vec<String>> {
        let mut loaded = Vec::new();

        if !path.exists() {
            tracing::debug!("Plugin path does not exist: {}", path.display());
            return Ok(loaded);
        }

        if !path.is_dir() {
            tracing::warn!("Plugin path is not a directory: {}", path.display());
            return Ok(loaded);
        }

        let entries = std::fs::read_dir(path)?;

        for entry in entries.flatten() {
            let entry_path = entry.path();

            if !entry_path.is_file() {
                continue;
            }

            let extension = entry_path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase());

            let is_plugin = extension.as_ref()
                .map(|ext| ext == "so" || ext == "dll" || ext == "dylib")
                .unwrap_or(false);

            if !is_plugin {
                continue;
            }

            tracing::info!("Found plugin: {}", entry_path.display());

            match self.load(&entry_path).await {
                Ok(plugin) => {
                    let name = registry.register(plugin).await?;
                    tracing::info!("Successfully loaded plugin: {}", name);
                    loaded.push(name);
                }
                Err(e) => {
                    tracing::error!("Failed to load plugin {}: {}", entry_path.display(), e);
                }
            }
        }

        Ok(loaded)
    }

    /// Loads a specific plugin file
    pub async fn load(&self, path: &PathBuf) -> PluginResult<Box<dyn DMSCHardwareDiscoveryPlugin>> {
        tracing::info!("Loading plugin from: {}", path.display());

        if !path.exists() {
            return Err(PluginError::LoadFailed(format!("Plugin file not found: {}", path.display())));
        }

        let library = Arc::new(
            unsafe {
                Library::new(path)
                    .map_err(|e| PluginError::LibraryLoadFailed(format!(
                        "Failed to load library {}: {}", path.display(), e
                    )))?
            }
        );

        let plugin = self.load_plugin_from_library(&library, path)?;

        Ok(plugin)
    }

    /// Loads a plugin from an already loaded library
    fn load_plugin_from_library(&self, library: &Arc<Library>, path: &PathBuf) -> PluginResult<Box<dyn DMSCHardwareDiscoveryPlugin>> {
        type CreatePluginFn = unsafe extern "C" fn() -> *mut dyn DMSCHardwareDiscoveryPlugin;

        unsafe {
            let create_symbol: Symbol<CreatePluginFn> = library
                .get(b"create_dmsc_plugin")
                .map_err(|e| PluginError::SymbolResolutionFailed(format!(
                    "Failed to resolve create_dmsc_plugin symbol in {}: {}", path.display(), e
                )))?;

            let plugin_ptr = create_symbol();

            if plugin_ptr.is_null() {
                return Err(PluginError::LoadFailed(format!(
                    "create_dmsc_plugin returned null pointer for {}", path.display()
                )));
            }

            let plugin = Box::from_raw(plugin_ptr);

            tracing::info!(
                "Successfully loaded plugin: {} v{}",
                plugin.metadata().name,
                plugin.metadata().version
            );

            Ok(plugin)
        }
    }

    /// Validates a plugin file without loading it
    pub async fn validate(&self, path: &PathBuf) -> PluginResult<PluginMetadata> {
        if !path.exists() {
            return Err(PluginError::LoadFailed(format!("Plugin file not found: {}", path.display())));
        }

        let library = Arc::new(
            unsafe {
                Library::new(path)
                    .map_err(|e| PluginError::LibraryLoadFailed(format!(
                        "Failed to load library {}: {}", path.display(), e
                    )))?
            }
        );

        type GetMetadataFn = unsafe extern "C" fn() -> PluginMetadata;

        unsafe {
            let metadata_symbol: Symbol<GetMetadataFn> = library
                .get(b"get_dmsc_plugin_metadata")
                .map_err(|e| PluginError::SymbolResolutionFailed(format!(
                    "Failed to resolve get_dmsc_plugin_metadata symbol in {}: {}", path.display(), e
                )))?;

            let metadata = metadata_symbol();

            tracing::info!(
                "Validated plugin: {} v{} (author: {})",
                metadata.name,
                metadata.version,
                metadata.author
            );

            Ok(metadata)
        }
    }

    /// Gets the plugin API version from a library
    pub async fn get_api_version(&self, path: &PathBuf) -> PluginResult<u32> {
        type GetVersionFn = unsafe extern "C" fn() -> u32;

        let library = Arc::new(
            unsafe {
                Library::new(path)
                    .map_err(|e| PluginError::LibraryLoadFailed(format!(
                        "Failed to load library {}: {}", path.display(), e
                    )))?
            }
        );

        unsafe {
            let version_symbol: Symbol<GetVersionFn> = library
                .get(b"get_dmsc_plugin_api_version")
                .map_err(|e| PluginError::SymbolResolutionFailed(format!(
                    "Failed to resolve get_dmsc_plugin_api_version symbol in {}: {}", path.display(), e
                )))?;

            let version = version_symbol();

            tracing::debug!("Plugin API version for {}: {}", path.display(), version);

            Ok(version)
        }
    }

    /// Checks if a plugin file is compatible with the current DMSC version
    pub async fn is_compatible(&self, path: &PathBuf) -> bool {
        const CURRENT_API_VERSION: u32 = 1;

        match self.get_api_version(path).await {
            Ok(version) => version == CURRENT_API_VERSION,
            Err(e) => {
                tracing::warn!("Failed to check API version for {}: {}", path.display(), e);
                false
            }
        }
    }
}

impl Drop for PluginLoader {
    fn drop(&mut self) {
        tracing::debug!("PluginLoader dropped");
    }
}

/// Built-in custom provider plugin for user-defined discovery
pub struct CustomProviderPlugin {
    name: String,
    version: String,
    description: String,
    discover_func: Arc<dyn Fn() -> Vec<DMSCDevice> + Send + Sync>,
    status: PluginStatus,
}

impl CustomProviderPlugin {
    /// Creates a new custom provider plugin
    pub fn new<F>(name: &str, version: &str, description: &str, discover_func: F) -> Self
    where
        F: Fn() -> Vec<DMSCDevice> + Send + Sync + 'static,
    {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            description: description.to_string(),
            discover_func: Arc::new(discover_func),
            status: PluginStatus::Loaded(),
        }
    }
}

#[async_trait]
impl DMSCHardwareDiscoveryPlugin for CustomProviderPlugin {
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata::new(
            self.name.clone(),
            self.version.clone(),
            "User".to_string(),
            self.description.clone(),
        )
    }

    async fn discover(&self, _platform: &PlatformInfo) -> PluginResult<Vec<DMSCDevice>> {
        let devices = (self.discover_func)();
        Ok(devices)
    }

    fn status(&self) -> PluginStatus {
        self.status.clone()
    }
}

/// Utility function to create a custom discovery plugin from a closure
pub fn create_custom_plugin<F>(name: &str, version: &str, description: &str, discover_fn: F) -> Box<dyn DMSCHardwareDiscoveryPlugin>
where
    F: Fn() -> Vec<DMSCDevice> + Send + Sync + 'static,
{
    Box::new(CustomProviderPlugin::new(name, version, description, discover_fn))
}

use std::collections::{HashMap, HashSet};
