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

//! # Rate Limiter Module
//! 
//! This module provides rate limiting functionality for the Ri gateway, allowing for
//! controlling the rate of requests from clients to prevent abuse and ensure fair usage.
//! 
//! ## Key Components
//! 
//! - **RiRateLimitConfig**: Configuration for rate limiting behavior
//! - **RiRateLimiter**: Token bucket based rate limiter implementation
//! - **RiSlidingWindowRateLimiter**: Sliding window based rate limiter for fine-grained control
//! - **RiRateLimitStats**: Metrics for monitoring rate limiter performance
//! 
//! ## Design Principles
//! 
//! 1. **Token Bucket Algorithm**: Implements the token bucket algorithm for smooth rate limiting
//! 2. **Sliding Window**: Provides a sliding window implementation for more precise control
//! 3. **Thread Safe**: Uses Arc and RwLock for safe operation in multi-threaded environments
//! 4. **Configurable**: Allows fine-tuning of requests per second, burst size, and window duration
//! 5. **Metrics Collection**: Tracks and reports rate limiter statistics
//! 6. **Async Compatibility**: Built with async/await patterns for modern Rust applications
//! 7. **Burst Support**: Allows for temporary bursts of requests beyond the steady rate
//! 8. **Key-Based Limiting**: Supports rate limiting by client IP or custom keys
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! async fn example() {
//!     // Create a rate limiter with default configuration
//!     let mut limiter = RiRateLimiter::new(RiRateLimitConfig::default());
//!     
//!     // Check if a request should be allowed
//!     let client_ip = "192.168.1.1";
//!     if limiter.check_rate_limit(client_ip, 1).await {
//!         println!("Request allowed");
//!     } else {
//!         println!("Request rate limited");
//!     }
//!     
//!     // Get rate limit stats for a client
//!     if let Some(stats) = limiter.get_stats(client_ip).await {
//!         println!("Current tokens: {}, Total requests: {}", 
//!             stats.current_tokens, stats.total_requests);
//!     }
//!     
//!     // Create a sliding window rate limiter
//!     let sliding_limiter = RiSlidingWindowRateLimiter::new(100, 60);
//!     if sliding_limiter.allow_request().await {
//!         println!("Sliding window request allowed");
//!     }
//! }
//! ```

use std::collections::HashMap as FxHashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

/// Configuration for rate limiting behavior.
/// 
/// This struct defines the parameters that control how the rate limiter behaves,
/// including the steady rate, burst capacity, and window duration.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone)]
pub struct RiRateLimitConfig {
    /// Maximum number of requests allowed per second in steady state
    pub requests_per_second: u32,
    
    /// Maximum number of requests allowed in a burst (temporary spike)
    pub burst_size: u32,
    
    /// Duration of the rate limiting window in seconds
    pub window_seconds: u64,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiRateLimitConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn py_new_with_values(requests_per_second: u32, burst_size: u32, window_seconds: u64) -> Self {
        Self {
            requests_per_second,
            burst_size,
            window_seconds,
        }
    }
    
    fn get_requests_per_second(&self) -> u32 {
        self.requests_per_second
    }
    
    fn set_requests_per_second(&mut self, value: u32) {
        self.requests_per_second = value;
    }
    
    fn get_burst_size(&self) -> u32 {
        self.burst_size
    }
    
    fn set_burst_size(&mut self, value: u32) {
        self.burst_size = value;
    }
    
    fn get_window_seconds(&self) -> u64 {
        self.window_seconds
    }
    
    fn set_window_seconds(&mut self, value: u64) {
        self.window_seconds = value;
    }
}

impl Default for RiRateLimitConfig {
    /// Creates a default rate limit configuration.
    /// 
    /// Default values:
    /// - requests_per_second: 10 requests per second
    /// - burst_size: 20 requests (temporary burst capacity)
    /// - window_seconds: 60 seconds window duration
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 20,
            window_seconds: 60,
        }
    }
}

