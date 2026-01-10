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

//! # Queue Core Implementation
//! 
//! This file defines the core queueing interfaces and message structures for the DMSC queue system.
//! It provides the fundamental building blocks for implementing various queue backends.
//! 
//! ## Key Components
//! 
//! - **DMSCQueueMessage**: Message structure for queue operations
//! - **QueueStats**: Statistics for queue monitoring
//! - **DMSCQueueProducer**: Trait for producing messages to queues
//! - **DMSCQueueConsumer**: Trait for consuming messages from queues
//! - **DMSCQueue**: Main queue trait defining queue operations
//! 
//! ## Design Principles
//! 
//! 1. **Async-First**: All queue operations are asynchronous
//! 2. **Type Safety**: Strongly typed message structures
//! 3. **Retry Mechanism**: Built-in support for message retry with configurable maximum attempts
//! 4. **Header Support**: Allows adding custom headers to messages
//! 5. **Statistics**: Comprehensive queue statistics for monitoring
//! 6. **Extensible**: Easy to implement new queue backends
//! 7. **Thread-safe**: All traits are Send + Sync for safe concurrent use
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::queue::{DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueue};
//! use dms::core::DMSCResult;
//! use serde_json::json;
//! 
//! async fn example(queue: &dyn DMSCQueue) -> DMSCResult<()> {
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let payload = json!({ "key": "value" }).to_string().into_bytes();
//!     let message = DMSCQueueMessage::new(payload)
//!         .with_max_retries(5);
//!     
//!     // Send the message
//!     producer.send(message).await?;
//!     
//!     // Create a consumer
//!     let consumer = queue.create_consumer("consumer_group_1").await?;
//!     
//!     // Receive a message
//!     if let Some(message) = consumer.receive().await? {
//!         // Process the message
//!         let payload = String::from_utf8_lossy(&message.payload);
//!         println!("Received message: {}", payload);
//!         
//!         // Acknowledge the message
//!         consumer.ack(&message.id).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::core::DMSCResult;

/// Error types for queue operations.
#[derive(Debug, Clone)]
pub enum DMSCQueueError {
    /// Backend-specific error with descriptive message
    BackendError(String),
    /// Configuration error
    ConfigError(String),
    /// Connection error
    ConnectionError(String),
    /// Message not found
    MessageNotFound(String),
    /// Consumer group error
    ConsumerGroupError(String),
    /// Serialization error
    SerializationError(String),
}

impl std::fmt::Display for DMSCQueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DMSCQueueError::BackendError(msg) => write!(f, "Queue backend error: {}", msg),
            DMSCQueueError::ConfigError(msg) => write!(f, "Queue configuration error: {}", msg),
            DMSCQueueError::ConnectionError(msg) => write!(f, "Queue connection error: {}", msg),
            DMSCQueueError::MessageNotFound(msg) => write!(f, "Message not found: {}", msg),
            DMSCQueueError::ConsumerGroupError(msg) => write!(f, "Consumer group error: {}", msg),
            DMSCQueueError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for DMSCQueueError {}

impl From<DMSCQueueError> for crate::core::DMSCError {
    fn from(error: DMSCQueueError) -> Self {
        crate::core::DMSCError::Queue(error.to_string())
    }
}

/// Message structure for queue operations.
/// 
/// This struct represents a message that can be sent to and received from queues. It includes
/// a unique ID, payload, headers, timestamp, and retry information.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCQueueMessage {
    /// Unique message ID
    pub id: String,
    /// Message payload as bytes
    pub payload: Vec<u8>,
    /// Custom headers for the message
    pub headers: HashMap<String, String>,
    /// Timestamp when the message was created
    pub timestamp: SystemTime,
    /// Number of times this message has been retried
    pub retry_count: u32,
    /// Maximum number of retry attempts allowed
    pub max_retries: u32,
}

impl DMSCQueueMessage {
    /// Creates a new message with the given payload.
    /// 
    /// # Parameters
    /// 
    /// - `payload`: The message payload as bytes
    /// 
    /// # Returns
    /// 
    /// A new `DMSCQueueMessage` instance
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            payload,
            headers: HashMap::new(),
            timestamp: SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
        }
    }

    /// Adds custom headers to the message.
    /// 
    /// # Parameters
    /// 
    /// - `headers`: A HashMap of custom headers
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCQueueMessage` instance
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    /// Sets the maximum number of retry attempts for this message.
    /// 
    /// # Parameters
    /// 
    /// - `max_retries`: The maximum number of retry attempts
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCQueueMessage` instance
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Increments the retry count for this message.
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }

    /// Checks if this message can be retried.
    /// 
    /// # Returns
    /// 
    /// `true` if the message can be retried, `false` otherwise
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCQueueMessage
#[pyo3::prelude::pymethods]
impl DMSCQueueMessage {
    #[new]
    fn py_new(payload: Vec<u8>) -> Self {
        Self::new(payload)
    }
    
