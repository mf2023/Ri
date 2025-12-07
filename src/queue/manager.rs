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

//! # Queue Manager Implementation
//! 
//! This file defines the queue management components for the DMS queue system, including the
//! queue module and queue manager. These components provide the infrastructure for creating,
//! managing, and shutting down queues across different backend implementations.
//! 
//! ## Key Components
//! 
//! - **DMSQueueModule**: Main queue module implementing the `AsyncServiceModule` trait
//! - **DMSQueueManager**: Central queue management component responsible for queue lifecycle
//! 
//! ## Design Principles
//! 
//! 1. **Async-First**: All queue management operations are asynchronous
//! 2. **Backend Agnostic**: Supports multiple queue backends through a unified interface
//! 3. **Thread-safe**: Uses Arc and RwLock for safe concurrent access
//! 4. **Singleton Pattern**: Each queue is created once and shared across the application
//! 5. **Lazy Initialization**: Queues are created on demand when requested
//! 6. **Clean Shutdown**: Properly cleans up resources during shutdown
//! 7. **Non-critical**: Queue failures should not break the entire application
//! 8. **Service Module Integration**: Implements async service module traits for seamless integration
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! use dms::queue::{DMSQueueConfig, QueueBackendType};
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create queue configuration
//!     let queue_config = DMSQueueConfig {
//!         enabled: true,
//!         backend_type: QueueBackendType::Memory,
//!         default_queue_name: "default".to_string(),
//!         max_retry_count: 3,
//!         retry_delay_ms: 1000,
//!         queue_url: "".to_string(),
//!         connection_string: "".to_string(),
//!     };
//!     
//!     // Create queue module
//!     let queue_module = DMSQueueModule::new(queue_config).await?;
    //!     
    //!     // Get queue manager
    //!     let queue_manager = queue_module.queue_manager();
    //!     
    //!     // Create or get a queue
    //!     let queue = queue_manager.create_queue("example_queue").await?;
    //!     
    //!     // List all queues
    //!     let queues = queue_manager.list_queues().await;
//!     println!("Available queues: {:?}", queues);
//!     
//!     Ok(())
//! }
//! ```

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::core::{DMSResult, AsyncServiceModule, DMSServiceContext};
use crate::queue::{DMSQueue, DMSQueueConfig, QueueBackendType};

/// Main queue module implementing the async service module trait.
/// 
/// This module provides the main entry point for the queue system, integrating with the
/// DMS service module system for lifecycle management.
pub struct DMSQueueModule {
    /// The queue manager instance
    queue_manager: Arc<DMSQueueManager>,
}

