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

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use serde::{Serialize, Deserialize};

/// Sliding window for time-series data collection
#[derive(Debug, Clone)]
pub struct DMSSlidingWindow<T> {
    _window_size: Duration,
    bucket_size: Duration,
    buckets: VecDeque<WindowBucket<T>>,
    current_time: Instant,
}

#[derive(Debug, Clone)]
struct WindowBucket<T> {
    _start_time: Instant,
    end_time: Instant,
    data_points: Vec<T>,
}

impl<T> DMSSlidingWindow<T> {
    pub fn _Fnew(window_size: Duration, bucket_size: Duration) -> Self {
        let bucket_count = (window_size.as_millis() / bucket_size.as_millis()).max(1) as usize;
        let mut buckets = VecDeque::with_capacity(bucket_count);
        
        let now = Instant::now();
        for i in 0..bucket_count {
            let bucket_start = now - window_size + Duration::from_millis(i as u64 * bucket_size.as_millis() as u64);
            buckets.push_back(WindowBucket {
                _start_time: bucket_start,
                end_time: bucket_start + bucket_size,
                data_points: Vec::new(),
            });
        }
        
        Self {
            _window_size: window_size,
            bucket_size,
            buckets,
            current_time: now,
        }
    }
    
    /// Add a data point to the current window
    pub fn _Fadd(&mut self, value: T) {
        self._Fadvance_window();
        
        if let Some(current_bucket) = self.buckets.back_mut() {
            current_bucket.data_points.push(value);
        }
    }
    
    /// Get all data points in the current window
    pub fn _Fget_data_points(&self) -> Vec<&T> {
        self.buckets
            .iter()
            .flat_map(|bucket| &bucket.data_points)
            .collect()
    }
    
    /// Get data points count in the current window
    pub fn _Fcount(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.data_points.len()).sum()
    }
    
    /// Advance the window to current time
    fn _Fadvance_window(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.current_time);
        
        if elapsed >= self.bucket_size {
            let buckets_to_advance = (elapsed.as_millis() / self.bucket_size.as_millis()) as usize;
            
            for _ in 0..buckets_to_advance.min(self.buckets.len()) {
                self.buckets.pop_front();
                
                let new_bucket_start = self.buckets.back().unwrap().end_time;
                self.buckets.push_back(WindowBucket {
                    _start_time: new_bucket_start,
                    end_time: new_bucket_start + self.bucket_size,
                    data_points: Vec::new(),
                });
            }
            
            self.current_time = now;
        }
    }
}

/// Quantile calculator for performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSQuantileCalculator {
    sorted_data: Vec<f64>,
}

impl DMSQuantileCalculator {
    pub fn _Fnew() -> Self {
        Self {
            sorted_data: Vec::new(),
        }
    }
    
    /// Add a data point
    pub fn _Fadd(&mut self, value: f64) {
        self.sorted_data.push(value);
    }
    
    /// Calculate quantile (0.0 to 1.0)
    pub fn _Fquantile(&mut self, q: f64) -> Option<f64> {
        if self.sorted_data.is_empty() || q < 0.0 || q > 1.0 {
            return None;
        }
        
        self.sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        let n = self.sorted_data.len();
        let index = q * (n - 1) as f64;
        let lower = index.floor() as usize;
        let upper = index.ceil() as usize;
        
        if lower == upper {
            Some(self.sorted_data[lower])
        } else {
            let weight = index - lower as f64;
            let lower_val = self.sorted_data[lower];
            let upper_val = self.sorted_data[upper];
            Some(lower_val + weight * (upper_val - lower_val))
        }
    }
    
    /// Calculate multiple quantiles at once
    pub fn _Fquantiles(&mut self, quantiles: &[f64]) -> Vec<Option<f64>> {
        quantiles.iter().map(|&q| self._Fquantile(q)).collect()
    }
    
    /// Get minimum value
    pub fn _Fmin(&self) -> Option<f64> {
        self.sorted_data.iter().fold(None, |min, &val| {
            match min {
                None => Some(val),
                Some(m) => Some(m.min(val)),
            }
        })
    }
    
    /// Get maximum value
    pub fn _Fmax(&self) -> Option<f64> {
        self.sorted_data.iter().fold(None, |max, &val| {
            match max {
                None => Some(val),
                Some(m) => Some(m.max(val)),
            }
        })
    }
    
    /// Get mean value
    pub fn _Fmean(&self) -> Option<f64> {
        if self.sorted_data.is_empty() {
            return None;
        }
        
        let sum: f64 = self.sorted_data.iter().sum();
        Some(sum / self.sorted_data.len() as f64)
    }
    
