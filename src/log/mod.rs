//! Copyright © 2025 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMS.
//! The DMS project belongs to the Dunimd Team.
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
//! This module provides a comprehensive logging system for DMS, supporting multiple output formats,
//! log levels, and configurable logging behavior. It includes support for structured logging,
//! distributed tracing integration, and log rotation.
//! 
//! ## Key Components
//! 
//! - **DMSLogLevel**: Enum defining supported log levels (Debug, Info, Warn, Error)
//! - **DMSLogConfig**: Configuration struct for logging behavior
//! - **DMSLogContext**: Thread-local context for adding contextual information to logs
//! - **DMSLogger**: Public-facing logger class for application use
//! - **_CLoggerImpl**: Internal logger implementation
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Outputs**: Supports both console and file logging
//! 2. **Structured Logging**: Supports both text and JSON formats
//! 3. **Distributed Tracing**: Integrates with distributed tracing context
//! 4. **Configurable**: Highly configurable through `DMSLogConfig`
//! 5. **Performance**: Includes sampling support for high-volume logging
//! 6. **Log Rotation**: Supports size-based log rotation
//! 7. **Contextual Logging**: Allows adding contextual information to logs
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a default log configuration
//!     let log_config = DMSLogConfig::_Fdefault();
//!     
//!     // Create a file system instance (usually provided by the service context)
//!     let fs = DMSFileSystem::_Fnew();
//!     
//!     // Create a logger
//!     let logger = DMSLogger::_Fnew(&log_config, fs);
//!     
//!     // Log messages at different levels
//!     logger._Fdebug("example", "Debug message")?;
//!     logger._Finfo("example", "Info message")?;
//!     logger._Fwarn("example", "Warning message")?;
//!     logger._Ferror("example", "Error message")?;
//!     
//!     Ok(())
//! }
//! ```

// Logging module for DMS.
// This is a first-stage implementation using std only; can be extended later.

use std::fmt::Debug;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::VecDeque;

use crate::core::DMSResult;
use crate::fs::DMSFileSystem;
use rand;
use serde_json::json;
use std::fs as stdfs;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
mod context;
pub use context::DMSLogContext;

/// Log level definition.
/// 
/// This enum defines the supported log levels in DMS, ordered by severity from lowest to highest.
#[derive(Clone, Copy, Debug)]
pub enum DMSLogLevel {
    /// Debug level: Detailed information for debugging purposes
    Debug,
    /// Info level: General information about application operation
    Info,
    /// Warn level: Warning messages about potential issues
    Warn,
    /// Error level: Error messages about failures
    Error,
}

impl DMSLogLevel {
    /// Returns the string representation of the log level.
    /// 
    /// # Returns
    /// 
    /// A static string representing the log level ("DEBUG", "INFO", "WARN", or "ERROR")
    pub fn _Fas_str(&self) -> &'static str {
        match self {
            DMSLogLevel::Debug => "DEBUG",
            DMSLogLevel::Info => "INFO",
            DMSLogLevel::Warn => "WARN",
            DMSLogLevel::Error => "ERROR",
        }
    }
}

/// Public logging configuration class.
/// 
/// This struct defines the configuration options for the DMS logging system, including
/// log level, output formats, sampling, and log rotation settings.
#[derive(Clone)]
pub struct DMSLogConfig {
    /// Minimum log level to be logged
    pub level: DMSLogLevel,
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

impl DMSLogConfig {
    /// Creates a new default log configuration.
    /// 
    /// Returns a `DMSLogConfig` instance with the following default values:
    /// - level: Info
    /// - console_enabled: true
    /// - file_enabled: true
    /// - sampling_default: 1.0
    /// - file_name: "dms.log"
    /// - json_format: false
    /// - rotate_when: "size"
    /// - max_bytes: 10 MB
    pub fn _Fdefault() -> Self {
        DMSLogConfig {
            level: DMSLogLevel::Info,
            console_enabled: true,
            file_enabled: true,
            sampling_default: 1.0,
            file_name: "dms.log".to_string(),
            json_format: false,
            rotate_when: "size".to_string(),
            max_bytes: 10 * 1024 * 1024,
        }
    }

