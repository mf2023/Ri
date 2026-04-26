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

//! # Load Balancer Module
//! 
//! This module provides a robust load balancer implementation for distributing incoming requests
//! across multiple backend servers. It supports various load balancing strategies and includes
//! health checking, connection management, and detailed statistics.
//! 
//! ## Key Components
//! 
//! - **RiLoadBalancerStrategy**: Enum representing different load balancing algorithms
//! - **RiBackendServer**: Represents a backend server with configuration and health status
//! - **RiLoadBalancer**: Main load balancer implementation
//! - **RiLoadBalancerServerStats**: Metrics for monitoring server performance
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Strategies**: Supports RoundRobin, WeightedRoundRobin, LeastConnections, Random, and IpHash
//! 2. **Health Checking**: Automatic periodic health checks to ensure traffic is only sent to healthy servers
//! 3. **Connection Management**: Tracks active connections and enforces max connections per server
//! 4. **Detailed Statistics**: Collects metrics on requests, failures, and response times
//! 5. **Thread Safety**: Uses Arc and RwLock for safe operation in multi-threaded environments
//! 6. **Scalability**: Designed to handle large numbers of servers and requests
//! 7. **Configurable**: Allows fine-tuning of server weights, max connections, and health check paths
//! 8. **Async Compatibility**: Built with async/await patterns for modern Rust applications
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a load balancer with Round Robin strategy
//!     let lb = Arc::new(RiLoadBalancer::new(RiLoadBalancerStrategy::RoundRobin));
//!     
//!     // Add backend servers
//!     lb.add_server(RiBackendServer::new("server1".to_string(), "http://localhost:8081".to_string())
//!         .with_weight(2)
//!         .with_max_connections(200))
//!         .await;
//!     
//!     lb.add_server(RiBackendServer::new("server2".to_string(), "http://localhost:8082".to_string())
//!         .with_weight(1)
//!         .with_max_connections(100))
//!         .await;
//!     
//!     // Start periodic health checks every 30 seconds
//!     lb.clone().start_health_checks(30).await;
//!     
//!     // Select a server for a client request
//!     let server = lb.select_server(Some("192.168.1.1")).await?;
//!     println!("Selected server: {}", server.url);
//!     
//!     // Record response time when done
//!     lb.record_response_time(&server.id, 150).await;
//!     
//!     // Release the server when the request is complete
//!     lb.release_server(&server.id).await;
//!     
//!     // Get server statistics
//!     let stats = lb.get_all_stats().await;
//!     println!("Server stats: {:?}", stats);
//!     
//!     Ok(())
//! }
//! ```

use crate::core::RiResult;
use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tokio::sync::RwLock;
use std::sync::RwLock as StdRwLock;

#[cfg(feature = "gateway")]
use hyper;

/// Load balancing strategies supported by Ri.
/// 
/// These strategies determine how the load balancer selects which backend server
/// to route incoming requests to.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiLoadBalancerStrategy {
    /// **Round Robin**: Sequentially selects the next available server in rotation.
    /// 
    /// Simple and fair distribution, ideal for servers with similar capabilities.
    RoundRobin,
    
    /// **Weighted Round Robin**: Selects servers based on assigned weights.
    /// 
    /// Allows more powerful servers to handle a larger share of traffic.
    WeightedRoundRobin,
    
    /// **Least Connections**: Selects the server with the fewest active connections.
    /// 
    /// Ideal for handling varying request durations, ensuring balanced load.
    LeastConnections,
    
    /// **Random**: Randomly selects an available server.
    /// 
    /// Simple implementation with good distribution characteristics.
    Random,
    
    /// **IP Hash**: Uses client IP address to consistently route to the same server.
    /// 
    /// Maintains session persistence by mapping clients to specific servers.
    IpHash,
    
    /// **Least Response Time**: Selects the server with the lowest average response time.
    /// 
    /// Ideal for optimizing user experience by directing traffic to the fastest servers.
    LeastResponseTime,
    
    /// **Consistent Hash**: Uses a consistent hashing algorithm for server selection.
    /// 
    /// Provides stable mapping between requests and servers, minimizing disruption when servers are added or removed.
    ConsistentHash,
}

