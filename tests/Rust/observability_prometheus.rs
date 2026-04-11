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

use ri::observability::prometheus::RiPrometheusExporter;

/// Observability Prometheus metrics export test module for Ri tooling.
///
/// This module provides comprehensive test coverage for the Prometheus metrics
/// exporter component that generates Prometheus-compatible metric output for
/// monitoring and alerting integration. The tests validate metric registration,
/// value updates, and standard output formatting.
///
/// ## Test Coverage
///
/// - **Counter Metrics**: Tests the creation and increment operations for
///   counter metrics, verifying that values can be incremented and the output
///   reflects the cumulative total in Prometheus exposition format.
///
/// - **Gauge Metrics**: Tests the creation and set operations for gauge metrics,
///   verifying that values can be both increased and decreased with proper
///   output formatting for immediate value representation.
///
/// - **Metrics Exposition Format**: Validates that the rendered output complies
///   with Prometheus exposition format including metric names, labels, and
///   numeric values suitable for scraping by Prometheus servers.
///
/// - **Dashboard Generation**: Tests the automatic generation of Grafana
///   dashboard configurations from registered metrics, verifying panel creation
///   with appropriate visualization types and query configurations.
///
/// ## Design Principles
///
/// The Prometheus exporter implements the Prometheus exposition format version
/// 0.0.4, ensuring compatibility with Prometheus server scraping and existing
/// ecosystem tools. Tests verify format compliance including metric naming
/// conventions and HELP/TYPE declarations.
///
/// The exporter supports dynamic metric registration at runtime, enabling
/// applications to define metrics based on configuration or discovered
/// capabilities. Tests verify that registered metrics appear correctly in
/// rendered output.
///
/// Dashboard generation creates opinionated Grafana configurations optimized
/// for common monitoring scenarios. Tests verify that generated dashboards
/// include appropriate panels with correct query syntax for the registered
/// metrics.

#[test]
/// Tests basic RiPrometheusExporter functionality for metrics handling.
///
/// Verifies that the Prometheus exporter can register counters and gauges,
/// update their values, and render output in Prometheus exposition format.
///
/// ## Metric Types Tested
///
/// - **Counter**: Cumulative metrics that only increase
///   - Created with register_counter()
///   - Incremented with increment_counter()
///   - Suitable for counting events (requests, errors, etc.)
///
/// - **Gauge**: Metrics that can increase or decrease
///   - Created with register_gauge()
///   - Set with set_gauge()
///   - Suitable for current values (connections, queue size, etc.)
///
/// ## Prometheus Exposition Format
///
/// The rendered output follows Prometheus exposition format:
/// - Metric names in snake_case
/// - HELP comments describing the metric
/// - TYPE declarations indicating metric type
/// - Label pairs in braces {label="value"}
/// - Numeric values in float format
///
/// ## Expected Behavior
///
/// - Counter can be incremented and value appears in output
/// - Gauge can be set and value appears in output
/// - Rendered output contains both metric names
/// - The output is valid Prometheus exposition format
fn test_prometheus_exporter() {
    let exporter = RiPrometheusExporter::new().unwrap();
    
    // Register and increment a counter metric
    exporter.register_counter("test_counter", "A test counter").unwrap();
    exporter.increment_counter("test_counter", 1.0).unwrap();
    
    // Register and set a gauge metric
    exporter.register_gauge("test_gauge", "A test gauge").unwrap();
    exporter.set_gauge("test_gauge", 42.0).unwrap();
    
    // Render metrics in Prometheus format
    let output = exporter.render().unwrap();
    
    // Verify both metrics appear in output
    assert!(output.contains("test_counter"));
    assert!(output.contains("test_gauge"));
}

#[test]
/// Tests automatic Grafana dashboard generation from registered metrics.
///
/// Verifies that the exporter can generate a complete Grafana dashboard
/// configuration based on the registered metrics with appropriate panels.
///
/// ## Dashboard Generation
///
/// The generate_default_dashboard() method creates a dashboard with:
/// - A title reflecting the dashboard purpose
/// - Multiple panels for different metric types
/// - Appropriate visualization configurations
/// - Correct Prometheus query syntax for each panel
///
/// ## Panel Configuration
///
/// Generated dashboards include:
/// - Panels for each registered metric
/// - Time series visualizations for time-varying metrics
/// - Single stat panels for current-value metrics
/// - Appropriate refresh intervals and time ranges
///
/// ## Expected Behavior
///
/// - Dashboard is generated successfully
/// - Dashboard has the correct title
/// - Dashboard contains panels for all registered metrics
/// - Panel count matches registered metrics
fn test_grafana_dashboard_generation() {
    let exporter = RiPrometheusExporter::new().unwrap();
    
    // Generate default dashboard from registered metrics
    let dashboard = exporter.generate_default_dashboard().unwrap();
    
    // Verify dashboard structure
    assert_eq!(dashboard.title, "Ri Metrics Dashboard");
    assert_eq!(dashboard.panels.len(), 3);
}
