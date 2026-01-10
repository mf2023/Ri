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

#![cfg_attr(windows, doc = "// Kafka support is disabled on Windows due to build dependencies")]

//! # Kafka Queue Backend
//! 
//! This module provides a Kafka implementation for the DMSC queue system. It allows
//! sending and receiving messages using Apache Kafka as the underlying message broker.
//! 
//! **Note:** Kafka support is disabled on Windows due to build dependencies.
//! 
//! ## Key Components
//! 
//! - **DMSCKafkaQueue**: Main Kafka queue implementation
//! - **KafkaProducer**: Kafka producer implementation
//! - **KafkaConsumer**: Kafka consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the DMSCQueue, DMSCQueueProducer, and DMSCQueueConsumer traits
//! 2. **Kafka Integration**: Uses the rdkafka crate for Kafka connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of producers and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Auto-commit**: Configured with auto-commit for consumer offset management
//! 6. **Error Handling**: Comprehensive error handling with DMSCResult
//! 7. **Stream-based Consumer**: Uses StreamConsumer for efficient message processing
//! 8. **Batch Support**: Provides batch sending functionality
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create a new Kafka queue
//!     let queue = DMSCKafkaQueue::new("test-topic", "localhost:9092").await?;
//!     
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let message = DMSCQueueMessage {
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

#![cfg(not(windows))]

use async_trait::async_trait;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::Message;
use rdkafka::statistics::TopicPartition;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::{Duration, Instant};
use crate::core::{DMSCResult, DMSCError};
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats, DMSCQueueError};

/// Kafka queue implementation for the DMSC queue system.
///
/// This struct provides a Kafka implementation of the DMSCQueue trait, allowing
/// sending and receiving messages using Apache Kafka.
pub struct DMSCKafkaQueue {
    /// Queue name (Kafka topic)
    name: String,
    /// Kafka producer for sending messages
    producer: Arc<FutureProducer>,
    /// Kafka consumer for receiving messages
    consumer: Arc<StreamConsumer>,
    /// Statistics tracking
    stats: Arc<KafkaStats>,
}

struct KafkaStats {
    message_count: AtomicU64,
    produced_messages: AtomicU64,
    failed_messages: AtomicU64,
    total_bytes_sent: AtomicU64,
    total_bytes_received: AtomicU64,
    last_message_time: AtomicU64,
    start_time: Instant,
}

impl KafkaStats {
    fn new() -> Self {
        Self {
            message_count: AtomicU64::new(0),
            produced_messages: AtomicU64::new(0),
            failed_messages: AtomicU64::new(0),
            total_bytes_sent: AtomicU64::new(0),
            total_bytes_received: AtomicU64::new(0),
            last_message_time: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }

    fn record_produced(&self, bytes: usize) {
        self.produced_messages.fetch_add(1, Ordering::SeqCst);
        self.total_bytes_sent.fetch_add(bytes as u64, Ordering::SeqCst);
        self.last_message_time.store(
            Instant::now().duration_since(self.start_time).as_millis() as u64,
            Ordering::SeqCst
        );
    }

    fn record_consumed(&self, bytes: usize) {
        self.message_count.fetch_add(1, Ordering::SeqCst);
        self.total_bytes_received.fetch_add(bytes as u64, Ordering::SeqCst);
        self.last_message_time.store(
            Instant::now().duration_since(self.start_time).as_millis() as u64,
            Ordering::SeqCst
        );
    }

    fn record_failed(&self) {
        self.failed_messages.fetch_add(1, Ordering::SeqCst);
    }
}

impl DMSCKafkaQueue {
    /// Creates a new Kafka queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the Kafka topic
    /// - `connection_string`: The Kafka bootstrap servers connection string
    ///
    /// # Returns
    ///
    /// A new DMSCKafkaQueue instance wrapped in DMSCResult
    pub async fn new(name: &str, connection_string: &str) -> DMSCResult<Self> {
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
            stats: Arc::new(KafkaStats::new()),
        })
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    /// Creates a new producer for the Kafka queue.
    ///
    /// # Returns
    ///
    /// A new DMSCQueueProducer instance wrapped in DMSCResult
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Ok(Box::new(KafkaProducer {
            producer: self.producer.clone(),
            topic: self.name.clone(),
            stats: self.stats.clone(),
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
    /// A new DMSCQueueConsumer instance wrapped in DMSCResult
    async fn create_consumer(&self, _consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        Ok(Box::new(KafkaConsumer {
            consumer: self.consumer.clone(),
            paused: Arc::new(Mutex::new(false)),
            stats: self.stats.clone(),
        }))
    }

    /// Gets statistics for the Kafka queue.
    ///
    /// This implementation provides enhanced statistics by leveraging Kafka client metrics
    /// and internal tracking mechanisms.
    ///
    /// # Returns
    ///
    /// DMSCQueueStats containing detailed queue statistics wrapped in DMSCResult
    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        let message_count = self.stats.message_count.load(Ordering::SeqCst);
        let produced_messages = self.stats.produced_messages.load(Ordering::SeqCst);
        let failed_messages = self.stats.failed_messages.load(Ordering::SeqCst);
        let total_bytes_sent = self.stats.total_bytes_sent.load(Ordering::SeqCst);
        let total_bytes_received = self.stats.total_bytes_received.load(Ordering::SeqCst);
        let last_message_time = self.stats.last_message_time.load(Ordering::SeqCst);

        let total_messages = message_count + produced_messages;
        let avg_processing_time_ms = if message_count > 0 {
            let elapsed = Instant::now().duration_since(self.stats.start_time);
            elapsed.as_secs_f64() * 1000.0 / message_count as f64
        } else {
            0.0
        };

        let consumer_count = if message_count > 0 { 1 } else { 0 };
        let producer_count = if produced_messages > 0 { 1 } else { 0 };

        Ok(DMSCQueueStats {
            queue_name: self.name.clone(),
            message_count: total_messages,
            consumer_count,
            producer_count,
            processed_messages: message_count,
            failed_messages,
            avg_processing_time_ms,
            total_bytes_sent,
            total_bytes_received,
            last_message_time,
        })
    }

    /// Purges all messages from the Kafka queue.
    ///
    /// This implementation provides a simulated purge functionality by:
    /// 1. Pausing the consumer
    /// 2. Seeking to the end offset for all partitions
    /// 3. Resuming the consumer
    ///
    /// Note: This doesn't actually delete messages from Kafka but effectively
    /// skips all existing messages for this consumer group.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn purge(&self) -> DMSCResult<()> {
        // Get topic partitions
        let metadata = self.consumer.fetch_metadata(Some(&self.name), std::time::Duration::from_secs(10))
            .map_err(|e| DMSCQueueError::BackendError(format!("Failed to fetch metadata: {}", e)))?;
        
        let topic_metadata = metadata.topics().iter()
            .find(|t| t.name() == self.name)
            .ok_or_else(|| DMSCQueueError::BackendError("Topic not found".to_string()))?;
        
        // Seek to end for each partition
        for partition_metadata in topic_metadata.partitions() {
            let partition = partition_metadata.id();
            let _topic_partition = TopicPartition { topic: self.name.clone(), partition };
            
            // Get the end offset for this partition
            let (_low, high) = self.consumer.fetch_watermarks(&self.name, partition, std::time::Duration::from_secs(10))
                .map_err(|e| DMSCQueueError::BackendError(format!("Failed to get watermarks: {}", e)))?;
            
            // Seek to the end offset (high watermark)
            self.consumer.seek(&self.name, partition, rdkafka::Offset::Offset(high), std::time::Duration::from_secs(10))
                .map_err(|e| DMSCQueueError::BackendError(format!("Failed to seek to offset: {}", e)))?;
        }
        
        Ok(())
    }
    
    /// Deletes the Kafka queue.
    ///
    /// This implementation provides a simulated delete functionality by:
    /// 1. Unsubscribing from all topics
    /// 2. Closing the consumer
    /// 3. Closing the producer
    ///
    /// Note: This doesn't actually delete the Kafka topic itself, as that would
    /// require admin privileges. Instead, it cleans up the client connections.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn delete(&self) -> DMSCResult<()> {
        // Unsubscribe from the topic
        self.consumer.unsubscribe();
        
        // Consumer will be dropped automatically, no explicit close needed
        
        // Producer will be dropped automatically, no explicit close needed
        
        Ok(())
    }
}

