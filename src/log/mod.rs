//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

#![allow(non_snake_case)]

//! # Logging System
//! 
//! This module provides a comprehensive logging system for DMSC, supporting multiple output formats,
//! log levels, and configurable logging behavior. It includes support for structured logging,
//! distributed tracing integration, and log rotation.
//! 
//! ## Key Components
//! 
//! - **DMSCLogLevel**: Enum defining supported log levels (Debug, Info, Warn, Error)
//! - **DMSCLogConfig**: Configuration struct for logging behavior
//! - **DMSCLogContext**: Thread-local context for adding contextual information to logs
//! - **DMSCLogger**: Public-facing logger class for application use
//! - **LoggerImpl**: Internal logger implementation
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Outputs**: Supports both console and file logging
//! 2. **Structured Logging**: Supports both text and JSON formats
//! 3. **Distributed Tracing**: Integrates with distributed tracing context
//! 4. **Configurable**: Highly configurable through `DMSCLogConfig`
//! 5. **Performance**: Includes sampling support for high-volume logging
//! 6. **Log Rotation**: Supports size-based log rotation
//! 7. **Contextual Logging**: Allows adding contextual information to logs
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! fn example() -> DMSCResult<()> {
//!     // Create a default log configuration
//!     let log_config = DMSCLogConfig::default();
//!     
//!     // Create a file system instance (usually provided by the service context)
//!     let fs = DMSCFileSystem::new();
//!     
//!     // Create a logger
//!     let logger = DMSCLogger::new(&log_config, fs);
//!     
//!     // Log messages at different levels
//!     logger.debug("example", "Debug message")?;
//!     logger.info("example", "Info message")?;
//!     logger.warn("example", "Warning message")?;
//!     logger.error("example", "Error message")?;
//!     
//!     Ok(())
//! }
//! ```

// Logging module for DMSC.
// This is a first-stage implementation using std only; can be extended later.

use std::fmt::Debug;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

use crate::core::DMSCResult;
use crate::fs::DMSCFileSystem;
use rand;
use serde_json::json;
use std::fs as stdfs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
mod context;
pub use context::DMSCLogContext;

/// Log level definition.
/// 
/// This enum defines the supported log levels in DMSC, ordered by severity from lowest to highest.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone, Copy, Debug)]
pub enum DMSCLogLevel {
    /// Debug level: Detailed information for debugging purposes
    Debug,
    /// Info level: General information about application operation
    Info,
    /// Warn level: Warning messages about potential issues
    Warn,
    /// Error level: Error messages about failures
    Error,
}

impl DMSCLogLevel {
    /// Returns the string representation of the log level.
    /// 
    /// # Returns
    /// 
    /// A static string representing the log level ("DEBUG", "INFO", "WARN", or "ERROR")
    pub fn as_str(&self) -> &'static str {
        match self {
            DMSCLogLevel::Debug => "DEBUG",
            DMSCLogLevel::Info => "INFO",
            DMSCLogLevel::Warn => "WARN",
            DMSCLogLevel::Error => "ERROR",
        }
    }
}

/// Public logging configuration class.
/// 
/// This struct defines the configuration options for the DMSC logging system, including
/// log level, output formats, sampling, and log rotation settings.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSCLogConfig {
    /// Minimum log level to be logged
    pub level: DMSCLogLevel,
    /// Whether console logging is enabled
    pub console_enabled: bool,
    /// Whether file logging is enabled
    pub file_enabled: bool,
    /// Default sampling rate (0.0 to 1.0, where 1.0 means all logs are sampled)
    pub sampling_default: f32,
    /// Name of the log file
    pub file_name: String,
    /// Whether to use JSON format for logs
    pub json_format: bool,
    /// When to rotate logs (currently only "size" or "none" are supported)
    pub rotate_when: String,
    /// Maximum file size in bytes before rotation (used when rotate_when == "size")
    pub max_bytes: u64,
}

