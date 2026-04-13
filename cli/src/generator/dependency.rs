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

//! Dependency Management Module
//!
//! This module provides functionality for managing project dependencies in Cargo.toml files.
//! It supports adding, removing, updating, and checking dependencies with full
//! version specification support.
//!
//! # Features
//!
//! - **Add Dependencies**: Add new dependencies with version and features
//! - **Remove Dependencies**: Remove existing dependencies from Cargo.toml
//! - **Update Dependencies**: Update dependency versions
//! - **Check Dependencies**: Check if a dependency exists
//! - **Feature Support**: Add dependencies with specific features
//!
//! # Architecture
//!
//! The module is built around the `DependencyManager` struct which provides:
//!
//! - Cargo.toml parsing and manipulation
//! - Dependency version management
//! - Feature flag support
//! - Validation and error handling
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::generator::dependency::DependencyManager;
//!
//! // Create a dependency manager
//! let mut manager = DependencyManager::new("Cargo.toml")?;
//!
//! // Add a dependency
//! manager.add_dependency("serde", "1.0", Some(&["derive"]))?;
//!
//! // Check if a dependency exists
//! if manager.check_dependency_exists("serde")? {
//!     println!("serde is installed");
//! }
//!
//! // Update a dependency
//! manager.update_dependency("serde", "1.0.150")?;
//!
//! // Remove a dependency
//! manager.remove_dependency("serde")?;
//! ```

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

// =============================================================================
// Dependency Information
// =============================================================================

/// Dependency information structure
///
/// Contains all information about a single dependency including
/// its name, version, and optional features.
///
/// # Fields
///
/// - `name`: The dependency name (crate name)
/// - `version`: The version specification (can be a simple version or complex spec)
/// - `features`: Optional list of features to enable
/// - `optional`: Whether this is an optional dependency
/// - `default_features`: Whether to enable default features
///
/// # Example
///
/// ```rust,ignore
/// let dep = DependencyInfo {
///     name: "serde".to_string(),
///     version: Some("1.0".to_string()),
///     features: Some(vec!["derive".to_string()]),
///     optional: false,
///     default_features: true,
/// };
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DependencyInfo {
    /// Dependency name (crate name)
    pub name: String,

    /// Version specification (e.g., "1.0", ">=1.0, <2.0")
    pub version: Option<String>,

    /// Features to enable for this dependency
    pub features: Option<Vec<String>>,

    /// Whether this is an optional dependency
    pub optional: bool,

    /// Whether to enable default features
    pub default_features: bool,
}

impl DependencyInfo {
    /// Create a new dependency info with just a name
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name
    ///
    /// # Returns
    ///
    /// Returns a new DependencyInfo with default settings.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let dep = DependencyInfo::new("serde");
    /// ```
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            version: None,
            features: None,
            optional: false,
            default_features: true,
        }
    }

    /// Create a dependency info with a version
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name
    /// * `version` - The version specification
    ///
    /// # Returns
    ///
    /// Returns a new DependencyInfo with the specified version.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let dep = DependencyInfo::with_version("serde", "1.0");
    /// ```
    pub fn with_version(name: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            version: Some(version.to_string()),
            features: None,
            optional: false,
            default_features: true,
        }
    }

    /// Create a dependency info with version and features
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name
    /// * `version` - The version specification
    /// * `features` - List of features to enable
    ///
    /// # Returns
    ///
    /// Returns a new DependencyInfo with version and features.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let dep = DependencyInfo::with_features("serde", "1.0", &["derive", "rc"]);
    /// ```
    pub fn with_features(name: &str, version: &str, features: &[&str]) -> Self {
        Self {
            name: name.to_string(),
            version: Some(version.to_string()),
            features: Some(features.iter().map(|s| s.to_string()).collect()),
            optional: false,
            default_features: true,
        }
    }

    /// Convert to Cargo.toml format string
    ///
    /// Generates the appropriate string representation for Cargo.toml.
    /// Uses simple format for simple dependencies, table format for complex ones.
    ///
    /// # Returns
    ///
    /// Returns the Cargo.toml format string.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let dep = DependencyInfo::with_features("serde", "1.0", &["derive"]);
    /// let toml = dep.to_toml_string();
    /// // Returns: serde = { version = "1.0", features = ["derive"] }
    /// ```
    pub fn to_toml_string(&self) -> String {
        // Simple format: name = "version"
        if self.features.is_none() && self.default_features && !self.optional {
            if let Some(ref version) = self.version {
                return format!("{} = \"{}\"", self.name, version);
            }
        }

        // Table format: name = { version = "...", features = [...], ... }
        let mut parts = Vec::new();

        if let Some(ref version) = self.version {
            parts.push(format!("version = \"{}\"", version));
        }

        if let Some(ref features) = self.features {
            let features_str = features
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<_>>()
                .join(", ");
            parts.push(format!("features = [{}]", features_str));
        }

        if !self.default_features {
            parts.push("default-features = false".to_string());
        }

        if self.optional {
            parts.push("optional = true".to_string());
        }

        format!("{} = {{ {} }}", self.name, parts.join(", "))
    }
}

