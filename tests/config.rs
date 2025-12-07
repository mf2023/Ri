// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate dms;

use dms::config::{DMSConfig, DMSConfigManager};
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_config_new() {
    let config = DMSConfig::new();
    assert!(config.get("non_existent_key").is_none());
}

#[test]
fn test_config_set_get() {
    let mut config = DMSConfig::new();
    
    // Test set and get
    config.set("test_key", "test_value");
    assert_eq!(config.get("test_key"), Some(&"test_value".to_string()));
    
    // Test non-existent key
    assert_eq!(config.get("non_existent_key"), None);
}

#[test]
fn test_config_get_str() {
    let mut config = DMSConfig::new();
    
    config.set("string_key", "string_value");
    assert_eq!(config.get_str("string_key"), Some("string_value"));
    assert_eq!(config.get_str("non_existent_key"), None);
}

#[test]
fn test_config_get_bool() {
    let mut config = DMSConfig::new();
    
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
fn test_config_get_i64() {
    let mut config = DMSConfig::new();
    
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
fn test_config_get_u64() {
    let mut config = DMSConfig::new();
    
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
fn test_config_get_f32() {
    let mut config = DMSConfig::new();
    
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
fn test_config_merge() {
    let mut config1 = DMSConfig::new();
    config1.set("key1", "value1");
    config1.set("key2", "value2");
    
    let mut config2 = DMSConfig::new();
    config2.set("key2", "new_value2");
    config2.set("key3", "value3");
    
    config1.merge(&config2);
    
    assert_eq!(config1.get_str("key1"), Some("value1"));
    assert_eq!(config1.get_str("key2"), Some("new_value2")); // Should be overwritten
    assert_eq!(config1.get_str("key3"), Some("value3")); // Should be added
}

#[test]
fn test_config_clear() {
    let mut config = DMSConfig::new();
    config.set("key1", "value1");
    config.set("key2", "value2");
    
    assert_eq!(config.get_str("key1"), Some("value1"));
    assert_eq!(config.get_str("key2"), Some("value2"));
    
    config.clear();
    
    assert_eq!(config.get_str("key1"), None);
    assert_eq!(config.get_str("key2"), None);
}

#[test]
fn test_config_manager_new() {
    let manager = DMSConfigManager::new();
    // Just test that creation works without panicking
    assert!(manager.config().get_str("non_existent_key").is_none());
}

#[test]
fn test_config_manager_add_sources() {
    let temp_dir = tempdir().unwrap();
    let mut manager = DMSConfigManager::new();
    
    // Test adding file source
    let file_path = temp_dir.path().join("test_config.yaml");
    manager.add_file_source(&file_path);
    
    // Test adding environment source
    manager.add_environment_source();
    
    // Test load (should not panic even if file doesn't exist)
    assert!(manager.load().is_ok());
}

#[test]
fn test_config_manager_new_default() {
    let manager = DMSConfigManager::new_default();
    // Just test that creation works without panicking
    assert!(manager.config().get_str("non_existent_key").is_none());
}

#[test]
fn test_config_manager_config_access() {
    let mut manager = DMSConfigManager::new();
    
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
async fn test_config_manager_start_watcher() {
    let mut manager = DMSConfigManager::new();
    // Just test that the simplified implementation works without panicking
    assert!(manager.start_watcher().await.is_ok());
}
