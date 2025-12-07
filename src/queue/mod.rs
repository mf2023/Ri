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

//! # Queue Module
//! 
//! This module provides a comprehensive queueing system for DMS, offering a unified interface
//! with support for multiple backend implementations. It enables reliable message passing and
//! task scheduling across distributed systems.
//! 
//! ## Key Components
//! 
//! - **DMSQueueModule**: Main queue module implementing service module traits
//! - **DMSQueueManager**: Central queue management component
//! - **DMSQueue**: Unified queue interface implemented by all backends
//! - **DMSQueueConfig**: Configuration for queue behavior
//! - **DMSQueueMessage**: Message structure for queue operations
//! - **DMSQueueConsumer**: Interface for consuming messages from queues
//! - **DMSQueueProducer**: Interface for producing messages to queues
//! - **QueueBackendType**: Enum defining supported queue backends
//! - **QueueStats**: Statistics for queue monitoring
//! 
//! ## Design Principles
//! 
//! 1. **Unified Interface**: Consistent API across all backend implementations
//! 2. **Multiple Backends**: Support for different queue storage options
//! 3. **Async Support**: Full async/await compatibility
//! 4. **Reliable Delivery**: Ensures messages are delivered reliably
//! 5. **Configurable**: Highly configurable queue behavior
//! 6. **Service Module Integration**: Implements service module traits for seamless integration
//! 7. **Thread-safe**: Safe for concurrent use across multiple threads
//! 8. **Statistics Collection**: Built-in queue statistics for monitoring
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use serde::{Serialize, Deserialize};
//! 
//! #[derive(Debug, Serialize, Deserialize)]
//! struct Task {
//!     id: String,
//!     data: String,
//! }
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create queue configuration
//!     let queue_config = DMSQueueConfig {
//!         enabled: true,
//!         backend_type: QueueBackendType::Memory,
//!         default_queue_name: "default".to_string(),
//!         max_retry_count: 3,
//!         retry_delay_ms: 1000,
//!         queue_url: "".to_string(), // Not needed for memory backend
//!     };
//!     
//!     // Create queue module
//!     let queue_module = DMSQueueModule::new(queue_config);
//!     
//!     // Get queue manager
//!     let queue_manager = queue_module.queue_manager();
//!     
//!     // Get queue instance
//!     let queue = queue_manager.read().await.queue("example_queue").await?;
//!     
//!     // Create producer and consumer
//!     let producer = queue.producer().await?;
//!     let consumer = queue.consumer().await?;
//!     
//!     // Create a task message
//!     let task = Task {
//!         id: "task-123".to_string(),
//!         data: "Hello, DMS Queue!".to_string(),
//!     };
//!     
//!     // Send message to queue
//!     let message_id = producer.send(&task).await?;
//!     println!("Sent message with ID: {}", message_id);
//!     
//!     // Receive message from queue
//!     if let Some(message) = consumer.receive().await? {
//!         let received_task: Task = message.deserialize()?;
//!         println!("Received task: {:?}", received_task);
//!         
//!         // Acknowledge message
//!         message.ack().await?;
//!     }
//!     
//!     Ok(())
//! }
//! ```

mod queue;
mod backends;
mod config;
mod manager;

pub use queue::{DMSQueue, DMSQueueMessage, DMSQueueConsumer, DMSQueueProducer, QueueStats};
pub use config::{DMSQueueConfig, QueueBackendType};
pub use manager::{DMSQueueManager, DMSQueueModule};