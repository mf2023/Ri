//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
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
use rdkafka::admin::{AdminClient, AdminOptions, NewTopic, TopicReplication};
use rdkafka::client::DefaultClientContext;
use rdkafka::config::{ClientConfig, RDKafkaLogLevel};
use rdkafka::consumer::{Consumer, DefaultConsumerContext, StreamConsumer};
use rdkafka::message::{BorrowedHeaders, Headers, Message};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::topic_partition_list::TopicPartitionList;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use crate::core::{DMSCError, DMSCResult};
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats};

type KafkaConsumer = StreamConsumer<DefaultConsumerContext>;

#[allow(dead_code)]
#[derive(Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCKafkaQueue {
    brokers: String,
    topic: String,
    producer: Arc<FutureProducer>,
    consumer: Arc<KafkaConsumer>,
    admin_client: Arc<AdminClient<DefaultClientContext>>,
}

impl DMSCKafkaQueue {
    pub async fn new(brokers: &str, topic: &str) -> DMSCResult<Self> {
        let mut config = ClientConfig::new();
        config.set("bootstrap.servers", brokers);
        config.set("message.timeout.ms", "30000");
        config.set("request.timeout.ms", "10000");
        config.set("session.timeout.ms", "30000");
        config.set("enable.auto.commit", "false");
        config.set("auto.offset.reset", "earliest");
        config.set_log_level(RDKafkaLogLevel::Warning);

        let producer: FutureProducer = config
            .create::<FutureProducer>()
            .map_err(|e| DMSCError::Queue(format!("Failed to create Kafka producer: {}", e)))?;

        let consumer: KafkaConsumer = config
            .create::<KafkaConsumer>()
            .map_err(|e| DMSCError::Queue(format!("Failed to create Kafka consumer: {}", e)))?;

        let admin_client: AdminClient<DefaultClientContext> = config
            .create::<AdminClient<DefaultClientContext>>()
            .map_err(|e| DMSCError::Queue(format!("Failed to create Kafka admin client: {}", e)))?;

        let queue = Self {
            brokers: brokers.to_string(),
            topic: topic.to_string(),
            producer: Arc::new(producer),
            consumer: Arc::new(consumer),
            admin_client: Arc::new(admin_client),
        };

        queue.ensure_topic_exists().await?;

        Ok(queue)
    }

