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
//! (OpenSSL, librdkafka). This stub provides clear error messages and alternative solutions
//! when users attempt to use Kafka on Windows.
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
//! ## Alternative Solutions for Windows Development
//!
//! ### Option 1: Use WSL2 (Recommended)
//! Run Kafka inside Windows Subsystem for Linux 2:
//! ```bash
//! wsl --install -d Ubuntu
//! wsl
//! docker run -p 9092:9092 -e ALLOW_PLAINTEXT_LISTENER=yes bitnami/kafka:latest
//! ```
//!
//! ### Option 2: Use Docker
//! Run Kafka in a Docker container:
//! ```bash
//! docker run -p 9092:9092 -e ALLOW_PLAINTEXT_LISTENER=yes bitnami/kafka:latest
//! ```
//! Then configure DMSC to connect to `localhost:9092` (Docker forwards the port).
//!
//! ### Option 3: Use Cloud Kafka
//! Use a managed Kafka service:
//! - Confluent Cloud (free tier available)
//! - Amazon MSK
//! - Azure Event Hubs
//!
//! ### Option 4: Use Alternative Queue Backends
//! During Windows development, use:
//! - `DMSCMemoryQueue`: In-memory queue for testing (no external dependencies)
//! - `DMSCRedisQueue`: Redis-based distributed queue (works on Windows)
//! - `DMSCRabbitMQQueue`: RabbitMQ-based queue (works on Windows)
//!
//! ## Code Examples for Alternatives
//!
//! ### Using Memory Queue (Recommended for Testing)
//! ```rust
//! use dmsc::prelude::*;
//!
//! async fn example() -> DMSCResult<()> {
//!     // Use memory queue for testing - works on all platforms
//!     let queue = DMSCMemoryQueue::new("test-topic").await?;
//!     Ok(())
//! }
//! ```
//!
//! ### Using Redis Queue
//! ```rust
//! use dmsc::prelude::*;
//!
//! async fn example() -> DMSCResult<()> {
//!     // Use Redis queue - requires Redis server
//!     let queue = DMSCRedisQueue::new("test-topic", "redis://localhost").await?;
//!     Ok(())
//! }
//! ```
//!
//! ## Building Kafka Support on Windows (Advanced Users)
//!
//! If you must build Kafka support on Windows:
//!
//! 1. Install prerequisites:
//!    ```bash
//!    # Install Strawberry Perl from https://strawberryperl.com/
//!    # Install NASM from https://www.nasm.us/
//!    # Install vcpkg from https://vcpkg.io/
//!    vcpkg integrate install
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
//! 3. Create a custom feature in Cargo.toml:
//!    ```toml
//!    [features]
//!    kafka-windows = []
//!    ```
//!
//! 4. Build with custom feature:
//!    ```bash
//!    cargo build --features "kafka,kafka-windows"
//!    ```
//!
//! ## DMSC Configuration for Windows Development
//!
//! ### Recommended: Use Memory Queue
//! ```yaml
//! queue:
//!   type: "memory"
//!   topic: "my-topic"
//! ```
//!
//! ### Alternative: Use Redis Queue
//! ```yaml
//! queue:
//!   type: "redis"
//!   topic: "my-topic"
//!   connection_string: "redis://localhost"
//! ```
//!
//! ## Feature Flag Usage
//!
//! To use Kafka in production (Linux/macOS):
//! ```toml
//! [dependencies]
//! dmsc = { version = "0.1", features = ["kafka"] }
//! ```
//!
//! On Windows, this feature will be automatically disabled with a compile-time warning.
//! Use one of the alternative backends instead.

use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error as ThisError;
use crate::core::{DMSCResult, DMSCError};
use crate::queue::{DMSCQueue, DMSCQueueMessage, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats, DMSCQueueError};

const KAFKA_WINDOWS_ERROR: &str = r#"Kafka support is not available on Windows platforms.

This is because the rdkafka library requires native dependencies (OpenSSL, librdkafka)
that have limited Windows support.

## Recommended Alternatives:

1. DMSCMemoryQueue (Recommended for testing)
   - No external dependencies
   - In-memory, perfect for unit tests
   - Configure: queue.type = "memory"

2. DMSCRedisQueue (Recommended for development)
   - Requires Redis server (works on Windows)
   - Configure: queue.type = "redis"

3. DMSCRabbitMQQueue
   - Requires RabbitMQ server (works on Windows)
   - Configure: queue.type = "rabbitmq"

4. Use WSL2 or Docker
   - Run Kafka inside WSL2 or Docker container
   - Configure queue.connection_string to point to the container

## Configuration Example:

```yaml
queue:
  type: "memory"  # or "redis" / "rabbitmq"
  topic: "my-topic"
  connection_string: "redis://localhost"  # for Redis/RabbitMQ
```

For more information, visit: https://dmsc.dunimd.dev/queue"#;

/// Detailed error information for Kafka Windows stub
#[derive(Debug, ThisError)]
pub enum KafkaWindowsError {
    #[error("Kafka is not supported on Windows: {0}")]
    UnsupportedPlatform(String),

