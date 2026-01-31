//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Kafka Backend Stub for Windows
//!
//! This module provides a stub implementation for Kafka on Windows.
//! On Windows, rdkafka requires manual build with CMake and vcpkg.
//!
//! To enable full Kafka support on Windows:
//! 1. Install CMake
//! 2. Install vcpkg and run: `vcpkg install librdkafka:x64-windows`
//! 3. Set environment variables: `RDKAFKA_SYS_LIBRDKAFKA_ROOT` and `VCPKG_ROOT`
//! 4. Build with: `cargo build --features kafka`

use crate::core::DMSCResult;
use crate::queue::{DMSCQueue, DMSCQueueProducer, DMSCQueueConsumer, DMSCQueueStats};
use async_trait::async_trait;

/// Stub implementation for Kafka queue on Windows
#[derive(Debug, Clone)]
pub struct DMSCKafkaQueue {
    _brokers: String,
    _topic: String,
}

impl DMSCKafkaQueue {
    pub fn new(_brokers: &str, _topic: &str) -> DMSCResult<Self> {
        Err(crate::core::DMSCError::Other(
            "Kafka backend on Windows requires manual rdkafka build. \
             Please install CMake and vcpkg, then run: \
             vcpkg install librdkafka:x64-windows".to_string()
        ))
    }

    pub fn with_config(_brokers: &str, _topic: &str, _config: crate::queue::config::DMSCQueueConfig) -> DMSCResult<Self> {
        Err(crate::core::DMSCError::Other(
            "Kafka backend on Windows requires manual rdkafka build. \
             Please install CMake and vcpkg, then run: \
             vcpkg install librdkafka:x64-windows".to_string()
        ))
    }
}

#[async_trait]
impl DMSCQueue for DMSCKafkaQueue {
    async fn create_producer(&self) -> DMSCResult<Box<dyn DMSCQueueProducer>> {
        unreachable!("Kafka stub should not be instantiated")
    }

    async fn create_consumer(&self, _consumer_group: &str) -> DMSCResult<Box<dyn DMSCQueueConsumer>> {
        unreachable!("Kafka stub should not be instantiated")
    }

    async fn get_stats(&self) -> DMSCResult<DMSCQueueStats> {
        unreachable!("Kafka stub should not be instantiated")
    }

    async fn purge(&self) -> DMSCResult<()> {
        unreachable!("Kafka stub should not be instantiated")
    }

    async fn delete(&self) -> DMSCResult<()> {
        unreachable!("Kafka stub should not be instantiated")
    }
}
