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

//! # Platform Detection Module
//!
//! This module provides cross-platform hardware detection capabilities.
//! It automatically detects the operating system and platform characteristics
//! to enable appropriate discovery strategies for any hardware type.
//!
//! ## Supported Platforms
//!
//! - **Linux**: Full support including sysfs, procfs, D-Bus, udev
//! - **macOS**: Full support using IOKit and system calls
//! - **Windows**: Full support using WMI and system APIs
//! - **WebAssembly**: Limited support with virtual devices
//!
//! ## Usage
//!
//! ```rust
//! use dmsc::device::discovery::platform::{PlatformInfo, PlatformType, get_platform_info};
//!
//! let platform = get_platform_info();
//! println!("Platform: {:?}", platform.platform_type);
//! println!("OS: {}", platform.os_name);
//! println!("Architecture: {}", platform.arch);
//! ```

use std::process::Command;
use serde::{Serialize, Deserialize};

/// Supported platform types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum PlatformType {
    /// Linux-based operating systems
    Linux,
    /// macOS (Darwin)
    MacOS,
    /// Microsoft Windows
    Windows,
    /// WebAssembly environment
    WebAssembly,
    /// Unknown or unsupported platform
    Unknown,
}

/// CPU architecture types
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum Architecture {
    /// x86_64 (AMD64)
    X86_64,
    /// AArch64 (ARM64)
    AArch64,
    /// x86 (i386/i686)
    X86,
    /// ARM (32-bit)
    Arm,
    /// WebAssembly
    Wasm,
    /// Unknown architecture
    Unknown,
}

/// Platform information container
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct PlatformInfo {
    /// Detected platform type
    pub platform_type: PlatformType,
    /// Operating system name
    pub os_name: String,
    /// Operating system version
    pub os_version: String,
    /// CPU architecture
    pub architecture: Architecture,
    /// Number of available CPU cores
    pub cpu_cores: usize,
    /// Amount of available memory in bytes
    pub total_memory: u64,
    /// Whether running in a container
    pub in_container: bool,
    /// Whether running in a virtual machine
    pub in_vm: bool,
    /// Available discovery strategies for this platform
    pub available_strategies: Vec<DiscoveryStrategy>,
}

/// Discovery strategies available on the current platform
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DiscoveryStrategy {
    /// Read from /proc filesystem (Linux)
    ProcFs,
    /// Read from /sys filesystem (Linux)
    SysFs,
    /// Use D-Bus for device enumeration (Linux)
    DBus,
    /// Use udev for device enumeration (Linux)
    UDev,
    /// Use IOKit (macOS)
    IOKit,
    /// Use WMI (Windows)
    WMI,
    /// Use Windows Registry
    Registry,
    /// Use system calls and APIs
    SystemCalls,
    /// Use network scanning
    NetworkScan,
    /// Use virtual filesystem
    VirtualFS,
    /// Fallback to mock devices
    MockFallback,
}

impl PlatformInfo {
    /// Creates a new platform info instance with detected values
    pub fn new() -> Self {
        let platform_info = detect_platform();
        platform_info
    }

    /// Checks if a specific discovery strategy is available
    pub fn has_strategy(&self, strategy: &DiscoveryStrategy) -> bool {
        self.available_strategies.contains(strategy)
    }

    /// Returns the best available discovery strategy for hardware
    pub fn best_hardware_strategy(&self) -> DiscoveryStrategy {
        match self.platform_type {
            PlatformType::Linux => {
                if self.has_strategy(&DiscoveryStrategy::SysFs) {
                    return DiscoveryStrategy::SysFs;
                }
                if self.has_strategy(&DiscoveryStrategy::ProcFs) {
                    return DiscoveryStrategy::ProcFs;
                }
                if self.has_strategy(&DiscoveryStrategy::UDev) {
                    return DiscoveryStrategy::UDev;
                }
                DiscoveryStrategy::MockFallback
            }
            PlatformType::MacOS => {
                if self.has_strategy(&DiscoveryStrategy::IOKit) {
                    return DiscoveryStrategy::IOKit;
                }
                DiscoveryStrategy::SystemCalls
            }
            PlatformType::Windows => {
                if self.has_strategy(&DiscoveryStrategy::WMI) {
                    return DiscoveryStrategy::WMI;
                }
                DiscoveryStrategy::SystemCalls
            }
            PlatformType::WebAssembly => DiscoveryStrategy::VirtualFS,
            PlatformType::Unknown => DiscoveryStrategy::MockFallback,
        }
    }

