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

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;
use async_trait::async_trait;
use crate::core::{DMSResult, _CAsyncServiceModule, DMSServiceContext};
use crate::queue::{DMSQueue, DMSQueueConfig, QueueBackendType};

pub struct DMSQueueModule {
    queue_manager: Arc<DMSQueueManager>,
}

impl DMSQueueModule {
    pub async fn _Fnew(config: DMSQueueConfig) -> DMSResult<Self> {
        let queue_manager = Arc::new(DMSQueueManager::_Fnew(config).await?);
        Ok(Self { queue_manager })
    }

    pub fn _Fqueue_manager(&self) -> Arc<DMSQueueManager> {
        self.queue_manager.clone()
    }
}

#[async_trait]
impl _CAsyncServiceModule for DMSQueueModule {
    async fn _Finit(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        self.queue_manager._Finit().await
    }

    async fn _Fafter_shutdown(&mut self, _ctx: &mut DMSServiceContext) -> DMSResult<()> {
        self.queue_manager._Fshutdown().await
    }

    fn _Fname(&self) -> &str {
        "dms-queue"
    }

    fn _Fis_critical(&self) -> bool {
        false
    }
}

pub struct DMSQueueManager {
    config: DMSQueueConfig,
    queues: Arc<RwLock<HashMap<String, Arc<dyn DMSQueue>>>>,
}

impl DMSQueueManager {
    pub async fn _Fnew(config: DMSQueueConfig) -> DMSResult<Self> {
        Ok(Self {
            config,
            queues: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn _Finit(&self) -> DMSResult<()> {
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

    pub async fn _Fcreate_queue(&self, name: &str) -> DMSResult<Arc<dyn DMSQueue>> {
        let mut queues = self.queues.write().await;
        
        if let Some(queue) = queues.get(name) {
            return Ok(queue.clone());
        }

        let queue = self._Fcreate_backend_queue(name).await?;
        queues.insert(name.to_string(), queue.clone());
        
        Ok(queue)
    }

    async fn _Fcreate_backend_queue(&self, name: &str) -> DMSResult<Arc<dyn DMSQueue>> {
        match self.config.backend_type {
            QueueBackendType::Memory => {
                Ok(Arc::new(crate::queue::backends::DMSMemoryQueue::_Fnew(name)))
            }
            QueueBackendType::RabbitMQ => {
                Ok(Arc::new(crate::queue::backends::DMSRabbitMQQueue::_Fnew(name, &self.config.connection_string).await?))
            }
            QueueBackendType::Kafka => {
                return Err(crate::core::DMSError::Config("Kafka support temporarily disabled due to build dependencies".to_string()));
            }
            QueueBackendType::Redis => {
                Ok(Arc::new(crate::queue::backends::DMSRedisQueue::_Fnew(name, &self.config.connection_string).await?))
            }
        }
    }

    pub async fn _Fget_queue(&self, name: &str) -> Option<Arc<dyn DMSQueue>> {
        let queues = self.queues.read().await;
        queues.get(name).cloned()
    }

    pub async fn _Flist_queues(&self) -> Vec<String> {
        let queues = self.queues.read().await;
        queues.keys().cloned().collect()
    }

    pub async fn _Fdelete_queue(&self, name: &str) -> DMSResult<()> {
        let mut queues = self.queues.write().await;
        if let Some(queue) = queues.remove(name) {
            queue._Fdelete().await?;
        }
        Ok(())
    }

    pub async fn _Fshutdown(&self) -> DMSResult<()> {
        let mut queues = self.queues.write().await;
        for (_, queue) in queues.drain() {
            // Cleanup each queue
            let _ = queue._Fpurge().await;
        }
        Ok(())
    }
}