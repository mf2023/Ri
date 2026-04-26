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

//! # Device Discovery Module
//!
//! This module provides a comprehensive cross-platform device discovery system
//! that can detect and enumerate hardware devices on any platform.
//!
//! ## Architecture
//!
//! - **RiDeviceDiscovery**: Main discovery engine combining providers and plugins
//! - **RiHardwareProvider**: Trait for built-in hardware providers
//! - **RiHardwareDiscoveryPlugin**: Trait for custom discovery plugins
//! - **PlatformInfo**: Cross-platform detection and compatibility info
//!
//! ## Features
//!
//! - **Universal Hardware Support**: CPU, Memory, Storage, Network, GPU, USB
//! - **Cross-Platform**: Linux, macOS, Windows, WebAssembly
//! - **Extensible Plugin System**: Custom discovery implementations
//! - **Async-First**: All operations are asynchronous
//! - **Fallback Strategies**: Graceful degradation when platforms lack support
//!
//! ## Usage
//!
//! ```rust,ignore
//! use ri::device::discovery::{RiDeviceDiscovery, DiscoveryConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = DiscoveryConfig::default();
//!     let discovery = RiDeviceDiscovery::new(config).await?;
//!
//!     // Discover all devices
//!     let devices = discovery.discover_all().await?;
//!     println!("Discovered {} devices", devices.len());
//!
//!     // Discover specific category
//!     let cpus = discovery.discover_category(ri::device::HardwareCategory::CPU).await?;
//!     println!("Found {} CPUs", cpus.len());
//!
//!     Ok(())
//! }
//! ```

pub mod platform;
pub mod providers;
pub mod plugins;

pub use platform::{
    PlatformInfo,
    PlatformType,
    Architecture,
    DiscoveryStrategy,
    HardwareCategory,
    PlatformCompatibility,
};

pub use providers::{
    RiHardwareProvider,
    ProviderRegistry,
    DiscoveryResult,
    CPUProvider,
    MemoryProvider,
    StorageProvider,
    NetworkProvider,
    GPUProvider,
    USBProvider,
};

pub use plugins::{
    RiHardwareDiscoveryPlugin,
    PluginRegistry,
    PluginMetadata,
    PluginStatus,
    PluginError,
    PluginResult,
    PluginLoader,
    create_custom_plugin,
};

use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

use super::core::{RiDevice, RiDeviceType, RiDeviceCapabilities};
use super::RiResult;

/// Discovery configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DiscoveryConfig {
    /// Enable CPU discovery
    pub enable_cpu_discovery: bool,
    /// Enable memory discovery
    pub enable_memory_discovery: bool,
    /// Enable storage discovery
    pub enable_storage_discovery: bool,
    /// Enable network discovery
    pub enable_network_discovery: bool,
    /// Enable GPU discovery
    pub enable_gpu_discovery: bool,
    /// Enable USB discovery
    pub enable_usb_discovery: bool,
    /// Enable custom plugins
    pub enable_plugins: bool,
    /// Enable fallback to mock devices
    pub enable_mock_fallback: bool,
    /// Discovery timeout in seconds
    pub timeout_secs: u64,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
}

impl Default for DiscoveryConfig {
    fn default() -> Self {
        Self {
            enable_cpu_discovery: true,
            enable_memory_discovery: true,
            enable_storage_discovery: true,
            enable_network_discovery: true,
            enable_gpu_discovery: true,
            enable_usb_discovery: true,
            enable_plugins: true,
            enable_mock_fallback: true,
            timeout_secs: 30,
            max_retries: 3,
            retry_delay_ms: 1000,
        }
    }
}

/// Discovery statistics
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DiscoveryStats {
    /// Total discovery operations
    pub total_operations: u64,
    /// Successful discoveries
    pub successful_discoveries: u64,
    /// Failed discoveries
    pub failed_discoveries: u64,
    /// Total devices discovered
    pub total_devices_discovered: u64,
    /// Discovery time in milliseconds
    pub last_discovery_time_ms: u128,
    /// Average discovery time in milliseconds
    pub avg_discovery_time_ms: f64,
    /// Number of providers used
    pub providers_used: usize,
    /// Number of plugins used
    pub plugins_used: usize,
}

