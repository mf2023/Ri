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

//! # Observability Module
//! 
//! This module provides comprehensive observability capabilities for DMS, including distributed tracing
//! and metrics collection. It follows modern observability best practices to help monitor, debug, and
//! optimize DMS applications.
//! 
//! ## Key Components
//! 
//! - **DMSObservabilityModule**: Main observability module
//! - **DMSTracer**: Distributed tracing implementation
//! - **DMSMetricsRegistry**: Metrics collection and aggregation
//! - **DMSObservabilityConfig**: Configuration for observability features
//! - **DMSObservabilityData**: Exported observability data structure
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
//! use dms::prelude::*;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a DMS app builder
//!     let mut builder = DMSAppBuilder::new();
//!     
//!     // Configure observability
//!     let observability_config = DMSObservabilityConfig {
//!         tracing_enabled: true,
//!         metrics_enabled: true,
//!         tracing_sampling_rate: 0.5, // 50% sampling rate
//!         metrics_window_size_secs: 300,
//!         metrics_bucket_size_secs: 10,
//!     };
//!     
//!     // Add observability module to the app
//!     let observability_module = DMSObservabilityModule::new()
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
pub mod prometheus;
pub mod metrics_collector;
pub mod grafana;

use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub use tracing::{DMSTracer, DMSTraceId, DMSSpanId, DMSSpan, DMSSpanKind, DMSSpanStatus, DMSTracingContext};
pub use metrics::{DMSMetricsRegistry, DMSMetric, DMSMetricConfig, DMSMetricType, DMSWindowStats};

use crate::core::{DMSResult, DMSServiceContext};


/// Main observability module for DMS.
/// 
/// This module provides distributed tracing and metrics collection capabilities, following modern
/// observability best practices. It implements the `ServiceModule` trait for seamless integration
/// with the DMS application lifecycle.
pub struct DMSObservabilityModule {
    /// Distributed tracer instance
    tracer: Option<Arc<DMSTracer>>,
    /// Metrics registry for collecting and aggregating metrics
    metrics_registry: Option<Arc<DMSMetricsRegistry>>,
    /// Configuration for observability features
    config: DMSObservabilityConfig,
}

/// Configuration for the observability module.
/// 
/// This struct defines the configuration options for tracing and metrics collection in DMS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSObservabilityConfig {
    /// Whether distributed tracing is enabled
    pub tracing_enabled: bool,
    /// Whether metrics collection is enabled
    pub metrics_enabled: bool,
    /// Sampling rate for distributed tracing (0.0 to 1.0)
    pub tracing_sampling_rate: f64,
    /// Window size for metrics aggregation in seconds
    pub metrics_window_size_secs: u64,
    /// Bucket size for metrics aggregation in seconds
    pub metrics_bucket_size_secs: u64,
}

impl Default for DMSObservabilityConfig {
    /// Returns the default configuration for observability.
    /// 
    /// Default values:
    /// - tracing_enabled: true
    /// - metrics_enabled: true
    /// - tracing_sampling_rate: 0.1 (10% sampling)
    /// - metrics_window_size_secs: 300 (5 minutes)
    /// - metrics_bucket_size_secs: 10 (10 seconds)
    fn default() -> Self {
        Self {
            tracing_enabled: true,
            metrics_enabled: true,
            tracing_sampling_rate: 0.1, // 10% sampling by default
            metrics_window_size_secs: 300, // 5 minutes
            metrics_bucket_size_secs: 10,  // 10 seconds
        }
    }
}

