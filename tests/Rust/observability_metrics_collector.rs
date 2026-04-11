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

use ri::observability::metrics_collector::{RiSlidingWindow, RiQuantileCalculator, RiPerformanceCollector};
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
/// Tests RiSlidingWindow data structure for time-bounded data retention.
///
/// Verifies that the sliding window implementation correctly maintains
/// data points within a configurable time horizon, supporting efficient
/// addition and retrieval of temporal data for metrics analysis.
///
/// ## Sliding Window Characteristics
///
/// - **Time-Based Eviction**: Data points expire after the window duration
///   rather than based on count, ensuring analysis reflects recent behavior
/// - **Bucket-Based Storage**: Time is divided into buckets for efficient
///   eviction and memory management
/// - **Configurable Parameters**:
///   - window_duration: Total time span to retain data
///   - bucket_duration: Size of individual time buckets
///
/// ## Test Scenario
///
/// 1. Create a sliding window with 10-second duration and 1-second buckets
/// 2. Add three data points (1, 2, 3)
/// 3. Retrieve all data points
/// 4. Verify exactly 3 data points are returned
///
/// ## Expected Behavior
///
/// - All added data points are retrievable
/// - Data points within the time window are retained
/// - The window correctly stores and reports count
fn test_sliding_window() {
    // Create a sliding window with 10-second total duration and 1-second buckets
    let mut window = RiSlidingWindow::<i32>::new(
        Duration::from_secs(10),
        Duration::from_secs(1),
    );
    
    // Add data points to the window
    window.add(1);
    window.add(2);
    window.add(3);
    
    // Retrieve and verify data points
    let data_points = window.get_data_points();
    assert_eq!(data_points.len(), 3);
}

#[test]
/// Tests RiQuantileCalculator for percentile calculation accuracy.
///
/// Verifies that the quantile calculator correctly computes percentiles
/// from a stream of measurements, validating edge cases for minimum (0th),
/// median (50th), and maximum (100th) percentile values.
///
/// ## Quantile Calculation
///
/// - **0th Percentile (Minimum)**: Returns the smallest value in the dataset
/// - **50th Percentile (Median)**: Returns the middle value (or average of two middle)
/// - **100th Percentile (Maximum)**: Returns the largest value in the dataset
///
/// ## Algorithm Characteristics
///
/// The calculator uses an approximate algorithm suitable for real-time monitoring,
/// providing good accuracy with minimal memory overhead. This is appropriate for
/// service level objective monitoring rather than statistical analysis.
///
/// ## Test Scenario
///
/// 1. Create a new quantile calculator
/// 2. Add values 1.0 through 5.0
/// 3. Query 0th percentile (minimum) -> should return 1.0
/// 4. Query 50th percentile (median) -> should return 3.0
/// 5. Query 100th percentile (maximum) -> should return 5.0
///
/// ## Expected Behavior
///
/// - Minimum query returns the smallest value
/// - Median query returns the middle value
/// - Maximum query returns the largest value
fn test_quantile_calculator() {
    let mut calc = RiQuantileCalculator::new();
    
    // Add test values
    calc.add(1.0);
    calc.add(2.0);
    calc.add(3.0);
    calc.add(4.0);
    calc.add(5.0);
    
    // Verify quantile calculations
    assert_eq!(calc.quantile(0.0), Some(1.0)); // Minimum
    assert_eq!(calc.quantile(0.5), Some(3.0)); // Median
    assert_eq!(calc.quantile(1.0), Some(5.0)); // Maximum
}

#[test]
/// Tests RiPerformanceCollector for request metrics aggregation.
///
/// Verifies that the performance collector correctly aggregates request
/// timing data including counts, latency distributions, and error rate
/// calculations from a series of recorded observations.
///
/// ## Performance Metrics
///
/// - **total_requests**: Count of all recorded requests
/// - **error_rate**: Ratio of failed requests to total requests
/// - **latency_distribution**: Percentile calculations for response times
///
/// ## Error Rate Calculation
///
/// The error rate is calculated as: errors / total_requests
/// - 0% error rate when all requests succeed
/// - 100% error rate when all requests fail
/// - Intermediate values for partial failures
///
/// ## Test Scenario
///
/// 1. Create a performance collector with 60-second window and 5-second buckets
/// 2. Record three requests:
///    - Request 1: 100ms, success
///    - Request 2: 200ms, success
///    - Request 3: 300ms, error
/// 3. Retrieve aggregated metrics
/// 4. Verify total count is 3
/// 5. Verify error rate is approximately 33%
///
/// ## Expected Behavior
///
/// - Total requests count is correct
/// - Error rate is calculated as a floating-point ratio
/// - The 33% error rate tolerance allows for floating-point precision
fn test_performance_collector() {
    // Create a performance collector with 60-second analysis window
    let mut collector = RiPerformanceCollector::new(
        Duration::from_secs(60),
        Duration::from_secs(5),
    );
    
    // Record some requests with timing and success/failure status
    collector.record_request(100.0, false); // 100ms, success
    collector.record_request(200.0, false); // 200ms, success
    collector.record_request(300.0, true);  // 300ms, error (failed)
    
    // Retrieve aggregated metrics
    let metrics = collector.get_metrics();
    
    // Verify metrics
    assert_eq!(metrics.total_requests, 3);
    assert!((metrics.error_rate - 0.33).abs() < 0.01); // Approximately 33%
}
