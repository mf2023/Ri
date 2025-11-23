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
use redis::{AsyncCommands, Client};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

pub struct DMSRedisQueue {
    name: String,
    client: Arc<Client>,
}

impl DMSRedisQueue {
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
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        let conn = self.client.get_async_connection().await?;
        
        Ok(Box::new(RedisQueueProducer {
            connection: Arc::new(Mutex::new(conn)),
            queue_name: self.name.clone(),
        }))
    }

    async fn _Fcreate_consumer(&self, _consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        let conn = self.client.get_async_connection().await?;
        
        Ok(Box::new(RedisQueueConsumer {
            connection: Arc::new(Mutex::new(conn)),
            queue_name: self.name.clone(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

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

    async fn _Fpurge(&self) -> DMSResult<()> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del::<_, ()>(&self.name).await?;
        Ok(())
    }

    async fn _Fdelete(&self) -> DMSResult<()> {
        self._Fpurge().await
    }
}

struct RedisQueueProducer {
    connection: Arc<Mutex<redis::aio::Connection>>,
    queue_name: String,
}

#[async_trait]
impl DMSQueueProducer for RedisQueueProducer {
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let mut conn = self.connection.lock().await;
        let payload = serde_json::to_vec(&message)?;
        
        conn.rpush::<_, _, ()>(&self.queue_name, payload).await?;
        Ok(())
    }

    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        let mut conn = self.connection.lock().await;
        
        for message in messages {
            let payload = serde_json::to_vec(&message)?;
            conn.rpush::<_, _, ()>(&self.queue_name, payload).await?;
        }
        Ok(())
    }
}

struct RedisQueueConsumer {
    connection: Arc<Mutex<redis::aio::Connection>>,
    queue_name: String,
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for RedisQueueConsumer {
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

    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // In Redis list-based queue, acknowledgment is implicit when message is popped
        Ok(())
    }

    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // Put the message back in the queue for retry
        // For simplicity, we'll create a new message with incremented retry count
        // In a real implementation, you'd track the original message
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