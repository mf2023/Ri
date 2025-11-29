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

//! # Load Balancer Module
//! 
//! This module provides a robust load balancer implementation for distributing incoming requests
//! across multiple backend servers. It supports various load balancing strategies and includes
//! health checking, connection management, and detailed statistics.
//! 
//! ## Key Components
//! 
//! - **DMSLoadBalancerStrategy**: Enum representing different load balancing algorithms
//! - **DMSBackendServer**: Represents a backend server with configuration and health status
//! - **DMSLoadBalancer**: Main load balancer implementation
//! - **LoadBalancerServerStats**: Metrics for monitoring server performance
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
//! use dms::prelude::*;
//! use std::sync::Arc;
//! 
//! async fn example() -> DMSResult<()> {
//!     // Create a load balancer with Round Robin strategy
//!     let lb = Arc::new(DMSLoadBalancer::_Fnew(DMSLoadBalancerStrategy::RoundRobin));
//!     
//!     // Add backend servers
//!     lb._Fadd_server(DMSBackendServer::_Fnew("server1".to_string(), "http://localhost:8081".to_string())
//!         ._Fwith_weight(2)
//!         ._Fwith_max_connections(200))
//!         .await;
//!     
//!     lb._Fadd_server(DMSBackendServer::_Fnew("server2".to_string(), "http://localhost:8082".to_string())
//!         ._Fwith_weight(1)
//!         ._Fwith_max_connections(100))
//!         .await;
//!     
//!     // Start periodic health checks every 30 seconds
//!     lb.clone()._Fstart_health_checks(30).await;
//!     
//!     // Select a server for a client request
//!     let server = lb._Fselect_server(Some("192.168.1.1")).await?;
//!     println!("Selected server: {}", server.url);
//!     
//!     // Record response time when done
//!     lb._Frecord_response_time(&server.id, 150).await;
//!     
//!     // Release the server when the request is complete
//!     lb._Frelease_server(&server.id).await;
//!     
//!     // Get server statistics
//!     let stats = lb._Fget_all_stats().await;
//!     println!("Server stats: {:?}", stats);
//!     
//!     Ok(())
//! }
//! ```

use crate::core::DMSResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use tokio::sync::RwLock;

/// Load balancing strategies supported by DMS.
/// 
/// These strategies determine how the load balancer selects which backend server
/// to route incoming requests to.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DMSLoadBalancerStrategy {
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
}

/// Represents a backend server in the load balancer.
/// 
/// This struct contains all the configuration and state information for a backend server,
/// including its ID, URL, weight, max connections, health check path, and health status.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DMSBackendServer {
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

impl DMSBackendServer {
    /// Creates a new backend server with default settings.
    /// 
    /// # Parameters
    /// 
    /// - `id`: Unique identifier for the server
    /// - `url`: Base URL of the server
    /// 
    /// # Returns
    /// 
    /// A new `DMSBackendServer` instance with default settings
    /// (weight = 1, max_connections = 100, health_check_path = "/health", is_healthy = true)
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

    /// Sets the weight for this server.
    /// 
    /// # Parameters
    /// 
    /// - `weight`: Weight to assign to the server
    /// 
    /// # Returns
    /// 
    /// The modified `DMSBackendServer` instance for method chaining
    pub fn _Fwith_weight(mut self, weight: u32) -> Self {
        self.weight = weight;
        self
    }

    /// Sets the maximum number of concurrent connections for this server.
    /// 
    /// # Parameters
    /// 
    /// - `max_connections`: Maximum number of concurrent connections allowed
    /// 
    /// # Returns
    /// 
    /// The modified `DMSBackendServer` instance for method chaining
    pub fn _Fwith_max_connections(mut self, max_connections: usize) -> Self {
        self.max_connections = max_connections;
        self
    }

    /// Sets the health check path for this server.
    /// 
    /// # Parameters
    /// 
    /// - `path`: Path to use for health checks (e.g., "/health")
    /// 
    /// # Returns
    /// 
    /// The modified `DMSBackendServer` instance for method chaining
    pub fn _Fwith_health_check_path(mut self, path: String) -> Self {
        self.health_check_path = path;
        self
    }
}