impl DMSQueueModule {
    /// Creates a new queue module with the given configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The queue configuration to use
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Self>` containing the new queue module instance
    pub async fn new(config: DMSQueueConfig) -> DMSResult<Self> {
        let queue_manager = Arc::new(DMSQueueManager::new(config).await?);
        Ok(Self { queue_manager })
    }

    /// Returns a reference to the queue manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<DMSQueueManager> providing thread-safe access to the queue manager
    pub fn queue_manager(&self) -> Arc<DMSQueueManager> {
        self.queue_manager.clone()
    }
}

#[async_trait]
impl AsyncServiceModule for DMSQueueModule {
    /// Initializes the queue module.
    /// 
    /// This method delegates to the queue manager's initialization method.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn init(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        self.queue_manager.init().await
    }

    /// Performs cleanup after the application has shut down.
    /// 
    /// This method delegates to the queue manager's shutdown method.
    /// 
    /// # Parameters
    /// 
    /// - `_ctx`: The service context (not used in this implementation)
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        self.queue_manager.shutdown().await
    }

    /// Returns the name of the queue module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "dms-queue"
    }

    /// Indicates whether the queue module is critical.
    /// 
    /// The queue module is non-critical, meaning that if it fails to initialize or operate,
    /// it should not break the entire application.
    /// 
    /// # Returns
    /// 
    /// `false` since queue is non-critical
    fn is_critical(&self) -> bool {
        false
    }
}

/// Central queue management component.
/// 
/// This struct is responsible for the lifecycle of queues, including creating, retrieving,
/// listing, and deleting queues. It manages queues across different backend implementations.
pub struct DMSQueueManager {
    /// Queue configuration
    config: DMSQueueConfig,
    /// Map of queue names to queue instances, protected by a RwLock for thread-safe access
    queues: Arc<RwLock<HashMap<String, Arc<dyn DMSQueue>>>>,
}

impl DMSQueueManager {
    /// Creates a new queue manager with the given configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The queue configuration to use
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Self>` containing the new queue manager instance
    pub async fn new(config: DMSQueueConfig) -> DMSResult<Self> {
        Ok(Self {
            config,
            queues: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    


    /// Initializes the queue manager.
    /// 
    /// This method performs backend-specific initialization, such as setting up connection pools
    /// for external queue systems.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub async fn init(&self) -> DMSResult<()> {
        // Initialize queue connections based on backend type
        match self.config.backend_type {
            QueueBackendType::Memory => {
                // Memory queues don't need special initialization
            }
            QueueBackendType::RabbitMQ | QueueBackendType::Kafka | QueueBackendType::Redis => {
                // For external queue systems, we could add connection pooling here
                // For now, we'll create connections on demand
            }
        }
        Ok(())
    }

    /// Creates a new queue or returns an existing one with the same name.
    /// 
    /// This method implements lazy initialization, creating the queue only when requested.
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the queue to create or retrieve
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Arc<dyn DMSQueue>>` containing the queue instance
    pub async fn create_queue(&self, name: &str) -> DMSResult<Arc<dyn DMSQueue>> {
        let mut queues = self.queues.write().await;
        
        if let Some(queue) = queues.get(name) {
            return Ok(queue.clone());
        }

        let queue = self.create_backend_queue(name).await?;
        queues.insert(name.to_string(), queue.clone());
        
        Ok(queue)
    }

    /// Creates a queue with the appropriate backend based on configuration.
    /// 
    /// This is an internal method that creates the actual queue instance based on the
    /// configured backend type.
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the queue to create
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<Arc<dyn DMSQueue>>` containing the created queue instance
    async fn create_backend_queue(&self, name: &str) -> DMSResult<Arc<dyn DMSQueue>> {
        match self.config.backend_type {
            QueueBackendType::Memory => {
                Ok(Arc::new(crate::queue::backends::DMSMemoryQueue::new(name)))
            }
            QueueBackendType::RabbitMQ => {
                Ok(Arc::new(crate::queue::backends::DMSRabbitMQQueue::new(name, &self.config.connection_string).await?))
            }
            QueueBackendType::Kafka => {
                Err(crate::core::DMSError::Config("Kafka support temporarily disabled due to build dependencies".to_string()))
            }
            QueueBackendType::Redis => {
                Ok(Arc::new(crate::queue::backends::DMSRedisQueue::new(name, &self.config.connection_string).await?))
            }
        }
    }

    /// Retrieves an existing queue by name.
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the queue to retrieve
    /// 
    /// # Returns
    /// 
    /// An `Option<Arc<dyn DMSQueue>>` containing the queue instance if it exists, or None otherwise
    pub async fn get_queue(&self, name: &str) -> Option<Arc<dyn DMSQueue>> {
        let queues = self.queues.read().await;
        queues.get(name).cloned()
    }

    /// Lists all currently created queues.
    /// 
    /// # Returns
    /// 
    /// A `Vec<String>` containing the names of all created queues
    pub async fn list_queues(&self) -> Vec<String> {
        let queues = self.queues.read().await;
        queues.keys().cloned().collect()
    }

    /// Deletes a queue by name.
    /// 
    /// This method removes the queue from the manager and calls the queue's delete method
    /// to clean up any backend-specific resources.
    /// 
    /// # Parameters
    /// 
    /// - `name`: The name of the queue to delete
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub async fn delete_queue(&self, name: &str) -> DMSResult<()> {
        let mut queues = self.queues.write().await;
        if let Some(queue) = queues.remove(name) {
            queue.delete().await?;
        }
        Ok(())
    }

    /// Shuts down the queue manager and cleans up resources.
    /// 
    /// This method purges all queues and cleans up any backend-specific resources.
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    pub async fn shutdown(&self) -> DMSResult<()> {
        let mut queues = self.queues.write().await;
        for (_, queue) in queues.drain() {
            // Cleanup each queue
            let _ = queue.purge().await;
        }
        Ok(())
    }
}