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
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum DMSCircuitBreakerState {
    Closed,    // Normal operation
    Open,      // Failing, rejecting requests
    HalfOpen,  // Testing if service recovered
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DMSCircuitBreakerConfig {
    pub failure_threshold: u32,        // Number of failures to open circuit
    pub success_threshold: u32,        // Number of successes to close circuit from half-open
    pub timeout_seconds: u64,          // Time to wait before trying half-open
    pub monitoring_period_seconds: u64, // Time window for counting failures
}

impl Default for DMSCircuitBreakerConfig {
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout_seconds: 60,
            monitoring_period_seconds: 30,
        }
    }
}

#[derive(Debug)]
struct CircuitBreakerStats {
    state: RwLock<DMSCircuitBreakerState>,
    failure_count: AtomicUsize,
    success_count: AtomicUsize,
    last_failure_time: RwLock<Option<Instant>>,
    last_state_change: RwLock<Instant>,
    consecutive_failures: AtomicUsize,
    consecutive_successes: AtomicUsize,
}

impl CircuitBreakerStats {
    fn _Fnew() -> Self {
        Self {
            state: RwLock::new(DMSCircuitBreakerState::Closed),
            failure_count: AtomicUsize::new(0),
            success_count: AtomicUsize::new(0),
            last_failure_time: RwLock::new(None),
            last_state_change: RwLock::new(Instant::now()),
            consecutive_failures: AtomicUsize::new(0),
            consecutive_successes: AtomicUsize::new(0),
        }
    }

    async fn _Frecord_success(&self) {
        self.success_count.fetch_add(1, Ordering::Relaxed);
        self.consecutive_failures.store(0, Ordering::Relaxed);
        self.consecutive_successes.fetch_add(1, Ordering::Relaxed);

        let state = self.state.read().await;
        match *state {
            DMSCircuitBreakerState::HalfOpen => {
                let successes = self.consecutive_successes.load(Ordering::Relaxed);
                if successes >= 3 { // Hardcoded for now, should use config
                    drop(state);
                    let mut state_write = self.state.write().await;
                    *state_write = DMSCircuitBreakerState::Closed;
                    self.consecutive_successes.store(0, Ordering::Relaxed);
                    self.failure_count.store(0, Ordering::Relaxed);
                    *self.last_state_change.write().await = Instant::now();
                }
            }
            DMSCircuitBreakerState::Open => {
                // Should not happen if used correctly
            }
            DMSCircuitBreakerState::Closed => {
                // Normal operation
            }
        }
    }

    async fn _Frecord_failure(&self, config: &DMSCircuitBreakerConfig) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        self.consecutive_successes.store(0, Ordering::Relaxed);
        self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
        *self.last_failure_time.write().await = Some(Instant::now());

