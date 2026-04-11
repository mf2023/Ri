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

//! # Configuration Module Tests
//!
//! This module contains comprehensive tests for the Ri configuration system,
//! covering configuration storage, retrieval with type conversion, merging operations,
//! and the configuration manager with multiple source support.
//!
//! ## Test Coverage
//!
//! - **RiConfig**: Tests for in-memory key-value configuration storage including
//!   basic operations (new, set, get), type-safe retrieval methods (get_str, get_bool,
//!   get_i64, get_u64, get_f32), configuration merging, and clearing operations
//! - **RiConfigManager**: Tests for the configuration manager that supports multiple
//!   configuration sources including file-based and environment-based sources, with
//!   hot-reload capabilities through the watcher functionality
//!
//! ## Configuration Types
//!
//! The configuration system supports various data types through type conversion:
//! - **Boolean**: Accepts "true", "1", "yes", "on" (case-insensitive) for true;
//!   "false", "0", "no", "off" for false
//! - **Integer (i64)**: Parses signed 64-bit integers with support for negative values
//! - **Unsigned Integer (u64)**: Parses non-negative 64-bit integers, rejects negatives
//! - **Floating Point (f32)**: Parses 32-bit floating point numbers with decimal support
//!
//! ## Design Notes
//!
//! Tests verify both successful conversions and error handling for invalid inputs.
//! The merge operation follows a last-wins semantics where values from the source
//! configuration override existing values in the target configuration.

use ri::config::{RiConfig, RiConfigManager};
use tempfile::tempdir;

#[test]
/// Tests RiConfig creation with new() constructor.
///
/// Verifies that a newly created RiConfig instance is empty and
/// does not contain any configuration values. The get() method
/// should return None for all non-existent keys.
///
/// ## Expected Behavior
///
/// - New config has no entries
/// - get() returns None for all keys
/// - The config is ready for configuration values to be added
///
/// ## Test Steps
///
/// 1. Create a new RiConfig using new()
/// 2. Attempt to retrieve a non-existent key
/// 3. Verify the retrieval returns None
fn test_config_new() {
    let config = RiConfig::new();
    assert!(config.get("non_existent_key").is_none());
}

#[test]
/// Tests basic RiConfig set and get operations.
///
/// Verifies that configuration values can be stored and retrieved
/// correctly using the set() and get() methods. The test covers
/// both successful retrieval of set values and handling of
/// non-existent keys.
///
/// ## Test Scenarios
///
/// 1. **Set and retrieve**: Store a value and verify it can be retrieved
/// 2. **Value integrity**: The retrieved value matches the stored value
/// 3. **Non-existent key**: Getting a non-existent key returns None
///
/// ## Expected Behavior
///
/// - set() stores the key-value pair
/// - get() returns a reference to the stored value
/// - Non-existent keys return None
fn test_config_set_get() {
    let mut config = RiConfig::new();
    
    // Test set and get
    config.set("test_key", "test_value");
    assert_eq!(config.get("test_key"), Some(&"test_value".to_string()));
    
    // Test non-existent key
    assert_eq!(config.get("non_existent_key"), None);
}

#[test]
/// Tests RiConfig type-safe string retrieval with get_str().
///
/// Verifies that the get_str() method correctly retrieves string
/// values from the configuration, returning Some(value) for existing
/// keys and None for non-existent keys.
///
/// ## Expected Behavior
///
/// - get_str() returns Some for existing string keys
/// - get_str() returns None for non-existent keys
/// - The returned value is a string slice
fn test_config_get_str() {
    let mut config = RiConfig::new();
    
    config.set("string_key", "string_value");
    assert_eq!(config.get_str("string_key"), Some("string_value"));
    assert_eq!(config.get_str("non_existent_key"), None);
}

