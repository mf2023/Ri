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

//! # Log Module C API
//!
//! This module provides C language bindings for DMSC's logging infrastructure. The logging module
//! delivers structured, high-performance logging capabilities with multiple output destinations,
//! configurable log levels, and rich formatting options. This C API enables C/C++ applications
//! to leverage DMSC's logging system for comprehensive observability and debugging support.
//!
//! ## Module Architecture
//!
//! The logging module comprises two primary components that together provide complete logging
//! functionality:
//!
//! - **DMSCLogConfig**: Configuration container for logger initialization parameters including log
//!   level thresholds, output destinations, formatting options, and sink configurations. The
//!   configuration object controls all aspects of logger behavior and resource allocation.
//!
//! - **DMSCLogger**: Primary logging interface providing methods for emitting log entries at
//!   various severity levels. The logger instance handles log message formatting, filtering based
//!   on configured levels, and routing to configured output sinks.
//!
//! ## Log Levels
//!
//! The logging system supports standard log severity levels:
//!
//! - **TRACE** (0): Most detailed level, typically used for debugging with very granular
//!   information about program execution flow.
//!
//! - **DEBUG** (1): Detailed information for debugging purposes, useful during development
//!   and troubleshooting.
//!
//! - **INFO** (2): General informational messages about normal application operation,
//!   confirming that expected events have occurred.
//!
//! - **WARN** (3): Warning messages indicating potential issues that don't prevent the
//!   application from functioning but may require attention.
//!
//! - **ERROR** (4): Error messages indicating failures that affect specific operations
//!   but don't prevent the overall application from continuing.
//!
//! - **FATAL** (5): Critical errors that prevent continued operation, typically followed
//!   by application shutdown or termination.
//!
//! ## Structured Logging
//!
//! The logger supports structured logging with key-value pairs:
//!
//! - **Context Fields**: Attach contextual information to log entries that provides
//!   additional context for debugging and analysis.
//!
//! - **Automatic Fields**: Built-in fields including timestamp, log level, target/module,
//!   thread ID, and line number information.
//!
//! - **Custom Fields**: User-defined fields for application-specific context like user IDs,
//!   request IDs, operation types, and business metrics.
//!
//! - **Field Types**: Support for various field types including strings, integers, floats,
//!   booleans, durations, and nested objects.
//!
//! ## Output Sinks
//!
//! The logging system supports multiple output destinations:
//!
//! - **Console Sink**: Standard output and standard error streams with colorized output
//!   support for terminal environments.
//!
//! - **File Sink**: Rotating log files with configurable size and time-based rotation.
//!   Supports log compression and retention policies.
//!
//! - **Syslog Sink**: Integration with system logging daemon (Unix/Linux systems).
//!   Follows standard syslog protocols and priorities.
//!
//! - **Journald Sink**: Integration with systemd journal for Linux systems with structured
//!   metadata support.
//!
//! - **Network Sink**: Remote log aggregation via TCP/UDP protocols. Supports buffering
//!   and retry logic for unreliable networks.
//!
//! - **Custom Sinks**: User-defined sink implementations for integration with external
//!   logging systems, databases, or cloud services.
//!
//! ## Log Formatting
//!
//! The logger provides configurable formatting options:
//!
//! - **Format Patterns**: Custom format strings using placeholders for log components.
//!   Supports strftime-style time formatting.
//!
//! - **Structured Format**: JSON output for machine-readable logs suitable for log
//!   aggregation and analysis tools.
//!
//! - **Pretty Print**: Human-readable output with colors and indentation for development
//!   and debugging.
//!
//! - **Compact Format**: Minimal output for high-volume logging scenarios.
//!
//! ## Log Rotation
//!
//! File-based logging implements comprehensive rotation strategies:
//!
//! - **Size-Based Rotation**: Rotate when log file reaches specified size threshold.
//!   Creates numbered backup files (.1, .2, etc.).
//!
//! - **Time-Based Rotation**: Rotate at specified time intervals (hourly, daily, weekly).
//!   Supports timestamp-based file naming.
//!
//! - **Retention Policies**: Configure maximum number of log files to retain and maximum
//!   total disk space for logs.
//!
//! - **Compression**: Automatic compression of rotated log files to save disk space.
//!
//! - **Async Write**: Non-blocking writes to disk to prevent logging from impacting
//!   application performance.
//!
//! ## Performance Characteristics
//!
//! Logging operations are optimized for minimal performance impact:
//!
//! - **Log Level Filtering**: Compile-time and runtime level checks prevent unnecessary
//!   message construction for disabled log levels.
//!
//! - **Async Logging**: Non-blocking API with background thread for log processing.
//!   Producer-consumer pattern prevents slow I/O from blocking application.
//!
//! - **Batching**: Multiple log entries batched together for efficient I/O operations.
//!
//! - **Memory Allocation**: Arena-based string formatting reduces per-message allocations.
//!
//! - **Lazy Evaluation**: Structured fields evaluated only when log level is enabled.
//!
//! ## Memory Management
//!
//! All C API objects use opaque pointers with manual memory management:
//!
//! - Constructor functions allocate new instances on the heap
//! - Destructor functions must be called to release memory
//! - Logger instances should be freed during application shutdown
//! - Log messages are managed internally by the logger
//!
//! ## Thread Safety
//!
//! The underlying implementations are thread-safe:
//!
//! - Concurrent log calls from multiple threads are synchronized internally
//! - Configuration can be modified safely at runtime
//! - Log sinks handle concurrent writes from multiple threads
//! - Async logging uses lock-free queues for producer-consumer pattern
//!
//! ## Usage Example
//!
//! ```c
//! // Create log configuration
//! DMSCLogConfig* config = dmsc_log_config_new();
//! if (config == NULL) {
//!     fprintf(stderr, "Failed to create log config\n");
//!     return ERROR_INIT;
//! }
//!
//! // Configure logging
//! dmsc_log_config_set_level(config, LOG_LEVEL_DEBUG);
//! dmsc_log_config_set_format(config, "[%t] %l: %m");
//! dmsc_log_config_set_console_sink(config, true);
//!
//! // Enable file logging with rotation
//! dmsc_log_config_set_file_sink(config, "/var/log/app.log", 10485760, 5);
//!
//! // Create logger instance
//! DMSCLogger* logger = dmsc_logger_new(config);
//! if (logger == NULL) {
//!     fprintf(stderr, "Failed to create logger\n");
//!     dmsc_log_config_free(config);
//!     return ERROR_INIT;
//! }
//!
//! // Log messages at different levels
//! dmsc_log_trace(logger, "Entering function calculate_metrics");
//! dmsc_log_debug(logger, "Processing request %d for user %s", request_id, username);
//! dmsc_log_info(logger, "User %s logged in successfully", username);
//! dmsc_log_warn(logger, "Rate limit approaching threshold: %d/%d", current, max);
//! dmsc_log_error(logger, "Failed to connect to database: %s", error_message);
//! dmsc_log_fatal(logger, "Critical failure in payment processing");
//!
//! // Structured logging with fields
//! DMSCHookContext* fields = dmsc_hook_context_create();
//! dmsc_hook_context_set_string(fields, "user_id", "12345");
//! dmsc_hook_context_set_int(fields, "request_duration_ms", 150);
//! dmsc_hook_context_set_string(fields, "endpoint", "/api/users");
//!
//! dmsc_log_with_fields(logger, LOG_LEVEL_INFO, "Request completed", fields);
//! dmsc_hook_context_free(fields);
//!
//! // Change log level at runtime
//! dmsc_logger_set_level(logger, LOG_LEVEL_INFO);
//!
//! // Flush pending logs
//! dmsc_logger_flush(logger);
//!
//! // Cleanup
//! dmsc_logger_free(logger);
//! dmsc_log_config_free(config);
//! ```
//!
//! ## Dependencies
//!
//! This module depends on the following DMSC components:
//!
//! - `crate::log`: Rust logging module implementation
//! - `crate::prelude`: Common types and traits
//! - time for timestamp formatting
//!
//! ## Feature Flags
//!
//! The logging module is always enabled as it provides fundamental observability
//! infrastructure for DMSC applications.
//!
//! Additional sinks enabled by feature flags:
//!
//! - `log-syslog`: Enable syslog sink (Unix only)
//! - `log-journald`: Enable journald sink (systemd systems)
//! - `log-network`: Enable network sink for remote logging

use crate::log::{DMSCLogConfig, DMSCLogger};


c_wrapper!(CDMSCLogConfig, DMSCLogConfig);
c_wrapper!(CDMSCLogger, DMSCLogger);

// DMSCLogConfig constructors and destructors
c_constructor!(
    dmsc_log_config_new,
    CDMSCLogConfig,
    DMSCLogConfig,
    DMSCLogConfig::default()
);
c_destructor!(dmsc_log_config_free, CDMSCLogConfig);
