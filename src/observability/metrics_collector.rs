//! Copyright © 2025 Wenze Wei. All Rights Reserved.
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

//! # Metrics Collector
//!
//! This file implements a comprehensive metrics collection system for the DMSC framework. It provides
//! tools for collecting, analyzing, and reporting performance and system metrics. The system includes
//! sliding window data structures for time-series data, quantile calculators for performance analysis,
//! and system metrics collectors for monitoring CPU, memory, disk, and network usage.
//!
//! ## Key Components
//!
//! - **DMSCSlidingWindow**: Sliding window for time-series data collection
//! - **DMSCQuantileCalculator**: Quantile calculator for performance metrics
//! - **DMSCPerformanceCollector**: Performance metrics collector with sliding window and quantile calculation
//! - **DMSCPerformanceMetrics**: Performance metrics snapshot structure
//! - **DMSCCPUMetrics**: CPU metrics structure
//! - **DMSCMemoryMetrics**: Memory metrics structure
//! - **DMSCDiskMetrics**: Disk metrics structure
//! - **DMSCNetworkMetrics**: Network metrics structure
//! - **DMSCSystemMetrics**: System metrics snapshot structure
//! - **DMSCSystemMetricsCollector**: System metrics collector
//!
//! ## Design Principles
//!
//! 1. **Time-Series Data**: Uses sliding windows for efficient time-series data collection
//! 2. **Performance Focus**: Includes quantile calculation for accurate performance analysis
//! 3. **System Monitoring**: Comprehensive system metrics collection
//! 4. **Serialization Support**: All metrics structures are serializable for easy reporting
//! 5. **Low Overhead**: Efficient data structures to minimize performance impact
//! 6. **Flexible Configuration**: Configurable window sizes and bucket sizes
//! 7. **Cross-Platform**: Uses sysinfo crate for cross-platform system metrics collection
//! 8. **Real-Time Analysis**: Provides real-time metrics calculation
//!
//! ## Usage
//!
//! ```rust
//! use dms::observability::{DMSCPerformanceCollector, DMSCSystemMetricsCollector};
//! use std::time::Duration;
//!
//! fn example() {
//!     // Create a performance collector with a 1-minute window and 5-second buckets
//!     let mut perf_collector = DMSCPerformanceCollector::new(
//!         Duration::from_secs(60),
//!         Duration::from_secs(5)
//!     );
//!     
//!     // Record a request
//!     perf_collector.record_request(12.5, false);
//!     
//!     // Get performance metrics
//!     let perf_metrics = perf_collector.get_metrics();
//!     println!("P50 latency: {}ms", perf_metrics.p50_latency_ms);
//!     println!("Throughput: {} rps", perf_metrics.throughput_rps);
//!     println!("Error rate: {:.2}%", perf_metrics.error_rate * 100.0);
//!     
//!     // Create a system metrics collector
//!     let mut sys_collector = DMSCSystemMetricsCollector::new();
//!     
//!     // Collect system metrics
//!     let sys_metrics = sys_collector.collect();
//!     println!("CPU usage: {:.2}%", sys_metrics.cpu.total_usage_percent);
//!     println!("Memory usage: {:.2}%", sys_metrics.memory.usage_percent);
//!     println!("Network received: {} bytes/s", sys_metrics.network.received_bytes_per_sec);
//! }
//! ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use sysinfo::{CpuExt, DiskExt, NetworkExt, System, SystemExt};

/// Sliding window for time-series data collection.
///
/// This struct implements a sliding window for efficient time-series data collection. It divides
/// the window into buckets and automatically advances the window as time passes, removing old
/// data points.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct DMSCSlidingWindow<T> {
    /// Total size of the sliding window
    _window_size: Duration,
    /// Size of each bucket within the window
    bucket_size: Duration,
    /// Queue of buckets containing data points
    buckets: VecDeque<WindowBucket<T>>,
    /// Current time for window advancement
    current_time: Instant,
}

