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

//! # Kafka Queue Backend (Windows Build Note)
//!
//! This module provides a stub implementation when the Kafka feature is enabled on Windows.
//!
//! **Note:** On Windows, building rdkafka from source requires additional build tools.
//! For production deployments, Kafka typically runs on Linux servers.
//!
//! ## Platform Support
//!
//! | Platform | Status | Notes |
//! |----------|--------|-------|
//! | Linux | ✅ Full Support | Native rdkafka support |
//! | macOS | ✅ Full Support | Native rdkafka support |
//! | Windows | ⚠️ Stub Only | Requires manual build |
//!
//! ## Options for Windows Development
//!
//! 1. **Use Linux/WSL2** (Recommended)
//!    Run Kafka in WSL2 or a Linux VM, then connect remotely.
//!
//! 2. **Use Docker**
//!    ```bash
//!    docker run -p 9092:9092 -e ALLOW_PLAINTEXT_LISTENER=yes bitnami/kafka:latest
//!    ```
//!
//! 3. **Use Alternative Backends**
//!    - `DMSCMemoryQueue`: In-memory queue for testing
//!    - `DMSCRedisQueue`: Requires Redis server
//!    - `DMSCRabbitMQQueue`: Requires RabbitMQ server
//!
//! ## Runtime Detection
//!
//! Use `DMSCKafkaQueue::is_available()` to check if Kafka is available at runtime.
//!
//! ## Building rdkafka on Windows (Advanced)
//!
//! If you need to build rdkafka on Windows:
//!
//! 1. Install vcpkg: https://vcpkg.io/
//! 2. Install dependencies:
//!    ```powershell
//!    vcpkg integrate install
//!    vcpkg install openssl:x64-windows
//!    vcpkg install librdkafka:x64-windows
//!    ```
//! 3. Set environment variables:
//!    ```powershell
//!    $env:OPENSSL_LIB_DIR = "C:\vcpkg\installed\x64-windows\lib"
//!    $env:OPENSSL_INCLUDE_DIR = "C:\vcpkg\installed\x64-windows\include"
//!    $env:RDKAFKA_LIB_DIR = "C:\vcpkg\installed\x64-windows\lib"
//!    $env:RDKAFKA_INCLUDE_DIR = "C:\vcpkg\installed\x64-windows\include"
//!    ```
//! 4. Build with vendored OpenSSL:
//!    ```toml
//!    rdkafka = { version = "0.38", features = ["tokio", "libz", "vendored"] }
//!    ```

#[cfg(windows)]
compile_error!("Kafka backend on Windows requires manual rdkafka build. See kafka_stub.rs documentation for details.");

use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error as ThisError;
use crate::core::{DMSCResult, DMSCError};
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats, DMSCQueueError};

const KAFKA_UNAVAILABLE_MESSAGE: &str = r#"Kafka support is not available on this platform.

This may occur because:
1. You are building on Windows (rdkafka requires manual build)
2. The rdkafka native library is not installed

For immediate use, consider these alternatives:
1. DMSCMemoryQueue - in-memory queue for testing
2. DMSCRedisQueue - requires Redis server
3. DMSCRabbitMQQueue - requires RabbitMQ server

For Linux/macOS, ensure rdkafka is installed:
- Ubuntu/Debian: apt install librdkafka-dev
- macOS: brew install librdkafka

Documentation: https://dmsc.dunimd.dev/queue
"#;

#[derive(Debug, ThisError)]
pub enum KafkaWindowsBuildError {
    #[error("Kafka requires Linux/macOS or manual build on Windows: {0}")]
    BuildRequired(String),
}

impl From<KafkaWindowsBuildError> for DMSCError {
    fn from(e: KafkaWindowsBuildError) -> Self {
        DMSCError::Queue(e.to_string())
    }
}

#[derive(Clone, Default)]
pub struct DMSCKafkaQueue;

impl DMSCKafkaQueue {
    pub async fn new(_name: &str, _connection_string: &str) -> DMSCResult<Self> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    pub fn is_available() -> bool {
        false
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn create_consumer(&self, _consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn purge(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn delete(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }
}

#[derive(Clone, Default)]
pub struct KafkaProducer;

#[async_trait]
impl DMSCQueueProducer for KafkaProducer {
    async fn send(&self, _message: DMSCQueueMessage) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn send_batch(&self, _messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }
}

#[derive(Clone, Default)]
pub struct KafkaConsumer;

#[async_trait]
impl DMSCQueueConsumer for KafkaConsumer {
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn nack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn pause(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }

    async fn resume(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_UNAVAILABLE_MESSAGE.to_string()))
    }
}
