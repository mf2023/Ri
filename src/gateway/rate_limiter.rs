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

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::RwLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct DMSRateLimitConfig {
    pub requests_per_second: u32,
    pub burst_size: u32,
    pub window_seconds: u64,
}

impl Default for DMSRateLimitConfig {
    fn default() -> Self {
        Self {
            requests_per_second: 10,
            burst_size: 20,
            window_seconds: 60,
        }
    }
}

#[derive(Debug)]
struct RateLimitBucket {
    tokens: AtomicUsize,
    last_update: RwLock<Instant>,
    request_count: AtomicUsize,
}

impl RateLimitBucket {
    fn _Fnew(tokens: usize) -> Self {
        Self {
            tokens: AtomicUsize::new(tokens),
            last_update: RwLock::new(Instant::now()),
            request_count: AtomicUsize::new(0),
        }
    }

    async fn _Ftry_consume(&self, tokens: usize, config: &DMSRateLimitConfig) -> bool {
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

    fn _Fget_stats(&self) -> RateLimitStats {
        RateLimitStats {
            current_tokens: self.tokens.load(Ordering::Relaxed),
            total_requests: self.request_count.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitStats {
    pub current_tokens: usize,
    pub total_requests: usize,
}

pub struct DMSRateLimiter {
    config: DMSRateLimitConfig,
    buckets: RwLock<HashMap<String, Arc<RateLimitBucket>>>,
}

impl DMSRateLimiter {
    pub fn _Fnew(config: DMSRateLimitConfig) -> Self {
        Self {
            config,
            buckets: RwLock::new(HashMap::new()),
        }
    }

    pub async fn _Fcheck_request(&self, request: &crate::gateway::DMSGatewayRequest) -> bool {
        // Use client IP as the key for rate limiting
        let key = request.remote_addr.clone();
        self._Fcheck_rate_limit(&key, 1).await
    }

    pub async fn _Fcheck_rate_limit(&self, key: &str, tokens: usize) -> bool {
        let buckets = self.buckets.read().await;
        
        if let Some(bucket) = buckets.get(key) {
            bucket._Ftry_consume(tokens, &self.config).await
        } else {
            // Create new bucket
            drop(buckets);
            let mut buckets = self.buckets.write().await;
            
            // Check again in case another thread created it
            if let Some(bucket) = buckets.get(key) {
                bucket._Ftry_consume(tokens, &self.config).await
            } else {
                let bucket = Arc::new(RateLimitBucket::_Fnew(self.config.burst_size as usize));
                let result = bucket._Ftry_consume(tokens, &self.config).await;
                buckets.insert(key.to_string(), bucket);
                result
            }
        }
    }

    pub async fn _Fget_stats(&self, key: &str) -> Option<RateLimitStats> {
        let buckets = self.buckets.read().await;
        buckets.get(key).map(|bucket| bucket._Fget_stats())
    }

    pub async fn _Fget_all_stats(&self) -> HashMap<String, RateLimitStats> {
        let buckets = self.buckets.read().await;
        let mut stats = HashMap::new();
        
        for (key, bucket) in buckets.iter() {
            stats.insert(key.clone(), bucket._Fget_stats());
        }
        
        stats
    }

    pub async fn _Freset_bucket(&self, key: &str) {
        let mut buckets = self.buckets.write().await;
        buckets.remove(key);
    }

    pub async fn _Fclear_all_buckets(&self) {
        let mut buckets = self.buckets.write().await;
        buckets.clear();
    }

    pub fn _Fget_config(&self) -> &DMSRateLimitConfig {
        &self.config
    }

    pub async fn _Fupdate_config(&mut self, config: DMSRateLimitConfig) {
        self.config = config;
        
        // Reset all buckets with new configuration
        let mut buckets = self.buckets.write().await;
        buckets.clear();
    }
}

// Sliding window rate limiter for more fine-grained control
pub struct DMSSlidingWindowRateLimiter {
    max_requests: u32,
    window_duration: Duration,
    requests: RwLock<Vec<Instant>>,
}

impl DMSSlidingWindowRateLimiter {
    pub fn _Fnew(max_requests: u32, window_seconds: u64) -> Self {
        Self {
            max_requests,
            window_duration: Duration::from_secs(window_seconds),
            requests: RwLock::new(Vec::new()),
        }
    }

    pub async fn _Fallow_request(&self) -> bool {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        // Remove old requests outside the window
        requests.retain(|&timestamp| now.duration_since(timestamp) < self.window_duration);
        
        // Check if we can add a new request
        if requests.len() < self.max_requests as usize {
            requests.push(now);
            true
        } else {
            false
        }
    }

    pub async fn _Fget_current_count(&self) -> usize {
        let mut requests = self.requests.write().await;
        let now = Instant::now();
        
        // Remove old requests outside the window
        requests.retain(|&timestamp| now.duration_since(timestamp) < self.window_duration);
        
        requests.len()
    }

    pub async fn _Freset(&self) {
        let mut requests = self.requests.write().await;
        requests.clear();
    }
}