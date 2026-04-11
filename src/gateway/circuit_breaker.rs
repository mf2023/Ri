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

//! # Circuit Breaker Module
//! 
//! This module provides robust circuit breaker implementations for fault tolerance in distributed systems.
//! Circuit breakers prevent cascading failures by temporarily stopping requests to failing services.
//! 
//! ## Key Components
//! 
//! - **RiCircuitBreakerState**: Enum representing the three states of a circuit breaker (Closed, Open, HalfOpen)
//! - **RiCircuitBreakerConfig**: Configuration for circuit breaker behavior
//! - **RiCircuitBreaker**: Basic circuit breaker implementation
//! - **RiAdvancedCircuitBreaker**: Advanced circuit breaker with error-type specific thresholds
//! - **RiCircuitBreakerMetrics**: Metrics for monitoring circuit breaker performance
//! 
//! ## Design Principles
//! 
//! 1. **Fault Isolation**: Prevent cascading failures by stopping requests to failing services
//! 2. **Automatic Recovery**: Automatically test and recover when services become healthy again
//! 3. **Configurable Behavior**: Allow fine-tuning of failure thresholds, timeouts, and recovery parameters
//! 4. **Metrics Collection**: Track and report circuit breaker performance for monitoring
//! 5. **Thread Safety**: Ensure safe operation in multi-threaded environments
//! 6. **Error Type Specificity**: Advanced implementation supports different thresholds for different error types
//! 7. **Async Compatibility**: Designed for use with async/await patterns
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! async fn example() -> RiResult<()> {
//!     // Create a circuit breaker with default configuration
//!     let cb_config = RiCircuitBreakerConfig::default();
//!     let cb = RiCircuitBreaker::new(cb_config);
//!     
//!     // Execute a risky operation with circuit breaker protection
//!     let result = cb.execute(async || {
//!         // This could be a network request, database operation, etc.
//!         Ok("Success!")
//!     }).await;
//!     
//!     // Get circuit breaker state and metrics
//!     let state = cb.get_state().await;
//!     let metrics = cb.get_stats();
//!     
//!     println!("Circuit breaker state: {:?}", state);
//!     println!("Circuit breaker metrics: {:?}", metrics);
//!     
//!     Ok(())
//! }
//! ```

use crate::core::RiResult;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

/// Represents the three states of a circuit breaker.
/// 
/// The circuit breaker transitions between these states based on the success and failure
/// patterns of the protected operations.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum RiCircuitBreakerState {
    /// **Closed State**: Normal operation. Requests are allowed to pass through.
    /// The circuit breaker monitors for failures.
    Closed,
    
    /// **Open State**: The circuit breaker has detected too many failures. Requests
    /// are rejected immediately to prevent cascading failures.
    Open,
    
    /// **HalfOpen State**: The circuit breaker is testing if the service has recovered.
    /// A limited number of requests are allowed through to test the service's health.
    HalfOpen,
}

/// Configuration for circuit breaker behavior.
/// 
/// This struct defines the thresholds and timeouts that control how the circuit breaker
/// transitions between states.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiCircuitBreakerConfig {
    /// Number of consecutive failures required to open the circuit breaker from Closed state.
    pub failure_threshold: u32,
    
    /// Number of consecutive successes required to close the circuit breaker from HalfOpen state.
    pub success_threshold: u32,
    
    /// Time in seconds to wait in Open state before transitioning to HalfOpen state.
    pub timeout_seconds: u64,
    
    /// Time window in seconds for counting failures. This defines how long failures are remembered.
    pub monitoring_period_seconds: u64,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiCircuitBreakerConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    #[staticmethod]
    fn py_new_with_values(failure_threshold: u32, success_threshold: u32, timeout_seconds: u64, monitoring_period_seconds: u64) -> Self {
        Self {
            failure_threshold,
            success_threshold,
            timeout_seconds,
            monitoring_period_seconds,
        }
    }
    
    fn get_failure_threshold(&self) -> u32 {
        self.failure_threshold
    }
    
    fn set_failure_threshold(&mut self, value: u32) {
        self.failure_threshold = value;
    }
    
    fn get_success_threshold(&self) -> u32 {
        self.success_threshold
    }
    
    fn set_success_threshold(&mut self, value: u32) {
        self.success_threshold = value;
    }
    
    fn get_timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }
    
    fn set_timeout_seconds(&mut self, value: u64) {
        self.timeout_seconds = value;
    }
    
    fn get_monitoring_period_seconds(&self) -> u64 {
        self.monitoring_period_seconds
    }
    
    fn set_monitoring_period_seconds(&mut self, value: u64) {
        self.monitoring_period_seconds = value;
    }
}

