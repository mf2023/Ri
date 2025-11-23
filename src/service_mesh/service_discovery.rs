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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::core::{DMSResult, DMSError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSServiceInstance {
    pub id: String,
    pub service_name: String,
    pub host: String,
    pub port: u16,
    pub metadata: HashMap<String, String>,
    pub registered_at: SystemTime,
    pub last_heartbeat: SystemTime,
    pub status: DMSServiceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DMSServiceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Unhealthy,
}

#[derive(Debug, Clone)]
pub struct DMSServiceRegistry {
    services: Arc<RwLock<HashMap<String, Vec<DMSServiceInstance>>>>,
    instance_index: Arc<RwLock<HashMap<String, DMSServiceInstance>>>,
}

impl DMSServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            instance_index: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_service(&self, instance: DMSServiceInstance) -> DMSResult<()> {
        let mut services = self.services.write().await;
        let mut instance_index = self.instance_index.write().await;

        services.entry(instance.service_name.clone())
            .or_insert_with(Vec::new)
            .push(instance.clone());

        instance_index.insert(instance.id.clone(), instance);

        Ok(())
    }

    pub async fn deregister_service(&self, instance_id: &str) -> DMSResult<()> {
        let mut instance_index = self.instance_index.write().await;
        
        if let Some(instance) = instance_index.remove(instance_id) {
            let mut services = self.services.write().await;
            if let Some(instances) = services.get_mut(&instance.service_name) {
                instances.retain(|inst| inst.id != instance_id);
                
                if instances.is_empty() {
                    services.remove(&instance.service_name);
                }
            }
        }

        Ok(())
    }

    pub async fn get_service_instances(&self, service_name: &str) -> DMSResult<Vec<DMSServiceInstance>> {
        let services = self.services.read().await;
        let instances = services.get(service_name)
            .cloned()
            .unwrap_or_default();

        Ok(instances)
    }

    pub async fn get_all_services(&self) -> DMSResult<Vec<String>> {
        let services = self.services.read().await;
        let service_names: Vec<String> = services.keys().cloned().collect();
        Ok(service_names)
    }

    pub async fn update_instance_status(&self, instance_id: &str, status: DMSServiceStatus) -> DMSResult<()> {
        let mut instance_index = self.instance_index.write().await;
        
        if let Some(instance) = instance_index.get_mut(instance_id) {
            instance.status = status;
            instance.last_heartbeat = SystemTime::now();
        }

        Ok(())
    }

    pub async fn get_healthy_instances(&self, service_name: &str) -> DMSResult<Vec<DMSServiceInstance>> {
        let instances = self.get_service_instances(service_name).await?;
        let healthy_instances: Vec<DMSServiceInstance> = instances
            .into_iter()
            .filter(|inst| inst.status == DMSServiceStatus::Running)
            .collect();

        Ok(healthy_instances)
    }

    pub async fn cleanup_expired_instances(&self, expiration_duration: Duration) -> DMSResult<()> {
        let now = SystemTime::now();
        let mut expired_instances = Vec::new();

        {
            let instance_index = self.instance_index.read().await;
            for (id, instance) in instance_index.iter() {
                if let Ok(elapsed) = now.duration_since(instance.last_heartbeat) {
                    if elapsed > expiration_duration {
                        expired_instances.push(id.clone());
                    }
                }
            }
        }

        for instance_id in expired_instances {
            self.deregister_service(&instance_id).await?;
        }

        Ok(())
    }
}

pub struct DMSServiceDiscovery {
    enabled: bool,
    registry: Arc<DMSServiceRegistry>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    cleanup_interval: Duration,
}

impl DMSServiceDiscovery {
    pub fn _Fnew(enabled: bool) -> Self {
        Self {
            enabled,
            registry: Arc::new(DMSServiceRegistry::new()),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            cleanup_interval: Duration::from_secs(60),
        }
    }

    pub async fn register_service(
        &self,
        service_name: &str,
        host: &str,
        port: u16,
        metadata: HashMap<String, String>,
    ) -> DMSResult<String> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        let instance_id = format!("{}:{}:{}", service_name, host, port);
        let instance = DMSServiceInstance {
            id: instance_id.clone(),
            service_name: service_name.to_string(),
            host: host.to_string(),
            port,
            metadata,
            registered_at: SystemTime::now(),
            last_heartbeat: SystemTime::now(),
            status: DMSServiceStatus::Starting,
        };

        self.registry.register_service(instance).await?;
        Ok(instance_id)
    }

    pub async fn deregister_service(&self, instance_id: &str) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        self.registry.deregister_service(instance_id).await
    }

    pub async fn discover_service(&self, service_name: &str) -> DMSResult<Vec<DMSServiceInstance>> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        self.registry.get_healthy_instances(service_name).await
    }

    pub async fn update_heartbeat(&self, instance_id: &str) -> DMSResult<()> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        self.registry.update_instance_status(instance_id, DMSServiceStatus::Running).await
    }

    pub async fn get_service_instances(&self, service_name: &str) -> DMSResult<Vec<DMSServiceInstance>> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        self.registry.get_service_instances(service_name).await
    }

    pub async fn get_all_services(&self) -> DMSResult<Vec<String>> {
        if !self.enabled {
            return Err(DMSError::ServiceMesh("Service discovery is disabled".to_string()));
        }

        self.registry.get_all_services().await
    }

    pub async fn start_background_tasks(&self) -> DMSResult<()> {
        if !self.enabled {
            return Ok(());
        }

        let registry_clone = Arc::clone(&self.registry);

        let cleanup_interval = self.cleanup_interval;

        let cleanup_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                if let Err(e) = registry_clone.cleanup_expired_instances(Duration::from_secs(300)).await {
                    eprintln!("Failed to cleanup expired instances: {}", e);
                }
            }
        });

        let mut tasks = self.background_tasks.write().await;
        tasks.push(cleanup_task);

        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> DMSResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn _Fhealth_check(&self) -> DMSResult<bool> {
        Ok(self.enabled)
    }
}