#[test]
/// Tests RiConfig boolean type conversion with get_bool().
///
/// Verifies that the get_bool() method correctly converts string
/// values to boolean values using case-insensitive matching.
/// The method accepts various true/false representations.
///
/// ## Supported Boolean Values
///
/// **True values** (case-insensitive):
/// - "true", "1", "yes", "on"
///
/// **False values** (case-insensitive):
/// - "false", "0", "no", "off"
///
/// ## Test Scenarios
///
/// 1. **True values**: All true representations convert to Some(true)
/// 2. **False values**: All false representations convert to Some(false)
/// 3. **Invalid value**: Invalid boolean strings return None
/// 4. **Non-existent key**: Returns None
///
/// ## Expected Behavior
///
/// - All valid true values return Some(true)
/// - All valid false values return Some(false)
/// - Invalid values return None
/// - Non-existent keys return None
fn test_config_get_bool() {
    let mut config = RiConfig::new();
    
    // Test true values
    let true_values = ["true", "1", "yes", "on"];
    for &val in &true_values {
        config.set("bool_key", val);
        assert_eq!(config.get_bool("bool_key"), Some(true));
    }
    
    // Test false values
    let false_values = ["false", "0", "no", "off"];
    for &val in &false_values {
        config.set("bool_key", val);
        assert_eq!(config.get_bool("bool_key"), Some(false));
    }
    
    // Test invalid boolean value
    config.set("bool_key", "invalid");
    assert_eq!(config.get_bool("bool_key"), None);
    
    // Test non-existent key
    assert_eq!(config.get_bool("non_existent_key"), None);
}

#[test]
/// Tests RiConfig signed 64-bit integer type conversion with get_i64().
///
/// Verifies that the get_i64() method correctly converts string
/// values to signed 64-bit integers, supporting positive and
/// negative values.
///
/// ## Test Scenarios
///
/// 1. **Positive integers**: "123" converts to Some(123)
/// 2. **Negative integers**: "-456" converts to Some(-456)
/// 3. **Invalid format**: Non-numeric strings return None
/// 4. **Non-existent key**: Returns None
///
/// ## Expected Behavior
///
/// - Valid integer strings convert to Some(i64)
/// - Negative signs are correctly handled
/// - Invalid numeric formats return None
/// - Non-existent keys return None
fn test_config_get_i64() {
    let mut config = RiConfig::new();
    
    // Test valid i64 values
    config.set("i64_key_positive", "123");
    assert_eq!(config.get_i64("i64_key_positive"), Some(123));
    
    config.set("i64_key_negative", "-456");
    assert_eq!(config.get_i64("i64_key_negative"), Some(-456));
    
    // Test invalid i64 value
    config.set("i64_key_invalid", "abc");
    assert_eq!(config.get_i64("i64_key_invalid"), None);
    
    // Test non-existent key
    assert_eq!(config.get_i64("non_existent_key"), None);
}

#[test]
/// Tests RiConfig unsigned 64-bit integer type conversion with get_u64().
///
/// Verifies that the get_u64() method correctly converts string
/// values to unsigned 64-bit integers. Unlike get_i64(), this
/// method rejects negative values.
///
/// ## Test Scenarios
///
/// 1. **Valid unsigned**: "123" converts to Some(123)
/// 2. **Negative values**: "-456" returns None (unsigned cannot be negative)
/// 3. **Invalid format**: Non-numeric strings return None
/// 4. **Non-existent key**: Returns None
///
/// ## Expected Behavior
///
/// - Valid non-negative integers convert to Some(u64)
/// - Negative values return None
/// - Invalid numeric formats return None
/// - Non-existent keys return None
fn test_config_get_u64() {
    let mut config = RiConfig::new();
    
    // Test valid u64 values
    config.set("u64_key", "123");
    assert_eq!(config.get_u64("u64_key"), Some(123));
    
    // Test invalid u64 value (negative)
    config.set("u64_key_negative", "-456");
    assert_eq!(config.get_u64("u64_key_negative"), None);
    
    // Test invalid u64 value (non-numeric)
    config.set("u64_key_invalid", "abc");
    assert_eq!(config.get_u64("u64_key_invalid"), None);
    
    // Test non-existent key
    assert_eq!(config.get_u64("non_existent_key"), None);
}

