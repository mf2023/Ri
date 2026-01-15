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
#![cfg(feature = "rabbitmq")]

//! # RabbitMQ Queue Backend
//! 
//! This module provides a RabbitMQ implementation for the DMSC queue system. It allows
//! sending and receiving messages using RabbitMQ as the underlying message broker.
//! 
//! ## Key Components
//! 
//! - **DMSCRabbitMQQueue**: Main RabbitMQ queue implementation
//! - **RabbitMQProducer**: RabbitMQ producer implementation
//! - **RabbitMQConsumer**: RabbitMQ consumer implementation
//! 
//! ## Design Principles
//! 
//! 1. **Async Trait Implementation**: Implements the DMSCQueue, DMSCQueueProducer, and DMSCQueueConsumer traits
//! 2. **RabbitMQ Integration**: Uses the lapin crate for RabbitMQ connectivity
//! 3. **Thread Safety**: Uses Arc for safe sharing of connections, channels, and consumers
//! 4. **Future-based API**: Leverages async/await for non-blocking operations
//! 5. **Durable Queues**: Configured with durable queues for message persistence
//! 6. **Error Handling**: Comprehensive error handling with DMSCResult
//! 7. **Stream-based Consumer**: Uses StreamExt for efficient message processing
//! 8. **Batch Support**: Provides batch sending functionality
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! 
//! async fn example() -> DMSCResult<()> {
//!     // Create a new RabbitMQ queue
//!     let queue = DMSCRabbitMQQueue::new("test-queue", "amqp://guest:guest@localhost:5672/%2f").await?;
//!     
//!     // Create a producer
//!     let producer = queue.create_producer().await?;
//!     
//!     // Create a message
//!     let message = DMSCQueueMessage {
//!         id: "12345".to_string(),
//!         payload: b"Hello, RabbitMQ!".to_vec(),
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
use lapin::{Connection, ConnectionProperties, Channel, Queue, Consumer};
use lapin::options::{QueueDeclareOptions, BasicConsumeOptions, BasicPublishOptions, BasicAckOptions, BasicNackOptions};
use lapin::types::FieldTable;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
#[cfg(feature = "http_client")]
use reqwest;
#[cfg(feature = "http_client")]
use urlencoding;
use crate::core::DMSCResult;
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats};

/// RabbitMQ queue implementation for the DMSC queue system.
///
/// This struct provides a RabbitMQ implementation of the DMSCQueue trait, allowing
/// sending and receiving messages using RabbitMQ as the underlying message broker.
pub struct DMSCRabbitMQQueue {
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
    /// RabbitMQ management API URL
    #[allow(dead_code)]
    management_url: Option<String>,
    /// Management API username
    #[allow(dead_code)]
    management_username: Option<String>,
    /// RabbitMQ management API password
    #[allow(dead_code)]
    management_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RabbitMQQueueInfo {
    name: String,
    messages: u64,
    consumers: u64,
    message_stats: Option<RabbitMQMessageStats>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RabbitMQMessageStats {
    publish: Option<u64>,
    deliver_no_ack: Option<u64>,
    get_no_ack: Option<u64>,
    redeliver: Option<u64>,
    deliver: Option<u64>,
    get: Option<u64>,
}

impl DMSCRabbitMQQueue {
    /// Creates a new RabbitMQ queue instance.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    /// - `connection_string`: The RabbitMQ connection string
    ///
    /// # Returns
    ///
    /// A new DMSCRabbitMQQueue instance wrapped in DMSCResult
    pub async fn new(name: &str, connection_string: &str) -> DMSCResult<Self> {
        let connection = Connection::connect(connection_string, ConnectionProperties::default()).await?;
        Self::new_with_connection(name, connection).await
    }

    /// Creates a new RabbitMQ queue instance with an existing connection.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    /// - `connection`: The existing RabbitMQ connection
    ///
    /// # Returns
    ///
    /// A new DMSCRabbitMQQueue instance wrapped in DMSCResult
    pub async fn new_with_connection(name: &str, connection: lapin::Connection) -> DMSCResult<Self> {
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
            management_url: None,
            management_username: None,
            management_password: None,
        })
    }