impl Default for RiCircuitBreakerConfig {
    /// Creates a default configuration for the circuit breaker.
    /// 
    /// Default values:
    /// - failure_threshold: 5 consecutive failures to open
    /// - success_threshold: 3 consecutive successes to close
    /// - timeout_seconds: 60 seconds before trying recovery
    /// - monitoring_period_seconds: 30 seconds failure window
    fn default() -> Self {
        Self {
            failure_threshold: 5,
            success_threshold: 3,
            timeout_seconds: 60,
            monitoring_period_seconds: 30,
        }
    }
}

/// Internal circuit breaker statistics and state management.
/// 
/// This struct tracks all the metrics and state transitions for a circuit breaker instance.
/// It is designed to be thread-safe for use in multi-threaded environments.
#[derive(Debug)]
struct CircuitBreakerStats {
    /// Current state of the circuit breaker (Closed, Open, HalfOpen)
    state: RwLock<RiCircuitBreakerState>,
    
    /// Total count of failures since the circuit breaker was created
    failure_count: AtomicUsize,
    
    /// Total count of successes since the circuit breaker was created
    success_count: AtomicUsize,
    
    /// Timestamp of the last failure, if any
    last_failure_time: RwLock<Option<Instant>>,
    
    /// Timestamp of the last state change
    last_state_change: RwLock<Instant>,
    
    /// Number of consecutive failures in the current sequence
    consecutive_failures: AtomicUsize,
    
    /// Number of consecutive successes in the current sequence
    consecutive_successes: AtomicUsize,
}

impl CircuitBreakerStats {
    /// Creates a new circuit breaker statistics instance with default values.
    /// 
    /// Initial state is Closed, with all counters set to zero.
    #[allow(dead_code)]
    fn new() -> Self {
        Self {
            state: RwLock::new(RiCircuitBreakerState::Closed),
            failure_count: AtomicUsize::new(0),
            success_count: AtomicUsize::new(0),
            last_failure_time: RwLock::new(None),
            last_state_change: RwLock::new(Instant::now()),
            consecutive_failures: AtomicUsize::new(0),
            consecutive_successes: AtomicUsize::new(0),
        }
    }

    /// Records a successful operation and updates the circuit breaker state if necessary.
    /// 
    /// - Increments total success count
    /// - Resets consecutive failure count
    /// - Increments consecutive success count
    /// - Transitions from HalfOpen to Closed if success threshold is met
    async fn record_success(&self, config: &RiCircuitBreakerConfig) {
        self.success_count.fetch_add(1, Ordering::Relaxed);
        self.consecutive_failures.store(0, Ordering::Relaxed);
        self.consecutive_successes.fetch_add(1, Ordering::Relaxed);

        let state = self.state.read().await;
        match *state {
            RiCircuitBreakerState::HalfOpen => {
                let successes = self.consecutive_successes.load(Ordering::Relaxed);
                if successes >= config.success_threshold as usize {
                    drop(state);
                    let mut state_write = self.state.write().await;
                    *state_write = RiCircuitBreakerState::Closed;
                    self.consecutive_successes.store(0, Ordering::Relaxed);
                    self.failure_count.store(0, Ordering::Relaxed);
                    *self.last_state_change.write().await = Instant::now();
                }
            }
            RiCircuitBreakerState::Open => {
            }
            RiCircuitBreakerState::Closed => {
                self.consecutive_failures.store(0, Ordering::Relaxed);
            }
        }
    }