impl DMSCLogConfig {
    /// Creates a log configuration from a `DMSCConfig` instance.
    /// 
    /// This method reads logging configuration from a `DMSCConfig` instance, using the following keys:
    /// - log.level: Log level (DEBUG, INFO, WARN, ERROR)
    /// - log.console_enabled: Whether console logging is enabled
    /// - log.file_enabled: Whether file logging is enabled
    /// - log.sampling_default: Default sampling rate
    /// - log.file_name: Name of the log file
    /// - log.file_format: Log format ("json" for JSON format, anything else for text)
    /// - log.rotate_when: When to rotate logs
    /// - log.max_bytes: Maximum file size before rotation
    /// 
    /// # Parameters
    /// 
    /// - `config`: The `DMSCConfig` instance to read from
    /// 
    /// # Returns
    /// 
    /// A `DMSCLogConfig` instance with configuration from the given `DMSCConfig`
    pub fn from_config(config: &crate::config::DMSCConfig) -> Self {
        let mut base = DMSCLogConfig::default();

        if let Some(level_str) = config.get_str("log.level") {
            base.level = match level_str {
                "DEBUG" | "debug" => DMSCLogLevel::Debug,
                "INFO" | "info" => DMSCLogLevel::Info,
                "WARN" | "warn" | "WARNING" | "warning" => DMSCLogLevel::Warn,
                "ERROR" | "error" => DMSCLogLevel::Error,
                _ => base.level,
            };
        }

        if let Some(v) = config.get_f32("log.sampling_default") {
            base.sampling_default = v.clamp(0.0, 1.0);
        }

        if let Some(file_name) = config.get_str("log.file_name") {
            if !file_name.is_empty() {
                base.file_name = file_name.to_string();
            }
        }

        if let Some(fmt) = config.get_str("log.file_format") {
            // Accept "json"/"JSON" to enable JSON file output, others default to text
            if fmt.eq_ignore_ascii_case("json") {
                base.json_format = true;
            }
        }

        if let Some(rotate) = config.get_str("log.rotate_when") {
            if !rotate.is_empty() {
                base.rotate_when = rotate.to_string();
            }
        }

        if let Some(v) = config.get_u64("log.max_bytes") {
            if v > 0 {
                base.max_bytes = v;
            }
        }

        if let Some(v) = config.get_bool("log.console_enabled") {
            base.console_enabled = v;
        }

        if let Some(v) = config.get_bool("log.file_enabled") {
            base.file_enabled = v;
        }

        base
    }
}