    async fn ensure_topic_exists(&self) -> DMSCResult<()> {
        let metadata = self.consumer.fetch_metadata(None, Duration::from_secs(5))
            .map_err(|e| DMSCError::Queue(format!("Failed to get Kafka metadata: {}", e)))?;

        let topic_exists = metadata.topics().iter().any(|t| t.name() == self.topic);

        if !topic_exists {
            let new_topic = NewTopic::new(&self.topic, 1, TopicReplication::Fixed(1));
            let admin_options = AdminOptions::new();

            self.admin_client.create_topics(std::slice::from_ref(&new_topic), &admin_options).await
                .map_err(|e| DMSCError::Queue(format!("Failed to create Kafka topic: {}", e)))?;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    async fn get_topic_metadata(&self) -> DMSCResult<i32> {
        let metadata = self.consumer.fetch_metadata(Some(&self.topic), Duration::from_secs(5))
            .map_err(|e| DMSCError::Queue(format!("Failed to get Kafka metadata: {}", e)))?;

        if let Some(topic_meta) = metadata.topics().first() {
            Ok(topic_meta.partitions().len() as i32)
        } else {
            Ok(0)
        }
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Ok(Box::new(KafkaQueueProducer {
            producer: self.producer.clone(),
            topic: self.topic.clone(),
        }))
    }

    async fn create_consumer(&self, consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        let consumer = self.consumer.clone();

        let mut partition_list = TopicPartitionList::new();
        let partition_count = self.get_topic_metadata().await?;

        for i in 0..partition_count.max(1) {
            partition_list.add_partition(&self.topic, i);
        }

        consumer.assign(&partition_list)
            .map_err(|e| DMSCError::Queue(format!("Failed to assign partitions: {}", e)))?;

        Ok(Box::new(KafkaQueueConsumer {
            consumer,
            topic: self.topic.clone(),
            consumer_group: consumer_group.to_string(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        let _partition_count = self.get_topic_metadata().await?;
        let topic = self.topic.clone();

        Ok(DMSCQueueStats {
            queue_name: topic.clone(),
            message_count: 0,
            consumer_count: 1,
            producer_count: 1,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
            total_bytes_sent: 0,
            total_bytes_received: 0,
            last_message_time: 0,
        })
    }

    async fn purge(&self) -> DMSCResult<()> {
        let admin_options = AdminOptions::new();
        self.admin_client.delete_topics(std::slice::from_ref(&self.topic.as_str()), &admin_options).await
            .map_err(|e| DMSCError::Queue(format!("Failed to purge Kafka topic: {}", e)))?;

        tokio::time::sleep(Duration::from_secs(1)).await;
        self.ensure_topic_exists().await?;

        Ok(())
    }

    async fn delete(&self) -> DMSCResult<()> {
        self.purge().await
    }
}

pub struct KafkaQueueProducer {
    producer: Arc<FutureProducer>,
    topic: String,
}

#[async_trait]
impl DMSCQueueProducer for KafkaQueueProducer {
    async fn send(&self, message: DMSCQueueMessage) -> DMSCResult<()> {
        let payload = if message.payload.is_empty() {
            vec![]
        } else {
            message.payload
        };

        let key = message.id.as_bytes();

        let future_record = FutureRecord::to(&self.topic)
            .key(key)
            .payload(&payload);

        self.producer.send(future_record, Duration::from_secs(10)).await
            .map_err(|(e, _)| DMSCError::Queue(format!("Failed to send message to Kafka: {}", e)))?;

        Ok(())
    }

    async fn send_batch(&self, messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        for message in messages {
            self.send(message).await?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub struct KafkaQueueConsumer {
    consumer: Arc<KafkaConsumer>,
    topic: String,
    consumer_group: String,
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSCQueueConsumer for KafkaQueueConsumer {
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        let paused = *self.paused.lock().await;
        if paused {
            return Ok(None);
        }

        let message = tokio::time::timeout(Duration::from_secs(5), self.consumer.recv()).await;

        match message {
            Ok(Ok(msg)) => {
                let payload = msg.payload().unwrap_or(&[]).to_vec();
                let key = msg.key().map(|k| String::from_utf8_lossy(k).to_string()).unwrap_or_default();
                let timestamp = msg.timestamp().to_millis().unwrap_or(0) as u64;

                let headers: HashMap<String, String> = msg.headers()
                    .map(|h: &BorrowedHeaders| {
                        h.iter().filter_map(|header| {
                            header.value.map(|v| (header.key.to_string(), String::from_utf8_lossy(v).to_string()))
                        }).collect()
                    })
                    .unwrap_or_default();

                let message = DMSCQueueMessage {
                    id: key,
                    payload,
                    headers,
                    timestamp: std::time::UNIX_EPOCH + Duration::from_millis(timestamp),
                    retry_count: 0,
                    max_retries: 3,
                };

                Ok(Some(message))
            }
            Ok(Err(e)) => Err(DMSCError::Queue(format!("Kafka receive error: {}", e))),
            Err(_) => Ok(None),
        }
    }

    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
        self.consumer.commit_consumer_state(rdkafka::consumer::CommitMode::Sync)
            .map_err(|e| DMSCError::Queue(format!("Failed to commit offset: {}", e)))?;
        Ok(())
    }

    async fn nack(&self, message_id: &str) -> DMSCResult<()> {
        log::info!("Message negatively acknowledged: {}", message_id);
        Ok(())
    }

    async fn pause(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = true;
        Ok(())
    }

    async fn resume(&self) -> DMSCResult<()> {
        let mut paused = self.paused.lock().await;
        *paused = false;
        Ok(())
    }
}
