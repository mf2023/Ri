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

//! # In-Memory Queue Implementation
//!
//! This file implements an in-memory queue backend for the DMSC queue system. The in-memory queue
//! provides a lightweight, fast queue implementation suitable for testing, development, and
//! scenarios where durability is not a strict requirement. It also supports optional persistence
//! to disk for basic durability.
//!
//! ## Key Components
//!
//! - **DMSCMemoryQueue**: Main in-memory queue implementation
//! - **MemoryQueueState**: Internal state management for the queue
//! - **MemoryQueueProducer**: Producer implementation for sending messages
//! - **MemoryQueueConsumer**: Consumer implementation for receiving messages
//!
//! ## Design Principles
//!
//! 1. **Lightweight**: Minimal dependencies and overhead
//! 2. **Fast Performance**: In-memory operations for low latency
//! 3. **Optional Persistence**: Can be configured to persist messages to disk
//! 4. **Consumer Groups**: Supports multiple consumer groups with message distribution
//! 5. **Async-First**: All operations are asynchronous
//! 6. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 7. **Durable Option**: Optional disk persistence for message durability
//! 8. **Simple API**: Implements the standard DMSCQueue interfaces
//! 9. **Non-blocking**: Uses tokio's spawn_blocking for file I/O operations
//! 10. **Message Retry**: Supports message requeueing with retry count increment
//!
//! ## Usage
//!
//! ```rust
//! use dms::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer};
//! use dms::queue::backends::DMSCMemoryQueue;
//! use dms::core::DMSCResult;
//! use serde_json::json;
//!
//! async fn example() -> DMSCResult<()> {
//!     // Create a basic in-memory queue
//!     let queue = DMSCMemoryQueue::new("example_queue");
//!     
//!     // Or create a queue with disk persistence
//!     // let queue = DMSCMemoryQueue::with_persistence("example_queue", "/tmp/queue_persistence");
//!     
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let payload = json!({ "key": "value" }).to_string().into_bytes();
//!     let message = DMSCQueueMessage::new(payload);
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

use crate::core::DMSCResult;
use crate::queue::{DMSCQueue, DMSCQueueConsumer, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueStats};
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::spawn_blocking;

/// Internal state management for the in-memory queue.
///
/// This struct holds the queue's messages and consumer-specific queues. It is protected by a
/// RwLock to ensure thread-safe access.
struct MemoryQueueState {
    /// Main queue of messages waiting to be consumed
    messages: VecDeque<DMSCQueueMessage>,
    /// Map of consumer group names to their respective message queues
    consumers: HashMap<String, VecDeque<DMSCQueueMessage>>,
}

impl MemoryQueueState {
    /// Creates a new MemoryQueueState with empty queues.
    ///
    /// # Returns
    ///
    /// A new MemoryQueueState instance
    fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            consumers: HashMap::new(),
        }
    }
}

/// In-memory queue implementation.
///
/// This struct implements the DMSCQueue trait for an in-memory queue backend. It supports optional
/// disk persistence for message durability.
pub struct DMSCMemoryQueue {
    /// Name of the queue
    name: String,
    /// Internal queue state protected by a RwLock
    state: Arc<RwLock<MemoryQueueState>>,
    /// Optional path for disk persistence
    persistence_path: Option<String>,
}

#[allow(dead_code)]
impl DMSCMemoryQueue {
    /// Creates a new in-memory queue without persistence.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    ///
    /// # Returns
    ///
    /// A new DMSCMemoryQueue instance
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Arc::new(RwLock::new(MemoryQueueState::new())),
            persistence_path: None,
        }
    }

    /// Creates a new in-memory queue with disk persistence.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    /// - `persistence_path`: Path to the file where messages will be persisted
    ///
    /// # Returns
    ///
    /// A new DMSCMemoryQueue instance with persistence enabled
    pub fn with_persistence(name: &str, persistence_path: &str) -> Self {
        let queue = Self {
            name: name.to_string(),
            state: Arc::new(RwLock::new(MemoryQueueState::new())),
            persistence_path: Some(persistence_path.to_string()),
        };

        // Load messages from disk if persistence is enabled
        if let Err(e) = queue.load_messages() {
            log::warn!("Failed to load persisted messages for queue '{name}': {e}");
        }

        queue
    }

    /// Loads messages from disk if persistence is enabled.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    fn load_messages(&self) -> DMSCResult<()> {
        if let Some(path) = &self.persistence_path {
            if Path::new(path).exists() {
                let mut file = File::open(path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;

                if !content.is_empty() {
                    let messages: VecDeque<DMSCQueueMessage> = serde_json::from_str(&content)?;
                    let mut state = self.state.blocking_write();
                    state.messages = messages;
                }
            }
        }
        Ok(())
    }

    /// Saves messages to disk if persistence is enabled.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    fn save_messages(&self) -> DMSCResult<()> {
        if let Some(path) = &self.persistence_path {
            let state = self.state.blocking_read();
            let content = serde_json::to_string(&state.messages)?;

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;

            file.write_all(content.as_bytes())?;
        }
        Ok(())
    }
}

