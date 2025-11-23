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

use serde::{Serialize, Deserialize};
use crate::core::DMSResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSGrafanaPanel {
    pub title: String,
    pub query: String,
    pub panel_type: String,
    pub grid_pos: DMSGridPos,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSGridPos {
    pub h: i32,
    pub w: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DMSGrafanaDashboard {
    pub title: String,
    pub panels: Vec<DMSGrafanaPanel>,
}

impl DMSGrafanaDashboard {
    pub fn _Fnew(title: &str) -> Self {
        DMSGrafanaDashboard {
            title: title.to_string(),
            panels: Vec::new(),
        }
    }
    
    pub fn _Fadd_panel(&mut self, panel: DMSGrafanaPanel) -> DMSResult<()> {
        self.panels.push(panel);
        Ok(())
    }
    
    pub fn _Fto_json(&self) -> DMSResult<String> {
        serde_json::to_string(self).map_err(|e| crate::core::DMSError::Serde(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grafana_dashboard() {
        let mut dashboard = DMSGrafanaDashboard::_Fnew("Test Dashboard");
        
        let panel = DMSGrafanaPanel {
            title: "CPU Usage".to_string(),
            query: "cpu_usage_percent".to_string(),
            panel_type: "graph".to_string(),
            grid_pos: DMSGridPos { h: 8, w: 12, x: 0, y: 0 },
        };
        
        dashboard._Fadd_panel(panel).unwrap();
        assert_eq!(dashboard.panels.len(), 1);
        
        let json = dashboard._Fto_json().unwrap();
        assert!(json.contains("Test Dashboard"));
        assert!(json.contains("CPU Usage"));
    }
}