    /// Records a failed operation and updates the circuit breaker state if necessary.
    /// 
    /// - Increments total failure count
    /// - Resets consecutive success count
    /// - Increments consecutive failure count
    /// - Updates last failure time
    /// - Transitions from Closed to Open if failure threshold is met
    /// - Transitions from HalfOpen to Open on any failure
    async fn record_failure(&self, config: &RiCircuitBreakerConfig) {
        self.failure_count.fetch_add(1, Ordering::Relaxed);
        self.consecutive_successes.store(0, Ordering::Relaxed);
        self.consecutive_failures.fetch_add(1, Ordering::Relaxed);
        *self.last_failure_time.write().await = Some(Instant::now());

        let state = self.state.read().await;
        match *state {
            RiCircuitBreakerState::Closed => {
                let failures = self.consecutive_failures.load(Ordering::Relaxed);
                if failures >= config.failure_threshold as usize {
                    drop(state);
                    let mut state_write = self.state.write().await;
                    *state_write = RiCircuitBreakerState::Open;
                    self.consecutive_failures.store(0, Ordering::Relaxed);
                    *self.last_state_change.write().await = Instant::now();
                }
            }
            RiCircuitBreakerState::HalfOpen => {
                // Any failure in HalfOpen state immediately opens the circuit
                drop(state);
                let mut state_write = self.state.write().await;
                *state_write = RiCircuitBreakerState::Open;
                *self.last_state_change.write().await = Instant::now();
            }
            RiCircuitBreakerState::Open => {
                // Already open - no state change needed
            }
        }
    }

    /// Determines if the circuit breaker should attempt to reset from Open to HalfOpen state.
    /// 
    /// Checks if the timeout period has elapsed since the last state change to Open.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The circuit breaker configuration containing the timeout setting
    /// 
    /// # Returns
    /// 
    /// `true` if the timeout has elapsed and a reset should be attempted, `false` otherwise
    async fn should_attempt_reset(&self, config: &RiCircuitBreakerConfig) -> bool {
        let state = self.state.read().await;
        if let RiCircuitBreakerState::Open = *state {
            let last_change = *self.last_state_change.read().await;
            Instant::now().duration_since(last_change) >= Duration::from_secs(config.timeout_seconds)
        } else {
            false
        }
    }

    /// Transitions the circuit breaker from Open to HalfOpen state.
    /// 
    /// This method is called when the timeout period has elapsed and the circuit breaker
    /// should test if the service has recovered.
    async fn transition_to_half_open(&self) {
        let mut state = self.state.write().await;
        *state = RiCircuitBreakerState::HalfOpen;
        *self.last_state_change.write().await = Instant::now();
    }

    /// Gets the current state of the circuit breaker.
    /// 
    /// # Returns
    /// 
    /// The current `RiCircuitBreakerState` (Closed, Open, or HalfOpen)
    async fn get_state(&self) -> RiCircuitBreakerState {
        self.state.read().await.clone()
    }

    /// Gets the current metrics for the circuit breaker.
    /// 
    /// This method provides comprehensive circuit breaker statistics including success/failure counts,
    /// consecutive streaks, and current state information for monitoring and alerting purposes.
    /// 
    /// # Returns
    /// 
    /// A `RiCircuitBreakerMetrics` struct containing the current statistics
    #[allow(dead_code)]
    fn get_stats(&self) -> RiCircuitBreakerMetrics {
        let state_str = match self.state.blocking_read().clone() {
            RiCircuitBreakerState::Closed => "Closed",
            RiCircuitBreakerState::Open => "Open",
            RiCircuitBreakerState::HalfOpen => "HalfOpen",
        };

        RiCircuitBreakerMetrics {
            state: state_str.to_string(),
            failure_count: self.failure_count.load(Ordering::Relaxed),
            success_count: self.success_count.load(Ordering::Relaxed),
            consecutive_failures: self.consecutive_failures.load(Ordering::Relaxed),
            consecutive_successes: self.consecutive_successes.load(Ordering::Relaxed),
        }
    }
}