/// Internal bucket structure for the sliding window.
///
/// This struct represents a single bucket within the sliding window, containing a collection
/// of data points for a specific time interval.
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct WindowBucket<T> {
    /// Start time of the bucket
    _start_time: Instant,
    /// End time of the bucket
    end_time: Instant,
    /// Data points collected in this bucket
    data_points: Vec<T>,
}

#[allow(dead_code)]
impl<T> DMSCSlidingWindow<T> {
    /// Creates a new sliding window with the specified window size and bucket size.
    ///
    /// # Parameters
    ///
    /// - `window_size`: Total size of the sliding window
    /// - `bucket_size`: Size of each bucket within the window
    ///
    /// # Returns
    ///
    /// A new DMSCSlidingWindow instance
    pub fn new(window_size: Duration, bucket_size: Duration) -> Self {
        let bucket_count = (window_size.as_millis() / bucket_size.as_millis()).max(1) as usize;
        let mut buckets = VecDeque::with_capacity(bucket_count);

        let now = Instant::now();
        for i in 0..bucket_count {
            let bucket_start = now - window_size
                + Duration::from_millis(i as u64 * bucket_size.as_millis() as u64);
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

    /// Adds a data point to the current window.
    ///
    /// # Parameters
    ///
    /// - `value`: The data point to add
    pub fn add(&mut self, value: T) {
        self.advance_window();

        if let Some(current_bucket) = self.buckets.back_mut() {
            current_bucket.data_points.push(value);
        }
    }

    /// Gets all data points in the current window.
    ///
    /// # Returns
    ///
    /// A vector of references to all data points in the current window
    pub fn get_data_points(&self) -> Vec<&T> {
        self.buckets
            .iter()
            .flat_map(|bucket| &bucket.data_points)
            .collect()
    }

    /// Gets the count of data points in the current window.
    ///
    /// # Returns
    ///
    /// The number of data points in the current window
    pub fn count(&self) -> usize {
        self.buckets
            .iter()
            .map(|bucket| bucket.data_points.len())
            .sum()
    }

    /// Advances the window to the current time, removing old buckets.
    fn advance_window(&mut self) {
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

/// Quantile calculator for performance metrics.
///
/// This struct provides methods for calculating quantiles (percentiles) from a set of data points.
/// It sorts the data and uses linear interpolation for non-integer indices.
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCQuantileCalculator {
    /// Sorted list of data points
    sorted_data: Vec<f64>,
}

#[allow(dead_code)]
impl Default for DMSCQuantileCalculator {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCQuantileCalculator {
    /// Creates a new quantile calculator.
    ///
    /// # Returns
    ///
    /// A new DMSCQuantileCalculator instance
    pub fn new() -> Self {
        Self {
            sorted_data: Vec::new(),
        }
    }

    /// Adds a data point to the calculator.
    ///
    /// # Parameters
    ///
    /// - `value`: The data point to add
    pub fn add(&mut self, value: f64) {
        self.sorted_data.push(value);
    }

    /// Calculates the specified quantile (0.0 to 1.0).
    ///
    /// # Parameters
    ///
    /// - `q`: The quantile to calculate (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// An `Option<f64>` containing the calculated quantile, or None if the data is empty or q is out of range
    pub fn quantile(&mut self, q: f64) -> Option<f64> {
        if self.sorted_data.is_empty() || !(0.0..=1.0).contains(&q) {
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

    /// Calculates multiple quantiles at once.
    ///
    /// # Parameters
    ///
    /// - `quantiles`: A slice of quantiles to calculate (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// A vector of `Option<f64>` containing the calculated quantiles
    pub fn quantiles(&mut self, quantiles: &[f64]) -> Vec<Option<f64>> {
        quantiles.iter().map(|&q| self.quantile(q)).collect()
    }

    /// Gets the minimum value in the data set.
    ///
    /// # Returns
    ///
    /// An `Option<f64>` containing the minimum value, or None if the data is empty
    pub fn min(&self) -> Option<f64> {
        self.sorted_data.iter().fold(None, |min, &val| match min {
            None => Some(val),
            Some(m) => Some(m.min(val)),
        })
    }

    /// Gets the maximum value in the data set.
    ///
    /// # Returns
    ///
    /// An `Option<f64>` containing the maximum value, or None if the data is empty
    pub fn max(&self) -> Option<f64> {
        self.sorted_data.iter().fold(None, |max, &val| match max {
            None => Some(val),
            Some(m) => Some(m.max(val)),
        })
    }

    /// Gets the mean value of the data set.
    ///
    /// # Returns
    ///
    /// An `Option<f64>` containing the mean value, or None if the data is empty
    pub fn mean(&self) -> Option<f64> {
        if self.sorted_data.is_empty() {
            return None;
        }

        let sum: f64 = self.sorted_data.iter().sum();
        Some(sum / self.sorted_data.len() as f64)
    }

    /// Gets the standard deviation of the data set.
    ///
    /// # Returns
    ///
    /// An `Option<f64>` containing the standard deviation, or None if the data is empty
    pub fn std_dev(&self) -> Option<f64> {
        let mean = self.mean()?;
        if self.sorted_data.len() <= 1 {
            return Some(0.0);
        }

        let variance: f64 = self
            .sorted_data
            .iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>()
            / (self.sorted_data.len() - 1) as f64;

        Some(variance.sqrt())
    }

    /// Clears all data from the calculator.
    pub fn clear(&mut self) {
        self.sorted_data.clear();
    }
}

/// Performance metrics collector with sliding window and quantile calculation.
///
/// This struct collects performance metrics using sliding windows and provides quantile-based
/// performance analysis.
#[allow(dead_code)]
pub struct DMSCPerformanceCollector {
    /// Sliding window for latency data
    latency_window: DMSCSlidingWindow<f64>,
    /// Sliding window for throughput data
    throughput_window: DMSCSlidingWindow<u64>,
    /// Sliding window for error rate data
    error_rate_window: DMSCSlidingWindow<bool>,
    /// Quantile calculator for performance analysis
    quantile_calculator: DMSCQuantileCalculator,
}

#[allow(dead_code)]
impl DMSCPerformanceCollector {
    /// Creates a new performance collector with the specified window size and bucket size.
    ///
    /// # Parameters
    ///
    /// - `window_size`: Total size of the sliding window
    /// - `bucket_size`: Size of each bucket within the window
    ///
    /// # Returns
    ///
    /// A new DMSCPerformanceCollector instance
    pub fn new(window_size: Duration, bucket_size: Duration) -> Self {
        Self {
            latency_window: DMSCSlidingWindow::new(window_size, bucket_size),
            throughput_window: DMSCSlidingWindow::new(window_size, bucket_size),
            error_rate_window: DMSCSlidingWindow::new(window_size, bucket_size),
            quantile_calculator: DMSCQuantileCalculator::new(),
        }
    }

    /// Records a request with latency and error status.
    ///
    /// # Parameters
    ///
    /// - `latency_ms`: The request latency in milliseconds
    /// - `is_error`: Whether the request resulted in an error
    pub fn record_request(&mut self, latency_ms: f64, is_error: bool) {
        self.latency_window.add(latency_ms);
        self.throughput_window.add(1);
        self.error_rate_window.add(is_error);
    }

    /// Gets the current performance metrics.
    ///
    /// # Returns
    ///
    /// A DMSCPerformanceMetrics instance containing the current performance metrics
    pub fn get_metrics(&mut self) -> DMSCPerformanceMetrics {
        let latencies: Vec<f64> = self
            .latency_window
            .get_data_points()
            .iter()
            .map(|&&x| x)
            .collect();

        // Update quantile calculator with current data
        self.quantile_calculator.clear();
        for &latency in &latencies {
            self.quantile_calculator.add(latency);
        }

        let p50 = self.quantile_calculator.quantile(0.5).unwrap_or(0.0);
        let p95 = self.quantile_calculator.quantile(0.95).unwrap_or(0.0);
        let p99 = self.quantile_calculator.quantile(0.99).unwrap_or(0.0);

        let errors: Vec<bool> = self
            .error_rate_window
            .get_data_points()
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

        DMSCPerformanceMetrics {
            p50_latency_ms: p50,
            p95_latency_ms: p95,
            p99_latency_ms: p99,
            mean_latency_ms: self.quantile_calculator.mean().unwrap_or(0.0),
            min_latency_ms: self.quantile_calculator.min().unwrap_or(0.0),
            max_latency_ms: self.quantile_calculator.max().unwrap_or(0.0),
            throughput_rps: self.throughput_window.count() as f64,
            error_rate,
            total_requests: total_requests as u64,
        }
    }
}

/// Performance metrics snapshot.
///
/// This struct represents a snapshot of performance metrics at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCPerformanceMetrics {
    /// 50th percentile latency in milliseconds
    pub p50_latency_ms: f64,
    /// 95th percentile latency in milliseconds
    pub p95_latency_ms: f64,
    /// 99th percentile latency in milliseconds
    pub p99_latency_ms: f64,
    /// Mean latency in milliseconds
    pub mean_latency_ms: f64,
    /// Minimum latency in milliseconds
    pub min_latency_ms: f64,
    /// Maximum latency in milliseconds
    pub max_latency_ms: f64,
    /// Throughput in requests per second
    pub throughput_rps: f64,
    /// Error rate (0.0 to 1.0)
    pub error_rate: f64,
    /// Total number of requests
    pub total_requests: u64,
}

/// CPU metrics.
///
/// This struct represents CPU usage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCCPUMetrics {
    /// Total CPU usage percentage
    pub total_usage_percent: f64,
    /// Per-core CPU usage percentages
    pub per_core_usage: Vec<f64>,
    /// Number of context switches (platform dependent)
    pub context_switches: u64,
    /// Number of interrupts (platform dependent)
    pub interrupts: u64,
}

/// Memory metrics.
///
/// This struct represents memory usage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCMemoryMetrics {
    /// Total memory in bytes
    pub total_bytes: u64,
    /// Used memory in bytes
    pub used_bytes: u64,
    /// Free memory in bytes
    pub free_bytes: u64,
    /// Memory usage percentage
    pub usage_percent: f64,
    /// Total swap memory in bytes
    pub swap_total_bytes: u64,
    /// Used swap memory in bytes
    pub swap_used_bytes: u64,
    /// Free swap memory in bytes
    pub swap_free_bytes: u64,
    /// Swap usage percentage
    pub swap_usage_percent: f64,
}

/// Disk metrics.
///
/// This struct represents disk usage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCDiskMetrics {
    /// Total disk space in bytes
    pub total_bytes: u64,
    /// Used disk space in bytes
    pub used_bytes: u64,
    /// Free disk space in bytes
    pub free_bytes: u64,
    /// Disk usage percentage
    pub usage_percent: f64,
    /// Total bytes read (platform dependent)
    pub read_bytes: u64,
    /// Total bytes written (platform dependent)
    pub write_bytes: u64,
    /// Total read operations (platform dependent)
    pub read_count: u64,
    /// Total write operations (platform dependent)
    pub write_count: u64,
}

/// Network metrics.
///
/// This struct represents network usage metrics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCNetworkMetrics {
    /// Total bytes received
    pub total_received_bytes: u64,
    /// Total bytes transmitted
    pub total_transmitted_bytes: u64,
    /// Bytes received per second
    pub received_bytes_per_sec: u64,
    /// Bytes transmitted per second
    pub transmitted_bytes_per_sec: u64,
    /// Total packets received
    pub total_received_packets: u64,
    /// Total packets transmitted
    pub total_transmitted_packets: u64,
    /// Packets received per second
    pub received_packets_per_sec: u64,
    /// Packets transmitted per second
    pub transmitted_packets_per_sec: u64,
}

/// System metrics snapshot.
///
/// This struct represents a snapshot of system metrics at a specific point in time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSCSystemMetrics {
    /// CPU metrics
    pub cpu: DMSCCPUMetrics,
    /// Memory metrics
    pub memory: DMSCMemoryMetrics,
    /// Disk metrics
    pub disk: DMSCDiskMetrics,
    /// Network metrics
    pub network: DMSCNetworkMetrics,
    /// Timestamp of the metrics collection (Unix timestamp in milliseconds)
    pub timestamp: u64,
}

/// System metrics collector.
///
/// This struct collects system metrics using the sysinfo crate, providing cross-platform
/// system monitoring capabilities.
#[allow(dead_code)]
pub struct DMSCSystemMetricsCollector {
    /// sysinfo System instance for collecting metrics
    system: System,
    /// Last network bytes received
    last_network_received: u64,
    /// Last network bytes transmitted
    last_network_transmitted: u64,
    /// Last network packets received
    last_network_received_packets: u64,
    /// Last network packets transmitted
    last_network_transmitted_packets: u64,
    /// Last network metrics collection time
    last_network_time: Instant,
}

#[allow(dead_code)]
impl Default for DMSCSystemMetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl DMSCSystemMetricsCollector {
    /// Creates a new system metrics collector.
    ///
    /// # Returns
    ///
    /// A new DMSCSystemMetricsCollector instance
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let (received_bytes, transmitted_bytes, received_packets, transmitted_packets) =
            Self::get_network_total(&system);

