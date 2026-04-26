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

//! # Grafana Integration Module
//! 
//! This module provides data structures for creating and managing Grafana dashboards and panels.
//! It enables programmatic generation of Grafana dashboards with panels, queries, and layout information.
//! 
//! ## Key Components
//! 
//! - **RiGridPos**: Represents the grid position of a panel on a dashboard
//! - **RiGrafanaPanel**: Represents a single Grafana panel with title, query, type, and position
//! - **RiGrafanaDashboard**: Represents a Grafana dashboard with multiple panels
//! 
//! ## Design Principles
//! 
//! 1. **Serde Integration**: All structs implement Serialize and Deserialize for easy JSON conversion
//! 2. **Simple API**: Easy-to-use methods for creating dashboards and adding panels
//! 3. **Layout Support**: Built-in support for Grafana's grid layout system
//! 4. **Extensible**: Can be extended to support additional panel types and dashboard features
//! 5. **Type Safety**: Strongly typed structs for all Grafana components
//! 6. **JSON Compatibility**: Generates JSON that is compatible with Grafana's API
//! 
//! ## Usage
//! 
//! ```rust
//! use ri::prelude::*;
//! 
//! fn example() -> RiResult<()> {
//!     // Create a new dashboard
//!     let mut dashboard = RiGrafanaDashboard::new("Ri Metrics");
//!     
//!     // Create a panel
//!     let panel = RiGrafanaPanel {
//!         title: "Request Rate".to_string(),
//!         query: "rate(http_requests_total[5m])".to_string(),
//!         panel_type: "graph".to_string(),
//!         grid_pos: RiGridPos {
//!             h: 8,
//!             w: 12,
//!             x: 0,
//!             y: 0,
//!         },
//!     };
//!     
//!     // Add panel to dashboard
//!     dashboard.add_panel(panel)?;
//!     
//!     // Convert to JSON for Grafana API
//!     let json = dashboard.to_json()?;
//!     println!("Dashboard JSON: {}", json);
//!     
//!     Ok(())
//! }
//! ```

use serde::{Serialize, Deserialize};
use crate::core::RiResult;

/// Grafana target configuration for data sources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGrafanaTarget {
    pub expr: String,
    pub ref_id: String,
    pub legend_format: Option<String>,
    pub interval: Option<String>,
}

/// Grafana grid position configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGridPos {
    pub h: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
}

/// Grafana panel configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGrafanaPanel {
    pub id: i32,
    pub title: String,
    pub type_: String,
    pub targets: Vec<RiGrafanaTarget>,
    pub grid_pos: RiGridPos,
    pub field_config: serde_json::Value,
    pub options: serde_json::Value,
    pub description: Option<String>,
    pub datasource: Option<String>,
}

/// Grafana time range configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGrafanaTimeRange {
    pub from: String,
    pub to: String,
}

/// Grafana dashboard tag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGrafanaTag {
    pub term: String,
    pub color: Option<String>,
}

/// Grafana dashboard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiGrafanaDashboard {
    pub title: String,
    pub panels: Vec<RiGrafanaPanel>,
    pub tags: Vec<RiGrafanaTag>,
    pub time: RiGrafanaTimeRange,
    pub refresh: String,
    pub timezone: String,
    pub schema_version: i32,
    pub uid: Option<String>,
    pub version: i32,
}

/// Grafana dashboard generator
pub struct RiGrafanaDashboardGenerator {
    next_panel_id: i32,
}

#[allow(dead_code)]
impl RiGrafanaDashboard {
    pub fn new(title: &str) -> Self {
        RiGrafanaDashboard {
            title: title.to_string(),
            panels: Vec::new(),
            tags: vec![RiGrafanaTag { term: "dms".to_string(), color: Some("#1F77B4".to_string()) }],
            time: RiGrafanaTimeRange { from: "now-1h".to_string(), to: "now".to_string() },
            refresh: "5s".to_string(),
            timezone: "browser".to_string(),
            schema_version: 38,
            uid: None,
            version: 1,
        }
    }
    
    pub fn add_panel(&mut self, panel: RiGrafanaPanel) -> RiResult<()> {
        self.panels.push(panel);
        Ok(())
    }
    
