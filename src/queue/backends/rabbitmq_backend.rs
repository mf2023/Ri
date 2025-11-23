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

use async_trait::async_trait;
use lapin::{Connection, ConnectionProperties, Channel, Queue, Consumer};
use lapin::options::{QueueDeclareOptions, BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use std::sync::Arc;
use tokio::sync::Mutex;
use futures::StreamExt;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

pub struct DMSRabbitMQQueue {
    name: String,
    #[allow(dead_code)]
    connection: Arc<Connection>,
    channel: Arc<Channel>,
    #[allow(dead_code)]
    queue: Arc<Queue>,
}

impl DMSRabbitMQQueue {
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
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        Ok(Box::new(RabbitMQProducer {
            channel: self.channel.clone(),
            queue_name: self.name.clone(),
        }))
    }

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

    async fn _Fpurge(&self) -> DMSResult<()> {
        self.channel.queue_purge(&self.name, Default::default()).await?;
        Ok(())
    }

    async fn _Fdelete(&self) -> DMSResult<()> {
        self.channel.queue_delete(&self.name, Default::default()).await?;
        Ok(())
    }
}

struct RabbitMQProducer {
    channel: Arc<Channel>,
    queue_name: String,
}

#[async_trait]
impl DMSQueueProducer for RabbitMQProducer {
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

    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        for message in messages {
            self._Fsend(message).await?;
        }
        Ok(())
    }
}

struct RabbitMQConsumer {
    consumer: Arc<Mutex<Consumer>>,
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for RabbitMQConsumer {
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let mut consumer = self.consumer.lock().await;
        
        if let Some(delivery_result) = consumer.next().await {
            let delivery = delivery_result.map_err(|e| crate::core::DMSError::Other(format!("Consumer error: {}", e)))?;
            let message: DMSQueueMessage = serde_json::from_slice(&delivery.data)?;
            
            // Store delivery tag for acknowledgment
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // In a real implementation, you'd track the delivery tag
        // For now, this is a placeholder
        Ok(())
    }

    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // In a real implementation, you'd track the delivery tag and use BasicNack
        Ok(())
    }

    async fn _Fpause(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    async fn _Fresume(&self) -> DMSResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}