/// Main device discovery engine
#[derive(Clone)]
pub struct RiDeviceDiscovery {
    config: Arc<DiscoveryConfig>,
    platform: Arc<PlatformInfo>,
    provider_registry: Arc<ProviderRegistry>,
    plugin_registry: Arc<RwLock<PluginRegistry>>,
    stats: Arc<RwLock<DiscoveryStats>>,
    discovered_devices: Arc<RwLock<FxHashMap<String, RiDevice>>>,
}

impl RiDeviceDiscovery {
    /// Creates a new device discovery engine
    pub async fn new(config: DiscoveryConfig) -> RiResult<Self> {
        let platform = Arc::new(PlatformInfo::new());
        let provider_registry = Arc::new(ProviderRegistry::new());
        let plugin_registry = PluginRegistry::new();

        // Register default providers
        provider_registry.register_defaults().await;

        // Register built-in plugins if enabled
        if config.enable_plugins {
            // Custom plugins can be registered here
        }

        let stats = Arc::new(RwLock::new(DiscoveryStats::default()));
        let discovered_devices = Arc::new(RwLock::new(FxHashMap::default()));

        Ok(Self {
            config: Arc::new(config),
            platform,
            provider_registry,
            plugin_registry: Arc::new(RwLock::new(plugin_registry)),
            stats,
            discovered_devices,
        })
    }

    /// Creates a discovery engine with default configuration
    pub async fn with_defaults() -> RiResult<Self> {
        Self::new(DiscoveryConfig::default()).await
    }

    /// Discovers all enabled device categories
    pub async fn discover_all(&self) -> RiResult<Vec<RiDevice>> {
        let start_time = std::time::Instant::now();
        let mut all_devices = Vec::with_capacity(8);
        let mut providers_used = 0;
        let mut plugins_used = 0;

        // Discover using providers
        if self.config.enable_cpu_discovery {
            match self.discover_category(HardwareCategory::CPU).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("CPU discovery failed: {}", e),
            }
        }

