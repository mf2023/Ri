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

#![allow(non_snake_case)]

//! # Metrics Collection and Aggregation Module
//! 
//! This module provides a comprehensive metrics collection and aggregation system for DMSC.
//! It supports various metric types, sliding window aggregation, and Prometheus-compatible export.
//! 
//! ## Key Components
//! 
//! - **DMSCMetricType**: Enum defining supported metric types (Counter, Gauge, Histogram, Summary)
//! - **DMSCMetricSample**: Represents a single metric sample with timestamp, value, and labels
//! - **DMSCMetricConfig**: Configuration for creating metrics
//! - **DMSCSlidingWindow**: Internal sliding time window for metric aggregation
//! - **DMSCWindowStats**: Aggregated statistics from the sliding window
//! - **DMSCMetric**: Individual metric with sliding window aggregation
//! - **DMSCMetricsRegistry**: Registry for managing multiple metrics
//! 
//! ## Design Principles
//! 
//! 1. **Multiple Metric Types**: Supports Counter, Gauge, Histogram, and Summary metrics
//! 2. **Sliding Window Aggregation**: Efficiently aggregates metrics over configurable time windows
//! 3. **Thread Safety**: Uses Arc and RwLock for safe concurrent access
//! 4. **Prometheus Compatible**: Exports metrics in Prometheus format
//! 5. **Label Support**: Allows adding custom labels to metric samples
//! 6. **Configurable**: Supports custom window sizes, bucket sizes, and other parameters
//! 7. **Type Safety**: Strongly typed metrics with compile-time checks
//! 8. **Efficient Memory Usage**: Automatically rotates and prunes old metric data
//! 
//! ## Usage
//! 
//! ```rust
//! use dmsc::prelude::*;
//! use std::time::Duration;
//! 
//! fn example() -> DMSCResult<()> {
//!     // Create a metrics registry
//!     let registry = DMSCMetricsRegistry::new();
//!     
//!     // Configure a counter metric
//!     let counter_config = DMSCMetricConfig {
//!         metric_type: DMSCMetricType::Counter,
//!         name: "http_requests_total".to_string(),
//!         help: "Total number of HTTP requests".to_string(),
//!         buckets: Vec::new(),
//!         quantiles: Vec::new(),
//!         max_age: Duration::from_secs(300),
//!         age_buckets: 5,
//!     };
//!     
//!     // Create and register the metric
//!     let counter = Arc::new(DMSCMetric::new(counter_config));
//!     registry.register(counter.clone())?;
//!     
//!     // Record some metrics
//!     counter.record(1.0, vec![("method".to_string(), "GET".to_string())])?;
//!     counter.record(1.0, vec![("method".to_string(), "POST".to_string())])?;
//!     
//!     // Export metrics in Prometheus format
//!     let prometheus_output = registry.export_prometheus();
//!     println!("{}", prometheus_output);
//!     
//!     Ok(())
//! }
//! ```

use std::collections::{VecDeque, HashMap};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use serde::{Serialize, Deserialize};

use crate::core::DMSCResult;

/// Metric types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub enum DMSCMetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// A single metric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMetricSample {
    pub timestamp: u64, // seconds since epoch
    pub value: f64,
    pub labels: Vec<(String, String)>,
}

/// Metric configuration
#[derive(Debug, Clone)]
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMetricConfig {
    pub metric_type: DMSCMetricType,
    pub name: String,
    pub help: String,
    pub buckets: Vec<f64>, // for histogram
    pub quantiles: Vec<f64>, // for summary
    pub max_age: Duration, // for summary
    pub age_buckets: usize, // for summary
}

/// Sliding time window for metric aggregation
#[allow(dead_code)]
struct DMSCSlidingWindow {
    #[allow(dead_code)]
    window_size: Duration,
    #[allow(dead_code)]
    bucket_size: Duration,
    buckets: VecDeque<Vec<DMSCMetricSample>>,
    current_bucket: Vec<DMSCMetricSample>,
    #[allow(dead_code)]
    last_rotation: u64,
}

impl DMSCSlidingWindow {
    fn new(window_size: Duration, bucket_size: Duration) -> Self {
        let bucket_count = window_size.as_secs().div_ceil(bucket_size.as_secs());
        
        Self {
            window_size,
            bucket_size,
            buckets: VecDeque::with_capacity(bucket_count as usize),
            current_bucket: Vec::new(),
            last_rotation: Self::current_timestamp(),
        }
    }
    
