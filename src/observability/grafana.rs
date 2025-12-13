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

//! # Grafana Integration Module
//! 
//! This module provides data structures for creating and managing Grafana dashboards and panels.
//! It enables programmatic generation of Grafana dashboards with panels, queries, and layout information.
//! 
//! ## Key Components
//! 
//! - **DMSGridPos**: Represents the grid position of a panel on a dashboard
//! - **DMSGrafanaPanel**: Represents a single Grafana panel with title, query, type, and position
//! - **DMSGrafanaDashboard**: Represents a Grafana dashboard with multiple panels
//! 
//! ## Design Principles
//! 
//! 1. **Serde Integration**: All structs implement Serialize and Deserialize for easy JSON conversion
//! 2. **Simple API**: Easy-to-use methods for creating dashboards and adding panels
//! 3. **Layout Support**: Built-in support for Grafana's grid layout system
//! 4. **Extensible**: Can be extended to support additional panel types and dashboard features
//! 5. **Type Safety**: Strongly typed structs for all Grafana components
//! 6. **JSON Compatibility**: Generates JSON that is compatible with Grafana's API
//! 
//! ## Usage
//! 
//! ```rust
//! use dms::prelude::*;
//! 
//! fn example() -> DMSResult<()> {
//!     // Create a new dashboard
//!     let mut dashboard = DMSGrafanaDashboard::new("DMS Metrics");
//!     
//!     // Create a panel
//!     let panel = DMSGrafanaPanel {
//!         title: "Request Rate".to_string(),
//!         query: "rate(http_requests_total[5m])".to_string(),
//!         panel_type: "graph".to_string(),
//!         grid_pos: DMSGridPos {
//!             h: 8,
//!             w: 12,
//!             x: 0,
//!             y: 0,
//!         },
//!     };
//!     
//!     // Add panel to dashboard
//!     dashboard.add_panel(panel)?;
//!     
//!     // Convert to JSON for Grafana API
//!     let json = dashboard.to_json()?;
//!     println!("Dashboard JSON: {}", json);
//!     
//!     Ok(())
//! }
//! ```

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

#[allow(dead_code)]
impl DMSGrafanaDashboard {
    pub fn new(title: &str) -> Self {
        DMSGrafanaDashboard {
            title: title.to_string(),
            panels: Vec::new(),
        }
    }
    
    pub fn add_panel(&mut self, panel: DMSGrafanaPanel) -> DMSResult<()> {
        self.panels.push(panel);
        Ok(())
    }
    
    pub fn to_json(&self) -> DMSResult<String> {
        serde_json::to_string(self).map_err(|e| crate::core::DMSError::Serde(e.to_string()))
    }
}