/// Internal server statistics tracking.
/// 
/// This struct tracks real-time statistics for each backend server, including active connections,
/// request counts, failures, and response times. It is designed to be thread-safe for use in
/// multi-threaded environments.
#[derive(Debug)]
struct _CServerStats {
    /// Number of currently active connections to the server
    active_connections: AtomicUsize,
    
    /// Total number of requests sent to the server since it was added
    total_requests: AtomicUsize,
    
    /// Number of failed requests to the server
    failed_requests: AtomicUsize,
    
    /// Most recent response time in milliseconds
    response_time_ms: AtomicUsize,
    
    /// Timestamp of when the server was last used
    last_used: RwLock<Instant>,
}

impl _CServerStats {
    /// Creates a new server statistics instance with default values.
    /// 
    /// Initializes all counters to zero and sets the last used time to now.
    /// 
    /// # Returns
    /// 
    /// A new `_CServerStats` instance with default values
    fn _Fnew() -> Self {
        Self {
            active_connections: AtomicUsize::new(0),
            total_requests: AtomicUsize::new(0),
            failed_requests: AtomicUsize::new(0),
            response_time_ms: AtomicUsize::new(0),
            last_used: RwLock::new(Instant::now()),
        }
    }

    /// Gets the current number of active connections to the server.
    /// 
    /// # Returns
    /// 
    /// The number of active connections as a `usize`
    fn _Fget_active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }

    /// Increments the active connection count and updates request statistics.
    /// 
    /// This method should be called when a new connection is established to the server.
    /// 
    /// - Increments active_connections by 1
    /// - Increments total_requests by 1
    /// - Updates last_used to the current time
    fn _Fincrement_connections(&self) {
        self.active_connections.fetch_add(1, Ordering::Relaxed);
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        let mut last_used = self.last_used.blocking_write();
        *last_used = Instant::now();
    }

    /// Decrements the active connection count.
    /// 
    /// This method should be called when a connection to the server is closed.
    fn _Fdecrement_connections(&self) {
        self.active_connections.fetch_sub(1, Ordering::Relaxed);
    }

    /// Records a failed request to the server.
    /// 
    /// This method should be called when a request to the server fails.
    /// 
    /// - Increments failed_requests by 1
    /// - Decrements active_connections by 1 (since the connection failed)
    fn _Frecord_failure(&self) {
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        self._Fdecrement_connections();
    }

    /// Records the response time for a successful request.
    /// 
    /// This method should be called when a request to the server completes successfully.
    /// 
    /// # Parameters
    /// 
    /// - `response_time_ms`: Response time in milliseconds
    fn _Frecord_response_time(&self, response_time_ms: u64) {
        self.response_time_ms.store(response_time_ms as usize, Ordering::Relaxed);
    }

    /// Gets a snapshot of the current server statistics.
    /// 
    /// This method converts the internal statistics into a public-facing `LoadBalancerServerStats` struct.
    /// 
    /// # Returns
    /// 
    /// A `LoadBalancerServerStats` struct containing the current statistics
    fn _Fget_stats(&self) -> LoadBalancerServerStats {
        LoadBalancerServerStats {
            active_connections: self._Fget_active_connections(),
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
pub struct LoadBalancerServerStats {
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
pub struct DMSLoadBalancer {
    /// Load balancing strategy to use
    strategy: DMSLoadBalancerStrategy,
    
    /// List of backend servers
    servers: RwLock<Vec<DMSBackendServer>>,
    
    /// Statistics for each backend server
    server_stats: RwLock<HashMap<String, Arc<_CServerStats>>>,
    
    /// Counter for round robin scheduling
    round_robin_counter: AtomicUsize,
}

impl Clone for DMSLoadBalancer {
    /// Creates a clone of the load balancer.
    /// 
    /// Note: The clone will have the same strategy and counter, but empty servers and stats
    /// since we can't await in the Clone trait.
    fn clone(&self) -> Self {
        Self {
            strategy: self.strategy.clone(),
            servers: RwLock::new(Vec::new()),
            server_stats: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(self.round_robin_counter.load(Ordering::Relaxed)),
        }
    }
}

impl DMSLoadBalancer {
    /// Creates a new load balancer with the specified strategy.
    /// 
    /// # Parameters
    /// 
    /// - `strategy`: The load balancing strategy to use
    /// 
    /// # Returns
    /// 
    /// A new `DMSLoadBalancer` instance with the specified strategy
    pub fn _Fnew(strategy: DMSLoadBalancerStrategy) -> Self {
        Self {
            strategy,
            servers: RwLock::new(Vec::new()),
            server_stats: RwLock::new(HashMap::new()),
            round_robin_counter: AtomicUsize::new(0),
        }
    }

    /// Adds a backend server to the load balancer.
    /// 
    /// # Parameters
    /// 
    /// - `server`: The backend server to add
    pub async fn _Fadd_server(&self, server: DMSBackendServer) {
        let mut servers = self.servers.write().await;
        let mut stats = self.server_stats.write().await;
        
        servers.push(server.clone());
        stats.insert(server.id.clone(), Arc::new(_CServerStats::_Fnew()));
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
    pub async fn _Fremove_server(&self, server_id: &str) -> bool {
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
    /// A vector of healthy `DMSBackendServer` instances
    pub async fn _Fget_healthy_servers(&self) -> Vec<DMSBackendServer> {
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
    /// A `DMSResult<DMSBackendServer>` with the selected server, or an error if no servers are available
    pub async fn _Fselect_server(&self, client_ip: Option<&str>) -> DMSResult<DMSBackendServer> {
        let healthy_servers = self._Fget_healthy_servers().await;
        
        if healthy_servers.is_empty() {
            return Err(crate::core::DMSError::Other("No healthy servers available".to_string()));
        }
        
        let stats = self.server_stats.read().await;
        
        // Filter servers that have available connections
        let available_servers: Vec<DMSBackendServer> = healthy_servers.into_iter()
            .filter(|server| {
                if let Some(server_stats) = stats.get(&server.id) {
                    let connections = server_stats._Fget_active_connections();
                    connections < server.max_connections
                } else {
                    true // If no stats, assume server is available
                }
            })
            .collect();
        
        if available_servers.is_empty() {
            return Err(crate::core::DMSError::Other("No servers with available connections".to_string()));
        }

        let server = match self.strategy {
            DMSLoadBalancerStrategy::RoundRobin => self._Fselect_round_robin(&available_servers).await,
            DMSLoadBalancerStrategy::WeightedRoundRobin => self._Fselect_weighted_round_robin(&available_servers).await,
            DMSLoadBalancerStrategy::LeastConnections => self._Fselect_least_connections(&available_servers).await,
            DMSLoadBalancerStrategy::Random => self._Fselect_random(&available_servers),
            DMSLoadBalancerStrategy::IpHash => self._Fselect_ip_hash(&available_servers, client_ip),
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
    async fn _Fselect_round_robin(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        let counter = self.round_robin_counter.fetch_add(1, Ordering::Relaxed);
        let index = counter % servers.len();
        servers.get(index).cloned()
    }

    /// Selects a server using the Weighted Round Robin strategy.
    /// 
    /// This method selects servers based on their assigned weights, allowing more powerful
    /// servers to handle a larger share of traffic.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers with weights
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
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

    /// Selects a server using the Least Connections strategy.
    /// 
    /// This method selects the server with the fewest active connections, ensuring balanced load.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
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
    fn _Fselect_random(&self, servers: &[DMSBackendServer]) -> Option<DMSBackendServer> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..servers.len());
        servers.get(index).cloned()
    }

    /// Selects a server using the IP Hash strategy.
    /// 
    /// This method uses the client IP address to consistently route to the same server,
    /// maintaining session persistence.
    /// 
    /// # Parameters
    /// 
    /// - `servers`: List of available servers
    /// - `client_ip`: Optional client IP address for hashing
    /// 
    /// # Returns
    /// 
    /// The selected server, or `None` if no servers are available
    fn _Fselect_ip_hash(&self, servers: &[DMSBackendServer], client_ip: Option<&str>) -> Option<DMSBackendServer> {
        if let Some(ip) = client_ip {
            let hash = self._Fhash_ip(ip);
            let index = hash as usize % servers.len();
            servers.get(index).cloned()
        } else {
            self._Fselect_random(servers)
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
    fn _Fhash_ip(&self, ip: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        ip.hash(&mut hasher);
        hasher.finish()
    }

    /// Releases a server after a request is completed.
    /// 
    /// This method decrements the active connection count for the specified server.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to release
    pub async fn _Frelease_server(&self, server_id: &str) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats._Fdecrement_connections();
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

    /// Records the response time for a successful request.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server that handled the request
    /// - `response_time_ms`: Response time in milliseconds
    pub async fn _Frecord_response_time(&self, server_id: &str, response_time_ms: u64) {
        if let Some(stats) = self.server_stats.read().await.get(server_id) {
            stats._Frecord_response_time(response_time_ms);
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
    /// An `Option<LoadBalancerServerStats>` with the server statistics, or `None` if the server doesn't exist
    pub async fn _Fget_server_stats(&self, server_id: &str) -> Option<LoadBalancerServerStats> {
        self.server_stats.read().await
            .get(server_id)
            .map(|stats| stats._Fget_stats())
    }

    /// Gets statistics for all servers.
    /// 
    /// # Returns
    /// 
    /// A `HashMap<String, LoadBalancerServerStats>` with statistics for all servers
    pub async fn _Fget_all_stats(&self) -> HashMap<String, LoadBalancerServerStats> {
        let stats = self.server_stats.read().await;
        let mut result = HashMap::new();
        
        for (server_id, server_stats) in stats.iter() {
            result.insert(server_id.clone(), server_stats._Fget_stats());
        }
        
        result
    }

    /// Marks a server as healthy or unhealthy.
    /// 
    /// # Parameters
    /// 
    /// - `server_id`: ID of the server to update
    /// - `healthy`: New health status (true = healthy, false = unhealthy)
    pub async fn _Fmark_server_healthy(&self, server_id: &str, healthy: bool) {
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
    pub async fn _Fperform_health_check(&self, server_id: &str) -> bool {
        let servers = self.servers.read().await;
        
        if let Some(server) = servers.iter().find(|s| s.id == server_id) {
            let health_check_url = format!("{}{}", server.url, server.health_check_path);
            
            match hyper::Client::new().get(hyper::Uri::from_maybe_shared(health_check_url).unwrap())
                .await {
                Ok(response) => {
                    // Consider server healthy if status code is 2xx
                    (200..300).contains(&response.status().as_u16())
                },
                Err(_) => false,
            }
        } else {
            false
        }
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
    pub async fn _Fstart_health_checks(self: Arc<Self>, interval_secs: u64) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(interval_secs));
            
            loop {
                interval.tick().await;
                
                let servers = this.servers.read().await;
                let server_ids: Vec<String> = servers.iter().map(|s| s.id.clone()).collect();
                
                for server_id in server_ids {
                    let is_healthy = this._Fperform_health_check(&server_id).await;
                    let _ = this._Fmark_server_healthy(&server_id, is_healthy).await;
                    
                    // If server is unhealthy, record the failure
                    if !is_healthy {
                        let _ = this._Frecord_server_failure(&server_id).await;
                    }
                }
            }
        })
    }

    /// Gets the current load balancing strategy.
    /// 
    /// # Returns
    /// 
    /// A reference to the current `DMSLoadBalancerStrategy`
    pub fn _Fget_strategy(&self) -> &DMSLoadBalancerStrategy {
        &self.strategy
    }

    /// Sets the load balancing strategy.
    /// 
    /// # Parameters
    /// 
    /// - `strategy`: The new load balancing strategy to use
    pub async fn _Fset_strategy(&mut self, strategy: DMSLoadBalancerStrategy) {
        self.strategy = strategy;
    }

    /// Gets the total number of servers.
    /// 
    /// # Returns
    /// 
    /// The total number of servers in the load balancer
    pub async fn _Fget_server_count(&self) -> usize {
        self.servers.read().await.len()
    }

    /// Gets the number of healthy servers.
    /// 
    /// # Returns
    /// 
    /// The number of healthy servers in the load balancer
    pub async fn _Fget_healthy_server_count(&self) -> usize {
        self._Fget_healthy_servers().await.len()
    }
}