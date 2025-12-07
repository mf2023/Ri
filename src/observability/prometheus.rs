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

//! # Prometheus Exporter
//! 
//! This module provides a Prometheus exporter implementation for the DMS framework. It allows
//! registering and managing Prometheus metrics (counters, gauges, histograms) and generating
//! Grafana dashboards from these metrics.
//! 
//! ## Key Components
//! 
//! - **DMSPrometheusExporter**: Main exporter class for managing Prometheus metrics
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
//! 8. **Error Handling**: Comprehensive error handling with DMSResult
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a new Prometheus exporter
//!     let exporter = DMSPrometheusExporter::new()?;
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

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use prometheus::{Counter, Gauge, Histogram, Registry, Encoder, TextEncoder};
use crate::core::DMSResult;

/// Prometheus exporter for managing metrics and generating Grafana dashboards.
///
/// This struct provides methods for registering and updating Prometheus metrics,
/// as well as generating Grafana dashboards from these metrics.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DMSPrometheusExporter {
    /// Prometheus registry for managing metrics
    registry: Arc<Registry>,
    /// Map of registered counter metrics
    counters: Arc<RwLock<HashMap<String, Counter>>>,
    /// Map of registered gauge metrics
    gauges: Arc<RwLock<HashMap<String, Gauge>>>,
    /// Map of registered histogram metrics
    histograms: Arc<RwLock<HashMap<String, Histogram>>>,
}

#[allow(dead_code)]
impl DMSPrometheusExporter {
    /// Creates a new Prometheus exporter instance.
    ///
    /// # Returns
    ///
    /// A new DMSPrometheusExporter instance wrapped in DMSResult
    pub fn new() -> DMSResult<Self> {
        let registry = Arc::new(Registry::new());
        
        Ok(DMSPrometheusExporter {
            registry: registry.clone(),
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
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
    /// DMSResult indicating success or failure
    pub fn register_counter(&self, name: &str, help: &str) -> DMSResult<()> {
        let counter = Counter::new(name, help)?;
        self.registry.register(Box::new(counter.clone()))?;
        
        let mut counters = self.counters.write().unwrap();
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
    /// DMSResult indicating success or failure
    pub fn increment_counter(&self, name: &str, value: f64) -> DMSResult<()> {
        let counters = self.counters.read().unwrap();
        if let Some(counter) = counters.get(name) {
            counter.inc_by(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Counter {name} not found")))
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
    /// DMSResult indicating success or failure
    pub fn register_gauge(&self, name: &str, help: &str) -> DMSResult<()> {
        let gauge = Gauge::new(name, help)?;
        self.registry.register(Box::new(gauge.clone()))?;
        
        let mut gauges = self.gauges.write().unwrap();
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
    /// DMSResult indicating success or failure
    pub fn set_gauge(&self, name: &str, value: f64) -> DMSResult<()> {
        let gauges = self.gauges.read().unwrap();
        if let Some(gauge) = gauges.get(name) {
            gauge.set(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Gauge {name} not found")))
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
    /// DMSResult indicating success or failure
    pub fn register_histogram(&self, name: &str, help: &str, buckets: Vec<f64>) -> DMSResult<()> {
        let histogram = Histogram::with_opts(prometheus::HistogramOpts::new(name, help).buckets(buckets))?;
        self.registry.register(Box::new(histogram.clone()))?;
        
        let mut histograms = self.histograms.write().unwrap();
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
    /// DMSResult indicating success or failure
    pub fn observe_histogram(&self, name: &str, value: f64) -> DMSResult<()> {
        let histograms = self.histograms.read().unwrap();
        if let Some(histogram) = histograms.get(name) {
            histogram.observe(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Histogram {name} not found")))
        }
    }
    
    /// Renders all metrics in Prometheus text format.
    ///
    /// # Returns
    ///
    /// A string containing all metrics in Prometheus text format wrapped in DMSResult
    pub fn render(&self) -> DMSResult<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        
        Ok(String::from_utf8(buffer).unwrap())
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
    /// DMSResult indicating success or failure
    pub fn add_counter_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "stat".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 0, y: 0 },
        };
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
    /// DMSResult indicating success or failure
    pub fn add_gauge_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "gauge".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 12, y: 0 },
        };
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
    /// DMSResult indicating success or failure
    pub fn add_stat_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "stat".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 0, y: 8 },
        };
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
    /// A Grafana dashboard with default panels wrapped in DMSResult
    pub fn generate_dashboard(&self, title: &str) -> DMSResult<crate::observability::grafana::DMSGrafanaDashboard> {
        let mut dashboard = crate::observability::grafana::DMSGrafanaDashboard {
            title: title.to_string(),
            panels: Vec::new(),
        };
        
        self.add_counter_panel(&mut dashboard, "Request Count", "dms_requests_total")?;
        self.add_gauge_panel(&mut dashboard, "Active Connections", "dms_active_connections")?;
        self.add_stat_panel(&mut dashboard, "Response Time", "dms_response_time_seconds")?;
        
        Ok(dashboard)
    }
    
    /// Generates a default Grafana dashboard with "DMS Metrics Dashboard" title.
    ///
    /// # Returns
    ///
    /// A default Grafana dashboard wrapped in DMSResult
    pub fn generate_default_dashboard(&self) -> DMSResult<crate::observability::grafana::DMSGrafanaDashboard> {
        self.generate_dashboard("DMS Metrics Dashboard")
    }
}