    #[allow(dead_code)]
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs()
    }
    
    #[allow(dead_code)]
    fn rotate_if_needed(&mut self) {
        let now = Self::current_timestamp();
        let elapsed = now.saturating_sub(self.last_rotation);
        
        if elapsed >= self.bucket_size.as_secs() {
            let rotations = elapsed / self.bucket_size.as_secs();
            
            for _ in 0..rotations {
                self.buckets.push_back(std::mem::take(&mut self.current_bucket));
                
                // Remove old buckets outside window
                let max_buckets = self.window_size.as_secs().div_ceil(self.bucket_size.as_secs());
                while self.buckets.len() > max_buckets as usize {
                    self.buckets.pop_front();
                }
            }
            
            self.last_rotation = now;
        }
    }
    
    #[allow(dead_code)]
    fn add_sample(&mut self, sample: DMSCMetricSample) {
        self.rotate_if_needed();
        self.current_bucket.push(sample);
    }
    
    #[allow(dead_code)]
    fn get_samples(&self) -> Vec<DMSCMetricSample> {
        let mut all_samples = Vec::new();
        
        for bucket in &self.buckets {
            all_samples.extend(bucket.iter().cloned());
        }
        all_samples.extend(self.current_bucket.iter().cloned());
        
        all_samples
    }
    
    #[allow(dead_code)]
    fn get_window_stats(&self) -> DMSCWindowStats {
        let samples = self.get_samples();
        
        if samples.is_empty() {
            return DMSCWindowStats::default();
        }
        
        let mut sorted_values: Vec<f64> = samples.iter().map(|s| s.value).collect();
        sorted_values.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        
        let count = sorted_values.len();
        let sum: f64 = sorted_values.iter().sum();
        let min = sorted_values[0];
        let max = sorted_values[count - 1];
        let mean = sum / count as f64;
        
        // Calculate variance and standard deviation
        let variance: f64 = sorted_values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / count as f64;
        let stddev = variance.sqrt();
        
        // Calculate quantiles
        let p50 = Self::quantile(&sorted_values, 0.50);
        let p90 = Self::quantile(&sorted_values, 0.90);
        let p95 = Self::quantile(&sorted_values, 0.95);
        let p99 = Self::quantile(&sorted_values, 0.99);
        
        DMSCWindowStats {
            count: count as u64,
            sum,
            min,
            max,
            mean,
            stddev,
            p50,
            p90,
            p95,
            p99,
        }
    }
    
    #[allow(dead_code)]
    fn quantile(sorted_values: &[f64], q: f64) -> f64 {
        if sorted_values.is_empty() {
            return 0.0;
        }
        
        let index = q * (sorted_values.len() - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        
        if lower == upper {
            sorted_values[lower]
        } else {
            let weight = index - lower as f64;
            sorted_values[lower] * (1.0 - weight) + sorted_values[upper] * weight
        }
    }
}

/// Window statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCWindowStats {
    pub count: u64,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub stddev: f64,
    pub p50: f64,
    pub p90: f64,
    pub p95: f64,
    pub p99: f64,
}

impl Default for DMSCWindowStats {
    fn default() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            min: 0.0,
            max: 0.0,
            mean: 0.0,
            stddev: 0.0,
            p50: 0.0,
            p90: 0.0,
            p95: 0.0,
            p99: 0.0,
        }
    }
}

/// A single metric with sliding window aggregation
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
pub struct DMSCMetric {
    config: DMSCMetricConfig,
    sliding_window: RwLock<DMSCSlidingWindow>,
    total_count: RwLock<u64>,
    #[allow(dead_code)]
    total_sum: RwLock<f64>,
}

impl DMSCMetric {
    pub fn new(config: DMSCMetricConfig) -> Self {
        let sliding_window = DMSCSlidingWindow::new(
            Duration::from_secs(300), // 5 minute window
            Duration::from_secs(10),  // 10 second buckets
        );
        
        Self {
            config,
            sliding_window: RwLock::new(sliding_window),
            total_count: RwLock::new(0),
            total_sum: RwLock::new(0.0),
        }
    }
    
    #[allow(dead_code)]
    fn record(&self, value: f64, labels: Vec<(String, String)>) -> DMSCResult<()> {
        let sample = DMSCMetricSample {
            timestamp: Self::current_timestamp(),
            value,
            labels,
        };
        
        {
            let mut window = self.sliding_window.write().expect("Failed to acquire write lock for sliding window");
            window.add_sample(sample);
        }
        
        {
            let mut count = self.total_count.write().expect("Failed to acquire write lock for total count");
            *count += 1;
        }
        
        {
            let mut sum = self.total_sum.write().expect("Failed to acquire write lock for total sum");
            *sum += value;
        }
        
        Ok(())
    }
    
    #[allow(dead_code)]
    fn get_stats(&self) -> DMSCWindowStats {
        self.sliding_window.read().expect("Failed to acquire read lock for sliding window").get_window_stats()
    }
    