/// Metrics for monitoring circuit breaker performance.
///
/// This struct contains statistics about the circuit breaker's performance, including
/// success and failure counts, consecutive success/failure streaks, and current state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiCircuitBreakerMetrics {
    /// Current state of the circuit breaker as a string
    pub state: String,

    /// Total number of failures since the circuit breaker was created
    pub failure_count: usize,

    /// Total number of successes since the circuit breaker was created
    pub success_count: usize,

    /// Number of consecutive failures in the current sequence
    pub consecutive_failures: usize,

    /// Number of consecutive successes in the current sequence
    pub consecutive_successes: usize,
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl RiCircuitBreakerMetrics {
    #[new]
    fn py_new(state: String, failure_count: usize, success_count: usize, consecutive_failures: usize, consecutive_successes: usize) -> Self {
        Self {
            state,
            failure_count,
            success_count,
            consecutive_failures,
            consecutive_successes,
        }
    }
    
    fn get_state(&self) -> &str {
        &self.state
    }
    
    fn get_failure_count(&self) -> usize {
        self.failure_count
    }
    
    fn get_success_count(&self) -> usize {
        self.success_count
    }
    
    fn get_consecutive_failures(&self) -> usize {
        self.consecutive_failures
    }
    
    fn get_consecutive_successes(&self) -> usize {
        self.consecutive_successes
    }
}

/// Basic circuit breaker implementation.
/// 
/// This struct provides a thread-safe circuit breaker that protects against cascading failures
/// by monitoring the success and failure patterns of operations and transitioning between states
/// (Closed, Open, HalfOpen) based on configurable thresholds.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct RiCircuitBreaker {
    /// Configuration for the circuit breaker behavior
    config: RiCircuitBreakerConfig,
    
    /// Internal statistics and state management
    stats: Arc<CircuitBreakerStats>,
}

impl RiCircuitBreaker {
    /// Creates a new circuit breaker with the specified configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The configuration for the circuit breaker behavior
    /// 
    /// # Returns
    /// 
    /// A new `RiCircuitBreaker` instance
    pub fn new(config: RiCircuitBreakerConfig) -> Self {
        Self {
            config,
            stats: Arc::new(CircuitBreakerStats::new()),
        }
    }

    /// Determines if a request should be allowed to proceed based on the current circuit breaker state.
    /// 
    /// - **Closed**: Always allows requests
    /// - **Open**: Rejects requests unless timeout has elapsed, then transitions to HalfOpen
    /// - **HalfOpen**: Allows limited requests to test service health
    /// 
    /// # Returns
    /// 
    /// `true` if the request should be allowed, `false` otherwise
    pub fn allow_request(&self) -> bool {
        let state = futures::executor::block_on(async {
            let state = self.stats.state.read().await;
            state.clone()
        });
        
        match state {
            RiCircuitBreakerState::Closed => true,
            RiCircuitBreakerState::Open => {
                let last_change = futures::executor::block_on(async {
                    let guard = self.stats.last_state_change.read().await;
                    *guard
                });
                if last_change.elapsed() >= Duration::from_secs(self.config.timeout_seconds) {
                    futures::executor::block_on(async {
                        let mut state = self.stats.state.write().await;
                        *state = RiCircuitBreakerState::HalfOpen;
                        *self.stats.last_state_change.write().await = Instant::now();
                    });
                    true
                } else {
                    false
                }
            }
            RiCircuitBreakerState::HalfOpen => true,
        }
    }

    /// Records a successful operation and updates the circuit breaker state if necessary.
    pub fn record_success(&self) {
        futures::executor::block_on(async {
            self.stats.record_success(&self.config).await;
        });
    }

    /// Records a failed operation and updates the circuit breaker state if necessary.
    pub fn record_failure(&self) {
        futures::executor::block_on(async {
            self.stats.record_failure(&self.config).await;
        });
    }

    /// Executes an operation with circuit breaker protection.
    /// 
    /// This method wraps an async operation and automatically handles success/failure recording
    /// and state transitions based on the operation's result.
    /// 
    /// # Type Parameters
    /// 
    /// - `F`: The async future type representing the operation
    /// - `R`: The result type of the operation
    /// 
    /// # Parameters
    /// 
    /// - `operation`: The async operation to execute with circuit breaker protection
    /// 
    /// # Returns
    /// 
    /// The result of the operation, or an error if the circuit breaker is open
    pub async fn execute<F, R>(&self, operation: F) -> RiResult<R>
    where
        F: std::future::Future<Output = RiResult<R>>,
    {
        if !self.allow_request() {
            return Err(crate::core::RiError::ServiceMesh("Circuit breaker is open".to_string()));
        }

        match operation.await {
            Ok(result) => {
                self.record_success();
                Ok(result)
            }
            Err(error) => {
                self.record_failure();
                Err(error)
            }
        }
    }