        if self.config.enable_memory_discovery {
            match self.discover_category(HardwareCategory::Memory).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("Memory discovery failed: {}", e),
            }
        }

        if self.config.enable_storage_discovery {
            match self.discover_category(HardwareCategory::Storage).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("Storage discovery failed: {}", e),
            }
        }

        if self.config.enable_network_discovery {
            match self.discover_category(HardwareCategory::Network).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("Network discovery failed: {}", e),
            }
        }

        if self.config.enable_gpu_discovery {
            match self.discover_category(HardwareCategory::GPU).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("GPU discovery failed: {}", e),
            }
        }

        if self.config.enable_usb_discovery {
            match self.discover_category(HardwareCategory::USB).await {
                Ok(devices) => {
                    all_devices.extend(devices);
                    providers_used += 1;
                }
                Err(e) => tracing::warn!("USB discovery failed: {}", e),
            }
        }

        // Discover using plugins
        if self.config.enable_plugins {
            let plugin_devices = self.discover_with_plugins().await?;
            if !plugin_devices.is_empty() {
                all_devices.extend(plugin_devices);
                plugins_used += 1;
            }
        }

        // Fallback to mock devices if enabled and no devices found
        if self.config.enable_mock_fallback && all_devices.is_empty() {
            tracing::info!("No devices discovered, using mock fallback");
            let mock_devices = self.create_mock_devices().await?;
            all_devices.extend(mock_devices);
        }

        // Update statistics
        let elapsed = start_time.elapsed();
        self.update_stats(all_devices.len(), providers_used, plugins_used, elapsed).await;

        // Cache discovered devices
        let mut cache = self.discovered_devices.write().await;
        for device in &all_devices {
            cache.insert(device.id().to_string(), device.clone());
        }

        Ok(all_devices)
    }

    /// Discovers devices of a specific category
    pub async fn discover_category(&self, category: HardwareCategory) -> RiResult<Vec<RiDevice>> {
        let start_time = std::time::Instant::now();

        let devices = self.provider_registry.discover_devices(&category, &self.platform).await
            .map_err(|e| crate::core::RiError::Other(format!("Discovery failed: {}", e)))?;

        // Update statistics
        let elapsed = start_time.elapsed().as_millis();
        self.stats.write().await.last_discovery_time_ms = elapsed;
        self.stats.write().await.total_devices_discovered += devices.len() as u64;

        Ok(devices)
    }

    /// Discovers devices using registered plugins
    pub async fn discover_with_plugins(&self) -> RiResult<Vec<RiDevice>> {
        let devices = self.plugin_registry.read().await.discover_all(&self.platform).await
            .map_err(|e| crate::core::RiError::Other(format!("Plugin discovery failed: {}", e)))?;
        Ok(devices)
    }

    /// Discovers CPU devices
    pub async fn discover_cpus(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::CPU).await
    }

    /// Discovers memory devices
    pub async fn discover_memory(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::Memory).await
    }

    /// Discovers storage devices
    pub async fn discover_storage(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::Storage).await
    }

    /// Discovers network devices
    pub async fn discover_network(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::Network).await
    }

    /// Discovers GPU devices
    pub async fn discover_gpus(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::GPU).await
    }

    /// Discovers USB devices
    pub async fn discover_usb(&self) -> RiResult<Vec<RiDevice>> {
        self.discover_category(HardwareCategory::USB).await
    }

    /// Returns platform information
    pub fn platform_info(&self) -> &PlatformInfo {
        &self.platform
    }

    /// Returns discovery statistics
    pub async fn stats(&self) -> DiscoveryStats {
        self.stats.read().await.clone()
    }

    /// Returns all discovered devices
    pub async fn get_discovered_devices(&self) -> Vec<RiDevice> {
        self.discovered_devices.read().await.values().cloned().collect()
    }

    /// Returns a device by ID
    pub async fn get_device(&self, id: &str) -> Option<RiDevice> {
        self.discovered_devices.read().await.get(id).cloned()
    }

    /// Clears all discovered devices
    pub async fn clear_cache(&self) {
        self.discovered_devices.write().await.clear();
    }

    /// Registers a custom hardware provider
    pub async fn register_provider<P: RiHardwareProvider + 'static>(&self, provider: P) {
        self.provider_registry.register(Box::new(provider)).await;
    }

    /// Registers a custom discovery plugin
    pub async fn register_plugin(&mut self, plugin: Box<dyn RiHardwareDiscoveryPlugin>) {
        self.plugin_registry.write().await.register(plugin).await.ok();
    }

    /// Enables a plugin by name
    pub async fn enable_plugin(&self, name: &str) {
        self.plugin_registry.read().await.enable(name).await.ok();
    }

    /// Disables a plugin by name
    pub async fn disable_plugin(&self, name: &str) {
        self.plugin_registry.read().await.disable(name).await.ok();
    }

    /// Returns the number of registered providers
    pub async fn provider_count(&self) -> usize {
        self.provider_registry.provider_count().await
    }

    /// Returns the number of registered plugins
    pub async fn plugin_count(&self) -> usize {
        self.plugin_registry.read().await.count().await
    }

    /// Returns platform compatibility information
    pub fn platform_compatibility(&self) -> PlatformCompatibility {
        PlatformCompatibility::from_platform(&self.platform)
    }

    /// Updates discovery statistics
    async fn update_stats(&self, device_count: usize, providers_used: usize, plugins_used: usize, elapsed: std::time::Duration) {
        let mut stats = self.stats.write().await;
        stats.total_operations += 1;
        stats.successful_discoveries += 1;
        stats.total_devices_discovered += device_count as u64;
        stats.last_discovery_time_ms = elapsed.as_millis();

        // Update average
        let total_time = stats.avg_discovery_time_ms * (stats.successful_discoveries as f64 - 1.0);
        stats.avg_discovery_time_ms = (total_time + elapsed.as_millis() as f64) / stats.successful_discoveries as f64;

        stats.providers_used = providers_used;
        stats.plugins_used = plugins_used;
    }

    /// Creates mock devices for testing/fallback
    async fn create_mock_devices(&self) -> RiResult<Vec<RiDevice>> {
        let mut devices = Vec::with_capacity(4);

        // Create a mock CPU
        if self.config.enable_cpu_discovery {
            let cpu_capabilities = RiDeviceCapabilities::new()
                .with_compute_units(self.platform.cpu_cores)
                .with_memory_gb(self.platform.total_memory as f64 / (1024.0 * 1024.0 * 1024.0));

            let cpu = RiDevice::new(
                format!("Mock CPU ({} cores)", self.platform.cpu_cores),
                RiDeviceType::CPU,
            ).with_capabilities(cpu_capabilities);
            devices.push(cpu);
        }

        // Create a mock memory
        if self.config.enable_memory_discovery {
            let mem_capabilities = RiDeviceCapabilities::new()
                .with_compute_units(0)
                .with_memory_gb(self.platform.total_memory as f64 / (1024.0 * 1024.0 * 1024.0));

            let memory = RiDevice::new(
                format!("Mock Memory ({:.2} GB)", self.platform.total_memory as f64 / (1024.0 * 1024.0 * 1024.0)),
                RiDeviceType::Memory,
            ).with_capabilities(mem_capabilities);
            devices.push(memory);
        }

        // Create a mock network adapter
        if self.config.enable_network_discovery {
            let net_capabilities = RiDeviceCapabilities::new()
                .with_bandwidth_gbps(1.0);

            let network = RiDevice::new(
                "Mock Loopback Interface".to_string(),
                RiDeviceType::Network,
            ).with_capabilities(net_capabilities);
            devices.push(network);
        }

        Ok(devices)
    }
}