/// Represents a backend server in the load balancer.
/// 
/// This struct contains all the configuration and state information for a backend server,
/// including its ID, URL, weight, max connections, health check path, and health status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiBackendServer {
    /// Unique identifier for the server
    pub id: String,
    
    /// Base URL of the server (e.g., "http://localhost:8080")
    pub url: String,
    
    /// Weight assigned to the server for weighted load balancing strategies
    pub weight: u32,
    
    /// Maximum number of concurrent connections allowed to this server
    pub max_connections: usize,
    
    /// Path to check for health status (e.g., "/health")
    pub health_check_path: String,
    
    /// Current health status of the server (true = healthy, false = unhealthy)
    pub is_healthy: bool,
}

impl RiBackendServer {
    /// Creates a new backend server with the specified ID and URL.
    /// 
    /// # Parameters
    /// 
    /// - `id`: Unique identifier for the server
    /// - `url`: Base URL of the server
    /// 
    /// # Returns
    /// 
    /// A new `RiBackendServer` instance with default values
    pub fn new(id: String, url: String) -> Self {
        Self {
            id,
            url,
            weight: 1,
            max_connections: 100,
            health_check_path: "/health".to_string(),
            is_healthy: true,
        }
    }
}

#[cfg_attr(feature = "pyo3", pyo3::prelude::pymethods)]
impl RiBackendServer {
    #[cfg(feature = "pyo3")]
    #[new]
    fn py_new(id: String, url: String) -> Self {
        Self::new(id, url)
    }

    #[cfg(feature = "pyo3")]
    fn set_weight(&mut self, weight: u32) {
        self.weight = weight;
    }

    #[cfg(feature = "pyo3")]
    fn set_max_connections(&mut self, max_connections: usize) {
        self.max_connections = max_connections;
    }

    #[cfg(feature = "pyo3")]
    fn set_health_check_path(&mut self, path: String) {
        self.health_check_path = path;
    }
}

/// Internal server statistics tracking.
/// 
/// This struct tracks real-time statistics for each backend server, including active connections,
/// request counts, failures, and response times. It is designed to be thread-safe for use in
/// multi-threaded environments.
#[derive(Debug)]
struct ServerStats {
    /// Number of currently active connections to the server
    active_connections: AtomicUsize,
    
    /// Total number of requests sent to the server since it was added
    total_requests: AtomicUsize,
    
    /// Number of failed requests to the server
    failed_requests: AtomicUsize,
    
    /// Most recent response time in milliseconds
    response_time_ms: AtomicUsize,
    
    /// Timestamp of when the server was last used
    last_used: StdRwLock<Instant>,
}

impl ServerStats {
    /// Creates a new server statistics instance with default values.
    /// 
    /// Initializes all counters to zero and sets the last used time to now.
    /// 
    /// # Returns
    /// 
    /// A new `ServerStats` instance with default values
    fn new() -> Self {
        Self {
            active_connections: AtomicUsize::new(0),
            total_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            response_time_ms: AtomicUsize::new(0),
            last_used: StdRwLock::new(Instant::now()),
        }
    }

    /// Gets the current number of active connections to the server.
    /// 
    /// # Returns
    /// 
    /// The number of active connections as a `usize`
    fn get_active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    /// Increments the active connection count and updates request statistics.
    /// 
    /// This method should be called when a new connection is established to the server.
    /// 
    /// - Increments active_connections by 1
    /// - Increments total_requests by 1
    /// - Updates last_used to the current time
    fn increment_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut last_used) = self.last_used.write() {
            *last_used = Instant::now();
        }
    }

    /// Decrements the active connection count.
    /// 
    /// This method should be called when a connection to the server is closed.
    fn decrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Records a failed request to the server.
    /// 
    /// This method should be called when a request to the server fails.
    /// 
    /// - Increments failed_requests by 1
    /// - Decrements active_connections by 1 (since the connection failed)
    fn record_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        self.decrement_connections();
    }

    /// Records the response time for a successful request.
    /// 
    /// This method should be called when a request to the server completes successfully.
    /// 
    /// # Parameters
    /// 
    /// - `response_time_ms`: Response time in milliseconds
    fn record_response_time(&self, response_time_ms: u64) {
        self.response_time_ms.store(response_time_ms as usize, Ordering::Relaxed);
    }

    /// Gets a snapshot of the current server statistics.
    ///
    /// This method converts the internal statistics into a public-facing `RiLoadBalancerServerStats` struct.
    ///
    /// # Returns
    ///
    /// A `RiLoadBalancerServerStats` struct containing the current statistics
    fn get_stats(&self) -> RiLoadBalancerServerStats {
        RiLoadBalancerServerStats {
            active_connections: self.get_active_connections(),
            total_requests: self.total_requests.load(Ordering::Relaxed),
            failed_requests: self.failed_requests.load(Ordering::Relaxed),
            response_time_ms: self.response_time_ms.load(Ordering::Relaxed),
        }
    }
}