    /// Gets the current state of the circuit breaker.
    /// 
    /// # Returns
    /// 
    /// The current `RiCircuitBreakerState` (Closed, Open, or HalfOpen)
    pub fn get_state(&self) -> RiCircuitBreakerState {
        futures::executor::block_on(async {
            self.stats.get_state().await
        })
    }

    /// Gets the current metrics for the circuit breaker.
    ///
    /// # Returns
    ///
    /// A `RiCircuitBreakerMetrics` struct containing the current statistics
    pub fn get_stats(&self) -> RiCircuitBreakerMetrics {
        let state_str = match futures::executor::block_on(async {
            let state = self.stats.state.read().await;
            state.clone()
        }) {
            RiCircuitBreakerState::Closed => "Closed",
            RiCircuitBreakerState::Open => "Open",
            RiCircuitBreakerState::HalfOpen => "HalfOpen",
        };

        RiCircuitBreakerMetrics {
            state: state_str.to_string(),
            failure_count: self.stats.failure_count.load(Ordering::Relaxed),
            success_count: self.stats.success_count.load(Ordering::Relaxed),
            consecutive_failures: self.stats.consecutive_failures.load(Ordering::Relaxed),
            consecutive_successes: self.stats.consecutive_successes.load(Ordering::Relaxed),
        }
    }

    /// Gets the configuration for the circuit breaker.
    /// 
    /// # Returns
    /// 
    /// A reference to the `RiCircuitBreakerConfig` used by this circuit breaker
    pub fn get_config(&self) -> RiCircuitBreakerConfig {
        self.config.clone()
    }

    /// Resets the circuit breaker to its initial state (Closed).
    /// 
    /// This method resets all counters and transitions the circuit breaker to Closed state.
    pub fn reset(&self) {
        futures::executor::block_on(async move {
            let mut state = self.stats.state.write().await;
            *state = RiCircuitBreakerState::Closed;
            self.stats.failure_count.store(0, Ordering::Relaxed);
            self.stats.success_count.store(0, Ordering::Relaxed);
            self.stats.consecutive_failures.store(0, Ordering::Relaxed);
            self.stats.consecutive_successes.store(0, Ordering::Relaxed);
            *self.stats.last_state_change.write().await = Instant::now();
        });
    }
    
    /// Forces the circuit breaker to transition to Open state.
    /// 
    /// This method immediately opens the circuit breaker, rejecting all requests until the timeout elapses.
    pub fn force_open(&self) {
        futures::executor::block_on(async move {
            let mut state = self.stats.state.write().await;
            *state = RiCircuitBreakerState::Open;
            *self.stats.last_state_change.write().await = Instant::now();
        });
    }
    
    /// Forces the circuit breaker to transition to Closed state.
    /// 
    /// This method immediately closes the circuit breaker, allowing all requests to proceed.
    pub fn force_close(&self) {
        futures::executor::block_on(async move {
            let mut state = self.stats.state.write().await;
            *state = RiCircuitBreakerState::Closed;
            *self.stats.last_state_change.write().await = Instant::now();
        });
    }

    pub fn failure_rate(&self) -> f64 {
        let failures = self.stats.failure_count.load(Ordering::Relaxed);
        let successes = self.stats.success_count.load(Ordering::Relaxed);
        let total = failures + successes;
        if total == 0 {
            0.0
        } else {
            failures as f64 / total as f64
        }
    }
    
    pub fn success_rate(&self) -> f64 {
        let failures = self.stats.failure_count.load(Ordering::Relaxed);
        let successes = self.stats.success_count.load(Ordering::Relaxed);
        let total = failures + successes;
        if total == 0 {
            1.0
        } else {
            successes as f64 / total as f64
        }
    }
    
