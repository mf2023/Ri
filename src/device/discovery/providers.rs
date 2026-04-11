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

//! # Hardware Discovery Providers
//!
//! This module provides hardware discovery providers for different device types.
//! Each provider implements a common interface to detect and enumerate hardware
//! on the current platform.
//!
//! ## Architecture
//!
//! - **RiHardwareProvider**: Trait defining the provider interface
//! - **CPUProvider**: Discovers CPU devices
//! - **MemoryProvider**: Discovers memory devices
//! - **StorageProvider**: Discovers storage devices
//! - **NetworkProvider**: Discovers network devices
//! - **GPUProvider**: Discovers GPU devices
//! - **USBProvider**: Discovers USB devices
//! - **ProviderRegistry**: Manages all available providers
//!
//! ## Usage
//!
//! ```rust,ignore
//! use ri::device::discovery::providers::{ProviderRegistry, CPUProvider};
//!
//! let mut registry = ProviderRegistry::new();
//! registry.register(Box::new(CPUProvider::new()));
//!
//! // Discover all CPU devices
//! let cpus = registry.discover_devices("cpu").await;
//! ```

use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::super::core::{RiDevice, RiDeviceType, RiDeviceCapabilities};
use super::platform::{PlatformInfo, HardwareCategory};

/// Result type for hardware discovery
pub type DiscoveryResult<T> = Result<T, String>;

/// Trait for hardware discovery providers
#[async_trait]
pub trait RiHardwareProvider: Send + Sync {
    /// Returns the provider name
    fn name(&self) -> &str;

    /// Returns the hardware categories this provider handles
    fn categories(&self) -> Vec<HardwareCategory>;

    /// Discovers devices of this type
    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>>;

    /// Returns the priority of this provider (lower = higher priority)
    fn priority(&self) -> u32;

    /// Checks if this provider is available on the current platform
    fn is_available(&self, platform: &PlatformInfo) -> bool;
}

/// CPU Discovery Provider
pub struct CPUProvider {
    priority: u32,
}

impl CPUProvider {
    /// Creates a new CPU provider
    pub fn new() -> Self {
        Self { priority: 10 }
    }
}

impl Default for CPUProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for CPUProvider {
    fn name(&self) -> &str {
        "CPUProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::CPU]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        // Get CPU information based on platform
        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_cpus(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_cpus(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_cpus(platform).await?);
            }
            _ => {
                // Generic fallback for other platforms
                devices.extend(discover_generic_cpus(platform).await?);
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, _platform: &PlatformInfo) -> bool {
        true
    }
}

/// Memory Discovery Provider
pub struct MemoryProvider {
    priority: u32,
}

impl MemoryProvider {
    pub fn new() -> Self {
        Self { priority: 20 }
    }
}

impl Default for MemoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for MemoryProvider {
    fn name(&self) -> &str {
        "MemoryProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::Memory]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_memory(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_memory(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_memory(platform).await?);
            }
            _ => {
                devices.extend(discover_generic_memory(platform).await?);
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, _platform: &PlatformInfo) -> bool {
        true
    }
}

/// Storage Discovery Provider
pub struct StorageProvider {
    priority: u32,
}

impl StorageProvider {
    pub fn new() -> Self {
        Self { priority: 30 }
    }
}

impl Default for StorageProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for StorageProvider {
    fn name(&self) -> &str {
        "StorageProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::Storage]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_storage(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_storage(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_storage(platform).await?);
            }
            _ => {
                devices.extend(discover_generic_storage(platform).await?);
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, _platform: &PlatformInfo) -> bool {
        true
    }
}

/// Network Discovery Provider
pub struct NetworkProvider {
    priority: u32,
}

impl NetworkProvider {
    pub fn new() -> Self {
        Self { priority: 40 }
    }
}

impl Default for NetworkProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for NetworkProvider {
    fn name(&self) -> &str {
        "NetworkProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::Network]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_network(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_network(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_network(platform).await?);
            }
            _ => {
                devices.extend(discover_generic_network(platform).await?);
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, _platform: &PlatformInfo) -> bool {
        true
    }
}

/// GPU Discovery Provider
pub struct GPUProvider {
    priority: u32,
}

impl GPUProvider {
    pub fn new() -> Self {
        Self { priority: 25 }
    }
}

impl Default for GPUProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for GPUProvider {
    fn name(&self) -> &str {
        "GPUProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::GPU]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_gpus(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_gpus(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_gpus(platform).await?);
            }
            _ => {
                devices.extend(discover_generic_gpus(platform).await?);
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, platform: &PlatformInfo) -> bool {
        // GPUs are not available in WebAssembly
        platform.platform_type != super::platform::PlatformType::WebAssembly
    }
}

/// USB Discovery Provider
pub struct USBProvider {
    priority: u32,
}

impl USBProvider {
    pub fn new() -> Self {
        Self { priority: 50 }
    }
}

impl Default for USBProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RiHardwareProvider for USBProvider {
    fn name(&self) -> &str {
        "USBProvider"
    }