impl std::fmt::Display for DependencyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_toml_string())
    }
}

// =============================================================================
// Dependency Manager
// =============================================================================

/// Dependency manager for Cargo.toml files
///
/// Provides functionality to add, remove, update, and check dependencies
/// in a Cargo.toml file while preserving existing content and formatting.
///
/// # Features
///
/// - **Parse Cargo.toml**: Read and parse existing Cargo.toml files
/// - **Add Dependencies**: Add new dependencies with full specification support
/// - **Remove Dependencies**: Remove dependencies by name
/// - **Update Dependencies**: Update dependency versions
/// - **Check Dependencies**: Verify if a dependency exists
/// - **Preserve Formatting**: Maintain existing file structure
///
/// # Example
///
/// ```rust,ignore
/// // Create a dependency manager
/// let mut manager = DependencyManager::new("Cargo.toml")?;
///
/// // Add a simple dependency
/// manager.add_dependency("serde", "1.0", None)?;
///
/// // Add a dependency with features
/// manager.add_dependency("tokio", "1.0", Some(&["full"]))?;
///
/// // Check if dependency exists
/// if manager.check_dependency_exists("serde")? {
///     println!("serde is installed");
/// }
///
/// // Update a dependency
/// manager.update_dependency("serde", "1.0.150")?;
///
/// // Remove a dependency
/// manager.remove_dependency("serde")?;
/// ```
pub struct DependencyManager {
    /// Path to the Cargo.toml file
    cargo_toml_path: PathBuf,

    /// Parsed dependencies section
    dependencies: HashMap<String, DependencyInfo>,

    /// Raw content of the Cargo.toml file
    raw_content: String,
}

impl DependencyManager {
    /// Create a new dependency manager
    ///
    /// Opens and parses the specified Cargo.toml file.
    ///
    /// # Arguments
    ///
    /// * `cargo_toml_path` - Path to the Cargo.toml file
    ///
    /// # Returns
    ///
    /// Returns `Ok(DependencyManager)` on success.
    /// Returns an error if the file cannot be read or parsed.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let manager = DependencyManager::new("Cargo.toml")?;
    /// ```
    pub fn new<P: AsRef<Path>>(cargo_toml_path: P) -> Result<Self> {
        let path = cargo_toml_path.as_ref();

        // Read the file
        let raw_content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read Cargo.toml: {}", path.display()))?;

        // Parse dependencies
        let dependencies = Self::parse_dependencies(&raw_content)?;

        Ok(Self {
            cargo_toml_path: path.to_path_buf(),
            dependencies,
            raw_content,
        })
    }

    /// Parse dependencies from Cargo.toml content
    ///
    /// Extracts the [dependencies] section and parses each dependency.
    ///
    /// # Arguments
    ///
    /// * `content` - The Cargo.toml file content
    ///
    /// # Returns
    ///
    /// Returns a HashMap of dependency names to DependencyInfo.
    fn parse_dependencies(content: &str) -> Result<HashMap<String, DependencyInfo>> {
        let mut dependencies = HashMap::new();
        let mut in_deps_section = false;

        for line in content.lines() {
            let trimmed = line.trim();

            // Check for section headers
            if trimmed.starts_with('[') {
                in_deps_section = trimmed == "[dependencies]";
                continue;
            }

            // Skip if not in dependencies section
            if !in_deps_section || trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            // Parse dependency line
            if let Some((name, info)) = Self::parse_dependency_line(trimmed) {
                dependencies.insert(name, info);
            }
        }

        Ok(dependencies)
    }

