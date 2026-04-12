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

//! Configuration Parser
//!
//! This module provides parsing capabilities for Ri configuration files.
//! It supports both YAML and JSON formats with automatic format detection.
//!
//! # Supported Formats
//!
//! - **YAML**: Primary format for Ri configuration files (`.yaml`, `.yml`)
//! - **JSON**: Alternative format for configuration files (`.json`)
//! - **TOML**: Supported for compatibility (`.toml`)
//!
//! # Usage
//!
//! ```rust,ignore
//! use ric::config::parser::{ConfigParser, ConfigFormat};
//!
//! // Create a parser
//! let parser = ConfigParser::new();
//!
//! // Parse a configuration file
//! let config = parser.parse_file("config.yaml")?;
//!
//! // Parse configuration content directly
//! let content = r#"
//! cache:
//!   enabled: true
//!   backend_type: Memory
//! "#;
//! let config = parser.parse_yaml(content)?;
//!
//! // Detect format from file extension
//! let format = ConfigParser::detect_format("config.json");
//! assert_eq!(format, Some(ConfigFormat::Json));
//! ```
//!
//! # Design Principles
//!
//! - **Format Agnostic**: Works with multiple configuration formats
//! - **Auto Detection**: Automatically detects format from file extension
//! - **Error Recovery**: Provides detailed error messages for parsing failures
//! - **Type Preservation**: Preserves type information during parsing

use std::path::Path;

use crate::error::{Result, RicError};

/// Configuration file format enumeration.
///
/// This enum defines the supported configuration file formats
/// for Ri configuration files.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    /// YAML format (default for Ri)
    Yaml,
    /// JSON format
    Json,
    /// TOML format
    Toml,
}

impl std::fmt::Display for ConfigFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigFormat::Yaml => write!(f, "YAML"),
            ConfigFormat::Json => write!(f, "JSON"),
            ConfigFormat::Toml => write!(f, "TOML"),
        }
    }
}

impl ConfigFormat {
    /// Returns the default file extension for this format.
    ///
    /// # Returns
    ///
    /// The file extension including the dot (e.g., ".yaml")
    pub fn default_extension(&self) -> &'static str {
        match self {
            ConfigFormat::Yaml => ".yaml",
            ConfigFormat::Json => ".json",
            ConfigFormat::Toml => ".toml",
        }
    }

    /// Returns all supported file extensions for this format.
    ///
    /// # Returns
    ///
    /// A vector of file extensions including the dot
    pub fn extensions(&self) -> Vec<&'static str> {
        match self {
            ConfigFormat::Yaml => vec![".yaml", ".yml"],
            ConfigFormat::Json => vec![".json"],
            ConfigFormat::Toml => vec![".toml"],
        }
    }

    /// Returns the MIME type for this format.
    ///
    /// # Returns
    ///
    /// The MIME type string
    pub fn mime_type(&self) -> &'static str {
        match self {
            ConfigFormat::Yaml => "application/x-yaml",
            ConfigFormat::Json => "application/json",
            ConfigFormat::Toml => "application/toml",
        }
    }
}

/// Parsed configuration structure.
///
/// This struct represents a parsed configuration file with
/// metadata about the source and format.
#[derive(Debug, Clone)]
pub struct ParsedConfig {
    /// The parsed configuration value
    pub value: serde_yaml::Value,
    /// The format of the source file
    pub format: ConfigFormat,
    /// Path to the source file (if parsed from file)
    pub source_path: Option<String>,
    /// Raw content of the configuration
    pub raw_content: Option<String>,
}

impl ParsedConfig {
    /// Creates a new parsed configuration.
    ///
    /// # Parameters
    ///
    /// - `value`: The parsed YAML value
    /// - `format`: The format of the source
    ///
    /// # Returns
    ///
    /// A new `ParsedConfig` instance
    pub fn new(value: serde_yaml::Value, format: ConfigFormat) -> Self {
        Self {
            value,
            format,
            source_path: None,
            raw_content: None,
        }
    }

    /// Sets the source file path.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the source file
    ///
    /// # Returns
    ///
    /// The updated `ParsedConfig` instance
    pub fn with_source(mut self, path: impl Into<String>) -> Self {
        self.source_path = Some(path.into());
        self
    }