    fn categories(&self) -> Vec<HardwareCategory> {
        vec![HardwareCategory::USB]
    }

    async fn discover(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let mut devices = Vec::new();

        match platform.platform_type {
            super::platform::PlatformType::Linux => {
                devices.extend(discover_linux_usb(platform).await?);
            }
            super::platform::PlatformType::MacOS => {
                devices.extend(discover_macos_usb(platform).await?);
            }
            super::platform::PlatformType::Windows => {
                devices.extend(discover_windows_usb(platform).await?);
            }
            _ => {
                // No USB support on other platforms
            }
        }

        Ok(devices)
    }

    fn priority(&self) -> u32 {
        self.priority
    }

    fn is_available(&self, platform: &PlatformInfo) -> bool {
        platform.platform_type != super::platform::PlatformType::WebAssembly
    }
}

/// Provider Registry - manages all hardware discovery providers
#[derive(Default)]
pub struct ProviderRegistry {
    providers: Arc<RwLock<Vec<Arc<dyn RiHardwareProvider>>>>,
}

impl ProviderRegistry {
    /// Creates a new provider registry
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Registers a provider
    pub async fn register(&self, provider: Box<dyn RiHardwareProvider>) {
        let mut providers = self.providers.write().await;
        providers.push(Arc::from(provider));
        // Sort by priority
        providers.sort_by_key(|p| p.priority());
    }

    /// Registers the default set of providers
    pub async fn register_defaults(&self) {
        self.register(Box::new(CPUProvider::new())).await;
        self.register(Box::new(MemoryProvider::new())).await;
        self.register(Box::new(StorageProvider::new())).await;
        self.register(Box::new(NetworkProvider::new())).await;
        self.register(Box::new(GPUProvider::new())).await;
        self.register(Box::new(USBProvider::new())).await;
    }

    /// Discovers devices of a specific type
    pub async fn discover_devices(
        &self,
        category: &HardwareCategory,
        platform: &PlatformInfo,
    ) -> DiscoveryResult<Vec<RiDevice>> {
        let providers = self.providers.read().await;
        let mut all_devices = Vec::new();

        for provider in providers.iter() {
            if provider.categories().contains(category) && provider.is_available(platform) {
                match provider.discover(platform).await {
                    Ok(devices) => all_devices.extend(devices),
                    Err(e) => tracing::warn!("Provider {} failed: {}", provider.name(), e),
                }
            }
        }

        Ok(all_devices)
    }

    /// Discovers all available devices
    pub async fn discover_all(&self, platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
        let providers = self.providers.read().await;
        let mut all_devices = Vec::new();

        for provider in providers.iter() {
            if provider.is_available(platform) {
                match provider.discover(platform).await {
                    Ok(devices) => all_devices.extend(devices),
                    Err(e) => tracing::warn!("Provider {} failed: {}", provider.name(), e),
                }
            }
        }

        Ok(all_devices)
    }