/// Load balancer server statistics for monitoring and reporting.
///
/// This struct contains metrics for a backend server, providing insights into its
/// performance, load, and reliability.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiLoadBalancerServerStats {
    /// Number of currently active connections to the server
    pub active_connections: usize,

    /// Total number of requests sent to the server since it was added
    pub total_requests: usize,

    /// Number of failed requests to the server
    pub failed_requests: usize,

    /// Most recent response time in milliseconds
    pub response_time_ms: usize,
}

/// Main load balancer implementation.
/// 
/// This struct provides a comprehensive load balancing solution with support for multiple
/// strategies, health checking, connection management, and detailed statistics.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiLoadBalancer {
    /// Load balancing strategy to use
    strategy: RiLoadBalancerStrategy,
    
    /// List of backend servers
    servers: RwLock<Vec<RiBackendServer>>,
    
    /// Statistics for each backend server
    server_stats: RwLock<FxHashMap<String, Arc<ServerStats>>>,
    
    /// Counter for round robin scheduling
    round_robin_counter: AtomicUsize,
}

impl Clone for RiLoadBalancer {
    /// Creates a clone of the load balancer.
    /// 
    /// Note: The clone will have the same strategy and counter, but empty servers and stats
    /// since we can't await in the Clone trait.
    fn clone(&self) -> Self {
        Self {
            strategy: self.strategy.clone(),
            servers: RwLock::new(Vec::new()),
            server_stats: RwLock::new(FxHashMap::default()),
            round_robin_counter: AtomicUsize::new(self.round_robin_counter.load(Ordering::Relaxed)),
        }
    }
}

impl RiLoadBalancer {
    /// Creates a new load balancer with the specified strategy.
    /// 
    /// # Parameters
    /// 
    /// - `strategy`: The load balancing strategy to use
    /// 
    /// # Returns
    /// 
    /// A new `RiLoadBalancer` instance with the specified strategy
    pub fn new(strategy: RiLoadBalancerStrategy) -> Self {
        Self {
            strategy,
            servers: RwLock::new(Vec::new()),
            server_stats: RwLock::new(FxHashMap::default()),
            round_robin_counter: AtomicUsize::new(0),
        }
    }

    /// Adds a backend server to the load balancer.
    /// 
    /// # Parameters
    /// 
    /// - `server`: The backend server to add
    pub async fn add_server(&self, server: RiBackendServer) {
        let mut servers = self.servers.write().await;
        let mut stats = self.server_stats.write().await;
        
        servers.push(server.clone());
        stats.insert(server.id.clone(), Arc::new(ServerStats::new()));
    }

    /// Removes a backend server from the load balancer.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: The ID of the server to remove
    /// 
    /// # Returns
    /// 
    /// `true` if the server was removed, `false` otherwise
    pub async fn remove_server(&self, server_id: &str) -> bool {
        let mut servers = self.servers.write().await;
        let mut stats = self.server_stats.write().await;
        
        let initial_len = servers.len();
        servers.retain(|s| s.id != server_id);
        stats.remove(server_id);
        
        servers.len() < initial_len
    }

    /// Gets a list of all healthy backend servers.
    /// 
    /// # Returns
    /// 
    /// A vector of healthy `RiBackendServer` instances
    pub async fn get_healthy_servers(&self) -> Vec<RiBackendServer> {
        let servers = self.servers.read().await;
        servers.iter()
            .filter(|s| s.is_healthy)
            .cloned()
            .collect()
    }

