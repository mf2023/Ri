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
use std::sync::{Arc, RwLock};
use prometheus::{Counter, Gauge, Histogram, Registry, Encoder, TextEncoder};
use crate::core::DMSResult;

#[derive(Debug, Clone)]
pub struct DMSPrometheusExporter {
    registry: Arc<Registry>,
    counters: Arc<RwLock<HashMap<String, Counter>>>,
    gauges: Arc<RwLock<HashMap<String, Gauge>>>,
    histograms: Arc<RwLock<HashMap<String, Histogram>>>,
}

impl DMSPrometheusExporter {
    pub fn _Fnew() -> DMSResult<Self> {
        let registry = Arc::new(Registry::new());
        
        Ok(DMSPrometheusExporter {
            registry: registry.clone(),
            counters: Arc::new(RwLock::new(HashMap::new())),
            gauges: Arc::new(RwLock::new(HashMap::new())),
            histograms: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub fn _Fregister_counter(&self, name: &str, help: &str) -> DMSResult<()> {
        let counter = Counter::new(name, help)?;
        self.registry.register(Box::new(counter.clone()))?;
        
        let mut counters = self.counters.write().unwrap();
        counters.insert(name.to_string(), counter);
        
        Ok(())
    }
    
    pub fn _Fincrement_counter(&self, name: &str, value: f64) -> DMSResult<()> {
        let counters = self.counters.read().unwrap();
        if let Some(counter) = counters.get(name) {
            counter.inc_by(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Counter {} not found", name)))
        }
    }
    
    pub fn _Fregister_gauge(&self, name: &str, help: &str) -> DMSResult<()> {
        let gauge = Gauge::new(name, help)?;
        self.registry.register(Box::new(gauge.clone()))?;
        
        let mut gauges = self.gauges.write().unwrap();
        gauges.insert(name.to_string(), gauge);
        
        Ok(())
    }
    
    pub fn _Fset_gauge(&self, name: &str, value: f64) -> DMSResult<()> {
        let gauges = self.gauges.read().unwrap();
        if let Some(gauge) = gauges.get(name) {
            gauge.set(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Gauge {} not found", name)))
        }
    }
    
    pub fn _Fregister_histogram(&self, name: &str, help: &str, buckets: Vec<f64>) -> DMSResult<()> {
        let histogram = Histogram::with_opts(prometheus::HistogramOpts::new(name, help).buckets(buckets))?;
        self.registry.register(Box::new(histogram.clone()))?;
        
        let mut histograms = self.histograms.write().unwrap();
        histograms.insert(name.to_string(), histogram);
        
        Ok(())
    }
    
    pub fn _Fobserve_histogram(&self, name: &str, value: f64) -> DMSResult<()> {
        let histograms = self.histograms.read().unwrap();
        if let Some(histogram) = histograms.get(name) {
            histogram.observe(value);
            Ok(())
        } else {
            Err(crate::core::DMSError::Io(format!("Histogram {} not found", name)))
        }
    }
    
    pub fn _Frender(&self) -> DMSResult<String> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = Vec::new();
        encoder.encode(&metric_families, &mut buffer)?;
        
        Ok(String::from_utf8(buffer).unwrap())
    }
    
    pub fn _Fadd_counter_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "stat".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 0, y: 0 },
        };
        dashboard.panels.push(panel);
        Ok(())
    }
    
    pub fn _Fadd_gauge_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "gauge".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 12, y: 0 },
        };
        dashboard.panels.push(panel);
        Ok(())
    }
    
    pub fn _Fadd_stat_panel(&self, dashboard: &mut crate::observability::grafana::DMSGrafanaDashboard, title: &str, query: &str) -> DMSResult<()> {
        let panel = crate::observability::grafana::DMSGrafanaPanel {
            title: title.to_string(),
            query: query.to_string(),
            panel_type: "stat".to_string(),
            grid_pos: crate::observability::grafana::DMSGridPos { h: 8, w: 12, x: 0, y: 8 },
        };
        dashboard.panels.push(panel);
        Ok(())
    }
    
    pub fn _Fgenerate_dashboard(&self, title: &str) -> DMSResult<crate::observability::grafana::DMSGrafanaDashboard> {
        let mut dashboard = crate::observability::grafana::DMSGrafanaDashboard {
            title: title.to_string(),
            panels: Vec::new(),
        };
        
        self._Fadd_counter_panel(&mut dashboard, "Request Count", "dms_requests_total")?;
        self._Fadd_gauge_panel(&mut dashboard, "Active Connections", "dms_active_connections")?;
        self._Fadd_stat_panel(&mut dashboard, "Response Time", "dms_response_time_seconds")?;
        
        Ok(dashboard)
    }
    
    pub fn _Fgenerate_default_dashboard(&self) -> DMSResult<crate::observability::grafana::DMSGrafanaDashboard> {
        self._Fgenerate_dashboard("DMS Metrics Dashboard")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prometheus_exporter() {
        let exporter = DMSPrometheusExporter::_Fnew().unwrap();
        
        exporter._Fregister_counter("test_counter", "A test counter").unwrap();
        exporter._Fincrement_counter("test_counter", 1.0).unwrap();
        
        exporter._Fregister_gauge("test_gauge", "A test gauge").unwrap();
        exporter._Fset_gauge("test_gauge", 42.0).unwrap();
        
        let output = exporter._Frender().unwrap();
        assert!(output.contains("test_counter"));
        assert!(output.contains("test_gauge"));
    }
    
    #[test]
    fn test_grafana_dashboard_generation() {
        let exporter = DMSPrometheusExporter::_Fnew().unwrap();
        let dashboard = exporter._Fgenerate_default_dashboard().unwrap();
        
        assert_eq!(dashboard.title, "DMS Metrics Dashboard");
        assert_eq!(dashboard.panels.len(), 3);
    }
}