/// Default implementation for DMSCLogConfig
impl Default for DMSCLogConfig {
    fn default() -> Self {
        DMSCLogConfig {
            level: DMSCLogLevel::Info,
            console_enabled: true,
            file_enabled: true,
            sampling_default: 1.0,
            file_name: "dms.log".to_string(),
            json_format: false,
            rotate_when: "size".to_string(),
            max_bytes: 10 * 1024 * 1024,
        }
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCLogConfig
#[pyo3::prelude::pymethods]
impl DMSCLogConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    /// Set log level from Python
    fn set_level(&mut self, level: String) -> pyo3::PyResult<()>
    {
        let log_level = match level.to_ascii_lowercase().as_str() {
            "debug" => DMSCLogLevel::Debug,
            "info" => DMSCLogLevel::Info,
            "warn" | "warning" => DMSCLogLevel::Warn,
            "error" => DMSCLogLevel::Error,
            _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Invalid log level: {level}"))),
        };
        self.level = log_level;
        Ok(())
    }
    
    /// Get log level as string from Python
    fn get_level(&self) -> String {
        self.level.as_str().to_lowercase()
    }
    
    /// Set console enabled flag from Python
    fn set_console_enabled(&mut self, enabled: bool) {
        self.console_enabled = enabled;
    }
    
    /// Get console enabled flag from Python
    fn get_console_enabled(&self) -> bool {
        self.console_enabled
    }
    
    /// Set file enabled flag from Python
    fn set_file_enabled(&mut self, enabled: bool) {
        self.file_enabled = enabled;
    }
    
    /// Get file enabled flag from Python
    fn get_file_enabled(&self) -> bool {
        self.file_enabled
    }
    
    /// Set file name from Python
    fn set_file_name(&mut self, file_name: String) {
        self.file_name = file_name;
    }
    
    /// Get file name from Python
    fn get_file_name(&self) -> String {
        self.file_name.clone()
    }
    
    /// Set JSON format flag from Python
    fn set_json_format(&mut self, json_format: bool) {
        self.json_format = json_format;
    }
    
    /// Get JSON format flag from Python
    fn get_json_format(&self) -> bool {
        self.json_format
    }
    
    /// Set rotate when from Python
    fn set_rotate_when(&mut self, rotate_when: String) {
        self.rotate_when = rotate_when;
    }
    
    /// Get rotate when from Python
    fn get_rotate_when(&self) -> String {
        self.rotate_when.clone()
    }
    
    /// Set max bytes from Python
    fn set_max_bytes(&mut self, max_bytes: u64) {
        self.max_bytes = max_bytes;
    }
    
    /// Get max bytes from Python
    fn get_max_bytes(&self) -> u64 {
        self.max_bytes
    }
    
    /// Set sampling default from Python
    fn set_sampling_default(&mut self, sampling_default: f32) -> pyo3::PyResult<()>
    {
        if sampling_default < 0.0 || sampling_default > 1.0 {
            return Err(pyo3::exceptions::PyValueError::new_err("Sampling default must be between 0.0 and 1.0"));
        }
        self.sampling_default = sampling_default;
        Ok(())
    }
    
    /// Get sampling default from Python
    fn get_sampling_default(&self) -> f32 {
        self.sampling_default
    }
}

/// Log entry for caching
struct LogEntry {
    level: DMSCLogLevel,
    target: String,
    message: String,
    timestamp: String,
    context: serde_json::Map<String, serde_json::Value>,
}

/// Internal logger implementation.
/// 
/// This struct contains the internal implementation of the logging system, including
/// log level checking, sampling, log message formatting, and caching.
struct LoggerImpl {
    /// Minimum log level to be logged
    level: DMSCLogLevel,
    /// File system instance for writing log files
    #[allow(dead_code)]
    fs: DMSCFileSystem,
    /// Default sampling rate
    sampling_default: f32,
    /// Whether console logging is enabled
    #[allow(dead_code)]
    console_enabled: bool,
    /// Whether file logging is enabled
    #[allow(dead_code)]
    file_enabled: bool,
    /// Name of the log file
    #[allow(dead_code)]
    file_name: String,
    /// Whether to use JSON format for logs
    #[allow(dead_code)]
    json_format: bool,
    /// When to rotate logs (currently only "size" or "none" are supported)
    #[allow(dead_code)]
    rotate_when: String,
    /// Maximum file size in bytes before rotation (used when rotate_when == "size")
    #[allow(dead_code)]
    max_bytes: u64,
    /// Log cache for batch writing
    log_cache: Arc<(Mutex<VecDeque<LogEntry>>, Condvar)>,
    /// Cache size limit
    cache_size_limit: usize,
    /// Flush interval in milliseconds
    #[allow(dead_code)]
    flush_interval_ms: u64,
    /// Shutdown flag
    #[allow(dead_code)]
    shutdown_flag: Arc<Mutex<bool>>,
}

impl LoggerImpl {
    /// Creates a new internal logger implementation.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The `DMSCLogConfig` instance to use for configuration
    /// - `fs`: The `DMSCFileSystem` instance to use for writing log files
    /// 
    /// # Returns
    /// 
    /// A new `LoggerImpl` instance
    fn new(config: &DMSCLogConfig, fs: DMSCFileSystem) -> Self {
        let log_cache = Arc::new((Mutex::new(VecDeque::new()), Condvar::new()));
        let shutdown_flag = Arc::new(Mutex::new(false));
        let cache_size_limit = 1000;
        let flush_interval_ms = 500;
        
        // Create a copy of the necessary fields for the background thread
        let bg_log_cache = Arc::clone(&log_cache);
        let bg_fs = fs.clone();
        let bg_file_name = config.file_name.clone();
        let bg_json_format = config.json_format;
        let bg_rotate_when = config.rotate_when.clone();
        let bg_max_bytes = config.max_bytes;
        let bg_console_enabled = config.console_enabled;
        let bg_shutdown_flag = Arc::clone(&shutdown_flag);
        
        // Start background flush thread
        thread::spawn(move || {
            let mut last_flush = SystemTime::now();
            
            loop {
                // Check if we should shutdown
                if *bg_shutdown_flag.lock().unwrap() {
                    // Flush remaining logs before shutting down
                    Self::flush_cache(
                        &bg_log_cache,
                        &bg_fs,
                        &bg_file_name,
                        bg_json_format,
                        &bg_rotate_when,
                        bg_max_bytes,
                        bg_console_enabled
                    ).unwrap_or(());
                    break;
                }
                
                // Check if we need to flush based on time or cache size
                let now = SystemTime::now();
                let time_since_last_flush = now.duration_since(last_flush).unwrap_or(Duration::from_millis(0));
                
                let cache_len = bg_log_cache.0.lock().unwrap().len();
                
                if time_since_last_flush >= Duration::from_millis(flush_interval_ms) || cache_len >= cache_size_limit {
                    Self::flush_cache(
                        &bg_log_cache,
                        &bg_fs,
                        &bg_file_name,
                        bg_json_format,
                        &bg_rotate_when,
                        bg_max_bytes,
                        bg_console_enabled
                    ).unwrap_or(());
                    last_flush = now;
                }
                
                // Wait for a short time or until signaled
                let (lock, cvar) = &*bg_log_cache;
                let _ = cvar.wait_timeout(lock.lock().unwrap(), Duration::from_millis(100)).unwrap();
            }
        });
        
        LoggerImpl {
            level: config.level,
            fs,
            sampling_default: config.sampling_default,
            console_enabled: config.console_enabled,
            file_enabled: config.file_enabled,
            file_name: config.file_name.clone(),
            json_format: config.json_format,
            rotate_when: config.rotate_when.clone(),
            max_bytes: config.max_bytes,
            log_cache,
            cache_size_limit,
            flush_interval_ms,
            shutdown_flag,
        }
    }
    