    pub fn total_requests(&self) -> usize {
        self.stats.failure_count.load(Ordering::Relaxed) + self.stats.success_count.load(Ordering::Relaxed)
    }

    pub fn is_open(&self) -> bool {
        let state = futures::executor::block_on(self.stats.state.read());
        matches!(*state, RiCircuitBreakerState::Open)
    }

    pub fn is_closed(&self) -> bool {
        let state = futures::executor::block_on(self.stats.state.read());
        matches!(*state, RiCircuitBreakerState::Closed)
    }

    pub fn is_half_open(&self) -> bool {
        let state = futures::executor::block_on(self.stats.state.read());
        matches!(*state, RiCircuitBreakerState::HalfOpen)
    }
}

/// Advanced circuit breaker with separate failure thresholds for different error types.
/// 
/// This struct extends the basic circuit breaker functionality by maintaining separate statistics
/// for different error types, allowing for more granular control over circuit breaker behavior.
pub struct RiAdvancedCircuitBreaker {
    /// Configuration for the circuit breaker behavior
    config: RiCircuitBreakerConfig,
    
    /// Error-type specific statistics and state management
    stats_by_error: RwLock<HashMap<String, Arc<CircuitBreakerStats>>>,
    
    /// Default statistics for unclassified errors
    default_stats: Arc<CircuitBreakerStats>,
}

impl RiAdvancedCircuitBreaker {
    /// Creates a new advanced circuit breaker with the specified configuration.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The configuration for the circuit breaker behavior
    /// 
    /// # Returns
    /// 
    /// A new `RiAdvancedCircuitBreaker` instance
    pub fn new(config: RiCircuitBreakerConfig) -> Self {
        Self {
            config,
            stats_by_error: RwLock::new(HashMap::new()),
            default_stats: Arc::new(CircuitBreakerStats::new()),
        }
    }

    /// Gets the statistics instance for a specific error type, creating it if necessary.
    /// 
    /// # Parameters
    /// 
    /// - `error_type`: The error type identifier, or `None` for default statistics
    /// 
    /// # Returns
    /// 
    /// An `Arc<CircuitBreakerStats>` instance for the specified error type
    async fn get_stats_for_error(&self, error_type: Option<&str>) -> Arc<CircuitBreakerStats> {
        match error_type {
            Some(error_type) => {
                let mut stats_map = self.stats_by_error.write().await;
                stats_map.entry(error_type.to_string())
                    .or_insert_with(|| Arc::new(CircuitBreakerStats::new()))
                    .clone()
            }
            None => self.default_stats.clone(),
        }
    }

    /// Records a successful operation for a specific error type and updates the circuit breaker state if necessary.
    /// 
    /// # Parameters
    /// 
    /// - `error_type`: The error type identifier, or `None` for default statistics
    pub async fn record_success_with_type(&self, error_type: Option<&str>) {
        let stats = self.get_stats_for_error(error_type).await;
        stats.record_success(&self.config).await;
    }

    /// Records a failed operation for a specific error type and updates the circuit breaker state if necessary.
    /// 
    /// # Parameters
    /// 
    /// - `error_type`: The error type identifier, or `None` for default statistics
    pub async fn record_failure_with_type(&self, error_type: Option<&str>) {
        let stats = self.get_stats_for_error(error_type).await;
        stats.record_failure(&self.config).await;
    }

    /// Determines if a request should be allowed to proceed for a specific error type.
    /// 
    /// # Parameters
    /// 
    /// - `error_type`: The error type identifier, or `None` for default statistics
    /// 
    /// # Returns
    /// 
    /// `true` if the request should be allowed, `false` otherwise
    pub async fn allow_request_for_type(&self, error_type: Option<&str>) -> bool {
        let stats = self.get_stats_for_error(error_type).await;
        let state = stats.get_state().await;
        
        match state {
            RiCircuitBreakerState::Closed => true,
            RiCircuitBreakerState::Open => {
                if stats.should_attempt_reset(&self.config).await {
                    stats.transition_to_half_open().await;
                    true
                } else {
                    false
                }
            }
            RiCircuitBreakerState::HalfOpen => true,
        }
    }
}
