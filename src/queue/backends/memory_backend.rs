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
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::sync::{RwLock, Mutex};
use crate::core::DMSResult;
use crate::queue::{DMSQueue, DMSQueueMessage, DMSQueueProducer, DMSQueueConsumer, QueueStats};

struct MemoryQueueState {
    messages: VecDeque<DMSQueueMessage>,
    consumers: HashMap<String, VecDeque<DMSQueueMessage>>,
}

impl MemoryQueueState {
    fn _Fnew() -> Self {
        Self {
            messages: VecDeque::new(),
            consumers: HashMap::new(),
        }
    }
}

pub struct DMSMemoryQueue {
    name: String,
    state: Arc<RwLock<MemoryQueueState>>,
}

impl DMSMemoryQueue {
    pub fn _Fnew(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Arc::new(RwLock::new(MemoryQueueState::_Fnew())),
        }
    }
}

#[async_trait]
impl DMSQueue for DMSMemoryQueue {
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>> {
        Ok(Box::new(MemoryQueueProducer {
            state: self.state.clone(),
        }))
    }

    async fn _Fcreate_consumer(&self, consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>> {
        Ok(Box::new(MemoryQueueConsumer {
            state: self.state.clone(),
            consumer_group: consumer_group.to_string(),
            paused: Arc::new(Mutex::new(false)),
        }))
    }

    async fn _Fget_stats(&self) -> DMSResult<QueueStats> {
        let state = self.state.read().await;
        Ok(QueueStats {
            queue_name: self.name.clone(),
            message_count: state.messages.len() as u64,
            consumer_count: state.consumers.len() as u32,
            producer_count: 1,
            processed_messages: 0,
            failed_messages: 0,
            avg_processing_time_ms: 0.0,
        })
    }

    async fn _Fpurge(&self) -> DMSResult<()> {
        let mut state = self.state.write().await;
        state.messages.clear();
        state.consumers.clear();
        Ok(())
    }

    async fn _Fdelete(&self) -> DMSResult<()> {
        self._Fpurge().await
    }
}

struct MemoryQueueProducer {
    state: Arc<RwLock<MemoryQueueState>>,
}

#[async_trait]
impl DMSQueueProducer for MemoryQueueProducer {
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()> {
        let mut state = self.state.write().await;
        state.messages.push_back(message);
        Ok(())
    }

    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()> {
        let mut state = self.state.write().await;
        for message in messages {
            state.messages.push_back(message);
        }
        Ok(())
    }
}

struct MemoryQueueConsumer {
    state: Arc<RwLock<MemoryQueueState>>,
    consumer_group: String,
    paused: Arc<Mutex<bool>>,
}

#[async_trait]
impl DMSQueueConsumer for MemoryQueueConsumer {
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>> {
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
            state.consumers.insert(self.consumer_group.clone(), consumer_queue);
            Ok(Some(message))
        } else {
            Ok(None)
        }
    }

    async fn _Fack(&self, _message_id: &str) -> DMSResult<()> {
        // In memory queue, acknowledgment is implicit when message is received
        Ok(())
    }

    async fn _Fnack(&self, _message_id: &str) -> DMSResult<()> {
        // Put the message back in the queue for retry
        // For simplicity, we'll just create a new message with incremented retry count
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