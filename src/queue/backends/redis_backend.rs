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

//! # Redis Queue Backend
//! 
//! This module provides a Redis implementation for the DMS queue system. It allows
//! sending and receiving messages using Redis lists as the underlying message broker.
//! 
//! ## Key Components
//! 
//! - **DMSRedisQueue**: Main Redis queue implementation
//! - **RedisQueueProducer**: Redis producer implementation
//! - **RedisQueueConsumer**: Redis consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the DMSQueue, DMSQueueProducer, and DMSQueueConsumer traits
//! 2. **Redis Integration**: Uses the redis crate for Redis connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of connections and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Blocking Operations**: Uses BLPOP for efficient blocking message consumption
//! 6. **Error Handling**: Comprehensive error handling with DMSResult
//! 7. **List-based Queue**: Uses Redis lists for simple FIFO queue functionality
//! 8. **Batch Support**: Provides batch sending functionality
//! 9. **Implicit Acknowledgment**: Acknowledgment is implicit when messages are popped from the list
//! 10. **Stats Support**: Provides queue length statistics using Redis LLEN command
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a new Redis queue
//!     let queue = DMSRedisQueue::_Fnew("test-queue", "redis://localhost:6379").await?;
//!     
//!     // Create a producer
//!     let producer = queue._Fcreate_producer().await?;
//!     
//!     // Create a message
//!     let message = DMSQueueMessage {
//!         id: "12345".to_string(),
//!         payload: b"Hello, Redis!".to_vec(),
//!         headers: vec![("key1".to_string(), "value1".to_string())],
//!         timestamp: chrono::Utc::now().timestamp_millis() as u64,
//!         priority: 0,
//!     };
//!     
//!     // Send the message
//!     producer._Fsend(message).await?;
//!     
//!     // Create a consumer
//!     let consumer = queue._Fcreate_consumer("test-consumer-group").await?;
//!     
//!     // Receive messages
//!     if let Some(received_message) = consumer._Freceive().await? {
//!         println!("Received message: {:?}", received_message);
//!         consumer._Fack(&received_message.id).await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

/// Redis queue implementation for the DMS queue system.
///
/// This struct provides a Redis implementation of the DMSQueue trait, allowing
/// sending and receiving messages using Redis lists as the underlying message broker.
pub struct DMSRedisQueue {
    /// Queue name (Redis key)
    name: String,
    /// Redis client for connecting to Redis
    client: Arc<Client>,
}

impl DMSRedisQueue {
    /// Creates a new Redis queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue (Redis key)
    /// - `connection_string`: The Redis connection string
    ///
    /// # Returns
    ///
    /// A new DMSRedisQueue instance wrapped in DMSResult
    pub async fn _Fnew(name: &str, connection_string: &str) -> DMSResult<Self> {
        let client = Client::open(connection_string)?;
        
        Ok(Self {
            name: name.to_string(),
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl DMSQueue for DMSRedisQueue {
    /// Creates a new producer for the Redis queue.
    ///
    /// # Returns
    ///
    /// A new DMSQueueProducer instance wrapped in DMSResult
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        let conn = self.client.get_async_connection().await?;
        
        Ok(Box::new(RedisQueueProducer {
            connection: Arc::new(Mutex::new(conn)),
            queue_name: self.name.clone(),
        }))
    }

    /// Creates a new consumer for the Redis queue.
    ///
    /// # Parameters
    ///
    /// - `_consumer_group`: The consumer group name (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// A new DMSQueueConsumer instance wrapped in DMSResult
    async fn _Fcreate_consumer(&self, _consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        let conn = self.client.get_async_connection().await?;
        
        Ok(Box::new(RedisQueueConsumer {
            connection: Arc::new(Mutex::new(conn)),
            queue_name: self.name.clone(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    /// Gets statistics for the Redis queue.
    ///
    /// # Returns
    ///
    /// QueueStats containing queue statistics wrapped in DMSResult
    async fn _Fget_stats(&self) -> DMSResult<QueueStats> {
        let mut conn = self.client.get_async_connection().await?;
        let len: i64 = conn.llen(&self.name).await?;
        
        Ok(QueueStats {
            queue_name: self.name.clone(),
            message_count: len as u64,
            consumer_count: 0,
            producer_count: 0,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
        })
    }

    /// Purges all messages from the Redis queue.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fpurge(&self) -> DMSResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del::<_, ()>(&self.name).await?;
        Ok(())
    }

    /// Deletes the Redis queue.
    ///
    /// Note: This implementation simply calls purge since deleting a Redis key
    /// is the same as purging all messages from the queue.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fdelete(&self) -> DMSResult<()> {
        self._Fpurge().await
    }
}

/// Redis queue producer implementation.
///
/// This struct provides a Redis implementation of the DMSQueueProducer trait,
/// allowing sending messages to Redis queues.
struct RedisQueueProducer {
    /// Redis async connection
    connection: Arc<Mutex<redis::aio::Connection>>,
    /// Queue name (Redis key) to send messages to
    queue_name: String,
}

#[async_trait]
impl DMSQueueProducer for RedisQueueProducer {
    /// Sends a single message to the Redis queue.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let mut conn = self.connection.lock().await;
        let payload = serde_json::to_vec(&message)?;
        
        conn.rpush::<_, _, ()>(&self.queue_name, payload).await?;
        Ok(())
    }

    /// Sends multiple messages to the Redis queue.
    ///
    /// # Parameters
    ///
    /// - `messages`: A vector of messages to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        let mut conn = self.connection.lock().await;
        
        for message in messages {
            let payload = serde_json::to_vec(&message)?;
            conn.rpush::<_, _, ()>(&self.queue_name, payload).await?;
        }
        Ok(())
    }
}

/// Redis queue consumer implementation.
///
/// This struct provides a Redis implementation of the DMSQueueConsumer trait,
/// allowing receiving messages from Redis queues.
struct RedisQueueConsumer {
    /// Redis async connection
    connection: Arc<Mutex<redis::aio::Connection>>,
    /// Queue name (Redis key) to receive messages from
    queue_name: String,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for RedisQueueConsumer {
    /// Receives a message from the Redis queue.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    /// or the BLPOP operation timed out
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut conn = self.connection.lock().await;
        
        // Use BLPOP for blocking pop with timeout
        let result: Option<(String, Vec<u8>)> = conn.blpop(&self.queue_name, 5.0).await?;
        
        if let Some((_, payload)) = result {
            let message: DMSQueueMessage = serde_json::from_slice(&payload)?;
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    /// Acknowledges a message.
    ///
    /// Note: In Redis list-based queues, acknowledgment is implicit when messages
    /// are popped from the list. This method is a no-op in this implementation.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // In Redis list-based queue, acknowledgment is implicit when message is popped
        Ok(())
    }

    /// Negatively acknowledges a message.
    ///
    /// Note: This implementation is a placeholder. In a real implementation, you'd
    /// need to track the original message and push it back to the queue for retry.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to negatively acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // Put the message back in the queue for retry
        // For simplicity, we'll create a new message with incremented retry count
        // In a real implementation, you'd track the original message
        Ok(())
    }

    /// Pauses the consumer.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fpause(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resumes the consumer.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fresume(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}