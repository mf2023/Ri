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

#[cfg(feature = "etcd")]
use etcd_client::{Client, PutOptions};

use crate::core::{DMSResult, DMSError};

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
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

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
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
    #[cfg(feature = "etcd")]
    etcd_client: Option<Arc<Client>>,
    _etcd_prefix: String,
}

impl Default for DMSServiceRegistry {
    fn default() -> Self {
        Self::new(None, "/dms/services".to_string())
    }
}

impl DMSServiceRegistry {
    #[cfg(feature = "etcd")]
    pub fn new(etcd_client: Option<Client>, etcd_prefix: String) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            instance_index: Arc::new(RwLock::new(HashMap::new())),
            etcd_client: etcd_client.map(Arc::new),
            _etcd_prefix: etcd_prefix,
        }
    }
    
    #[cfg(not(feature = "etcd"))]
    pub fn new(_etcd_client: Option<()>, etcd_prefix: String) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            instance_index: Arc::new(RwLock::new(HashMap::new())),
            _etcd_prefix: etcd_prefix,
        }
    }

    pub async fn register_service(&self, instance: DMSServiceInstance) -> DMSResult<()> {
        // Update in-memory registry
        let mut services = self.services.write().await;
        let mut instance_index = self.instance_index.write().await;

        services.entry(instance.service_name.clone())
            .or_insert_with(Vec::new)
            .push(instance.clone());

        instance_index.insert(instance.id.clone(), instance.clone());
        
        // Persist to etcd if client is available
        #[cfg(feature = "etcd")]
        if let Some(_client) = &self.etcd_client {
            let key = format!("{}/{}/{}", self.etcd_prefix, instance.service_name, instance.id);
            let value = serde_json::to_string(&instance)?;
            
            // Set with TTL of 5 minutes (300 seconds)
            Arc::make_mut(&mut self.etcd_client.as_ref().unwrap()).put(key, value, Some(PutOptions::new().with_lease(300)))
                .await
                .map_err(|e| DMSError::ServiceMesh(format!("Failed to register service in etcd: {}", e)))?;
        }

        Ok(())
    }

    pub async fn deregister_service(&self, instance_id: &str) -> DMSResult<()> {
        let mut instance_index = self.instance_index.write().await;
        
        if let Some(instance) = instance_index.remove(instance_id) {
            // Update in-memory registry
            let mut services = self.services.write().await;
            if let Some(instances) = services.get_mut(&instance.service_name) {
                instances.retain(|inst| inst.id != instance_id);
                
                if instances.is_empty() {
                    services.remove(&instance.service_name);
                }
            }
            
            // Remove from etcd if client is available
            #[cfg(feature = "etcd")]
            if let Some(_client) = &self.etcd_client {
                let key = format!("{}/{}/{}", self.etcd_prefix, instance.service_name, instance_id);
                Arc::make_mut(&mut self.etcd_client.as_ref().unwrap()).delete(key, None)
                    .await
                    .map_err(|e| DMSError::ServiceMesh(format!("Failed to deregister service in etcd: {}", e)))?;
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
            
            // Update in etcd if client is available
            #[cfg(feature = "etcd")]
            if let Some(_client) = &self.etcd_client {
                let key = format!("{}/{}/{}", self.etcd_prefix, instance.service_name, instance_id);
                let value = serde_json::to_string(instance)?;
                Arc::make_mut(&mut self.etcd_client.as_ref().unwrap()).put(key, value, None)
                    .await
                    .map_err(|e| DMSError::ServiceMesh(format!("Failed to update service status in etcd: {}", e)))?;
            }
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
    
    /// Sync registry from etcd
    #[cfg(feature = "etcd")]
    pub async fn sync_from_etcd(&self) -> DMSResult<()> {
        if let Some(client) = &self.etcd_client {
            // List all services from etcd
            let prefix = format!("{}/", self.etcd_prefix);
            let response = client.get(prefix, Some(etcd_client::GetOptions::new().with_prefix()))
                .await
                .map_err(|e| DMSError::ServiceMesh(format!("Failed to sync from etcd: {}", e)))?;
            
            // Clear current in-memory registry
            let mut services = self.services.write().await;
            let mut instance_index = self.instance_index.write().await;
            services.clear();
            instance_index.clear();
            
            // Reconstruct from etcd data
            for kv in response.kvs() {
                let instance: DMSServiceInstance = serde_json::from_slice(kv.value())?;
                
                services.entry(instance.service_name.clone())
                    .or_insert_with(Vec::new)
                    .push(instance.clone());
                
                instance_index.insert(instance.id.clone(), instance);
            }
        }
        
        Ok(())
    }
    
    /// Start etcd watcher to sync changes in real-time
    #[cfg(feature = "etcd")]
    pub async fn start_etcd_watcher(&self) -> DMSResult<JoinHandle<()>> {
        if let Some(client) = &self.etcd_client {
            let client = client.clone();
            let prefix = self.etcd_prefix.clone();
            let registry = self.clone();
            
            let handle = tokio::spawn(async move {
                let prefix = format!("{}/", prefix);
                
                loop {
                    let mut watcher = client.watch(prefix, Some(etcd_client::WatchOptions::new().with_prefix()))
                        .await
                        .expect("Failed to start etcd watcher");
                    
                    while let Some(res) = watcher.message().await {
                        match res {
                            Ok(watch_response) => {
                                for event in watch_response.events() {
                                    match event.kind() {
                                        etcd_client::EventKind::Put(kv) => {
                                            // Update or add instance
                                            if let Ok(instance) = serde_json::from_slice(kv.value()) {
                                                let _ = registry.register_service(instance).await;
                                            }
                                        },
                                        etcd_client::EventKind::Delete(_) => {
                                            // Delete instance - we'd need to parse the key to get instance ID
                                            // This is simplified for now
                                        },
                                    }
                                }
                            },
                            Err(e) => {
                                log::warn!("Etcd watch error: {}", e);
                                // Sleep and retry
                                tokio::time::sleep(Duration::from_secs(1)).await;
                                break;
                            },
                        }
                    }
                }
            });
            
            Ok(handle)
        } else {
            Err(DMSError::ServiceMesh("No etcd client available".to_string()))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSEtcdConfig {
    pub endpoints: Vec<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub prefix: String,
}

impl Default for DMSEtcdConfig {
    fn default() -> Self {
        Self {
            endpoints: vec!["http://localhost:2379".to_string()],
            username: None,
            password: None,
            prefix: "/dms/services".to_string(),
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct DMSServiceDiscovery {
    enabled: bool,
    registry: Arc<DMSServiceRegistry>,
    background_tasks: Arc<RwLock<Vec<JoinHandle<()>>>>,
    cleanup_interval: Duration,
    _etcd_config: Option<DMSEtcdConfig>,
}

impl DMSServiceDiscovery {
    pub fn new(enabled: bool) -> Self {
        Self {
            enabled,
            registry: Arc::new(DMSServiceRegistry::new(None::<()>, "/dms/services".to_string())),
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            cleanup_interval: Duration::from_secs(60),
            _etcd_config: None,
        }
    }
    
    #[cfg(feature = "etcd")]
    pub async fn new_with_etcd(enabled: bool, etcd_config: DMSEtcdConfig) -> DMSResult<Self> {
        // Create etcd client
        let client = Client::connect(etcd_config.endpoints.clone(), None)
            .await
            .map_err(|e| DMSError::ServiceMesh(format!("Failed to connect to etcd: {}", e)))?;
        
        let registry = Arc::new(DMSServiceRegistry::new(Some(client), etcd_config.prefix.clone()));
        
        let discovery = Self {
            enabled,
            registry,
            background_tasks: Arc::new(RwLock::new(Vec::new())),
            cleanup_interval: Duration::from_secs(60),
            _etcd_config: Some(etcd_config),
        };
        
        // Sync from etcd on startup
        discovery.registry.sync_from_etcd().await?;
        
        Ok(discovery)
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

        let instance_id = format!("{service_name}:{host}:{port}");
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

        // Start cleanup task
        let cleanup_task = tokio::spawn(async move {
            let mut interval = tokio::time::interval(cleanup_interval);
            loop {
                interval.tick().await;
                if let Err(e) = registry_clone.cleanup_expired_instances(Duration::from_secs(300)).await {
                    log::warn!("Failed to cleanup expired instances: {e}");
                }
            }
        });

        let mut tasks = self.background_tasks.write().await;
        tasks.push(cleanup_task);
        
        // Start etcd watcher if etcd is configured
        #[cfg(feature = "etcd")]
        if self.etcd_config.is_some() {
            let watcher_task = self.registry.start_etcd_watcher().await?;
            tasks.push(watcher_task);
            
            // Start periodic sync from etcd (every 30 seconds)
            let registry_clone = Arc::clone(&self.registry);
            let sync_task = tokio::spawn(async move {
                let mut interval = tokio::time::interval(Duration::from_secs(30));
                loop {
                    interval.tick().await;
                    if let Err(e) = registry_clone.sync_from_etcd().await {
                        log::warn!("Failed to sync from etcd: {e}");
                    }
                }
            });
            tasks.push(sync_task);
        }

        Ok(())
    }

    pub async fn stop_background_tasks(&self) -> DMSResult<()> {
        let mut tasks = self.background_tasks.write().await;
        for task in tasks.drain(..) {
            task.abort();
        }
        Ok(())
    }

    pub async fn health_check(&self) -> DMSResult<bool> {
        Ok(self.enabled)
    }
}