/// Internal token bucket for rate limiting.
/// 
/// This struct implements the token bucket algorithm for rate limiting, tracking
/// available tokens, last update time, and request count.
#[derive(Debug)]
struct RateLimitBucket {
    /// Current number of available tokens in the bucket
    tokens: AtomicUsize,
    
    /// Timestamp of the last token refill
    last_update: RwLock<Instant>,
    
    /// Total number of requests processed by this bucket
    request_count: AtomicUsize,
}

impl RateLimitBucket {
    /// Creates a new token bucket with the specified initial tokens.
    /// 
    /// # Parameters
    /// 
    /// - `tokens`: Initial number of tokens in the bucket
    /// 
    /// # Returns
    /// 
    /// A new `RateLimitBucket` instance
    fn new(tokens: usize) -> Self {
        Self {
            tokens: AtomicUsize::new(tokens),
            last_update: RwLock::new(Instant::now()),
            request_count: AtomicUsize::new(0),
        }
    }

    /// Attempts to consume tokens from the bucket.
    /// 
    /// This method refills tokens based on time elapsed since the last update,
    /// then attempts to consume the requested number of tokens.
    /// 
    /// # Parameters
    /// 
    /// - `tokens`: Number of tokens to consume
    /// - `config`: Rate limit configuration for token refill
    /// 
    /// # Returns
    /// 
    /// `true` if tokens were successfully consumed, `false` otherwise
    async fn try_consume(&self, tokens: usize, config: &RiRateLimitConfig) -> bool {
        let now = Instant::now();
        let mut last_update = self.last_update.write().await;
        
        // Refill tokens based on time elapsed
        let elapsed = now.duration_since(*last_update).as_secs_f64();
        let tokens_to_add = (elapsed * config.requests_per_second as f64) as usize;
        
        if tokens_to_add > 0 {
            let current_tokens = self.tokens.load(Ordering::Relaxed);
            let new_tokens = std::cmp::min(current_tokens + tokens_to_add, config.burst_size as usize);
            self.tokens.store(new_tokens, Ordering::Relaxed);
            *last_update = now;
        }
        
        // Try to consume tokens
        let current_tokens = self.tokens.load(Ordering::Relaxed);
        if current_tokens >= tokens {
            self.tokens.fetch_sub(tokens, Ordering::Relaxed);
            self.request_count.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    /// Gets the current statistics for this bucket.
    /// 
    /// # Returns
    /// 
    /// A `RiRateLimitStats` struct containing current tokens and total requests
    fn get_stats(&self) -> RiRateLimitStats {
        RiRateLimitStats {
            current_tokens: self.tokens.load(Ordering::Relaxed),
            total_requests: self.request_count.load(Ordering::Relaxed),
        }
    }
}

/// Statistics for rate limiting monitoring.
/// 
/// This struct contains metrics about a rate limiter bucket, including the current
/// number of available tokens and the total number of requests processed.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass(get_all, set_all))]
#[derive(Debug, Clone)]
pub struct RiRateLimitStats {
    /// Current number of available tokens in the bucket
    pub current_tokens: usize,

    /// Total number of requests processed by the bucket
    pub total_requests: usize,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiRateLimitStats {
    #[new]
    fn py_new(current_tokens: usize, total_requests: usize) -> Self {
        Self {
            current_tokens,
            total_requests,
        }
    }
    
    fn get_current_tokens(&self) -> usize {
        self.current_tokens
    }
    
    fn get_total_requests(&self) -> usize {
        self.total_requests
    }
}

/// Token bucket based rate limiter implementation.
/// 
/// This struct implements the token bucket algorithm for rate limiting, allowing
/// for both steady-state rate limiting and temporary bursts of requests.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiRateLimiter {
    /// Configuration for rate limiting behavior
    config: RiRateLimitConfig,
    
    /// Map of key to token bucket instances
    buckets: RwLock<FxHashMap<String, Arc<RateLimitBucket>>>,
}

impl RiRateLimiter {
    /// Creates a new rate limiter with the specified configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The configuration for rate limiting behavior
    /// 
    /// # Returns
    /// 
    /// A new `RiRateLimiter` instance
    pub fn new(config: RiRateLimitConfig) -> Self {
        Self {
            config,
            buckets: RwLock::new(FxFxHashMap::default()),
        }
    }

    /// Checks if a gateway request should be allowed based on rate limiting.
    /// 
    /// This method uses the client IP address as the key for rate limiting.
    /// 
    /// # Parameters
    /// 
    /// - `request`: The gateway request to check
    /// 
    /// # Returns
    /// 
    /// `true` if the request should be allowed, `false` otherwise
    pub async fn check_request(&self, request: &crate::gateway::RiGatewayRequest) -> bool {
        // Use client IP as the key for rate limiting
        let key = request.remote_addr.clone();
        self.check_rate_limit(&key, 1)
    }

    /// Checks if a request with a custom key should be allowed based on rate limiting.
    /// 
    /// This method attempts to consume tokens from the bucket associated with the given key.
    /// If no bucket exists for the key, a new one is created.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The key to use for rate limiting (e.g., client IP, API key)
    /// - `tokens`: Number of tokens to consume for this request
    /// 
    /// # Returns
    /// 
    /// `true` if the request should be allowed, `false` otherwise
    pub fn check_rate_limit(&self, key: &str, tokens: usize) -> bool {
        futures::executor::block_on(async {
            let buckets = self.buckets.read().await;
            
            if let Some(bucket) = buckets.get(key) {
                bucket.try_consume(tokens, &self.config).await
            } else {
                drop(buckets);
                let mut buckets = self.buckets.write().await;
                
                if let Some(bucket) = buckets.get(key) {
                    bucket.try_consume(tokens, &self.config).await
                } else {
                    let bucket = Arc::new(RateLimitBucket::new(self.config.burst_size as usize));
                    let result = bucket.try_consume(tokens, &self.config).await;
                    buckets.insert(key.to_string(), bucket);
                    result
                }
            }
        })
    }

    /// Gets rate limit statistics for a specific key.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The key to get statistics for
    /// 
    /// # Returns
    /// 
    /// An `Option<RiRateLimitStats>` with the statistics, or `None` if no bucket exists for the key
    pub fn get_stats(&self, key: &str) -> Option<RiRateLimitStats> {
        futures::executor::block_on(async {
            let buckets = self.buckets.read().await;
            buckets.get(key).map(|bucket| bucket.get_stats())
        })
    }
    
    /// Gets the remaining tokens for a specific key.
    pub fn get_remaining(&self, key: &str) -> Option<f64> {
        futures::executor::block_on(async {
            let buckets = self.buckets.read().await;
            buckets.get(key).map(|bucket| {
                let stats = bucket.get_stats();
                stats.current_tokens as f64
            })
        })
    }

    /// Gets rate limit statistics for all keys.
    /// 
    /// # Returns
    /// 
    /// A `FxHashMap<String, RiRateLimitStats>` with statistics for all keys
    pub fn get_all_stats(&self) -> FxHashMap<String, RiRateLimitStats> {
        futures::executor::block_on(async {
            let buckets = self.buckets.read().await;
            let mut stats = FxFxHashMap::default();
            
            for (key, bucket) in buckets.iter() {
                stats.insert(key.clone(), bucket.get_stats());
            }
            
            stats
        })
    }

    /// Resets the rate limit bucket for a specific key.
    /// 
    /// This method removes the bucket for the given key, effectively resetting the rate limit.
    /// 
    /// # Parameters
    /// 
    /// - `key`: The key to reset the bucket for
    pub fn reset_bucket(&self, key: &str) {
        futures::executor::block_on(async {
            let mut buckets = self.buckets.write().await;
            buckets.remove(key);
        })
    }

    /// Clears all rate limit buckets.
    /// 
    /// This method removes all buckets, effectively resetting rate limits for all keys.
    pub fn clear_all_buckets(&self) {
        futures::executor::block_on(async {
            let mut buckets = self.buckets.write().await;
            buckets.clear();
        })
    }

    /// Gets the current rate limit configuration.
    /// 
    /// # Returns
    /// 
    /// A reference to the current `RiRateLimitConfig`
    pub fn get_config(&self) -> RiRateLimitConfig {
        self.config.clone()
    }

    /// Updates the rate limit configuration.
    /// 
    /// This method updates the configuration and resets all buckets with the new settings.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The new rate limit configuration
    pub async fn update_config(&mut self, config: RiRateLimitConfig) {
        self.config = config;
        
        let mut buckets = self.buckets.write().await;
        buckets.clear();
    }

    pub async fn check_multi(&self, keys: &[String], tokens: usize) -> Vec<bool> {
        let mut results = Vec::with_capacity(keys.len());
        for key in keys {
            results.push(self.check_rate_limit(key, tokens));
        }
        results
    }

    pub async fn get_keys(&self) -> Vec<String> {
        let buckets = self.buckets.read().await;
        buckets.keys().cloned().collect()
    }
    
    pub fn bucket_count(&self) -> usize {
        futures::executor::block_on(async {
            let buckets = self.buckets.read().await;
            buckets.len()
        })
    }
}

/// Sliding window rate limiter for fine-grained control.
/// 
/// This struct implements a sliding window rate limiter, which provides more precise
/// rate limiting by tracking all requests within a sliding time window.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiSlidingWindowRateLimiter {
    /// Maximum number of requests allowed within the window
    max_requests: u32,
    /// Duration of the sliding window
    window_duration: Duration,
    /// Vector of request timestamps within the window
    requests: RwLock<Vec<Instant>>,
}

impl RiSlidingWindowRateLimiter {
    /// Creates a new sliding window rate limiter.
    /// 
    /// # Parameters
    /// 
    /// - `max_requests`: Maximum number of requests allowed within the window
    /// - `window_seconds`: Duration of the window in seconds
    /// 
    /// # Returns
    /// 
    /// A new `RiSlidingWindowRateLimiter` instance
    pub fn new(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
            requests: RwLock::new(Vec::new()),
        }
    }