    /// Sets the raw content.
    ///
    /// # Parameters
    ///
    /// - `content`: The raw configuration content
    ///
    /// # Returns
    ///
    /// The updated `ParsedConfig` instance
    pub fn with_raw_content(mut self, content: impl Into<String>) -> Self {
        self.raw_content = Some(content.into());
        self
    }

    /// Gets a value from the configuration by path.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to the value (e.g., "cache.enabled")
    ///
    /// # Returns
    ///
    /// An optional reference to the value
    pub fn get(&self, path: &str) -> Option<&serde_yaml::Value> {
        let parts: Vec<&str> = path.split('.').collect();
        let mut current = &self.value;

        for part in parts {
            match current {
                serde_yaml::Value::Mapping(map) => {
                    current = map.get(&serde_yaml::Value::String(part.to_string()))?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Gets a string value from the configuration by path.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// An optional string value
    pub fn get_str(&self, path: &str) -> Option<&str> {
        self.get(path).and_then(|v| v.as_str())
    }

    /// Gets an integer value from the configuration by path.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// An optional i64 value
    pub fn get_i64(&self, path: &str) -> Option<i64> {
        self.get(path).and_then(|v| v.as_i64())
    }

    /// Gets a boolean value from the configuration by path.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// An optional bool value
    pub fn get_bool(&self, path: &str) -> Option<bool> {
        self.get(path).and_then(|v| v.as_bool())
    }

    /// Gets a float value from the configuration by path.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to the value
    ///
    /// # Returns
    ///
    /// An optional f64 value
    pub fn get_f64(&self, path: &str) -> Option<f64> {
        self.get(path).and_then(|v| v.as_f64())
    }

    /// Checks if a path exists in the configuration.
    ///
    /// # Parameters
    ///
    /// - `path`: Dot-separated path to check
    ///
    /// # Returns
    ///
    /// `true` if the path exists
    pub fn has(&self, path: &str) -> bool {
        self.get(path).is_some()
    }

    /// Returns all top-level keys in the configuration.
    ///
    /// # Returns
    ///
    /// A vector of top-level key names
    pub fn top_level_keys(&self) -> Vec<String> {
        match &self.value {
            serde_yaml::Value::Mapping(map) => {
                map.keys()
                    .filter_map(|k| k.as_str().map(|s| s.to_string()))
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    /// Converts the configuration to a pretty-printed string.
    ///
    /// # Returns
    ///
    /// A YAML-formatted string of the configuration
    pub fn to_yaml_string(&self) -> Result<String> {
        serde_yaml::to_string(&self.value)
            .map_err(|e| RicError::Yaml(e))
    }

    /// Converts the configuration to a JSON string.
    ///
    /// # Returns
    ///
    /// A JSON-formatted string of the configuration
    pub fn to_json_string(&self) -> Result<String> {
        serde_json::to_string_pretty(&self.value)
            .map_err(|e| RicError::Json(e))
    }
}

/// Configuration file parser.
///
/// This struct provides methods for parsing configuration files
/// in various formats (YAML, JSON, TOML).
pub struct ConfigParser {
    /// Default format to use when format cannot be detected
    default_format: ConfigFormat,
}

impl Default for ConfigParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigParser {
    /// Creates a new configuration parser with YAML as default format.
    ///
    /// # Returns
    ///
    /// A new `ConfigParser` instance
    pub fn new() -> Self {
        Self {
            default_format: ConfigFormat::Yaml,
        }
    }

    /// Creates a new configuration parser with a custom default format.
    ///
    /// # Parameters
    ///
    /// - `format`: The default format to use
    ///
    /// # Returns
    ///
    /// A new `ConfigParser` instance
    pub fn with_default_format(format: ConfigFormat) -> Self {
        Self {
            default_format: format,
        }
    }

    /// Detects the configuration format from a file path.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the configuration file
    ///
    /// # Returns
    ///
    /// The detected format, or None if the extension is not recognized
    pub fn detect_format<P: AsRef<Path>>(path: P) -> Option<ConfigFormat> {
        let path = path.as_ref();
        let extension = path.extension()?.to_str()?.to_lowercase();

        match extension.as_str() {
            "yaml" | "yml" => Some(ConfigFormat::Yaml),
            "json" => Some(ConfigFormat::Json),
            "toml" => Some(ConfigFormat::Toml),
            _ => None,
        }
    }

    /// Detects the configuration format from content.
    ///
    /// This method attempts to detect the format by analyzing the content.
    /// It looks for common patterns in each format.
    ///
    /// # Parameters
    ///
    /// - `content`: The configuration content
    ///
    /// # Returns
    ///
    /// The detected format
    pub fn detect_format_from_content(content: &str) -> ConfigFormat {
        let trimmed = content.trim();

        // JSON typically starts with { or [
        if trimmed.starts_with('{') || trimmed.starts_with('[') {
            // Try to parse as JSON first
            if serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                return ConfigFormat::Json;
            }
        }

        // TOML typically has key = value pairs on the first line
        // or starts with [section]
        if trimmed.starts_with('[') && trimmed.contains(']') && !trimmed.contains(':') {
            // Could be TOML section header
            if let Some(first_line) = trimmed.lines().next() {
                if first_line.starts_with('[') && first_line.ends_with(']') {
                    return ConfigFormat::Toml;
                }
            }
        }

        // Check for TOML patterns (key = value)
        if let Some(first_line) = trimmed.lines().next() {
            if first_line.contains('=') && !first_line.contains(':') {
                return ConfigFormat::Toml;
            }
        }

        // Default to YAML
        ConfigFormat::Yaml
    }

    /// Parses YAML configuration content.
    ///
    /// # Parameters
    ///
    /// - `content`: YAML content to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the YAML is invalid
    pub fn parse_yaml(&self, content: &str) -> Result<ParsedConfig> {
        let value = serde_yaml::from_str(content)
            .map_err(|e| RicError::Yaml(e))?;

        Ok(ParsedConfig::new(value, ConfigFormat::Yaml)
            .with_raw_content(content))
    }

    /// Parses JSON configuration content.
    ///
    /// # Parameters
    ///
    /// - `content`: JSON content to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the JSON is invalid
    pub fn parse_json(&self, content: &str) -> Result<ParsedConfig> {
        // Parse as JSON first, then convert to YAML value for consistency
        let json_value: serde_json::Value = serde_json::from_str(content)
            .map_err(|e| RicError::Json(e))?;

        // Convert JSON to YAML value
        let yaml_value = serde_yaml::to_value(&json_value)
            .map_err(|e| RicError::Yaml(e))?;

        Ok(ParsedConfig::new(yaml_value, ConfigFormat::Json)
            .with_raw_content(content))
    }

    /// Parses TOML configuration content.
    ///
    /// # Parameters
    ///
    /// - `content`: TOML content to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the TOML is invalid
    pub fn parse_toml(&self, content: &str) -> Result<ParsedConfig> {
        // Parse TOML to a generic value
        let toml_value: toml::Value = toml::from_str(content)
            .map_err(|e| RicError::ConfigInvalid(format!("TOML parsing error: {}", e)))?;

        // Convert TOML to JSON, then to YAML for consistency
        let json_str = serde_json::to_string(&toml_value)
            .map_err(|e| RicError::Json(e))?;
        
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&json_str)
            .map_err(|e| RicError::Yaml(e))?;

        Ok(ParsedConfig::new(yaml_value, ConfigFormat::Toml)
            .with_raw_content(content))
    }

    /// Parses configuration content with automatic format detection.
    ///
    /// # Parameters
    ///
    /// - `content`: Configuration content to parse
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails
    pub fn parse(&self, content: &str) -> Result<ParsedConfig> {
        let format = Self::detect_format_from_content(content);
        self.parse_with_format(content, format)
    }

    /// Parses configuration content with a specified format.
    ///
    /// # Parameters
    ///
    /// - `content`: Configuration content to parse
    /// - `format`: The format to use for parsing
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if parsing fails
    pub fn parse_with_format(&self, content: &str, format: ConfigFormat) -> Result<ParsedConfig> {
        match format {
            ConfigFormat::Yaml => self.parse_yaml(content),
            ConfigFormat::Json => self.parse_json(content),
            ConfigFormat::Toml => self.parse_toml(content),
        }
    }

    /// Parses a configuration file.
    ///
    /// This method reads the file, detects the format from the extension,
    /// and parses the content.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the configuration file
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed configuration
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or parsed
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<ParsedConfig> {
        let path = path.as_ref();
        
        // Read file content
        let content = std::fs::read_to_string(path)
            .map_err(|e| RicError::Io(e))?;

        // Detect format from file extension
        let format = Self::detect_format(path).unwrap_or(self.default_format);

        // Parse the content
        let mut config = self.parse_with_format(&content, format)?;
        config.source_path = Some(path.to_string_lossy().to_string());

        Ok(config)
    }

    /// Parses multiple configuration files and merges them.
    ///
    /// Files are merged in order, with later files overriding earlier ones.
    /// This is useful for layered configuration (e.g., default + environment-specific).
    ///
    /// # Parameters
    ///
    /// - `paths`: Paths to configuration files in merge order
    ///
    /// # Returns
    ///
    /// A `Result` containing the merged configuration
    ///
    /// # Errors
    ///
    /// Returns an error if any file cannot be read or parsed
    pub fn parse_and_merge<P: AsRef<Path>>(&self, paths: &[P]) -> Result<ParsedConfig> {
        let mut merged = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
        let mut last_format = self.default_format;
        let mut last_path: Option<String> = None;

        for path in paths {
            let config = self.parse_file(path)?;
            last_format = config.format;
            last_path = config.source_path.clone();

            // Merge the values
            if let serde_yaml::Value::Mapping(new_map) = config.value {
                if let serde_yaml::Value::Mapping(ref mut merged_map) = merged {
                    for (key, value) in new_map {
                        merged_map.insert(key, value);
                    }
                }
            }
        }

        Ok(ParsedConfig {
            value: merged,
            format: last_format,
            source_path: last_path,
            raw_content: None,
        })
    }

    /// Validates that a configuration file exists and is readable.
    ///
    /// # Parameters
    ///
    /// - `path`: Path to the configuration file
    ///
    /// # Returns
    ///
    /// `true` if the file exists and is readable
    pub fn file_exists<P: AsRef<Path>>(path: P) -> bool {
        std::fs::metadata(path).map(|m| m.is_file()).unwrap_or(false)
    }

    /// Returns the default configuration format.
    ///
    /// # Returns
    ///
    /// The default format for this parser
    pub fn default_format(&self) -> ConfigFormat {
        self.default_format
    }
}

/// Helper function to create a parser with default settings.
///
/// # Returns
///
/// A new `ConfigParser` instance
pub fn parser() -> ConfigParser {
    ConfigParser::new()
}

/// Helper function to parse a configuration file.
///
/// # Parameters
///
/// - `path`: Path to the configuration file
///
/// # Returns
///
/// A `Result` containing the parsed configuration
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<ParsedConfig> {
    ConfigParser::new().parse_file(path)
}

/// Helper function to parse configuration content.
///
/// # Parameters
///
/// - `content`: Configuration content
///
/// # Returns
///
/// A `Result` containing the parsed configuration
pub fn parse(content: &str) -> Result<ParsedConfig> {
    ConfigParser::new().parse(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_format_yaml() {
        assert_eq!(ConfigParser::detect_format("config.yaml"), Some(ConfigFormat::Yaml));
        assert_eq!(ConfigParser::detect_format("config.yml"), Some(ConfigFormat::Yaml));
    }

    #[test]
    fn test_detect_format_json() {
        assert_eq!(ConfigParser::detect_format("config.json"), Some(ConfigFormat::Json));
    }

    #[test]
    fn test_detect_format_toml() {
        assert_eq!(ConfigParser::detect_format("config.toml"), Some(ConfigFormat::Toml));
    }

    #[test]
    fn test_detect_format_unknown() {
        assert_eq!(ConfigParser::detect_format("config.txt"), None);
        assert_eq!(ConfigParser::detect_format("config"), None);
    }

    #[test]
    fn test_detect_format_from_content_json() {
        let json = r#"{"key": "value"}"#;
        assert_eq!(ConfigParser::detect_format_from_content(json), ConfigFormat::Json);
    }

    #[test]
    fn test_detect_format_from_content_yaml() {
        let yaml = "key: value\nnumber: 42";
        assert_eq!(ConfigParser::detect_format_from_content(yaml), ConfigFormat::Yaml);
    }

    #[test]
    fn test_parse_yaml() {
        let parser = ConfigParser::new();
        let yaml = "key: value\nnumber: 42";
        let result = parser.parse_yaml(yaml);
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get_str("key"), Some("value"));
        assert_eq!(config.get_i64("number"), Some(42));
    }

    #[test]
    fn test_parse_json() {
        let parser = ConfigParser::new();
        let json = r#"{"key": "value", "number": 42}"#;
        let result = parser.parse_json(json);
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get_str("key"), Some("value"));
        assert_eq!(config.get_i64("number"), Some(42));
    }

    #[test]
    fn test_parse_toml() {
        let parser = ConfigParser::new();
        let toml = "key = \"value\"\nnumber = 42";
        let result = parser.parse_toml(toml);
        assert!(result.is_ok());
        
        let config = result.unwrap();
        assert_eq!(config.get_str("key"), Some("value"));
        assert_eq!(config.get_i64("number"), Some(42));
    }

    #[test]
    fn test_parsed_config_get() {
        let parser = ConfigParser::new();
        let yaml = r#"
cache:
  enabled: true
  backend_type: Memory
  max_memory_mb: 512
"#;
        let config = parser.parse_yaml(yaml).unwrap();

        assert_eq!(config.get_bool("cache.enabled"), Some(true));
        assert_eq!(config.get_str("cache.backend_type"), Some("Memory"));
        assert_eq!(config.get_i64("cache.max_memory_mb"), Some(512));
    }

    #[test]
    fn test_parsed_config_has() {
        let parser = ConfigParser::new();
        let yaml = "cache:\n  enabled: true";
        let config = parser.parse_yaml(yaml).unwrap();

        assert!(config.has("cache"));
        assert!(config.has("cache.enabled"));
        assert!(!config.has("cache.nonexistent"));
    }

    #[test]
    fn test_parsed_config_top_level_keys() {
        let parser = ConfigParser::new();
        let yaml = "cache:\n  enabled: true\nqueue:\n  enabled: true";
        let config = parser.parse_yaml(yaml).unwrap();

        let keys = config.top_level_keys();
        assert!(keys.contains(&"cache".to_string()));
        assert!(keys.contains(&"queue".to_string()));
    }

    #[test]
    fn test_parsed_config_to_yaml_string() {
        let parser = ConfigParser::new();
        let yaml = "key: value";
        let config = parser.parse_yaml(yaml).unwrap();

        let result = config.to_yaml_string();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("key"));
    }

    #[test]
    fn test_parsed_config_to_json_string() {
        let parser = ConfigParser::new();
        let yaml = "key: value";
        let config = parser.parse_yaml(yaml).unwrap();

        let result = config.to_json_string();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("key"));
    }

    #[test]
    fn test_config_format_extensions() {
        assert!(ConfigFormat::Yaml.extensions().contains(&".yaml"));
        assert!(ConfigFormat::Yaml.extensions().contains(&".yml"));
        assert!(ConfigFormat::Json.extensions().contains(&".json"));
        assert!(ConfigFormat::Toml.extensions().contains(&".toml"));
    }

    #[test]
    fn test_config_format_mime_type() {
        assert_eq!(ConfigFormat::Yaml.mime_type(), "application/x-yaml");
        assert_eq!(ConfigFormat::Json.mime_type(), "application/json");
        assert_eq!(ConfigFormat::Toml.mime_type(), "application/toml");
    }

    #[test]
    fn test_parser_default_format() {
        let parser = ConfigParser::new();
        assert_eq!(parser.default_format(), ConfigFormat::Yaml);

        let parser = ConfigParser::with_default_format(ConfigFormat::Json);
        assert_eq!(parser.default_format(), ConfigFormat::Json);
    }
}