    #[allow(dead_code)]
    fn get_total_count(&self) -> u64 {
        let count = self.total_count.read().expect("Failed to acquire read lock for total count");
        *count
    }
    
    #[allow(dead_code)]
    fn get_total_sum(&self) -> f64 {
        let sum = self.total_sum.read().expect("Failed to acquire read lock for total sum");
        *sum
    }
    
    fn get_config(&self) -> &DMSCMetricConfig {
        &self.config
    }

    pub fn get_value(&self) -> f64 {
        let count = self.total_count.read().expect("Failed to acquire read lock for count");
        *count as f64
    }

    #[allow(dead_code)]
    fn current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    }
}

/// Metrics registry to manage multiple metrics
#[cfg_attr(feature = "pyo3", pyo3::prelude::pyclass)]
#[derive(Clone)]
pub struct DMSCMetricsRegistry {
    metrics: Arc<RwLock<HashMap<String, Arc<DMSCMetric>>>>,
}

impl Default for DMSCMetricsRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCMetricsRegistry {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn register(&self, metric: Arc<DMSCMetric>) -> DMSCResult<()> {
        let name = metric.get_config().name.clone();
        self.metrics.write().expect("Failed to acquire write lock for metrics registry").insert(name, metric);
        Ok(())
    }
    
    pub fn get_metric(&self, name: &str) -> Option<Arc<DMSCMetric>> {
        self.metrics.read().expect("Failed to acquire read lock for metrics registry").get(name).cloned()
    }
    
    pub fn get_all_metrics(&self) -> HashMap<String, Arc<DMSCMetric>> {
        self.metrics.read().expect("Failed to acquire read lock for metrics registry").clone()
    }
    
    /// Export metrics in Prometheus format
    #[cfg(feature = "observability")]
    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        let metrics = self.metrics.read().expect("Failed to acquire read lock for metrics registry");
        
        for (name, metric) in metrics.iter() {
            let config = metric.get_config();
            
            // Write help and type
            output.push_str(&format!("# HELP {} {}\n", name, config.help));
            output.push_str(&format!("# TYPE {} {:?}\n", name, config.metric_type));
            
            // Write metric value
            let stats = metric.get_stats();
            match config.metric_type {
                DMSCMetricType::Counter => {
                    output.push_str(&format!("{} {}\n", name, metric.get_total_count()));
                }
                DMSCMetricType::Gauge => {
                    output.push_str(&format!("{} {}\n", name, stats.mean));
                }
                DMSCMetricType::Histogram => {
                    output.push_str(&format!("{}_count {}\n", name, stats.count));
                    output.push_str(&format!("{}_sum {}\n", name, stats.sum));
                    output.push_str(&format!("{}_min {}\n", name, stats.min));
                    output.push_str(&format!("{}_max {}\n", name, stats.max));
                    output.push_str(&format!("{}_avg {}\n", name, stats.mean));
                    output.push_str(&format!("{}_p50 {}\n", name, stats.p50));
                    output.push_str(&format!("{}_p90 {}\n", name, stats.p90));
                    output.push_str(&format!("{}_p95 {}\n", name, stats.p95));
                    output.push_str(&format!("{}_p99 {}\n", name, stats.p99));
                }
                DMSCMetricType::Summary => {
                    output.push_str(&format!("{} {}\n", name, stats.mean));
                }
            }
            
            output.push('\n');
        }
        
        output
    }
}

#[cfg(feature = "pyo3")]
/// Python methods for DMSCMetricsRegistry
#[pyo3::prelude::pymethods]
impl DMSCMetricsRegistry {
    /// Create a new metrics registry from Python
    #[new]
    fn py_new() -> Self {
        Self::new()
    }
    
    /// Get a metric's current value by name from Python
    #[pyo3(name = "get_metric_value")]
    fn get_metric_value_impl(&self, name: &str) -> Option<f64> {
        self.get_metric(name).map(|m| m.get_value())
    }

    /// Get all metric names from Python
    #[pyo3(name = "get_all_metric_names")]
    fn get_all_metric_names_impl(&self) -> Vec<String> {
        self.metrics.read().unwrap().keys().cloned().collect()
    }
    
    /// Export metrics in Prometheus format from Python
    #[pyo3(name = "export_prometheus")]
    fn export_prometheus_impl(&self) -> String {
        #[cfg(feature = "observability")]
        {
            self.export_prometheus()
        }
        #[cfg(not(feature = "observability"))]
        {
            "# Observability feature not enabled".to_string()
        }
    }
    
    /// Get metric count from Python
    #[pyo3(name = "get_metric_count")]
    fn get_metric_count_impl(&self) -> usize {
        self.metrics.read().unwrap().len()
    }
}