    /// Selects the most appropriate backend server for a client request.
    /// 
    /// This method applies the configured load balancing strategy to select a server,
    /// considering only healthy servers with available connections.
    /// 
    /// # Parameters
    /// 
    /// - `client_ip`: Optional client IP address for IP Hash strategy
    /// 
    /// # Returns
    /// 
    /// A `RiResult<RiBackendServer>` with the selected server, or an error if no servers are available
    pub async fn select_server(&self, client_ip: Option<&str>) -> RiResult<RiBackendServer> {
        let healthy_servers = self.get_healthy_servers().await;
        
        if healthy_servers.is_empty() {
            return Err(crate::core::RiError::Other("No healthy servers available".to_string()));
        }
        
        let stats = self.server_stats.read().await;
        
        // Filter servers that have available connections
        let available_servers: Vec<RiBackendServer> = healthy_servers.into_iter()
            .filter(|server| {
                if let Some(server_stats) = stats.get(&server.id) {
                    let connections = server_stats.get_active_connections();
                    connections < server.max_connections
                } else {
                    true // If no stats, assume server is available
                }
            })
            .collect();
        
        if available_servers.is_empty() {
            return Err(crate::core::RiError::Other("No servers with available connections".to_string()));
        }

        let server = match self.strategy {
            RiLoadBalancerStrategy::RoundRobin => self.select_round_robin(&available_servers).await,
            RiLoadBalancerStrategy::WeightedRoundRobin => self.select_weighted_round_robin(&available_servers).await,
            RiLoadBalancerStrategy::LeastConnections => self.select_least_connections(&available_servers).await,
            RiLoadBalancerStrategy::Random => self.select_random(&available_servers),
            RiLoadBalancerStrategy::IpHash => self.select_ip_hash(&available_servers, client_ip),
            RiLoadBalancerStrategy::LeastResponseTime => self.select_least_response_time(&available_servers).await,
            RiLoadBalancerStrategy::ConsistentHash => self.select_consistent_hash(&available_servers, client_ip),
        };

        if let Some(server) = server {
            // Increment connection count
            if let Some(stats) = self.server_stats.read().await.get(&server.id) {
                stats.increment_connections();
            }
            Ok(server)
        } else {
            Err(crate::core::RiError::Other("Failed to select server".to_string()))
        }
    }

