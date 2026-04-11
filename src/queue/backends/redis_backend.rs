//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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
//! This module provides a Redis implementation for the Ri queue system. It allows
//! sending and receiving messages using Redis lists as the underlying message broker.
//! 
//! ## Key Components
//! 
//! - **RiRedisQueue**: Main Redis queue implementation
//! - **RedisQueueProducer**: Redis producer implementation
//! - **RedisQueueConsumer**: Redis consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the RiQueue, RiQueueProducer, and RiQueueConsumer traits
//! 2. **Redis Integration**: Uses the redis crate for Redis connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of connections and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Blocking Operations**: Uses BLPOP for efficient blocking message consumption
//! 6. **Error Handling**: Comprehensive error handling with RiResult
//! 7. **List-based Queue**: Uses Redis lists for simple FIFO queue functionality
//! 8. **Batch Support**: Provides batch sending functionality
//! 9. **Implicit Acknowledgment**: Acknowledgment is implicit when messages are popped from the list
//! 10. **Stats Support**: Provides queue length statistics using Redis LLEN command
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a new Redis queue
//!     let queue = RiRedisQueue::new("test-queue", "redis://localhost:6379").await?;
//!     
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let message = RiQueueMessage {
//!         id: "12345".to_string(),
//!         payload: b"Hello, Redis!".to_vec(),
//!         headers: vec![("key1".to_string(), "value1".to_string())],
//!         timestamp: chrono::Utc::now().timestamp_millis() as u64,
//!         priority: 0,
//!     };
//!     
//!     // Send the message
//!     producer.send(message).await?;
//!     
//!     // Create a consumer
//!     let consumer = queue.create_consumer("test-consumer-group").await?;
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
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::core::RiResult;
use crate::queue::{RiQueue, RiQueueMessage, RiQueueProducer, RiQueueConsumer, RiQueueStats};

/// Redis queue implementation for the Ri queue system.
///
/// This struct provides a Redis implementation of the RiQueue trait, allowing
/// sending and receiving messages using Redis lists as the underlying message broker.
pub struct RiRedisQueue {
    /// Queue name (Redis key)
    name: String,
    /// Redis client for connecting to Redis
    client: Arc<Client>,
}

impl RiRedisQueue {
    /// Creates a new Redis queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue (Redis key)
    /// - `connection_string`: The Redis connection string
    ///
    /// # Returns
    ///
    /// A new RiRedisQueue instance wrapped in RiResult
    pub async fn new(name: &str, connection_string: &str) -> RiResult<Self> {
        let client = Client::open(connection_string)?;
        Self::new_with_client(name, client)
    }

    /// Creates a new Redis queue instance with an existing client.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue (Redis key)
    /// - `client`: The existing Redis client
    ///
    /// # Returns
    ///
    /// A new RiRedisQueue instance wrapped in RiResult
    pub fn new_with_client(name: &str, client: Client) -> RiResult<Self> {
        Ok(Self {
            name: name.to_string(),
            client: Arc::new(client),
        })
    }

    /// Creates a new Redis queue instance with an existing connection.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue (Redis key)
    /// - `connection_string`: The Redis connection string
    ///
    /// # Returns
    ///
    /// A new RiRedisQueue instance wrapped in RiResult
    pub async fn new_with_connection(name: &str, connection_string: &str) -> RiResult<Self> {
        let client = Client::open(connection_string)?;
        Ok(Self {
            name: name.to_string(),
            client: Arc::new(client),
        })
    }
}

#[async_trait]
impl RiQueue for RiRedisQueue {
    /// Creates a new producer for the Redis queue.
    ///
    /// # Returns
    ///
    /// A new RiQueueProducer instance wrapped in RiResult
    async fn create_producer(&self) -> RiResult<Box<dyn RiQueueProducer>> {
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
    /// A new RiQueueConsumer instance wrapped in RiResult
    async fn create_consumer(&self, _consumer_group: &str) -> RiResult<Box<dyn RiQueueConsumer>> {
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
    /// RiQueueStats containing queue statistics wrapped in RiResult
    async fn get_stats(&self) -> RiResult<RiQueueStats> {
        let mut conn = self.client.get_async_connection().await?;
        let len: i64 = conn.llen(&self.name).await?;
        
        Ok(RiQueueStats {
            queue_name: self.name.clone(),
            message_count: len as u64,
            consumer_count: 0,
            producer_count: 0,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        })
    }

    /// Purges all messages from the Redis queue.
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    async fn purge(&self) -> RiResult<()> {
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
    /// RiResult indicating success or failure
    async fn delete(&self) -> RiResult<()> {
        self.purge().await
    }
}

/// Redis queue producer implementation.
///
/// This struct provides a Redis implementation of the RiQueueProducer trait,
/// allowing sending messages to Redis queues.
struct RedisQueueProducer {
    /// Redis async connection
    connection: Arc<Mutex<redis::aio::Connection>>,
    /// Queue name (Redis key) to send messages to
    queue_name: String,
}

#[async_trait]
impl RiQueueProducer for RedisQueueProducer {
    /// Sends a single message to the Redis queue.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    async fn send(&self, message: RiQueueMessage) -> RiResult<()> {
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
    /// RiResult indicating success or failure
    async fn send_batch(&self, messages: Vec<RiQueueMessage>) -> RiResult<()> {
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
/// This struct provides a Redis implementation of the RiQueueConsumer trait,
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
impl RiQueueConsumer for RedisQueueConsumer {
    /// Receives a message from the Redis queue.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    /// or the BLPOP operation timed out
    async fn receive(&self) -> RiResult<Option<RiQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut conn = self.connection.lock().await;
        
        // Use BLPOP for blocking pop with timeout
        let result: Option<(String, Vec<u8>)> = conn.blpop(&self.queue_name, 5.0).await?;
        
        if let Some((_, payload)) = result {
            let message: RiQueueMessage = serde_json::from_slice(&payload)?;
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
    /// RiResult indicating success or failure
    async fn ack(&self, _message_id: &str) -> RiResult<()> {
        // In Redis list-based queue, acknowledgment is implicit when message is popped
        Ok(())
    }

    /// Negatively acknowledges a message.
    ///
    /// This implementation handles message retry by pushing the message back to the queue
    /// with appropriate retry logic and delay mechanisms. It tracks retry counts and
    /// implements exponential backoff for retry delays.
    ///
    /// # Parameters
    ///
    /// - `message_id`: The message ID to negatively acknowledge
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    async fn nack(&self, message_id: &str) -> RiResult<()> {
        log::info!("Message negatively acknowledged: {message_id}");

        let mut conn = self.connection.lock().await;

        let (original_data, retry_count): (Option<Vec<u8>>, u32) = conn.hgetall::<_, (Option<Vec<u8>>, u32)>(&format!("{}_meta", self.queue_name)).await
            .map(|(data, count)| (data, count))
            .unwrap_or((None, 0));

        let max_retries = 3u32;
        let base_delay_ms = 1000u64;

        if retry_count >= max_retries {
            log::warn!("Message {message_id} exceeded max retries ({max_retries}), moving to dead letter queue");
            conn.rpush::<_, _, ()>(&format!("{}_dlq", self.queue_name), message_id.as_bytes()).await?;
            conn.hdel::<_, &str, ()>(&format!("{}_meta", self.queue_name), "retry_count").await?;
            return Ok(());
        }

        let new_retry_count = retry_count + 1;
        let retry_delay_ms = base_delay_ms * (2u64.pow(new_retry_count - 1));

        log::info!("Message {message_id} scheduled for retry {new_retry_count}/{max_retries} after {retry_delay_ms}ms delay");

        conn.hset::<_, &str, u32, ()>(&format!("{}_meta", self.queue_name), "retry_count", new_retry_count).await?;

        tokio::time::sleep(Duration::from_millis(retry_delay_ms)).await;

        if let Some(data) = original_data {
            conn.rpush::<_, _, ()>(&self.queue_name, &data).await?;
        }

        Ok(())
    }

    /// Pauses the consumer.
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    async fn pause(&self) -> RiResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    /// Resumes the consumer.
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    async fn resume(&self) -> RiResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}