    /// Checks if a request should be allowed based on the sliding window.
    /// 
    /// This method removes old requests outside the window, then checks if the number
    /// of remaining requests is below the maximum allowed.
    /// 
    /// # Returns
    /// 
    /// `true` if the request should be allowed, `false` otherwise
    pub fn allow_request(&self) -> bool {
        futures::executor::block_on(async {
            let mut requests = self.requests.write().await;
            let now = Instant::now();
            
            requests.retain(|&timestamp| now.duration_since(timestamp) < self.window_duration);
            
            if requests.len() < self.max_requests as usize {
                requests.push(now);
                true
            } else {
                false
            }
        })
    }

    /// Gets the current number of requests within the sliding window.
    /// 
    /// This method removes old requests outside the window, then returns the count
    /// of remaining requests.
    /// 
    /// # Returns
    /// 
    /// The number of requests within the current window
    pub fn get_current_count(&self) -> usize {
        futures::executor::block_on(async {
            let mut requests = self.requests.write().await;
            let now = Instant::now();
            
            requests.retain(|&timestamp| now.duration_since(timestamp) < self.window_duration);
            
            requests.len()
        })
    }

    /// Resets the sliding window by clearing all request timestamps.
    pub fn reset(&self) {
        futures::executor::block_on(async {
            let mut requests = self.requests.write().await;
            requests.clear();
        })
    }
    
    pub fn get_max_requests(&self) -> u32 {
        self.max_requests
    }
    
    pub fn get_window_seconds(&self) -> u64 {
        self.window_duration.as_secs()
    }
}