#[async_trait]
impl DMSCQueue for DMSCMemoryQueue {
    /// Creates a new producer for this queue.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Box<dyn DMSCQueueProducer>>` containing the producer
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Ok(Box::new(MemoryQueueProducer {
            state: self.state.clone(),
            persistence_path: self.persistence_path.clone(),
        }))
    }

    /// Creates a new consumer for this queue with the given consumer group.
    ///
    /// # Parameters
    ///
    /// - `consumer_group`: The name of the consumer group
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Box<dyn DMSCQueueConsumer>>` containing the consumer
    async fn create_consumer(
        &self,
        consumer_group: &str,
    ) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        Ok(Box::new(MemoryQueueConsumer {
            state: self.state.clone(),
            consumer_group: consumer_group.to_string(),
            paused: Arc::new(Mutex::new(false)),
            persistence_path: self.persistence_path.clone(),
        }))
    }

    /// Gets statistics for this queue.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<DMSCQueueStats>` containing the queue statistics
    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        let state = self.state.read().await;
        Ok(DMSCQueueStats {
            queue_name: self.name.clone(),
            message_count: state.messages.len() as u64,
            consumer_count: state.consumers.len() as u32,
            producer_count: 1,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        })
    }

    /// Purges all messages from this queue.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn purge(&self) -> DMSCResult<()> {
        let mut state = self.state.write().await;
        state.messages.clear();
        state.consumers.clear();

        // Clear persistence file if enabled
        if let Some(path) = &self.persistence_path {
            let path_clone = path.clone();
            spawn_blocking(move || {
                if Path::new(&path_clone).exists() {
                    if let Err(e) = std::fs::remove_file(&path_clone) {
                        log::warn!("Failed to remove persistence file '{path_clone}': {e}");
                    }
                }
            })
            .await
            .map_err(|e| {
                log::error!("Failed to execute persistence file removal: {e}");
                crate::core::DMSCError::Other(format!("Failed to clear persistence: {e}"))
            })?;
        }

        Ok(())
    }

    /// Deletes this queue.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn delete(&self) -> DMSCResult<()> {
        self.purge().await
    }
}

/// Producer implementation for the in-memory queue.
///
/// This struct implements the DMSCQueueProducer trait for sending messages to the in-memory queue.
struct MemoryQueueProducer {
    /// Shared queue state
    state: Arc<RwLock<MemoryQueueState>>,
    /// Optional path for disk persistence
    persistence_path: Option<String>,
}

#[async_trait]
impl DMSCQueueProducer for MemoryQueueProducer {
    /// Sends a single message to the queue.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn send(&self, message: DMSCQueueMessage) -> DMSCResult<()> {
        let mut state = self.state.write().await;
        state.messages.push_back(message);

        // Save to disk if persistence is enabled
        if let Some(path) = &self.persistence_path {
            let messages_clone = state.messages.clone();
            let path_clone = path.clone();

            let _ = spawn_blocking(move || {
            let content = serde_json::to_string(&messages_clone)
                .map_err(|e| {
                    log::error!("Failed to serialize messages for persistence: {e}");
                    crate::core::DMSCError::Serde(format!("Serialization failed: {e}"))
                })?;
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path_clone)
                .map_err(|e| {
                    log::error!("Failed to open persistence file: {e}");
                    crate::core::DMSCError::Io(format!("File open failed: {e}"))
                })?;
            file.write_all(content.as_bytes())
                .map_err(|e| {
                    log::error!("Failed to write persistence file: {e}");
                    crate::core::DMSCError::Io(format!("File write failed: {e}"))
                })?;
            Ok::<(), crate::core::DMSCError>(())
        })
        .await
        .map_err(|e| {
            log::error!("Failed to execute persistence task: {e}");
            crate::core::DMSCError::Other(format!("Persistence task failed: {e}"))
        });
        }

        Ok(())
    }

    /// Sends multiple messages to the queue in a batch.
    ///
    /// # Parameters
    ///
    /// - `messages`: A vector of messages to send
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn send_batch(&self, messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        let mut state = self.state.write().await;
        for message in messages {
            state.messages.push_back(message);
        }

        // Save to disk if persistence is enabled
        if let Some(path) = &self.persistence_path {
            let messages_clone = state.messages.clone();
            let path_clone = path.clone();

            let _ = spawn_blocking(move || {
            let content = serde_json::to_string(&messages_clone)
                .map_err(|e| {
                    log::error!("Failed to serialize messages for persistence: {e}");
                    crate::core::DMSCError::Serde(format!("Serialization failed: {e}"))
                })?;
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(path_clone)
                .map_err(|e| {
                    log::error!("Failed to open persistence file: {e}");
                    crate::core::DMSCError::Io(format!("File open failed: {e}"))
                })?;
            file.write_all(content.as_bytes())
                .map_err(|e| {
                    log::error!("Failed to write persistence file: {e}");
                    crate::core::DMSCError::Io(format!("File write failed: {e}"))
                })?;
            Ok::<(), crate::core::DMSCError>(())
        })
        .await
        .map_err(|e| {
            log::error!("Failed to execute persistence task: {e}");
            crate::core::DMSCError::Other(format!("Persistence task failed: {e}"))
        });
        }

        Ok(())
    }
}

