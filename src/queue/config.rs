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

#![allow(non_snake_case)]

//! # Queue Configuration
//!
//! This file defines the configuration structures for the DMSC queue system. These structures
//! provide a centralized way to configure queue behavior, including backend selection,
//! connection settings, retry policies, and dead letter queue configuration.
//!
//! ## Key Components
//!
//! - **DMSCQueueConfig**: Main queue configuration structure
//! - **QueueBackendType**: Enum for supported queue backends
//! - **RetryPolicy**: Configuration for message retry behavior
//! - **DeadLetterConfig**: Configuration for dead letter queue functionality
//!
//! ## Design Principles
//!
//! 1. **Default Values**: All configuration structures have sensible default values
//! 2. **Serialization Support**: All structures are serializable/deserializable for config file support
//! 3. **Type Safety**: Strongly typed enums for backend selection
//! 4. **Flexibility**: Supports multiple queue backends through a unified configuration
//! 5. **Retry Mechanism**: Configurable retry policies with exponential backoff support
//! 6. **Dead Letter Support**: Optional dead letter queue configuration for failed messages
//! 7. **Backend Agnostic**: Configuration can be used with any queue backend
//! 8. **Timeout Configuration**: Separate timeouts for producers and consumers
//!
//! ## Usage
//!
//! ```rust
//! use dms::queue::{DMSCQueueConfig, QueueBackendType, RetryPolicy, DeadLetterConfig};
//!
//! // Create default queue configuration
//! let default_config = DMSCQueueConfig::default();
//!
//! // Create custom queue configuration
//! let custom_config = DMSCQueueConfig {
//!     enabled: true,
//!     backend_type: QueueBackendType::RabbitMQ,
//!     connection_string: "amqp://guest:guest@localhost:5672/".to_string(),
//!     max_connections: 20,
//!     message_max_size: 2 * 1024 * 1024, // 2MB
//!     consumer_timeout_ms: 60000, // 60 seconds
//!     producer_timeout_ms: 10000, // 10 seconds
//!     retry_policy: RetryPolicy {
//!         max_retries: 5,
//!         initial_delay_ms: 2000,
//!         max_delay_ms: 30000,
//!         backoff_multiplier: 1.5,
//!     },
//!     dead_letter_config: Some(DeadLetterConfig {
//!         enabled: true,
//!         max_retry_count: 3,
//!         dead_letter_queue_name: "dead_letters".to_string(),
//!         ttl_hours: 24,
//!     }),
//! };
//!
//! // Parse backend type from string
//! let backend_type = QueueBackendType::from_str("redis").unwrap();
//! assert_eq!(backend_type, QueueBackendType::Redis);
//! ```

use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Main queue configuration structure.
///
/// This structure contains all the configuration options for the queue system, including
/// backend selection, connection settings, timeouts, retry policies, and dead letter queue
/// configuration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCQueueConfig {
    /// Whether the queue system is enabled
    pub enabled: bool,
    /// The type of queue backend to use
    pub backend_type: DMSCQueueBackendType,
    /// Connection string for the queue backend
    pub connection_string: String,
    /// Maximum number of connections to the queue backend
    pub max_connections: u32,
    /// Maximum size of messages in bytes
    pub message_max_size: usize,
    /// Timeout for consumer operations in milliseconds
    pub consumer_timeout_ms: u64,
    /// Timeout for producer operations in milliseconds
    pub producer_timeout_ms: u64,
    /// Configuration for message retry behavior
    pub retry_policy: DMSCRetryPolicy,
    /// Configuration for dead letter queue functionality
    pub dead_letter_config: Option<DMSCDeadLetterConfig>,
}

impl Default for DMSCQueueConfig {
    /// Creates a new queue configuration with sensible default values.
    ///
    /// # Returns
    ///
    /// A `DMSCQueueConfig` instance with default values
    fn default() -> Self {
        Self {
            enabled: true,
            backend_type: DMSCQueueBackendType::Memory,
            connection_string: "memory://localhost".to_string(),
            max_connections: 10,
            message_max_size: 1024 * 1024, // 1MB
            consumer_timeout_ms: 30000,    // 30 seconds
            producer_timeout_ms: 5000,     // 5 seconds
            retry_policy: DMSCRetryPolicy::default(),
            dead_letter_config: None,
        }
    }
}

/// Enum representing supported queue backend types.
///
/// This enum defines the different queue backends that can be used with the DMSC queue system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DMSCQueueBackendType {
    /// In-memory queue implementation for testing and development
    Memory,
    /// RabbitMQ queue backend for production use
    RabbitMQ,
    /// Kafka queue backend for high-throughput scenarios
    Kafka,
    /// Redis queue backend for simple, lightweight queueing
    Redis,
}

impl FromStr for DMSCQueueBackendType {
    type Err = String;

    /// Parses a string into a QueueBackendType.
    ///
    /// # Parameters
    ///
    /// - `s`: The string to parse
    ///
    /// # Returns
    ///
    /// A `Result<Self, Self::Err>` containing the parsed backend type or an error
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "memory" => Ok(DMSCQueueBackendType::Memory),
            "rabbitmq" => Ok(DMSCQueueBackendType::RabbitMQ),
            "kafka" => Ok(DMSCQueueBackendType::Kafka),
            "redis" => Ok(DMSCQueueBackendType::Redis),
            _ => Err(format!("Unknown queue backend type: {s}")),
        }
    }
}

/// Configuration for message retry behavior.
///
/// This structure defines the retry policy for failed messages, including maximum retry
/// attempts, initial delay, maximum delay, and backoff multiplier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCRetryPolicy {
    /// Maximum number of retry attempts for a failed message
    pub max_retries: u32,
    /// Initial delay in milliseconds before the first retry
    pub initial_delay_ms: u64,
    /// Maximum delay in milliseconds between retries
    pub max_delay_ms: u64,
    /// Multiplier for exponential backoff (e.g., 2.0 for doubling delay each retry)
    pub backoff_multiplier: f64,
}

impl Default for DMSCRetryPolicy {
    /// Creates a new retry policy with sensible default values.
    ///
    /// # Returns
    ///
    /// A `DMSCRetryPolicy` instance with default values
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Configuration for dead letter queue functionality.
///
/// This structure defines the configuration for dead letter queues, which are used to store
/// messages that have failed to process after maximum retry attempts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDeadLetterConfig {
    /// Whether dead letter queue functionality is enabled
    pub enabled: bool,
    /// Maximum number of retry attempts before a message is sent to the dead letter queue
    pub max_retry_count: u32,
    /// Name of the dead letter queue
    pub dead_letter_queue_name: String,
    /// Time-to-live for messages in the dead letter queue in hours
    pub ttl_hours: u32,
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCQueueConfig
#[pyo3::prelude::pymethods]
impl DMSCQueueConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn py_new_with_memory_backend() -> Self {
        Self {
            enabled: true,
            backend_type: DMSCQueueBackendType::Memory,
            connection_string: "memory://localhost".to_string(),
            ..Self::default()
        }
    }
    
    #[staticmethod]
    fn py_new_with_redis_backend(connection_string: String) -> Self {
        Self {
            enabled: true,
            backend_type: DMSCQueueBackendType::Redis,
            connection_string,
            ..Self::default()
        }
    }
}