    /// Creates a new RabbitMQ queue instance with management API support.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the queue
    /// - `connection`: The existing RabbitMQ connection
    /// - `management_url`: RabbitMQ management API URL (e.g., "http://localhost:15672")
    /// - `management_username`: Management API username
    /// - `management_password`: Management API password
    ///
    /// # Returns
    ///
    /// A new DMSCRabbitMQQueue instance wrapped in DMSCResult
    pub async fn new_with_management(
        name: &str,
        connection: lapin::Connection,
        management_url: &str,
        management_username: &str,
        management_password: &str,
    ) -> DMSCResult<Self> {
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
            management_url: Some(management_url.to_string()),
            management_username: Some(management_username.to_string()),
            management_password: Some(management_password.to_string()),
        })
    }

    /// Fetches detailed statistics from RabbitMQ management API.
    ///
    /// This method attempts to connect to RabbitMQ's management API to retrieve
    /// comprehensive queue statistics including message counts, consumer counts,
    /// and message processing rates.
    ///
    /// # Returns
    ///
    /// Detailed DMSCQueueStats wrapped in DMSCResult
    #[cfg(feature = "http_client")]
    async fn fetch_rabbitmq_stats(&self) -> DMSCResult<DMSCQueueStats> {
        let (Some(management_url), Some(username), Some(password)) = (
            &self.management_url,
            &self.management_username,
            &self.management_password,
        ) else {
            return Err(crate::core::DMSCError::Other(
                "Management API not configured. Use new_with_management() to enable.".to_string(),
            ));
        };

        let client = reqwest::Client::new();
        let url = format!(
            "{}/api/queues/%2f/{}",
            management_url.trim_end_matches('/'),
            urlencoding::encode(&self.name)
        );

        let response = client
            .get(&url)
            .basic_auth(username, Some(password))
            .send()
            .await
            .map_err(|e| crate::core::DMSCError::Other(format!("Failed to connect to RabbitMQ Management API: {}", e)))?;

        if !response.status().is_success() {
            return Err(crate::core::DMSCError::Other(format!(
                "RabbitMQ Management API returned error: {}",
                response.status()
            )));
        }

        let queue_info: RabbitMQQueueInfo = response
            .json()
            .await
            .map_err(|e| crate::core::DMSCError::Other(format!("Failed to parse RabbitMQ response: {}", e)))?;

        let processed_messages = queue_info
            .message_stats
            .as_ref()
            .and_then(|s| s.publish.or(s.deliver).or(s.get))
            .unwrap_or(0);

        let failed_messages = queue_info
            .message_stats
            .as_ref()
            .and_then(|s| s.redeliver)
            .unwrap_or(0);

        Ok(DMSCQueueStats {
            queue_name: queue_info.name,
            message_count: queue_info.messages,
            consumer_count: queue_info.consumers as u32,
            producer_count: 0,
            processed_messages,
            failed_messages,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        })
    }
    
    async fn get_basic_stats(&self) -> DMSCResult<DMSCQueueStats> {
        let queue_info = self.channel
            .queue_declare(
                &self.name,
                lapin::options::QueueDeclareOptions {
                    passive: true,
                    ..Default::default()
                },
                lapin::types::FieldTable::default(),
            )
            .await?;
        
        Ok(DMSCQueueStats {
            queue_name: self.name.clone(),
            message_count: queue_info.message_count() as u64,
            consumer_count: queue_info.consumer_count() as u32,
            producer_count: 0,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        })
    }

}

#[async_trait]
impl DMSCQueue for DMSCRabbitMQQueue {
    /// Creates a new producer for the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// A new DMSCQueueProducer instance wrapped in DMSCResult
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
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
    /// A new DMSCQueueConsumer instance wrapped in DMSCResult
    async fn create_consumer(&self, consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        let consumer = self.channel
            .basic_consume(
                &self.name,
                consumer_group,
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(Box::new(RabbitMQConsumer::new(self.channel.clone(), consumer)))
    }

    /// Gets statistics for the RabbitMQ queue.
    ///
    /// This implementation integrates with RabbitMQ management API to provide detailed
    /// queue statistics including message counts, consumer counts, and processing metrics.
    ///
    /// # Returns
    ///
    /// DMSCQueueStats containing detailed queue statistics wrapped in DMSCResult
    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        #[cfg(feature = "http_client")]
        match self.fetch_rabbitmq_stats().await {
            Ok(detailed_stats) => Ok(detailed_stats),
            Err(_) => {
                self.get_basic_stats().await
            }
        }
        #[cfg(not(feature = "http_client"))]
        self.get_basic_stats().await
    }

    /// Purges all messages from the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure

    /// Purges all messages from the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn purge(&self) -> DMSCResult<()> {
        self.channel.queue_purge(&self.name, Default::default()).await?;
        Ok(())
    }

    /// Deletes the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn delete(&self) -> DMSCResult<()> {
        self.channel.queue_delete(&self.name, Default::default()).await?;
        Ok(())
    }
}

/// RabbitMQ producer implementation.
///
/// This struct provides a RabbitMQ implementation of the DMSCQueueProducer trait,
/// allowing sending messages to RabbitMQ queues.
struct RabbitMQProducer {
    /// RabbitMQ channel
    channel: Arc<Channel>,
    /// Queue name to send messages to
    queue_name: String,
}

