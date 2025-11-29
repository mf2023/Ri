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

//! # RabbitMQ Queue Backend
//! 
//! This module provides a RabbitMQ implementation for the DMS queue system. It allows
//! sending and receiving messages using RabbitMQ as the underlying message broker.
//! 
//! ## Key Components
//! 
//! - **DMSRabbitMQQueue**: Main RabbitMQ queue implementation
//! - **RabbitMQProducer**: RabbitMQ producer implementation
//! - **RabbitMQConsumer**: RabbitMQ consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the DMSQueue, DMSQueueProducer, and DMSQueueConsumer traits
//! 2. **RabbitMQ Integration**: Uses the lapin crate for RabbitMQ connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of connections, channels, and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Durable Queues**: Configured with durable queues for message persistence
//! 6. **Error Handling**: Comprehensive error handling with DMSResult
//! 7. **Stream-based Consumer**: Uses StreamExt for efficient message processing
//! 8. **Batch Support**: Provides batch sending functionality
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a new RabbitMQ queue
//!     let queue = DMSRabbitMQQueue::_Fnew("test-queue", "amqp://guest:guest@localhost:5672/%2f").await?;
//!     
//!     // Create a producer
//!     let producer = queue._Fcreate_producer().await?;
//!     
//!     // Create a message
//!     let message = DMSQueueMessage {
//!         id: "12345".to_string(),
//!         payload: b"Hello, RabbitMQ!".to_vec(),
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
use lapin::{Connection, ConnectionProperties, Channel, Queue, Consumer};
use lapin::options::{QueueDeclareOptions, BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

/// RabbitMQ queue implementation for the DMS queue system.
///
/// This struct provides a RabbitMQ implementation of the DMSQueue trait, allowing
/// sending and receiving messages using RabbitMQ as the underlying message broker.
pub struct DMSRabbitMQQueue {
    /// Queue name
    name: String,
    /// RabbitMQ connection
    #[allow(dead_code)]
    connection: Arc<Connection>,
    /// RabbitMQ channel
    channel: Arc<Channel>,
    /// RabbitMQ queue
    #[allow(dead_code)]
    queue: Arc<Queue>,
}

impl DMSRabbitMQQueue {
    /// Creates a new RabbitMQ queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    /// - `connection_string`: The RabbitMQ connection string
    ///
    /// # Returns
    ///
    /// A new DMSRabbitMQQueue instance wrapped in DMSResult
    pub async fn _Fnew(name: &str, connection_string: &str) -> DMSResult<Self> {
        let connection = Connection::connect(connection_string, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        
        let queue = channel
            .queue_declare(
                name,
                QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                FieldTable::default(),
            )
            .await?;

        Ok(Self {
            name: name.to_string(),
            connection: Arc::new(connection),
            channel: Arc::new(channel),
            queue: Arc::new(queue),
        })
    }
}

#[async_trait]
impl DMSQueue for DMSRabbitMQQueue {
    /// Creates a new producer for the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// A new DMSQueueProducer instance wrapped in DMSResult
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        Ok(Box::new(RabbitMQProducer {
            channel: self.channel.clone(),
            queue_name: self.name.clone(),
        }))
    }

    /// Creates a new consumer for the RabbitMQ queue.
    ///
    /// # Parameters
    ///
    /// - `consumer_group`: The consumer group name
    ///
    /// # Returns
    ///
    /// A new DMSQueueConsumer instance wrapped in DMSResult
    async fn _Fcreate_consumer(&self, consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        let consumer = self.channel
            .basic_consume(
                &self.name,
                consumer_group,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Box::new(RabbitMQConsumer {
            consumer: Arc::new(Mutex::new(consumer)),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    /// Gets statistics for the RabbitMQ queue.
    ///
    /// Note: This implementation returns basic stats since RabbitMQ provides detailed metrics
    /// through its management API, which is not implemented here.
    ///
    /// # Returns
    ///
    /// QueueStats containing basic queue statistics wrapped in DMSResult
    async fn _Fget_stats(&self) -> DMSResult<QueueStats> {
        // RabbitMQ provides queue statistics through management API
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

    /// Purges all messages from the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fpurge(&self) -> DMSResult<()> {
        self.channel.queue_purge(&self.name, Default::default()).await?;
        Ok(())
    }

    /// Deletes the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fdelete(&self) -> DMSResult<()> {
        self.channel.queue_delete(&self.name, Default::default()).await?;
        Ok(())
    }
}

/// RabbitMQ producer implementation.
///
/// This struct provides a RabbitMQ implementation of the DMSQueueProducer trait,
/// allowing sending messages to RabbitMQ queues.
struct RabbitMQProducer {
    /// RabbitMQ channel
    channel: Arc<Channel>,
    /// Queue name to send messages to
    queue_name: String,
}

#[async_trait]
impl DMSQueueProducer for RabbitMQProducer {
    /// Sends a single message to the RabbitMQ queue.
    ///
    /// # Parameters
    ///
    /// - `message`: The message to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let payload = serde_json::to_vec(&message)?;
        
        self.channel
            .basic_publish(
                "",
                &self.queue_name,
                BasicPublishOptions::default(),
                &payload,
                Default::default(),
            )
            .await?;
        
        Ok(())
    }

    /// Sends multiple messages to the RabbitMQ queue.
    ///
    /// # Parameters
    ///
    /// - `messages`: A vector of messages to send
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        for message in messages {
            self._Fsend(message).await?;
        }
        Ok(())
    }
}

/// RabbitMQ consumer implementation.
///
/// This struct provides a RabbitMQ implementation of the DMSQueueConsumer trait,
/// allowing receiving messages from RabbitMQ queues.
struct RabbitMQConsumer {
    /// RabbitMQ consumer
    consumer: Arc<Mutex<Consumer>>,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for RabbitMQConsumer {
    /// Receives a message from the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut consumer = self.consumer.lock().await;
        
        if let Some(delivery_result) = consumer.next().await {
            let delivery = delivery_result.map_err(|e| crate::core::DMSError::Other(format!("Consumer error: {e}")))?;
            let message: DMSQueueMessage = serde_json::from_slice(&delivery.data)?;
            
            // Store delivery tag for acknowledgment
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    /// Acknowledges a message.
    ///
    /// Note: This implementation is a placeholder. In a real implementation, you'd track
    /// the delivery tag and use basic_ack to acknowledge the message.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // In a real implementation, you'd track the delivery tag
        // For now, this is a placeholder
        Ok(())
    }

    /// Negatively acknowledges a message.
    ///
    /// Note: This implementation is a placeholder. In a real implementation, you'd track
    /// the delivery tag and use basic_nack to negatively acknowledge the message.
    ///
    /// # Parameters
    ///
    /// - `_message_id`: The message ID to negatively acknowledge (ignored in this implementation)
    ///
    /// # Returns
    ///
    /// DMSResult indicating success or failure
    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // In a real implementation, you'd track the delivery tag and use BasicNack
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