    pub fn to_json(&self) -> RiResult<String> {
        serde_json::to_string(self).map_err(|e| crate::core::RiError::Serde(e.to_string()))
    }
}

impl RiGrafanaDashboardGenerator {
    pub fn new() -> Self {
        RiGrafanaDashboardGenerator {
            next_panel_id: 1,
        }
    }
    
    /// Convert a string to title case (first letter of each word uppercase)
    fn title_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for c in s.chars() {
            if c == '_' || c == ' ' {
                result.push(' ');
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c.to_ascii_lowercase());
            }
        }
        
        result
    }
    
    /// Create a new dashboard with default settings
    pub fn create_dashboard(&self, title: &str) -> RiGrafanaDashboard {
        RiGrafanaDashboard::new(title)
    }
    
    /// Create a Prometheus target
    pub fn create_prometheus_target(&self, expr: &str, ref_id: &str, legend_format: Option<&str>) -> RiGrafanaTarget {
        RiGrafanaTarget {
            expr: expr.to_string(),
            ref_id: ref_id.to_string(),
            legend_format: legend_format.map(|s| s.to_string()),
            interval: None,
        }
    }
    
    /// Create a new panel with default settings
    pub fn create_panel(&mut self, title: &str, panel_type: &str, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let panel_id = self.next_panel_id;
        self.next_panel_id += 1;
        
        RiGrafanaPanel {
            id: panel_id,
            title: title.to_string(),
            type_: panel_type.to_string(),
            targets: Vec::new(),
            grid_pos: RiGridPos { h, w, x, y },
            field_config: serde_json::json!({ "defaults": {}, "overrides": [] }),
            options: serde_json::json!({}),
            description: None,
            datasource: Some("Prometheus".to_string()),
        }
    }
    
    /// Create a graph panel for request rate
    pub fn create_request_rate_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Request Rate", "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "rate(dms_requests_total[5m])",
            "A",
            Some("{{instance}}")
        ));
        panel
    }
    
    /// Create a graph panel for request duration
    pub fn create_request_duration_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Request Duration", "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "histogram_quantile(0.95, sum(rate(dms_request_duration_seconds_bucket[5m])) by (le))",
            "A",
            Some("95th Percentile")
        ));
        panel.targets.push(self.create_prometheus_target(
            "histogram_quantile(0.5, sum(rate(dms_request_duration_seconds_bucket[5m])) by (le))",
            "B",
            Some("50th Percentile")
        ));
        panel
    }
    
    /// Create a stat panel for active connections
    pub fn create_active_connections_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Active Connections", "stat", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "dms_active_connections",
            "A",
            None
        ));
        panel
    }
    
    /// Create a graph panel for error rate
    pub fn create_error_rate_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Error Rate", "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "rate(dms_errors_total[5m])",
            "A",
            Some("{{instance}}")
        ));
        panel
    }
    
    /// Create a graph panel for cache metrics
    pub fn create_cache_metrics_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Cache Metrics", "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "rate(dms_cache_hits_total[5m])",
            "A",
            Some("Hits")
        ));
        panel.targets.push(self.create_prometheus_target(
            "rate(dms_cache_misses_total[5m])",
            "B",
            Some("Misses")
        ));
        panel
    }
    
    /// Create a graph panel for database query time
    pub fn create_db_query_time_panel(&mut self, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let mut panel = self.create_panel("Database Query Time", "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(
            "histogram_quantile(0.95, sum(rate(dms_db_query_duration_seconds_bucket[5m])) by (le))",
            "A",
            Some("95th Percentile")
        ));
        panel
    }
    
    /// Generate a default Ri dashboard with common metrics panels
    pub fn generate_default_dashboard(&mut self) -> RiResult<RiGrafanaDashboard> {
        let mut dashboard = self.create_dashboard("Ri Default Dashboard");
        
        // First row: Request metrics
        dashboard.add_panel(self.create_request_rate_panel(0, 0, 12, 8))?;
        dashboard.add_panel(self.create_request_duration_panel(12, 0, 12, 8))?;
        
        // Second row: Error and connection metrics
        dashboard.add_panel(self.create_error_rate_panel(0, 8, 12, 8))?;
        dashboard.add_panel(self.create_active_connections_panel(12, 8, 6, 8))?;
        
        // Third row: Cache and database metrics
        dashboard.add_panel(self.create_cache_metrics_panel(0, 16, 12, 8))?;
        dashboard.add_panel(self.create_db_query_time_panel(12, 16, 12, 8))?;
        
        Ok(dashboard)
    }
    
    /// Generate a dashboard automatically based on available metrics
    /// 
    /// This method analyzes available metrics and generates an appropriate dashboard
    /// with panels matching the metric types and values.
    /// 
    /// # Parameters
    /// 
    /// - `metrics`: List of available metric names
    /// - `dashboard_title`: Title for the generated dashboard
    /// 
    /// # Returns
    /// 
    /// A Grafana dashboard automatically generated based on the provided metrics
    pub fn generate_auto_dashboard(&mut self, metrics: Vec<&str>, dashboard_title: &str) -> RiResult<RiGrafanaDashboard> {
        let mut dashboard = self.create_dashboard(dashboard_title);
        
        // Analyze metrics and group by type
        let mut counter_metrics = Vec::with_capacity(4);
        let mut gauge_metrics = Vec::with_capacity(4);
        let mut histogram_metrics = Vec::with_capacity(4);
        
        for metric in metrics {
            if metric.ends_with("_total") || metric.contains("count") {
                counter_metrics.push(metric);
            } else if metric.ends_with("_seconds") || metric.ends_with("_bytes") || metric.contains("time") {
                histogram_metrics.push(metric);
            } else {
                gauge_metrics.push(metric);
            }
        }
        
        // Generate panels based on metric types
        let mut current_row = 0;
        
        // Add counter panels
        for (i, metric) in counter_metrics.iter().enumerate() {
            let panel = self.create_counter_panel(*metric, i as i32 * 12, current_row, 12, 8);
            dashboard.add_panel(panel)?;
            if (i + 1) % 2 == 0 {
                current_row += 8;
            }
        }
        
        if counter_metrics.len() % 2 != 0 {
            current_row += 8;
        }
        
        // Add gauge panels
        for (i, metric) in gauge_metrics.iter().enumerate() {
            let panel = self.create_gauge_panel(*metric, i as i32 * 12, current_row, 12, 8);
            dashboard.add_panel(panel)?;
            if (i + 1) % 2 == 0 {
                current_row += 8;
            }
        }
        
        if gauge_metrics.len() % 2 != 0 {
            current_row += 8;
        }
        
        // Add histogram panels
        for (i, metric) in histogram_metrics.iter().enumerate() {
            let panel = self.create_histogram_panel(*metric, i as i32 * 12, current_row, 12, 8);
            dashboard.add_panel(panel)?;
            if (i + 1) % 2 == 0 {
                current_row += 8;
            }
        }
        
        Ok(dashboard)
    }
    
    /// Create a counter panel for a metric
    pub fn create_counter_panel(&mut self, metric_name: &str, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let title = self.title_case(metric_name);
        let query = format!("rate({}[5m])", metric_name);
        
        let mut panel = self.create_panel(&title, "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(&query, "A", Some("{{instance}}")));
        panel
    }
    
    /// Create a gauge panel for a metric
    pub fn create_gauge_panel(&mut self, metric_name: &str, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let title = self.title_case(metric_name);
        
        let mut panel = self.create_panel(&title, "stat", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(metric_name, "A", None));
        panel
    }
    
    /// Create a histogram panel for a metric
    pub fn create_histogram_panel(&mut self, metric_name: &str, x: i32, y: i32, w: i32, h: i32) -> RiGrafanaPanel {
        let title = self.title_case(metric_name);
        let query_95 = format!("histogram_quantile(0.95, sum(rate({}_bucket[5m])) by (le))", metric_name);
        let query_50 = format!("histogram_quantile(0.5, sum(rate({}_bucket[5m])) by (le))", metric_name);
        
        let mut panel = self.create_panel(&title, "timeseries", x, y, w, h);
        panel.targets.push(self.create_prometheus_target(&query_95, "A", Some("95th Percentile")));
        panel.targets.push(self.create_prometheus_target(&query_50, "B", Some("50th Percentile")));
        panel
    }
}
