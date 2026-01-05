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

//! # Kafka Queue Backend (Windows Stub)
//!
//! This module provides a stub implementation for Kafka on Windows platforms.
//!
//! **Note:** Apache Kafka support is disabled on Windows due to native library dependencies
//! (OpenSSL, librdkafka). This stub provides a clear error message when users attempt to use
//! Kafka on Windows.
//!
//! ## Why is Kafka disabled on Windows?
//!
//! 1. **Native Dependencies**: The `rdkafka` crate requires librdkafka, which depends on
//!    OpenSSL and other native libraries that have limited Windows support.
//!
//! 2. **Build Complexity**: Compiling rdkafka on Windows requires:
//!    - Visual Studio build tools
//!    - Perl for OpenSSL configuration
//!    - Rust toolchain with C++ compiler
//!
//! 3. **Runtime Dependencies**: Even if built, rdkafka requires:
//!    - Visual C++ Redistributable
//!    - OpenSSL DLLs in system PATH
//!
//! ## Alternatives for Windows Development
//!
//! 1. **Use WSL2 (Recommended)**: Run Kafka inside Windows Subsystem for Linux 2
//!
//! 2. **Use Docker**: Run Kafka in a Docker container:
//!    ```bash
//!    docker run -p 9092:9092 -e ALLOW_PLAINTEXT_LISTENER=yes bitnami/kafka:latest
//!    ```
//!
//! 3. **Use Cloud Kafka**: Use a managed Kafka service like:
//!    - Confluent Cloud
//!    - Amazon MSK
//!    - Azure Event Hubs
//!
//! 4. **Use Alternative Queue Backends**: During Windows development, use:
//!    - `DMSCMemoryQueue`: In-memory queue for testing
//!    - `DMSCRedisQueue`: Redis-based distributed queue
//!    - `DMSCRabbitMQQueue`: RabbitMQ-based queue
//!
//! ## Building Kafka Support on Windows (Advanced Users)
//!
//! If you must build Kafka support on Windows:
//!
//! 1. Install prerequisites:
//!    ```bash
//!    # Install Strawberry Perl
//!    # Install NASM
//!    # Install vcpkg
//!    vcpkg install openssl:x64-windows
//!    vcpkg install librdkafka:x64-windows
//!    ```
//!
//! 2. Set environment variables:
//!    ```powershell
//!    $env:OPENSSL_LIB_DIR = "C:\vcpkg\installed\x64-windows\lib"
//!    $env:OPENSSL_INCLUDE_DIR = "C:\vcpkg\installed\x64-windows\include"
//!    $env:RDKAFKA_LIB_DIR = "C:\vcpkg\installed\x64-windows\lib"
//!    $env:RDKAFKA_INCLUDE_DIR = "C:\vcpkg\installed\x64-windows\include"
//!    ```
//!
//! 3. Build with custom feature:
//!    ```bash
//!    cargo build --features kafka-windows-build
//!    ```
//!
//! ## Usage Example (This will fail on Windows)
//!
//! ```rust,ignore
//! use dms::prelude::*;
//!
//! async fn example() -> DMSCResult<()> {
//!     // This will return an error on Windows
//!     let queue = DMSCKafkaQueue::new("test-topic", "localhost:9092").await?;
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use std::sync::Arc;
use crate::core::{DMSCResult, DMSCError};
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats};

const KAFKA_WINDOWS_ERROR: &str = "Kafka support is not available on Windows platforms. \
Please use one of the following alternatives: \
1. DMSCMemoryQueue (in-memory, for testing) \
2. DMSCRedisQueue (requires Redis server) \
3. DMSCRabbitMQQueue (requires RabbitMQ server) \
4. Use WSL2 or Docker for Kafka development";

/// Stub Kafka queue implementation for Windows platforms.
///
/// This struct provides a stub that returns errors when any method is called,
/// with helpful error messages explaining why Kafka is not available on Windows.
#[derive(Clone)]
pub struct DMSCKafkaQueue;

impl DMSCKafkaQueue {
    /// Creates a new stub Kafka queue instance.
    ///
    /// **This will always return an error on Windows.**
    ///
    /// # Parameters
    ///
    /// - `_name`: The name of the Kafka topic (ignored)
    /// - `_connection_string`: The Kafka bootstrap servers connection string (ignored)
    ///
    /// # Returns
    ///
    /// Always returns `DMSCError::Config` with a helpful message about Windows limitations
    pub async fn new(_name: &str, _connection_string: &str) -> DMSCResult<Self> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn create_consumer(&self, _consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn purge(&self) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn delete(&self) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

/// Stub Kafka producer implementation for Windows platforms.
#[derive(Clone)]
pub struct KafkaProducer;

#[async_trait]
impl DMSCQueueProducer for KafkaProducer {
    async fn send(&self, _message: DMSCQueueMessage) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn send_batch(&self, _messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

/// Stub Kafka consumer implementation for Windows platforms.
#[derive(Clone)]
pub struct KafkaConsumer;

#[async_trait]
impl DMSCQueueConsumer for KafkaConsumer {
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn nack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn pause(&self) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn resume(&self) -> DMSCResult<()> {
        Err(DMSCError::Config(KAFKA_WINDOWS_ERROR.to_string()))
    }
}
