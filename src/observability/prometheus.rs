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

//! # Prometheus Exporter
//! 
//! This module provides a Prometheus exporter implementation for the Ri framework. It allows
//! registering and managing Prometheus metrics (counters, gauges, histograms) and generating
//! Grafana dashboards from these metrics.
//! 
//! ## Key Components
//! 
//! - **RiPrometheusExporter**: Main exporter class for managing Prometheus metrics
//! 
//! ## Design Principles
//! 
//! 1. **Prometheus Integration**: Uses the official prometheus crate for metric collection
//! 2. **Thread Safety**: Uses Arc and RwLock for safe concurrent access
//! 3. **Multiple Metric Types**: Supports Counter, Gauge, and Histogram metrics
//! 4. **Grafana Integration**: Provides methods to generate Grafana dashboards and panels
//! 5. **Easy to Use**: Simple API for registering and updating metrics
//! 6. **Text Encoding**: Exports metrics in Prometheus text format
//! 7. **Registry Management**: Maintains its own Prometheus registry
//! 8. **Error Handling**: Comprehensive error handling with RiResult
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! fn example() -> RiResult<()> {
//!     // Create a new Prometheus exporter
//!     let exporter = RiPrometheusExporter::new()?;
//!     
//!     // Register a counter metric
//!     exporter.register_counter("http_requests_total", "Total number of HTTP requests")?;
//!     
//!     // Register a gauge metric
//!     exporter.register_gauge("active_connections", "Number of active connections")?;
//!     
//!     // Register a histogram metric
//!     exporter.register_histogram(
//!         "response_time_seconds", 
//!         "Response time in seconds", 
//!         vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
//!     )?;
//!     
//!     // Update metrics
//!     exporter.increment_counter("http_requests_total", 1.0)?;
//!     exporter.set_gauge("active_connections", 10.0)?;
//!     exporter.observe_histogram("response_time_seconds", 0.123)?;
//!     
//!     // Render metrics in Prometheus format
//!     let metrics_text = exporter.render()?;
//!     println!("Prometheus metrics:\n{}", metrics_text);
//!     
//!     // Generate a Grafana dashboard
//!     let dashboard = exporter.generate_default_dashboard()?;
//!     let dashboard_json = dashboard.to_json()?;
//!     println!("Grafana dashboard JSON:\n{}", dashboard_json);
//!     
//!     Ok(())
//! }
//! ```

use std::collections::HashMap as FxHashMap;
use std::sync::{Arc, RwLock};
#[cfg(feature = "observability")]
use prometheus::{Counter, Gauge, Histogram, Registry, Encoder, TextEncoder};
use crate::core::RiResult;
use crate::core::lock::RwLockExtensions;

/// Prometheus exporter for managing metrics and generating Grafana dashboards.
///
/// This struct provides methods for registering and updating Prometheus metrics,
/// as well as generating Grafana dashboards from these metrics.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct RiPrometheusExporter {
    /// Prometheus registry for managing metrics
    registry: Arc<Registry>,
    /// Map of registered counter metrics
    counters: Arc<RwLock<FxHashMap<String, Counter>>>,
    /// Map of registered gauge metrics
    gauges: Arc<RwLock<FxHashMap<String, Gauge>>>,
    /// Map of registered histogram metrics
    histograms: Arc<RwLock<FxHashMap<String, Histogram>>>,
}

#[allow(dead_code)]
impl RiPrometheusExporter {
    /// Creates a new Prometheus exporter instance.
    ///
    /// # Returns
    ///
    /// A new RiPrometheusExporter instance wrapped in RiResult
    pub fn new() -> RiResult<Self> {
        let registry = Arc::new(Registry::new());
        
        Ok(RiPrometheusExporter {
            registry: registry.clone(),
            counters: Arc::new(RwLock::new(FxHashMap::default())),
            gauges: Arc::new(RwLock::new(FxHashMap::default())),
            histograms: Arc::new(RwLock::new(FxHashMap::default())),
        })
    }
    
    /// Registers a new counter metric.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the counter metric
    /// - `help`: Help text describing the counter
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn register_counter(&self, name: &str, help: &str) -> RiResult<()> {
        let counter = Counter::new(name, help)?;
        self.registry.register(Box::new(counter.clone()))?;
        
        let mut counters = self.counters.write_safe("counters for register")?;
        counters.insert(name.to_string(), counter);
        
        Ok(())
    }
    
    /// Increments a counter metric by the specified value.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the counter metric
    /// - `value`: The value to increment by
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn increment_counter(&self, name: &str, value: f64) -> RiResult<()> {
        let counters = self.counters.read_safe("counters for increment")?;
        if let Some(counter) = counters.get(name) {
            counter.inc_by(value);
            Ok(())
        } else {
            Err(crate::core::RiError::Io(format!("Counter {name} not found")))
        }
    }
    
    /// Registers a new gauge metric.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the gauge metric
    /// - `help`: Help text describing the gauge
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn register_gauge(&self, name: &str, help: &str) -> RiResult<()> {
        let gauge = Gauge::new(name, help)?;
        self.registry.register(Box::new(gauge.clone()))?;
        
        let mut gauges = self.gauges.write_safe("gauges for register")?;
        gauges.insert(name.to_string(), gauge);
        
        Ok(())
    }
    