/// Consumer implementation for the in-memory queue.
///
/// This struct implements the DMSCQueueConsumer trait for receiving messages from the in-memory queue.
struct MemoryQueueConsumer {
    /// Shared queue state
    state: Arc<RwLock<MemoryQueueState>>,
    /// Name of the consumer group
    consumer_group: String,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
    /// Optional path for disk persistence
    persistence_path: Option<String>,
}

#[async_trait]
impl DMSCQueueConsumer for MemoryQueueConsumer {
    /// Receives a message from the queue.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<Option<DMSCQueueMessage>>` containing the message if available, or None if no message is available
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut state = self.state.write().await;

        // If consumer queue exists and has messages, return one
        if let Some(consumer_queue) = state.consumers.get_mut(&self.consumer_group) {
            if let Some(message) = consumer_queue.pop_front() {
                return Ok(Some(message));
            }
        }

        // If main queue has messages, move one to consumer queue
        if let Some(message) = state.messages.pop_front() {
            let mut consumer_queue = VecDeque::new();
            consumer_queue.push_back(message.clone());
            state
                .consumers
                .insert(self.consumer_group.clone(), consumer_queue);

            // Save to disk if persistence is enabled (since main queue changed)
            if let Some(path) = &self.persistence_path {
                let messages_clone = state.messages.clone();
                let path_clone = path.clone();

                let _ = spawn_blocking(move || {
                    let content = serde_json::to_string(&messages_clone)
                        .map_err(|e| {
                            log::error!("Failed to serialize messages for persistence: {e}");
                            crate::core::DMSCError::Serde(format!("Serialization failed: {e}"))
                        })?;
                    let mut file = OpenOptions::new()
                        .write(true)
                        .create(true)
                        .truncate(true)
                        .open(path_clone)
                        .map_err(|e| {
                            log::error!("Failed to open persistence file: {e}");
                            crate::core::DMSCError::Io(format!("File open failed: {e}"))
                        })?;
                    file.write_all(content.as_bytes())
                        .map_err(|e| {
                            log::error!("Failed to write persistence file: {e}");
                            crate::core::DMSCError::Io(format!("File write failed: {e}"))
                        })?;
                    Ok::<(), crate::core::DMSCError>(())
                })
                .await
                .map_err(|e| {
                    log::error!("Failed to execute persistence task: {e}");
                    crate::core::DMSCError::Other(format!("Persistence task failed: {e}"))
                });
            }

            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    /// Acknowledges a message, indicating it has been successfully processed.
    ///
    /// For in-memory queues, acknowledgment is implicit when the message is received.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The ID of the message to acknowledge (not used for in-memory queues)
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
        // In memory queue, acknowledgment is implicit when message is received
        Ok(())
    }

    /// Negatively acknowledges a message, indicating it failed to process and should be retried.
    ///
    /// # Parameters
    ///
    /// - `message_id`: The ID of the message to negatively acknowledge
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn nack(&self, message_id: &str) -> DMSCResult<()> {
        // Find the message in consumer queue and put it back in main queue
        let mut state = self.state.write().await;

        if let Some(consumer_queue) = state.consumers.get_mut(&self.consumer_group) {
            // Find the message by ID
            let mut message_to_requeue: Option<DMSCQueueMessage> = None;

            // Iterate through consumer queue to find the message
            let mut index = 0;
            for (i, message) in consumer_queue.iter().enumerate() {
                if message.id == message_id {
                    message_to_requeue = Some(message.clone());
                    index = i;
                    break;
                }
            }

            // If found, remove from consumer queue and put back in main queue
            if let Some(mut message) = message_to_requeue {
                consumer_queue.remove(index);
                message.increment_retry();
                state.messages.push_back(message);

                // Save to disk if persistence is enabled
                if let Some(path) = &self.persistence_path {
                    let messages_clone = state.messages.clone();
                    let path_clone = path.clone();

                    spawn_blocking(move || {
                        let content = serde_json::to_string(&messages_clone).unwrap();
                        let mut file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(path_clone)
                            .unwrap();
                        file.write_all(content.as_bytes()).unwrap();
                    })
                    .await
                    .unwrap();
                }
            }
        }

        Ok(())
    }

    /// Pauses message consumption.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn pause(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resumes message consumption after pausing.
    ///
    /// # Returns
    ///
    /// A `DMSCResult<()>` indicating success or failure
    async fn resume(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}
