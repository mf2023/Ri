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

use crate::core::DMSResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DMSLoadBalancerStrategy {
    RoundRobin,
    WeightedRoundRobin,
    LeastConnections,
    Random,
    IpHash,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DMSBackendServer {
    pub id: String,
    pub url: String,
    pub weight: u32,
    pub max_connections: usize,
    pub health_check_path: String,
    pub is_healthy: bool,
}

impl DMSBackendServer {
    pub fn _Fnew(id: String, url: String) -> Self {
        Self {
            id,
            url,
            weight: 1,
            max_connections: 100,
            health_check_path: "/health".to_string(),
            is_healthy: true,
        }
    }

    pub fn _Fwith_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    pub fn _Fwith_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }

    pub fn _Fwith_health_check_path(mut self, path: String) -> Self {
        self.health_check_path = path;
        self
    }
}

#[derive(Debug)]
struct ServerStats {
    active_connections: AtomicUsize,
    total_requests: AtomicUsize,
    failed_requests: AtomicUsize,
    response_time_ms: AtomicUsize,
    last_used: RwLock<Instant>,
}

impl ServerStats {
    fn _Fnew() -> Self {
        Self {
            active_connections: AtomicUsize::new(0),
            total_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            response_time_ms: AtomicUsize::new(0),
            last_used: RwLock::new(Instant::now()),
        }
    }