    /// Sets a gauge metric to the specified value.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the gauge metric
    /// - `value`: The value to set
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn set_gauge(&self, name: &str, value: f64) -> RiResult<()> {
        let gauges = self.gauges.read_safe("gauges for set")?;
        if let Some(gauge) = gauges.get(name) {
            gauge.set(value);
            Ok(())
        } else {
            Err(crate::core::RiError::Io(format!("Gauge {name} not found")))
        }
    }
    
    /// Registers a new histogram metric.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the histogram metric
    /// - `help`: Help text describing the histogram
    /// - `buckets`: The histogram buckets
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn register_histogram(&self, name: &str, help: &str, buckets: Vec<f64>) -> RiResult<()> {
        let histogram = Histogram::with_opts(prometheus::HistogramOpts::new(name, help).buckets(buckets))?;
        self.registry.register(Box::new(histogram.clone()))?;
        
        let mut histograms = self.histograms.write_safe("histograms for register")?;
        histograms.insert(name.to_string(), histogram);
        
        Ok(())
    }
    
    /// Observes a value in a histogram metric.
    ///
    /// # Parameters
    ///
    /// - `name`: The name of the histogram metric
    /// - `value`: The value to observe
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn observe_histogram(&self, name: &str, value: f64) -> RiResult<()> {
        let histograms = self.histograms.read_safe("histograms for observe")?;
        if let Some(histogram) = histograms.get(name) {
            histogram.observe(value);
            Ok(())
        } else {
            Err(crate::core::RiError::Io(format!("Histogram {name} not found")))
        }
    }
    
    /// Renders all metrics in Prometheus text format.
    ///
    /// # Returns
    ///
    /// A string containing all metrics in Prometheus text format wrapped in RiResult
    pub fn render(&self) -> RiResult<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::with_capacity(1024);
        encoder.encode(&metric_families, &mut buffer)?;
        
        String::from_utf8(buffer).map_err(|e| crate::core::RiError::Serde(format!("Invalid UTF-8 in Prometheus output: {}", e)))
    }
    
    /// Adds a counter panel to a Grafana dashboard.
    ///
    /// # Parameters
    ///
    /// - `dashboard`: The Grafana dashboard to add the panel to
    /// - `title`: The title of the panel
    /// - `query`: The Prometheus query for the panel
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn add_counter_panel(&self, dashboard: &mut crate::observability::grafana::RiGrafanaDashboard, title: &str, query: &str) -> RiResult<()> {
        let mut generator = crate::observability::grafana::RiGrafanaDashboardGenerator::new();
        let mut panel = generator.create_panel(title, "stat", 0, 0, 12, 8);
        panel.targets.push(generator.create_prometheus_target(query, "A", None));
        dashboard.panels.push(panel);
        Ok(())
    }
    
    /// Adds a gauge panel to a Grafana dashboard.
    ///
    /// # Parameters
    ///
    /// - `dashboard`: The Grafana dashboard to add the panel to
    /// - `title`: The title of the panel
    /// - `query`: The Prometheus query for the panel
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn add_gauge_panel(&self, dashboard: &mut crate::observability::grafana::RiGrafanaDashboard, title: &str, query: &str) -> RiResult<()> {
        let mut generator = crate::observability::grafana::RiGrafanaDashboardGenerator::new();
        let mut panel = generator.create_panel(title, "gauge", 12, 0, 12, 8);
        panel.targets.push(generator.create_prometheus_target(query, "A", None));
        dashboard.panels.push(panel);
        Ok(())
    }
    
    /// Adds a stat panel to a Grafana dashboard.
    ///
    /// # Parameters
    ///
    /// - `dashboard`: The Grafana dashboard to add the panel to
    /// - `title`: The title of the panel
    /// - `query`: The Prometheus query for the panel
    ///
    /// # Returns
    ///
    /// RiResult indicating success or failure
    pub fn add_stat_panel(&self, dashboard: &mut crate::observability::grafana::RiGrafanaDashboard, title: &str, query: &str) -> RiResult<()> {
        let mut generator = crate::observability::grafana::RiGrafanaDashboardGenerator::new();
        let mut panel = generator.create_panel(title, "stat", 0, 8, 12, 8);
        panel.targets.push(generator.create_prometheus_target(query, "A", None));
        dashboard.panels.push(panel);
        Ok(())
    }
    
    /// Generates a Grafana dashboard with default panels.
    ///
    /// # Parameters
    ///
    /// - `title`: The title of the dashboard
    ///
    /// # Returns
    ///
    /// A Grafana dashboard with default panels wrapped in RiResult
    pub fn generate_dashboard(&self, title: &str) -> RiResult<crate::observability::grafana::RiGrafanaDashboard> {
        let mut dashboard = crate::observability::grafana::RiGrafanaDashboard::new(title);
        
        self.add_counter_panel(&mut dashboard, "Request Count", "dms_requests_total")?;
        self.add_gauge_panel(&mut dashboard, "Active Connections", "dms_active_connections")?;
        self.add_stat_panel(&mut dashboard, "Response Time", "dms_response_time_seconds")?;
        
        Ok(dashboard)
    }
    
    /// Generates a default Grafana dashboard with "Ri Metrics Dashboard" title.
    ///
    /// # Returns
    ///
    /// A default Grafana dashboard wrapped in RiResult
    pub fn generate_default_dashboard(&self) -> RiResult<crate::observability::grafana::RiGrafanaDashboard> {
        self.generate_dashboard("Ri Metrics Dashboard")
    }
}

