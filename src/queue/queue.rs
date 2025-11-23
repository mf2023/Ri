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
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::SystemTime;
use crate::core::DMSResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSQueueMessage {
    pub id: String,
    pub payload: Vec<u8>,
    pub headers: HashMap<String, String>,
    pub timestamp: SystemTime,
    pub retry_count: u32,
    pub max_retries: u32,
}

impl DMSQueueMessage {
    pub fn _Fnew(payload: Vec<u8>) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            payload,
            headers: HashMap::new(),
            timestamp: SystemTime::now(),
            retry_count: 0,
            max_retries: 3,
        }
    }

    pub fn _Fwith_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }

    pub fn _Fwith_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    pub fn _Fincrement_retry(&mut self) {
        self.retry_count += 1;
    }

    pub fn _Fcan_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
}

#[derive(Debug, Clone)]
pub struct QueueStats {
    pub queue_name: String,
    pub message_count: u64,
    pub consumer_count: u32,
    pub producer_count: u32,
    pub processed_messages: u64,
    pub failed_messages: u64,
    pub avg_processing_time_ms: f64,
}

#[async_trait]
pub trait DMSQueueProducer: Send + Sync {
    async fn _Fsend(&self, message: DMSQueueMessage) -> DMSResult<()>;
    async fn _Fsend_batch(&self, messages: Vec<DMSQueueMessage>) -> DMSResult<()>;
}

#[async_trait]
pub trait DMSQueueConsumer: Send + Sync {
    async fn _Freceive(&self) -> DMSResult<Option<DMSQueueMessage>>;
    async fn _Fack(&self, message_id: &str) -> DMSResult<()>;
    async fn _Fnack(&self, message_id: &str) -> DMSResult<()>;
    async fn _Fpause(&self) -> DMSResult<()>;
    async fn _Fresume(&self) -> DMSResult<()>;
}

#[async_trait]
pub trait DMSQueue: Send + Sync {
    async fn _Fcreate_producer(&self) -> DMSResult<Box<dyn DMSQueueProducer>>;
    async fn _Fcreate_consumer(&self, consumer_group: &str) -> DMSResult<Box<dyn DMSQueueConsumer>>;
    async fn _Fget_stats(&self) -> DMSResult<QueueStats>;
    async fn _Fpurge(&self) -> DMSResult<()>;
    async fn _Fdelete(&self) -> DMSResult<()>;
}