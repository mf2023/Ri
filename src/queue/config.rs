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

use serde::{Serialize, Deserialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSQueueConfig {
    pub enabled: bool,
    pub backend_type: QueueBackendType,
    pub connection_string: String,
    pub max_connections: u32,
    pub message_max_size: usize,
    pub consumer_timeout_ms: u64,
    pub producer_timeout_ms: u64,
    pub retry_policy: RetryPolicy,
    pub dead_letter_config: Option<DeadLetterConfig>,
}

impl Default for DMSQueueConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            backend_type: QueueBackendType::Memory,
            connection_string: "memory://localhost".to_string(),
            max_connections: 10,
            message_max_size: 1024 * 1024, // 1MB
            consumer_timeout_ms: 30000, // 30 seconds
            producer_timeout_ms: 5000,  // 5 seconds
            retry_policy: RetryPolicy::default(),
            dead_letter_config: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueueBackendType {
    Memory,
    RabbitMQ,
    Kafka,
    Redis,
}

impl FromStr for QueueBackendType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "memory" => Ok(QueueBackendType::Memory),
            "rabbitmq" => Ok(QueueBackendType::RabbitMQ),
            "kafka" => Ok(QueueBackendType::Kafka),
            "redis" => Ok(QueueBackendType::Redis),
            _ => Err(format!("Unknown queue backend type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub initial_delay_ms: u64,
    pub max_delay_ms: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    pub enabled: bool,
    pub max_retry_count: u32,
    pub dead_letter_queue_name: String,
    pub ttl_hours: u32,
}