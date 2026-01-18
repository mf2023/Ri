//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Logging Module Tests
//!
//! This module contains comprehensive tests for the DMSC logging system, providing
//! structured, contextual logging capabilities with configurable output destinations,
//! log levels, and sampling strategies for production observability.
//!
//! ## Test Coverage
//!
//! - **DMSCLogLevel**: Tests for log severity levels (Debug, Info, Warn, Error) with
//!   string representations for configuration and display purposes
//!
//! - **DMSCLogConfig**: Tests for logging configuration including level thresholds,
//!   console and file output toggles, sampling rates, log rotation settings, and
//!   format options (plain text vs JSON)
//!
//! - **DMSCLogContext**: Tests for thread-local and global log context management
//!   supporting structured logging with key-value pairs, context inheritance, and
//!   distributed tracing support (trace_id, span_id, parent_span_id)
//!
//! - **DMSCLogger**: Tests for the logging interface including level-based filtering,
//!   contextual log enrichment, JSON formatting, and output routing
//!
//! ## Design Principles
//!
//! The logging system is designed with these principles:
//! - **Structured Logging**: All logs are key-value pairs enabling powerful queries
//!   and analysis in log aggregation systems
//! - **Context Isolation**: Each thread/request has isolated log context that can be
//!   enriched without affecting other concurrent operations
//! - **Performance**: Logging is designed to have minimal impact when disabled,
//!   with sampling support for high-volume debug logging
//! - **Observability Integration**: First-class support for distributed tracing
//!   through context propagation
//!
//! ## Log Levels
//!
//! The system supports standard severity levels:
//! - **Debug**: Detailed information for troubleshooting, typically disabled in production
//! - **Info**: General operational information about system progress
//! - **Warning**: Abnormal conditions that don't prevent operation but may indicate issues
//! - **Error**: Failures that may affect functionality but don't crash the system
//!
//! Each level can be independently configured for console and file outputs.
//!
//! ## Structured Context
//!
//! The log context enables correlation and debugging:
//! - **Key-Value Storage**: Arbitrary metadata can be attached to logs
//! - **Trace Correlation**: Trace IDs and span IDs link logs across service boundaries
//! - **Context Inheritance**: Child threads/spans automatically inherit parent context
//! - **Scoped Modifications**: Context changes can be made locally without affecting
//!   global state
//!
//! ## Log Rotation
//!
//! The file-based logging supports rotation policies:
//! - **Size-based**: Rotate when log file exceeds configured bytes (default 10MB)
//! - **Retention**: Old logs are archived or deleted based on policy
//! - **Atomic Rotation**: Rotation happens atomically to prevent log loss
//!
//! ## JSON Format
//!
//! When json_format is enabled, logs are emitted as JSON objects with:
//! - Timestamp in ISO 8601 format
//! - Log level as string
//! - Target/component name
//! - Message text
//! - All context key-value pairs
//! - Trace and span identifiers (if present)

use dmsc::log::{DMSCLogContext, DMSCLogLevel, DMSCLogConfig, DMSCLogger};
use dmsc::fs::DMSCFileSystem;
use tempfile::tempdir;

#[test]
/// Tests DMSCLogLevel conversion to string with as_str().
///
/// Verifies that each log level correctly returns its string
/// representation for logging output and filtering purposes.
///
/// ## Log Level String Mappings
///
/// - Error -> "ERROR"
/// - Warn -> "WARN"
/// - Info -> "INFO"
/// - Debug -> "DEBUG"
/// - Trace -> "TRACE"
///
/// ## Expected Behavior
///
/// Each level returns the correct uppercase string representation
fn test_log_level_as_str() {
    assert_eq!(DMSCLogLevel::Debug.as_str(), "DEBUG");
    assert_eq!(DMSCLogLevel::Info.as_str(), "INFO");
    assert_eq!(DMSCLogLevel::Warn.as_str(), "WARN");
    assert_eq!(DMSCLogLevel::Error.as_str(), "ERROR");
}

#[test]
fn test_log_config_default() {
    let config = DMSCLogConfig::default();
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
    DMSCLogContext::clear();
    
    // Test put and get
    DMSCLogContext::put("test_key", "test_value");
    assert_eq!(DMSCLogContext::get("test_key"), Some("test_value".to_string()));
    
    // Test non-existent key
    assert_eq!(DMSCLogContext::get("non_existent_key"), None);
    
    // Test remove
    DMSCLogContext::remove("test_key");
    assert_eq!(DMSCLogContext::get("test_key"), None);
}