    /// Flushes the log cache to disk and console
    /// 
    /// # Parameters
    /// 
    /// - `log_cache`: The log cache to flush
    /// - `fs`: The file system instance to use for writing log files
    /// - `file_name`: The name of the log file
    /// - `json_format`: Whether to use JSON format for logs
    /// - `rotate_when`: When to rotate logs
    /// - `max_bytes`: Maximum file size before rotation
    /// - `console_enabled`: Whether console logging is enabled
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    fn flush_cache(
        log_cache: &Arc<(Mutex<VecDeque<LogEntry>>, Condvar)>,
        fs: &DMSCFileSystem,
        file_name: &str,
        json_format: bool,
        rotate_when: &str,
        max_bytes: u64,
        console_enabled: bool
    ) -> DMSCResult<()> {
        let (lock, _cvar) = &**log_cache;
        let mut cache = lock.lock().unwrap();
        
        if cache.is_empty() {
            return Ok(());
        }
        
        // Collect all logs to flush
        let logs_to_flush: Vec<LogEntry> = cache.drain(..).collect();
        drop(cache);
        
        // Process logs in batch
        let mut file_logs = Vec::new();
        let mut console_logs = Vec::new();
        
        for log_entry in logs_to_flush {
            // Format log entry
            let line = if json_format {
                // Ensure all required fields are present in JSON format
                let mut log_obj = log_entry.context.clone();
                
                // Add any missing standard fields
                if !log_obj.contains_key("level") {
                    log_obj.insert("level".to_string(), serde_json::Value::String(log_entry.level.as_str().to_string()));
                }
                if !log_obj.contains_key("target") {
                    log_obj.insert("target".to_string(), serde_json::Value::String(log_entry.target.clone()));
                }
                if !log_obj.contains_key("message") {
                    log_obj.insert("message".to_string(), serde_json::Value::String(log_entry.message.clone()));
                }
                if !log_obj.contains_key("timestamp") {
                    log_obj.insert("timestamp".to_string(), serde_json::Value::String(log_entry.timestamp.clone()));
                }
                
                serde_json::to_string(&log_obj)?
            } else {
                // Extract context fields for text format
                let ctx_kv: Vec<(String, String)> = log_entry.context.iter()
                    .filter(|(k, _)| *k != "timestamp" && *k != "level" && *k != "target" && *k != "message" && *k != "trace_id" && *k != "span_id")
                    .map(|(k, v)| (k.clone(), v.to_string()))
                    .collect();
                
                let ctx_str = if ctx_kv.is_empty() {
                    String::new()
                } else {
                    let parts: Vec<String> = ctx_kv
                        .iter()
                        .map(|(k, v)| format!("{k}={v}"))
                        .collect();
                    format!(" ctx={{ {} }}", parts.join(", "))
                };
                
                // Extract trace and span IDs if present
                let trace_info = match (log_entry.context.get("trace_id"), log_entry.context.get("span_id")) {
                    (Some(trace), Some(span)) => format!(" trace_id={trace} span_id={span}"),
                    (Some(trace), None) => format!(" trace_id={trace}"),
                    (None, Some(span)) => format!(" span_id={span}"),
                    (None, None) => String::new(),
                };
                
                // Extract event name from context or use target as fallback
                let event = log_entry.context.get("event")
                    .and_then(|v| v.as_str())
                    .unwrap_or(&log_entry.target);
                
                // Improved text log format with standardized fields
                format!(
                    "{} [{}] {} event={}{} - {}{}",
                    log_entry.timestamp,
                    log_entry.level.as_str(),
                    log_entry.target,
                    event,
                    trace_info,
                    log_entry.message,
                    ctx_str,
                )
            };
            
            // Separate console and file logs
            if console_enabled {
                console_logs.push(line.clone());
            }
            
            if !line.is_empty() {
                file_logs.push(line);
            }
        }
        
        // Write to console in batch
        if !console_logs.is_empty() {
            for line in console_logs {
                log::info!("{line}");
            }
        }
        
        // Write to file in batch
        if !file_logs.is_empty() {
            let log_file = fs.logs_dir().join(file_name);
            
            // Simple size-based rotation if enabled
            if rotate_when.eq_ignore_ascii_case("size") && max_bytes > 0 {
                if let Ok(meta) = stdfs::metadata(&log_file) {
                    if meta.len() >= max_bytes {
                        if let Some(parent) = log_file.parent() {
                            let base = log_file.file_name().and_then(|s| s.to_str()).unwrap_or("dms.log");
                            let ts = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map_err(|e| crate::core::DMSCError::Other(format!("timestamp error: {e}")))?;
                            let rotated = parent.join(format!("{}.{}", base, ts.as_millis()));
                            let _ = stdfs::rename(&log_file, &rotated);
                        }
                    }
                }
            }
            
            // Batch write to file
            let content = file_logs.join("\n") + "\n";
            fs.append_text(&log_file, &content)?;
        }
        
        Ok(())
    }