    /// Parse a single dependency line
    ///
    /// Handles both simple format (`name = "version"`) and table format
    /// (`name = { version = "...", features = [...] }`).
    ///
    /// # Arguments
    ///
    /// * `line` - The dependency line to parse
    ///
    /// # Returns
    ///
    /// Returns Some((name, DependencyInfo)) if parsing succeeds.
    fn parse_dependency_line(line: &str) -> Option<(String, DependencyInfo)> {
        // Split on first '='
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            return None;
        }

        let name = parts[0].trim().to_string();
        let value = parts[1].trim();

        // Simple format: "version"
        if value.starts_with('"') && value.ends_with('"') {
            let version = value.trim_matches('"').to_string();
            return Some((
                name.clone(),
                DependencyInfo {
                    name: name.clone(),
                    version: Some(version),
                    features: None,
                    optional: false,
                    default_features: true,
                },
            ));
        }

        // Table format: { version = "...", features = [...] }
        if value.starts_with('{') && value.ends_with('}') {
            let inner = value.trim_matches('{').trim_matches('}');
            let mut version = None;
            let mut features = None;
            let mut optional = false;
            let mut default_features = true;

            // Parse key-value pairs
            for part in inner.split(',') {
                let part = part.trim();
                if part.starts_with("version") {
                    if let Some(v) = part.split('"').nth(1) {
                        version = Some(v.to_string());
                    }
                } else if part.starts_with("features") {
                    // Extract features array
                    if let Some(start) = part.find('[') {
                        if let Some(end) = part.find(']') {
                            let features_str = &part[start + 1..end];
                            features = Some(
                                features_str
                                    .split(',')
                                    .map(|s| s.trim().trim_matches('"').to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect(),
                            );
                        }
                    }
                } else if part.starts_with("optional") {
                    optional = part.contains("true");
                } else if part.starts_with("default-features") {
                    default_features = part.contains("true");
                }
            }

            return Some((
                name.clone(),
                DependencyInfo {
                    name: name.clone(),
                    version,
                    features,
                    optional,
                    default_features,
                },
            ));
        }

        None
    }

    /// Add a dependency to Cargo.toml
    ///
    /// Adds a new dependency with the specified version and optional features.
    /// If the dependency already exists, it will be updated.
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name (crate name)
    /// * `version` - The version specification
    /// * `features` - Optional list of features to enable
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    /// Returns an error if the operation fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// // Add a simple dependency
    /// manager.add_dependency("serde", "1.0", None)?;
    ///
    /// // Add a dependency with features
    /// manager.add_dependency("tokio", "1.0", Some(&["full", "sync"]))?;
    /// ```
    pub fn add_dependency(
        &mut self,
        name: &str,
        version: &str,
        features: Option<&[&str]>,
    ) -> Result<()> {
        let dep_info = DependencyInfo {
            name: name.to_string(),
            version: Some(version.to_string()),
            features: features.map(|f| f.iter().map(|s| s.to_string()).collect()),
            optional: false,
            default_features: true,
        };

        // Add to internal map
        self.dependencies.insert(name.to_string(), dep_info.clone());

        // Update the file
        self.update_file()?;

        Ok(())
    }

    /// Remove a dependency from Cargo.toml
    ///
    /// Removes the specified dependency if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name to remove
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the dependency was removed.
    /// Returns `Ok(false)` if the dependency didn't exist.
    /// Returns an error if the operation fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// if manager.remove_dependency("serde")? {
    ///     println!("serde was removed");
    /// }
    /// ```
    pub fn remove_dependency(&mut self, name: &str) -> Result<bool> {
        if self.dependencies.remove(name).is_none() {
            return Ok(false);
        }

        // Update the file
        self.update_file()?;

        Ok(true)
    }