    #[staticmethod]
    fn py_new_with_string(payload: String) -> Self {
        Self::new(payload.into_bytes())
    }
    
    fn get_payload_string(&self) -> String {
        String::from_utf8_lossy(&self.payload).to_string()
    }
    
    fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Statistics for queue monitoring.
/// 
/// This struct contains comprehensive statistics about a queue's performance and usage.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSCQueueStats {
    /// Name of the queue
    pub queue_name: String,
    /// Current number of messages in the queue
    pub message_count: u64,
    /// Number of active consumers
    pub consumer_count: u32,
    /// Number of active producers
    pub producer_count: u32,
    /// Total number of processed messages
    pub processed_messages: u64,
    /// Total number of failed messages
    pub failed_messages: u64,
    /// Average processing time in milliseconds
    pub avg_processing_time_ms: f64,
    /// Total bytes sent
    pub total_bytes_sent: u64,
    /// Total bytes received
    pub total_bytes_received: u64,
    /// Timestamp of last message (milliseconds since start)
    pub last_message_time: u64,
}

#[cfg(feature = "pyo3")]
/// Python bindings for DMSCQueueStats
#[pyo3::prelude::pymethods]
impl DMSCQueueStats {
    #[new]
    fn py_new(queue_name: String) -> Self {
        Self {
            queue_name,
            message_count: 0,
            consumer_count: 0,
            producer_count: 0,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        }
    }
}

/// Trait for producing messages to queues.
/// 
/// This trait defines the interface for sending messages to queues, including single message
/// sends and batch sends.
#[async_trait]
pub trait DMSCQueueProducer: Send + Sync {
    async fn send(&self, message: DMSCQueueMessage) -> DMSCResult<()>;
    
    async fn send_batch(&self, messages: Vec<DMSCQueueMessage>) -> DMSCResult<()>;

    async fn send_multi(&self, messages: &[DMSCQueueMessage]) -> DMSCResult<()> {
        for message in messages {
            self.send(message.clone()).await?;
        }
        Ok(())
    }
}

/// Trait for consuming messages from queues.
/// 
/// This trait defines the interface for receiving and acknowledging messages from queues.
#[async_trait]
pub trait DMSCQueueConsumer: Send + Sync {
    /// Receives a message from the queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Option<DMSCQueueMessage>>` containing the message if available, or None if no message is available
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>>;
    
    /// Acknowledges a message, indicating it has been successfully processed.
    /// 
    /// # Parameters
    /// 
    /// - `message_id`: The ID of the message to acknowledge
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn ack(&self, message_id: &str) -> DMSCResult<()>;
    
    /// Negatively acknowledges a message, indicating it failed to process and should be retried.
    /// 
    /// # Parameters
    /// 
    /// - `message_id`: The ID of the message to negatively acknowledge
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn nack(&self, message_id: &str) -> DMSCResult<()>;
    
    /// Pauses message consumption.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn pause(&self) -> DMSCResult<()>;
    
    /// Resumes message consumption after pausing.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn resume(&self) -> DMSCResult<()>;

    async fn receive_multi(&self, count: usize) -> DMSCResult<Vec<Option<DMSCQueueMessage>>> {
        let mut messages = Vec::with_capacity(count);
        for _ in 0..count {
            messages.push(self.receive().await?);
        }
        Ok(messages)
    }

    async fn ack_multi(&self, message_ids: &[String]) -> DMSCResult<()> {
        for id in message_ids {
            self.ack(id).await?;
        }
        Ok(())
    }
}

/// Main queue trait defining queue operations.
/// 
/// This trait defines the core operations for queues, including creating producers and consumers,
/// getting statistics, purging queues, and deleting queues.
#[async_trait]
pub trait DMSCQueue: Send + Sync {
    /// Creates a new producer for this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Box<dyn DMSCQueueProducer>>` containing the producer
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>>;
    
    /// Creates a new consumer for this queue with the given consumer group.
    /// 
    /// # Parameters
    /// 
    /// - `consumer_group`: The name of the consumer group
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<Box<dyn DMSCQueueConsumer>>` containing the consumer
    async fn create_consumer(&self, consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>>;
    
    /// Gets statistics for this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<DMSCQueueStats>` containing the queue statistics
    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats>;
    
    /// Purges all messages from this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn purge(&self) -> DMSCResult<()>;
    
    /// Deletes this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn delete(&self) -> DMSCResult<()>;
}