        let state = self.state.read().await;
        match *state {
            DMSCircuitBreakerState::Closed => {
                let failures = self.consecutive_failures.load(Ordering::Relaxed);
                if failures >= config.failure_threshold as usize {
                    drop(state);
                    let mut state_write = self.state.write().await;
                    *state_write = DMSCircuitBreakerState::Open;
                    self.consecutive_failures.store(0, Ordering::Relaxed);
                    *self.last_state_change.write().await = Instant::now();
                }
            }
            DMSCircuitBreakerState::HalfOpen => {
                drop(state);
                let mut state_write = self.state.write().await;
                *state_write = DMSCircuitBreakerState::Open;
                *self.last_state_change.write().await = Instant::now();
            }
            DMSCircuitBreakerState::Open => {
                // Already open
            }
        }
    }

    async fn _Fshould_attempt_reset(&self, config: &DMSCircuitBreakerConfig) -> bool {
        let state = self.state.read().await;
        if let DMSCircuitBreakerState::Open = *state {
            let last_change = *self.last_state_change.read().await;
            Instant::now().duration_since(last_change) >= Duration::from_secs(config.timeout_seconds)
        } else {
            false
        }
    }

    async fn _Ftransition_to_half_open(&self) {
        let mut state = self.state.write().await;
        *state = DMSCircuitBreakerState::HalfOpen;
        *self.last_state_change.write().await = Instant::now();
    }

    async fn _Fget_state(&self) -> DMSCircuitBreakerState {
        self.state.read().await.clone()
    }

    fn _Fget_stats(&self) -> CircuitBreakerMetrics {
        CircuitBreakerMetrics {
            state: "Unknown".to_string(), // This is a placeholder, the actual state is async
            failure_count: self.failure_count.load(Ordering::Relaxed),
            success_count: self.success_count.load(Ordering::Relaxed),
            consecutive_failures: self.consecutive_failures.load(Ordering::Relaxed),
            consecutive_successes: self.consecutive_successes.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircuitBreakerMetrics {
    pub state: String,
    pub failure_count: usize,
    pub success_count: usize,
    pub consecutive_failures: usize,
    pub consecutive_successes: usize,
}

pub struct DMSCircuitBreaker {
    config: DMSCircuitBreakerConfig,
    stats: Arc<CircuitBreakerStats>,
}

impl DMSCircuitBreaker {
    pub fn _Fnew(config: DMSCircuitBreakerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(CircuitBreakerStats::_Fnew()),
        }
    }

    pub async fn _Fallow_request(&self) -> bool {
        let state = self.stats._Fget_state().await;
        
        match state {
            DMSCircuitBreakerState::Closed => true,
            DMSCircuitBreakerState::Open => {
                if self.stats._Fshould_attempt_reset(&self.config).await {
                    self.stats._Ftransition_to_half_open().await;
                    true
                } else {
                    false
                }
            }
            DMSCircuitBreakerState::HalfOpen => true,
        }
    }

    pub async fn _Frecord_success(&self) {
        self.stats._Frecord_success().await;
    }

    pub async fn _Frecord_failure(&self) {
        self.stats._Frecord_failure(&self.config).await;
    }

    pub async fn _Fexecute<F, R>(&self, operation: F) -> DMSResult<R>
    where
        F: std::future::Future<Output = DMSResult<R>>,
    {
        if !self._Fallow_request().await {
            return Err(crate::core::DMSError::ServiceMesh("Circuit breaker is open".to_string()));
        }

        match operation.await {
            Ok(result) => {
                self._Frecord_success().await;
                Ok(result)
            }
            Err(error) => {
                self._Frecord_failure().await;
                Err(error)
            }
        }
    }

    pub async fn _Fget_state(&self) -> DMSCircuitBreakerState {
        self.stats._Fget_state().await
    }

    pub fn _Fget_stats(&self) -> CircuitBreakerMetrics {
        self.stats._Fget_stats()
    }

    pub fn _Fget_config(&self) -> &DMSCircuitBreakerConfig {
        &self.config
    }

    pub async fn _Freset(&self) {
        let mut state = self.stats.state.write().await;
        *state = DMSCircuitBreakerState::Closed;
        self.stats.failure_count.store(0, Ordering::Relaxed);
        self.stats.success_count.store(0, Ordering::Relaxed);
        self.stats.consecutive_failures.store(0, Ordering::Relaxed);
        self.stats.consecutive_successes.store(0, Ordering::Relaxed);
        *self.stats.last_state_change.write().await = Instant::now();
    }

    pub async fn _Fforce_open(&self) {
        let mut state = self.stats.state.write().await;
        *state = DMSCircuitBreakerState::Open;
        *self.stats.last_state_change.write().await = Instant::now();
    }

    pub async fn _Fforce_close(&self) {
        let mut state = self.stats.state.write().await;
        *state = DMSCircuitBreakerState::Closed;
        *self.stats.last_state_change.write().await = Instant::now();
    }
}

// Advanced circuit breaker with separate failure thresholds for different error types
pub struct DMSAdvancedCircuitBreaker {
    config: DMSCircuitBreakerConfig,
    stats_by_error: RwLock<HashMap<String, Arc<CircuitBreakerStats>>>,
    default_stats: Arc<CircuitBreakerStats>,
}

impl DMSAdvancedCircuitBreaker {
    pub fn _Fnew(config: DMSCircuitBreakerConfig) -> Self {
        Self {
            config,
            stats_by_error: RwLock::new(HashMap::new()),
            default_stats: Arc::new(CircuitBreakerStats::_Fnew()),
        }
    }

    async fn _Fget_stats_for_error(&self, error_type: Option<&str>) -> Arc<CircuitBreakerStats> {
        match error_type {
            Some(error_type) => {
                let mut stats_map = self.stats_by_error.write().await;
                stats_map.entry(error_type.to_string())
                    .or_insert_with(|| Arc::new(CircuitBreakerStats::_Fnew()))
                    .clone()
            }
            None => self.default_stats.clone(),
        }
    }

    pub async fn _Frecord_success_with_type(&self, error_type: Option<&str>) {
        let stats = self._Fget_stats_for_error(error_type).await;
        stats._Frecord_success().await;
    }

    pub async fn _Frecord_failure_with_type(&self, error_type: Option<&str>) {
        let stats = self._Fget_stats_for_error(error_type).await;
        stats._Frecord_failure(&self.config).await;
    }

    pub async fn _Fallow_request_for_type(&self, error_type: Option<&str>) -> bool {
        let stats = self._Fget_stats_for_error(error_type).await;
        let state = stats._Fget_state().await;
        
        match state {
            DMSCircuitBreakerState::Closed => true,
            DMSCircuitBreakerState::Open => {
                if stats._Fshould_attempt_reset(&self.config).await {
                    stats._Ftransition_to_half_open().await;
                    true
                } else {
                    false
                }
            }
            DMSCircuitBreakerState::HalfOpen => true,
        }
    }
}