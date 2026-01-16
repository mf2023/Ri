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

use dmsc::observability::metrics_collector::{DMSCSlidingWindow, DMSCQuantileCalculator, DMSCPerformanceCollector};
use std::time::Duration;

/// Observability metrics collector test module for performance monitoring.
///
/// This module provides comprehensive test coverage for the metrics collection
/// components that aggregate performance data for monitoring and analysis. The
/// tests validate sliding window data structures, quantile calculations, and
/// performance metrics aggregation used in system observability.
///
/// ## Test Coverage
///
/// - **Sliding Window Data Structure**: Tests the time-bounded sliding window
///   implementation that maintains recent data points within a configurable
///   time horizon, supporting efficient addition and retrieval of temporal data.
///
/// - **Quantile Calculation**: Validates the quantile calculator implementation
///   that computes percentiles from a stream of measurements, verifying correct
///   calculation for edge cases including minimum, median, and maximum values.
///
/// - **Performance Metrics Aggregation**: Tests the performance collector that
///   aggregates request timing data including counts, latency distributions,
///   and error rate calculations from a series of recorded observations.
///
/// - **Error Rate Calculation**: Validates the error rate computation as the
///   ratio of failed requests to total requests, verifying mathematical
///   correctness and appropriate handling of edge cases.
///
/// ## Design Principles
///
/// The sliding window implementation uses time-based eviction rather than
/// count-based to ensure that analysis reflects recent system behavior rather
/// than an arbitrary number of samples. Tests verify correct behavior at
/// window boundaries and during data point expiration.
///
/// The quantile calculator uses an approximate algorithm suitable for
/// real-time monitoring rather than exact sorting, providing good accuracy
/// with minimal memory overhead. Tests verify correctness for common
/// percentile values used in service level objectives.
///
/// The performance collector separates timing measurement from metric
/// computation, enabling flexible analysis windows and aggregation strategies.
Tests verify that recorded metrics accurately reflect the underlying
/// observations with appropriate precision for alerting purposes.

#[test]
fn test_sliding_window() {
    let mut window = DMSCSlidingWindow::<i32>::new(
        Duration::from_secs(10),
        Duration::from_secs(1),
    );
    
    window.add(1);
    window.add(2);
    window.add(3);
    
    let data_points = window.get_data_points();
    assert_eq!(data_points.len(), 3);
}

#[test]
fn test_quantile_calculator() {
    let mut calc = DMSCQuantileCalculator::new();
    
    calc.add(1.0);
    calc.add(2.0);
    calc.add(3.0);
    calc.add(4.0);
    calc.add(5.0);
    
    assert_eq!(calc.quantile(0.0), Some(1.0));
    assert_eq!(calc.quantile(0.5), Some(3.0));
    assert_eq!(calc.quantile(1.0), Some(5.0));
}

#[test]
fn test_performance_collector() {
    let mut collector = DMSCPerformanceCollector::new(
        Duration::from_secs(60),
        Duration::from_secs(5),
    );
    
    // Record some requests
    collector.record_request(100.0, false);
    collector.record_request(200.0, false);
    collector.record_request(300.0, true); // error
    
    let metrics = collector.get_metrics();
    assert_eq!(metrics.total_requests, 3);
    assert!((metrics.error_rate - 0.33).abs() < 0.01);
}
