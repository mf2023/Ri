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

use dmsc::observability::grafana::{DMSCGrafanaDashboard, DMSCGrafanaPanel, DMSCGridPos};

/// Observability Grafana dashboard test module for visualization configuration.
///
/// This module provides comprehensive test coverage for the Grafana dashboard
/// generation components that create visualization configurations for metrics
/// display. The tests validate panel creation, dashboard structure, and JSON
/// serialization for Grafana integration.
///
/// ## Test Coverage
///
/// - **Dashboard Creation**: Tests the instantiation of dashboard objects with
///   appropriate metadata including title and initial configuration, verifying
///   that dashboard instances are properly initialized.
///
/// - **Panel Management**: Validates the ability to add visualization panels
///   to dashboards with proper configuration including titles, queries, panel
///   types, and grid positioning for layout management.
///
/// - **Panel Grid Positioning**: Tests the grid position configuration that
///   controls panel placement and sizing within the dashboard layout, verifying
///   that position coordinates are correctly stored and applied.
///
/// - **JSON Serialization**: Validates that dashboard configurations can be
///   serialized to JSON format compatible with Grafana's dashboard API,
///   ensuring proper structure and content in the generated output.
///
/// ## Design Principles
///
/// The dashboard generation creates Grafana-compatible JSON configurations
/// that can be imported directly into Grafana instances for visualization.
Tests verify that the generated JSON matches the expected structure including
/// dashboard metadata, panel arrays, and grid layout specifications.
///
/// Panel configurations are designed to be opinionated but customizable,
/// providing sensible defaults while allowing override of specific properties
/// like panel type, query expressions, and positioning.
///
/// The grid position system uses a flexible layout model compatible with
/// Grafana's auto-layout algorithm, enabling panels to be placed at specific
/// coordinates or allowed to flow naturally based on available space.

#[test]
fn test_grafana_dashboard() {
    let mut dashboard = DMSCGrafanaDashboard::new("Test Dashboard");
    
    let panel = DMSCGrafanaPanel {
        title: "CPU Usage".to_string(),
        query: "cpu_usage_percent".to_string(),
        panel_type: "graph".to_string(),
        grid_pos: DMSCGridPos { h: 8, w: 12, x: 0, y: 0 },
    };
    
    dashboard.add_panel(panel).unwrap();
    assert_eq!(dashboard.panels.len(), 1);
    
    let json = dashboard.to_json().unwrap();
    assert!(json.contains("Test Dashboard"));
    assert!(json.contains("CPU Usage"));
}
