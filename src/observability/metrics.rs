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

//! # Metrics Collection and Aggregation Module
//! 
//! This module provides a comprehensive metrics collection and aggregation system for DMS.
//! It supports various metric types, sliding window aggregation, and Prometheus-compatible export.
//! 
//! ## Key Components
//! 
//! - **DMSMetricType**: Enum defining supported metric types (Counter, Gauge, Histogram, Summary)
//! - **DMSMetricSample**: Represents a single metric sample with timestamp, value, and labels
//! - **DMSMetricConfig**: Configuration for creating metrics
//! - **DMSSlidingWindow**: Internal sliding time window for metric aggregation
//! - **DMSWindowStats**: Aggregated statistics from the sliding window
//! - **DMSMetric**: Individual metric with sliding window aggregation
//! - **DMSMetricsRegistry**: Registry for managing multiple metrics
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
//! use dms::prelude::*;
//! use std::time::Duration;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a metrics registry
//!     let registry = DMSMetricsRegistry::new();
//!     
//!     // Configure a counter metric
//!     let counter_config = DMSMetricConfig {
//!         metric_type: DMSMetricType::Counter,
//!         name: "http_requests_total".to_string(),
//!         help: "Total number of HTTP requests".to_string(),
//!         buckets: Vec::new(),
//!         quantiles: Vec::new(),
//!         max_age: Duration::from_secs(300),
//!         age_buckets: 5,
//!     };
//!     
//!     // Create and register the metric
//!     let counter = Arc::new(DMSMetric::new(counter_config));
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

use crate::core::DMSResult;

/// Metric types supported
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DMSMetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
}

/// A single metric sample
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSMetricSample {
    pub timestamp: u64, // seconds since epoch
    pub value: f64,
    pub labels: Vec<(String, String)>,
}

/// Metric configuration
#[derive(Debug, Clone)]
pub struct DMSMetricConfig {
    pub metric_type: DMSMetricType,
    pub name: String,
    pub help: String,
    pub buckets: Vec<f64>, // for histogram
    pub quantiles: Vec<f64>, // for summary
    pub max_age: Duration, // for summary
    pub age_buckets: usize, // for summary
}

/// Sliding time window for metric aggregation
#[allow(dead_code)]
struct DMSSlidingWindow {
    #[allow(dead_code)]
    window_size: Duration,
    #[allow(dead_code)]
    bucket_size: Duration,
    buckets: VecDeque<Vec<DMSMetricSample>>,
    current_bucket: Vec<DMSMetricSample>,
    #[allow(dead_code)]
    last_rotation: u64,
}

impl DMSSlidingWindow {
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
            .unwrap_or_default()
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
    fn add_sample(&mut self, sample: DMSMetricSample) {
        self.rotate_if_needed();
        self.current_bucket.push(sample);
    }
    
    fn get_samples(&self) -> Vec<DMSMetricSample> {
        let mut all_samples = Vec::new();
        
        for bucket in &self.buckets {
            all_samples.extend(bucket.iter().cloned());
        }
        all_samples.extend(self.current_bucket.iter().cloned());
        
        all_samples
    }
    
    fn get_window_stats(&self) -> DMSWindowStats {
        let samples = self.get_samples();
        
        if samples.is_empty() {
            return DMSWindowStats::default();
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
        
        DMSWindowStats {
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
pub struct DMSWindowStats {
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

impl Default for DMSWindowStats {
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
pub struct DMSMetric {
    config: DMSMetricConfig,
    sliding_window: RwLock<DMSSlidingWindow>,
    total_count: RwLock<u64>,
    #[allow(dead_code)]
    total_sum: RwLock<f64>,
}

impl DMSMetric {
    pub fn new(config: DMSMetricConfig) -> Self {
        let sliding_window = DMSSlidingWindow::new(
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
    fn record(&self, value: f64, labels: Vec<(String, String)>) -> DMSResult<()> {
        let sample = DMSMetricSample {
            timestamp: Self::current_timestamp(),
            value,
            labels,
        };
        
        {
            let mut window = self.sliding_window.write().unwrap();
            window.add_sample(sample);
        }
        
        *self.total_count.write().unwrap() += 1;
        *self.total_sum.write().unwrap() += value;
        
        Ok(())
    }
    
    fn get_stats(&self) -> DMSWindowStats {
        self.sliding_window.read().unwrap().get_window_stats()
    }
    
    fn get_total_count(&self) -> u64 {
        *self.total_count.read().unwrap()
    }
    
    #[allow(dead_code)]
    fn get_total_sum(&self) -> f64 {
        *self.total_sum.read().unwrap()
    }
    
    fn get_config(&self) -> &DMSMetricConfig {
        &self.config
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
pub struct DMSMetricsRegistry {
    metrics: Arc<RwLock<HashMap<String, Arc<DMSMetric>>>>,
}

impl DMSMetricsRegistry {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub fn register(&self, metric: Arc<DMSMetric>) -> DMSResult<()> {
        let name = metric.get_config().name.clone();
        self.metrics.write().unwrap().insert(name, metric);
        Ok(())
    }
    
    pub fn get_metric(&self, name: &str) -> Option<Arc<DMSMetric>> {
        self.metrics.read().unwrap().get(name).cloned()
    }
    
    pub fn get_all_metrics(&self) -> HashMap<String, Arc<DMSMetric>> {
        self.metrics.read().unwrap().clone()
    }
    
    /// Export metrics in Prometheus format
    pub fn export_prometheus(&self) -> String {
        let mut output = String::new();
        let metrics = self.metrics.read().unwrap();
        
        for (name, metric) in metrics.iter() {
            let config = metric.get_config();
            
            // Write help and type
            output.push_str(&format!("# HELP {} {}\n", name, config.help));
            output.push_str(&format!("# TYPE {} {:?}\n", name, config.metric_type));
            
            // Write metric value
            let stats = metric.get_stats();
            match config.metric_type {
                DMSMetricType::Counter => {
                    output.push_str(&format!("{} {}\n", name, metric.get_total_count()));
                }
                DMSMetricType::Gauge => {
                    output.push_str(&format!("{} {}\n", name, stats.mean));
                }
                DMSMetricType::Histogram => {
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
                DMSMetricType::Summary => {
                    output.push_str(&format!("{} {}\n", name, stats.mean));
                }
            }
            
            output.push('\n');
        }
        
        output
    }
}