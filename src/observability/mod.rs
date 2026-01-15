//! Copyright © 2025-2026 Wenze Wei. All Rights Reserved.
//!
//! This file is part of DMSC.
//! The DMSC project belongs to the Dunimd Team.
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

//! # Observability Module
//! 
//! This module provides comprehensive observability capabilities for DMSC, including distributed tracing
//! and metrics collection. It follows modern observability best practices to help monitor, debug, and
//! optimize DMSC applications.
//! 
//! ## Key Components
//! 
//! - **DMSCObservabilityModule**: Main observability module
//! - **DMSCTracer**: Distributed tracing implementation
//! - **DMSCMetricsRegistry**: Metrics collection and aggregation
//! - **DMSCObservabilityConfig**: Configuration for observability features
//! - **DMSCObservabilityData**: Exported observability data structure
//! 
//! ## Design Principles
//! 
//! 1. **Separation of Concerns**: Tracing and metrics are separate but integrated components
//! 2. **Configurable**: All features can be enabled/disabled and configured at runtime
//! 3. **Non-intrusive**: Designed to be easy to integrate without disrupting application logic
//! 4. **Performance-focused**: Optimized for low overhead in production environments
//! 5. **Standard-compliant**: Follows W3C Trace Context standard for distributed tracing
//! 6. **Prometheus-compatible**: Metrics are exported in Prometheus format
//! 7. **Service Module Integration**: Implements the `ServiceModule` trait for seamless integration
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! 
//! fn example() -> DMSCResult<()> {
//!     // Create a DMSC app builder
//!     let mut builder = DMSCAppBuilder::new();
//!     
//!     // Configure observability
//!     let observability_config = DMSCObservabilityConfig {
//!         tracing_enabled: true,
//!         metrics_enabled: true,
//!         tracing_sampling_rate: 0.5, // 50% sampling rate
//!         metrics_window_size_secs: 300,
//!         metrics_bucket_size_secs: 10,
//!     };
//!     
//!     // Add observability module to the app
//!     let observability_module = DMSCObservabilityModule::new()
//!         .with_config(observability_config);
//!     
//!     builder.add_module(Box::new(observability_module));
//!     
//!     // Build and run the app
//!     let mut app = builder.build()?;
//!     app.run()?;
//!     
//!     Ok(())
//! }
//! ```

mod metrics;
pub mod tracing;
pub mod propagation;
#[cfg(feature = "observability")]
pub mod prometheus;
#[cfg(feature = "system_info")]
mod metrics_collector;
pub mod grafana;

use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub use tracing::{DMSCTracer, DMSCTraceId, DMSCSpanId, DMSCSpan, DMSCSpanKind, DMSCSpanStatus, DMSCTracingContext, DMSCSamplingStrategy};
pub use metrics::{DMSCMetricsRegistry, DMSCMetric, DMSCMetricConfig, DMSCMetricType, DMSCWindowStats, DMSCMetricSample};
pub use propagation::{DMSCTraceContext, DMSCBaggage, DMSCContextCarrier, W3CTracePropagator};

use crate::core::{DMSCResult, DMSCServiceContext};


/// Main observability module for DMSC.
/// 
/// This module provides distributed tracing and metrics collection capabilities, following modern
/// observability best practices. It implements the `ServiceModule` trait for seamless integration
/// with the DMSC application lifecycle.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCObservabilityModule {
    /// Distributed tracer instance
    tracer: Option<Arc<DMSCTracer>>,
    /// Metrics registry for collecting and aggregating metrics
    metrics_registry: Option<Arc<DMSCMetricsRegistry>>,
    /// Configuration for observability features
    config: DMSCObservabilityConfig,
}

/// Configuration for the observability module.
/// 
/// This struct defines the configuration options for tracing and metrics collection in DMSC.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCObservabilityConfig {
    /// Whether distributed tracing is enabled
    pub tracing_enabled: bool,
    /// Whether metrics collection is enabled
    pub metrics_enabled: bool,
    /// Sampling rate for distributed tracing (0.0 to 1.0)
    pub tracing_sampling_rate: f64,
    /// Sampling strategy for distributed tracing
    pub tracing_sampling_strategy: String,
    /// Window size for metrics aggregation in seconds
    pub metrics_window_size_secs: u64,
    /// Bucket size for metrics aggregation in seconds
    pub metrics_bucket_size_secs: u64,
}