#[async_trait]
impl DMSCQueueProducer for RabbitMQProducer {
    /// Sends a single message to the RabbitMQ queue.
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
    /// DMSCResult indicating success or failure
    async fn send_batch(&self, messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        for message in messages {
            self.send(message).await?;
        }
        Ok(())
    }
}

/// RabbitMQ consumer implementation.
///
/// This struct provides a RabbitMQ implementation of the DMSCQueueConsumer trait,
/// allowing receiving messages from RabbitMQ queues.
struct RabbitMQConsumer {
    /// RabbitMQ channel for sending acks/nacks
    channel: Arc<Channel>,
    /// RabbitMQ consumer
    consumer: Arc<Mutex<Consumer>>,
    /// Flag indicating if the consumer is paused
    paused: Arc<Mutex<bool>>,
    /// Message tracking: delivery_tag -> message_id
    delivery_tags: Arc<Mutex<HashMap<u64, String>>>,
    /// Next delivery tag counter
    next_delivery_tag: Arc<AtomicU64>,
}

impl RabbitMQConsumer {
    fn new(channel: Arc<Channel>, consumer: Consumer) -> Self {
        Self {
            channel,
            consumer: Arc::new(Mutex::new(consumer)),
            paused: Arc::new(Mutex::new(false)),
            delivery_tags: Arc::new(Mutex::new(HashMap::new())),
            next_delivery_tag: Arc::new(AtomicU64::new(1)),
        }
    }
}

#[async_trait]
impl DMSCQueueConsumer for RabbitMQConsumer {
    /// Receives a message from the RabbitMQ queue.
    ///
    /// # Returns
    ///
    /// An Option containing the received message, or None if the consumer is paused
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut consumer = self.consumer.lock().await;
        
        match consumer.next().await {
            Some(delivery_result) => {
                let delivery = delivery_result.map_err(|e| crate::core::DMSCError::Other(format!("Consumer error: {e}")))?;
                
                let _message_id = {
                    let delivery_tag = delivery.delivery_tag;
                    let message_id = format!("msg_{}", uuid::Uuid::new_v4());
                    
                    let mut tags = self.delivery_tags.lock().await;
                    tags.insert(delivery_tag, message_id.clone());
                    
                    message_id
                };
                
                let message: DMSCQueueMessage = serde_json::from_slice(&delivery.data)?;
                
                Ok(Some(message))
            },
            None => Ok(None)
        }
    }

    /// Acknowledges a message.
    ///
    /// This implementation tracks delivery tags and uses basic_ack to acknowledge messages.
    ///
    /// # Parameters
    ///
    /// - `message_id`: The message ID to acknowledge
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn ack(&self, message_id: &str) -> DMSCResult<()> {
        log::debug!("Acknowledging message: {}", message_id);
        
        let delivery_tag = {
            let tags = self.delivery_tags.lock().await;
            tags.iter()
                .find(|(_, id)| *id == message_id)
                .map(|(tag, _)| *tag)
        };

        if let Some(tag) = delivery_tag {
            let channel = self.channel.clone();
            channel.basic_ack(tag, BasicAckOptions { multiple: false }).await
                .map_err(|e| crate::core::DMSCError::Other(format!("Failed to ack message: {e}")))?;
            
            let mut tags = self.delivery_tags.lock().await;
            tags.remove(&tag);
            
            log::debug!("Message {} acknowledged successfully", message_id);
        } else {
            log::warn!("Message ID not found for acknowledgment: {}", message_id);
        }
        
        Ok(())
    }

    /// Negatively acknowledges a message.
    ///
    /// This implementation tracks delivery tags and uses basic_nack to negatively acknowledge messages.
    ///
    /// # Parameters
    ///
    /// - `message_id`: The message ID to negatively acknowledge
    ///
    /// # Returns
    ///
    /// DMSCResult indicating success or failure
    async fn nack(&self, message_id: &str) -> DMSCResult<()> {
        log::debug!("Negatively acknowledging message: {}", message_id);
        
        let delivery_tag = {
            let tags = self.delivery_tags.lock().await;
            tags.iter()
                .find(|(_, id)| *id == message_id)
                .map(|(tag, _)| *tag)
        };

        if let Some(tag) = delivery_tag {
            let channel = self.channel.clone();
            channel.basic_nack(tag, false, true).await
                .map_err(|e| crate::core::DMSCError::Other(format!("Failed to nack message: {e}")))?;
            
            let mut tags = self.delivery_tags.lock().await;
            tags.remove(&tag);
            
            log::debug!("Message {} negatively acknowledged (will be requeued)", message_id);
        } else {
            log::warn!("Message ID not found for negative acknowledgment: {}", message_id);
        }
        
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
