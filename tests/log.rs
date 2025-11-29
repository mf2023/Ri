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

use dms::log::{DMSLogContext, DMSLogLevel, DMSLogConfig, DMSLogger};
use dms::fs::DMSFileSystem;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn test_log_level_as_str() {
    assert_eq!(DMSLogLevel::Debug._Fas_str(), "DEBUG");
    assert_eq!(DMSLogLevel::Info._Fas_str(), "INFO");
    assert_eq!(DMSLogLevel::Warn._Fas_str(), "WARN");
    assert_eq!(DMSLogLevel::Error._Fas_str(), "ERROR");
}

#[test]
fn test_log_config_default() {
    let config = DMSLogConfig::_Fdefault();
    assert_eq!(config.level, DMSLogLevel::Info);
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
    DMSLogContext::_Fclear();
    
    // Test put and get
    DMSLogContext::_Fput("test_key", "test_value");
    assert_eq!(DMSLogContext::_Fget("test_key"), Some("test_value".to_string()));
    
    // Test non-existent key
    assert_eq!(DMSLogContext::_Fget("non_existent_key"), None);
    
    // Test remove
    DMSLogContext::_Fremove("test_key");
    assert_eq!(DMSLogContext::_Fget("test_key"), None);
}

#[test]
fn test_log_context_put_all() {
    // Clear any existing context first
    DMSLogContext::_Fclear();
    
    let mut values = std::collections::HashMap::new();
    values.insert("key1".to_string(), "value1".to_string());
    values.insert("key2".to_string(), "value2".to_string());
    
    DMSLogContext::_Fput_all(values);
    assert_eq!(DMSLogContext::_Fget("key1"), Some("value1".to_string()));
    assert_eq!(DMSLogContext::_Fget("key2"), Some("value2".to_string()));
}

#[test]
fn test_log_context_get_all() {
    // Clear any existing context first
    DMSLogContext::_Fclear();
    
    DMSLogContext::_Fput("key1", "value1");
    DMSLogContext::_Fput("key2", "value2");
    
    let all = DMSLogContext::_Fget_all();
    assert_eq!(all.get("key1"), Some(&"value1".to_string()));
    assert_eq!(all.get("key2"), Some(&"value2".to_string()));
    assert_eq!(all.len(), 2);
}

#[test]
fn test_log_context_clear() {
    // Clear any existing context first
    DMSLogContext::_Fclear();
    
    DMSLogContext::_Fput("key1", "value1");
    DMSLogContext::_Fput("key2", "value2");
    assert_eq!(DMSLogContext::_Fget_all().len(), 2);
    
    DMSLogContext::_Fclear();
    assert_eq!(DMSLogContext::_Fget_all().len(), 0);
}

#[test]
fn test_log_context_tracing_support() {
    // Clear any existing context first
    DMSLogContext::_Fclear();
    
    // Test trace id
    let trace_id = "test-trace-id-123";
    DMSLogContext::_Fset_trace_id(trace_id);
    assert_eq!(DMSLogContext::_Fget_trace_id(), Some(trace_id.to_string()));
    
    // Test span id
    let span_id = "test-span-id-456";
    DMSLogContext::_Fset_span_id(span_id);
    assert_eq!(DMSLogContext::_Fget_span_id(), Some(span_id.to_string()));
    
    // Test parent span id
    let parent_span_id = "test-parent-span-id-789";
    DMSLogContext::_Fset_parent_span_id(parent_span_id);
    assert_eq!(DMSLogContext::_Fget_parent_span_id(), Some(parent_span_id.to_string()));
    
    // Test generate trace id
    let generated_trace_id = DMSLogContext::_Fgenerate_trace_id();
    assert!(!generated_trace_id.is_empty());
    
    // Test generate span id
    let generated_span_id = DMSLogContext::_Fgenerate_span_id();
    assert!(!generated_span_id.is_empty());
}

#[test]
fn test_logger_creation() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    let config = DMSLogConfig::_Fdefault();
    let logger = DMSLogger::_Fnew(&config, fs);
    // Just test that creation works without panicking
    assert!(logger._Finfo("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_different_levels() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    // Test with Info level
    let mut config = DMSLogConfig::_Fdefault();
    config.level = DMSLogLevel::Info;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::_Fnew(&config, fs.clone());
    
    // All levels should work without errors
    assert!(logger._Fdebug("test_target", "debug_message").is_ok());
    assert!(logger._Finfo("test_target", "info_message").is_ok());
    assert!(logger._Fwarn("test_target", "warn_message").is_ok());
    assert!(logger._Ferror("test_target", "error_message").is_ok());
    
    // Test with Error level
    config.level = DMSLogLevel::Error;
    let logger = DMSLogger::_Fnew(&config, fs);
    
    // All levels should still work without errors
    assert!(logger._Fdebug("test_target", "debug_message").is_ok());
    assert!(logger._Finfo("test_target", "info_message").is_ok());
    assert!(logger._Fwarn("test_target", "warn_message").is_ok());
    assert!(logger._Ferror("test_target", "error_message").is_ok());
}

#[test]
fn test_logger_with_json_format() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    let mut config = DMSLogConfig::_Fdefault();
    config.json_format = true;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::_Fnew(&config, fs);
    
    // Should work without errors
    assert!(logger._Finfo("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_with_context() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSFileSystem::_Fnew_with_root(temp_dir.path().to_path_buf());
    
    let config = DMSLogConfig::_Fdefault();
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSLogger::_Fnew(&config, fs);
    
    // Clear any existing context first
    DMSLogContext::_Fclear();
    
    // Set context and log
    DMSLogContext::_Fput("test_context_key", "test_context_value");
    assert!(logger._Finfo("test_target", "test_message").is_ok());
    
    // Clear context
    DMSLogContext::_Fclear();
}
