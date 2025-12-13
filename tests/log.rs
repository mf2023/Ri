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

use dms_core::log::{DMSLogContext, DMSLogLevel, DMSLogConfig, DMSLogger};
use dms_core::fs::DMSFileSystem;
use tempfile::tempdir;

#[test]
fn test_log_level_as_str() {
    assert_eq!(DMSLogLevel::Debug.as_str(), "DEBUG");
    assert_eq!(DMSLogLevel::Info.as_str(), "INFO");
    assert_eq!(DMSLogLevel::Warn.as_str(), "WARN");
    assert_eq!(DMSLogLevel::Error.as_str(), "ERROR");
}

#[test]
fn test_log_config_default() {
    let config = DMSLogConfig::default();
    assert_eq!(config.level.as_str(), "INFO");
    assert!(config.console_enabled);
    assert!(config.file_enabled);
    assert_eq!(config.sampling_default, 1.0);
    assert_eq!(config.file_name, "dms.log");
    assert!(!config.json_format);
    assert_eq!(config.rotate_when, "size");
    assert_eq!(config.max_bytes, 10 * 1024 * 1024);
}

#[test]
fn test_log_context_put_get() {
    // Clear any existing context first
    DMSLogContext::clear();
    
    // Test put and get
    DMSLogContext::put("test_key", "test_value");
    assert_eq!(DMSLogContext::get("test_key"), Some("test_value".to_string()));
    
    // Test non-existent key
    assert_eq!(DMSLogContext::get("non_existent_key"), None);
    
    // Test remove
    DMSLogContext::remove("test_key");
    assert_eq!(DMSLogContext::get("test_key"), None);
}

#[test]
fn test_log_context_put_all() {
    // Clear any existing context first
    DMSLogContext::clear();
    
    let mut values = std::collections::HashMap::new();
    values.insert("key1".to_string(), "value1".to_string());
    values.insert("key2".to_string(), "value2".to_string());
    
    DMSLogContext::put_all(values);
    assert_eq!(DMSLogContext::get("key1"), Some("value1".to_string()));
    assert_eq!(DMSLogContext::get("key2"), Some("value2".to_string()));
}

#[test]
fn test_log_context_get_all() {
    // Clear any existing context first
    DMSLogContext::clear();
    
    DMSLogContext::put("key1", "value1");
    DMSLogContext::put("key2", "value2");
    
    let all = DMSLogContext::get_all();
    assert_eq!(all.get("key1"), Some(&"value1".to_string()));
    assert_eq!(all.get("key2"), Some(&"value2".to_string()));
    assert_eq!(all.len(), 2);
}

#[test]
fn test_log_context_clear() {
    // Clear any existing context first
    DMSLogContext::clear();
    
    DMSLogContext::put("key1", "value1");
    DMSLogContext::put("key2", "value2");
    assert_eq!(DMSLogContext::get_all().len(), 2);
    
    DMSLogContext::clear();
    assert_eq!(DMSLogContext::get_all().len(), 0);
}

#[test]
fn test_log_context_tracing_support() {
    // Clear any existing context first
    DMSLogContext::clear();
    
    // Test trace id
    let trace_id = "test-trace-id-123";
    DMSLogContext::set_trace_id(trace_id);
    assert_eq!(DMSLogContext::get_trace_id(), Some(trace_id.to_string()));
    
    // Test span id
    let span_id = "test-span-id-456";
    DMSLogContext::set_span_id(span_id);
    assert_eq!(DMSLogContext::get_span_id(), Some(span_id.to_string()));
    
    // Test parent span id
    let parent_span_id = "test-parent-span-id-789";
    DMSLogContext::set_parent_span_id(parent_span_id);
    assert_eq!(DMSLogContext::get_parent_span_id(), Some(parent_span_id.to_string()));
    
    // Test generate trace id
    let generated_trace_id = DMSLogContext::generate_trace_id();
    assert!(!generated_trace_id.is_empty());
    
    // Test generate span id
    let generated_span_id = DMSLogContext::generate_span_id();
    assert!(!generated_span_id.is_empty());
}

#[test]
fn test_logger_creation() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let config = DMSLogConfig::default();
    let logger = DMSLogger::new(&config, fs);
    // Just test that creation works without panicking
    assert!(logger.info("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_different_levels() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    // Test with Info level
    let mut config = DMSLogConfig::default();
    config.level = DMSLogLevel::Info;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::new(&config, fs.clone());
    
    // All levels should work without errors
    assert!(logger.debug("test_target", "debug_message").is_ok());
    assert!(logger.info("test_target", "info_message").is_ok());
    assert!(logger.warn("test_target", "warn_message").is_ok());
    assert!(logger.error("test_target", "error_message").is_ok());
    
    // Test with Error level
    let mut config = DMSLogConfig::default();
    config.level = DMSLogLevel::Error;
    config.console_enabled = false;
    let logger = DMSLogger::new(&config, fs);
    
    // All levels should still work without errors
    assert!(logger.debug("test_target", "debug_message").is_ok());
    assert!(logger.info("test_target", "info_message").is_ok());
    assert!(logger.warn("test_target", "warn_message").is_ok());
    assert!(logger.error("test_target", "error_message").is_ok());
}

#[test]
fn test_logger_with_json_format() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let mut config = DMSLogConfig::default();
    config.json_format = true;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::new(&config, fs);
    
    // Should work without errors
    assert!(logger.info("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_with_context() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let mut config = DMSLogConfig::default();
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::new(&config, fs);
    
    // Clear any existing context first
    DMSLogContext::clear();
    
    // Set context and log
    DMSLogContext::put("test_context_key", "test_context_value");
    assert!(logger.info("test_target", "test_message").is_ok());
    
    // Clear context
    DMSLogContext::clear();
}