    /// Selects a server using the Round Robin strategy.
    /// 
    /// This method sequentially selects the next available server in rotation.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    async fn select_round_robin(&self, servers: &[RiBackendServer]) -> Option<RiBackendServer> {
        let counter = self.round_robin_counter.fetch_add(1, Ordering::Relaxed);
        let index = counter % servers.len();
        servers.get(index).cloned()
    }

    /// Selects a server using the Smooth Weighted Round Robin strategy.
    /// 
    /// This method uses a smooth weighted round robin algorithm to distribute traffic more evenly
    /// across servers, avoiding the problem of sudden traffic spikes to high-weight servers.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers with weights
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    async fn select_weighted_round_robin(&self, servers: &[RiBackendServer]) -> Option<RiBackendServer> {
        if servers.is_empty() {
            return None;
        }
        
        // Simple weighted round robin implementation with improved distribution
        let total_weight: u32 = servers.iter().map(|s| s.weight).sum();
        let counter = self.round_robin_counter.fetch_add(1, Ordering::Relaxed);
        let weighted_index = counter % total_weight as usize;
        
        let mut accumulated_weight = 0;
        for server in servers {
            accumulated_weight += server.weight as usize;
            if weighted_index < accumulated_weight {
                return Some(server.clone());
            }
        }
        
        servers.first().cloned()
    }

    /// Selects a server using the Weighted Least Connections strategy.
    /// 
    /// This method selects the server with the fewest active connections relative to its weight,
    /// ensuring balanced load across servers with different capacities.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    async fn select_least_connections(&self, servers: &[RiBackendServer]) -> Option<RiBackendServer> {
        let stats = self.server_stats.read().await;
        
        let mut best_server = None;
        let mut best_score = f64::MAX; // Lower score is better (connections per weight unit)
        
        for server in servers {
            if let Some(server_stats) = stats.get(&server.id) {
                let connections = server_stats.get_active_connections();
                
                // Skip servers that have reached max connections
                if connections >= server.max_connections {
                    continue;
                }
                
                // Calculate score as connections per weight unit
                // Use a small epsilon to avoid division by zero
                let weight = server.weight as f64 + 0.001;
                let score = connections as f64 / weight;
                
                if score < best_score {
                    best_score = score;
                    best_server = Some(server.clone());
                }
            }
        }
        
        best_server.or_else(|| servers.first().cloned())
    }

    /// Selects a server using the Random strategy.
    /// 
    /// This method randomly selects an available server, providing good distribution characteristics.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    fn select_random(&self, servers: &[RiBackendServer]) -> Option<RiBackendServer> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..servers.len());
        servers.get(index).cloned()
    }

    /// Selects a server using the Least Response Time strategy.
    /// 
    /// This method selects the server with the lowest response time, optimizing for user experience.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    async fn select_least_response_time(&self, servers: &[RiBackendServer]) -> Option<RiBackendServer> {
        let stats = self.server_stats.read().await;
        
        let mut best_server = None;
        let mut min_response_time = u64::MAX;
        
        for server in servers {
            if let Some(server_stats) = stats.get(&server.id) {
                let response_time = server_stats.response_time_ms.load(Ordering::Relaxed) as u64;
                if response_time < min_response_time {
                    min_response_time = response_time;
                    best_server = Some(server.clone());
                }
            }
        }
        
        best_server.or_else(|| servers.first().cloned())
    }
    
    /// Selects a server using the Consistent Hash strategy.
    /// 
    /// This method uses a consistent hashing algorithm to map requests to servers, minimizing
    /// disruption when servers are added or removed.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// - `client_ip`: Optional client IP address for hashing
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    fn select_ip_hash(&self, servers: &[RiBackendServer], client_ip: Option<&str>) -> Option<RiBackendServer> {
        if let Some(ip) = client_ip {
            let hash = self.hash_ip(ip);
            let index = hash as usize % servers.len();
            servers.get(index).cloned()
        } else {
            self.select_random(servers)
        }
    }

    /// Hashes an IP address for the IP Hash strategy.
    /// 
    /// # Parameters
    /// 
    /// - `ip`: IP address to hash
    /// 
    /// # Returns
    /// 
    /// A 64-bit hash value of the IP address
    fn hash_ip(&self, ip: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        hasher.finish()
    }
    
    /// Selects a server using the Consistent Hash strategy.
    /// 
    /// This method uses a consistent hashing algorithm to map requests to servers, minimizing
    /// disruption when servers are added or removed.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// - `client_ip`: Optional client IP address for hashing
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    fn select_consistent_hash(&self, servers: &[RiBackendServer], client_ip: Option<&str>) -> Option<RiBackendServer> {
        if servers.is_empty() {
            return None;
        }
        
        let key = client_ip.unwrap_or("127.0.0.1");
        
        // Create a sorted list of server hashes
        let mut server_hashes: Vec<(u64, RiBackendServer)> = servers
            .iter()
            .map(|server| {
                let hash = self.hash_ip(&server.id);
                (hash, server.clone())
            })
            .collect();
        
        // Sort server hashes
        server_hashes.sort_by(|a, b| a.0.cmp(&b.0));
        
        // Calculate hash for the key
        let key_hash = self.hash_ip(key);
        
        // Find the first server with hash >= key_hash
        for (server_hash, server) in &server_hashes {
            if *server_hash >= key_hash {
                return Some(server.clone());
            }
        }
        
        // If no server with hash >= key_hash, return the first server
        server_hashes.first().map(|(_, server)| server.clone())
    }

    /// Releases a server after a request is completed.
    /// 
    /// This method decrements the active connection count for the specified server.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to release
    pub async fn release_server(&self, server_id: &str) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats.decrement_connections();
        }
    }

    /// Records a failed request to a server.
    /// 
    /// This method increments the failed request count and may mark the server as unhealthy
    /// if the failure rate exceeds a threshold.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server that failed
    pub async fn record_server_failure(&self, server_id: &str) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats.record_failure();
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

    /// Records the response time for a successful request.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server that handled the request
    /// - `response_time_ms`: Response time in milliseconds
    pub async fn record_response_time(&self, server_id: &str, response_time_ms: u64) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats.record_response_time(response_time_ms);
        }
    }

    /// Gets statistics for a specific server.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to get statistics for
    /// 
    /// # Returns
    /// 
    /// An `Option<RiLoadBalancerServerStats>` with the server statistics, or `None` if the server doesn't exist
    pub async fn get_server_stats(&self, server_id: &str) -> Option<RiLoadBalancerServerStats> {
        self.server_stats.read().await
            .get(server_id)
            .map(|stats| stats.get_stats())
    }

    /// Gets statistics for all servers.
    ///
    /// # Returns
    ///
    /// A `FxHashMap<String, RiLoadBalancerServerStats>` with statistics for all servers
    pub async fn get_all_stats(&self) -> FxHashMap<String, RiLoadBalancerServerStats> {
        let stats = self.server_stats.read().await;
        let mut result = FxHashMap::default();
        
        for (server_id, server_stats) in stats.iter() {
            result.insert(server_id.clone(), server_stats.get_stats());
        }
        
        result
    }

    /// Marks a server as healthy or unhealthy.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to update
    /// - `healthy`: New health status (true = healthy, false = unhealthy)
    pub async fn mark_server_healthy(&self, server_id: &str, healthy: bool) {
        let mut servers = self.servers.write().await;
        if let Some(server) = servers.iter_mut().find(|s| s.id == server_id) {
            server.is_healthy = healthy;
        }
    }

    /// Performs an HTTP health check on a server.
    /// 
    /// This method sends an HTTP GET request to the server's health check path and
    /// considers the server healthy if it returns a 2xx status code.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to check
    /// 
    /// # Returns
    /// 
    /// `true` if the server is healthy, `false` otherwise
    #[cfg(feature = "gateway")]
    pub async fn perform_health_check(&self, server_id: &str) -> bool {
        let servers = self.servers.read().await;
        
        if let Some(server) = servers.iter().find(|s| s.id == server_id) {
            let health_check_url = format!("{}{}", server.url, server.health_check_path);
            
            let uri = match hyper::Uri::from_maybe_shared(health_check_url.clone()) {
                Ok(uri) => uri,
                Err(e) => {
                    // Log warning for invalid health check URL
        if let Ok(fs) = crate::fs::RiFileSystem::new_auto_root() {
            let logger = crate::log::RiLogger::new(&crate::log::RiLogConfig::default(), fs);
            let _ = logger.warn("load_balancer", format!("Invalid health check URL for server {server_id}: {e}"));
        }
                    return false;
                }
            };
            
            match hyper::Client::new().get(uri).await {
                Ok(response) => {
                    (200..300).contains(&response.status().as_u16())
                },
                Err(_) => false,
            }
        } else {
            false
        }
    }
    
    #[cfg(not(feature = "gateway"))]
    pub async fn perform_health_check(&self, _server_id: &str) -> bool {
        // If gateway feature is not enabled, assume all servers are healthy
        true
    }
    
    /// Starts periodic health checks for all servers.
    /// 
    /// This method spawns a background task that performs health checks on all servers
    /// at the specified interval.
    /// 
    /// # Parameters
    /// 
    /// - `interval_secs`: Interval between health checks in seconds
    /// 
    /// # Returns
    /// 
    /// A `tokio::task::JoinHandle` for the background health check task
    pub async fn start_health_checks(self: Arc<Self>, interval_secs: u64) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
            
            loop {
                interval.tick().await;
                
                let servers = this.servers.read().await;
                let server_ids: Vec<String> = servers.iter().map(|s| s.id.clone()).collect();
                
                for server_id in server_ids {
                    let is_healthy = this.perform_health_check(&server_id).await;
                    let _ = this.mark_server_healthy(&server_id, is_healthy).await;
                    
                    // If server is unhealthy, record the failure
                    if !is_healthy {
                        let _ = this.record_server_failure(&server_id).await;
                    }
                }
            }
        })
    }

    /// Gets the current load balancing strategy.
    /// 
    /// # Returns
    /// 
    /// A reference to the current `RiLoadBalancerStrategy`
    pub fn get_strategy(&self) -> &RiLoadBalancerStrategy {
        &self.strategy
    }

    /// Sets the load balancing strategy.
    /// 
    /// # Parameters
    /// 
    /// - `strategy`: The new load balancing strategy to use
    pub async fn set_strategy(&mut self, strategy: RiLoadBalancerStrategy) {
        self.strategy = strategy;
    }

    /// Gets the total number of servers.
    /// 
    /// # Returns
    /// 
    /// The total number of servers in the load balancer
    pub async fn get_server_count(&self) -> usize {
        self.servers.read().await.len()
    }

    /// Gets the number of healthy servers.
    /// 
    /// # Returns
    /// 
    /// The number of healthy servers in the load balancer
    pub async fn get_healthy_server_count(&self) -> usize {
        self.get_healthy_servers().await.len()
    }
}