    /// Returns the best available discovery strategy for network devices
    pub fn best_network_strategy(&self) -> DiscoveryStrategy {
        match self.platform_type {
            PlatformType::Linux => {
                if self.has_strategy(&DiscoveryStrategy::NetworkScan) {
                    return DiscoveryStrategy::NetworkScan;
                }
                DiscoveryStrategy::SysFs
            }
            PlatformType::MacOS => DiscoveryStrategy::SystemCalls,
            PlatformType::Windows => DiscoveryStrategy::WMI,
            PlatformType::WebAssembly => DiscoveryStrategy::VirtualFS,
            PlatformType::Unknown => DiscoveryStrategy::MockFallback,
        }
    }
}

impl Default for PlatformInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// Detects the current platform and returns platform information
pub fn get_platform_info() -> PlatformInfo {
    detect_platform()
}

/// Detects the current platform
fn detect_platform() -> PlatformInfo {
    let (os_name, os_version) = detect_os();
    let architecture = detect_architecture();
    let cpu_cores = detect_cpu_cores();
    let total_memory = detect_total_memory();
    let in_container = detect_container();
    let in_vm = detect_virtual_machine();
    let platform_type = detect_platform_type(&os_name);
    let available_strategies = detect_available_strategies(&platform_type);

    PlatformInfo {
        platform_type,
        os_name,
        os_version,
        architecture,
        cpu_cores,
        total_memory,
        in_container,
        in_vm,
        available_strategies,
    }
}

/// Detects the operating system name and version
fn detect_os() -> (String, String) {
    let os = std::env::consts::OS;
    let version = std::env::consts::FAMILY;

    match os {
        "linux" => {
            let version_info = read_os_release();
            let name = version_info.get("NAME").cloned().unwrap_or_else(|| "Linux".to_string());
            let version_id = version_info.get("VERSION_ID")
                .or(version_info.get("VERSION"))
                .cloned()
                .unwrap_or_else(|| "unknown".to_string());
            (name, version_id)
        }
        "macos" => {
            let version = detect_macos_version();
            ("macOS".to_string(), version)
        }
        "windows" => {
            let version = detect_windows_version();
            ("Windows".to_string(), version)
        }
        _ => (os.to_string(), version.to_string()),
    }
}

/// Reads /etc/os-release for Linux distribution info
fn read_os_release() -> HashMap<String, String> {
    let mut result = HashMap::new();

    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].to_string();
                let value = line[eq_pos + 1..].trim_matches('"').to_string();
                result.insert(key, value);
            }
        }
    }

    result
}

/// Detects macOS version
fn detect_macos_version() -> String {
    let output = Command::new("sw_vers")
        .arg("-productVersion")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }
        }
        Err(_) => {}
    }

    "unknown".to_string()
}

/// Detects Windows version
fn detect_windows_version() -> String {
    let output = Command::new("cmd")
        .args(&["/c", "ver"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                if let Ok(version) = String::from_utf8(output.stdout) {
                    return version.trim().to_string();
                }
            }
        }
        Err(_) => {}
    }

    "unknown".to_string()
}

/// Detects the CPU architecture
fn detect_architecture() -> Architecture {
    let arch = std::env::consts::ARCH;

    match arch {
        "x86_64" => Architecture::X86_64,
        "aarch64" | "arm64" => Architecture::AArch64,
        "x86" | "i686" | "i386" => Architecture::X86,
        "arm" | "thumbv7" => Architecture::Arm,
        "wasm32" => Architecture::Wasm,
        _ => Architecture::Unknown,
    }
}

/// Detects the number of CPU cores
fn detect_cpu_cores() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// Detects total system memory
fn detect_total_memory() -> u64 {
    if let Ok(content) = std::fs::read_to_string("/proc/meminfo") {
        for line in content.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(kb_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = kb_str.parse::<u64>() {
                        return kb * 1024;
                    }
                }
            }
        }
    }
    4 * 1024 * 1024 * 1024 // Default to 4GB if detection fails
}