#[test]
/// Tests RiConfig 32-bit floating point type conversion with get_f32().
///
/// Verifies that the get_f32() method correctly converts string
/// values to 32-bit floating point numbers, supporting integers,
/// decimals, and negative values.
///
/// ## Test Scenarios
///
/// 1. **Integer values**: "123" converts to Some(123.0)
/// 2. **Decimal values**: "123.456" converts to Some(123.456)
/// 3. **Negative decimals**: "-789.123" converts to Some(-789.123)
/// 4. **Invalid format**: Non-numeric strings return None
/// 5. **Non-existent key**: Returns None
///
/// ## Expected Behavior
///
/// - Valid float strings convert to Some(f32)
/// - Decimal points are correctly parsed
/// - Negative values are correctly handled
/// - Invalid numeric formats return None
/// - Non-existent keys return None
fn test_config_get_f32() {
    let mut config = RiConfig::new();
    
    // Test valid f32 values
    config.set("f32_key_int", "123");
    assert_eq!(config.get_f32("f32_key_int"), Some(123.0));
    
    config.set("f32_key_float", "123.456");
    assert_eq!(config.get_f32("f32_key_float"), Some(123.456));
    
    config.set("f32_key_negative", "-789.123");
    assert_eq!(config.get_f32("f32_key_negative"), Some(-789.123));
    
    // Test invalid f32 value
    config.set("f32_key_invalid", "abc");
    assert_eq!(config.get_f32("f32_key_invalid"), None);
    
    // Test non-existent key
    assert_eq!(config.get_f32("non_existent_key"), None);
}

#[test]
/// Tests RiConfig merge operation with merge().
///
/// Verifies that the merge() method correctly combines two configurations
/// using last-wins semantics. Values from the source configuration
/// override existing values in the target configuration, and new
/// values are added.
///
/// ## Merge Semantics
///
/// - **Existing keys**: If key exists in both, source value overwrites target
/// - **New keys**: Keys only in source are added to target
/// - **Unchanged keys**: Keys only in target remain unchanged
///
/// ## Test Scenarios
///
/// 1. **Overwrite existing**: key2's value is replaced with source value
/// 2. **Add new key**: key3 is added from source
/// 3. **Preserve existing**: key1's value remains unchanged
///
/// ## Test Steps
///
/// 1. Create config1 with key1 and key2
/// 2. Create config2 with key2 (new value) and key3
/// 3. Merge config2 into config1
/// 4. Verify key1 unchanged, key2 overwritten, key3 added
///
/// ## Expected Behavior
///
/// - key1 retains its original value
/// - key2 gets the new value from config2
/// - key3 is added from config2
fn test_config_merge() {
    let mut config1 = RiConfig::new();
    config1.set("key1", "value1");
    config1.set("key2", "value2");
    
    let mut config2 = RiConfig::new();
    config2.set("key2", "new_value2");
    config2.set("key3", "value3");
    
    config1.merge(&config2);
    
    assert_eq!(config1.get_str("key1"), Some("value1"));
    assert_eq!(config1.get_str("key2"), Some("new_value2")); // Should be overwritten
    assert_eq!(config1.get_str("key3"), Some("value3")); // Should be added
}

#[test]
/// Tests RiConfig clear operation with clear().
///
/// Verifies that the clear() method removes all configuration
/// values from the configuration instance, returning it to
/// an empty state.
///
/// ## Test Scenarios
///
/// 1. **Before clear**: Values are accessible via get methods
/// 2. **After clear**: All values return None
/// 3. **Empty state**: Config is ready for new values
///
/// ## Expected Behavior
///
/// - clear() removes all key-value pairs
/// - After clear, get() returns None for all keys
/// - The config can be reused for new configuration
fn test_config_clear() {
    let mut config = RiConfig::new();
    config.set("key1", "value1");
    config.set("key2", "value2");
    
    assert_eq!(config.get_str("key1"), Some("value1"));
    assert_eq!(config.get_str("key2"), Some("value2"));
    
    config.clear();
    
    assert_eq!(config.get_str("key1"), None);
    assert_eq!(config.get_str("key2"), None);
}