    /// Update a dependency version
    ///
    /// Updates the version of an existing dependency.
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name
    /// * `version` - The new version specification
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the dependency was updated.
    /// Returns `Ok(false)` if the dependency didn't exist.
    /// Returns an error if the operation fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// if manager.update_dependency("serde", "1.0.150")? {
    ///     println!("serde was updated");
    /// }
    /// ```
    pub fn update_dependency(&mut self, name: &str, version: &str) -> Result<bool> {
        if let Some(dep) = self.dependencies.get_mut(name) {
            dep.version = Some(version.to_string());

            // Update the file
            self.update_file()?;

            return Ok(true);
        }

        Ok(false)
    }

    /// Check if a dependency exists
    ///
    /// Checks whether the specified dependency is present in Cargo.toml.
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name to check
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the dependency exists.
    /// Returns `Ok(false)` if the dependency doesn't exist.
    /// Returns an error if the check fails.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// if manager.check_dependency_exists("serde")? {
    ///     println!("serde is installed");
    /// }
    /// ```
    pub fn check_dependency_exists(&self, name: &str) -> Result<bool> {
        Ok(self.dependencies.contains_key(name))
    }

    /// Get dependency information
    ///
    /// Retrieves detailed information about a specific dependency.
    ///
    /// # Arguments
    ///
    /// * `name` - The dependency name
    ///
    /// # Returns
    ///
    /// Returns `Some(DependencyInfo)` if the dependency exists.
    /// Returns `None` if the dependency doesn't exist.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// if let Some(dep) = manager.get_dependency("serde")? {
    ///     println!("serde version: {:?}", dep.version);
    /// }
    /// ```
    pub fn get_dependency(&self, name: &str) -> Option<&DependencyInfo> {
        self.dependencies.get(name)
    }

    /// List all dependencies
    ///
    /// Returns a list of all dependencies in the Cargo.toml file.
    ///
    /// # Returns
    ///
    /// Returns a vector of DependencyInfo for all dependencies.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let manager = DependencyManager::new("Cargo.toml")?;
    ///
    /// for dep in manager.list_dependencies() {
    ///     println!("{}: {:?}", dep.name, dep.version);
    /// }
    /// ```
    pub fn list_dependencies(&self) -> Vec<&DependencyInfo> {
        self.dependencies.values().collect()
    }

    /// Update the Cargo.toml file
    ///
    /// Writes the current state of dependencies back to the Cargo.toml file.
    /// Preserves other sections and comments.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    /// Returns an error if writing fails.
    fn update_file(&self) -> Result<()> {
        let lines: Vec<String> = self.raw_content.lines().map(String::from).collect();
        let mut in_deps_section = false;
        let mut deps_start = None;
        let mut deps_end = None;

        // Find dependencies section boundaries
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with('[') {
                if in_deps_section {
                    deps_end = Some(i);
                    break;
                }
                if trimmed == "[dependencies]" {
                    in_deps_section = true;
                    deps_start = Some(i);
                }
            }
        }

        // If we're in deps section and reached end of file
        if in_deps_section && deps_end.is_none() {
            deps_end = Some(lines.len());
        }

        // Build new content
        let mut new_lines = Vec::new();

        if let (Some(start), Some(end)) = (deps_start, deps_end) {
            // Copy lines before dependencies section
            new_lines.extend(lines[..=start].iter().cloned());

            // Add dependencies
            for dep in self.dependencies.values() {
                new_lines.push(dep.to_toml_string());
            }

            // Copy lines after dependencies section
            new_lines.extend(lines[end..].iter().cloned());
        } else {
            // No dependencies section found, add one
            new_lines.extend(lines.iter().cloned());
            new_lines.push(String::new());
            new_lines.push("[dependencies]".to_string());
            for dep in self.dependencies.values() {
                new_lines.push(dep.to_toml_string());
            }
        }

        // Write back to file
        let new_content = new_lines.join("\n");
        fs::write(&self.cargo_toml_path, new_content).with_context(|| {
            format!(
                "Failed to write Cargo.toml: {}",
                self.cargo_toml_path.display()
            )
        })?;

