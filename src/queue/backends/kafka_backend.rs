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
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use futures::StreamExt;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

pub struct DMSKafkaQueue {
    name: String,
    producer: Arc<FutureProducer>,
    consumer: Arc<StreamConsumer>,
}

impl DMSKafkaQueue {
    pub async fn _Fnew(name: &str, connection_string: &str) -> DMSResult<Self> {
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
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        Ok(Box::new(KafkaProducer {
            producer: self.producer.clone(),
            topic: self.name.clone(),
        }))
    }

    async fn _Fcreate_consumer(&self, _consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        Ok(Box::new(KafkaConsumer {
            consumer: self.consumer.clone(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    async fn _Fget_stats(&self) -> DMSResult<QueueStats> {
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

    async fn _Fpurge(&self) -> DMSResult<()> {
        // Kafka doesn't support purging topics directly
        // This would require admin operations
        Ok(())
    }

    async fn _Fdelete(&self) -> DMSResult<()> {
        // Kafka doesn't support deleting topics through client API
        // This would require admin operations
        Ok(())
    }
}

struct KafkaProducer {
    producer: Arc<FutureProducer>,
    topic: String,
}

#[async_trait]
impl DMSQueueProducer for KafkaProducer {
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let payload = serde_json::to_vec(&message)?;
        
        let record = FutureRecord::to(&self.topic)
            .payload(&payload)
            .key(&message.id);

        self.producer.send(record, std::time::Duration::from_secs(0)).await?;
        Ok(())
    }

    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        for message in messages {
            self._Fsend(message).await?;
        }
        Ok(())
    }
}

struct KafkaConsumer {
    consumer: Arc<StreamConsumer>,
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for KafkaConsumer {
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>> {
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

    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // Kafka auto-commit is enabled, so acknowledgment is automatic
        Ok(())
    }

    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // In Kafka, negative acknowledgment typically means seeking back
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