#[test]
/// Tests RiConfigManager creation with new().
///
/// Verifies that a RiConfigManager can be created successfully
/// and starts with an empty configuration that returns None
/// for non-existent keys.
///
/// ## Expected Behavior
///
/// - Manager is created without errors
/// - Initial config is empty
/// - Non-existent keys return None
fn test_config_manager_new() {
    let manager = RiConfigManager::new();
    // Just test that creation works without panicking
    assert!(manager.config().get_str("non_existent_key").is_none());
}

#[test]
/// Tests RiConfigManager adding configuration sources.
///
/// Verifies that the configuration manager supports adding
/// multiple configuration sources including file-based and
/// environment-based sources.
///
/// ## Configuration Sources
///
/// - **File Source**: Loads configuration from YAML files
/// - **Environment Source**: Reads configuration from environment variables
///
/// ## Test Scenarios
///
/// 1. **File source**: Can add a file path as configuration source
/// 2. **Environment source**: Can add environment variables as source
/// 3. **Load operation**: Load completes without errors even if file doesn't exist
///
/// ## Expected Behavior
///
/// - add_file_source() accepts file paths
/// - add_environment_source() enables env var support
/// - load() succeeds regardless of source contents
fn test_config_manager_add_sources() {
    let temp_dir = tempdir().unwrap();
    let mut manager = RiConfigManager::new();
    
    // Test adding file source
    let file_path = temp_dir.path().join("test_config.yaml");
    manager.add_file_source(&file_path);
    
    // Test adding environment source
    manager.add_environment_source();
    
    // Test load (should not panic even if file doesn't exist)
    assert!(manager.load().is_ok());
}

#[test]
/// Tests RiConfigManager creation with new_default().
///
/// Verifies that new_default() creates a configuration manager
/// with default configuration sources pre-configured.
///
/// ## Expected Behavior
///
/// - Manager is created with defaults
/// - Initial config is empty
/// - Non-existent keys return None
fn test_config_manager_new_default() {
    let manager = RiConfigManager::new_default();
    // Just test that creation works without panicking
    assert!(manager.config().get_str("non_existent_key").is_none());
}

#[test]
/// Tests RiConfigManager configuration access methods.
///
/// Verifies that the configuration manager provides both immutable
/// and mutable access to its underlying configuration through
/// config() and config_mut() methods.
///
/// ## Access Methods
///
/// - **config()**: Returns immutable reference to configuration
/// - **config_mut()**: Returns mutable reference to configuration
///
/// ## Test Scenarios
///
/// 1. **Immutable access**: config() returns reference for reading
/// 2. **Mutable access**: config_mut() allows modifications
/// 3. **Change propagation**: Changes via config_mut() are visible via config()
///
/// ## Expected Behavior
///
/// - config() returns &RiConfig for read operations
/// - config_mut() returns &mut RiConfig for write operations
/// - Changes made through config_mut() are visible through config()
fn test_config_manager_config_access() {
    let mut manager = RiConfigManager::new();
    
    // Test config() method
    let config = manager.config();
    assert!(config.get_str("non_existent_key").is_none());
    
    // Test config_mut() method
    let config_mut = manager.config_mut();
    config_mut.set("test_key", "test_value");
    
    // Verify the change is reflected
    assert_eq!(manager.config().get_str("test_key"), Some("test_value"));
}

#[tokio::test]
/// Tests RiConfigManager configuration watcher startup.
///
/// Verifies that the configuration watcher can be started
/// successfully to enable hot-reload of configuration changes.
///
/// ## Hot Reload Capability
///
/// The watcher monitors configuration sources for changes and
/// reloads configuration when changes are detected. This enables
/// applications to pick up configuration changes without restart.
///
/// ## Expected Behavior
///
/// - start_watcher() returns Ok on success
/// - The watcher runs asynchronously
/// - Configuration changes can be detected and reloaded
async fn test_config_manager_start_watcher() {
    let mut manager = RiConfigManager::new();
    // Just test that the simplified implementation works without panicking
    assert!(manager.start_watcher().await.is_ok());
}
