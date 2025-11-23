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

pub mod metrics;
pub mod tracing;
pub mod propagation;
pub mod prometheus;
pub mod metrics_collector;
pub mod grafana;

use std::sync::Arc;
use serde::{Serialize, Deserialize};

pub use tracing::{DMSTracer, DMSTraceId, DMSSpanId, DMSSpan, DMSSpanKind, DMSSpanStatus, DMSTracingContext, _Finit_tracer, _Ftracer};
pub use metrics::{DMSMetricsRegistry, DMSMetric, DMSMetricConfig, DMSMetricType, DMSWindowStats};

use crate::core::{DMSResult, DMSServiceContext};


/// Observability module for DMS - provides distributed tracing and metrics collection
pub struct DMSObservabilityModule {
    tracer: Option<Arc<DMSTracer>>,
    metrics_registry: Option<Arc<DMSMetricsRegistry>>,
    config: DMSObservabilityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSObservabilityConfig {
    pub tracing_enabled: bool,
    pub metrics_enabled: bool,
    pub tracing_sampling_rate: f64,
    pub metrics_window_size_secs: u64,
    pub metrics_bucket_size_secs: u64,
}

impl Default for DMSObservabilityConfig {
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

impl DMSObservabilityModule {
    pub fn _Fnew() -> Self {
        Self {
            tracer: None,
            metrics_registry: None,
            config: DMSObservabilityConfig::default(),
        }
    }
    
    pub fn _Fwith_config(mut self, config: DMSObservabilityConfig) -> Self {
        self.config = config;
        self
    }
    
    /// Initialize tracing with configured sampling rate
    fn _Finit_tracing(&mut self) {
        if self.config.tracing_enabled {
            _Finit_tracer(self.config.tracing_sampling_rate);
            // Note: In a real implementation, we'd store the Arc properly
            // This is simplified for demonstration
        }
    }
    
    /// Initialize metrics registry
    fn _Finit_metrics(&mut self) {
        if self.config.metrics_enabled {
            let registry = Arc::new(DMSMetricsRegistry::_Fnew());
            self.metrics_registry = Some(registry);
        }
    }
    
    /// Create common service metrics
    fn _Fcreate_service_metrics(&self) -> DMSResult<()> {
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
            
            let request_duration_metric = Arc::new(DMSMetric::_Fnew(request_duration_config));
            registry._Fregister(request_duration_metric)?;
            
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
            
            let request_counter_metric = Arc::new(DMSMetric::_Fnew(request_counter_config));
            registry._Fregister(request_counter_metric)?;
            
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
            
            let error_counter_metric = Arc::new(DMSMetric::_Fnew(error_counter_config));
            registry._Fregister(error_counter_metric)?;
            
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
            
            let connections_gauge_metric = Arc::new(DMSMetric::_Fnew(connections_gauge_config));
            registry._Fregister(connections_gauge_metric)?;
        }
        
        Ok(())
    }
    
    /// Export observability data
    pub fn _Fexport_data(&self) -> DMSObservabilityData {
        DMSObservabilityData {
            metrics: self.metrics_registry.as_ref().map(|r| r._Fexport_prometheus()).unwrap_or_default(),
            active_traces: self.tracer.as_ref().map(|_| _Ftracer()._Factive_trace_count()).unwrap_or(0),
            active_spans: self.tracer.as_ref().map(|_| _Ftracer()._Factive_span_count()).unwrap_or(0),
        }
    }
}

/// Observability data export
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSObservabilityData {
    pub metrics: String,
    pub active_traces: usize,
    pub active_spans: usize,
}

impl crate::core::_CServiceModule for DMSObservabilityModule {
    fn _Fname(&self) -> &str {
        "DMS.Observability"
    }
    
    fn _Fis_critical(&self) -> bool {
        false // Non-critical, should not break the app if observability fails
    }
    
    fn _Finit(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Load configuration
        let cfg = ctx._Fconfig()._Fconfig();
        
        self.config = DMSObservabilityConfig {
            tracing_enabled: cfg._Fget_bool("observability.tracing_enabled").unwrap_or(true),
            metrics_enabled: cfg._Fget_bool("observability.metrics_enabled").unwrap_or(true),
            tracing_sampling_rate: cfg._Fget_f32("observability.tracing_sampling_rate").unwrap_or(0.1) as f64,
            metrics_window_size_secs: cfg._Fget_u64("observability.metrics_window_size_secs").unwrap_or(300),
            metrics_bucket_size_secs: cfg._Fget_u64("observability.metrics_bucket_size_secs").unwrap_or(10),
        };
        
        // Initialize components
        self._Finit_tracing();
        self._Finit_metrics();
        self._Fcreate_service_metrics()?;
        
        // Register lifecycle hooks
        let hooks: &mut crate::hooks::DMSHookBus = ctx._Fhooks_mut();
        
        // Hook into request lifecycle for automatic metrics collection
        hooks._Fregister(
            crate::hooks::DMSHookKind::Startup,
            "dms.observability.lifecycle".to_string(),
            |_ctx, _event: &crate::hooks::DMSHookEvent| {
                // Could add automatic span creation here
                Ok(())
            },
        );
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.Observability", "Observability module initialized")?;
        
        Ok(())
    }
    
    fn _Fafter_shutdown(&mut self, ctx: &mut DMSServiceContext) -> DMSResult<()> {
        // Export final observability data
        let data = self._Fexport_data();
        
        let logger = ctx._Flogger();
        logger._Finfo("DMS.Observability", format!("Final observability data: {} active traces, {} active spans", 
            data.active_traces, data.active_spans))?;
        
        Ok(())
    }
}