    fn _Fget_active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    fn _Fincrement_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        let mut last_used = self.last_used.blocking_write();
        *last_used = Instant::now();
    }

    fn _Fdecrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    fn _Frecord_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        self._Fdecrement_connections();
    }

    fn _Frecord_response_time(&self, response_time_ms: u64) {
        self.response_time_ms.store(response_time_ms as usize, Ordering::Relaxed);
    }

    fn _Fget_stats(&self) -> LoadBalancerServerStats {
        LoadBalancerServerStats {
            active_connections: self._Fget_active_connections(),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            response_time_ms: self.response_time_ms.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadBalancerServerStats {
    pub active_connections: usize,
    pub total_requests: usize,
    pub failed_requests: usize,
    pub response_time_ms: usize,
}

pub struct DMSLoadBalancer {
    strategy: DMSLoadBalancerStrategy,
    servers: RwLock<Vec<DMSBackendServer>>,
    server_stats: RwLock<HashMap<String, Arc<ServerStats>>>,
    round_robin_counter: AtomicUsize,
}

impl DMSLoadBalancer {
    pub fn _Fnew(strategy: DMSLoadBalancerStrategy) -> Self {
        Self {
            strategy,
            servers: RwLock::new(Vec::new()),
            server_stats: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(0),
        }
    }

    pub async fn _Fadd_server(&self, server: DMSBackendServer) {
        let mut servers = self.servers.write().await;
        let mut stats = self.server_stats.write().await;
        
        servers.push(server.clone());
        stats.insert(server.id.clone(), Arc::new(ServerStats::_Fnew()));
    }

    pub async fn _Fremove_server(&self, server_id: &str) -> bool {
        let mut servers = self.servers.write().await;
        let mut stats = self.server_stats.write().await;
        
        let initial_len = servers.len();
        servers.retain(|s| s.id != server_id);
        stats.remove(server_id);
        
        servers.len() < initial_len
    }

    pub async fn _Fget_healthy_servers(&self) -> Vec<DMSBackendServer> {
        let servers = self.servers.read().await;
        servers.iter()
            .filter(|s| s.is_healthy)
            .cloned()
            .collect()
    }

    pub async fn _Fselect_server(&self, client_ip: Option<&str>) -> DMSResult<DMSBackendServer> {
        let healthy_servers = self._Fget_healthy_servers().await;
        
        if healthy_servers.is_empty() {
            return Err(crate::core::DMSError::Other("No healthy servers available".to_string()));
        }

        let server = match self.strategy {
            DMSLoadBalancerStrategy::RoundRobin => self._Fselect_round_robin(&healthy_servers).await,
            DMSLoadBalancerStrategy::WeightedRoundRobin => self._Fselect_weighted_round_robin(&healthy_servers).await,
            DMSLoadBalancerStrategy::LeastConnections => self._Fselect_least_connections(&healthy_servers).await,
            DMSLoadBalancerStrategy::Random => self._Fselect_random(&healthy_servers),
            DMSLoadBalancerStrategy::IpHash => self._Fselect_ip_hash(&healthy_servers, client_ip),
        };

        if let Some(server) = server {
            // Increment connection count
            if let Some(stats) = self.server_stats.read().await.get(&server.id) {
                stats._Fincrement_connections();
            }
            Ok(server)
        } else {
            Err(crate::core::DMSError::Other("Failed to select server".to_string()))
        }
    }

    async fn _Fselect_round_robin(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        let counter = self.round_robin_counter.fetch_add(1, Ordering::Relaxed);
        let index = counter % servers.len();
        servers.get(index).cloned()
    }

    async fn _Fselect_weighted_round_robin(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        // Simple weighted round robin implementation
        let total_weight: u32 = servers.iter().map(|s| s.weight).sum();
        let counter = self.round_robin_counter.fetch_add(1, Ordering::Relaxed);
        let mut weighted_index = counter % total_weight as usize;
        
        for server in servers {
            if weighted_index < server.weight as usize {
                return Some(server.clone());
            }
            weighted_index -= server.weight as usize;
        }
        
        servers.first().cloned()
    }

    async fn _Fselect_least_connections(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        let stats = self.server_stats.read().await;
        
        let mut best_server = None;
        let mut min_connections = usize::MAX;
        
        for server in servers {
            if let Some(server_stats) = stats.get(&server.id) {
                let connections = server_stats._Fget_active_connections();
                if connections < min_connections && connections < server.max_connections {
                    min_connections = connections;
                    best_server = Some(server.clone());
                }
            }
        }
        
        best_server.or_else(|| servers.first().cloned())
    }

    fn _Fselect_random(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..servers.len());
        servers.get(index).cloned()
    }

    fn _Fselect_ip_hash(&self, servers: &[DMSBackendServer], client_ip: Option<&str>) -> Option<DMSBackendServer> {
        if let Some(ip) = client_ip {
            let hash = self._Fhash_ip(ip);
            let index = hash as usize % servers.len();
            servers.get(index).cloned()
        } else {
            self._Fselect_random(servers)
        }
    }

    fn _Fhash_ip(&self, ip: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        hasher.finish()
    }

    pub async fn _Frelease_server(&self, server_id: &str) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats._Fdecrement_connections();
        }
    }

    pub async fn _Frecord_server_failure(&self, server_id: &str) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats._Frecord_failure();
        }
        
        // Mark server as unhealthy if too many failures
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
            // Simple heuristic: mark unhealthy if failure rate > 50%
            if let Some(stats) = self.server_stats.read().await.get(server_id) {
                let total = stats.total_requests.load(Ordering::Relaxed);
                let failed = stats.failed_requests.load(Ordering::Relaxed);
                
                if total > 10 && (failed as f64 / total as f64) > 0.5 {
                    server.is_healthy = false;
                }
            }
        }
    }

    pub async fn _Frecord_response_time(&self, server_id: &str, response_time_ms: u64) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats._Frecord_response_time(response_time_ms);
        }
    }

    pub async fn _Fget_server_stats(&self, server_id: &str) -> Option<LoadBalancerServerStats> {
        self.server_stats.read().await
            .get(server_id)
            .map(|stats| stats._Fget_stats())
    }

    pub async fn _Fget_all_stats(&self) -> HashMap<String, LoadBalancerServerStats> {
        let stats = self.server_stats.read().await;
        let mut result = HashMap::new();
        
        for (server_id, server_stats) in stats.iter() {
            result.insert(server_id.clone(), server_stats._Fget_stats());
        }
        
        result
    }

    pub async fn _Fmark_server_healthy(&self, server_id: &str, healthy: bool) {
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
            server.is_healthy = healthy;
        }
    }

    pub async fn _Fperform_health_check(&self, _server_id: &str) -> bool {
        // In a real implementation, this would perform an actual HTTP health check
        // For now, we'll just return true for demonstration
        true
    }

    pub fn _Fget_strategy(&self) -> &DMSLoadBalancerStrategy {
        &self.strategy
    }

    pub async fn _Fset_strategy(&mut self, strategy: DMSLoadBalancerStrategy) {
        self.strategy = strategy;
    }

    pub async fn _Fget_server_count(&self) -> usize {
        self.servers.read().await.len()
    }

    pub async fn _Fget_healthy_server_count(&self) -> usize {
        self._Fget_healthy_servers().await.len()
    }
}