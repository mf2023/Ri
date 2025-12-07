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

//! # Queue Core Implementation
//! 
//! This file defines the core queueing interfaces and message structures for the DMS queue system.
//! It provides the fundamental building blocks for implementing various queue backends.
//! 
//! ## Key Components
//! 
//! - **DMSQueueMessage**: Message structure for queue operations
//! - **QueueStats**: Statistics for queue monitoring
//! - **DMSQueueProducer**: Trait for producing messages to queues
//! - **DMSQueueConsumer**: Trait for consuming messages from queues
//! - **DMSQueue**: Main queue trait defining queue operations
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
//! use dms::queue::{DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, DMSQueue};
//! use dms::core::DMSResult;
//! use serde_json::json;
//! 
//! async fn example(queue: &dyn DMSQueue) -> DMSResult<()> {
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let payload = json!({ "key": "value" }).to_string().into_bytes();
//!     let message = DMSQueueMessage::new(payload)
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
use crate::core::DMSResult;

/// Message structure for queue operations.
/// 
/// This struct represents a message that can be sent to and received from queues. It includes
/// a unique ID, payload, headers, timestamp, and retry information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSQueueMessage {
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

impl DMSQueueMessage {
    /// Creates a new message with the given payload.
    /// 
    /// # Parameters
    /// 
    /// - `payload`: The message payload as bytes
    /// 
    /// # Returns
    /// 
    /// A new `DMSQueueMessage` instance
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
    /// The updated `DMSQueueMessage` instance
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
    /// The updated `DMSQueueMessage` instance
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

/// Statistics for queue monitoring.
/// 
/// This struct contains comprehensive statistics about a queue's performance and usage.
#[derive(Debug, Clone)]
pub struct QueueStats {
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
}

/// Trait for producing messages to queues.
/// 
/// This trait defines the interface for sending messages to queues, including single message
/// sends and batch sends.
#[async_trait]
pub trait DMSQueueProducer: Send + Sync {
    /// Sends a single message to the queue.
    /// 
    /// # Parameters
    /// 
    /// - `message`: The message to send
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn send(&self, message: DMSQueueMessage) -> DMSResult<()>;
    
    /// Sends multiple messages to the queue in a batch.
    /// 
    /// # Parameters
    /// 
    /// - `messages`: A vector of messages to send
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn send_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()>;
}

/// Trait for consuming messages from queues.
/// 
/// This trait defines the interface for receiving and acknowledging messages from queues.
#[async_trait]
pub trait DMSQueueConsumer: Send + Sync {
    /// Receives a message from the queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Option<DMSQueueMessage>>` containing the message if available, or None if no message is available
    async fn receive(&self) -> DMSResult<Option<DMSQueueMessage>>;
    
    /// Acknowledges a message, indicating it has been successfully processed.
    /// 
    /// # Parameters
    /// 
    /// - `message_id`: The ID of the message to acknowledge
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn ack(&self, message_id: &str) -> DMSResult<()>;
    
    /// Negatively acknowledges a message, indicating it failed to process and should be retried.
    /// 
    /// # Parameters
    /// 
    /// - `message_id`: The ID of the message to negatively acknowledge
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn nack(&self, message_id: &str) -> DMSResult<()>;
    
    /// Pauses message consumption.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn pause(&self) -> DMSResult<()>;
    
    /// Resumes message consumption after pausing.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn resume(&self) -> DMSResult<()>;
}

/// Main queue trait defining queue operations.
/// 
/// This trait defines the core operations for queues, including creating producers and consumers,
/// getting statistics, purging queues, and deleting queues.
#[async_trait]
pub trait DMSQueue: Send + Sync {
    /// Creates a new producer for this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Box<dyn DMSQueueProducer>>` containing the producer
    async fn create_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>>;
    
    /// Creates a new consumer for this queue with the given consumer group.
    /// 
    /// # Parameters
    /// 
    /// - `consumer_group`: The name of the consumer group
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Box<dyn DMSQueueConsumer>>` containing the consumer
    async fn create_consumer(&self, consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>>;
    
    /// Gets statistics for this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<QueueStats>` containing the queue statistics
    async fn get_stats(&self) -> DMSResult<QueueStats>;
    
    /// Purges all messages from this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn purge(&self) -> DMSResult<()>;
    
    /// Deletes this queue.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn delete(&self) -> DMSResult<()>;
}