/// Detects if running inside a container
fn detect_container() -> bool {
    // Check for container-specific files and environment variables

    // Docker
    if std::env::var("DOCKER_CONTAINER").is_ok() {
        return true;
    }

    // Check for .dockerenv file
    if std::path::Path::new("/.dockerenv").exists() {
        return true;
    }

    // Check for container cgroup
    if let Ok(content) = std::fs::read_to_string("/proc/1/cgroup") {
        if content.contains("docker") || content.contains("containerd") {
            return true;
        }
    }

    // Check for k8s
    if std::env::var("KUBERNETES_SERVICE_HOST").is_ok() {
        return true;
    }

    false
}

/// Detects if running inside a virtual machine
fn detect_virtual_machine() -> bool {
    // Check for hypervisor
    if let Ok(content) = std::fs::read_to_string("/sys/hypervisor/type") {
        if content.trim() == "xen" || content.contains("vmware") || content.contains("kvm") {
            return true;
        }
    }

    // Check for VMware
    if std::path::Path::new("/sys/class/dmi/id/sys_vendor").exists() {
        if let Ok(content) = std::fs::read_to_string("/sys/class/dmi/id/sys_vendor") {
            let vendor = content.to_lowercase();
            if vendor.contains("vmware") || vendor.contains("virtualbox") ||
               vendor.contains("qemu") || vendor.contains("hyper-v") {
                return true;
            }
        }
    }

    false
}

/// Detects the platform type from OS name
fn detect_platform_type(os_name: &str) -> PlatformType {
    let os_lower = os_name.to_lowercase();

    if os_lower.contains("linux") || os_lower.contains("ubuntu") ||
       os_lower.contains("debian") || os_lower.contains("centos") ||
       os_lower.contains("fedora") || os_lower.contains("rhel") ||
       os_lower.contains("alpine") {
        return PlatformType::Linux;
    }

    if os_lower.contains("mac") || os_lower.contains("darwin") {
        return PlatformType::MacOS;
    }

    if os_lower.contains("windows") {
        return PlatformType::Windows;
    }

    if std::env::consts::FAMILY == "wasm" {
        return PlatformType::WebAssembly;
    }

    PlatformType::Unknown
}

/// Detects available discovery strategies for the platform
fn detect_available_strategies(platform_type: &PlatformType) -> Vec<DiscoveryStrategy> {
    let mut strategies = Vec::new();

    match platform_type {
        PlatformType::Linux => {
            if std::path::Path::new("/proc").exists() {
                strategies.push(DiscoveryStrategy::ProcFs);
            }
            if std::path::Path::new("/sys").exists() {
                strategies.push(DiscoveryStrategy::SysFs);
            }
            if Command::new("dbus-send").output().is_ok() {
                strategies.push(DiscoveryStrategy::DBus);
            }
            if std::path::Path::new("/dev").exists() {
                strategies.push(DiscoveryStrategy::UDev);
            }
            strategies.push(DiscoveryStrategy::NetworkScan);
            strategies.push(DiscoveryStrategy::MockFallback);
        }
        PlatformType::MacOS => {
            strategies.push(DiscoveryStrategy::IOKit);
            strategies.push(DiscoveryStrategy::SystemCalls);
            strategies.push(DiscoveryStrategy::NetworkScan);
            strategies.push(DiscoveryStrategy::MockFallback);
        }
        PlatformType::Windows => {
            if Command::new("wmic").output().is_ok() {
                strategies.push(DiscoveryStrategy::WMI);
            }
            strategies.push(DiscoveryStrategy::Registry);
            strategies.push(DiscoveryStrategy::SystemCalls);
            strategies.push(DiscoveryStrategy::NetworkScan);
            strategies.push(DiscoveryStrategy::MockFallback);
        }
        PlatformType::WebAssembly => {
            strategies.push(DiscoveryStrategy::VirtualFS);
            strategies.push(DiscoveryStrategy::MockFallback);
        }
        PlatformType::Unknown => {
            strategies.push(DiscoveryStrategy::SystemCalls);
            strategies.push(DiscoveryStrategy::MockFallback);
        }
    }

    strategies
}

