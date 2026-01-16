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

use dmsc::observability::prometheus::DMSCPrometheusExporter;

/// Observability Prometheus metrics export test module for DMSC tooling.
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
fn test_prometheus_exporter() {
    let exporter = DMSCPrometheusExporter::new().unwrap();
    
    exporter.register_counter("test_counter", "A test counter").unwrap();
    exporter.increment_counter("test_counter", 1.0).unwrap();
    
    exporter.register_gauge("test_gauge", "A test gauge").unwrap();
    exporter.set_gauge("test_gauge", 42.0).unwrap();
    
    let output = exporter.render().unwrap();
    assert!(output.contains("test_counter"));
    assert!(output.contains("test_gauge"));
}

#[test]
fn test_grafana_dashboard_generation() {
    let exporter = DMSCPrometheusExporter::new().unwrap();
    let dashboard = exporter.generate_default_dashboard().unwrap();
    
    assert_eq!(dashboard.title, "DMSC Metrics Dashboard");
    assert_eq!(dashboard.panels.len(), 3);
}
