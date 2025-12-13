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

use dms_core::observability::grafana::{DMSGrafanaDashboard, DMSGrafanaPanel, DMSGridPos};

#[test]
fn test_grafana_dashboard() {
    let mut dashboard = DMSGrafanaDashboard::new("Test Dashboard");
    
    let panel = DMSGrafanaPanel {
        title: "CPU Usage".to_string(),
        query: "cpu_usage_percent".to_string(),
        panel_type: "graph".to_string(),
        grid_pos: DMSGridPos { h: 8, w: 12, x: 0, y: 0 },
    };
    
    dashboard.add_panel(panel).unwrap();
    assert_eq!(dashboard.panels.len(), 1);
    
    let json = dashboard.to_json().unwrap();
    assert!(json.contains("Test Dashboard"));
    assert!(json.contains("CPU Usage"));
}