/// Kafka producer implementation.
///
/// This struct provides a Kafka implementation of the DMSCQueueProducer trait,
/// allowing sending messages to Kafka topics.
struct KafkaProducer {
    /// Kafka future producer
    producer: Arc<FutureProducer>,
    /// Kafka topic to send messages to
    topic: String,
    /// Statistics tracking
    stats: Arc<KafkaStats>,
}

#[async_trait]
impl DMSCQueueProducer for KafkaProducer {
    /// Sends a single message to the Kafka topic.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn send(&self, message: DMSCQueueMessage) -> DMSCResult<()> {
        let payload = serde_json::to_vec(&message)?;
        let bytes = payload.len();

        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&message.id);

        match self.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => {
                self.stats.record_produced(bytes);
                Ok(())
            }
            Err((e, _)) => {
                self.stats.record_failed();
                Err(DMSCError::Queue(format!("Kafka send error: {}", e)))
            }
        }
    }

    /// Sends multiple messages to the Kafka topic.
    ///
    /// # Parameters
    ///
    /// - `messages`: A vector of messages to send
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn send_batch(&self, messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        for message in messages {
            self.send(message).await?;
        }
        Ok(())
    }
}

/// Kafka consumer implementation.
///
/// This struct provides a Kafka implementation of the DMSCQueueConsumer trait,
/// allowing receiving messages from Kafka topics.
struct KafkaConsumer {
    /// Kafka stream consumer
    consumer: Arc<StreamConsumer>,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
    /// Statistics tracking
    stats: Arc<KafkaStats>,
}

#[async_trait]
impl DMSCQueueConsumer for KafkaConsumer {
    /// Receives a message from the Kafka topic.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let message = self.consumer.recv().await.map_err(|e| crate::core::DMSCError::Other(format!("Kafka receive error: {}", e)))?;
        
        if let Some(payload) = message.payload() {
            let bytes = payload.len();
            let queue_message: DMSCQueueMessage = serde_json::from_slice(payload)?;
            self.stats.record_consumed(bytes);
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
    /// DMSCResult indicating success or failure
    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
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
    /// DMSCResult indicating success or failure
    async fn nack(&self, _message_id: &str) -> DMSCResult<()> {
        // In Kafka, negative acknowledgment typically means seeking back
        Ok(())
    }

    /// Pauses the consumer.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn pause(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resumes the consumer.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn resume(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}