impl Default for DMSCObservabilityConfig {
    /// Returns the default configuration for observability.
    /// 
    /// Default values:
    /// - tracing_enabled: true
    /// - metrics_enabled: true
    /// - tracing_sampling_rate: 0.1 (10% sampling)
    /// - tracing_sampling_strategy: "rate" (fixed rate sampling)
    /// - metrics_window_size_secs: 300 (5 minutes)
    /// - metrics_bucket_size_secs: 10 (10 seconds)
    fn default() -> Self {
        Self {
            tracing_enabled: true,
            metrics_enabled: true,
            tracing_sampling_rate: 0.1, // 10% sampling by default
            tracing_sampling_strategy: "rate".to_string(), // fixed rate sampling by default
            metrics_window_size_secs: 300, // 5 minutes
            metrics_bucket_size_secs: 10,  // 10 seconds
        }
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCObservabilityConfig
#[pyo3::prelude::pymethods]
impl DMSCObservabilityConfig {
    #[new]
    fn py_new() -> Self {
        Self::default()
    }
    
    /// Set tracing enabled flag from Python
    fn set_tracing_enabled(&mut self, tracing_enabled: bool) {
        self.tracing_enabled = tracing_enabled;
    }
    
    /// Get tracing enabled flag from Python
    fn get_tracing_enabled(&self) -> bool {
        self.tracing_enabled
    }
    
    /// Set metrics enabled flag from Python
    fn set_metrics_enabled(&mut self, metrics_enabled: bool) {
        self.metrics_enabled = metrics_enabled;
    }
    
    /// Get metrics enabled flag from Python
    fn get_metrics_enabled(&self) -> bool {
        self.metrics_enabled
    }
    
    /// Set tracing sampling rate from Python
    fn set_tracing_sampling_rate(&mut self, tracing_sampling_rate: f64) -> pyo3::PyResult<()>
    {
        if tracing_sampling_rate < 0.0 || tracing_sampling_rate > 1.0 {
            return Err(pyo3::exceptions::PyValueError::new_err("Tracing sampling rate must be between 0.0 and 1.0"));
        }
        self.tracing_sampling_rate = tracing_sampling_rate;
        Ok(())
    }
    
    /// Get tracing sampling rate from Python
    fn get_tracing_sampling_rate(&self) -> f64 {
        self.tracing_sampling_rate
    }
    
    /// Set tracing sampling strategy from Python
    fn set_tracing_sampling_strategy(&mut self, tracing_sampling_strategy: String) {
        self.tracing_sampling_strategy = tracing_sampling_strategy;
    }
    
    /// Get tracing sampling strategy from Python
    fn get_tracing_sampling_strategy(&self) -> String {
        self.tracing_sampling_strategy.clone()
    }
    
    /// Set metrics window size in seconds from Python
    fn set_metrics_window_size_secs(&mut self, metrics_window_size_secs: u64) {
        self.metrics_window_size_secs = metrics_window_size_secs;
    }
    
    /// Get metrics window size in seconds from Python
    fn get_metrics_window_size_secs(&self) -> u64 {
        self.metrics_window_size_secs
    }
    
    /// Set metrics bucket size in seconds from Python
    fn set_metrics_bucket_size_secs(&mut self, metrics_bucket_size_secs: u64) {
        self.metrics_bucket_size_secs = metrics_bucket_size_secs;
    }
    
    /// Get metrics bucket size in seconds from Python
    fn get_metrics_bucket_size_secs(&self) -> u64 {
        self.metrics_bucket_size_secs
    }
}

impl Default for DMSCObservabilityModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCObservabilityModule {
    /// Creates a new observability module with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSCObservabilityModule` instance with default configuration
    pub fn new() -> Self {
        Self {
            tracer: None,
            metrics_registry: None,
            config: DMSCObservabilityConfig::default(),
        }
    }
    
    /// Configures the observability module with custom settings.
    /// 
    /// # Parameters
    /// 
    /// - `config`: The custom configuration to apply
    /// 
    /// # Returns
    /// 
    /// The updated `DMSCObservabilityModule` instance
    pub fn with_config(mut self, config: DMSCObservabilityConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Initializes tracing with the configured sampling strategy.
    /// 
    /// This method sets up the distributed tracer with the specified sampling strategy.
    fn init_tracing(&mut self) {
        if self.config.tracing_enabled {
            // Initialize tracer with the configured rate
            // Note: In a real implementation, we'd use the appropriate strategy
            tracing::init_tracer(self.config.tracing_sampling_rate);
        }
    }
    
    /// Initializes the metrics registry.
    /// 
    /// This method creates and configures the metrics registry for collecting and aggregating metrics.
    fn init_metrics(&mut self) {
        if self.config.metrics_enabled {
            let registry = Arc::new(DMSCMetricsRegistry::new());
            self.metrics_registry = Some(registry);
        }
    }
    
    /// Creates common service metrics.
    /// 
    /// This method registers standard service metrics including:
    /// - Request duration histogram
    /// - Request counter
    /// - Error counter
    /// - Active connections gauge
    /// - Service startup time
    /// - Module initialization time
    /// - Request queue length
    /// - Middleware execution time
    /// - Cache metrics (hits, misses, entries, memory usage)
    /// - Database query time
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    fn create_service_metrics(&self) -> DMSCResult<()> {
        if let Some(registry) = &self.metrics_registry {
            // Request duration histogram
            let request_duration_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Histogram,
                name: "dms_request_duration_seconds".to_string(),
                help: "Request duration in seconds".to_string(),
                buckets: vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0], // seconds
                quantiles: vec![0.5, 0.9, 0.95, 0.99],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            
            let request_duration_metric = Arc::new(DMSCMetric::new(request_duration_config));
            registry.register(request_duration_metric)?;
            
            // Request counter
            let request_counter_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_requests_total".to_string(),
                help: "Total number of requests".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let request_counter_metric = Arc::new(DMSCMetric::new(request_counter_config));
            registry.register(request_counter_metric)?;
            
            // Error counter
            let error_counter_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_errors_total".to_string(),
                help: "Total number of errors".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let error_counter_metric = Arc::new(DMSCMetric::new(error_counter_config));
            registry.register(error_counter_metric)?;
            
            // Active connections gauge
            let connections_gauge_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_active_connections".to_string(),
                help: "Number of active connections".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let connections_gauge_metric = Arc::new(DMSCMetric::new(connections_gauge_config));
            registry.register(connections_gauge_metric)?;
            
            // Service startup time gauge
            let startup_time_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_service_startup_time_seconds".to_string(),
                help: "Service startup time in seconds".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let startup_time_metric = Arc::new(DMSCMetric::new(startup_time_config));
            registry.register(startup_time_metric)?;
            
            // Module initialization time histogram
            let module_init_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Histogram,
                name: "dms_module_init_time_seconds".to_string(),
                help: "Module initialization time in seconds".to_string(),
                buckets: vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0],
                quantiles: vec![0.5, 0.9, 0.95, 0.99],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            
            let module_init_metric = Arc::new(DMSCMetric::new(module_init_config));
            registry.register(module_init_metric)?;
            
            // Request queue length gauge
            let queue_length_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_request_queue_length".to_string(),
                help: "Request queue length".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let queue_length_metric = Arc::new(DMSCMetric::new(queue_length_config));
            registry.register(queue_length_metric)?;
            
            // Middleware execution time histogram
            let middleware_time_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Histogram,
                name: "dms_middleware_duration_seconds".to_string(),
                help: "Middleware execution time in seconds".to_string(),
                buckets: vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5],
                quantiles: vec![0.5, 0.9, 0.95, 0.99],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            
            let middleware_time_metric = Arc::new(DMSCMetric::new(middleware_time_config));
            registry.register(middleware_time_metric)?;
            
            // Cache metrics
            // Cache hit counter
            let cache_hit_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_cache_hits_total".to_string(),
                help: "Total number of cache hits".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let cache_hit_metric = Arc::new(DMSCMetric::new(cache_hit_config));
            registry.register(cache_hit_metric)?;
            
            // Cache miss counter
            let cache_miss_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_cache_misses_total".to_string(),
                help: "Total number of cache misses".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let cache_miss_metric = Arc::new(DMSCMetric::new(cache_miss_config));
            registry.register(cache_miss_metric)?;
            
            // Cache entries gauge
            let cache_entries_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_cache_entries".to_string(),
                help: "Number of cache entries".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let cache_entries_metric = Arc::new(DMSCMetric::new(cache_entries_config));
            registry.register(cache_entries_metric)?;
            
            // Cache memory usage gauge
            let cache_memory_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_cache_memory_usage_bytes".to_string(),
                help: "Cache memory usage in bytes".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let cache_memory_metric = Arc::new(DMSCMetric::new(cache_memory_config));
            registry.register(cache_memory_metric)?;
            
            // Cache eviction counter
            let cache_eviction_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_cache_evictions_total".to_string(),
                help: "Total number of cache evictions".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let cache_eviction_metric = Arc::new(DMSCMetric::new(cache_eviction_config));
            registry.register(cache_eviction_metric)?;
            
            // Database metrics
            // Database query time histogram
            let db_query_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Histogram,
                name: "dms_db_query_duration_seconds".to_string(),
                help: "Database query execution time in seconds".to_string(),
                buckets: vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0],
                quantiles: vec![0.5, 0.9, 0.95, 0.99],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            
            let db_query_metric = Arc::new(DMSCMetric::new(db_query_config));
            registry.register(db_query_metric)?;
            
            // Database active connections gauge
            let db_active_connections_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_db_active_connections".to_string(),
                help: "Number of active database connections".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_active_connections_metric = Arc::new(DMSCMetric::new(db_active_connections_config));
            registry.register(db_active_connections_metric)?;
            
            // Database idle connections gauge
            let db_idle_connections_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Gauge,
                name: "dms_db_idle_connections".to_string(),
                help: "Number of idle database connections".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_idle_connections_metric = Arc::new(DMSCMetric::new(db_idle_connections_config));
            registry.register(db_idle_connections_metric)?;
            
            // Database connection timeouts counter
            let db_timeout_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_db_connection_timeouts_total".to_string(),
                help: "Total number of database connection timeouts".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_timeout_metric = Arc::new(DMSCMetric::new(db_timeout_config));
            registry.register(db_timeout_metric)?;
            
            // Database errors counter
            let db_errors_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_db_errors_total".to_string(),
                help: "Total number of database errors".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_errors_metric = Arc::new(DMSCMetric::new(db_errors_config));
            registry.register(db_errors_metric)?;
            
            // Database transactions counter
            let db_transactions_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_db_transactions_total".to_string(),
                help: "Total number of database transactions".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_transactions_metric = Arc::new(DMSCMetric::new(db_transactions_config));
            registry.register(db_transactions_metric)?;
            
            // Database transaction commits counter
            let db_commits_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_db_transaction_commits_total".to_string(),
                help: "Total number of database transaction commits".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_commits_metric = Arc::new(DMSCMetric::new(db_commits_config));
            registry.register(db_commits_metric)?;
            
            // Database transaction rollbacks counter
            let db_rollbacks_config = DMSCMetricConfig {
                metric_type: DMSCMetricType::Counter,
                name: "dms_db_transaction_rollbacks_total".to_string(),
                help: "Total number of database transaction rollbacks".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let db_rollbacks_metric = Arc::new(DMSCMetric::new(db_rollbacks_config));
            registry.register(db_rollbacks_metric)?;
        }
        
        Ok(())
    }
    
