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

use dms::observability::prometheus::DMSPrometheusExporter;

#[test]
fn test_prometheus_exporter() {
    let exporter = DMSPrometheusExporter::_Fnew().unwrap();
    
    exporter._Fregister_counter("test_counter", "A test counter").unwrap();
    exporter._Fincrement_counter("test_counter", 1.0).unwrap();
    
    exporter._Fregister_gauge("test_gauge", "A test gauge").unwrap();
    exporter._Fset_gauge("test_gauge", 42.0).unwrap();
    
    let output = exporter._Frender().unwrap();
    assert!(output.contains("test_counter"));
    assert!(output.contains("test_gauge"));
}

#[test]
fn test_grafana_dashboard_generation() {
    let exporter = DMSPrometheusExporter::_Fnew().unwrap();
    let dashboard = exporter._Fgenerate_default_dashboard().unwrap();
    
    assert_eq!(dashboard.title, "DMS Metrics Dashboard");
    assert_eq!(dashboard.panels.len(), 3);
}