    /// Creates a log configuration from a `DMSConfig` instance.
    /// 
    /// This method reads logging configuration from a `DMSConfig` instance, using the following keys:
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
    /// - `config`: The `DMSConfig` instance to read from
    /// 
    /// # Returns
    /// 
    /// A `DMSLogConfig` instance with configuration from the given `DMSConfig`
    pub fn _Ffrom_config(config: &crate::config::DMSConfig) -> Self {
        let mut base = DMSLogConfig::_Fdefault();

        if let Some(level_str) = config._Fget_str("log.level") {
            base.level = match level_str {
                "DEBUG" | "debug" => DMSLogLevel::Debug,
                "INFO" | "info" => DMSLogLevel::Info,
                "WARN" | "warn" | "WARNING" | "warning" => DMSLogLevel::Warn,
                "ERROR" | "error" => DMSLogLevel::Error,
                _ => base.level,
            };
        }

        if let Some(v) = config._Fget_f32("log.sampling_default") {
            base.sampling_default = v.clamp(0.0, 1.0);
        }

        if let Some(file_name) = config._Fget_str("log.file_name") {
            if !file_name.is_empty() {
                base.file_name = file_name.to_string();
            }
        }

        if let Some(fmt) = config._Fget_str("log.file_format") {
            // Accept "json"/"JSON" to enable JSON file output, others default to text
            if fmt.eq_ignore_ascii_case("json") {
                base.json_format = true;
            }
        }

        if let Some(rotate) = config._Fget_str("log.rotate_when") {
            if !rotate.is_empty() {
                base.rotate_when = rotate.to_string();
            }
        }

        if let Some(v) = config._Fget_u64("log.max_bytes") {
            if v > 0 {
                base.max_bytes = v;
            }
        }

        if let Some(v) = config._Fget_bool("log.console_enabled") {
            base.console_enabled = v;
        }

        if let Some(v) = config._Fget_bool("log.file_enabled") {
            base.file_enabled = v;
        }

        base
    }
}

/// Log entry for caching
struct LogEntry {
    level: DMSLogLevel,
    target: String,
    message: String,
    timestamp: String,
    context: serde_json::Map<String, serde_json::Value>,
}

/// Internal logger implementation.
/// 
/// This struct contains the internal implementation of the logging system, including
/// log level checking, sampling, log message formatting, and caching.
struct _CLoggerImpl {
    /// Minimum log level to be logged
    level: DMSLogLevel,
    /// File system instance for writing log files
    #[allow(dead_code)]
    fs: DMSFileSystem,
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

impl _CLoggerImpl {
    /// Creates a new internal logger implementation.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The `DMSLogConfig` instance to use for configuration
    /// - `fs`: The `DMSFileSystem` instance to use for writing log files
    /// 
    /// # Returns
    /// 
    /// A new `_CLoggerImpl` instance
    fn _Fnew(config: &DMSLogConfig, fs: DMSFileSystem) -> Self {
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
                    Self::_Fflush_cache(
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
                    Self::_Fflush_cache(
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
        
        _CLoggerImpl {
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
    /// A `DMSResult` indicating success or failure
    fn _Fflush_cache(
        log_cache: &Arc<(Mutex<VecDeque<LogEntry>>, Condvar)>,
        fs: &DMSFileSystem,
        file_name: &str,
        json_format: bool,
        rotate_when: &str,
        max_bytes: u64,
        console_enabled: bool
    ) -> DMSResult<()> {
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
                serde_json::to_string(&log_entry.context)?
            } else {
                // Extract context fields for text format
                let ctx_kv: Vec<(String, String)> = log_entry.context.iter()
                    .filter(|(k, _)| *k != "timestamp" && *k != "level" && *k != "target" && *k != "event" && *k != "message")
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
                
                let trace_str = if let Some(trace_id) = log_entry.context.get("trace_id") {
                    format!(" trace_id={}", trace_id)
                } else {
                    String::new()
                };
                
                format!(
                    "{} [{}] {} event={}{} - {}{}",
                    log_entry.timestamp,
                    log_entry.level._Fas_str(),
                    log_entry.target,
                    log_entry.target, // Using target as event for simplicity
                    trace_str,
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
                println!("{line}");
            }
        }
        
        // Write to file in batch
        if !file_logs.is_empty() {
            let log_file = fs._Flogs_dir().join(file_name);
            
            // Simple size-based rotation if enabled
            if rotate_when.eq_ignore_ascii_case("size") && max_bytes > 0 {
                if let Ok(meta) = stdfs::metadata(&log_file) {
                    if meta.len() >= max_bytes {
                        if let Some(parent) = log_file.parent() {
                            let base = log_file.file_name().and_then(|s| s.to_str()).unwrap_or("dms.log");
                            let ts = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .map_err(|e| crate::core::DMSError::Other(format!("timestamp error: {e}")))?;
                            let rotated = parent.join(format!("{}.{}", base, ts.as_millis()));
                            let _ = stdfs::rename(&log_file, &rotated);
                        }
                    }
                }
            }
            
            // Batch write to file
            let content = file_logs.join("\n") + "\n";
            fs._Fappend_text(&log_file, &content)?;
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
    fn _Fshould_log(&self, level: DMSLogLevel) -> bool {
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
    fn _Fshould_log_event(&self, _event: &str) -> bool {
        // Placeholder: simple sampling based on sampling_default; can be extended per-event.
        if self.sampling_default >= 1.0 {
            true
        } else if self.sampling_default <= 0.0 {
            false
        } else {
            let r = rand::random::<f32>();
            r < self.sampling_default
        }
    }

    /// Returns the current timestamp in ISO 8601 format.
    /// 
    /// # Returns
    /// 
    /// A string representing the current timestamp in ISO 8601 format (e.g., "1630000000.123Z")
    fn _Fnow_timestamp() -> String {
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
    /// A `DMSResult` indicating success or failure
    fn _Flog_message<T: Debug>(&self, level: DMSLogLevel, target: &str, message: T) -> DMSResult<()> {
        if !self._Fshould_log(level) {
            return Ok(());
        }

        let event = target; // simple default; can be extended to accept explicit event names.
        if !self._Fshould_log_event(event) {
            return Ok(());
        }

        let ts = Self::_Fnow_timestamp();
        let message_str = format!("{:?}", message);
        let ctx_kv = DMSLogContext::_Fget_all();

        // Create log entry with structured data
        let mut log_entry_context = serde_json::Map::new();
        log_entry_context.insert("timestamp".to_string(), json!(ts));
        log_entry_context.insert("level".to_string(), json!(level._Fas_str()));
        log_entry_context.insert("target".to_string(), json!(target));
        log_entry_context.insert("event".to_string(), json!(event));
        log_entry_context.insert("message".to_string(), json!(message_str));
        
        // Add distributed tracing fields if present
        if let Some(trace_id) = DMSLogContext::_Fget_trace_id() {
            log_entry_context.insert("trace_id".to_string(), json!(trace_id));
        }
        if let Some(span_id) = DMSLogContext::_Fget_span_id() {
            log_entry_context.insert("span_id".to_string(), json!(span_id));
        }
        if let Some(parent_span_id) = DMSLogContext::_Fget_parent_span_id() {
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
/// This struct provides the public API for logging in DMS, wrapping the internal `_CLoggerImpl`.
pub struct DMSLogger {
    /// Internal logger implementation
    inner: _CLoggerImpl,
}

impl DMSLogger {
    /// Creates a new logger instance.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The `DMSLogConfig` instance to use for configuration
    /// - `fs`: The `DMSFileSystem` instance to use for writing log files
    /// 
    /// # Returns
    /// 
    /// A new `DMSLogger` instance
    pub fn _Fnew(config: &DMSLogConfig, fs: DMSFileSystem) -> Self {
        let inner = _CLoggerImpl::_Fnew(config, fs);
        DMSLogger { inner }
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
    /// A `DMSResult` indicating success or failure
    pub fn _Fdebug<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Debug, target, message)
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
    /// A `DMSResult` indicating success or failure
    pub fn _Finfo<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Info, target, message)
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
    /// A `DMSResult` indicating success or failure
    pub fn _Fwarn<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Warn, target, message)
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
    /// A `DMSResult` indicating success or failure
    pub fn _Ferror<T: Debug>(&self, target: &str, message: T) -> DMSResult<()> {
        self.inner._Flog_message(DMSLogLevel::Error, target, message)
    }
}