/// Async discovery operation with progress tracking
pub struct AsyncDiscovery {
    discovery: Arc<RiDeviceDiscovery>,
    current_progress: Arc<RwLock<f32>>,
    is_cancelled: Arc<RwLock<bool>>,
}

impl AsyncDiscovery {
    /// Creates a new async discovery operation
    pub fn new(discovery: Arc<RiDeviceDiscovery>) -> Self {
        Self {
            discovery,
            current_progress: Arc::new(RwLock::new(0.0)),
            is_cancelled: Arc::new(RwLock::new(false)),
        }
    }

    /// Discovers all devices with progress tracking
    pub async fn discover_all(&self) -> RiResult<Vec<RiDevice>> {
        *self.current_progress.write().await = 0.0;

        *self.current_progress.write().await = 0.1; // 10% - Starting
        if *self.is_cancelled.read().await {
            return Err(crate::core::RiError::Other("Discovery cancelled".to_string()));
        }

        let result = self.discovery.discover_all().await;

        *self.current_progress.write().await = 1.0; // 100% - Complete

        result
    }

    /// Returns current progress (0.0 to 1.0)
    pub async fn progress(&self) -> f32 {
        *self.current_progress.read().await
    }

    /// Cancels the discovery operation
    pub async fn cancel(&self) {
        *self.is_cancelled.write().await = true;
    }
}

/// Extension trait for RiDeviceController to integrate discovery
#[async_trait::async_trait]
pub trait DeviceDiscoveryExtension {
    /// Performs device discovery and returns discovered devices
    async fn perform_discovery(&mut self) -> RiResult<Vec<RiDevice>>;

    /// Returns the discovery engine
    fn discovery_engine(&self) -> Option<&RiDeviceDiscovery>;
}

use std::collections::HashMap as FxHashMap;