    /// Returns the number of registered providers
    pub async fn provider_count(&self) -> usize {
        self.providers.read().await.len()
    }
}

/// Creates a default device with basic capabilities
fn create_device(
    name: String,
    device_type: RiDeviceType,
    capabilities: RiDeviceCapabilities,
) -> RiDevice {
    RiDevice::new(name, device_type)
        .with_capabilities(capabilities)
}

// Platform-specific discovery implementations

async fn discover_linux_cpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    // Read CPU info from /proc/cpuinfo
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        let mut core_count = 0;
        let mut model_name = String::new();

        for line in content.lines() {
            if line.starts_with("processor") {
                core_count += 1;
            }
            if line.starts_with("model name") || line.starts_with("Model name") {
                if let Some(pos) = line.find(':') {
                    model_name = line[pos + 1..].trim().to_string();
                }
            }
        }

        if core_count > 0 {
            let capabilities = RiDeviceCapabilities::new()
                .with_compute_units(core_count)
                .with_memory_gb(0.0); // Memory will be set by memory provider

            let device = create_device(
                format!("CPU: {}", model_name),
                RiDeviceType::CPU,
                capabilities,
            );
            devices.push(device);
        }
    }

    Ok(devices)
}

async fn discover_macos_cpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    // Use sysctl for CPU info
    let output = std::process::Command::new("sysctl")
        .args(&["-n", "machdep.cpu.brand_string"])
        .output();

    let model_name = match output {
        Ok(output) => {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                "Unknown CPU".to_string()
            }
        }
        Err(_) => "Unknown CPU".to_string(),
    };

    let core_count = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);

    let capabilities = RiDeviceCapabilities::new()
        .with_compute_units(core_count)
        .with_memory_gb(0.0);

    let device = create_device(
        format!("CPU: {}", model_name),
        RiDeviceType::CPU,
        capabilities,
    );
    devices.push(device);

    Ok(devices)
}

async fn discover_windows_cpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("wmic")
        .args(&["CPU", "Get", "Name,NumberOfCores", "/VALUE"])
        .output();

    let model_name = match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = String::from_utf8_lossy(&output.stdout);
                if let Some(name_line) = output_str.lines().find(|l| l.starts_with("Name=")) {
                    name_line[5..].to_string()
                } else {
                    "Unknown CPU".to_string()
                }
            } else {
                "Unknown CPU".to_string()
            }
        }
        Err(_) => "Unknown CPU".to_string(),
    };

    let core_count = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1);

    let capabilities = RiDeviceCapabilities::new()
        .with_compute_units(core_count)
        .with_memory_gb(0.0);

    let device = create_device(
        format!("CPU: {}", model_name),
        RiDeviceType::CPU,
        capabilities,
    );
    devices.push(device);

    Ok(devices)
}

async fn discover_generic_cpus(platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let capabilities = RiDeviceCapabilities::new()
        .with_compute_units(platform.cpu_cores)
        .with_memory_gb(0.0);

    let device = create_device(
        format!("Generic CPU ({} cores)", platform.cpu_cores),
        RiDeviceType::CPU,
        capabilities,
    );

    Ok(vec![device])
}

async fn discover_linux_memory(platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let capabilities = RiDeviceCapabilities::new()
        .with_compute_units(0)
        .with_memory_gb(platform.total_memory as f64 / (1024.0 * 1024.0 * 1024.0));

    let device = create_device(
        format!("System Memory ({:.2} GB)", platform.total_memory as f64 / (1024.0 * 1024.0 * 1024.0)),
        RiDeviceType::Memory,
        capabilities,
    );

    Ok(vec![device])
}

async fn discover_macos_memory(platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    discover_linux_memory(platform).await
}

async fn discover_windows_memory(platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    discover_linux_memory(platform).await
}

async fn discover_generic_memory(platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    discover_linux_memory(platform).await
}