    /// Get standard deviation
    pub fn _Fstd_dev(&self) -> Option<f64> {
        let mean = self._Fmean()?;
        if self.sorted_data.len() <= 1 {
            return Some(0.0);
        }
        
        let variance: f64 = self.sorted_data
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / (self.sorted_data.len() - 1) as f64;
        
        Some(variance.sqrt())
    }
    
    /// Clear all data
    pub fn _Fclear(&mut self) {
        self.sorted_data.clear();
    }
}

/// Performance metrics collector with sliding window and quantile calculation
pub struct DMSPerformanceCollector {
    latency_window: DMSSlidingWindow<f64>,
    throughput_window: DMSSlidingWindow<u64>,
    error_rate_window: DMSSlidingWindow<bool>,
    quantile_calculator: DMSQuantileCalculator,
}

impl DMSPerformanceCollector {
    pub fn _Fnew(window_size: Duration, bucket_size: Duration) -> Self {
        Self {
            latency_window: DMSSlidingWindow::_Fnew(window_size, bucket_size),
            throughput_window: DMSSlidingWindow::_Fnew(window_size, bucket_size),
            error_rate_window: DMSSlidingWindow::_Fnew(window_size, bucket_size),
            quantile_calculator: DMSQuantileCalculator::_Fnew(),
        }
    }
    
    /// Record a request with latency and error status
    pub fn _Frecord_request(&mut self, latency_ms: f64, is_error: bool) {
        self.latency_window._Fadd(latency_ms);
        self.throughput_window._Fadd(1);
        self.error_rate_window._Fadd(is_error);
    }
    
    /// Get current performance metrics
    pub fn _Fget_metrics(&mut self) -> DMSPerformanceMetrics {
        let latencies: Vec<f64> = self.latency_window._Fget_data_points()
            .iter()
            .map(|&&x| x)
            .collect();
        
        // Update quantile calculator with current data
        self.quantile_calculator._Fclear();
        for &latency in &latencies {
            self.quantile_calculator._Fadd(latency);
        }
        
        let p50 = self.quantile_calculator._Fquantile(0.5).unwrap_or(0.0);
        let p95 = self.quantile_calculator._Fquantile(0.95).unwrap_or(0.0);
        let p99 = self.quantile_calculator._Fquantile(0.99).unwrap_or(0.0);
        
        let errors: Vec<bool> = self.error_rate_window._Fget_data_points()
            .iter()
            .map(|&&x| x)
            .collect();
        
        let error_count = errors.iter().filter(|&&x| x).count();
        let total_requests = errors.len();
        let error_rate = if total_requests > 0 {
            error_count as f64 / total_requests as f64
        } else {
            0.0
        };
        
        DMSPerformanceMetrics {
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            mean_latency_ms: self.quantile_calculator._Fmean().unwrap_or(0.0),
            min_latency_ms: self.quantile_calculator._Fmin().unwrap_or(0.0),
            max_latency_ms: self.quantile_calculator._Fmax().unwrap_or(0.0),
            throughput_rps: self.throughput_window._Fcount() as f64,
            error_rate,
            total_requests: total_requests as u64,
        }
    }
}

/// Performance metrics snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSPerformanceMetrics {
    pub p50_latency_ms: f64,
    pub p95_latency_ms: f64,
    pub p99_latency_ms: f64,
    pub mean_latency_ms: f64,
    pub min_latency_ms: f64,
    pub max_latency_ms: f64,
    pub throughput_rps: f64,
    pub error_rate: f64,
    pub total_requests: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_sliding_window() {
        let mut window = DMSSlidingWindow::<i32>::_Fnew(
            Duration::from_secs(10),
            Duration::from_secs(1),
        );
        
        window._Fadd(1);
        window._Fadd(2);
        window._Fadd(3);
        
        let data_points = window._Fget_data_points();
        assert_eq!(data_points.len(), 3);
    }
    
    #[test]
    fn test_quantile_calculator() {
        let mut calc = DMSQuantileCalculator::_Fnew();
        
        calc._Fadd(1.0);
        calc._Fadd(2.0);
        calc._Fadd(3.0);
        calc._Fadd(4.0);
        calc._Fadd(5.0);
        
        assert_eq!(calc._Fquantile(0.0), Some(1.0));
        assert_eq!(calc._Fquantile(0.5), Some(3.0));
        assert_eq!(calc._Fquantile(1.0), Some(5.0));
    }
    
    #[test]
    fn test_performance_collector() {
        let mut collector = DMSPerformanceCollector::_Fnew(
            Duration::from_secs(60),
            Duration::from_secs(5),
        );
        
        // Record some requests
        collector._Frecord_request(100.0, false);
        collector._Frecord_request(200.0, false);
        collector._Frecord_request(300.0, true); // error
        
        let metrics = collector._Fget_metrics();
        assert_eq!(metrics.total_requests, 3);
        assert!((metrics.error_rate - 0.33).abs() < 0.01);
    }
}