    #[error("Alternative queue backend recommendation: {0}")]
    AlternativeBackend(&'static str),

    #[error("Documentation URL: https://dmsc.dunimd.dev/queue")]
    DocumentationUrl,
}

impl From<KafkaWindowsError> for DMSCError {
    fn from(e: KafkaWindowsError) -> Self {
        DMSCError::Queue(e.to_string())
    }
}

/// Provides helpful information about alternative queue backends
#[derive(Clone)]
pub struct KafkaWindowsHelper;

impl KafkaWindowsHelper {
    /// Returns a list of recommended alternative backends
    pub fn recommended_backends() -> Vec<(&'static str, &'static str, &'static str)> {
        vec![
            ("memory", "In-memory queue for testing", "No external dependencies required"),
            ("redis", "Redis-based distributed queue", "Requires Redis server (works on Windows)"),
            ("rabbitmq", "RabbitMQ-based queue", "Requires RabbitMQ server (works on Windows)"),
        ]
    }

    /// Returns configuration example for a given backend type
    pub fn config_example(backend_type: &str) -> String {
        match backend_type {
            "memory" => r#"queue:
  type: "memory"
  topic: "my-topic""#.to_string(),
            "redis" => r#"queue:
  type: "redis"
  topic: "my-topic"
  connection_string: "redis://localhost""#.to_string(),
            "rabbitmq" => r#"queue:
  type: "rabbitmq"
  topic: "my-topic"
  connection_string: "amqp://localhost""#.to_string(),
            _ => format!("Unknown backend type: {}", backend_type),
        }
    }

    /// Returns the Docker command to start Kafka in WSL2/Docker
    pub fn docker_command() -> String {
        "docker run -p 9092:9092 -e ALLOW_PLAINTEXT_LISTENER=yes bitnami/kafka:latest".to_string()
    }

    /// Checks if the current platform supports Kafka
    pub fn is_supported() -> bool {
        false // Always returns false on Windows
    }
}

/// Stub Kafka queue implementation for Windows platforms.
///
/// This struct provides a stub that returns errors when any method is called,
/// with helpful error messages explaining why Kafka is not available on Windows
/// and providing alternative solutions.
///
/// # Example
///
/// ```rust,ignore
/// use dmsc::queue::DMSCKafkaQueue;
///
/// async fn example() -> DMSCResult<()> {
///     // This will always fail on Windows with a helpful error message
///     let queue = DMSCKafkaQueue::new("test-topic", "localhost:9092").await?;
///     Ok(())
/// }
/// ```
///
/// # Recommendation
///
/// Use `DMSCMemoryQueue` instead for testing:
///
/// ```rust,ignore
/// use dmsc::queue::DMSCMemoryQueue;
///
/// async fn example() -> DMSCResult<()> {
///     // This works on all platforms
///     let queue = DMSCMemoryQueue::new("test-topic").await?;
///     Ok(())
/// }
/// ```
#[derive(Clone, Default)]
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
    /// Always returns `DMSCError::Queue` with a helpful message about Windows limitations
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let result = DMSCKafkaQueue::new("test-topic", "localhost:9092").await;
    /// assert!(result.is_err());
    /// ```
    pub async fn new(_name: &str, _connection_string: &str) -> DMSCResult<Self> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    /// Returns information about alternative backends
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - List of recommended alternative backends
    /// - Docker command to run Kafka
    /// - Whether Kafka is supported (always false on Windows)
    pub fn get_alternatives() -> (Vec<(&'static str, &'static str, &'static str)>, String, bool) {
        (
            KafkaWindowsHelper::recommended_backends(),
            KafkaWindowsHelper::docker_command(),
            KafkaWindowsHelper::is_supported(),
        )
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn create_consumer(&self, _consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn purge(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn delete(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

/// Stub Kafka producer implementation for Windows platforms.
#[derive(Clone, Default)]
pub struct KafkaProducer;

#[async_trait]
impl DMSCQueueProducer for KafkaProducer {
    async fn send(&self, _message: DMSCQueueMessage) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn send_batch(&self, _messages: Vec<DMSCQueueMessage>) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

/// Stub Kafka consumer implementation for Windows platforms.
#[derive(Clone, Default)]
pub struct KafkaConsumer;

#[async_trait]
impl DMSCQueueConsumer for KafkaConsumer {
    async fn receive(&self) -> DMSCResult<Option<DMSCQueueMessage>> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn ack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn nack(&self, _message_id: &str) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn pause(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }

    async fn resume(&self) -> DMSCResult<()> {
        Err(DMSCError::Queue(KAFKA_WINDOWS_ERROR.to_string()))
    }
}

/// Compile-time check to warn users when trying to use Kafka on Windows
#[cfg(all(feature = "kafka", windows))]
compile_error!(
    "Kafka support is not available on Windows platforms.

Please use one of the following alternative queue backends:
1. DMSCMemoryQueue (for testing) - configure queue.type = \"memory\"
2. DMSCRedisQueue (for development) - configure queue.type = \"redis\"
3. DMSCRabbitMQQueue - configure queue.type = \"rabbitmq\"

Or run Kafka in Docker/WSL2 and configure the connection string accordingly.

Remove the \"kafka\" feature from your Cargo.toml dependencies to silence this error."
);

#[cfg(all(feature = "kafka", windows))]
const _: () = {
    // This will cause a compile-time error if kafka feature is enabled on Windows
    // The actual implementation is in the compile_error! macro above
};