async fn discover_linux_storage(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    // Read /proc/mounts to find mounted filesystems
    if let Ok(content) = std::fs::read_to_string("/proc/mounts") {
        let mut seen = std::collections::HashSet::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let device = parts[0];
                if device.starts_with("/dev/") && !seen.contains(device) {
                    seen.insert(device);

                    let capabilities = RiDeviceCapabilities::new()
                        .with_storage_gb(100.0);

                    let device_info = RiDevice::new(
                        device.to_string(),
                        RiDeviceType::Storage,
                    ).with_capabilities(capabilities);
                    devices.push(device_info);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_macos_storage(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("df")
        .arg("-l")
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let mut seen = std::collections::HashSet::new();
            for line in String::from_utf8_lossy(&output.stdout).lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 9 {
                    let device = parts[0];
                    if !seen.contains(device) && device.starts_with("/dev/") {
                        seen.insert(device);
                        let capabilities = RiDeviceCapabilities::new()
                            .with_storage_gb(100.0);

                        let device_info = RiDevice::new(
                            device.to_string(),
                            RiDeviceType::Storage,
                        ).with_capabilities(capabilities);
                        devices.push(device_info);
                    }
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_windows_storage(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("wmic")
        .args(&["LogicalDisk", "Get", "Name,Size", "/VALUE"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.starts_with("Name=") {
                    let drive = &line[5..];
                    let capabilities = RiDeviceCapabilities::new()
                        .with_storage_gb(100.0);

                    let device_info = RiDevice::new(
                        drive.to_string(),
                        RiDeviceType::Storage,
                    ).with_capabilities(capabilities);
                    devices.push(device_info);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_generic_storage(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let capabilities = RiDeviceCapabilities::new()
        .with_storage_gb(100.0);

    let device = create_device(
        "Generic Storage".to_string(),
        RiDeviceType::Storage,
        capabilities,
    );

    Ok(vec![device])
}

async fn discover_linux_network(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
        for line in content.lines().skip(2) {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 {
                let interface = parts[0].trim();
                if !interface.is_empty() && interface != "lo" {
                    let capabilities = RiDeviceCapabilities::new()
                        .with_bandwidth_gbps(1.0);

                    let device = RiDevice::new(
                        format!("Network Interface: {}", interface),
                        RiDeviceType::Network,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_macos_network(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("ifconfig")
        .output();

    if let Ok(output) = output {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if line.starts_with_flags(&['a'..='z', 'A'..='Z']) && !line.starts_with("lo") {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 1 {
                    let interface = parts[0].trim();
                    if !interface.is_empty() {
                        let capabilities = RiDeviceCapabilities::new()
                            .with_bandwidth_gbps(1.0);

                        let device = RiDevice::new(
                            format!("Network Interface: {}", interface),
                            RiDeviceType::Network,
                        ).with_capabilities(capabilities);
                        devices.push(device);
                    }
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_windows_network(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("wmic")
        .args(&["NicConfig", "Get", "Description,MACAddress", "/VALUE"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.starts_with("Description=") {
                    let name = &line[12..];
                    let capabilities = RiDeviceCapabilities::new()
                        .with_bandwidth_gbps(1.0);

                    let device = RiDevice::new(
                        name.to_string(),
                        RiDeviceType::Network,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_generic_network(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let capabilities = RiDeviceCapabilities::new()
        .with_bandwidth_gbps(1.0);

    let device = create_device(
        "Generic Network Adapter".to_string(),
        RiDeviceType::Network,
        capabilities,
    );

    Ok(vec![device])
}

async fn discover_linux_gpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    // Check for NVIDIA GPUs
    if let Ok(nvidia_output) = std::process::Command::new("nvidia-smi")
        .arg("--query-gpu=name,memory.total,driver_version")
        .arg("--format=csv,noheader")
        .output()
    {
        if nvidia_output.status.success() {
            for line in String::from_utf8_lossy(&nvidia_output.stdout).lines() {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 1 {
                    let name = parts[0].trim();
                    let capabilities = RiDeviceCapabilities::new()
                        .with_compute_units(1)
                        .with_memory_gb(8.0);

                    let device = RiDevice::new(
                        format!("NVIDIA GPU: {}", name),
                        RiDeviceType::GPU,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    // Check for AMD GPUs in sysfs
    if let Ok(amd_dirs) = std::fs::read_dir("/sys/class/drm") {
        for entry in amd_dirs.flatten() {
            if let Ok(path) = entry.path().join("device").read_link() {
                if path.to_string_lossy().contains("pci") {
                    let capabilities = RiDeviceCapabilities::new()
                        .with_compute_units(1)
                        .with_memory_gb(4.0);

                    let device = RiDevice::new(
                        format!("GPU: {}", entry.file_name().to_string_lossy()),
                        RiDeviceType::GPU,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_macos_gpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("system_profiler")
        .args(&["SPDisplaysDataType", "-detailLevel", "mini"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let capabilities = RiDeviceCapabilities::new()
                .with_compute_units(1)
                .with_memory_gb(4.0);

            let device = RiDevice::new(
                format!("GPU: {}", content.lines().next().unwrap_or("Unknown")),
                RiDeviceType::GPU,
            ).with_capabilities(capabilities);
            devices.push(device);
        }
    }

    Ok(devices)
}

async fn discover_windows_gpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("wmic")
        .args(&["Path", "Win32_VideoController", "Get", "Name,AdapterRAM", "/VALUE"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.starts_with("Name=") {
                    let name = &line[5..];
                    let capabilities = RiDeviceCapabilities::new()
                        .with_compute_units(1)
                        .with_memory_gb(4.0);

                    let device = RiDevice::new(
                        name.to_string(),
                        RiDeviceType::GPU,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_generic_gpus(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let capabilities = RiDeviceCapabilities::new()
        .with_compute_units(1)
        .with_memory_gb(4.0);

    let device = create_device(
        "Generic GPU".to_string(),
        RiDeviceType::GPU,
        capabilities,
    );

    Ok(vec![device])
}

async fn discover_linux_usb(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    if let Ok(usb_dirs) = std::fs::read_dir("/sys/bus/usb/devices") {
        for entry in usb_dirs.flatten() {
            if let Ok(id) = entry.file_name().into_string() {
                if !id.is_empty() && !id.starts_with('.') {
                    let capabilities = RiDeviceCapabilities::new();

                    let device = RiDevice::new(
                        format!("USB Device: {}", id),
                        RiDeviceType::Custom,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

async fn discover_macos_usb(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("system_profiler")
        .args(&["SPUSBDataType", "-detailLevel", "mini"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            let content = String::from_utf8_lossy(&output.stdout);
            let capabilities = RiDeviceCapabilities::new();

            let device = RiDevice::new(
                format!("USB: {}", content.lines().next().unwrap_or("Unknown")),
                RiDeviceType::Custom,
            ).with_capabilities(capabilities);
            devices.push(device);
        }
    }

    Ok(devices)
}

async fn discover_windows_usb(_platform: &PlatformInfo) -> DiscoveryResult<Vec<RiDevice>> {
    let mut devices = Vec::new();

    let output = std::process::Command::new("wmic")
        .args(&["USBController", "Get", "Name", "/VALUE"])
        .output();

    if let Ok(output) = output {
        if output.status.success() {
            for line in String::from_utf8_lossy(&output.stdout).lines() {
                if line.starts_with("Name=") {
                    let name = &line[5..];
                    let capabilities = RiDeviceCapabilities::new();

                    let device = RiDevice::new(
                        name.to_string(),
                        RiDeviceType::Custom,
                    ).with_capabilities(capabilities);
                    devices.push(device);
                }
            }
        }
    }

    Ok(devices)
}

trait StartsWith {
    fn starts_with_flags(&self, ranges: &[std::ops::RangeInclusive<char>]) -> bool;
}

impl StartsWith for str {
    fn starts_with_flags(&self, ranges: &[std::ops::RangeInclusive<char>]) -> bool {
        if let Some(first_char) = self.chars().next() {
            ranges.iter().any(|r| r.contains(&first_char))
        } else {
            false
        }
    }
}