        Ok(())
    }

    /// Save changes to a different file
    ///
    /// Writes the current state to a different Cargo.toml file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to save the new Cargo.toml file
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut manager = DependencyManager::new("Cargo.toml")?;
    /// manager.add_dependency("serde", "1.0", None)?;
    /// manager.save_as("Cargo.toml.new")?;
    /// ```
    pub fn save_as<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let lines: Vec<String> = self.raw_content.lines().map(String::from).collect();
        let mut in_deps_section = false;
        let mut deps_start = None;
        let mut deps_end = None;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            if trimmed.starts_with('[') {
                if in_deps_section {
                    deps_end = Some(i);
                    break;
                }
                if trimmed == "[dependencies]" {
                    in_deps_section = true;
                    deps_start = Some(i);
                }
            }
        }

        if in_deps_section && deps_end.is_none() {
            deps_end = Some(lines.len());
        }

        let mut new_lines = Vec::new();

        if let (Some(start), Some(end)) = (deps_start, deps_end) {
            new_lines.extend(lines[..=start].iter().cloned());

            for dep in self.dependencies.values() {
                new_lines.push(dep.to_toml_string());
            }

            new_lines.extend(lines[end..].iter().cloned());
        } else {
            new_lines.extend(lines.iter().cloned());
            new_lines.push(String::new());
            new_lines.push("[dependencies]".to_string());
            for dep in self.dependencies.values() {
                new_lines.push(dep.to_toml_string());
            }
        }

        let new_content = new_lines.join("\n");
        fs::write(path.as_ref(), new_content)
            .with_context(|| format!("Failed to write file: {}", path.as_ref().display()))?;

        Ok(())
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dependency_info_new() {
        let dep = DependencyInfo::new("serde");
        assert_eq!(dep.name, "serde");
        assert!(dep.version.is_none());
        assert!(dep.features.is_none());
    }

    #[test]
    fn test_dependency_info_with_version() {
        let dep = DependencyInfo::with_version("serde", "1.0");
        assert_eq!(dep.name, "serde");
        assert_eq!(dep.version, Some("1.0".to_string()));
    }

    #[test]
    fn test_dependency_info_with_features() {
        let dep = DependencyInfo::with_features("serde", "1.0", &["derive", "rc"]);
        assert_eq!(dep.name, "serde");
        assert_eq!(dep.version, Some("1.0".to_string()));
        assert_eq!(
            dep.features,
            Some(vec!["derive".to_string(), "rc".to_string()])
        );
    }

    #[test]
    fn test_dependency_info_to_toml_string_simple() {
        let dep = DependencyInfo::with_version("serde", "1.0");
        assert_eq!(dep.to_toml_string(), "serde = \"1.0\"");
    }

    #[test]
    fn test_dependency_info_to_toml_string_with_features() {
        let dep = DependencyInfo::with_features("serde", "1.0", &["derive"]);
        assert_eq!(
            dep.to_toml_string(),
            "serde = { version = \"1.0\", features = [\"derive\"] }"
        );
    }

    #[test]
    fn test_dependency_info_to_toml_string_no_default_features() {
        let mut dep = DependencyInfo::with_version("serde", "1.0");
        dep.default_features = false;
        assert_eq!(
            dep.to_toml_string(),
            "serde = { version = \"1.0\", default-features = false }"
        );
    }

    #[test]
    fn test_dependency_info_to_toml_string_optional() {
        let mut dep = DependencyInfo::with_version("serde", "1.0");
        dep.optional = true;
        assert_eq!(
            dep.to_toml_string(),
            "serde = { version = \"1.0\", optional = true }"
        );
    }

    #[test]
    fn test_parse_dependency_line_simple() {
        let result = DependencyManager::parse_dependency_line("serde = \"1.0\"");
        assert!(result.is_some());

        let (name, dep) = result.unwrap();
        assert_eq!(name, "serde");
        assert_eq!(dep.version, Some("1.0".to_string()));
    }

    #[test]
    fn test_parse_dependency_line_table() {
        let result =
            DependencyManager::parse_dependency_line("serde = { version = \"1.0\", features = [\"derive\"] }");
        assert!(result.is_some());

        let (name, dep) = result.unwrap();
        assert_eq!(name, "serde");
        assert_eq!(dep.version, Some("1.0".to_string()));
        assert_eq!(dep.features, Some(vec!["derive".to_string()]));
    }
}