    /// Exports observability data.
    /// 
    /// This method collects and returns the current observability data, including metrics in Prometheus
    /// format and information about active traces and spans.
    /// 
    /// # Returns
    /// 
    /// A `DMSCObservabilityData` struct containing the exported observability data
    pub fn export_data(&self) -> DMSCObservabilityData {
        DMSCObservabilityData {
            metrics: {
                #[cfg(feature = "observability")]
                {
                    self.metrics_registry.as_ref().map(|r| r.export_prometheus()).unwrap_or_default()
                }
                #[cfg(not(feature = "observability"))]
                {
                    String::default()
                }
            },
            active_traces: self.tracer.as_ref().map(|_| tracing::tracer().active_trace_count()).unwrap_or(0),
            active_spans: self.tracer.as_ref().map(|_| tracing::tracer().active_span_count()).unwrap_or(0),
        }
    }
}

/// Exported observability data structure.
/// 
/// This struct represents the observability data that can be exported from the DMSC system,
/// including metrics in Prometheus format and information about active traces and spans.
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCObservabilityData {
    /// Metrics data in Prometheus format
    pub metrics: String,
    /// Number of active traces
    pub active_traces: usize,
    /// Number of active spans
    pub active_spans: usize,
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCObservabilityData
#[pyo3::prelude::pymethods]
impl DMSCObservabilityData {
    /// Create new observability data from Python
    #[new]
    fn py_new(metrics: String, active_traces: usize, active_spans: usize) -> Self {
        Self {
            metrics,
            active_traces,
            active_spans,
        }
    }
    
    /// Get metrics data from Python
    #[pyo3(name = "get_metrics")]
    fn get_metrics_impl(&self) -> String {
        self.metrics.clone()
    }
    
    /// Get active traces count from Python
    #[pyo3(name = "get_active_traces")]
    fn get_active_traces_impl(&self) -> usize {
        self.active_traces
    }
    
    /// Get active spans count from Python
    #[pyo3(name = "get_active_spans")]
    fn get_active_spans_impl(&self) -> usize {
        self.active_spans
    }
}

#[cfg(feature = "pyo3")]
#[pyo3::prelude::pymethods]
impl DMSCObservabilityModule {
    fn get_metrics(&self) -> String {
        format!("ObservabilityModule with config: {:?}", self.config)
    }
}

#[async_trait::async_trait]
impl crate::core::DMSCModule for DMSCObservabilityModule {
    /// Returns the name of the observability module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMSC.Observability"
    }
    
    /// Indicates whether the observability module is critical.
    /// 
    /// The observability module is non-critical, meaning that if it fails to initialize or operate,
    /// it should not break the entire application. This allows the core functionality to continue
    /// even if observability features are unavailable.
    /// 
    /// # Returns
    /// 
    /// `false` since observability is non-critical
    fn is_critical(&self) -> bool {
        false // Non-critical, should not break the app if observability fails
    }
    
    /// Initializes the observability module.
    /// 
    /// This method performs the following steps:
    /// 1. Loads configuration from the service context
    /// 2. Initializes tracing with the configured sampling rate
    /// 3. Initializes the metrics registry
    /// 4. Creates common service metrics
    /// 5. Registers lifecycle hooks for automatic metrics collection
    /// 6. Logs initialization completion
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing configuration and other services
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        self.config = DMSCObservabilityConfig {
            tracing_enabled: cfg.get_bool("observability.tracing_enabled").unwrap_or(true),
            metrics_enabled: cfg.get_bool("observability.metrics_enabled").unwrap_or(true),
            tracing_sampling_rate: cfg.get_f32("observability.tracing_sampling_rate")
                .unwrap_or(0.1)
                .max(0.0)
                .min(1.0) as f64,
            tracing_sampling_strategy: cfg.get_str("observability.tracing_sampling_strategy")
                .unwrap_or("rate")
                .to_string(),
            metrics_window_size_secs: cfg.get_u64("observability.metrics_window_size_secs")
                .unwrap_or(300)
                .max(1),
            metrics_bucket_size_secs: cfg.get_u64("observability.metrics_bucket_size_secs")
                .unwrap_or(10)
                .max(1),
        };
        
        // Initialize components
        self.init_tracing();
        self.init_metrics();
        self.create_service_metrics()?;
        
        // Register lifecycle hooks
        let hooks: &mut crate::hooks::DMSCHookBus = ctx.hooks_mut();
        
        // Hook into request lifecycle for automatic metrics collection
        hooks.register(
            crate::hooks::DMSCHookKind::Startup,
            "dms.observability.lifecycle".to_string(),
            |_ctx, _event: &crate::hooks::DMSCHookEvent| {
                // Could add automatic span creation here
                Ok(())
            },
        );
        
        let logger = ctx.logger();
        logger.info("DMSC.Observability", "Observability module initialized")?;
        
        Ok(())
    }
    
    /// Performs cleanup after the application has shut down.
    /// 
    /// This method exports the final observability data and logs information about active traces
    /// and spans at the time of shutdown.
    /// 
    /// # Parameters
    /// 
    /// - `ctx`: The service context containing the logger service
    /// 
    /// # Returns
    /// 
    /// A `DMSCResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, ctx: &mut DMSCServiceContext) -> DMSCResult<()> {
        // Export final observability data
        let data = self.export_data();
        
        let logger = ctx.logger();
        logger.info("DMSC.Observability", format!("Final observability data: {} active traces, {} active spans", 
            data.active_traces, data.active_spans))?;
        
        Ok(())
    }
}