#[test]
fn test_log_context_put_all() {
    // Clear any existing context first
    DMSCLogContext::clear();
    
    let mut values = std::collections::HashMap::new();
    values.insert("key1".to_string(), "value1".to_string());
    values.insert("key2".to_string(), "value2".to_string());
    
    DMSCLogContext::put_all(values);
    assert_eq!(DMSCLogContext::get("key1"), Some("value1".to_string()));
    assert_eq!(DMSCLogContext::get("key2"), Some("value2".to_string()));
}

#[test]
fn test_log_context_get_all() {
    // Clear any existing context first
    DMSCLogContext::clear();
    
    DMSCLogContext::put("key1", "value1");
    DMSCLogContext::put("key2", "value2");
    
    let all = DMSCLogContext::get_all();
    assert_eq!(all.get("key1"), Some(&"value1".to_string()));
    assert_eq!(all.get("key2"), Some(&"value2".to_string()));
    assert_eq!(all.len(), 2);
}

#[test]
fn test_log_context_clear() {
    // Clear any existing context first
    DMSCLogContext::clear();
    
    DMSCLogContext::put("key1", "value1");
    DMSCLogContext::put("key2", "value2");
    assert_eq!(DMSCLogContext::get_all().len(), 2);
    
    DMSCLogContext::clear();
    assert_eq!(DMSCLogContext::get_all().len(), 0);
}

#[test]
fn test_log_context_tracing_support() {
    // Clear any existing context first
    DMSCLogContext::clear();
    
    // Test trace id
    let trace_id = "test-trace-id-123";
    DMSCLogContext::set_trace_id(trace_id);
    assert_eq!(DMSCLogContext::get_trace_id(), Some(trace_id.to_string()));
    
    // Test span id
    let span_id = "test-span-id-456";
    DMSCLogContext::set_span_id(span_id);
    assert_eq!(DMSCLogContext::get_span_id(), Some(span_id.to_string()));
    
    // Test parent span id
    let parent_span_id = "test-parent-span-id-789";
    DMSCLogContext::set_parent_span_id(parent_span_id);
    assert_eq!(DMSCLogContext::get_parent_span_id(), Some(parent_span_id.to_string()));
    
    // Test generate trace id
    let generated_trace_id = DMSCLogContext::generate_trace_id();
    assert!(!generated_trace_id.is_empty());
    
    // Test generate span id
    let generated_span_id = DMSCLogContext::generate_span_id();
    assert!(!generated_span_id.is_empty());
}

#[test]
fn test_logger_creation() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    let config = DMSCLogConfig::default();
    let logger = DMSCLogger::new(&config, fs);
    // Just test that creation works without panicking
    assert!(logger.info("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_different_levels() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    // Test with Info level
    let mut config = DMSCLogConfig::default();
    config.level = DMSCLogLevel::Info;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSCLogger::new(&config, fs.clone());
    
    // All levels should work without errors
    assert!(logger.debug("test_target", "debug_message").is_ok());
    assert!(logger.info("test_target", "info_message").is_ok());
    assert!(logger.warn("test_target", "warn_message").is_ok());
    assert!(logger.error("test_target", "error_message").is_ok());
    
    // Test with Error level
    let mut config = DMSCLogConfig::default();
    config.level = DMSCLogLevel::Error;
    config.console_enabled = false;
    let logger = DMSCLogger::new(&config, fs);
    
    // All levels should still work without errors
    assert!(logger.debug("test_target", "debug_message").is_ok());
    assert!(logger.info("test_target", "info_message").is_ok());
    assert!(logger.warn("test_target", "warn_message").is_ok());
    assert!(logger.error("test_target", "error_message").is_ok());
}

#[test]
fn test_logger_with_json_format() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let mut config = DMSCLogConfig::default();
    config.json_format = true;
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSCLogger::new(&config, fs);
    
    // Should work without errors
    assert!(logger.info("test_target", "test_message").is_ok());
}

#[test]
fn test_logger_with_context() {
    let temp_dir = tempdir().unwrap();
    let fs = DMSCFileSystem::new_with_root(temp_dir.path().to_path_buf());
    
    let mut config = DMSCLogConfig::default();
    config.console_enabled = false; // Disable console output for tests
    let logger = DMSCLogger::new(&config, fs);
    
    // Clear any existing context first
    DMSCLogContext::clear();
    
    // Set context and log
    DMSCLogContext::put("test_context_key", "test_context_value");
    assert!(logger.info("test_target", "test_message").is_ok());
    
    // Clear context
    DMSCLogContext::clear();
}