/// Platform detection result for hardware compatibility
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct PlatformCompatibility {
    /// Whether the platform is fully supported
    pub is_fully_supported: bool,
    /// Platform type
    pub platform_type: PlatformType,
    /// List of supported hardware categories
    pub supported_hardware: Vec<HardwareCategory>,
    /// Recommended discovery strategies
    pub recommended_strategies: Vec<DiscoveryStrategy>,
    /// Any limitations
    pub limitations: Vec<String>,
}

/// Categories of hardware that can be discovered
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum HardwareCategory {
    /// CPU and processor devices
    CPU,
    /// GPU and graphics processors
    GPU,
    /// Memory and RAM
    Memory,
    /// Storage devices (HDD, SSD, NVMe)
    Storage,
    /// Network interfaces and adapters
    Network,
    /// USB devices
    USB,
    /// PCI devices
    PCI,
    /// Input devices (keyboard, mouse)
    Input,
    /// Display devices (monitors)
    Display,
    /// Bluetooth devices
    Bluetooth,
    /// Other or custom devices
    Other,
}

impl PlatformCompatibility {
    /// Creates platform compatibility info for current platform
    pub fn current() -> Self {
        let platform = get_platform_info();
        Self::from_platform(&platform)
    }

    /// Creates platform compatibility from platform info
    pub fn from_platform(platform: &PlatformInfo) -> Self {
        let mut supported_hardware = Vec::new();
        let mut limitations = Vec::new();

        supported_hardware.extend(vec![
            HardwareCategory::CPU,
            HardwareCategory::Memory,
            HardwareCategory::Storage,
        ]);

        let recommended_strategies = match platform.platform_type {
            PlatformType::Linux => {
                supported_hardware.extend(vec![
                    HardwareCategory::GPU,
                    HardwareCategory::Network,
                    HardwareCategory::USB,
                    HardwareCategory::PCI,
                    HardwareCategory::Input,
                    HardwareCategory::Display,
                    HardwareCategory::Bluetooth,
                ]);
                vec![
                    DiscoveryStrategy::SysFs,
                    DiscoveryStrategy::ProcFs,
                    DiscoveryStrategy::UDev,
                    DiscoveryStrategy::NetworkScan,
                ]
            }
            PlatformType::MacOS => {
                supported_hardware.extend(vec![
                    HardwareCategory::GPU,
                    HardwareCategory::Network,
                    HardwareCategory::USB,
                    HardwareCategory::PCI,
                    HardwareCategory::Input,
                    HardwareCategory::Display,
                ]);
                vec![
                    DiscoveryStrategy::IOKit,
                    DiscoveryStrategy::SystemCalls,
                    DiscoveryStrategy::NetworkScan,
                ]
            }
            PlatformType::Windows => {
                supported_hardware.extend(vec![
                    HardwareCategory::GPU,
                    HardwareCategory::Network,
                    HardwareCategory::USB,
                    HardwareCategory::PCI,
                    HardwareCategory::Input,
                    HardwareCategory::Display,
                    HardwareCategory::Bluetooth,
                ]);
                vec![
                    DiscoveryStrategy::WMI,
                    DiscoveryStrategy::SystemCalls,
                    DiscoveryStrategy::NetworkScan,
                ]
            }
            PlatformType::WebAssembly => {
                limitations.push("Limited hardware access in WebAssembly environment".to_string());
                vec![
                    DiscoveryStrategy::VirtualFS,
                    DiscoveryStrategy::MockFallback,
                ]
            }
            PlatformType::Unknown => {
                limitations.push("Unknown platform - using fallback strategies".to_string());
                vec![
                    DiscoveryStrategy::SystemCalls,
                    DiscoveryStrategy::MockFallback,
                ]
            }
        };

        Self {
            is_fully_supported: platform.platform_type != PlatformType::Unknown,
            platform_type: platform.platform_type.clone(),
            supported_hardware,
            recommended_strategies,
            limitations,
        }
    }

    /// Checks if a hardware category is supported
    pub fn supports_category(&self, category: &HardwareCategory) -> bool {
        self.supported_hardware.contains(category)
    }
}

use std::collections::HashMap;
