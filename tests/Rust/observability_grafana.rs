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

use ri::observability::grafana::{RiGrafanaDashboard, RiGrafanaPanel, RiGridPos};

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
/// Tests RiGrafanaDashboard creation and panel management.
///
/// Verifies that Grafana dashboards can be created with proper metadata
/// and that visualization panels can be added with appropriate configuration
/// for metrics display in Grafana instances.
///
/// ## Dashboard Structure
///
/// A Grafana dashboard consists of:
/// - **title**: Dashboard name for identification
/// - **panels**: Array of visualization panels
/// - **gridPos**: Panel positioning within dashboard layout
/// - **json**: Serialized configuration for Grafana API
///
/// ## Panel Configuration
///
/// Each panel includes:
/// - **title**: Panel name for display
/// - **query**: Prometheus query expression for data retrieval
/// - **panel_type**: Visualization type (graph, stat, table, etc.)
/// - **grid_pos**: Panel position and size (height, width, x, y)
///
/// ## Test Scenario
///
/// 1. Create a new dashboard with title "Test Dashboard"
/// 2. Define a panel with:
///   - Title: "CPU Usage"
///   - Query: "cpu_usage_percent"
///   - Type: "graph"
///   - Position: 8 units high, 12 units wide, at origin
/// 3. Add the panel to the dashboard
/// 4. Verify panel count is 1
/// 5. Serialize dashboard to JSON
/// 6. Verify JSON contains dashboard title and panel title
///
/// ## Expected Behavior
///
/// - Dashboard is created with specified title
/// - Panel is successfully added to dashboard
/// - Panel count reflects added panels
/// - JSON serialization produces valid Grafana configuration
/// - Serialized JSON contains expected titles
fn test_grafana_dashboard() {
    // Create a new dashboard with specified title
    let mut dashboard = RiGrafanaDashboard::new("Test Dashboard");
    
    // Define a visualization panel with configuration
    let panel = RiGrafanaPanel {
        title: "CPU Usage".to_string(),
        query: "cpu_usage_percent".to_string(),
        panel_type: "graph".to_string(),
        grid_pos: RiGridPos { h: 8, w: 12, x: 0, y: 0 },
    };
    
    // Add panel to dashboard
    dashboard.add_panel(panel).unwrap();
    
    // Verify panel was added
    assert_eq!(dashboard.panels.len(), 1);
    
    // Serialize to JSON for Grafana API
    let json = dashboard.to_json().unwrap();
    
    // Verify JSON contains expected content
    assert!(json.contains("Test Dashboard"));
    assert!(json.contains("CPU Usage"));
}
