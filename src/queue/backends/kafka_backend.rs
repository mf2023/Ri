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

//! # Kafka Queue Backend
//! 
//! This module provides a Kafka implementation for the DMS queue system. It allows
//! sending and receiving messages using Apache Kafka as the underlying message broker.
//! 
//! ## Key Components
//! 
//! - **DMSKafkaQueue**: Main Kafka queue implementation
//! - **KafkaProducer**: Kafka producer implementation
//! - **KafkaConsumer**: Kafka consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the DMSQueue, DMSQueueProducer, and DMSQueueConsumer traits
//! 2. **Kafka Integration**: Uses the rdkafka crate for Kafka connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of producers and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Auto-commit**: Configured with auto-commit for consumer offset management
//! 6. **Error Handling**: Comprehensive error handling with DMSResult
//! 7. **Stream-based Consumer**: Uses StreamConsumer for efficient message processing
//! 8. **Batch Support**: Provides batch sending functionality
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a new Kafka queue
//!     let queue = DMSKafkaQueue::new("test-topic", "localhost:9092").await?;
//!     
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let message = DMSQueueMessage {
//!         id: "12345".to_string(),
//!         payload: b"Hello, Kafka!".to_vec(),
//!         headers: vec![("key1".to_string(), "value1".to_string())],
//!         timestamp: chrono::Utc::now().timestamp_millis() as u64,
//!         priority: 0,
//!     };
//!     
//!     // Send the message
//!     producer.send(message).await?;
//!     
//!     // Create a consumer
//!     let consumer = queue.create_consumer("test-group").await?;
//!     
//!     // Receive messages
//!     if let Some(received_message) = consumer.receive().await? {
//!         println!("Received message: {:?}", received_message);
//!         consumer.ack(&received_message.id).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

/// Kafka queue implementation for the DMS queue system.
///
/// This struct provides a Kafka implementation of the DMSQueue trait, allowing
/// sending and receiving messages using Apache Kafka.
pub struct DMSKafkaQueue {
    /// Queue name (Kafka topic)
    name: String,
    /// Kafka producer for sending messages
    producer: Arc<FutureProducer>,
    /// Kafka consumer for receiving messages
    consumer: Arc<StreamConsumer>,
}

impl DMSKafkaQueue {
    /// Creates a new Kafka queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the Kafka topic
    /// - `connection_string`: The Kafka bootstrap servers connection string
    ///
    /// # Returns
    ///
    /// A new DMSKafkaQueue instance wrapped in DMSResult
    pub async fn new(name: &str, connection_string: &str) -> DMSResult<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", connection_string)
            .set("message.timeout.ms", "5000")
            .create()?;

        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", connection_string)
            .set("group.id", format!("{}-group", name))
            .set("enable.auto.commit", "true")
            .create()?;

        consumer.subscribe(&[&name])?;

        Ok(Self {
            name: name.to_string(),
            producer: Arc::new(producer),
            consumer: Arc::new(consumer),
        })
    }
}

#[async_trait]
impl DMSQueue for DMSKafkaQueue {
    /// Creates a new producer for the Kafka queue.
    ///
    /// # Returns
    ///
    /// A new DMSQueueProducer instance wrapped in DMSResult
    async fn create_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        Ok(Box::new(KafkaProducer {
            producer: self.producer.clone(),
            topic: self.name.clone(),
        }))
    }

    /// Creates a new consumer for the Kafka queue.
    ///
    /// # Parameters
    ///
    /// - `_consumer_group`: The consumer group name (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// A new DMSQueueConsumer instance wrapped in DMSResult
    async fn create_consumer(&self, _consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        Ok(Box::new(KafkaConsumer {
            consumer: self.consumer.clone(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    /// Gets statistics for the Kafka queue.
    ///
    /// Note: This implementation returns basic stats since Kafka provides detailed metrics
    /// through JMX or admin client API, which is not implemented here.
    ///
    /// # Returns
    ///
    /// QueueStats containing basic queue statistics wrapped in DMSResult
    async fn get_stats(&self) -> DMSResult<QueueStats> {
        // Kafka provides metrics through JMX or admin client
        // For now, return basic stats
        Ok(QueueStats {
            queue_name: self.name.clone(),
            message_count: 0,
            consumer_count: 0,
            producer_count: 0,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
        })
    }

    /// Purges all messages from the Kafka queue.
    ///
    /// Note: Kafka doesn't support purging topics directly through the client API.
    /// This would require admin operations, which are not implemented here.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn purge(&self) -> DMSResult<()> {
        // Kafka doesn't support purging topics directly
        // This would require admin operations
        Ok(())
    }

    /// Deletes the Kafka queue.
    ///
    /// Note: Kafka doesn't support deleting topics through the client API.
    /// This would require admin operations, which are not implemented here.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn delete(&self) -> DMSResult<()> {
        // Kafka doesn't support deleting topics through client API
        // This would require admin operations
        Ok(())
    }
}

/// Kafka producer implementation.
///
/// This struct provides a Kafka implementation of the DMSQueueProducer trait,
/// allowing sending messages to Kafka topics.
struct KafkaProducer {
    /// Kafka future producer
    producer: Arc<FutureProducer>,
    /// Kafka topic to send messages to
    topic: String,
}

#[async_trait]
impl DMSQueueProducer for KafkaProducer {
    /// Sends a single message to the Kafka topic.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn send(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let payload = serde_json::to_vec(&message)?;
        
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&message.id);

        self.producer.send(record, std::time::Duration::from_secs(0)).await?;
        Ok(())
    }

    /// Sends multiple messages to the Kafka topic.
    ///
    /// # Parameters
    ///
    /// - `messages`: A vector of messages to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn send_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        for message in messages {
            self.send(message).await?;
        }
        Ok(())
    }
}

/// Kafka consumer implementation.
///
/// This struct provides a Kafka implementation of the DMSQueueConsumer trait,
/// allowing receiving messages from Kafka topics.
struct KafkaConsumer {
    /// Kafka stream consumer
    consumer: Arc<StreamConsumer>,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for KafkaConsumer {
    /// Receives a message from the Kafka topic.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    async fn receive(&self) -> DMSResult<Option<DMSQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let message = self.consumer.recv().await.map_err(|e| crate::core::DMSError::Other(format!("Kafka receive error: {}", e)))?;
        
        if let Some(payload) = message.payload() {
            let queue_message: DMSQueueMessage = serde_json::from_slice(payload)?;
            Ok(Some(queue_message))
        } else {
            Ok(None)
        }
    }

    /// Acknowledges a message.
    ///
    /// Note: Kafka is configured with auto-commit, so acknowledgment is automatic.
    /// This method is a no-op in this implementation.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn ack(&self, _message_id: &str) -> DMSResult<()> {
        // Kafka auto-commit is enabled, so acknowledgment is automatic
        Ok(())
    }

    /// Negatively acknowledges a message.
    ///
    /// Note: In Kafka, negative acknowledgment typically means seeking back to the
    /// message offset, which is not implemented here.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to negatively acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn nack(&self, _message_id: &str) -> DMSResult<()> {
        // In Kafka, negative acknowledgment typically means seeking back
        Ok(())
    }

    /// Pauses the consumer.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn pause(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resumes the consumer.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn resume(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}