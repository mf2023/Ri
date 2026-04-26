//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of Ri.
//! The Ri project belongs to the Dunimd Team.
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
//! This file defines the queue management components for the Ri queue system, including the
//! queue module and queue manager. These components provide the infrastructure for creating,
//! managing, and shutting down queues across different backend implementations.
//! 
//! ## Key Components
//! 
//! - **RiQueueModule**: Main queue module implementing the `AsyncServiceModule` trait
//! - **RiQueueManager**: Central queue management component responsible for queue lifecycle
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
//! use ri::prelude::*;
//! use ri::queue::{RiQueueConfig, RiQueueBackendType};
//! 
//! async fn example() -> RiResult<()> {
//!     // Create queue configuration
//!     let queue_config = RiQueueConfig {
//!         enabled: true,
//!         backend_type: RiQueueBackendType::Memory,
//!         default_queue_name: "default".to_string(),
//!         max_retry_count: 3,
//!         retry_delay_ms: 1000,
//!         queue_url: "".to_string(),
//!         connection_string: "".to_string(),
//!     };
//!     
//!     // Create queue module
//!     let queue_module = RiQueueModule::new(queue_config).await?;
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
use std::collections::HashMap as FxHashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::core::{RiResult, AsyncServiceModule, RiServiceContext};
use crate::queue::{RiQueue, RiQueueConfig, RiQueueBackendType};

#[cfg(feature = "pyo3")]
use pyo3::PyResult;

/// Connection pool for external queue backends
struct QueueConnectionPool {
    backend_type: RiQueueBackendType,
    connections: Arc<RwLock<Vec<Arc<dyn std::any::Any + Send + Sync>>>>,
    max_connections: usize,
    #[allow(dead_code)]
    connection_string: String,
}

impl QueueConnectionPool {
    fn new(backend_type: RiQueueBackendType, connection_string: String, max_connections: usize) -> Self {
        Self {
            backend_type,
            connections: Arc::new(RwLock::new(Vec::new())),
            max_connections,
            connection_string,
        }
    }

    #[allow(dead_code)]
    async fn get_connection(&self) -> RiResult<Arc<dyn std::any::Any + Send + Sync>> {
        let mut connections = self.connections.write().await;
        
        if let Some(conn) = connections.pop() {
            return Ok(conn);
        }

        // Create new connection if pool is empty and under limit
        if connections.len() < self.max_connections {
            let conn = self.create_connection().await?;
            return Ok(conn);
        }

        Err(crate::core::RiError::Other("Connection pool exhausted".to_string()))
    }

    async fn return_connection(&self, connection: Arc<dyn std::any::Any + Send + Sync>) {
        let mut connections = self.connections.write().await;
        if connections.len() < self.max_connections {
            connections.push(connection);
        }
    }

    async fn create_connection(&self) -> RiResult<Arc<dyn std::any::Any + Send + Sync>> {
        match self.backend_type {
            #[cfg(feature = "rabbitmq")]
            RiQueueBackendType::RabbitMQ => {
                use lapin::{Connection, ConnectionProperties};
                let conn = Connection::connect(&self.connection_string, ConnectionProperties::default()).await?;
                Ok(Arc::new(conn))
            }
            #[cfg(not(feature = "rabbitmq"))]
            RiQueueBackendType::RabbitMQ => {
                Err(crate::core::RiError::Config(
                    "RabbitMQ support is disabled. Enable the 'rabbitmq' feature to use RabbitMQ backend.".to_string(),
                ))
            }
            #[cfg(feature = "redis")]
            RiQueueBackendType::Redis => {
                use redis::Client;
                let client = Client::open(self.connection_string.as_str())?;
                let conn = client.get_multiplexed_async_connection().await?;
                Ok(Arc::new(conn))
            }
            #[cfg(not(feature = "redis"))]
            RiQueueBackendType::Redis => {
                Err(crate::core::RiError::Config(
                    "Redis support is disabled. Enable the 'redis' feature to use Redis backend.".to_string(),
                ))
            }
            #[cfg(all(feature = "kafka", not(windows)))]
            RiQueueBackendType::Kafka => {
                // For Kafka, we don't need persistent connections like RabbitMQ/Redis
                // The Kafka client manages its own connections internally
                Ok(Arc::new(()))
            }
            #[cfg(any(not(feature = "kafka"), windows))]
            RiQueueBackendType::Kafka => {
                Err(crate::core::RiError::Config(
                    "Kafka support is disabled. Enable the 'kafka' feature to use Kafka backend.".to_string(),
                ))
            }
            RiQueueBackendType::Memory => {
                // Memory queue doesn't require any external connections
                Ok(Arc::new(()))
            }
        }
    }