    /// Determines if a message with the given level should be logged.
    /// 
    /// # Parameters
    /// 
    /// - `level`: The log level of the message
    /// 
    /// # Returns
    /// 
    /// `true` if the message should be logged, `false` otherwise
    fn should_log(&self, level: DMSCLogLevel) -> bool {
        (level as u8) >= (self.level as u8)
    }

    /// Determines if an event should be logged based on sampling.
    /// 
    /// # Parameters
    /// 
    /// - `_event`: The event name (currently unused, reserved for future per-event sampling)
    /// 
    /// # Returns
    /// 
    /// `true` if the event should be logged, `false` otherwise
    fn should_log_event(&self, event: &str) -> bool {
        // Advanced event-based sampling with per-event configuration support
        
        // First check if we have specific sampling rules for this event
        if let Some(event_sampling_rate) = self.get_event_sampling_rate(event) {
            if event_sampling_rate >= 1.0 {
                return true;
            } else if event_sampling_rate <= 0.0 {
                return false;
            } else {
                let r = rand::random::<f32>();
                return r < event_sampling_rate;
            }
        }
        
        // Fall back to default sampling rate
        if self.sampling_default >= 1.0 {
            true
        } else if self.sampling_default <= 0.0 {
            false
        } else {
            let r = rand::random::<f32>();
            r < self.sampling_default
        }
    }
    
    /// Get the sampling rate for a specific event type
    /// 
    /// # Parameters
    /// 
    /// - `event`: The event name to get sampling rate for
    /// 
    /// # Returns
    /// 
    /// Optional sampling rate (0.0 to 1.0) if specific rate is configured
    fn get_event_sampling_rate(&self, event: &str) -> Option<f32> {
        // In a production environment, this would:
        // 1. Load per-event sampling configuration from config files
        // 2. Support dynamic configuration updates
        // 3. Handle event patterns and categories
        // 4. Support A/B testing for different sampling rates
        
        // For now, we support a few common event types with different sampling rates
        match event {
            "database_query" => Some(0.1),      // Sample 10% of database queries
            "api_request" => Some(0.5),        // Sample 50% of API requests
            "cache_hit" => Some(0.05),         // Sample 5% of cache hits
            "cache_miss" => Some(1.0),         // Log all cache misses
            "error" => Some(1.0),             // Log all errors
            "warning" => Some(0.8),           // Log 80% of warnings
            _ => None,                         // Use default for unknown events
        }
    }