        Self {
            system,
            last_network_received: received_bytes,
            last_network_transmitted: transmitted_bytes,
            last_network_received_packets: received_packets,
            last_network_transmitted_packets: transmitted_packets,
            last_network_time: Instant::now(),
        }
    }

    /// Collects the current system metrics.
    ///
    /// # Returns
    ///
    /// A DMSCSystemMetrics instance containing the current system metrics
    pub fn collect(&mut self) -> DMSCSystemMetrics {
        self.system.refresh_all();

        let cpu = self.get_cpu_metrics();
        let memory = self.get_memory_metrics();
        let disk = self.get_disk_metrics();
        let network = self.get_network_metrics();

        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        
        DMSCSystemMetrics {
            cpu,
            memory,
            disk,
            network,
            timestamp,
        }
    }

    /// Gets CPU metrics from the system.
    ///
    /// # Returns
    ///
    /// A DMSCCPUMetrics instance containing the CPU metrics
    fn get_cpu_metrics(&self) -> DMSCCPUMetrics {
        let total_usage = self.system.global_cpu_info().cpu_usage();
        let per_core_usage: Vec<f64> = self
            .system
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage() as f64)
            .collect();

        // Note: sysinfo crate doesn't expose context switches and interrupts on all platforms
        // These values will be 0 on platforms where they're not available
        DMSCCPUMetrics {
            total_usage_percent: total_usage as f64,
            per_core_usage,
            context_switches: 0,
            interrupts: 0,
        }
    }

    /// Gets memory metrics from the system.
    ///
    /// # Returns
    ///
    /// A DMSCMemoryMetrics instance containing the memory metrics
    fn get_memory_metrics(&self) -> DMSCMemoryMetrics {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let free = self.system.free_memory();
        let usage_percent = (used as f64 / total as f64) * 100.0;

        let swap_total = self.system.total_swap();
        let swap_used = self.system.used_swap();
        let swap_free = self.system.free_swap();
        let swap_usage_percent = if swap_total > 0 {
            (swap_used as f64 / swap_total as f64) * 100.0
        } else {
            0.0
        };

        DMSCMemoryMetrics {
            total_bytes: total,
            used_bytes: used,
            free_bytes: free,
            usage_percent,
            swap_total_bytes: swap_total,
            swap_used_bytes: swap_used,
            swap_free_bytes: swap_free,
            swap_usage_percent,
        }
    }

    /// Gets disk metrics from the system.
    ///
    /// # Returns
    ///
    /// A DMSCDiskMetrics instance containing the disk metrics
    fn get_disk_metrics(&self) -> DMSCDiskMetrics {
        // Get first disk for now
        if let Some(disk) = self.system.disks().first() {
            let total = disk.total_space();
            let available = disk.available_space();
            let used = total - available;
            let usage_percent = (used as f64 / total as f64) * 100.0;

            // Note: sysinfo crate doesn't expose I/O statistics on all platforms
            // These values will be 0 on platforms where they're not available
            DMSCDiskMetrics {
                total_bytes: total,
                used_bytes: used,
                free_bytes: available,
                usage_percent,
                read_bytes: 0,
                write_bytes: 0,
                read_count: 0,
                write_count: 0,
            }
        } else {
            DMSCDiskMetrics {
                total_bytes: 0,
                used_bytes: 0,
                free_bytes: 0,
                usage_percent: 0.0,
                read_bytes: 0,
                write_bytes: 0,
                read_count: 0,
                write_count: 0,
            }
        }
    }

    /// Gets network metrics from the system.
    ///
    /// # Returns
    ///
    /// A DMSCNetworkMetrics instance containing the network metrics
    fn get_network_metrics(&mut self) -> DMSCNetworkMetrics {
        let (received_bytes, transmitted_bytes, received_packets, transmitted_packets) =
            Self::get_network_total(&self.system);

        let now = Instant::now();
        let elapsed = now
            .duration_since(self.last_network_time)
            .as_secs_f64()
            .max(1.0);

        let received_bytes_per_sec =
            ((received_bytes - self.last_network_received) as f64 / elapsed) as u64;
        let transmitted_bytes_per_sec =
            ((transmitted_bytes - self.last_network_transmitted) as f64 / elapsed) as u64;
        let received_packets_per_sec =
            ((received_packets - self.last_network_received_packets) as f64 / elapsed) as u64;
        let transmitted_packets_per_sec =
            ((transmitted_packets - self.last_network_transmitted_packets) as f64 / elapsed) as u64;

        // Update last values
        self.last_network_received = received_bytes;
        self.last_network_transmitted = transmitted_bytes;
        self.last_network_received_packets = received_packets;
        self.last_network_transmitted_packets = transmitted_packets;
        self.last_network_time = now;

        DMSCNetworkMetrics {
            total_received_bytes: received_bytes,
            total_transmitted_bytes: transmitted_bytes,
            received_bytes_per_sec,
            transmitted_bytes_per_sec,
            total_received_packets: received_packets,
            total_transmitted_packets: transmitted_packets,
            received_packets_per_sec,
            transmitted_packets_per_sec,
        }
    }

    /// Gets total network metrics from all interfaces.
    ///
    /// # Parameters
    ///
    /// - `system`: The sysinfo System instance
    ///
    /// # Returns
    ///
    /// A tuple containing (received_bytes, transmitted_bytes, received_packets, transmitted_packets)
    fn get_network_total(system: &System) -> (u64, u64, u64, u64) {
        let mut received_bytes = 0;
        let mut transmitted_bytes = 0;
        let mut received_packets = 0;
        let mut transmitted_packets = 0;

        for (_, data) in system.networks() {
            received_bytes += data.received();
            transmitted_bytes += data.transmitted();
            received_packets += data.packets_received();
            transmitted_packets += data.packets_transmitted();
        }

        (
            received_bytes,
            transmitted_bytes,
            received_packets,
            transmitted_packets,
        )
    }
}