    async fn close_all(&self) {
        let mut connections = self.connections.write().await;
        connections.clear();
    }
}

/// Main queue module implementing the async service module trait.
/// 
/// This module provides the main entry point for the queue system, integrating with the
/// Ri service module system for lifecycle management.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiQueueModule {
    /// The queue manager instance
    queue_manager: Arc<RiQueueManager>,
}

impl RiQueueModule {
    /// Creates a new queue module with the given configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The queue configuration to use
    /// 
    /// # Returns
    /// 
    /// A `RiResult<Self>` containing the new queue module instance
    pub async fn new(config: RiQueueConfig) -> RiResult<Self> {
        let queue_manager = Arc::new(RiQueueManager::new(config).await?);
        Ok(Self { queue_manager })
    }

    /// Creates a new queue module with the given configuration (synchronous version).
    /// 
    /// This is a synchronous wrapper for use in the builder pattern.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The queue configuration to use
    /// 
    /// # Returns
    /// 
    /// A `RiResult<Self>` containing the new queue module instance
    pub fn with_config(_config: RiQueueConfig) -> RiResult<Self> {
        // Create a simple memory-based queue manager for synchronous initialization
        let queue_manager = Arc::new(RiQueueManager::default());
        Ok(Self { queue_manager })
    }

    /// Returns a reference to the queue manager.
    /// 
    /// # Returns
    /// 
    /// An Arc<RiQueueManager> providing thread-safe access to the queue manager
    pub fn queue_manager(&self) -> Arc<RiQueueManager> {
        self.queue_manager.clone()
    }
}