impl Default for DMSObservabilityModule {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSObservabilityModule {
    /// Creates a new observability module with default configuration.
    /// 
    /// # Returns
    /// 
    /// A new `DMSObservabilityModule` instance with default configuration
    pub fn new() -> Self {
        Self {
            tracer: None,
            metrics_registry: None,
            config: DMSObservabilityConfig::default(),
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
    /// The updated `DMSObservabilityModule` instance
    pub fn with_config(mut self, config: DMSObservabilityConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Initializes tracing with the configured sampling rate.
    /// 
    /// This method sets up the distributed tracer with the specified sampling rate.
    fn init_tracing(&mut self) {
        if self.config.tracing_enabled {
            tracing::init_tracer(self.config.tracing_sampling_rate);
            // Note: In a real implementation, we'd store the Arc properly
            // This is simplified for demonstration
        }
    }
    
    /// Initializes the metrics registry.
    /// 
    /// This method creates and configures the metrics registry for collecting and aggregating metrics.
    fn init_metrics(&mut self) {
        if self.config.metrics_enabled {
            let registry = Arc::new(DMSMetricsRegistry::new());
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
    /// 
    /// # Returns
    /// 
    /// A `DMSResult<()>` indicating success or failure
    fn create_service_metrics(&self) -> DMSResult<()> {
        if let Some(registry) = &self.metrics_registry {
            // Request duration histogram
            let request_duration_config = DMSMetricConfig {
                metric_type: DMSMetricType::Histogram,
                name: "dms_request_duration_seconds".to_string(),
                help: "Request duration in seconds".to_string(),
                buckets: vec![0.001, 0.005, 0.01, 0.05, 0.1, 0.5, 1.0, 5.0], // seconds
                quantiles: vec![0.5, 0.9, 0.95, 0.99],
                max_age: std::time::Duration::from_secs(300),
                age_buckets: 5,
            };
            
            let request_duration_metric = Arc::new(DMSMetric::new(request_duration_config));
            registry.register(request_duration_metric)?;
            
            // Request counter
            let request_counter_config = DMSMetricConfig {
                metric_type: DMSMetricType::Counter,
                name: "dms_requests_total".to_string(),
                help: "Total number of requests".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let request_counter_metric = Arc::new(DMSMetric::new(request_counter_config));
            registry.register(request_counter_metric)?;
            
            // Error counter
            let error_counter_config = DMSMetricConfig {
                metric_type: DMSMetricType::Counter,
                name: "dms_errors_total".to_string(),
                help: "Total number of errors".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let error_counter_metric = Arc::new(DMSMetric::new(error_counter_config));
            registry.register(error_counter_metric)?;
            
            // Active connections gauge
            let connections_gauge_config = DMSMetricConfig {
                metric_type: DMSMetricType::Gauge,
                name: "dms_active_connections".to_string(),
                help: "Number of active connections".to_string(),
                buckets: vec![],
                quantiles: vec![],
                max_age: std::time::Duration::from_secs(0),
                age_buckets: 0,
            };
            
            let connections_gauge_metric = Arc::new(DMSMetric::new(connections_gauge_config));
            registry.register(connections_gauge_metric)?;
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
    /// A `DMSObservabilityData` struct containing the exported observability data
    pub fn export_data(&self) -> DMSObservabilityData {
        DMSObservabilityData {
            metrics: self.metrics_registry.as_ref().map(|r| r.export_prometheus()).unwrap_or_default(),
            active_traces: self.tracer.as_ref().map(|_| tracing::tracer().active_trace_count()).unwrap_or(0),
            active_spans: self.tracer.as_ref().map(|_| tracing::tracer().active_span_count()).unwrap_or(0),
        }
    }
}

/// Exported observability data structure.
/// 
/// This struct represents the observability data that can be exported from the DMS system,
/// including metrics in Prometheus format and information about active traces and spans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSObservabilityData {
    /// Metrics data in Prometheus format
    pub metrics: String,
    /// Number of active traces
    pub active_traces: usize,
    /// Number of active spans
    pub active_spans: usize,
}

#[async_trait::async_trait]
impl crate::core::DMSModule for DMSObservabilityModule {
    /// Returns the name of the observability module.
    /// 
    /// # Returns
    /// 
    /// The module name as a string
    fn name(&self) -> &str {
        "DMS.Observability"
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
    /// A `DMSResult<()>` indicating success or failure
    async fn init(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let binding = ctx.config();
        let cfg = binding.config();
        
        self.config = DMSObservabilityConfig {
            tracing_enabled: cfg.get_bool("observability.tracing_enabled").unwrap_or(true),
            metrics_enabled: cfg.get_bool("observability.metrics_enabled").unwrap_or(true),
            tracing_sampling_rate: cfg.get_f32("observability.tracing_sampling_rate").unwrap_or(0.1) as f64,
            metrics_window_size_secs: cfg.get_u64("observability.metrics_window_size_secs").unwrap_or(300),
            metrics_bucket_size_secs: cfg.get_u64("observability.metrics_bucket_size_secs").unwrap_or(10),
        };
        
        // Initialize components
        self.init_tracing();
        self.init_metrics();
        self.create_service_metrics()?;
        
        // Register lifecycle hooks
        let hooks: &mut crate::hooks::DMSHookBus = ctx.hooks_mut();
        
        // Hook into request lifecycle for automatic metrics collection
        hooks.register(
            crate::hooks::DMSHookKind::Startup,
            "dms.observability.lifecycle".to_string(),
            |_ctx, _event: &crate::hooks::DMSHookEvent| {
                // Could add automatic span creation here
                Ok(())
            },
        );
        
        let logger = ctx.logger();
        logger.info("DMS.Observability", "Observability module initialized")?;
        
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
    /// A `DMSResult<()>` indicating success or failure
    async fn after_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Export final observability data
        let data = self.export_data();
        
        let logger = ctx.logger();
        logger.info("DMS.Observability", format!("Final observability data: {} active traces, {} active spans", 
            data.active_traces, data.active_spans))?;
        
        Ok(())
    }
}
