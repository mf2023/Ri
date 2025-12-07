// Copyright © 2025 Wenze Wei. All Rights Reserved.
//
// This file is part of DMS.
// The DMS project belongs to the Dunimd Team.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate dms;

use std::time::Duration;
use dms::observability::metrics_collector::{DMSSlidingWindow, DMSQuantileCalculator, DMSPerformanceCollector};

#[test]
fn test_sliding_window() {
    let mut window = DMSSlidingWindow::<i32>::new(
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
    let mut calc = DMSQuantileCalculator::new();
    
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
    let mut collector = DMSPerformanceCollector::new(
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