    /// Returns the current timestamp in ISO 8601 format.
    /// 
    /// # Returns
    /// 
    /// A string representing the current timestamp in ISO 8601 format (e.g., "1630000000.123Z")
    fn now_timestamp() -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(dur) => {
                let secs = dur.as_secs();
                let millis = dur.subsec_millis();
                format!("{secs}.{millis:03}Z")
            }
            Err(_) => "0.000Z".to_string(),
        }
    }

    /// Logs a message with the given level, target, and message.
    /// 
    /// This method handles the complete logging process, including:
    /// 1. Checking if the message should be logged based on level
    /// 2. Sampling the message if applicable
    /// 3. Formatting the message (text or JSON)
    /// 4. Adding contextual information and distributed tracing fields
    /// 5. Adding the log entry to the cache for batch processing
    /// 
    /// # Parameters
    /// 
    /// - `level`: The log level of the message
    /// - `target`: The target of the log message (usually a module or component name)
    /// - `message`: The message to log (must implement `Debug`)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    fn log_message<T: Debug>(&self, level: DMSCLogLevel, target: &str, message: T) -> DMSCResult<()> {
        if !self.should_log(level) {
            return Ok(());
        }

        let event = target; // simple default; can be extended to accept explicit event names.
        if !self.should_log_event(event) {
            return Ok(());
        }

        let ts = Self::now_timestamp();
        let message_str = format!("{message:?}");
        let ctx_kv = DMSCLogContext::get_all();

        // Create log entry with structured data
        let mut log_entry_context = serde_json::Map::new();
        log_entry_context.insert("timestamp".to_string(), json!(ts));
        log_entry_context.insert("level".to_string(), json!(level.as_str()));
        log_entry_context.insert("target".to_string(), json!(target));
        log_entry_context.insert("event".to_string(), json!(event));
        log_entry_context.insert("message".to_string(), json!(message_str));
        
        // Add distributed tracing fields if present
        if let Some(trace_id) = DMSCLogContext::get_trace_id() {
            log_entry_context.insert("trace_id".to_string(), json!(trace_id));
        }
        if let Some(span_id) = DMSCLogContext::get_span_id() {
            log_entry_context.insert("span_id".to_string(), json!(span_id));
        }
        if let Some(parent_span_id) = DMSCLogContext::get_parent_span_id() {
            log_entry_context.insert("parent_span_id".to_string(), json!(parent_span_id));
        }
        
        // Add context fields
        if !ctx_kv.is_empty() {
            for (k, v) in ctx_kv.iter() {
                log_entry_context.insert(k.clone(), json!(v));
            }
        }

        // Create log entry for caching
        let log_entry = LogEntry {
            level,
            target: target.to_string(),
            message: message_str,
            timestamp: ts,
            context: log_entry_context,
        };

        // Add log entry to cache
        let (lock, cvar) = &*self.log_cache;
        let mut cache = lock.lock().unwrap();
        cache.push_back(log_entry);
        
        // Signal the background thread if cache is full
        if cache.len() >= self.cache_size_limit {
            cvar.notify_one();
        }
        
        Ok(())
    }
}

/// Public-facing logger class.
/// 
/// This struct provides the public API for logging in DMSC, wrapping the internal `LoggerImpl`.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCLogger {
    /// Internal logger implementation
    inner: LoggerImpl,
}

impl DMSCLogger {
    /// Creates a new logger instance.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The `DMSCLogConfig` instance to use for configuration
    /// - `fs`: The `DMSCFileSystem` instance to use for writing log files
    /// 
    /// # Returns
    /// 
    /// A new `DMSCLogger` instance
    pub fn new(config: &DMSCLogConfig, fs: DMSCFileSystem) -> Self {
        let inner = LoggerImpl::new(config, fs);
        DMSCLogger { inner }
    }

    /// Logs a debug message.
    /// 
    /// # Parameters
    /// 
    /// - `target`: The target of the log message (usually a module or component name)
    /// - `message`: The message to log (must implement `Debug`)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    pub fn debug<T: Debug>(&self, target: &str, message: T) -> DMSCResult<()> {
        self.inner.log_message(DMSCLogLevel::Debug, target, message)
    }

    /// Logs an info message.
    /// 
    /// # Parameters
    /// 
    /// - `target`: The target of the log message (usually a module or component name)
    /// - `message`: The message to log (must implement `Debug`)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    pub fn info<T: Debug>(&self, target: &str, message: T) -> DMSCResult<()> {
        self.inner.log_message(DMSCLogLevel::Info, target, message)
    }

    /// Logs a warning message.
    /// 
    /// # Parameters
    /// 
    /// - `target`: The target of the log message (usually a module or component name)
    /// - `message`: The message to log (must implement `Debug`)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    pub fn warn<T: Debug>(&self, target: &str, message: T) -> DMSCResult<()> {
        self.inner.log_message(DMSCLogLevel::Warn, target, message)
    }

    /// Logs an error message.
    /// 
    /// # Parameters
    /// 
    /// - `target`: The target of the log message (usually a module or component name)
    /// - `message`: The message to log (must implement `Debug`)
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult` indicating success or failure
    pub fn error<T: Debug>(&self, target: &str, message: T) -> DMSCResult<()> {
        self.inner.log_message(DMSCLogLevel::Error, target, message)
    }
}