#[cfg(feature = "pyo3")]
/// Python bindings for RiQueueModule
#[pyo3::prelude::pymethods]
impl RiQueueModule {
    #[new]
    fn py_new() -> PyResult<Self> {
        use crate::queue::RiQueueConfig;
        use crate::queue::RiQueueBackendType;
        
        let config = RiQueueConfig {
            enabled: true,
            backend_type: RiQueueBackendType::Memory,
            connection_string: "memory://localhost".to_string(),
            max_connections: 10,
            message_max_size: 1024 * 1024,
            consumer_timeout_ms: 30000,
            producer_timeout_ms: 30000,
            retry_policy: crate::queue::config::RiRetryPolicy::default(),
            dead_letter_config: None,
        };
        
        let runtime = tokio::runtime::Handle::current();
        let result = runtime.block_on(async {
            Self::new(config).await
        });
        
        match result {
            Ok(module) => Ok(module),
            Err(e) => Err(pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create queue module: {e}"))),
        }
    }
}

#[async_trait]
impl AsyncServiceModule for RiQueueModule {
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
    /// A `RiResult<()>` indicating success or failure
    async fn init(&mut self, _ctx: &mut RiServiceContext) -> RiResult<()> {
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
    /// A `RiResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, _ctx: &mut RiServiceContext) -> RiResult<()> {
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

impl crate::core::ServiceModule for RiQueueModule {
    fn name(&self) -> &str {
        "Ri.Queue"
    }

    fn is_critical(&self) -> bool {
        false
    }

    fn priority(&self) -> i32 {
        15
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }

    fn init(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }

    fn start(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }

    fn shutdown(&mut self, _ctx: &mut crate::core::RiServiceContext) -> crate::core::RiResult<()> {
        Ok(())
    }
}

/// Central queue management component.
/// 
/// This struct is responsible for the lifecycle of queues, including creating, retrieving,
/// listing, and deleting queues. It manages queues across different backend implementations.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiQueueManager {
    /// Queue configuration
    config: RiQueueConfig,
    /// Map of queue names to queue instances, protected by a RwLock for thread-safe access
    queues: Arc<RwLock<FxHashMap<String, Arc<dyn RiQueue>>>>,
    /// Connection pool for external backends
    connection_pool: Option<Arc<QueueConnectionPool>>,
}

impl RiQueueManager {
    /// Creates a new queue manager with the given configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The queue configuration to use
    /// 
    /// # Returns
    /// 
    /// A `RiResult<Self>` containing the new queue manager instance
    pub async fn new(config: RiQueueConfig) -> RiResult<Self> {
        let backend_type = config.backend_type.clone();
        let connection_string = config.connection_string.clone();
        
        let connection_pool = match backend_type {
            RiQueueBackendType::RabbitMQ | RiQueueBackendType::Redis => {
                Some(Arc::new(QueueConnectionPool::new(
                    backend_type,
                    connection_string,
                    10, // max_connections
                )))
            }
            #[cfg(all(feature = "kafka", not(windows)))]
            RiQueueBackendType::Kafka => {
                Some(Arc::new(QueueConnectionPool::new(
                    backend_type,
                    connection_string,
                    10, // max_connections
                )))
            }
            #[cfg(any(not(feature = "kafka"), windows))]
            RiQueueBackendType::Kafka => {
                None
            }
            _ => None,
        };

        Ok(Self {
            config: RiQueueConfig {
                enabled: config.enabled,
                backend_type: config.backend_type.clone(),
                connection_string: config.connection_string.clone(),
                max_connections: config.max_connections,
                message_max_size: config.message_max_size,
                consumer_timeout_ms: config.consumer_timeout_ms,
                producer_timeout_ms: config.producer_timeout_ms,
                retry_policy: config.retry_policy.clone(),
                dead_letter_config: config.dead_letter_config.clone(),
            },
            queues: Arc::new(RwLock::new(FxHashMap::default())),
            connection_pool,
        })
    }
}

impl Default for RiQueueManager {
    fn default() -> Self {
        Self {
            config: RiQueueConfig {
                enabled: true,
                backend_type: RiQueueBackendType::Memory,
                connection_string: "memory://localhost".to_string(),
                max_connections: 10,
                message_max_size: 1024 * 1024,
                consumer_timeout_ms: 30000,
                producer_timeout_ms: 30000,
                retry_policy: crate::queue::config::RiRetryPolicy::default(),
                dead_letter_config: None,
            },
            queues: Arc::new(RwLock::new(FxHashMap::default())),
            connection_pool: None,
        }
    }
}

impl RiQueueManager {
    /// Initializes the queue manager.
    /// 
    /// This method performs backend-specific initialization, such as setting up connection pools
    /// for external queue systems.
    /// 
    /// # Returns
    /// 
    /// A `RiResult<()>` indicating success or failure
    pub async fn init(&self) -> RiResult<()> {
        // Initialize connection pool for external backends
        if let Some(ref pool) = self.connection_pool {
            // Pre-create a few connections for better performance
            for _ in 0..3 {
                if let Ok(conn) = pool.create_connection().await {
                    let _ = pool.return_connection(conn).await;
                }
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
    /// A `RiResult<Arc<dyn RiQueue>>` containing the queue instance
    pub async fn create_queue(&self, name: &str) -> RiResult<Arc<dyn RiQueue>> {
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
    /// A `RiResult<Arc<dyn RiQueue>>` containing the created queue instance
    async fn create_backend_queue(&self, name: &str) -> RiResult<Arc<dyn RiQueue>> {
        match self.config.backend_type {
            RiQueueBackendType::Memory => {
                Ok(Arc::new(crate::queue::backends::RiMemoryQueue::new(name)))
            }
            #[cfg(feature = "rabbitmq")]
            RiQueueBackendType::RabbitMQ => {
                if let Some(ref pool) = self.connection_pool {
                    let conn = pool.get_connection().await?;
                    // Extract the actual lapin connection from the pooled connection
                    if let Some(_lapin_conn) = conn.downcast_ref::<lapin::Connection>() {
                        // Since lapin::Connection doesn't implement Clone, we need to create a new connection
                        // This is a workaround - in a real system, you'd want to use connection pooling properly
                        let queue = crate::queue::backends::RiRabbitMQQueue::new(
                            name,
                            &self.config.connection_string,
                        )
                        .await?;
                        // Return connection to pool
                        let _ = pool.return_connection(conn).await;
                        return Ok(Arc::new(queue));
                    }
                }
                Ok(Arc::new(
                    crate::queue::backends::RiRabbitMQQueue::new(
                        name,
                        &self.config.connection_string,
                    )
                    .await?,
                ))
            }
            #[cfg(not(feature = "rabbitmq"))]
            RiQueueBackendType::RabbitMQ => {
                Err(crate::core::RiError::Config(
                    "RabbitMQ support is disabled. Enable the 'rabbitmq' feature to use RabbitMQ backend.".to_string(),
                ))
            }
            #[cfg(all(feature = "kafka", not(windows)))]
            RiQueueBackendType::Kafka => {
                Ok(Arc::new(
                    crate::queue::backends::RiKafkaQueue::new(
                        name,
                        &self.config.connection_string,
                    )
                    .await?,
                ))
            }
            #[cfg(any(not(feature = "kafka"), windows))]
            RiQueueBackendType::Kafka => {
                Err(crate::core::RiError::Config(
                    "Kafka support is disabled. Enable the 'kafka' feature to use Kafka backend.".to_string(),
                ))
            }
            #[cfg(feature = "redis")]
            RiQueueBackendType::Redis => {
                Ok(Arc::new(
                    crate::queue::backends::RiRedisQueue::new(
                        name,
                        &self.config.connection_string,
                    )
                    .await?,
                ))
            }
            #[cfg(not(feature = "redis"))]
            RiQueueBackendType::Redis => {
                Err(crate::core::RiError::Config(
                    "Redis support is disabled. Enable the 'redis' feature to use Redis backend.".to_string(),
                ))
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
    /// An `Option<Arc<dyn RiQueue>>` containing the queue instance if it exists, or None otherwise
    pub async fn get_queue(&self, name: &str) -> Option<Arc<dyn RiQueue>> {
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
    /// A `RiResult<()>` indicating success or failure
    pub async fn delete_queue(&self, name: &str) -> RiResult<()> {
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
    /// A `RiResult<()>` indicating success or failure
    pub async fn shutdown(&self) -> RiResult<()> {
        let mut queues = self.queues.write().await;
        for (_, queue) in queues.drain() {
            // Cleanup each queue
            let _ = queue.purge().await;
        }
        
        // Close connection pool
        if let Some(ref pool) = self.connection_pool {
            pool.close_all().await;
        }
        
        Ok(())
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiQueueManager {
    #[pyo3(name = "create_queue")]
    fn create_queue_impl(&self, name: String) -> PyResult<String> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.create_queue(&name)
                .await
                .map(|_| name)
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create queue: {e}")))
        })
    }
    
    #[pyo3(name = "get_queue")]
    fn get_queue_impl(&self, name: String) -> PyResult<Option<()>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        Ok(rt.block_on(async {
            self.get_queue(&name).await.map(|_| ())
        }))
    }
    
    #[pyo3(name = "list_queues")]
    fn list_queues_impl(&self) -> PyResult<Vec<String>> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        Ok(rt.block_on(async {
            self.list_queues().await
        }))
    }
    
    #[pyo3(name = "delete_queue")]
    fn delete_queue_impl(&self, name: String) -> PyResult<()> {
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create runtime: {}", e))
        })?;
        
        rt.block_on(async {
            self.delete_queue(&name)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to delete queue: {e}")))
        })
    }
}
